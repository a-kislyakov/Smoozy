//! **Files — a server's main face, and the largest single screen in the
//! drawing.** The page half is `web/files.js`.
//!
//! **This file is the surface and not the deciding.** Every judgement below is
//! `File Browser`'s, in `App/File Browser/`, and this file translates: it reads
//! what there is, asks that module what it means, composes the words, and hands
//! them over. **Nothing here decides what a collision is, what a warning names,
//! or whether an offer exists.**
//!
//! ## What this piece draws, cited the way the runner ruled on 2026-09-12
//!
//! **The Design document and the heading**, and **the mockup state names**.
//!
//! - `Design/File Browser § Where it sits`, `§ The list`, `§ Acting on files`,
//!   `§ Uploading`, `§ When the files are already there`,
//!   `§ Renaming into a name that is taken`,
//!   `§ The browser opened so the user can point at a file`,
//!   `§ When another module moves the browser to a different server`,
//!   `§ The live-job warning, on all four acts`, `§ Deleting`,
//!   `§ Disturbing a file nothing live is using`, `§ Keyboard shortcuts`,
//!   `§ While the connection is down`.
//! - The mockup states, which are the `<option>` values of
//!   `Preparation/Design/Mockup/Smoozy.html`'s own state list and are what this
//!   half is counted by. **Thirty — the *A server* group entire, less the
//!   sixteen `chk-` states, which are the dependency check's** (nine from piece
//!   9, `chk-added` and `chk-folders` from the plan pass for the Creator's
//!   notes on 0.1.8, `chk-reading` from the drawing pass for their notes on
//!   the test copy, `chk-clean` and `chk-sel` from the dependency check's
//!   fresh look, `chk-none` from the drawing pass for the Creator's
//!   thirteenth notes, and `chk-nosearch` from the drawing pass for their
//!   fourteenth): `browse`, `browse-other`,
//!   `fb-reading`, `drag`, `menus`, `offline`, `collide`, `collide-live`,
//!   `collide-resume`, `collide-cut`, `collide-copy`, `collide-dl`,
//!   `fb-upkind`, `delete`, `delete-dep`, `delete-small`, `fb-live-rename`,
//!   `fb-live-move`, `fb-pick`, `fb-moved`, `fb-movedup`, `fb-rename`,
//!   `fb-renamehint`, `fb-refused`, `fb-refusednone`, `fb-refusedplain`,
//!   `fb-newname`, `fb-newfolder`, `fb-readnone`, `fb-multi`, `fb-hidden`,
//!   `fb-check`, `fb-sizemenu`, `fb-size`, `fb-sizeold`, `fb-view`,
//!   `fb-viewexr`, `fb-viewbig`. *(The last three drawn 2026-10-31 by the
//!   plan and drawing pass for the seventeenth notes, note 11 — a picture
//!   looked at in Files — and built the same day. The three before them
//!   drawn 2026-10-30 by the twenty-third notes' design pass, note 2, and
//!   built the same day. The two before them drawn 2026-10-29 by the twenty-first notes'
//!   design pass, notes 3 and 6, and not yet built here. Twenty-nine until
//!   2026-10-03, when the drawing pass for the Creator's notes on the test copy
//!   drew `fb-newname`, a new folder being named, their note 4. Thirty until
//!   2026-10-04, when the drawing pass for their second notes dropped
//!   `upconf`: there is no confirmation before an upload, their note 2. That
//!   map's piece 9 took the built one out on 2026-10-06. Twenty-nine again
//!   until 2026-10-18, when the drawing pass for their thirteenth notes drew
//!   `fb-multi`, three rows chosen, their note 6.)* **A test
//!   reads the drawing's own list and fails if the two have drifted, in both
//!   directions.**
//!
//! ## What can actually happen today, said plainly
//!
//! **Files lists the real machine and uploads to it, from step 10c piece 4.**
//! Until then nothing in this application listed a machine: step 10b piece 5
//! built connecting, and step 10c piece 2 built what lends a connection's
//! sessions out — `Server protocol and transfers` T5's per-server point,
//! `src/servers/sessions.rs`. *(This file blamed `Servers` Q1 step 7 until that
//! piece, and Q1 step 7 hands out no session.)* **This screen asks that point
//! now**:
//!
//! - **The listing** is one directory reading on the connection's reading
//!   session (`File Browser` L1), with the free-space figure beside it
//!   (`File Browser` L11). A folder the login may not read is told apart from
//!   one that is not there (`File Browser` P1), and the component that refused
//!   is named by `File Browser` P3's walk.
//! - **Upload and a drop** run to published bytes on the machine through
//!   `src/uploading.rs`: the request written down (`File Browser` U1), the
//!   already-exists walk and its one question (`File Browser` X1,
//!   `File Browser` U4), the
//!   landing folders, and each file admitted, sent, published and recorded.
//!
//! **A server this run never held a period for shows no listing, because no
//! entry was ever made** — `File Browser` L14's own last sentence — and that is
//! what this screen draws wherever no connection is live.
//!
//! **What still cannot happen, each for its own reason, and the screen says so
//! in the words the All servers screen already uses for an act that is not
//! finished:**
//!
//! - **The stale listing of `File Browser` L15.** Keeping a folder's last
//!   listing across a drop wants the tree to be told which of `Servers` S10's
//!   six endings ended the period (`File Browser` L16), and the endings arrive
//!   in `src/servers/a_period.rs`, which is not this screen's. So a listing is
//!   read afresh on every visit and nothing is shown where no connection is
//!   live.
//! - **Delete, rename, move, copy, paste, a new folder, the elevated retry and
//!   the hand-over to the dependency check.** Each is its own act on the machine
//!   and none is built. **The offer to try again with admin access is not drawn
//!   beside a refusal** for the same reason: the retry it would start is not
//!   built. **Download left this list at step 10c**: the toolbar's control and
//!   the two menu items now send what the user picked to Transfers, which
//!   composes `Download` M1's request over it.
//! - **The collision box in its readings other than an upload's and a
//!   download's**, which belong to the acts still owed above.
//!
//! **The two warnings read the local record, from step 10c's Files' owed acts
//! piece 1.** They are answered with no read of the machine (`File Browser`
//! W1), through
//! [`crate::record::neighbouring_records::WhatTheLocalRecordHolds`]:
//! `Dependency Upload & Remap`'s own record, the local job records and the
//! run's queue reading. *(Until that piece this head said the eleven
//! `dependency_…` tables had no reader anywhere. That stopped being true at
//! the dependency wave's piece 4, which gave them a keeper, and every box here
//! went on being composed from nothing — so an upload's *Replace* over a
//! texture a rendering job reads warned nobody.)*
//!
//! **The `.blend` version tag runs whole, from step 10c's twelfth-pass fix
//! piece 6b** (`File Browser` L7 to L10): the rows come back first, and the page
//! then asks for each `.blend`'s tag, one prefix reading at a time on the
//! connection's reading session, behind anything the user is waiting on — see
//! [`the_saved_with_version_of`]. *(Until then this head said the tag was blank
//! because its reading took the decoder as a trait object that could not cross
//! threads. That was true, and it meant Render Settings' automatic match never
//! ran on a real file: every file needed a Blender picked by hand, and the page
//! said the file did not say which Blender saved it.)*
//!
//! **What runs whole all the same, and did before this piece:**
//!
//! - **The system file picker**, which this piece is the first caller of
//!   anywhere in Smoozy. Pressing Upload puts a real panel on the screen.
//! - **The kind question** (`fb-upkind`), on Windows and Linux only.
//!   [`crate::picker`] answers [`crate::picker::Picked::ThereIsNoPanelForThisAsk`]
//!   for files and folders together there; on a Mac its own panel takes both
//!   and nothing is asked (the Creator's second notes on the test copy, piece
//!   9, note 2).
//! - **No confirmation**: a pick or a drop starts the upload, with the paths
//!   it handed over and nothing else, which is `File Browser` H3a's own bound.
//! - **The drop reading** (`drag`), from the shell's own drag-and-drop paths.
//! - **The path arithmetic** — the crumbs, the folder above, the landing paths
//!   — all of which is byte work on the desktop.
//! - **The toolbar's own rule** (`File Browser` N7a), which is the chosen file's kind and
//!   nothing else.
//!
//! **Nothing is stubbed and nothing pretends.** A control whose act is owed
//! draws exactly as Design draws it and answers with the one thing this piece
//! can honestly say — [`TheAnswer::TheActIsNotBuiltYet`], which is piece 3's
//! shape and its reasoning carries here unchanged.
//!
//! ## `WhoseSentenceItIs`
//!
//! **Nothing in this piece composes a failure sentence, and a test says so.**
//! A refusal on a row is not a failure record: nothing durable is written, no
//! [`crate::record::failure_records::Standing`] is raised, and there is
//! therefore no diary capture to spend. **The screen that composes a real
//! failure sentence for this module is the transfer window** — `Design/File
//! Browser`'s own sentence, *"A refused write on an upload surfaces on the
//! transfer row"* — and that is piece 12's.
//!
//! ## How a clause marker is written in this crate
//!
//! A marker with no document beside it means a clause of this crate's own plan,
//! `Preparation/Architecture/Desktop app and delivery.md`. **Every other
//! document's marker is written with that document** — at the site, or in the
//! header list below — because this crate's plan defines an H-series, a
//! K-series, an L-series, an M-series, an N-series, a P-series, a W-series and
//! an X-series of its own, and a bare one does not fail to resolve: it resolves
//! to a real and wrong clause, which nobody reading it can tell from a right
//! one.
//!
//! **Every marker of another document is written out with that document at its
//! own site**, and there is no header list: this crate's plan defines an H, an
//! L, an M, a P, a W and an X of nearly every number `File Browser` does, and a
//! header list cannot tell the two apart.

use std::sync::{Arc, Mutex};

use tauri::{AppHandle, Manager, Runtime, State};

use smoozy_file_browser::already_exists::{
    self, AComparison, CollisionAnswer, Comparison, TheAnswer as TheWalksAnswer,
};
use smoozy_file_browser::cache::{
    availability, how_the_browser_composes, AnActThatNeedsTheMachine, Availability,
    HowTheBrowserComposes,
};
use smoozy_file_browser::blend_version::{read_the_tag, PrefixGrowth, Tag, TagKey, TagsThisPeriod};
use smoozy_file_browser::listing::{
    the_list_shows, ApparentSize, DirectoryAnswer, EntryName, Kind, LastChanged, Lease, ListedEntry,
    ReadsADirectory,
};
use smoozy_file_browser::managing::{the_answers_offered, CutOrCopy, TheAnswersOffered};
use smoozy_file_browser::partial::THE_RESERVED_STAGING_FOLDER;
use smoozy_file_browser::permission::{self, Offer, WalkOutcome, WhatSmoozyMet, WhyThereIsNoOffer};
use smoozy_file_browser::uploading::{landing_path, the_toolbar_for};
use smoozy_file_browser::warnings::{AJobNamed, HowTheJobStands};
use smoozy_file_browser::wiring::{TheReadingChannel, TheZstandardDecoder};
use smoozy_server_protocol::free_space;
use smoozy_servers::condition::ObservedValue;
use smoozy_servers::store::ServerRow;
use smoozy_state::identities::{Identity, IdentityKind};
use smoozy_state::outcomes::UnitOutcome;

use crate::downloading::TheRunsTransfers;
use crate::events::{Event, Said, Subject as About, TheOneQueue};
use crate::picker::{self, Asking, HowMany, Picked, WhatMayBeChosen};
use crate::record::server_rows;
use crate::servers::a_period::ThePeriodsThisRun;
use crate::servers::sessions::{AHeldConnection, ThisServersSessions};
use crate::start::Started;
use crate::typography::{Prose, Verbatim};
use crate::uploading::{
    self, AnUploadReadyToRun, TheRecordAnUploadWrites, WhatIsBeingUploaded, WhatTheAnswerLeft,
    WhatTheUploadCameTo, WhenAFileIsPublished, WhyTheUploadDidNotStart,
};

/// **The one kind this piece posts**, declared here beside the service that
/// posts it and copied into `web/files.js`. Nothing holds the two together, so
/// copy carefully.
///
/// **One kind, because every act on this screen answers one question**: what
/// the user asked the browser to do. A second kind would be a second drawing
/// path for one surface. That is piece 1's own shape and its reasoning carries
/// here unchanged.
pub const THE_ANSWER: &str = "files:the-answer";

/// **`File Browser` M3's suppression identity, declared there and read here.**
///
/// *"Its identity is declared here, stable for the life of the application:
/// `file-browser.delete`"*. It is [`smoozy_file_browser::declarations`]'s own
/// constant and is not written out a second time.
pub fn the_delete_warnings_suppression_row() -> &'static str {
    smoozy_file_browser::managing::the_delete_warnings_suppression_row()
}

/// The module name the suppression table and the declaration register know
/// this module by.
pub const THIS_MODULE: &str = "File Browser";

/// **The Go menu item this piece makes live.**
///
/// `src/menus.rs` holds ten items whose act is `Act::AGoing` and its own
/// comment says every one is a screen or a panel that does not exist yet.
/// **This is the one that is a control on a screen rather than a place**, so it
/// does not join `web/the-rail.js`'s own map of goings to places: the going
/// arrives on the one ordered queue like any other, and the module that owns
/// the act performs it. **`src/menus.rs` is not edited to make it live** and
/// nothing about the item moves.
pub const UP_ONE_FOLDER: &str = "go.up-one-folder";

/// **The one sentence every act on a server's own screens answers with while
/// nothing in this application sends a server an instruction** — Set up, the
/// render settings (its read and its submit), the dependency check and the
/// Queue say it. **Files said it too until step 10c piece 4, which made its
/// listing and its upload real**: on this screen the sentence would now be
/// false, so Files no longer reads it, and says the All servers screen's own
/// sentence for the acts it still owes (`words::NOT_BUILT_YET`). **The constant
/// stays here, where the four still read it**, until the last of them stops.
///
/// **Opening a connection is built**, since step 10b piece 5, so the sentence
/// no longer says that is what is missing. **What is missing is the instruction
/// that would travel over one.** *(Corrected at step 10c piece 2: this blamed
/// `Servers` Q1 step 7, which hands out no session. What does is
/// `Server protocol and transfers` T5's per-server point, and that piece built
/// it — `src/servers/sessions.rs` lends the reading session, one command at a
/// time and a transfer's channel, and asks the machine's supervisor a framed
/// request.)* **What is still true is that no screen's act uses it yet**:
/// nothing on the desktop composes a request for a machine's supervisor to
/// carry out, and each screen stops saying this sentence in the piece that
/// makes its own acts real. The wording is the build runner's
/// ruling of 2026-09-18, `Audits/Build/Step 10b/Ruling — the sentence five
/// screens say is no longer true.md`, and it claims nothing about the moment:
/// it stays true whether or not a connection stands, which is what lets it stay
/// one constant.
///
/// **One constant, so the five pieces cannot drift apart again.** Until the
/// step 10a round 1 audit of these pieces they said it four ways, one of which
/// (*Smoozy could not reach the server*) sent a user whose server is healthy
/// looking for a network fault that does not exist. **It is not a failure
/// record, it is not a failure sentence, and it is never handed to the diary**
/// — nothing was attempted (`Desktop app and delivery` §16.8 M32, second limb).
pub const A_CONNECTION_IS_NOT_BUILT: &str =
    "Not available yet.";

// ════════════════════════════════════════════════════════════════════════════
// The words
// ════════════════════════════════════════════════════════════════════════════
//
// **Every word the user reads on this screen is here, each as its own string,
// and each goes through the typography point on its way out** (A6, X1, X10).
// **No value is ever joined into one** — X7, and the folder head of
// `src/presentation/mod.rs` names that join as the trap of this whole step.
//
// **The mockup's every string is a literal, so the real work is the split**:
// which of them is prose the application composes and which is a value that
// travels verbatim. `kitchen/`, `kitchen_v14.blend`, `284 MB`, `61 GB`,
// `today 09:12`, `5.1` and every job's name are values. Everything below is
// prose.

mod words {
    // `Design/File Browser § Where it sits`, and the tab strip's own name for
    // this tab, which is piece 5's and is not repeated here.

    // ── The bar along the top ────────────────────────────────────────────────

    // `Design/File Browser § While the connection is down` — *"The listing
    // stays and says it is offline, with when it was last updated."*
    pub const NOT_LIVE_LEAD: &str = "Offline — last updated";
    pub const NOT_LIVE_REST: &str = "Reconnecting.";

    // `Design/File Browser § The browser opened so the user can point at a
    // file` — the bar says who is asking and for what, with a way out beside
    // it.
    pub const PICK_LEAD: &str = "Pick the .blend for";
    pub const PICK_REST: &str = ".";
    pub const CANCEL: &str = "Cancel";

    // `Design/File Browser § When another module moves the browser to a
    // different server`. **The second sentence is drawn only where an upload is
    // actually in flight** — on a quiet browser it would answer a question
    // nobody asked.
    pub const MOVED_LEAD: &str = "Moved to";
    pub const MOVED_REST: &str = "to show this job's outputs.";
    pub const STILL_UPLOADING: &str =
        "Uploads in progress continue to their original server.";

    // The re-submission bar. **Its state is `res-find`, which is the Render
    // settings group's and piece 8's**; the bar's markup is drawn in this
    // screen's body, so the shape is here and the raising is that piece's.
    pub const FIND_LEAD: &str = "Submitting";
    pub const FIND_REST: &str = "again. Its server was removed and its old folder does not exist here — find its .blend on this server.";

    // ── The toolbar ─────────────────────────────────────────────────────────

    // `Design/File Browser § The list` — *"The file manager's eight are icons;
    // Smoozy's two are words."* **Every icon carries its word on hover and on
    // keyboard focus**, so each of the eight is prose even though it is drawn
    // as a shape.
    pub const RENDER_SETTINGS: &str = "Render settings…";
    pub const CHECK_DEPENDENCIES: &str = "Check dependencies…";
    pub const UPLOAD_CONTROL: &str = "Upload…";
    pub const DOWNLOAD: &str = "Download";
    pub const NEW_FOLDER: &str = "New folder";
    pub const RENAME: &str = "Rename";
    pub const CUT: &str = "Cut";
    pub const COPY: &str = "Copy";
    pub const PASTE: &str = "Paste";
    pub const PASTE_ITEMS: &str = "items";
    pub const PASTE_ONE_ITEM: &str = "item";
    pub const DELETE: &str = "Delete";
    // *"How many files are chosen is said at the far end of the toolbar."*
    pub const SELECTED: &str = "selected";

    // ── The list ────────────────────────────────────────────────────────────

    // `Design/File Browser § The list` — *"Four things per file … name, kind,
    // size, and when it last changed."* **The headers sort.**
    pub const NAME: &str = "Name";
    pub const KIND: &str = "Kind";
    pub const SIZE: &str = "Size";
    pub const CHANGED: &str = "Changed";

    // The five kinds, as the drawing writes them. **Rule 9: the industry's own
    // words**, and *Blender file* is what a 3D artist calls a `.blend`.
    pub const A_FOLDER: &str = "Folder";
    pub const A_BLEND: &str = "Blender file";
    pub const AN_IMAGE: &str = "Image";
    pub const AN_ARCHIVE: &str = "Archive";
    pub const A_FILE: &str = "File";
    // A symbolic link is told apart because `File Browser` L1's own `Kind`
    // does, and nothing here follows one.
    pub const A_LINK: &str = "Link";

    // **The place a value would have stood and there is none.** The drawing
    // writes an em dash in the size and changed columns of a row that has
    // neither. It is punctuation rather than a word and it is drawn by the page
    // in a class of its own; nothing here composes it into a sentence.

    // `Design/File Browser § The list` — a folder being read. *"Smoozy has no
    // total to count against until the machine answers, so it says so rather
    // than showing a figure it does not have."*
    pub const READING_LEAD: &str = "Reading";
    pub const READING_REST: &str = "— waiting for the server.";

    // **`File Browser` L14's own last sentence, and it is the state this
    // application is always in today**: a server whose files were never listed
    // shows no listing, because no entry was ever made. **Smoozy connects now**
    // — step 10b piece 5 — **and what is missing is listing a server's files**:
    // the reading session a listing runs on is lent by `Server protocol and
    // transfers` T5's per-server point (built at step 10c piece 2, which
    // corrected this from `Servers` Q1 step 7), and nothing here asks it yet.
    // So the rest of the sentence says that, and is true whether or not a
    // connection is open (step 10b round 1 of *jobs, history
    // and downloads*, must-do 7). *(It read "It reads them when it connects,
    // and it cannot connect yet." — which a user who had just watched the
    // server connect was told on this screen.)* It is not the five-screen
    // sentence: that one is about a different missing thing. **The words are
    // ruled here** — see this file's *rulings*.
    //
    // **Corrected at step 10c piece 4, which built the reading**: the rest said
    // *"Reading them is not built yet."* It now says when Smoozy does read them,
    // and it still never says Smoozy cannot connect.
    pub const NOTHING_LISTED_LEAD: &str = "Not listed yet.";
    pub const NOTHING_LISTED_REST: &str = "Files are listed while the server is connected.";

    // **`File Browser` P2 and P4 for a folder whose reading did not come back
    // whole** — the machine did not answer, or the listing was cut. *The
    // machine did not answer* is not a statement about the folder (P2), so
    // this names what did not happen and nothing about why.
    pub const THE_FOLDER_WAS_NOT_READ: &str = "Could not read this folder.";

    // L8a's shape, for a read of the local record that did not answer.
    pub const THE_RECORD_WOULD_NOT_READ: &str = "Could not read Smoozy's database.";
    pub const NO_SUCH_SERVER: &str = "Server removed.";

    // ── The refusals ────────────────────────────────────────────────────────

    // `Design/File Browser § The list` — *"the row says the login cannot read
    // the folder, and where the machine allows one, the offer to try again with
    // admin access is beside it."* **Five rows and five sentences**, and the
    // three that differ do so only in what is beside them.
    pub const CANNOT_READ_THE_FOLDER: &str = "Permission denied: cannot read this folder.";
    pub const CANNOT_READ_AND_NO_ROUTE: &str =
        "Permission denied: cannot read this folder. No admin access on this server.";
    pub const CANNOT_DELETE: &str = "Permission denied: cannot delete this folder.";
    pub const CANNOT_DELETE_AND_NO_ROUTE: &str =
        "Permission denied: cannot delete this folder. No admin access on this server.";
    pub const CANNOT_MAKE_A_FOLDER: &str = "Permission denied: cannot make a folder here.";
    pub const CANNOT_MAKE_AND_NO_ROUTE: &str =
        "Permission denied: cannot make a folder here. No admin access on this server.";
    pub const CANNOT_RENAME: &str = "Permission denied: cannot rename this.";
    pub const CANNOT_RENAME_AND_NO_ROUTE: &str =
        "Permission denied: cannot rename this. No admin access on this server.";
    pub const CANNOT_MOVE: &str = "Permission denied: cannot move this.";
    pub const CANNOT_MOVE_AND_NO_ROUTE: &str =
        "Permission denied: cannot move this. No admin access on this server.";
    pub const CANNOT_COPY: &str = "Permission denied: cannot copy this.";
    pub const CANNOT_COPY_AND_NO_ROUTE: &str =
        "Permission denied: cannot copy this. No admin access on this server.";
    pub const TRY_WITH_ADMIN: &str = "Retry with admin access";
    pub const DISMISS: &str = "Dismiss";

    // `File Browser` P4's residue, and `File Browser` P3's two named components. **The
    // message says which of
    // the two it met and stops there.**
    pub const NOT_PERMITTED_AT_LEAD: &str = "Permission denied:";
    pub const IT_IS_NOT_THERE_AT_LEAD: &str = "Not found:";
    pub const COULD_NOT_SAY_WHERE_REFUSED: &str = "Permission denied.";
    pub const COULD_NOT_SAY_WHERE_GONE: &str = "Not found.";
    // `Design/File Browser § When another module moves the browser`, added
    // 2026-10-07: *show outputs* on a job's folder that is not there, for a
    // job whose record shows no frame saved. The Creator's fourth notes,
    // piece O.
    pub const IT_IS_NOT_THERE_YET_AT_LEAD: &str = "Does not exist yet:";
    pub const NO_FRAMES_HAVE_BEEN_SAVED: &str = ". No frames saved.";

    // ── Renaming ────────────────────────────────────────────────────────────

    // `Design/File Browser § Renaming into a name that is taken`.
    pub const THE_NAME_IS_TAKEN: &str = "Name already in use.";
    pub const THE_LISTING_SAW_THE_NAME: &str =
        "Name in use as of the last refresh.";

    // ── Acting on files: the words around an act ────────────────────────────
    //
    // `Design/File Browser § What the screen says around an act`, added
    // 2026-10-02, and `File Browser` M9's names refused before any attempt.
    // Step 10c's Files' owed acts piece 3.
    pub const A_NEW_FOLDERS_NAME: &str = "New folder";
    pub const TYPE_A_NAME: &str = "Type a name.";
    pub const A_NAME_HOLDS_NO_SLASH: &str = "Name cannot contain /.";
    pub const A_NAME_IS_NOT_A_DOT: &str = "Name cannot be . or ..";
    pub const THE_NAME_IS_TOO_LONG: &str = "Name too long.";
    pub const THE_NAME_IS_SMOOZYS_OWN: &str = "Name reserved by Smoozy.";
    pub const SELECT_TO_RENAME: &str = "Select a file or folder to rename.";
    pub const SELECT_TO_DELETE: &str = "Select a file or folder to delete.";
    pub const SELECT_TO_CUT: &str = "Select a file or folder to cut.";
    pub const SELECT_TO_COPY: &str = "Select a file or folder to copy.";
    pub const RENAMED_NOW_HIDDEN_LEAD: &str = "Renamed to";
    pub const RENAMED_NOW_HIDDEN_REST: &str =
        ". Hidden while Show hidden files is off.";
    pub const IT_IS_NOT_THERE_ANY_MORE: &str =
        "Not found. Folder refreshed.";
    pub const THE_SERVER_REFUSED_IT: &str = "The server refused it.";
    pub const THE_CONNECTION_WENT: &str =
        "Connection lost before the server replied.";
    pub const NOTHING_RENAMED_THE_TELLING: &str =
        "Could not save to Smoozy's database. Nothing was renamed.";
    pub const NOT_CONNECTED_FOR_AN_ACT: &str = "Server not connected.";
    // A delete, under way and ended (step 10c's Files' owed acts piece 4).
    pub const DELETING: &str = "Deleting";
    pub const OF: &str = "of";
    pub const DELETED_LEAD: &str = "Deleted";
    pub const DELETED_COUNT_REST: &str = "deleted,";
    pub const REFUSED_COUNT_REST: &str = "failed,";
    pub const NOT_TRIED_COUNT_REST: &str = "skipped.";
    pub const SOME_NEVER_ANSWERED: &str =
        "Connection lost before the server replied for some items.";
    // A paste after a cut (step 10c's Files' owed acts piece 5).
    pub const SELECT_TO_PASTE: &str = "Cut or copy a file or folder first.";
    pub const A_FOLDER_INTO_ITSELF: &str = "Cannot paste a folder into itself or its subfolders.";
    pub const NOTHING_WAS_MOVED_IT_IS_HERE: &str = "Already in this folder.";
    pub const MOVED_ITEMS_LEAD: &str = "Moved";
    pub const MOVED_COUNT_REST: &str = "moved,";
    pub const NOT_DONE_COUNT_REST: &str = "skipped.";
    pub const ON_ANOTHER_DISK_REST: &str = "on another disk were not moved.";
    // A paste after a copy (step 10c's Files' owed acts piece 6), and
    // `Design/File Browser § What the screen says around an act`'s two
    // sentences where the server cannot copy files on itself.
    pub const COPYING: &str = "Copying";
    pub const COPIED_LEAD: &str = "Copied";
    pub const COPIED_COUNT_REST: &str = "copied,";
    pub const SET_UP_TO_COPY: &str = "Set up Smoozy on this server to copy files.";
    pub const SET_UP_TO_MOVE_BETWEEN_DISKS: &str = "Set up Smoozy on this server to move files between disks.";
    // Design's *"Where Smoozy there is older than the app, Update replaces Set
    // up"* — step 10c's Files' owed acts piece 7, where the program on the
    // server refuses the copy request by name.
    pub const UPDATE_TO_COPY: &str = "Update Smoozy on this server to copy files.";
    // A folder's size, counted on request — `Design/File Browser`, the Size
    // paragraph and the folder menu's *Calculate size* (the Creator's
    // twenty-third notes, note 2).
    pub const CALCULATE_SIZE: &str = "Calculate size";
    pub const CALCULATING: &str = "Calculating…";

    // ── Looking at a picture ────────────────────────────────────────────────

    // `Design/File Browser § Looking at a picture`, added 2026-10-31 — the
    // Creator's seventeenth notes, note 11. The box's own name, for a screen
    // reader, is the drawing's (`#fbview`'s `aria-label`), and so are the
    // two arrows' and the close button's.
    pub const A_PICTURE: &str = "Picture";
    pub const LOADING: &str = "Loading…";
    pub const CANNOT_SHOW_LEAD: &str = "Can't preview";
    pub const CANNOT_SHOW_REST: &str = "files.";
    pub const TOO_LARGE_TO_SHOW: &str = "Too large to show (over 100\u{a0}MB).";
    pub const COULD_NOT_SHOW_THIS_FILE: &str = "Could not show this file.";
    pub const COULD_NOT_READ_THE_FILE: &str = "Could not read the file.";
    pub const THE_VIEWER_IS_NOT_CONNECTED: &str = "Not connected.";
    pub const RETRY: &str = "Retry";
    pub const CLOSE: &str = "Close";
    pub const PREVIOUS_PICTURE: &str = "Previous picture";
    pub const NEXT_PICTURE: &str = "Next picture";
    // The reason under *Could not read the file.* (`File Browser` L29: *"a
    // dropped line, a refusal, a cut"*), in the words this screen already
    // uses for each.
    pub const FILE_NOT_FOUND: &str = "File not found.";
    pub const AT_LEAST: &str = "at least";
    pub const FAILED: &str = "Failed";
    pub const COUNTED_ONE_FILE: &str = "file,";
    pub const COUNTED_FILES: &str = "files,";
    pub const COUNTED_ONE_FOLDER: &str = "folder";
    pub const COUNTED_FOLDERS: &str = "folders";
    pub const SOME_FOLDERS_COULD_NOT_BE_READ: &str = "Some folders could not be read.";
    pub const SET_UP_TO_CALCULATE: &str = "Set up Smoozy on this server to calculate folder sizes.";
    pub const UPDATE_TO_CALCULATE: &str = "Update Smoozy on this server to calculate folder sizes.";
    pub const NOT_A_FOLDER_ANY_MORE: &str = "No longer a folder.";
    pub const THE_FOLDER_COULD_NOT_BE_READ: &str = "Could not read this folder.";
    pub const NO_REPLY_TO_THE_COUNT: &str = "Timed out waiting for a reply.";
    pub const UPDATE_TO_MOVE_BETWEEN_DISKS: &str = "Update Smoozy on this server to move files between disks.";
    pub const NOTHING_COPIED_THE_RECORD: &str =
        "Could not save to Smoozy's database. Nothing was copied.";
    pub const NOTHING_MOVED_THE_RECORD: &str =
        "Could not save to Smoozy's database. Nothing was moved.";
    pub const NOTHING_DELETED_THE_RECORD: &str =
        "Could not save to Smoozy's database. Nothing was deleted.";

    // ── Uploading ───────────────────────────────────────────────────────────

    // `Design/File Browser § Uploading` — *"Where the system file picker cannot
    // take files and folders in one go, the same Upload control asks which kind
    // first."*
    pub const WHICH_KIND: &str = "Upload files or folders?";
    // **Worded for the computer it is said on** — `Design/File Browser §
    // Uploading`, the Creator's note 2 on the test copy. A Mac is not asked.
    #[cfg(target_os = "windows")]
    pub const THE_PICKER_CANNOT_TAKE_BOTH: &str =
        "The Windows file picker cannot select files and folders together.";
    #[cfg(not(target_os = "windows"))]
    pub const THE_PICKER_CANNOT_TAKE_BOTH: &str =
        "This computer's file picker cannot select files and folders together.";
    pub const FILES: &str = "Files";
    pub const FOLDERS: &str = "Folders";

    // The panel's own title, which is the picker's chrome and passes the point
    // like every other word (X5).
    pub const PICK_WHAT_TO_UPLOAD: &str = "Choose what to upload";

    // **There is no confirmation before an upload** — `Design/File Browser §
    // Uploading`, struck 2026-10-04 at the Creator's note 2 on the test copy:
    // a pick or a drop starts it. Its words went with it at piece 9.

    // The drop zone. **The count is a count of what was dropped**, `File Browser` H3a's own
    // bound: paths and nothing else are known at the drop.
    pub const DROP_TO_UPLOAD_INTO: &str = "Drop to upload into";
    pub const DROPPED_FILES: &str = "files";
    pub const DROPPED_ONE_FILE: &str = "file";
    pub const DROPPED_FOLDERS: &str = "folders";
    pub const DROPPED_ONE_FOLDER: &str = "folder";
    pub const AND: &str = "and";

    // ── The already-exists box ──────────────────────────────────────────────

    // `Design/File Browser § When the files are already there` — *"Asked once
    // for the whole upload, leading with how many."*
    pub const ALREADY_THERE_MIDDLE: &str = "of these";
    pub const ALREADY_THERE_UPLOAD: &str = "files are already in";
    pub const ALREADY_THERE_DOWNLOAD: &str = "files are already in";
    pub const ALREADY_THERE_MOVING_LEAD: &str = "of the";
    pub const ALREADY_THERE_MOVING: &str = "items you are moving is already in";
    pub const ALREADY_THERE_MOVING_MANY: &str = "items you are moving are already in";
    pub const ALREADY_THERE_COPYING: &str = "items you are copying is already in";
    pub const ALREADY_THERE_COPYING_MANY: &str = "items you are copying are already in";
    // **The count takes its singular** — step 10c's eleventh pass read *"1 of
    // these 1 files are already in /home/ubuntu"*. One colliding among several
    // is *1 of these 3 files is already in*; one alone is *1 file is already
    // in*, with no *of these 1*. The same words, in their singular (Rule 9,
    // *Short, too*). Added 2026-09-30, step 10c's eleventh-pass fix piece 7.
    pub const ALREADY_THERE_UPLOAD_ONE_OF_SEVERAL: &str = "files is already in";
    pub const ALREADY_THERE_ONE_FILE_ALONE: &str = "file is already in";
    pub const ALREADY_THERE_MOVING_ONE_ALONE: &str = "item you are moving is already in";
    pub const ALREADY_THERE_COPYING_ONE_ALONE: &str = "item you are copying is already in";
    // *"One line says what the comparison was, and it stops there."*
    pub const COMPARED_BY: &str = "Compared by name and location only.";
    pub const AND_MORE: &str = "more";

    // The four answers.
    pub const REPLACE: &str = "Replace";
    pub const KEEP_BOTH: &str = "Keep both";
    pub const SKIP_EXISTING: &str = "Skip existing";
    pub const STOP: &str = "Stop";

    // *"One line above them is a separate warning."* `File Browser` W1b's line, and the
    // offer
    // beside it.
    pub const PLACED_BY_THE_CHECK_REST: &str = "were uploaded by the dependency check.";
    pub const ONE_PLACED_BY_THE_CHECK_REST: &str = "was uploaded by the dependency check.";
    pub const PLACED_BY_THE_CHECK_SAID: &str =
        "Replacing them brings back paths the render cannot find.";
    pub const CHECK_THEM_INSTEAD: &str = "Check their dependencies instead";

    // `File Browser` U9b's reason line. *"The moment named is when a file last finished
    // uploading, not when the upload stopped."*
    pub const INTERRUPTED_LEAD: &str = "Upload interrupted — last file uploaded on";
    pub const INTERRUPTED_REST: &str = "Skip existing uploads only what is missing.";

    // ── The live-job warning, on all four acts ──────────────────────────────
    //
    // `Design/File Browser § The live-job warning, on all four acts`. **`File Browser`
    // W5a's
    // bound: uniform across files, differentiated across jobs**, and the
    // sentence is a *may* and never a description.
    pub const IS_RENDERING_WITH_THIS_FILE: &str = "is rendering with this file";
    pub const IS_RENDERING_WITH: &str = "is rendering with";
    pub const OF_THESE_FILES: &str = "of these files";
    pub const IS_WAITING_ON_IT: &str = "is waiting on it.";
    pub const IS_WAITING_ON_THEM: &str = "is waiting on them.";
    pub const IS_HELD_ON_IT: &str = "is held and waiting on it.";
    pub const AND_ALSO: &str = "and";
    pub const A_RENDER_MAY_FINISH_ANYWAY: &str =
        "A running render may finish with wrong frames.";
    // `File Browser` W6's widest case, said rather than implied. **The folder
    // is the job's output folder**, and the words say so: *its folder* read as
    // the `.blend`'s own folder (after 0.1.8, the checks' leftovers, item 2).
    pub const ONLY_THE_BLEND_AND_ITS_FOLDER: &str =
        "For that job, only its .blend and output folder were checked.";
    // The two acts that take the same box with a title of their own.
    pub const RENAME_QUESTION_LEAD: &str = "Rename";
    pub const MOVE_QUESTION_LEAD: &str = "Move";
    pub const MOVE_QUESTION_MIDDLE: &str = "items into";
    // **Where no destination is known yet the question ends without *into***,
    // so it is still a whole question — must-do 27 of the step 10a round 1
    // audit of these pieces.
    pub const MOVE_QUESTION_NO_DESTINATION: &str = "items?";
    // **One item is one item** — the pass before 0.1.9 on Vast read *Move 1
    // items*.
    pub const MOVE_QUESTION_MIDDLE_ONE: &str = "item into";
    pub const MOVE_QUESTION_NO_DESTINATION_ONE: &str = "item?";
    // **A copy takes the same box with its own act in the title** (`Design/File
    // Browser § The live-job warning, on all four acts`). It read *Rename* with
    // no name until the pass before 0.1.9's fixes.
    pub const COPY_QUESTION_LEAD: &str = "Copy";
    pub const THE_QUESTION_MARK: &str = "?";

    // ── Deleting ────────────────────────────────────────────────────────────

    // `Design/File Browser § Deleting`.
    pub const DELETE_QUESTION_LEAD: &str = "Delete";
    pub const DELETE_QUESTION_MANY_LEAD: &str = "Delete";
    pub const DELETE_QUESTION_MANY_REST: &str = "items?";
    pub const PERMANENT: &str = "This cannot be undone.";
    pub const DONT_WARN_AGAIN: &str = "Don't warn me about deleting again";
    // *"the checkbox sits under the permanence warning and says in one line
    // that it does not cover the other."*
    pub const WHAT_IT_DOES_NOT_COVER: &str = "The warning about jobs using the file always shows.";

    // `Design/File Browser § Disturbing a file nothing live is using`. *"It
    // says what the file is for rather than what the record is."*
    pub const OTHER_REFERENCES_REST: &str = "other references point at this file.";
    pub const ONE_OTHER_REFERENCE_REST: &str = "other reference points at this file.";
    pub const REFERENCES_SAID: &str =
        "Deleting it breaks them. Their files would need a new dependency check.";

    // `File Browser` M3's own extra true thing, where the victim holds live staged
    // material.
    pub const STAGED_MATERIAL_REST: &str =
        "partial transfers are stored here. Deleting restarts them from the beginning.";
    pub const ONE_STAGED_MATERIAL_REST: &str =
        "partial transfer is stored here. Deleting restarts it from the beginning.";

    // ── The right-click menus ───────────────────────────────────────────────

    // `Design/File Browser § Acting on files` — three menus, and *"Upload and
    // new folder are on no menu at all."* **A menu that opens carries no
    // caption**: the drawing's *Right-click a folder* labels are there to tell
    // its three side-by-side displays apart (must-do 20).
    pub const DOWNLOAD_TO: &str = "Download to…";
    pub const PASTE_INTO_THIS_FOLDER: &str = "into this folder";

    // ── The bottom bar ──────────────────────────────────────────────────────

    // `Design/File Browser § The list` — *"a standing line saying where an
    // upload will land … and a count of what is in the folder"*, with the free
    // disk beside them.
    pub const UPLOADS_GO_TO: &str = "Uploads go to";
    pub const FILES_IN_THE_FOLDER: &str = "files,";
    pub const ONE_FILE_IN_THE_FOLDER: &str = "file,";
    pub const FOLDERS_IN_THE_FOLDER: &str = "folders";
    pub const ONE_FOLDER_IN_THE_FOLDER: &str = "folder";
    pub const FREE_ON_THIS_SERVER: &str = "free on this server";
    pub const FREE_LAST_SEEN: &str = "free on this server when last checked";
    pub const NO_FREE_SPACE_READING: &str = "Free space unknown";
    // `Specs/File Browser § Browsing a server` and the drawing's own checkbox
    // in the bottom bar — Creator-stated 2026-10-01, Row 520.
    pub const SHOW_HIDDEN_FILES: &str = "Show hidden files";

    // ── The path ────────────────────────────────────────────────────────────

    /// The root of the machine, which has no name of its own to show.
    pub const THE_TOP_OF_THE_SERVER: &str = "Root";

    // ── The one honest answer where an act needs the server instructed ──────
    //
    // **It is not a failure record and it is not a stand-in for one**: it says
    // what is true. Piece 3's shape, and its reasoning carries here unchanged.
    //
    // **From step 10c piece 4 it is the All servers screen's own sentence, and
    // not the five-screen one.** Files now sends the server instructions — a
    // listing and an upload — so *"Sending instructions to the server is not
    // built"* would be false here. The acts still owed on this screen each wait
    // on a different piece, which is exactly the case the build runner's ruling
    // of 2026-09-18 gave the All servers screen its own sentence for; **Files
    // reads that one rather than writing a third.**
    pub const NOT_BUILT_YET: &str = crate::surface::servers::words::NOT_BUILT_YET;

    // ── An upload, going and gone ───────────────────────────────────────────
    //
    // **`Design/File Browser` draws no line for these**: it says transfers are
    // watched in Transfers, and that screen reads no running request yet (step
    // 10c piece 10's). **So the answer is said where the user asked, on the
    // line above the list** — the shape piece 3 gave an answer on the server
    // list — rather than nowhere, which Rule 8 makes a fault. See the account.
    //
    // **Since the Creator's notes piece 6 an upload says nothing here while it
    // runs** — `Design/File Browser § The list`: the bottom bar's transfer
    // count says it is running, and this line says only how it ended.
    pub const UPLOADED: &str = "Uploaded";
    // **The bottom bar's transfer count** — `Design/File Browser § The list`:
    // *3 transfers*, a link to Transfers. The Creator's notes piece 6.
    pub const TRANSFERS_ON_THIS_SERVER: &str = "transfers";
    pub const ONE_TRANSFER_ON_THIS_SERVER: &str = "transfer";
    pub const FILES_INTO: &str = "files into";
    pub const ONE_FILE_INTO: &str = "file into";
    pub const FULL_STOP: &str = ".";
    pub const WERE_SKIPPED: &str = "files already there were skipped.";
    pub const WAS_SKIPPED: &str = "file already there was skipped.";
    pub const COULD_NOT_BE_UPLOADED: &str = "files failed to upload:";
    pub const ONE_COULD_NOT_BE_UPLOADED: &str = "file failed to upload:";
    pub const A_COMMA: &str = ",";
    pub const AND_MORE_FILES: &str = "and more.";
    pub const DID_NOT_FINISH: &str = "files did not finish uploading.";
    pub const ONE_DID_NOT_FINISH: &str = "file did not finish uploading.";
    /// Beside a refused write: where the refused file is drawn, with what the
    /// server said.
    pub const SEE_TRANSFERS: &str = "Details in Transfers.";

    // Why an upload did not start. **Each names what did not happen**, and the
    // one that names a folder names it as a value.
    pub const THE_UPLOAD_DID_NOT_START: &str = "Upload not started.";
    pub const NOT_CONNECTED: &str = "Upload not started: server not connected.";
    pub const NOTHING_TO_UPLOAD: &str = "Nothing to upload: the chosen folders are empty.";
    pub const COULD_NOT_READ_ON_THIS_COMPUTER: &str =
        "Upload not started. Could not read on your computer:";
    pub const THE_RECORD_WOULD_NOT_TAKE_IT: &str =
        "Upload not started: could not save it to Smoozy's database.";
    // `File Browser` W8: the telling could not be written, so the act is not
    // attempted, naming the telling. Step 10c's Files' owed acts piece 2.
    pub const THE_TELLING_WOULD_NOT_WRITE: &str =
        "Could not save to Smoozy's database. Nothing was replaced.";
    // *Upload it again* or *Try again* from Transfers, where a file now stands
    // at the name and the first request asked nothing (the 38's piece 9).
    pub const A_FILE_STANDS_THERE_NOW: &str =
        "Not uploaded: a file with this name now exists on the server. Upload it from Files.";

    // A switch the user ticked that the record would not take — must-do 24 of
    // the step 10a round 1 audit of these pieces. The words were the ones
    // `src/surface/settings.rs` said for a setting that did not save; step 13's
    // words sweep reworded this copy as its scout proposed for all six copies,
    // and the other five are their own screens' to change.
    pub const THE_SETTING_DID_NOT_SAVE: &str = "Could not save the setting.";

    // `File Browser` X12 as amended 2026-10-02: with *Show hidden files* on,
    // the staging folder *"lists as a reserved Smoozy staging folder, not as an
    // ordinary user folder"*, and the spec's *"listed and marked as Smoozy's"*.
    // It is the row's kind word, where *Folder* would otherwise stand.
    pub const A_STAGING_FOLDER: &str = "Smoozy staging folder";
}

// ════════════════════════════════════════════════════════════════════════════
// What the page is given
// ════════════════════════════════════════════════════════════════════════════

/// **A sentence, in the pieces it is made of — words and values side by side,
/// and never one inside the other** (X7).
///
/// **It is a sequence rather than a lead, a list of values and a rest**, which
/// is what the drawing's own sentences need: *42 of these 300 files are already
/// in kitchen/* puts words **between** two figures and again before a path, and
/// a shape with one set of words at each end cannot say it. **The first shape
/// this piece wrote was that one, and the box came out reading `42 300
/// kitchen/`** — three values and not one word — which is what a person looking
/// at it found in a minute.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct Words {
    pub pieces: Vec<APieceOfIt>,
}

/// One piece of a sentence: **words, or a value, and never both.**
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct APieceOfIt {
    /// Prose, through the typography point (X1).
    pub said: Option<Prose>,
    /// A value, by the verbatim route (X9).
    pub value: Option<Verbatim>,
    /// **Whether it follows the piece before it with nothing between them.**
    /// The drawing writes *Delete kitchen_v14.blend?* with the mark against the
    /// name, and whether a sentence has a space there is the sentence's own
    /// fact — a page deciding it would be a page reading prose.
    pub joins: bool,
    /// **Whether it follows the piece before it with a non-breaking space**
    /// rather than an ordinary one: words after a figure — *5 files*,
    /// *61 GB free on this server*, *about 6 minutes left* — so the figure and
    /// what it counts never part across two lines (`Design/Whole app §
    /// Typography`). Set by [`Words::then`] alone, where it sees the figure,
    /// so a sentence composed later is bound without anybody remembering to.
    pub binds: bool,
    /// **The value is a moment — seconds since the epoch — and the page renders
    /// it in the user's own zone** (`File Browser` L4). Without this mark the
    /// page drew the stale bar's moment as the bare figure.
    pub moment: bool,
    /// A path worth copying, distinguished from names and figures before the
    /// page draws it (Design/Whole app, Selecting text).
    #[serde(skip_serializing_if = "the_piece_is_not_a_path")]
    pub is_path: bool,
}

fn the_piece_is_not_a_path(is_path: &bool) -> bool {
    !is_path
}

impl Words {
    fn empty() -> Words {
        Words { pieces: Vec::new() }
    }

    /// Words alone.
    pub fn plain(said: &str) -> Words {
        Words::empty().then(said)
    }

    /// Words, then a value.
    pub fn with(said: &str, value: impl Into<String>) -> Words {
        Words::plain(said).and(value)
    }

    /// A value, then the words about it — *5 files*, *61 GB free on this
    /// server*. **The two are still two pieces**; which comes first is a fact
    /// about the sentence and not a licence to join them.
    pub fn after(value: impl Into<String>, said: &str) -> Words {
        Words::empty().and(value).then(said)
    }

    /// Words, a value, then more words.
    pub fn around(lead: &str, value: impl Into<String>, rest: &str) -> Words {
        Words::with(lead, value).then(rest)
    }

    /// More words on the end. **Words after a figure are bound to it** —
    /// `Design/Whole app § Typography`; see [`APieceOfIt::binds`].
    pub fn then(mut self, said: &str) -> Words {
        if said.is_empty() {
            return self;
        }
        let binds = self.pieces.last().is_some_and(|last| {
            !last.moment
                && last
                    .value
                    .as_ref()
                    .is_some_and(|value| crate::typography::is_a_figure(value.as_str()))
        });
        self.pieces.push(APieceOfIt {
            said: Some(Prose::new(said)),
            value: None,
            joins: false,
            binds,
            moment: false,
            is_path: false,
        });
        self
    }

    /// Another value on the end.
    pub fn and(mut self, value: impl Into<String>) -> Words {
        self.pieces.push(APieceOfIt {
            said: None,
            value: Some(Verbatim::new(value.into())),
            joins: false,
            binds: false,
            moment: false,
            is_path: false,
        });
        self
    }

    /// A path on the end. Only a surface that offers copying uses the mark;
    /// clickable rows and path parts still draw their values unselectable.
    pub fn and_a_path(self, path: impl Into<String>) -> Words {
        let mut words = self.and(path);
        if let Some(last) = words.pieces.last_mut() {
            last.is_path = true;
        }
        words
    }

    /// **A moment on the end**, as seconds since the epoch, for the page to
    /// render. Nothing here subtracts it from anything (`State, identity and
    /// recovery` D5).
    pub fn and_a_moment(mut self, seconds: i64) -> Words {
        self.pieces.push(APieceOfIt {
            said: None,
            value: Some(Verbatim::new(seconds.to_string())),
            joins: false,
            binds: false,
            moment: true,
            is_path: false,
        });
        self
    }

    /// **The last piece follows the one before it with nothing between them.**
    pub fn joined(mut self) -> Words {
        if let Some(last) = self.pieces.last_mut() {
            last.joins = true;
        }
        self
    }

    /// Every value in the sentence, in order. **For a test and for nothing
    /// else** — the page draws the pieces.
    #[cfg(test)]
    pub fn values(&self) -> Vec<&str> {
        self.pieces
            .iter()
            .filter_map(|piece| piece.value.as_ref().map(|value| value.as_str()))
            .collect()
    }

    /// Every word in the sentence, in order, joined with a space. **For a test
    /// and for nothing else.**
    #[cfg(test)]
    pub fn said(&self) -> String {
        self.pieces
            .iter()
            .filter_map(|piece| piece.said.as_ref().map(|said| said.as_str()))
            .collect::<Vec<_>>()
            .join(" ")
    }
}

/// **A path or a name, in both the forms the page needs** — `File Browser` L1a.
///
/// *"A name that comes back from the machine is carried as bytes and decoded
/// only for display, and a listing that decodes eagerly is forbidden … the
/// enumeration holds every name as the bytes the subsystem returned, every
/// comparison and every path built from it uses those bytes, and a decode
/// happens only at the point a name is shown — where an undecodable byte is
/// replaced for display and never for identity."*
///
/// **So a path crosses to the page twice**: the bytes, which the page hands
/// back untouched and which every comparison on this side is made on, and the
/// shown form, which is what a person reads. **A page that composed a path out
/// of the shown form would be composing an identity out of a display**, and on
/// a name holding a byte no encoding carries it would compose the wrong one.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct APath {
    /// The bytes, exactly as the machine gave them. **Identity.**
    pub bytes: Vec<u8>,
    /// The display form, decoded with an undecodable byte replaced.
    /// **A value, by the verbatim route** (X9) — never prose.
    pub shown: Verbatim,
}

impl APath {
    pub fn of(bytes: &[u8]) -> APath {
        APath {
            bytes: bytes.to_vec(),
            shown: Verbatim::new(String::from_utf8_lossy(bytes).into_owned()),
        }
    }
}

/// One part of the path along the top, and every part of it is clickable.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct APathPart {
    /// The whole path this part stands for, which is what a press goes to.
    pub folder: APath,
    /// What is drawn — the component's own name, or the top of the server.
    pub shown: Verbatim,
    /// Prose, for the root, which has no name of its own.
    pub said: Option<Prose>,
    /// The last part, which is where the user is standing and is not a link.
    pub here: bool,
}

/// **Which icon a row draws.** The name of a shape and not a word a person
/// reads, so it is neither prose nor a value.
///
/// `Design/File Browser § The list`: *"Five icons, and the ones that matter are
/// not pages."* **A folder is solid where a file is outlined**, which is what
/// separates the two before either shape is read.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum AnIcon {
    Folder,
    Blend,
    Image,
    Archive,
    File,
}

/// **What one row of the list is.**
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct ARow {
    pub name: APath,
    /// **The row's whole path on the machine**, composed by `File Browser`
    /// N2's own [`landing_path`] from the folder and the name. **It is what
    /// the render settings panel is opened on**, because the page composes no
    /// path — the decisions record's ruling 4 of 2026-09-13.
    pub path: APath,
    pub icon: AnIcon,
    /// The kind, as a word. Prose.
    pub kind: Prose,
    /// The size, or nothing where the row has none. **A value.**
    pub size: Option<Verbatim>,
    /// When it last changed, rendered from the epoch value in the user's own
    /// zone (`File Browser` L4). **A value**, and no elapsed time is ever derived from it.
    pub changed: Option<Verbatim>,
    /// **The Blender version the file was saved with** (`File Browser` L7–`File Browser`
    /// L10), as a small tag
    /// on the name rather than a column. **Blank is never a guess** (`File Browser` L8),
    /// and a
    /// blank tag is this being `None`.
    pub saved_with: Option<Verbatim>,
    /// **The tag is still to be read** — a `.blend` this connected period has
    /// no reading for at the listing's own size and time (`File Browser` L10).
    /// The page asks [`the_saved_with_version_is_read`] for it after the rows
    /// are drawn, so the four fields show first and the tag fills in behind
    /// them (`File Browser` L7). `false` once a reading is held, blank or not.
    pub the_tag_follows: bool,
    /// **`File Browser` N7a: the toolbar's rule is the file's kind and only its kind.**
    pub is_a_blend: bool,
    /// Whether the row may be opened — a folder, and nothing else.
    pub opens: bool,
    /// **`File Browser` L19: the name's first byte is a period**, read off the
    /// bytes and never off the shown form. **The row is carried whatever the
    /// choice**, so ticking or unticking *Show hidden files* re-composes from
    /// the listing the page already holds and never reads the machine
    /// (`File Browser` L21); the page leaves a hidden row out while the box is
    /// unticked — **unless a refusal arrived on it**, which is never left out
    /// (`File Browser` L21 as amended 2026-10-02).
    pub hidden: bool,
    /// **The refusal that arrived on this row**, where one did.
    pub refusal: Option<ARefusal>,
    /// **The row in editing**, where it is: a rename being typed, or a new
    /// folder that was refused.
    pub editing: Option<ARowInEditing>,
    /// **What the row is sorted by**, handed back by the page when a header
    /// is pressed so the rows held are reordered without a reading (the
    /// Creator's twenty-first notes, note 2).
    pub sorts_by: WhatTheRowSortsBy,
    /// **A picture's row opens the viewer** — `File Browser` L29 and
    /// `Design/File Browser § Looking at a picture`, added 2026-10-31. Nothing
    /// on a row that is not a picture.
    pub picture: Option<APicture>,
}

/// **What a picture row's viewer shows** — `File Browser` L29: the ending
/// offers the act, and the listed size is checked before anything is read.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
#[serde(tag = "is", rename_all = "kebab-case")]
pub enum APicture {
    /// **A PNG or JPEG under the cap**: the viewer reads it
    /// ([`a_picture_is_read`]), and decoding decides.
    ToBeRead,
    /// **Not read at all**: the line that stands where the picture would —
    /// the format Smoozy does not show, or the cap.
    NotShown { said: Words },
}

/// **The viewer's own words**, which do not change with what is shown.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheViewersWords {
    pub picture: Prose,
    pub loading: Words,
    pub could_not_show: Words,
    pub not_connected: Words,
    pub download: Prose,
    pub close: Prose,
    pub previous: Prose,
    pub next: Prose,
}

/// **Why the viewer shows no picture**, answered by [`a_picture_is_read`]
/// where it has no bytes to hand over.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
#[serde(tag = "is", rename_all = "kebab-case")]
pub enum ThePictureIsNotShown {
    /// The line where the picture would be, and nothing else.
    NotShown { said: Words },
    /// **A read that failed**: *Could not read the file.*, the reason under
    /// it, and *Retry*. Nothing retries by itself.
    NotRead { said: Words, why: Words, retry: Prose },
    /// **The viewer was closed or stepped on** before the read ended; the page
    /// has already moved on and draws nothing from it.
    LetGo,
}

/// **A refusal, told where it happened** — `Design/File Browser § The list`.
///
/// *"The note and the offer beside it are the failure, and they take the
/// whole-app failure shape: the red dot, one plain sentence saying the login
/// cannot read the folder, and Retry with admin access as the act that
/// would try again."* (Design's words since step 13's final words sweep.)
///
/// **It is not a failure record and it is not drawn through
/// [`crate::presentation::the_failure_shape`].** That builder takes a
/// [`crate::record::failure_records::Standing`] — a durable record with a
/// subject, a mark, a moment and a diary capture — and a permission refusal is
/// none of those: nothing is written anywhere, `File Browser` raises no failure
/// record for one, and §8.7's durable subjects hold nothing a machine path
/// could be. **The shape it takes is the drawing's own `.deny`**, whose own
/// comment in `the-drawing.css` says exactly that: *"The note beside it is the
/// failure, and it takes the whole-app shape: dot, plain sentence, then the way
/// out."* See the account for the whole of this ruling.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct ARefusal {
    /// The plain sentence saying what the login cannot do.
    pub said: Words,
    /// **The offer, where the machine allows one** (`File Browser` P5). Four outcomes, and
    /// three of them offer nothing.
    pub the_offer: Option<Prose>,
    /// The cross that clears it.
    pub dismiss: Prose,
    /// **The greying comes with the refusal rather than standing before it**,
    /// and says the folder cannot be opened and nothing more.
    pub the_row_is_dimmed: bool,
}

/// A row in editing — a rename being typed, or a folder that was not made.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct ARowInEditing {
    /// What the user typed. **A value.**
    pub typed: Verbatim,
    /// **The machine's own refusal** (`File Browser` M4). Red, and it stops the rename.
    pub refused: Option<Words>,
    /// **A warning that came earlier, from the listing.** *"It is a hint about
    /// what Smoozy last saw; the refusal above is the machine's answer, and
    /// only the second one stops anything."*
    pub the_listing_saw_it: Option<Words>,
}

/// **The list, or what stands where one would.**
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
#[serde(tag = "is", rename_all = "kebab-case")]
pub enum TheListing {
    /// The rows, with the moment the listing was taken.
    Rows {
        rows: Vec<ARow>,
        /// **`File Browser` L14's own moment**, carried so the stale bar can say when
        /// Smoozy
        /// last looked. Seconds since the epoch, on the machine's clock.
        taken_at: Option<i64>,
        /// **`File Browser` P8: what an elevated act shows is what elevation could
    /// see**, and
        /// it is not remembered as the machine's ordinary truth.
        read_elevated: bool,
    },
    /// A folder being read. *"Nothing in the list can be used until the answer
    /// arrives, and nothing outside the list is covered."*
    BeingRead { said: Words },
    /// **`File Browser` L14: a server this run never held a period for shows no listing,
    /// because no entry was ever made.**
    NothingHasBeenListed { lead: Prose, rest: Prose },
    /// **L8a: a read that did not answer says so.**
    TheReadDidNotAnswer { said: Prose },
    /// **A folder the machine did not list** — `File Browser` P1's separation,
    /// said in as many words (`File Browser` P2), with the component `File
    /// Browser` P3's walk named where it could name one. **It is not drawn as
    /// an empty list**, which would say the folder is empty.
    TheFolderWasNotRead { said: Words },
    /// The server is gone from the record.
    NoSuchServer { said: Prose },
}

/// The toolbar, composed for what is chosen.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheToolbar {
    /// **`File Browser` N7a: both controls for a `.blend` and neither for anything else** —
    /// *"a control that cannot apply to what is selected is not a control, it
    /// is a question"*. Taken from
    /// [`smoozy_file_browser::uploading::the_toolbar_for`] and not decided
    /// here.
    pub the_blend_pair: bool,
    pub render_settings: Prose,
    pub check_dependencies: Prose,
    pub upload: Prose,
    pub download: Prose,
    pub new_folder: Prose,
    pub rename: Prose,
    pub cut: Prose,
    pub copy: Prose,
    /// **Paste carries a count of what is on it** (Design, Creator-stated), and
    /// **it is offered only where the pasteboard's machine is the one being
    /// browsed** (`File Browser` M15) — *"nowhere else is it greyed, refused or explained;
    /// it
    /// is not offered at all"*. `None` is that.
    pub paste: Option<Words>,
    pub delete: Prose,
    /// *"How many files are chosen is said at the far end of the toolbar."*
    pub how_many_chosen: Option<Words>,
    /// **The same words for one file chosen, carried whether or not one is.**
    /// Added 2026-09-28, step 10c's transfer wave piece 1: **a click on a
    /// plain file is drawn from the listing the page already holds** and asks
    /// the application nothing, so the note it then shows has to have come
    /// with the answer. It is [`TheToolbar::how_many_chosen`]'s own
    /// composition, and the page writes no word of it.
    pub one_chosen: Words,
    /// **The words the page says around an act on a row** — `Design/File
    /// Browser § What the screen says around an act`. Carried with the
    /// toolbar because they are said where its controls are pressed, and the
    /// page writes no word of them. Step 10c's Files' owed acts piece 3.
    pub around_an_act: TheWordsAroundAnAct,
}

/// **`Design/File Browser § What the screen says around an act`**, as the page
/// needs them before it asks the application anything.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheWordsAroundAnAct {
    /// *A new folder's row starts named* New folder*, selected.*
    /// **A value and not prose**: it is the name the row is typed over, so
    /// no typography may touch it.
    pub a_new_folders_name: Verbatim,
    /// *Where the list already shows that name, the row says so as the user
    /// types, the same hint a rename gets* (`File Browser` M4's advisory
    /// listing).
    pub the_listing_saw_it: Words,
    /// *A control pressed with nothing chosen says so in one line.*
    pub select_to_rename: Words,
    pub select_to_delete: Words,
    pub select_to_cut: Words,
    pub select_to_copy: Words,
}

/// The words, composed once.
pub fn the_words_around_an_act() -> TheWordsAroundAnAct {
    TheWordsAroundAnAct {
        a_new_folders_name: Verbatim::new(words::A_NEW_FOLDERS_NAME.to_string()),
        the_listing_saw_it: Words::plain(words::THE_LISTING_SAW_THE_NAME),
        select_to_rename: Words::plain(words::SELECT_TO_RENAME),
        select_to_delete: Words::plain(words::SELECT_TO_DELETE),
        select_to_cut: Words::plain(words::SELECT_TO_CUT),
        select_to_copy: Words::plain(words::SELECT_TO_COPY),
    }
}

/// The bar along the bottom.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheBottomBar {
    /// *"a standing line saying where an upload will land"*.
    pub uploads_go_to: Words,
    /// *"a count of what is in the folder"*, where there is a listing —
    /// **of what the list shows** (`File Browser` L21, `Design/File Browser §
    /// The list`), so with the hidden names left out while the box is
    /// unticked.
    pub what_is_in_the_folder: Option<Words>,
    /// **The same count with the box turned the other way**, so a tick or an
    /// untick swaps the two and asks nothing (`File Browser` L21: changing the
    /// choice re-composes from what is held and never reads the machine).
    pub what_is_in_the_folder_when_the_box_is_turned: Option<Words>,
    /// **The *Show hidden files* checkbox**, beside the count.
    pub show_hidden_files: TheHiddenFilesBox,
    /// **`File Browser` L11's per-path figure, and `File Browser` L13's stale reading
    /// beside it.**
    pub free_space: Option<Words>,
}

/// ***Show hidden files*** — `File Browser` §1.6, Row 520. **A checkbox in the
/// bottom bar, beside the count, unticked until the user ticks it**
/// (`Design/File Browser § The list`).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheHiddenFilesBox {
    pub label: Prose,
    /// **The user's choice as the record holds it** (`File Browser` L20) —
    /// absent is off.
    pub ticked: bool,
}

/// The bar another screen puts across the top of this one.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
#[serde(tag = "is", rename_all = "kebab-case")]
pub enum ABarAcrossTheTop {
    /// **§12's entry point from `Rendering`** — the browser opened so the user
    /// can point at a file. *"What marks it is a bar along the top, not a
    /// different screen."*
    Pick { said: Words, way_out: Prose },
    /// **§12's entry point from `Queue & History`** — *show outputs* on a job
    /// that ran elsewhere.
    Moved {
        said: Words,
        /// The second sentence, **drawn only where an upload is actually in
        /// flight** — on a quiet browser it would answer a question nobody
        /// asked.
        still_uploading: Option<Prose>,
    },
    /// **`Rendering` A1a** — a submit-again whose own machine is gone. Its
    /// state is `res-find`, which is the Render settings group's.
    Find { said: Words, way_out: Prose },
}

/// **`File Browser` L15's stale bar** — the listing shown with its moment, stale said
/// plainly.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheStaleBar {
    pub said: Words,
}

/// One item of a right-click menu.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct AMenuItem {
    pub label: Words,
    /// The token the page hands back. A value, never prose.
    pub act: Verbatim,
    /// Whether a divider goes above this item.
    pub after_a_divider: bool,
    /// The two at the head of the `.blend` menu, drawn heavier.
    pub at_the_head: bool,
    /// Delete, which is the one in red.
    pub in_red: bool,
    /// **Greyed, with why on hover**, where the act cannot be offered here —
    /// *Calculate size* on a server whose Smoozy cannot count (`File Browser`
    /// L28). Where there is no sentence to give, greyed with nothing on
    /// hover. Added 2026-10-30, the twenty-third notes, note 2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub greyed: Option<AGreying>,
}

/// **Why a menu item is greyed**, where it is.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct AGreying {
    pub why: Option<Words>,
}

/// One of the three right-click menus.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct AMenu {
    /// Which of the three. A token, not a word.
    pub which: &'static str,
    pub items: Vec<AMenuItem>,
}

/// **Everything the page needs to draw the browser, in one answer.**
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheBrowser {
    /// The server this reading is about. A value.
    pub server: Verbatim,
    /// The folder being browsed.
    pub folder: APath,
    /// **The folder above, where there is one.** *Up one folder* on the Go
    /// menu and Backspace both go here, and **the page composes no path of its
    /// own** — a path is bytes and the arithmetic is the application's.
    pub the_folder_above: Option<APath>,
    /// The path along the top, part by part.
    pub path: Vec<APathPart>,
    /// The bar another screen put on this one, where one is there.
    pub bar: Option<ABarAcrossTheTop>,
    /// **`File Browser` L15's stale bar**, where the line is down and there is a listing.
    pub stale: Option<TheStaleBar>,
    pub toolbar: TheToolbar,
    pub listing: TheListing,
    pub bottom: TheBottomBar,
    /// The three right-click menus.
    pub menus: Vec<AMenu>,
    /// The four column headers, which sort.
    pub headers: TheHeaders,
    /// **`File Browser` L15, per act: unavailable rather than failing on a click.** The
    /// tokens
    /// of the acts that are unavailable, taken from
    /// [`smoozy_file_browser::cache::availability`] and not decided here.
    pub unavailable: Vec<&'static str>,
    /// **The folder is being read and the reading follows** — the page draws
    /// this answer's *being read* at once and asks [`the_folder_is_read`] for
    /// the rows, so the drawing thread never waits on a machine (P5).
    pub a_reading_follows: bool,
    /// **What a folder's Size cell says while its count is out** — the
    /// twenty-third notes, note 2.
    pub calculating: Words,
    /// **The picture viewer's words** — `Design/File Browser § Looking at a
    /// picture`, added 2026-10-31.
    pub viewer: TheViewersWords,
}

/// The four sortable column headers.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheHeaders {
    pub name: Prose,
    pub kind: Prose,
    pub size: Prose,
    pub changed: Prose,
    /// **The header the list is sorted by**, which the page marks.
    /// `Design/File Browser § The list`: *"The headers sort."*
    pub sorted_by: TheColumn,
    /// **Whether a second press reversed it** — the mark points down.
    pub reversed: bool,
}

/// **One of the four columns the list sorts by.** The name of a column and not
/// a word a person reads.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TheColumn {
    /// **Hole 8 (the 38, piece 13): by name ascending by default, as before.**
    #[default]
    Name,
    Kind,
    Size,
    Changed,
}

/// **How the list is sorted** — a column and a direction.
///
/// **Hole 8 of the 38 promises no step owned, ruled 2026-10-03** (`File
/// Browser` §1 hands *"how the list is laid out, sorted and shown"* to Design,
/// and `Design/File Browser § The list` says only *"The headers sort."*): by
/// name ascending by default; folders first; names compared as shown, ignoring
/// case, with a name that cannot be shown as text after those that can; kind
/// by extension; **the choice held for the run and not stored** — see
/// [`the_order_held`].
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct TheOrder {
    pub by: TheColumn,
    pub reversed: bool,
}

impl TheOrder {
    /// **A press on a header**: the header already sorted by reverses, and
    /// another one sorts by itself, from the start of its order.
    pub fn after_a_press_on(self, column: TheColumn) -> TheOrder {
        if column == self.by {
            TheOrder { by: column, reversed: !self.reversed }
        } else {
            TheOrder { by: column, reversed: false }
        }
    }

    /// **Where one entry falls against another.**
    ///
    /// **Folders come first, whichever way the list is sorted**, and a press
    /// reverses the order inside each of the two groups. Under *Size* the
    /// folders, which show no size, keep their order by name; under *Kind* they
    /// all say *Folder* and do the same. **Every column falls back on the name**,
    /// so two rows that tie are never left in the machine's order.
    pub fn of(self, one: &ListedEntry, other: &ListedEntry) -> std::cmp::Ordering {
        self.between(TheSortFacts::of_an_entry(one), TheSortFacts::of_an_entry(other))
    }

    /// **Where one row the page holds falls against another** — the same
    /// comparison as [`TheOrder::of`], over what each row carries
    /// ([`WhatTheRowSortsBy`]), so a header press reorders the rows held
    /// exactly as a fresh reading would have (the Creator's twenty-first notes,
    /// note 2).
    pub fn of_rows(self, one: &WhatTheRowSortsBy, other: &WhatTheRowSortsBy) -> std::cmp::Ordering {
        self.between(TheSortFacts::of_a_row(one), TheSortFacts::of_a_row(other))
    }

    fn between(self, one: TheSortFacts<'_>, other: TheSortFacts<'_>) -> std::cmp::Ordering {
        let both_folders = one.folder && other.folder;
        other.folder.cmp(&one.folder).then_with(|| {
            let by = match self.by {
                TheColumn::Name => std::cmp::Ordering::Equal,
                TheColumn::Kind if both_folders => std::cmp::Ordering::Equal,
                TheColumn::Kind => the_extension_of(one.name).cmp(&the_extension_of(other.name)),
                TheColumn::Size if both_folders => std::cmp::Ordering::Equal,
                TheColumn::Size => one.size.cmp(&other.size),
                // A row the machine gave no time for comes after those it did.
                TheColumn::Changed => match (one.changed, other.changed) {
                    (Some(one), Some(other)) => one.cmp(&other),
                    (Some(_), None) => std::cmp::Ordering::Less,
                    (None, Some(_)) => std::cmp::Ordering::Greater,
                    (None, None) => std::cmp::Ordering::Equal,
                },
            };
            let by = by.then_with(|| by_the_name_as_shown(one.name, other.name));
            if self.reversed {
                by.reverse()
            } else {
                by
            }
        })
    }
}

/// **What a row is sorted by**, carried on the row as this application
/// composed it — the name's bytes, whether it is a folder, the apparent size
/// (`File Browser` L5) and the last-changed moment in the epoch form (L4).
/// **The page hands it back untouched** when a header is pressed, and the
/// rows it holds are put in order here ([`the_rows_are_put_in_order`]), so
/// the folder is never read again for a sort (`Design/File Browser § The
/// list`, the Creator's twenty-first notes, note 2). The page compares
/// nothing itself.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct WhatTheRowSortsBy {
    pub name: Vec<u8>,
    pub folder: bool,
    pub size: u64,
    pub changed: Option<i64>,
}

impl WhatTheRowSortsBy {
    pub fn of(entry: &ListedEntry) -> WhatTheRowSortsBy {
        let facts = TheSortFacts::of_an_entry(entry);
        WhatTheRowSortsBy {
            name: facts.name.to_vec(),
            folder: facts.folder,
            size: facts.size,
            changed: facts.changed,
        }
    }
}

/// The four things [`TheOrder`] compares, borrowed from a listed entry or
/// from a row the page handed back, so both are put in order by one rule.
#[derive(Clone, Copy)]
struct TheSortFacts<'a> {
    name: &'a [u8],
    folder: bool,
    size: u64,
    changed: Option<i64>,
}

impl<'a> TheSortFacts<'a> {
    fn of_an_entry(entry: &'a ListedEntry) -> TheSortFacts<'a> {
        TheSortFacts {
            name: entry.name.as_bytes(),
            folder: entry.kind == Kind::Folder,
            size: entry.size.bytes(),
            changed: entry.last_changed.map(|at| at.epoch_seconds()),
        }
    }

    fn of_a_row(row: &'a WhatTheRowSortsBy) -> TheSortFacts<'a> {
        TheSortFacts { name: &row.name, folder: row.folder, size: row.size, changed: row.changed }
    }
}

/// **Names compared as shown, ignoring case** (hole 8). A name that is not
/// text — `File Browser` X5's raw bytes, shown with a replaced character —
/// **comes after every name that is**, and those are put in order by their
/// bytes. Two names the same but for case fall back on the exact text, so the
/// order never depends on what the machine listed first.
fn by_the_name_as_shown(one: &[u8], other: &[u8]) -> std::cmp::Ordering {
    match (std::str::from_utf8(one).ok(), std::str::from_utf8(other).ok()) {
        (Some(one), Some(other)) => one.to_lowercase().cmp(&other.to_lowercase()).then_with(|| one.cmp(other)),
        (Some(_), None) => std::cmp::Ordering::Less,
        (None, Some(_)) => std::cmp::Ordering::Greater,
        (None, None) => one.cmp(other),
    }
}

/// **Kind by extension** (hole 8): what follows the name's last full stop,
/// ignoring case. **A name with none sorts first**, and so does a name whose
/// only full stop opens it — `.env` is a hidden name, not an extension.
fn the_extension_of(name: &[u8]) -> String {
    match name.iter().rposition(|byte| *byte == b'.') {
        Some(at) if at > 0 => String::from_utf8_lossy(&name[at + 1..]).to_lowercase(),
        _ => String::new(),
    }
}

/// **The headers, with the one the list is sorted by marked.**
fn the_headers(order: TheOrder) -> TheHeaders {
    TheHeaders {
        name: Prose::new(words::NAME),
        kind: Prose::new(words::KIND),
        size: Prose::new(words::SIZE),
        changed: Prose::new(words::CHANGED),
        sorted_by: order.by,
        reversed: order.reversed,
    }
}

/// **The order chosen in this run** (hole 8: *"the choice held for the run and
/// not stored"*). **Process memory and durable nowhere** — the record is never
/// written with it — so every folder opened in the run is sorted the same way,
/// on every server, and a new run starts by name.
fn the_order_this_run() -> &'static Mutex<TheOrder> {
    static HELD: std::sync::OnceLock<Mutex<TheOrder>> = std::sync::OnceLock::new();
    HELD.get_or_init(|| Mutex::new(TheOrder::default()))
}

/// **The order the list is sorted by now.**
pub fn the_order_held() -> TheOrder {
    the_order_this_run().lock().map(|held| *held).unwrap_or_default()
}

/// **A header was pressed** (`Design/File Browser § The list`: *"The headers
/// sort."*). The page then hands back the rows it holds and they are put in
/// the new order ([`the_rows_are_put_in_order`]). **Nothing is read or written
/// but the choice itself.**
///
/// *(Until 2026-10-29 the page asked for the folder again after a press, and on
/// a live connection that read it from the server. The Creator's twenty-first
/// notes, note 2: "simply rearranging stuff already loaded on screen shouldn't
/// reload the folder altogether.")*
#[tauri::command]
pub fn the_list_is_sorted_by(by: TheColumn) {
    if let Ok(mut held) = the_order_this_run().lock() {
        *held = held.after_a_press_on(by);
    }
}

/// **What putting the rows held in order answered**: the headers, with the one
/// the list is now sorted by marked, and the order — for each place in the
/// list, which of the rows handed in stands there.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheRowsInOrder {
    pub headers: TheHeaders,
    pub order: Vec<usize>,
}

/// **The rows the page holds, put in the order held for the run** —
/// `Design/File Browser § The list`: *"A sort reorders the rows Smoozy already
/// holds and never reads the folder from the server again"*, as *Show hidden
/// files* does not (`File Browser` L14 and `File Browser` L21; the Creator's twenty-first
/// notes, note 2). The list keeps its folder, its selection and its count, and
/// only the order changes. **Nothing is read, from the machine or the record**,
/// so it works the same while the connection is down.
#[tauri::command]
pub fn the_rows_are_put_in_order(rows: Vec<WhatTheRowSortsBy>) -> TheRowsInOrder {
    the_rows_in(the_order_held(), &rows)
}

/// The whole of [`the_rows_are_put_in_order`] but the held choice, so a test
/// can hand it one.
pub fn the_rows_in(order: TheOrder, rows: &[WhatTheRowSortsBy]) -> TheRowsInOrder {
    let mut places: Vec<usize> = (0..rows.len()).collect();
    places.sort_by(|one, other| order.of_rows(&rows[*one], &rows[*other]));
    TheRowsInOrder { headers: the_headers(order), order: places }
}

/// **What the Upload control answered.**
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
#[serde(tag = "is", rename_all = "kebab-case")]
pub enum TheUploadAsk {
    /// **`File Browser` H4's fallback: the same Upload control asks which kind first.**
    /// *"That
    /// is one control either way: the fallback is a question inside Upload,
    /// never a second control beside it."*
    WhichKind {
        title: Prose,
        said: Prose,
        files: Prose,
        folders: Prose,
        cancel: Prose,
    },
    /// **The upload starts, with nothing asked first** — `Design/File Browser
    /// § Uploading`: a pick in the system panel, or a drop on the list, is the
    /// decision (the Creator's note 2 on the test copy). It carries the paths
    /// the user chose and no words, so the page hands the same ones back to
    /// `the_upload_is_confirmed` at once.
    Go { chosen: Vec<APath> },
    /// The user closed the panel.
    NothingWasChosen,
    /// Legacy closed-act result; ordinary free-software uploads do not raise it.
    SendingFilesUpIsClosed,
}

/// **What an upload's confirmation led to.**
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
#[serde(tag = "is", rename_all = "kebab-case")]
pub enum TheUploadGoes {
    /// **`File Browser` U4's one question**, where the walk found something
    /// already there. The request is named so the answer comes back to it.
    TheQuestion {
        request: Verbatim,
        question: Box<TheCollisionQuestion>,
    },
    /// **The files are going.** Nothing is said above the list while they
    /// do: the bottom bar's transfer count says so (`Design/File Browser §
    /// The list`, the Creator's notes piece 6).
    Underway { request: Verbatim },
    /// **Nothing moved**, and this says why.
    DidNotStart { said: Words },
    /// Legacy closed-act result; ordinary free-software uploads do not raise it.
    SendingFilesUpIsClosed,
    /// The user answered Stop, or the question was no longer held.
    NothingHappened,
}

/// **What is being dragged over the list** — `drag`.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheDropReading {
    pub into: Words,
    /// *"how many there are"* — **a count of what was dropped**, because paths
    /// and nothing else are known at that moment (`File Browser` H3a).
    pub how_many: Words,
}

/// **The already-exists box, in one shape and six readings.**
///
/// *"Three boxes rather than one"* is the mockup's own comment about its
/// markup; **it is one box with one sentence changed**, which is what the
/// mockup's own script does and what Design says: the question, the list, the
/// comparison line, the two warnings, and the answers the act offers.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheCollisionQuestion {
    /// The headline, leading with how many (the spec requires the number).
    pub headline: Words,
    /// *"One line says what the comparison was, and it stops there."*
    pub compared_by: Prose,
    /// The colliding names, listed beneath and scrollable.
    pub names: Vec<Verbatim>,
    /// *"…and 37 more"*, where the list is cut.
    pub and_more: Option<Words>,
    /// **`File Browser` W1b's line, and only an upload can meet it** — only an upload can
    /// meet files the dependency check put there.
    pub placed_by_the_check: Option<TheCheckLine>,
    /// **The live-job warning, above all four answers and never beside one.**
    pub live_jobs: Option<TheLiveJobWarning>,
    /// **`File Browser` U9b's reason line**, and nothing where no candidate record exists.
    pub interrupted: Option<Words>,
    /// The answers this act offers, in the drawing's own order. Each is
    /// `{ answer, label, put_forward }`.
    pub answers: Vec<AnAnswer>,
}

/// One answer of the already-exists box.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct AnAnswer {
    /// The token the page hands back — `replace`, `keep-both`, `skip-existing`,
    /// `stop`, `cancel`. A value.
    pub answer: Verbatim,
    pub label: Prose,
    /// **Only the resume reading carries a filled control**, and it carries one
    /// because Smoozy can say why.
    pub put_forward: bool,
}

/// **One quiet warning: the half the drawing bolds, then the half it does
/// not.**
///
/// The drawing writes *__3 were uploaded by the dependency check.__ Replacing
/// them brings back paths the render cannot find.* — **the first sentence
/// carries the figure and is bold, and the sentence that explains it is not.**
/// The two are two strings, which is X10: the point takes one piece of prose at
/// a time and never parses sentences out of a paragraph.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct AWarningLine {
    /// The bold half, with its figure beside its words.
    pub named: Words,
    /// The quiet half, where there is one.
    pub said: Option<Prose>,
}

/// `File Browser` W1b's line and the offer beside it.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheCheckLine {
    pub named: Words,
    pub said: Prose,
    pub offer: Prose,
}

/// **The live-job warning** — `File Browser` W5, `File Browser` W5a, `File Browser` W6.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheLiveJobWarning {
    /// The jobs, named, with what each is doing.
    pub said: Words,
    /// **`File Browser` W6's widest case, said rather than implied** — *"For that
    /// job, only its .blend and output folder were checked."*
    pub only_the_blend_and_its_folder: Option<Prose>,
    /// **The *may*, and never a description.**
    pub a_render_may_finish_anyway: Prose,
}

/// **The delete box, carrying two warnings of different weight.**
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheDeleteBox {
    pub question: Words,
    /// **First, in red.** It cannot be switched off.
    pub live_jobs: Option<TheLiveJobWarning>,
    /// **The second warning, with no live job behind it, and quieter** —
    /// `File Browser` W1.
    pub other_references: Option<AWarningLine>,
    /// **`File Browser` M3's own extra true thing**, where the victim holds
    /// live staged material.
    pub staged_material: Option<AWarningLine>,
    /// **The permanence warning, and it can be switched off.**
    pub permanent: Prose,
    pub dont_warn_again: Prose,
    /// *"says in one line that it does not cover the other"*.
    pub what_it_does_not_cover: Prose,
    pub cancel: Prose,
    pub delete: Prose,
    /// Whether the permanence half is suppressed already.
    pub the_permanence_warning_is_switched_off: bool,
}

/// **The live-job box the other three acts take** — the same box and the same
/// words, and only the title says which act is being asked about.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheLiveJobBox {
    pub question: Words,
    pub live_jobs: TheLiveJobWarning,
    pub cancel: Prose,
    pub carry_on: Prose,
}

/// **What an act answered.**
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
#[serde(tag = "is", rename_all = "kebab-case")]
pub enum TheAnswer {
    /// **The honest answer where an act needs the server instructed and nothing
    /// in this application instructs it.** Piece 3's shape.
    ///
    /// **It is not a failure record and it is not a stand-in for one.** It says
    /// what is true, which is Rule 8's own line and the fault L8a is written
    /// against: the alternative is a control that does nothing.
    TheActIsNotBuiltYet { said: Prose },
    /// Legacy closed-act result retained in this result type.
    TheActIsClosed { said: Prose },
    /// Nothing to do: the user cancelled, or the answer was empty.
    NothingHappened,
}

// ════════════════════════════════════════════════════════════════════════════
// The commands
// ════════════════════════════════════════════════════════════════════════════

/// **The whole browser for one server, standing in one folder.**
///
/// **A command rather than an event, and P6 is not being bent.** P6 is about a
/// module's *results*; this is the page asking for the facts it needs before it
/// can draw anything, which is the shape every other screen's own reading has.
///
/// **H34: a closed request and response type**, and nothing beneath the
/// application-service surface goes out.
///
/// **It reads nothing of the machine.** Where a connection is live it answers
/// *being read* with [`TheBrowser::a_reading_follows`] set, and the page asks
/// [`the_folder_is_read`] for the rows — which waits on the machine off the
/// drawing thread (P5).
///
/// **`async`, so it reads the record off the thread that draws** (P4, P5).
/// Changed 2026-09-28, step 10c's transfer wave piece 1. It takes the
/// application handle and reaches the state inside, the way
/// [`the_folder_is_read`] already does. **The case it was first written for**
/// is a record whose lock is held for a moment at a time, where a plain
/// command — which runs on the thread that draws — waited for nothing anybody
/// could see. **The case it now also meets** is a record held by a transfer:
/// the lock's holds were up to 41 s long near the end of a 4 GB upload before
/// the same piece made a save one save, and a press on Files waited that long
/// with the window frozen. The page code does not change.
#[tauri::command]
pub async fn the_files_tab<R: Runtime>(
    app: AppHandle<R>,
    server: String,
    folder: Option<Vec<u8>>,
    chosen: Option<Vec<u8>>,
) -> TheBrowser {
    let moved = the_move_stands(&server, folder.as_deref());
    let (there_is, shown) = what_there_is_and_the_choice(&app, &server);
    let periods = app
        .try_state::<Arc<ThePeriodsThisRun>>()
        .map(|periods| Arc::clone(periods.inner()));
    let there_is = with_what_servers_observed(there_is, periods.as_deref(), &server);
    let live = the_live_point_of(periods.as_deref(), &server).is_some();
    let paste = the_row_of(&app, &server).as_ref().and_then(what_paste_carries_on);
    let mut browser = the_tab_from(there_is, &server, folder, chosen, live, shown);
    paste_is_offered_on(&mut browser, paste);
    calculate_size_is_offered_on(&mut browser, the_row_of(&app, &server).as_ref().map(|row| the_count_stands_on(row, &server)));
    // **The headers say the order held for the run** (hole 8), so the mark
    // does not jump back to *Name* while a folder is read.
    browser.headers = the_headers(the_order_held());
    browser.bar = the_bar_across_the_top(&app, &server, moved);
    browser
}

/// **The one bar across the top**: a pick in hand (`File Browser` §12's entry
/// point from `Rendering`), or else the moved line where *show outputs* moved
/// the browser here ([`WhereTheBrowserIs`]). A pick is held only while the
/// user stays in the browser, so the two meet only where a pick was put in
/// hand after the move, and then the pick is what the user is doing.
fn the_bar_across_the_top<R: Runtime>(app: &AppHandle<R>, server: &str, moved: bool) -> Option<ABarAcrossTheTop> {
    crate::a_new_job_from_an_old_one::the_bar_in_hand()
        .or_else(|| moved.then(|| the_moved_bar_on(app, server)).flatten())
}

/// **The bar's way out, and the user leaving the browser** — `Design/File
/// Browser § The browser opened so the user can point at a file`: *"the
/// caller is told nothing was picked"*. The setup a re-submission held is let
/// go of and nothing is written (`crate::a_new_job_from_an_old_one`, the 38's
/// piece 2).
#[tauri::command]
pub fn the_pick_is_dropped() {
    crate::a_new_job_from_an_old_one::the_pick_is_dropped();
}

/// **The folder, read on the machine** — `File Browser` L1's one reading on the
/// connection's reading session, lent by `Server protocol and transfers` T5's
/// per-server point, with `File Browser` L11's free-space figure beside it.
///
/// **`async`, so it waits on the machine off the drawing thread** (P5); the
/// reading itself runs on the period's own thread, where the point is.
#[tauri::command]
pub async fn the_folder_is_read<R: Runtime>(
    app: AppHandle<R>,
    server: String,
    folder: Option<Vec<u8>>,
    chosen: Option<Vec<u8>>,
) -> TheBrowser {
    let moved = the_move_stands(&server, folder.as_deref());
    let no_frame_is_saved = no_frame_is_saved_here();
    let (there_is, shown) = what_there_is_and_the_choice(&app, &server);
    let periods = app
        .try_state::<Arc<ThePeriodsThisRun>>()
        .map(|periods| Arc::clone(periods.inner()));
    let there_is = with_what_servers_observed(there_is, periods.as_deref(), &server);
    let observed = match &there_is {
        WhatThereIs::TheServer(_, observed) => *observed,
        _ => None,
    };
    let point = the_live_point_of(periods.as_deref(), &server);
    let order = the_order_held();
    let (WhatThereIs::TheServer(..), Some(point)) = (&there_is, point) else {
        let mut browser = the_tab_from(there_is, &server, folder, chosen, false, shown);
        browser.headers = the_headers(order);
        browser.bar = the_bar_across_the_top(&app, &server, moved);
        return browser;
    };
    let folder = folder.unwrap_or_else(|| b"/".to_vec());
    let found = a_folder_is_read(&point, &folder).await;
    // **A job's folder *show outputs* landed on, not there yet, for a job
    // whose record shows no frame saved** — said as such (the Creator's
    // fourth notes, piece O).
    let found = if no_frame_is_saved { not_there_yet_where_no_frame_is_saved(found) } else { found };
    // **A reading the connection ended under is drawn with the server as it
    // stands now**, not as it stood when the reading was asked — the break
    // that ended it has moved the server off *connected* (`Servers` K6), and
    // the screen agrees with the server row (`Design/File Browser § While the
    // connection is down`). Step 10c's check-before-0.1.7 fix piece 1.
    let observed = if found == WhatTheReadingFound::NothingWasLent {
        let now = match app.try_state::<Mutex<Started>>() {
            Some(started) => started
                .lock()
                .map(|started| what_there_is(&started, &server))
                .unwrap_or(WhatThereIs::TheRecordWouldNotRead),
            None => WhatThereIs::TheRecordWouldNotRead,
        };
        match with_what_servers_observed(now, periods.as_deref(), &server) {
            WhatThereIs::TheServer(_, now) => now,
            _ => observed,
        }
    } else {
        observed
    };
    let paste = the_row_of(&app, &server).as_ref().and_then(what_paste_carries_on);
    let mut browser = the_tab_after_the_reading(order, &server, &folder, chosen.as_deref(), found, observed, shown);
    paste_is_offered_on(&mut browser, paste);
    calculate_size_is_offered_on(&mut browser, the_row_of(&app, &server).as_ref().map(|row| the_count_stands_on(row, &server)));
    // **`File Browser` §12's bar, while a pick is in hand** (the 38's piece 2),
    // or the moved line.
    browser.bar = the_bar_across_the_top(&app, &server, moved);
    browser
}

/// **How many transfers to or from this server are running or waiting**, as
/// the bottom bar says it — `Design/File Browser § The list`: *"While any
/// transfer to or from this server is running or waiting, the bottom bar says
/// how many — 3 transfers — as a link to Transfers."* **Nothing where there are
/// none.** The count is read from what Transfers lists for this server
/// ([`crate::surface::transfers::how_many_are_running_on`]) and never counted
/// a second time. The Creator's note 10 on 0.1.7, the Creator's notes piece 6.
///
/// **`async`, so it reads the record off the thread that draws**, as
/// Transfers' own screen does: the page asks again whenever a transfer moves.
#[tauri::command]
pub async fn the_transfers_on_this_server<R: Runtime>(app: AppHandle<R>, server: String) -> Option<Words> {
    the_transfer_count(crate::surface::transfers::how_many_are_running_on(&app, &server)?)
}

/// *3 transfers*, *1 transfer*, and nothing at none.
pub fn the_transfer_count(how_many: usize) -> Option<Words> {
    (how_many > 0).then(|| {
        Words::after(
            how_many.to_string(),
            if how_many == 1 { words::ONE_TRANSFER_ON_THIS_SERVER } else { words::TRANSFERS_ON_THIS_SERVER },
        )
    })
}

/// **The user ticked or unticked *Show hidden files*** — `File Browser` L20
/// and `File Browser` L21.
///
/// **It writes the choice and does nothing else**: the page re-composes from
/// the listing it already holds, so **the machine is never read** and it
/// works the same while the connection is down (`Design/File Browser § While
/// the connection is down`). **The answer is whether the record took it**; a
/// write that did not go down is said on this screen in the words a setting
/// that did not save already has, and the box stays as it was.
#[tauri::command]
pub fn show_hidden_files_was_ticked(
    started: State<'_, Mutex<Started>>,
    queue: State<'_, std::sync::Arc<TheOneQueue>>,
    server: String,
    shown: bool,
) -> bool {
    let saved = started
        .lock()
        .map(|started| the_choice_is_written(&started, shown))
        .unwrap_or(false);
    if !saved {
        the_queue_is_told(
            &queue,
            Event::about(About::TheRun, THE_ANSWER)
                .saying("what", Said::Value(Verbatim::new("the-setting-did-not-save")))
                .saying("server", Said::Value(Verbatim::new(server)))
                .saying("act", Said::Value(Verbatim::new("show-hidden-files")))
                .saying("said", Said::Prose(Prose::new(words::THE_SETTING_DID_NOT_SAVE))),
        );
    }
    saved
}

/// The whole of [`show_hidden_files_was_ticked`] but the telling, so a test
/// can stand where the command stands.
pub fn the_choice_is_written(started: &Started, shown: bool) -> bool {
    started
        .record
        .as_ref()
        .map(|record| crate::record::settings::hidden_files_were_switched(record.connection(), shown).is_ok())
        .unwrap_or(false)
}

/// **An upload the user confirmed** — the picked paths, into the folder being
/// browsed. `File Browser` §3 to §5, run by `src/uploading.rs`.
///
/// **Everything up to `File Browser` U4's question happens here**: the
/// enumeration, the request written down, and the walk. **Where something
/// collides the question goes back to the page and nothing moves until it is answered**; where nothing does, the files
/// go.
#[tauri::command]
pub async fn the_upload_is_confirmed<R: Runtime>(
    app: AppHandle<R>,
    server: String,
    folder: Vec<u8>,
    chosen: Vec<Vec<u8>>,
) -> TheUploadGoes {
    let mut row = None;
    let mut run = None;
    if let Some(started) = app.try_state::<Mutex<Started>>() {
        if let Ok(started) = started.lock() {
            if let WhatThereIs::TheServer(found, _) = what_there_is(&started, &server) {
                row = Some(*found);
            }
            run = started.run.clone();
        }
    }
    let (Some(row), Some(run)) = (row, run) else {
        return TheUploadGoes::DidNotStart {
            said: Words::plain(words::THE_RECORD_WOULD_NOT_TAKE_IT),
        };
    };
    let Some(server_row) = Identity::read_back(IdentityKind::LocalServer, &row.local_server_identifier)
    else {
        return TheUploadGoes::DidNotStart {
            said: Words::plain(words::THE_RECORD_WOULD_NOT_TAKE_IT),
        };
    };
    let periods = app
        .try_state::<Arc<ThePeriodsThisRun>>()
        .map(|periods| Arc::clone(periods.inner()));
    let Some(point) = the_live_point_of(periods.as_deref(), &server) else {
        return TheUploadGoes::DidNotStart {
            said: Words::plain(words::NOT_CONNECTED),
        };
    };
    let asked = WhatIsBeingUploaded {
        server_row,
        installation: row
            .last_confirmed_installation_identity
            .as_deref()
            .and_then(|text| Identity::read_back(IdentityKind::Installation, text)),
        destination: folder.clone(),
        chosen: chosen.iter().map(|bytes| the_path_of(bytes)).collect(),
        run,
    };
    let door = TheApplicationsRecord { app: app.clone() };
    let ready = match uploading::an_upload_is_prepared(&asked, &point, &door).await {
        Ok(ready) => ready,
        Err(WhyTheUploadDidNotStart::SendingFilesUpIsClosed) => {
            return TheUploadGoes::SendingFilesUpIsClosed
        }
        Err(why) => {
            return TheUploadGoes::DidNotStart {
                said: why_it_did_not_start(&why),
            }
        }
    };
    if ready.something_collides() {
        let request = ready.request.record.as_text();
        // **`File Browser` W1b's line and the live-job warning, read from the
        // local record at the pre-flight** (`File Browser` W2). Until step
        // 10c's Files' owed acts piece 1 both were composed from nothing here,
        // so a *Replace* over a texture a rendering job reads warned nobody.
        let holds = what_the_record_holds(&app, &server);
        let (placed, named, labels) = the_upload_warnings(&ready.the_walk, holds.as_ref());
        let question = the_collision_question(
            TheCollidingAct::AnUpload,
            &ready.the_walk,
            &folder,
            placed,
            &named,
            &labels,
            ready.the_reason_line.as_ref().map(|line| seconds_since_the_epoch(line.the_last_file_it_landed)),
        );
        uploads_waiting_on_their_question().insert(request.clone(), (server, row, ready, named));
        return TheUploadGoes::TheQuestion {
            request: Verbatim::new(request),
            question: Box::new(question),
        };
    }
    let mut ready = ready;
    match uploading::the_answer_is_applied(&mut ready, None, &point, &door).await {
        WhatTheAnswerLeft::ItGoes => the_files_go(app, server, &row, ready, point),
        _ => TheUploadGoes::DidNotStart {
            said: Words::plain(words::THE_RECORD_WOULD_NOT_TAKE_IT),
        },
    }
}

/// **One file sent up again, from Transfers** — *Upload it again* on a file
/// that changed while it was going up (`File Browser` K8's offer to send the
/// new version), and *Try again* on a failed upload. **A new request for that
/// one file** (the 38's piece 9, hole 10; `State, identity and recovery` P7:
/// *"Trying again is a new action with a new operation identity"*), into the
/// folder its first landing was in: enumerate, record the request, walk, send.
///
/// **Where something now stands at its name, the question is not asked
/// again**: Transfers has no already-exists box, and the user answered that
/// question for this very file when the first request was made. So
/// `the_earlier_answer` — the first request's own `File Browser` U4 answer —
/// is applied, with `File Browser` W8's telling first where it is *Replace*,
/// exactly as [`the_already_exists_question_was_answered`] applies one.
/// **Where the first request had no answer** (nothing stood there then), the
/// file is not sent and the act says so: something has arrived at the name
/// since, and choosing what happens to it is Files' question.
pub(crate) async fn one_file_goes_up_again<R: Runtime>(
    app: AppHandle<R>,
    server: String,
    folder: Vec<u8>,
    the_file: Vec<u8>,
    the_earlier_answer: Option<CollisionAnswer>,
) -> TheUploadGoes {
    let mut row = None;
    let mut run = None;
    if let Some(started) = app.try_state::<Mutex<Started>>() {
        if let Ok(started) = started.lock() {
            if let WhatThereIs::TheServer(found, _) = what_there_is(&started, &server) {
                row = Some(*found);
            }
            run = started.run.clone();
        }
    }
    let (Some(row), Some(run)) = (row, run) else {
        return TheUploadGoes::DidNotStart {
            said: Words::plain(words::THE_RECORD_WOULD_NOT_TAKE_IT),
        };
    };
    let Some(server_row) = Identity::read_back(IdentityKind::LocalServer, &row.local_server_identifier)
    else {
        return TheUploadGoes::DidNotStart {
            said: Words::plain(words::THE_RECORD_WOULD_NOT_TAKE_IT),
        };
    };
    let periods = app
        .try_state::<Arc<ThePeriodsThisRun>>()
        .map(|periods| Arc::clone(periods.inner()));
    let Some(point) = the_live_point_of(periods.as_deref(), &server) else {
        return TheUploadGoes::DidNotStart {
            said: Words::plain(words::NOT_CONNECTED),
        };
    };
    let asked = WhatIsBeingUploaded {
        server_row,
        installation: row
            .last_confirmed_installation_identity
            .as_deref()
            .and_then(|text| Identity::read_back(IdentityKind::Installation, text)),
        destination: folder,
        chosen: vec![the_path_of(&the_file)],
        run,
    };
    let door = TheApplicationsRecord { app: app.clone() };
    let mut ready = match uploading::an_upload_is_prepared(&asked, &point, &door).await {
        Ok(ready) => ready,
        Err(WhyTheUploadDidNotStart::SendingFilesUpIsClosed) => {
            return TheUploadGoes::SendingFilesUpIsClosed
        }
        Err(why) => {
            return TheUploadGoes::DidNotStart {
                said: why_it_did_not_start(&why),
            }
        }
    };
    let answer = if ready.something_collides() {
        match the_earlier_answer {
            Some(answer) if answer != CollisionAnswer::Stop => Some(answer),
            _ => {
                return TheUploadGoes::DidNotStart {
                    said: Words::plain(words::A_FILE_STANDS_THERE_NOW),
                }
            }
        }
    } else {
        None
    };
    if answer == Some(CollisionAnswer::Replace) {
        let holds = what_the_record_holds(&app, &server);
        let (_, named, _) = the_upload_warnings(&ready.the_walk, holds.as_ref());
        if uploading::the_telling_stands_first(&mut ready, &named, &door).is_err() {
            return TheUploadGoes::DidNotStart {
                said: Words::plain(words::THE_TELLING_WOULD_NOT_WRITE),
            };
        }
    }
    match uploading::the_answer_is_applied(&mut ready, answer, &point, &door).await {
        WhatTheAnswerLeft::ItGoes => the_files_go(app, server, &row, ready, point),
        WhatTheAnswerLeft::TheUserStoppedIt => TheUploadGoes::NothingHappened,
        WhatTheAnswerLeft::TheServerLentNothing(_) => TheUploadGoes::DidNotStart {
            said: Words::plain(words::NOT_CONNECTED),
        },
        WhatTheAnswerLeft::TheRecordWouldNotTakeIt => TheUploadGoes::DidNotStart {
            said: Words::plain(words::THE_RECORD_WOULD_NOT_TAKE_IT),
        },
    }
}

/// **`File Browser` U4: the answer to the already-exists question**, written
/// down before any transfer is arranged, and then the files go.
#[tauri::command]
pub async fn the_already_exists_question_was_answered<R: Runtime>(
    app: AppHandle<R>,
    request: String,
    answer: String,
) -> TheUploadGoes {
    let Some((server, row, mut ready, named)) = uploads_waiting_on_their_question().take(&request) else {
        return TheUploadGoes::NothingHappened;
    };
    let answer = match answer.as_str() {
        "replace" => CollisionAnswer::Replace,
        "keep-both" => CollisionAnswer::KeepBoth,
        "skip-existing" => CollisionAnswer::SkipExisting,
        _ => CollisionAnswer::Stop,
    };
    let periods = app
        .try_state::<Arc<ThePeriodsThisRun>>()
        .map(|periods| Arc::clone(periods.inner()));
    let Some(point) = the_live_point_of(periods.as_deref(), &server) else {
        return TheUploadGoes::DidNotStart {
            said: Words::plain(words::NOT_CONNECTED),
        };
    };
    let door = TheApplicationsRecord { app: app.clone() };
    // **`File Browser` W8: *Replace* over files a live job reads pushes past
    // the live-job warning, so the telling and each job's mark stand before
    // anything moves** — and where they cannot be written, nothing is
    // uploaded. Step 10c's Files' owed acts piece 2. Keep both, Skip existing
    // and Stop write over nothing, so they tell nothing.
    if answer == CollisionAnswer::Replace
        && uploading::the_telling_stands_first(&mut ready, &named, &door).is_err()
    {
        return TheUploadGoes::DidNotStart {
            said: Words::plain(words::THE_TELLING_WOULD_NOT_WRITE),
        };
    }
    match uploading::the_answer_is_applied(&mut ready, Some(answer), &point, &door).await {
        WhatTheAnswerLeft::ItGoes => the_files_go(app, server, &row, ready, point),
        WhatTheAnswerLeft::TheUserStoppedIt => TheUploadGoes::NothingHappened,
        WhatTheAnswerLeft::TheServerLentNothing(_) => TheUploadGoes::DidNotStart {
            said: Words::plain(words::NOT_CONNECTED),
        },
        WhatTheAnswerLeft::TheRecordWouldNotTakeIt => TheUploadGoes::DidNotStart {
            said: Words::plain(words::THE_RECORD_WOULD_NOT_TAKE_IT),
        },
    }
}

/// **What taking the offer beside `File Browser` W1b's line came to** — the
/// 38's piece 7, item 31.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheHandOver {
    /// **What the rest of the upload came to**: its question again where other
    /// files still collide, or the files that remain going.
    pub goes: TheUploadGoes,
    /// **The `.blend` whose check the handed files go through**, as its exact
    /// bytes on the machine (`File Browser` L1a) — the check the page opens and
    /// starts, where the files are sent, read and fixed as rows of their own.
    /// Nothing where no checked file's tree reached them.
    pub the_check_of: Option<Vec<u8>>,
    /// How many files were handed over.
    pub handed: usize,
}

/// **`File Browser` U12 to `File Browser` U15 and `Dependency Upload & Remap` A15 to `Dependency Upload & Remap` A18: the
/// offer beside `File Browser` W1b's line, taken** — the files the dependency check placed
/// or fixed, which this upload would have written over, are sent through the
/// check instead. The 38's piece 7, item 31; until then the offer answered
/// *not built yet*.
///
/// - **`File Browser` U12: each named unit takes the fourth disposition, *handed over*, and
///   the outcome *not attempted, handed to `Dependency Upload & Remap`***,
///   naming the act's identity — written through `File Browser`'s own
///   [`smoozy_file_browser::request::hand_over`]. **The enumeration is never
///   shortened.** A unit whose installation is not the server's current one
///   opens nothing and is left as the question found it (`Dependency Upload & Remap` A16).
/// - **`File Browser` U13: the rest of the request keeps the question.** Where other files
///   still collide, the question comes back over them alone; where none do,
///   the files that remain go, and a request every one of whose units was
///   handed ends having moved nothing (`File Browser` U14).
/// - **`Dependency Upload & Remap` A17's send, reading and fixing are the check's own**: the page opens
///   the check of the `.blend` whose tree reached the handed files
///   (`Dependency Upload & Remap` V13a's reading of which roots resolved a
///   location) and starts it. There the files read *found on your computer*
///   — their contents differ from what the check placed — and *Upload* sends
///   each to its own mirror (`Dependency Upload & Remap` A18), with the check's own replace-or-stop
///   (`Dependency Upload & Remap` Q8) before anything is written over, and a
///   linked `.blend` among them is fixed at the check that follows (M16).
#[tauri::command]
pub async fn the_files_the_check_placed_go_through_it<R: Runtime>(
    app: AppHandle<R>,
    request: String,
) -> TheHandOver {
    use smoozy_dependencies::landing::{opens, HandedUnit, PlacedOrFixed};
    use smoozy_file_browser::request::hand_over;
    use smoozy_file_browser::warnings::{the_plain_upload_warning, WhatTheLandingPathIsFor};
    let nothing = |goes: TheUploadGoes| TheHandOver { goes, the_check_of: None, handed: 0 };
    let Some((server, row, mut ready, named)) = uploads_waiting_on_their_question().take(&request) else {
        return nothing(TheUploadGoes::NothingHappened);
    };
    let holds = what_the_record_holds(&app, &server);
    let lines = match holds.as_ref() {
        Some(holds) => the_plain_upload_warning(&ready.the_walk, &holds.knows()),
        None => Vec::new(),
    };
    let current = row
        .last_confirmed_installation_identity
        .as_deref()
        .and_then(|text| Identity::read_back(IdentityKind::Installation, text));
    let layout = crate::servers::installation::the_layout_recorded_for(&row);
    let door = TheApplicationsRecord { app: app.clone() };
    // **The units that open** (`Dependency Upload & Remap` A16), with the mirror each goes to (`Dependency Upload & Remap` A18).
    let mut opening: Vec<(Vec<u8>, smoozy_dependencies::bytes::MachineLocation)> = Vec::new();
    for line in &lines {
        let Some(entry) = ready.request.entries.iter().find(|entry| {
            smoozy_file_browser::already_exists::paths_are_equal(&entry.landing_path, &line.landing_path)
        }) else {
            continue;
        };
        let unit = HandedUnit {
            source_path: smoozy_dependencies::bytes::StoredLocation::as_the_system_returned_it(&entry.source_path),
            server_row: ready.request.server_row.clone(),
            installation: ready.request.installation.clone(),
            occupied_landing_path: smoozy_dependencies::bytes::MachineLocation::composed(entry.landing_path.clone()),
            class: match line.class {
                WhatTheLandingPathIsFor::Fixed => PlacedOrFixed::Fixed,
                _ => PlacedOrFixed::Placed,
            },
        };
        let (Some(current), Some(layout)) = (current.as_ref(), layout.as_ref()) else {
            continue;
        };
        if opens(&unit, current).is_err() {
            continue;
        }
        if let Ok(mirror) = smoozy_dependencies::landing::the_destination_of(layout, &unit) {
            opening.push((entry.landing_path.clone(), mirror));
        }
    }
    let mirrors: Vec<smoozy_dependencies::bytes::MachineLocation> =
        opening.iter().map(|(_, mirror)| mirror.clone()).collect();
    let the_check_of = the_root_whose_tree_reached(&app, &row, &mirrors);
    // **No checked file's tree reached them, so no check can take them**: the
    // hand-over does not open, the entries stay as the question found them
    // (`File Browser` U12), and the question comes back without the offer.
    let mut handed: Vec<Vec<u8>> = Vec::new();
    if the_check_of.is_some() {
        for (landing, _) in &opening {
            let Ok(the_act) = Identity::issue(IdentityKind::Operation) else {
                continue;
            };
            let mut written = false;
            crate::uploading::TheRecordAnUploadWrites::with_the_record(&door, &mut |connection| {
                let mut keeper = crate::record::upload_requests::UploadRequests::over(connection);
                written = hand_over(
                    &mut ready.request,
                    landing,
                    the_act.clone(),
                    std::time::SystemTime::now(),
                    &mut keeper,
                )
                .is_ok();
            });
            if written {
                handed.push(landing.clone());
            }
        }
    }

    // **`File Browser` U13: the rest keeps the question.** The walk's comparisons stay whole
    // — the answer is matched against every entry — and only its count of
    // what still collides moves; the box is drawn over the files left.
    ready.the_walk.colliding = ready.the_walk.colliding.saturating_sub(handed.len());
    if ready.something_collides() {
        let mut left = ready.the_walk.clone();
        let the_check_of = the_check_of.filter(|_| !handed.is_empty());
        left.comparisons
            .retain(|one| !handed.iter().any(|gone| smoozy_file_browser::already_exists::paths_are_equal(gone, &one.landing_path)));
        let (_, named_left, labels) = the_upload_warnings(&left, holds.as_ref());
        let question = the_collision_question(
            TheCollidingAct::AnUpload,
            &left,
            &ready.request.destination,
            0,
            &named_left,
            &labels,
            ready.the_reason_line.as_ref().map(|line| seconds_since_the_epoch(line.the_last_file_it_landed)),
        );
        let request = ready.request.record.as_text();
        uploads_waiting_on_their_question().insert(request.clone(), (server, row, ready, named));
        return TheHandOver {
            goes: TheUploadGoes::TheQuestion {
                request: Verbatim::new(request),
                question: Box::new(question),
            },
            the_check_of,
            handed: handed.len(),
        };
    }
    let periods = app
        .try_state::<Arc<ThePeriodsThisRun>>()
        .map(|periods| Arc::clone(periods.inner()));
    let Some(point) = the_live_point_of(periods.as_deref(), &server) else {
        return TheHandOver {
            goes: TheUploadGoes::DidNotStart { said: Words::plain(words::NOT_CONNECTED) },
            the_check_of,
            handed: handed.len(),
        };
    };
    let goes = match uploading::the_answer_is_applied(&mut ready, None, &point, &door).await {
        WhatTheAnswerLeft::ItGoes => the_files_go(app, server, &row, ready, point),
        WhatTheAnswerLeft::TheUserStoppedIt => TheUploadGoes::NothingHappened,
        WhatTheAnswerLeft::TheServerLentNothing(_) => {
            TheUploadGoes::DidNotStart { said: Words::plain(words::NOT_CONNECTED) }
        }
        WhatTheAnswerLeft::TheRecordWouldNotTakeIt => {
            TheUploadGoes::DidNotStart { said: Words::plain(words::THE_RECORD_WOULD_NOT_TAKE_IT) }
        }
    };
    TheHandOver { goes, the_check_of, handed: handed.len() }
}

/// **The `.blend` whose check the handed files go through** — the first root,
/// in the record's order, whose checked tree resolved one of their mirrors:
/// `Dependency Upload & Remap` V13a's own reading of which roots a
/// publication at a location disturbs, taken from the record alone. Nothing
/// where the record would not read or no tree reached them.
fn the_root_whose_tree_reached<R: Runtime>(
    app: &AppHandle<R>,
    row: &ServerRow,
    mirrors: &[smoozy_dependencies::bytes::MachineLocation],
) -> Option<Vec<u8>> {
    let layout = crate::servers::installation::the_layout_recorded_for(row)?;
    let server_row = Identity::read_back(IdentityKind::LocalServer, &row.local_server_identifier)?;
    let installation = row
        .last_confirmed_installation_identity
        .as_deref()
        .and_then(|text| Identity::read_back(IdentityKind::Installation, text))?;
    let machine = smoozy_dependencies::record::MachineKey::of(server_row, installation);
    let started = app.try_state::<Mutex<Started>>()?;
    let started = started.lock().ok()?;
    let connection = started.record.as_ref()?.connection();
    let keeper = crate::record::dependency_records::TheDependencyRecord::over(connection);
    let snapshot = keeper.what_it_holds_for(&machine).ok()?;
    let roots: Vec<smoozy_dependencies::bytes::MachineLocation> = keeper
        .the_standings_on(&machine)
        .ok()?
        .into_iter()
        .map(|standing| standing.root)
        .collect();
    let files_says = crate::record::dependency_records::WhatFilesSays::read(connection, &machine);
    mirrors.iter().find_map(|mirror| {
        smoozy_dependencies::acting::marks::what_this_publication_marks(
            mirror,
            &machine,
            &layout,
            &snapshot,
            &roots,
            &|file| files_says.where_it_came_from(file),
        )
        .disturbed
        .first()
        .map(|root| root.as_bytes().to_vec())
    })
}

/// **What an upload came to, said once** — the page asks for it when the queue
/// says the upload ended, and the words are let go of as they are handed over.
#[tauri::command]
pub fn what_the_upload_came_to(request: String) -> Option<Words> {
    uploads_that_ended().take(&request)
}

/// **The Upload control, and this is the picker's first real caller anywhere in
/// Smoozy.**
///
/// `what` is `"both"` on the first press, and `"files"` or `"folders"` where
/// the user answered `File Browser` H4's question.
///
/// **`async`, because the system's file panel is put up here** and a plain
/// command runs on the drawing thread — see [`crate::picker::asked`]. So the
/// record is reached through the handle rather than as a borrowed argument,
/// which the shell does not take on an `async` command that answers a value.
#[tauri::command]
pub async fn an_upload_is_asked_for<R: Runtime>(app: AppHandle<R>, what: String) -> TheUploadAsk {
    let what = match what.as_str() {
        "files" => WhatMayBeChosen::Files,
        "folders" => WhatMayBeChosen::Folders,
        _ => WhatMayBeChosen::FilesAndFolders,
    };
    let picked = picker::asked(
        &app,
        Asking {
            what,
            how_many: HowMany::Any,
            starting_at: None,
            title: Prose::new(words::PICK_WHAT_TO_UPLOAD),
        },
    )
    .await;
    the_upload_ask_from(picked)
}

/// **What was dropped on the list goes** — no confirmation (`Design/File
/// Browser § Uploading`, the Creator's note 2 on the test copy).
///
/// The paths arrive from the shell's own drag-and-drop facility (`File Browser` H3a) and
/// **paths are all that arrives**: no sizes, no counts, no walk.
#[tauri::command]
pub fn files_were_dropped(paths: Vec<String>) -> TheUploadAsk {
    let chosen: Vec<std::path::PathBuf> = paths.into_iter().map(std::path::PathBuf::from).collect();
    if chosen.is_empty() {
        return TheUploadAsk::NothingWasChosen;
    }
    the_upload_goes_with(&chosen)
}

/// **What is being dragged over the list** — the dashed border's own reading.
#[tauri::command]
pub fn what_is_being_dropped(folder: Vec<u8>, paths: Vec<String>) -> TheDropReading {
    let chosen: Vec<std::path::PathBuf> = paths.into_iter().map(std::path::PathBuf::from).collect();
    the_drop_reading_from(&chosen, &folder)
}

/// **The delete box, composed at the moment the victim is named, before any
/// write, with no read of the machine** (`File Browser` W1, `File Browser` W2).
///
/// **The victims are paths on the machine**, as the page hands the chosen
/// row's path over — never a bare name, which names a different thing.
///
/// **The two warnings read the local record** — `Dependency Upload & Remap`'s
/// own record, the local job records and the run's queue reading, through
/// [`the_delete_box_over_the_record`]. Until step 10c's Files' owed acts piece
/// 1 this box was composed from nothing and carried the permanence warning
/// alone.
#[tauri::command]
pub fn the_delete_of<R: Runtime>(
    app: AppHandle<R>,
    started: State<'_, Mutex<Started>>,
    server: String,
    victims: Vec<Vec<u8>>,
) -> TheDeleteBox {
    let switched_off = started
        .lock()
        .ok()
        .map(|started| the_delete_warning_is_switched_off(&started))
        .unwrap_or(false);
    let holds = what_the_record_holds(&app, &server);
    let transfers = started
        .lock()
        .ok()
        .and_then(|started| {
            started
                .record
                .as_ref()
                .and_then(|record| crate::record::transfer_records::all(record.connection()).ok())
        })
        .unwrap_or_default();
    the_delete_box_over_the_record(&victims, holds.as_ref(), &transfers, switched_off).0
}

/// **The live-job box the rename and the move take** — the same box and the
/// same words, the title saying which act. **Paths, never names**, and read
/// from the local record as the delete box is (step 10c's Files' owed acts
/// piece 1).
#[tauri::command]
pub fn what_disturbing_costs<R: Runtime>(
    app: AppHandle<R>,
    server: String,
    act: String,
    paths: Vec<Vec<u8>>,
    into: Option<Vec<u8>>,
) -> Option<TheLiveJobBox> {
    let holds = what_the_record_holds(&app, &server);
    the_live_job_box_over_the_record(&act, &paths, into.as_deref(), holds.as_ref()).0
}

/// **The delete box's *Don't warn me about deleting again*, saved** — must-do
/// 24 of the step 10a round 1 audit of these pieces.
///
/// **The user was asked a question and answered it**, so the switch-off is sent
/// at the press even though the delete itself still answers that it cannot be
/// done. **It goes through the shared service** —
/// [`crate::surface::said_once::a_warning_was_switched_off`], with `File
/// Browser` M3's own identity — **and a write that did not go down says so**
/// on this screen rather than being taken as done.
#[tauri::command]
pub fn the_delete_warning_was_switched_off(
    started: State<'_, Mutex<Started>>,
    queue: State<'_, std::sync::Arc<TheOneQueue>>,
    server: String,
) {
    let saved = started
        .lock()
        .map(|started| {
            crate::surface::said_once::a_warning_was_switched_off(
                &started,
                THIS_MODULE,
                the_delete_warnings_suppression_row(),
            )
        })
        .unwrap_or(false);
    if saved {
        return;
    }
    the_queue_is_told(
        &queue,
        Event::about(About::TheRun, THE_ANSWER)
            .saying("what", Said::Value(Verbatim::new("the-setting-did-not-save")))
            .saying("server", Said::Value(Verbatim::new(server)))
            .saying("act", Said::Value(Verbatim::new("dont-warn-about-deleting")))
            .saying("said", Said::Prose(Prose::new(words::THE_SETTING_DID_NOT_SAVE))),
    );
}

/// **Every act that needs the machine, answered rather than met with silence.**
///
/// Delete, rename, move, copy, paste, a new folder, the elevated retry and the
/// collision answer all end here today. **Rule 8, and the fault L8a is written
/// against**: a control that does nothing is Smoozy going quiet. **Download is
/// no longer among them** — it goes to Transfers with what the user picked and
/// is answered there.
#[tauri::command]
pub fn a_file_act_that_needs_a_connection(
    queue: State<'_, std::sync::Arc<TheOneQueue>>,
    server: String,
    act: String,
) -> TheAnswer {
    let answer = TheAnswer::TheActIsNotBuiltYet {
        said: Prose::new(words::NOT_BUILT_YET),
    };
    the_queue_is_told(
        &queue,
        Event::about(About::TheRun, THE_ANSWER)
            .saying("what", Said::Value(Verbatim::new("the-act-is-not-built")))
            .saying("server", Said::Value(Verbatim::new(server)))
            .saying("act", Said::Value(Verbatim::new(act)))
            .saying("said", Said::Prose(Prose::new(words::NOT_BUILT_YET))),
    );
    answer
}

// ════════════════════════════════════════════════════════════════════════════
// New folder and Rename — step 10c's Files' owed acts piece 3
// ════════════════════════════════════════════════════════════════════════════

/// **What an act on one row came to, as the page draws it.**
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
#[serde(tag = "is", rename_all = "kebab-case")]
pub enum AnActsAnswer {
    /// **It was done.** The list is read again; `said`, where there is
    /// anything to say, goes on the line above it.
    Done { said: Option<Words> },
    /// **The row stays in editing**, red, with the line under it — a name
    /// refused before trying, a taken name, or the login may not.
    OnTheRow {
        editing: ARowInEditing,
        refusal: Option<ARefusal>,
    },
    /// **Said on the line above the list**, and the list is read again.
    Said { said: Words },
    /// **`File Browser` M8's question comes first**: the box, and the act is
    /// asked again with the answer (step 10c's Files' owed acts piece 5).
    Asks { question: TheCollisionQuestion },
}

/// **What the page is told after a new folder or a rename** — composed here so
/// the desk can drive every reading of it. `File Browser` M4, `File Browser`
/// M9, `File Browser` P1;
/// `Design/File Browser § Renaming into a name that is taken`, `§ What the
/// screen says around an act`.
pub fn what_the_row_says(
    came_to: &crate::managing_files::WhatTheActCameTo,
    typed: &[u8],
    what: TheRefusedAct,
    hidden_files_are_shown: bool,
) -> AnActsAnswer {
    use crate::managing_files::WhatTheActCameTo as Came;
    use smoozy_file_browser::managing::WhyTheNameIsRefused as Why;
    let in_editing = |said: &str| ARowInEditing {
        typed: Verbatim::new(String::from_utf8_lossy(typed).into_owned()),
        refused: Some(Words::plain(said)),
        the_listing_saw_it: None,
    };
    match came_to {
        Came::Done => AnActsAnswer::Done {
            // **A rename to a name starting with a period, with *Show hidden
            // files* off, takes the row out of the list** (`File Browser`
            // L19), so the line says where it went.
            said: (what == TheRefusedAct::Renaming
                && typed.first() == Some(&b'.')
                && !hidden_files_are_shown)
                .then(|| {
                    Words::with(words::RENAMED_NOW_HIDDEN_LEAD, String::from_utf8_lossy(typed).into_owned())
                        .then(words::RENAMED_NOW_HIDDEN_REST)
                        .joined()
                }),
        },
        Came::TheNameIsRefused(why) => AnActsAnswer::OnTheRow {
            editing: in_editing(match why {
                Why::Empty => words::TYPE_A_NAME,
                Why::HoldsASlash => words::A_NAME_HOLDS_NO_SLASH,
                Why::IsADotName => words::A_NAME_IS_NOT_A_DOT,
                Why::TooLong => words::THE_NAME_IS_TOO_LONG,
                Why::IsTheStagingFoldersName => words::THE_NAME_IS_SMOOZYS_OWN,
            }),
            refusal: None,
        },
        Came::TheNameIsTaken => AnActsAnswer::OnTheRow {
            editing: in_editing(words::THE_NAME_IS_TAKEN),
            refusal: None,
        },
        // **The login may not, with no admin offer** — the retry it would
        // start is not built (`Preparation/Build/Plan.md`, step 10c).
        Came::TheLoginMayNot => AnActsAnswer::OnTheRow {
            editing: ARowInEditing {
                typed: Verbatim::new(String::from_utf8_lossy(typed).into_owned()),
                refused: None,
                the_listing_saw_it: None,
            },
            refusal: Some(a_refusal_with_no_retry_built(what)),
        },
        Came::ItIsNotThere => AnActsAnswer::Said {
            said: Words::plain(words::IT_IS_NOT_THERE_ANY_MORE),
        },
        Came::TheMachineRefused => AnActsAnswer::Said {
            said: Words::plain(words::THE_SERVER_REFUSED_IT),
        },
        Came::TheTellingWouldNotWrite => AnActsAnswer::Said {
            said: Words::plain(words::NOTHING_RENAMED_THE_TELLING),
        },
        Came::TheConnectionWent => AnActsAnswer::Said {
            said: Words::plain(words::THE_CONNECTION_WENT),
        },
    }
}

/// **The row, its machine and its live point**, for an act — or what to say
/// where there is none.
fn what_an_act_runs_on<R: Runtime>(
    app: &AppHandle<R>,
    server: &str,
) -> Result<(ServerRow, ThisServersSessions, bool), AnActsAnswer> {
    let not_connected = || AnActsAnswer::Said {
        said: Words::plain(words::NOT_CONNECTED_FOR_AN_ACT),
    };
    let (row, shown) = {
        let Some(started) = app.try_state::<Mutex<Started>>() else {
            return Err(not_connected());
        };
        let Ok(started) = started.lock() else {
            return Err(not_connected());
        };
        match what_there_is_with_the_choice(&started, server) {
            (WhatThereIs::TheServer(row, _), shown) => (*row, shown),
            _ => return Err(not_connected()),
        }
    };
    let periods = app
        .try_state::<Arc<ThePeriodsThisRun>>()
        .map(|periods| Arc::clone(periods.inner()));
    let Some(point) = the_live_point_of(periods.as_deref(), server) else {
        return Err(not_connected());
    };
    Ok((row, point, shown))
}

/// The door `File Browser` W11's marks go through for an act on this row's machine.
fn the_marking_door<R: Runtime>(app: &AppHandle<R>, row: &ServerRow) -> TheNeighboursOfThisMachine<R> {
    TheNeighboursOfThisMachine {
        app: app.clone(),
        server_row: Identity::read_back(IdentityKind::LocalServer, &row.local_server_identifier)
            .unwrap_or_else(|| Identity::issue(IdentityKind::LocalServer).expect("randomness")),
        installation: row
            .last_confirmed_installation_identity
            .as_deref()
            .and_then(|text| Identity::read_back(IdentityKind::Installation, text)),
        layout: crate::servers::installation::the_layout_recorded_for(row),
    }
}

/// **New folder** — `File Browser` M9, `Design/File Browser § What the screen
/// says around an act`. `shown` is the names the list the user is looking at
/// holds, which answers *already exists* before the attempt.
#[tauri::command]
pub async fn a_new_folder_is_made<R: Runtime>(
    app: AppHandle<R>,
    server: String,
    folder: Vec<u8>,
    name: Vec<u8>,
    shown: Vec<Vec<u8>>,
) -> AnActsAnswer {
    let (row, point, hidden_files_are_shown) = match what_an_act_runs_on(&app, &server) {
        Ok(held) => held,
        Err(said) => return said,
    };
    let came_to = crate::managing_files::a_folder_is_made(&point, &folder, &name, &shown).await;
    the_act_goes_in_the_diary(
        &app,
        &row,
        &came_to,
        smoozy_file_browser::diary::FileAct::NewFolder,
        &crate::managing_files::inside(&folder, &name),
        &[],
    );
    what_the_row_says(&came_to, &name, TheRefusedAct::MakingAFolder, hidden_files_are_shown)
}

/// **Rename** — `File Browser` M4. **The live-job box came first on the page
/// where `File Browser` W5 named a job**; here the jobs are read again at the
/// moment of the act (`File Browser` W2), and `File Browser` W8's marks stand
/// before the rename.
#[tauri::command]
pub async fn a_rename_is_made<R: Runtime>(
    app: AppHandle<R>,
    server: String,
    path: Vec<u8>,
    name: Vec<u8>,
) -> AnActsAnswer {
    let (row, point, hidden_files_are_shown) = match what_an_act_runs_on(&app, &server) {
        Ok(held) => held,
        Err(said) => return said,
    };
    let holds = what_the_record_holds(&app, &server);
    let (_, named) = the_live_job_box_over_the_record("rename", &[path.clone()], None, holds.as_ref());
    let door = TheApplicationsRecord { app: app.clone() };
    let marking = the_marking_door(&app, &row);
    let came_to =
        crate::managing_files::a_rename_is_made(&point, &door, &marking, &path, &name, &named).await;
    // **The jobs on the line are the ones the telling made durable**
    // (`File Browser` B3, `File Browser` W8), so none where it could not
    // stand.
    let told: &[AJobNamed] = if came_to == crate::managing_files::WhatTheActCameTo::TheTellingWouldNotWrite {
        &[]
    } else {
        &named
    };
    the_act_goes_in_the_diary(&app, &row, &came_to, smoozy_file_browser::diary::FileAct::Rename, &path, told);
    what_the_row_says(&came_to, &name, TheRefusedAct::Renaming, hidden_files_are_shown)
}

/// **Delete** — `File Browser` M1 to `File Browser` M3 and `File Browser` X12,
/// step 10c's Files' owed acts
/// piece 4. **The box came first on the page**, and its warnings are read
/// again here at the moment of the act (`File Browser` W2), so the jobs the
/// telling names are the ones the record holds now. An upload this run is
/// still sending into a victim is stopped through the run's own Stop first
/// (`File Browser` X12: *an active transfer is stopped under the same ordering
/// before its bytes can continue*).
///
/// **The line above the list says how far it has got** as each batch of
/// removals answers, at most four times a second, and then how it ended.
#[tauri::command]
pub async fn a_delete_is_made<R: Runtime>(
    app: AppHandle<R>,
    server: String,
    victims: Vec<Vec<u8>>,
) -> AnActsAnswer {
    let (row, point, _) = match what_an_act_runs_on(&app, &server) {
        Ok(held) => held,
        Err(said) => return said,
    };
    let holds = what_the_record_holds(&app, &server);
    let (_, named) = the_delete_box_over_the_record(&victims, holds.as_ref(), &[], false);
    the_uploads_going_into_them_are_stopped(&app, &row, &victims);
    let run = app
        .try_state::<Mutex<Started>>()
        .and_then(|started| started.lock().ok().and_then(|started| started.run.clone()))
        .or_else(|| Identity::issue(IdentityKind::Run).ok());
    let (Some(server_row), Some(run)) =
        (Identity::read_back(IdentityKind::LocalServer, &row.local_server_identifier), run)
    else {
        return AnActsAnswer::Said {
            said: Words::plain(words::NOTHING_DELETED_THE_RECORD),
        };
    };
    let asked = crate::managing_files::ADeleteAsked {
        victims: victims.clone(),
        server_row,
        installation: row
            .last_confirmed_installation_identity
            .as_deref()
            .and_then(|text| Identity::read_back(IdentityKind::Installation, text)),
        run,
    };
    let door = TheApplicationsRecord { app: app.clone() };
    let marking = the_marking_door(&app, &row);
    let last_said = Mutex::new(None::<std::time::Instant>);
    let queue = app.try_state::<Arc<TheOneQueue>>().map(|queue| Arc::clone(queue.inner()));
    let moved = |done: usize, total: usize| {
        let Ok(mut last) = last_said.lock() else {
            return;
        };
        if last.is_some_and(|at| at.elapsed() < std::time::Duration::from_millis(250)) && done < total {
            return;
        }
        *last = Some(std::time::Instant::now());
        if let Some(queue) = queue.as_ref() {
            the_queue_is_told(queue, an_act_moves(&server, "delete", done, total));
        }
    };
    let came_to = crate::managing_files::a_delete_is_made(
        &point,
        &door,
        &marking,
        &asked,
        &named,
        crate::managing_files::THE_REMOVALS_IN_FLIGHT,
        &moved,
    )
    .await;
    the_delete_goes_in_the_diary(&app, &row, &victims, &came_to, &named);
    what_the_delete_says(&came_to, &victims)
}

/// **How far an act has got**, as the line above the list says it: the act's
/// word, the count done and the count there is, each figure by the verbatim
/// route.
pub fn an_act_moves(server: &str, act: &str, done: usize, total: usize) -> Event {
    Event::about(About::TheRun, THE_ANSWER)
        .saying("what", Said::Value(Verbatim::new("an-act-moves")))
        .saying("server", Said::Value(Verbatim::new(server.to_string())))
        .saying("act", Said::Value(Verbatim::new(act.to_string())))
        .saying("lead", Said::Prose(Prose::new(if act == "copy" { words::COPYING } else { words::DELETING })))
        .saying("done", Said::Value(Verbatim::new(done.to_string())))
        .saying("of", Said::Prose(Prose::new(words::OF)))
        .saying("total", Said::Value(Verbatim::new(total.to_string())))
}

/// **What the page is told after a delete** — `Design/File Browser § What the
/// screen says around an act`: what was done, what was refused, what was not
/// attempted, and the list read again.
pub fn what_the_delete_says(
    came_to: &crate::managing_files::WhatTheDeleteCameTo,
    victims: &[Vec<u8>],
) -> AnActsAnswer {
    use crate::managing_files::{WhatTheActCameTo as Came, WhatTheDeleteCameTo as Delete};
    match came_to {
        Delete::NothingWasRemoved(why) => AnActsAnswer::Said {
            said: match why {
                Came::ItIsNotThere => Words::plain(words::IT_IS_NOT_THERE_ANY_MORE),
                Came::TheLoginMayNot => a_refusal_with_no_retry_built(TheRefusedAct::Deleting).said,
                Came::TheTellingWouldNotWrite => Words::plain(words::NOTHING_DELETED_THE_RECORD),
                Came::TheConnectionWent => Words::plain(words::THE_CONNECTION_WENT),
                _ => Words::plain(words::THE_SERVER_REFUSED_IT),
            },
        },
        Delete::Ran {
            done,
            refused: 0,
            not_attempted: 0,
            never_learned: 0,
            ..
        } if *done > 0 => AnActsAnswer::Done {
            said: Some(match victims {
                [one] => Words::around(
                    words::DELETED_LEAD,
                    String::from_utf8_lossy(the_last_name(one)).into_owned(),
                    ".",
                )
                .joined(),
                _ => Words::after(done.to_string(), words::DELETED_COUNT_REST.trim_end_matches(',')).joined(),
            }),
        },
        Delete::Ran {
            done,
            refused,
            not_attempted,
            never_learned,
            ..
        } => {
            let mut said = Words::after(done.to_string(), words::DELETED_COUNT_REST)
                .and(refused.to_string())
                .then(words::REFUSED_COUNT_REST)
                .and(not_attempted.to_string())
                .then(words::NOT_TRIED_COUNT_REST);
            if *never_learned > 0 {
                said = said.then(words::SOME_NEVER_ANSWERED);
            }
            AnActsAnswer::Said { said }
        }
    }
}

/// **`File Browser` B3's line for a delete**: one line for the act, the
/// counts over its units. **Written where the act's record stood** — a delete
/// that never got that far asked nothing of the machine.
fn the_delete_goes_in_the_diary<R: Runtime>(
    app: &AppHandle<R>,
    row: &ServerRow,
    victims: &[Vec<u8>],
    came_to: &crate::managing_files::WhatTheDeleteCameTo,
    told: &[AJobNamed],
) {
    let crate::managing_files::WhatTheDeleteCameTo::Ran {
        done,
        refused,
        not_attempted,
        ..
    } = came_to
    else {
        return;
    };
    let about = match victims {
        [one] => smoozy_file_browser::diary::WhatTheActWasAbout::OneThing(String::from_utf8_lossy(one).into_owned()),
        many => smoozy_file_browser::diary::WhatTheActWasAbout::AFolderAndACount {
            folder: String::from_utf8_lossy(&crate::managing_files::the_folder_holding(&many[0])).into_owned(),
            entries: many.len(),
        },
    };
    let Some(started) = app.try_state::<Mutex<Started>>() else {
        return;
    };
    let Ok(started) = started.lock() else {
        return;
    };
    let _ = crate::managing_files::the_act_is_written_down(
        &started,
        row,
        smoozy_file_browser::diary::FileAct::Delete,
        about,
        None,
        smoozy_file_browser::diary::TheThreeCounts {
            done: *done,
            refused: *refused,
            not_attempted: *not_attempted,
        },
        told,
    );
}

/// **`File Browser` X12's active transfer, stopped before the delete**: an
/// upload this run is still sending into a victim has those files stopped
/// through the run's own Stop (`File Browser` U6), which the byte loop reads
/// at its next accounting interval and records as stopped itself.
fn the_uploads_going_into_them_are_stopped<R: Runtime>(app: &AppHandle<R>, row: &ServerRow, victims: &[Vec<u8>]) {
    let Some(the_run) = app.try_state::<Arc<TheRunsTransfers>>() else {
        return;
    };
    let mut going: Vec<(String, Vec<u8>)> = Vec::new();
    if let Some(started) = app.try_state::<Mutex<Started>>() {
        if let Ok(started) = started.lock() {
            if let Some(record) = started.record.as_ref() {
                let this_run = started.run.as_ref().map(Identity::as_text);
                for one in crate::record::transfer_records::all(record.connection()).unwrap_or_default() {
                    let inside_a_victim = one.every_staging_path_it_has_named().iter().any(|staging| {
                        victims.iter().any(|victim| {
                            smoozy_file_browser::already_exists::is_inside(&staging.as_bytes(), victim)
                        })
                    });
                    if one.ending.is_some()
                        || one.server.as_text() != row.local_server_identifier
                        || Some(one.written_by_run.as_text()) != this_run
                        || !inside_a_victim
                    {
                        continue;
                    }
                    let request: Option<String> = record
                        .connection()
                        .query_row(
                            "SELECT record FROM upload_request_entry WHERE transfer = ?1",
                            [one.transfer.as_text()],
                            |found| found.get(0),
                        )
                        .ok();
                    if let Some(request) = request {
                        going.push((request, one.final_path.clone().into_bytes()));
                    }
                }
            }
        }
    }
    for (request, landing) in going {
        the_run.the_user_stops(&request, Some(&landing));
    }
}

/// **Cut or Copy** — `File Browser` M11 and `File Browser` M12: the chosen entries go on the
/// pasteboard, with the server row and installation they were taken under and
/// which act it was, replacing whatever was there. **Nothing is read and
/// nothing is sent** (`File Browser` M17); the folder is read again so Paste
/// shows its count. Step 10c's Files' owed acts piece 5.
#[tauri::command]
pub fn a_cut_or_copy_is_taken<R: Runtime>(
    app: AppHandle<R>,
    server: String,
    act: String,
    paths: Vec<Vec<u8>>,
) -> AnActsAnswer {
    let row = app.try_state::<Mutex<Started>>().and_then(|started| {
        started.lock().ok().and_then(|started| match what_there_is(&started, &server) {
            WhatThereIs::TheServer(row, _) => Some(*row),
            _ => None,
        })
    });
    let Some(server_row) = row
        .as_ref()
        .and_then(|row| Identity::read_back(IdentityKind::LocalServer, &row.local_server_identifier))
    else {
        return AnActsAnswer::Said {
            said: Words::plain(words::NOT_CONNECTED_FOR_AN_ACT),
        };
    };
    let installation = row
        .as_ref()
        .and_then(|row| row.last_confirmed_installation_identity.as_deref())
        .and_then(|text| Identity::read_back(IdentityKind::Installation, text));
    crate::managing_files::the_pasteboard_takes(
        &server,
        smoozy_file_browser::managing::Pasteboard {
            server_row,
            installation,
            act: if act == "copy" { CutOrCopy::Copy } else { CutOrCopy::Cut },
            entries: paths,
        },
    );
    AnActsAnswer::Done { said: None }
}

/// **`File Browser` M15: how many entries Paste carries on this row, or
/// nothing where it is not offered at all.**
fn what_paste_carries_on(row: &ServerRow) -> Option<usize> {
    let server_row = Identity::read_back(IdentityKind::LocalServer, &row.local_server_identifier)?;
    let installation = row
        .last_confirmed_installation_identity
        .as_deref()
        .and_then(|text| Identity::read_back(IdentityKind::Installation, text));
    crate::managing_files::paste_is_offered_with(&server_row, installation.as_ref())
}

/// **Paste is put on the toolbar and the folder menu, with its count, where
/// the pasteboard is this machine's** — and nowhere else is it greyed or
/// explained (`File Browser` M15).
pub fn paste_is_offered_on(browser: &mut TheBrowser, count: Option<usize>) {
    let Some(count) = count else {
        return;
    };
    browser.toolbar.paste = Some(the_paste_words(count));
    browser.menus = the_menus_with(Some(count));
}

fn the_paste_words(count: usize) -> Words {
    Words::around(
        words::PASTE,
        count.to_string(),
        if count == 1 { words::PASTE_ONE_ITEM } else { words::PASTE_ITEMS },
    )
}

/// The row the browser is on, for Paste's count.
fn the_row_of<R: Runtime>(app: &AppHandle<R>, server: &str) -> Option<ServerRow> {
    let started = app.try_state::<Mutex<Started>>()?;
    let started = started.lock().ok()?;
    match what_there_is(&started, server) {
        WhatThereIs::TheServer(row, _) => Some(*row),
        _ => None,
    }
}

/// **What a paste disturbs, before it goes** — `File Browser` W5 over the
/// sources of a cut and the folder written into, the box a move or a copy
/// takes (`File Browser` W2: read when the destination is named).
#[tauri::command]
pub fn what_a_paste_costs<R: Runtime>(app: AppHandle<R>, server: String, into: Vec<u8>) -> Option<TheLiveJobBox> {
    let (_, pasteboard) = crate::managing_files::what_the_pasteboard_holds()?;
    let holds = what_the_record_holds(&app, &server);
    let act = match pasteboard.act {
        CutOrCopy::Cut => "move",
        CutOrCopy::Copy => "copy",
    };
    the_live_job_box_over_the_record(act, &pasteboard.entries, Some(&into), holds.as_ref()).0
}

/// **Paste** — `File Browser` M5, `File Browser` M8, `File Browser` M14 and `File Browser` M15. `answer` is the box's,
/// where it was asked; the plan is read again on the machine either way, at
/// the moment of the act (`File Browser` M8). Step 10c's Files' owed acts
/// piece 5: a paste after a cut. A paste after a copy is piece 6's.
#[tauri::command]
pub async fn a_paste_is_made<R: Runtime>(
    app: AppHandle<R>,
    server: String,
    into: Vec<u8>,
    answer: Option<String>,
) -> AnActsAnswer {
    let (row, point, _) = match what_an_act_runs_on(&app, &server) {
        Ok(held) => held,
        Err(said) => return said,
    };
    let Some((_, pasteboard)) = crate::managing_files::what_the_pasteboard_holds()
        .filter(|(on, _)| *on == server)
        .filter(|_| what_paste_carries_on(&row).is_some())
    else {
        return AnActsAnswer::Said {
            said: Words::plain(words::SELECT_TO_PASTE),
        };
    };
    if pasteboard.act == CutOrCopy::Copy {
        return a_paste_after_a_copy(&app, &server, &row, &point, &pasteboard, &into, answer).await;
    }
    let answer = the_answer_from(answer.as_deref());
    let holds = what_the_record_holds(&app, &server);
    let (_, named) =
        the_live_job_box_over_the_record("move", &pasteboard.entries, Some(&into), holds.as_ref());
    let run = app
        .try_state::<Mutex<Started>>()
        .and_then(|started| started.lock().ok().and_then(|started| started.run.clone()))
        .or_else(|| Identity::issue(IdentityKind::Run).ok());
    let Some(run) = run else {
        return AnActsAnswer::Said {
            said: Words::plain(words::NOTHING_MOVED_THE_RECORD),
        };
    };
    let door = TheApplicationsRecord { app: app.clone() };
    let marking = the_marking_door(&app, &row);
    let came_to = crate::managing_files::a_move_is_made(
        &point,
        &door,
        &marking,
        &pasteboard,
        &into,
        answer,
        &named,
        &run,
        the_package_area_to_copy_with(&row).as_deref(),
    )
    .await;
    if let crate::managing_files::WhatThePasteCameTo::Ran { done, refused, not_done, .. } = &came_to {
        the_paste_goes_in_the_diary(
            &app,
            &row,
            smoozy_file_browser::diary::FileAct::Move,
            &pasteboard.entries,
            &into,
            (*done, *refused, *not_done),
            &named,
        );
    }
    crate::managing_files::the_paste_has_come_to(&server, &pasteboard, &came_to);
    let labels = holds.as_ref().map(|holds| holds.the_labels()).unwrap_or_default();
    what_the_paste_says(&came_to, &pasteboard.entries, &into, &named, &labels, TheCollidingAct::APasteAfterACut)
}

/// **The box's answer, as the page sends it** — Replace, Keep both, or
/// anything else read as Cancel.
fn the_answer_from(answer: Option<&str>) -> Option<CollisionAnswer> {
    match answer? {
        "replace" => Some(CollisionAnswer::Replace),
        "keep-both" => Some(CollisionAnswer::KeepBoth),
        _ => Some(CollisionAnswer::Stop),
    }
}

/// **A paste after a copy** — `File Browser` M7, `File Browser` M8 and `File Browser` X9, step 10c's Files'
/// owed acts piece 6: on the machine and never across the wire, *Keep both*
/// the counted name, a copy pasted into its own folder the duplicate. The
/// line says how far it has got, file by file.
async fn a_paste_after_a_copy<R: Runtime>(
    app: &AppHandle<R>,
    server: &str,
    row: &ServerRow,
    point: &ThisServersSessions,
    pasteboard: &smoozy_file_browser::managing::Pasteboard,
    into: &[u8],
    answer: Option<String>,
) -> AnActsAnswer {
    let answer = the_answer_from(answer.as_deref());
    let holds = what_the_record_holds(app, server);
    let (_, named) = the_live_job_box_over_the_record("copy", &pasteboard.entries, Some(into), holds.as_ref());
    let Some(run) = app
        .try_state::<Mutex<Started>>()
        .and_then(|started| started.lock().ok().and_then(|started| started.run.clone()))
        .or_else(|| Identity::issue(IdentityKind::Run).ok())
    else {
        return AnActsAnswer::Said {
            said: Words::plain(words::NOTHING_COPIED_THE_RECORD),
        };
    };
    let door = TheApplicationsRecord { app: app.clone() };
    let marking = the_marking_door(app, row);
    let last_said = Mutex::new(None::<std::time::Instant>);
    let queue = app.try_state::<Arc<TheOneQueue>>().map(|queue| Arc::clone(queue.inner()));
    let moved = |done: usize, total: usize| {
        let Ok(mut last) = last_said.lock() else {
            return;
        };
        if last.is_some_and(|at| at.elapsed() < std::time::Duration::from_millis(250)) && done < total {
            return;
        }
        *last = Some(std::time::Instant::now());
        if let Some(queue) = queue.as_ref() {
            the_queue_is_told(queue, an_act_moves(server, "copy", done, total));
        }
    };
    let package_area = the_package_area_to_copy_with(row);
    let came_to = crate::managing_files::a_copy_is_made(
        point,
        &door,
        &marking,
        pasteboard,
        into,
        answer,
        &named,
        &run,
        package_area.as_deref(),
        &moved,
    )
    .await;
    if let crate::managing_files::WhatTheCopyCameTo::Ran { done, refused, not_done, .. } = &came_to {
        the_paste_goes_in_the_diary(
            app,
            row,
            smoozy_file_browser::diary::FileAct::Copy,
            &pasteboard.entries,
            into,
            (*done, *refused, *not_done),
            &named,
        );
    }
    let labels = holds.as_ref().map(|holds| holds.the_labels()).unwrap_or_default();
    what_the_copy_says(&came_to, &pasteboard.entries, into, &named, &labels)
}

/// **Where Smoozy's own program sits on this row's server**, for a copy where
/// the file service does not announce copy-data (`File Browser` M7, step
/// 10c's Files' owed acts piece 7) — **only where the row records an
/// installation this copy confirmed**, which is the same condition Paste is
/// offered under (`File Browser` M15). Nothing means Smoozy is not set up
/// there.
fn the_package_area_to_copy_with(row: &ServerRow) -> Option<String> {
    row.last_confirmed_installation_identity.as_ref()?;
    crate::setting_up::the_package_area_of(row)
}

/// **What the page is told after a paste after a copy** — `Design/File
/// Browser § What the screen says around an act`.
pub fn what_the_copy_says(
    came_to: &crate::managing_files::WhatTheCopyCameTo,
    entries: &[Vec<u8>],
    into: &[u8],
    named: &[AJobNamed],
    labels: &[(String, String)],
) -> AnActsAnswer {
    use crate::managing_files::{WhatTheActCameTo as Came, WhatTheCopyCameTo as Copy, WhatThePasteCameTo as Paste};
    match came_to {
        Copy::Refused(why) => what_the_paste_says(&Paste::Refused(*why), entries, into, named, labels, TheCollidingAct::APasteAfterACopy),
        Copy::AsksFirst(comparisons) => what_the_paste_says(
            &Paste::AsksFirst(comparisons.clone()),
            entries,
            into,
            named,
            labels,
            TheCollidingAct::APasteAfterACopy,
        ),
        Copy::Cancelled => AnActsAnswer::Done { said: None },
        Copy::TheServerCannotCopy(why) => AnActsAnswer::Said {
            said: Words::plain(match why {
                crate::managing_files::WhyTheServerCannotCopy::SmoozyIsNotSetUpThere => words::SET_UP_TO_COPY,
                crate::managing_files::WhyTheServerCannotCopy::SmoozyThereIsOlder => words::UPDATE_TO_COPY,
            }),
        },
        Copy::NothingWasCopied(why) => AnActsAnswer::Said {
            said: match why {
                Came::TheTellingWouldNotWrite => Words::plain(words::NOTHING_COPIED_THE_RECORD),
                Came::TheConnectionWent => Words::plain(words::THE_CONNECTION_WENT),
                _ => Words::plain(words::THE_SERVER_REFUSED_IT),
            },
        },
        Copy::Ran {
            done,
            refused: 0,
            not_done: 0,
            never_learned: 0,
            ..
        } if *done > 0 => AnActsAnswer::Done {
            said: Some(match entries {
                [one] if *done == 1 => Words::around(
                    words::COPIED_LEAD,
                    String::from_utf8_lossy(the_last_name(one)).into_owned(),
                    ".",
                )
                .joined(),
                _ => Words::with(words::COPIED_LEAD, done.to_string())
                    .then(if *done == 1 { words::PASTE_ONE_ITEM } else { words::PASTE_ITEMS })
                    .then(".")
                    .joined(),
            }),
        },
        Copy::Ran {
            done,
            refused,
            not_done,
            never_learned,
            ..
        } => {
            let mut said = Words::after(done.to_string(), words::COPIED_COUNT_REST)
                .and(refused.to_string())
                .then(words::REFUSED_COUNT_REST)
                .and(not_done.to_string())
                .then(words::NOT_DONE_COUNT_REST);
            if *never_learned > 0 {
                said = said.then(words::SOME_NEVER_ANSWERED);
            }
            AnActsAnswer::Said { said }
        }
    }
}

/// **What the page is told after a paste** — `Design/File Browser § What the
/// screen says around an act`.
pub fn what_the_paste_says(
    came_to: &crate::managing_files::WhatThePasteCameTo,
    entries: &[Vec<u8>],
    into: &[u8],
    named: &[AJobNamed],
    labels: &[(String, String)],
    act: TheCollidingAct,
) -> AnActsAnswer {
    use crate::managing_files::{WhatTheActCameTo as Came, WhatThePasteCameTo as Paste, WhyThePasteIsRefused};
    match came_to {
        Paste::Refused(WhyThePasteIsRefused::AFolderIntoItself) => AnActsAnswer::Said {
            said: Words::plain(words::A_FOLDER_INTO_ITSELF),
        },
        Paste::NothingToMove => AnActsAnswer::Said {
            said: Words::plain(words::NOTHING_WAS_MOVED_IT_IS_HERE),
        },
        Paste::Cancelled => AnActsAnswer::Done { said: None },
        Paste::AsksFirst(comparisons) => {
            let walk = TheWalksAnswer {
                colliding: comparisons.iter().filter(|one| one.what.is_a_collision()).count(),
                arriving_files: comparisons.len(),
                comparisons: comparisons.clone(),
                as_of: std::time::SystemTime::now(),
                readings_taken: 1,
            };
            AnActsAnswer::Asks {
                question: the_collision_question(act, &walk, into, 0, named, labels, None),
            }
        }
        Paste::NothingWasMoved(why) => AnActsAnswer::Said {
            said: match why {
                Came::TheTellingWouldNotWrite => Words::plain(words::NOTHING_MOVED_THE_RECORD),
                Came::TheConnectionWent => Words::plain(words::THE_CONNECTION_WENT),
                _ => Words::plain(words::THE_SERVER_REFUSED_IT),
            },
        },
        Paste::Ran {
            done,
            refused: 0,
            not_done: 0,
            never_learned: 0,
            ..
        } if *done > 0 => AnActsAnswer::Done {
            // ***Moved textures.*** — the name is the item moved, so the verb
            // stands alone, as *Copied* and *Deleted* do. *Moved to* is the
            // moved-browser line's, which names where the browser went; the
            // pass before 0.1.9 on Vast read *Moved to file1.txt.* And one item
            // is one item (it read *1 items*).
            said: Some(match entries {
                [one] => Words::around(
                    words::MOVED_ITEMS_LEAD,
                    String::from_utf8_lossy(the_last_name(one)).into_owned(),
                    ".",
                )
                .joined(),
                _ => Words::with(words::MOVED_ITEMS_LEAD, done.to_string())
                    .then(if *done == 1 { words::PASTE_ONE_ITEM } else { words::PASTE_ITEMS })
                    .then(".")
                    .joined(),
            }),
        },
        Paste::Ran {
            done,
            refused,
            not_done,
            never_learned,
            crossings,
            crossings_want_an_update,
            ..
        } => {
            let mut said = Words::after(done.to_string(), words::MOVED_COUNT_REST)
                .and(refused.to_string())
                .then(words::REFUSED_COUNT_REST)
                .and(not_done.to_string())
                .then(words::NOT_DONE_COUNT_REST);
            if *crossings > 0 {
                said = said
                    .and(crossings.to_string())
                    .then(words::ON_ANOTHER_DISK_REST)
                    .then(if *crossings_want_an_update {
                        words::UPDATE_TO_MOVE_BETWEEN_DISKS
                    } else {
                        words::SET_UP_TO_MOVE_BETWEEN_DISKS
                    });
            }
            if *never_learned > 0 {
                said = said.then(words::SOME_NEVER_ANSWERED);
            }
            AnActsAnswer::Said { said }
        }
    }
}

/// **`File Browser` B3's line for a move or a copy**: the entry or the folder
/// with the count, and the destination.
fn the_paste_goes_in_the_diary<R: Runtime>(
    app: &AppHandle<R>,
    row: &ServerRow,
    act: smoozy_file_browser::diary::FileAct,
    entries: &[Vec<u8>],
    into: &[u8],
    (done, refused, not_attempted): (usize, usize, usize),
    told: &[AJobNamed],
) {
    let about = match entries {
        [one] => smoozy_file_browser::diary::WhatTheActWasAbout::OneThing(String::from_utf8_lossy(one).into_owned()),
        many => smoozy_file_browser::diary::WhatTheActWasAbout::AFolderAndACount {
            folder: String::from_utf8_lossy(&crate::managing_files::the_folder_holding(&many[0])).into_owned(),
            entries: many.len(),
        },
    };
    let Some(started) = app.try_state::<Mutex<Started>>() else {
        return;
    };
    let Ok(started) = started.lock() else {
        return;
    };
    let _ = crate::managing_files::the_act_is_written_down(
        &started,
        row,
        act,
        about,
        Some(String::from_utf8_lossy(into).into_owned()),
        smoozy_file_browser::diary::TheThreeCounts {
            done,
            refused,
            not_attempted,
        },
        told,
    );
}

/// **`File Browser` B3's line for an act of one item**, where one is owed
/// ([`crate::managing_files::the_lines_counts`]), written through the
/// module's own diary under the application's lock.
fn the_act_goes_in_the_diary<R: Runtime>(
    app: &AppHandle<R>,
    row: &ServerRow,
    came_to: &crate::managing_files::WhatTheActCameTo,
    act: smoozy_file_browser::diary::FileAct,
    about: &[u8],
    told: &[AJobNamed],
) {
    let Some(counts) = crate::managing_files::the_lines_counts(came_to) else {
        return;
    };
    let Some(started) = app.try_state::<Mutex<Started>>() else {
        return;
    };
    let Ok(started) = started.lock() else {
        return;
    };
    let _ = crate::managing_files::the_act_is_written_down(
        &started,
        row,
        act,
        smoozy_file_browser::diary::WhatTheActWasAbout::OneThing(String::from_utf8_lossy(about).into_owned()),
        None,
        counts,
        told,
    );
}

// ════════════════════════════════════════════════════════════════════════════
// Reading what there is
// ════════════════════════════════════════════════════════════════════════════

/// **A read of the record answers three ways and never two** — L8a.
///
/// **The rows and the empty record are not the same answer as a read that did
/// not happen**, which is the fault this application met on 2026-09-13: a
/// failed read answered *nothing is there* and no screen said otherwise. Piece
/// 5 wrote the same shape for the server's own head and this is that shape for
/// its first tab.
pub enum WhatThereIs {
    /// The row, and the connection's observed value for it.
    TheServer(Box<ServerRow>, Option<ObservedValue>),
    /// The record read and holds no such row.
    NoSuchServer,
    /// The read did not answer.
    TheRecordWouldNotRead,
}

/// **What there is, and the *Show hidden files* choice beside it** — one read
/// of the record for both.
///
/// **`File Browser` L20: a read of the choice that does not answer is a
/// failure, never a silent *off*** (`Desktop app and delivery` L8a), so it is
/// the record not reading and the screen says so — the same answer the server
/// rows not reading gets.
fn what_there_is_and_the_choice<R: Runtime>(app: &AppHandle<R>, server: &str) -> (WhatThereIs, bool) {
    match app.try_state::<Mutex<Started>>() {
        Some(started) => started
            .lock()
            .map(|started| what_there_is_with_the_choice(&started, server))
            .unwrap_or((WhatThereIs::TheRecordWouldNotRead, false)),
        None => (WhatThereIs::TheRecordWouldNotRead, false),
    }
}

/// [`what_there_is_and_the_choice`] over the run's own [`Started`], so a test
/// can stand where the command stands.
pub fn what_there_is_with_the_choice(started: &Started, server: &str) -> (WhatThereIs, bool) {
    let there_is = what_there_is(started, server);
    let choice = started
        .record
        .as_ref()
        .map(|record| crate::record::settings::hidden_files_are_shown(record.connection()));
    match (there_is, choice) {
        (WhatThereIs::TheServer(..), None | Some(Err(_))) => (WhatThereIs::TheRecordWouldNotRead, false),
        (there_is, Some(Ok(shown))) => (there_is, shown),
        (there_is, _) => (there_is, false),
    }
}

fn what_there_is(started: &Started, server: &str) -> WhatThereIs {
    let Some(record) = started.record.as_ref() else {
        return WhatThereIs::TheRecordWouldNotRead;
    };
    let Ok(rows) = server_rows::all(record.connection()) else {
        return WhatThereIs::TheRecordWouldNotRead;
    };
    match rows
        .into_iter()
        .find(|row| row.local_server_identifier == server)
    {
        // **The observed value is `Servers`' and is read beside this, from the
        // run's periods** — [`with_what_servers_observed`]. `None` until then.
        Some(row) => WhatThereIs::TheServer(Box::new(row), None),
        None => WhatThereIs::NoSuchServer,
    }
}

/// **`Servers`' own observed value for the server, where the run holds one** —
/// `File Browser` L15 composes from it and from nothing else, and it is never
/// derived here (`Server protocol and transfers` C18). *(Until step 10c piece 4
/// this screen read none, because nothing in the application connected; step
/// 10b piece 5 made that false.)*
fn with_what_servers_observed(
    there_is: WhatThereIs,
    periods: Option<&ThePeriodsThisRun>,
    server: &str,
) -> WhatThereIs {
    match there_is {
        WhatThereIs::TheServer(row, _) => WhatThereIs::TheServer(
            row,
            periods.and_then(|periods| {
                periods
                    .the_publisher()
                    .what_is_held_about(server)
                    .map(|(observed, _)| observed)
            }),
        ),
        other => other,
    }
}

/// **This server's point, where a connection is live and lending** —
/// `Server protocol and transfers` T5. **Nothing where none is**: a period
/// between a break and a repair lends nothing, and the screen then reads as it
/// does with no period at all.
fn the_live_point_of(periods: Option<&ThePeriodsThisRun>, server: &str) -> Option<ThisServersSessions> {
    periods?
        .the_sessions_of(server)
        .filter(ThisServersSessions::a_connection_is_live)
}

// ════════════════════════════════════════════════════════════════════════════
// Reading a folder on the machine
// ════════════════════════════════════════════════════════════════════════════

/// **What one reading of a folder found.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheReadingFound {
    /// **The folder, listed whole** — `File Browser` L3: complete because the
    /// subsystem reported its end — with `File Browser` L11's available bytes
    /// where the reading could be taken.
    TheFolder {
        entries: Vec<ListedEntry>,
        free: Option<u64>,
    },
    /// **`File Browser` P2: the login may not read the folder, and its parent
    /// could be read** — so the refusal is told on the folder's own row, where
    /// Design draws it.
    ItsRowCarriesTheRefusal {
        parent: Vec<u8>,
        entries: Vec<ListedEntry>,
        free: Option<u64>,
    },
    /// **Not listed, and the words say which of `File Browser` P1's answers it
    /// met** — or that the reading did not come back whole.
    NotListed(Words),
    /// **The point lent nothing** — the connection went while the reading
    /// waited.
    NothingWasLent,
}

/// **One folder, read on the connection's reading session.**
///
/// **One reading, and everything it needs rides it**: the directory reading,
/// then the free-space figure against the same path (`File Browser` L11), or —
/// where the folder answered with a refusal — `File Browser` P3's walk and the
/// parent's listing. **The reading is `Server protocol and transfers` §1.1 item
/// 2's user-waited kind**, so it goes before anything nobody is waiting on.
pub async fn a_folder_is_read<C: AHeldConnection>(
    point: &ThisServersSessions<C>,
    folder: &[u8],
) -> WhatTheReadingFound {
    // **The reading channel takes a path as text**, and a folder whose bytes
    // are not text is not reached on it — which is said as a folder not read,
    // never read under a lossy spelling that names a different folder
    // (`File Browser` L1a).
    let Ok(path) = String::from_utf8(folder.to_vec()) else {
        return WhatTheReadingFound::NotListed(Words::plain(words::THE_FOLDER_WAS_NOT_READ));
    };
    let above = the_folder_above(folder).and_then(|above| String::from_utf8(above).ok());
    let read = point
        .a_reading(true, move |subsystem| {
            Box::pin(async move {
                let answer = TheReadingChannel::on(subsystem, THE_ONE_READING)
                    .read_a_directory(&path, THE_ONE_READING)
                    .await;
                let failure = match answer {
                    DirectoryAnswer::Entries {
                        entries,
                        the_subsystem_reported_the_end: true,
                    } => {
                        let free = free_space::read_it(subsystem, &path)
                            .await
                            .ok()
                            .map(|reading| reading.available_bytes);
                        return Ok(WhatTheReadingFound::TheFolder { entries, free });
                    }
                    // **`File Browser` L3: output arrived is never a listing
                    // complete.** A cut listing is no listing.
                    DirectoryAnswer::Entries { .. } => {
                        return Ok(WhatTheReadingFound::NotListed(Words::plain(
                            words::THE_FOLDER_WAS_NOT_READ,
                        )))
                    }
                    DirectoryAnswer::Refused(failure) => failure,
                };
                let met = WhatSmoozyMet::from(&failure);
                // **`File Browser` P2: *the machine did not answer* is not a
                // statement about the folder at all** — *"it is the
                // connection's case, `Servers`' condition covers it"* — so it
                // is told as the connection's and never as the folder's: the
                // screen says what it says where no connection is live
                // (`File Browser` L14's last sentence). **The case this was
                // first written for** was a reading the machine answered
                // oddly. **The case it now also meets** is a reading the
                // connection ended under, which the Hyperstack check before
                // 0.1.7 left standing on *Reading…* for six minutes and which
                // now ends at the break — and *Smoozy could not read this
                // folder.* would then have blamed the folder for the line.
                // Step 10c's check-before-0.1.7 fix piece 1.
                if met == WhatSmoozyMet::TheMachineDidNotAnswer {
                    return Ok(WhatTheReadingFound::NothingWasLent);
                }
                // **And neither is a reading that failed some other way** — so
                // no walk is taken and no refusal or absence is claimed.
                if !matches!(met, WhatSmoozyMet::Refused | WhatSmoozyMet::Gone) {
                    return Ok(WhatTheReadingFound::NotListed(Words::plain(
                        words::THE_FOLDER_WAS_NOT_READ,
                    )));
                }
                let walked = {
                    let mut channel = TheReadingChannel::on(subsystem, THE_ONE_READING);
                    permission::walk(&path, &met, &mut channel).await
                };
                // **The refusal is the folder's own and its parent lists**:
                // told on the folder's row, where Design draws it.
                if let (WalkOutcome::RefusedAt(at), Some(above)) = (&walked, &above) {
                    if already_exists::paths_are_equal(at.as_bytes(), path.as_bytes()) {
                        let parent = TheReadingChannel::on(subsystem, THE_ONE_READING)
                            .read_a_directory(above, THE_ONE_READING)
                            .await;
                        if let DirectoryAnswer::Entries {
                            entries,
                            the_subsystem_reported_the_end: true,
                        } = parent
                        {
                            let free = free_space::read_it(subsystem, above)
                                .await
                                .ok()
                                .map(|reading| reading.available_bytes);
                            return Ok(WhatTheReadingFound::ItsRowCarriesTheRefusal {
                                parent: above.as_bytes().to_vec(),
                                entries,
                                free,
                            });
                        }
                    }
                }
                Ok(WhatTheReadingFound::NotListed(what_the_walk_said(&walked)))
            })
        })
        .await;
    match read {
        Ok(Ok(found)) => found,
        Ok(Err(_)) => WhatTheReadingFound::NotListed(Words::plain(words::THE_FOLDER_WAS_NOT_READ)),
        Err(_) => WhatTheReadingFound::NothingWasLent,
    }
}

/// The one lease a reading here is taken on — `File Browser` L3a's lease change
/// cannot fall inside one reading, because the point runs a reading to its end
/// before a command may close the session.
const THE_ONE_READING: Lease = Lease(1);

/// **The browser, composed from what the reading found**, its rows in `order`
/// (hole 8) — the order held for the run, where a command composes it.
pub fn the_tab_after_the_reading(
    order: TheOrder,
    server: &str,
    folder: &[u8],
    chosen: Option<&[u8]>,
    found: WhatTheReadingFound,
    observed: Option<ObservedValue>,
    hidden_files_are_shown: bool,
) -> TheBrowser {
    let (standing_in, listing, free) = match found {
        WhatTheReadingFound::TheFolder { entries, free } => (
            folder.to_vec(),
            TheListing::Rows {
                rows: the_rows_of(server, folder, entries, None, order),
                taken_at: None,
                read_elevated: false,
            },
            Some(free),
        ),
        WhatTheReadingFound::ItsRowCarriesTheRefusal {
            parent,
            entries,
            free,
        } => {
            let rows = the_rows_of(server, &parent, entries, Some(folder), order);
            (
                parent,
                TheListing::Rows {
                    rows,
                    taken_at: None,
                    read_elevated: false,
                },
                Some(free),
            )
        }
        WhatTheReadingFound::NotListed(said) => {
            (folder.to_vec(), TheListing::TheFolderWasNotRead { said }, None)
        }
        WhatTheReadingFound::NothingWasLent => (
            folder.to_vec(),
            TheListing::NothingHasBeenListed {
                lead: Prose::new(words::NOTHING_LISTED_LEAD),
                rest: Prose::new(words::NOTHING_LISTED_REST),
            },
            None,
        ),
    };
    let mut browser = the_tab(server, &standing_in, chosen, listing, observed, None, hidden_files_are_shown);
    browser.headers = the_headers(order);
    // **`File Browser` L11's figure, where it was read with the listing**, and
    // said as not read where the server gave none — never nought, which would
    // be a figure.
    if let Some(free) = free {
        browser.bottom.free_space = Some(match free {
            Some(bytes) => Words::after(the_size_of(bytes), words::FREE_ON_THIS_SERVER),
            None => Words::plain(words::NO_FREE_SPACE_READING),
        });
    }
    browser
}

/// **The rows, in the order the headers chose** — by name until a header is
/// pressed, which is the header the drawing marks on (hole 8, [`TheOrder::of`]).
/// The order is a presentation of the listing and nothing is decided from it.
fn the_rows_of(
    server: &str,
    folder: &[u8],
    mut entries: Vec<ListedEntry>,
    refused: Option<&[u8]>,
    order: TheOrder,
) -> Vec<ARow> {
    entries.sort_by(|one, other| order.of(one, other));
    entries
        .iter()
        .map(|entry| {
            let mut row = a_row(folder, entry, None);
            // **`File Browser` L10: the tag this period already read, at the
            // listing's own size and time**, rides the row. A `.blend` with
            // none held has its tag read behind the rows (`File Browser` L7).
            let key = the_key_of(&row.path.bytes, entry);
            if row.is_a_blend && key.is_some() {
                match the_tag_held_for(server, &key) {
                    Some(Tag::SavedWith(version)) => {
                        row.saved_with = Some(Verbatim::new(version.as_a_tag()))
                    }
                    Some(Tag::Blank(_)) => {}
                    None => row.the_tag_follows = true,
                }
            }
            if refused.is_some_and(|refused| already_exists::paths_are_equal(&row.path.bytes, refused)) {
                row.refusal = Some(a_refusal_with_no_retry_built(TheRefusedAct::ReadingAFolder));
            }
            row
        })
        .collect()
}

// ════════════════════════════════════════════════════════════════════════════
// The `.blend` version tag — `File Browser` L7 to L10
// ════════════════════════════════════════════════════════════════════════════

/// **`File Browser` L10: the tags read in this connected period, per server**,
/// keyed by the path and the listing's own size and time.
///
/// **Process memory and durable nowhere** — the type has no write route out —
/// and **it ends with the period** ([`the_tags_period_ended`], called where
/// every other period-held thing ends). A fresh period reads afresh.
fn the_tags_this_period() -> &'static Mutex<std::collections::BTreeMap<String, TagsThisPeriod>> {
    static HELD: std::sync::OnceLock<Mutex<std::collections::BTreeMap<String, TagsThisPeriod>>> =
        std::sync::OnceLock::new();
    HELD.get_or_init(|| Mutex::new(std::collections::BTreeMap::new()))
}

/// **`File Browser` L10 and L16: the tag is period-held and ends with the
/// period** — called by `crate::servers::a_period::ThePeriodsThisRun::a_period_has_ended`.
pub fn the_tags_period_ended(server: &str) {
    if let Ok(mut held) = the_tags_this_period().lock() {
        held.remove(server);
    }
    // **`File Browser` M13: the pasteboard's life is the connected period's
    // too**, and it ends at the same moment (step 10c's Files' owed acts piece
    // 5) — the one place a period's end reaches this screen.
    crate::managing_files::the_pasteboard_goes_with_the_period(server);
}

/// The tag this period holds for a key, where it holds one.
fn the_tag_held_for(server: &str, key: &Option<TagKey>) -> Option<Tag> {
    let key = key.as_ref()?;
    the_tags_this_period()
        .lock()
        .ok()?
        .get(server)
        .and_then(|tags| tags.tag_for(key))
}

fn the_tag_is_remembered(server: &str, key: TagKey, tag: Tag) {
    if let Ok(mut held) = the_tags_this_period().lock() {
        held.entry(server.to_string()).or_default().remember(key, tag);
    }
}

/// **`File Browser` L10's key for a listed entry.** The reading channel takes a
/// path as text, so a path whose bytes are not text has no key and no tag —
/// never one read under a lossy spelling that names a different file
/// (`File Browser` L1a).
fn the_key_of(path: &[u8], entry: &ListedEntry) -> Option<TagKey> {
    Some(TagKey {
        path: String::from_utf8(path.to_vec()).ok()?,
        size: entry.size,
        last_changed: entry.last_changed,
    })
}

/// **The saved-with version of one `.blend` on the machine** — `File Browser`
/// L7 to L10, the reading Row 345's tag shows and `Render Settings` B2's
/// automatic match is made on. `None` is the blank tag, and blank is never a
/// guess (`File Browser` L8).
///
/// **One reading on the connection's reading session**, lent by the server's
/// point (`Server protocol and transfers` T5), so it rides the reading channel
/// behind the listing and opens no channel of its own (`File Browser` L7).
/// Inside it: the file's own size and time, the tag this period already holds
/// for them (`File Browser` L10), and only where none is held, the bounded
/// prefix read and Smoozy's own interpretation of it (`File Browser` L8,
/// `File Browser` L9). **No Blender runs and nothing opens the file for one.**
///
/// **`someone_is_waiting` is `Server protocol and transfers` §1.1 item 2's
/// ordering**: the tags that fill in behind a listing are read with nobody
/// waiting, so a folder the user opens goes first; Render Settings and the
/// dependency check, which wait on the answer, read with someone waiting.
///
/// **The size and time are the link's own** (the listing's reading), so a key
/// taken here meets the key [`the_rows_of`] composes from the listing.
///
/// *Added 2026-10-01, step 10c's twelfth-pass fix piece 6b.* **The case
/// `File Browser`'s reading was first written for** is the listing's tag, read
/// where it was awaited; **the case it now also meets** is the application's
/// own three callers — the listing, Render Settings and the dependency check —
/// none of which had ever called it, so every file needed a Blender picked by
/// hand.
pub async fn the_saved_with_version_of<C: AHeldConnection>(
    point: &ThisServersSessions<C>,
    server: &str,
    path: &[u8],
    someone_is_waiting: bool,
) -> Option<String> {
    let text = String::from_utf8(path.to_vec()).ok()?;
    let server = server.to_string();
    let read = point
        .a_reading(someone_is_waiting, move |subsystem| {
            Box::pin(async move {
                let attributes = subsystem.lstat(text.as_bytes()).await?;
                let key = TagKey {
                    path: text.clone(),
                    size: ApparentSize::of(attributes.size.unwrap_or(0)),
                    last_changed: attributes
                        .modification_time
                        .map(|seconds| LastChanged::at_epoch_seconds(i64::from(seconds))),
                };
                if let Some(tag) = the_tag_held_for(&server, &Some(key.clone())) {
                    return Ok(tag);
                }
                let mut channel = TheReadingChannel::on(subsystem, THE_ONE_READING);
                let tag = read_the_tag(
                    &text,
                    &mut channel,
                    &mut TheZstandardDecoder,
                    PrefixGrowth::DECLARED,
                )
                .await;
                the_tag_is_remembered(&server, key, tag);
                Ok(tag)
            })
        })
        .await;
    match read {
        Ok(Ok(Tag::SavedWith(version))) => Some(version.as_a_tag()),
        _ => None,
    }
}

/// **One `.blend`'s tag, asked for by the page after the rows are drawn** —
/// `File Browser` L7's *the four fields show first and the version tag fills in
/// as each prefix arrives behind them*. **The page asks one file at a time**, so
/// each reading is its own place in the point's order and a folder the user
/// opens is never queued behind a whole folder's tags.
///
/// **`async`, so it waits on the machine off the drawing thread** (P5). `None`
/// where no connection is live, the path is not text, or the tag is blank.
#[tauri::command]
pub async fn the_saved_with_version_is_read<R: Runtime>(
    app: AppHandle<R>,
    server: String,
    path: Vec<u8>,
) -> Option<String> {
    let periods = app
        .try_state::<Arc<ThePeriodsThisRun>>()
        .map(|periods| Arc::clone(periods.inner()));
    let point = the_live_point_of(periods.as_deref(), &server)?;
    the_saved_with_version_of(&point, &server, &path, false).await
}

// ════════════════════════════════════════════════════════════════════════════
// A picture looked at
// ════════════════════════════════════════════════════════════════════════════

/// **Which read the viewer is waiting on** — `File Browser` L29: *"Closing the
/// viewer, or stepping to another entry, cancels a read in flight and frees
/// its channel. No read ahead."* **Every read takes the next number and every
/// close takes one too**, so a read whose number is no longer the one held has
/// been left, and it stops before its next piece. One viewer opens at a time,
/// so one number serves every server.
static THE_PICTURE_ON_SCREEN: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// **One picture read off the machine to be shown** — `File Browser` L29.
///
/// In this order: the listed size against the cap, and nothing is asked of
/// anyone above it; **admission at the run's one desktop point in the download
/// direction, under the picture view's own class**
/// ([`smoozy_file_browser::declarations::THE_PICTURE_VIEW_CLASS`], third in
/// `File Browser` O4's order — `crate::downloading::the_declared_order`); a
/// transfer channel of its own from the server's point (`Server protocol and
/// transfers` T5), never the reading session the listing and the version tags
/// ride; and `File Browser`'s own read on it. **Each piece draws its bytes from
/// the download direction's budget before it is asked for** (`Desktop app and
/// delivery` R2), and each answer is bounded by `Server protocol and
/// transfers` C19's deadline, so a line that goes quiet ends the read rather
/// than holding the viewer on *Loading…*.
///
/// **Memory only**: the bytes come back and nothing is written — no transfer
/// record, no Transfers row, nothing in the local record, and Download is not
/// asked. **No machine-side program is asked**, so it works on a server not set
/// up yet. **The channel and the draw are let go at this call's end**,
/// whatever happened, which is what frees them when the viewer moves on.
pub async fn the_picture_of<C: AHeldConnection>(
    point: &ThisServersSessions<C>,
    the_run: &TheRunsTransfers,
    path: &[u8],
    listed: u64,
    still_wanted: &(dyn Fn() -> bool + Sync),
) -> smoozy_file_browser::picture::ThePictureRead {
    use smoozy_file_browser::picture::{the_listed_size_is_over_the_cap, ThePictureRead, WhyItWasNotRead};
    use smoozy_server_protocol::transfers::Direction;
    if the_listed_size_is_over_the_cap(listed) {
        return ThePictureRead::TooLarge;
    }
    let class = smoozy_file_browser::declarations::THE_PICTURE_VIEW_CLASS;
    let mut draw = loop {
        if !still_wanted() {
            return ThePictureRead::LetGo;
        }
        match the_run.admit_to_carry(Direction::Down, &[class], class) {
            Ok(draw) => break draw,
            Err(_) if point.a_connection_is_live() => {
                tokio::time::sleep(crate::uploading::THE_PAUSE_BEFORE_ASKING_AGAIN).await;
            }
            Err(_) => return ThePictureRead::NotRead(WhyItWasNotRead::TheLineWent),
        }
    };
    let Ok(channel) = point.a_transfer_channel().await else {
        return ThePictureRead::NotRead(WhyItWasNotRead::TheLineWent);
    };
    let negotiated = tokio::time::timeout(
        crate::downloading::THE_INACTIVITY_DEADLINE,
        smoozy_server_protocol::subsystem::client::Subsystem::negotiate(channel),
    )
    .await;
    let Ok(Ok(mut subsystem)) = negotiated else {
        return ThePictureRead::NotRead(WhyItWasNotRead::TheLineWent);
    };
    let read = smoozy_file_browser::picture::read_the_picture(
        path,
        listed,
        &mut ThePacedRead {
            subsystem: &mut subsystem,
            draw: &mut draw,
            still_wanted,
        },
        still_wanted,
    )
    .await;
    draw.the_channel_ended(std::time::Instant::now());
    read
}

/// **`File Browser`'s read, over a lent transfer channel, paced by the run's
/// budget** — `Desktop app and delivery` R2: a channel's byte loop draws from
/// the direction's shared budget, and the draw's floor and the cap the user
/// set hold for a picture as for a download.
struct ThePacedRead<'a, S> {
    subsystem: &'a mut smoozy_server_protocol::subsystem::client::Subsystem<S>,
    draw: &'a mut crate::scheduling::TheChannelsDraw,
    still_wanted: &'a (dyn Fn() -> bool + Sync),
}

impl<S> smoozy_file_browser::picture::ReadsAFileInPieces for ThePacedRead<'_, S>
where
    S: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin + Send,
{
    type Handle = smoozy_server_protocol::subsystem::Handle;

    async fn open_the_file(&mut self, path: &[u8]) -> Result<Self::Handle, smoozy_file_browser::picture::WhyItWasNotRead> {
        use smoozy_file_browser::picture::WhyItWasNotRead;
        tokio::time::timeout(crate::downloading::THE_INACTIVITY_DEADLINE, self.subsystem.open_the_file(path))
            .await
            .unwrap_or(Err(WhyItWasNotRead::TheLineWent))
    }

    /// **The piece's bytes are drawn before it is asked for**, the whole of
    /// them, so a capped line paces the read and never cuts a piece short.
    /// A viewer left while it waits stops waiting.
    async fn send_a_piece(
        &mut self,
        handle: &Self::Handle,
        at: u64,
        want: usize,
    ) -> Result<u32, smoozy_file_browser::picture::WhyItWasNotRead> {
        let mut drawn = 0;
        while drawn < want {
            if !(self.still_wanted)() {
                break;
            }
            let got = self.draw.draw_at(want - drawn, std::time::Instant::now());
            drawn += got;
            if got == 0 {
                tokio::time::sleep(crate::uploading::THE_PAUSE_BEFORE_ASKING_AGAIN).await;
            }
        }
        self.subsystem.send_a_piece(handle, at, want).await
    }

    async fn the_piece(&mut self, ticket: u32) -> Result<Vec<u8>, smoozy_file_browser::picture::WhyItWasNotRead> {
        use smoozy_file_browser::picture::WhyItWasNotRead;
        tokio::time::timeout(crate::downloading::THE_INACTIVITY_DEADLINE, self.subsystem.the_piece(ticket))
            .await
            .unwrap_or(Err(WhyItWasNotRead::TheLineWent))
    }

    async fn close_the_file(&mut self, handle: &Self::Handle) {
        let _ = tokio::time::timeout(crate::downloading::THE_INACTIVITY_DEADLINE, self.subsystem.close_the_file(handle)).await;
    }

    fn the_piece_size(&self) -> usize {
        smoozy_file_browser::picture::ReadsAFileInPieces::the_piece_size(&*self.subsystem)
    }
}

/// **What the read came to, for the page** — the bytes, or the line where the
/// picture would be, in `Design/File Browser § Looking at a picture`'s words.
pub fn what_the_picture_read_came_to(
    read: smoozy_file_browser::picture::ThePictureRead,
) -> Result<Vec<u8>, ThePictureIsNotShown> {
    use smoozy_file_browser::picture::{ThePictureRead, WhyItWasNotRead};
    match read {
        ThePictureRead::TheBytes { bytes, .. } => Ok(bytes),
        ThePictureRead::TooLarge => Err(ThePictureIsNotShown::NotShown {
            said: Words::plain(words::TOO_LARGE_TO_SHOW),
        }),
        ThePictureRead::CannotBeShown => Err(ThePictureIsNotShown::NotShown {
            said: Words::plain(words::COULD_NOT_SHOW_THIS_FILE),
        }),
        ThePictureRead::NotRead(why) => Err(ThePictureIsNotShown::NotRead {
            said: Words::plain(words::COULD_NOT_READ_THE_FILE),
            why: Words::plain(match why {
                WhyItWasNotRead::TheLoginMayNot => words::COULD_NOT_SAY_WHERE_REFUSED,
                WhyItWasNotRead::ItIsNotThere => words::FILE_NOT_FOUND,
                WhyItWasNotRead::TheLineWent => words::THE_CONNECTION_WENT,
                WhyItWasNotRead::ItRefused => words::THE_SERVER_REFUSED_IT,
            }),
            retry: Prose::new(words::RETRY),
        }),
        ThePictureRead::LetGo => Err(ThePictureIsNotShown::LetGo),
    }
}

/// **A picture asked for by the viewer** — a double-click or Return on a
/// picture's row, a step to one, or *Retry*. `listed` is the row's own size,
/// L1's listed size, which the cap is checked on first (`File Browser` L29).
///
/// **While no connection is live the act is unavailable** (`File Browser`
/// L29, L15): *Not connected.*, and nothing is asked. **The bytes cross as
/// themselves** (`Desktop app and delivery` H32's typed service, as Queue's
/// picture does), and the page makes a `blob:` address of them that it
/// revokes when it lets the picture go.
#[tauri::command]
pub async fn a_picture_is_read<R: Runtime>(
    app: AppHandle<R>,
    server: String,
    path: Vec<u8>,
    listed: u64,
) -> Result<tauri::ipc::Response, ThePictureIsNotShown> {
    use std::sync::atomic::Ordering;
    let mine = THE_PICTURE_ON_SCREEN.fetch_add(1, Ordering::SeqCst) + 1;
    let still_wanted = move || THE_PICTURE_ON_SCREEN.load(Ordering::SeqCst) == mine;
    let periods = app
        .try_state::<Arc<ThePeriodsThisRun>>()
        .map(|periods| Arc::clone(periods.inner()));
    let not_connected = || ThePictureIsNotShown::NotShown { said: Words::plain(words::THE_VIEWER_IS_NOT_CONNECTED) };
    let Some(point) = the_live_point_of(periods.as_deref(), &server) else {
        return Err(not_connected());
    };
    let Some(the_run) = app.try_state::<Arc<TheRunsTransfers>>().map(|run| Arc::clone(run.inner())) else {
        return Err(not_connected());
    };
    let read = the_picture_of(&point, &the_run, &path, listed, &still_wanted).await;
    what_the_picture_read_came_to(read).map(tauri::ipc::Response::new)
}

/// **The viewer closed, or stepped to a row that reads nothing** — the read
/// in flight stops before its next piece and gives its channel back
/// (`File Browser` L29).
#[tauri::command]
pub fn the_picture_is_let_go() {
    THE_PICTURE_ON_SCREEN.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
}

/// **What the user calls this server** — its label, or `username@host` where
/// it has none, the same naming the rail and the head take
/// ([`crate::surface::servers::what_a_server_is_called`]). **Read the way
/// [`what_there_is`] reads the row**, and `None` where the record would not read
/// or holds no such server.
///
/// **The name is what a person reads and the identifier is only ever a key** —
/// must-do 5 of the step 10a round 1 audit of these pieces, which found three
/// places on the render settings and the check drawing the local identifier as
/// though it were the server's name. `Render Settings`' own declaration puts
/// the label beside the identifier and never in its place.
pub(crate) fn what_the_server_is_called(started: &Started, server: &str) -> Option<String> {
    match what_there_is(started, server) {
        WhatThereIs::TheServer(row, _) => Some(crate::surface::servers::what_a_server_is_called(&row)),
        WhatThereIs::NoSuchServer | WhatThereIs::TheRecordWouldNotRead => None,
    }
}

fn the_delete_warning_is_switched_off(started: &Started) -> bool {
    started
        .record
        .as_ref()
        .map(|record| {
            crate::record::suppressed_warnings::it_is_switched_off(
                record.connection(),
                the_delete_warnings_suppression_row(),
            )
        })
        .and_then(Result::ok)
        .unwrap_or(false)
}

// ════════════════════════════════════════════════════════════════════════════
// Composing the tab
// ════════════════════════════════════════════════════════════════════════════

/// The answer itself, with what there is handed in, **so the whole of it can be
/// tested** — the shell's own managed state has no constructor outside a
/// running application.
///
/// **`a_connection_is_live` is whether the server's point is lending**, and
/// where it is the folder is *being read* and the reading follows.
pub fn the_tab_from(
    there_is: WhatThereIs,
    server: &str,
    folder: Option<Vec<u8>>,
    chosen: Option<Vec<u8>>,
    a_connection_is_live: bool,
    hidden_files_are_shown: bool,
) -> TheBrowser {
    let folder = folder.unwrap_or_else(|| b"/".to_vec());
    let mut follows = false;
    let listing = match &there_is {
        WhatThereIs::TheRecordWouldNotRead => TheListing::TheReadDidNotAnswer {
            said: Prose::new(words::THE_RECORD_WOULD_NOT_READ),
        },
        WhatThereIs::NoSuchServer => TheListing::NoSuchServer {
            said: Prose::new(words::NO_SUCH_SERVER),
        },
        // **A live connection: the folder is being read**, and the rows come
        // from [`the_folder_is_read`].
        WhatThereIs::TheServer(_, _) if a_connection_is_live => {
            follows = true;
            a_folder_being_read(&folder)
        }
        // **`File Browser` L14's last sentence**: no connection is live, so no
        // entry was made in this run, and none is kept across a drop — see
        // this file's head for why `File Browser` L15's stale listing is owed.
        WhatThereIs::TheServer(_, _) => TheListing::NothingHasBeenListed {
            lead: Prose::new(words::NOTHING_LISTED_LEAD),
            rest: Prose::new(words::NOTHING_LISTED_REST),
        },
    };
    let observed = match &there_is {
        WhatThereIs::TheServer(_, observed) => *observed,
        _ => None,
    };
    let mut browser = the_tab(server, &folder, chosen.as_deref(), listing, observed, None, hidden_files_are_shown);
    browser.a_reading_follows = follows;
    browser
}

/// **The whole composition, with the listing handed in.**
///
/// It takes the listing rather than reading one because there is no route to a
/// listing in this application and because every drawing path has to be
/// testable against one all the same.
pub fn the_tab(
    server: &str,
    folder: &[u8],
    chosen: Option<&[u8]>,
    listing: TheListing,
    observed: Option<ObservedValue>,
    bar: Option<ABarAcrossTheTop>,
    hidden_files_are_shown: bool,
) -> TheBrowser {
    // **`File Browser` L15's composition, read off `Servers`' own observed value and
    // nothing
    // else.** No value at all is not *connected*: the browser composes as
    // though the line were down, which is what keeps every machine-needing act
    // unavailable rather than failing on a click.
    let composing = observed
        .map(how_the_browser_composes)
        .unwrap_or(HowTheBrowserComposes::FromTheCacheAndStale);

    let rows = match &listing {
        TheListing::Rows { rows, .. } => rows.as_slice(),
        _ => &[],
    };
    let there_is_a_listing = matches!(listing, TheListing::Rows { .. });

    // **`File Browser` L22: a name the list does not show is never chosen**,
    // so no act is aimed at it — the page let go of it at the untick, and a
    // name handed back from before is not taken up here either.
    let chosen_row = chosen.and_then(|name| {
        rows.iter()
            .filter(|row| the_list_shows_the_row(row, hidden_files_are_shown))
            .find(|row| row.name.bytes.as_slice() == name)
    });

    TheBrowser {
        server: Verbatim::new(server.to_string()),
        folder: APath::of(folder),
        the_folder_above: the_folder_above(folder).map(|above| APath::of(&above)),
        path: the_path(folder),
        bar,
        stale: (composing == HowTheBrowserComposes::FromTheCacheAndStale && there_is_a_listing)
            .then(|| the_stale_bar(&listing)),
        toolbar: the_toolbar(chosen_row),
        bottom: the_bottom_bar(folder, rows, there_is_a_listing, composing, hidden_files_are_shown),
        menus: the_menus(),
        headers: the_headers(TheOrder::default()),
        unavailable: the_unavailable_acts(composing),
        listing,
        a_reading_follows: false,
        calculating: Words::plain(words::CALCULATING),
        viewer: the_viewers_words(),
    }
}

/// The viewer's words, composed once for every answer.
pub fn the_viewers_words() -> TheViewersWords {
    TheViewersWords {
        picture: Prose::new(words::A_PICTURE),
        loading: Words::plain(words::LOADING),
        could_not_show: Words::plain(words::COULD_NOT_SHOW_THIS_FILE),
        not_connected: Words::plain(words::THE_VIEWER_IS_NOT_CONNECTED),
        download: Prose::new(words::DOWNLOAD),
        close: Prose::new(words::CLOSE),
        previous: Prose::new(words::PREVIOUS_PICTURE),
        next: Prose::new(words::NEXT_PICTURE),
    }
}

/// **`File Browser` L15, per act, and the list is `File Browser`'s own closed one.**
/// Nothing
/// here decides which acts need the machine; [`AnActThatNeedsTheMachine::ALL`]
/// does.
fn the_unavailable_acts(composing: HowTheBrowserComposes) -> Vec<&'static str> {
    AnActThatNeedsTheMachine::ALL
        .iter()
        .filter(|act| {
            // **A folder already in the tree stays openable from the tree**,
            // and there is no tree, so the answer is the same either way here
            // and the argument is passed honestly rather than assumed.
            availability(**act, composing, false) == Availability::Unavailable
        })
        .map(|act| the_token_for(*act))
        // **A folder's count needs the machine like every act above**
        // (`File Browser` L27), and is unavailable whenever they are.
        .chain(
            AnActThatNeedsTheMachine::ALL
                .iter()
                .any(|act| availability(*act, composing, false) == Availability::Unavailable)
                .then_some(THE_CALCULATE_SIZE_ACT),
        )
        .collect()
}

/// The token the page routes on. **A name the two sides agree on, and neither
/// prose nor a value the user reads.**
fn the_token_for(act: AnActThatNeedsTheMachine) -> &'static str {
    match act {
        AnActThatNeedsTheMachine::OpeningAFolderSmoozyHasNotSeen => "open-a-folder",
        AnActThatNeedsTheMachine::Uploading => "upload",
        AnActThatNeedsTheMachine::Downloading => "download",
        AnActThatNeedsTheMachine::Deleting => "delete",
        AnActThatNeedsTheMachine::Renaming => "rename",
        AnActThatNeedsTheMachine::Moving => "move",
        AnActThatNeedsTheMachine::Copying => "copy",
        AnActThatNeedsTheMachine::MakingAFolder => "new-folder",
    }
}

/// **The folder above, composed on the bytes.**
///
/// The root has none, which is what makes *Up one folder* do nothing there
/// rather than compose a path that is not a path.
pub fn the_folder_above(folder: &[u8]) -> Option<Vec<u8>> {
    let trimmed = trim_trailing_slashes(folder);
    // **The top of the server has nothing above it**, which is what makes
    // *Up one folder* do nothing there rather than compose a path that is not
    // one.
    if trimmed.is_empty() || trimmed == b"/" {
        return None;
    }
    let at = trimmed.iter().rposition(|byte| *byte == b'/')?;
    if at == 0 {
        return Some(b"/".to_vec());
    }
    Some(trimmed[..at].to_vec())
}

fn trim_trailing_slashes(path: &[u8]) -> &[u8] {
    let mut end = path.len();
    while end > 1 && path[end - 1] == b'/' {
        end -= 1;
    }
    &path[..end]
}

/// **The path along the top, each part of it clickable.**
///
/// **Composed on the bytes and shown lossily**, which is `File Browser` L1a: the identity
/// is
/// the bytes and the decode happens only where a person reads it.
pub fn the_path(folder: &[u8]) -> Vec<APathPart> {
    let trimmed = trim_trailing_slashes(folder);
    let mut parts = vec![APathPart {
        folder: APath::of(b"/"),
        shown: Verbatim::new(String::new()),
        said: Some(Prose::new(words::THE_TOP_OF_THE_SERVER)),
        here: trimmed == b"/" || trimmed.is_empty(),
    }];
    let mut so_far: Vec<u8> = Vec::new();
    let components: Vec<&[u8]> = trimmed
        .split(|byte| *byte == b'/')
        .filter(|part| !part.is_empty())
        .collect();
    for (at, component) in components.iter().enumerate() {
        so_far.push(b'/');
        so_far.extend_from_slice(component);
        parts.push(APathPart {
            folder: APath::of(&so_far),
            shown: Verbatim::new(String::from_utf8_lossy(component).into_owned()),
            said: None,
            here: at + 1 == components.len(),
        });
    }
    parts
}

fn the_stale_bar(listing: &TheListing) -> TheStaleBar {
    let at = match listing {
        TheListing::Rows { taken_at, .. } => *taken_at,
        _ => None,
    };
    TheStaleBar {
        said: match at {
            // **The moment is the machine's own clock** (`File Browser` L4,
            // `State, identity and recovery` D5), rendered by
            // the page from the epoch value. It travels as a value.
            Some(seconds) => Words::plain(words::NOT_LIVE_LEAD)
                .and_a_moment(seconds)
                .then(words::NOT_LIVE_REST),
            None => Words::plain(words::NOT_LIVE_REST),
        },
    }
}

/// **The toolbar, and the one thing that changes it is the chosen file's
/// kind** — `File Browser` N7a, taken from that clause's own function.
fn the_toolbar(chosen: Option<&ARow>) -> TheToolbar {
    let the_blend_pair = chosen.map(|row| row.is_a_blend).unwrap_or(false);
    TheToolbar {
        the_blend_pair,
        render_settings: Prose::new(words::RENDER_SETTINGS),
        check_dependencies: Prose::new(words::CHECK_DEPENDENCIES),
        upload: Prose::new(words::UPLOAD_CONTROL),
        download: Prose::new(words::DOWNLOAD),
        new_folder: Prose::new(words::NEW_FOLDER),
        rename: Prose::new(words::RENAME),
        cut: Prose::new(words::CUT),
        copy: Prose::new(words::COPY),
        // **`File Browser` M15: paste is offered only where the pasteboard's machine is the
        // one being browsed, and nowhere else is it greyed, refused or
        // explained.** There is no pasteboard in this application — cut and
        // copy are not built — so it is never offered, which is that clause
        // kept rather than a control left out.
        paste: None,
        delete: Prose::new(words::DELETE),
        how_many_chosen: chosen.map(|_| the_note_for_one_chosen()),
        one_chosen: the_note_for_one_chosen(),
        around_an_act: the_words_around_an_act(),
    }
}

/// *"How many files are chosen is said at the far end of the toolbar"*, for
/// one — composed once, for both of [`TheToolbar`]'s fields that carry it.
fn the_note_for_one_chosen() -> Words {
    the_note_for_how_many_chosen(1).expect("one row is a choice")
}

/// **The toolbar's note for any number of rows chosen** — *3 selected*
/// (`Design/File Browser § Acting on files`, *Rows are chosen the way a file
/// manager chooses them*, the Creator's thirteenth notes, note 6). **Nothing
/// where nothing is chosen**: the note is absent then, never *0 selected*.
pub fn the_note_for_how_many_chosen(how_many: usize) -> Option<Words> {
    (how_many > 0).then(|| Words::after(how_many.to_string(), words::SELECTED))
}

/// **The note, asked for by the page when several rows are chosen.**
///
/// The choice is the page's own, held by name and drawn from the listing it
/// already holds, so a click asks nothing of the machine and does not ask for
/// the tab again (step 10c's transfer wave piece 1). **The one thing it cannot
/// write is the note**, a count beside its word, so it asks this once for each
/// count it meets and keeps the answer for the run. Added 2026-10-18, the
/// Creator's thirteenth notes, piece G.
#[tauri::command]
pub fn the_words_for_how_many_are_chosen(count: usize) -> Option<Words> {
    the_note_for_how_many_chosen(count)
}

fn the_bottom_bar(
    folder: &[u8],
    rows: &[ARow],
    there_is_a_listing: bool,
    composing: HowTheBrowserComposes,
    hidden_files_are_shown: bool,
) -> TheBottomBar {
    TheBottomBar {
        uploads_go_to: Words::with(
            words::UPLOADS_GO_TO,
            String::from_utf8_lossy(folder).into_owned(),
        ),
        what_is_in_the_folder: there_is_a_listing.then(|| the_count(rows, hidden_files_are_shown)),
        what_is_in_the_folder_when_the_box_is_turned: there_is_a_listing
            .then(|| the_count(rows, !hidden_files_are_shown)),
        show_hidden_files: TheHiddenFilesBox {
            label: Prose::new(words::SHOW_HIDDEN_FILES),
            ticked: hidden_files_are_shown,
        },
        // **`File Browser` L11's figure is a reading of the machine and there is none.** It
        // is
        // absent rather than nought: nought is a figure and would be a lie.
        free_space: (there_is_a_listing
            && composing == HowTheBrowserComposes::FromTheCacheAndStale)
            .then(|| Words::plain(words::NO_FREE_SPACE_READING)),
    }
}

/// **The count of what the list shows** — `File Browser` L21: *"The bottom
/// bar's count and 1 selected are composed from the filtered list."*
fn the_count(rows: &[ARow], hidden_files_are_shown: bool) -> Words {
    let shown: Vec<&ARow> = rows
        .iter()
        .filter(|row| the_list_shows_the_row(row, hidden_files_are_shown))
        .collect();
    let files = shown
        .iter()
        .filter(|row| !matches!(row.icon, AnIcon::Folder))
        .count();
    let folders = shown.len() - files;
    Words::after(
        files.to_string(),
        if files == 1 {
            words::ONE_FILE_IN_THE_FOLDER
        } else {
            words::FILES_IN_THE_FOLDER
        },
    )
    .and(folders.to_string())
    .then(if folders == 1 {
        words::ONE_FOLDER_IN_THE_FOLDER
    } else {
        words::FOLDERS_IN_THE_FOLDER
    })
}

/// **Whether the list shows this row** — `File Browser` L21's own filter, on
/// the row's bytes and whether a refusal arrived on it (`File Browser` L21 as
/// amended 2026-10-02: a row that carries a refusal is never left out). Nothing
/// here decides it.
fn the_list_shows_the_row(row: &ARow, hidden_files_are_shown: bool) -> bool {
    the_list_shows(
        &EntryName::from_the_machine(row.name.bytes.clone()),
        row.refusal.is_some(),
        hidden_files_are_shown,
    )
}

// ════════════════════════════════════════════════════════════════════════════
// The rows
// ════════════════════════════════════════════════════════════════════════════

/// **One listed entry, composed into a row.**
///
/// **The five icons are `Design/File Browser § The list`'s** and the kind word
/// beside them is Rule 9's. The suffix decides the icon for an image and an
/// archive because nothing else can: the listing carries a mode and a name, and
/// the mode says only file or folder.
pub fn a_row(folder: &[u8], entry: &ListedEntry, saved_with: Option<&str>) -> ARow {
    let name = entry.name.as_bytes();
    let is_a_blend = the_toolbar_for(&entry.name, entry.kind).render_settings;
    let icon = the_icon(name, entry.kind);
    // **`File Browser` X12, as amended 2026-10-02: the staging folder lists as
    // a reserved Smoozy staging folder** — which it only ever does with the box
    // ticked, its name starting with a period (`File Browser` L19).
    let kind = if entry.kind == Kind::Folder && name == THE_RESERVED_STAGING_FOLDER.as_bytes() {
        words::A_STAGING_FOLDER
    } else {
        the_kind_word(icon, entry.kind)
    };
    ARow {
        name: APath::of(name),
        path: APath::of(&landing_path(folder, std::slice::from_ref(&entry.name))),
        icon,
        kind: Prose::new(kind),
        size: (entry.kind != Kind::Folder).then(|| Verbatim::new(the_size(entry))),
        changed: entry
            .last_changed
            .map(|at| Verbatim::new(at.epoch_seconds().to_string())),
        saved_with: saved_with.map(|version| Verbatim::new(version.to_string())),
        the_tag_follows: false,
        is_a_blend,
        opens: entry.kind == Kind::Folder,
        hidden: entry.name.is_hidden(),
        refusal: None,
        editing: None,
        sorts_by: WhatTheRowSortsBy::of(entry),
        picture: (entry.kind != Kind::Folder)
            .then(|| the_picture_on(name, entry.size.bytes()))
            .flatten(),
    }
}

/// **`File Browser` L29's *which files*, and its cap on L1's listed size**,
/// for one row. **A format Smoozy does not show says so whatever its size**,
/// because the format is the reason it is not shown; a PNG or JPEG over the
/// cap is the cap's line.
pub fn the_picture_on(name: &[u8], listed: u64) -> Option<APicture> {
    use smoozy_file_browser::picture::{the_listed_size_is_over_the_cap, what_the_ending_offers, WhatTheEndingOffers};
    match what_the_ending_offers(name) {
        WhatTheEndingOffers::NotAPicture => None,
        WhatTheEndingOffers::APictureNotShown(format) => Some(APicture::NotShown {
            said: Words::around(words::CANNOT_SHOW_LEAD, format.the_name(), words::CANNOT_SHOW_REST),
        }),
        WhatTheEndingOffers::APictureToShow if the_listed_size_is_over_the_cap(listed) => {
            Some(APicture::NotShown { said: Words::plain(words::TOO_LARGE_TO_SHOW) })
        }
        WhatTheEndingOffers::APictureToShow => Some(APicture::ToBeRead),
    }
}

fn the_icon(name: &[u8], kind: Kind) -> AnIcon {
    if kind == Kind::Folder {
        return AnIcon::Folder;
    }
    let lower: Vec<u8> = name.iter().map(|byte| byte.to_ascii_lowercase()).collect();
    let ends_with = |suffix: &[u8]| lower.ends_with(suffix);
    if ends_with(b".blend") {
        return AnIcon::Blend;
    }
    for suffix in [
        b".exr".as_slice(),
        b".png",
        b".jpg",
        b".jpeg",
        b".tif",
        b".tiff",
        b".tga",
        b".hdr",
        b".webp",
    ] {
        if ends_with(suffix) {
            return AnIcon::Image;
        }
    }
    for suffix in [b".zip".as_slice(), b".7z", b".rar", b".tar", b".gz", b".bz2", b".xz"] {
        if ends_with(suffix) {
            return AnIcon::Archive;
        }
    }
    AnIcon::File
}

fn the_kind_word(icon: AnIcon, kind: Kind) -> &'static str {
    match icon {
        AnIcon::Folder => words::A_FOLDER,
        AnIcon::Blend => words::A_BLEND,
        AnIcon::Image => words::AN_IMAGE,
        AnIcon::Archive => words::AN_ARCHIVE,
        AnIcon::File => match kind {
            Kind::Link => words::A_LINK,
            _ => words::A_FILE,
        },
    }
}

/// **The apparent size, in the industry's own units** (`File Browser` L5).
///
/// **Divided by 1024 three times**, which is piece 3's own ruling for a card's
/// memory and holds here for the same reason: a machine's `df -h` writes a `G`
/// and a 24 GB card is 25,769,803,776 bytes. **Nothing decides anything from
/// this figure** — `File Browser` L5 says no will-it-fit decision is made from a listed
/// size,
/// and none is.
fn the_size(entry: &ListedEntry) -> String {
    the_size_of(entry.size.bytes())
}

/// A byte count, in the same units a row's size reads in — **so the free-space
/// figure beside the list reads the way the list does** (`File Browser` L11's
/// figure, shown the way `File Browser` L5's are).
fn the_size_of(bytes: u64) -> String {
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

/// **A refusal on a row, with the offer `File Browser` P5 composed** — five readings and
/// one
/// shape.
///
/// **Nothing here decides whether the offer exists**: [`Offer`] is
/// `File Browser`'s own answer, composed per machine from the machine kind and
/// the non-interactive probe, and this reads it.
pub fn a_refusal(what: TheRefusedAct, offer: Offer) -> ARefusal {
    let (with_a_route, with_no_route) = match what {
        TheRefusedAct::ReadingAFolder => {
            (words::CANNOT_READ_THE_FOLDER, words::CANNOT_READ_AND_NO_ROUTE)
        }
        TheRefusedAct::Deleting => (words::CANNOT_DELETE, words::CANNOT_DELETE_AND_NO_ROUTE),
        TheRefusedAct::MakingAFolder => (
            words::CANNOT_MAKE_A_FOLDER,
            words::CANNOT_MAKE_AND_NO_ROUTE,
        ),
        TheRefusedAct::Renaming => (words::CANNOT_RENAME, words::CANNOT_RENAME_AND_NO_ROUTE),
        TheRefusedAct::Moving => (words::CANNOT_MOVE, words::CANNOT_MOVE_AND_NO_ROUTE),
        TheRefusedAct::Copying => (words::CANNOT_COPY, words::CANNOT_COPY_AND_NO_ROUTE),
    };
    // **The three no-offer outcomes do not read alike, and `File Browser` P5 is why.** A
    // container's login and a machine the kind table cannot place are two
    // different reasons, and the container's sentence *would name a wrong
    // reason* on the second — so only the container's own case says that a
    // route cannot be given, and the other two say the refusal and stop.
    let (said, the_offer) = match offer {
        Offer::OfferedAndNothingIsAsked | Offer::OfferedAndAPasswordIsAsked => (
            with_a_route,
            Some(Prose::new(words::TRY_WITH_ADMIN)),
        ),
        Offer::NoOffer(WhyThereIsNoOffer::AContainersLoginDemandsAPasswordSmoozyDoesNotHold) => {
            (with_no_route, None)
        }
        Offer::NoOffer(_) => (with_a_route, None),
    };
    ARefusal {
        said: Words::plain(said),
        the_offer,
        dismiss: Prose::new(words::DISMISS),
        the_row_is_dimmed: what == TheRefusedAct::ReadingAFolder,
    }
}

/// **A refusal on a row with no offer beside it, because the retry the offer
/// would start is not built** — `File Browser` P6's elevated act, and P5's two
/// readings that decide whether it is offered.
///
/// **The sentence is the one `File Browser` P5's offering cases carry**, which
/// says the refusal and stops: it claims nothing about whether admin access
/// could be had, so it is true on every machine. **Only a container's own case
/// says that it cannot**, and nothing here knows it is one.
pub fn a_refusal_with_no_retry_built(what: TheRefusedAct) -> ARefusal {
    let mut refusal = a_refusal(what, Offer::OfferedAndNothingIsAsked);
    refusal.the_offer = None;
    refusal
}

/// Which act was refused. **The wording differs per act because the sentence
/// names what the login cannot do**, and the offer's own condition does not.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheRefusedAct {
    ReadingAFolder,
    Deleting,
    MakingAFolder,
    Renaming,
    Moving,
    Copying,
}

// ════════════════════════════════════════════════════════════════════════════
// The right-click menus
// ════════════════════════════════════════════════════════════════════════════

/// **Three menus** — `Design/File Browser § Acting on files`.
///
/// *"All three carry download, cut, copy, rename and delete. Only the folder
/// menu carries paste … Upload and new folder are on no menu at all."*
pub fn the_menus() -> Vec<AMenu> {
    the_menus_with(None)
}

/// [`the_menus`], with Paste's count on the folder menu where the pasteboard
/// is this machine's (`File Browser` M15). Step 10c's Files' owed acts piece 5.
pub fn the_menus_with(paste: Option<usize>) -> Vec<AMenu> {
    let item = |label: Words, act: &str| AMenuItem {
        label,
        act: Verbatim::new(act.to_string()),
        after_a_divider: false,
        at_the_head: false,
        in_red: false,
        greyed: None,
    };
    let after = |mut one: AMenuItem| {
        one.after_a_divider = true;
        one
    };
    let red = |mut one: AMenuItem| {
        one.in_red = true;
        one
    };
    let head = |mut one: AMenuItem| {
        one.at_the_head = true;
        one
    };

    // **Only the folder menu carries paste**, a folder being the one thing
    // clicked that a paste can go into — and **`File Browser` M15 puts a count of what is
    // on
    // the pasteboard on it**. There is no pasteboard in this application, so
    // the item is absent rather than greyed, which is `File Browser` M15's own wording:
    // *"nowhere else is it greyed, refused or explained — it is not offered at
    // all"*.
    let the_common = |paste: Option<usize>| {
        let mut items = vec![
            item(Words::plain(words::DOWNLOAD), "download"),
            item(Words::plain(words::DOWNLOAD_TO), "download-to"),
            after(item(Words::plain(words::CUT), "cut")),
            item(Words::plain(words::COPY), "copy"),
        ];
        if let Some(how_many) = paste {
            items.push(item(
                Words::around(
                    words::PASTE,
                    how_many.to_string(),
                    if how_many == 1 {
                        words::PASTE_ONE_ITEM
                    } else {
                        words::PASTE_ITEMS
                    },
                ),
                "paste",
            ));
        }
        items.push(after(item(Words::plain(words::RENAME), "rename")));
        items.push(red(after(item(Words::plain(words::DELETE), "delete"))));
        items
    };

    let mut a_blend = vec![
        head(item(Words::plain(words::RENDER_SETTINGS), "render-settings")),
        head(item(
            Words::plain(words::CHECK_DEPENDENCIES),
            "check-dependencies",
        )),
    ];
    for (at, one) in the_common(None).into_iter().enumerate() {
        let mut one = one;
        if at == 0 {
            one.after_a_divider = true;
        }
        a_blend.push(one);
    }

    // **The folder menu carries *Calculate size* in a group of its own right
    // under the two downloads** — it is not a transfer, so it does not join
    // theirs (`Design/File Browser`, the twenty-third notes, note 2).
    let mut a_folder = the_common(paste);
    a_folder.insert(2, after(item(Words::plain(words::CALCULATE_SIZE), THE_CALCULATE_SIZE_ACT)));

    vec![
        AMenu {
            which: "folder",
            items: a_folder,
        },
        AMenu {
            which: "file",
            items: the_common(None),
        },
        AMenu {
            which: "blend",
            items: a_blend,
        },
    ]
}

// ════════════════════════════════════════════════════════════════════════════
// A folder's size, counted on request — the Creator's twenty-third notes, note 2
// ════════════════════════════════════════════════════════════════════════════

/// The token the folder menu's *Calculate size* hands back.
pub const THE_CALCULATE_SIZE_ACT: &str = "calculate-size";

/// **Where the count stands on a server** — `File Browser` L28.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhereTheCountStands {
    /// The server's program counts.
    Offered,
    /// Before hand-over: there is no program to ask.
    SmoozyIsNotSetUpThere,
    /// The program's release is below the one that brings the count.
    SmoozyThereIsOlder,
    /// The program's release has not been read in this run; greyed, with
    /// nothing guessed and nothing said (as `Telegram` V1c's change is).
    NotYetKnown,
}

/// **Where the count stands on this row's server**: set up (the same
/// condition the copy on the machine asks), then the release step 9 read.
fn the_count_stands_on(row: &ServerRow, server: &str) -> WhereTheCountStands {
    if the_package_area_to_copy_with(row).is_none() {
        return WhereTheCountStands::SmoozyIsNotSetUpThere;
    }
    match crate::servers::published::a_folders_size_can_be_counted_on(server) {
        Some(true) => WhereTheCountStands::Offered,
        Some(false) => WhereTheCountStands::SmoozyThereIsOlder,
        None => WhereTheCountStands::NotYetKnown,
    }
}

/// **The folder menu's *Calculate size* is greyed where the server cannot
/// count**, saying why in Copy's words (`Design/File Browser`). Nothing — no
/// server row — greys it with nothing said.
pub fn calculate_size_is_offered_on(browser: &mut TheBrowser, stands: Option<WhereTheCountStands>) {
    let greyed = match stands {
        Some(WhereTheCountStands::Offered) => None,
        Some(WhereTheCountStands::SmoozyIsNotSetUpThere) => {
            Some(AGreying { why: Some(Words::plain(words::SET_UP_TO_CALCULATE)) })
        }
        Some(WhereTheCountStands::SmoozyThereIsOlder) => {
            Some(AGreying { why: Some(Words::plain(words::UPDATE_TO_CALCULATE)) })
        }
        Some(WhereTheCountStands::NotYetKnown) | None => Some(AGreying { why: None }),
    };
    for menu in &mut browser.menus {
        for item in &mut menu.items {
            if item.act.as_str() == THE_CALCULATE_SIZE_ACT {
                item.greyed = greyed.clone();
            }
        }
    }
}

/// **A folder's Size cell, as the page draws it** — `Design/File Browser`,
/// the Size paragraph.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct AFolderSize {
    /// The folder counted, handed back so the page fills the right row.
    pub path: APath,
    /// *3.8 GB*, *at least 41.7 GB*, or *Failed*. The figure is a value.
    pub cell: Words,
    /// The hover: *1,912 files, 140 folders* and, on *at least*, *Some
    /// folders could not be read.*; on *Failed*, the reason.
    pub hover: Vec<Words>,
    /// **What a sort by size uses**, where there is a figure.
    pub sorts_by: Option<u64>,
}

/// A count with its thousands marked, as the hover writes it: *1,204*.
fn a_count(count: u64) -> String {
    let digits = count.to_string();
    let mut out = String::with_capacity(digits.len() + digits.len() / 3);
    for (at, digit) in digits.chars().enumerate() {
        if at > 0 && (digits.len() - at) % 3 == 0 {
            out.push(',');
        }
        out.push(digit);
    }
    out
}

/// **What the cell says for one count's outcome.**
pub fn what_the_folder_size_says(path: &[u8], came_to: &crate::managing_files::WhatTheCountCameTo) -> AFolderSize {
    use crate::managing_files::WhatTheCountCameTo as Came;
    let failed = |why: Words| AFolderSize {
        path: APath::of(path),
        cell: Words::plain(words::FAILED),
        hover: vec![why],
        sorts_by: None,
    };
    match came_to {
        Came::Counted(found) => {
            let figure = the_size_of(found.bytes);
            let counted = Words::after(
                a_count(found.files),
                if found.files == 1 { words::COUNTED_ONE_FILE } else { words::COUNTED_FILES },
            )
            .and(a_count(found.folders))
            .then(if found.folders == 1 { words::COUNTED_ONE_FOLDER } else { words::COUNTED_FOLDERS });
            let mut hover = vec![counted];
            if found.incomplete {
                hover.push(Words::plain(words::SOME_FOLDERS_COULD_NOT_BE_READ));
            }
            AFolderSize {
                path: APath::of(path),
                cell: if found.incomplete {
                    Words::with(words::AT_LEAST, figure)
                } else {
                    Words::empty().and(figure)
                },
                hover,
                sorts_by: Some(found.bytes),
            }
        }
        Came::NothingThere => failed(Words::plain(words::IT_IS_NOT_THERE_ANY_MORE)),
        Came::NotAFolder => failed(Words::plain(words::NOT_A_FOLDER_ANY_MORE)),
        Came::NotRead(machine) if machine.is_empty() => failed(Words::plain(words::THE_FOLDER_COULD_NOT_BE_READ)),
        Came::NotRead(machine) => failed(Words::plain(words::THE_FOLDER_COULD_NOT_BE_READ).and(machine.clone())),
        Came::TheProgramIsOlder => failed(Words::plain(words::UPDATE_TO_CALCULATE)),
        Came::NeverAnswered => failed(Words::plain(words::NO_REPLY_TO_THE_COUNT)),
        Came::Refused => failed(Words::plain(words::THE_SERVER_REFUSED_IT)),
    }
}

/// **`File Browser` L25: one folder counted on the machine** — the page asks
/// once per chosen folder and fills each row as its own answer lands. **The
/// page holds the figure** beside the listing it drew and drops it on the
/// next reading of that folder (`File Browser` L27), so nothing is held here.
#[tauri::command]
pub async fn a_folders_size_is_counted<R: Runtime>(app: AppHandle<R>, server: String, path: Vec<u8>) -> AFolderSize {
    let (row, point, _) = match what_an_act_runs_on(&app, &server) {
        Ok(held) => held,
        Err(_) => return what_the_folder_size_says(&path, &crate::managing_files::WhatTheCountCameTo::NeverAnswered)
            .with_the_reason(Words::plain(words::NOT_CONNECTED_FOR_AN_ACT)),
    };
    let came_to = match (the_count_stands_on(&row, &server), the_package_area_to_copy_with(&row)) {
        (WhereTheCountStands::Offered | WhereTheCountStands::NotYetKnown, Some(package_area)) => {
            crate::managing_files::the_program_counts_a_folder(&point, &package_area, &path).await
        }
        (WhereTheCountStands::SmoozyIsNotSetUpThere, _) | (_, None) => {
            return what_the_folder_size_says(&path, &crate::managing_files::WhatTheCountCameTo::Refused)
                .with_the_reason(Words::plain(words::SET_UP_TO_CALCULATE));
        }
        (WhereTheCountStands::SmoozyThereIsOlder, Some(_)) => {
            crate::managing_files::WhatTheCountCameTo::TheProgramIsOlder
        }
    };
    what_the_folder_size_says(&path, &came_to)
}

impl AFolderSize {
    fn with_the_reason(mut self, why: Words) -> AFolderSize {
        self.hover = vec![why];
        self
    }
}

// ════════════════════════════════════════════════════════════════════════════
// Uploading
// ════════════════════════════════════════════════════════════════════════════

fn the_upload_ask_from(picked: Picked) -> TheUploadAsk {
    match picked {
        // **`File Browser` H4's fallback, on Windows and Linux.** `File
        // Browser` H1 and H2 answer *no* for those two, so the picker has no
        // panel for both kinds there. **A Mac is not asked**: `File Browser` H3
        // records its own panel taking both, and [`crate::picker`] opens it.
        Picked::ThereIsNoPanelForThisAsk(_) => TheUploadAsk::WhichKind {
            title: Prose::new(words::WHICH_KIND),
            said: Prose::new(words::THE_PICKER_CANNOT_TAKE_BOTH),
            files: Prose::new(words::FILES),
            folders: Prose::new(words::FOLDERS),
            cancel: Prose::new(words::CANCEL),
        },
        Picked::TheUserChoseNothing => TheUploadAsk::NothingWasChosen,
        Picked::TheUserChose(paths) => {
            let chosen: Vec<std::path::PathBuf> = paths
                .iter()
                .map(|one| one.as_the_user_gave_it().to_path_buf())
                .collect();
            the_upload_goes_with(&chosen)
        }
    }
}

/// **The upload goes, with the paths and nothing else** — `File Browser` H3a.
/// No confirmation is composed: a pick or a drop is the decision
/// (`Design/File Browser § Uploading`, struck 2026-10-04).
fn the_upload_goes_with(chosen: &[std::path::PathBuf]) -> TheUploadAsk {
    TheUploadAsk::Go {
        chosen: chosen
            .iter()
            .map(|path| APath::of(the_bytes_of(path).as_slice()))
            .collect(),
    }
}

/// **What is being dragged over the list.**
pub fn the_drop_reading_from(chosen: &[std::path::PathBuf], folder: &[u8]) -> TheDropReading {
    let folders = chosen.iter().filter(|path| path.is_dir()).count();
    let files = chosen.len() - folders;
    // **Two counts side by side, never one sentence with two numbers in it.**
    // The drawing writes *3 files and 1 folder*; the words between them are
    // prose and the figures are values.
    let mut how_many = Words::plain("");
    if files > 0 {
        how_many = how_many.and(files.to_string()).then(if files == 1 {
            words::DROPPED_ONE_FILE
        } else {
            words::DROPPED_FILES
        });
    }
    if folders > 0 {
        if files > 0 {
            how_many = how_many.then(words::AND);
        }
        how_many = how_many.and(folders.to_string()).then(if folders == 1 {
            words::DROPPED_ONE_FOLDER
        } else {
            words::DROPPED_FOLDERS
        });
    }
    TheDropReading {
        into: Words::with(
            words::DROP_TO_UPLOAD_INTO,
            String::from_utf8_lossy(folder).into_owned(),
        ),
        how_many,
    }
}

/// **A source path as stored on disk** — `File Browser` N2, and that
/// document's X1's local side.
///
/// *"Source names are read as stored on the user's disk through the platform
/// file adapter and carried onto the wire as their bytes; nothing rewrites,
/// normalises or re-encodes a name in flight."*
fn the_bytes_of(path: &std::path::Path) -> Vec<u8> {
    #[cfg(unix)]
    {
        use std::os::unix::ffi::OsStrExt;
        path.as_os_str().as_bytes().to_vec()
    }
    #[cfg(not(unix))]
    {
        // On Windows a path is not bytes and the receiving system's own path
        // unit is what crosses — `Server protocol and transfers` P7, which
        // settles the seam for the whole application and names
        // `Desktop app and delivery` C3's name check as the one place one
        // becomes the other. **Nothing here invents a second encoding.**
        path.to_string_lossy().into_owned().into_bytes()
    }
}

// ════════════════════════════════════════════════════════════════════════════
// An upload, from the confirmation to its last file
// ════════════════════════════════════════════════════════════════════════════

/// **The application's own record, as an upload reaches it** — the lock the
/// shell manages, taken for one act and let go of at once, so an upload running
/// off the drawing thread never holds the record while a file moves.
struct TheApplicationsRecord<R: Runtime> {
    app: AppHandle<R>,
}

impl<R: Runtime> TheRecordAnUploadWrites for TheApplicationsRecord<R> {
    fn with_the_record(&self, act: &mut dyn FnMut(&rusqlite::Connection)) -> bool {
        let Some(started) = self.app.try_state::<Mutex<Started>>() else {
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

    /// **An upload's row on Transfers moves as its files land** — the one event
    /// the download doors post too.
    fn a_transfer_moved(&self) {
        crate::surface::transfers::the_transfers_moved(&self.app);
    }
}

/// **`File Browser` X7a's writes, where the running application keeps what
/// they reach**, composed by that module's own
/// `publication::at_this_publication` over this crate's
/// [`crate::record::neighbouring_records::TheNeighbours`].
///
/// **What each write reaches, said plainly.** `Render Settings`' recorded
/// readings are the run's own store, `crate::surface::render_settings::the_run_holds`.
/// **`Dependency Upload & Remap`'s check standings are the local record's**,
/// read and written back by
/// [`crate::record::neighbouring_records::a_publication_through_files_is_marked`]
/// under the application's lock, and the lock on the run's readings is taken
/// after it — the order every act on these pages takes them in.
///
/// **The case this was first written for, and the case it now meets.** It was
/// written when the standings were kept nowhere in the running application, so
/// `File Browser` X7a's first write and `File Browser` W11's third found no standing and
/// marked none. **From dependency piece 7, when the band began to read the
/// record, an upload through Files over a checked root marked nothing**, and
/// the warning stayed gone over a file the check no longer described. Step
/// 10c's dependency wave piece 7b wired it, in the same act that let a
/// project uploaded through Files stop the warning at all.
///
/// **A record that will not answer marks nothing**, and the run's recorded
/// reading is still dropped (`Render Settings` J10), with no standing handed
/// in.
struct TheNeighboursOfThisMachine<R: Runtime> {
    app: AppHandle<R>,
    server_row: Identity,
    installation: Option<Identity>,
    layout: Option<smoozy_job_files::layout::TheLayout>,
}

impl<R: Runtime> WhenAFileIsPublished for TheNeighboursOfThisMachine<R> {
    fn a_file_was_published(&self, landing_path: &[u8], outcome: &UnitOutcome) {
        // **No installation, or no recorded root, and nothing can stand**: a
        // check standing is keyed to both (`Dependency Upload & Remap` V10).
        let (Some(installation), Some(layout)) = (self.installation.clone(), self.layout.as_ref())
        else {
            return;
        };
        let machine = smoozy_dependencies::record::MachineKey {
            server_row: self.server_row.clone(),
            installation,
        };
        let started = self.app.try_state::<Mutex<Started>>();
        let started = started.as_ref().and_then(|started| started.lock().ok());
        let Ok(mut held) = crate::surface::render_settings::the_run_holds().lock() else {
            return;
        };
        let marked = started.as_ref().and_then(|started| started.record.as_ref()).map(|record| {
            crate::record::neighbouring_records::a_publication_through_files_is_marked(
                record.connection(),
                &machine,
                layout,
                &mut held.readings,
                landing_path,
                outcome,
            )
        });
        if matches!(marked, Some(Ok(_))) {
            return;
        }
        let mut no_standing_is_kept = Vec::new();
        let no_record_is_kept = smoozy_dependencies::record::InMemoryRecord::default();
        let mut neighbours = crate::record::neighbouring_records::TheNeighbours {
            machine,
            standings: &mut no_standing_is_kept,
            readings: &mut held.readings,
            record: &no_record_is_kept,
            layout,
            came_up_through_files: &smoozy_dependencies::standing::nothing_came_up_through_files,
        };
        smoozy_file_browser::publication::at_this_publication(
            landing_path,
            Some(outcome),
            &mut neighbours,
        );
    }
}

/// **`File Browser` W11 and `File Browser` X7a at a unit of an act of that module's §7**,
/// onto the local record through
/// [`crate::record::neighbouring_records::a_unit_of_a_managing_act_is_marked`]
/// — the same door an upload's publication goes through, taken the same way:
/// the application's lock, then the run's readings. Step 10c's Files' owed
/// acts piece 3.
impl<R: Runtime> crate::managing_files::WhenAUnitOfAnActEnds for TheNeighboursOfThisMachine<R> {
    fn a_unit_ended(
        &self,
        location: &[u8],
        did: smoozy_file_browser::managing::WhatTheUnitDid,
        outcome: &UnitOutcome,
    ) {
        self.units_ended(&[(location.to_vec(), did, outcome.clone())]);
    }

    /// **One reading of the record and one save for the whole batch**, which
    /// is what a delete of thousands of entries hands over (step 10c's Files'
    /// owed acts piece 4).
    fn units_ended(&self, units: &[(Vec<u8>, smoozy_file_browser::managing::WhatTheUnitDid, UnitOutcome)]) {
        let (Some(installation), Some(layout)) = (self.installation.clone(), self.layout.as_ref())
        else {
            return;
        };
        let machine = smoozy_dependencies::record::MachineKey {
            server_row: self.server_row.clone(),
            installation,
        };
        let started = self.app.try_state::<Mutex<Started>>();
        let started = started.as_ref().and_then(|started| started.lock().ok());
        let Ok(mut held) = crate::surface::render_settings::the_run_holds().lock() else {
            return;
        };
        if let Some(record) = started.as_ref().and_then(|started| started.record.as_ref()) {
            let _ = crate::record::neighbouring_records::units_of_a_managing_act_are_marked(
                record.connection(),
                &machine,
                layout,
                &mut held.readings,
                units,
            );
        }
    }
}

/// **Uploads waiting on the user's answer to `File Browser` U4's question**,
/// keyed by their request's identity. Run-held, and let go of as the answer
/// arrives.
struct Held<T>(Mutex<Option<std::collections::HashMap<String, T>>>);

impl<T> Held<T> {
    const fn empty() -> Self {
        Held(Mutex::new(None))
    }

    fn insert(&self, key: String, value: T) {
        if let Ok(mut held) = self.0.lock() {
            held.get_or_insert_with(Default::default).insert(key, value);
        }
    }

    fn take(&self, key: &str) -> Option<T> {
        self.0.lock().ok()?.as_mut()?.remove(key)
    }
}

/// The server, its row, the upload, and **the jobs the question's live-job
/// warning named** — what `File Browser` W8's telling is written from if the
/// answer is *Replace*.
type AnUploadWaiting = (String, ServerRow, AnUploadReadyToRun, Vec<AJobNamed>);

fn uploads_waiting_on_their_question() -> &'static Held<AnUploadWaiting> {
    static HELD: Held<AnUploadWaiting> = Held::empty();
    &HELD
}

fn uploads_that_ended() -> &'static Held<Words> {
    static HELD: Held<Words> = Held::empty();
    &HELD
}

/// **The files go, off the drawing thread and off the command that asked**, on
/// the shell's own runtime (P5). **When the last one ends the queue is told**,
/// and the page asks [`what_the_upload_came_to`] for the words.
fn the_files_go<R: Runtime>(
    app: AppHandle<R>,
    server: String,
    row: &ServerRow,
    ready: AnUploadReadyToRun,
    point: ThisServersSessions,
) -> TheUploadGoes {
    let request = ready.request.record.as_text();
    let publishing = TheNeighboursOfThisMachine {
        app: app.clone(),
        server_row: ready.request.server_row.clone(),
        installation: ready.request.installation.clone(),
        layout: crate::servers::installation::the_layout_recorded_for(row),
    };
    let destination = ready.request.destination.clone();
    let ended = request.clone();
    tauri::async_runtime::spawn(async move {
        let the_run = match app.try_state::<Arc<TheRunsTransfers>>() {
            Some(the_run) => Arc::clone(the_run.inner()),
            None => return,
        };
        let door: Arc<dyn TheRecordAnUploadWrites> = Arc::new(TheApplicationsRecord { app: app.clone() });
        let came_to = uploading::the_upload_runs(ready, &point, &the_run, door, &publishing).await;
        uploads_that_ended().insert(ended.clone(), what_it_came_to(&came_to, &destination));
        if let Some(queue) = app.try_state::<Arc<TheOneQueue>>() {
            the_queue_is_told(
                &queue,
                Event::about(About::TheRun, THE_ANSWER)
                    .saying("what", Said::Value(Verbatim::new("the-upload-ended")))
                    .saying("server", Said::Value(Verbatim::new(server)))
                    .saying("request", Said::Value(Verbatim::new(ended))),
            );
        }
    });
    TheUploadGoes::Underway {
        request: Verbatim::new(request),
    }
}

/// **What the upload came to, counted the way `File Browser` U11 counts it** —
/// the files that landed, then what Skip existing did, then the files that
/// failed by name (`File Browser` N5), then the ones that did not finish.
fn what_it_came_to(came_to: &WhatTheUploadCameTo, destination: &[u8]) -> Words {
    let mut said = Words::empty();
    let nothing_else = came_to.skipped == 0 && came_to.failed.is_empty() && came_to.did_not_finish == 0;
    if came_to.uploaded > 0 || nothing_else {
        said = said
            .then(words::UPLOADED)
            .and(came_to.uploaded.to_string())
            .then(if came_to.uploaded == 1 { words::ONE_FILE_INTO } else { words::FILES_INTO })
            .and_a_path(String::from_utf8_lossy(destination).into_owned())
            .then(words::FULL_STOP)
            .joined();
    }
    if came_to.skipped > 0 {
        said = said
            .and(came_to.skipped.to_string())
            .then(if came_to.skipped == 1 { words::WAS_SKIPPED } else { words::WERE_SKIPPED });
    }
    if !came_to.failed.is_empty() {
        let many = came_to.failed.len();
        said = said.and(many.to_string()).then(if many == 1 {
            words::ONE_COULD_NOT_BE_UPLOADED
        } else {
            words::COULD_NOT_BE_UPLOADED
        });
        for (at, landing) in came_to.failed.iter().take(THE_NAMES_SHOWN).enumerate() {
            if at > 0 {
                said = said.then(words::A_COMMA).joined();
            }
            said = said.and(String::from_utf8_lossy(the_last_name(landing)).into_owned());
        }
        if many > THE_NAMES_SHOWN {
            said = said.then(words::AND_MORE_FILES);
        } else {
            said = said.then(words::FULL_STOP).joined();
        }
        // **`File Browser` P1 and `File Browser` P6a: a refused write is said as one** —
        // Smoozy's own sentence here, the server's words behind *What the
        // server said* on the Transfers row, where the refused file is drawn
        // (`Design/File Browser`: *a refused write on an upload surfaces on the
        // transfer row*). The sentence is Transfers' own, so the two screens
        // say one thing.
        if !came_to.refused.is_empty() {
            said = said
                .then(crate::surface::transfers::words::CANNOT_WRITE_THERE)
                .then(words::SEE_TRANSFERS);
        }
    }
    if came_to.did_not_finish > 0 {
        said = said.and(came_to.did_not_finish.to_string()).then(if came_to.did_not_finish == 1 {
            words::ONE_DID_NOT_FINISH
        } else {
            words::DID_NOT_FINISH
        });
    }
    said
}

/// **Why an upload did not start, in words** — each names what did not
/// happen, and `File Browser` X2's folder is named with P1's separation.
fn why_it_did_not_start(why: &WhyTheUploadDidNotStart) -> Words {
    match why {
        WhyTheUploadDidNotStart::SendingFilesUpIsClosed
        | WhyTheUploadDidNotStart::TheRecordWouldNotTakeIt => {
            Words::plain(words::THE_RECORD_WOULD_NOT_TAKE_IT)
        }
        WhyTheUploadDidNotStart::ThereIsNothingToSend => Words::plain(words::NOTHING_TO_UPLOAD),
        WhyTheUploadDidNotStart::SomethingPickedCouldNotBeRead(path) => Words::plain(
            words::COULD_NOT_READ_ON_THIS_COMPUTER,
        ).and_a_path(path.to_string_lossy().into_owned()),
        WhyTheUploadDidNotStart::TheServerLentNothing(_) => Words::plain(words::NOT_CONNECTED),
        WhyTheUploadDidNotStart::AFolderCouldNotBeRead { folder, met } => {
            let folder = String::from_utf8_lossy(folder).into_owned();
            let named = match met {
                WhatSmoozyMet::Refused => Words::plain(words::NOT_PERMITTED_AT_LEAD)
                    .and_a_path(folder)
                    .then(words::FULL_STOP)
                    .joined(),
                WhatSmoozyMet::Gone => Words::plain(words::IT_IS_NOT_THERE_AT_LEAD)
                    .and_a_path(folder)
                    .then(words::FULL_STOP)
                    .joined(),
                _ => Words::plain(words::THE_FOLDER_WAS_NOT_READ),
            };
            named.then(words::THE_UPLOAD_DID_NOT_START)
        }
    }
}

/// A picked path, back from the bytes the page handed over — **the bytes the
/// picker gave, and nothing rewritten** (`File Browser` N2).
#[cfg(unix)]
fn the_path_of(bytes: &[u8]) -> std::path::PathBuf {
    use std::os::unix::ffi::OsStrExt;
    std::path::PathBuf::from(std::ffi::OsStr::from_bytes(bytes))
}

#[cfg(not(unix))]
fn the_path_of(bytes: &[u8]) -> std::path::PathBuf {
    std::path::PathBuf::from(String::from_utf8_lossy(bytes).into_owned())
}

/// A desktop-clock moment, as seconds since the epoch, for the page to render.
fn seconds_since_the_epoch(at: std::time::SystemTime) -> i64 {
    match at.duration_since(std::time::UNIX_EPOCH) {
        Ok(since) => since.as_secs() as i64,
        Err(before) => -(before.duration().as_secs() as i64),
    }
}

// ════════════════════════════════════════════════════════════════════════════
// The already-exists box, in one shape and six readings
// ════════════════════════════════════════════════════════════════════════════

/// Which act met the collision. **The readings differ in the sentence and in
/// the answers, and in nothing else** — which is why there is one box.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheCollidingAct {
    AnUpload,
    ADownload,
    APasteAfterACut,
    APasteAfterACopy,
}

/// How many names the list shows before it says how many more.
const THE_NAMES_SHOWN: usize = 5;

/// **The already-exists box, in one shape.**
///
/// *"Asked once for the whole upload, leading with how many"*, the colliding
/// names beneath, one line saying what the comparison was, the two warnings
/// above the answers, and the answers the act offers.
pub fn the_collision_question(
    act: TheCollidingAct,
    answer: &TheWalksAnswer,
    landing_folder: &[u8],
    placed_by_the_check: usize,
    live_jobs: &[AJobNamed],
    the_job_labels: &[(String, String)],
    interrupted_on: Option<i64>,
) -> TheCollisionQuestion {
    let colliding = answer.colliding;
    // **An upload's and a download's total is the files arriving**, never the
    // folders among them (`Design/File Browser § When the files are already
    // there`: *of these 300 files*). A paste counts the items picked. After
    // 0.1.8, the checks' leftovers, item 5.
    let of_all = match act {
        TheCollidingAct::AnUpload | TheCollidingAct::ADownload => answer.arriving_files,
        TheCollidingAct::APasteAfterACut | TheCollidingAct::APasteAfterACopy => {
            answer.comparisons.len()
        }
    };
    let into = String::from_utf8_lossy(landing_folder).into_owned();

    // **The headline, and the words sit between the figures rather than at
    // either end of them.** *42 of these 300 files are already in `kitchen/`* —
    // a value, words, a value, words, a value.
    //
    // **The count takes its singular** (`Design/File Browser § When the files
    // are already there`, the drawing's own *42 of these 300 files are already
    // in*, and Rule 9): *1 of these 3 files is already in*, and where the one
    // is the whole act, *1 file is already in* with no *of these 1*. Step 10c's
    // eleventh pass read *1 of these 1 files are already in /home/ubuntu*.
    let one_alone = colliding == 1 && of_all == 1;
    let headline = match act {
        TheCollidingAct::AnUpload | TheCollidingAct::ADownload if one_alone => Words::empty()
            .and(colliding.to_string())
            .then(words::ALREADY_THERE_ONE_FILE_ALONE)
            .and(into.clone()),
        TheCollidingAct::AnUpload | TheCollidingAct::ADownload => Words::empty()
            .and(colliding.to_string())
            .then(words::ALREADY_THERE_MIDDLE)
            .and(of_all.to_string())
            .then(if colliding == 1 {
                words::ALREADY_THERE_UPLOAD_ONE_OF_SEVERAL
            } else {
                words::ALREADY_THERE_UPLOAD
            })
            .and(into.clone()),
        TheCollidingAct::APasteAfterACut if one_alone => Words::empty()
            .and(colliding.to_string())
            .then(words::ALREADY_THERE_MOVING_ONE_ALONE)
            .and(into.clone()),
        TheCollidingAct::APasteAfterACopy if one_alone => Words::empty()
            .and(colliding.to_string())
            .then(words::ALREADY_THERE_COPYING_ONE_ALONE)
            .and(into.clone()),
        TheCollidingAct::APasteAfterACut => Words::empty()
            .and(colliding.to_string())
            .then(words::ALREADY_THERE_MOVING_LEAD)
            .and(of_all.to_string())
            .then(if colliding == 1 {
                words::ALREADY_THERE_MOVING
            } else {
                words::ALREADY_THERE_MOVING_MANY
            })
            .and(into.clone()),
        TheCollidingAct::APasteAfterACopy => Words::empty()
            .and(colliding.to_string())
            .then(words::ALREADY_THERE_MOVING_LEAD)
            .and(of_all.to_string())
            .then(if colliding == 1 {
                words::ALREADY_THERE_COPYING
            } else {
                words::ALREADY_THERE_COPYING_MANY
            })
            .and(into.clone()),
    };

    let names: Vec<Verbatim> = answer
        .comparisons
        .iter()
        .filter(|one: &&AComparison| one.what.is_a_collision())
        .take(THE_NAMES_SHOWN)
        .map(|one| Verbatim::new(String::from_utf8_lossy(&one.landing_path).into_owned()))
        .collect();

    TheCollisionQuestion {
        headline,
        compared_by: Prose::new(words::COMPARED_BY),
        and_more: (colliding > names.len()).then(|| {
            Words::after((colliding - names.len()).to_string(), words::AND_MORE)
        }),
        names,
        // **Only an upload can meet files the dependency check put there**, so
        // the line is composed for one act and for no other.
        placed_by_the_check: (act == TheCollidingAct::AnUpload && placed_by_the_check > 0).then(
            || TheCheckLine {
                named: Words::after(
                    placed_by_the_check.to_string(),
                    if placed_by_the_check == 1 {
                        words::ONE_PLACED_BY_THE_CHECK_REST
                    } else {
                        words::PLACED_BY_THE_CHECK_REST
                    },
                ),
                said: Prose::new(words::PLACED_BY_THE_CHECK_SAID),
                offer: Prose::new(words::CHECK_THEM_INSTEAD),
            },
        ),
        // **The live-job warning is about the machine's own work**, so a
        // download onto the user's own computer never carries one.
        live_jobs: (act != TheCollidingAct::ADownload)
            .then(|| the_live_job_warning(live_jobs, the_job_labels))
            .flatten(),
        // **`File Browser` U9b's moment is the desktop's own clock, and the
        // page renders it in the user's zone** (`File Browser` L4's rule for a
        // moment, applied here). Until step 10c piece 4 it arrived as text and
        // nothing supplied one.
        // *…last file uploaded on 12 Aug. Skip existing uploads only what
        // is missing.* — the moment ends its sentence and the next stands
        // apart. The pass before 0.1.9 on Vast read *…08:31 PMSkip existing…*:
        // the second sentence was joined to the moment.
        interrupted: interrupted_on.map(|on| {
            Words::plain(words::INTERRUPTED_LEAD)
                .and_a_moment(on)
                .then(words::FULL_STOP)
                .joined()
                .then(words::INTERRUPTED_REST)
        }),
        answers: the_answers(act, interrupted_on.is_some()),
    }
}

/// **The answers the act offers, and the pasteboard has to remember which it
/// was** — `Design/File Browser § Acting on files`, and `File Browser` M8 and `File
/// Browser` M18.
fn the_answers(act: TheCollidingAct, put_skip_forward: bool) -> Vec<AnAnswer> {
    let one = |answer: &str, label: &str, put_forward: bool| AnAnswer {
        answer: Verbatim::new(answer.to_string()),
        label: Prose::new(label),
        put_forward,
    };
    match act {
        TheCollidingAct::AnUpload | TheCollidingAct::ADownload => {
            let mut answers = vec![
                one("replace", words::REPLACE, false),
                one("keep-both", words::KEEP_BOTH, false),
                one("skip-existing", words::SKIP_EXISTING, put_forward_skip(act, put_skip_forward)),
                one("stop", words::STOP, false),
            ];
            // **Where there is no record of an earlier request the four answers
            // sit level** — nothing is put forward, and least of all the one
            // that overwrites.
            if put_forward_skip(act, put_skip_forward) {
                answers.swap(0, 2);
            }
            answers
        }
        // **`File Browser` M8 and `File Browser` M18: a colliding move offers Replace or
        // Cancel; a colliding
        // copy offers Replace, Keep both or Cancel.** Taken from that clause's
        // own function rather than written out again.
        TheCollidingAct::APasteAfterACut | TheCollidingAct::APasteAfterACopy => {
            let cut_or_copy = if act == TheCollidingAct::APasteAfterACut {
                CutOrCopy::Cut
            } else {
                CutOrCopy::Copy
            };
            let mut answers = vec![one("replace", words::REPLACE, false)];
            if the_answers_offered(cut_or_copy) == TheAnswersOffered::ReplaceKeepBothOrCancel {
                answers.push(one("keep-both", words::KEEP_BOTH, false));
            }
            answers.push(one("cancel", words::CANCEL, false));
            answers
        }
    }
}

fn put_forward_skip(act: TheCollidingAct, holds_an_earlier_request: bool) -> bool {
    act == TheCollidingAct::AnUpload && holds_an_earlier_request
}

// ════════════════════════════════════════════════════════════════════════════
// The warnings
// ════════════════════════════════════════════════════════════════════════════

/// **The live-job warning, and the sentence is a *may* and never a
/// description** — `File Browser` W5a, and `Dependency Upload & Remap` V8's uniformity.
///
/// *"It names every job that holds the file, and where Smoozy's set for a job
/// is only the `.blend` and its folder it says so rather than implying it
/// checked more."*
pub fn the_live_job_warning(
    jobs: &[AJobNamed],
    labels: &[(String, String)],
) -> Option<TheLiveJobWarning> {
    if jobs.is_empty() {
        return None;
    }
    let label_of = |job: &AJobNamed| {
        labels
            .iter()
            .find(|(identity, _)| *identity == job.job.as_text())
            .map(|(_, label)| label.clone())
            .unwrap_or_else(|| job.job.as_text())
    };
    let rendering: Vec<&AJobNamed> = jobs
        .iter()
        .filter(|job| job.stands == HowTheJobStands::Rendering)
        .collect();
    let waiting: Vec<&AJobNamed> = jobs
        .iter()
        .filter(|job| job.stands != HowTheJobStands::Rendering)
        .collect();

    // **Per job the warning may differentiate, and `File Browser` W5 supplies
    // what on.** The names are values and the words between them are prose;
    // nothing is joined into anything.
    let mut said = Words::empty();
    for job in &rendering {
        said = said.and(label_of(job));
    }
    if !rendering.is_empty() {
        said = said.then(words::IS_RENDERING_WITH_THIS_FILE);
        if waiting.is_empty() {
            said = said.then(".").joined();
        }
    }
    if !waiting.is_empty() {
        if !rendering.is_empty() {
            said = said.then(",").joined().then(words::AND_ALSO);
        }
        for job in &waiting {
            said = said.and(label_of(job));
        }
        said = said.then(if waiting.len() == 1 {
            words::IS_WAITING_ON_IT
        } else {
            words::IS_WAITING_ON_THEM
        });
    }

    Some(TheLiveJobWarning {
        said,
        only_the_blend_and_its_folder: jobs
            .iter()
            .any(|job| job.the_set_was_the_widest_case)
            .then(|| Prose::new(words::ONLY_THE_BLEND_AND_ITS_FOLDER)),
        a_render_may_finish_anyway: Prose::new(words::A_RENDER_MAY_FINISH_ANYWAY),
    })
}

/// **The delete box** — `Design/File Browser § Deleting`.
///
/// *"Two warnings of different weight, in one box, and the difference is the
/// design problem."*
pub fn the_delete_box_from(
    victims: &[Vec<u8>],
    live_jobs: &[AJobNamed],
    the_job_labels: &[(String, String)],
    other_references: Option<usize>,
    staged_material: Option<usize>,
    the_permanence_warning_is_switched_off: bool,
) -> TheDeleteBox {
    let question = match victims {
        [one] => Words::around(
            words::DELETE_QUESTION_LEAD,
            String::from_utf8_lossy(the_last_name(one)).into_owned(),
            "?",
        )
        .joined(),
        many => Words::around(
            words::DELETE_QUESTION_MANY_LEAD,
            many.len().to_string(),
            words::DELETE_QUESTION_MANY_REST,
        ),
    };
    TheDeleteBox {
        question,
        live_jobs: the_live_job_warning(live_jobs, the_job_labels),
        other_references: other_references
            .filter(|count| *count > 0)
            .map(|count| AWarningLine {
                named: Words::after(
                    count.to_string(),
                    if count == 1 {
                        words::ONE_OTHER_REFERENCE_REST
                    } else {
                        words::OTHER_REFERENCES_REST
                    },
                ),
                said: Some(Prose::new(words::REFERENCES_SAID)),
            }),
        staged_material: staged_material
            .filter(|count| *count > 0)
            .map(|count| AWarningLine {
                named: Words::after(
                    count.to_string(),
                    if count == 1 {
                        words::ONE_STAGED_MATERIAL_REST
                    } else {
                        words::STAGED_MATERIAL_REST
                    },
                ),
                said: None,
            }),
        permanent: Prose::new(words::PERMANENT),
        dont_warn_again: Prose::new(words::DONT_WARN_AGAIN),
        what_it_does_not_cover: Prose::new(words::WHAT_IT_DOES_NOT_COVER),
        cancel: Prose::new(words::CANCEL),
        delete: Prose::new(words::DELETE),
        the_permanence_warning_is_switched_off,
    }
}

/// **The same box and the same words for the other three acts, the title
/// saying which.**
pub fn the_live_job_box_from(
    act: &str,
    live_jobs: &[AJobNamed],
    the_job_labels: &[(String, String)],
    victims: &[Vec<u8>],
    into: Option<&[u8]>,
) -> Option<TheLiveJobBox> {
    let warning = the_live_job_warning(live_jobs, the_job_labels)?;
    let (question, carry_on) = match act {
        // **`Design/File Browser § The live-job warning`'s own title names the
        // destination** — *Move 2 items into `assets/`?* — so it is a value
        // beside the words; where none is known the question stops without
        // *into* rather than trailing off.
        "move" | "copy" => {
            let lead = if act == "copy" { words::COPY_QUESTION_LEAD } else { words::MOVE_QUESTION_LEAD };
            let one = victims.len() == 1;
            (
                match into {
                    Some(folder) => Words::around(
                        lead,
                        victims.len().to_string(),
                        if one { words::MOVE_QUESTION_MIDDLE_ONE } else { words::MOVE_QUESTION_MIDDLE },
                    )
                    .and(String::from_utf8_lossy(folder).into_owned())
                    .then(words::THE_QUESTION_MARK)
                    .joined(),
                    None => Words::around(
                        lead,
                        victims.len().to_string(),
                        if one { words::MOVE_QUESTION_NO_DESTINATION_ONE } else { words::MOVE_QUESTION_NO_DESTINATION },
                    ),
                },
                lead,
            )
        }
        _ => (
            Words::around(
                words::RENAME_QUESTION_LEAD,
                victims
                    .first()
                    .map(|one| String::from_utf8_lossy(the_last_name(one)).into_owned())
                    .unwrap_or_default(),
                words::THE_QUESTION_MARK,
            )
            .joined(),
            words::RENAME_QUESTION_LEAD,
        ),
    };
    Some(TheLiveJobBox {
        question,
        live_jobs: warning,
        cancel: Prose::new(words::CANCEL),
        carry_on: Prose::new(carry_on),
    })
}

// ════════════════════════════════════════════════════════════════════════════
// The warnings, read from the local record
// ════════════════════════════════════════════════════════════════════════════

/// **What the local record holds for this server row**, for one question —
/// [`crate::record::neighbouring_records::WhatTheLocalRecordHolds`], keyed by
/// the row's server and its confirmed installation, over the layout the row
/// recorded. Step 10c's Files' owed acts piece 1.
///
/// **Nothing where the row has no confirmed installation or no recorded
/// layout**: nothing is placed on such a machine, so no job of Smoozy's is on
/// it and no check placed anything there — the record holds nothing a warning
/// could name. **A record that will not read answers nothing too**, and the
/// box carries what is true without it.
pub fn what_the_record_holds_for_the_row(
    connection: &rusqlite::Connection,
    row: &ServerRow,
    the_machines_reading: Option<&smoozy_queue_history::acting::adoption::TheMachinesWorkingState>,
) -> Option<crate::record::neighbouring_records::WhatTheLocalRecordHolds> {
    let server_row = Identity::read_back(IdentityKind::LocalServer, &row.local_server_identifier)?;
    let installation = row
        .last_confirmed_installation_identity
        .as_deref()
        .and_then(|text| Identity::read_back(IdentityKind::Installation, text))?;
    let layout = crate::servers::installation::the_layout_recorded_for(row)?;
    crate::record::neighbouring_records::WhatTheLocalRecordHolds::read(
        connection,
        &smoozy_dependencies::record::MachineKey::of(server_row, installation),
        layout,
        the_machines_reading,
    )
    .ok()
}

/// **The machine's last queue reading this run holds for `server`** —
/// `Queue & History` C6's period-held record, read and never changed.
pub(crate) fn the_last_queue_reading<R: Runtime>(
    app: &AppHandle<R>,
    server: &str,
) -> Option<smoozy_queue_history::acting::adoption::TheMachinesWorkingState> {
    let periods = app.try_state::<Arc<ThePeriodsThisRun>>()?;
    periods
        .the_machines_work()
        .every_server()
        .into_iter()
        .find(|(held, _)| held == server)
        .and_then(|(_, period)| period.reading.map(|reading| reading.value().clone()))
}

/// [`what_the_record_holds_for_the_row`], reached through the application —
/// the row, the record and the run's reading, each lent for this one read.
fn what_the_record_holds<R: Runtime>(
    app: &AppHandle<R>,
    server: &str,
) -> Option<crate::record::neighbouring_records::WhatTheLocalRecordHolds> {
    let reading = the_last_queue_reading(app, server);
    let started = app.try_state::<Mutex<Started>>()?;
    let started = started.lock().ok()?;
    let WhatThereIs::TheServer(row, _) = what_there_is(&started, server) else {
        return None;
    };
    what_the_record_holds_for_the_row(started.record.as_ref()?.connection(), &row, reading.as_ref())
}

/// **The two warnings an upload's already-exists question carries**, read
/// from the local record (`File Browser` W1b, W5, W2's pre-flight moment).
///
/// - **The plain-upload line** counts the landing paths that both collide and
///   class *placed* or *fixed* (`File Browser` W1a, `File Browser` W1b).
/// - **The live-job warning** names every live job whose set a colliding
///   landing path reaches — only a colliding file is written over, so only
///   those disturb anything.
///
/// The named jobs come back beside the counts, because they are what the
/// telling is written from when the user answers *Replace* (`File Browser`
/// W8). **Nothing where the record holds nothing for this machine.**
pub fn the_upload_warnings(
    walk: &TheWalksAnswer,
    holds: Option<&crate::record::neighbouring_records::WhatTheLocalRecordHolds>,
) -> (usize, Vec<AJobNamed>, Vec<(String, String)>) {
    let Some(holds) = holds else {
        return (0, Vec::new(), Vec::new());
    };
    let knows = holds.knows();
    let placed = smoozy_file_browser::warnings::the_plain_upload_warning(walk, &knows).len();
    let colliding: Vec<Vec<u8>> = walk
        .comparisons
        .iter()
        .filter(|one| one.what.is_a_collision())
        .map(|one| one.landing_path.clone())
        .collect();
    let named = smoozy_file_browser::warnings::the_live_job_warning(&colliding, &knows);
    (placed, named, holds.the_labels())
}

/// **How many other references the record shows pointing at what is being
/// deleted** — `File Browser` W1's delete warning, counted the way
/// `Design/File Browser § Disturbing a file nothing live is using` says it:
/// *3 other references point at this file*. **A reference is a fixed `.blend`
/// whose fixing points at the location, or a source a checked project placed
/// there** (`Dependency Upload & Remap` V2), at the victim or anywhere the
/// record holds under it. **A live job's own set is not counted here**: it is
/// the live-job warning's, above it in the box.
pub fn the_other_references(lines: &[smoozy_file_browser::warnings::ADeleteWarningLine]) -> usize {
    let mut seen: Vec<Vec<u8>> = Vec::new();
    let mut count = |what: &smoozy_file_browser::warnings::WhatDependedOnIt| {
        for one in what
            .fixed_copies_whose_fixing_points_at_it
            .iter()
            .chain(what.placed_from.iter())
        {
            if !seen.contains(one) {
                seen.push(one.clone());
            }
        }
    };
    for line in lines {
        count(&line.what_depended_on_it);
        for under in &line.what_is_under_it {
            count(&under.what_depended_on_it);
        }
    }
    seen.len()
}

/// **The delete box, composed over the local record** — `File Browser` W1's
/// delete line, W5's live jobs and M3's staging line, each read at the moment
/// the victims are named with no read of the machine (`File Browser` W2).
///
/// **M3's staging line reads the transfer records under the machine's current
/// installation**, through `File Browser`'s own
/// `what_a_delete_of_staging_costs`; the confirmation against the exact remote
/// object comes before the act, which is the act's.
pub fn the_delete_box_over_the_record(
    victims: &[Vec<u8>],
    holds: Option<&crate::record::neighbouring_records::WhatTheLocalRecordHolds>,
    transfers: &[smoozy_server_protocol::transfers::TransferRecord],
    the_permanence_warning_is_switched_off: bool,
) -> (TheDeleteBox, Vec<AJobNamed>) {
    let Some(holds) = holds else {
        return (
            the_delete_box_from(victims, &[], &[], None, None, the_permanence_warning_is_switched_off),
            Vec::new(),
        );
    };
    let knows = holds.knows();
    let named = smoozy_file_browser::warnings::the_live_job_warning(victims, &knows);
    let references = the_other_references(&smoozy_file_browser::warnings::the_delete_warning(victims, &knows));
    let staged: usize = victims
        .iter()
        .filter_map(|victim| {
            smoozy_file_browser::managing::what_a_delete_of_staging_costs(
                victim,
                transfers,
                Some(&holds.machine.installation),
            )
        })
        .map(|costs| costs.staged_material.len())
        .sum();
    let the_box = the_delete_box_from(
        victims,
        &named,
        &holds.the_labels(),
        Some(references),
        Some(staged),
        the_permanence_warning_is_switched_off,
    );
    (the_box, named)
}

/// **The live-job box a rename, a move or a copy takes, composed over the
/// local record** — `File Browser` W5 over the paths being disturbed: the
/// sources of a rename or a move, and **the destination of a move or a copy,
/// which is where each item lands** — the folder written into joined with
/// the item's own name. A landing inside a job's folder sits inside a member,
/// and one onto a job's file equals it, so both are named.
///
/// **The case this was first written for** took the destination as the
/// folder written into. **The case it now also meets** is a folder above a
/// job's files: it *contains* a member, so a move into it named the job and
/// marked it, though nothing of the job was touched — the pass before 0.1.9
/// on Vast, new fault 3 (`/usr/share/doc/adduser` into `/root`, beside
/// `/root/v19-tex/c18-tex.blend`).
///
/// **`paths` are the items acted on**; a copy leaves them where they are, so
/// for a copy only their landings are disturbed.
pub fn the_live_job_box_over_the_record(
    act: &str,
    paths: &[Vec<u8>],
    into: Option<&[u8]>,
    holds: Option<&crate::record::neighbouring_records::WhatTheLocalRecordHolds>,
) -> (Option<TheLiveJobBox>, Vec<AJobNamed>) {
    let Some(holds) = holds else {
        return (None, Vec::new());
    };
    let mut disturbed = if act == "copy" { Vec::new() } else { paths.to_vec() };
    if let Some(into) = into {
        disturbed.extend(paths.iter().map(|path| {
            landing_path(into, &[EntryName::from_the_machine(the_last_name(path).to_vec())])
        }));
    }
    let named = smoozy_file_browser::warnings::the_live_job_warning(&disturbed, &holds.knows());
    (
        the_live_job_box_from(act, &named, &holds.the_labels(), paths, into),
        named,
    )
}

/// The last component of a path, which is what a box names.
fn the_last_name(path: &[u8]) -> &[u8] {
    let trimmed = trim_trailing_slashes(path);
    match trimmed.iter().rposition(|byte| *byte == b'/') {
        Some(at) => &trimmed[at + 1..],
        None => trimmed,
    }
}

/// **A rename refused in place** — `File Browser` M4, and `Design/File Browser § Renaming
/// into
/// a name that is taken`.
pub fn a_rename_refused(typed: &[u8]) -> ARowInEditing {
    ARowInEditing {
        typed: Verbatim::new(String::from_utf8_lossy(typed).into_owned()),
        refused: Some(Words::plain(words::THE_NAME_IS_TAKEN)),
        the_listing_saw_it: None,
    }
}

/// **The warning that came earlier, from the listing** — a hint about what
/// Smoozy last saw, which stops nothing.
pub fn a_rename_hint(typed: &[u8]) -> ARowInEditing {
    ARowInEditing {
        typed: Verbatim::new(String::from_utf8_lossy(typed).into_owned()),
        refused: None,
        the_listing_saw_it: Some(Words::plain(words::THE_LISTING_SAW_THE_NAME)),
    }
}

/// **A folder the login is not permitted to make** — the row that was about to
/// be created stays in place with the note on it.
pub fn a_new_folder_refused(folder: &[u8], typed: &[u8], offer: Offer) -> ARow {
    ARow {
        name: APath::of(typed),
        path: APath::of(&landing_path(
            folder,
            &[EntryName::from_the_machine(typed.to_vec())],
        )),
        icon: AnIcon::Folder,
        kind: Prose::new(words::A_FOLDER),
        size: None,
        changed: None,
        saved_with: None,
        the_tag_follows: false,
        is_a_blend: false,
        opens: false,
        // **Not a listed entry** — the row the user is typing, which never
        // became one — so `File Browser` L19's filter has nothing to leave out.
        hidden: false,
        refusal: Some(a_refusal(TheRefusedAct::MakingAFolder, offer)),
        editing: Some(ARowInEditing {
            typed: Verbatim::new(String::from_utf8_lossy(typed).into_owned()),
            refused: None,
            the_listing_saw_it: None,
        }),
        // **Sorted as the folder it was about to be**, with no size and no
        // moment, since the machine never listed it.
        sorts_by: WhatTheRowSortsBy { name: typed.to_vec(), folder: true, size: 0, changed: None },
        picture: None,
    }
}

/// **The bar `Rendering`'s entry point puts across the top** — §12.
pub fn the_pick_bar(for_the_job: &str) -> ABarAcrossTheTop {
    ABarAcrossTheTop::Pick {
        said: Words::around(words::PICK_LEAD, for_the_job.to_string(), words::PICK_REST).joined(),
        way_out: Prose::new(words::CANCEL),
    }
}

/// **The bar `Queue & History`'s *show outputs* puts across the top** — §12.
pub fn the_moved_bar(to_the_server: &str, an_upload_is_in_flight: bool) -> ABarAcrossTheTop {
    ABarAcrossTheTop::Moved {
        said: Words::around(words::MOVED_LEAD, to_the_server.to_string(), words::MOVED_REST),
        still_uploading: an_upload_is_in_flight.then(|| Prose::new(words::STILL_UPLOADING)),
    }
}

/// **Which server the browser is on, and whether *show outputs* moved it
/// there** — `Design/File Browser § When another module moves the browser to a
/// different server`. Held for the run and nowhere else, as the folder being
/// browsed is (`File Browser` N1).
///
/// - **The server browsed** is the one the page last asked this screen about:
///   one machine is browsed at a time, and the browser stays on it while the
///   user is on another screen.
/// - **A move** is *show outputs* on a job whose server is not that one. A job
///   on the server already browsed is no move, and before any server has been
///   browsed in this run there is nothing to move from: Design's line is about
///   the browser *moving*, so neither puts it up.
/// - **The line stands while the browser stays at the folder the move opened**,
///   and goes for good the first time it is asked about another folder or
///   another server — Design's *"It clears when the user next changes folder
///   by hand."* A redraw after an act asks about the same folder, so it keeps
///   the line; only a press that opens somewhere else asks about another.
///
/// Step 10c's *Files on the server* follow-up.
///
/// **And whether the job *show outputs* landed for has saved no frame** —
/// `Design/File Browser`, the paragraph added 2026-10-07: where the folder is
/// not there, Files says *does not exist yet* and that no frames have been saved.
/// It is held for the same folder the same way, but **on any server, moved
/// or not**: the words are about the job, not about the move. The Creator's
/// fourth notes, piece O.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct WhereTheBrowserIs {
    browsed: Option<String>,
    moved_to: Option<(String, Vec<u8>)>,
    no_frame_saved_at: Option<(String, Vec<u8>)>,
}

impl WhereTheBrowserIs {
    /// ***Show outputs* opens this server's Files at this folder**, for a job
    /// whose record shows no frame saved, or not.
    pub fn outputs_are_shown_on(&mut self, server: &str, folder: &[u8], no_frame_is_saved: bool) {
        self.moved_to = match &self.browsed {
            Some(browsed) if browsed != server => Some((server.to_string(), folder.to_vec())),
            _ => None,
        };
        self.no_frame_saved_at = no_frame_is_saved.then(|| (server.to_string(), folder.to_vec()));
    }

    /// **The page asks about a server and a folder** — and the answer is
    /// whether the moved line stands on what it draws. **Asked about anywhere
    /// else, the job's fact goes too**, as the line does.
    pub fn the_browser_is_asked(&mut self, server: &str, folder: Option<&[u8]>) -> bool {
        self.browsed = Some(server.to_string());
        let here = |held: &Option<(String, Vec<u8>)>| {
            matches!(
                (held, folder),
                (Some((to, at)), Some(folder)) if to == server && at.as_slice() == folder
            )
        };
        let stands = here(&self.moved_to);
        if !stands {
            self.moved_to = None;
        }
        if !here(&self.no_frame_saved_at) {
            self.no_frame_saved_at = None;
        }
        stands
    }

    /// **The folder last asked about is the one *show outputs* landed on for
    /// a job that has saved no frame.** Read after
    /// [`WhereTheBrowserIs::the_browser_is_asked`].
    pub fn no_frame_is_saved_here(&self) -> bool {
        self.no_frame_saved_at.is_some()
    }
}

fn where_the_browser_is() -> &'static Mutex<WhereTheBrowserIs> {
    static HELD: std::sync::OnceLock<Mutex<WhereTheBrowserIs>> = std::sync::OnceLock::new();
    HELD.get_or_init(Default::default)
}

/// ***Show outputs* opens Files on `server` at `folder`** — told by `Queue &
/// History`'s press ([`crate::surface::the_queue`]) before the page is sent
/// there — and whether the job's record shows no frame saved.
pub fn the_browser_is_moved_to(server: &str, folder: &[u8], no_frame_is_saved: bool) {
    if let Ok(mut held) = where_the_browser_is().lock() {
        held.outputs_are_shown_on(server, folder, no_frame_is_saved);
    }
}

/// **Whether the moved line stands on this answer** — and the server and
/// folder asked about become the browser's.
pub(crate) fn the_move_stands(server: &str, folder: Option<&[u8]>) -> bool {
    where_the_browser_is()
        .lock()
        .map(|mut held| held.the_browser_is_asked(server, folder))
        .unwrap_or(false)
}

/// **Whether the folder just asked about is a job's that has saved no
/// frame** — read after [`the_move_stands`], which asks. Crate-wide so the
/// press's own test can see the fact arrive.
pub(crate) fn no_frame_is_saved_here() -> bool {
    where_the_browser_is()
        .lock()
        .map(|held| held.no_frame_is_saved_here())
        .unwrap_or(false)
}

/// **`File Browser` P3's *Not found: …*, said for a job's folder
/// that is not there yet** — `Design/File Browser § When another module
/// moves the browser`, added 2026-10-07: *Does not exist yet: /…/beauty.
/// No frames saved.* **The part named is the walk's own**, so the
/// words still name the missing part of the path (Rule 10), and nothing else
/// is opened. **Every other reading is left as it came**: a folder that is
/// there, one refused, one the walk could not place.
pub fn not_there_yet_where_no_frame_is_saved(found: WhatTheReadingFound) -> WhatTheReadingFound {
    let WhatTheReadingFound::NotListed(said) = found else {
        return found;
    };
    let part = said
        .pieces
        .get(1)
        .and_then(|piece| piece.value.as_ref())
        .map(|value| value.as_str().to_string())
        .filter(|part| said == Words::with(words::IT_IS_NOT_THERE_AT_LEAD, part.clone()));
    WhatTheReadingFound::NotListed(match part {
        Some(part) => {
            Words::around(words::IT_IS_NOT_THERE_YET_AT_LEAD, part, words::NO_FRAMES_HAVE_BEEN_SAVED).joined()
        }
        None => said,
    })
}

/// **The moved line, worded** — the server by the name the server list gives
/// it (`Design/Servers`), and the second sentence only where an upload is in
/// flight. **Nothing where the record holds no row for the server**: the
/// browser then says the server has been removed, and a line naming it would
/// name nothing.
fn the_moved_bar_on<R: Runtime>(app: &AppHandle<R>, server: &str) -> Option<ABarAcrossTheTop> {
    let started = app.try_state::<Mutex<Started>>()?;
    let started = started.lock().ok()?;
    let under_way = app
        .try_state::<Arc<TheRunsTransfers>>()
        .map(|the_run| the_run.what_is_under_way())
        .unwrap_or_default();
    the_moved_bar_from(started.record.as_ref()?.connection(), server, &under_way)
}

/// [`the_moved_bar_on`] over the record and what the run holds under way, so
/// a test can stand where the command stands.
pub fn the_moved_bar_from(
    connection: &rusqlite::Connection,
    server: &str,
    under_way: &std::collections::HashMap<String, crate::downloading::ARequestUnderWay>,
) -> Option<ABarAcrossTheTop> {
    let rows = server_rows::all(connection).ok()?;
    let row = rows.iter().find(|row| row.local_server_identifier == server)?;
    let name = crate::surface::servers::what_a_server_is_called_among(row, &rows);
    Some(the_moved_bar(&name, an_upload_is_among(connection, under_way)))
}

/// **An upload is in flight** — a request this run holds under way
/// (`File Browser` U7: a request's activity is run-held) that is an upload
/// request in the record, and that neither its server's period's end nor the
/// user's *Stop* has ended. The run holds uploads and manual downloads alike,
/// so the record says which is which.
pub fn an_upload_is_among(
    connection: &rusqlite::Connection,
    under_way: &std::collections::HashMap<String, crate::downloading::ARequestUnderWay>,
) -> bool {
    under_way
        .iter()
        .filter(|(_, request)| !request.its_period_ended && !request.the_user_stopped_it)
        .any(|(request, _)| {
            connection
                .query_row("SELECT 1 FROM upload_request WHERE record = ?1", [request], |_| Ok(()))
                .is_ok()
        })
}

/// **The bar a submit-again puts across the top** — `Rendering` A1a, whose
/// mockup state is `res-find` and is piece 8's.
pub fn the_find_bar(for_the_job: &str) -> ABarAcrossTheTop {
    ABarAcrossTheTop::Find {
        said: Words::around(words::FIND_LEAD, for_the_job.to_string(), words::FIND_REST),
        way_out: Prose::new(words::CANCEL),
    }
}

/// **A folder being read** — the one reading that covers the list and nothing
/// else on the screen.
pub fn a_folder_being_read(folder: &[u8]) -> TheListing {
    TheListing::BeingRead {
        said: Words::around(
            words::READING_LEAD,
            String::from_utf8_lossy(folder).into_owned(),
            words::READING_REST,
        ),
    }
}

/// **`File Browser` P3's and `File Browser` P4's own answers, worded.**
///
/// *"The component that answered permission denied, named as the component it
/// is"*, and where the walk cannot name one, *"the message says which of the
/// two it met and stops there"*.
pub fn what_the_walk_said(outcome: &smoozy_file_browser::permission::WalkOutcome) -> Words {
    use smoozy_file_browser::permission::{WalkOutcome, WhatSmoozyMet};
    match outcome {
        WalkOutcome::RefusedAt(component) => {
            Words::with(words::NOT_PERMITTED_AT_LEAD, component.clone())
        }
        WalkOutcome::GoneAt(component) => {
            Words::with(words::IT_IS_NOT_THERE_AT_LEAD, component.clone())
        }
        WalkOutcome::CouldNotSayWhere(WhatSmoozyMet::Gone) => {
            Words::plain(words::COULD_NOT_SAY_WHERE_GONE)
        }
        WalkOutcome::CouldNotSayWhere(_) => Words::plain(words::COULD_NOT_SAY_WHERE_REFUSED),
    }
}

/// **`File Browser` N2's landing path, exposed so the confirmation and the box name the
/// same
/// thing the transfer will.** Composed by `File Browser`'s own function.
pub fn where_it_lands(destination: &[u8], relative: &[EntryName]) -> Vec<u8> {
    landing_path(destination, relative)
}

/// A collision, for the composition above, made from the parts the walk gives.
pub fn a_collision(landing_path: &[u8], what: Comparison) -> AComparison {
    AComparison {
        landing_path: landing_path.to_vec(),
        what,
    }
}

/// **On to the one ordered queue** — P6, and the same door
/// `src/surface/servers.rs` takes for an answer that is read rather than
/// written.
fn the_queue_is_told(queue: &TheOneQueue, event: Event) {
    let _ = queue.read_then_post::<(), std::convert::Infallible>(|| Ok(()), |_| event.clone());
}

#[cfg(test)]
mod tests {
    use super::*;
    use smoozy_file_browser::listing::{ApparentSize, LastChanged};
    use smoozy_file_browser::permission::{MachineKind, TheProbe, WalkOutcome, WhatSmoozyMet};
    use std::time::SystemTime;

    /// **Everything above the test module.**
    ///
    /// It splits on the test module's own opening rather than on
    /// `#[cfg(test)]` alone, because this file carries two test-only helpers
    /// inside `impl Words` — and splitting on the attribute made the working
    /// half **empty**, which three of the checks below then passed over in
    /// silence. Found by the counts in them going to nought.
    fn the_working_half() -> &'static str {
        include_str!("files.rs")
            .split("#[cfg(test)]\nmod tests {")
            .next()
            .expect("the file has a working half")
    }

    fn the_page() -> &'static str {
        include_str!("../../web/files.js")
    }

    fn an_entry(name: &str, kind: Kind, size: u64) -> ListedEntry {
        ListedEntry {
            name: EntryName::from_the_machine(name.as_bytes().to_vec()),
            size: ApparentSize::of(size),
            last_changed: Some(LastChanged::at_epoch_seconds(1_754_483_000)),
            kind,
        }
    }

    fn a_reading(entries: &[ListedEntry]) -> TheListing {
        TheListing::Rows {
            rows: entries.iter().map(|entry| a_row(b"/home/artist", entry, None)).collect(),
            taken_at: Some(1_754_483_040),
            read_elevated: false,
        }
    }

    /// **The thirty states this file claims are the thirty the drawing has,
    /// read out of the drawing in both directions.** *(Twenty-nine until
    /// 2026-10-18, when `fb-multi` was drawn for the thirteenth notes, note 6.)*
    ///
    /// **The runner ruled on 2026-09-12 that the mockup state names are what
    /// this half is counted by**, so a state the drawing has and this file does
    /// not claim is a state nobody built and nothing else would say so — and a
    /// state this file claims and the drawing does not have is a screen for
    /// something Design never drew.
    ///
    /// **The sixteen `chk-` states are the dependency check's** — piece 9's
    /// nine, the two the plan pass for the Creator's notes on 0.1.8 drew
    /// (`chk-added`, `chk-folders`), `chk-reading`, drawn by the drawing
    /// pass for their notes on the test copy, `chk-clean` and `chk-sel`,
    /// drawn for the dependency check's fresh look (2026-10-18), `chk-none`,
    /// drawn for the Creator's thirteenth notes, note 2, and `chk-nosearch`,
    /// drawn for their fourteenth notes, note 3 — and are
    /// excluded here by the same rule the reading of 2026-09-12 cut them out
    /// by.
    #[test]
    fn the_narrow_set_sentence_names_the_jobs_output_folder_as_the_drawing_does() {
        // **`File Browser` W6's folder is the job's output folder, and the
        // words say so** — after 0.1.8, the checks' leftovers, item 2: *its
        // folder* read as the `.blend`'s own folder.
        assert_eq!(
            words::ONLY_THE_BLEND_AND_ITS_FOLDER,
            "For that job, only its .blend and output folder were checked."
        );
        let mockup = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../../Preparation/Design/Mockup/Smoozy.html"),
        )
        .expect("the drawing is readable");
        assert!(mockup.contains(
            "For that job, only its <span class=\"mono\">.blend</span> and output folder were checked."
        ));
    }

    #[test]
    fn the_thirty_states_are_the_drawings_own_thirty() {
        let mockup = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../../Preparation/Design/Mockup/Smoozy.html"),
        )
        .expect("the drawing is readable");
        let opens = mockup
            .find("<optgroup label=\"A server\">")
            .expect("the drawing has an A server group");
        let group = &mockup[opens..opens + mockup[opens..].find("</optgroup>").expect("it closes")];
        let mut drawn: Vec<String> = Vec::new();
        let mut theirs = 0;
        for line in group.lines() {
            let Some(at) = line.find("<option value=\"") else {
                continue;
            };
            let rest = &line[at + "<option value=\"".len()..];
            let closes = rest.find('"').expect("the value closes");
            let state = rest[..closes].to_string();
            if state.starts_with("pane-") {
                continue; // The server heading and rail menu belong to a_server.rs.
            }
            if state.starts_with("chk-") {
                theirs += 1;
                continue;
            }
            drawn.push(state);
        }
        drawn.sort();

        // **As many as the dependency check names as its own**, read from its
        // list rather than written here again (seventeen from 2026-10-24, when
        // the eighteenth notes' design pass drew `chk-sending`).
        assert_eq!(
            theirs,
            crate::surface::the_dependency_check::THE_STATES_THIS_PIECE_DRAWS.len(),
            "the `chk-` states are the dependency check's"
        );
        // Thirty-eight from 2026-10-31, when the plan and drawing pass for the
        // seventeenth notes drew `fb-view`, `fb-viewexr` and `fb-viewbig`
        // (note 11, the picture viewer). Thirty-five from 2026-10-30, when the
        // twenty-third notes' design pass
        // drew `fb-sizemenu`, `fb-size` and `fb-sizeold` (note 2). Thirty-two
        // from 2026-10-29, when the twenty-first notes' design pass drew
        // `fb-check` and `fb-hidden` (notes 6 and 3).
        assert_eq!(drawn.len(), 38, "the A server group has thirty-eight of ours");

        let claimed = the_working_half();
        let named: Vec<String> = drawn
            .iter()
            .filter(|state| claimed.contains(&format!("`{state}`")))
            .cloned()
            .collect();
        assert_eq!(
            named, drawn,
            "this file's head names the mockup states it draws and the two lists \
             have drifted"
        );
    }

    /// **X7, over every sentence this file composes: no value is ever joined
    /// into one.**
    ///
    /// **This is the trap the whole step is written against.** A drawing that
    /// writes `format!("{} could not be reached", name)` has broken X7 and no
    /// other test anywhere would notice — the string is prose-shaped and the
    /// substitution lands silently inside a name the user copies.
    ///
    /// The two `format!` calls in the working half both compose a **value** out
    /// of values — a size, and nothing else — and they are counted here so a
    /// third has to be argued for.
    #[test]
    fn no_sentence_is_built_out_of_a_value() {
        let source = the_working_half();
        for (at, _) in source.match_indices("Prose::new(") {
            let rest = &source[at..];
            let closes = rest.find(')').unwrap_or(rest.len());
            assert!(
                !rest[..closes].contains("format!"),
                "a sentence is composed with `format!`. **X7: the typography \
                 point never touches a job's name, a server's name, a path, a \
                 host or a port.** Put the words in one field and the values in \
                 another"
            );
        }
        // **The apparent size beside its unit, in the three ranges
        // [`the_size`] has**, is composed by the one rule for a figure and its
        // unit (`Design/Whole app § Typography`); the one `format!` left is
        // the figure's own decimal. Until 2026-09-28 the three were
        // `format!("{} {}")`.
        assert_eq!(
            source.matches("a_figure_with_its_unit(").count(),
            3,
            "the apparent size is composed in three ranges, each by the one rule"
        );
        assert_eq!(
            source.matches("format!(").count(),
            1,
            "the working half composes one string with `format!` and it is a \
             **value** — the size's own decimal figure. **A second is a \
             sentence somebody joined a value into unless it is argued for \
             here.**"
        );
    }

    /// **Nothing in this piece composes a failure sentence, so nothing reaches
    /// `WhoseSentenceItIs`** — and a piece that leaves the type unmentioned is
    /// indistinguishable from one that got it wrong, which is why this is a
    /// test rather than a sentence.
    ///
    /// A refusal on a row is not a failure record: nothing durable is written,
    /// no `Standing` is raised, and there is no diary capture to spend.
    #[test]
    fn nothing_here_reaches_the_two_hooks() {
        // **The code rather than the prose**, because this file's own head
        // has to be able to say which type it is not reaching and why.
        let source: String = the_working_half()
            .lines()
            .filter(|line| {
                let line = line.trim_start();
                !(line.starts_with("//") || line.starts_with("///") || line.starts_with("*"))
            })
            .collect::<Vec<_>>()
            .join("\n");
        for named in [
            "WhoseSentenceItIs",
            "TheFailureHandover",
            "the_diary_is_handed_the_sentence",
            "failure_records",
            "the_failure_shape",
        ] {
            assert!(
                !source.contains(named),
                "this file names `{named}` in its code. **A refusal on a row is \
                 not a failure record and this screen composes no failure \
                 sentence**; the screen that does for this module is the \
                 transfer window, which is piece 12's"
            );
        }
        let page: String = the_page()
            .lines()
            .filter(|line| {
                let line = line.trim_start();
                !(line.starts_with("//") || line.starts_with("*"))
            })
            .collect::<Vec<_>>()
            .join("\n");
        assert!(
            !page.contains("theFailureShape"),
            "the page draws the one failure shape. **A refusal on a row is the \
             drawing's own `.deny` and carries no record**, and that builder \
             takes one"
        );
    }

    /// **Every name the event can carry is read by the page.**
    ///
    /// F-13's fault in the other direction: a string the application sends and
    /// the page drops is a word the user does not see, with every other test
    /// passing.
    #[test]
    fn every_name_the_event_can_carry_is_read_by_the_page() {
        let page = the_page();
        for name in ["what", "server", "act", "said", "request"] {
            assert!(
                page.contains(&format!("said.{name}"))
                    || page.contains(&format!("said[\"{name}\"]")),
                "the event can carry `{name}` and the page never reads it. \
                 **F-13's fault in the other direction**: a string the \
                 application sends and the page drops is a word the user does \
                 not see, with every other test passing"
            );
        }
    }

    /// **The page never composes a path**, which is `File Browser` L1a on this
    /// side: the bytes
    /// are identity and the shown form is a display.
    #[test]
    fn the_page_composes_no_path() {
        let page = the_page();
        for shape in ["folder + ", "\"/\" +", ".split(\"/\")", "lastIndexOf(\"/\")"] {
            assert!(
                !page.contains(shape),
                "the page composes a path with `{shape}`. **A path is bytes and \
                 the arithmetic is the application's** — the folder above and \
                 every crumb arrive on the answer"
            );
        }
    }

    /// **A folder row opens by its whole path, never its bare name** — the
    /// Gemini catch-up of step 10a, Area 5 (F-5). `a_row` sets `name` to the
    /// entry's own name and `path` to the whole path; before 2026-09-19 the
    /// press sent the name, so the folder asked for sat at the top of the
    /// server and *Up one folder* found nothing above it.
    /// **`File Browser` L7: the four fields show first and each tag fills in
    /// behind them** — the page asks for a `.blend`'s tag where the row says
    /// one follows, through the one command, which the shell carries. Step
    /// 10c's twelfth-pass fix piece 6b: nothing ever asked, and every tag was
    /// blank.
    #[test]
    fn the_page_asks_for_each_tag_that_follows_the_rows() {
        let page = the_page();
        assert!(page.contains("if (!row.the_tag_follows) continue;"));
        assert!(page.contains("shell.core.invoke(\"the_saved_with_version_is_read\""));
        assert!(page.contains("theTagsAreAskedFor(asked.server, answer.listing);"));
        assert!(include_str!("mod.rs").contains("files::the_saved_with_version_is_read,"));
    }

    /// **A folder row opens by its whole path — on a double-click, or on
    /// Return with it chosen, and never on one click.** The Creator's eighth
    /// notes, note 2, and `Design/File Browser`: *"One click chooses any row
    /// … a double-click opens it"*, and *"Return does what a double-click
    /// does"*. So the two arrive at one opening, which opens a folder by the
    /// row's path (the Gemini catch-up of step 10a, Area 5, F-5: the bare
    /// name asked for a folder at the top) and a `.blend`'s settings.
    #[test]
    fn a_folder_row_opens_by_its_path() {
        let code = the_page_code();
        let opening = the_pages_function("function theRowIsOpened(row)");
        assert!(
            opening.contains("if (row.opens) {\n    theBrowserIsAskedFor(row.path.bytes);"),
            "opening a folder row does not pass the row's whole path: {opening}"
        );
        assert!(
            opening.contains("theSettingsAreOpenedOn(server, row.path.bytes, row.saved_with)"),
            "opening a `.blend` does not open its settings: {opening}"
        );
        assert!(
            !code.contains("theBrowserIsAskedFor(row.name.bytes)"),
            "a folder is asked for by its bare name"
        );
        let row = the_pages_function("function aRow(row)");
        let click = &row[row.find("element.addEventListener(\"click\"").expect("the click")..];
        let click = &click[..click.find("element.addEventListener(\"dblclick\"").expect("the double-click follows")];
        assert!(!click.contains("theRowIsOpened("), "one click opens a row: {click}");
        assert!(!click.contains("theBrowserIsAskedFor("), "one click asks for a listing: {click}");
        let double = &row[row.find("element.addEventListener(\"dblclick\"").expect("the double-click")..];
        let double = &double[..double.find("element.addEventListener(\"contextmenu\"").expect("the right-click follows")];
        assert!(double.contains("theRowIsOpened(row);"), "a double-click does not open the row: {double}");
        assert!(!row.contains("if (row.is_a_blend) {\n      element.addEventListener(\"dblclick\""), "only a `.blend` opens on a double-click");
        let back = the_pages_function("function theChosenRowIsOpened()");
        assert!(back.contains("theRowIsOpened(theChosenRow());"), "Return does not do what a double-click does: {back}");
    }

    /// **A click on a plain file is drawn from the listing the page already
    /// holds, and asks for nothing** — step 10c's transfer wave piece 1. Until
    /// 2026-09-28 the click asked for the whole tab again, which read the
    /// folder on the machine again and blanked the list while it did: about a
    /// hundred round trips on a 10,000-entry folder, for a row being marked.
    ///
    /// **The drawing it redraws with is the application's**: the note is
    /// [`TheToolbar::one_chosen`], and whether the pair of `.blend` controls
    /// shows is the row's own `is_a_blend`.
    #[test]
    fn a_click_on_a_file_redraws_from_the_listing_held_and_asks_nothing() {
        let press = the_pages_function("function aRow(row)");
        let press = &press[press.find("element.addEventListener(\"click\"").expect("the press")..];
        let press = &press[..press
            .find("element.addEventListener(\"dblclick\"")
            .expect("the press ends where the double-click begins")];
        assert!(
            press.contains("theChosenFileIsShown();"),
            "a click on a row does not redraw from what is held: {press}"
        );
        // **Since the eighth notes, note 2, no click asks for the tab**: one
        // click on a folder chooses it as one on a file does.
        assert_eq!(press.matches("theBrowserIsAskedFor(").count(), 0, "a press asks for the tab: {press}");
        let shown = the_pages_function("function theChosenFileIsShown()");
        for asked in ["invoke(", "theBrowserIsAskedFor(", "theReadingIsAskedFor("] {
            assert!(!shown.contains(asked), "a chosen file asks with `{asked}`: {shown}");
        }
        assert!(shown.contains("theBrowserIsDrawn(said)"), "{shown}");
        // **The toolbar is told the choice as it is drawn** (the thirteenth
        // notes, note 6): one row's note is `one_chosen`, the pair the row's
        // own `is_a_blend`. **The one thing asked of the application is the
        // note for a count of several it has not met** — words, never the
        // machine and never the tab.
        let told = the_pages_function("function theToolbarIsToldTheChoice(answer) {");
        assert!(told.contains("toolbar.one_chosen"), "{told}");
        assert!(told.contains("is_a_blend"), "{told}");
        for asked in ["invoke(", "theBrowserIsAskedFor(", "theReadingIsAskedFor("] {
            assert!(!told.contains(asked), "the toolbar's telling asks with `{asked}`: {told}");
        }
        let note = the_pages_function("function theNoteForHowMany(count) {");
        assert_eq!(note.matches("invoke(").count(), 1, "{note}");
        assert!(note.contains("\"the_words_for_how_many_are_chosen\""), "{note}");
        for asked in ["theBrowserIsAskedFor(", "theReadingIsAskedFor("] {
            assert!(!note.contains(asked), "the note asks with `{asked}`: {note}");
        }
    }

    /// **The folder above is composed on the bytes, and the root has none.**
    #[test]
    fn the_folder_above_is_the_bytes_arithmetic() {
        assert_eq!(the_folder_above(b"/"), None);
        assert_eq!(the_folder_above(b""), None);
        assert_eq!(the_folder_above(b"/home"), Some(b"/".to_vec()));
        assert_eq!(the_folder_above(b"/home/artist"), Some(b"/home".to_vec()));
        assert_eq!(the_folder_above(b"/home/artist/"), Some(b"/home".to_vec()));
        // **A name no encoding carries is a name all the same.** The arithmetic
        // is on bytes and never on a decoded string.
        let odd = b"/home/bad\xff\xfe";
        assert_eq!(the_folder_above(odd), Some(b"/home".to_vec()));
    }

    /// **The path along the top, part by part, with the root named.**
    #[test]
    fn the_path_names_every_part_and_marks_where_the_user_is() {
        let parts = the_path(b"/home/artist/kitchen");
        assert_eq!(parts.len(), 4);
        assert!(parts[0].said.is_some(), "the root has a word of its own");
        assert_eq!(parts[1].folder.bytes, b"/home".to_vec());
        assert_eq!(parts[3].folder.bytes, b"/home/artist/kitchen".to_vec());
        assert!(parts[3].here);
        assert!(!parts[0].here);
        assert_eq!(the_path(b"/").len(), 1);
        assert!(the_path(b"/")[0].here);
    }

    /// **An undecodable byte is replaced for display and never for identity** —
    /// `File Browser` L1a, and it is the one rule this whole page turns on.
    /// **`Design/Whole app § Typography`: words after a figure are bound to
    /// it**, and words after a name or a moment are not — *5 files*,
    /// *61 GB free on this server*. The size itself is one value joined by
    /// U+00A0.
    #[test]
    fn words_after_a_figure_are_bound_and_after_a_name_are_not() {
        let files = Words::after("5", "files");
        assert!(files.pieces[1].binds, "5 and files stay on one line");
        let free = Words::after(the_size_of(61 * 1024 * 1024 * 1024), words::FREE_ON_THIS_SERVER);
        assert_eq!(free.values(), vec!["61\u{00A0}GB"]);
        assert!(free.pieces[1].binds);
        assert!(!Words::with("Upload", "kitchen.blend").then("now").pieces[2].binds);
        assert!(!Words::plain("").and_a_moment(1_759_000_000).then("ago").pieces[1].binds);
        let serialised = serde_json::to_value(&files).expect("it serialises");
        assert_eq!(serialised["pieces"][1]["binds"], serde_json::json!(true));
    }

    #[test]
    fn a_name_is_bytes_for_identity_and_a_decode_for_display() {
        let name = APath::of(b"bad\xff\xfe.txt");
        assert_eq!(name.bytes, b"bad\xff\xfe.txt".to_vec());
        assert_ne!(name.shown.as_str(), "bad\u{fffd}\u{fffd}.txt".to_string() + "x");
        assert!(name.shown.as_str().contains('\u{fffd}'));
    }

    /// **`File Browser` N7a: the toolbar's rule is the chosen file's kind and
    /// only its kind**,
    /// and it is taken from that clause's own function rather than decided
    /// here.
    #[test]
    fn only_a_blend_puts_the_two_smoozy_controls_on_the_toolbar() {
        let blend = a_row(b"/home/artist", &an_entry("kitchen_v14.blend", Kind::File, 297_795_584), Some("5.1"));
        let image = a_row(b"/home/artist", &an_entry("studio_backplate.exr", Kind::File, 100_663_296), None);
        assert!(the_toolbar(Some(&blend)).the_blend_pair);
        assert!(!the_toolbar(Some(&image)).the_blend_pair);
        assert!(!the_toolbar(None).the_blend_pair);
        // **A control that cannot apply is absent rather than greyed**, which
        // is Design's own reasoning, and the page reads `the_blend_pair`.
        assert!(the_page().contains("the_blend_pair"));
    }

    /// **Five icons, and the two that matter are not pages** — Design.
    #[test]
    fn the_five_icons_are_the_drawings_five() {
        let of = |name: &str, kind: Kind| a_row(b"/home/artist", &an_entry(name, kind, 1), None).icon;
        assert_eq!(of("assets", Kind::Folder), AnIcon::Folder);
        assert_eq!(of("kitchen_v14.blend", Kind::File), AnIcon::Blend);
        assert_eq!(of("KITCHEN.BLEND", Kind::File), AnIcon::Blend);
        assert_eq!(of("studio_backplate.exr", Kind::File), AnIcon::Image);
        assert_eq!(of("refs.zip", Kind::File), AnIcon::Archive);
        assert_eq!(of("notes.txt", Kind::File), AnIcon::File);
        // **A link is not a folder here**, because following one is the alias
        // case W4 declines to model.
        assert_eq!(of("shortcut", Kind::Link), AnIcon::File);
    }

    /// **A size is divided by 1024 three times**, because a machine's own `df`
    /// writes a `G` and an artist reads GB.
    #[test]
    fn a_size_reads_in_the_units_the_machine_writes() {
        let of = |bytes: u64| {
            a_row(b"/home/artist", &an_entry("one", Kind::File, bytes), None)
                .size
                .expect("a file has a size")
                .as_str()
                .to_string()
        };
        assert_eq!(of(0), "0\u{a0}bytes");
        assert_eq!(of(1023), "1023\u{a0}bytes");
        assert_eq!(of(297_795_584), "284\u{a0}MB");
        assert_eq!(of(1_288_490_188), "1.2\u{a0}GB");
        // **A folder has no size at all**, and the drawing writes an em dash.
        assert!(a_row(b"/home/artist", &an_entry("assets", Kind::Folder, 0), None).size.is_none());
    }

    /// **The four fields, and no fifth** — L1's own list, plus the `.blend`'s
    /// version tag as a tag on the name rather than a column.
    #[test]
    fn a_row_carries_the_four_fields_and_the_tag() {
        let row = a_row(b"/home/artist", &an_entry("kitchen_v14.blend", Kind::File, 297_795_584), Some("5.1"));
        assert_eq!(row.name.shown.as_str(), "kitchen_v14.blend");
        assert_eq!(row.saved_with.expect("a tag").as_str(), "5.1");
        assert!(row.changed.is_some());
        // **L4: the moment travels as the epoch value and the page renders it.**
        // Nothing here subtracts one reading from another.
        assert_eq!(
            a_row(b"/home/artist", &an_entry("one", Kind::File, 1), None)
                .changed
                .expect("a moment")
                .as_str(),
            "1754483000"
        );
        // **Blank is never a guess** (L8): no tag is `None` rather than a word.
        assert!(a_row(b"/home/artist", &an_entry("one.blend", Kind::File, 1), None).saved_with.is_none());
    }

    /// **P5's four outcomes, and three of them offer nothing.**
    ///
    /// **The container's sentence would name a wrong reason on a machine the
    /// kind table cannot place**, which is Design's own words, so only the
    /// container's case says a route cannot be given.
    #[test]
    fn the_offer_is_composed_per_machine_and_three_outcomes_offer_nothing() {
        struct Reads(MachineKind, TheProbe);
        impl smoozy_file_browser::permission::ReadsWhatElevationWouldDo for Reads {
            fn the_machine_kind(&mut self) -> MachineKind {
                self.0
            }
            fn the_non_interactive_probe(&mut self) -> TheProbe {
                self.1
            }
        }
        use smoozy_file_browser::permission::{the_offer, ElevatableAct};

        let passwordless = the_offer(
            ElevatableAct::Remove,
            false,
            &mut Reads(MachineKind::ASystemdMachine, TheProbe::Succeeded),
        );
        let refusal = a_refusal(TheRefusedAct::Deleting, passwordless);
        assert!(refusal.the_offer.is_some());
        assert!(!refusal.the_row_is_dimmed);

        let container = the_offer(
            ElevatableAct::Remove,
            false,
            &mut Reads(MachineKind::AContainerUnderANonRootLogin, TheProbe::Failed),
        );
        let refusal = a_refusal(TheRefusedAct::Deleting, container);
        assert!(refusal.the_offer.is_none());
        assert!(
            refusal.said.said().contains("No admin access"),
            "the container's own sentence says a route cannot be given"
        );

        let unplaceable = the_offer(
            ElevatableAct::Remove,
            false,
            &mut Reads(MachineKind::TheTableDoesNotPlaceIt, TheProbe::Succeeded),
        );
        let refusal = a_refusal(TheRefusedAct::Deleting, unplaceable);
        assert!(refusal.the_offer.is_none());
        assert!(
            !refusal.said.said().contains("Admin access"),
            "**a machine the table cannot place is not a container**, and the \
             container's sentence would name a wrong reason there"
        );

        // **A folder that cannot be read is the one refusal that dims its row**,
        // and the greying comes with the refusal rather than standing before it.
        let read = a_refusal(
            TheRefusedAct::ReadingAFolder,
            the_offer(
                ElevatableAct::AListing,
                false,
                &mut Reads(MachineKind::ASystemdMachine, TheProbe::Succeeded),
            ),
        );
        assert!(read.the_row_is_dimmed);
    }

    /// **P4's residue names which of the two it met and stops there**, and P3's
    /// two name the component.
    #[test]
    fn the_walks_answer_never_names_the_wrong_one_of_the_two() {
        let refused = what_the_walk_said(&WalkOutcome::RefusedAt("/jobs".to_string()));
        assert_eq!(refused.values()[0], "/jobs");
        let gone = what_the_walk_said(&WalkOutcome::GoneAt("/jobs".to_string()));
        assert_eq!(gone.values()[0], "/jobs");
        assert_ne!(refused.said(), gone.said());
        let residue = what_the_walk_said(&WalkOutcome::CouldNotSayWhere(WhatSmoozyMet::Gone));
        assert!(residue.values().is_empty());
        assert_ne!(
            residue.said(),
            what_the_walk_said(&WalkOutcome::CouldNotSayWhere(WhatSmoozyMet::Refused))
                .said(),
            "**a message naming the wrong one of the two is worse than a message \
             naming neither**"
        );
    }

    /// **The collision question's count takes its singular** — step 10c's
    /// eleventh pass read *"1 of these 1 files are already in /home/ubuntu"*.
    /// `Design/File Browser`'s own sentence is *42 of these 300 files are
    /// already in*; one among several is *1 of these 3 files is already in*,
    /// and one alone is *1 file is already in*. Rule 9, *Short, too*. The same
    /// for a paste of one item. Added 2026-09-30, step 10c's eleventh-pass fix
    /// piece 7.
    #[test]
    fn the_collision_questions_count_takes_its_singular() {
        let read = |words: &Words| {
            words
                .pieces
                .iter()
                .map(|piece| {
                    piece
                        .value
                        .as_ref()
                        .map(|value| value.as_str().to_string())
                        .or_else(|| piece.said.as_ref().map(|said| said.as_str().to_string()))
                        .unwrap_or_default()
                })
                .collect::<Vec<_>>()
                .join(" ")
                .replace('\u{a0}', " ")
        };
        let of = |colliding: usize, all: usize, act| {
            let comparisons = (0..all)
                .map(|at| {
                    a_collision(
                        format!("/home/ubuntu/{at}.blend").as_bytes(),
                        if at < colliding { Comparison::CollidesWithAFile } else { Comparison::LandsFresh },
                    )
                })
                .collect();
            let answer = TheWalksAnswer {
                comparisons,
                colliding,
                arriving_files: all,
                as_of: SystemTime::UNIX_EPOCH,
                readings_taken: 1,
            };
            read(&the_collision_question(act, &answer, b"/home/ubuntu", 0, &[], &[], None).headline)
        };
        assert_eq!(of(1, 1, TheCollidingAct::AnUpload), "1 file is already in /home/ubuntu");
        assert_eq!(of(1, 1, TheCollidingAct::ADownload), "1 file is already in /home/ubuntu");
        assert_eq!(of(1, 3, TheCollidingAct::AnUpload), "1 of these 3 files is already in /home/ubuntu");
        assert_eq!(of(2, 3, TheCollidingAct::AnUpload), "2 of these 3 files are already in /home/ubuntu");
        assert_eq!(of(3, 3, TheCollidingAct::AnUpload), "3 of these 3 files are already in /home/ubuntu");
        assert_eq!(of(1, 1, TheCollidingAct::APasteAfterACut), "1 item you are moving is already in /home/ubuntu");
        assert_eq!(of(1, 1, TheCollidingAct::APasteAfterACopy), "1 item you are copying is already in /home/ubuntu");
        assert_eq!(of(1, 2, TheCollidingAct::APasteAfterACut), "1 of the 2 items you are moving is already in /home/ubuntu");
        assert_eq!(of(2, 2, TheCollidingAct::APasteAfterACopy), "2 of the 2 items you are copying are already in /home/ubuntu");

        // **A folder of 4 files counts 4, not its folders too** — after 0.1.8,
        // the checks' leftovers, item 5: the check before 0.1.8 read *4 of
        // these 6 files* for `c18-dots`, whose folder and hidden folder both
        // merge.
        let answer = TheWalksAnswer {
            comparisons: vec![
                a_collision(b"/home/ubuntu/c18-dots", Comparison::Merges),
                a_collision(b"/home/ubuntu/c18-dots/.dotfile", Comparison::CollidesWithAFile),
                a_collision(b"/home/ubuntu/c18-dots/.env", Comparison::CollidesWithAFile),
                a_collision(b"/home/ubuntu/c18-dots/.hidden-dir", Comparison::Merges),
                a_collision(b"/home/ubuntu/c18-dots/.hidden-dir/a.txt", Comparison::CollidesWithAFile),
                a_collision(b"/home/ubuntu/c18-dots/visible.txt", Comparison::CollidesWithAFile),
            ],
            colliding: 4,
            arriving_files: 4,
            as_of: SystemTime::UNIX_EPOCH,
            readings_taken: 2,
        };
        assert_eq!(
            read(&the_collision_question(TheCollidingAct::AnUpload, &answer, b"/home/ubuntu", 0, &[], &[], None).headline),
            "4 of these 4 files are already in /home/ubuntu"
        );
    }

    /// **The already-exists box is one shape in six readings**, and the answers
    /// are the act's own.
    #[test]
    fn the_collision_box_is_one_shape_and_the_answers_are_the_acts() {
        let answer = TheWalksAnswer {
            comparisons: vec![
                a_collision(b"kitchen/textures/metallic.png", Comparison::CollidesWithAFile),
                a_collision(b"kitchen/textures/wood_oak.png", Comparison::CollidesWithAFile),
                a_collision(b"kitchen/new.png", Comparison::LandsFresh),
            ],
            colliding: 2,
            arriving_files: 3,
            as_of: SystemTime::UNIX_EPOCH,
            readings_taken: 1,
        };
        let of = |act, interrupted| {
            the_collision_question(act, &answer, b"kitchen/", 3, &[], &[], interrupted)
        };

        let upload = of(TheCollidingAct::AnUpload, None);
        assert_eq!(upload.names.len(), 2);
        assert!(upload.placed_by_the_check.is_some());
        assert!(upload.interrupted.is_none());
        let tokens: Vec<&str> = upload.answers.iter().map(|one| one.answer.as_str()).collect();
        assert_eq!(tokens, vec!["replace", "keep-both", "skip-existing", "stop"]);
        assert!(
            !upload.answers.iter().any(|one| one.put_forward),
            "**with no record of an earlier request the four answers sit level**"
        );

        let resumed = of(TheCollidingAct::AnUpload, Some(1_754_483_000));
        let why = resumed.interrupted.as_ref().expect("the reason line");
        assert!(
            why.pieces.iter().any(|piece| piece.moment),
            "the moment is rendered by the page in the user's zone and never drawn as a figure"
        );
        // **The moment ends its sentence and the next one stands apart** —
        // *…on 12 Aug. Skip existing uploads only what is missing.* The pass
        // before 0.1.9 on Vast read *…08:31 PMSkip existing…*.
        let after_the_moment: Vec<(Option<Prose>, bool)> = why
            .pieces
            .iter()
            .skip_while(|piece| !piece.moment)
            .skip(1)
            .map(|piece| (piece.said.clone(), piece.joins))
            .collect();
        assert_eq!(
            after_the_moment,
            vec![
                (Some(Prose::new(words::FULL_STOP)), true),
                (Some(Prose::new(words::INTERRUPTED_REST)), false)
            ]
        );
        assert!(
            resumed.answers[0].answer.as_str() == "skip-existing" && resumed.answers[0].put_forward,
            "**Skip existing is put forward only where Smoozy can say why**"
        );

        // **The download reading is the plainest one**: no live-job warning,
        // because it is about the machine's own work, and no check line.
        let download = of(TheCollidingAct::ADownload, None);
        assert!(download.placed_by_the_check.is_none());
        assert!(download.live_jobs.is_none());

        // **M8 and M18: a paste after a cut offers Replace or Cancel; a paste
        // after a copy offers Replace, Keep both or Cancel.**
        let after_a_cut = of(TheCollidingAct::APasteAfterACut, None);
        let cut: Vec<&str> = after_a_cut.answers.iter().map(|one| one.answer.as_str()).collect();
        assert_eq!(cut, vec!["replace", "cancel"]);
        let after_a_copy = of(TheCollidingAct::APasteAfterACopy, None);
        let copy: Vec<&str> = after_a_copy.answers.iter().map(|one| one.answer.as_str()).collect();
        assert_eq!(copy, vec!["replace", "keep-both", "cancel"]);
    }

    /// **`File Browser` W5a: uniform across files, differentiated across jobs,
    /// and the
    /// sentence is a *may* and never a description.**
    #[test]
    fn the_live_job_warning_differentiates_across_jobs_and_never_across_files() {
        use smoozy_state::identities::{Identity, IdentityKind};
        let one = Identity::issue(IdentityKind::Job).expect("randomness");
        let two = Identity::issue(IdentityKind::Job).expect("randomness");
        let labels = vec![
            (one.as_text(), "Kitchen — final".to_string()),
            (two.as_text(), "Kitchen — clay".to_string()),
        ];
        let jobs = vec![
            AJobNamed {
                job: one,
                stands: HowTheJobStands::Rendering,
                the_members_reached: vec![b"kitchen/kitchen_v14.blend".to_vec()],
                the_set_was_the_widest_case: false,
            },
            AJobNamed {
                job: two,
                stands: HowTheJobStands::Waiting,
                the_members_reached: vec![b"kitchen/kitchen_v14.blend".to_vec()],
                the_set_was_the_widest_case: true,
            },
        ];
        let warning = the_live_job_warning(&jobs, &labels).expect("two jobs are named");
        let names = warning.said.values();
        assert_eq!(names, vec!["Kitchen — final", "Kitchen — clay"]);
        assert!(warning.only_the_blend_and_its_folder.is_some());
        assert!(
            warning
                .a_render_may_finish_anyway
                .as_str()
                .contains("may finish"),
            "**the sentence is a *may* and never a description**"
        );
        // **A name never travels inside the sentence** (X7).
        assert!(
            !warning.said.said().contains("Kitchen"),
            "**a name never travels inside the sentence** (X7)"
        );
        assert!(the_live_job_warning(&[], &[]).is_none());
    }

    /// **The delete box carries two warnings of different weight, and the
    /// checkbox says in one line that it does not cover the other.**
    #[test]
    fn the_delete_box_says_what_the_checkbox_does_not_cover() {
        let box_ =
            the_delete_box_from(&[b"kitchen/kitchen_v14.blend".to_vec()], &[], &[], Some(3), None, false);
        assert_eq!(box_.question.values()[0], "kitchen_v14.blend");
        assert!(box_.question.pieces.last().expect("a piece").joins, "the mark goes against the name");
        assert!(box_.other_references.is_some());
        assert!(box_.live_jobs.is_none());
        assert!(box_
            .what_it_does_not_cover
            .as_str()
            .contains("jobs using the file"));
        // **No suppression row exists for the live-job warning** — M3, and it
        // is that crate's own answer rather than one taken here.
        assert!(
            smoozy_file_browser::warnings::no_suppression_row_exists_for_the_live_job_warning()
        );
        assert_eq!(the_delete_warnings_suppression_row(), "file-browser.delete");
    }

    /// **L15, per act: unavailable rather than failing on a click**, and the
    /// list is `File Browser`'s own closed one.
    #[test]
    fn every_act_that_needs_the_machine_is_unavailable_while_the_line_is_down() {
        let tab = the_tab(
            "a-server",
            b"/kitchen",
            None,
            TheListing::NothingHasBeenListed {
                lead: Prose::new("one"),
                rest: Prose::new("two"),
            },
            None,
            None,
            false,
        );
        // **One more token than the closed list: a folder's count**, which
        // is unavailable whenever they are (the twenty-third notes, note 2).
        assert_eq!(
            tab.unavailable.len(),
            AnActThatNeedsTheMachine::ALL.len() + 1,
            "**no value at all is not *connected***: the browser composes as \
             though the line were down"
        );
        let live = the_tab(
            "a-server",
            b"/kitchen",
            None,
            a_reading(&[an_entry("assets", Kind::Folder, 0)]),
            Some(ObservedValue::Connected),
            None,
            false,
        );
        assert!(live.unavailable.is_empty());
        assert!(live.stale.is_none(), "a live line draws no stale bar");
        let stale = the_tab(
            "a-server",
            b"/kitchen",
            None,
            a_reading(&[an_entry("assets", Kind::Folder, 0)]),
            Some(ObservedValue::Reconnecting),
            None,
            false,
        );
        assert!(stale.stale.is_some(), "a listing with the line down says so");
    }

    /// **L14's own last sentence is what a server with no live connection
    /// shows**, and it is not the same answer as a read that did not happen
    /// (L8a) — and **where the connection is live, the folder is being read
    /// and the reading follows** (step 10c piece 4).
    #[test]
    fn the_three_readings_that_are_not_a_listing_are_three_and_not_one() {
        let there = the_tab_from(
            WhatThereIs::TheRecordWouldNotRead,
            "a-server",
            None,
            None,
            true,
            false,
        );
        assert!(matches!(there.listing, TheListing::TheReadDidNotAnswer { .. }));
        assert!(!there.a_reading_follows);
        let gone = the_tab_from(WhatThereIs::NoSuchServer, "a-server", None, None, true, false);
        assert!(matches!(gone.listing, TheListing::NoSuchServer { .. }));
        assert!(!gone.a_reading_follows);
    }

    /// **The bottom bar's standing line answers *where will an upload land*
    /// before anything is asked**, which is the one thing on a quiet screen
    /// that does.
    #[test]
    fn the_bottom_bar_says_where_an_upload_lands() {
        let tab = the_tab(
            "a-server",
            b"/home/artist/kitchen",
            None,
            a_reading(&[
                an_entry("assets", Kind::Folder, 0),
                an_entry("one.blend", Kind::File, 10),
                an_entry("two.blend", Kind::File, 10),
            ]),
            Some(ObservedValue::Connected),
            None,
            false,
        );
        assert_eq!(
            tab.bottom.uploads_go_to.values()[0],
            "/home/artist/kitchen"
        );
        let counts = tab
            .bottom
            .what_is_in_the_folder
            .expect("a listing carries a count");
        assert_eq!(counts.values()[0], "2");
        assert_eq!(counts.values()[1], "1");
    }

    /// **Three menus, and only the folder menu could carry paste** — Design.
    /// **Upload and new folder are on no menu at all.**
    #[test]
    fn the_three_menus_carry_what_acts_on_the_thing_clicked() {
        let menus = the_menus();
        assert_eq!(menus.len(), 3);
        for menu in &menus {
            let acts: Vec<&str> = menu.items.iter().map(|one| one.act.as_str()).collect();
            for owed in ["download", "cut", "copy", "rename", "delete"] {
                assert!(acts.contains(&owed), "{} has no {owed}", menu.which);
            }
            for never in ["upload", "new-folder"] {
                assert!(
                    !acts.contains(&never),
                    "**{never} acts on the folder being browsed rather than on \
                     the thing clicked**, and a menu item about somewhere else \
                     is not what a right-click is for"
                );
            }
        }
        let blend: Vec<&str> = menus[2].items.iter().map(|one| one.act.as_str()).collect();
        assert_eq!(blend[0], "render-settings");
        assert_eq!(blend[1], "check-dependencies");
        assert!(menus[2].items[0].at_the_head);
        assert!(menus
            .iter()
            .all(|menu| menu.items.iter().any(|one| one.in_red && one.act.as_str() == "delete")));
    }

    /// A scratch folder no other run can be standing in, because the process
    /// id is part of its name. It is emptied before it is handed back, and the
    /// test that takes it removes it at the end.
    ///
    /// **The two tests below named their folder after themselves alone**, so
    /// two test runs at once on one computer used the one folder and deleted
    /// each other's files. That is the fault diagnosed on 2026-09-18 in
    /// `src/presentation/the_two_hooks.rs`, and these were two more of it:
    /// measured here beforehand, two runs at once failed 9 of 40 while one run
    /// alone failed 0 of 20.
    ///
    /// **This is the crate's ordinary helper** — the one at
    /// `src/record/failure_records.rs`. `std::process` is refused under
    /// `src/presentation/` and nowhere else, and this file is not there. The
    /// name is joined rather than composed, because
    /// [`no_sentence_is_built_out_of_a_value`] counts every `format!` in this
    /// file and a fourth would have to be argued for.
    fn a_scratch_area(named: &str) -> std::path::PathBuf {
        let mut name = String::from("smoozy-files-test-");
        name.push_str(named);
        name.push('-');
        name.push_str(&std::process::id().to_string());
        let area = std::env::temp_dir().join(name);
        let _ = std::fs::remove_dir_all(&area);
        area
    }

    /// **A pick goes with what was picked, and no confirmation is composed** —
    /// `Design/File Browser § Uploading`, struck 2026-10-04; the Creator's
    /// second notes on the test copy, piece 9 (note 2). A folder and two files
    /// chosen together in one pick, as a Mac's own panel hands them over, start
    /// the upload with all three and nothing asked. **Where no panel takes both
    /// (Windows and Linux) the question is asked**, and on Windows it names
    /// Windows.
    #[test]
    fn a_pick_starts_the_upload_and_only_a_system_with_no_panel_for_both_is_asked() {
        let chosen = ["/Users/a/textures", "/Users/a/one.png", "/Users/a/two.png"];
        let picked = crate::picker::Picked::TheUserChose(
            chosen
                .iter()
                .map(|one| a_path_from_the_user(std::path::Path::new(one)))
                .collect(),
        );
        let goes = serde_json::to_value(the_upload_ask_from(picked)).expect("serialises");
        assert_eq!(
            goes,
            serde_json::json!({
                "is": "go",
                "chosen": chosen.iter().map(|one| serde_json::to_value(APath::of(one.as_bytes())).expect("a path")).collect::<Vec<_>>(),
            }),
            "a pick composes something other than the upload going with what was picked"
        );
        assert_eq!(
            the_upload_ask_from(crate::picker::Picked::TheUserChoseNothing),
            TheUploadAsk::NothingWasChosen
        );

        let asked = the_upload_ask_from(crate::picker::Picked::ThereIsNoPanelForThisAsk(
            crate::picker::WhatMayBeChosen::FilesAndFolders,
        ));
        let TheUploadAsk::WhichKind { title, said, .. } = asked else {
            panic!("no panel for both kinds and no question: {asked:?}");
        };
        assert_eq!(title, Prose::new(words::WHICH_KIND));
        let wanted = if cfg!(target_os = "windows") {
            "The Windows file picker cannot select files and folders together."
        } else {
            "This computer's file picker cannot select files and folders together."
        };
        assert_eq!(said, Prose::new(wanted));
    }

    /// A picked path, as the picker hands one over. The picker's own type has
    /// no public maker, so it is made the one way it is made: by a pick.
    fn a_path_from_the_user(path: &std::path::Path) -> crate::picker::APathFromTheUser {
        match crate::picker::the_answer_from(Some(vec![path.to_path_buf()])) {
            crate::picker::Picked::TheUserChose(mut one) => one.remove(0),
            other => panic!("a path made nothing: {other:?}"),
        }
    }

    /// **The drop reading counts what was dropped and never what is inside it.**
    #[test]
    fn the_drop_reading_counts_what_was_dropped() {
        let area = a_scratch_area("drop");
        std::fs::create_dir_all(area.join("sim")).expect("the area is made");
        std::fs::write(area.join("one.png"), b"x").expect("a file");
        let reading = the_drop_reading_from(&[area.join("sim"), area.join("one.png")], b"/kitchen");
        let values = reading.how_many.values();
        assert_eq!(values, vec!["1", "1"]);
        assert_eq!(reading.into.values()[0], "/kitchen");
        let _ = std::fs::remove_dir_all(&area);
    }

    /// **A rename is refused by the machine and warned about by the listing,
    /// and only the first stops anything** — M4.
    #[test]
    fn the_listings_warning_and_the_machines_refusal_are_two_different_things() {
        let refused = a_rename_refused(b"studio_backplate.exr");
        assert!(refused.refused.is_some());
        assert!(refused.the_listing_saw_it.is_none());
        let hint = a_rename_hint(b"studio_backplate.exr");
        assert!(hint.refused.is_none());
        assert!(hint.the_listing_saw_it.is_some());
        assert_ne!(
            refused.refused.expect("a refusal").said(),
            hint.the_listing_saw_it.expect("a hint").said()
        );
    }

    /// **The three bars another screen puts on this one, each with its own
    /// sentence and none of them composed on the page.**
    #[test]
    fn the_three_bars_are_composed_here_and_name_their_job_verbatim() {
        let pick = the_pick_bar("Kitchen — final");
        let ABarAcrossTheTop::Pick { said, .. } = &pick else {
            panic!("a pick bar");
        };
        assert_eq!(said.values()[0], "Kitchen — final");
        assert!(
            said.pieces.last().expect("a piece").joins,
            "the mark goes against the name"
        );

        let moved = the_moved_bar("Hetzner box", true);
        let ABarAcrossTheTop::Moved {
            said,
            still_uploading,
        } = &moved
        else {
            panic!("a moved bar");
        };
        assert_eq!(said.values()[0], "Hetzner box");
        assert!(
            still_uploading.is_some(),
            "**the second sentence is drawn only where an upload is actually in \
             flight**"
        );
        let quiet = the_moved_bar("Hetzner box", false);
        let ABarAcrossTheTop::Moved { still_uploading, .. } = &quiet else {
            panic!("a moved bar");
        };
        assert!(still_uploading.is_none());

        assert!(matches!(
            the_find_bar("Kitchen — final v3"),
            ABarAcrossTheTop::Find { .. }
        ));
    }

    /// **Step 10c's *Files on the server* follow-up: *show outputs* on a job on
    /// another server puts the moved line up, and it clears when the user
    /// changes folder** — `Design/File Browser § When another module moves
    /// the browser to a different server`. **A job on the server already
    /// browsed is no move**, and neither is the first browsing of the run:
    /// the line is about the browser moving.
    ///
    /// **Undone** — the line put up on the same server, or never put up, or
    /// kept after a folder changed or another server was shown — it fails.
    #[test]
    fn follow_up_show_outputs_on_another_server_moves_the_browser_and_says_so() {
        let kitchen: &[u8] = b"/home/ubuntu/Render/kitchen/";
        let mut at = WhereTheBrowserIs::default();
        assert!(!at.the_browser_is_asked("box-a", Some(b"/home/ubuntu")), "nothing moved it");

        // **Another server: the line stands at the folder the move opened**,
        // on the reading that follows and on a redraw after an act alike.
        at.outputs_are_shown_on("box-b", kitchen, false);
        assert!(at.the_browser_is_asked("box-b", Some(kitchen)), "the move puts the line up");
        assert!(at.the_browser_is_asked("box-b", Some(kitchen)), "a redraw at the same folder keeps it");

        // **The user changes folder: it goes, and does not come back.**
        assert!(!at.the_browser_is_asked("box-b", Some(b"/home/ubuntu/Render/kitchen/exr")));
        assert!(!at.the_browser_is_asked("box-b", Some(kitchen)), "it clears for good");

        // **The server already browsed: no move, no line.**
        at.outputs_are_shown_on("box-b", kitchen, false);
        assert!(!at.the_browser_is_asked("box-b", Some(kitchen)), "moving to the server shown is not a move");

        // **Going to another server takes it with it.**
        at.outputs_are_shown_on("box-a", kitchen, false);
        assert!(!at.the_browser_is_asked("box-b", Some(kitchen)), "another server was shown");
        assert!(!at.the_browser_is_asked("box-a", Some(kitchen)));

        // **Nothing browsed yet in this run: nothing to move from.**
        let mut fresh = WhereTheBrowserIs::default();
        fresh.outputs_are_shown_on("box-b", kitchen, false);
        assert!(!fresh.the_browser_is_asked("box-b", Some(kitchen)));
    }

    /// **The Creator's fourth notes, piece O: *show outputs* on a job's folder
    /// that is not there.** For a job whose record shows no frame saved, Files
    /// says *Does not exist yet: … No frames saved.*, naming the
    /// walk's own missing part; for a job that has saved frames, `File
    /// Browser` P3's *Not found: …* stands — `Design/File Browser §
    /// When another module moves the browser`, added 2026-10-07. **On the
    /// server already browsed as much as on another**, and only at the folder
    /// the press opened: the user going elsewhere, and coming back by hand,
    /// reads P3's. A refusal is never reworded.
    ///
    /// **Undone** (the fact not kept, kept for every folder, or the words
    /// never changed), it fails.
    #[test]
    fn piece_o_a_jobs_folder_not_there_yet_says_no_frames_have_been_saved_only_where_none_are() {
        let beauty: &[u8] = b"/root/Render/kitchen/beauty";
        let walked = |outcome: WalkOutcome| WhatTheReadingFound::NotListed(what_the_walk_said(&outcome));
        let gone = || walked(WalkOutcome::GoneAt("/root/Render/kitchen".to_string()));
        // **What Files draws in place of the list**, asked the way the
        // reading command asks: the browser told first, then the reading.
        let drawn = |at: &mut WhereTheBrowserIs, server: &str, folder: &[u8], found: WhatTheReadingFound| {
            at.the_browser_is_asked(server, Some(folder));
            let found = if at.no_frame_is_saved_here() { not_there_yet_where_no_frame_is_saved(found) } else { found };
            match the_tab_after_the_reading(TheOrder::default(), server, folder, None, found, None, false).listing {
                TheListing::TheFolderWasNotRead { said } => said,
                other => panic!("the folder was listed: {other:?}"),
            }
        };
        let p3 = Words::with(words::IT_IS_NOT_THERE_AT_LEAD, "/root/Render/kitchen");

        // **No frame saved, on the server already browsed: the new words.**
        let mut at = WhereTheBrowserIs::default();
        at.the_browser_is_asked("box-a", Some(b"/root"));
        at.outputs_are_shown_on("box-a", beauty, true);
        let said = drawn(&mut at, "box-a", beauty, gone());
        assert_eq!(
            as_read(&said).replace('\u{a0}', " "),
            "Does not exist yet: /root/Render/kitchen . No frames saved."
        );
        assert_eq!(said.values(), vec!["/root/Render/kitchen"], "the walk's own missing part is named");
        assert!(said.pieces[2].joins, "the full stop sits against the path");
        assert!(!said.pieces[2].binds);
        // A redraw at the same folder keeps them.
        assert_ne!(drawn(&mut at, "box-a", beauty, gone()), p3);

        // **Another folder, then back by hand: P3's.**
        assert_eq!(drawn(&mut at, "box-a", b"/root/Render", gone()), p3);
        assert_eq!(drawn(&mut at, "box-a", beauty, gone()), p3, "the fact went with the folder");

        // **On another server, moved there, the same.**
        at.outputs_are_shown_on("box-b", beauty, true);
        assert_eq!(said, drawn(&mut at, "box-b", beauty, gone()));

        // **A job that has saved frames: P3's, moved or not.**
        at.outputs_are_shown_on("box-b", beauty, false);
        assert_eq!(drawn(&mut at, "box-b", beauty, gone()), p3);
        at.outputs_are_shown_on("box-a", beauty, false);
        assert_eq!(drawn(&mut at, "box-a", beauty, gone()), p3);

        // **Never a refusal, never the walk that could not place it.**
        at.outputs_are_shown_on("box-a", beauty, true);
        let refused = walked(WalkOutcome::RefusedAt("/root/Render".to_string()));
        assert_eq!(not_there_yet_where_no_frame_is_saved(refused.clone()), refused);
        let unplaced = walked(WalkOutcome::CouldNotSayWhere(WhatSmoozyMet::Gone));
        assert_eq!(not_there_yet_where_no_frame_is_saved(unplaced.clone()), unplaced);
        let listed = WhatTheReadingFound::TheFolder { entries: Vec::new(), free: None };
        assert_eq!(not_there_yet_where_no_frame_is_saved(listed.clone()), listed);
    }

    /// **The moved line names the server as the server list does, and says
    /// the second half only while an upload is in flight** — a request this
    /// run holds under way that the record has as an upload. A download under
    /// way, or an upload whose server's period has ended, is not one.
    /// **Undone** (another name, or the second half on a quiet browser), it
    /// fails.
    #[test]
    fn follow_up_the_moved_line_names_the_server_and_says_an_upload_goes_on_only_where_one_does() {
        use smoozy_servers::identity::{Host, KeyReference, Port, Username};
        let connection = crate::record::deletions::tests::a_record_store();
        let server = Identity::issue(IdentityKind::LocalServer).expect("random bytes").as_text();
        let mut row = ServerRow::saved(
            server.clone(),
            Host::what_the_user_typed("127.0.0.1").expect("a host"),
            Port::what_the_user_typed(22).expect("a port"),
            Username::what_the_user_typed("root").expect("a username"),
            KeyReference { local_key_identifier: "key-1".into(), fingerprint: "SHA256:aaa".into() },
        );
        row.label = Some("Hetzner box".into());
        server_rows::add(&connection, &row).expect("the row");
        connection
            .execute(
                "INSERT INTO upload_request (record, server_row, destination, opened_by_run)
                 VALUES ('up-1', 'box-a', x'2f', 'run-1')",
                [],
            )
            .expect("an upload request");
        let line = |under_way: &[(&str, bool)]| {
            let under_way = under_way
                .iter()
                .map(|(id, ended)| {
                    let mut one = crate::downloading::ARequestUnderWay::default();
                    one.its_period_ended = *ended;
                    (id.to_string(), one)
                })
                .collect();
            match the_moved_bar_from(&connection, &server, &under_way) {
                Some(ABarAcrossTheTop::Moved { said, still_uploading }) => {
                    (said.values()[0].to_string(), still_uploading.is_some())
                }
                other => panic!("a moved bar, not {other:?}"),
            }
        };
        assert_eq!(line(&[]), ("Hetzner box".to_string(), false));
        assert_eq!(line(&[("down-1", false)]), ("Hetzner box".to_string(), false), "a download is not an upload");
        assert_eq!(line(&[("up-1", true)]), ("Hetzner box".to_string(), false), "an upload whose period ended");
        assert_eq!(line(&[("up-1", false)]), ("Hetzner box".to_string(), true));
        assert!(the_moved_bar_from(&connection, "box-gone", &Default::default()).is_none(), "no row, no line");
    }

    /// **The one Go item this piece makes live**, and `src/menus.rs` is not
    /// edited to do it.
    #[test]
    fn up_one_folder_is_made_live_without_the_menu_file_moving() {
        let menus = include_str!("../menus.rs");
        assert!(
            menus.contains("\"go.up-one-folder\""),
            "the item is the menus' own and this piece does not add it"
        );
        assert_eq!(UP_ONE_FOLDER, "go.up-one-folder");
        assert!(
            the_page().contains("UP_ONE_FOLDER"),
            "the page acts on the going"
        );
        // **It is not a place**, so it is not in the rail's own map of goings to
        // places — a place would put the browser in the main pane's place list.
        let rail = include_str!("../../web/the-rail.js");
        assert!(
            !rail.contains("go.up-one-folder"),
            "**`Up one folder` is a control on a screen rather than a place**"
        );
    }

    /// **The stale bar's moment is marked as one, and the page renders it**,
    /// rather than drawing the bare epoch figure — which is what a running copy
    /// showed on 2026-09-13.
    #[test]
    fn the_stale_bars_moment_is_rendered_rather_than_drawn_as_a_figure() {
        let listing = TheListing::Rows {
            rows: Vec::new(),
            taken_at: Some(1_754_483_000),
            read_elevated: false,
        };
        let bar = the_stale_bar(&listing);
        let moments: Vec<&APieceOfIt> = bar.said.pieces.iter().filter(|one| one.moment).collect();
        assert_eq!(moments.len(), 1, "the one value in the bar is the moment");
        assert_eq!(moments[0].value.as_ref().expect("a value").as_str(), "1754483000");
        assert!(
            the_page().contains("piece.moment ? aCalendarTime(piece.value)")
                || the_page().contains("piece.moment) node.textContent = aCalendarTime(piece.value)"),
            "the page draws a marked moment through its own rendering"
        );
    }

    /// **Ruling 4 of 2026-09-13, the version deadlock.** `Render Settings` B2
    /// takes the file's saved-with version from `File Browser`'s read of the
    /// unopened file, and a panel that is never handed it can never be read
    /// (`Render Settings` B2a, then `Render Settings` B5a). **So every way this page opens the render settings
    /// hands over the row's whole path and its version**, and none opens the
    /// bare panel, which is exactly the route that dropped the version.
    #[test]
    fn the_render_settings_are_opened_with_the_files_own_version() {
        let row = a_row(
            b"/home/artist/projects/kitchen/",
            &an_entry("kitchen_v14.blend", Kind::File, 297_795_584),
            Some("5.1"),
        );
        assert_eq!(row.path.bytes, b"/home/artist/projects/kitchen/kitchen_v14.blend".to_vec());
        assert_eq!(row.saved_with.expect("a tag").as_str(), "5.1");

        let code: String = the_page()
            .lines()
            .filter(|line| {
                let line = line.trim_start();
                !(line.starts_with("//") || line.starts_with("*"))
            })
            .collect::<Vec<_>>()
            .join("\n");
        assert!(
            code.contains("import { theSettingsAreOpenedOn } from \"./render-settings.js\""),
            "the page does not import the render settings' own way in"
        );
        assert!(
            !code.contains("RENDER_SETTINGS"),
            "**the page opens the bare render settings panel**, which is the \
             route that never hands over the saved-with version — ruling 4"
        );
        assert_eq!(
            code.matches("theSettingsAreOpenedOn(server, row.path.bytes, row.saved_with)").count(),
            2,
            "a `.blend` row's double-click and the one helper the toolbar and \
             the right-click menu share both hand over the path and the version"
        );
        assert_eq!(
            code.matches("theSettingsAreOpenedForTheChosen()").count(),
            3,
            "the helper's own definition, and the toolbar's Render settings \
             and the right-click menu's item, both of which go through it"
        );
    }

    /// **Every act on this screen that starts a transfer goes to Transfers** —
    /// `Design/File Browser`: *"Transfers it starts are watched in Transfers."*
    /// The confirmed upload, the toolbar's Download and the menus' Download
    /// and Download to… all go through the one helper, and none of them is
    /// answered here any more.
    #[test]
    fn the_acts_that_start_a_transfer_go_to_transfers() {
        let code: String = the_page()
            .lines()
            .filter(|line| {
                let line = line.trim_start();
                !(line.starts_with("//") || line.starts_with("*"))
            })
            .collect::<Vec<_>>()
            .join("\n");
        assert!(code.contains("import { aTransferIsAskedFor, theTransfersAreShown } from \"./transfers.js\""));
        assert!(code.contains(
            "const THE_ACTS_THAT_START_A_TRANSFER = [\"download\", \"download-to\"];"
        ));
        // **What the user picked travels with the act** — `Download` M1 and
        // M2, and it is the row's path and never its name (`File Browser` N2).
        // **Every row chosen goes** (the thirteenth notes, note 6).
        assert!(
            code.contains("aTransferIsAskedFor(act, server, theChosenPaths());"),
            "the helper hands over the server and what was picked, by its path"
        );
        assert!(
            the_pages_function("function theChosenPaths() {").contains("row.path.bytes"),
            "the chosen rows go by their paths"
        );
        assert!(
            !code.contains("[row.name.bytes]"),
            "a name was sent where a path belongs"
        );
        assert!(
            code.contains("function theChosenRow()"),
            "the one lookup of the chosen row"
        );
        // **Step 10c piece 4: the confirmed upload runs from this screen**,
        // through `src/uploading.rs`, and no longer goes to Transfers.
        assert!(code.contains("theUploadIsConfirmed(ask.chosen);"), "the confirmed upload");
        assert!(!code.contains("aTransferIsStarted(\"upload\")"), "the upload still goes to Transfers");
        assert!(code.contains("() => aTransferIsStarted(\"download\")"), "the toolbar's Download");
        assert!(code.contains("aTransferIsStarted(item.act);"), "the menus' Download and Download to…");
        for answered_here in ["anActNeedsAConnection(\"upload\")", "anActNeedsAConnection(\"download\")"] {
            assert!(!code.contains(answered_here), "{answered_here} is still answered on this screen");
        }
        // The menus still carry both items, so the list above names real acts.
        let working = include_str!("files.rs");
        assert!(working.contains("item(Words::plain(words::DOWNLOAD), \"download\")"));
        assert!(working.contains("item(Words::plain(words::DOWNLOAD_TO), \"download-to\")"));
    }

    /// **The dependency check is opened on the chosen file, never on a bare
    /// server.** Piece 9's panel opened with no file says it does not know
    /// which file to check, and that is what the menu item and the toolbar
    /// control did until the repair of 2026-09-13.
    #[test]
    fn the_dependency_check_is_opened_on_the_chosen_file() {
        let code: String = the_page()
            .lines()
            .filter(|line| {
                let line = line.trim_start();
                !(line.starts_with("//") || line.starts_with("*"))
            })
            .collect::<Vec<_>>()
            .join("\n");
        assert!(
            code.contains("import { theCheckIsOpenedOn } from \"./the-dependency-check.js\""),
            "the page does not import the dependency check's own way in"
        );
        assert!(
            !code.contains("THE_DEPENDENCY_CHECK") && !code.contains("thePanelIsOpenedOver"),
            "**the page opens the bare dependency check panel**, which hands it \
             a server and no file"
        );
        assert_eq!(
            code.matches("theCheckIsOpenedOn(server, row.path.bytes)").count(),
            1,
            "the one helper hands over the chosen row's path"
        );
        assert_eq!(
            code.matches("theCheckIsOpenedForTheChosen()").count(),
            3,
            "the helper's own definition, and the toolbar's Check dependencies \
             and the right-click menu's item, both of which go through it"
        );
    }

    /// **The page adds nothing to `index.html`**, which is the layout rule, and
    /// the icons are made here rather than referred to.
    #[test]
    fn the_page_makes_its_own_icons_rather_than_naming_a_symbol() {
        let page: String = the_page()
            .lines()
            .filter(|line| !line.trim_start().starts_with("//"))
            .collect::<Vec<_>>()
            .join("\n");
        assert!(
            !page.contains("<use") && !page.contains("href=\"#i-"),
            "**a symbol definition would be markup in `index.html` and a piece \
             adds nothing to that file**"
        );
        assert!(!page.contains("innerHTML"), "H32: values are content, never markup");
        // **The one menu is placed the way `web/servers.js`'s is**: two custom
        // properties the stylesheet reads, which are values and not
        // declarations — and **a folder size's hover line the same way**
        // (the twenty-third notes, note 2). No other use of the element's
        // style is allowed.
        assert_eq!(
            page.matches(".style.").count(),
            page.matches(".style.setProperty(\"--fctx-").count()
                + page.matches(".style.setProperty(\"--fsz-").count(),
            "H33: no inline style"
        );
        assert!(!page.contains("setAttribute(\"style\""), "H33: no inline style");
        assert!(
            !page.contains("setInterval") && !page.contains("setTimeout"),
            "P9 and P10: nothing here polls and nothing here repeats on a timer"
        );
        let markup = include_str!("../../web/index.html");
        for named in ["files.js", "files.css", "flist", "ftools", "crumbs", "fbar"] {
            assert!(
                !markup.contains(named),
                "the shared document names `{named}`. **A piece adds nothing at \
                 all to `index.html`** — every element this screen has is made \
                 in this module"
            );
        }
    }

    // ════════════════════════════════════════════════════════════════════════
    // The step 10a round 1 audit of these pieces: one test per must-do that a
    // test can catch, each one that would have failed before its fix
    // ════════════════════════════════════════════════════════════════════════

    fn the_page_code() -> String {
        the_page()
            .lines()
            .filter(|line| {
                let line = line.trim_start();
                !(line.starts_with("//") || line.starts_with('*'))
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn the_pages_function(opening: &str) -> String {
        the_page_code()
            .split(opening)
            .nth(1)
            .and_then(|rest| rest.split("\n}").next())
            .unwrap_or_else(|| panic!("the page has {opening}"))
            .to_string()
    }

    /// **Must-do 20: one menu, at the pointer, no caption, closed by a press
    /// elsewhere or by Escape.**
    #[test]
    fn the_right_click_menu_is_a_real_menu() {
        let code = the_page_code();
        assert!(!code.contains("mcap"), "the drawing's side-by-side caption is drawn");
        assert!(!code.contains("className = \"menus\""), "the whole pane is shaded for a menu");
        let shown = the_pages_function("function theRightClickMenuIsShown(row, press) {");
        assert!(shown.contains("press.clientX") && shown.contains("press.clientY"));
        assert!(shown.contains("\"mousedown\"") && shown.contains("\"Escape\""));
        assert!(the_menus().iter().all(|menu| !format!("{menu:?}").contains("Right-click")));
        let sheet = include_str!("../../web/files.css");
        assert!(sheet.contains(".fctx {") && sheet.contains("var(--fctx-x"));
    }

    /// **Must-do 21: the browser looks unavailable while acts are unavailable**,
    /// through the drawing's own `.files.offline`, and it comes off again.
    #[test]
    fn files_looks_unavailable_while_its_acts_are() {
        let toolbar = the_pages_function("function theToolbarIsDrawn(answer) {");
        assert!(toolbar.contains("host.classList.toggle(\"offline\", answer.unavailable.length > 0)"));
        assert!(!the_page_code().contains("dataset[`unavailable"), "the attribute nothing reads is still written");
        let drawing = include_str!("../../web/the-drawing.css");
        assert!(drawing.contains(".files.offline .flist"));
    }

    /// **Must-do 22: both panels are opened on the row's exact bytes.**
    #[test]
    fn both_panels_are_opened_on_the_rows_bytes() {
        let code = the_page_code();
        assert!(!code.contains("row.path.shown"), "a panel is opened on the display name");
    }

    /// **Must-do 23: moving to another server starts its browser fresh**, and a
    /// folder another screen handed over still wins.
    #[test]
    fn another_server_starts_its_browser_fresh() {
        let showing = the_pages_function("  itIsShowing(which) {");
        let fresh = showing.find("folder = null;").expect("the last server's folder is let go of");
        // **The hand-over is taken in one place**, which a hand-over while
        // Files shows takes too (the Creator's ninth notes, note 5).
        let asked = showing.find("theFolderAskedForIsTaken();").expect("a folder handed over is taken");
        assert!(fresh < asked, "the folder another screen handed over does not win");
        assert!(showing.contains("theChoiceIsLetGo();"));
        assert!(the_pages_function("function theChoiceIsLetGo() {").contains("chosen = [];"));
        assert!(the_pages_function("function theFolderAskedForIsTaken() {").contains("folder = theFolderAskedFor;"));
    }

    /// **Must-do 24: the delete box's tick is saved through the shared
    /// service**, and a refusal is said.
    #[test]
    fn the_delete_boxs_tick_is_saved() {
        let drawn = the_pages_function("function theDeleteBoxIsDrawn(reading, victims) {");
        assert!(drawn.contains("\"the_delete_warning_was_switched_off\""));
        let working = the_working_half();
        let command = working
            .split("pub fn the_delete_warning_was_switched_off(")
            .nth(1)
            .and_then(|rest| rest.split("\n}").next())
            .expect("the command");
        assert!(command.contains("crate::surface::said_once::a_warning_was_switched_off("));
        assert!(command.contains("the_delete_warnings_suppression_row()"));
        assert!(command.contains("words::THE_SETTING_DID_NOT_SAVE"));
        assert!(include_str!("mod.rs").contains("files::the_delete_warning_was_switched_off,"));
        assert!(the_page_code().contains("what !== \"the-setting-did-not-save\""));
    }

    /// **Must-do 26: the delete key with nothing chosen does nothing.**
    #[test]
    fn delete_with_nothing_chosen_asks_nothing() {
        // **The control says so in one line and the key does nothing**
        // (`Design/File Browser § What the screen says around an act`), and
        // neither asks for the box.
        let asked = the_pages_function("function theDeleteIsAskedAbout(fromTheControl) {");
        let nothing_chosen = asked.find("if (chosen.length === 0) {").expect("nothing chosen is met first");
        let returns = asked[nothing_chosen..].find("return;").expect("and returns") + nothing_chosen;
        let invokes = asked.find("\"the_delete_of\"").expect("the box is asked for");
        assert!(returns < invokes);
        assert!(asked[nothing_chosen..returns].contains("fromTheControl"), "the key says something too");
    }

    /// **Must-do 27: the move question is a whole question** — the destination
    /// as a value where one is known, and no trailing *into* where none is.
    #[test]
    fn the_move_question_is_a_whole_question() {
        use smoozy_state::identities::{Identity, IdentityKind};
        let jobs = vec![AJobNamed {
            job: Identity::issue(IdentityKind::Job).expect("randomness"),
            stands: HowTheJobStands::Rendering,
            the_members_reached: Vec::new(),
            the_set_was_the_widest_case: false,
        }];
        let victims = vec![b"a.png".to_vec(), b"b.png".to_vec()];
        let into = the_live_job_box_from("move", &jobs, &[], &victims, Some(b"assets/"))
            .expect("a box");
        assert_eq!(into.question.values(), vec!["2", "assets/"]);
        assert!(into.question.pieces.last().expect("a piece").joins, "the mark goes against the folder");
        let nowhere = the_live_job_box_from("move", &jobs, &[], &victims, None).expect("a box");
        assert!(!nowhere.question.said().ends_with("into"), "the question trails off: {}", nowhere.question.said());
    }

    /// **Must-do 19: one sentence for an act that cannot instruct the server,
    /// and the five pieces all name the one constant.**
    ///
    /// **The Queue joined the four at step 10b piece 8**, and the sentence was
    /// reworded on 2026-09-18 by the build runner's ruling. **Its old first
    /// half is kept on the forbidden list below and the new one is added to
    /// it**: a screen that grew its own copy would read right and drift, which
    /// is what the single constant exists to stop.
    #[test]
    fn the_no_connection_sentence_is_one_constant() {
        // **Files stopped saying it at step 10c piece 4**, which made its
        // listing and its upload real; it says the All servers screen's own
        // sentence for the acts it still owes, and writes no third. **Since
        // step 13's final words sweep (2026-10-28) the two constants read the
        // same words, *Not available yet.***, and stay two constants; this
        // test no longer asks that their words differ.
        assert_eq!(words::NOT_BUILT_YET, crate::surface::servers::words::NOT_BUILT_YET);
        for (name, source) in [
            ("render_settings.rs", include_str!("render_settings.rs")),
            ("set_up.rs", include_str!("set_up.rs")),
            ("the_dependency_check.rs", include_str!("the_dependency_check.rs")),
            ("the_queue.rs", include_str!("the_queue.rs")),
        ] {
            assert!(
                source.contains("crate::surface::files::A_CONNECTION_IS_NOT_BUILT"),
                "{name} does not say the one sentence"
            );
            let working = source.split("#[cfg(test)]").next().unwrap_or_default();
            for old in [
                "nothing in this copy opens a connection",
                "nothing is connected to the server",
                "Smoozy cannot do this yet. It needs a connection",
                "Smoozy cannot do this yet. Sending instructions",
            ] {
                assert!(!working.contains(old), "{name} still says {old:?}");
            }
        }
    }

    /// **Must-do 25: Backspace and Delete are not the browser's while a panel
    /// covers it.**
    #[test]
    fn the_browsers_keys_step_aside_for_an_open_panel() {
        let keys = the_pages_function("function theKeysAreLive() {");
        assert_eq!(keys.matches("if (aPanelCoversTheBrowser()) return;").count(), 2);
        let covers = the_pages_function("function aPanelCoversTheBrowser() {");
        assert!(covers.contains("aPanelIsOpenOver(server)"), "the page keeps its own copy of which panel is open");
        assert!(include_str!("../../web/a-server.js").contains("export function aPanelIsOpenOver("));
    }

    /// **Must-do 7 of step 10b round 1 of *jobs, history and downloads*: the
    /// Files screen never says Smoozy cannot connect.** Step 10b piece 5 built
    /// connecting, and a server the user has just watched connect was told it
    /// could not. **This failed before the fix.**
    #[test]
    fn the_unlisted_server_says_what_is_missing_and_not_that_it_cannot_connect() {
        for said in [words::NOTHING_LISTED_LEAD, words::NOTHING_LISTED_REST] {
            assert!(!said.contains("cannot connect") && !said.contains("not built"), "{said}");
        }
        // **Step 10c piece 4 built the reading**, so the sentence says when
        // Smoozy reads the files rather than that it cannot.
        assert_eq!(words::NOTHING_LISTED_REST, "Files are listed while the server is connected.");
    }

    /// **A refused write is counted and told as failed, by name and with
    /// Smoozy's own sentence, and never as *did not finish*** — `File Browser`
    /// P1 and `File Browser` P6a. The seventh pass on Hyperstack was told only *1 file did not
    /// finish uploading.*
    #[test]
    fn a_refused_upload_is_told_as_failed_and_says_why() {
        let came_to = WhatTheUploadCameTo {
            failed: vec![b"/cube.blend".to_vec()],
            refused: vec![(
                b"/cube.blend".to_vec(),
                "the server said permission was denied".to_string(),
            )],
            ..WhatTheUploadCameTo::default()
        };
        let told = what_it_came_to(&came_to, b"/");
        let said = told.said().replace('\u{a0}', " ");
        assert!(said.contains(words::ONE_COULD_NOT_BE_UPLOADED), "{said}");
        assert!(told.values().contains(&"cube.blend"), "{:?}", told.values());
        assert!(said.contains(crate::surface::transfers::words::CANNOT_WRITE_THERE), "{said}");
        assert!(said.contains(words::SEE_TRANSFERS), "{said}");
        assert!(!said.contains(words::ONE_DID_NOT_FINISH), "{said}");
    }

    /// **Step 10c's twelfth-pass fix piece 2: the list keeps the user's place
    /// when a reading of the same folder comes back without rows between two
    /// that do** — `Design/Whole app § The window`: *"a list drawn again with
    /// the same things in it keeps its place"*. The twelfth pass on Hyperstack
    /// saw the Files list back at its top when the Render Settings panel over
    /// it was collapsed. At the desk the list came back at its top whenever the
    /// same folder answered once without rows (the connection lent nothing, the
    /// folder was not read) and then with them; the ordinary redraws under the
    /// panel kept the place.
    ///
    /// **The answers are the application's own**, composed here and written out
    /// through the serialisation the commands' replies take, so a shape changed
    /// on either side fails here. `web/files.js` runs under Node beside
    /// stand-ins for what it imports, over a document whose list scrolls as a
    /// browser's does: thirty points a row, its offset held inside what its
    /// rows fill. **Skipped, and says so, where no Node is installed.**
    #[test]
    fn the_list_keeps_its_place_across_a_reading_that_came_back_without_rows() {
        let entries: Vec<ListedEntry> =
            (0..150).map(|n| an_entry(&format!("frame_{n:04}.png"), Kind::File, 1000 + n)).collect();
        let found = |entries: &[ListedEntry]| WhatTheReadingFound::TheFolder { entries: entries.to_vec(), free: Some(1 << 30) };
        let here: &[u8] = b"/home/ubuntu";
        let there: &[u8] = b"/home/ubuntu/renders";
        let mut reading = the_tab("row", here, None, a_folder_being_read(here), None, None, false);
        reading.a_reading_follows = true;
        let mut reading_there = the_tab("row", there, None, a_folder_being_read(there), None, None, false);
        reading_there.a_reading_follows = true;
        let one = |value: &TheBrowser| serde_json::to_string(value).expect("the answer serialises");
        let answers = format!(
            r#"{{ "reading": {}, "rows": {}, "not_lent": {}, "not_listed": {}, "reading_there": {}, "rows_there": {} }}"#,
            one(&reading),
            one(&the_tab_after_the_reading(TheOrder::default(), "row", here, None, found(&entries), None, false)),
            one(&the_tab_after_the_reading(TheOrder::default(), "row", here, None, WhatTheReadingFound::NothingWasLent, None, false)),
            one(&the_tab_after_the_reading(
                TheOrder::default(),
                "row",
                here,
                None,
                WhatTheReadingFound::NotListed(Words::plain(words::THE_FOLDER_WAS_NOT_READ)),
                None,
                false,
            )),
            one(&reading_there),
            one(&the_tab_after_the_reading(TheOrder::default(), "row", there, None, found(&entries), None, false)),
        );

        let folder = std::env::temp_dir().join(format!("smoozy-files-page-harness-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a scratch folder");
        let web = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        for page in ["files.js", "the-value-drawn.js"] {
            std::fs::copy(web.join(page), folder.join(page)).expect("the page is copied");
        }
        for (name, text) in THE_LISTS_STAND_INS {
            std::fs::write(folder.join(name), text).expect("a stand-in is written");
        }
        std::fs::write(folder.join("answers.json"), answers).expect("the answers are written");
        let ran = std::process::Command::new("node").arg(folder.join("harness.js")).current_dir(&folder).output();
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the Files page was not run");
            let _ = std::fs::remove_dir_all(&folder);
            return;
        };
        let _ = std::fs::remove_dir_all(&folder);
        assert!(ran.status.success(), "the page did not run: {}", String::from_utf8_lossy(&ran.stderr));
        let said = String::from_utf8_lossy(&ran.stdout).trim().to_string();
        let expected = [
            // 150 rows and the headers' own row, at thirty points each.
            "drawn: 151 rows, at 0",
            "scrolled: at 1500, frame_0049.png at the top",
            "read again: at 1500, frame_0049.png at the top",
            "read again, nothing lent between: at 1500, frame_0049.png at the top",
            "read again, not read between: at 1500, frame_0049.png at the top",
            "another folder: 151 rows, at 0",
        ]
        .join("\n");
        assert_eq!(said, expected);
    }

    /// **What `web/files.js` imports, standing in**, and the harness. The
    /// document is the least the page needs — the shape `src/surface/the_queue.rs`'s
    /// page harness uses — and **its elements scroll as a browser's do**: a
    /// child sits thirty points below the one before it, and an offset is held
    /// between nought and what the children fill past a height of three hundred,
    /// so a list emptied is a list at its top.
    const THE_LISTS_STAND_INS: [(&str, &str); 8] = [
        ("package.json", r#"{ "type": "module" }"#),
        ("event-route.js", r#"export function whenTold() {}"#),
        (
            "a-server.js",
            r#"export const FILES = "files";
export function aTabIsRegistered(tab) { globalThis.theTab = tab; }
export function aPanelIsOpenOver() { return false; }"#,
        ),
        ("the-dependency-check.js", r#"export function theCheckIsOpenedOn() {}"#),
        ("transfers.js", r#"export function aTransferIsAskedFor() {}
export function theTransfersAreShown() {}"#),
        ("render-settings.js", r#"export async function theSettingsAreOpenedOn() {}"#),
        ("the-underway-shape.js", r#"export function theMarkOverAPane() { return document.createElement("div"); }"#),
        (
            "harness.js",
            r#"import { readFileSync } from "node:fs";
const ROW = 30;
const HEIGHT = 300;
function aTest(selector) {
  const found = /^([a-z0-9]*)((?:\.[\w-]+)*)$/.exec(selector.trim());
  if (!found) return () => false;
  const [, tag, classes] = found;
  const names = classes ? classes.split(".").filter(Boolean) : [];
  return (element) => {
    if (!element || !element.classList) return false;
    if (tag && element.tag !== tag) return false;
    return names.every((name) => element.classList.contains(name));
  };
}
function everyUnder(element, test, into) {
  for (const child of element.children || []) {
    if (test(child)) into.push(child);
    everyUnder(child, test, into);
  }
  return into;
}
function anElement(tag) {
  const names = new Set();
  const listeners = new Map();
  let offset = 0;
  const element = {
    tag, children: [], parentNode: null, dataset: {}, style: { setProperty() {} }, hidden: false, text: "", type: "", id: "", title: "",
    classList: {
      add: (...more) => more.forEach((one) => names.add(one)),
      remove: (...less) => less.forEach((one) => names.delete(one)),
      toggle: (one, on) => { if (on === undefined ? !names.has(one) : on) names.add(one); else names.delete(one); },
      contains: (one) => names.has(one),
    },
    set className(value) { names.clear(); String(value).split(" ").filter(Boolean).forEach((one) => names.add(one)); },
    get className() { return Array.from(names).join(" "); },
    set textContent(value) { for (const one of element.children) one.parentNode = null; element.children = []; element.text = value == null ? "" : String(value); element.scrollTop = offset; },
    get textContent() { return element.text + element.children.map((one) => one.textContent || one.text || "").join(""); },
    get scrollTop() { return offset; },
    set scrollTop(value) { offset = Math.max(0, Math.min(Number(value) || 0, element.children.length * ROW - HEIGHT)); },
    get offsetTop() { return element.parentNode ? element.parentNode.children.indexOf(element) * ROW : 0; },
    get offsetHeight() { return ROW; },
    appendChild(child) {
      if (child.parentNode && child.parentNode.removeChild) child.parentNode.removeChild(child);
      child.parentNode = element;
      element.children.push(child);
      return child;
    },
    insertBefore(child, before) {
      if (child.parentNode && child.parentNode.removeChild) child.parentNode.removeChild(child);
      child.parentNode = element;
      const at = element.children.indexOf(before);
      if (at < 0) element.children.push(child); else element.children.splice(at, 0, child);
      return child;
    },
    removeChild(child) { element.children = element.children.filter((one) => one !== child); child.parentNode = null; element.scrollTop = offset; return child; },
    replaceChildren(...fresh) { for (const one of element.children) one.parentNode = null; element.children = []; fresh.forEach((one) => element.appendChild(one)); element.scrollTop = offset; },
    replaceWith(fresh) {
      const holder = element.parentNode;
      if (!holder) return;
      holder.children = holder.children.map((one) => (one === element ? fresh : one));
      fresh.parentNode = holder;
      element.parentNode = null;
    },
    remove() { if (element.parentNode) element.parentNode.removeChild(element); },
    setAttribute() {},
    getAttribute() { return null; },
    addEventListener(kind, handler) { const held = listeners.get(kind) || []; held.push(handler); listeners.set(kind, held); },
    focus() {},
    contains(other) { let at = other; while (at) { if (at === element) return true; at = at.parentNode; } return false; },
    closest() { return null; },
    querySelector(selector) { return everyUnder(element, aTest(selector), [])[0] || null; },
    querySelectorAll(selector) { return everyUnder(element, aTest(selector), []); },
    getBoundingClientRect() { return { left: 0, top: 0, bottom: 0, width: 0, height: 0 }; },
  };
  return element;
}
const body = anElement("body");
globalThis.document = {
  createElement: anElement,
  createElementNS: (_, tag) => anElement(tag),
  createTextNode: (text) => ({ text, children: [] }),
  addEventListener() {},
  removeEventListener() {},
  body,
};
const answers = JSON.parse(readFileSync("answers.json", "utf8"));
let plan = { tab: answers.reading, reading: answers.rows };
globalThis.window = {
  innerWidth: 1000, innerHeight: 800,
  __TAURI__: {
    core: {
      invoke(command) {
        if (command === "the_files_tab") return Promise.resolve(plan.tab);
        if (command === "the_folder_is_read") return Promise.resolve(plan.reading);
        return new Promise(() => {});
      },
    },
  },
};

const page = await import("./files.js");
const host = anElement("div");
body.appendChild(host);
globalThis.theTab.itIsDrawn(host);
const settled = () => new Promise((done) => setTimeout(done, 0));
// **What `web/a-server.js` does at every post of the server's own screen**: the
// showing tab is shown again, and it asks for its folder again.
const aPost = async (next) => {
  plan = next;
  globalThis.theTab.itIsShowing("row");
  await settled();
  await settled();
};
const list = () => host.querySelector("div.flist");
const atTheTop = () => {
  const shown = list();
  for (const one of shown.children) {
    if (!one.classList.contains("frow") || !/^frame/.test(one.textContent)) continue;
    if (one.offsetTop + one.offsetHeight > shown.scrollTop) return one.textContent.slice(0, 14);
  }
  return "nothing";
};
const say = (what) => console.log(what + ": at " + list().scrollTop + ", " + atTheTop() + " at the top");

await aPost({ tab: answers.reading, reading: answers.rows });
console.log("drawn: " + list().children.filter((one) => one.classList.contains("frow")).length + " rows, at " + list().scrollTop);
list().scrollTop = 1500;
say("scrolled");
await aPost({ tab: answers.reading, reading: answers.rows });
say("read again");
await aPost({ tab: answers.reading, reading: answers.not_lent });
await aPost({ tab: answers.reading, reading: answers.rows });
say("read again, nothing lent between");
await aPost({ tab: answers.reading, reading: answers.not_listed });
await aPost({ tab: answers.reading, reading: answers.rows });
say("read again, not read between");
// **Another folder, opened the way Set up's folder link opens one**, holds
// rows of the same names: a place kept for the last folder is not put on it.
page.theBrowserIsOpenedAt("row", Array.from(new TextEncoder().encode("/home/ubuntu/renders")));
await aPost({ tab: answers.reading_there, reading: answers.rows_there });
console.log("another folder: " + list().children.filter((one) => one.classList.contains("frow")).length + " rows, at " + list().scrollTop);
"#,
        ),
    ];

    /// **Files lists again the moment a connection is handed over** — `File
    /// Browser` Row 198, *"the listing is read again when the connection comes
    /// back"*, and `File Browser` L17. The Creator's eighteenth notes, note 14: disconnected
    /// and connected again from the pane, Files showed nothing *"for quite a
    /// while"*. **The cause, timed at the desk**
    /// (`tests/files_list_again_once_the_connection_is_back.rs`): Files asks as
    /// the server reads *connected*, which `Servers` publishes before its own
    /// first checks and so before the connection is handed over, so that ask
    /// has nothing to read on; the next post the page heard was the work column
    /// moving after step 7. At a quarter of a second each way the folder
    /// listed 30 s after the press, 15 s after the hand-over.
    ///
    /// **The real `web/files.js` under Node**, the list test's stand-ins beside
    /// it and an event route that keeps its handlers: Files showing a server
    /// that reads *connected* with nothing lent yet, then the hand-over told
    /// for another server, for this one, and for this one again once Files is
    /// put away. **The event is the application's own** —
    /// [`crate::surface::render_settings::THE_CONNECTION_IS_LENT`], posted at
    /// every hand-over, so a page naming another string fails here.
    #[test]
    fn files_reads_its_folder_again_when_the_connection_is_handed_over() {
        let here: &[u8] = b"/home/ubuntu/renders";
        let nothing_lent = the_tab(
            "row",
            here,
            None,
            TheListing::NothingHasBeenListed {
                lead: Prose::new(words::NOTHING_LISTED_LEAD),
                rest: Prose::new(words::NOTHING_LISTED_REST),
            },
            Some(ObservedValue::Connected),
            None,
            false,
        );
        assert!(!nothing_lent.a_reading_follows, "nothing lent is nothing to read on");
        let mut reading = the_tab("row", here, None, a_folder_being_read(here), Some(ObservedValue::Connected), None, false);
        reading.a_reading_follows = true;
        let entries: Vec<ListedEntry> =
            (0..40).map(|n| an_entry(&format!("frame_{n:04}.png"), Kind::File, 1000 + n)).collect();
        let rows = the_tab_after_the_reading(
            TheOrder::default(),
            "row",
            here,
            None,
            WhatTheReadingFound::TheFolder { entries, free: Some(1 << 30) },
            Some(ObservedValue::Connected),
            false,
        );
        let one = |value: &TheBrowser| serde_json::to_string(value).expect("the answer serialises");
        let answers = format!(
            r#"{{ "lent": {}, "nothing_lent": {}, "reading": {}, "rows": {} }}"#,
            serde_json::to_string(crate::surface::render_settings::THE_CONNECTION_IS_LENT).expect("the kind"),
            one(&nothing_lent),
            one(&reading),
            one(&rows),
        );

        let folder = std::env::temp_dir().join(format!("smoozy-files-handed-over-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a scratch folder");
        let web = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        for page in ["files.js", "the-value-drawn.js"] {
            std::fs::copy(web.join(page), folder.join(page)).expect("the page is copied");
        }
        for (name, text) in THE_LISTS_STAND_INS {
            // **The list test's document, and this test's own scenario after
            // it**; and an event route that keeps what each page hears.
            let text = match name {
                "event-route.js" => {
                    "export function whenTold(kind, handler) { ((globalThis.told ||= {})[kind] ||= []).push(handler); }"
                        .to_string()
                }
                "harness.js" => {
                    let document = text.split("const answers = ").next().expect("the list test's document");
                    format!("{document}{THE_HAND_OVER_SCENARIO}")
                }
                _ => text.to_string(),
            };
            std::fs::write(folder.join(name), text).expect("a stand-in is written");
        }
        std::fs::write(folder.join("answers.json"), answers).expect("the answers are written");
        let ran = std::process::Command::new("node").arg(folder.join("harness.js")).current_dir(&folder).output();
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the Files page was not run");
            let _ = std::fs::remove_dir_all(&folder);
            return;
        };
        let _ = std::fs::remove_dir_all(&folder);
        assert!(ran.status.success(), "the page did not run: {}", String::from_utf8_lossy(&ran.stderr));
        let said = String::from_utf8_lossy(&ran.stdout).trim().to_string();
        let expected = [
            "connected, nothing lent: asked 1, 0 rows",
            "handed over on another server: asked 0, 0 rows",
            "handed over: asked 1, read 1, 40 rows",
            "handed over while put away: asked 0",
        ]
        .join("\n");
        assert_eq!(said, expected);
    }

    /// **What [`files_reads_its_folder_again_when_the_connection_is_handed_over`]
    /// does with the page**, after [`THE_LISTS_STAND_INS`]' own document: how
    /// many times the page asks for its tab and its rows, and how many rows it
    /// draws, at each telling.
    const THE_HAND_OVER_SCENARIO: &str = r#"const answers = JSON.parse(readFileSync("answers.json", "utf8"));
let plan = { tab: answers.nothing_lent, reading: answers.rows };
const asked = { tab: 0, read: 0 };
globalThis.window = {
  innerWidth: 1000, innerHeight: 800,
  __TAURI__: {
    core: {
      invoke(command) {
        if (command === "the_files_tab") { asked.tab += 1; return Promise.resolve(plan.tab); }
        if (command === "the_folder_is_read") { asked.read += 1; return Promise.resolve(plan.reading); }
        return new Promise(() => {});
      },
    },
  },
};
await import("./files.js");
const host = anElement("div");
body.appendChild(host);
globalThis.theTab.itIsDrawn(host);
const settled = () => new Promise((done) => setTimeout(done, 0));
const settle = async () => { for (let n = 0; n < 4; n += 1) await settled(); };
const rows = () =>
  host.querySelector("div.flist").children.filter((one) => one.classList.contains("frow") && /^frame/.test(one.textContent)).length;
const tell = async (server) => {
  for (const handler of (globalThis.told || {})[answers.lent] || []) handler({ said: { server } });
  await settle();
};
const taken = () => { const now = { ...asked }; asked.tab = 0; asked.read = 0; return now; };

// **What `web/a-server.js` does as the server reads connected**: the showing
// tab is shown again, and the application has lent nothing yet.
globalThis.theTab.itIsShowing("row");
await settle();
let now = taken();
console.log(`connected, nothing lent: asked ${now.tab}, ${rows()} rows`);
plan = { tab: answers.reading, reading: answers.rows };
await tell("another-server");
now = taken();
console.log(`handed over on another server: asked ${now.tab}, ${rows()} rows`);
await tell("row");
now = taken();
console.log(`handed over: asked ${now.tab}, read ${now.read}, ${rows()} rows`);
globalThis.theTab.itIsPutAway();
await tell("row");
now = taken();
console.log(`handed over while put away: asked ${now.tab}`);
"#;

    /// **The Creator's fifth notes, piece R: the file manager's own controls
    /// never move when a `.blend` is chosen** — `Design/File Browser`: choosing
    /// one *"puts Render settings… and Check dependencies… at the end of the
    /// toolbar, after Delete, so the file manager's own controls never move"*
    /// (Creator-stated 2026-10-08, the fifth notes, note 1).
    ///
    /// The toolbar is drawn by `web/files.js` itself, under Node and beside the
    /// list test's stand-ins, for the same folder three times: nothing chosen, a
    /// `.blend` chosen, a picture chosen. Each control's place from the left is
    /// laid out the way the drawing's stylesheet lays the row out — seven
    /// points between controls, an icon twenty-eight wide, a divider seven with
    /// its margins, a word button as wide as its words, and the two words'
    /// holder adding nothing of its own and gone while no `.blend` is chosen.
    /// **Skipped, and says so, where no Node is installed.**
    #[test]
    fn the_toolbars_icons_keep_their_places_when_a_blend_is_chosen() {
        let Some(said) = the_toolbar_as_drawn() else { return };
        let places = |case: &str, what: &str| -> Vec<(String, i64)> {
            said[case][what]
                .as_array()
                .unwrap_or_else(|| panic!("{case} was drawn"))
                .iter()
                .map(|one| (one[0].as_str().unwrap_or_default().to_string(), one[1].as_i64().unwrap_or(-1)))
                .collect()
        };

        let toolbar = the_toolbar(None);
        let word = |prose: &Prose| serde_json::to_value(prose).ok().and_then(|one| one.as_str().map(String::from)).unwrap_or_default();
        let icons = places("nothing", "icons");
        let names: Vec<String> = icons.iter().map(|(name, _)| name.clone()).collect();
        let expected: Vec<String> = [&toolbar.upload, &toolbar.download, &toolbar.new_folder, &toolbar.rename, &toolbar.cut, &toolbar.copy, &toolbar.delete]
            .into_iter()
            .map(word)
            .collect();
        assert_eq!(names, expected);
        assert_eq!(places("blend", "icons"), icons, "a .blend chosen moves the file manager's own controls");
        assert_eq!(places("picture", "icons"), icons, "a picture chosen moves the file manager's own controls");

        assert!(places("nothing", "words").is_empty(), "the two words are drawn with nothing chosen");
        assert!(places("picture", "words").is_empty(), "the two words are drawn for a picture");
        let delete = icons.last().expect("Delete").1;
        let pair = places("blend", "words");
        let names: Vec<String> = pair.iter().map(|(name, _)| name.clone()).collect();
        assert_eq!(names, [word(&toolbar.render_settings), word(&toolbar.check_dependencies)]);
        assert!(pair.iter().all(|(_, left)| *left > delete), "the two words are not after Delete: {pair:?}");
    }

    /// **The Creator's sixth notes, note 1: the toolbar is as tall with a
    /// `.blend` chosen as without one, so the list below never moves** —
    /// `Design/File Browser`: the two words *"go last so nothing else moves
    /// when they appear."* Before the piece a word button had no height of its
    /// own, about 28.6 points against an icon's 26, and the row grew when the
    /// two appeared.
    ///
    /// The toolbar is drawn by `web/files.js` under Node for the three cases;
    /// each control is as tall as `web/the-drawing.css` makes it, read from
    /// the stylesheet itself, and the row is as tall as its tallest control.
    /// **Skipped, and says so, where no Node is installed.**
    #[test]
    fn the_toolbar_is_as_tall_with_a_blend_chosen_as_without() {
        let Some(said) = the_toolbar_as_drawn() else { return };
        let tall = |case: &str| said[case]["height"].as_f64().unwrap_or_else(|| panic!("{case} was measured"));
        assert_eq!(tall("nothing"), the_height_in_the_toolbar("ib"), "the row is not as tall as its icons");
        assert_eq!(tall("blend"), tall("nothing"), "the toolbar grows when a .blend is chosen");
        assert_eq!(tall("picture"), tall("nothing"), "the toolbar grows when a picture is chosen");
    }

    /// **How tall a control is in the toolbar, read from the stylesheet**:
    /// every rule for `.ib`, or for `.mini` and `.ftools .mini`, in order. A
    /// height the rules set is the height, since every box is measured border
    /// to border; otherwise the words' line plus the padding and border above
    /// and below.
    fn the_height_in_the_toolbar(control: &str) -> f64 {
        let css = include_str!("../../web/the-drawing.css");
        let mut text = String::new();
        let mut rest = css;
        while let Some(at) = rest.find("/*") {
            text.push_str(&rest[..at]);
            rest = rest[at..].find("*/").map(|end| &rest[at + end + 2..]).unwrap_or("");
        }
        text.push_str(rest);
        let selectors: &[&str] = if control == "ib" { &[".ib"] } else { &[".mini", ".ftools .mini"] };
        let points = |value: &str| -> f64 {
            let value = value.trim();
            if let Some(name) = value.strip_prefix("var(").and_then(|one| one.strip_suffix(')')) {
                let declared = text.split(&format!("{name}:")).nth(1).and_then(|one| one.split(';').next());
                return declared.map(|one| one.trim().trim_end_matches("px").parse().unwrap_or(0.0)).unwrap_or(0.0);
            }
            value.trim_end_matches("px").parse().unwrap_or(0.0)
        };
        let (mut height, mut font, mut padding, mut border) = (None, points("var(--t-base)"), 0.0, 0.0);
        for chunk in text.split('}') {
            let Some(open) = chunk.rfind('{') else { continue };
            let selector = chunk[..open].rsplit('{').next().unwrap_or("");
            if !selector.split(',').any(|one| selectors.contains(&one.trim())) {
                continue;
            }
            for declaration in chunk[open + 1..].split(';') {
                let Some((name, value)) = declaration.split_once(':') else { continue };
                let first = value.split_whitespace().next().unwrap_or("");
                match name.trim() {
                    "height" => height = Some(points(value)),
                    "font-size" => font = points(value),
                    "padding" | "padding-top" | "padding-block" => padding = points(first),
                    "border" | "border-width" => border = points(first),
                    _ => {}
                }
            }
        }
        // **The line height is the page's own**, `body`'s 1.62.
        height.unwrap_or(font * 1.62 + 2.0 * padding + 2.0 * border)
    }

    /// **The toolbar as `web/files.js` draws it** for one folder three times —
    /// nothing chosen, a `.blend` chosen, a picture chosen — each control's
    /// place from the left and the row's height. `None` where no Node is
    /// installed, and it says so.
    fn the_toolbar_as_drawn() -> Option<serde_json::Value> {
        let entries = vec![
            an_entry("kitchen_v14.blend", Kind::File, 297_795_584),
            an_entry("studio_backplate.exr", Kind::File, 100_663_296),
        ];
        let here: &[u8] = b"/home/ubuntu";
        let mut reading = the_tab("row", here, None, a_folder_being_read(here), None, None, false);
        reading.a_reading_follows = true;
        let chosen = |name: Option<&[u8]>| {
            let found = WhatTheReadingFound::TheFolder { entries: entries.clone(), free: Some(1 << 30) };
            the_tab_after_the_reading(TheOrder::default(), "row", here, name, found, None, false)
        };
        let one = |value: &TheBrowser| serde_json::to_string(value).expect("the answer serialises");
        let answers = format!(
            r#"{{ "reading": {}, "nothing": {}, "blend": {}, "picture": {} }}"#,
            one(&reading),
            one(&chosen(None)),
            one(&chosen(Some(b"kitchen_v14.blend"))),
            one(&chosen(Some(b"studio_backplate.exr"))),
        );

        // **Each run its own folder**: two tests draw the toolbar at once.
        static RUNS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        let run = RUNS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let folder = std::env::temp_dir().join(format!("smoozy-files-toolbar-harness-{}-{run}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a scratch folder");
        let web = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        for page in ["files.js", "the-value-drawn.js"] {
            std::fs::copy(web.join(page), folder.join(page)).expect("the page is copied");
        }
        // **The list test's stand-ins and its document**, with this test's own
        // steps in place of the list test's.
        for (name, text) in THE_LISTS_STAND_INS {
            let text = if name == "harness.js" {
                let (document, _) = text.split_once("const answers = JSON.parse").expect("the list harness's document");
                format!("{document}{THE_TOOLBARS_STEPS}")
            } else {
                text.to_string()
            };
            std::fs::write(folder.join(name), text).expect("a stand-in is written");
        }
        std::fs::write(folder.join("answers.json"), answers).expect("the answers are written");
        let heights = serde_json::json!({ "ib": the_height_in_the_toolbar("ib"), "mini": the_height_in_the_toolbar("mini") });
        std::fs::write(folder.join("heights.json"), heights.to_string()).expect("the heights are written");
        let ran = std::process::Command::new("node").arg(folder.join("harness.js")).current_dir(&folder).output();
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the Files page was not run");
            let _ = std::fs::remove_dir_all(&folder);
            return None;
        };
        let _ = std::fs::remove_dir_all(&folder);
        assert!(ran.status.success(), "the page did not run: {}", String::from_utf8_lossy(&ran.stderr));
        let said: serde_json::Value =
            serde_json::from_slice(&ran.stdout).unwrap_or_else(|_| panic!("{}", String::from_utf8_lossy(&ran.stdout)));
        Some(said)
    }

    /// **The toolbar test's steps**, after the list harness's document: each
    /// answer is posted and the toolbar measured, printed as one reading.
    const THE_TOOLBARS_STEPS: &str = r#"const answers = JSON.parse(readFileSync("answers.json", "utf8"));
let plan = { tab: answers.reading, reading: answers.nothing };
globalThis.window = {
  innerWidth: 1000, innerHeight: 800,
  __TAURI__: {
    core: {
      invoke(command) {
        if (command === "the_files_tab") return Promise.resolve(plan.tab);
        if (command === "the_folder_is_read") return Promise.resolve(plan.reading);
        return new Promise(() => {});
      },
    },
  },
};
// **The row is chosen on the page, the way another screen hands one over**
// (`theBrowserIsOpenedAt`): since the thirteenth notes, note 6, the toolbar
// is told the page's own choice as it is drawn, whatever the answer composed
// for.
const page = await import("./files.js");
const bytes = (text) => Array.from(new TextEncoder().encode(text));
const handed = { nothing: null, blend: "kitchen_v14.blend", picture: "studio_backplate.exr" };
const host = anElement("div");
body.appendChild(host);
globalThis.theTab.itIsDrawn(host);
const settled = () => new Promise((done) => setTimeout(done, 0));
const GAP = 7;
// **The drawing's own row**: `.ftools` is a flex row with a gap of seven;
// `.blendonly` is `display: contents`, and `none` under `.notblend`.
function theItemsOf(element, shown) {
  const items = [];
  for (const child of element.children) {
    if (!child.classList || child.hidden) continue;
    if (child.classList.contains("blendonly")) {
      if (shown) items.push(...theItemsOf(child, shown));
      continue;
    }
    items.push(child);
  }
  return items;
}
function theWidthOf(item) {
  if (item.classList.contains("ib")) return 28;
  if (item.classList.contains("tdiv")) return 7;
  if (item.classList.contains("mini")) return 22 + 7 * item.textContent.length;
  if (item.classList.contains("grow") || item.classList.contains("selnote")) return 0;
  return item.children.filter((one) => one.classList && !one.hidden).reduce((sum, one) => sum + theWidthOf(one), 0);
}
// **How tall each control is**, as the stylesheet makes it; the row is as
// tall as its tallest.
const HEIGHTS = JSON.parse(readFileSync("heights.json", "utf8"));
function theHeightOf(item) {
  if (item.classList.contains("ib")) return HEIGHTS.ib;
  if (item.classList.contains("mini")) return HEIGHTS.mini;
  if (item.classList.contains("tdiv") || item.classList.contains("grow") || item.classList.contains("selnote")) return 0;
  return Math.max(0, ...item.children.filter((one) => one.classList && !one.hidden).map(theHeightOf));
}
function theIconsIn(item, left, into) {
  if (item.classList.contains("ib")) into.icons.push([item.title, left]);
  else if (item.classList.contains("mini")) into.words.push([item.textContent, left]);
  else for (const one of item.children) if (one.classList && !one.hidden) theIconsIn(one, left, into);
}
const measured = {};
for (const which of ["nothing", "blend", "picture"]) {
  plan = { tab: answers.reading, reading: answers[which] };
  if (handed[which]) page.theBrowserIsOpenedAt("row", bytes("/home/ubuntu"), bytes(handed[which]));
  globalThis.theTab.itIsShowing("row");
  await settled();
  await settled();
  const tools = host.querySelector("div.ftools") || everyUnder(host, (one) => one.classList && one.classList.contains("ftools"), [])[0];
  const into = { icons: [], words: [], height: 0 };
  if (tools) {
    let left = 0;
    for (const item of theItemsOf(tools, !tools.classList.contains("notblend"))) {
      theIconsIn(item, left, into);
      into.height = Math.max(into.height, theHeightOf(item));
      left += theWidthOf(item) + GAP;
    }
  }
  measured[which] = into;
}
console.log(JSON.stringify(measured));
"#;

    // ════════════════════════════════════════════════════════════════════════
    // *Show hidden files* — `File Browser` §1.6, `File Browser` L19 to `File Browser` L23, Row 520
    // ════════════════════════════════════════════════════════════════════════

    /// A folder holding the staging folder, a dot-file, a dot-name whose bytes
    /// are not text, and two ordinary things.
    fn a_folder_with_hidden_names() -> Vec<ListedEntry> {
        let mut odd = an_entry("x", Kind::File, 7);
        odd.name = EntryName::from_the_machine(b".\xff\xfe".to_vec());
        vec![
            an_entry(".smoozy-staging", Kind::Folder, 0),
            an_entry(".env", Kind::File, 12),
            odd,
            an_entry("renders", Kind::Folder, 0),
            an_entry("scene.blend", Kind::File, 1_000),
        ]
    }

    fn the_listed_rows(browser: &TheBrowser) -> &[ARow] {
        match &browser.listing {
            TheListing::Rows { rows, .. } => rows,
            other => panic!("the folder listed: {other:?}"),
        }
    }

    fn as_read(words: &Words) -> String {
        words
            .pieces
            .iter()
            .map(|piece| match (&piece.said, &piece.value) {
                (Some(said), _) => said.as_str().to_string(),
                (None, Some(value)) => value.as_str().to_string(),
                (None, None) => String::new(),
            })
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// **Tests 1 and 9 of the plan text: unticked, the count leaves every
    /// dot-name out — `.smoozy-staging` and a name that is not text among
    /// them — and ticked, it takes them in, with the staging folder marked as
    /// Smoozy's.** Every row is carried both ways, each saying whether it is
    /// hidden (`File Browser` L19, `File Browser` L21; `File Browser` X12 as
    /// amended).
    #[test]
    fn hidden_names_are_carried_marked_and_left_out_of_the_count_until_ticked() {
        let off = the_tab_after_the_reading(
            TheOrder::default(),
            "a-server",
            b"/home/ubuntu",
            None,
            WhatTheReadingFound::TheFolder { entries: a_folder_with_hidden_names(), free: None },
            Some(ObservedValue::Connected),
            false,
        );
        let hidden: Vec<(Vec<u8>, bool)> =
            the_listed_rows(&off).iter().map(|row| (row.name.bytes.clone(), row.hidden)).collect();
        assert_eq!(hidden.len(), 5, "a hidden row was not carried, so a tick would have to read the server");
        for (name, is_hidden) in &hidden {
            assert_eq!(*is_hidden, name.first() == Some(&b'.'), "{name:?}");
        }
        assert!(!off.bottom.show_hidden_files.ticked, "the box is ticked on a fresh choice");
        assert_eq!(off.bottom.show_hidden_files.label.as_str().replace('\u{a0}', " "), words::SHOW_HIDDEN_FILES);
        assert_eq!(as_read(off.bottom.what_is_in_the_folder.as_ref().unwrap()), "1 file, 1 folder");
        assert_eq!(
            as_read(off.bottom.what_is_in_the_folder_when_the_box_is_turned.as_ref().unwrap()),
            "3 files, 2 folders"
        );

        let on = the_tab_after_the_reading(
            TheOrder::default(),
            "a-server",
            b"/home/ubuntu",
            None,
            WhatTheReadingFound::TheFolder { entries: a_folder_with_hidden_names(), free: None },
            Some(ObservedValue::Connected),
            true,
        );
        assert!(on.bottom.show_hidden_files.ticked);
        assert_eq!(as_read(on.bottom.what_is_in_the_folder.as_ref().unwrap()), "3 files, 2 folders");
        assert_eq!(
            as_read(on.bottom.what_is_in_the_folder_when_the_box_is_turned.as_ref().unwrap()),
            "1 file, 1 folder"
        );
        let staging = the_listed_rows(&on)
            .iter()
            .find(|row| row.name.bytes == b".smoozy-staging")
            .expect("the staging folder is listed");
        assert_eq!(staging.kind.as_str().replace('\u{a0}', " "), words::A_STAGING_FOLDER);
        let renders = the_listed_rows(&on).iter().find(|row| row.name.bytes == b"renders").unwrap();
        assert_eq!(renders.kind.as_str(), words::A_FOLDER, "an ordinary folder was marked as Smoozy's");
    }

    /// **Test 6 and `File Browser` L22: a hidden name handed back as chosen
    /// while the box is off is not chosen** — no *1 selected*, no `.blend`
    /// pair — **and with the box on it is.**
    #[test]
    fn a_hidden_name_is_never_chosen_while_the_list_does_not_show_it() {
        let reading = |shown| {
            the_tab_after_the_reading(
                TheOrder::default(),
                "a-server",
                b"/home/ubuntu",
                Some(b".env"),
                WhatTheReadingFound::TheFolder { entries: a_folder_with_hidden_names(), free: None },
                Some(ObservedValue::Connected),
                shown,
            )
        };
        assert!(reading(false).toolbar.how_many_chosen.is_none(), "a name the list does not show is chosen");
        assert!(reading(true).toolbar.how_many_chosen.is_some());
    }

    /// **Test 7 and `File Browser` L23: a hidden folder reached directly opens
    /// and lists with the box off**, the path bar showing the hidden part as it
    /// is — and a live connection reads it, as for any other folder.
    #[test]
    fn a_hidden_folder_reached_directly_opens_and_lists_with_the_box_off() {
        let folder: &[u8] = b"/home/ubuntu/.cache/job-outputs";
        let tab = the_tab_from(
            WhatThereIs::TheServer(Box::new(a_server_row("a-server")), None),
            "a-server",
            Some(folder.to_vec()),
            None,
            true,
            false,
        );
        assert!(tab.a_reading_follows, "a hidden folder was not read");
        assert_eq!(tab.folder.bytes, folder);
        let listed = the_tab_after_the_reading(
            TheOrder::default(),
            "a-server",
            folder,
            None,
            WhatTheReadingFound::TheFolder {
                entries: vec![an_entry("frame_0001.png", Kind::File, 9), an_entry("frame_0002.png", Kind::File, 9)],
                free: None,
            },
            Some(ObservedValue::Connected),
            false,
        );
        assert_eq!(the_listed_rows(&listed).len(), 2);
        assert_eq!(as_read(listed.bottom.what_is_in_the_folder.as_ref().unwrap()), "2 files, 0 folders");
        assert!(
            listed.path.iter().any(|part| part.shown.as_str() == ".cache"),
            "the path bar does not show the hidden part as it is"
        );
    }

    /// **`File Browser` L21 as amended 2026-10-02, the hidden-row seam: a
    /// hidden folder the login may not read, reached directly with the box off,
    /// still shows its refusal** on its own row in the parent's list (`File
    /// Browser` P2), and is counted — while its hidden neighbours stay out.
    /// Before the amendment the row was left out and the user saw the parent
    /// folder with nothing said.
    #[test]
    fn a_refused_hidden_folder_reached_directly_shows_its_refusal_with_the_box_off() {
        let refused = the_tab_after_the_reading(
            TheOrder::default(),
            "a-server",
            b"/home/ubuntu/.cache",
            None,
            WhatTheReadingFound::ItsRowCarriesTheRefusal {
                parent: b"/home/ubuntu".to_vec(),
                entries: vec![
                    an_entry(".cache", Kind::Folder, 0),
                    an_entry(".env", Kind::File, 12),
                    an_entry("renders", Kind::Folder, 0),
                ],
                free: None,
            },
            Some(ObservedValue::Connected),
            false,
        );
        let rows = the_listed_rows(&refused);
        let cache = rows.iter().find(|row| row.name.bytes == b".cache").expect("the refused row is carried");
        assert!(cache.hidden && cache.refusal.is_some(), "{cache:#?}");
        assert_eq!(
            as_read(refused.bottom.what_is_in_the_folder.as_ref().unwrap()),
            "0 files, 2 folders",
            "the refused hidden folder is counted, and .env is not"
        );
        assert!(the_list_shows_the_row(cache, false), "a refused hidden folder is left out with the box off");
        let env = rows.iter().find(|row| row.name.bytes == b".env").unwrap();
        assert!(!the_list_shows_the_row(env, false), "a hidden row with no refusal is shown with the box off");
    }

    fn a_server_row(name: &str) -> ServerRow {
        ServerRow {
            local_server_identifier: name.to_string(),
            host: smoozy_servers::identity::Host::what_the_user_typed("10.0.0.4").expect("a host"),
            port: smoozy_servers::identity::Port::STANDARD_SSH,
            username: smoozy_servers::identity::Username::what_the_user_typed("root").expect("a username"),
            key: smoozy_servers::identity::KeyReference {
                local_key_identifier: "k1".to_string(),
                fingerprint: "SHA256:one".to_string(),
            },
            label: None,
            colour: None,
            remembered_setup: None,
            remembered_host_key: None,
            remembered_hardware_reading: None,
            recorded_machine_root: None,
            last_confirmed_installation_identity: None,
            event_cursor: None,
            machine_password_subject: None,
            chosen_fallback_folder: None,
            the_driver_missed_the_optix_bar: false,
        }
    }

    fn a_started_over(folder: &std::path::Path) -> Started {
        let record = crate::record::LocalRecord::open_at(&folder.join("local-record.sqlite3"))
            .expect("a throwaway record");
        Started {
            the_sweep_left_behind: Vec::new(),
            record: Some(record),
            record_trouble: None,
            diary: None,
            declarations: smoozy_reporting::lines::Declarations::new(),
            what_modules_declare: crate::presentation::declarations::TheDeclarations::default(),
            said_once: crate::presentation::said_once::TheThingsSaidOnce::new(),
            run: None,
            the_previous_run: smoozy_state::run_mark::WhatTheLaunchFound::NoPreviousRun,

        }
    }

    /// **Tests 1 and 2 at the commands' own reading: a fresh install reads
    /// off; a tick written by the command's own route is read back after a
    /// restart, and is the same on every server** (`File Browser` L20,
    /// `State, identity and recovery` I7). **And a choice that will not read
    /// is the record not reading**, never a silent *off* (L8a).
    #[test]
    fn the_choice_is_off_on_a_fresh_install_survives_a_restart_and_is_one_for_every_server() {
        let folder = std::env::temp_dir().join(format!("smoozy-hidden-files-choice-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a scratch folder");
        let a_server = || Identity::issue(IdentityKind::LocalServer).expect("randomness").as_text();
        let (one, two) = (a_server(), a_server());
        {
            let started = a_started_over(&folder);
            let connection = started.record.as_ref().unwrap().connection();
            server_rows::add(connection, &a_server_row(&one)).expect("a row");
            server_rows::add(connection, &a_server_row(&two)).expect("a row");
            let (there_is, shown) = what_there_is_with_the_choice(&started, &one);
            assert!(matches!(there_is, WhatThereIs::TheServer(..)));
            assert!(!shown, "a fresh install shows hidden files");
            assert!(the_choice_is_written(&started, true));
        }
        let started = a_started_over(&folder);
        for server in [&one, &two] {
            let (there_is, shown) = what_there_is_with_the_choice(&started, server);
            assert!(matches!(there_is, WhatThereIs::TheServer(..)));
            assert!(shown, "the tick did not survive the restart, or is not the same on {server}");
        }
        assert!(the_choice_is_written(&started, false));
        assert!(!what_there_is_with_the_choice(&started, &two).1);
        started.record.as_ref().unwrap().connection().execute_batch("DROP TABLE setting").unwrap();
        assert!(
            matches!(what_there_is_with_the_choice(&started, &one).0, WhatThereIs::TheRecordWouldNotRead),
            "a choice that would not read was taken as off"
        );
        assert!(!the_choice_is_written(&started, true), "a write the record refused was answered as saved");
        drop(started);
        let _ = std::fs::remove_dir_all(&folder);
    }

    /// **Test 3 and `File Browser` L21: the tick writes the choice and reads
    /// nothing from the server** — the command takes no connection, no period
    /// and no session, and only it and the two tab commands read or write the
    /// choice anywhere in this crate's working code (`File Browser` L21: *"this document hands
    /// the choice to no other document"*, so the walks, the transfer window,
    /// the uploads and the downloads never see it).
    #[test]
    fn the_tick_reads_nothing_from_the_server_and_nothing_else_takes_the_choice() {
        let working = the_working_half();
        let at = working.find("pub fn show_hidden_files_was_ticked(").expect("the command");
        let command = &working[at..];
        let command = &command[..command.find("\n}\n").expect("its end")];
        for reaching in ["ThePeriodsThisRun", "the_live_point_of", "a_reading(", "a_folder_is_read", "sessions"] {
            assert!(!command.contains(reaching), "the tick reaches the server through {reaching}");
        }
        let mut readers = Vec::new();
        let source = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        fn walk(folder: &std::path::Path, found: &mut Vec<String>) {
            let Ok(entries) = std::fs::read_dir(folder) else { return };
            for entry in entries {
                let path = entry.unwrap().path();
                if path.is_dir() {
                    walk(&path, found);
                } else if path.extension().and_then(|e| e.to_str()) == Some("rs") {
                    let text = std::fs::read_to_string(&path).unwrap();
                    let working = text.split("#[cfg(test)]\nmod tests {").next().unwrap_or_default();
                    if working.contains("hidden_files_are_shown(") || working.contains("hidden_files_were_switched(") {
                        found.push(path.file_name().unwrap().to_str().unwrap().to_string());
                    }
                }
            }
        }
        walk(&source, &mut readers);
        readers.sort();
        assert_eq!(readers, vec!["files.rs".to_string(), "settings.rs".to_string()]);
        // **And nothing on the page but the box itself sends it.**
        let page = the_page();
        assert_eq!(page.matches("show_hidden_files_was_ticked").count(), 1);
    }

    /// **The page half of tests 1, 3 and 6, run under Node over the
    /// application's own answers** (`File Browser` L19 to `File Browser` L22,
    /// `Design/File Browser § The list` and `§ While the connection is down`).
    ///
    /// A fresh install draws no dot-name and an unticked box; a tick draws them
    /// — the staging folder marked as Smoozy's — and swaps the count, **asking
    /// nothing but the write**; a hidden file chosen and then unticked leaves
    /// nothing chosen, and Delete then asks nothing; a stale listing with the
    /// line down ticks the same way; and a write the record refused changes
    /// nothing. **Skipped, and says so, where no Node is installed.**
    #[test]
    fn the_page_hides_shows_counts_and_lets_go_of_hidden_names_without_asking_the_server() {
        let here: &[u8] = b"/home/ubuntu";
        let mut reading = the_tab("row", here, None, a_folder_being_read(here), None, None, false);
        reading.a_reading_follows = true;
        let rows = the_tab_after_the_reading(
            TheOrder::default(),
            "row",
            here,
            None,
            WhatTheReadingFound::TheFolder { entries: a_folder_with_hidden_names(), free: None },
            Some(ObservedValue::Connected),
            false,
        );
        let stale = the_tab(
            "row",
            here,
            None,
            TheListing::Rows {
                rows: the_listed_rows(&rows).to_vec(),
                taken_at: None,
                read_elevated: false,
            },
            Some(ObservedValue::Reconnecting),
            None,
            false,
        );
        assert!(!stale.unavailable.is_empty(), "the stale listing is not drawn as the line being down");
        // **`File Browser` L21 as amended 2026-10-02**: a hidden folder the
        // login may not read, reached directly with the box off.
        let refused_hidden = the_tab_after_the_reading(
            TheOrder::default(),
            "row",
            b"/home/ubuntu/.cache",
            None,
            WhatTheReadingFound::ItsRowCarriesTheRefusal {
                parent: here.to_vec(),
                entries: vec![
                    an_entry(".cache", Kind::Folder, 0),
                    an_entry(".env", Kind::File, 12),
                    an_entry("renders", Kind::Folder, 0),
                ],
                free: None,
            },
            Some(ObservedValue::Connected),
            false,
        );
        let answers =
            serde_json::json!({ "reading": reading, "rows": rows, "stale": stale, "refusedHidden": refused_hidden });

        let folder = std::env::temp_dir().join(format!("smoozy-hidden-files-page-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a scratch folder");
        let web = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        for page in ["files.js", "the-value-drawn.js"] {
            std::fs::copy(web.join(page), folder.join(page)).expect("the page is copied");
        }
        for (name, text) in THE_LISTS_STAND_INS {
            if name != "harness.js" {
                std::fs::write(folder.join(name), text).expect("a stand-in is written");
            }
        }
        std::fs::write(folder.join("harness.js"), THE_HIDDEN_FILES_HARNESS).expect("the harness is written");
        std::fs::write(folder.join("answers.json"), answers.to_string()).expect("the answers are written");
        let ran = std::process::Command::new("node").arg(folder.join("harness.js")).current_dir(&folder).output();
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the Files page was not run");
            let _ = std::fs::remove_dir_all(&folder);
            return;
        };
        let _ = std::fs::remove_dir_all(&folder);
        assert!(ran.status.success(), "the page did not run: {}", String::from_utf8_lossy(&ran.stderr));
        let got: serde_json::Value =
            serde_json::from_str(String::from_utf8_lossy(&ran.stdout).trim()).expect("the harness prints one answer");
        let wanted = serde_json::json!({
            "fresh": {
                "names": ["renders", "scene.blend"],
                "ticked": false,
                "mark": "",
                "last": true,
                "label": "Show hidden files",
                "count": "1 file, 1 folder",
            },
            "ticked": {
                "names": [".smoozy-staging", "renders", ".env", "scene.blend", ".\u{FFFD}\u{FFFD}"],
                "ticked": true,
                "mark": "\u{2713}",
                "last": true,
                "staging": "Smoozy staging folder",
                "count": "3 files, 2 folders",
                "asked": [{ "command": "show_hidden_files_was_ticked", "shown": true }],
                // **The Creator's twenty-first notes, note 3: with the box
                // ticked the hidden rows are grey**, the staging folder among
                // them, and the others are not.
                "grey": [".smoozy-staging", ".env", ".\u{FFFD}\u{FFFD}"],
            },
            "hover": { "cut": "Smoozy staging folder", "fits": "" },
            "chosen": { "note": "1 selected" },
            "theChosenRowsPath": {
                "asked": [
                    { "command": "the_delete_of", "victims": [b"/home/ubuntu/.env".to_vec()] },
                    { "command": "what_disturbing_costs", "paths": [b"/home/ubuntu/.env".to_vec()] },
                ],
            },
            "unticked": {
                "names": ["renders", "scene.blend"],
                "ticked": false,
                "mark": "",
                "last": true,
                "note": null,
                "count": "1 file, 1 folder",
                "asked": [{ "command": "show_hidden_files_was_ticked", "shown": false }],
            },
            "deleteAfterTheUntick": { "asked": [] },
            "lineDown": {
                "names": [".smoozy-staging", "renders", ".env", "scene.blend", ".\u{FFFD}\u{FFFD}"],
                "ticked": true,
                "offline": true,
                "asked": [{ "command": "show_hidden_files_was_ticked", "shown": true }],
            },
            "refused": {
                "names": [".smoozy-staging", "renders", ".env", "scene.blend", ".\u{FFFD}\u{FFFD}"],
                "ticked": true,
                "asked": [{ "command": "show_hidden_files_was_ticked", "shown": false }],
            },
            "refusedHidden": {
                "rows": [[".cache", true], ["renders", false]],
                "ticked": false,
                "count": "0 files, 2 folders",
                // **A row that carries a refusal keeps the refusal's own
                // look**, hidden or not (the twenty-first notes, note 3).
                "grey": [],
            },
        });
        assert_eq!(got, wanted);
    }

    /// **The harness for the box.** The document is the one
    /// [`THE_LISTS_STAND_INS`]' harness uses, with two things more: an
    /// element's listeners can be pressed, and the document's own key
    /// listeners are held so the Delete key can be pressed.
    const THE_HIDDEN_FILES_HARNESS: &str = r#"import { readFileSync } from "node:fs";
function aTest(selector) {
  const found = /^([a-z0-9]*)((?:\.[\w-]+)*)$/.exec(selector.trim());
  if (!found) return () => false;
  const [, tag, classes] = found;
  const names = classes ? classes.split(".").filter(Boolean) : [];
  return (element) => {
    if (!element || !element.classList) return false;
    if (tag && element.tag !== tag) return false;
    return names.every((name) => element.classList.contains(name));
  };
}
function everyUnder(element, test, into) {
  for (const child of element.children || []) {
    if (test(child)) into.push(child);
    everyUnder(child, test, into);
  }
  return into;
}
function anElement(tag) {
  const names = new Set();
  const listeners = new Map();
  const attributes = new Map();
  const element = {
    tag, children: [], parentNode: null, dataset: {}, style: { setProperty() {} }, hidden: false, text: "", type: "", id: "", title: "", scrollTop: 0,
    classList: {
      add: (...more) => more.forEach((one) => names.add(one)),
      remove: (...less) => less.forEach((one) => names.delete(one)),
      toggle: (one, on) => { if (on === undefined ? !names.has(one) : on) names.add(one); else names.delete(one); },
      contains: (one) => names.has(one),
    },
    set className(value) { names.clear(); String(value).split(" ").filter(Boolean).forEach((one) => names.add(one)); },
    get className() { return Array.from(names).join(" "); },
    set textContent(value) { for (const one of element.children) one.parentNode = null; element.children = []; element.text = value == null ? "" : String(value); },
    get textContent() { return element.text + element.children.map((one) => one.textContent || one.text || "").join(""); },
    get offsetTop() { return 0; },
    get offsetHeight() { return 30; },
    get clientHeight() { return 300; },
    appendChild(child) {
      if (child.parentNode && child.parentNode.removeChild) child.parentNode.removeChild(child);
      child.parentNode = element;
      element.children.push(child);
      return child;
    },
    insertBefore(child, before) {
      if (child.parentNode && child.parentNode.removeChild) child.parentNode.removeChild(child);
      child.parentNode = element;
      const at = element.children.indexOf(before);
      if (at < 0) element.children.push(child); else element.children.splice(at, 0, child);
      return child;
    },
    removeChild(child) { element.children = element.children.filter((one) => one !== child); child.parentNode = null; return child; },
    replaceChildren(...fresh) { for (const one of element.children) one.parentNode = null; element.children = []; fresh.forEach((one) => element.appendChild(one)); },
    replaceWith(fresh) {
      const holder = element.parentNode;
      if (!holder) return;
      holder.children = holder.children.map((one) => (one === element ? fresh : one));
      fresh.parentNode = holder;
      element.parentNode = null;
    },
    remove() { if (element.parentNode) element.parentNode.removeChild(element); },
    setAttribute(name, value) { attributes.set(name, String(value)); },
    getAttribute(name) { return attributes.has(name) ? attributes.get(name) : null; },
    addEventListener(kind, handler) { const held = listeners.get(kind) || []; held.push(handler); listeners.set(kind, held); },
    press(kind) { for (const handler of listeners.get(kind) || []) handler({ preventDefault() {}, target: element }); },
    fire(kind, carrying) { for (const handler of listeners.get(kind) || []) handler({ preventDefault() {}, target: element, ...carrying }); },
    focus() {},
    select() {},
    contains(other) { let at = other; while (at) { if (at === element) return true; at = at.parentNode; } return false; },
    closest() { return null; },
    querySelector(selector) { return everyUnder(element, aTest(selector), [])[0] || null; },
    querySelectorAll(selector) { return everyUnder(element, aTest(selector), []); },
    getBoundingClientRect() { return { left: 0, top: 0, bottom: 0, width: 0, height: 0 }; },
  };
  return element;
}
const body = anElement("body");
const keys = [];
globalThis.document = {
  createElement: anElement,
  createElementNS: (_, tag) => anElement(tag),
  createTextNode: (text) => ({ text, children: [] }),
  addEventListener(kind, handler) { if (kind === "keydown") keys.push(handler); },
  removeEventListener() {},
  body,
};
const answers = JSON.parse(readFileSync("answers.json", "utf8"));
let plan = { tab: answers.reading, reading: answers.rows };
let asked = [];
let theWriteIsTaken = true;
globalThis.window = {
  innerWidth: 1000, innerHeight: 800,
  __TAURI__: {
    core: {
      invoke(command, args) {
        // **Every command is written down**, the readings of the server among
        // them, so a tick that read anything would show here.
        asked.push(
          command === "show_hidden_files_was_ticked"
            ? { command, shown: args.shown }
            : command === "the_delete_of"
              ? { command, victims: args.victims }
              : command === "what_disturbing_costs"
                ? { command, paths: args.paths }
                : { command },
        );
        if (command === "the_files_tab") return Promise.resolve(plan.tab);
        if (command === "the_folder_is_read") return Promise.resolve(plan.reading);
        if (command === "the_saved_with_version_is_read") return Promise.resolve(null);
        if (command === "show_hidden_files_was_ticked") return Promise.resolve(theWriteIsTaken);
        return new Promise(() => {});
      },
    },
  },
};

await import("./files.js");
const host = anElement("div");
body.appendChild(host);
globalThis.theTab.itIsDrawn(host);
const settled = async () => { for (let at = 0; at < 4; at += 1) await new Promise((done) => setTimeout(done, 0)); };
const rows = () => host.querySelector("div.flist").children.filter((one) => one.classList.contains("frow") && !one.classList.contains("fhead"));
const names = () => rows().map((one) => one.querySelector(".fname").textContent);
const theBox = () => host.querySelector("label.fhid");
const ticked = () => theBox().querySelector(".cb").classList.contains("on");
// **The box is the bar's last thing, and ticked it shows a tick mark**
// (`Design/File Browser § The list`; the Creator's note 2 on the test copy).
const mark = () => theBox().querySelector(".cb").textContent;
const last = () => { const bar = host.querySelector("div.fbar"); return bar.children[bar.children.length - 1] === theBox(); };
const count = () => {
  const bar = host.querySelector("div.fbar");
  const pieces = bar.children.filter((one) => one.tag === "span" && !one.classList.contains("grow"));
  return pieces.length > 1 ? pieces[1].textContent.replace(/\u00A0/g, " ") : null;
};
const note = () => { const one = host.querySelector(".selnote"); return one ? plainly(one.textContent) : null; };
const plainly = (text) => (text == null ? text : text.replace(/\u00A0/g, " "));
const press = async (element) => { asked = []; element.press("click"); await settled(); };

globalThis.theTab.itIsShowing("row");
await settled();
const out = {};
out.fresh = { names: names(), ticked: ticked(), mark: mark(), last: last(), label: plainly(theBox().textContent.trim()), count: count() };
await press(theBox());
const staging = rows().find((one) => one.querySelector(".fname").textContent === ".smoozy-staging");
const grey = () => rows().filter((one) => one.classList.contains("fhidden")).map((one) => one.querySelector(".fname").textContent);
out.ticked = { names: names(), ticked: ticked(), mark: mark(), last: last(), staging: plainly(staging.querySelector(".kind").textContent), count: count(), asked, grey: grey() };
// **A cell that does not fit shows whole on hover, and one that fits carries
// nothing** (`Design/Whole app`; the Creator's note 9 on the test copy). The
// widths are the browser's, so they are stood in here.
const theKindOf = (name) => rows().find((one) => one.querySelector(".fname").textContent === name).querySelector(".kind");
const cut = theKindOf(".smoozy-staging");
cut.scrollWidth = 160; cut.clientWidth = 110; cut.fire("mouseenter", {});
const fits = theKindOf("renders");
fits.scrollWidth = 40; fits.clientWidth = 110; fits.fire("mouseenter", {});
out.hover = { cut: plainly(cut.title), fits: fits.title };
await press(rows().find((one) => one.querySelector(".fname").textContent === ".env"));
out.chosen = { note: note() };
// **The two boxes are handed the chosen row's path and never its bare name**
// (step 10c's Files' owed acts piece 1): Delete, then F2.
asked = [];
for (const handler of keys) handler({ key: "Delete", preventDefault() {}, defaultPrevented: false, target: body });
await settled();
for (const handler of keys) handler({ key: "F2", preventDefault() {}, defaultPrevented: false, target: body });
await settled();
// **F2 opens the row for typing** (step 10c's Files' owed acts piece 3), and
// Return asks the live-job box about the row's path.
const typed = host.querySelector("input");
typed.value = "env.bak";
typed.fire("input", {});
typed.fire("keydown", { key: "Enter" });
await settled();
out.theChosenRowsPath = { asked: asked.slice() };
typed.fire("keydown", { key: "Escape" });
await settled();
await press(theBox());
out.unticked = { names: names(), ticked: ticked(), mark: mark(), last: last(), note: note(), count: count(), asked };
asked = [];
for (const handler of keys) handler({ key: "Delete", preventDefault() {}, defaultPrevented: false, target: body });
await settled();
out.deleteAfterTheUntick = { asked: asked.slice() };
plan = { tab: answers.stale, reading: answers.stale };
globalThis.theTab.itIsShowing("row");
await settled();
await press(theBox());
out.lineDown = { names: names(), ticked: ticked(), offline: host.classList.contains("offline"), asked };
theWriteIsTaken = false;
await press(theBox());
out.refused = { names: names(), ticked: ticked(), asked };
asked = [];
plan = { tab: answers.refusedHidden, reading: answers.refusedHidden };
globalThis.theTab.itIsShowing("row");
await settled();
out.refusedHidden = {
  rows: rows().map((one) => [one.querySelector(".fname").children[0].textContent, Boolean(one.querySelector("span.deny"))]),
  ticked: ticked(),
  count: count(),
  grey: rows().filter((one) => one.classList.contains("fhidden")).map((one) => one.querySelector(".fname").children[0].textContent),
};
console.log(JSON.stringify(out));
"#;

    // ── Step 10c's Files' owed acts piece 1: the warnings read the record ───

    /// **A server row on a placed machine** — a confirmed installation and a
    /// recorded root, so the record can be keyed and a job folder composed.
    fn a_placed_row(server: &Identity, installation: &Identity) -> ServerRow {
        let mut row = a_server_row(&server.as_text());
        row.recorded_machine_root = Some(smoozy_servers::store::RecordedMachineRoot {
            path: "/ephemeral/smoozy".into(),
            the_package_areas_own_location: None,
            read_under: installation.as_text(),
        });
        row.last_confirmed_installation_identity = Some(installation.as_text());
        row
    }

    /// **A provisional job record, as a submission writes it**, naming its
    /// `.blend` on the machine.
    fn a_job_record(
        job: &Identity,
        server: &Identity,
        blend: &str,
        label: &str,
    ) -> smoozy_queue_history::record::LocalJobRecord {
        use smoozy_queue_history::record::*;
        LocalJobRecord {
            job: job.clone(),
            group: None,
            server: TheServerAsItStood {
                local_server: server.clone(),
                label: None,
                hue: None,
            },
            submitted_at: None,
            label: Some(label.to_string()),
            blend_path_on_the_machine: Some(blend.to_string()),
            scene_name: None,
            submitted_with_no_scene_named: false,
            resolution_percentage_typed: None,
            picture_size_from_the_read: None,
            main_output_from_the_read: None,
            blender: TheBlenderTheEntryKeeps {
                pick: smoozy_render_settings::payload::VersionPick::of(
                    &smoozy_render_settings::file_half::BlenderIdentity {
                        path: "/ephemeral/smoozy/blender/b".to_string(),
                        version: smoozy_server_setup::blender::Version { major: 4, minor: 5, patch: 0 },
                    },
                ),
                origin: AnswerOrigin::ResolvedForThem,
            },
            device_asked_for: TheDeviceAsk::TheFilesOwnUnread,
            the_files_values_beside_the_overrides: None,
            dependency_check: None,
            carries_on_from: None,
            disturbances: vec![],
            attempts_where_the_file_changed: vec![],
            the_frames: smoozy_queue_history::record::TheFramesTheReadFound::default(),
            standing: Standing::Provisional,
            the_render_start_has_been_learned: false,
            what_the_note_carried: WhatTheNoteCarried::default(),
            the_job_output_folder: Default::default(),
            the_driver_was_below_the_bar: None,
        }
    }

    /// One collision, as the already-exists walk would answer it.
    fn one_collision(landing: &[u8]) -> TheWalksAnswer {
        TheWalksAnswer {
            comparisons: vec![AComparison {
                landing_path: landing.to_vec(),
                what: Comparison::CollidesWithAFile,
            }],
            colliding: 1,
            arriving_files: 1,
            as_of: std::time::SystemTime::UNIX_EPOCH,
            readings_taken: 1,
        }
    }

    /// **The fault this piece fixes, proved over a throwaway record and then
    /// closed: an upload's already-exists question over a file a live job
    /// reads names the job.** Until step 10c's Files' owed acts piece 1 the
    /// question was built with no live jobs at all, so a *Replace* over a
    /// rendering job's `.blend` warned nobody (`File Browser` W5, W2).
    ///
    /// **And the machine's reading tells rendering from waiting** (`File
    /// Browser` W5a): with no reading held the job reads waiting, and with
    /// one saying *rendering* it reads rendering. **A job never checked says
    /// so** (`File Browser` W6's narrow set).
    #[test]
    fn an_upload_over_a_live_jobs_blend_asks_a_question_that_names_the_job() {
        let area = a_scratch_area("upload-over-a-live-job");
        std::fs::create_dir_all(&area).expect("a scratch folder");
        let started = a_started_over(&area);
        let connection = started.record.as_ref().unwrap().connection();
        let server = Identity::issue(IdentityKind::LocalServer).expect("randomness");
        let installation = Identity::issue(IdentityKind::Installation).expect("randomness");
        let row = a_placed_row(&server, &installation);
        server_rows::add(connection, &row).expect("a row");
        let blend = "/home/ubuntu/kitchen/kitchen_v14.blend";
        let job = Identity::issue(IdentityKind::Job).expect("randomness");
        crate::record::job_records::JobRecords::over(connection)
            .write_the_provisional_record(&a_job_record(&job, &server, blend, "Kitchen"))
            .expect("the provisional record");

        let walk = one_collision(blend.as_bytes());
        let holds = what_the_record_holds_for_the_row(connection, &row, None);
        assert!(holds.is_some(), "a placed row with a readable record holds something");
        let (placed, named, labels) = the_upload_warnings(&walk, holds.as_ref());
        assert_eq!(placed, 0, "nothing the check placed is there");
        assert_eq!(named.len(), 1, "the live job was not named");
        assert_eq!(named[0].job, job);
        assert_eq!(named[0].stands, HowTheJobStands::Waiting);
        let question = the_collision_question(
            TheCollidingAct::AnUpload,
            &walk,
            b"/home/ubuntu/kitchen",
            placed,
            &named,
            &labels,
            None,
        );
        let warning = question.live_jobs.expect("the question carries the live-job warning");
        assert_eq!(warning.said.values(), vec!["Kitchen"]);
        assert!(warning.said.said().contains(Prose::new(words::IS_WAITING_ON_IT).as_str()), "{:?}", warning.said);
        assert!(
            warning.only_the_blend_and_its_folder.is_some(),
            "a job never checked says Smoozy's set is only the .blend and its folder"
        );

        // **The machine says it is rendering**, and the warning says so.
        let reading = smoozy_queue_history::acting::adoption::TheMachinesWorkingState {
            jobs: vec![smoozy_queue_history::readings::AJobAsTheMachineReadsIt {
                job: job.clone(),
                state: smoozy_queue_history::line::QueueState::Rendering {
                    operation: Identity::issue(IdentityKind::Operation).expect("randomness"),
                },
                live: smoozy_queue_history::readings::WhatTheEntryHasLive::NoReadings,
                the_hold: None,
            }],
            the_admission_point_stands_at: None,
            notes: vec![],
            the_history_copys_count: 0,
            the_history_copys_entries: vec![],
        };
        let holds = what_the_record_holds_for_the_row(connection, &row, Some(&reading));
        let (_, named, labels) = the_upload_warnings(&walk, holds.as_ref());
        assert_eq!(named[0].stands, HowTheJobStands::Rendering);
        let warning = the_live_job_warning(&named, &labels).expect("a warning");
        assert!(warning.said.said().contains(Prose::new(words::IS_RENDERING_WITH_THIS_FILE).as_str()));

        // **A file nobody's job reads asks the plain question.**
        let elsewhere = one_collision(b"/home/ubuntu/kitchen/notes.txt");
        let (_, named, _) = the_upload_warnings(&elsewhere, holds.as_ref());
        assert!(named.is_empty());
        drop(started);
        let _ = std::fs::remove_dir_all(&area);
    }

    /// **The delete box over a folder carries the dependency line for a placed
    /// mirror under it, and names a live job whose output folder is inside it**
    /// — `File Browser` W1's folder reading and W5's second direction, on the
    /// real record.
    #[test]
    fn a_delete_of_a_folder_carries_what_the_record_shows_under_it() {
        let area = a_scratch_area("delete-over-the-record");
        std::fs::create_dir_all(&area).expect("a scratch folder");
        let started = a_started_over(&area);
        let connection = started.record.as_ref().unwrap().connection();
        let server = Identity::issue(IdentityKind::LocalServer).expect("randomness");
        let installation = Identity::issue(IdentityKind::Installation).expect("randomness");
        let row = a_placed_row(&server, &installation);
        server_rows::add(connection, &row).expect("a row");
        let machine = smoozy_dependencies::record::MachineKey::of(server.clone(), installation.clone());

        // **A placed mirror under the folder**, as the dependency check's send
        // writes it.
        let mirror = b"/ephemeral/smoozy/assets/files/Users/a/tex/wood.png";
        crate::record::dependency_records::TheDependencyRecord::over(connection)
            .a_placement_is_written(&smoozy_dependencies::record::PlacementRecord::at_publication(
                machine.clone(),
                smoozy_dependencies::record::Published {
                    at: smoozy_dependencies::bytes::MachineLocation::composed(mirror.to_vec()),
                    source: smoozy_dependencies::bytes::StoredLocation::as_the_system_returned_it(
                        b"/Users/a/tex/wood.png",
                    ),
                    content: smoozy_dependencies::contents::ContentIdentity {
                        size: 3,
                        digest: smoozy_server_protocol::transfers::integrity::Digest::of(b"red"),
                    },
                    four_values: smoozy_server_protocol::transfers::SourceReading {
                        size: 3,
                        inode: "7".to_string(),
                        modification_time: 1,
                        change_time: 1,
                    },
                },
            ))
            .expect("the placement");
        let holds = what_the_record_holds_for_the_row(connection, &row, None);
        let (the_box, named) = the_delete_box_over_the_record(
            &[b"/ephemeral/smoozy/assets/files/Users/a/tex".to_vec()],
            holds.as_ref(),
            &[],
            false,
        );
        assert!(named.is_empty());
        let line = the_box.other_references.expect("the dependency line");
        assert_eq!(line.named.values(), vec!["1"]);
        assert!(the_box.live_jobs.is_none());

        // **A live job's output folder sits inside the victim**, so the job is
        // named — the job folder is a member, and the second direction of the
        // containment test reaches it.
        let job = Identity::issue(IdentityKind::Job).expect("randomness");
        crate::record::job_records::JobRecords::over(connection)
            .write_the_provisional_record(&a_job_record(
                &job,
                &server,
                "/ephemeral/smoozy/assets/files/Users/a/scene.blend",
                "Scene",
            ))
            .expect("the provisional record");
        let holds = what_the_record_holds_for_the_row(connection, &row, None);
        let output = smoozy_job_files::publication::AJobsPaths::composed(
            &crate::servers::installation::the_layout_recorded_for(&row).expect("a layout"),
            &job,
            Some("Scene"),
        )
        .expect("it fits")
        .job_output_folder
        .into_bytes();
        let above = the_folder_above(&output).expect("the folder above the job's own");
        let (the_box, named) = the_delete_box_over_the_record(&[above], holds.as_ref(), &[], false);
        assert_eq!(named.len(), 1, "the job whose output folder is inside the victim was not named");
        assert_eq!(the_box.live_jobs.expect("the live-job line").said.values(), vec!["Scene"]);

        // **And a frame inside the job's output folder names it too.**
        let mut a_frame = output.clone();
        a_frame.extend_from_slice(b"/frame_0001.png");
        let (the_box, _) = the_delete_box_over_the_record(&[a_frame.clone()], holds.as_ref(), &[], false);
        assert!(the_box.live_jobs.is_some(), "a frame inside a live job's folder warned nobody");
        let (live, _) = the_live_job_box_over_the_record("rename", &[a_frame], None, holds.as_ref());
        assert!(live.is_some(), "the rename box warned nobody");

        // **A move or a copy into a folder above a job's files lands beside
        // them and touches none**, so nobody is named — `File Browser` W5's
        // destination is where the item lands. The pass before 0.1.9 on Vast,
        // new fault 3: `/usr/share/doc/adduser` pasted into `/root` named the
        // job whose `.blend` was `/root/v19-tex/c18-tex.blend`, and marked it.
        let elsewhere = b"/usr/share/doc/adduser".to_vec();
        let (live, named) =
            the_live_job_box_over_the_record("move", &[elsewhere.clone()], Some(b"/ephemeral"), holds.as_ref());
        assert!(live.is_none() && named.is_empty(), "a move beside a job's files named it: {named:?}");
        let (live, named) = the_live_job_box_over_the_record("copy", &[elsewhere.clone()], Some(b"/ephemeral"), holds.as_ref());
        assert!(live.is_none() && named.is_empty(), "a copy beside a job's files named it: {named:?}");
        // **Landing inside the job's own folder still names it**, and the
        // title counts one item as one.
        let (live, named) = the_live_job_box_over_the_record("move", &[elsewhere], Some(&output), holds.as_ref());
        assert_eq!(named.len(), 1, "a move into a live job's folder named nobody");
        let title = live.expect("the move box").question;
        assert_eq!(title.values().first().copied(), Some("1"));
        assert!(title.said().contains(Prose::new(words::MOVE_QUESTION_MIDDLE_ONE).as_str()), "{}", title.said());
        assert!(!title.said().contains("items"), "{}", title.said());

        // **No record held, no line** — and the permanence warning stands.
        let (the_box, named) = the_delete_box_over_the_record(&[mirror.to_vec()], None, &[], false);
        assert!(named.is_empty() && the_box.other_references.is_none() && the_box.live_jobs.is_none());
        drop(started);
        let _ = std::fs::remove_dir_all(&area);
    }

    /// **`File Browser` M3's staging line over the transfer records**: a part-
    /// sent upload staged under the victim is said, and one under another
    /// installation is not.
    #[test]
    fn the_delete_box_names_staged_material_under_the_victim() {
        use smoozy_server_protocol::transfers::{Direction, TransferRecord};
        let server = Identity::issue(IdentityKind::LocalServer).expect("randomness");
        let installation = Identity::issue(IdentityKind::Installation).expect("randomness");
        let machine = smoozy_dependencies::record::MachineKey::of(server, installation.clone());
        let holds = crate::record::neighbouring_records::WhatTheLocalRecordHolds {
            record: Default::default(),
            machine,
            layout: smoozy_job_files::layout::TheLayout::of(
                smoozy_job_files::layout::MachineRoot::chosen("/ephemeral/smoozy").expect("a root"),
                smoozy_job_files::layout::WhereThePackageAreaSits::UnderTheRoot,
            ),
            queue: vec![],
            entries: vec![],
        };
        let record = TransferRecord::arranged(
            Identity::issue(IdentityKind::Transfer).expect("randomness"),
            Direction::Up,
            Identity::issue(IdentityKind::LocalServer).expect("randomness"),
            Some(installation.clone()),
            "/Users/a/big.blend".to_string(),
            "/home/ubuntu/kitchen/big.blend".to_string(),
            "/home/ubuntu/kitchen/.smoozy-staging",
            100,
            Identity::issue(IdentityKind::Run).expect("randomness"),
        );
        let mut elsewhere = record.clone();
        elsewhere.installation = Some(Identity::issue(IdentityKind::Installation).expect("randomness"));
        let (the_box, _) =
            the_delete_box_over_the_record(&[b"/home/ubuntu/kitchen".to_vec()], Some(&holds), &[record, elsewhere], false);
        assert_eq!(the_box.staged_material.expect("the staging line").named.values(), vec!["1"]);
    }

    // ── Step 10c's Files' owed acts piece 3: New folder and Rename ──────────

    /// **What the page is told after a new folder or a rename**, every
    /// reading of it — `File Browser` M4, `File Browser` M9, `File Browser`
    /// P1, and `Design/File Browser §
    /// What the screen says around an act`: a refusal before trying and a
    /// taken name keep the row in editing, red, with one line under it; the
    /// login may not carries the sentence **and no admin offer**
    /// (`Preparation/Build/Plan.md`, step 10c); a rename to a dot-name with
    /// *Show hidden files* off says where the row went.
    #[test]
    fn the_row_says_what_the_act_came_to() {
        use crate::managing_files::WhatTheActCameTo as Came;
        use smoozy_file_browser::managing::WhyTheNameIsRefused as Why;
        // **The typography point binds short words**, so the sentences are
        // compared with its no-break spaces read as spaces.
        let plainly = |words: Words| words.said().replace('\u{a0}', " ");
        let on_the_row = |answer: AnActsAnswer| match answer {
            AnActsAnswer::OnTheRow { editing, refusal } => (editing, refusal),
            other => panic!("the row was let go of: {other:?}"),
        };
        for (why, said) in [
            (Why::Empty, words::TYPE_A_NAME),
            (Why::HoldsASlash, words::A_NAME_HOLDS_NO_SLASH),
            (Why::IsADotName, words::A_NAME_IS_NOT_A_DOT),
            (Why::TooLong, words::THE_NAME_IS_TOO_LONG),
            (Why::IsTheStagingFoldersName, words::THE_NAME_IS_SMOOZYS_OWN),
        ] {
            let (editing, refusal) =
                on_the_row(what_the_row_says(&Came::TheNameIsRefused(why), b"a/b", TheRefusedAct::Renaming, false));
            assert_eq!(editing.typed.as_str(), "a/b", "the typed name stays in the row");
            assert_eq!(plainly(editing.refused.expect("red")), said);
            assert!(refusal.is_none());
        }
        let (editing, _) = on_the_row(what_the_row_says(&Came::TheNameIsTaken, b"b.png", TheRefusedAct::Renaming, false));
        assert_eq!(plainly(editing.refused.expect("red")), words::THE_NAME_IS_TAKEN);
        let (editing, refusal) =
            on_the_row(what_the_row_says(&Came::TheLoginMayNot, b"caches", TheRefusedAct::MakingAFolder, false));
        assert!(editing.refused.is_none());
        let refusal = refusal.expect("the login may not");
        assert!(refusal.the_offer.is_none(), "an admin offer was made for a retry that is not built");
        assert_eq!(
            refusal.said,
            a_refusal_with_no_retry_built(TheRefusedAct::MakingAFolder).said
        );

        // **Done**: nothing to say, unless the row left the list.
        assert_eq!(
            what_the_row_says(&Came::Done, b"wood.png", TheRefusedAct::Renaming, false),
            AnActsAnswer::Done { said: None }
        );
        let AnActsAnswer::Done { said: Some(said) } =
            what_the_row_says(&Came::Done, b".wood.png", TheRefusedAct::Renaming, false)
        else {
            panic!("a rename to a hidden name said nothing with Show hidden files off");
        };
        assert_eq!(said.values(), vec![".wood.png"]);
        assert!(said.said().contains("hidden"), "{}", said.said());
        assert_eq!(
            what_the_row_says(&Came::Done, b".wood.png", TheRefusedAct::Renaming, true),
            AnActsAnswer::Done { said: None },
            "the row is still in the list with Show hidden files on"
        );
        assert_eq!(
            what_the_row_says(&Came::Done, b".cache", TheRefusedAct::MakingAFolder, false),
            AnActsAnswer::Done { said: None },
            "only a rename takes a row out of the list"
        );

        // **Said on the line above the list.**
        for (came, said) in [
            (Came::ItIsNotThere, words::IT_IS_NOT_THERE_ANY_MORE),
            (Came::TheMachineRefused, words::THE_SERVER_REFUSED_IT),
            (Came::TheTellingWouldNotWrite, words::NOTHING_RENAMED_THE_TELLING),
            (Came::TheConnectionWent, words::THE_CONNECTION_WENT),
        ] {
            let AnActsAnswer::Said { said: words } = what_the_row_says(&came, b"x", TheRefusedAct::Renaming, false) else {
                panic!("{came:?} was not said on the line");
            };
            assert_eq!(plainly(words), said);
        }
    }

    /// **The row being typed, on the page, run under Node over the
    /// application's own answers** — `Design/File Browser § What the screen
    /// says around an act` and `§ Renaming into a name that is taken`. *New
    /// folder* opens a row at the top named *New folder*; a name the list
    /// already shows gets the listing's hint and is still sent; the machine's
    /// *taken* keeps the row in editing, red, with the line under it; Escape
    /// puts the list back; F2 with nothing chosen says so; a name left as it
    /// was sends nothing; a new name asks the live-job box first and then the
    /// rename, **with the row's path**; and the Delete key while a name is
    /// typed edits the name and deletes nothing. **Skipped, and says so, where
    /// no Node is installed.**
    #[test]
    fn the_page_types_a_name_in_the_row_and_sends_it_on_return() {
        use crate::managing_files::WhatTheActCameTo as Came;
        let here: &[u8] = b"/home/ubuntu";
        let mut reading = the_tab("row", here, None, a_folder_being_read(here), None, None, false);
        reading.a_reading_follows = true;
        let rows = the_tab_after_the_reading(
            TheOrder::default(),
            "row",
            here,
            None,
            WhatTheReadingFound::TheFolder { entries: a_folder_with_hidden_names(), free: None },
            Some(ObservedValue::Connected),
            false,
        );
        let answers = serde_json::json!({
            "reading": reading,
            "rows": rows,
            "taken": what_the_row_says(&Came::TheNameIsTaken, b"renders", TheRefusedAct::MakingAFolder, false),
            "done": what_the_row_says(&Came::Done, b"kitchen.blend", TheRefusedAct::Renaming, false),
        });
        let folder = std::env::temp_dir().join(format!("smoozy-typed-row-page-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a scratch folder");
        let web = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        for page in ["files.js", "the-value-drawn.js"] {
            std::fs::copy(web.join(page), folder.join(page)).expect("the page is copied");
        }
        for (name, text) in THE_LISTS_STAND_INS {
            if name != "harness.js" {
                std::fs::write(folder.join(name), text).expect("a stand-in is written");
            }
        }
        let the_document = &THE_HIDDEN_FILES_HARNESS
            [..THE_HIDDEN_FILES_HARNESS.find("const answers = JSON.parse").expect("the shared document")];
        std::fs::write(folder.join("harness.js"), format!("{the_document}{THE_TYPED_ROW_HARNESS}"))
            .expect("the harness is written");
        std::fs::write(folder.join("answers.json"), answers.to_string()).expect("the answers are written");
        let ran = std::process::Command::new("node").arg(folder.join("harness.js")).current_dir(&folder).output();
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the Files page was not run");
            let _ = std::fs::remove_dir_all(&folder);
            return;
        };
        let _ = std::fs::remove_dir_all(&folder);
        assert!(ran.status.success(), "the page did not run: {}", String::from_utf8_lossy(&ran.stderr));
        let got: serde_json::Value =
            serde_json::from_str(String::from_utf8_lossy(&ran.stdout).trim()).expect("the harness prints one answer");
        let bytes = |text: &str| text.as_bytes().to_vec();
        let wanted = serde_json::json!({
            "started": {
                "first": "New folder",
                "editing": 1,
                "field": ["fedit", "fld"],
                "cell": ["fediting", "fname"],
            },
            "hinted": { "hint": "Name in use as of the last refresh." },
            "sent": {
                "asked": [{
                    "command": "a_new_folder_is_made",
                    "folder": bytes("/home/ubuntu"),
                    "name": bytes("renders"),
                    "shown": 5,
                }],
            },
            "taken": { "editing": 1, "bad": true, "field": ["bad", "fedit", "fld"], "line": "Name already in use.", "value": "renders" },
            "escaped": { "editing": 0, "names": ["renders", "scene.blend"] },
            "nothingChosen": { "line": "Select a file or folder to rename.", "asked": [] },
            "unchanged": { "editing": 0, "asked": [] },
            "renamed": {
                "asked": [
                    { "command": "what_disturbing_costs", "paths": [bytes("/home/ubuntu/scene.blend")] },
                    { "command": "a_rename_is_made", "path": bytes("/home/ubuntu/scene.blend"), "name": bytes("kitchen.blend") },
                    { "command": "the_files_tab" },
                ],
                "editing": 0,
            },
            "deleteWhileTyping": { "asked": [] },
        });
        assert_eq!(got, wanted);
    }

    /// **The row being typed into keeps the list's look** (`Design/File
    /// Browser § Acting on files`; the Creator's note 4 on the test copy,
    /// map piece 5): the drawing's `.fname .fedit` takes the list's font and
    /// line, sits where the name sits, and shows its focus by a one-pixel accent
    /// border and never the system's outline; the cell lets the border show;
    /// the input is as wide as the name column, as the drawing's span is; and
    /// the page never gives it a form field's fixed width. The page half — that
    /// the field wears these classes — is
    /// [`the_page_types_a_name_in_the_row_and_sends_it_on_return`].
    #[test]
    fn the_typing_field_takes_the_lists_look() {
        let drawing = include_str!("../../web/the-drawing.css");
        let rule = |opening: &str| {
            let at = drawing.find(opening).unwrap_or_else(|| panic!("the drawing has {opening}"));
            let rule = &drawing[at..];
            rule[..rule.find('}').expect("its end")].to_string()
        };
        let field = rule(".fname .fedit {");
        for declared in [
            "font: inherit;",
            "line-height: inherit;",
            "padding: 0 4px;",
            "margin: -1px -5px;",
            "border: 1px solid var(--accent);",
            "outline: none;",
        ] {
            assert!(field.contains(declared), "the typing field does not declare {declared}");
        }
        assert!(rule(".fname.fediting {").contains("overflow: visible;"), "the cell clips the border");
        let sheet = include_str!("../../web/files.css");
        let input = &sheet[sheet.find(".fname input.fedit {").expect("the input's own rule")..];
        let input = &input[..input.find('}').expect("its end")];
        assert!(input.contains("width: calc(100% + 10px);"), "the input is not the name column's width");
        assert!(!the_page_code().contains("w-list"), "the typing field takes a form field's fixed width");
    }

    /// **The scenario for [`the_page_types_a_name_in_the_row_and_sends_it_on_return`]**,
    /// run after the document [`THE_HIDDEN_FILES_HARNESS`] stands up.
    const THE_TYPED_ROW_HARNESS: &str = r#"const answers = JSON.parse(readFileSync("answers.json", "utf8"));
let asked = [];
globalThis.window = {
  innerWidth: 1000, innerHeight: 800,
  __TAURI__: {
    core: {
      invoke(command, args) {
        if (command === "the_files_tab") { asked.push({ command }); return Promise.resolve(answers.reading); }
        if (command === "the_folder_is_read") return Promise.resolve(answers.rows);
        if (command === "the_saved_with_version_is_read") return Promise.resolve(null);
        if (command === "a_new_folder_is_made") {
          asked.push({ command, folder: args.folder, name: args.name, shown: args.shown.length });
          return Promise.resolve(answers.taken);
        }
        if (command === "what_disturbing_costs") { asked.push({ command, paths: args.paths }); return Promise.resolve(null); }
        if (command === "a_rename_is_made") {
          asked.push({ command, path: args.path, name: args.name });
          return Promise.resolve(answers.done);
        }
        asked.push({ command });
        return new Promise(() => {});
      },
    },
  },
};
await import("./files.js");
const host = anElement("div");
body.appendChild(host);
globalThis.theTab.itIsDrawn(host);
const settled = async () => { for (let at = 0; at < 6; at += 1) await new Promise((done) => setTimeout(done, 0)); };
const rows = () => host.querySelector("div.flist").children.filter((one) => one.classList.contains("frow") && !one.classList.contains("fhead"));
const editingRows = () => rows().filter((one) => one.classList.contains("editing"));
const names = () => rows().filter((one) => !one.classList.contains("editing")).map((one) => one.querySelector(".fname").textContent);
const plainly = (text) => (text == null ? text : text.replace(/ /g, " "));
const key = async (which, target) => { for (const handler of keys) handler({ key: which, preventDefault() {}, defaultPrevented: false, target: target || body }); await settled(); };
const theControl = (label) => host.querySelectorAll("button").find((one) => plainly(one.getAttribute("aria-label")) === label);
globalThis.theTab.itIsShowing("row");
await settled();
const out = {};

asked = [];
theControl("New folder").press("click");
await settled();
let field = host.querySelector("input");
out.started = { first: rows()[0].classList.contains("editing") ? field.value : null, editing: editingRows().length };
// **The field is the drawing's typing field, not a form's** (`Design/File
// Browser § Acting on files`; the Creator's note 4 on the test copy): it wears
// `fedit`, whose rule takes the list's font, the name column's width and a
// one-pixel accent border, and never `w-list`'s fixed width; and the name
// cell lets that border show.
out.started.field = field.className.split(" ").sort();
out.started.cell = field.parentNode.className.split(" ").sort();
field.value = "renders";
field.fire("input", {});
out.hinted = { hint: plainly(host.querySelector("span.fhint").textContent) };
field.fire("keydown", { key: "Enter" });
await settled();
out.sent = { asked: asked.slice() };
field = host.querySelector("input");
out.taken = {
  editing: editingRows().length,
  bad: field.classList.contains("bad"),
  field: field.className.split(" ").sort(),
  line: plainly(editingRows()[0].querySelector("span.fname").children.filter((one) => one !== field && one.tag !== "span" || (one.classList && one.classList.contains("deny"))).map((one) => one.textContent).join("").trim()),
  value: field.value,
};
field.fire("keydown", { key: "Escape" });
await settled();
out.escaped = { editing: editingRows().length, names: names() };

asked = [];
await key("F2");
const line = host.querySelector(".fnotyet");
out.nothingChosen = { line: line ? plainly(line.textContent) : null, asked: asked.slice() };

rows().find((one) => one.querySelector(".fname").textContent === "scene.blend").press("click");
await settled();
asked = [];
await key("F2");
field = host.querySelector("input");
field.fire("keydown", { key: "Enter" });
await settled();
out.unchanged = { editing: editingRows().length, asked: asked.slice() };

await key("F2");
field = host.querySelector("input");
asked = [];
await key("Delete", field);
out.deleteWhileTyping = { asked: asked.slice() };
field.value = "kitchen.blend";
field.fire("input", {});
field.fire("keydown", { key: "Enter" });
await settled();
out.renamed = { asked: asked.slice(), editing: editingRows().length };
console.log(JSON.stringify(out));
"#;

    /// **The delete, on the page, run under Node over the application's own
    /// answers** (step 10c's Files' owed acts piece 4): the box's *Delete*
    /// sends the chosen row's **path** and the line then says how it ended and
    /// the folder is read again; a delete warning the user switched off, with
    /// nothing that cannot be switched off beside it, asks nothing and goes
    /// straight; and the line says how far a long delete has got. **Skipped,
    /// and says so, where no Node is installed.**
    #[test]
    fn recheck_a_deleted_row_goes_before_the_next_folder_reading() {
        use crate::managing_files::WhatTheDeleteCameTo;
        let here: &[u8] = b"/home/ubuntu";
        let mut reading = the_tab("row", here, None, a_folder_being_read(here), None, None, false);
        reading.a_reading_follows = true;
        let rows = the_tab_after_the_reading(
            TheOrder::default(),
            "row",
            here,
            None,
            WhatTheReadingFound::TheFolder { entries: a_folder_with_hidden_names(), free: None },
            Some(ObservedValue::Connected),
            false,
        );
        let victim = b"/home/ubuntu/scene.blend".to_vec();
        let answers = serde_json::json!({
            "reading": reading,
            "rows": rows,
            "box": the_delete_box_from(&[victim.clone()], &[], &[], None, None, false),
            "quiet": the_delete_box_from(&[victim.clone()], &[], &[], None, None, true),
            "deleted": what_the_delete_says(
                &WhatTheDeleteCameTo::Ran {
                    record: Identity::issue(IdentityKind::Operation).expect("randomness"),
                    done: 1,
                    refused: 0,
                    not_attempted: 0,
                    never_learned: 0,
                },
                &[victim],
            ),
            "moves": {
                "what": "an-act-moves",
                "server": "row",
                "act": "delete",
                "lead": words::DELETING,
                "done": "1200",
                "of": words::OF,
                "total": "10000",
            },
        });
        let folder = std::env::temp_dir().join(format!("smoozy-recheck-delete-page-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a scratch folder");
        let web = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        for page in ["files.js", "the-value-drawn.js"] {
            std::fs::copy(web.join(page), folder.join(page)).expect("the page is copied");
        }
        for (name, text) in THE_LISTS_STAND_INS {
            if name != "harness.js" {
                std::fs::write(folder.join(name), text).expect("a stand-in is written");
            }
        }
        // **The event route holds what the page asks to be told**, so the
        // harness can tell it.
        std::fs::write(
            folder.join("event-route.js"),
            "export function whenTold(kind, handler) { (globalThis.told ||= {})[kind] = handler; }",
        )
        .expect("the route's stand-in");
        let the_document = &THE_HIDDEN_FILES_HARNESS
            [..THE_HIDDEN_FILES_HARNESS.find("const answers = JSON.parse").expect("the shared document")];
        std::fs::write(folder.join("harness.js"), format!("{the_document}{}", THE_DELETE_HARNESS
                .replace("let theBox = answers.box;", "let theBox = answers.box; let deleted = false; let reads = 0; let oldReading; let deleteEnds;")
                .replace("if (command === \"the_files_tab\") {", "if (command === \"the_files_tab\") { if (deleted) return new Promise(() => {});")
                .replace("if (command === \"a_delete_is_made\") {", "if (command === \"a_delete_is_made\") { return new Promise((done) => { deleteEnds = () => { deleted = true; done(answers.deleted); }; });")
                .replace("if (command === \"the_folder_is_read\") return Promise.resolve(answers.rows);",
                    "if (command === \"the_folder_is_read\") { if (reads++ === 0) return Promise.resolve(answers.rows); return new Promise((done) => { oldReading = done; }); }")
                .replace("await import(\"./files.js\");", "const page = await import(\"./files.js\");")
                .split("out.pressed =").next().unwrap().to_string()
                + "page.theBrowserIsAskedAgain(); await settled(); deleteEnds(); await settled(); const names = () => rows().map((one) => one.querySelector('.fname').textContent); const after = names(); oldReading(answers.rows); await settled(); console.log(JSON.stringify({ after, late: names() }));"))
            .expect("the harness is written");
        std::fs::write(folder.join("answers.json"), answers.to_string()).expect("the answers are written");
        let ran = std::process::Command::new("node").arg(folder.join("harness.js")).current_dir(&folder).output();
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the Files page was not run");
            let _ = std::fs::remove_dir_all(&folder);
            return;
        };
        let _ = std::fs::remove_dir_all(&folder);
        assert!(ran.status.success(), "the page did not run: {}", String::from_utf8_lossy(&ran.stderr));
        let got: serde_json::Value =
            serde_json::from_str(String::from_utf8_lossy(&ran.stdout).trim()).expect("the harness prints one answer");
        for when in ["after", "late"] {
            assert!(!got[when].as_array().unwrap().iter().any(|name| name == "scene.blend"),
                "the confirmed delete still shows while a reading waits or after an older one returns: {got}");
        }
    }

    #[test]
    fn the_page_deletes_the_chosen_path_from_the_box_and_says_how_far_it_has_got() {
        use crate::managing_files::WhatTheDeleteCameTo;
        let here: &[u8] = b"/home/ubuntu";
        let mut reading = the_tab("row", here, None, a_folder_being_read(here), None, None, false);
        reading.a_reading_follows = true;
        let rows = the_tab_after_the_reading(
            TheOrder::default(),
            "row",
            here,
            None,
            WhatTheReadingFound::TheFolder { entries: a_folder_with_hidden_names(), free: None },
            Some(ObservedValue::Connected),
            false,
        );
        let victim = b"/home/ubuntu/scene.blend".to_vec();
        let answers = serde_json::json!({
            "reading": reading,
            "rows": rows,
            "box": the_delete_box_from(&[victim.clone()], &[], &[], None, None, false),
            "quiet": the_delete_box_from(&[victim.clone()], &[], &[], None, None, true),
            "deleted": what_the_delete_says(
                &WhatTheDeleteCameTo::Ran {
                    record: Identity::issue(IdentityKind::Operation).expect("randomness"),
                    done: 1,
                    refused: 0,
                    not_attempted: 0,
                    never_learned: 0,
                },
                &[victim],
            ),
            "moves": {
                "what": "an-act-moves",
                "server": "row",
                "act": "delete",
                "lead": words::DELETING,
                "done": "1200",
                "of": words::OF,
                "total": "10000",
            },
        });
        let folder = std::env::temp_dir().join(format!("smoozy-delete-page-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a scratch folder");
        let web = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        for page in ["files.js", "the-value-drawn.js"] {
            std::fs::copy(web.join(page), folder.join(page)).expect("the page is copied");
        }
        for (name, text) in THE_LISTS_STAND_INS {
            if name != "harness.js" {
                std::fs::write(folder.join(name), text).expect("a stand-in is written");
            }
        }
        // **The event route holds what the page asks to be told**, so the
        // harness can tell it.
        std::fs::write(
            folder.join("event-route.js"),
            "export function whenTold(kind, handler) { (globalThis.told ||= {})[kind] = handler; }",
        )
        .expect("the route's stand-in");
        let the_document = &THE_HIDDEN_FILES_HARNESS
            [..THE_HIDDEN_FILES_HARNESS.find("const answers = JSON.parse").expect("the shared document")];
        std::fs::write(folder.join("harness.js"), format!("{the_document}{THE_DELETE_HARNESS}"))
            .expect("the harness is written");
        std::fs::write(folder.join("answers.json"), answers.to_string()).expect("the answers are written");
        let ran = std::process::Command::new("node").arg(folder.join("harness.js")).current_dir(&folder).output();
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the Files page was not run");
            let _ = std::fs::remove_dir_all(&folder);
            return;
        };
        let _ = std::fs::remove_dir_all(&folder);
        assert!(ran.status.success(), "the page did not run: {}", String::from_utf8_lossy(&ran.stderr));
        let got: serde_json::Value =
            serde_json::from_str(String::from_utf8_lossy(&ran.stdout).trim()).expect("the harness prints one answer");
        let path = b"/home/ubuntu/scene.blend".to_vec();
        let wanted = serde_json::json!({
            "boxed": {
                "asked": [{ "command": "the_delete_of" }],
                "boxes": 1,
                "permanent": true,
                "marks": ["", "\u{2713}", ""],
            },
            "pressed": {
                "asked": [{ "command": "a_delete_is_made", "victims": [path.clone()] }, { "command": "the_files_tab" }],
                "line": "Deleted scene.blend.",
            },
            "quiet": {
                "asked": [{ "command": "the_delete_of" }, { "command": "a_delete_is_made", "victims": [path] }, { "command": "the_files_tab" }],
                "boxes": 0,
            },
            "moves": { "line": "Deleting 1200 of 10000" },
        });
        assert_eq!(got, wanted);
    }

    /// **The scenario for [`the_page_deletes_the_chosen_path_from_the_box_and_says_how_far_it_has_got`]**.
    const THE_DELETE_HARNESS: &str = r#"const answers = JSON.parse(readFileSync("answers.json", "utf8"));
let asked = [];
let theBox = answers.box;
globalThis.window = {
  innerWidth: 1000, innerHeight: 800,
  __TAURI__: {
    core: {
      invoke(command, args) {
        if (command === "the_files_tab") { asked.push({ command }); return Promise.resolve(answers.reading); }
        if (command === "the_folder_is_read") return Promise.resolve(answers.rows);
        if (command === "the_saved_with_version_is_read") return Promise.resolve(null);
        if (command === "the_delete_of") { asked.push({ command }); return Promise.resolve(theBox); }
        if (command === "a_delete_is_made") { asked.push({ command, victims: args.victims }); return Promise.resolve(answers.deleted); }
        asked.push({ command });
        return new Promise(() => {});
      },
    },
  },
};
await import("./files.js");
const host = anElement("div");
body.appendChild(host);
globalThis.theTab.itIsDrawn(host);
const settled = async () => { for (let at = 0; at < 6; at += 1) await new Promise((done) => setTimeout(done, 0)); };
const rows = () => host.querySelector("div.flist").children.filter((one) => one.classList.contains("frow") && !one.classList.contains("fhead"));
const plainly = (text) => (text == null ? text : text.replace(/ /g, " "));
const key = async (which) => { for (const handler of keys) handler({ key: which, preventDefault() {}, defaultPrevented: false, target: body }); await settled(); };
const theBoxes = () => body.querySelectorAll("div.modal");
const line = () => { const one = host.querySelector(".fnotyet"); return one ? plainly(one.textContent) : null; };
globalThis.theTab.itIsShowing("row");
await settled();
const out = {};
rows().find((one) => one.querySelector(".fname").textContent === "scene.blend").press("click");
await settled();
asked = [];
await key("Delete");
out.boxed = { asked: asked.slice(), boxes: theBoxes().length, permanent: Boolean(theBoxes()[0] && theBoxes()[0].querySelector("p.msub")) };
// **Ticked, the box shows a tick mark, and unticked again it shows none**
// (the Creator's note 2 on the test copy, the same omission as Files' own box).
const theTick = theBoxes()[0].querySelector("label.mcheck");
const marks = [theTick.querySelector(".cb").textContent];
theTick.press("click");
marks.push(theTick.querySelector(".cb").textContent);
theTick.press("click");
marks.push(theTick.querySelector(".cb").textContent);
out.boxed.marks = marks;
asked = [];
theBoxes()[0].querySelector("button.btn.danger").press("click");
await settled();
out.pressed = { asked: asked.slice(), line: line() };
// Re-check fix: a completed delete clears its selection. This stand-in's
// next independent case lists a file at the same path again; choose it again.
rows().find((one) => one.querySelector(".fname").textContent === "scene.blend").press("click");
await settled();
theBox = answers.quiet;
asked = [];
await key("Delete");
out.quiet = { asked: asked.slice(), boxes: theBoxes().length };
globalThis.told["files:the-answer"]({ said: answers.moves });
out.moves = { line: line() };
console.log(JSON.stringify(out));
"#;

    /// **Files' keys, run under Node over the application's own answers** (the
    /// Creator's second notes on the test copy, piece 8; `Design/File Browser
    /// § Keyboard shortcuts`, the Creator's note 4): ↓ with nothing chosen
    /// chooses the first row and twice more the third, and ↑ steps back,
    /// **choosing and never opening**; Return on a chosen folder asks for its
    /// listing, on a plain file asks nothing, and on a chosen `.blend` opens
    /// Render Settings on its path, as a double-click does (the eighth notes,
    /// note 2); elsewhere Backspace goes up; on a
    /// Mac the *delete* key — which reports itself as Backspace — opens the
    /// delete box, and ⌘↑ goes up. **Skipped, and says so, where no Node is
    /// installed.**
    #[test]
    fn the_keys_move_the_chosen_row_open_a_folder_and_are_the_macs_own_on_a_mac() {
        let here: &[u8] = b"/home/ubuntu";
        let mut reading = the_tab("row", here, None, a_folder_being_read(here), None, None, false);
        reading.a_reading_follows = true;
        let rows = the_tab_after_the_reading(
            TheOrder::default(),
            "row",
            here,
            None,
            WhatTheReadingFound::TheFolder {
                entries: vec![
                    an_entry("renders", Kind::Folder, 0),
                    an_entry("textures", Kind::Folder, 0),
                    an_entry("notes.txt", Kind::File, 12),
                    an_entry("scene.blend", Kind::File, 1_000),
                ],
                free: None,
            },
            Some(ObservedValue::Connected),
            false,
        );
        let answers = serde_json::json!({ "reading": reading, "rows": rows });
        let folder = std::env::temp_dir().join(format!("smoozy-files-keys-page-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a scratch folder");
        let web = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        for page in ["files.js", "the-value-drawn.js"] {
            std::fs::copy(web.join(page), folder.join(page)).expect("the page is copied");
        }
        for (name, text) in THE_LISTS_STAND_INS {
            if name != "harness.js" {
                std::fs::write(folder.join(name), text).expect("a stand-in is written");
            }
        }
        // **Render Settings, written down when it is opened**, so Return on a
        // chosen `.blend` shows whether it opened the settings.
        std::fs::write(
            folder.join("render-settings.js"),
            "export async function theSettingsAreOpenedOn(server, path) { globalThis.opened.push(String.fromCharCode(...path)); }",
        )
        .expect("the render settings stand-in is written");
        let the_document = &THE_HIDDEN_FILES_HARNESS
            [..THE_HIDDEN_FILES_HARNESS.find("const answers = JSON.parse").expect("the shared document")];
        std::fs::write(folder.join("harness.js"), format!("{the_document}{THE_KEYS_HARNESS}"))
            .expect("the harness is written");
        std::fs::write(folder.join("answers.json"), answers.to_string()).expect("the answers are written");
        let ran = std::process::Command::new("node").arg(folder.join("harness.js")).current_dir(&folder).output();
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the Files page was not run");
            let _ = std::fs::remove_dir_all(&folder);
            return;
        };
        let _ = std::fs::remove_dir_all(&folder);
        assert!(ran.status.success(), "the page did not run: {}", String::from_utf8_lossy(&ran.stderr));
        let got: serde_json::Value =
            serde_json::from_str(String::from_utf8_lossy(&ran.stdout).trim()).expect("the harness prints one answer");
        let wanted = serde_json::json!({
            "order": ["renders", "textures", "notes.txt", "scene.blend"],
            "first": { "chosen": ["renders"], "asked": [] },
            "third": { "chosen": ["notes.txt"], "asked": [] },
            "back": { "chosen": ["textures"], "asked": [] },
            "returnOnAFolder": [{ "command": "the_files_tab", "folder": "/home/ubuntu/textures" }],
            "returnOnAFile": { "asked": [], "chosen": ["notes.txt"] },
            "returnOnABlend": { "asked": [], "chosen": ["scene.blend"], "opened": ["/home/ubuntu/scene.blend"] },
            "backspaceElsewhere": [{ "command": "the_files_tab", "folder": "/home" }],
            "commandUpElsewhere": { "asked": [], "chosen": ["notes.txt"] },
            "backspaceOnAMac": [{ "command": "the_delete_of" }],
            "commandUpOnAMac": [{ "command": "the_files_tab", "folder": "/home" }],
        });
        assert_eq!(got, wanted);
    }

    /// **The scenario for [`the_keys_move_the_chosen_row_open_a_folder_and_are_the_macs_own_on_a_mac`]**.
    const THE_KEYS_HARNESS: &str = r#"const answers = JSON.parse(readFileSync("answers.json", "utf8"));
let asked = [];
globalThis.opened = [];
globalThis.window = {
  innerWidth: 1000, innerHeight: 800,
  __TAURI__: {
    core: {
      invoke(command, args) {
        if (command === "the_files_tab") {
          asked.push({ command, folder: args.folder ? String.fromCharCode(...args.folder) : null });
          return Promise.resolve(answers.reading);
        }
        if (command === "the_folder_is_read") return Promise.resolve(answers.rows);
        if (command === "the_saved_with_version_is_read") return Promise.resolve(null);
        asked.push({ command });
        return new Promise(() => {});
      },
    },
  },
};
await import("./files.js");
const host = anElement("div");
body.appendChild(host);
globalThis.theTab.itIsDrawn(host);
const settled = async () => { for (let at = 0; at < 6; at += 1) await new Promise((done) => setTimeout(done, 0)); };
const rows = () => host.querySelector("div.flist").children.filter((one) => one.classList.contains("frow") && !one.classList.contains("fhead"));
const theName = (one) => one.querySelector(".fname").textContent;
const chosen = () => rows().filter((one) => one.getAttribute("aria-current") === "true").map(theName);
const key = async (which, held = {}) => {
  asked = [];
  for (const handler of keys) {
    handler({ key: which, shiftKey: false, metaKey: false, ctrlKey: false, altKey: false, ...held, preventDefault() {}, defaultPrevented: false, target: body });
  }
  await settled();
  return asked.slice();
};
globalThis.theTab.itIsShowing("row");
await settled();
const out = { order: rows().map(theName) };
out.first = { asked: await key("ArrowDown"), chosen: chosen() };
await key("ArrowDown");
out.third = { asked: await key("ArrowDown"), chosen: chosen() };
out.back = { asked: await key("ArrowUp"), chosen: chosen() };
out.returnOnAFolder = await key("Enter");
// The stand-in answers every folder with the first, so the folder the user
// was in is shown again before the file is tried.
globalThis.theTab.itIsShowing("row");
await settled();
await key("ArrowDown");
out.returnOnAFile = { asked: await key("Enter"), chosen: chosen() };
// **Return does what a double-click does** — the eighth notes, note 2: on a
// chosen `.blend` it opens Render Settings on its path, and asks nothing else.
await key("ArrowDown");
out.returnOnABlend = { asked: await key("Enter"), chosen: chosen(), opened: globalThis.opened.slice() };
await key("ArrowUp");
// **The page learns it is on a Mac the one way it already does**: the
// system's name `web/chrome.js` writes on the document.
globalThis.document.documentElement = { dataset: { system: "macos" } };
out.backspaceOnAMac = await key("Backspace");
out.commandUpOnAMac = await key("ArrowUp", { metaKey: true });
globalThis.document.documentElement = { dataset: { system: "windows" } };
globalThis.theTab.itIsShowing("row");
await settled();
out.commandUpElsewhere = { asked: await key("ArrowUp", { metaKey: true }), chosen: chosen() };
out.backspaceElsewhere = await key("Backspace");
console.log(JSON.stringify(out));
"#;

    // ── The thirteenth notes, note 6: choosing several rows ─────────────────

    /// **The toolbar's note for any number chosen is the application's** —
    /// *3 selected*, the count a value beside its word and never joined into
    /// it (X7), nothing at all for nought, and one row's note the same words
    /// the toolbar carries as `one_chosen`. The command the page asks is the
    /// same composition. **The delete box names how many** where several go.
    #[test]
    fn the_toolbar_says_how_many_rows_are_chosen() {
        assert_eq!(the_note_for_how_many_chosen(0), None, "nothing chosen is no note, not 0 selected");
        assert_eq!(as_read(&the_note_for_how_many_chosen(1).expect("one")), "1 selected");
        let three = the_note_for_how_many_chosen(3).expect("three");
        assert_eq!(as_read(&three), "3 selected");
        assert_eq!(three.pieces.len(), 2, "the count and its word are two pieces");
        assert_eq!(three.pieces[0].value.as_ref().map(|value| value.as_str()), Some("3"));
        assert!(three.pieces[1].binds, "the word is bound to its figure");
        assert_eq!(the_words_for_how_many_are_chosen(3), Some(three));
        assert_eq!(the_words_for_how_many_are_chosen(0), None);
        assert_eq!(the_toolbar(None).one_chosen, the_note_for_how_many_chosen(1).expect("one"));
        assert!(include_str!("mod.rs").contains("files::the_words_for_how_many_are_chosen,"));
        let victims = vec![
            b"/home/ubuntu/kitchen_v13.blend".to_vec(),
            b"/home/ubuntu/kitchen_v14.blend".to_vec(),
            b"/home/ubuntu/studio_backplate.exr".to_vec(),
        ];
        let the_box = the_delete_box_from(&victims, &[], &[], None, None, false);
        assert_eq!(as_read(&the_box.question), "Delete 3 items?");
    }

    /// **The four presses, the count, Rename greyed, the `.blend` pair, the
    /// acts on every chosen row, Return, the double-click and the right-click
    /// rules** — the real `web/files.js` under Node, the Creator's thirteenth
    /// notes, note 6: *"Why can't I ctrl/cmd+click to select multiple files in
    /// the file browser? Same for shift-clicking."* `Design/File Browser §
    /// Acting on files`, *Rows are chosen the way a file manager chooses
    /// them*. The notes for several are the application's own
    /// ([`the_note_for_how_many_chosen`]), handed to the page's stand-in for
    /// the command. **Skipped, and says so, where no Node is installed.**
    #[test]
    fn rows_are_chosen_the_way_a_file_manager_chooses_them_and_every_act_takes_them_all() {
        let here: &[u8] = b"/home/ubuntu";
        let mut reading = the_tab("row", here, None, a_folder_being_read(here), None, None, false);
        reading.a_reading_follows = true;
        let rows = the_tab_after_the_reading(
            TheOrder::default(),
            "row",
            here,
            None,
            WhatTheReadingFound::TheFolder {
                entries: vec![
                    an_entry("renders", Kind::Folder, 0),
                    an_entry("textures", Kind::Folder, 0),
                    an_entry("kitchen_v13.blend", Kind::File, 1_000),
                    an_entry("kitchen_v14.blend", Kind::File, 1_000),
                    an_entry("notes.txt", Kind::File, 12),
                    an_entry("studio_backplate.exr", Kind::File, 2_000),
                ],
                free: None,
            },
            Some(ObservedValue::Connected),
            false,
        );
        let notes: std::collections::BTreeMap<String, Option<Words>> =
            (2..=6).map(|count| (count.to_string(), the_note_for_how_many_chosen(count))).collect();
        let answers = serde_json::json!({ "reading": reading, "rows": rows, "notes": notes });
        let folder = std::env::temp_dir().join(format!("smoozy-files-several-page-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a scratch folder");
        let web = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        for page in ["files.js", "the-value-drawn.js"] {
            std::fs::copy(web.join(page), folder.join(page)).expect("the page is copied");
        }
        for (name, text) in THE_LISTS_STAND_INS {
            if name != "harness.js" {
                std::fs::write(folder.join(name), text).expect("a stand-in is written");
            }
        }
        // **Transfers and Render Settings, written down when they are asked**.
        std::fs::write(
            folder.join("transfers.js"),
            "export function aTransferIsAskedFor(act, server, paths) { globalThis.asked.push({ command: act, paths: paths.map((one) => String.fromCharCode(...one)) }); } \
             export function theTransfersAreShown() {}",
        )
        .expect("the transfers stand-in is written");
        std::fs::write(
            folder.join("render-settings.js"),
            "export async function theSettingsAreOpenedOn(server, path) { globalThis.asked.push({ command: 'render-settings', paths: [String.fromCharCode(...path)] }); }",
        )
        .expect("the render settings stand-in is written");
        let the_document = &THE_HIDDEN_FILES_HARNESS
            [..THE_HIDDEN_FILES_HARNESS.find("const answers = JSON.parse").expect("the shared document")];
        std::fs::write(folder.join("harness.js"), format!("{the_document}{THE_SEVERAL_ROWS_HARNESS}"))
            .expect("the harness is written");
        std::fs::write(folder.join("answers.json"), answers.to_string()).expect("the answers are written");
        let ran = std::process::Command::new("node").arg(folder.join("harness.js")).current_dir(&folder).output();
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the Files page was not run");
            let _ = std::fs::remove_dir_all(&folder);
            return;
        };
        let _ = std::fs::remove_dir_all(&folder);
        assert!(ran.status.success(), "the page did not run: {}", String::from_utf8_lossy(&ran.stderr));
        let got: serde_json::Value =
            serde_json::from_str(String::from_utf8_lossy(&ran.stdout).trim()).expect("the harness prints one answer");
        let three = ["/home/ubuntu/kitchen_v13.blend", "/home/ubuntu/kitchen_v14.blend", "/home/ubuntu/notes.txt"];
        let wanted = serde_json::json!({
            "order": ["renders", "textures", "kitchen_v13.blend", "kitchen_v14.blend", "notes.txt", "studio_backplate.exr"],
            "click": { "chosen": ["kitchen_v13.blend"], "note": "1 selected", "pair": true, "renameGreyed": false },
            "ctrlClick": { "chosen": ["kitchen_v13.blend", "studio_backplate.exr"], "note": "2 selected", "pair": false, "renameGreyed": true },
            "twoBlends": { "chosen": ["kitchen_v13.blend", "kitchen_v14.blend"], "note": "2 selected", "pair": false, "renameGreyed": true },
            "ctrlClickTakesAway": { "chosen": ["kitchen_v13.blend"], "note": "1 selected", "pair": true, "renameGreyed": false },
            "metaClickElsewhereChoosesAlone": { "chosen": ["notes.txt"], "note": "1 selected" },
            "metaClickOnAMac": { "chosen": ["notes.txt", "studio_backplate.exr"], "note": "2 selected" },
            "shiftClick": { "chosen": ["kitchen_v13.blend", "kitchen_v14.blend", "notes.txt"], "note": "3 selected", "pair": false, "renameGreyed": true },
            "shiftClickFromTheSameRow": { "chosen": ["textures", "kitchen_v13.blend"], "note": "2 selected" },
            "shiftDown": { "chosen": ["kitchen_v13.blend", "kitchen_v14.blend"], "note": "2 selected" },
            "shiftDownAgain": { "chosen": ["kitchen_v13.blend", "kitchen_v14.blend", "notes.txt"], "note": "3 selected" },
            "shiftUp": { "chosen": ["kitchen_v13.blend", "kitchen_v14.blend"], "note": "2 selected" },
            "downFromTheMovingEnd": { "chosen": ["notes.txt"], "note": "1 selected" },
            "theDeleteKey": [{ "command": "the_delete_of", "paths": three }],
            "theDeleteControl": [{ "command": "the_delete_of", "paths": three }],
            "download": [{ "command": "download", "paths": three }],
            "cut": [{ "command": "a_cut_or_copy_is_taken", "act": "cut", "paths": three }],
            "copy": [{ "command": "a_cut_or_copy_is_taken", "act": "copy", "paths": three }],
            "f2": { "asked": [], "typing": false, "line": null },
            "renamePressed": { "asked": [], "typing": false, "line": null },
            "return": { "asked": [], "chosen": ["kitchen_v13.blend", "kitchen_v14.blend", "notes.txt"] },
            "rightClickInside": {
                "chosen": ["kitchen_v13.blend", "kitchen_v14.blend", "notes.txt"],
                "items": ["Download", "Download to…", "Cut", "Copy", "Rename", "Delete"],
                "greyed": ["Rename"],
            },
            "itsDelete": [{ "command": "the_delete_of", "paths": three }],
            "itsDownload": [{ "command": "download", "paths": three }],
            "rightClickOutside": {
                "chosen": ["renders"],
                "items": ["Download", "Download to…", "Calculate size", "Cut", "Copy", "Rename", "Delete"],
                "greyed": [],
                "note": "1 selected",
            },
            "rightClickOnABlendAlone": {
                "chosen": ["kitchen_v14.blend"],
                "items": ["Render settings…", "Check dependencies…", "Download", "Download to…", "Cut", "Copy", "Rename", "Delete"],
                "greyed": [],
            },
            "doubleClick": {
                "chosen": ["kitchen_v14.blend"],
                "note": "1 selected",
                "asked": [{ "command": "render-settings", "paths": ["/home/ubuntu/kitchen_v14.blend"] }],
            },
            "emptySpace": { "chosen": [], "note": null, "pair": false },
            "notesAsked": [2, 3],
        });
        assert_eq!(got, wanted);
    }

    /// **The scenario for
    /// [`rows_are_chosen_the_way_a_file_manager_chooses_them_and_every_act_takes_them_all`]**.
    const THE_SEVERAL_ROWS_HARNESS: &str = r#"const answers = JSON.parse(readFileSync("answers.json", "utf8"));
globalThis.asked = [];
const notesAsked = [];
globalThis.window = {
  innerWidth: 1000, innerHeight: 800,
  __TAURI__: {
    core: {
      invoke(command, args) {
        if (command === "the_files_tab") return Promise.resolve(answers.reading);
        if (command === "the_folder_is_read") return Promise.resolve(answers.rows);
        if (command === "the_saved_with_version_is_read") return Promise.resolve(null);
        if (command === "the_transfers_on_this_server") return Promise.resolve(null);
        if (command === "the_words_for_how_many_are_chosen") {
          notesAsked.push(args.count);
          return Promise.resolve(answers.notes[String(args.count)] || null);
        }
        const text = (bytes) => String.fromCharCode(...bytes);
        if (command === "the_delete_of") globalThis.asked.push({ command, paths: args.victims.map(text) });
        else if (command === "a_cut_or_copy_is_taken") globalThis.asked.push({ command, act: args.act, paths: args.paths.map(text) });
        else globalThis.asked.push({ command });
        return new Promise(() => {});
      },
    },
  },
};
await import("./files.js");
const host = anElement("div");
body.appendChild(host);
globalThis.theTab.itIsDrawn(host);
const settled = async () => { for (let at = 0; at < 6; at += 1) await new Promise((done) => setTimeout(done, 0)); };
const plainly = (text) => (text == null ? text : text.replace(/ /g, " "));
const theList = () => host.querySelector("div.flist");
const rows = () => theList().children.filter((one) => one.classList.contains("frow") && !one.classList.contains("fhead"));
const theName = (one) => one.querySelector(".fname").textContent;
const theRow = (name) => rows().find((one) => theName(one) === name);
const chosen = () => rows().filter((one) => one.getAttribute("aria-current") === "true").map(theName);
const note = () => { const one = host.querySelector(".selnote"); return one ? plainly(one.textContent) : null; };
const tools = () => host.querySelector("div.ftools");
const theControl = (label) => tools().children.find((one) => one.getAttribute && one.getAttribute("aria-label") === label);
const state = () => ({ chosen: chosen(), note: note(), pair: !tools().classList.contains("notblend"), renameGreyed: Boolean(theControl("Rename").disabled) });
const click = async (name, held = {}) => { theRow(name).fire("click", { shiftKey: false, metaKey: false, ctrlKey: false, altKey: false, ...held }); await settled(); };
const key = async (which, held = {}) => {
  for (const handler of keys) {
    handler({ key: which, shiftKey: false, metaKey: false, ctrlKey: false, altKey: false, ...held, preventDefault() {}, defaultPrevented: false, target: body });
  }
  await settled();
};
const asking = async (act) => { globalThis.asked = []; await act(); await settled(); return globalThis.asked.slice(); };
const theMenu = () => host.querySelector("div.fctx");
const menuItems = () => theMenu().children.filter((one) => one.tag === "button");
const menu = () => ({
  items: menuItems().map((one) => plainly(one.textContent)),
  greyed: menuItems().filter((one) => one.disabled && one.classList.contains("off")).map((one) => plainly(one.textContent)),
});
const rightClick = async (name) => { theRow(name).fire("contextmenu", { clientX: 10, clientY: 10 }); await settled(); };

globalThis.theTab.itIsShowing("row");
await settled();
const out = { order: rows().map(theName) };
// **A click chooses one row alone.**
await click("kitchen_v13.blend");
out.click = state();
// **Ctrl-click adds a row, and takes it away** (Windows and Linux).
await click("studio_backplate.exr", { ctrlKey: true });
out.ctrlClick = state();
await click("studio_backplate.exr", { ctrlKey: true });
await click("kitchen_v14.blend", { ctrlKey: true });
out.twoBlends = state();
await click("kitchen_v14.blend", { ctrlKey: true });
out.ctrlClickTakesAway = state();
// **Cmd is a Mac's adding key, and only a Mac's.**
await click("notes.txt", { metaKey: true });
out.metaClickElsewhereChoosesAlone = { chosen: chosen(), note: note() };
globalThis.document.documentElement = { dataset: { system: "macos" } };
await click("studio_backplate.exr", { metaKey: true });
out.metaClickOnAMac = { chosen: chosen(), note: note() };
globalThis.document.documentElement = { dataset: { system: "windows" } };
// **Shift-click chooses the range from the last row clicked, in the list's
// drawn order**, and leaves the last row clicked where it was.
await click("kitchen_v13.blend");
await click("notes.txt", { shiftKey: true });
out.shiftClick = state();
await click("textures", { shiftKey: true });
out.shiftClickFromTheSameRow = { chosen: chosen(), note: note() };
// **Shift with ↓ or ↑ widens the choice by one row**; a plain ↓ steps from
// the moving end and chooses alone.
await click("kitchen_v13.blend");
await key("ArrowDown", { shiftKey: true });
out.shiftDown = { chosen: chosen(), note: note() };
await key("ArrowDown", { shiftKey: true });
out.shiftDownAgain = { chosen: chosen(), note: note() };
await key("ArrowUp", { shiftKey: true });
out.shiftUp = { chosen: chosen(), note: note() };
await key("ArrowDown");
out.downFromTheMovingEnd = { chosen: chosen(), note: note() };
// **Delete, Download, Cut and Copy act on every chosen row**, by path, in
// the drawn order.
await click("kitchen_v13.blend");
await click("notes.txt", { shiftKey: true });
out.theDeleteKey = await asking(() => key("Delete"));
out.theDeleteControl = await asking(() => theControl("Delete").press("click"));
out.download = await asking(() => theControl("Download").press("click"));
out.cut = await asking(() => theControl("Cut").press("click"));
out.copy = await asking(() => theControl("Copy").press("click"));
// **Rename is greyed while several are chosen**: F2 and the control do
// nothing and say nothing.
out.f2 = { asked: await asking(() => key("F2")), typing: Boolean(host.querySelector("input")), line: (() => { const one = host.querySelector(".fnotyet"); return one && !one.hidden ? one.textContent : null; })() };
out.renamePressed = { asked: await asking(() => theControl("Rename").press("click")), typing: Boolean(host.querySelector("input")), line: (() => { const one = host.querySelector(".fnotyet"); return one && !one.hidden ? one.textContent : null; })() };
// **Return does nothing while several are chosen.**
out.return = { asked: await asking(() => key("Enter")), chosen: chosen() };
// **A right-click on a chosen row acts on all of them, Rename greyed.**
await rightClick("kitchen_v14.blend");
out.rightClickInside = { chosen: chosen(), ...menu() };
out.itsDelete = await asking(() => menuItems().find((one) => plainly(one.textContent) === "Delete").press("click"));
await rightClick("kitchen_v14.blend");
out.itsDownload = await asking(() => menuItems().find((one) => plainly(one.textContent) === "Download").press("click"));
// **On a row outside the choice it chooses that row alone first.**
await rightClick("renders");
out.rightClickOutside = { chosen: chosen(), ...menu(), note: note() };
await rightClick("kitchen_v14.blend");
out.rightClickOnABlendAlone = { chosen: chosen(), ...menu() };
// **A double-click chooses its row alone and opens it.**
await click("kitchen_v13.blend");
await click("notes.txt", { shiftKey: true });
const opened = await asking(() => theRow("kitchen_v14.blend").fire("dblclick", {}));
out.doubleClick = { chosen: chosen(), note: note(), asked: opened };
// **A click on the list's empty space lets the choice go.**
await click("notes.txt", { shiftKey: true });
host.fire("click", { target: theList() });
await settled();
out.emptySpace = { chosen: chosen(), note: note(), pair: !tools().classList.contains("notblend") };
out.notesAsked = notesAsked;
console.log(JSON.stringify(out));
"#;

    /// **Step 10c's owed piece 2: Render Settings opened from its strip is
    /// handed the version Files read** — the real `web/files.js` and the real
    /// `web/render-settings.js` run together under Node. Files lists a folder
    /// holding `scene.blend` and asks for its tag behind the rows. Render
    /// Settings, never opened over that server, is opened from its strip on
    /// that file (the way `web/a-server.js` presses it): **before the tag
    /// arrives** it hands no version, leaving the application's own fallback
    /// (`the_file_was_pointed_at`); **after it arrives** it hands the version
    /// Files read; **on another server** the same path is handed nothing, the
    /// read being that server's. No check starts and the page reads nothing for
    /// itself (`Render Settings` B2): the only reading of the file is Files'
    /// own. **Skipped, and says so, where no Node is installed.**
    #[test]
    fn render_settings_opened_from_its_strip_is_handed_the_version_files_read() {
        let here: &[u8] = b"/home/ubuntu";
        let server = "owed-piece-2-server";
        let mut reading = the_tab(server, here, None, a_folder_being_read(here), None, None, false);
        reading.a_reading_follows = true;
        let rows = the_tab_after_the_reading(
            TheOrder::default(),
            server,
            here,
            None,
            WhatTheReadingFound::TheFolder {
                entries: vec![an_entry("textures", Kind::Folder, 0), an_entry("scene.blend", Kind::File, 1_000)],
                free: None,
            },
            Some(ObservedValue::Connected),
            false,
        );
        let answers = serde_json::json!({ "reading": reading, "rows": rows, "server": server });
        let folder = std::env::temp_dir().join(format!("smoozy-files-to-settings-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a scratch folder");
        let web = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        for page in ["files.js", "render-settings.js", "the-value-drawn.js"] {
            std::fs::copy(web.join(page), folder.join(page)).expect("the page is copied");
        }
        for (name, text) in THE_LISTS_STAND_INS {
            if !["harness.js", "render-settings.js"].contains(&name) {
                std::fs::write(folder.join(name), text).expect("a stand-in is written");
            }
        }
        // **What the two real pages import between them**, over the lists'
        // own stand-ins: the panel registry records the panels, and a check
        // started anywhere is counted.
        for (name, text) in [
            (
                "a-server.js",
                r#"export const FILES = "files"; export const RENDER_SETTINGS = "set"; export const SET_UP = "setupview";
export function aTabIsRegistered(tab) { globalThis.theTab = tab; }
export function aPanelIsOpenOver() { return false; }
export const panels = [];
export function aPanelIsRegistered(panel) { panels.push(panel); }
export function thePanelIsOpenedOver() {} export function theStripReads() {} export function theCurrentTabIs() {}
export function thePanelCollapsesAsItsChevronDoes() {}"#,
            ),
            (
                "the-dependency-check.js",
                r#"export function theCheckIsOpenedOn() { globalThis.checksStarted = (globalThis.checksStarted || 0) + 1; }
export function theCheckIsShownOn() {}"#,
            ),
            (
                "transfers.js",
                r#"export function aTransferIsAskedFor() {} export function theTransfersAreShown() {}
export async function theFormatBoxIsOpenedForAFile() {}"#,
            ),
            (
                "the-underway-shape.js",
                r#"export function theMarkOverAPane() { return document.createElement("div"); }
export function theMarkOnAControl() {} export function theMarkOnARow() { return document.createElement("span"); }"#,
            ),
            ("servers.js", "export function theMomentReads(seconds) { return `at ${seconds}`; }"),
            ("the-rail.js", "export const THE_QUEUE = 'queueview'; export function theCurrentPlaceIs() {}"),
            (
                "the-failure-shape.js",
                "export const EXPANDED = 'e'; export function theFailureShape() { return document.createElement('div'); }",
            ),
            ("the-machines-own-words.js", "export function theMachinesOwnWordsFor() {}"),
            (
                "the-typed-text.js",
                "export function theTypingIsTakenFrom() { return null; } export function theTypingIsPutBackInto() {}",
            ),
        ] {
            std::fs::write(folder.join(name), text).expect("a stand-in is written");
        }
        let the_document = &THE_HIDDEN_FILES_HARNESS
            [..THE_HIDDEN_FILES_HARNESS.find("const answers = JSON.parse").expect("the shared document")];
        std::fs::write(folder.join("harness.js"), format!("{the_document}{THE_FILES_TO_SETTINGS_HARNESS}"))
            .expect("the harness is written");
        std::fs::write(folder.join("answers.json"), answers.to_string()).expect("the answers are written");
        let ran = std::process::Command::new("node").arg(folder.join("harness.js")).current_dir(&folder).output();
        let _ = std::fs::remove_dir_all(&folder);
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the two pages were not run");
            return;
        };
        assert!(ran.status.success(), "the pages did not run: {}", String::from_utf8_lossy(&ran.stderr));
        let got: serde_json::Value =
            serde_json::from_str(String::from_utf8_lossy(&ran.stdout).trim()).expect("the harness prints one answer");
        let scene: Vec<u8> = b"/home/ubuntu/scene.blend".to_vec();
        assert_eq!(
            got,
            serde_json::json!({
                "tagAsked": [scene.clone()],
                "beforeTheRead": { "pointedAt": [[server, scene.clone(), ""]], "readsOfTheFile": 0 },
                "drawn": "4.5.3",
                "afterTheRead": { "pointedAt": [[server, scene.clone(), "4.5.3"]], "readsOfTheFile": 0 },
                "onAnotherServer": { "pointedAt": [["another-server", scene, ""]], "readsOfTheFile": 0 },
                "checksStarted": 0,
            })
        );
    }

    /// **The scenario for
    /// [`render_settings_opened_from_its_strip_is_handed_the_version_files_read`]**.
    /// The tag is held back until the harness lets it arrive, so the strip can
    /// be pressed on either side of Files' read.
    const THE_FILES_TO_SETTINGS_HARNESS: &str = r#"const answers = JSON.parse(readFileSync("answers.json", "utf8"));
let asked = [];
const tagAsked = [];
let theTagArrives = null;
globalThis.window = {
  innerWidth: 1000, innerHeight: 800,
  __TAURI__: {
    core: {
      invoke(command, args) {
        asked.push({ command, args });
        if (command === "the_files_tab") return Promise.resolve(answers.reading);
        if (command === "the_folder_is_read") return Promise.resolve(answers.rows);
        if (command === "the_saved_with_version_is_read") {
          tagAsked.push(args.path);
          return new Promise((arrive) => { theTagArrives = arrive; });
        }
        if (command === "the_file_was_pointed_at" || command === "the_version_list_is_asked_for") return Promise.resolve(null);
        return new Promise(() => {});
      },
    },
  },
};
await import("./files.js");
const { panels } = await import("./a-server.js");
const host = anElement("div");
body.appendChild(host);
globalThis.theTab.itIsDrawn(host);
const settled = async () => { for (let at = 0; at < 8; at += 1) await new Promise((done) => setTimeout(done, 0)); };
globalThis.theTab.itIsShowing(answers.server);
await settled();
const scene = Array.from(Buffer.from("/home/ubuntu/scene.blend"));
const theSettings = panels.find((one) => typeof one.itIsShownOn === "function" && one.panel === "set");
// **The strip pressed**, as `web/a-server.js` presses it: the panel's own way
// in, on the open panel's file.
const fromTheStrip = async (server) => {
  asked = [];
  await theSettings.itIsShownOn(server, scene);
  await settled();
  return {
    pointedAt: asked.filter((one) => one.command === "the_file_was_pointed_at").map((one) => [one.args.server, one.args.file, one.args.savedWith]),
    readsOfTheFile: asked.filter((one) => one.command === "the_saved_with_version_is_read").length,
  };
};
const out = { tagAsked };
out.beforeTheRead = await fromTheStrip(answers.server);
theTagArrives("4.5.3");
await settled();
const row = host.querySelector("div.flist").children.find((one) => one.querySelector && one.querySelector(".fname") && one.querySelector(".fname").textContent.startsWith("scene.blend"));
const tag = row && row.querySelector(".ver");
out.drawn = tag ? tag.textContent : null;
out.afterTheRead = await fromTheStrip(answers.server);
out.onAnotherServer = await fromTheStrip("another-server");
out.checksStarted = globalThis.checksStarted || 0;
console.log(JSON.stringify(out));
"#;

    /// **The Creator's third notes on the test copy, piece D — Files' clicks
    /// and keys**, run under Node over the application's own answers.
    ///
    /// - **Note 6**: ↓ with nothing chosen takes the first row with the focus
    ///   left on the Files tab, and ↑ the last with it left on the server's row
    ///   in the list of servers — both outside the tab's own body, where the
    ///   keys went nowhere before; a click on the list's empty space lets the
    ///   choice go, and a click that reached a row does not.
    /// - **Note 5**: going up the whole list one row at a time, the chosen row
    ///   always lies below the pinned column heading, never under it. The stand
    ///   page scrolls to the nearest edge, as the browser does.
    /// - **Note 7**: one click on a `.blend` chooses it and opens nothing and
    ///   asks nothing; a double-click opens Render Settings on its path. **And
    ///   since the eighth notes, note 2, a folder is the same**: one click
    ///   chooses it and asks for no listing, and a double-click — two clicks
    ///   and the double-click, as a browser sends it — asks for its listing
    ///   once, by its whole path. Until 2026-10-12 one click opened it.
    /// - **Note 8**: a press inside the row being typed keeps it; a press
    ///   anywhere else puts the name back, asking nothing.
    ///
    /// **Skipped, and says so, where no Node is installed.**
    #[test]
    fn files_clicks_and_keys_follow_the_creators_third_notes() {
        let here: &[u8] = b"/home/ubuntu";
        let mut reading = the_tab("row", here, None, a_folder_being_read(here), None, None, false);
        reading.a_reading_follows = true;
        let mut entries = vec![an_entry("a-folder", Kind::Folder, 0)];
        for at in 1..=10 {
            entries.push(an_entry(&format!("file-{at:02}.txt"), Kind::File, 12));
        }
        entries.push(an_entry("scene.blend", Kind::File, 1_000));
        let rows = the_tab_after_the_reading(
            TheOrder::default(),
            "row",
            here,
            None,
            WhatTheReadingFound::TheFolder { entries, free: None },
            Some(ObservedValue::Connected),
            false,
        );
        let answers = serde_json::json!({ "reading": reading, "rows": rows });
        let folder = std::env::temp_dir().join(format!("smoozy-files-piece-d-page-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a scratch folder");
        let web = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        for page in ["files.js", "the-value-drawn.js"] {
            std::fs::copy(web.join(page), folder.join(page)).expect("the page is copied");
        }
        for (name, text) in THE_LISTS_STAND_INS {
            if name != "harness.js" {
                std::fs::write(folder.join(name), text).expect("a stand-in is written");
            }
        }
        // **Render Settings, written down when it is opened**, so a click that
        // opened it would show.
        std::fs::write(
            folder.join("render-settings.js"),
            "export async function theSettingsAreOpenedOn(server, path) { globalThis.opened.push(String.fromCharCode(...path)); }",
        )
        .expect("the render settings stand-in is written");
        let the_document = &THE_HIDDEN_FILES_HARNESS
            [..THE_HIDDEN_FILES_HARNESS.find("const answers = JSON.parse").expect("the shared document")];
        std::fs::write(folder.join("harness.js"), format!("{the_document}{THE_PIECE_D_HARNESS}"))
            .expect("the harness is written");
        std::fs::write(folder.join("answers.json"), answers.to_string()).expect("the answers are written");
        let ran = std::process::Command::new("node").arg(folder.join("harness.js")).current_dir(&folder).output();
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the Files page was not run");
            let _ = std::fs::remove_dir_all(&folder);
            return;
        };
        let _ = std::fs::remove_dir_all(&folder);
        assert!(ran.status.success(), "the page did not run: {}", String::from_utf8_lossy(&ran.stderr));
        let got: serde_json::Value =
            serde_json::from_str(String::from_utf8_lossy(&ran.stdout).trim()).expect("the harness prints one answer");
        let mut order = vec!["a-folder".to_owned()];
        order.extend((1..=10).map(|at| format!("file-{at:02}.txt")));
        order.push("scene.blend".to_owned());
        let wanted = serde_json::json!({
            "order": order,
            "downFromTheFilesTab": ["a-folder"],
            "aClickOnARowKeepsIt": ["a-folder"],
            "aClickOnEmptySpaceLetsGo": [],
            "upFromTheServerList": ["scene.blend"],
            "underTheHeadingGoingUp": [],
            "theTopReached": { "chosen": ["a-folder"], "scrolled": 0 },
            "oneClickOnABlend": { "chosen": ["scene.blend"], "asked": [], "opened": [] },
            "doubleClickOnABlend": { "opened": ["/home/ubuntu/scene.blend"] },
            "oneClickOnAFolder": { "chosen": ["a-folder"], "asked": [], "opened": [] },
            "aDoubleClickOnAFolder": {
                "asked": [{ "command": "the_files_tab", "folder": "/home/ubuntu/a-folder" }],
                "opened": [],
            },
            "typing": { "field": true, "value": "file-02.txt" },
            "aPressInsideTheRow": { "field": true, "value": "half typed" },
            "aPressOutside": { "field": false, "chosen": ["file-02.txt"], "asked": [] },
        });
        assert_eq!(got, wanted);
    }

    /// **The scenario for [`files_clicks_and_keys_follow_the_creators_third_notes`]**.
    ///
    /// **The stand-in page gains what this scenario needs**: a press heard on
    /// the document, rows that stand 30 pixels apart in a list 300 tall, and a
    /// row's scrolling into view to the nearest edge, as the browser does.
    const THE_PIECE_D_HARNESS: &str = r#"const answers = JSON.parse(readFileSync("answers.json", "utf8"));
let asked = [];
globalThis.opened = [];
const presses = [];
document.addEventListener = (kind, handler) => {
  if (kind === "keydown") keys.push(handler);
  if (kind === "mousedown") presses.push(handler);
};
document.removeEventListener = (kind, handler) => {
  const from = kind === "keydown" ? keys : kind === "mousedown" ? presses : [];
  const at = from.indexOf(handler);
  if (at >= 0) from.splice(at, 1);
};
const made = document.createElement;
document.createElement = (tag) => {
  const one = made(tag);
  Object.defineProperty(one, "offsetTop", { get() { return one.parentNode ? one.parentNode.children.indexOf(one) * 30 : 0; } });
  one.scrollIntoView = () => {
    const holder = one.parentNode;
    if (!holder) return;
    const top = one.offsetTop;
    const end = top + one.offsetHeight;
    if (top < holder.scrollTop) holder.scrollTop = top;
    else if (end > holder.scrollTop + holder.clientHeight) holder.scrollTop = end - holder.clientHeight;
  };
  const finding = one.querySelector;
  one.querySelector = (selector) =>
    selector === "[aria-current=true]"
      ? one.children.find((child) => child.getAttribute && child.getAttribute("aria-current") === "true") || null
      : finding(selector);
  return one;
};
globalThis.window = {
  innerWidth: 1000, innerHeight: 800,
  __TAURI__: {
    core: {
      invoke(command, args) {
        if (command === "the_files_tab") {
          asked.push({ command, folder: args.folder ? String.fromCharCode(...args.folder) : null });
          return Promise.resolve(answers.reading);
        }
        if (command === "the_folder_is_read") return Promise.resolve(answers.rows);
        if (command === "the_saved_with_version_is_read") return Promise.resolve(null);
        asked.push({ command });
        return new Promise(() => {});
      },
    },
  },
};
await import("./files.js");
// **The server's screen**: the Files tab above the tab's own body, and the
// list of servers beside it, outside the screen.
const screen = anElement("div");
body.appendChild(screen);
const filesTab = anElement("button");
screen.appendChild(filesTab);
const host = anElement("div");
screen.appendChild(host);
const aServersRow = anElement("div");
body.appendChild(aServersRow);
globalThis.theTab.itIsDrawn(host);
const settled = async () => { for (let at = 0; at < 6; at += 1) await new Promise((done) => setTimeout(done, 0)); };
const list = () => host.querySelector("div.flist");
const rows = () => list().children.filter((one) => one.classList.contains("frow") && !one.classList.contains("fhead"));
const theRow = (name) => rows().find((one) => theName(one) === name);
const theName = (one) => one.querySelector(".fname").textContent;
const chosen = () => rows().filter((one) => one.getAttribute("aria-current") === "true").map(theName);
const key = async (which, target) => {
  for (const handler of keys) {
    handler({ key: which, shiftKey: false, metaKey: false, ctrlKey: false, altKey: false, preventDefault() {}, defaultPrevented: false, target });
  }
  await settled();
};
const aPressOn = async (target) => {
  for (const handler of presses.slice()) handler({ target, preventDefault() {} });
  await settled();
};
globalThis.theTab.itIsShowing("row");
await settled();
const out = { order: rows().map(theName) };

// Note 6.
await key("ArrowDown", filesTab);
out.downFromTheFilesTab = chosen();
host.fire("click", { target: rows()[3] });
out.aClickOnARowKeepsIt = chosen();
host.fire("click", { target: list() });
out.aClickOnEmptySpaceLetsGo = chosen();
await key("ArrowUp", aServersRow);
out.upFromTheServerList = chosen();

// Note 5.
out.underTheHeadingGoingUp = [];
const head = list().children.find((one) => one.classList.contains("fhead"));
for (let step = 0; step < 11; step += 1) {
  await key("ArrowUp", body);
  const marked = rows().find((one) => one.getAttribute("aria-current") === "true");
  const from = marked.offsetTop - list().scrollTop;
  if (from < head.offsetHeight || from + marked.offsetHeight > list().clientHeight) out.underTheHeadingGoingUp.push(theName(marked));
}
out.theTopReached = { chosen: chosen(), scrolled: list().scrollTop };

// Note 7.
asked = [];
theRow("scene.blend").press("click");
await settled();
out.oneClickOnABlend = { chosen: chosen(), asked: asked.slice(), opened: globalThis.opened.slice() };
theRow("scene.blend").press("dblclick");
await settled();
out.doubleClickOnABlend = { opened: globalThis.opened.slice() };
globalThis.opened = [];
asked = [];
theRow("a-folder").press("click");
await settled();
out.oneClickOnAFolder = { chosen: chosen(), asked: asked.slice(), opened: globalThis.opened.slice() };
// **A double-click, as a browser sends one**: two clicks, then the
// double-click. Only the last asks for anything, and it asks once.
theRow("a-folder").press("click");
await settled();
theRow("a-folder").press("dblclick");
await settled();
out.aDoubleClickOnAFolder = { asked: asked.slice(), opened: globalThis.opened.slice() };

// Note 8. The stand-in answers every folder with the first, so the folder
// the user was in is shown again first.
globalThis.theTab.itIsShowing("row");
await settled();
asked = [];
host.fire("click", { target: list() });
await key("ArrowDown", body);
await key("ArrowDown", body);
await key("ArrowDown", body);
await key("F2", body);
const field = () => list().querySelector("input");
out.typing = { field: Boolean(field()), value: field() && field().value };
field().value = "half typed";
field().fire("input");
await aPressOn(field());
out.aPressInsideTheRow = { field: Boolean(field()), value: field() && field().value };
await aPressOn(list());
out.aPressOutside = { field: Boolean(field()), chosen: chosen(), asked: asked.slice() };
console.log(JSON.stringify(out));
"#;

    /// **A pick or a drop starts the upload, and no box comes up** —
    /// `Design/File Browser § Uploading`, struck 2026-10-04; the Creator's
    /// second notes on the test copy, piece 9 (note 2). Run under Node over the
    /// application's own answers: Upload pressed where no panel takes both
    /// draws the kind question inside Upload and starts nothing; *Files*
    /// answered and picked, the upload goes at once with what was picked; a
    /// drop on the list goes at once with what was dropped. **No box is raised
    /// at any point.** Skipped, and says so, where no Node is installed.
    #[test]
    fn a_pick_or_a_drop_starts_the_upload_with_no_box() {
        let here: &[u8] = b"/home/ubuntu";
        let mut reading = the_tab("row", here, None, a_folder_being_read(here), None, None, false);
        reading.a_reading_follows = true;
        let rows = the_tab_after_the_reading(
            TheOrder::default(),
            "row",
            here,
            None,
            WhatTheReadingFound::TheFolder { entries: vec![an_entry("renders", Kind::Folder, 0)], free: None },
            Some(ObservedValue::Connected),
            false,
        );
        let picked = [
            std::path::PathBuf::from("/Users/a/textures"),
            std::path::PathBuf::from("/Users/a/one.png"),
        ];
        let answers = serde_json::json!({
            "reading": reading,
            "rows": rows,
            "whichKind": the_upload_ask_from(crate::picker::Picked::ThereIsNoPanelForThisAsk(
                crate::picker::WhatMayBeChosen::FilesAndFolders,
            )),
            "go": the_upload_goes_with(&picked),
        });
        let folder = std::env::temp_dir().join(format!("smoozy-files-upload-page-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a scratch folder");
        let web = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        for page in ["files.js", "the-value-drawn.js"] {
            std::fs::copy(web.join(page), folder.join(page)).expect("the page is copied");
        }
        for (name, text) in THE_LISTS_STAND_INS {
            if name != "harness.js" {
                std::fs::write(folder.join(name), text).expect("a stand-in is written");
            }
        }
        let the_document = &THE_HIDDEN_FILES_HARNESS
            [..THE_HIDDEN_FILES_HARNESS.find("const answers = JSON.parse").expect("the shared document")];
        std::fs::write(folder.join("harness.js"), format!("{the_document}{THE_UPLOAD_HARNESS}"))
            .expect("the harness is written");
        std::fs::write(folder.join("answers.json"), answers.to_string()).expect("the answers are written");
        let ran = std::process::Command::new("node").arg(folder.join("harness.js")).current_dir(&folder).output();
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the Files page was not run");
            let _ = std::fs::remove_dir_all(&folder);
            return;
        };
        let _ = std::fs::remove_dir_all(&folder);
        assert!(ran.status.success(), "the page did not run: {}", String::from_utf8_lossy(&ran.stderr));
        let got: serde_json::Value =
            serde_json::from_str(String::from_utf8_lossy(&ran.stdout).trim()).expect("the harness prints one answer");
        let went = serde_json::json!({ "command": "the_upload_is_confirmed", "chosen": ["/Users/a/textures", "/Users/a/one.png"] });
        let wanted = serde_json::json!({
            "uploadPressed": { "asked": [{ "command": "an_upload_is_asked_for", "what": "both" }], "question": true, "boxes": 0 },
            "filesAnswered": { "asked": [{ "command": "an_upload_is_asked_for", "what": "files" }, went], "question": false, "boxes": 0 },
            "dropped": { "asked": [{ "command": "files_were_dropped", "paths": ["/Users/a/textures", "/Users/a/one.png"] }, went], "boxes": 0 },
        });
        assert_eq!(got, wanted);
    }

    /// **The scenario for [`a_pick_or_a_drop_starts_the_upload_with_no_box`]**.
    const THE_UPLOAD_HARNESS: &str = r#"const answers = JSON.parse(readFileSync("answers.json", "utf8"));
let asked = [];
let theDrop = null;
const theAsks = [answers.whichKind, answers.go];
globalThis.window = {
  innerWidth: 1000, innerHeight: 800,
  __TAURI__: {
    core: {
      invoke(command, args) {
        if (command === "the_files_tab") return Promise.resolve(answers.reading);
        if (command === "the_folder_is_read") return Promise.resolve(answers.rows);
        if (command === "the_saved_with_version_is_read") return Promise.resolve(null);
        if (command === "what_is_being_dropped") return new Promise(() => {});
        if (command === "an_upload_is_asked_for") {
          asked.push({ command, what: args.what });
          return Promise.resolve(theAsks.shift());
        }
        if (command === "files_were_dropped") {
          asked.push({ command, paths: args.paths });
          return Promise.resolve(answers.go);
        }
        if (command === "the_upload_is_confirmed") {
          asked.push({ command, chosen: args.chosen.map((bytes) => String.fromCharCode(...bytes)) });
          return new Promise(() => {});
        }
        asked.push({ command });
        return new Promise(() => {});
      },
    },
    // **The shell's own drag and drop**, held so the scenario can drop.
    webviewWindow: {
      getCurrentWebviewWindow: () => ({
        onDragDropEvent(handler) { theDrop = handler; return Promise.resolve(() => {}); },
      }),
    },
  },
};
await import("./files.js");
const host = anElement("div");
body.appendChild(host);
globalThis.theTab.itIsDrawn(host);
const settled = async () => { for (let at = 0; at < 6; at += 1) await new Promise((done) => setTimeout(done, 0)); };
const boxes = () => host.querySelectorAll("div.modal").length;
const question = () => host.querySelector("div.fask") !== null;
const taken = () => { const out = asked.slice(); asked = []; return out; };
globalThis.theTab.itIsShowing("row");
await settled();
asked = [];
host.querySelector("span.uphold").querySelector("button.ib").press("click");
await settled();
const out = {};
out.uploadPressed = { asked: taken(), question: question(), boxes: boxes() };
host.querySelector("div.fask").querySelector("button").press("click");
await settled();
out.filesAnswered = { asked: taken(), question: question(), boxes: boxes() };
theDrop({ payload: { type: "drop", paths: ["/Users/a/textures", "/Users/a/one.png"] } });
await settled();
out.dropped = { asked: taken(), boxes: boxes() };
console.log(JSON.stringify(out));
"#;

    /// **The line above the list belongs to the server it was said on**
    /// (`Design/File Browser § The list`; the Creator's notes 2 and 3 on 0.1.8,
    /// step 10c piece 1), run under Node over the application's own answers:
    /// going to another server clears it; an upload's ending, or an act's
    /// answer, that comes while the user is on another server — or while this
    /// server's Files is not the tab showing — is kept for its server and
    /// shown once, the next time that server's Files is shown, and nowhere
    /// else; and the same server shown again keeps its line. **Skipped, and
    /// says so, where no Node is installed.**
    #[test]
    fn the_line_above_the_list_belongs_to_the_server_it_was_said_on() {
        use crate::managing_files::WhatTheDeleteCameTo;
        let here: &[u8] = b"/home/ubuntu";
        let mut reading = the_tab("row", here, None, a_folder_being_read(here), None, None, false);
        reading.a_reading_follows = true;
        let rows = the_tab_after_the_reading(
            TheOrder::default(),
            "row",
            here,
            None,
            WhatTheReadingFound::TheFolder { entries: a_folder_with_hidden_names(), free: None },
            Some(ObservedValue::Connected),
            false,
        );
        let uploaded = |files: usize, into: &[u8]| {
            what_it_came_to(
                &WhatTheUploadCameTo { uploaded: files, skipped: 0, failed: Vec::new(), did_not_finish: 0, refused: Vec::new() },
                into,
            )
        };
        let victim = b"/home/ubuntu/scene.blend".to_vec();
        let answers = serde_json::json!({
            "reading": reading,
            "rows": rows,
            "endings": {
                "to-root": uploaded(1, b"/root"),
                "to-data": uploaded(2, b"/data"),
                "to-home": uploaded(1, b"/home/ubuntu"),
                "while-hidden": uploaded(3, b"/srv"),
            },
            "box": the_delete_box_from(&[victim.clone()], &[], &[], None, None, false),
            "deleted": what_the_delete_says(
                &WhatTheDeleteCameTo::Ran {
                    record: Identity::issue(IdentityKind::Operation).expect("randomness"),
                    done: 1,
                    refused: 0,
                    not_attempted: 0,
                    never_learned: 0,
                },
                &[victim],
            ),
        });
        let folder = std::env::temp_dir().join(format!("smoozy-files-line-page-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a scratch folder");
        let web = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        for page in ["files.js", "the-value-drawn.js"] {
            std::fs::copy(web.join(page), folder.join(page)).expect("the page is copied");
        }
        for (name, text) in THE_LISTS_STAND_INS {
            if name != "harness.js" {
                std::fs::write(folder.join(name), text).expect("a stand-in is written");
            }
        }
        std::fs::write(
            folder.join("event-route.js"),
            "export function whenTold(kind, handler) { (globalThis.told ||= {})[kind] = handler; }",
        )
        .expect("the route's stand-in");
        let the_document = &THE_HIDDEN_FILES_HARNESS
            [..THE_HIDDEN_FILES_HARNESS.find("const answers = JSON.parse").expect("the shared document")];
        std::fs::write(folder.join("harness.js"), format!("{the_document}{THE_LINES_SERVER_HARNESS}"))
            .expect("the harness is written");
        std::fs::write(folder.join("answers.json"), answers.to_string()).expect("the answers are written");
        let ran = std::process::Command::new("node").arg(folder.join("harness.js")).current_dir(&folder).output();
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the Files page was not run");
            let _ = std::fs::remove_dir_all(&folder);
            return;
        };
        let _ = std::fs::remove_dir_all(&folder);
        assert!(ran.status.success(), "the page did not run: {}", String::from_utf8_lossy(&ran.stderr));
        let got: serde_json::Value =
            serde_json::from_str(String::from_utf8_lossy(&ran.stdout).trim()).expect("the harness prints one answer");
        let wanted = serde_json::json!({
            "said": "Uploaded 1 file into /root.",
            "moved": null,
            "elsewhere": "Uploaded 2 files into /data.",
            "back": "Uploaded 1 file into /home/ubuntu.",
            "kept": "Uploaded 1 file into /home/ubuntu.",
            "again": null,
            "hiddenBack": "Uploaded 3 files into /srv.",
            "beforeTheAnswer": null,
            "answerElsewhere": null,
            "answerBack": "Deleted scene.blend.",
            "answerAgain": null,
            "taken": ["to-root", "to-data", "to-home", "while-hidden"],
        });
        assert_eq!(got, wanted);
    }

    /// **The line above the list belongs to the folder and the act that made
    /// it** — the runner's ruling (c) of 2026-10-03, written into
    /// `Design/File Browser § The list`. Out of the Vast pass before 0.1.9,
    /// where the line followed the user into other folders. Run under Node
    /// over the application's own answers: the same folder read again keeps
    /// the line; a new act's line replaces it; opening another folder clears
    /// it, and coming back does not bring it back. **Skipped, and says so,
    /// where no Node is installed.**
    #[test]
    fn the_line_above_the_list_belongs_to_the_folder_it_was_said_in() {
        let home: &[u8] = b"/home/ubuntu";
        let root: &[u8] = b"/root";
        let reading_of = |here: &[u8]| {
            let mut reading = the_tab("row", here, None, a_folder_being_read(here), None, None, false);
            reading.a_reading_follows = true;
            reading
        };
        let rows_of = |here: &[u8]| {
            the_tab_after_the_reading(
                TheOrder::default(),
                "row",
                here,
                None,
                WhatTheReadingFound::TheFolder { entries: a_folder_with_hidden_names(), free: None },
                Some(ObservedValue::Connected),
                false,
            )
        };
        let uploaded = |files: usize, into: &[u8]| {
            what_it_came_to(
                &WhatTheUploadCameTo { uploaded: files, skipped: 0, failed: Vec::new(), did_not_finish: 0, refused: Vec::new() },
                into,
            )
        };
        let answers = serde_json::json!({
            "home": { "reading": reading_of(home), "rows": rows_of(home) },
            "root": { "reading": reading_of(root), "rows": rows_of(root) },
            "rootBytes": root,
            "homeBytes": home,
            "endings": {
                "to-home": uploaded(1, home),
                "to-home-again": uploaded(2, home),
            },
        });
        let folder = std::env::temp_dir().join(format!("smoozy-files-line-folder-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a scratch folder");
        let web = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        for page in ["files.js", "the-value-drawn.js"] {
            std::fs::copy(web.join(page), folder.join(page)).expect("the page is copied");
        }
        for (name, text) in THE_LISTS_STAND_INS {
            if name != "harness.js" {
                std::fs::write(folder.join(name), text).expect("a stand-in is written");
            }
        }
        std::fs::write(
            folder.join("event-route.js"),
            "export function whenTold(kind, handler) { (globalThis.told ||= {})[kind] = handler; }",
        )
        .expect("the route's stand-in");
        let the_document = &THE_HIDDEN_FILES_HARNESS
            [..THE_HIDDEN_FILES_HARNESS.find("const answers = JSON.parse").expect("the shared document")];
        std::fs::write(folder.join("harness.js"), format!("{the_document}{THE_LINES_FOLDER_HARNESS}"))
            .expect("the harness is written");
        std::fs::write(folder.join("answers.json"), answers.to_string()).expect("the answers are written");
        let ran = std::process::Command::new("node").arg(folder.join("harness.js")).current_dir(&folder).output();
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the Files page was not run");
            let _ = std::fs::remove_dir_all(&folder);
            return;
        };
        let _ = std::fs::remove_dir_all(&folder);
        assert!(ran.status.success(), "the page did not run: {}", String::from_utf8_lossy(&ran.stderr));
        let got: serde_json::Value =
            serde_json::from_str(String::from_utf8_lossy(&ran.stdout).trim()).expect("the harness prints one answer");
        let wanted = serde_json::json!({
            "said": "Uploaded 1 file into /home/ubuntu.",
            "readAgain": "Uploaded 1 file into /home/ubuntu.",
            "replaced": "Uploaded 2 files into /home/ubuntu.",
            "inAnotherFolder": null,
            "backAgain": null,
        });
        assert_eq!(got, wanted);
    }

    /// **The scenario for [`the_line_above_the_list_belongs_to_the_folder_it_was_said_in`]**.
    const THE_LINES_FOLDER_HARNESS: &str = r#"const answers = JSON.parse(readFileSync("answers.json", "utf8"));
const which = (args) => (args && args.folder && args.folder.join(",") === answers.rootBytes.join(",") ? answers.root : answers.home);
globalThis.window = {
  innerWidth: 1000, innerHeight: 800,
  __TAURI__: {
    core: {
      invoke(command, args) {
        if (command === "the_files_tab") return Promise.resolve(which(args).reading);
        if (command === "the_folder_is_read") return Promise.resolve(which(args).rows);
        if (command === "the_saved_with_version_is_read") return Promise.resolve(null);
        if (command === "what_the_upload_came_to") {
          const words = answers.endings[args.request] || null;
          delete answers.endings[args.request];
          return Promise.resolve(words);
        }
        return new Promise(() => {});
      },
    },
  },
};
const page = await import("./files.js");
const host = anElement("div");
body.appendChild(host);
globalThis.theTab.itIsDrawn(host);
const settled = async () => { for (let at = 0; at < 6; at += 1) await new Promise((done) => setTimeout(done, 0)); };
const plainly = (text) => (text == null ? text : text.replace(/ /g, " "));
const line = () => { const one = host.querySelector(".fnotyet"); return one && !one.hidden && one.textContent ? plainly(one.textContent) : null; };
const shows = async (which) => { globalThis.theTab.itIsShowing(which); await settled(); };
const opens = async (bytes) => { page.theBrowserIsOpenedAt("server-1", bytes); await shows("server-1"); };
const ends = async (request) => { globalThis.told["files:the-answer"]({ said: { what: "the-upload-ended", server: "server-1", request } }); await settled(); };
const out = {};
await shows("server-1");
await ends("to-home");
out.said = line();
await shows("server-1");
out.readAgain = line();
await ends("to-home-again");
out.replaced = line();
await opens(answers.rootBytes);
out.inAnotherFolder = line();
await opens(answers.homeBytes);
out.backAgain = line();
console.log(JSON.stringify(out));
"#;

    /// **The file's name in a panel's title opens Files on that file's folder
    /// with the file chosen** — `Design/Whole app § Panels over a server`,
    /// added 2026-10-12 (the Creator's ninth notes, note 5).
    ///
    /// `web/files.js` runs under Node over the application's own answers, and
    /// is handed a folder and a name the way the two panels hand them
    /// (`theBrowserIsOpenedAt(server, folder, name)`): **while Files is the
    /// tab showing**, which is the ordinary case — the panel covered Files —
    /// and while Set up was. Each time the folder and the name are what Files
    /// asks the application for, the row by that name is the one marked, the
    /// toolbar says one is chosen, and the row is brought into view in a folder
    /// longer than the list. **A file that has gone leaves its folder open with
    /// nothing chosen**, and Set up's folder link, handed no name, chooses
    /// nothing. **It fails if the name is dropped, if a hand-over while Files
    /// shows waits for the next showing, or if a gone file is marked.**
    /// Skipped, and says so, where no Node is installed.
    #[test]
    fn a_folder_and_a_name_handed_over_open_files_with_that_file_chosen() {
        let here: &[u8] = b"/home/ubuntu/kitchen";
        let mut entries: Vec<ListedEntry> =
            (1..=20).map(|at| an_entry(&format!("shot_{at:02}"), Kind::Folder, 0)).collect();
        entries.push(an_entry("kitchen_v14.blend", Kind::File, 1_000));
        entries.push(an_entry("props.blend", Kind::File, 500));
        let mut reading = the_tab("row", here, None, a_folder_being_read(here), None, None, false);
        reading.a_reading_follows = true;
        let rows_of = |chosen: Option<&[u8]>| {
            the_tab_after_the_reading(
                TheOrder::default(),
                "row",
                here,
                chosen,
                WhatTheReadingFound::TheFolder { entries: entries.clone(), free: None },
                Some(ObservedValue::Connected),
                false,
            )
        };
        let answers = serde_json::json!({
            "reading": reading,
            "rows": {
                "": rows_of(None),
                "kitchen_v14.blend": rows_of(Some(b"kitchen_v14.blend")),
                "props.blend": rows_of(Some(b"props.blend")),
                "gone.blend": rows_of(Some(b"gone.blend")),
            },
        });
        let folder = std::env::temp_dir().join(format!("smoozy-files-a-name-handed-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a scratch folder");
        let web = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        for page in ["files.js", "the-value-drawn.js"] {
            std::fs::copy(web.join(page), folder.join(page)).expect("the page is copied");
        }
        for (name, text) in THE_LISTS_STAND_INS {
            if name != "harness.js" {
                std::fs::write(folder.join(name), text).expect("a stand-in is written");
            }
        }
        // **The shared document, with rows that sit thirty points apart**, so a
        // row below the list's three hundred is out of view until it is brought
        // in.
        let the_document = THE_HIDDEN_FILES_HARNESS
            [..THE_HIDDEN_FILES_HARNESS.find("const answers = JSON.parse").expect("the shared document")]
            .replace(
                "get offsetTop() { return 0; },",
                "get offsetTop() { return element.parentNode ? element.parentNode.children.indexOf(element) * 30 : 0; },",
            );
        assert!(the_document.contains("indexOf(element) * 30"), "the shared document's rows no longer stand where this test puts them");
        std::fs::write(folder.join("harness.js"), format!("{the_document}{THE_NAME_HANDED_HARNESS}"))
            .expect("the harness is written");
        std::fs::write(folder.join("answers.json"), answers.to_string()).expect("the answers are written");
        let ran = std::process::Command::new("node").arg(folder.join("harness.js")).current_dir(&folder).output();
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the Files page was not run");
            let _ = std::fs::remove_dir_all(&folder);
            return;
        };
        let _ = std::fs::remove_dir_all(&folder);
        assert!(ran.status.success(), "the page did not run: {}", String::from_utf8_lossy(&ran.stderr));
        let got: serde_json::Value =
            serde_json::from_str(String::from_utf8_lossy(&ran.stdout).trim()).expect("the harness prints one answer");
        let at = "/home/ubuntu/kitchen";
        let wanted = serde_json::json!({
            "first": { "asked": [{ "folder": null, "chosen": null }], "chosen": [], "selected": false, "inView": true },
            "whileFilesShows": {
                "asked": [{ "folder": at, "chosen": "kitchen_v14.blend" }],
                "chosen": ["kitchen_v14.blend"],
                "selected": true,
                "inView": true,
            },
            "aFolderAlone": { "asked": [{ "folder": at, "chosen": null }], "chosen": [], "selected": false, "inView": true },
            "whileSetUpShowed": {
                "asked": [{ "folder": at, "chosen": "props.blend" }],
                "chosen": ["props.blend"],
                "selected": true,
                "inView": true,
            },
            "aFileThatHasGone": { "asked": [{ "folder": at, "chosen": "gone.blend" }], "chosen": [], "selected": false, "inView": true },
        });
        assert_eq!(got, wanted, "Files handed a folder and a name does not open on that file chosen");
    }

    /// **The scenario for [`a_folder_and_a_name_handed_over_open_files_with_that_file_chosen`]**.
    const THE_NAME_HANDED_HARNESS: &str = r#"const answers = JSON.parse(readFileSync("answers.json", "utf8"));
const text = (bytes) => (bytes ? new TextDecoder().decode(Uint8Array.from(bytes)) : null);
let asked = [];
globalThis.window = {
  innerWidth: 1000, innerHeight: 800,
  __TAURI__: {
    core: {
      invoke(command, args) {
        if (command === "the_files_tab") {
          asked.push({ folder: text(args.folder), chosen: text(args.chosen) });
          return Promise.resolve(answers.reading);
        }
        if (command === "the_folder_is_read") return Promise.resolve(answers.rows[text(args.chosen) || ""]);
        if (command === "the_saved_with_version_is_read") return Promise.resolve(null);
        return new Promise(() => {});
      },
    },
  },
};
const page = await import("./files.js");
const host = anElement("div");
body.appendChild(host);
globalThis.theTab.itIsDrawn(host);
const settled = async () => { for (let at = 0; at < 6; at += 1) await new Promise((done) => setTimeout(done, 0)); };
const bytes = (one) => Array.from(new TextEncoder().encode(one));
const list = () => host.querySelector("div.flist");
const rows = () => list().children.filter((one) => one.classList.contains("frow") && !one.classList.contains("fhead"));
const theState = () => {
  const marked = rows().filter((one) => one.getAttribute("aria-current") === "true");
  const shown = list();
  return {
    asked: asked.splice(0),
    chosen: marked.map((one) => one.querySelector(".fname").textContent),
    selected: Boolean(host.querySelector(".selnote")),
    inView: marked.every((one) => one.offsetTop >= shown.scrollTop && one.offsetTop + one.offsetHeight <= shown.scrollTop + shown.clientHeight),
  };
};
const folder = bytes("/home/ubuntu/kitchen");
const out = {};
globalThis.theTab.itIsShowing("row");
await settled();
out.first = theState();
// **Files is the tab showing** — the panel the name was pressed in covered it.
page.theBrowserIsOpenedAt("row", folder, bytes("kitchen_v14.blend"));
await settled();
out.whileFilesShows = theState();
// **Set up's folder link hands a folder and no name.**
page.theBrowserIsOpenedAt("row", folder);
await settled();
out.aFolderAlone = theState();
// **Set up was the tab showing**, so Files takes the two as it shows.
globalThis.theTab.itIsPutAway();
page.theBrowserIsOpenedAt("row", folder, bytes("props.blend"));
await settled();
globalThis.theTab.itIsShowing("row");
await settled();
out.whileSetUpShowed = theState();
page.theBrowserIsOpenedAt("row", folder, bytes("gone.blend"));
await settled();
out.aFileThatHasGone = theState();
console.log(JSON.stringify(out));
"#;

    /// **The scenario for [`the_line_above_the_list_belongs_to_the_server_it_was_said_on`]**.
    /// Two servers share the one Files host, as they do in the app.
    const THE_LINES_SERVER_HARNESS: &str = r#"const answers = JSON.parse(readFileSync("answers.json", "utf8"));
let taken = [];
let theDeleteEnds = null;
globalThis.window = {
  innerWidth: 1000, innerHeight: 800,
  __TAURI__: {
    core: {
      invoke(command, args) {
        if (command === "the_files_tab") return Promise.resolve(answers.reading);
        if (command === "the_folder_is_read") return Promise.resolve(answers.rows);
        if (command === "the_saved_with_version_is_read") return Promise.resolve(null);
        // **The application lets the words go as it hands them over**, as
        // `what_the_upload_came_to` does.
        if (command === "what_the_upload_came_to") {
          taken.push(args.request);
          const words = answers.endings[args.request] || null;
          delete answers.endings[args.request];
          return Promise.resolve(words);
        }
        if (command === "the_delete_of") return Promise.resolve(answers.box);
        if (command === "a_delete_is_made") return new Promise((done) => { theDeleteEnds = () => done(answers.deleted); });
        return new Promise(() => {});
      },
    },
  },
};
await import("./files.js");
const host = anElement("div");
body.appendChild(host);
globalThis.theTab.itIsDrawn(host);
const settled = async () => { for (let at = 0; at < 6; at += 1) await new Promise((done) => setTimeout(done, 0)); };
const rows = () => host.querySelector("div.flist").children.filter((one) => one.classList.contains("frow") && !one.classList.contains("fhead"));
const plainly = (text) => (text == null ? text : text.replace(/\u00A0/g, " "));
const key = async (which) => { for (const handler of keys) handler({ key: which, preventDefault() {}, defaultPrevented: false, target: body }); await settled(); };
const line = () => { const one = host.querySelector(".fnotyet"); return one && !one.hidden && one.textContent ? plainly(one.textContent) : null; };
const shows = async (which) => { globalThis.theTab.itIsShowing(which); await settled(); };
const ends = async (which, request) => { globalThis.told["files:the-answer"]({ said: { what: "the-upload-ended", server: which, request } }); await settled(); };
const out = {};
await shows("server-1");
await ends("server-1", "to-root");
out.said = line();
await shows("server-2");
out.moved = line();
await ends("server-2", "to-data");
await ends("server-1", "to-home");
out.elsewhere = line();
await shows("server-1");
out.back = line();
await shows("server-1");
out.kept = line();
await shows("server-2");
await shows("server-1");
out.again = line();
// **This server's Files not the tab showing** — Set up is, on the same server.
host.hidden = true;
await ends("server-1", "while-hidden");
host.hidden = false;
await shows("server-1");
out.hiddenBack = line();
// **An act's answer**: a delete asked on server-1, answered on server-2.
rows().find((one) => one.querySelector(".fname").textContent === "scene.blend").press("click");
await settled();
await key("Delete");
body.querySelectorAll("div.modal")[0].querySelector("button.btn.danger").press("click");
await settled();
await shows("server-2");
out.beforeTheAnswer = line();
theDeleteEnds();
await settled();
out.answerElsewhere = line();
await shows("server-1");
out.answerBack = line();
await shows("server-2");
await shows("server-1");
out.answerAgain = line();
out.taken = taken;
console.log(JSON.stringify(out));
"#;

    /// **What a delete says when it ends** — `Design/File Browser § What the
    /// screen says around an act`: what was done, what was refused, what was
    /// not attempted, and where the line went, that some were never answered.
    #[test]
    fn a_delete_says_what_went_what_was_refused_and_what_was_not_tried() {
        use crate::managing_files::{WhatTheActCameTo, WhatTheDeleteCameTo};
        let id = || Identity::issue(IdentityKind::Operation).expect("randomness");
        let victims = [b"/home/ubuntu/cache".to_vec()];
        let ran = |done, refused, not_attempted, never_learned| {
            what_the_delete_says(
                &WhatTheDeleteCameTo::Ran { record: id(), done, refused, not_attempted, never_learned },
                &victims,
            )
        };
        let AnActsAnswer::Done { said: Some(said) } = ran(40, 0, 0, 0) else {
            panic!("a whole delete said nothing");
        };
        assert_eq!(said.values(), vec!["cache"]);
        let AnActsAnswer::Said { said } = ran(38, 1, 2, 0) else {
            panic!("a partial delete was called done");
        };
        assert_eq!(said.values(), vec!["38", "1", "2"]);
        assert!(!said.said().contains("Connection"));
        let AnActsAnswer::Said { said } = ran(10, 0, 0, 3) else {
            panic!("a delete with answers never learned was called done");
        };
        assert!(said.said().contains("Connection lost"), "{}", said.said());
        let AnActsAnswer::Said { said } = what_the_delete_says(
            &WhatTheDeleteCameTo::NothingWasRemoved(WhatTheActCameTo::TheTellingWouldNotWrite),
            &victims,
        ) else {
            panic!("a delete not attempted was called done");
        };
        assert_eq!(said.said().replace('\u{a0}', " "), words::NOTHING_DELETED_THE_RECORD);
    }

    /// **`File Browser` M15 on the screen: Paste carries its count on the
    /// toolbar and on the folder menu, and on no other menu** — and nothing at
    /// all where the pasteboard is not this machine's.
    #[test]
    fn paste_carries_its_count_on_the_toolbar_and_the_folder_menu_alone() {
        let here: &[u8] = b"/home/ubuntu";
        let mut browser = the_tab_after_the_reading(
            TheOrder::default(),
            "row",
            here,
            None,
            WhatTheReadingFound::TheFolder { entries: a_folder_with_hidden_names(), free: None },
            Some(ObservedValue::Connected),
            false,
        );
        paste_is_offered_on(&mut browser, None);
        assert!(browser.toolbar.paste.is_none());
        assert!(browser.menus.iter().all(|menu| menu.items.iter().all(|item| item.act.as_str() != "paste")));
        paste_is_offered_on(&mut browser, Some(3));
        assert_eq!(browser.toolbar.paste.as_ref().expect("Paste").values(), vec!["3"]);
        for menu in &browser.menus {
            let pastes = menu.items.iter().filter(|item| item.act.as_str() == "paste").count();
            assert_eq!(pastes, usize::from(menu.which == "folder"), "{}", menu.which);
        }
    }

    /// **What a paste says** — `Design/File Browser § What the screen says
    /// around an act`, and the question that comes first where something
    /// collides (`File Browser` M8: a move offers Replace or Cancel).
    #[test]
    fn a_paste_says_what_moved_and_asks_first_where_something_collides() {
        use crate::managing_files::{WhatThePasteCameTo as Paste, WhyThePasteIsRefused};
        use smoozy_file_browser::already_exists::{AComparison, Comparison};
        let plainly = |words: &Words| words.said().replace('\u{a0}', " ");
        let entries = [b"/home/ubuntu/textures".to_vec()];
        let says = |came: Paste| what_the_paste_says(&came, &entries, b"/home/ubuntu/archive", &[], &[], TheCollidingAct::APasteAfterACut);
        let AnActsAnswer::Said { said } = says(Paste::Refused(WhyThePasteIsRefused::AFolderIntoItself)) else {
            panic!("a folder into itself was not refused on the line");
        };
        assert_eq!(plainly(&said), words::A_FOLDER_INTO_ITSELF);
        let AnActsAnswer::Asks { question } = says(Paste::AsksFirst(vec![
            AComparison { landing_path: b"/home/ubuntu/archive/b.png".to_vec(), what: Comparison::CollidesWithAFile },
            AComparison { landing_path: b"/home/ubuntu/archive/a.png".to_vec(), what: Comparison::LandsFresh },
        ])) else {
            panic!("a collision did not ask first");
        };
        let answers: Vec<&str> = question.answers.iter().map(|one| one.answer.as_str()).collect();
        assert_eq!(answers, vec!["replace", "cancel"], "a move offers Replace or Cancel");
        assert_eq!(question.names.len(), 1);
        let AnActsAnswer::Done { said: Some(said) } = says(Paste::Ran {
            record: Identity::issue(IdentityKind::Operation).expect("randomness"),
            done: 1,
            refused: 0,
            not_done: 0,
            never_learned: 0,
            crossings: 0,
            crossings_want_an_update: false,
        }) else {
            panic!("a whole move said nothing");
        };
        assert_eq!(said.values(), vec!["textures"]);
        // ***Moved textures.***, not *Moved to textures.*: the name is the
        // item moved, never where it went. The pass before 0.1.9 on Vast read
        // *Moved to file1.txt.*
        assert_eq!(plainly(&said), format!("{} .", words::MOVED_ITEMS_LEAD), "{}", plainly(&said));
        // **One item is one item**, where a merge moved one of several — the
        // pass before 0.1.9 on Vast read *1 items*.
        let two = [b"/home/ubuntu/textures".to_vec(), b"/home/ubuntu/wood.png".to_vec()];
        let AnActsAnswer::Done { said: Some(one_of_two) } = what_the_paste_says(
            &Paste::Ran {
                record: Identity::issue(IdentityKind::Operation).expect("randomness"),
                done: 1,
                refused: 0,
                not_done: 0,
                never_learned: 0,
                crossings: 0,
                crossings_want_an_update: false,
            },
            &two,
            b"/home/ubuntu/archive",
            &[],
            &[],
            TheCollidingAct::APasteAfterACut,
        ) else {
            panic!("a whole move said nothing");
        };
        assert!(plainly(&one_of_two).contains(words::PASTE_ONE_ITEM), "{}", plainly(&one_of_two));
        assert!(!plainly(&one_of_two).contains(words::PASTE_ITEMS), "{}", plainly(&one_of_two));
        let AnActsAnswer::Said { said } = says(Paste::Ran {
            record: Identity::issue(IdentityKind::Operation).expect("randomness"),
            done: 2,
            refused: 0,
            not_done: 1,
            never_learned: 0,
            crossings: 1,
            crossings_want_an_update: false,
        }) else {
            panic!("a partial move was called done");
        };
        assert_eq!(said.values(), vec!["2", "0", "1", "1"]);
        assert!(plainly(&said).contains("another disk"), "{}", plainly(&said));
        assert!(plainly(&said).ends_with(words::SET_UP_TO_MOVE_BETWEEN_DISKS), "{}", plainly(&said));
        // **Design's *Update* where Smoozy on the server is older than the
        // copy request** (step 10c's Files' owed acts piece 7).
        let AnActsAnswer::Said { said } = says(Paste::Ran {
            record: Identity::issue(IdentityKind::Operation).expect("randomness"),
            done: 0,
            refused: 0,
            not_done: 1,
            never_learned: 0,
            crossings: 1,
            crossings_want_an_update: true,
        }) else {
            panic!("a move that could not cross was called done");
        };
        assert!(plainly(&said).ends_with(words::UPDATE_TO_MOVE_BETWEEN_DISKS), "{}", plainly(&said));
    }

    /// **Cut, Copy and Paste on the page, run under Node over the
    /// application's own answers** (step 10c's Files' owed acts piece 5): Cut
    /// with nothing chosen says so; Cut with a row chosen hands its **path**
    /// over and the folder is read again; Paste on the toolbar asks the
    /// live-job box, then the paste into the folder being browsed; the
    /// question is drawn, and *Replace* asks the paste again with the answer.
    /// **Skipped, and says so, where no Node is installed.**
    #[test]
    fn the_page_cuts_the_chosen_path_and_pastes_into_the_folder_with_the_answer() {
        use smoozy_file_browser::already_exists::{AComparison, Comparison};
        let here: &[u8] = b"/home/ubuntu";
        let mut reading = the_tab("row", here, None, a_folder_being_read(here), None, None, false);
        reading.a_reading_follows = true;
        let mut rows = the_tab_after_the_reading(
            TheOrder::default(),
            "row",
            here,
            None,
            WhatTheReadingFound::TheFolder { entries: a_folder_with_hidden_names(), free: None },
            Some(ObservedValue::Connected),
            false,
        );
        paste_is_offered_on(&mut rows, Some(1));
        let asks = what_the_paste_says(
            &crate::managing_files::WhatThePasteCameTo::AsksFirst(vec![AComparison {
                landing_path: b"/home/ubuntu/scene.blend".to_vec(),
                what: Comparison::CollidesWithAFile,
            }]),
            &[b"/home/ubuntu/renders/scene.blend".to_vec()],
            here,
            &[],
            &[],
            TheCollidingAct::APasteAfterACut,
        );
        let answers = serde_json::json!({
            "reading": reading,
            "rows": rows,
            "asks": asks,
            "done": AnActsAnswer::Done { said: None },
        });
        let folder = std::env::temp_dir().join(format!("smoozy-paste-page-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a scratch folder");
        let web = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        for page in ["files.js", "the-value-drawn.js"] {
            std::fs::copy(web.join(page), folder.join(page)).expect("the page is copied");
        }
        for (name, text) in THE_LISTS_STAND_INS {
            if name != "harness.js" {
                std::fs::write(folder.join(name), text).expect("a stand-in is written");
            }
        }
        let the_document = &THE_HIDDEN_FILES_HARNESS
            [..THE_HIDDEN_FILES_HARNESS.find("const answers = JSON.parse").expect("the shared document")];
        std::fs::write(folder.join("harness.js"), format!("{the_document}{THE_PASTE_HARNESS}"))
            .expect("the harness is written");
        std::fs::write(folder.join("answers.json"), answers.to_string()).expect("the answers are written");
        let ran = std::process::Command::new("node").arg(folder.join("harness.js")).current_dir(&folder).output();
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the Files page was not run");
            let _ = std::fs::remove_dir_all(&folder);
            return;
        };
        let _ = std::fs::remove_dir_all(&folder);
        assert!(ran.status.success(), "the page did not run: {}", String::from_utf8_lossy(&ran.stderr));
        let got: serde_json::Value =
            serde_json::from_str(String::from_utf8_lossy(&ran.stdout).trim()).expect("the harness prints one answer");
        let wanted = serde_json::json!({
            "nothingChosen": { "line": "Select a file or folder to cut.", "asked": [] },
            "cut": { "asked": [
                { "command": "a_cut_or_copy_is_taken", "act": "cut", "paths": [b"/home/ubuntu/scene.blend".to_vec()] },
                { "command": "the_files_tab" },
            ] },
            "pasted": { "asked": [
                { "command": "what_a_paste_costs", "into": here.to_vec() },
                { "command": "a_paste_is_made", "into": here.to_vec(), "answer": null },
            ], "boxes": 1 },
            "answered": { "asked": [
                { "command": "a_paste_is_made", "into": here.to_vec(), "answer": "replace" },
                { "command": "the_files_tab" },
            ] },
        });
        assert_eq!(got, wanted);
    }

    // ── The twenty-third notes, note 2: a folder's size, counted on request ──

    fn in_plain(words: &Words) -> String {
        as_read(words).replace('\u{a0}', " ")
    }

    fn a_count_of(bytes: u64, files: u64, folders: u64, incomplete: bool) -> crate::managing_files::WhatTheCountCameTo {
        crate::managing_files::WhatTheCountCameTo::Counted(crate::managing_files::WhatTheCountFound {
            bytes,
            files,
            folders,
            incomplete,
        })
    }

    /// **The folder menu carries *Calculate size* in a group of its own right
    /// under the two downloads**, and no other menu does; **greyed where the
    /// server cannot count**, saying why in Copy's words, and greyed with
    /// nothing said before the release is known — and Paste's own rebuild of
    /// the menus keeps none of that by accident, since the gate comes after it.
    #[test]
    fn twenty_third_2_the_menu_item_sits_under_the_downloads_and_greys_on_an_older_server() {
        let menus = the_menus_with(Some(2));
        let folder: Vec<&str> = menus[0].items.iter().map(|one| one.act.as_str()).collect();
        assert_eq!(&folder[..4], &["download", "download-to", THE_CALCULATE_SIZE_ACT, "cut"]);
        assert!(menus[0].items[2].after_a_divider && menus[0].items[3].after_a_divider);
        assert!(menus[1..].iter().all(|menu| menu.items.iter().all(|one| one.act.as_str() != THE_CALCULATE_SIZE_ACT)));
        let here: &[u8] = b"/home/ubuntu";
        let mut browser = the_tab_after_the_reading(
            TheOrder::default(),
            "row",
            here,
            None,
            WhatTheReadingFound::TheFolder { entries: a_folder_with_hidden_names(), free: None },
            Some(ObservedValue::Connected),
            false,
        );
        let the_item = |browser: &TheBrowser| {
            browser.menus[0].items.iter().find(|one| one.act.as_str() == THE_CALCULATE_SIZE_ACT).cloned().expect("the item")
        };
        paste_is_offered_on(&mut browser, Some(1));
        calculate_size_is_offered_on(&mut browser, Some(WhereTheCountStands::Offered));
        assert_eq!(the_item(&browser).greyed, None);
        calculate_size_is_offered_on(&mut browser, Some(WhereTheCountStands::SmoozyThereIsOlder));
        let why = the_item(&browser).greyed.and_then(|greyed| greyed.why).expect("a reason");
        assert_eq!(in_plain(&why), words::UPDATE_TO_CALCULATE);
        calculate_size_is_offered_on(&mut browser, Some(WhereTheCountStands::SmoozyIsNotSetUpThere));
        let why = the_item(&browser).greyed.and_then(|greyed| greyed.why).expect("a reason");
        assert_eq!(in_plain(&why), words::SET_UP_TO_CALCULATE);
        calculate_size_is_offered_on(&mut browser, Some(WhereTheCountStands::NotYetKnown));
        assert_eq!(the_item(&browser).greyed, Some(AGreying { why: None }));
        // **Unavailable while the listing is stale**, as every act that needs the machine.
        assert!(!the_unavailable_acts(HowTheBrowserComposes::Live).contains(&THE_CALCULATE_SIZE_ACT));
        assert!(the_unavailable_acts(HowTheBrowserComposes::FromTheCacheAndStale).contains(&THE_CALCULATE_SIZE_ACT));
    }

    /// **Each state of the cell** — the total in a file's form, *at least*
    /// with the second hover line, *Failed* with the reason — and the hover
    /// counts with their thousands marked.
    #[test]
    fn twenty_third_2_the_cell_says_each_state() {
        let path: &[u8] = b"/home/ubuntu/assets";
        let whole = what_the_folder_size_says(path, &a_count_of(4_080_218_931, 1_204, 87, false));
        assert_eq!(whole.cell.values(), vec!["3.8\u{a0}GB"]);
        assert_eq!(whole.hover.len(), 1);
        assert_eq!(in_plain(&whole.hover[0]), "1,204 files, 87 folders");
        assert_eq!(whole.sorts_by, Some(4_080_218_931));
        let one = what_the_folder_size_says(path, &a_count_of(10, 1, 1, false));
        assert_eq!(in_plain(&one.hover[0]), "1 file, 1 folder");
        let at_least = what_the_folder_size_says(path, &a_count_of(44_775_000_000, 1_912, 140, true));
        assert_eq!(in_plain(&at_least.cell), "at least 42 GB", "a file's form: one decimal below 10 only");
        assert_eq!(in_plain(&at_least.hover[1]), words::SOME_FOLDERS_COULD_NOT_BE_READ);
        for (came_to, why) in [
            (crate::managing_files::WhatTheCountCameTo::NeverAnswered, words::NO_REPLY_TO_THE_COUNT),
            (crate::managing_files::WhatTheCountCameTo::TheProgramIsOlder, words::UPDATE_TO_CALCULATE),
            (crate::managing_files::WhatTheCountCameTo::NothingThere, words::IT_IS_NOT_THERE_ANY_MORE),
            (crate::managing_files::WhatTheCountCameTo::NotAFolder, words::NOT_A_FOLDER_ANY_MORE),
        ] {
            let failed = what_the_folder_size_says(path, &came_to);
            assert_eq!(in_plain(&failed.cell), words::FAILED);
            assert_eq!(in_plain(&failed.hover[0]), in_plain(&Words::plain(why)));
            assert_eq!(failed.sorts_by, None);
        }
        assert_eq!(a_count(1_234_567), "1,234,567");
        assert_eq!(a_count(999), "999");
    }

    /// **The page counts a folder from its menu and fills its row** — run
    /// under Node over the application's own answers: *Calculate size* asks
    /// once for the folder's path, the cell says *Calculating…* and the item
    /// greys while it does, the answer lands as *at least 42 GB* with its
    /// two hover lines, and on an older server the item is greyed with its
    /// reason on hover and asks nothing. **Skipped, and says so, where no
    /// Node is installed.**
    #[test]
    fn twenty_third_2_the_page_counts_a_folder_from_its_menu_and_fills_its_row() {
        let here: &[u8] = b"/home/ubuntu";
        let mut reading = the_tab("row", here, None, a_folder_being_read(here), None, None, false);
        reading.a_reading_follows = true;
        let rows_with = |stands: WhereTheCountStands| {
            let mut rows = the_tab_after_the_reading(
                TheOrder::default(),
                "row",
                here,
                None,
                WhatTheReadingFound::TheFolder { entries: a_folder_with_hidden_names(), free: None },
                Some(ObservedValue::Connected),
                false,
            );
            calculate_size_is_offered_on(&mut rows, Some(stands));
            rows
        };
        let counted = what_the_folder_size_says(b"/home/ubuntu/renders", &a_count_of(44_775_000_000, 1_912, 140, true));
        let run = |rows: TheBrowser| -> Option<serde_json::Value> {
            let answers = serde_json::json!({ "reading": reading, "rows": rows, "counted": counted });
            let folder = std::env::temp_dir().join(format!(
                "smoozy-size-page-{}-{}",
                std::process::id(),
                &Identity::issue(IdentityKind::Operation).expect("randomness").as_text()[..8]
            ));
            let _ = std::fs::remove_dir_all(&folder);
            std::fs::create_dir_all(&folder).expect("a scratch folder");
            let web = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
            for page in ["files.js", "the-value-drawn.js"] {
                std::fs::copy(web.join(page), folder.join(page)).expect("the page is copied");
            }
            for (name, text) in THE_LISTS_STAND_INS {
                if name != "harness.js" {
                    std::fs::write(folder.join(name), text).expect("a stand-in is written");
                }
            }
            let the_document = &THE_HIDDEN_FILES_HARNESS
                [..THE_HIDDEN_FILES_HARNESS.find("const answers = JSON.parse").expect("the shared document")];
            std::fs::write(folder.join("harness.js"), format!("{the_document}{THE_SIZE_HARNESS}"))
                .expect("the harness is written");
            std::fs::write(folder.join("answers.json"), answers.to_string()).expect("the answers are written");
            let ran = std::process::Command::new("node").arg(folder.join("harness.js")).current_dir(&folder).output();
            let _ = std::fs::remove_dir_all(&folder);
            let ran = ran.ok()?;
            assert!(ran.status.success(), "the page did not run: {}", String::from_utf8_lossy(&ran.stderr));
            Some(serde_json::from_str(String::from_utf8_lossy(&ran.stdout).trim()).expect("the harness prints one answer"))
        };
        let Some(offered) = run(rows_with(WhereTheCountStands::Offered)) else {
            eprintln!("SKIPPED: no Node on this computer, so the Files page was not run");
            return;
        };
        let renders = b"/home/ubuntu/renders".to_vec();
        assert_eq!(
            offered,
            serde_json::json!({
                "menu": { "items": ["Download", "Download to…", "Calculate size", "Cut", "Copy", "Rename", "Delete"], "greyed": [] },
                "asked": [{ "command": "a_folders_size_is_counted", "path": renders }],
                "while": { "cell": "Calculating…", "greyed": ["Calculate size"] },
                "landed": { "cell": "at least 42 GB", "hover": "1,912 files, 140 foldersSome folders could not be read." },
            })
        );
        let older = run(rows_with(WhereTheCountStands::SmoozyThereIsOlder)).expect("Node ran once");
        assert_eq!(
            older,
            serde_json::json!({
                "menu": { "items": ["Download", "Download to…", "Calculate size", "Cut", "Copy", "Rename", "Delete"], "greyed": ["Calculate size"] },
                "hover": words::UPDATE_TO_CALCULATE,
                "asked": [],
            })
        );
    }

    /// **The scenario for [`twenty_third_2_the_page_counts_a_folder_from_its_menu_and_fills_its_row`]**.
    const THE_SIZE_HARNESS: &str = r#"const answers = JSON.parse(readFileSync("answers.json", "utf8"));
let asked = [];
let land = null;
globalThis.window = {
  innerWidth: 1000, innerHeight: 800,
  __TAURI__: {
    core: {
      invoke(command, args) {
        if (command === "the_files_tab") return Promise.resolve(answers.reading);
        if (command === "the_folder_is_read") return Promise.resolve(answers.rows);
        if (command === "a_folders_size_is_counted") {
          asked.push({ command, path: args.path });
          return new Promise((done) => { land = () => done(answers.counted); });
        }
        if (command === "the_rows_are_put_in_order") return new Promise(() => {});
        return new Promise(() => {});
      },
    },
  },
};
await import("./files.js");
const host = anElement("div");
body.appendChild(host);
globalThis.theTab.itIsDrawn(host);
const settled = async () => { for (let at = 0; at < 8; at += 1) await new Promise((done) => setTimeout(done, 0)); };
const plainly = (text) => (text == null ? text : text.replace(/\u00A0/g, " "));
const rows = () => host.querySelector("div.flist").children.filter((one) => one.classList.contains("frow") && !one.classList.contains("fhead"));
const theRow = (name) => rows().find((one) => one.querySelector(".fname").textContent === name);
const theCell = () => theRow("renders").children.filter((one) => one.classList && one.classList.contains("r") && !one.classList.contains("when"))[0];
const theMenu = () => host.querySelector("div.fctx");
const menuItems = () => theMenu().children.filter((one) => one.tag === "button");
const menu = () => ({
  items: menuItems().map((one) => plainly(one.textContent)),
  greyed: menuItems().filter((one) => one.disabled && one.classList.contains("off")).map((one) => plainly(one.textContent)),
});
const theTip = () => { const one = host.querySelector("span.fsztip"); return one ? plainly(one.textContent) : null; };
const rightClick = async () => { theRow("renders").fire("contextmenu", { clientX: 10, clientY: 10 }); await settled(); };
globalThis.theTab.itIsShowing("row");
await settled();
const out = {};
await rightClick();
out.menu = menu();
const item = menuItems().find((one) => plainly(one.textContent) === "Calculate size");
if (item.disabled) {
  item.fire("mouseenter", {});
  out.hover = theTip();
  item.press("click");
  await settled();
  out.asked = asked.slice();
} else {
  item.press("click");
  await settled();
  out.asked = asked.slice();
  const cell = plainly(theCell().textContent);
  await rightClick();
  out.while = { cell, greyed: menu().greyed };
  host.fire("keydown", { key: "Escape" });
  land();
  await settled();
  theCell().fire("mouseenter", {});
  out.landed = { cell: plainly(theCell().textContent), hover: theTip() };
}
console.log(JSON.stringify(out));
"#;

    /// **The scenario for [`the_page_cuts_the_chosen_path_and_pastes_into_the_folder_with_the_answer`]**.
    const THE_PASTE_HARNESS: &str = r#"const answers = JSON.parse(readFileSync("answers.json", "utf8"));
let asked = [];
globalThis.window = {
  innerWidth: 1000, innerHeight: 800,
  __TAURI__: {
    core: {
      invoke(command, args) {
        if (command === "the_files_tab") { asked.push({ command }); return Promise.resolve(answers.reading); }
        if (command === "the_folder_is_read") return Promise.resolve(answers.rows);
        if (command === "the_saved_with_version_is_read") return Promise.resolve(null);
        if (command === "a_cut_or_copy_is_taken") { asked.push({ command, act: args.act, paths: args.paths }); return Promise.resolve(answers.done); }
        if (command === "what_a_paste_costs") { asked.push({ command, into: args.into }); return Promise.resolve(null); }
        if (command === "a_paste_is_made") {
          asked.push({ command, into: args.into, answer: args.answer });
          return Promise.resolve(args.answer ? answers.done : answers.asks);
        }
        asked.push({ command });
        return new Promise(() => {});
      },
    },
  },
};
await import("./files.js");
const host = anElement("div");
body.appendChild(host);
globalThis.theTab.itIsDrawn(host);
const settled = async () => { for (let at = 0; at < 8; at += 1) await new Promise((done) => setTimeout(done, 0)); };
const rows = () => host.querySelector("div.flist").children.filter((one) => one.classList.contains("frow") && !one.classList.contains("fhead"));
const plainly = (text) => (text == null ? text : text.replace(/ /g, " "));
const theControl = (label) => host.querySelectorAll("button").find((one) => plainly(one.getAttribute("aria-label") || "") === label);
const theBoxes = () => body.querySelectorAll("div.modal");
globalThis.theTab.itIsShowing("row");
await settled();
const out = {};
asked = [];
theControl("Cut").press("click");
await settled();
out.nothingChosen = { line: plainly(host.querySelector(".fnotyet").textContent), asked: asked.slice() };
rows().find((one) => one.querySelector(".fname").textContent === "scene.blend").press("click");
await settled();
asked = [];
theControl("Cut").press("click");
await settled();
out.cut = { asked: asked.slice() };
asked = [];
host.querySelectorAll("button.ib").find((one) => one.querySelector("span.pb")).press("click");
await settled();
out.pasted = { asked: asked.slice(), boxes: theBoxes().length };
asked = [];
theBoxes()[0].querySelectorAll("button").find((one) => one.textContent === "Replace").press("click");
await settled();
out.answered = { asked: asked.slice() };
console.log(JSON.stringify(out));
"#;

    // ── The Creator's notes piece 6: icons, a transfer count, one sentence fewer ──

    /// **Every solid file icon is filled with the theme's own ink**, never
    /// SVG's default black — `Design/File Browser § The list`: *"in the
    /// theme's own grey in light and dark alike"*, the colour `.fic` gives.
    /// **The page's own icon code runs under Node**, and every painted part's
    /// fill is read the way SVG reads it: its own `fill`, else the icon's,
    /// else black. Before the fix the folder, the `.blend` and the image's sun
    /// and hill read black. The Creator's note 11 on 0.1.7. **Skipped, and
    /// says so, where no Node is installed.**
    #[test]
    fn every_solid_file_icon_is_filled_with_the_themes_ink() {
        let page = the_page_code();
        let from = page.find("function anIcon(paths, options) {").expect("the page draws icons");
        let to = from + page[from..].find("const THE_TOOL_ICONS = {").expect("the file icons end");
        let probe = format!(
            r#"const SVG_NS = "http://www.w3.org/2000/svg";
const anElement = (tag) => {{ const attributes = {{}}; const el = {{ tag, attributes, children: [], setAttribute(n, v) {{ attributes[n] = String(v); }}, appendChild(c) {{ el.children.push(c); return c; }} }}; return el; }};
globalThis.document = {{ createElementNS: (_, tag) => anElement(tag) }};
{}
for (const [kind, make] of Object.entries(THE_FILE_ICONS)) {{
  const svg = make();
  console.log(kind + ": " + svg.children.map((one) => one.attributes.fill || svg.attributes.fill || "black").join(", "));
}}
"#,
            &page[from..to]
        );
        let folder = std::env::temp_dir().join(format!("smoozy-files-icons-{}", std::process::id()));
        std::fs::create_dir_all(&folder).expect("a scratch folder");
        std::fs::write(folder.join("probe.mjs"), probe).expect("the probe is written");
        let ran = std::process::Command::new("node").arg(folder.join("probe.mjs")).output();
        let _ = std::fs::remove_dir_all(&folder);
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the icons were not drawn");
            return;
        };
        assert!(ran.status.success(), "the icons did not draw: {}", String::from_utf8_lossy(&ran.stderr));
        let said = String::from_utf8_lossy(&ran.stdout).to_string();
        assert_eq!(said.lines().count(), 5, "{said}");
        assert!(!said.contains("black"), "a part of an icon is drawn in SVG's default black:\n{said}");
        assert!(said.lines().any(|line| line == "folder: currentColor"), "{said}");
    }

    /// **An upload under way says nothing above the list**, and its ending
    /// still does — `Design/File Browser § The list`: the bottom bar's count
    /// says it is running. The answer carries no sentence, and the page takes
    /// an older line away rather than leaving it to read as this one's. The
    /// Creator's note 10 on 0.1.7.
    #[test]
    fn an_upload_under_way_says_nothing_above_the_list() {
        let goes = serde_json::to_value(TheUploadGoes::Underway { request: Verbatim::new("r1".to_string()) })
            .expect("serialises");
        assert_eq!(goes, serde_json::json!({ "is": "underway", "request": "r1" }));
        let page = the_page_code();
        assert!(page.contains(r#"if (goes.is === "underway") theLineIsTakenAway();"#));
        assert!(page.contains(r#"if (goes.is === "did-not-start") theLineSays(goes.said, "upload");"#));
    }

    /// **The bottom bar's count: *1 transfer*, *2 transfers*, nothing at
    /// none** — `Design/File Browser § The list`.
    #[test]
    fn the_transfer_count_says_how_many_and_nothing_at_none() {
        assert_eq!(the_transfer_count(0), None);
        // **The figure is a value and the word is prose**, as every count on
        // this screen is; the page draws them as one line.
        let one = the_transfer_count(1).expect("one");
        assert_eq!(one.values(), vec!["1"]);
        assert_eq!(one.said().replace('\u{a0}', " ").trim(), "transfer");
        let three = the_transfer_count(3).expect("three");
        assert_eq!(three.values(), vec!["3"]);
        assert_eq!(three.said().replace('\u{a0}', " ").trim(), "transfers");
    }

    /// **The count on the page** — `web/files.js` under Node over the
    /// application's own answers: the bar shows *2 transfers* as a link once
    /// asked, the link goes when a transfer's move leaves none, comes back
    /// reading *1 transfer*, and a press on it goes to Transfers. The Creator's
    /// note 10 on 0.1.7. **Skipped, and says so, where no Node is installed.**
    #[test]
    fn the_bar_counts_this_servers_transfers_as_a_link_to_transfers() {
        let here: &[u8] = b"/home/ubuntu";
        let mut reading = the_tab("row", here, None, a_folder_being_read(here), None, None, false);
        reading.a_reading_follows = true;
        let entries = vec![an_entry("wood.png", Kind::File, 1000)];
        let rows = the_tab_after_the_reading(
            TheOrder::default(),
            "row",
            here,
            None,
            WhatTheReadingFound::TheFolder { entries, free: Some(1 << 30) },
            None,
            false,
        );
        let answers = serde_json::json!({
            "reading": reading,
            "rows": rows,
            "two": the_transfer_count(2),
            "one": the_transfer_count(1),
        });
        let prefix = THE_LISTS_STAND_INS
            .iter()
            .find(|(name, _)| *name == "harness.js")
            .map(|(_, text)| *text)
            .expect("the list harness");
        let prefix = prefix
            .split("const answers = JSON.parse")
            .next()
            .expect("the document half")
            .replace(
                "addEventListener(kind, handler) {",
                "press(kind) { for (const one of listeners.get(kind) || []) one({ preventDefault() {}, stopPropagation() {} }); },\n    addEventListener(kind, handler) {",
            );
        let harness = format!(
            r#"{prefix}
const answers = JSON.parse(readFileSync("answers.json", "utf8"));
let count = answers.two;
globalThis.window = {{
  innerWidth: 1000, innerHeight: 800,
  __TAURI__: {{ core: {{ invoke(command) {{
    if (command === "the_files_tab") return Promise.resolve(answers.reading);
    if (command === "the_folder_is_read") return Promise.resolve(answers.rows);
    if (command === "the_transfers_on_this_server") return Promise.resolve(count);
    return new Promise(() => {{}});
  }} }} }},
}};
await import("./files.js");
const host = anElement("div");
body.appendChild(host);
globalThis.theTab.itIsDrawn(host);
const settled = async () => {{ for (let at = 0; at < 6; at += 1) await new Promise((done) => setTimeout(done, 0)); }};
const link = () => host.querySelector("button.ftrans");
const said = () => (link() ? link().textContent.replace(/ /g, " ") : "nothing");
const moved = async () => {{ for (const handler of (globalThis.told["transfers:moved"] || [])) handler({{}}); await settled(); }};
globalThis.theTab.itIsShowing("row");
await settled();
console.log("shown: " + said());
count = null;
await moved();
console.log("none: " + said());
count = answers.one;
await moved();
console.log("one: " + said());
link().press("click");
console.log("pressed: " + (globalThis.shown || 0));
"#
        );
        let folder = std::env::temp_dir().join(format!("smoozy-files-count-harness-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a scratch folder");
        let web = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        for page in ["files.js", "the-value-drawn.js"] {
            std::fs::copy(web.join(page), folder.join(page)).expect("the page is copied");
        }
        for (name, text) in THE_LISTS_STAND_INS {
            std::fs::write(folder.join(name), text).expect("a stand-in is written");
        }
        std::fs::write(
            folder.join("event-route.js"),
            "export function whenTold(kind, handler) { globalThis.told ||= {}; (globalThis.told[kind] ||= []).push(handler); }",
        )
        .expect("written");
        std::fs::write(
            folder.join("transfers.js"),
            "export function aTransferIsAskedFor() {}\nexport function theTransfersAreShown() { globalThis.shown = (globalThis.shown || 0) + 1; }",
        )
        .expect("written");
        std::fs::write(folder.join("harness.js"), harness).expect("the harness is written");
        std::fs::write(folder.join("answers.json"), answers.to_string()).expect("the answers are written");
        let ran = std::process::Command::new("node").arg(folder.join("harness.js")).current_dir(&folder).output();
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the Files page was not run");
            let _ = std::fs::remove_dir_all(&folder);
            return;
        };
        let _ = std::fs::remove_dir_all(&folder);
        assert!(ran.status.success(), "the page did not run: {}", String::from_utf8_lossy(&ran.stderr));
        assert_eq!(
            String::from_utf8_lossy(&ran.stdout).trim(),
            "shown: 2 transfers\nnone: nothing\none: 1 transfer\npressed: 1"
        );
    }

    /// **What a paste after a copy says** — a copy offers Replace, Keep both
    /// or Cancel (`File Browser` M8); where the server cannot copy on itself,
    /// Design's own sentence (`Design/File Browser § What the screen says
    /// around an act`). Step 10c's Files' owed acts piece 6.
    #[test]
    fn a_copy_asks_with_keep_both_and_says_when_the_server_cannot_copy() {
        use crate::managing_files::WhatTheCopyCameTo as Copy;
        use smoozy_file_browser::already_exists::{AComparison, Comparison};
        let entries = [b"/home/ubuntu/wood.png".to_vec()];
        let says = |came: Copy| what_the_copy_says(&came, &entries, b"/home/ubuntu", &[], &[]);
        let AnActsAnswer::Asks { question } = says(Copy::AsksFirst(vec![AComparison {
            landing_path: b"/home/ubuntu/wood.png".to_vec(),
            what: Comparison::CollidesWithAFile,
        }])) else {
            panic!("a collision did not ask first");
        };
        let answers: Vec<&str> = question.answers.iter().map(|one| one.answer.as_str()).collect();
        assert_eq!(answers, vec!["replace", "keep-both", "cancel"]);
        let AnActsAnswer::Said { said } =
            says(Copy::TheServerCannotCopy(crate::managing_files::WhyTheServerCannotCopy::SmoozyIsNotSetUpThere))
        else {
            panic!("a server that cannot copy was not said");
        };
        assert_eq!(said.said().replace('\u{a0}', " "), words::SET_UP_TO_COPY);
        // **Design's *Update*** where Smoozy there is older than the copy
        // request (step 10c's Files' owed acts piece 7).
        let AnActsAnswer::Said { said } =
            says(Copy::TheServerCannotCopy(crate::managing_files::WhyTheServerCannotCopy::SmoozyThereIsOlder))
        else {
            panic!("an older Smoozy was not said");
        };
        assert_eq!(said.said().replace('\u{a0}', " "), words::UPDATE_TO_COPY);
        let AnActsAnswer::Done { said: Some(said) } = says(Copy::Ran {
            record: Identity::issue(IdentityKind::Operation).expect("randomness"),
            done: 1,
            refused: 0,
            not_done: 0,
            never_learned: 0,
        }) else {
            panic!("a whole copy said nothing");
        };
        assert_eq!(said.values(), vec!["wood.png"]);
    }

    /// **Smoozy's own program is asked to copy only on a server it was placed
    /// on and this copy confirmed** — the condition Paste is offered under
    /// (`File Browser` M15). A recorded root with no confirmed installation is
    /// not set up. Step 10c's Files' owed acts piece 7.
    #[test]
    fn the_program_is_asked_to_copy_only_where_the_installation_is_confirmed() {
        let server = Identity::issue(IdentityKind::LocalServer).expect("randomness");
        let installation = Identity::issue(IdentityKind::Installation).expect("randomness");
        let placed = a_placed_row(&server, &installation);
        assert_eq!(
            the_package_area_to_copy_with(&placed),
            crate::setting_up::the_package_area_of(&placed),
            "a placed row copies with its package area"
        );
        assert!(the_package_area_to_copy_with(&placed).is_some());
        let mut unconfirmed = placed.clone();
        unconfirmed.last_confirmed_installation_identity = None;
        assert_eq!(the_package_area_to_copy_with(&unconfirmed), None);
        assert_eq!(the_package_area_to_copy_with(&a_server_row("never")), None);
    }

    /// **`File Browser` B3's line is owed where the act's answer was learned,
    /// and nowhere else.**
    #[test]
    fn a_line_is_owed_where_the_acts_answer_was_learned() {
        use crate::managing_files::{the_lines_counts, WhatTheActCameTo as Came};
        let counts = |came: Came| the_lines_counts(&came).map(|c| (c.done, c.refused, c.not_attempted));
        assert_eq!(counts(Came::Done), Some((1, 0, 0)));
        assert_eq!(counts(Came::TheNameIsTaken), Some((0, 1, 0)));
        assert_eq!(counts(Came::TheLoginMayNot), Some((0, 1, 0)));
        assert_eq!(counts(Came::TheTellingWouldNotWrite), Some((0, 0, 1)));
        assert_eq!(
            counts(Came::TheNameIsRefused(smoozy_file_browser::managing::WhyTheNameIsRefused::Empty)),
            None
        );
        assert_eq!(counts(Came::TheConnectionWent), None);
    }

    // ── The 38, piece 13: the headers sort (hole 8) ─────────────────────────

    /// **A folder of every kind the four columns tell apart**, out of order.
    fn a_folder_to_sort() -> Vec<ListedEntry> {
        let at = |name: &[u8], kind: Kind, size: u64, changed: Option<i64>| ListedEntry {
            name: EntryName::from_the_machine(name.to_vec()),
            size: ApparentSize::of(size),
            last_changed: changed.map(LastChanged::at_epoch_seconds),
            kind,
        };
        vec![
            at(b"scene.blend", Kind::File, 300, Some(30)),
            at(b"renders", Kind::Folder, 4096, Some(50)),
            at(b"Beauty.EXR", Kind::File, 900, Some(10)),
            at(b"\xff\xfe.png", Kind::File, 5, Some(70)),
            at(b"Assets", Kind::Folder, 4096, Some(20)),
            at(b"notes", Kind::File, 100, None),
            at(b"alpha.png", Kind::File, 700, Some(60)),
        ]
    }

    fn the_names(browser: &TheBrowser) -> Vec<String> {
        the_listed_rows(browser).iter().map(|row| row.name.shown.as_str().to_string()).collect()
    }

    /// **Hole 8: folders first**, whichever header the list is sorted by.
    #[test]
    fn folders_come_first() {
        let tab = the_tab_after_the_reading(
            TheOrder::default(),
            "row",
            b"/home/ubuntu",
            None,
            WhatTheReadingFound::TheFolder { entries: a_folder_to_sort(), free: None },
            Some(ObservedValue::Connected),
            false,
        );
        let names = the_names(&tab);
        assert_eq!(&names[..2], &["Assets".to_string(), "renders".to_string()], "{names:?}");
    }

    /// **Each header sorts, and a second press on it reverses** — folders
    /// first both ways, every column falling back on the name, and a name that
    /// cannot be shown as text after those that can (hole 8).
    #[test]
    fn each_header_sorts_and_a_second_press_reverses() {
        let sorted = |by: TheColumn, reversed: bool| {
            the_names(&the_tab_after_the_reading(
                TheOrder { by, reversed },
                "row",
                b"/home/ubuntu",
                None,
                WhatTheReadingFound::TheFolder { entries: a_folder_to_sort(), free: None },
                Some(ObservedValue::Connected),
                false,
            ))
        };
        let odd = "\u{FFFD}\u{FFFD}.png";
        let wanted: [(TheColumn, [&str; 7]); 4] = [
            (TheColumn::Name, ["Assets", "renders", "alpha.png", "Beauty.EXR", "notes", "scene.blend", odd]),
            // The folders show no size and keep their order by name.
            (TheColumn::Size, ["Assets", "renders", odd, "notes", "scene.blend", "alpha.png", "Beauty.EXR"]),
            // A file the machine gave no time for comes after those it did.
            (TheColumn::Changed, ["Assets", "renders", "Beauty.EXR", "scene.blend", "alpha.png", odd, "notes"]),
            // By extension, ignoring case; none first; a tie by name.
            (TheColumn::Kind, ["Assets", "renders", "notes", "scene.blend", "Beauty.EXR", "alpha.png", odd]),
        ];
        for (by, names) in wanted {
            assert_eq!(sorted(by, false), names, "sorted by {by:?}");
            let mut folders = names[..2].to_vec();
            let mut files = names[2..].to_vec();
            folders.reverse();
            files.reverse();
            folders.extend(files);
            assert_eq!(sorted(by, true), folders, "sorted by {by:?}, reversed: folders still first");
        }
        // **The headers say which one is on**, for the page to mark.
        let tab = the_tab_after_the_reading(
            TheOrder { by: TheColumn::Changed, reversed: true },
            "row",
            b"/home/ubuntu",
            None,
            WhatTheReadingFound::TheFolder { entries: a_folder_to_sort(), free: None },
            Some(ObservedValue::Connected),
            false,
        );
        assert_eq!((tab.headers.sorted_by, tab.headers.reversed), (TheColumn::Changed, true));
        let json = serde_json::to_value(&tab.headers).expect("the headers serialise");
        assert_eq!(json["sorted_by"], "changed");
    }

    /// **Names compared as shown, ignoring case — beyond the plain alphabet.**
    /// Comparing the bytes with only A to Z lowered put *Éz* before *éa*.
    #[test]
    fn names_are_compared_as_shown_ignoring_case() {
        let entries = vec![
            an_entry("Éz.png", Kind::File, 1),
            an_entry("éa.png", Kind::File, 1),
            an_entry("B.png", Kind::File, 1),
            an_entry("b.png", Kind::File, 1),
            an_entry("a.png", Kind::File, 1),
        ];
        let tab = the_tab_after_the_reading(
            TheOrder::default(),
            "row",
            b"/home/ubuntu",
            None,
            WhatTheReadingFound::TheFolder { entries, free: None },
            Some(ObservedValue::Connected),
            false,
        );
        assert_eq!(the_names(&tab), ["a.png", "B.png", "b.png", "éa.png", "Éz.png"]);
    }

    /// **The choice is held for the run and not stored** (hole 8): a press
    /// sorts every folder opened after it, a second press reverses, another
    /// header starts from the top of its own order, and **the command touches
    /// nothing but the choice** — no record, no reading. The only test that
    /// presses, so the held choice is this test's alone.
    #[test]
    fn the_choice_holds_for_the_run_while_other_folders_open() {
        assert_eq!(TheOrder::default(), TheOrder { by: TheColumn::Name, reversed: false });
        the_list_is_sorted_by(TheColumn::Size);
        assert_eq!(the_order_held(), TheOrder { by: TheColumn::Size, reversed: false });
        for folder in [b"/home/ubuntu".as_slice(), b"/home/ubuntu/renders", b"/srv"] {
            let tab = the_tab_after_the_reading(
                the_order_held(),
                "row",
                folder,
                None,
                WhatTheReadingFound::TheFolder { entries: a_folder_to_sort(), free: None },
                Some(ObservedValue::Connected),
                false,
            );
            assert_eq!(the_names(&tab)[2], "\u{FFFD}\u{FFFD}.png", "the smallest file leads in {folder:?}");
            assert_eq!(tab.headers.sorted_by, TheColumn::Size);
        }
        the_list_is_sorted_by(TheColumn::Size);
        assert_eq!(the_order_held(), TheOrder { by: TheColumn::Size, reversed: true });
        the_list_is_sorted_by(TheColumn::Kind);
        assert_eq!(the_order_held(), TheOrder { by: TheColumn::Kind, reversed: false });
        the_list_is_sorted_by(TheColumn::Name);
        assert_eq!(the_order_held(), TheOrder::default());
        // **Not stored**: the command's body reads and writes nothing else.
        let body = the_working_half()
            .split("pub fn the_list_is_sorted_by(")
            .nth(1)
            .and_then(|rest| rest.split("\n}\n").next())
            .expect("the command is there");
        assert!(!body.contains("record") && !body.contains("Started") && !body.contains("AppHandle"), "{body}");
        // **And both commands that compose the browser read the held choice.**
        for command in ["pub async fn the_files_tab<", "pub async fn the_folder_is_read<"] {
            let body = the_working_half().split(command).nth(1).and_then(|rest| rest.split("\n}\n").next());
            assert!(body.is_some_and(|body| body.contains("the_order_held()")), "{command} reads no order");
        }
    }

    /// **The Creator's twenty-first notes, note 2: the rows held, put in order,
    /// stand exactly as a fresh reading in that order would** — every column,
    /// both ways — and the headers mark the order handed. Composed from the
    /// rows of a reading sorted by name, so nothing is read again.
    #[test]
    fn twenty_first_the_rows_held_are_put_in_the_order_a_reading_would_give() {
        let reading = |order: TheOrder| {
            the_tab_after_the_reading(
                order,
                "row",
                b"/home/ubuntu",
                None,
                WhatTheReadingFound::TheFolder { entries: a_folder_to_sort(), free: None },
                Some(ObservedValue::Connected),
                false,
            )
        };
        let held = reading(TheOrder::default());
        let rows = the_listed_rows(&held);
        let places: Vec<WhatTheRowSortsBy> = rows.iter().map(|row| row.sorts_by.clone()).collect();
        for by in [TheColumn::Name, TheColumn::Kind, TheColumn::Size, TheColumn::Changed] {
            for reversed in [false, true] {
                let order = TheOrder { by, reversed };
                let put = the_rows_in(order, &places);
                let names: Vec<String> =
                    put.order.iter().map(|at| rows[*at].name.shown.as_str().to_string()).collect();
                assert_eq!(names, the_names(&reading(order)), "{order:?}");
                assert_eq!((put.headers.sorted_by, put.headers.reversed), (by, reversed));
            }
        }
        // **A row carries what it sorts by as the reading composed it**, and
        // it travels to the page and back whole.
        let json = serde_json::to_value(&rows[0].sorts_by).expect("it serialises");
        let back: WhatTheRowSortsBy = serde_json::from_value(json).expect("it comes back");
        assert_eq!(back, rows[0].sorts_by);
        assert!(the_rows_in(TheOrder::default(), &[]).order.is_empty());
        // **Registered**, so the page can reach it.
        assert!(include_str!("mod.rs").contains("files::the_rows_are_put_in_order,"));
    }

    /// **The page half: a header press asks for that order and has the rows
    /// it holds put in it, and never asks for the folder again** — the
    /// Creator's twenty-first notes, note 2. The header the answer names is
    /// the one marked, the rows stand in the order a reading would give, and
    /// the chosen row stays chosen. Run under Node over the application's own
    /// answers, on the box test's document. **Skipped, and says so, where no
    /// Node is installed.**
    ///
    /// *(Until 2026-10-29 a press asked for* `the_files_tab` *and*
    /// `the_folder_is_read` *after the order, and this test pinned it.)*
    #[test]
    fn a_header_press_reorders_the_rows_held_and_never_asks_for_the_folder() {
        let here: &[u8] = b"/home/ubuntu";
        let mut reading = the_tab("row", here, None, a_folder_being_read(here), None, None, false);
        reading.a_reading_follows = true;
        let rows = the_tab_after_the_reading(
            TheOrder::default(),
            "row",
            here,
            None,
            WhatTheReadingFound::TheFolder { entries: a_folder_to_sort(), free: None },
            Some(ObservedValue::Connected),
            false,
        );
        let by_size = the_tab_after_the_reading(
            TheOrder { by: TheColumn::Size, reversed: false },
            "row",
            here,
            None,
            WhatTheReadingFound::TheFolder { entries: a_folder_to_sort(), free: None },
            Some(ObservedValue::Connected),
            false,
        );
        let held: Vec<WhatTheRowSortsBy> = the_listed_rows(&rows).iter().map(|row| row.sorts_by.clone()).collect();
        let put_in_order = the_rows_in(TheOrder { by: TheColumn::Size, reversed: false }, &held);
        let answers = serde_json::json!({ "reading": reading, "rows": rows, "putInOrder": put_in_order });

        let folder = std::env::temp_dir().join(format!("smoozy-sorting-page-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a scratch folder");
        let web = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        for page in ["files.js", "the-value-drawn.js"] {
            std::fs::copy(web.join(page), folder.join(page)).expect("the page is copied");
        }
        for (name, text) in THE_LISTS_STAND_INS {
            if name != "harness.js" {
                std::fs::write(folder.join(name), text).expect("a stand-in is written");
            }
        }
        // **The box test's document and answers, up to the page's import**,
        // then this test's own presses.
        let document = THE_HIDDEN_FILES_HARNESS
            .split("await import(\"./files.js\");")
            .next()
            .expect("the box harness imports the page");
        std::fs::write(folder.join("harness.js"), format!("{document}{THE_SORTING_PRESSES}"))
            .expect("the harness is written");
        std::fs::write(folder.join("answers.json"), answers.to_string()).expect("the answers are written");
        let ran = std::process::Command::new("node").arg(folder.join("harness.js")).current_dir(&folder).output();
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the Files page was not run");
            let _ = std::fs::remove_dir_all(&folder);
            return;
        };
        let _ = std::fs::remove_dir_all(&folder);
        assert!(ran.status.success(), "the page did not run: {}", String::from_utf8_lossy(&ran.stderr));
        let got: serde_json::Value =
            serde_json::from_str(String::from_utf8_lossy(&ran.stdout).trim()).expect("the harness prints one answer");
        let wanted = serde_json::json!({
            "before": {
                "headers": [["Name ▲", true], ["Kind", false], ["Size", false], ["Changed", false]],
                "names": the_names(&rows),
            },
            "afterSize": {
                "asked": [
                    { "command": "the_list_is_sorted_by", "by": "size" },
                    { "command": "the_rows_are_put_in_order", "rows": serde_json::to_value(&held).expect("the rows") },
                ],
                "headers": [["Name", false], ["Kind", false], ["Size ▲", true], ["Changed", false]],
                "names": the_names(&by_size),
                "chosen": ["notes"],
            },
        });
        assert_eq!(got, wanted);
    }

    /// **The presses**, appended to the box harness's document.
    const THE_SORTING_PRESSES: &str = r#"
const theAppsOwnAnswer = window.__TAURI__.core.invoke;
window.__TAURI__.core.invoke = (command, args) => {
  if (command === "the_list_is_sorted_by") {
    asked.push({ command, by: args.by });
    return Promise.resolve(null);
  }
  if (command === "the_rows_are_put_in_order") {
    asked.push({ command, rows: args.rows });
    return Promise.resolve(answers.putInOrder);
  }
  return theAppsOwnAnswer(command, args);
};
await import("./files.js");
const host = anElement("div");
body.appendChild(host);
globalThis.theTab.itIsDrawn(host);
const settled = async () => { for (let at = 0; at < 4; at += 1) await new Promise((done) => setTimeout(done, 0)); };
const headers = () => host.querySelector("div.fhead").children.filter((one) => one.tag === "button");
const drawn = () => headers().map((one) => [one.textContent, one.classList.contains("on")]);
const theRows = () => host.querySelector("div.flist").children.filter((one) => one.classList.contains("frow") && !one.classList.contains("fhead"));
const names = () => theRows().map((one) => one.querySelector(".fname").textContent);
const chosen = () => theRows().filter((one) => one.getAttribute("aria-current") === "true").map((one) => one.querySelector(".fname").textContent);
globalThis.theTab.itIsShowing("row");
await settled();
const out = { before: { headers: drawn(), names: names() } };
theRows().find((one) => one.querySelector(".fname").textContent === "notes").press("click");
await settled();
asked = [];
// **A reading asked for now would answer with nothing**, so a press that read
// the folder again could not draw the rows it is checked for.
plan = { tab: null, reading: null };
headers()[2].press("click");
await settled();
out.afterSize = { asked: asked.filter((one) => one.command !== "the_saved_with_version_is_read"), headers: drawn(), names: names(), chosen: chosen() };
console.log(JSON.stringify(out));
"#;

    /// **The 38's piece 7, item 31: the offer beside `File Browser` W1b's
    /// line hands the files over and opens the check that sends them** — read
    /// in the page's source, because the command needs a running application.
    /// Until this piece the offer answered *not built yet*. **And `File
    /// Browser` U13 in the upload's own answer**: a unit handed over keeps its
    /// disposition when the rest of the request goes.
    #[test]
    fn the_offer_beside_the_check_line_hands_the_files_over_and_opens_the_check() {
        let page = include_str!("../../web/files.js");
        let after = page
            .split("offer.addEventListener(\"click\"")
            .nth(1)
            .expect("the offer's handler");
        let handler = &after[..after.find("line.appendChild(offer)").expect("the handler ends")];
        assert!(handler.contains("\"the_files_the_check_placed_go_through_it\""), "{handler}");
        assert!(handler.contains("theUploadGoes(handed.goes)"), "{handler}");
        assert!(handler.contains("theCheckIsOpenedOn(over, handed.the_check_of)"), "{handler}");
        assert!(!page.contains("hand-over-to-the-check"), "the act that answered not built yet");
        assert!(include_str!("mod.rs").contains("files::the_files_the_check_placed_go_through_it,"));
        let answered = include_str!("../uploading.rs");
        let no_question = answered
            .split("let Some(answer) = answer.filter(|_| ready.something_collides()) else {")
            .nth(1)
            .expect("the answer with nothing colliding");
        let no_question = &no_question[..no_question.find("};").expect("its end")];
        assert!(no_question.contains("Disposition::HandedOver"), "{no_question}");
    }

}

#[cfg(test)]
mod selecting_text {
    use super::*;

    #[test]
    fn an_uploads_answer_marks_only_the_destination_path() {
        let came_to = WhatTheUploadCameTo::default();
        let answer = what_it_came_to(&came_to, b"/root/uploads");
        let paths: Vec<_> = answer.pieces.iter().filter(|part| part.is_path)
            .filter_map(|part| part.value.as_ref().map(|value| value.as_str())).collect();
        assert_eq!(paths, vec!["/root/uploads"]);
        assert!(answer.pieces.iter().any(|part| !part.is_path && part.value.as_ref().is_some_and(|value| value.as_str() == "0")));
    }
}

/// **A picture looked at** — `File Browser` L29 and `Design/File Browser §
/// Looking at a picture`, added 2026-10-31 (the Creator's seventeenth notes,
/// note 11). The row's offer and the read's words at the desk, and the read
/// itself over the real point, lending OpenSSH's own `sftp-server` the way
/// `src/the_found_files_sent.rs`' stand-in does.
#[cfg(test)]
mod a_picture_looked_at {
    use super::*;
    use smoozy_file_browser::picture::{ThePictureRead, WhyItWasNotRead};

    fn an_entry(name: &str, kind: Kind, size: u64) -> ListedEntry {
        ListedEntry {
            name: EntryName::from_the_machine(name.as_bytes().to_vec()),
            size: ApparentSize::of(size),
            last_changed: Some(LastChanged::at_epoch_seconds(1_754_483_000)),
            kind,
        }
    }

    #[test]
    fn a_png_or_jpeg_row_opens_the_viewer_and_the_others_say_why_not() {
        let folder = b"/home/ubuntu/renders".as_slice();
        let png = a_row(folder, &an_entry("kitchen_0002.png", Kind::File, 3_900_000), None);
        assert_eq!(png.picture, Some(APicture::ToBeRead));
        let jpeg = a_row(folder, &an_entry("still.JPG", Kind::File, 10), None);
        assert_eq!(jpeg.picture, Some(APicture::ToBeRead));
        // **Can't preview OpenEXR files.**, the format a value beside the
        // words, and nothing is read whatever its size.
        let exr = a_row(folder, &an_entry("studio_backplate.exr", Kind::File, 96_000_000), None);
        let Some(APicture::NotShown { said }) = exr.picture else { panic!("an OpenEXR says why") };
        assert_eq!(said.said().replace('\u{a0}', " "), "Can't preview files.");
        assert_eq!(said.values(), vec!["OpenEXR"]);
        // **A file over 100 MB is not read**, on its listed size.
        let big = a_row(folder, &an_entry("studio_matte_16bit.png", Kind::File, 214_000_000), None);
        assert_eq!(
            big.picture,
            Some(APicture::NotShown { said: Words::plain(words::TOO_LARGE_TO_SHOW) })
        );
        assert_eq!(
            a_row(folder, &an_entry("exactly.png", Kind::File, 100_000_000), None).picture,
            Some(APicture::ToBeRead),
            "the cap is a hundred million bytes, and a file at it is shown"
        );
        // **Not pictures**: a `.blend`, a video, and a folder with a picture's
        // name.
        assert_eq!(a_row(folder, &an_entry("scene.blend", Kind::File, 10), None).picture, None);
        assert_eq!(a_row(folder, &an_entry("shot.mp4", Kind::File, 10), None).picture, None);
        assert_eq!(a_row(folder, &an_entry("frames.png", Kind::Folder, 0), None).picture, None);
    }

    #[test]
    fn a_read_that_ends_some_other_way_is_said_in_designs_words() {
        assert_eq!(
            what_the_picture_read_came_to(ThePictureRead::TooLarge),
            Err(ThePictureIsNotShown::NotShown { said: Words::plain(words::TOO_LARGE_TO_SHOW) })
        );
        assert_eq!(
            what_the_picture_read_came_to(ThePictureRead::CannotBeShown),
            Err(ThePictureIsNotShown::NotShown { said: Words::plain(words::COULD_NOT_SHOW_THIS_FILE) })
        );
        let Err(ThePictureIsNotShown::NotRead { said, why, retry }) =
            what_the_picture_read_came_to(ThePictureRead::NotRead(WhyItWasNotRead::TheLineWent))
        else {
            panic!("a failed read says so");
        };
        assert_eq!(said.said().replace('\u{a0}', " "), "Could not read the file.");
        assert_eq!(why.said().replace('\u{a0}', " "), "Connection lost before the server replied.");
        assert_eq!(retry.as_str(), "Retry");
        let Err(ThePictureIsNotShown::NotRead { why, .. }) =
            what_the_picture_read_came_to(ThePictureRead::NotRead(WhyItWasNotRead::TheLoginMayNot))
        else {
            panic!("a refusal says so");
        };
        assert_eq!(why.said().replace('\u{a0}', " "), "Permission denied.");
        assert_eq!(what_the_picture_read_came_to(ThePictureRead::LetGo), Err(ThePictureIsNotShown::LetGo));
        assert_eq!(
            what_the_picture_read_came_to(ThePictureRead::TheBytes {
                bytes: vec![1, 2],
                are: smoozy_file_browser::picture::TheBytesAre::Png
            }),
            Ok(vec![1, 2])
        );
    }

    #[test]
    fn the_viewers_words_are_designs_and_ride_every_answer() {
        let browser = the_tab("row", b"/", None, a_folder_being_read(b"/"), Some(ObservedValue::Connected), None, false);
        assert_eq!(browser.viewer, the_viewers_words());
        assert_eq!(browser.viewer.loading.said().replace('\u{a0}', " "), "Loading…");
        assert_eq!(browser.viewer.not_connected.said().replace('\u{a0}', " "), "Not connected.");
        assert_eq!(browser.viewer.picture.as_str(), "Picture");
    }

    #[test]
    fn the_viewer_is_offered_by_the_shell() {
        let registered = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/surface/mod.rs"),
        )
        .expect("the registration is readable");
        assert!(registered.contains("files::a_picture_is_read,"));
        assert!(registered.contains("files::the_picture_is_let_go,"));
    }

    /// **The page half, run under Node over the application's own answers** —
    /// `Design/File Browser § Looking at a picture`. A double-click on a PNG
    /// opens it, reads it and shows its size and pixel size; › steps to the
    /// next picture **skipping a `.blend`**, and the list's choice follows; →
    /// reaches an OpenEXR, which says *Can't preview OpenEXR files.* and
    /// reads nothing; the next, a 214 MB PNG, says it is too large, reads
    /// nothing, and its › is grey; Escape closes and lets the read go; a read
    /// that fails says so with the reason and *Retry*; and **with the line down
    /// a double-click opens nothing and asks nothing**. Skipped, and says so,
    /// where no Node is installed.
    #[test]
    fn the_page_opens_steps_and_closes_the_viewer() {
        let here: &[u8] = b"/home/ubuntu/renders";
        let entries = vec![
            an_entry("frame_0001.png", Kind::File, 4_089_446),
            an_entry("frame_0002.blend", Kind::File, 1_000),
            an_entry("frame_0003.png", Kind::File, 4_089_446),
            an_entry("studio_backplate.exr", Kind::File, 100_663_296),
            an_entry("studio_matte_16bit.png", Kind::File, 224_395_264),
        ];
        let mut reading = the_tab("row", here, None, a_folder_being_read(here), None, None, false);
        reading.a_reading_follows = true;
        let rows = the_tab_after_the_reading(
            TheOrder::default(),
            "row",
            here,
            None,
            WhatTheReadingFound::TheFolder { entries, free: None },
            Some(ObservedValue::Connected),
            false,
        );
        let listed = match &rows.listing {
            TheListing::Rows { rows, .. } => rows.clone(),
            other => panic!("the folder listed: {other:?}"),
        };
        let stale = the_tab(
            "row",
            here,
            None,
            TheListing::Rows { rows: listed, taken_at: None, read_elevated: false },
            None,
            None,
            false,
        );
        assert!(stale.stale.is_some(), "the stale listing is drawn as the line being down");
        let failed = what_the_picture_read_came_to(ThePictureRead::NotRead(WhyItWasNotRead::ItIsNotThere))
            .expect_err("a failed read");
        let answers = serde_json::json!({ "reading": reading, "rows": rows, "stale": stale, "failed": failed });

        let folder = std::env::temp_dir().join(format!("smoozy-picture-viewer-page-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a scratch folder");
        let web = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        for page in ["files.js", "the-value-drawn.js"] {
            std::fs::copy(web.join(page), folder.join(page)).expect("the page is copied");
        }
        for (name, text) in THE_VIEWERS_STAND_INS {
            std::fs::write(folder.join(name), text).expect("a stand-in is written");
        }
        std::fs::write(folder.join("harness.js"), THE_VIEWERS_HARNESS).expect("the harness is written");
        std::fs::write(folder.join("answers.json"), answers.to_string()).expect("the answers are written");
        let ran = std::process::Command::new("node").arg(folder.join("harness.js")).current_dir(&folder).output();
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the Files page was not run");
            let _ = std::fs::remove_dir_all(&folder);
            return;
        };
        let _ = std::fs::remove_dir_all(&folder);
        assert!(ran.status.success(), "the page did not run: {}", String::from_utf8_lossy(&ran.stderr));
        let got: serde_json::Value =
            serde_json::from_str(String::from_utf8_lossy(&ran.stdout).trim()).expect("the harness prints one answer");
        let png = |name: &str| format!("{}/{name}", String::from_utf8_lossy(here)).into_bytes();
        let wanted = serde_json::json!({
            "names": ["frame_0001.png", "frame_0002.blend", "frame_0003.png", "studio_backplate.exr", "studio_matte_16bit.png"],
            "opened": {
                "open": true, "label": "Picture", "title": "frame_0001.png", "meta": "3.9 MB · 1920 × 1080",
                "picture": true, "say": null, "previous": true, "next": false, "chosen": "frame_0001.png",
                "asked": [{ "command": "a_picture_is_read", "path": png("frame_0001.png"), "listed": 4_089_446 }],
            },
            "stepped": {
                "title": "frame_0003.png", "meta": "3.9 MB · 1920 × 1080", "picture": true, "previous": false,
                "next": false, "chosen": "frame_0003.png",
                "asked": [{ "command": "a_picture_is_read", "path": png("frame_0003.png"), "listed": 4_089_446 }],
            },
            "exr": {
                "title": "studio_backplate.exr", "meta": "96 MB", "picture": false,
                "say": "Can't preview OpenEXR files.", "previous": false, "next": false,
                "chosen": "studio_backplate.exr", "asked": [{ "command": "the_picture_is_let_go" }],
            },
            "big": {
                "title": "studio_matte_16bit.png", "meta": "214 MB", "picture": false,
                "say": "Too large to show (over 100 MB).", "previous": false, "next": true,
                "chosen": "studio_matte_16bit.png", "asked": [{ "command": "the_picture_is_let_go" }],
            },
            "atTheEnd": { "title": "studio_matte_16bit.png", "asked": [] },
            "closed": { "open": false, "chosen": "studio_matte_16bit.png", "asked": [{ "command": "the_picture_is_let_go" }] },
            "failed": {
                "say": "Could not read the file. File not found. Retry",
                "retried": [{ "command": "a_picture_is_read", "path": png("frame_0001.png"), "listed": 4_089_446 }],
            },
            "downloaded": [["download", "row", png("frame_0001.png")]],
            "lineDown": { "open": false, "asked": [] },
        });
        assert_eq!(got, wanted);
    }

    const THE_VIEWERS_STAND_INS: [(&str, &str); 7] = [
        ("package.json", r#"{ "type": "module" }"#),
        ("event-route.js", r#"export function whenTold() {}"#),
        (
            "a-server.js",
            r#"export const FILES = "files";
export function aTabIsRegistered(tab) { globalThis.theTab = tab; }
export function aPanelIsOpenOver() { return false; }"#,
        ),
        ("the-dependency-check.js", r#"export function theCheckIsOpenedOn() {}"#),
        (
            "transfers.js",
            r#"globalThis.downloads = [];
export function aTransferIsAskedFor(act, whose, picked) { globalThis.downloads.push([act, whose, ...picked]); }
export function theTransfersAreShown() {}"#,
        ),
        ("render-settings.js", r#"export async function theSettingsAreOpenedOn() {}"#),
        ("the-underway-shape.js", r#"export function theMarkOverAPane() { return document.createElement("div"); }"#),
    ];

    /// **The harness for the viewer** — the hidden-files harness's document,
    /// with a web view's own decode stood in: every picture decodes at
    /// 1920 × 1080.
    const THE_VIEWERS_HARNESS: &str = r#"import { readFileSync } from "node:fs";
function aTest(selector) {
  const found = /^([a-z0-9]*)((?:\.[\w-]+)*)$/.exec(selector.trim());
  if (!found) return () => false;
  const [, tag, classes] = found;
  const names = classes ? classes.split(".").filter(Boolean) : [];
  return (element) => {
    if (!element || !element.classList) return false;
    if (tag && element.tag !== tag) return false;
    return names.every((name) => element.classList.contains(name));
  };
}
function everyUnder(element, test, into) {
  for (const child of element.children || []) {
    if (test(child)) into.push(child);
    everyUnder(child, test, into);
  }
  return into;
}
function anElement(tag) {
  const names = new Set();
  const listeners = new Map();
  const attributes = new Map();
  const element = {
    tag, children: [], parentNode: null, dataset: {}, style: { setProperty() {} }, hidden: false, disabled: false, text: "", type: "", id: "", title: "", scrollTop: 0,
    classList: {
      add: (...more) => more.forEach((one) => names.add(one)),
      remove: (...less) => less.forEach((one) => names.delete(one)),
      toggle: (one, on) => { if (on === undefined ? !names.has(one) : on) names.add(one); else names.delete(one); },
      contains: (one) => names.has(one),
    },
    set className(value) { names.clear(); String(value).split(" ").filter(Boolean).forEach((one) => names.add(one)); },
    get className() { return Array.from(names).join(" "); },
    set textContent(value) { for (const one of element.children) one.parentNode = null; element.children = []; element.text = value == null ? "" : String(value); },
    get textContent() { return element.text + element.children.map((one) => one.textContent || one.text || "").join(""); },
    get offsetTop() { return 0; },
    get offsetHeight() { return 30; },
    get clientHeight() { return 300; },
    appendChild(child) {
      if (child.parentNode && child.parentNode.removeChild) child.parentNode.removeChild(child);
      child.parentNode = element;
      element.children.push(child);
      return child;
    },
    insertBefore(child, before) {
      if (child.parentNode && child.parentNode.removeChild) child.parentNode.removeChild(child);
      child.parentNode = element;
      const at = element.children.indexOf(before);
      if (at < 0) element.children.push(child); else element.children.splice(at, 0, child);
      return child;
    },
    removeChild(child) { element.children = element.children.filter((one) => one !== child); child.parentNode = null; return child; },
    replaceChildren(...fresh) { for (const one of element.children) one.parentNode = null; element.children = []; fresh.forEach((one) => element.appendChild(one)); },
    remove() { if (element.parentNode) element.parentNode.removeChild(element); },
    setAttribute(name, value) { attributes.set(name, String(value)); },
    getAttribute(name) { return attributes.has(name) ? attributes.get(name) : null; },
    addEventListener(kind, handler) { const held = listeners.get(kind) || []; held.push(handler); listeners.set(kind, held); },
    fire(kind, carrying) { for (const handler of listeners.get(kind) || []) handler({ preventDefault() {}, stopPropagation() {}, target: element, ...carrying }); },
    focus() {},
    select() {},
    contains(other) { let at = other; while (at) { if (at === element) return true; at = at.parentNode; } return false; },
    closest() { return null; },
    querySelector(selector) { return everyUnder(element, aTest(selector), [])[0] || null; },
    querySelectorAll(selector) { return everyUnder(element, aTest(selector), []); },
    getBoundingClientRect() { return { left: 0, top: 0, bottom: 0, width: 0, height: 0 }; },
  };
  return element;
}
const body = anElement("body");
const keys = [];
globalThis.document = {
  createElement: anElement,
  createElementNS: (_, tag) => anElement(tag),
  createTextNode: (text) => ({ text, children: [] }),
  addEventListener(kind, handler) { if (kind === "keydown") keys.push(handler); },
  removeEventListener() {},
  body,
};
globalThis.Image = class {
  set src(address) { this.address = address; setTimeout(() => { this.naturalWidth = 1920; this.naturalHeight = 1080; this.onload && this.onload(); }, 0); }
};
const answers = JSON.parse(readFileSync("answers.json", "utf8"));
let plan = { tab: answers.rows, reading: answers.rows, fails: false };
let asked = [];
const A_PNG = new Uint8Array([0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, 1, 2, 3]).buffer;
globalThis.window = {
  innerWidth: 1000, innerHeight: 800,
  __TAURI__: {
    core: {
      invoke(command, args) {
        if (command === "a_picture_is_read") {
          asked.push({ command, path: args.path, listed: args.listed });
          return plan.fails ? Promise.reject(answers.failed) : Promise.resolve(A_PNG);
        }
        if (command === "the_picture_is_let_go") { asked.push({ command }); return Promise.resolve(); }
        if (command === "the_files_tab") return Promise.resolve(plan.tab);
        if (command === "the_folder_is_read") return Promise.resolve(plan.reading);
        if (command === "the_saved_with_version_is_read") return Promise.resolve(null);
        return new Promise(() => {});
      },
    },
  },
};

await import("./files.js");
const host = anElement("div");
body.appendChild(host);
globalThis.theTab.itIsDrawn(host);
const settled = async () => { for (let at = 0; at < 6; at += 1) await new Promise((done) => setTimeout(done, 0)); };
const plainly = (text) => (text == null ? text : text.replace(/ /g, " "));
const rows = () => host.querySelector("div.flist").children.filter((one) => one.classList.contains("frow") && !one.classList.contains("fhead"));
const nameOf = (row) => row.querySelector(".fname").textContent;
const theRow = (name) => rows().find((one) => nameOf(one) === name);
const chosenName = () => { const one = rows().find((row) => row.getAttribute("aria-current") === "true"); return one ? nameOf(one) : null; };
const viewer = () => host.querySelector("div.fbview");
const theButtons = () => viewer().querySelector("div.fbvstep").querySelectorAll("button");
const what = () => {
  const box = viewer();
  const say = box.querySelector("p.fbvsay");
  const picture = box.querySelector("div.fbvpic").children[0];
  const [previous, next] = theButtons();
  return {
    title: box.querySelector("h3").textContent,
    meta: plainly(box.querySelector("span.fbvmeta").textContent),
    picture: !picture.hidden && Boolean(picture.querySelector("svg")),
    say: say.hidden ? null : plainly(say.textContent),
    previous: previous.disabled,
    next: next.disabled,
    chosen: chosenName(),
    asked,
  };
};
const press = (key) => { for (const handler of keys) handler({ key, preventDefault() {}, stopPropagation() {}, defaultPrevented: false, target: body }); };

globalThis.theTab.itIsShowing("row");
await settled();
const out = { names: rows().map(nameOf) };
asked = [];
theRow("frame_0001.png").fire("dblclick", {});
await settled();
out.opened = { open: Boolean(viewer()), label: viewer().getAttribute("aria-label"), ...what() };
const { previous: _p, ...opened } = out.opened;
out.opened = { ...opened, previous: out.opened.previous };
asked = [];
theButtons()[1].fire("click", {});
await settled();
const { say: _s, ...stepped } = what();
out.stepped = stepped;
asked = [];
press("ArrowRight");
await settled();
out.exr = what();
asked = [];
theButtons()[1].fire("click", {});
await settled();
out.big = what();
asked = [];
press("ArrowRight");
await settled();
out.atTheEnd = { title: what().title, asked };
asked = [];
press("Escape");
await settled();
out.closed = { open: Boolean(viewer()), chosen: chosenName(), asked };
asked = [];
plan.fails = true;
theRow("frame_0001.png").fire("dblclick", {});
await settled();
const failing = plainly(viewer().querySelector("p.fbvsay").children.map((one) => one.textContent || one.text || "").join(" "));
asked = [];
plan.fails = false;
const retry = viewer().querySelector("p.fbvsay").querySelector("button.mini");
retry.fire("click", {});
await settled();
out.failed = { say: failing.replace(/\s+/g, " ").trim(), retried: asked.filter((one) => one.command === "a_picture_is_read") };
viewer().querySelector("div.pvtitle").querySelector("button.mini").fire("click", {});
out.downloaded = globalThis.downloads;
press("Escape");
await settled();
plan = { tab: answers.stale, reading: answers.stale, fails: false };
globalThis.theTab.itIsShowing("row");
await settled();
asked = [];
theRow("frame_0001.png").fire("dblclick", {});
await settled();
out.lineDown = { open: Boolean(viewer()), asked: asked.filter((one) => one.command === "a_picture_is_read" || one.command === "the_picture_is_let_go") };
console.log(JSON.stringify(out));
"#;

    #[cfg(unix)]
    mod over_a_real_point {
        use std::os::fd::OwnedFd;
        use std::path::PathBuf;
        use std::process::{Command, Stdio};
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::sync::Arc;

        use smoozy_file_browser::picture::{ThePictureRead, TheBytesAre};
        use smoozy_server_protocol::transport::{CommandResult, CommandString, Ending};

        use crate::downloading::TheRunsTransfers;
        use crate::servers::sessions::{
            lend_until_it_ends, AHeldConnection, NoSession, TheAdditionalConnection, ThisServersSessions,
        };

        const WHERE_SFTP_SERVER_LIVES: &[&str] = &[
            "/usr/libexec/sftp-server",
            "/usr/lib/openssh/sftp-server",
            "/usr/libexec/openssh/sftp-server",
            "/usr/lib/ssh/sftp-server",
        ];

        struct AMachine {
            sftp_server: PathBuf,
            home: PathBuf,
            sessions: AtomicUsize,
        }

        impl AMachine {
            fn standing(named: &str) -> Option<Arc<AMachine>> {
                let sftp_server = WHERE_SFTP_SERVER_LIVES.iter().map(PathBuf::from).find(|path| path.exists())?;
                let home = std::env::temp_dir().join(format!("smzy-picture-{named}-{}", std::process::id()));
                let _ = std::fs::remove_dir_all(&home);
                std::fs::create_dir_all(&home).expect("the login's home");
                let home = std::fs::canonicalize(&home).expect("the home is there");
                Some(Arc::new(AMachine { sftp_server, home, sessions: AtomicUsize::new(0) }))
            }
        }

        impl Drop for AMachine {
            fn drop(&mut self) {
                let _ = std::fs::remove_dir_all(&self.home);
            }
        }

        struct AConnectionToIt(Arc<AMachine>);

        impl AHeldConnection for AConnectionToIt {
            type Channel = tokio::net::UnixStream;

            async fn open_a_session(&mut self) -> Result<tokio::net::UnixStream, NoSession> {
                self.0.sessions.fetch_add(1, Ordering::SeqCst);
                let (ours, theirs) = std::os::unix::net::UnixStream::pair().expect("a socket pair");
                ours.set_nonblocking(true).expect("the end we drive");
                let reading: OwnedFd = theirs.try_clone().expect("its reading end").into();
                let writing: OwnedFd = theirs.into();
                let mut child = Command::new(&self.0.sftp_server)
                    .current_dir(&self.0.home)
                    .stdin(Stdio::from(reading))
                    .stdout(Stdio::from(writing))
                    .stderr(Stdio::null())
                    .spawn()
                    .expect("the machine's own file-transfer server starts");
                std::thread::spawn(move || {
                    let _ = child.wait();
                });
                Ok(tokio::net::UnixStream::from_std(ours).expect("the end we drive"))
            }

            async fn a_command(&mut self, _command: CommandString, _frame: Option<Vec<u8>>) -> Result<CommandResult, NoSession> {
                Err(NoSession::Refused)
            }

            async fn until_it_ends(&mut self) -> Ending {
                std::future::pending().await
            }

            async fn torn_down(self) -> Ending {
                Ending::TornDownBySmoozy
            }
        }

        struct APointLending {
            sessions: ThisServersSessions<AConnectionToIt>,
            lending: tokio::task::JoinHandle<Ending>,
        }

        impl APointLending {
            async fn over(machine: &Arc<AMachine>) -> APointLending {
                let sessions = ThisServersSessions::<AConnectionToIt>::for_one_server();
                let more = Arc::clone(machine);
                let additional: TheAdditionalConnection<AConnectionToIt> = Arc::new(move || {
                    let more = Arc::clone(&more);
                    Box::pin(async move { Some(AConnectionToIt(more)) })
                });
                let mut primary = AConnectionToIt(Arc::clone(machine));
                let lending = tokio::spawn({
                    let sessions = sessions.clone();
                    async move { lend_until_it_ends(&sessions, &mut primary, &additional).await }
                });
                while !sessions.a_connection_is_live() {
                    tokio::task::yield_now().await;
                }
                APointLending { sessions, lending }
            }
        }

        impl Drop for APointLending {
            fn drop(&mut self) {
                self.lending.abort();
            }
        }

        fn a_png(len: usize) -> Vec<u8> {
            let mut bytes = b"\x89PNG\r\n\x1a\n".to_vec();
            bytes.extend((0..len - 8).map(|at| (at % 253) as u8));
            bytes
        }

        /// **A PNG of a few megabytes is read whole over a transfer channel of
        /// its own**, admitted at the run's desktop point under the picture
        /// view's class, and the seat and the draw are back at the end.
        #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
        async fn a_png_on_the_machine_is_read_whole_and_nothing_is_kept() {
            let Some(machine) = AMachine::standing("whole") else {
                println!("SKIPPED: no sftp-server on this system");
                return;
            };
            let png = a_png(3_900_000);
            let at = machine.home.join("kitchen_0002.png");
            std::fs::write(&at, &png).expect("the frame is written");
            let point = APointLending::over(&machine).await;
            let run = TheRunsTransfers::for_this_run();
            let path = at.to_str().expect("a path in words").as_bytes().to_vec();
            let read = super::super::the_picture_of(&point.sessions, &run, &path, png.len() as u64, &|| true).await;
            assert_eq!(read, ThePictureRead::TheBytes { bytes: png, are: TheBytesAre::Png });
            assert_eq!(
                run.grants_made(smoozy_server_protocol::transfers::Direction::Down),
                1,
                "one admission at the desktop point, in the download direction"
            );
            // **Nothing landed anywhere**: the machine's folder holds the one
            // file, and the read made nothing beside it.
            assert_eq!(std::fs::read_dir(&machine.home).expect("the home").count(), 1);
        }

        /// **A 214 MB PNG is not read**: no seat is asked for and nothing is
        /// opened. **And a read the viewer left stops**, with its seat back.
        #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
        async fn a_file_past_the_cap_asks_for_nothing_and_a_read_let_go_stops() {
            let Some(machine) = AMachine::standing("cap") else {
                println!("SKIPPED: no sftp-server on this system");
                return;
            };
            let at = machine.home.join("studio_matte_16bit.png");
            std::fs::write(&at, a_png(1_000)).expect("the file is written");
            let point = APointLending::over(&machine).await;
            let run = TheRunsTransfers::for_this_run();
            let path = at.to_str().expect("a path in words").as_bytes().to_vec();
            let before = machine.sessions.load(Ordering::SeqCst);
            let read = super::super::the_picture_of(&point.sessions, &run, &path, 214_000_000, &|| true).await;
            assert_eq!(read, ThePictureRead::TooLarge);
            assert_eq!(machine.sessions.load(Ordering::SeqCst), before, "no session was opened for it");
            assert_eq!(run.grants_made(smoozy_server_protocol::transfers::Direction::Down), 0);

            // Closed after the seat was had and before the second piece.
            let asked = AtomicUsize::new(0);
            let still_wanted = || asked.fetch_add(1, Ordering::SeqCst) < 3;
            let read = super::super::the_picture_of(&point.sessions, &run, &path, 1_000, &still_wanted).await;
            assert_eq!(read, ThePictureRead::LetGo);
            // **The seat is free again**: the next look is read whole.
            let read = super::super::the_picture_of(&point.sessions, &run, &path, 1_000, &|| true).await;
            assert!(matches!(read, ThePictureRead::TheBytes { .. }), "{read:?}");
        }
    }
}

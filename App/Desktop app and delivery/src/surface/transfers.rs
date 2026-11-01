//! **Transfers — the one window holding every upload and download, and the three
//! boxes that ask what a job's outputs download as.**
//!
//! The page half is `web/transfers.js`, registered under the rail's
//! `transfersview` place. **The mounting contract at the head of
//! `web/the-rail.js` is what it obeys**, and nothing here builds a screen host of
//! its own.
//!
//! **This file is the surface and not the deciding.** Whether a waiting row is a
//! pause or a start-over is `Download` M11's and `Download` M11a's
//! ([`smoozy_download::request::how_the_row_composes`]); what a stop leaves is
//! `Download` M12's ([`smoozy_download::request::what_a_stop_leaves`]); the two
//! marks it reads are `Server protocol and transfers` G4a's and
//! `Server protocol and transfers` G4b's; where a
//! download is proposed to land is this crate's §8, at
//! [`crate::platform::downloads_folder`]; opening a folder is `Download` H7 over
//! this crate's K20 to K24. **Nothing here re-decides any of them.**
//!
//! ## What this piece draws, cited the way the runner ruled on 2026-09-12
//!
//! `Preparation/Design/Transfers.md` carries no clause marker, so the headings
//! are the citation:
//!
//! - `Design/Transfers § Where it sits` — a screen in the rail.
//! - `§ Four kinds, told apart by a tag` — one list, the tag with its arrow,
//!   each request's server badge and name, alternating ground, the narrowing.
//! - `§ An auto-download is not a request, and reads differently` — *Turn off*,
//!   files not bytes, *ended* and never *complete*.
//! - `§ A request shows the whole and the files under it` — the header, the bar,
//!   where it goes, the files, a failed file in the failure shape, *Stop* on the
//!   request and the way back once there is nothing to stop.
//! - `§ What Stop says before it does it` and `§ Stopping one file`.
//! - `§ The dependency check's own transfers`.
//! - `§ What happens to a finished request` and `§ What a past sitting left
//!   behind`.
//! - `§ Waiting for the connection to come back`, `§ When a download meets
//!   trouble`, `§ When the job ended badly`, `§ A file that changed while it was
//!   going up`, `§ A part-sent file that can be carried on from`, `§ While the
//!   part already sent is being checked`, `§ A part-sent file that starts over`.
//! - `§ Along the bottom`, and `§ What this document left open` for the caps'
//!   link to Settings.
//! - `Design/Render Settings`'s *Download as it renders* boxes, which are the
//!   drawing's *Downloading* group.
//!
//! **The ten mockup states** are [`THE_STATES_THIS_PIECE_DRAWS`], read out of
//! `Preparation/Design/Mockup/Smoozy.html`'s own state list by a test below. The
//! account is `Audits/Build/Step 10a/Piece 12 — what landed.md`.
//!
//! ## What can actually happen today, said plainly
//!
//! - **Four downloads run for real, and they are one route entered four ways.**
//!   A queue or history entry's *Download all* and *Download all to…* since step
//!   10c piece 10; a server's file browser's *Download* and *Download to…* since
//!   step 10c's *Files' own Download*. Each composes one `Download` M1 request,
//!   walks the machine (M2), asks M6's question where it has something to ask,
//!   and fetches through [`crate::downloading::the_download_runs`].
//! - **An auto-download arrangement is kept and driven.** It is made at
//!   submission and a connected period walks the job's output folder and brings
//!   its frames home, which is `Download` R1 and §7.
//! - **The dependency check's found files go up and are drawn here** —
//!   `§ The dependency check's own transfers`, step 10c's dependency wave
//!   piece 6. The check's own upload control starts them
//!   (`crate::the_found_files_sent`) and then shows this screen; each press is
//!   one request named after the `.blend`, read from that file's run-held
//!   register ([`the_requests_the_sends_hold`]) and drawn with the same rows
//!   as any upload. Files' own Upload runs from its own screen (step 10c
//!   piece 4) and no longer comes here.
//! - ~~**Every file row, every running request and the stop box are still drawn
//!   against a reading put in by hand.**~~ **An upload's and a manual
//!   download's files in flight are drawn from what the run is moving**, with
//!   the bytes staged inside each (`Server protocol and transfers` G4), and a
//!   request nothing in the run is moving any more reads as ended rather than
//!   running — [`what_this_run_is_moving`], step 10c's transfer wave piece 5.
//!   **`File Browser` U8's leftover row is composed** — an ended request whose
//!   part-sent files are still staged, with *Remove* where the record lets it
//!   remove them — and **Transfers' own *Stop*, on a request and on one file,
//!   stops what it names** (`Server protocol and transfers` C22), both step
//!   10c's transfer wave piece 7 (2026-09-29).
//! - **The format boxes are composed only where a job's reading is on this
//!   computer.** [`the_format_box_from`] takes §8's proposal as a value; the
//!   proposal's route is [`crate::platform::downloads_folder`], and the
//!   photographed build asked it on the Creator's Mac. **After Submit the
//!   queue entry's box is composed from the job's recorded reading, read over
//!   the connection, and its *Done* writes the arrangement** — the 38's piece
//!   9 ([`the_format_box`], [`the_formats_are_chosen_for_a_job`]).
//! - **The 38's piece 9 (2026-10-04)**: a failed download's *Try again* and a
//!   changed upload's *Upload it again* send that one file again as a new
//!   request ([`a_file_is_sent_again`]); *Which ones* names the files an amber
//!   line counts ([`the_files_a_line_names`]); an upload's leftover's *Remove*
//!   removes the part on the machine, inside Smoozy's staging folder only
//!   (`File Browser` U8 as amended); and a queue or History entry's *Files on
//!   your computer* opens `Download` H5's set ([`the_jobs_files_are_shown`]).
//! - **No act goes quiet.** *Files on your computer* opens the folder for real
//!   through `Download` H7; a download that cannot start says which thing did
//!   not happen; and the one act that is not built says so.
//!
//! ## `WhoseSentenceItIs`
//!
//! **A failed file's sentence is drawn through the one failure shape and raises
//! no failure record**, because nothing in this application ends a transfer and
//! so no record exists to raise. When the byte loop is driven, **this screen is
//! the composer of the failure sentence the user reads** (`Design/File Browser`
//! gives it here) and its path must pass `TheComposersOwn`. A test below holds
//! that nothing in this file spends the capture before then.
//!
//! ## How a clause marker is written in this file
//!
//! A marker with no document beside it means a clause of this crate's own plan,
//! `Preparation/Architecture/Desktop app and delivery.md`. **Every marker of
//! another document is written with that document at the site**; `Download`'s
//! series collide with this crate's on nearly every letter, so no header list is
//! used.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use tauri::State;

use smoozy_download::acting::revealing::{open_every_place, OpensAFolder};
use smoozy_file_browser::already_exists::CollisionAnswer;
use smoozy_download::arrangement::{DesiredState, KeepsTheArrangement, TheNarrowing};
use smoozy_download::request::{
    how_the_row_composes, what_a_stop_leaves, HowTheRowComposes, WhatAStopReaches,
    WhatThePreFlightHasToSay,
};
use smoozy_state::identities::{Identity, IdentityKind};
use smoozy_server_protocol::transfers::{Direction, Ending, Report, StartOverCause};

use crate::downloading::{ARequestUnderWay, TheRunsTransfers};
use crate::record::download_records::{DownloadArrangements, DownloadRequests};
use crate::events::{Event, Said, Subject, TheOneQueue};
use crate::platform::downloads_folder::TheProposal;
use crate::record::server_rows;
use crate::scheduling::{BytesPerSecond, TheCap};
use crate::start::Started;
use crate::surface::files::Words;
use crate::surface::render_settings::a_format_in_smoozys_words;
use crate::surface::the_dependency_check::AShapesControl;
use crate::typography::{Prose, Verbatim};

// ════════════════════════════════════════════════════════════════════════════
// The names this piece and its page agree on
// ════════════════════════════════════════════════════════════════════════════

/// **The kind, declared beside the service that posts it.** `web/transfers.js`
/// names the same string.
pub const AN_ACT_ON_TRANSFERS: &str = "transfers:an-act";

/// **A transfer moved — a unit ended, or an auto-download's whole grew, and it
/// is written down; or bytes moved inside a file, or a request's run ended.**
/// `web/transfers.js` names the same string and, where Transfers is the page
/// showing, asks for [`the_transfers_screen`] again.
///
/// **Bytes moving inside a file are told no more than four times a second**,
/// over every transfer of the run together
/// ([`crate::downloading::THE_PAGE_IS_TOLD_AT_MOST_EVERY`]) — added
/// 2026-09-29, step 10c's transfer wave piece 5. What they tell the page about
/// is the run's register and nothing durable (`Server protocol and transfers`
/// G4); the event carries nothing either way.
///
/// **It says nothing, on purpose** (`Desktop app and delivery` P8: events are
/// volatile and nothing is replayed, so the page asks rather than being handed
/// the state). It is posted by the record doors of the three routes that move
/// files — a manual download, an auto-download and an upload — through
/// [`the_transfers_moved`], straight after the durable write (P7). **One event
/// for both directions**, because both draw on this one page (`Download` M16,
/// `File Browser` U11, `Design/Whole app § Anything that isn't instant`: a
/// transfer is *on the row* and says how far along it is). *Added 2026-09-27:
/// until then a row stood at 0 / 3 with every file on disk until the page was
/// left and opened again.*
pub const TRANSFERS_MOVED: &str = "transfers:moved";

/// **A *…to* act's folder panel was answered with a folder** — the Creator's
/// fifth note 7, piece Q. `web/transfers.js` names the same string and goes to
/// Transfers on it.
///
/// **A *…to* act stays where it was pressed until this arrives**, so a panel
/// closed without a folder leaves the user on Queue, History or Files with
/// nothing said. **It is told the moment the folder is chosen** rather than when
/// the request answers, because the count that follows can take minutes and its
/// *Counting files…* row and *Stop* are drawn on Transfers. **It says nothing**:
/// one folder panel is up at a time, and the page knows which act it is
/// waiting on.
pub const A_DOWNLOAD_FOLDER_WAS_CHOSEN: &str = "transfers:a-download-folder-was-chosen";

/// **[`TRANSFERS_MOVED`], posted on the one queue** where the running
/// application holds one. Called by the doors, never by a drawing.
pub fn the_transfers_moved<R: tauri::Runtime>(app: &tauri::AppHandle<R>) {
    use tauri::Manager;
    if let Some(queue) = app.try_state::<Arc<TheOneQueue>>() {
        let the_run = app.try_state::<Arc<TheRunsTransfers>>();
        the_moves_are_posted(&queue, the_run.as_ref().map(|held| held.inner().as_ref()));
    }
}

/// **The moves event, and the title bar's speed beside it.**
///
/// `Design/Whole app § Where things live` puts the speed in the title bar,
/// *"readable from every screen … there only while something is moving"*.
/// **This is where it is sent from, because this is the one place every route
/// that moves a file already tells the page something moved** — bytes inside a
/// file no more than four times a second, and every outcome written. So the
/// bar follows exactly the moments Transfers' own rows follow, and the reading
/// is [`the_live_readings`], the pair Transfers and Settings draw. **Where no
/// channel runs in either direction the bar is sent an empty reading**, which
/// is what takes it away. Added 2026-10-03, step 10c; until then nothing sent
/// the title bar anything.
pub(crate) fn the_moves_are_posted(queue: &TheOneQueue, the_run: Option<&TheRunsTransfers>) {
    let _ = queue.read_then_post::<(), std::convert::Infallible>(
        || Ok(()),
        |_| Event::about(Subject::TheRun, TRANSFERS_MOVED),
    );
    if let Some(the_run) = the_run {
        let _ = crate::surface::the_rail::the_speed_now(queue, the_run);
    }
}

/// **The ten mockup states this piece draws**, exactly as the drawing's own
/// `<select id="state">` names them: the six `t` states of its *Queue & history*
/// group and the whole *Downloading* group. *(Nine until 2026-10-04, when the
/// drawing pass for the Creator's second notes on the test copy drew `fmt-q`,
/// the box opened from a queue entry with its folder as a plain line — their
/// note 16. Nothing here builds that reading yet: it is that map's piece 2.)*
pub const THE_STATES_THIS_PIECE_DRAWS: [&str; 10] = [
    "t", "t-stop", "t-stopwait", "t-verify", "t-restart", "t-left", "fmt", "fmtone", "fmtnone",
    "fmt-q",
];

// ════════════════════════════════════════════════════════════════════════════
// Every word the user reads
// ════════════════════════════════════════════════════════════════════════════

pub(crate) mod words {
    // `§ Where it sits`, `§ Four kinds`, and the caps' link.
    pub const TRANSFERS: &str = "Transfers";
    pub const NO_CAP_SET: &str = "no cap set";
    // **The caps Settings saved, in Settings' own words and figures** — must-do
    // 4 of step 10a round 1 of the queue, history, settings and downloads.
    pub const NO_UPLOAD_CAP_SET: &str = "no upload cap set";
    pub const NO_DOWNLOAD_CAP_SET: &str = "no download cap set";
    /// **Must-do 14: a read that failed is said** — in `Servers`' own words for
    /// the same case, **taken from that screen rather than written again**
    /// (step 10a, the screens round's loose ends, L6). ~~It was a second copy
    /// held here, with a test to catch the two drifting~~, because that file's
    /// words were private to it; they are the crate's now and this is the one
    /// sentence.
    pub const THE_SERVERS_WOULD_NOT_READ: &str =
        crate::surface::servers::words::THE_LIST_COULD_NOT_BE_READ;
    pub const SETTINGS: &str = "Settings";
    pub const SHOW: &str = "Show";
    pub const EVERY_SERVER: &str = "Every server";
    pub const NOTHING_IS_MOVING: &str = "No transfers.";
    pub const NOTHING_IS_MOVING_ON: &str = "No transfers on";
    pub const FULL_STOP: &str = ".";

    // The four tags. `§ An auto-download … is called auto-download, never
    // standing`.
    pub const AUTO_DOWNLOAD: &str = "Auto-download";
    pub const UPLOAD: &str = "Upload";
    pub const DOWNLOAD: &str = "Download";
    pub const DEPENDENCIES: &str = "Dependencies";

    // What a request is called.
    pub const DOWNLOADING_AS_IT_RENDERS: &str = "downloading as it renders";
    pub const DOWNLOADED_AS_IT_RENDERED: &str = "downloaded as it rendered";
    pub const ENDED: &str = "ended";
    pub const DOWNLOADING: &str = "downloading";
    pub const STARTED: &str = "Started";
    pub const START_TIME_NOT_RECORDED: &str = "Start time not recorded";
    pub const RUNNING_NEWEST_FIRST: &str = "In progress · newest first";
    pub const FINISHED_NEWEST_FIRST: &str = "Finished · newest first";
    pub const FILES_INTO: &str = "files into";
    pub const FILE_INTO: &str = "file into";
    pub const FILES: &str = "files";
    pub const FILE: &str = "file";
    pub const FOR: &str = "For";
    pub const DID_NOT_FINISH: &str = "— did not finish";
    /// **A folder download before Smoozy knows what is in it** —
    /// `Design/Transfers`' paragraph of 2026-10-01. Step 10c's twelfth-pass
    /// fix piece 3.
    pub const COUNTING_FILES: &str = "Counting files…";
    /// *p12-many and 2 more*, where several things were picked at once.
    pub const MORE: &str = "more";
    pub const DONE: &str = "done";
    pub const DOT: &str = "·";

    // The one control in a request's header.
    pub const TURN_OFF: &str = "Turn off";
    pub const STOP: &str = "Stop";
    pub const OPEN_THE_CHECK: &str = "Open dependency check";
    pub const OPEN_THE_JOB: &str = "Open the job";
    pub const FILES_ON_YOUR_COMPUTER: &str = "Files on your computer";
    pub const REMOVE: &str = "Remove";

    // Where it goes.
    pub const FROM: &str = "From";
    pub const TO: &str = "To";
    pub const INTO: &str = "into";
    pub const OF: &str = "of";
    pub const AND: &str = "and";
    pub const ONLY: &str = "only";
    pub const ABOUT: &str = "about";
    pub const MINUTES_LEFT: &str = "minutes left";
    pub const MINUTE_LEFT: &str = "minute left";
    pub const EVERYTHING_THIS_JOB_WRITES: &str = "everything this job writes";
    pub const EACH_FILE_KEEPS_ITS_FOLDER: &str = "folder structure kept";
    pub const ENDED_DASH: &str = "ended —";

    // The amber lines. `§ When a download meets trouble`, `§ When the job
    // ended badly`.
    pub const FILES_WERE_ALREADY_HERE: &str = "files already in this folder were not overwritten.";
    pub const FILE_WAS_ALREADY_HERE: &str = "file already in this folder was not overwritten.";
    pub const WHICH_ONES: &str = "Which ones";
    /// **A manual download's names its check refused** — `Download` P8 and
    /// `Download` N7's *cannot land*: the file stays on the server. `Design/Transfers`
    /// has no words for it; these are the shortest Rule 9 says, written for
    /// the Design records. The two transfer faults after the 38's pieces 7
    /// and 9, fault 2.
    pub const FILES_WERE_NOT_DOWNLOADED_THEIR_NAMES: &str =
        "files not downloaded: names not valid on this computer.";
    pub const FILE_WAS_NOT_DOWNLOADED_ITS_NAME: &str =
        "file not downloaded: name not valid on this computer.";
    pub const MAY_BE_INCOMPLETE: &str =
        "downloaded frames may be incomplete — the job stopped while writing them.";
    pub const MAY_BE_INCOMPLETE_ONE: &str =
        "downloaded frame may be incomplete — the job stopped while writing it.";
    pub const THE_FOLDER_CHANGED: &str = "The download folder changed during the job.";
    pub const FILES_ARE_IN: &str = "files are in";
    pub const FILE_IS_IN: &str = "file is in";
    pub const COMMA_AND: &str = ", and";
    pub const ARE_IN: &str = "are in";
    pub const IS_IN: &str = "is in";

    // `§ Waiting for the connection to come back`.
    pub const RECONNECTING: &str = "Reconnecting…";
    pub const WAITING_FOR_CONNECTION: &str = "Waiting for connection…";
    pub const TRYING_AGAIN_IN: &str = "retrying in";
    pub const TRYING_AGAIN_NOW: &str = "retrying now";

    // `Download` D2's four classes, each with its remedy where Smoozy knows one.
    pub const THE_FOLDER_IS_FULL: &str = "Download folder is full.";
    pub const FREE_SOME_SPACE: &str = "Free up space to resume.";
    pub const THE_FOLDER_REFUSES: &str = "Permission denied: cannot write to the download folder.";
    pub const ALLOW_WRITING: &str = "Allow write access to resume.";
    pub const THE_FOLDER_IS_GONE: &str = "Download folder not found.";
    pub const MAKE_IT_EXIST: &str = "Restore the folder to resume.";
    /// **`Download` D2: no cause is known, so none is named** — *disk write
    /// error* says what failed and nothing behind it.
    pub const THE_DEVICE_FAILED: &str = "Disk write error in the download folder.";

    // `§ What a past sitting left behind`.
    pub const STOPPED_WHEN_SMOOZY_CLOSED: &str = "stopped when Smoozy closed";
    pub const STILL_USING_SPACE_ON_THE_SERVER: &str =
        "partial files still use space on the server";
    /// Named for the old words, *part-sent*; the words are *partial* since step
    /// 13's words sweep.
    pub const PART_SENT_FILES_ARE_STILL_IN: &str = "partial files are still in";

    // A file's own row.
    pub const QUEUED: &str = "queued";
    pub const REFUSED: &str = "refused";
    pub const FAILED: &str = "failed";
    pub const CHANGED: &str = "changed";
    /// **A partial file waiting for its connection, after its figure** — an
    /// upload's; named for the old word, *sent*. [`SENT_DOWN`] is a
    /// download's.
    pub const SENT: &str = "uploaded";
    pub const SENT_DOWN: &str = "downloaded";
    pub const CANNOT_WRITE_THERE: &str = "Permission denied: cannot write to this folder.";
    pub const CANNOT_READ_IT_THERE: &str = "Permission denied: cannot read this file.";
    pub const COULD_NOT_READ_ON_YOUR_COMPUTER: &str = "Could not read this file on your computer.";
    /// **A file Smoozy downloaded but could not put in place on this
    /// computer** — `Design/Transfers § A file that failed`, E2-1, with the
    /// reason behind *Details*. Step 10c's thirteenth-pass fix piece 3.
    pub const DOWNLOAD_FAILED: &str = "Download failed.";
    pub const DETAILS: &str = "Details";
    pub const TRY_AGAIN_WITH_ADMIN_ACCESS: &str = "Retry with admin access";
    pub const TRY_AGAIN: &str = "Retry";
    pub const WHAT_THE_SERVER_SAID: &str = "What the server said";
    pub const IT_CHANGED_WHILE_UPLOADING: &str = "File changed during upload — nothing was written.";
    pub const UPLOAD_IT_AGAIN: &str = "Upload again";
    // **A start-over, said the way the underway line says it** —
    // `src/surface/the_underway_shape.rs`: *could not resume* for a partial file
    // that could not be carried on from, *the file changed on the server* for
    // the other cause, and *from the beginning* for both.
    pub const UPLOADING_FROM_THE_START: &str =
        "Could not resume — uploading from the beginning.";
    pub const DOWNLOADING_FROM_THE_START: &str =
        "Could not resume — downloading from the beginning.";
    pub const DOWNLOADING_IT_AGAIN_IT_CHANGED: &str =
        "File changed on the server — downloading from the beginning.";
    pub const VERIFYING_THE: &str = "Verifying the";
    /// After the size, on an upload; [`ALREADY_DOWNLOADED_BEING_CHECKED`] on a
    /// download. Named for the old words, *already sent*.
    pub const ALREADY_SENT_BEING_CHECKED: &str =
        "already uploaded — this can take several minutes.";
    pub const ALREADY_DOWNLOADED_BEING_CHECKED: &str =
        "already downloaded — this can take several minutes.";

    // `§ Along the bottom` — two sentences, two strings (X10).
    pub const CLOSING_STOPS_TRANSFERS: &str = "Closing Smoozy stops transfers.";
    pub const AN_AUTO_DOWNLOAD_RESUMES: &str =
        "Auto-downloads resume on the next connection. Other transfers do not.";

    // `§ What Stop says before it does it`.
    pub const STOP_THIS_UPLOAD: &str = "Stop this upload?";
    pub const STOP_THIS_DOWNLOAD: &str = "Stop this download?";
    pub const FILES_ARE_ALREADY_THERE: &str = "files already there";
    pub const FILE_IS_ALREADY_THERE: &str = "file already there";
    pub const THEY_STAY: &str = "kept";
    pub const WILL_NOT_BE_UPLOADED: &str = "will not be uploaded";
    pub const WILL_NOT_BE_DOWNLOADED: &str = "will not be downloaded";
    pub const NOTHING_UPLOADED_YET: &str = "not started";
    pub const NOTHING_DOWNLOADED_YET: &str = "not started";
    pub const UPLOADING_NOW_DISCARDED: &str = "uploading now — partial upload discarded";
    pub const DOWNLOADING_NOW_DISCARDED: &str =
        "downloading now — partial download discarded";
    pub const NOTHING_IS_UPLOADING_NOW: &str = "Nothing uploading now — nothing is discarded.";
    pub const NOTHING_IS_DOWNLOADING_NOW: &str = "Nothing downloading now — nothing is discarded.";
    pub const KEEP_UPLOADING: &str = "Keep uploading";
    pub const KEEP_DOWNLOADING: &str = "Keep downloading";
    pub const STOP_IT: &str = "Stop it";

    // The three format boxes.
    pub const DOWNLOAD_AS_IT_RENDERS: &str = "Download as it renders";
    pub const THESE_ARE_THE_FORMATS: &str = "Formats this job writes, read from the file.";
    pub const FILES_FULL_STOP: &str = "files.";
    pub const FILE_FULL_STOP: &str = "file.";
    pub const THE_REST_STAY: &str = "The rest stay on the server.";
    pub const YOUR_DOWNLOAD_FOLDER: &str = "— your download folder";
    pub const SOMEWHERE_ELSE: &str = "Somewhere else…";
    pub const WHERE_DOWNLOADS_GO: &str = "Where downloads go";
    pub const FIXED_ONCE_SUBMITTED: &str = "The folder is fixed once the job is submitted.";
    pub const FORMATS_CAN_CHANGE_LATER: &str = "Formats can be changed later from the queue.";
    pub const ONE_FORMAT: &str = "This job writes one format.";
    pub const ALL: &str = "All";
    pub const FILES_ARE_DOWNLOADED: &str = "files are downloaded.";
    pub const THE_ONE: &str = "The";
    pub const FILE_IS_DOWNLOADED: &str = "file is downloaded.";
    pub const NOT_READ: &str = "File not read; formats unknown.";
    /// **The file was read, but not for the scene ticked** — another scene, or
    /// a read that could not name its scene (`Design/Render Settings § What it
    /// will write`, the Creator's sixth notes, note 2).
    pub const READ_ANOTHER_SCENE: &str =
        "Only the active scene was read; the selected scene’s formats are unknown.";
    /// The same, where several scenes are ticked.
    pub const READ_ANOTHER_SCENE_SEVERAL_TICKED: &str =
        "Only the active scene was read; the selected scenes’ formats are unknown.";
    pub const EVERYTHING_DOWNLOADS: &str = "Everything downloads.";
    /// **A narrowing that stands, where the box has no list to show it in** —
    /// the formats in Smoozy's words, joined by [`AND`], then
    /// [`FILES_ARE_DOWNLOADED`]: *EXR and PNG files are downloaded.* (the
    /// runner's ruling of 2026-10-17 on the tenth notes). **This is its one new string**: a
    /// narrowing to no format, which the list says as *0 of 720 files.* and
    /// which nothing but a count could say in the list's words.
    pub const NO_FORMAT_IS_DOWNLOADED: &str = "No format is downloaded.";
    pub const READ_IT_NOW: &str = "Read it now";
    pub const TO_CHOOSE: &str = "to choose formats, or change them later from the queue.";
    pub const CANCEL: &str = "Cancel";
    pub const DONE_BUTTON: &str = "Done";

    // What an act answers where it cannot happen.
    /// **`Design`'s own *not finished* words, and the one sentence an act on
    /// this screen that nothing performs says** — step 10c's transfer wave
    /// piece 7, item 3. ~~*"Smoozy cannot do this yet, because nothing in
    /// Smoozy moves a transfer yet."* and *"…because nothing in Smoozy keeps an
    /// auto-download yet."*~~ Both were true when they were written and both
    /// stopped being true at step 10c piece 10, when arrangements were kept
    /// and downloads ran; each went on naming a reason that was no longer so.
    /// **Taken from `Servers`' screen rather than written again**, so there is
    /// one copy of it.
    pub const NOT_FINISHED: &str = crate::surface::servers::words::NOT_BUILT_YET;
    pub const COULD_NOT_OPEN_THE_FOLDER: &str = "Could not open this folder.";
    /// **A place the files were downloaded to that is no longer there** —
    /// `Download` H8, the Creator's thirteenth notes, note 4: it is not opened,
    /// and its parent is never opened instead (`Design/Queue & History`, the
    /// sentence after *"Its two shortcuts to the files sit together"*).
    pub const THE_FOLDER_IS_NO_LONGER_THERE: &str = "Folder not found.";
    /// **The same, where the one thing a Transfers download brought is a file**,
    /// which *folder* would misname. Not in `Design` yet; written for the
    /// decisions records by note 4's builder.
    pub const THE_FILE_IS_NO_LONGER_THERE: &str = "File not found.";
    /// **The same, where a download brought several things into a folder that
    /// is still there and every one of them is gone** — `Design/Queue &
    /// History`, beside *This folder is no longer there.*, which would name a
    /// folder that is still there. Added 2026-10-25, the small owed fixes after
    /// the eighteenth notes.
    pub const THE_FILES_ARE_NO_LONGER_THERE: &str = "Files not found.";
    /// **A queue or History entry's *Files on your computer* where nothing is
    /// in `Download` H5's set** — `Download` H7a's two readings, `Queue & History` N18's
    /// fourth bullet. The 38's piece 9, item 9.
    pub const NOTHING_OF_THIS_JOB_IS_DOWNLOADED: &str = "Nothing from this job has been downloaded.";
    pub const ONLY_PARTS_WERE_DOWNLOADED: &str =
        "Downloaded in part, from Files.";
    /// **`Download` R11's telling in the box after Submit**, around the folder
    /// the arrangement downloaded into. The 38's piece 9, item 9.
    pub const FILES_ALREADY_DOWNLOADED_STAY_IN: &str = "Downloaded files stay in";
    pub const ONLY_THE_REST_GO_TO_ANOTHER: &str = "— another folder gets only the rest.";
    /// **A *Try again* or *Upload it again* whose request or file is no longer
    /// on record** — the 38's piece 9, item 17.
    pub const THE_REQUEST_IS_NO_LONGER_THERE: &str = "Transfer not found.";
    /// **An upload's *Remove* with the server not connected** — the part is on
    /// the machine. The 38's piece 9, item 33.
    pub const NOT_CONNECTED_SO_NOT_REMOVED: &str = "Not removed: server not connected.";

    // **`Download` M6's pre-flight, for the whole-job download a queue or
    // history entry starts** — step 10c piece 10. The four answers are the
    // spec's own words, and `Download` M4's refusals are said beside them.
    pub const REPLACE: &str = "Replace";
    pub const KEEP_BOTH: &str = "Keep both";
    pub const SKIP_EXISTING: &str = "Skip existing";
    pub const COULD_NOT_BE_READ: &str = "items on the server could not be read.";
    pub const COULD_NOT_BE_READ_ONE: &str = "item on the server could not be read.";

    // **Why a download the user asked for did not start** — step 10c, Files'
    // own Download. **Each names what did not happen**, which is the shape
    // `src/surface/files.rs` already says an upload's refusals in, and none of
    // them is the *nothing moves a transfer yet* sentence: a transfer moves
    // now, so that sentence would be false here.
    pub const NOTHING_IS_SELECTED: &str = "Select a file or folder to download.";
    pub const THE_JOB_WOULD_NOT_READ: &str =
        "Could not read Smoozy's database.";
    /// **A job whose folder is chosen at render start has none yet**
    /// (`Job files and outputs` P21) — `Design/Queue & History`'s own line for
    /// that wait. The Creator's second notes, piece 4.
    pub const THE_FOLDER_IS_MADE_WHEN_THE_RENDER_STARTS: &str = "The folder is made when the render starts.";
    pub const THE_FOLDER_PANEL_DID_NOT_OPEN: &str = "Could not open the folder picker.";
    pub const THE_REQUEST_IS_NO_LONGER_WAITING: &str =
        "Download not found.";
    pub const THE_SERVER_IS_NOT_CONNECTED: &str = "Download not started: server not connected.";
    pub const THERE_IS_NO_DOWNLOAD_FOLDER: &str = "No download folder set. Choose one in Settings.";
    pub const NOTHING_PICKED_COULD_BE_READ: &str =
        "Download not started: could not read the selected items on the server.";
    pub const THE_RECORD_WOULD_NOT_TAKE_IT: &str =
        "Download not started: could not save it to Smoozy's database.";
    pub const WHERE_THE_DOWNLOAD_GOES: &str = "Download to";
}

/// **The counted word, chosen by the count** — must-do 12 of step 10a round 1 of
/// the queue, history, settings and downloads. The page never adds an *s*.
fn by_count(count: impl TryInto<u64>, one: &'static str, many: &'static str) -> &'static str {
    if count.try_into().ok() == Some(1) {
        one
    } else {
        many
    }
}

// ════════════════════════════════════════════════════════════════════════════
// What is known — the readings the screen is composed from
// ════════════════════════════════════════════════════════════════════════════

/// **Everything the screen is composed from.** A reading and never a store: the
/// running half of it is run-held and the rest is read from the local record
/// each time the screen is asked for.
#[derive(Debug, Clone)]
pub struct WhatIsKnown {
    pub servers: Vec<AServerKnown>,
    pub requests: Vec<ARequestKnown>,
    /// The achieved-rate reading in each direction, as `Desktop app and
    /// delivery` R15a publishes it, where anything is moving.
    pub the_speed_up: Option<String>,
    pub the_speed_down: Option<String>,
    /// **The two caps Settings saved**, upload then download — `None` where the
    /// settings would not read, which is said rather than drawn as no cap
    /// (L8a, must-do 4).
    pub the_caps: Option<(TheCap, TheCap)>,
    /// **The server rows would not read** (must-do 14). The screen then says so
    /// rather than drawing an empty filter or *nothing is moving*.
    pub the_servers_would_not_read: bool,
    /// **The first-frame pairs among the auto-downloads, check then range**, by
    /// job identity, as `Queue & History` P5a's record holds them — so the
    /// screen draws each pair as one row ([`the_pairs_are_folded`]).
    /// `Design/Transfers § An auto-download is not a request`, its bullet of
    /// 2026-10-31, the twenty-sixth notes, note 4.
    pub the_first_frame_pairs: Vec<(String, String)>,
}

impl Default for WhatIsKnown {
    fn default() -> WhatIsKnown {
        WhatIsKnown {
            servers: Vec::new(),
            requests: Vec::new(),
            the_speed_up: None,
            the_speed_down: None,
            the_caps: Some((TheCap::NoCapIsSet, TheCap::NoCapIsSet)),
            the_servers_would_not_read: false,
            the_first_frame_pairs: Vec::new(),
        }
    }
}

impl WhatIsKnown {
    /// **Nothing could be read at all** — no record, or a lock that would not
    /// open. Said, never drawn as an empty screen (must-do 14).
    pub fn nothing_would_read() -> WhatIsKnown {
        WhatIsKnown {
            the_caps: None,
            the_servers_would_not_read: true,
            ..WhatIsKnown::default()
        }
    }
}

#[derive(Debug, Clone)]
pub struct AServerKnown {
    pub row: String,
    pub name: String,
    pub badge: Option<&'static str>,
}

/// `§ Four kinds, told apart by a tag`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheKind {
    AutoDownload,
    Upload,
    Download,
    Dependencies,
}

impl TheKind {
    fn goes_up(self) -> bool {
        matches!(self, TheKind::Upload | TheKind::Dependencies)
    }
}

/// What a request is of — which is what it is called.
#[derive(Debug, Clone)]
pub enum WhatItIsOf {
    /// An auto-download, or a whole-job download, named after its job. `job`
    /// is the job's own identity, which *Open the job* hands History (must-do 1).
    AJob { name: String, job: String },
    /// An upload, named by how many files and into which folder.
    AFolder { folder: String },
    /// The dependency check's own, named after the `.blend`.
    ABlend { file: String },
    /// **An upload of the user's own Blender build, named by the file the user
    /// picked** — never by the staging folder it goes into, or the name the
    /// machine reads it under (`Design/Server Setup § Pointing at a build on
    /// your own computer`). Added 2026-10-16, step 10c's owed piece 5.
    ABuild { file: String },
    /// **A manual download being counted**, named after the folder — step
    /// 10c's twelfth-pass fix piece 3, `Design/Transfers`' paragraph of
    /// 2026-10-01. Its request is not written yet (`Download` M1), so the
    /// run holds it ([`TheRunsTransfers::a_count_begins`]).
    AFolderBeingCounted { named: String },
}

/// `Download` D1's four classes, consumed rather than re-derived.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheDestinationTrouble {
    TheVolumeIsFull,
    WritingIsRefused,
    TheDestinationIsNoLongerThere,
    TheWriteFailedAtTheDevice,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhereTheRequestStands {
    /// Something is still to happen.
    Running,
    /// Nothing is left to stop, at this moment.
    Finished { at: String },
    /// **`Download` M13 and `File Browser` U8: an ended request whose
    /// part-sent files are still staged** — told as a leftover, whatever ended
    /// it: an earlier sitting's close, or this run's period ending (`File
    /// Browser` U7's paragraph of 2026-09-28). Step 10c's transfer wave piece
    /// 7.
    ///
    /// **`removable`**: a transfer record of it still reads *interrupted*, so
    /// *Remove* has something to act on (`File Browser` U8's first bullet). Where only owed
    /// cleanups stand (`Server protocol and transfers` S27) the files are
    /// still there and the row says so, with no control: nothing more a
    /// *Remove* could do.
    ///
    /// **`by_an_earlier_run`**: it was not under way at any moment of this run,
    /// so Smoozy closing is what ended it — the only case the drawing's
    /// *stopped when Smoozy closed* is true of.
    LeftOver { removable: bool, by_an_earlier_run: bool },
    /// **Nothing in this run is moving it, and a unit is still owed** —
    /// `File Browser` U7 and its paragraph of 2026-09-28, `Download` M13's.
    /// It ended with an earlier run, with its server's period, or its own run
    /// returned with files not across; nothing will move it again, so it is
    /// never *running*. Drawn like a finished request, with the count it came
    /// to and *— did not finish*. Step 10c's transfer wave piece 5.
    Ended,
}

#[derive(Debug, Clone)]
pub struct ARequestKnown {
    pub id: String,
    pub start: Option<crate::record::transfer_records::TransferStart>,
    pub kind: TheKind,
    pub server: String,
    pub of: WhatItIsOf,
    pub files_done: usize,
    pub files_in_the_whole: usize,
    /// Bytes staged against bytes expected, where the whole is in bytes — an
    /// upload and a manual download (`Download` M16). **An auto-download has
    /// none** (`Download` N8).
    pub bytes: Option<(u64, u64)>,
    /// `Desktop app and delivery` R15b's estimate, where it has one.
    pub seconds_left: Option<u64>,
    /// **In the system's own path form** — `Server protocol and transfers` P7:
    /// text only at the point a name is shown (must-do 18).
    pub destination: PathBuf,
    /// An auto-download's narrowing, as the format names it holds; empty is
    /// *everything*.
    pub narrowed_to: Vec<String>,
    /// `Servers`' repair, where the connection is being repaired: the moment the
    /// next attempt falls, where one is published.
    ///
    /// **Filled from `Servers`' own reading and never derived here** — only
    /// where the request waits for the connection and the server's own row
    /// reads *Reconnecting…*, so the two say the same thing
    /// ([`the_repairs_the_servers_read`]). Step 10c's transfer wave piece 6.
    pub reconnecting: Option<Option<i64>>,
    /// **The request waits for its server's connection to come back** —
    /// `File Browser` U5 and `Download` M11's *waiting*, from the run's own
    /// register. Step 10c's transfer wave piece 6.
    pub waits_for_the_connection: bool,
    /// **It ended in this run because its server's period ended** — its line
    /// cut and not repaired, or the period ended under it — **and not by the
    /// user's *Stop***, from the run's own register
    /// ([`TheRunsTransfers::its_period_ended_it`]). Its ending word is red:
    /// the Creator's seventh notes, piece Z5b.
    pub its_period_ended_it: bool,
    pub trouble: Option<TheDestinationTrouble>,
    pub left_alone: usize,
    /// **A manual download's files whose names its check refused** —
    /// `Download` P8, `Download` N7's *cannot land*: out of the whole, said with the
    /// refusal, and never under *left alone*'s words, which say something
    /// stood in the folder. The two transfer faults after the 38's pieces 7
    /// and 9, fault 2.
    pub cannot_land: usize,
    pub may_be_incomplete: usize,
    pub split_between: Option<(usize, String, usize, String)>,
    /// For an auto-download: whether its job has ended.
    pub the_job_ended: bool,
    /// **For an auto-download: whether its job ended short of a clean
    /// finish** — failed, stopped or given up. **Its whole is then what the
    /// ending's walk found** (`Download` N6's last sentence), which is the
    /// reckoning's own length, and the asked range's count is never put in its
    /// place ([`the_wholes_this_run_has_read`]). Added 2026-10-19, out of the
    /// live pass on Vast for the thirteenth and fourteenth notes: *ended 100 /
    /// 450* for a job stopped at 52 frames.
    pub the_job_ended_short: bool,
    /// **For an auto-download: whether it is done** — `Download` E1 step 4,
    /// *every unit is terminal*. Between the job's ending and this, `Download`
    /// E2's reading holds: the job has ended and files are still coming home,
    /// so E1 step 4's *ended — 3 of 3 files* is not said yet.
    pub the_arrangement_is_done: bool,
    /// **For an auto-download: whether a pass in this run has read the job's
    /// whole** ([`the_wholes_this_run_has_read`]). Where the job has ended and
    /// neither a whole nor a file has been read, its download has not started:
    /// it waits its turn behind another. After 0.1.8, the checks' leftovers.
    pub the_whole_was_read: bool,
    pub stands: WhereTheRequestStands,
    pub files: Vec<AFileKnown>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatHappenedToTheFile {
    Moving,
    WaitingItsTurn,
    /// A permission refusal on the machine.
    RefusedByTheServer,
    /// It never reached the server: Smoozy could not read it on this computer.
    CouldNotBeReadHere,
    /// `File Browser` K8: the source moved under the upload.
    ChangedWhileUploading,
    /// **A file downloaded but not put in place on this computer** —
    /// `Download` P3's recorded *failed*, drawn as `Design/Transfers § A file
    /// that failed` has it with E2-1. **`has_details`**: this run holds what
    /// this computer said, so *Details* is drawn; where it does not, the
    /// control is absent rather than opening an empty panel. Step 10c's
    /// thirteenth-pass fix piece 3.
    NotPutInPlace { has_details: bool },
}

#[derive(Debug, Clone)]
pub struct AFileKnown {
    pub path: String,
    pub record_ending: Option<Ending>,
    pub report: Report,
    pub happened: WhatHappenedToTheFile,
}

// ════════════════════════════════════════════════════════════════════════════
// The answer the page draws — closed types (H34)
// ════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct AControl {
    pub label: Prose,
    pub act: Verbatim,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct AServerToShow {
    pub row: Verbatim,
    pub name: Verbatim,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheTransfersScreen {
    pub heading: Prose,
    pub speeds: Option<Words>,
    pub caps: Words,
    pub settings: Prose,
    pub show: Prose,
    pub every_server: Prose,
    pub servers: Vec<AServerToShow>,
    pub showing: Verbatim,
    pub requests: Vec<ARequestDrawn>,
    pub nothing_here: Option<Words>,
    /// **Must-do 14: the servers would not read**, drawn in place of the list
    /// and the filter.
    pub could_not_read: Option<Prose>,
    pub foot: Words,
    pub the_rails_figure: Option<Verbatim>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct ABar {
    pub value: u64,
    pub max: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct ALine {
    /// `plain`, `amber` (something to notice), `amber2` (work standing still) or
    /// `bad2` (the user has something to put right).
    pub tone: &'static str,
    pub words: Words,
    pub link: Option<AControl>,
    /// `Servers` N10a's published moment, where the line carries a countdown.
    pub due_at: Option<Verbatim>,
    pub trying_again_in: Option<Prose>,
    pub trying_again_now: Option<Prose>,
    pub cross: Option<Prose>,
}

impl ALine {
    fn plain(tone: &'static str, words: Words) -> ALine {
        ALine {
            tone,
            words,
            link: None,
            due_at: None,
            trying_again_in: None,
            trying_again_now: None,
            cross: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct AFailureDrawn {
    pub sentence: Prose,
    pub controls: Vec<AShapesControl>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct AFileDrawn {
    pub path: Verbatim,
    /// The drawing's own row class: empty, `waiting`, `failed`, `changed` or
    /// `waitpart`.
    pub state: &'static str,
    /// Bytes staged and expected, for `theMarkOnARow`, where the row has a bar.
    pub staged: Option<Verbatim>,
    pub expected: Option<Verbatim>,
    pub of: Prose,
    pub tail: Option<Prose>,
    pub note: Option<ALine>,
    pub failure: Option<AFailureDrawn>,
    /// `§ Stopping one file`: the right-click menu exists only where there is
    /// something to stop.
    pub stop: Option<Prose>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct ARequestDrawn {
    pub id: Verbatim,
    pub tag_class: &'static str,
    pub tag: Prose,
    pub arrow: Verbatim,
    pub name: Words,
    /// **The name's first words lead it, in the secondary ink** — a dependency
    /// upload's *For* before its file's name, so the two never read as one
    /// (`Design/Transfers § The dependency check's own transfers`, the
    /// Creator's fourteenth notes, note 4; the drawing's `.rqlead`).
    pub name_leads: bool,
    pub count: Words,
    /// **How a finished request ended, as the ending word's class** — the
    /// Creator's seventh notes, note 9 (`Design/Transfers`, *the finished
    /// line's ending word*): `e-ok` where every file asked for arrived,
    /// `e-fail` where one failed, none where the user stopped it or where an
    /// auto-download only ended.
    pub ending_class: Option<&'static str>,
    pub control: Option<AControl>,
    pub collapsed: bool,
    pub terminal: bool,
    pub started_label: Prose,
    pub started_at_millis: Option<Verbatim>,
    pub group_heading: Option<Prose>,
    pub bar: Option<ABar>,
    pub badge: Option<&'static str>,
    pub where_it_goes: Words,
    pub lines: Vec<ALine>,
    pub files: Vec<AFileDrawn>,
    /// For *Open the check*: the server and the `.blend` the check is of.
    pub server: Verbatim,
    pub the_blend: Option<Verbatim>,
    /// For *Open the job*: the job's own identity, which History chooses by.
    pub job: Option<Verbatim>,
}

// ════════════════════════════════════════════════════════════════════════════
// The composition
// ════════════════════════════════════════════════════════════════════════════

/// **The whole screen, from a reading**, narrowed to one server where the user
/// asked (`§ Four kinds … The list can be narrowed to one server, not
/// reordered`).
pub fn the_screen_from(known: &WhatIsKnown, showing: Option<&str>) -> TheTransfersScreen {
    let folded = the_pairs_are_folded(known);
    let shown: Vec<&ARequestKnown> = folded
        .iter()
        .filter(|request| showing.is_none_or(|row| request.server == row))
        .collect();

    // `Server protocol and transfers` S0a: one desktop clock and allocator across all kinds and servers.
    // Unknown legacy starts follow known ones; identities only break unknown ties.
    let mut ordered = shown;
    ordered.sort_by(|a, b| {
        is_running_or_waiting(b).cmp(&is_running_or_waiting(a))
            .then_with(|| b.start.cmp(&a.start))
            .then_with(|| a.id.cmp(&b.id))
    });

    let running = folded
        .iter()
        .filter(|request| is_running_or_waiting(request))
        .count();

    let nothing_here = (ordered.is_empty() && !known.the_servers_would_not_read).then(|| match showing
        .and_then(|row| known.servers.iter().find(|server| server.row == row))
    {
        Some(server) => Words::with(words::NOTHING_IS_MOVING_ON, server.name.clone())
            .then(words::FULL_STOP)
            .joined(),
        None => Words::plain(words::NOTHING_IS_MOVING),
    });

    let speeds = match (&known.the_speed_up, &known.the_speed_down) {
        (None, None) => None,
        (up, down) => {
            let mut said = Words::plain("");
            if let Some(up) = up {
                said = said.and(format!("↑ {up}"));
            }
            if up.is_some() && down.is_some() {
                said = said.then(words::DOT);
            }
            if let Some(down) = down {
                said = said.and(format!("↓ {down}"));
            }
            Some(said)
        }
    };

    TheTransfersScreen {
        heading: Prose::new(words::TRANSFERS),
        speeds,
        // `Desktop app and delivery` R8: where the user has set no cap the
        // budget is unlimited. **The caps are Settings' and the record holds
        // them** (since 2026-09-13), so each direction shows the cap saved for
        // it, in Settings' own conversion, and *no cap set* only where there is
        // none. **Nothing obeys a cap yet**; that is the board's, not this
        // line's (must-do 4).
        caps: the_caps_line(known.the_caps),
        settings: Prose::new(words::SETTINGS),
        show: Prose::new(words::SHOW),
        every_server: Prose::new(words::EVERY_SERVER),
        servers: known
            .servers
            .iter()
            .map(|server| AServerToShow {
                row: Verbatim::new(server.row.clone()),
                name: Verbatim::new(server.name.clone()),
            })
            .collect(),
        showing: Verbatim::new(showing.unwrap_or_default().to_string()),
        requests: {
            let mut previous_group = None;
            ordered.into_iter().map(|request| {
                let running = is_running_or_waiting(request);
                let mut drawn = a_request_drawn(known, request);
                if previous_group != Some(running) {
                    drawn.group_heading = Some(Prose::new(if running { words::RUNNING_NEWEST_FIRST } else { words::FINISHED_NEWEST_FIRST }));
                    previous_group = Some(running);
                }
                drawn
            }).collect()
        },
        nothing_here,
        could_not_read: known
            .the_servers_would_not_read
            .then(|| Prose::new(words::THE_SERVERS_WOULD_NOT_READ)),
        foot: Words::plain(words::CLOSING_STOPS_TRANSFERS).then(words::AN_AUTO_DOWNLOAD_RESUMES),
        the_rails_figure: (running > 0).then(|| Verbatim::new(running.to_string())),
    }
}

/// **The caps line, in Settings' words and Settings' figures.**
fn the_caps_line(caps: Option<(TheCap, TheCap)>) -> Words {
    use crate::surface::settings::{megabytes_per_second, words as settings_words};
    let Some((up, down)) = caps else {
        return Words::plain(settings_words::THE_SETTINGS_WOULD_NOT_READ);
    };
    if up == TheCap::NoCapIsSet && down == TheCap::NoCapIsSet {
        return Words::plain(words::NO_CAP_SET);
    }
    let one = |said: Words, cap: TheCap, named: &str, none: &str| match cap {
        TheCap::Of(rate) => said.then(named).and(megabytes_per_second(rate)).then(settings_words::MB_PER_SECOND),
        TheCap::NoCapIsSet => said.then(none),
    };
    let said = one(Words::plain(""), up, settings_words::UPLOAD_CAP, words::NO_UPLOAD_CAP_SET);
    one(said.then(words::DOT), down, settings_words::DOWNLOAD_CAP, words::NO_DOWNLOAD_CAP_SET)
}

fn the_server<'a>(known: &'a WhatIsKnown, row: &str) -> Option<&'a AServerKnown> {
    known.servers.iter().find(|server| server.row == row)
}

fn a_request_drawn(known: &WhatIsKnown, request: &ARequestKnown) -> ARequestDrawn {
    let server = the_server(known, &request.server);
    let server_name = server.map(|server| server.name.clone()).unwrap_or_default();
    let (tag_class, tag) = match request.kind {
        TheKind::AutoDownload => ("auto", words::AUTO_DOWNLOAD),
        TheKind::Upload => ("up", words::UPLOAD),
        TheKind::Download => ("dn", words::DOWNLOAD),
        TheKind::Dependencies => ("dep", words::DEPENDENCIES),
    };
    let finished = matches!(request.stands, WhereTheRequestStands::Finished { .. });
    let left_over = matches!(request.stands, WhereTheRequestStands::LeftOver { .. });
    // **Step 10c's transfer wave piece 5: a request nothing is moving any
    // more**, drawn as finished is except that it says it did not finish.
    let ended = request.stands == WhereTheRequestStands::Ended;
    // **An auto-download that brought every file of its whole home** — its job
    // has ended and its arrangement is done, so it is finished, though it
    // still stands *running* while switched on. `Design/Transfers`: *Finished
    // requests collapse to one line each*. The Creator's third notes, note 12:
    // 5 of 5 pictures home, and the row was never greyed. One a file short is
    // not this, and keeps its bar.
    let brought_every_file_home = request.kind == TheKind::AutoDownload
        && request.the_job_ended
        && request.the_arrangement_is_done
        && request.files_done >= request.files_in_the_whole;
    // **An auto-download waiting its turn** — its job has ended, and nothing
    // of its download has started: no file counted and no whole read in this
    // run. It says so in the word this screen says for a file waiting its
    // turn, and draws no count and no bar. After 0.1.8, the checks' leftovers:
    // it read *ended 0 / 0*, as though it had ended with nothing.
    let waiting_its_turn = request.kind == TheKind::AutoDownload
        && request.the_job_ended
        && !request.the_arrangement_is_done
        && !request.the_whole_was_read
        && request.files_done == 0
        && request.files_in_the_whole == 0
        && request.stands == WhereTheRequestStands::Running;

    let terminal_auto = request.kind == TheKind::AutoDownload
        && request.the_job_ended && request.the_arrangement_is_done;
    let terminal = finished || left_over || ended || terminal_auto;
    let name = match (&request.of, request.kind) {
        (WhatItIsOf::AJob { name, .. }, TheKind::AutoDownload) => {
            let tail = if waiting_its_turn {
                words::QUEUED
            } else if !request.the_job_ended {
                words::DOWNLOADING_AS_IT_RENDERS
            } else if !terminal {
                words::DOWNLOADING
            } else if finished && request.left_alone > 0 {
                words::DOWNLOADED_AS_IT_RENDERED
            } else {
                words::ENDED
            };
            Words::plain("").and(name.clone()).then(words::DOT).then(tail)
        }
        (WhatItIsOf::AJob { name, .. }, _) => Words::plain("")
            .and(name.clone())
            .then(words::DOT)
            .and(request.files_in_the_whole.to_string())
            .then(by_count(request.files_in_the_whole, words::FILE, words::FILES)),
        (WhatItIsOf::AFolder { folder }, _) => {
            let said = Words::after(
                request.files_in_the_whole.to_string(),
                by_count(request.files_in_the_whole, words::FILE_INTO, words::FILES_INTO),
            )
            .and(folder.clone());
            if left_over || ended {
                said.then(words::DID_NOT_FINISH)
            } else {
                said
            }
        }
        // **Named after the folder, and counting** — `Design/Transfers`'
        // paragraph of 2026-10-01.
        (WhatItIsOf::AFolderBeingCounted { named }, _) => Words::plain("").and(named.clone()),
        // **Named after the build's own file**, and saying it did not finish
        // as an upload's row does. Added 2026-10-16, step 10c's owed piece 5.
        (WhatItIsOf::ABuild { file }, _) => {
            let said = Words::plain("").and(file.clone());
            if left_over || ended {
                said.then(words::DID_NOT_FINISH)
            } else {
                said
            }
        }
        // **Named after the `.blend`** — its own name, where `file` is the
        // whole path the way back hands the check.
        (WhatItIsOf::ABlend { file }, _) => Words::with(
            words::FOR,
            file.trim_end_matches('/').rsplit('/').next().unwrap_or(file).to_string(),
        ),
    };

    // `§ A request shows the whole … Stop sits on the request`, and `Once there
    // is nothing left to stop, that same place carries the way back`.
    let control = if let WhereTheRequestStands::LeftOver { removable, .. } = request.stands {
        removable.then(|| a_control(words::REMOVE, "remove"))
    } else {
        match (request.kind, finished || ended || request.the_job_ended) {
            (TheKind::AutoDownload, false) => Some(a_control(words::TURN_OFF, "turn-off")),
            (TheKind::AutoDownload, true) => Some(a_control(words::OPEN_THE_JOB, "open-the-job")),
            // **A running dependency upload offers Stop** — `§ A request shows
            // the whole`; the way back replaces it only once there is nothing
            // left to stop (must-do 17).
            (TheKind::Dependencies, true) => Some(a_control(words::OPEN_THE_CHECK, "open-the-check")),
            (TheKind::Download, true) => Some(a_control(
                words::FILES_ON_YOUR_COMPUTER,
                "files-on-your-computer",
            )),
            (TheKind::Upload, true) => None,
            (_, false) => Some(a_control(words::STOP, "stop")),
        }
    };

    let count = if finished && request.kind != TheKind::AutoDownload {
        Words::plain(words::DONE)
    } else if matches!(request.of, WhatItIsOf::AFolderBeingCounted { .. }) {
        Words::plain(if ended { words::DID_NOT_FINISH } else { words::COUNTING_FILES })
    } else if waiting_its_turn {
        Words::plain("")
    } else {
        Words::plain("").and(format!("{} / {}", request.files_done, request.files_in_the_whole))
    };

    // `§ An auto-download … Its bar is drawn in files`; everything else is
    // drawn in bytes over a total known before the first byte moves.
    let bar = if terminal || waiting_its_turn {
        None
    } else if request.kind == TheKind::AutoDownload {
        Some(ABar {
            value: request.files_done as u64,
            max: request.files_in_the_whole.max(1) as u64,
        })
    } else {
        request.bytes.map(|(staged, expected)| ABar {
            value: staged,
            max: expected.max(1),
        })
    };

    let mut lines = Vec::new();

    // `File Browser` U5 and `Download` M11: both directions state the wait.
    // The primary may still be connected while the extra connection is being
    // replaced (`Servers` S14 condition 6). Only a primary repair has the
    // server's Reconnecting line and its next-attempt time.
    if request.waits_for_the_connection || request.reconnecting.is_some() {
        lines.push(match request.reconnecting {
            Some(due) => ALine {
                tone: "amber2",
                words: Words::plain(words::RECONNECTING),
                link: None,
                due_at: due.map(|at| Verbatim::new(at.to_string())),
                trying_again_in: Some(Prose::new(words::TRYING_AGAIN_IN)),
                trying_again_now: Some(Prose::new(words::TRYING_AGAIN_NOW)),
                cross: None,
            },
            None => ALine::plain("amber2", Words::plain(words::WAITING_FOR_CONNECTION)),
        });
    }

    // `§ When a download meets trouble` — `Download` D2's words, the remedy
    // only where Smoozy knows one.
    if let Some(trouble) = request.trouble {
        let said = match trouble {
            TheDestinationTrouble::TheVolumeIsFull => {
                Words::plain(words::THE_FOLDER_IS_FULL).then(words::FREE_SOME_SPACE)
            }
            TheDestinationTrouble::WritingIsRefused => {
                Words::plain(words::THE_FOLDER_REFUSES).then(words::ALLOW_WRITING)
            }
            TheDestinationTrouble::TheDestinationIsNoLongerThere => {
                Words::plain(words::THE_FOLDER_IS_GONE).then(words::MAKE_IT_EXIST)
            }
            // **`Download` D2: for the device failure no cause is known, so no
            // remedy is named.**
            TheDestinationTrouble::TheWriteFailedAtTheDevice => {
                Words::plain(words::THE_DEVICE_FAILED)
            }
        };
        let mut line = ALine::plain("bad2", said);
        line.cross = Some(Prose::new(crate::presentation::the_failure_shape::CLEAR_THIS));
        lines.push(line);
    }

    if request.kind == TheKind::AutoDownload && request.the_job_ended && request.may_be_incomplete > 0 {
        let mut line = ALine::plain(
            "amber",
            Words::after(
                request.may_be_incomplete.to_string(),
                by_count(request.may_be_incomplete, words::MAY_BE_INCOMPLETE_ONE, words::MAY_BE_INCOMPLETE),
            ),
        );
        line.link = Some(a_control(words::WHICH_ONES, "which-ones-may-be-incomplete"));
        lines.push(line);
    }
    if let Some((first_count, first, second_count, second)) = &request.split_between {
        lines.push(ALine::plain(
            "amber",
            Words::plain(words::THE_FOLDER_CHANGED)
                .and(first_count.to_string())
                .then(by_count(*first_count, words::FILE_IS_IN, words::FILES_ARE_IN))
                .and(first.clone())
                .then(words::COMMA_AND)
                .joined()
                .and(second_count.to_string())
                .then(by_count(*second_count, words::IS_IN, words::ARE_IN))
                .and(second.clone())
                .then(words::FULL_STOP)
                .joined(),
        ));
    }
    if request.left_alone > 0 {
        let mut line = ALine::plain(
            "amber",
            Words::after(
                request.left_alone.to_string(),
                by_count(request.left_alone, words::FILE_WAS_ALREADY_HERE, words::FILES_WERE_ALREADY_HERE),
            ),
        );
        line.link = Some(a_control(words::WHICH_ONES, "which-ones-were-left-alone"));
        lines.push(line);
    }
    if request.cannot_land > 0 {
        let mut line = ALine::plain(
            "amber",
            Words::after(
                request.cannot_land.to_string(),
                by_count(
                    request.cannot_land,
                    words::FILE_WAS_NOT_DOWNLOADED_ITS_NAME,
                    words::FILES_WERE_NOT_DOWNLOADED_THEIR_NAMES,
                ),
            ),
        );
        line.link = Some(a_control(words::WHICH_ONES, "which-ones-cannot-land"));
        lines.push(line);
    }

    // `§ What happens to a finished request … does not collapse where it
    // carries an amber line`.
    // **A file that failed keeps its row after the request ends** —
    // `Design/Transfers`: *a failed transfer keeps its red*, and a finished
    // request collapses only where nothing on it still needs reading.
    let a_file_failed = |file: &&AFileKnown| {
        matches!(
            file.happened,
            WhatHappenedToTheFile::RefusedByTheServer
                | WhatHappenedToTheFile::CouldNotBeReadHere
                | WhatHappenedToTheFile::NotPutInPlace { .. }
                // **And a file that changed while it went up**, whose *Upload
                // it again* is the user's to press after the request ends
                // (the 38's piece 9, item 17).
                | WhatHappenedToTheFile::ChangedWhileUploading
        )
    };
    let collapsed = (finished || left_over || ended || brought_every_file_home)
        && !lines.iter().any(|line| line.tone == "amber")
        && !request.files.iter().any(|file| a_file_failed(&file));

    // **The ending word takes History's colours** — the Creator's seventh
    // notes, note 9. Red where a file failed, **and where its line was cut or
    // its server's period ended it** (piece Z5b, the Creator, 2026-10-10:
    // *"Yes, it should."*); green only for *done*; a request the user
    // stopped, one Smoozy closing stopped, and an auto-download's *ended*
    // stay grey. **Smoozy closing stays grey** because closing it is the
    // user's own act, and the row says so (*stopped when Smoozy closed*).
    let ending_class = if !(finished || left_over || ended) {
        None
    } else if request.files.iter().any(|file| a_file_failed(&file)) || request.its_period_ended_it {
        Some("e-fail")
    } else if finished && request.kind != TheKind::AutoDownload {
        Some("e-ok")
    } else {
        None
    };

    ARequestDrawn {
        id: Verbatim::new(request.id.clone()),
        tag_class,
        tag: Prose::new(tag),
        arrow: Verbatim::new(if request.kind.goes_up() { "↑" } else { "↓" }),
        name,
        name_leads: matches!(request.of, WhatItIsOf::ABlend { .. }),
        count,
        ending_class,
        control,
        collapsed,
        terminal,
        started_label: Prose::new(if request.start.is_some() { words::STARTED } else { words::START_TIME_NOT_RECORDED }),
        started_at_millis: request.start.map(|start| Verbatim::new(start.started_at_utc.to_string())),
        group_heading: None,
        bar,
        badge: server.and_then(|server| server.badge),
        where_it_goes: the_line_saying_where_it_goes(
            request,
            &server_name,
            finished || ended,
            left_over,
        ),
        lines,
        files: if finished || left_over || ended || brought_every_file_home {
            request
                .files
                .iter()
                .filter(a_file_failed)
                .map(|file| a_file_drawn(request, file))
                .collect()
        } else {
            request
                .files
                .iter()
                .map(|file| a_file_drawn(request, file))
                .collect()
        },
        server: Verbatim::new(request.server.clone()),
        the_blend: match &request.of {
            WhatItIsOf::ABlend { file } => Some(Verbatim::new(file.clone())),
            _ => None,
        },
        job: match &request.of {
            WhatItIsOf::AJob { job, .. } => Some(Verbatim::new(job.clone())),
            _ => None,
        },
    }
}

fn a_control(label: &str, act: &str) -> AControl {
    AControl {
        label: Prose::new(label),
        act: Verbatim::new(act.to_string()),
    }
}

fn the_line_saying_where_it_goes(
    request: &ARequestKnown,
    server_name: &str,
    finished: bool,
    left_over: bool,
) -> Words {
    let mut said = Words::with(
        if request.kind.goes_up() { words::TO } else { words::FROM },
        server_name.to_string(),
    );
    if left_over {
        // **`stopped when Smoozy closed` only where it is true** — a request
        // this run's period ended was stopped by that, not by Smoozy closing
        // (step 10c's transfer wave piece 7).
        if matches!(
            request.stands,
            WhereTheRequestStands::LeftOver { by_an_earlier_run: true, .. }
        ) {
            said = said.then(words::DOT).then(words::STOPPED_WHEN_SMOOZY_CLOSED);
        }
        said = said.then(words::DOT);
        // `§ What a past sitting left behind`: **which disk, and no size.** An
        // upload's staged files are on the server; a download's are in the folder
        // they were coming to (`Server protocol and transfers` S28).
        return if request.kind.goes_up() {
            said.then(words::STILL_USING_SPACE_ON_THE_SERVER)
        } else {
            said.then(words::PART_SENT_FILES_ARE_STILL_IN)
                .and_a_path(request.destination.display().to_string())
        };
    }
    if request.kind != TheKind::Upload && request.kind != TheKind::Dependencies {
        if !(request.kind == TheKind::AutoDownload && request.the_job_ended && request.split_between.is_some()) {
            said = said.then(words::DOT).then(words::INTO).and_a_path(request.destination.display().to_string());
        }
    }
    if request.kind == TheKind::AutoDownload {
        said = said.then(words::DOT);
        said = match request.narrowed_to.as_slice() {
            [] => said.then(words::EVERYTHING_THIS_JOB_WRITES),
            [one] => said.and(one.clone()).then(words::ONLY),
            many => {
                let mut with = said;
                for (at, format) in many.iter().enumerate() {
                    if at > 0 {
                        with = with.then(words::AND);
                    }
                    with = with.and(format.clone());
                }
                with
            }
        };
        // **`Download` E1 step 4's reading, and only once the arrangement is
        // done.** Said before its first file had landed, *ended — 0 of 3 files*
        // read as a finished count of nothing (the seventh pass on Hyperstack,
        // fault 8); until done, E2's reading stands and the row's own count
        // moves as the files land.
        if request.the_job_ended && request.the_arrangement_is_done {
            said = said
                .then(words::DOT)
                .then(words::ENDED_DASH)
                .and(request.files_done.to_string())
                .then(words::OF)
                .and(request.files_in_the_whole.to_string())
                .then(by_count(request.files_in_the_whole, words::FILE, words::FILES));
        }
    }
    if request.kind == TheKind::Dependencies {
        said = said.then(words::DOT).then(words::EACH_FILE_KEEPS_ITS_FOLDER);
    }
    if !finished {
        if let Some((staged, expected)) = request.bytes {
            said = said
                .then(words::DOT)
                .and(a_size(staged))
                .then(words::OF)
                .and(a_size(expected));
        }
        if let Some(seconds) = request.seconds_left {
            let minutes = seconds.div_ceil(60);
            said = said
                .then(words::DOT)
                .then(words::ABOUT)
                .and(minutes.to_string())
                .then(by_count(minutes, words::MINUTE_LEFT, words::MINUTES_LEFT));
        }
    }
    if let WhereTheRequestStands::Finished { at } = &request.stands {
        said = said.then(words::DOT).and(at.clone());
    }
    said
}

/// **A file's own row.** Which line it carries is read off the marks, **never
/// off the figure** (`Download` M11a, `Design/Transfers § Waiting for the
/// connection to come back`).
fn a_file_drawn(request: &ARequestKnown, file: &AFileKnown) -> AFileDrawn {
    let up = request.kind.goes_up();
    let mut drawn = AFileDrawn {
        path: Verbatim::new(file.path.clone()),
        state: "",
        staged: Some(Verbatim::new(file.report.bytes_staged.to_string())),
        expected: Some(Verbatim::new(file.report.bytes_expected.to_string())),
        of: Prose::new(words::OF),
        tail: None,
        note: None,
        failure: None,
        stop: None,
    };

    match file.happened {
        WhatHappenedToTheFile::WaitingItsTurn => {
            // `§ A request shows … A file waiting its turn says so`.
            drawn.state = "waiting";
            drawn.staged = None;
            drawn.expected = None;
            drawn.tail = Some(Prose::new(words::QUEUED));
            drawn.stop = Some(Prose::new(words::STOP));
            return drawn;
        }
        WhatHappenedToTheFile::RefusedByTheServer => {
            drawn.state = "failed";
            drawn.staged = None;
            drawn.expected = None;
            drawn.tail = Some(Prose::new(words::REFUSED));
            // **No transfer carries the admin-access offer, in either
            // direction.** `File Browser` P6a: *there is no elevated upload* —
            // the staging sits inside the folder the login cannot write, so
            // *a write refused for permission on an upload is reported as it
            // stands*, and P5's offer *is attached to the acts P6 names and
            // never to a transfer*; `Download` M5 says the same of a download.
            // **So the row says so and stops**, which is `Design/Transfers`'
            // own outcome for a machine with no route. *Until 2026-09-27 an
            // upload's row offered* Try again with admin access*, which
            // `Design/Transfers § A file that failed` asks for and `File Browser` P6a rules
            // out; under Rule 10 the design gives way. A plain* Try again *is
            // not offered either: the same sentence of that design says it
            // would retry the same refused write for ever.*
            let mut controls = Vec::new();
            controls.push(a_shapes_control("raw-text", words::WHAT_THE_SERVER_SAID, "what-the-server-said"));
            controls.push(a_shapes_control(
                "cross",
                crate::presentation::the_failure_shape::CLEAR_THIS,
                "clear-a-failure",
            ));
            drawn.failure = Some(AFailureDrawn {
                sentence: Prose::new(if up {
                    words::CANNOT_WRITE_THERE
                } else {
                    words::CANNOT_READ_IT_THERE
                }),
                controls,
            });
            return drawn;
        }
        WhatHappenedToTheFile::CouldNotBeReadHere => {
            drawn.state = "failed";
            drawn.staged = None;
            drawn.expected = None;
            drawn.tail = Some(Prose::new(words::FAILED));
            // It never reached the server, so the control naming the server's
            // words is absent rather than opening an empty panel.
            drawn.failure = Some(AFailureDrawn {
                sentence: Prose::new(words::COULD_NOT_READ_ON_YOUR_COMPUTER),
                controls: vec![
                    a_shapes_control("retry", words::TRY_AGAIN, "try-again"),
                    a_shapes_control(
                        "cross",
                        crate::presentation::the_failure_shape::CLEAR_THIS,
                        "clear-a-failure",
                    ),
                ],
            });
            return drawn;
        }
        WhatHappenedToTheFile::NotPutInPlace { has_details } => {
            // **`Design/Transfers § A file that failed`, E2-1: *Download
            // failed.*, then *Try again*, then the reason behind *Details*** —
            // this computer's own words, never in front of the user — in the
            // whole-app failure shape's order. **`Download` P3's per-file *Try
            // again* is the user's act**, a new request for that one file
            // ([`a_file_is_sent_again`]; `State, identity and recovery` P7).
            // Step 10c's thirteenth-pass fix piece 3; *Try again* the 38's
            // piece 9, item 17.
            drawn.state = "failed";
            drawn.staged = None;
            drawn.expected = None;
            drawn.tail = Some(Prose::new(words::FAILED));
            let mut controls = vec![a_shapes_control("retry", words::TRY_AGAIN, "try-again")];
            if has_details {
                controls.push(a_shapes_control("raw-text", words::DETAILS, "details"));
            }
            controls.push(a_shapes_control(
                "cross",
                crate::presentation::the_failure_shape::CLEAR_THIS,
                "clear-a-failure",
            ));
            drawn.failure = Some(AFailureDrawn {
                sentence: Prose::new(words::DOWNLOAD_FAILED),
                controls,
            });
            return drawn;
        }
        WhatHappenedToTheFile::ChangedWhileUploading => {
            // `§ A file that changed while it was going up` — amber, *Upload it
            // again*, never *Try again*.
            drawn.state = "changed";
            drawn.staged = None;
            drawn.expected = None;
            drawn.tail = Some(Prose::new(words::CHANGED));
            let mut note = ALine::plain("amber", Words::plain(words::IT_CHANGED_WHILE_UPLOADING));
            note.link = Some(a_control(words::UPLOAD_IT_AGAIN, "upload-it-again"));
            drawn.note = Some(note);
            return drawn;
        }
        WhatHappenedToTheFile::Moving => {}
    }

    drawn.stop = Some(Prose::new(words::STOP));

    // **`Server protocol and transfers` G4b's mark first, then
    // `Server protocol and transfers` G4a's**, and only then the composition that
    // turns on the record's ending.
    let already_checked =
        if up { words::ALREADY_SENT_BEING_CHECKED } else { words::ALREADY_DOWNLOADED_BEING_CHECKED };
    if let Some(prefix) = file.report.verifying_a_prefix_of {
        drawn.note = Some(ALine::plain(
            "amber",
            Words::with(words::VERIFYING_THE, a_size(prefix)).then(already_checked),
        ));
        return drawn;
    }
    if let Some(cause) = file.report.started_over {
        drawn.note = Some(ALine::plain("amber", Words::plain(the_start_over_words(up, cause))));
        return drawn;
    }
    match how_the_row_composes(file.record_ending.as_ref(), true, &file.report) {
        HowTheRowComposes::WaitingAndTheFigureIsTruthful => {
            // `§ A part-sent file that can be carried on from`: the bar where it
            // stood, *uploaded* (or *downloaded*) beside it, and no control.
            drawn.state = "waitpart";
            drawn.tail = Some(Prose::new(if up { words::SENT } else { words::SENT_DOWN }));
            drawn.stop = None;
        }
        HowTheRowComposes::WaitingAndItStartedOver(cause) => {
            drawn.note = Some(ALine::plain("amber", Words::plain(the_start_over_words(up, cause))));
        }
        HowTheRowComposes::VerifyingAStagedPrefixOf(prefix) => {
            drawn.note = Some(ALine::plain(
                "amber",
                Words::with(words::VERIFYING_THE, a_size(prefix)).then(already_checked),
            ));
        }
        HowTheRowComposes::NotWaiting => {}
    }
    drawn
}

fn the_start_over_words(up: bool, cause: StartOverCause) -> &'static str {
    match (up, cause) {
        (true, _) => words::UPLOADING_FROM_THE_START,
        (false, StartOverCause::TheSourceChangedUnderADownload) => {
            words::DOWNLOADING_IT_AGAIN_IT_CHANGED
        }
        (false, StartOverCause::TheStagedPrefixDidNotVerify) => words::DOWNLOADING_FROM_THE_START,
    }
}

fn a_shapes_control(is: &str, label: &str, act: &str) -> AShapesControl {
    AShapesControl {
        is: Verbatim::new(is.to_string()),
        label: Prose::new(label),
        act: Verbatim::new(act.to_string()),
    }
}

/// A byte count as the drawing writes one, in the same decimal units the shared
/// underway shape uses.
fn a_size(bytes: u64) -> String {
    const STEPS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];
    let mut left = bytes as f64;
    let mut at = 0;
    while left >= 1000.0 && at < STEPS.len() - 1 {
        left /= 1000.0;
        at += 1;
    }
    if at == 0 {
        crate::typography::a_figure_with_its_unit(bytes, "B")
    } else {
        crate::typography::a_figure_with_its_unit(format!("{left:.1}"), STEPS[at])
    }
}

// ════════════════════════════════════════════════════════════════════════════
// The stop box
// ════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct ABullet {
    pub bold: Words,
    pub a_name: bool,
    pub rest: Prose,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheStopBox {
    pub request: Verbatim,
    pub heading: Prose,
    pub bullets: Vec<ABullet>,
    pub nothing_is_thrown_away: Option<Prose>,
    pub keep: Prose,
    pub stop_it: Prose,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
#[serde(tag = "is", rename_all = "kebab-case")]
pub enum TheStopAnswer {
    TheBox(TheStopBox),
    NothingToStop,
}

/// **`§ What Stop says before it does it`**: what is already across, what will
/// never be sent, and each file in flight by name — **however many are in
/// flight, never one by assumption**. On a request waiting for its connection,
/// nothing is in flight and the box says so.
pub fn the_stop_box_for(request: &ARequestKnown) -> TheStopAnswer {
    if request.stands != WhereTheRequestStands::Running || request.kind == TheKind::AutoDownload {
        return TheStopAnswer::NothingToStop;
    }
    let up = request.kind.goes_up();
    // **A count has nothing across and nothing in flight** — its files are not
    // known yet — so the box asks and says nothing is discarded. Step 10c's
    // twelfth-pass fix piece 3.
    if matches!(request.of, WhatItIsOf::AFolderBeingCounted { .. }) {
        return TheStopAnswer::TheBox(TheStopBox {
            request: Verbatim::new(request.id.clone()),
            heading: Prose::new(words::STOP_THIS_DOWNLOAD),
            bullets: Vec::new(),
            nothing_is_thrown_away: Some(Prose::new(words::NOTHING_IS_DOWNLOADING_NOW)),
            keep: Prose::new(words::KEEP_DOWNLOADING),
            stop_it: Prose::new(words::STOP_IT),
        });
    }
    let leaves = what_a_stop_leaves(WhatAStopReaches::TheWholeRequest);
    // **A request waiting for its connection has nothing in flight**, whether
    // or not its server's row reads *Reconnecting…* at this moment — step 10c's
    // transfer wave piece 6 fills the first from the run's own register.
    let waiting = request.reconnecting.is_some() || request.waits_for_the_connection;
    let mut bullets = Vec::new();
    if leaves.finished_files_stay {
        bullets.push(ABullet {
            bold: Words::after(
                request.files_done.to_string(),
                by_count(request.files_done, words::FILE_IS_ALREADY_THERE, words::FILES_ARE_ALREADY_THERE),
            ),
            a_name: false,
            rest: Prose::new(words::THEY_STAY),
        });
    }
    let in_flight: Vec<&AFileKnown> = if waiting {
        Vec::new()
    } else {
        request
            .files
            .iter()
            .filter(|file| file.happened == WhatHappenedToTheFile::Moving)
            .collect()
    };
    if leaves.unarranged_files_are_never_arranged {
        let never = request
            .files_in_the_whole
            .saturating_sub(request.files_done)
            .saturating_sub(in_flight.len());
        bullets.push(ABullet {
            bold: Words::after(
                never.to_string(),
                if up { words::WILL_NOT_BE_UPLOADED } else { words::WILL_NOT_BE_DOWNLOADED },
            ),
            a_name: false,
            rest: Prose::new(if up { words::NOTHING_UPLOADED_YET } else { words::NOTHING_DOWNLOADED_YET }),
        });
    }
    for file in in_flight {
        bullets.push(ABullet {
            bold: Words::plain("").and(file.path.clone()),
            a_name: true,
            rest: Prose::new(if up {
                words::UPLOADING_NOW_DISCARDED
            } else {
                words::DOWNLOADING_NOW_DISCARDED
            }),
        });
    }
    TheStopAnswer::TheBox(TheStopBox {
        request: Verbatim::new(request.id.clone()),
        heading: Prose::new(if up { words::STOP_THIS_UPLOAD } else { words::STOP_THIS_DOWNLOAD }),
        bullets,
        nothing_is_thrown_away: waiting.then(|| {
            Prose::new(if up { words::NOTHING_IS_UPLOADING_NOW } else { words::NOTHING_IS_DOWNLOADING_NOW })
        }),
        keep: Prose::new(if up { words::KEEP_UPLOADING } else { words::KEEP_DOWNLOADING }),
        stop_it: Prose::new(words::STOP_IT),
    })
}

// ════════════════════════════════════════════════════════════════════════════
// The three format boxes
// ════════════════════════════════════════════════════════════════════════════

/// **What the box is composed from.** `Download` N1 and `Download` N2: the formats a job
/// writes are read off the recorded reading, and until that reading lands there
/// is no list and nothing to narrow with.
#[derive(Debug, Clone)]
pub struct WhatTheBoxKnows {
    /// `None` is `Download` N2's *it has not landed*.
    pub formats: Option<Vec<AFormatKnown>>,
    /// **The narrowing that stands, each format as the reading recorded its
    /// name, and `None` every format** — what goes with the job where this
    /// box changes nothing. With a list it is drawn as the ticks
    /// ([`AFormatKnown::chosen`]); **with none, the box says it in words**
    /// ([`what_downloads_without_a_list`]) and Done keeps it. The
    /// runner's ruling of 2026-10-17 on the tenth notes: the box said *Everything downloads.*
    /// while a narrowing still went with the job.
    pub the_formats_chosen: Option<Vec<Vec<u8>>>,
    pub files_in_the_job: usize,
    /// `Download` H2's standing setting, where the user has one.
    ///
    /// **In the system's own path form and never as text** — `Server protocol
    /// and transfers` P7: *text only at the point a name is shown*. It was
    /// `String` until step 10a, the screens round's loose ends, L6, which
    /// carries out must-do 18 of that round's Verdict: a folder turned into
    /// text and back again is no longer the folder the user chose, on any
    /// system whose names are not valid text.
    pub the_standing_folder: Option<Vec<u8>>,
    /// **The folder already chosen for this one job**, where the box was opened
    /// on a submission the user has already answered — `Design/Render Settings
    /// § Submitting`: *"Where the files go is the one settled at this moment."*
    /// It is offered first, so the box opens on what the user chose. **In the
    /// system's own path form**, as above.
    pub the_folder_chosen_for_this_job: Option<Vec<u8>>,
}

/// **How many files a choice of formats brings**, out of what the box knows —
/// the box's own total and the render settings strip's summary read this one
/// sum, so the two can never say different figures. `Download` N4: a file with
/// no format is outside a narrowing and is in neither figure's first half, as
/// the inventory's split already has it.
pub fn the_files_the_choice_brings(formats: &[AFormatKnown]) -> usize {
    formats.iter().filter(|f| f.chosen).map(|f| f.files).sum()
}

/// **The folders the box offers, in order, each once**: the one already chosen
/// for this job, the standing folder, then §8's proposal. The second value says
/// whether the row is the proposal, which is the one row that says *your
/// download folder* beside it. **`Download` H3: no folder is offered that
/// nobody named.**
///
/// **Each folder is the system's own path form and never text** — `Server
/// protocol and transfers` P7, and must-do 18 of the queue, history, settings
/// and downloads round (step 10a, the screens round's loose ends, L6).
/// ~~The proposal was turned into text with `display()` and every row compared
/// as text~~, so a name that is not valid text came back as something else.
/// **The page is given text for display and hands back which row it was**, not
/// what that row read.
pub fn the_folders_offered(knows: &WhatTheBoxKnows, proposal: &TheProposal) -> Vec<(Vec<u8>, bool)> {
    let proposed: Option<Vec<u8>> = match proposal {
        TheProposal::TheSystemNamesThisFolder(folder) => {
            Some(folder.as_os_str().as_encoded_bytes().to_vec())
        }
        TheProposal::TheSystemNamesNoFolder(_) => None,
    };
    let mut offered: Vec<(Vec<u8>, bool)> = Vec::new();
    for folder in [
        knows.the_folder_chosen_for_this_job.clone(),
        knows.the_standing_folder.clone(),
        proposed.clone(),
    ]
    .into_iter()
    .flatten()
    {
        if offered.iter().any(|(already, _)| *already == folder) {
            continue;
        }
        let is_the_proposal =
            proposed.as_deref() == Some(folder.as_slice()) && knows.the_standing_folder.as_deref() != Some(folder.as_slice());
        offered.push((folder, is_the_proposal));
    }
    offered
}

#[derive(Debug, Clone)]
pub struct AFormatKnown {
    /// **The format as the file records it** (`OPEN_EXR_MULTILAYER`) — what a
    /// narrowing keeps (`Download` N1) and what the page hands back. The box
    /// shows it in Smoozy's words
    /// ([`crate::surface::render_settings::a_format_in_smoozys_words`]).
    pub name: String,
    /// **The folders under the job's folder it is written into**, as `PNG/`,
    /// each once — never the path the file records. Empty where it is written
    /// only at the top of the job's folder.
    pub where_in_the_job: String,
    /// **Whether it is written at the top of the job's folder**, which the box
    /// says in words. The Creator's second notes, piece 2.
    pub at_the_top: bool,
    pub files: usize,
    pub chosen: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct AFormatDrawn {
    /// The identifier, which the page hands back and never shows.
    pub name: Verbatim,
    /// **What the user reads**: the format in Smoozy's words (Rule 9; the
    /// Creator's note 16 on their second notes: the box printed
    /// `OPEN_EXR_MULTILAYER`).
    pub said: Verbatim,
    pub where_in_the_job: Verbatim,
    /// ***top of the job's folder***, where the format is written there.
    pub the_top: Option<Prose>,
    pub files: Words,
    pub chosen: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct AFolderToChoose {
    pub folder: Verbatim,
    pub said: Words,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
#[serde(tag = "reads", rename_all = "kebab-case")]
pub enum TheFormatReading {
    SeveralFormats {
        intro: Prose,
        formats: Vec<AFormatDrawn>,
        total: Words,
        the_rest: Prose,
    },
    OneFormat {
        said: Words,
    },
    NotRead {
        warning: Prose,
        /// [`what_downloads_without_a_list`].
        what_downloads: Words,
        read_it_now: Prose,
        to_choose: Prose,
    },
    /// **The file was read, but not for the scene ticked**, so its list would
    /// be another scene's outputs. There is no *Read it now*: a read would
    /// cover the same scene again (`Design/Render Settings`, the Creator's
    /// sixth notes, note 2). **Also the shape for a typed frame outside the
    /// read** (T1). What downloads is said as where nothing was read.
    ReadForAnotherScene {
        warning: Prose,
        /// [`what_downloads_without_a_list`].
        what_downloads: Words,
    },
}

impl TheFormatReading {
    /// **The box's case where the file was read for another scene** — one
    /// scene ticked, or several — saying what downloads as the box composed
    /// it ([`what_downloads_without_a_list`]).
    pub fn read_for_another_scene(several_ticked: bool, what_downloads: Words) -> Self {
        TheFormatReading::ReadForAnotherScene {
            warning: Prose::new(if several_ticked {
                words::READ_ANOTHER_SCENE_SEVERAL_TICKED
            } else {
                words::READ_ANOTHER_SCENE
            }),
            what_downloads,
        }
    }
}

/// **What downloads, said where the box has no list to show it** — the
/// runner's ruling of 2026-10-17 on the tenth notes: *the box says what will actually go.*
/// *Everything downloads.* where no narrowing stands. Where one does, its
/// formats in Smoozy's words — each a value (X7), joined by *and*, as the
/// strip's summary and the Transfers row name a narrowing — then the box's
/// own *files are downloaded.*: *EXR and PNG files are downloaded.* A
/// narrowing to no format says that.
pub fn what_downloads_without_a_list(chosen: Option<&[Vec<u8>]>) -> Words {
    let Some(chosen) = chosen else {
        return Words::plain(words::EVERYTHING_DOWNLOADS);
    };
    if chosen.is_empty() {
        return Words::plain(words::NO_FORMAT_IS_DOWNLOADED);
    }
    let mut said = Words::plain("");
    for (at, format) in chosen.iter().enumerate() {
        if at > 0 {
            said = said.then(words::AND);
        }
        said = said.and(a_format_in_smoozys_words(format));
    }
    said.then(words::FILES_ARE_DOWNLOADED)
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheFormatBox {
    pub heading: Prose,
    /// **The folder is the one the arrangement was switched on with**, offered
    /// alone and with no *Somewhere else…* — `Download` R1. The queue entry's
    /// box with the arrangement on; never Render Settings'. The 38's piece 9.
    pub the_folder_is_fixed: bool,
    /// **`Download` R11's telling, said before a switch-on proceeds** where the
    /// arrangement already downloaded files. The 38's piece 9.
    pub before_it_moves: Option<Words>,
    pub reading: TheFormatReading,
    pub to: Prose,
    pub where_downloads_go: Prose,
    pub folders: Vec<AFolderToChoose>,
    /// **Where the folder is fixed, it is one plain line and not a choice** —
    /// *To ~/Renders/kitchen*, with no radio button (`Design/Render Settings
    /// § Submitting`, the Creator's note 16 on their second notes). The queue
    /// entry's box with the arrangement on; never Render Settings'.
    pub the_fixed_folder: Option<Words>,
    pub somewhere_else: Prose,
    /// **The sentence under the folders**, where there is one to say: *where
    /// they go is fixed once the job is submitted*, and *the formats can be
    /// changed later from the queue* only where that is still ahead — never
    /// in the box the queue itself opens, and nothing at all where the folder
    /// is already fixed (the drawing's `fmt-q`).
    pub fixed: Option<Words>,
    pub cancel: Prose,
    pub done: Prose,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
#[serde(tag = "is", rename_all = "kebab-case")]
pub enum TheFormatAnswer {
    TheBox(TheFormatBox),
    TheActIsNotBuiltYet { said: Prose, close: Prose },
}

/// **The box, from what it knows and from §8's proposal.**
///
/// **`Desktop app and delivery` W4: the downloads folder is proposed and never
/// written into**; `Download` H2: every write follows a request or an
/// arrangement whose destination was put in front of the user at that moment,
/// which is what this box is; `Download` H3: where the route returns nothing and
/// no standing folder is set, the choice is the user's before anything
/// proceeds — so no folder is offered that nobody named.
pub fn the_format_box_from(knows: &WhatTheBoxKnows, proposal: &TheProposal) -> TheFormatBox {
    let reading = match &knows.formats {
        None => TheFormatReading::NotRead {
            warning: Prose::new(words::NOT_READ),
            what_downloads: what_downloads_without_a_list(knows.the_formats_chosen.as_deref()),
            read_it_now: Prose::new(words::READ_IT_NOW),
            to_choose: Prose::new(words::TO_CHOOSE),
        },
        // **A list of one has only two settings, everything and nothing**, and
        // the second is what switching auto-download off already does — the
        // drawing's own comment on `fmtone`.
        Some(formats) if formats.len() == 1 => TheFormatReading::OneFormat {
            said: Words::plain(words::ONE_FORMAT)
                .then(by_count(formats[0].files, words::THE_ONE, words::ALL))
                // **The count and the format name are one value**, *240 EXR*,
                // and they part no more than a figure and its unit do.
                .and(crate::typography::a_figure_with_its_unit(
                    formats[0].files,
                    &a_format_in_smoozys_words(formats[0].name.as_bytes()),
                ))
                .then(by_count(formats[0].files, words::FILE_IS_DOWNLOADED, words::FILES_ARE_DOWNLOADED)),
        },
        Some(formats) => {
            let chosen = the_files_the_choice_brings(formats);
            TheFormatReading::SeveralFormats {
                intro: Prose::new(words::THESE_ARE_THE_FORMATS),
                formats: formats
                    .iter()
                    .map(|format| AFormatDrawn {
                        name: Verbatim::new(format.name.clone()),
                        said: Verbatim::new(a_format_in_smoozys_words(format.name.as_bytes())),
                        where_in_the_job: Verbatim::new(format.where_in_the_job.clone()),
                        the_top: format
                            .at_the_top
                            .then(|| Prose::new(crate::surface::render_settings::words::TOP_OF_THE_JOBS_FOLDER)),
                        files: Words::after(format.files.to_string(), by_count(format.files, words::FILE, words::FILES)),
                        chosen: format.chosen,
                    })
                    .collect(),
                total: Words::plain("")
                    .and(chosen.to_string())
                    .then(words::OF)
                    .and(knows.files_in_the_job.to_string())
                    .then(by_count(knows.files_in_the_job, words::FILE_FULL_STOP, words::FILES_FULL_STOP)),
                the_rest: Prose::new(words::THE_REST_STAY),
            }
        }
    };

    // **The text is for showing and for nothing else** (must-do 18): the page
    // hands back which row was chosen, by its place in this list, and never
    // what the row read.
    let folders: Vec<AFolderToChoose> = the_folders_offered(knows, proposal)
        .into_iter()
        .map(|(folder, is_the_proposal)| {
            let shown = String::from_utf8_lossy(&folder).to_string();
            let said = Words::plain("").and(shown.clone());
            AFolderToChoose {
                folder: Verbatim::new(shown),
                said: if is_the_proposal { said.then(words::YOUR_DOWNLOAD_FOLDER) } else { said },
            }
        })
        .collect();

    let fixed = Some(match &knows.formats {
        Some(formats) if formats.len() > 1 => {
            Words::plain(words::FIXED_ONCE_SUBMITTED).then(words::FORMATS_CAN_CHANGE_LATER)
        }
        _ => Words::plain(words::FIXED_ONCE_SUBMITTED),
    });

    TheFormatBox {
        heading: Prose::new(words::DOWNLOAD_AS_IT_RENDERS),
        the_folder_is_fixed: false,
        before_it_moves: None,
        reading,
        to: Prose::new(words::TO),
        where_downloads_go: Prose::new(words::WHERE_DOWNLOADS_GO),
        folders,
        the_fixed_folder: None,
        somewhere_else: Prose::new(words::SOMEWHERE_ELSE),
        fixed,
        cancel: Prose::new(words::CANCEL),
        done: Prose::new(words::DONE_BUTTON),
    }
}

// ════════════════════════════════════════════════════════════════════════════
// The acts
// ════════════════════════════════════════════════════════════════════════════

/// **Every act the page may hand back — a closed set** (H34).
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TheAct {
    StopTheRequest,
    StopOneFile,
    TurnOff,
    Remove,
    TryAgain,
    AdminAccess,
    UploadItAgain,
    WhatTheServerSaid,
    /// **A failed download's *Details*** — what this computer said, for the
    /// one file the page names (`Design/Transfers § A file that failed`,
    /// E2-1). Step 10c's thirteenth-pass fix piece 3.
    Details,
    WhichOnesWereLeftAlone,
    /// *Which ones* on a manual download's refused names (`Download` P8).
    WhichOnesCannotLand,
    WhichOnesMayBeIncomplete,
    FilesOnYourComputer,
    ChooseTheFormats,
    // **The acts the three screens that start a transfer hand this one.**
    // `Design/Queue & History`: *"Downloads appear in Transfers"*;
    // `Design/File Browser`: *"Transfers it starts are watched in Transfers"*;
    // `Design/Dependency Upload & Remap`: *"its uploads run in Transfers"*.
    /// A queue or history entry's *Download all*. `request` is the job.
    DownloadAll,
    /// *Download all to…*. `request` is the job.
    DownloadAllTo,
    /// Files' confirmed upload. `request` is the server.
    Upload,
    /// Files' *Download*, from the toolbar or a right-click menu.
    Download,
    /// Files' *Download to…*.
    DownloadTo,
    /// The dependency check's upload control. `request` is the server.
    UploadTheFound,
}

/// **H7 over K20 to K24**: `Download`'s opening, handed this crate's reveal.
///
/// **Each place is shown selected in the folder that holds it** — `Download`
/// H7's paragraph of 2026-10-03, the Creator's note 15 on the test copy — by
/// the reveal's first act, with the second act on the folder that holds it
/// where the first refuses ([`a_place_is_shown`]).
pub(crate) struct TheReveal;

impl OpensAFolder for TheReveal {
    fn open_this_folder(&mut self, folder: &[u8]) -> bool {
        // **The stored path, shown as it was stored** — never through a
        // lossy text conversion, which can change a name that is not valid
        // text (`Server protocol and transfers` P7; must-do 18).
        a_place_is_shown(
            &the_stored_folder(folder),
            crate::reveal::a_file_in_its_folder,
            crate::reveal::a_folder,
        )
        .it_opened()
    }
}

/// **`Download` H7's opening of one place, and its fallback.** The place is
/// named to the reveal's first act, so it shows selected in its folder.
/// **Where that act refuses, the folder that holds it is opened with nothing
/// selected**, which is what the opening did before 2026-10-03 (`Desktop app
/// and delivery` K21's paragraph of that date: the first act on a folder is
/// unresolved on macOS and hedged on Linux). **Only a refusal of that is
/// `Download` H8's *cannot open*.**
///
/// **The fallback is taken only where the place is still there** — the
/// Creator's thirteenth notes, note 4. A place deleted by hand is refused by
/// the first act the same way, and opening its parent then read as the files
/// simply not being selected, with nothing said. **A place that is gone is not
/// opened at all**, and the caller says so ([`it_is_gone`]).
fn a_place_is_shown(
    place: &std::path::Path,
    select: impl FnOnce(&std::path::Path) -> crate::reveal::Revealed,
    open: impl FnOnce(&std::path::Path) -> crate::reveal::Revealed,
) -> crate::reveal::Revealed {
    match select(place) {
        crate::reveal::Revealed::ItOpened => crate::reveal::Revealed::ItOpened,
        crate::reveal::Revealed::ItCouldNotBeOpened if it_is_gone(place) => {
            crate::reveal::Revealed::ItCouldNotBeOpened
        }
        crate::reveal::Revealed::ItCouldNotBeOpened => open(place.parent().unwrap_or(place)),
    }
}

/// **Whether a place is known to be no longer there**, read now and never
/// cached (`Download` H8). **Only a plain *not there* counts**: where the
/// system will not say (a folder Smoozy may not look into), the place is not
/// called gone, and the ordinary *could not open* stands. A link whose target
/// is gone is gone, since the reveal resolves links before it asks.
fn it_is_gone(place: &std::path::Path) -> bool {
    matches!(place.try_exists(), Ok(false))
}

/// **An opener that is never asked for a place that is gone** — note 4: such
/// a place is not opened, whoever the opener is, and reads as not opened.
struct OnlyWhatIsThere<'a>(&'a mut dyn OpensAFolder);

impl OpensAFolder for OnlyWhatIsThere<'_> {
    fn open_this_folder(&mut self, folder: &[u8]) -> bool {
        !it_is_gone(&the_stored_folder(folder)) && self.0.open_this_folder(folder)
    }
}

/// **The sentence for a set of places of which some did not open** — note 4:
/// where one of them is gone, that is what is said; otherwise *could not
/// open*. Nothing where every place opened.
fn what_is_said_of(not_opened: &[Vec<u8>]) -> Option<&'static str> {
    if not_opened.is_empty() {
        None
    } else if not_opened.iter().any(|place| it_is_gone(&the_stored_folder(place))) {
        Some(words::THE_FOLDER_IS_NO_LONGER_THERE)
    } else {
        Some(words::COULD_NOT_OPEN_THE_FOLDER)
    }
}

/// **What a finished download's *Files on your computer* shows** —
/// `Design/Transfers`' paragraph of 2026-10-03: *"opens the folder the
/// download went to with what it brought selected: the job's folder for a
/// whole job, or the one file or folder picked; where several things were
/// picked, it opens that folder with nothing selected."*
#[derive(Debug, Clone, PartialEq, Eq)]
enum WhatTheFilesShow {
    /// The one thing the request landed, selected in its folder.
    ThisSelected(Vec<u8>),
    /// The destination, opened with nothing selected.
    TheDestination(Vec<u8>),
}

/// **The choice, read off the request's own entry records** at the moment the
/// control is used (`Download` H8: never cached).
fn what_the_files_show(request: &smoozy_download::request::AManualRequest) -> WhatTheFilesShow {
    match smoozy_download::landing::what_the_request_landed(request).as_slice() {
        [one] => WhatTheFilesShow::ThisSelected(one.clone()),
        _ => WhatTheFilesShow::TheDestination(request.destination.clone()),
    }
}

/// **The manual request `request` names, read whole from the local record**
/// under its server's row, for *Files on your computer*. Nothing where the
/// request will not read, and the act then opens the destination as the row
/// knows it.
fn the_request_held(
    connection: &rusqlite::Connection,
    server: &str,
    request: &str,
) -> Option<smoozy_download::request::AManualRequest> {
    use smoozy_download::request::KeepsTheRequest as _;
    let row = Identity::read_back(IdentityKind::LocalServer, server)?;
    DownloadRequests::over(connection)
        .every_request_for(&row)
        .ok()?
        .into_iter()
        .find(|one| one.record.as_text() == request)
}

/// **A finished download's *Files on your computer*, performed.** `held` is
/// the request read whole ([`the_request_held`]); without it the destination
/// the row knows is opened with nothing selected, which is what this act did
/// before 2026-10-03. **`Download` H8: a place that cannot be opened is said,
/// not dropped.**
fn the_files_are_shown(
    known: &WhatIsKnown,
    request: &str,
    held: Option<&smoozy_download::request::AManualRequest>,
) -> Option<Prose> {
    the_files_are_shown_through(known, request, held, &mut TheReveal, &mut |at| {
        crate::reveal::a_folder(at).it_opened()
    })
}

/// [`the_files_are_shown`] over the two openings a test can stand in for:
/// `select`, the one thing shown selected in its folder, and `open_a_folder`,
/// the destination opened with nothing selected.
fn the_files_are_shown_through(
    known: &WhatIsKnown,
    request: &str,
    held: Option<&smoozy_download::request::AManualRequest>,
    select: &mut dyn OpensAFolder,
    open_a_folder: &mut dyn FnMut(&std::path::Path) -> bool,
) -> Option<Prose> {
    let Some(found) = known.requests.iter().find(|one| one.id == request) else {
        return Some(Prose::new(words::COULD_NOT_OPEN_THE_FOLDER));
    };
    let shows = match held {
        Some(held) => what_the_files_show(held),
        None => WhatTheFilesShow::TheDestination(found.destination.as_os_str().as_encoded_bytes().to_vec()),
    };
    // **Note 4: a place that is gone is not opened, and is said gone** — the
    // one thing selected through [`a_place_is_shown`], which then opens no
    // parent, and the destination by its own check.
    //
    // **The eighteenth notes, note 15: the same look Queue and History take,
    // before the system is asked** ([`OnlyWhatIsThere`]), so whether a place
    // is gone is Smoozy's own reading and never the system's. **And where
    // several things were brought into a folder that is still there, but every
    // one of them is gone**, the folder is not opened over nothing: that is
    // said, as *These files are no longer there.* (`Design/Queue & History`,
    // 2026-10-25).
    let (opened, gone) = match shows {
        WhatTheFilesShow::ThisSelected(place) => {
            let opened = open_every_place(&mut OnlyWhatIsThere(select), std::slice::from_ref(&place))
                .iter()
                .all(|one| one.it_opened);
            let a_file = held.is_some_and(|held| it_landed_as_a_file(held, &place));
            let gone = it_is_gone(&the_stored_folder(&place)).then_some(if a_file {
                words::THE_FILE_IS_NO_LONGER_THERE
            } else {
                words::THE_FOLDER_IS_NO_LONGER_THERE
            });
            (opened, gone)
        }
        WhatTheFilesShow::TheDestination(folder) => {
            let at = the_stored_folder(&folder);
            let brought = held.map(smoozy_download::landing::what_the_request_landed).unwrap_or_default();
            let all_of_it_is_gone =
                !brought.is_empty() && brought.iter().all(|one| it_is_gone(&the_stored_folder(one)));
            // **The folder itself gone is said as the folder**; the folder
            // still there with everything it was brought gone is said as the
            // files (2026-10-25: it read *This folder is no longer there.*,
            // naming a folder that is there).
            let gone = if it_is_gone(&at) {
                Some(words::THE_FOLDER_IS_NO_LONGER_THERE)
            } else if all_of_it_is_gone {
                Some(words::THE_FILES_ARE_NO_LONGER_THERE)
            } else {
                None
            };
            (gone.is_none() && open_a_folder(&at), gone)
        }
    };
    if opened {
        return None;
    }
    Some(Prose::new(gone.unwrap_or(words::COULD_NOT_OPEN_THE_FOLDER)))
}

/// **Whether the one thing a request landed is a file** rather than a folder:
/// one of its entries landed at that very path, read off the request's own
/// entry records, since a thing that is gone can no longer be looked at.
fn it_landed_as_a_file(request: &smoozy_download::request::AManualRequest, place: &[u8]) -> bool {
    let parts = |path: &[u8]| -> Vec<Vec<u8>> {
        path.split(|byte| *byte == b'/')
            .filter(|part| !part.is_empty())
            .map(<[u8]>::to_vec)
            .collect()
    };
    let place = parts(place);
    request.entries.iter().any(|entry| parts(&entry.landing_path) == place)
}

/// **A folder's stored bytes, back in the system's own path form.** The bytes
/// were made by `OsStr::as_encoded_bytes` on this system, which is the one
/// thing that makes the way back sound.
fn the_stored_folder(folder: &[u8]) -> PathBuf {
    // SAFETY: every folder handed here was made by `as_encoded_bytes` of an
    // `OsStr` on this same system and in this same build (`what_an_act_answers`
    // below), and `from_encoded_bytes_unchecked` is sound for exactly those
    // bytes.
    PathBuf::from(unsafe { std::ffi::OsStr::from_encoded_bytes_unchecked(folder) })
}

/// **What an act on this screen answers, where it answers in prose.**
///
/// **Step 10c's transfer wave piece 7, item 3: no answer names a reason that
/// is no longer true.** Every act here used to fall through to *"…because
/// nothing in Smoozy moves a transfer yet"*, which stopped being true at step
/// 10c piece 10. **Where an act works it is not answered here at all** — the
/// two Stops and Remove are taken by [`an_act_on_transfers`] itself — **and
/// where it does not, it says `Design`'s own *not finished* words**
/// ([`words::NOT_FINISHED`]) and names no reason.
fn what_an_act_answers(known: &WhatIsKnown, act: TheAct, request: &str) -> Option<Prose> {
    match act {
        // **What the server said, for a refused upload** — `Design/Whole
        // app`'s failure shape keeps the server's words behind the control
        // that names whose they are, and this is that control's answer.
        // **The control is drawn only where this run holds the words**, so the
        // other branch is reached only by a page drawn before they went.
        TheAct::WhatTheServerSaid => Some(Prose::new(
            &what_the_server_said_for(request).unwrap_or_else(|| words::NOT_FINISHED.to_string()),
        )),
        // **Taken by [`an_act_on_transfers`] with the file the page names**
        // ([`the_details_of`]); reached here only without one, which names no
        // file and so has nothing to say.
        TheAct::Details => None,
        // **Taken by [`an_act_on_transfers`] with the request read whole**
        // ([`the_request_held`]); reached here only where no record is in
        // hand, which opens the destination with nothing selected.
        TheAct::FilesOnYourComputer => the_files_are_shown(known, request, None),
        // **`Download` R3: switching the arrangement off ends the arranging of
        // new fetches from that moment, removes nothing, and lets every
        // transfer already arranged run to its ending.** It is the arranging
        // that stops and not the fetching, which is why this writes the desired
        // state and takes nothing away.
        // **Taken by [`the_switch_off_is_taken`], where the record is in
        // hand**; reached here only where no record is, which is a write that
        // could not be made.
        TheAct::TurnOff => Some(Prose::new(THE_SWITCH_DID_NOT_SAVE)),
        // **The box is the act**, and the page opens it rather than this
        // answering in prose.
        TheAct::ChooseTheFormats => None,
        // **The dependency check's upload control** is taken on the check
        // itself, which holds the rows (`crate::the_found_files_sent`, step
        // 10c's dependency wave piece 6), and this act is only the way to this
        // screen, where the request then shows: it answers nothing. **Files'
        // own Upload runs** — step 10c piece 4 — and that screen no longer
        // sends `TheAct::Upload` here, so it reaches this arm from nowhere and
        // keeps the *not finished* words only because the act is still in the
        // closed set.
        TheAct::UploadTheFound => None,
        TheAct::Upload => Some(Prose::new(words::NOT_FINISHED)),
        // **The four downloads run, each through its own command**, and the
        // running half answers on its own. **No sentence belongs here**: this
        // arm is reached only if a page took the synchronous route for one of
        // them, which composes no request and would then say something false.
        // [`tests::every_download_act_takes_its_own_command`] is what makes the
        // silence unreachable rather than remembered.
        TheAct::Download | TheAct::DownloadTo | TheAct::DownloadAll | TheAct::DownloadAllTo => None,
        // **Step 10c's transfer wave piece 7: the two Stops and Remove act**,
        // in [`an_act_on_transfers`], and answer nothing: the screen is asked
        // again and says what became of the request.
        TheAct::StopTheRequest | TheAct::StopOneFile | TheAct::Remove => None,
        // **The 38's piece 9: these four act, each through its own command** —
        // *Try again* and *Upload it again* through [`a_file_is_sent_again`],
        // the two *Which ones* through [`the_files_a_line_names`] — and the
        // page takes those commands for them
        // ([`tests::the_acts_that_send_or_name_take_their_own_commands`]). This
        // arm is reached only by a page that did not, which composes nothing,
        // so it says nothing rather than something false.
        TheAct::TryAgain
        | TheAct::UploadItAgain
        | TheAct::WhichOnesWereLeftAlone
        | TheAct::WhichOnesCannotLand
        | TheAct::WhichOnesMayBeIncomplete => None,
        // **Still not built, and it says so and no more**: the admin-access
        // offer no transfer carries any longer (`File Browser` P6a).
        TheAct::AdminAccess => Some(Prose::new(words::NOT_FINISHED)),
    }
}

/// **What the switch-off says where its write could not be made** —
/// `Settings`' own sentence for a write that would not go through, the one the
/// whole app says it in. Step 10c's transfer wave piece 7: it said *"…because
/// nothing in Smoozy keeps an auto-download yet"*, which stopped being true at
/// step 10c piece 10 — the arrangement is kept and the switch-off writes it.
const THE_SWITCH_DID_NOT_SAVE: &str = crate::surface::settings::words::THE_SETTING_DID_NOT_SAVE;

/// **A failed download's *Details*: what this computer said for the one file
/// named**, where this run holds it — `Design/Transfers § A file that failed`,
/// E2-1. Nothing where no file is named or nothing is held; the control is
/// drawn only where something is. Step 10c's thirteenth-pass fix piece 3.
fn the_details_of(request: &str, file: Option<&str>) -> Option<Prose> {
    let said = crate::downloading::what_this_computer_said_for(request, file?.as_bytes())?;
    Some(Prose::new(&said))
}

/// **The server's own words for a request's refused file**, where this run
/// holds them ([`crate::uploading::the_refusals_this_run`]), with a capital
/// to open the sentence.
fn what_the_server_said_for(request: &str) -> Option<String> {
    let held = crate::uploading::the_refusals_this_run().lock().ok()?;
    let said = held
        .iter()
        .find(|((record, _), _)| record == request)
        .map(|(_, said)| said.clone())?;
    let mut letters = said.chars();
    let first = letters.next()?;
    Some(format!("{}{}.", first.to_uppercase(), letters.as_str()))
}

/// **The switch-off, over the application's own record.**
///
/// **A job identity that will not read back, an arrangement that is not there
/// and a record that would not open all answer the same sentence**, because
/// none of the three is a state the user can act on and telling them apart
/// would put the store's own shape in front of them.
fn the_switch_off_is_taken(started: &Mutex<Started>, request: &str) -> Option<Prose> {
    let said = Some(Prose::new(THE_SWITCH_DID_NOT_SAVE));
    let Some(job) = Identity::read_back(IdentityKind::Job, request) else {
        return said;
    };
    let Ok(started) = started.lock() else {
        return said;
    };
    let Some(record) = started.record.as_ref() else {
        return said;
    };
    match the_arrangement_was_switched_off(record.connection(), &job) {
        true => None,
        false => said,
    }
}

/// **`Download` R3's switch-off, written against one open record.**
///
/// **It is the desired state and nothing else.** Every transfer already
/// arranged runs to its ending, the reckoning is untouched, and the record stays
/// readable as part of the entry's account — that clause's own rule, and the
/// reason nothing here removes anything.
///
/// **Answering *true* is answering nothing to the user**, because the screen is
/// redrawn from the store and the arrangement then reads as switched off; a
/// sentence saying it worked would be a second telling of what the row already
/// says.
pub fn the_arrangement_was_switched_off(
    connection: &rusqlite::Connection,
    job: &Identity,
) -> bool {
    let mut keeper = DownloadArrangements::over(connection);
    let Ok(Some(mut arrangement)) = keeper.the_arrangement_for(job) else {
        return false;
    };
    arrangement.desired_state = DesiredState::Off;
    keeper.write_it_down(&arrangement).is_ok()
}

/// **What an act said**, on the one queue. The page draws it on the screen.
pub fn an_act_said(request: &str, said: Option<Prose>) -> Event {
    let event = Event::about(Subject::TheRun, AN_ACT_ON_TRANSFERS)
        .saying("request", Said::Value(Verbatim::new(request.to_string())));
    match said {
        Some(said) => event.saying("said", Said::Prose(said)),
        None => event,
    }
}

// ════════════════════════════════════════════════════════════════════════════
// What the application actually knows, read now
// ════════════════════════════════════════════════════════════════════════════

/// **The reading, taken from the local record at the moment it is asked for.**
///
/// **The servers are real, and since step 10c piece 10 so are the two
/// `Download` stores**: an auto-download arrangement and a manual download
/// request are read out of the local record for every server row, which is
/// `Design/Transfers § Four kinds, told apart by a tag` with real material
/// behind two of the four; **an upload's own request is read too** (since
/// 2026-09-27). ~~An upload's own request is still not read — `File Browser`
/// U8 presents a leftover *"under the current installation identity"*, which
/// is a reading of the machine.~~ **A leftover is told since step 10c's
/// transfer wave piece 7** ([`the_leftovers_are_told`]), and *the current
/// installation* is the row's last confirmed one — the identity every request
/// is arranged under here.
///
/// **What the run is moving is read beside it** ([`what_this_run_is_moving`]),
/// taken by the caller while it holds the record — step 10c's transfer wave
/// piece 5.
fn what_is_known_now(
    started: &Started,
    under_way: &HashMap<String, ARequestUnderWay>,
    the_run: Option<&TheRunsTransfers>,
) -> WhatIsKnown {
    what_is_known_from(started.record.as_ref(), under_way, the_run, started.run.as_ref())
}

/// **The live reading beside the cap — R15 and R15a, one per direction.**
///
/// **It is the run's own admission point that is asked and never a second
/// count**: R15 has the reading taken *"at the same accounting point as the
/// enforcement, per channel and summed"*, so a figure composed anywhere else
/// would be a second answer to how fast the line is going. **Nothing where
/// that direction has no channel**, which is `Server protocol and transfers`
/// R12a's own shape — its first publication moment is an accounting interval,
/// and one exists only while a channel is running.
///
/// **Settings draws this same pair beside its two caps** (`Design/Settings`:
/// *"Each cap has what is actually running beside it"*), so it is
/// crate-visible and read there rather than composed a second time — step
/// 10c's piece 17 of the 38.
pub(crate) fn the_live_readings(transfers: &TheRunsTransfers) -> (Option<String>, Option<String>) {
    let said = |direction| {
        transfers
            .the_reading(direction)
            .map(|reading| a_live_rate_reads(reading.value()))
    };
    (said(Direction::Up), said(Direction::Down))
}

/// **R2's one conversion, and the rounding the drawing gives it.**
///
/// The conversion is `crate::surface::settings::megabytes_per_second` and
/// there is no second one: R15a has the reading pass through R2's conversion
/// *"at the settings edge, once, beside the cap it is read against"*. **This
/// takes that function's figure and rounds its digits**, so the reading is
/// converted exactly once. *(Until step 10b round 1 of* jobs, history and
/// downloads*, must-do 2, it divided by 100,000 itself while this comment said
/// there was no second conversion.)*
///
/// **What is added here is the rounding and nothing else, and it is the
/// drawing's.** That function keeps every digit because the cap field has to
/// give back exactly what the user typed; **a live reading is not something
/// the user typed**, and `Design/Transfers`' own state draws it to one decimal
/// — *↑ 12.4 MB/s · ↓ 4.1 MB/s*. A reading of a slow channel written to six
/// places would be a number nobody can read at a glance, changing every tenth
/// of a second.
///
/// **A moving channel below a tenth reads *<0.1 MB/s*, never *0.0 MB/s*.** The
/// floor itself is 16,384 bytes a second, so every channel held at the floor
/// read as not moving. It follows the History entry's *<1m* for a render under
/// a minute and `src/surface/settings.rs`'s refusal of a zero for something
/// that happened. A reading of nought, which is what a direction is stated as
/// when its last channel ends, still reads *0.0*.
fn a_live_rate_reads(rate: BytesPerSecond) -> String {
    let unit = crate::surface::settings::words::MB_PER_SECOND;
    let converted = crate::surface::settings::megabytes_per_second(rate);
    let (whole, part) = converted.split_once('.').unwrap_or((converted.as_str(), ""));
    let digit = |at: usize| {
        part.as_bytes()
            .get(at)
            .map_or(0, |held| u64::from(held.saturating_sub(b'0')))
    };
    let whole: u64 = whole.parse().unwrap_or(0);
    if rate.0 > 0 && whole == 0 && digit(0) == 0 {
        return crate::typography::a_figure_with_its_unit("<0.1", unit);
    }
    let mut tenths = whole * 10 + digit(0);
    if digit(1) >= 5 {
        tenths += 1;
    }
    crate::typography::a_figure_with_its_unit(format!("{}.{}", tenths / 10, tenths % 10), unit)
}

/// **The screen as the record alone answers it** — everything
/// [`the_transfers_screen`] composes except the live reading beside the cap,
/// which is the run's own admission point and not the record's.
///
/// **It exists for the kept harness** (`tests/huge_transfers_at_the_desk.rs`),
/// which times a Transfers reading taken through the record's own lock while a
/// transfer runs, and has to time the composition the command really does
/// rather than a copy of it.
///
/// **With the run's own register beside it**, since step 10c's transfer wave
/// piece 5: which requests are under way, and what their files have moved, is
/// the run's and not the record's. The caller holds the record while this
/// reads the register, as the command does.
///
/// **It names no run**, so a manual download nothing has decided yet reads as
/// one an earlier run left (step 10c's eleventh-pass fix piece 1). The harness
/// times requests that are moving and makes none of those.
pub fn the_screen_the_record_answers(
    record: Option<&crate::record::LocalRecord>,
    the_run: &TheRunsTransfers,
) -> TheTransfersScreen {
    the_screen_from(
        &what_is_known_from(record, &the_run.what_is_under_way(), Some(the_run), None),
        None,
    )
}

/// **How many of one server's requests are running or waiting** — what a
/// server's Files bar counts (`Design/File Browser § The list`, the Creator's
/// notes piece 6). **It is this screen's own reading of the record and the
/// run, taken the way [`the_transfers_screen`] takes it**, so the bar and this
/// screen cannot count two ways. Nothing where the record could not be read.
pub fn how_many_are_running_on<R: tauri::Runtime>(app: &tauri::AppHandle<R>, server: &str) -> Option<usize> {
    use tauri::Manager as _;
    let the_run = app
        .try_state::<Arc<TheRunsTransfers>>()
        .map(|the_run| Arc::clone(the_run.inner()));
    let started = app.try_state::<Mutex<Started>>()?;
    let started = started.lock().ok()?;
    let known = what_is_known_now(
        &started,
        &the_run.as_ref().map(|the_run| the_run.what_is_under_way()).unwrap_or_default(),
        the_run.as_deref(),
    );
    (!known.the_servers_would_not_read).then(|| the_requests_running_on(&known, server))
}

/// The count itself, over what is known: every kind, running or waiting, as
/// one number — [`is_running_or_waiting`], the rule the rail's figure counts by.
pub fn the_requests_running_on(known: &WhatIsKnown, server: &str) -> usize {
    the_pairs_are_folded(known)
        .iter()
        .filter(|request| request.server == server && is_running_or_waiting(request))
        .count()
}

/// **Whether a request is running or waiting** — what Files' bar and the
/// rail's figure count (`Design/File Browser § The list`).
/// `WhereTheRequestStands::Running` is that for everything but an
/// auto-download, which stands there while it is switched on, ended or not.
/// **An auto-download whose job has ended and whose every file has an outcome
/// is neither**: the row reads *ended*, carries *Open the job*, and nothing is
/// left to move. One whose job has ended with files still coming, or still
/// waiting its turn, is counted.
///
/// **The case the count was first written for** was a request, which leaves
/// *running* when it ends. **The case it now also meets** is an auto-download
/// left switched on after its job: the pass before 0.1.9 on Vast read *6
/// transfers* with nothing moving, for the rest of the session.
pub fn is_running_or_waiting(request: &ARequestKnown) -> bool {
    request.stands == WhereTheRequestStands::Running
        && !(request.kind == TheKind::AutoDownload && request.the_job_ended && request.the_arrangement_is_done)
}

/// **How far the dependency check's uploads of one file on one server have
/// got: the files uploaded so far, of all the files its running or waiting
/// sends are sending** — what *Uploading dependencies: 4 of 12 files.* by
/// Render and the check's strip read (`Design/Render Settings § Submitting`,
/// `Design/Dependency Upload & Remap § Where it sits`; the Creator's note 6 on
/// 0.1.8). **It is this screen's own reading, taken the way
/// [`how_many_are_running_on`] takes it**, so the line, the strip and this
/// screen cannot count two ways. `None` where nothing of that check is going,
/// or the record could not be read.
pub fn the_checks_uploads_on<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    server: &str,
    file: &str,
) -> Option<(usize, usize)> {
    use tauri::Manager as _;
    let the_run = app
        .try_state::<Arc<TheRunsTransfers>>()
        .map(|the_run| Arc::clone(the_run.inner()));
    let started = app.try_state::<Mutex<Started>>()?;
    let started = started.lock().ok()?;
    let known = what_is_known_now(
        &started,
        &the_run.as_ref().map(|the_run| the_run.what_is_under_way()).unwrap_or_default(),
        the_run.as_deref(),
    );
    if known.the_servers_would_not_read {
        return None;
    }
    the_checks_uploads_in(&known, server, file)
}

/// The count itself, over what is known: **the check's own sends** (the
/// *Dependencies* kind) **of this file on this server**, running or waiting,
/// their files summed — files uploaded, of files sent.
pub fn the_checks_uploads_in(known: &WhatIsKnown, server: &str, file: &str) -> Option<(usize, usize)> {
    let (done, whole) = known
        .requests
        .iter()
        .filter(|request| {
            request.kind == TheKind::Dependencies
                && request.server == server
                && request.stands == WhereTheRequestStands::Running
                && matches!(&request.of, WhatItIsOf::ABlend { file: of } if of == file)
        })
        .fold((0, 0), |(done, whole), request| {
            (done + request.files_done, whole + request.files_in_the_whole)
        });
    (whole > 0).then_some((done, whole))
}

/// The same reading, from the record and what the run is moving, so a test
/// can stand where the command stands.
///
/// **`this_run` is this run's own identity** (`State, identity and recovery`
/// Y5), which a manual download's record carries as `Download` M13's marker.
/// It is what tells a request this run holds before its dispositions from one
/// an earlier run left that way — step 10c's eleventh-pass fix piece 1.
fn what_is_known_from(
    record: Option<&crate::record::LocalRecord>,
    under_way: &HashMap<String, ARequestUnderWay>,
    the_run: Option<&TheRunsTransfers>,
    this_run: Option<&Identity>,
) -> WhatIsKnown {
    let Some(record) = record else {
        return WhatIsKnown::nothing_would_read();
    };
    let the_caps = crate::record::settings::read(record.connection())
        .ok()
        .map(|settings| (settings.the_upload_cap, settings.the_download_cap));
    let Ok(rows) = server_rows::all(record.connection()) else {
        return WhatIsKnown {
            the_caps,
            ..WhatIsKnown::nothing_would_read()
        };
    };
    // **Each row's last confirmed installation**, which is what `File Browser`
    // U8 and `Download` M13 mean by *the current installation identity*.
    let installations: HashMap<String, Option<String>> = rows
        .iter()
        .map(|row| {
            (
                row.local_server_identifier.clone(),
                row.last_confirmed_installation_identity.clone(),
            )
        })
        .collect();
    let servers: Vec<AServerKnown> = rows
        .into_iter()
        .map(|row| AServerKnown {
            name: match row.label.as_deref().map(str::trim) {
                Some(label) if !label.is_empty() => label.to_string(),
                _ => format!("{}@{}", row.username.as_str(), row.host.as_str()),
            },
            badge: row.colour.as_deref().and_then(|colour| {
                crate::surface::the_rail::THE_EIGHT_BADGES
                    .iter()
                    .find(|badge| **badge == colour)
                    .copied()
            }),
            row: row.local_server_identifier,
        })
        .collect();
    let (mut requests, held_before_their_dispositions) =
        the_requests_the_stores_hold_and_this_runs_undecided(record.connection(), &servers, this_run);
    for mut live in the_requests_the_sends_hold(&servers) {
        if let Some(saved) = requests.iter_mut().find(|r| r.kind == TheKind::Dependencies && r.id == live.id) {
            live.start = saved.start;
            *saved = live;
        } else {
            requests.push(live);
        }
    }
    // **A manual download this run holds before its dispositions is under way
    // with nothing moving yet** — step 10c's eleventh-pass fix piece 1. The
    // run's register holds a request only from the moment its files begin to
    // be fetched; between its record and that moment — the first read of the
    // destination, and `Download` M6's question — this run is still holding
    // it, and it is never *ended*.
    let mut this_run_holds = under_way.clone();
    for (request, held) in held_before_their_dispositions {
        this_run_holds.entry(request).or_insert(held);
    }
    the_ending_walks_this_run_has_read(&mut requests, the_run, &|job| {
        Identity::read_back(IdentityKind::Job, job).is_some_and(|job| {
            DownloadArrangements::over(record.connection())
                .the_ending_walk_is_written_down(&job)
                .unwrap_or(false)
        })
    });
    what_this_run_is_moving(&mut requests, &this_run_holds);
    the_leftovers_are_told(record.connection(), &mut requests, under_way, the_run, &installations);
    if let Some(the_run) = the_run {
        the_counts_are_told(&mut requests, &servers, the_run);
    }
    let the_first_frame_pairs = the_first_frame_pairs_among(record.connection(), &requests);
    WhatIsKnown {
        servers,
        requests,
        the_caps,
        the_first_frame_pairs,
        ..WhatIsKnown::default()
    }
}

/// **The first-frame pairs among the auto-downloads, check then range** —
/// `Queue & History` P5a's record read for each auto-download that is a check,
/// kept only where its range job's auto-download is on the screen too. Added
/// 2026-10-31, the twenty-sixth notes, note 4.
fn the_first_frame_pairs_among(connection: &rusqlite::Connection, requests: &[ARequestKnown]) -> Vec<(String, String)> {
    let jobs = crate::record::job_records::JobRecords::over(connection);
    let an_auto_download = |id: &str| requests.iter().any(|one| one.kind == TheKind::AutoDownload && one.id == id);
    requests
        .iter()
        .filter(|one| one.kind == TheKind::AutoDownload)
        .filter_map(|one| {
            let job = Identity::read_back(IdentityKind::Job, &one.id)?;
            let purpose = jobs.first_frame_purpose(&job).ok().flatten()?;
            (purpose.check_job == one.id && an_auto_download(&purpose.range_job))
                .then_some((purpose.check_job, purpose.range_job))
        })
        .collect()
}

/// **A first-frame check and its full range, one auto-download row** —
/// `Design/Transfers § An auto-download is not a request`, its bullet of
/// 2026-10-31: the row is the range job's, its count holds the check's file
/// beside the range's frames, and it is finished when both are. **A check
/// whose range is not on the screen keeps its own row.** Folded here, last,
/// so every reading before it — the wholes, the run's proof, what is moving —
/// is each half's own. The twenty-sixth notes, note 4: the pair was two rows
/// of one name.
pub(crate) fn the_pairs_are_folded(known: &WhatIsKnown) -> Vec<ARequestKnown> {
    let mut requests = known.requests.clone();
    for (check, range) in &known.the_first_frame_pairs {
        let at = |requests: &[ARequestKnown], id: &str| {
            requests.iter().position(|one| one.kind == TheKind::AutoDownload && one.id == id)
        };
        let (Some(the_check_at), Some(_)) = (at(&requests, check), at(&requests, range)) else {
            continue;
        };
        let the_check = requests.remove(the_check_at);
        let Some(the_range_at) = at(&requests, range) else {
            continue;
        };
        let row = &mut requests[the_range_at];
        row.files_done += the_check.files_done;
        row.files_in_the_whole += the_check.files_in_the_whole;
        row.left_alone += the_check.left_alone;
        row.cannot_land += the_check.cannot_land;
        row.may_be_incomplete += the_check.may_be_incomplete;
        row.the_arrangement_is_done &= the_check.the_arrangement_is_done;
        row.the_whole_was_read |= the_check.the_whole_was_read;
        row.waits_for_the_connection |= the_check.waits_for_the_connection;
        if row.trouble.is_none() {
            row.trouble = the_check.trouble;
        }
        if is_running_or_waiting(&the_check) {
            row.stands = WhereTheRequestStands::Running;
        }
        row.files.extend(the_check.files);
    }
    requests
}

/// `Download` E1 / `Download` E2: terminal file accounts alone do not prove the ending walk ran.
/// This proof is run-held, derived afresh after reopening, and tied to `Server protocol and transfers` S0a's
/// arrangement generation. No doneness judgement is written to the record.
///
/// **`Download` R5's dated note of 2026-10-31: an ending walk written down as
/// completed for the arrangement's current start is proof too** —
/// `written_down`, asked of the record by job — so a finished row stays
/// finished across a restart, connected or not. The twenty-sixth notes, note
/// 4: every ended auto-download came back as *downloading* after Smoozy
/// opened again, and stayed so on a server never connected again.
fn the_ending_walks_this_run_has_read(
    requests: &mut [ARequestKnown],
    the_run: Option<&TheRunsTransfers>,
    written_down: &dyn Fn(&str) -> bool,
) {
    for request in requests {
        if request.kind == TheKind::AutoDownload && request.the_job_ended && request.the_arrangement_is_done {
            request.the_arrangement_is_done = the_run.is_some_and(|run| run.the_auto_ending_walk_completed(
                &request.id, request.start.map(|s| s.start_order),
            )) || written_down(&request.id);
        }
    }
}

/// **The manual downloads this run is counting, each a row from the press** —
/// `Design/Transfers`' paragraph of 2026-10-01: *"A download of a folder is on
/// Transfers from the moment it is asked for. Until Smoozy knows what is in
/// the folder, its row names the folder and reads Counting files…, with
/// Stop."* Step 10c's twelfth-pass fix piece 3.
///
/// `Server protocol and transfers` S0a writes an empty parent and its start before counting. While the count
/// is held it replaces that parent's row, preserving its stamp; dropping the
/// count reveals the full enumeration without a second row.
fn the_counts_are_told(requests: &mut Vec<ARequestKnown>, servers: &[AServerKnown], the_run: &TheRunsTransfers) {
    for (id, count) in the_run.what_is_being_counted() {
        if !servers.iter().any(|one| one.row == count.server) { continue; }
        if requests.iter().any(|one| one.id == id && one.files_in_the_whole > 0) { continue; }
        let start = requests.iter().find(|one| one.id == id).and_then(|one| one.start);
        requests.retain(|one| one.id != id);
        requests.push(ARequestKnown {
            start,
            id,
            kind: TheKind::Download,
            server: count.server,
            of: WhatItIsOf::AFolderBeingCounted { named: count.named },
            files_done: 0,
            files_in_the_whole: 0,
            bytes: None,
            seconds_left: None,
            destination: the_stored_folder(&count.destination),
            narrowed_to: Vec::new(),
            reconnecting: None,
            waits_for_the_connection: false,
            its_period_ended_it: false,
            trouble: None,
            left_alone: 0,
            cannot_land: 0,
            may_be_incomplete: 0,
            split_between: None,
            the_job_ended: false,
            the_job_ended_short: false,
            the_arrangement_is_done: false,
            the_whole_was_read: false,
            stands: WhereTheRequestStands::Running,
            files: Vec::new(),
        });
    }
}

/// **`Download`'s two durable stores and `File Browser`'s upload requests,
/// read into the screen's own reading — as counts the store answers.**
///
/// **One pass per server row**, because every store is keyed by it, and a row
/// whose store will not read contributes nothing rather than an empty request —
/// which is the same refusal every other read on this screen makes.
///
/// **Nothing here judges anything.** What an arrangement has taken is `Download`
/// R4's reckoning as it stands; what a request came to is its own unit
/// outcomes. **No walk, no whole and no doneness is derived**, because
/// `Download` R5 makes every one of those a per-walk derivation rather than
/// something the record holds.
///
/// **Counted by the store, and not read back whole.** Changed 2026-09-28, step
/// 10c's transfer wave piece 1. **The case this was first written for** was a
/// request of a few files, where reading every entry of every request back on
/// every ask cost nothing. **The case it now also meets** is a 10,000-file
/// upload or a 5,000-frame auto-download, whose every finished file tells the
/// page to ask again — and each ask composed every entry of every request back
/// into memory while the record's lock was held, to count them. **The row needs
/// the counts — files done, bytes done, the whole — and the store answers
/// them** (`every_request_counted_for`, `every_arrangement_counted_for`),
/// skipping the same records the whole reading skips.
pub(crate) fn the_requests_the_two_stores_hold(
    connection: &rusqlite::Connection,
    servers: &[AServerKnown],
) -> Vec<ARequestKnown> {
    the_requests_the_stores_hold_and_this_runs_undecided(connection, servers, None).0
}

/// **The same reading, and beside it the manual downloads this run holds
/// before their dispositions** — each as a request under way with nothing
/// moving, keyed by its identity. Step 10c's eleventh-pass fix piece 1.
///
/// **A request nothing has decided** (`AManualRequestCounted::nothing_is_decided`)
/// **and that this run opened** (`Download` M13's marker) is one this run is
/// still holding: its first read of the destination has not returned, or
/// `Download` M6's question is open. **One an earlier run opened is not**: M13,
/// closing Smoozy ended it, and it reads as ended, with nothing staged.
fn the_requests_the_stores_hold_and_this_runs_undecided(
    connection: &rusqlite::Connection,
    servers: &[AServerKnown],
    this_run: Option<&Identity>,
) -> (Vec<ARequestKnown>, HashMap<String, ARequestUnderWay>) {
    let mut requests = Vec::new();
    let mut undecided = HashMap::new();
    for server in servers {
        let Some(row) = Identity::read_back(IdentityKind::LocalServer, &server.row) else {
            continue;
        };
        let arrangements = DownloadArrangements::over(connection)
            .every_arrangement_counted_for(&row)
            .unwrap_or_default();
        for arrangement in arrangements {
            requests.push(an_arrangement_known(connection, &server.row, &arrangement));
        }
        let manual = DownloadRequests::over(connection)
            .every_request_counted_for(&row)
            .unwrap_or_default();
        for request in manual {
            if request.nothing_is_decided && this_run == Some(&request.opened_by_run) {
                undecided.insert(
                    request.record.as_text(),
                    ARequestUnderWay {
                        server: server.row.clone(),
                        ..ARequestUnderWay::default()
                    },
                );
            }
            let mut known = a_manual_request_known(&server.row, &request);
            let counting_name: Option<String> = connection.query_row(
                "SELECT counting_name FROM download_request WHERE record = ?1", [request.record.as_text()], |r| r.get(0),
            ).ok().flatten();
            if let Some(named) = counting_name {
                known.of = WhatItIsOf::AFolderBeingCounted { named };
                known.stands = WhereTheRequestStands::Running;
            }
            requests.push(known);
        }
        // **`File Browser`'s upload requests, which this screen never read.**
        // `Design/Transfers`: *the one window holding every upload and
        // download*; until 2026-09-27 an upload ran, and failed, while this
        // screen said *Nothing is uploading or downloading.* (the seventh pass
        // on Hyperstack, fault 6).
        let uploads = crate::record::upload_requests::UploadRequests::over(connection)
            .every_request_counted_for(&row)
            .unwrap_or_default();
        for request in uploads {
            // **An upload nothing has decided that this run holds before its
            // answer** — its walk under way or its question open — **is under
            // way with nothing moving**, as a manual download is above; one
            // this run does not hold has ended without sending anything. Step
            // 10c's check-before-0.1.7 fix piece 1.
            if request.nothing_is_decided
                && crate::uploading::this_run_holds_it_before_its_answer(&request.record)
            {
                undecided.insert(
                    request.record.as_text(),
                    ARequestUnderWay {
                        server: server.row.clone(),
                        ..ARequestUnderWay::default()
                    },
                );
            }
            requests.push(an_upload_request_known(&server.row, &request));
        }
    }
    requests.extend(crate::record::dependency_records::dependency_sends_counted(connection)
        .unwrap_or_default().into_iter()
        .filter(|send| servers.iter().any(|server| server.row == send.server))
        .map(|send| {
            let mut known = a_dependency_send_known(crate::the_found_files_sent::ASend {
                request: send.request, server: send.server, file: send.blend, files: Vec::new(),
            });
            known.files_done = send.published;
            known.files_in_the_whole = send.files;
            known.bytes = Some((send.published_bytes, send.bytes));
            known.stands = where_a_request_stands(send.all_ended, send.failed);
            known
        }));
    for request in &mut requests {
        let (table, key) = match request.kind {
            TheKind::Upload => ("upload_request", "record"),
            TheKind::Download => ("download_request", "record"),
            TheKind::AutoDownload => ("download_arrangement", "job"),
            TheKind::Dependencies => ("dependency_upload_request", "request"),
        };
        request.start = crate::record::transfer_records::parent_start(connection, table, key, &request.id).ok().flatten();
    }
    (requests, undecided)
}

/// **What this run is moving, read against every upload and manual download
/// row** — step 10c's transfer wave piece 5, added 2026-09-29.
///
/// **A request the record holds with a unit still owed is one of two things,
/// and the record alone cannot say which**: one this run is moving now, or one
/// nothing will ever move again — it ended with an earlier run, with its
/// server's period (`File Browser` U7's paragraph of 2026-09-28, `Download`
/// M13's), or its own run returned with files not across. **The run's register
/// says which** ([`crate::downloading::WhatIsMovingNow`]).
///
/// - **Under way**: the bar is the finished files' sizes plus the bytes staged
///   by the files in flight, against the whole (`File Browser` U11, `Download`
///   M16, `Server protocol and transfers` G4 and G6), and the files in flight
///   are drawn with their own bars, then the next few waiting their turn
///   (`Design/Transfers § A request shows the whole and the files under it`).
///   Each file's report carries `Server protocol and transfers` G4a's mark and
///   `Server protocol and transfers` G4b's, which the file's row already reads.
/// - **Not under way**: it composes as ended — [`WhereTheRequestStands::Ended`],
///   no Stop, no bar, the rail not counting it, and *— did not finish* on its
///   name — the drawing's own words for a request that stopped with files
///   owed (`Preparation/Design/Mockup/Smoozy.html`, the *t-left* row). A row
///   reading *running* with nothing alive behind it says something that is
///   not true, which Rule 8 makes a fault whatever caused it.
///
/// **The case the row was first written for** was a request whose run was
/// still going whenever the screen was asked, so *a unit is owed* and *it is
/// under way* were the same fact. **The case it now also meets** is a request
/// whose line was cut: its run returned with two files owed, and the row went
/// on offering Stop, drawing a bar and counting on the rail — for ever, since
/// nothing resumes on a period-opening connection (`File Browser` K3).
///
/// **The bar never passes the whole**: a file whose outcome has gone down and
/// that the register has not yet let go of would otherwise be counted twice for
/// a moment.
pub(crate) fn what_this_run_is_moving(
    requests: &mut [ARequestKnown],
    under_way: &HashMap<String, ARequestUnderWay>,
) {
    for request in requests.iter_mut() {
        if !matches!(request.kind, TheKind::Upload | TheKind::Download | TheKind::Dependencies)
            || request.stands != WhereTheRequestStands::Running
        {
            continue;
        }
        let Some(moving) = under_way.get(&request.id) else {
            request.stands = WhereTheRequestStands::Ended;
            continue;
        };
        // **Its server's period has ended, and the request with it** (`File
        // Browser` U7's paragraph of 2026-09-28, `Download` M13's) — its run
        // may take a moment more to return, and the row does not wait for it.
        // Step 10c's transfer wave piece 6.
        if moving.its_period_ended {
            request.stands = WhereTheRequestStands::Ended;
            continue;
        }
        request.waits_for_the_connection = moving.waits_for_the_connection;
        if let Some((done, whole)) = request.bytes {
            let in_flight: u64 = moving.moving.iter().map(|file| file.report.bytes_staged).sum();
            request.bytes = Some((done.saturating_add(in_flight).min(whole), whole));
        }
        let mut files: Vec<AFileKnown> = moving
            .moving
            .iter()
            .map(|file| AFileKnown {
                path: String::from_utf8_lossy(&file.path).into_owned(),
                // **A file the line cut and that waits for the connection
                // reads its record's *interrupted*** — `File Browser` U5 and
                // `Download` M11: record interrupted, request open, period
                // standing. `Download` M11a's composition then draws it as the
                // partial file it is, its figure where it stood and *uploaded*
                // (or *downloaded*) beside it, or as a start-over where its mark
                // says so.
                record_ending: file.waits_for_the_connection.then_some(Ending::Interrupted),
                report: file.report.clone(),
                happened: WhatHappenedToTheFile::Moving,
            })
            .collect();
        files.extend(moving.waiting.iter().map(|path| AFileKnown {
            path: String::from_utf8_lossy(path).into_owned(),
            record_ending: None,
            report: Report {
                bytes_staged: 0,
                bytes_expected: 0,
                started_over: None,
                verifying_a_prefix_of: None,
            },
            happened: WhatHappenedToTheFile::WaitingItsTurn,
        }));
        files.append(&mut request.files);
        request.files = files;
    }
}

/// **`File Browser` U8's first bullet and `Download` M13's second: an ended
/// request whose part-sent files are still staged is told as a leftover** —
/// step 10c's transfer wave piece 7, added 2026-09-29.
///
/// **Over every upload and manual download that reads ended**
/// ([`what_this_run_is_moving`]): its transfer records are read, and where any
/// reads *interrupted* under its server's current installation, the request is
/// [`WhereTheRequestStands::LeftOver`] with *Remove*; where none does but an
/// owed cleanup stands (`Server protocol and transfers` S27), it is a leftover
/// with no control — the files are still there, and a *Remove* could do no
/// more. **Whatever ended it**: an earlier sitting's close, or this run's
/// period ending under `File Browser` U7's paragraph of 2026-09-28.
///
/// **Read once per request per run and kept in the run's register**
/// ([`crate::downloading::WhatARequestLeft`]), because the reading that carries
/// an entry's transfer identity is the server's requests read whole — the cost
/// step 10c's transfer wave piece 1 took off every Transfers ask. **A request
/// whose run has not yet returned is read fresh and not kept**, since its last
/// file's ending may not be written yet.
///
/// **The case the ended row was first written for** was a request nothing
/// would move again and that nothing was left of. **The case it now also
/// meets** is one whose last file the line cut: the record says *interrupted,
/// the part is kept for you*, the part sits on the server or in the user's own
/// folder, and the row said only *— did not finish*.
pub(crate) fn the_leftovers_are_told(
    connection: &rusqlite::Connection,
    requests: &mut [ARequestKnown],
    under_way: &HashMap<String, ARequestUnderWay>,
    the_run: Option<&TheRunsTransfers>,
    installations: &HashMap<String, Option<String>>,
) {
    use crate::downloading::WhatARequestLeft;
    for request in requests.iter_mut() {
        if request.stands != WhereTheRequestStands::Ended {
            continue;
        }
        let direction = match request.kind {
            TheKind::Upload | TheKind::Dependencies => Direction::Up,
            TheKind::Download => Direction::Down,
            _ => continue,
        };
        // **Why it ended, where this run ended it** — the Creator's seventh
        // notes, piece Z5b: its line cut and not repaired, or its server's
        // period ended under it, and not the user's *Stop*.
        request.its_period_ended_it = the_run.is_some_and(|the_run| the_run.its_period_ended_it(&request.id));
        let installation_now = installations.get(&request.server).cloned().flatten();
        let its_run_has_returned = !under_way.contains_key(&request.id);
        let kept = the_run
            .filter(|_| its_run_has_returned)
            .and_then(|the_run| the_run.what_it_left(&request.id))
            .filter(|left| left.under == installation_now);
        let left = match kept {
            Some(left) => left,
            None => {
                let (Some(identity), Some(row)) = (
                    Identity::read_back(IdentityKind::Operation, &request.id),
                    Identity::read_back(IdentityKind::LocalServer, &request.server),
                ) else {
                    continue;
                };
                let under_this_installation = |one: &smoozy_server_protocol::transfers::TransferRecord| {
                    one.installation.as_ref().map(Identity::as_text) == installation_now
                };
                // **A dependency send's records are named by the run's own
                // register of sends**, not by a request store (step 10c's
                // dependency wave piece 6).
                let records = if request.kind == TheKind::Dependencies {
                    crate::the_found_files_sent::the_transfers_a_send_left(connection, &request.id)
                } else {
                    crate::downloading::the_transfers_a_request_left(connection, &identity, &row, direction)
                };
                let left = WhatARequestLeft {
                    interrupted: records
                        .iter()
                        .filter(|one| one.ending == Some(Ending::Interrupted) && under_this_installation(one))
                        .count(),
                    owed: records
                        .iter()
                        .filter(|one| {
                            matches!(one.ending, Some(Ending::Stopped(_) | Ending::Failed | Ending::Stalled))
                                && one.cleanup_is_owed
                                && under_this_installation(one)
                        })
                        .count(),
                    under: installation_now.clone(),
                };
                if let Some(the_run) = the_run.filter(|_| its_run_has_returned) {
                    the_run.it_left(&request.id, left.clone());
                }
                left
            }
        };
        if left.interrupted == 0 && left.owed == 0 {
            continue;
        }
        request.stands = WhereTheRequestStands::LeftOver {
            // **An upload's part and a dependency send's are removed on the
            // machine by *Remove*** (`File Browser` U8 as amended 2026-10-03,
            // `Dependency Upload & Remap` A12; the 38's piece 9, item 33), so
            // a part whose cleanup is still owed keeps its *Remove* too —
            // until then the row said the files were using space on the
            // server and offered nothing. **A download's part is on this
            // computer** and is removed at once or owed for the sweep, so
            // only an *interrupted* one is offered.
            removable: left.interrupted > 0 || (direction == Direction::Up && left.owed > 0),
            by_an_earlier_run: the_run.is_some_and(|the_run| !the_run.it_ran_in_this_run(&request.id)),
        };
    }
}

/// **`Design/Transfers § Waiting for the connection to come back`: the line is
/// the same one the server's own row carries** — *Reconnecting…*, with when
/// the next attempt falls. Added 2026-09-29, step 10c's transfer wave piece 6.
///
/// **`servers_own_reading` is `Servers`' reading of one server and nothing
/// composed here**: the moment the next attempt falls where the server's own
/// row reads *Reconnecting…* (`Servers` K8a's *repairing*, and `Servers` N10a's
/// moment where one is published), and nothing where it does not — a server
/// `Servers` K8 has *restarting*, say, whose row says that instead and whose
/// words this screen does not have. **A request waiting for the connection
/// whose server reads anything else carries no server-repair line**; the
/// composer states its transfer's wait without claiming a primary repair.
pub(crate) fn the_repairs_the_servers_read(
    requests: &mut [ARequestKnown],
    servers_own_reading: &dyn Fn(&str) -> Option<Option<i64>>,
) {
    for request in requests.iter_mut() {
        if request.waits_for_the_connection && request.stands == WhereTheRequestStands::Running {
            request.reconnecting = servers_own_reading(&request.server);
        }
    }
}

/// **`Servers`' own reading of one server's repair, as its row takes it** —
/// the same two readings `src/surface/servers.rs` draws the row's
/// *Reconnecting…* from: `Servers` K8a's presentation, and `Servers` N10a's
/// moment. Step 10c's transfer wave piece 6.
fn what_the_servers_row_reads(
    periods: &crate::servers::a_period::ThePeriodsThisRun,
    server: &str,
) -> Option<Option<i64>> {
    use smoozy_servers::condition::{presentation, Presentation};
    if presentation(periods.what_is_true_of(server)) != Presentation::Repairing {
        return None;
    }
    Some(match periods.the_next_attempt_due_for(server) {
        smoozy_servers::repair::NextAttemptDue::At(moment) => moment
            .duration_since(std::time::UNIX_EPOCH)
            .ok()
            .map(|since| since.as_secs() as i64),
        smoozy_servers::repair::NextAttemptDue::Absent => None,
    })
}

/// **One upload request, as the screen reads it** — `File Browser` U11's
/// count in files and bytes over the units whose disposition is to send, and
/// **the files whose write the machine refused, each on its own row** with the
/// failure shape (`File Browser` P1 and `File Browser` P6a; `Design/File Browser`: *a refused
/// write on an upload surfaces on the transfer row*). **A refusal is known for
/// this run only** — [`crate::uploading::the_refusals_this_run`] says why — so a
/// later sitting's row counts the file as failed and draws no row for it.
fn an_upload_request_known(
    server: &str,
    request: &crate::record::upload_requests::AnUploadRequestCounted,
) -> ARequestKnown {
    // **A file that changed while it went up is on its own row too**, with
    // *Upload it again* (`File Browser` K8, `Design/Transfers § A file that
    // changed while it was going up`), read from its transfer record's own
    // ending — durable, so a later sitting draws it as well. The 38's piece 9,
    // item 17: until then such a file was counted as failed and drawn nowhere.
    let files = request
        .failed_landings
        .iter()
        .filter_map(|(landing, size)| {
            let happened = if crate::uploading::what_the_server_said_refusing(&request.record, landing).is_some() {
                WhatHappenedToTheFile::RefusedByTheServer
            } else if request.changed_landings.contains(landing) {
                WhatHappenedToTheFile::ChangedWhileUploading
            } else {
                return None;
            };
            Some(AFileKnown {
                path: String::from_utf8_lossy(landing).into_owned(),
                record_ending: None,
                report: Report {
                    bytes_staged: 0,
                    bytes_expected: *size,
                    started_over: None,
                    verifying_a_prefix_of: None,
                },
                happened,
            })
        })
        .collect();
    // **A user's own Blender build is named by its file** (step 10c's owed
    // piece 5), where this run sent it.
    let of = match crate::setting_up::the_build_this_upload_sends(&request.record.as_text()) {
        Some(file) => WhatItIsOf::ABuild { file },
        None => WhatItIsOf::AFolder {
            folder: String::from_utf8_lossy(&request.destination).into_owned(),
        },
    };
    ARequestKnown {
        start: None,
        id: request.record.as_text(),
        kind: TheKind::Upload,
        server: server.to_string(),
        of,
        files_done: request.files_done,
        files_in_the_whole: request.files_in_the_whole,
        bytes: Some((request.bytes_done, request.bytes_in_the_whole)),
        seconds_left: None,
        destination: the_stored_folder(&request.destination),
        narrowed_to: Vec::new(),
        reconnecting: None,
        waits_for_the_connection: false,
        its_period_ended_it: false,
        trouble: None,
        left_alone: 0,
        cannot_land: 0,
        may_be_incomplete: 0,
        split_between: None,
        the_job_ended: false,
        the_job_ended_short: false,
        the_arrangement_is_done: false,
        the_whole_was_read: false,
        // **A refused or failed file means it did not finish** — the one rule
        // a manual download keeps too ([`where_a_request_stands`]). Step 10c's
        // thirteenth-pass fix piece 3b.
        //
        // **And so does one stopped at `File Browser` U4's question**, which
        // sent nothing it was asked for. **The case this was first written
        // for** was a request whose every unit had an outcome; **the case it
        // now also meets** has no unit with a disposition at all, so *every
        // unit has ended* held of none and the row read *done*. **A request
        // nothing decided is not ended by its units** (the count), so it reads
        // under way while this run holds it before its answer and ended
        // otherwise ([`what_this_run_is_moving`]) — the Hyperstack check before
        // 0.1.7 saw *0 files into /home/ubuntu · done* for one whose walk was
        // lent nothing. The download's own sentence (`Design/Transfers § A
        // file that failed`, 2026-10-01): what the user asked for and did not
        // get never reads *done*. Step 10c's check-before-0.1.7 fix piece 1.
        stands: where_a_request_stands(
            request.every_unit_has_ended,
            !request.failed_landings.is_empty() || request.stopped_at_the_question,
        ),
        files,
    }
}

/// **Where a request stands from its units: still going, ended with every
/// file across, or ended with a file that failed** — one rule for an upload, a
/// dependency check's send and a manual download. Rule 8, `Design/Transfers §
/// A file that failed`: *"A request with a file that failed never reads done;
/// it reads — did not finish with its count."*
/// [`WhereTheRequestStands::Ended`] draws exactly that.
///
/// **The case it was first written for** (step 10c's thirteenth-pass fix piece
/// 3) was a manual download whose *Replace* wrote nothing and read *done*.
/// **Piece 3b brought the upload and the send under it**: an upload with a
/// refused file read *1 file into / · done* above its own red row.
fn where_a_request_stands(every_unit_has_ended: bool, a_file_failed: bool) -> WhereTheRequestStands {
    match (every_unit_has_ended, a_file_failed) {
        (true, false) => WhereTheRequestStands::Finished { at: String::new() },
        (true, true) => WhereTheRequestStands::Ended,
        (false, _) => WhereTheRequestStands::Running,
    }
}

/// **The dependency check's sends, as the screen reads them** —
/// `Design/Transfers § The dependency check's own transfers`: *"Its rows are
/// the same rows"*, with the *Dependencies* tag, named after the `.blend`, and
/// *Open the check* as the way back. Step 10c's dependency wave piece 6.
///
/// **Read from the run's register of sends** (`crate::the_found_files_sent`),
/// which is run-held as every request's activity is (`File Browser` U7); the
/// transfer records beneath it are durable. **Counted the way an upload's row
/// is** (`File Browser` U11): files and bytes over the files the send
/// arranged, and **the files the machine refused, each on its own row**, where
/// this run holds the machine's words.
pub fn the_requests_the_sends_hold(servers: &[AServerKnown]) -> Vec<ARequestKnown> {
    let sends = match crate::the_found_files_sent::the_sends_this_run().lock() {
        Ok(sends) => sends.clone(),
        Err(poisoned) => poisoned.into_inner().clone(),
    };
    sends
        .into_iter()
        .filter(|send| servers.iter().any(|server| server.row == send.server))
        .map(a_dependency_send_known)
        .collect()
}

fn a_dependency_send_known(send: crate::the_found_files_sent::ASend) -> ARequestKnown {
    use crate::the_found_files_sent::HowItEnded;
    let published = |file: &&crate::the_found_files_sent::AFileSent| {
        file.ended == Some(HowItEnded::Published)
    };
    let files = send
        .files
        .iter()
        .filter(|file| file.ended == Some(HowItEnded::Refused))
        .filter(|file| {
            crate::uploading::what_the_server_said_refusing(&send.request, &file.landing).is_some()
        })
        .map(|file| AFileKnown {
            path: String::from_utf8_lossy(&file.landing).into_owned(),
            record_ending: None,
            report: Report {
                bytes_staged: 0,
                bytes_expected: file.size,
                started_over: None,
                verifying_a_prefix_of: None,
            },
            happened: WhatHappenedToTheFile::RefusedByTheServer,
        })
        .collect();
    let every_file_has_ended = send
        .files
        .iter()
        .all(|file| matches!(file.ended, Some(HowItEnded::Published | HowItEnded::Refused | HowItEnded::Failed)));
    ARequestKnown {
        start: None,
        id: send.request.as_text(),
        kind: TheKind::Dependencies,
        server: send.server.clone(),
        of: WhatItIsOf::ABlend { file: send.file.clone() },
        files_done: send.files.iter().filter(published).count(),
        files_in_the_whole: send.files.len(),
        bytes: Some((
            send.files.iter().filter(published).map(|file| file.size).sum(),
            send.files.iter().map(|file| file.size).sum(),
        )),
        seconds_left: None,
        destination: PathBuf::new(),
        narrowed_to: Vec::new(),
        reconnecting: None,
        waits_for_the_connection: false,
        its_period_ended_it: false,
        trouble: None,
        left_alone: 0,
        cannot_land: 0,
        may_be_incomplete: 0,
        split_between: None,
        the_job_ended: false,
        the_job_ended_short: false,
        the_arrangement_is_done: false,
        the_whole_was_read: false,
        // **A refused or failed file means it did not finish** —
        // step 10c's thirteenth-pass fix piece 3b.
        stands: where_a_request_stands(
            every_file_has_ended,
            send.files
                .iter()
                .any(|file| matches!(file.ended, Some(HowItEnded::Refused | HowItEnded::Failed))),
        ),
        files,
    }

}

/// **`Download` N6's whole, read against every auto-download row where a pass
/// in this run has had the job's inventory in hand** — the narrowed inventory's
/// count, which the passes hand to the run
/// (`crate::servers::the_machines_work::WhatThisRunHolds::the_whole_of`).
/// **Where no pass in this run has, the row keeps the reckoning's length**,
/// which [`an_arrangement_known`] says is the honest figure then.
///
/// **The case the row was first written for** was a job whose reckoning had
/// caught up with its inventory, so the two figures agreed. **The case it now
/// also meets** is a job whose frames come home while it renders: the
/// reckoning holds only the files acted on so far, and the tenth pass on
/// Hyperstack read *6 / 73* and *36 / 188* for a 240-frame job — `Specs/Download`
/// promises the user can see how much has come home *"against the whole"*.
///
/// **Never below the reckoning's own length**: a file the arrangement acted on
/// outside the inventory (`Download` J8's unmatched file, or a video's split
/// file the statement allowed for) is a file on the row, and a whole smaller
/// than the files already counted would read as more than all of it. **The
/// video statement has no words of its own on this screen**, so a whole that
/// may grow is drawn as its count and no word is added (`Design/Transfers`
/// draws none). Added 2026-09-29, step 10c's transfer wave piece 4.
pub(crate) fn the_wholes_this_run_has_read(
    requests: &mut [ARequestKnown],
    the_whole_of: &dyn Fn(&str) -> Option<crate::servers::the_machines_work::TheJobsWhole>,
) {
    for request in requests.iter_mut() {
        if request.kind != TheKind::AutoDownload {
            continue;
        }
        let WhatItIsOf::AJob { job, .. } = &request.of else {
            continue;
        };
        if let Some(whole) = the_whole_of(job) {
            // **Once the job has ended short of a clean finish, the whole is
            // what the ending's walk found, since no more will come** —
            // `Download` N6's last sentence — and never the asked range's
            // count. The reckoning's own length is that, the ending walk
            // having entered what it found and nothing it did not
            // (`smoozy_download::acting::running`). *(Until 2026-10-19 the
            // larger of the two stood here for every row, so a job stopped at
            // 52 of 150 frames read* ended 100 / 450*.)*
            if !request.the_job_ended_short {
                request.files_in_the_whole = request.files_in_the_whole.max(
                    whole.files.saturating_sub(request.left_alone).saturating_sub(request.cannot_land)
                );
            }
            request.the_whole_was_read = true;
        }
    }
}

/// **One arrangement, as the screen reads it** —
/// `Design/Transfers § An auto-download is not a request, and reads
/// differently`: *Turn off*, files rather than bytes, *ended* and never
/// *complete*.
///
/// **The whole is the reckoning's own length here, and that is stated rather
/// than smoothed.** `Download` N6's whole is the narrowed inventory's count, which
/// lives on the machine and is a reading rather than a durable value
/// (`Download` R5) — so until a walk has run in this application run, the
/// figure the record can answer with is how many files the arrangement has
/// acted on. **It understates the whole and never overstates it**, which is the
/// direction that reads as *still going* rather than as *finished short*.
/// **Once a pass in this run has read the whole, [`the_wholes_this_run_has_read`]
/// puts it in place** (step 10c's transfer wave piece 4).
fn an_arrangement_known(
    connection: &rusqlite::Connection,
    server: &str,
    arrangement: &crate::record::download_records::AnArrangementCounted,
) -> ARequestKnown {
    let record = crate::record::job_records::JobRecords::over(connection)
        .read(&arrangement.job)
        .ok()
        .flatten();
    // **Named as Queue and History name it** —
    // `crate::surface::the_queue::what_the_job_is_called`: the label, else the
    // file's name (`Design/Queue & History § A rendering entry`, Creator-stated:
    // *"Where the job carries no label of its own, the name is the file"*).
    // **The case this was first written for** was a job the user named. **The
    // case it now also meets** is one sent with no name, which read as its
    // identity here (step 10c's Vast-pass fix piece 1, at the runner's word
    // after piece 2). The identity stands only where the record will not read.
    let name = record
        .as_ref()
        .map(crate::surface::the_queue::what_the_job_is_called)
        .unwrap_or_else(|| arrangement.job.as_text());
    let the_job_ended = record
        .as_ref()
        .is_some_and(|held| held.the_ending_learned().is_some());
    let the_job_ended_short = record.as_ref().is_some_and(|held| {
        held.the_ending_learned()
            .is_some_and(|ending| ending != smoozy_queue_history::ending::Ending::Finished)
    });
    // `Download` E1's unit-outcome half. The live composer also requires the ending
    // walk's proof, including an empty walk, before this can mean done.
    let the_arrangement_is_done =
        the_job_ended && arrangement.owed == 0;
    ARequestKnown {
        start: None,
        id: arrangement.job.as_text(),
        kind: TheKind::AutoDownload,
        server: server.to_string(),
        of: WhatItIsOf::AJob {
            name,
            job: arrangement.job.as_text(),
        },
        files_done: arrangement.fetched,
        // `Download` N7 excludes narrowing skips, left-alone and cannot-land files.
        // Never-coming files remain in the count as the honest shortfall.
        files_in_the_whole: arrangement.entries
            .saturating_sub(arrangement.skipped_by_narrowing)
            .saturating_sub(arrangement.left_alone)
            .saturating_sub(arrangement.cannot_land),
        // **`Download` N8: an auto-download has no byte whole**, so the bar is
        // files and the figure is never bytes.
        bytes: None,
        seconds_left: None,
        destination: the_stored_folder(&arrangement.destination),
        // **Each format in Smoozy's words** (Rule 9), from the one table the
        // box reads — the Creator's second notes, piece 2.
        narrowed_to: match &arrangement.narrowing {
            TheNarrowing::Everything => Vec::new(),
            TheNarrowing::TheseFormats(formats) => {
                formats.iter().map(|format| a_format_in_smoozys_words(format)).collect()
            }
        },
        reconnecting: None,
        waits_for_the_connection: false,
        its_period_ended_it: false,
        trouble: None,
        left_alone: arrangement.left_alone,
        cannot_land: arrangement.cannot_land,
        may_be_incomplete: 0,
        split_between: None,
        the_job_ended,
        the_job_ended_short,
        the_arrangement_is_done,
        the_whole_was_read: false,
        // **`Download` R11: switching it off ends the arranging of new fetches
        // and removes nothing**, so an arrangement that is off has nothing left
        // to stop.
        stands: match arrangement.desired_state {
            DesiredState::On => WhereTheRequestStands::Running,
            DesiredState::Off => WhereTheRequestStands::Finished {
                at: String::new(),
            },
        },
        files: Vec::new(),
    }
}

/// **One manual request, as the screen reads it** — `Design/Transfers § A
/// request shows the whole and the files under it`.
///
/// **`Download` M13: closing Smoozy ends the request.** A record opened by an
/// earlier run is a leftover and reads as one; nothing resumes it.
///
/// **M16's two totals**: the file count is unit outcomes over the units a
/// transfer is arranged for, and the byte whole is those units' enumerated
/// sizes. **Skipped, refused, left-alone and cannot-land units are in neither
/// and are reported separately.**
///
/// **A request nothing has decided yet is running, over its enumeration** —
/// the store counts it so (step 10c's eleventh-pass fix piece 1), and it is
/// never *done*: `Download` M16's two totals exist before the first transfer
/// starts, and none has started. Whether this run still holds it, or an
/// earlier run left it, is [`what_this_run_is_moving`]'s to say.
///
/// **A request with a file that failed never reads *done*** — Rule 8,
/// `Design/Transfers § A file that failed` with E2-1: once every unit has an
/// outcome and one of them failed, it is [`WhereTheRequestStands::Ended`],
/// which draws *— did not finish* with its count, and **each failed file is on
/// its own row**. Step 10c's thirteenth-pass fix piece 3.
///
/// **The case this was first written for** was a request whose every file came
/// down, so *every unit has an outcome* and *it finished* were one fact. **The
/// case it now also meets** is a file this computer would not take — on the
/// thirteenth pass a *Replace* whose publication met the old file and refused —
/// where the record said *failed* and the row said *done*, with no reason
/// anywhere.
fn a_manual_request_known(
    server: &str,
    request: &crate::record::download_records::AManualRequestCounted,
) -> ARequestKnown {
    let id = request.record.as_text();
    let files = request
        .failed_landings
        .iter()
        .map(|(landing, size)| AFileKnown {
            path: String::from_utf8_lossy(landing).into_owned(),
            record_ending: None,
            report: Report {
                bytes_staged: 0,
                bytes_expected: *size,
                started_over: None,
                verifying_a_prefix_of: None,
            },
            happened: WhatHappenedToTheFile::NotPutInPlace {
                has_details: crate::downloading::what_this_computer_said_for(&id, landing).is_some(),
            },
        })
        .collect();
    ARequestKnown {
        start: None,
        id,
        kind: TheKind::Download,
        server: server.to_string(),
        of: WhatItIsOf::AFolder {
            folder: the_stored_folder(&request.destination)
                .display()
                .to_string(),
        },
        files_done: request.files_done,
        files_in_the_whole: request.files_in_the_whole,
        bytes: Some((request.bytes_done, request.bytes_in_the_whole)),
        seconds_left: None,
        destination: the_stored_folder(&request.destination),
        narrowed_to: Vec::new(),
        reconnecting: None,
        waits_for_the_connection: false,
        its_period_ended_it: false,
        trouble: None,
        left_alone: 0,
        cannot_land: request.cannot_land,
        may_be_incomplete: 0,
        split_between: None,
        the_job_ended: false,
        the_job_ended_short: false,
        the_arrangement_is_done: false,
        the_whole_was_read: false,
        // **What the user asked for and did not get never reads *done***:
        // a file that failed, a request stopped at `Download` M6's question,
        // or something on the server it could not read (M4) — `Design/Transfers
        // § A file that failed`, the sentence of 2026-10-01. **Why the last two
        // escaped pieces 3 and 3b**: their rule is keyed on a failed unit, and
        // neither has a unit, so *every unit has ended* held of none. Step
        // 10c's Vast-pass fix piece 1.
        stands: where_a_request_stands(
            request.every_unit_has_ended,
            !request.failed_landings.is_empty()
                || request.stopped_at_the_question
                || request.could_not_be_read > 0,
        ),
        files,
    }
}

// ════════════════════════════════════════════════════════════════════════════
// The commands
// ════════════════════════════════════════════════════════════════════════════

/// **The whole screen.** The page asking for the facts it needs before it can
/// draw anything — the same shape every other screen's own reading has.
///
/// **`async`, so it reads the record off the thread that draws** (P4, P5).
/// Changed 2026-09-28, step 10c's transfer wave piece 1. It takes the
/// application handle and reaches the state inside, the way
/// `crate::surface::files::the_folder_is_read` does. **The case it was first
/// written for** is a record whose lock is held for a moment at a time.
/// **The case it now also meets** is a record a transfer is writing, where
/// this screen is the one the page asks again at every file's end: the tester
/// measured a reading waiting 27, 28 and 41 seconds for the lock near the end
/// of a 4 GB upload, and a plain command runs on the thread that draws, so
/// that wait was the window frozen. The page code does not change.
#[tauri::command]
pub async fn the_transfers_screen<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    showing: Option<String>,
) -> TheTransfersScreen {
    use tauri::Manager as _;
    let the_run = app
        .try_state::<Arc<TheRunsTransfers>>()
        .map(|the_run| Arc::clone(the_run.inner()));
    let mut known = match app.try_state::<Mutex<Started>>() {
        Some(started) => match started.lock() {
            // **What the run is moving is read while the record is held**,
            // so a file whose outcome goes down in the same hold is counted
            // once (step 10c's transfer wave piece 5).
            Ok(started) => what_is_known_now(
                &started,
                &the_run
                    .as_ref()
                    .map(|the_run| the_run.what_is_under_way())
                    .unwrap_or_default(),
                the_run.as_deref(),
            ),
            // **A lock that would not open is a read that failed**, said as
            // one and never drawn as *nothing is moving* (must-do 14).
            Err(_) => WhatIsKnown::nothing_would_read(),
        },
        None => WhatIsKnown::nothing_would_read(),
    };
    // **The live reading beside the cap**, taken at the moment the screen is
    // asked for. **R15a: a published reading is held and never aged** — this
    // draws whatever the accounting point last counted and runs nothing down
    // between askings, which is what P10b forbids a drawing to do with
    // anything but a moment.
    if let Some(transfers) = the_run.as_deref() {
        (known.the_speed_up, known.the_speed_down) = the_live_readings(transfers);
    }
    // **`Download` N6's whole, where a pass in this run has read it** — step
    // 10c's transfer wave piece 4.
    if let Some(periods) = app.try_state::<Arc<crate::servers::a_period::ThePeriodsThisRun>>() {
        let holds = periods.the_machines_work();
        the_wholes_this_run_has_read(&mut known.requests, &|job| holds.the_whole_of(job));
        // **A request waiting for its connection carries the line its
        // server's own row carries** — step 10c's transfer wave piece 6.
        the_repairs_the_servers_read(&mut known.requests, &|server| {
            what_the_servers_row_reads(&periods, server)
        });
    }
    let showing = showing.filter(|row| !row.is_empty());
    the_screen_from(&known, showing.as_deref())
}

/// **The stop box, composed at the moment Stop is pressed.**
#[tauri::command]
///
/// **It names the files in flight from what the run is moving**, read while
/// the record is held (step 10c's transfer wave piece 5), and a request nothing
/// is moving has nothing to stop.
pub fn the_stop_box(
    started: State<'_, Mutex<Started>>,
    the_run: State<'_, Arc<TheRunsTransfers>>,
    request: String,
) -> TheStopAnswer {
    let Ok(started) = started.lock() else {
        return TheStopAnswer::NothingToStop;
    };
    match what_is_known_now(&started, &the_run.what_is_under_way(), Some(&the_run))
        .requests
        .iter()
        .find(|one| one.id == request)
    {
        Some(found) => the_stop_box_for(found),
        None => TheStopAnswer::NothingToStop,
    }
}

/// **One of the three format boxes, for a job after Submit** — the queue
/// entry's narrowing control and its switch turned on (`Queue & History` L1a;
/// `Download` R1, `Download` R11, `Download` N1 to `Download` N3). Composed
/// from the job's recorded reading,
/// read over the connection where the server is connected, and from what the
/// arrangement already holds ([`the_jobs_box_is_read`]). The 38's piece 9,
/// item 9: until then it answered `Design`'s *not finished* words.
///
/// **The read is never offered here** — a submitted job's file is read on the
/// machine at render start (`Download` N2), so the box's *Read it now* is drawn
/// unavailable.
#[tauri::command]
pub async fn the_format_box<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    job: String,
) -> crate::surface::render_settings::TheDownloadBox {
    let the_box = match Identity::read_back(IdentityKind::Job, &job) {
        Some(job) => match the_jobs_box_is_read(&app, &job).await {
            Ok(read) => TheFormatAnswer::TheBox(the_jobs_box_is_composed(&read)),
            Err(said) => TheFormatAnswer::TheActIsNotBuiltYet {
                said,
                close: Prose::new(words::CANCEL),
            },
        },
        None => TheFormatAnswer::TheActIsNotBuiltYet {
            said: Prose::new(words::THE_JOB_WOULD_NOT_READ),
            close: Prose::new(words::CANCEL),
        },
    };
    crate::surface::render_settings::TheDownloadBox {
        the_box,
        the_read_is_offered: false,
    }
}

/// **An act on a request or on one of its files.** The answer arrives as an
/// event (P6).
///
/// **`async`, so what an act writes is written off the thread that draws**
/// (`Desktop app and delivery` P4, P5) — a *Remove* writes each of a leftover's
/// records and removes what it can. Changed 2026-09-29, step 10c's transfer
/// wave piece 7, which is also when the three acts below began to act.
///
/// - **Stop, on the request or on one file** — `Server protocol and transfers`
///   C22: desktop-local, it closes the channel and stops the loop. The stop is
///   marked in the run's register ([`TheRunsTransfers::the_user_stops`]) and
///   the request's own run performs it. `file` is the landing path the file's
///   row names.
/// - **Remove, on a leftover** — `File Browser` U8's first bullet and
///   `Download` M13's second ([`crate::downloading::a_leftover_is_removed`]).
///
/// **None of the three answers in prose**: the page is told a transfer moved,
/// asks for the screen again, and the row says what became of the request.
#[tauri::command]
pub async fn an_act_on_transfers<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    act: TheAct,
    request: String,
    file: Option<String>,
) {
    use tauri::Manager as _;
    let the_run = app
        .try_state::<Arc<TheRunsTransfers>>()
        .map(|the_run| Arc::clone(the_run.inner()));
    let known = match app.try_state::<Mutex<Started>>() {
        Some(started) => match started.lock() {
            Ok(started) => what_is_known_now(
                &started,
                &the_run
                    .as_ref()
                    .map(|the_run| the_run.what_is_under_way())
                    .unwrap_or_default(),
                the_run.as_deref(),
            ),
            Err(_) => WhatIsKnown::default(),
        },
        None => WhatIsKnown::default(),
    };
    let said = match act {
        // **`Download` R3's switch-off is a write and is taken where the
        // record is in hand.**
        TheAct::TurnOff => match app.try_state::<Mutex<Started>>() {
            Some(started) => the_switch_off_is_taken(started.inner(), &request),
            None => Some(Prose::new(THE_SWITCH_DID_NOT_SAVE)),
        },
        TheAct::StopTheRequest | TheAct::StopOneFile => {
            if let Some(the_run) = the_run.as_deref() {
                let door = TheApplicationsRecord { app: app.clone() };
                a_stop_is_taken_wherever_the_request_stands(the_run, &known, &door, act, &request, file.as_deref());
            }
            the_transfers_moved(&app);
            None
        }
        TheAct::Remove => {
            let mut said = None;
            if let Some(the_run) = the_run.as_deref() {
                let door = TheApplicationsRecord { app: app.clone() };
                said = match known.requests.iter().find(|one| one.id == request) {
                    // **An upload's part is on the machine** (`File Browser`
                    // `File Browser` U8 as amended 2026-10-03), so its *Remove* needs the
                    // server's own point — and where none is live, it says
                    // so and changes nothing, so the row keeps its *Remove*.
                    Some(found) if found.kind.goes_up() => match the_live_point_of(&app, &found.server) {
                        Some(point) => {
                            the_part_on_the_machine_is_removed(&known, &door, the_run, &point, &request).await;
                            None
                        }
                        None => Some(Prose::new(words::NOT_CONNECTED_SO_NOT_REMOVED)),
                    },
                    _ => {
                        the_leftover_is_removed(&known, &door, the_run, &request);
                        None
                    }
                };
            }
            the_transfers_moved(&app);
            said
        }
        TheAct::Details => the_details_of(&request, file.as_deref()),
        // **`Download` H7's paragraph of 2026-10-03: what it brought is shown
        // selected**, read off the request's own entry records.
        TheAct::FilesOnYourComputer => {
            let held = app.try_state::<Mutex<Started>>().and_then(|started| {
                let found = known.requests.iter().find(|one| one.id == request)?;
                let started = started.lock().ok()?;
                the_request_held(started.record.as_ref()?.connection(), &found.server, &request)
            });
            the_files_are_shown(&known, &request, held.as_ref())
        }
        _ => what_an_act_answers(&known, act, &request),
    };
    if let Some(queue) = app.try_state::<Arc<TheOneQueue>>() {
        let event = an_act_said(&request, said);
        let _ = queue.read_then_post::<Event, std::convert::Infallible>(
            || Ok(event.clone()),
            |event| event.clone(),
        );
    }
}

/// **The user's *Stop*, marked** — on the whole request, or on the one file
/// `file` names. `Server protocol and transfers` C22, `File Browser` U6,
/// `Download` M12; `Design/Transfers § What Stop says before it does it` and
/// `§ Stopping one file`. Step 10c's transfer wave piece 7.
pub(crate) fn the_stop_is_taken(the_run: &TheRunsTransfers, act: TheAct, request: &str, file: Option<&str>) -> bool {
    match (act, file) {
        (TheAct::StopTheRequest, _) => the_run.the_user_stops(request, None),
        (TheAct::StopOneFile, Some(file)) => the_run.the_user_stops(request, Some(file.as_bytes())),
        _ => false,
    }
}

/// **A Stop, taken wherever the request stands**: in the run's register where
/// it is moving ([`the_stop_is_taken`]), and as `Download` M6's own *Stop*
/// where it is a manual download this run holds before its dispositions
/// ([`a_download_not_yet_decided_is_stopped`]) — step 10c's eleventh-pass fix
/// piece 1.
pub(crate) fn a_stop_is_taken_wherever_the_request_stands(
    the_run: &TheRunsTransfers,
    known: &WhatIsKnown,
    door: &dyn crate::downloading::TheRecordADownloadWrites,
    act: TheAct,
    request: &str,
    file: Option<&str>,
) {
    // **A count under way is stopped where it stands** — step 10c's
    // twelfth-pass fix piece 3. **The stop is also held as one before the
    // request's dispositions**, for the one moment a count can have finished
    // and written its request before it looks: the request is then answered
    // M6's own *Stop* as it returns ([`what_a_stop_before_its_dispositions_answers`]),
    // and a count that did see it lets the hold go
    // ([`the_request_is_put_to_the_user`]).
    if act == TheAct::StopTheRequest && the_run.the_user_stops_the_count(request) {
        if let Ok(mut stopped) = downloads_stopped_before_their_dispositions().lock() {
            stopped.insert(request.to_string());
        }
        return;
    }
    let taken = the_stop_is_taken(the_run, act, request, file);
    if !taken && act == TheAct::StopTheRequest {
        a_download_not_yet_decided_is_stopped(known, door, request);
    }
}

/// **The request's *Stop*, on a manual download this run holds before its
/// dispositions** — step 10c's eleventh-pass fix piece 1. `Download` M12:
/// *"unarranged files are never arranged"*, and here none is arranged yet;
/// `Design/Transfers § What Stop says before it does it`: *"The request is
/// still Smoozy's to stop even while nothing is crossing."*
///
/// **The row reads under way and offers Stop** ([`what_is_known_from`]), and
/// the run's register does not hold it yet, so [`the_stop_is_taken`] reaches
/// nothing. **The stop is written as `Download` M6's own *Stop***, the answer
/// that is *"the request not proceeding at all"*: at once where M6's question
/// is open, and where the request is still in its first read of the
/// destination, the moment that read returns
/// ([`the_request_is_put_to_the_user`]). **No word is added**: the row then
/// reads what a request answered *Stop* at M6 reads.
fn a_download_not_yet_decided_is_stopped(
    known: &WhatIsKnown,
    door: &dyn crate::downloading::TheRecordADownloadWrites,
    request: &str,
) {
    let Some(found) = known.requests.iter().find(|one| one.id == request) else {
        return;
    };
    if found.kind != TheKind::Download || found.stands != WhereTheRequestStands::Running {
        return;
    }
    let asked = downloads_waiting_on_their_question().lock().ok().and_then(|mut waiting| {
        waiting
            .iter()
            .position(|(id, _, _)| id == request)
            .map(|at| waiting.remove(at))
    });
    match asked {
        Some((_, _, mut ready)) => {
            let _ = crate::downloading::the_answer_is_applied(&mut ready, door, CollisionAnswer::Stop);
        }
        None => {
            if let Ok(mut stopped) = downloads_stopped_before_their_dispositions().lock() {
                stopped.insert(request.to_string());
            }
        }
    }
}

/// **The manual downloads the user stopped while their first read of the
/// destination had not returned**, held for the run and never written down:
/// the stop is written, as M6's *Stop*, when that read returns. Step 10c's
/// eleventh-pass fix piece 1.
fn downloads_stopped_before_their_dispositions() -> &'static Mutex<std::collections::HashSet<String>> {
    static STOPPED: std::sync::OnceLock<Mutex<std::collections::HashSet<String>>> = std::sync::OnceLock::new();
    STOPPED.get_or_init(|| Mutex::new(std::collections::HashSet::new()))
}

/// **An upload's or a dependency send's leftover, removed on the machine** —
/// `File Browser` U8 as amended 2026-10-03, and `Dependency Upload & Remap` A12
/// for a send (the 38's piece 9, item 33). The records are the request's own,
/// under the server's current installation, each reading *interrupted* or
/// owing its cleanup; [`crate::downloading::an_upload_leftover_is_removed_on_the_machine`]
/// does the rest.
async fn the_part_on_the_machine_is_removed(
    known: &WhatIsKnown,
    door: &dyn crate::downloading::TheRecordADownloadWrites,
    the_run: &TheRunsTransfers,
    point: &crate::servers::sessions::ThisServersSessions,
    request: &str,
) -> Option<crate::downloading::TheLeftoverRemoved> {
    let found = known.requests.iter().find(|one| one.id == request)?;
    if !matches!(found.stands, WhereTheRequestStands::LeftOver { removable: true, .. }) {
        return None;
    }
    let row = Identity::read_back(IdentityKind::LocalServer, &found.server)?;
    let identity = Identity::read_back(IdentityKind::Operation, request)?;
    let kind = found.kind;
    let mut records = Vec::new();
    door.with_the_record(&mut |connection| {
        let installation_now = server_rows::one(connection, &row.as_text())
            .ok()
            .flatten()
            .and_then(|one| one.last_confirmed_installation_identity);
        records = match kind {
            TheKind::Dependencies => crate::the_found_files_sent::the_transfers_a_send_left(connection, request),
            _ => crate::downloading::the_transfers_a_request_left(connection, &identity, &row, Direction::Up),
        }
        .into_iter()
        .filter(|one| one.installation.as_ref().map(Identity::as_text) == installation_now)
        .collect();
    });
    Some(crate::downloading::an_upload_leftover_is_removed_on_the_machine(door, the_run, point, records).await)
}

/// **A download's leftover's *Remove*, taken** — the request is found on the
/// screen's own reading, and only a leftover is acted on. Step 10c's transfer
/// wave piece 7. *(An upload's goes to the machine since the 38's piece 9:
/// [`the_part_on_the_machine_is_removed`].)*
fn the_leftover_is_removed(
    known: &WhatIsKnown,
    door: &dyn crate::downloading::TheRecordADownloadWrites,
    the_run: &TheRunsTransfers,
    request: &str,
) -> Option<crate::downloading::TheLeftoverRemoved> {
    let found = known.requests.iter().find(|one| one.id == request)?;
    if !matches!(found.stands, WhereTheRequestStands::LeftOver { removable: true, .. }) {
        return None;
    }
    let direction = match found.kind {
        TheKind::Upload => Direction::Up,
        TheKind::Download => Direction::Down,
        _ => return None,
    };
    let identity = Identity::read_back(IdentityKind::Operation, request)?;
    let row = Identity::read_back(IdentityKind::LocalServer, &found.server)?;
    Some(crate::downloading::a_leftover_is_removed(
        door, the_run, &identity, &row, direction,
    ))
}

// ════════════════════════════════════════════════════════════════════════════
// The downloads the user starts by hand — `Download` M1, step 10c pieces 10
// and *Files' own Download*
// ════════════════════════════════════════════════════════════════════════════

/// **The manual route, and it is genuinely its own.**
///
/// `Design/Queue & History` sends *Download all* and *Download all to…* here,
/// and **step 10b's scenario is the route that needs it**: its situation has the
/// outputs *"already downloaded"* and says outright *"auto-download was never
/// switched on."* So no arrangement is read, made, consulted or written
/// anywhere in this act — `Download` M1's request is one multi-part action the
/// user confirmed, and nothing about it is standing work.
///
/// **Two screens enter it and it is one route, not two.** A queue or history
/// entry sends the whole job, which lands on that job's own folder under
/// `Download` P10's name (M9); `Design/File Browser` sends what the user picked
/// in the browser, which `Download` M2 is written for in as many words —
/// *"anything the login can read can be requested — files, folders, any mix, in
/// any number … descending from what the user picked."* Both compose one M1
/// request through [`crate::downloading::a_download_is_prepared`], and this is
/// the one answer both give back.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
#[serde(tag = "what")]
pub enum TheDownloadAnswer {
    /// The pre-flight has something to put in front of the user — a collision
    /// to answer, a cannot-vouch mark to name, or a refusal to say
    /// (`Download` M6).
    ItAsks {
        request: Verbatim,
        /// **M6's headline: how many of the request already exist** — absent
        /// where none does, so no count of nothing is said.
        headline: Option<Words>,
        /// **The cannot-vouch files, named beside the same question**, so the
        /// user answers once knowing all of it. Absent where there are none.
        cannot_vouch_for: Option<Words>,
        /// **M4's refusals, said at the same moment**, and absent where the
        /// walk met none.
        refusals: Option<Words>,
        /// **The four answers, which are the spec's own** — Replace, Keep both,
        /// Skip existing, Stop — **or, where nothing is already there,
        /// `Download` J10's go-ahead and Stop.**
        answers: Vec<AControl>,
    },
    /// **`Download` M6, narrowed: a request whose complete enumeration finds
    /// none of the three starts without a question**, and no empty surface is
    /// composed for it.
    ItStarted {
        request: Verbatim,
    },
    DidNotStart {
        said: Prose,
    },
    /// **An upload sent again from this screen did not start** — `Files`' own
    /// sentence, which can carry the file's name as a value (X7), so it is
    /// words rather than one prose sentence. The 38's piece 9, item 17.
    TheUploadDidNotStart {
        said: Words,
    },
    /// **The user closed the folder panel on a *…to* act.** They asked for
    /// nothing to happen and nothing did, so there is nothing to say — which is
    /// the same answer [`crate::picker`]'s other callers give a closed panel.
    TheUserChoseNoFolder,
    /// **The user pressed *Stop* while the files were being counted**, and
    /// nothing was fetched, written or made. Nothing to say either — step 10c's
    /// twelfth-pass fix piece 3.
    TheUserStoppedIt,
}

/// **The requests waiting on `Download` M6's question**, held for the life of
/// the run and never written down: the answer is durable the moment it is
/// given, and what waits here is the enumeration the answer is applied to.
fn downloads_waiting_on_their_question(
) -> &'static Mutex<Vec<(String, String, crate::downloading::ADownloadReadyToRun)>> {
    static WAITING: std::sync::OnceLock<
        Mutex<Vec<(String, String, crate::downloading::ADownloadReadyToRun)>>,
    > = std::sync::OnceLock::new();
    WAITING.get_or_init(|| Mutex::new(Vec::new()))
}

/// **`Download` J9's reading, off the local job records alone and with no read
/// of the machine.**
///
/// **The mark is `Download` J9a's**: it is the ending's kind rather than an
/// ending's absence, and the whole folder rather than the ending walk's
/// frontier.
struct TheJobRecordsHere<'a> {
    /// **The record door and never a held connection**, because the enumeration
    /// this reads for runs over the machine: a lock held across it would hold
    /// the one local record for the length of a remote walk.
    record: &'a dyn crate::downloading::TheRecordADownloadWrites,
    job: Identity,
    the_jobs_output_folder: Vec<u8>,
    label: Option<String>,
    /// **`Download` N5a's answer for this job, read off its record directory
    /// before the walk**: every file Smoozy placed in the job's folder before
    /// the render that nothing has touched since, by whole machine path.
    /// **Empty where nothing was placed, or where it could not be read** —
    /// and then everything comes home, the clause's own safe direction.
    /// Added 2026-10-02, the fixes before 0.1.8, fault 3.
    smoozys_own_copies: Vec<Vec<u8>>,
}

impl smoozy_download::acting::enumerating::ReadsTheLocalJobRecords for TheJobRecordsHere<'_> {
    fn is_a_copy_smoozy_placed(&self, source_path: &[u8]) -> bool {
        self.smoozys_own_copies
            .iter()
            .any(|placed| placed.as_slice() == source_path)
    }

    fn what_the_local_record_says(
        &self,
        source_path: &[u8],
    ) -> smoozy_download::vouching::WhatTheLocalRecordSays {
        let inside = source_path.starts_with(&self.the_jobs_output_folder);
        let mut ending = None;
        self.record.with_the_record(&mut |connection| {
            ending = crate::record::job_records::JobRecords::over(connection)
                .read(&self.job)
                .ok()
                .flatten()
                .and_then(|held| held.the_ending_learned());
        });
        smoozy_download::vouching::WhatTheLocalRecordSays {
            the_file_is_inside_the_jobs_output_folder: inside,
            the_recorded_ending: ending,
            // **`Rendering` K18's per-frame reckoning is not in the local job
            // record**, and `Rendering` K22's rule is that an outcome carrying
            // no statement is never read as an empty one — so the absence is
            // carried as an absence and `Download` J9b lays the mark over the whole folder.
            the_completeness_statement: None,
        }
    }

    fn the_files_frame(&self, _source_path: &[u8]) -> Option<i64> {
        // **The inventory is `Job files and outputs` §7's and lives on the
        // machine**; nothing on this computer names a file's frame for a job
        // that was not read. **Absent is a value** and `Download` J9b's second bullet gives
        // such a file the whole-folder mark.
        None
    }

    fn the_job_whose_output_folder_this_is(
        &self,
        picked: &[u8],
    ) -> Option<smoozy_download::acting::enumerating::AJobsOwnFolder> {
        (picked == self.the_jobs_output_folder.as_slice()).then(|| {
            // **`Download` P10's dated note: a first-frame check lands under
            // its range job's name**, the folder on the machine being the
            // range job's (`Job files and outputs` P18a).
            let mut job = self.job.clone();
            self.record.with_the_record(&mut |connection| {
                job = crate::record::job_records::JobRecords::over(connection)
                    .the_job_whose_folder_it_lands_in(&self.job);
            });
            smoozy_download::acting::enumerating::AJobsOwnFolder {
                job,
                label: self.label.clone(),
            }
        })
    }
}

/// The record door, over the running application's own record.
struct TheApplicationsRecord<R: tauri::Runtime> {
    app: tauri::AppHandle<R>,
}

impl<R: tauri::Runtime> crate::downloading::TheRecordADownloadWrites for TheApplicationsRecord<R> {
    fn with_the_record(&self, act: &mut dyn FnMut(&rusqlite::Connection)) -> bool {
        use tauri::Manager;
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

    fn a_transfer_moved(&self) {
        the_transfers_moved(&self.app);
    }
}

/// What the act needs before it can compose anything, read once out of the
/// local record and let go of.
struct WhatTheJobIs {
    server: String,
    server_row: Identity,
    the_jobs_output_folder: Vec<u8>,
    /// **The job's own label, which `Download` P10 composes its landing
    /// folder's name from** — absent where none was given, and never the file.
    label: Option<String>,
    /// **What the job is called on screen**, by the one rule Queue and History
    /// name it by (`crate::surface::the_queue::what_the_job_is_called`). Step
    /// 10c's Vast-pass fix piece 1.
    called: String,
    /// **`Download` H2's standing folder, and empty where there is none** —
    /// W5's *the system names no folder* with no folder chosen in Settings.
    /// **It is not a refusal here**, because a *…to* act is about to name one.
    destination: Vec<u8>,
    installation: Option<Identity>,
    run: Identity,
    /// **The job's record directory and how many attempts the machine's note
    /// carried**, which is where and for which attempt `Download` N5a's list
    /// of what Smoozy placed is read. Added 2026-10-02, the fixes before
    /// 0.1.8, fault 3.
    the_jobs_record_directory: Vec<u8>,
    attempts: u32,
    /// **The `.blend` the job was submitted with, on the machine** (`Queue &
    /// History` N11), which the box's folders are composed against. Added by
    /// the Creator's second notes, piece 2.
    the_blend_path: Option<String>,
    /// **The payload the job was submitted with**, as the record's kept setup
    /// holds it — read for its View Layer pins, which the box counts with.
    /// Absent where no setup was kept. The Creator's second notes, piece 2b.
    the_setup_it_was_submitted_with: Option<smoozy_render_settings::payload::Payload>,
}

/// **The payload one job was submitted with**, read off the local job record's
/// kept setup (record version 25) — and nothing where none was kept or the
/// column will not read back, so a box over such a job counts as it always
/// did. The Creator's second notes, piece 2b.
fn the_setup_the_job_was_submitted_with(
    connection: &rusqlite::Connection,
    job: &Identity,
) -> Option<smoozy_render_settings::payload::Payload> {
    match crate::record::job_records::JobRecords::over(connection).the_kept_setup_of(job) {
        Ok(Some(crate::record::job_records::TheKeptSetup::Kept(payload))) => Some(payload),
        _ => None,
    }
}

fn what_the_job_is<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    job: &Identity,
) -> Result<WhatTheJobIs, Prose> {
    use tauri::Manager;
    let nothing = || Prose::new(words::THE_JOB_WOULD_NOT_READ);
    let started = app.try_state::<Mutex<Started>>().ok_or_else(nothing)?;
    let started = started.lock().map_err(|_| nothing())?;
    let record = started.record.as_ref().ok_or_else(nothing)?;
    let run = started.run.clone().ok_or_else(nothing)?;
    what_the_job_is_in(record.connection(), run, job)
}

/// [`what_the_job_is`], over the local record already in hand — so a test can
/// read a job the way the act does, with no running app.
fn what_the_job_is_in(
    connection: &rusqlite::Connection,
    run: Identity,
    job: &Identity,
) -> Result<WhatTheJobIs, Prose> {
    let nothing = || Prose::new(words::THE_JOB_WOULD_NOT_READ);
    let held = crate::record::job_records::JobRecords::over(connection)
        .read(job)
        .ok()
        .flatten()
        .ok_or_else(nothing)?;
    let server_row = held.server.local_server.clone();
    let server = server_row.as_text();
    let row = server_rows::all(connection)
        .ok()
        .and_then(|rows| {
            rows.into_iter()
                .find(|row| row.local_server_identifier == server)
        })
        .ok_or_else(nothing)?;
    // **`Job files and outputs` N1: every path Smoozy composes on a rented
    // machine is composed by that document's rules**, from the layout the row
    // recorded at hand-over — never by joining strings here.
    let layout = crate::servers::installation::the_layout_recorded_for(&row).ok_or_else(nothing)?;
    // **`Job files and outputs` N3: the job record directory is named by the
    // identity alone**, composed from the layout as it always was.
    let paths = smoozy_job_files::publication::AJobsPaths::composed(&layout, job, held.label.as_deref())
        .map_err(|_| nothing())?;
    // **The job's folder as the local record holds it** (`Queue & History`
    // Y8) — the one the machine chose at render start, or the job area's for a
    // job submitted before §6.6 — and **none until the render has chosen it**,
    // which is said in the Design's own words for that wait. The Creator's
    // second notes, piece 4.
    let the_jobs_output_folder = held
        .the_job_output_folder(&layout)
        .ok_or_else(|| Prose::new(words::THE_FOLDER_IS_MADE_WHEN_THE_RENDER_STARTS))?;
    // **`Download` H2's standing setting**, which is what the request lands
    // under; the job's own folder goes between it and the files (M9, P10).
    let destination = the_standing_folder(connection);
    Ok(WhatTheJobIs {
        server,
        server_row,
        the_jobs_output_folder: the_jobs_output_folder.into_bytes(),
        label: held.label.clone(),
        called: crate::surface::the_queue::what_the_job_is_called(&held),
        destination,
        // **`State, identity and recovery` §2.3: the installation identity is
        // the only thing that tells a recycled machine apart**, and it is the
        // row's own last confirmed one. **Absent is a state** and the request
        // carries none.
        installation: row
            .last_confirmed_installation_identity
            .as_deref()
            .and_then(|text| Identity::read_back(IdentityKind::Installation, text)),
        run,
        the_jobs_record_directory: paths.job_record_directory.into_bytes(),
        attempts: held.what_the_note_carried.attempts,
        the_blend_path: held.blend_path_on_the_machine.clone(),
        the_setup_it_was_submitted_with: the_setup_the_job_was_submitted_with(connection, job),
    })
}

/// **`Download` H2's standing setting, as bytes, and empty where there is
/// none.** W4 is untouched: the folder is proposed and never made here.
fn the_standing_folder(connection: &rusqlite::Connection) -> Vec<u8> {
    crate::downloading::where_a_download_lands(connection)
        .folder()
        .map(|folder| folder.as_os_str().as_encoded_bytes().to_vec())
        .unwrap_or_default()
}

/// What Files' own Download needs before it can compose anything, read once out
/// of the local record and let go of.
struct WhatThisServerIs {
    server_row: Identity,
    installation: Option<Identity>,
    run: Identity,
    /// `Download` H2's standing folder, empty where there is none.
    destination: Vec<u8>,
    /// **Every job this desktop holds on this server, with its output folder
    /// composed** — `Download` J9's whole reading, taken once with no read of
    /// the machine.
    jobs: Vec<AJobOnThisServer>,
}

fn what_this_server_is<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    server: &str,
) -> Result<WhatThisServerIs, Prose> {
    use tauri::Manager;
    let nothing = || Prose::new(words::THE_RECORD_WOULD_NOT_TAKE_IT);
    let started = app.try_state::<Mutex<Started>>().ok_or_else(nothing)?;
    let started = started.lock().map_err(|_| nothing())?;
    let record = started.record.as_ref().ok_or_else(nothing)?;
    let run = started.run.clone().ok_or_else(nothing)?;
    let row = server_rows::all(record.connection())
        .ok()
        .and_then(|rows| {
            rows.into_iter()
                .find(|row| row.local_server_identifier == server)
        })
        .ok_or_else(nothing)?;
    let server_row = Identity::read_back(IdentityKind::LocalServer, server).ok_or_else(nothing)?;
    Ok(WhatThisServerIs {
        installation: row
            .last_confirmed_installation_identity
            .as_deref()
            .and_then(|text| Identity::read_back(IdentityKind::Installation, text)),
        destination: the_standing_folder(record.connection()),
        jobs: the_jobs_on(record.connection(), &row, &server_row),
        server_row,
        run,
    })
}

/// **One job of this server, as `Download` J9 needs it** — its own output
/// folder and the ending the local record holds, and nothing of the machine.
struct AJobOnThisServer {
    job: Identity,
    /// **The range job, where this job is a first-frame check** — whose name
    /// the pair's one folder lands under (`Download` P10's dated note).
    /// Nothing for every other job. Added 2026-10-31.
    lands_under: Option<Identity>,
    label: Option<String>,
    output_folder: Vec<u8>,
    ending: Option<smoozy_queue_history::ending::Ending>,
    /// **The job's record directory and its latest attempt**, where `Download`
    /// N5a's placed-copy list and the reading after the render are read.
    /// After 0.1.8, the checks' leftovers, item 4.
    record_directory: Vec<u8>,
    attempts: u32,
}

/// **Every job this desktop tracks on one server, with its output folder
/// composed** — `Job files and outputs` N1: a path on a rented machine is
/// composed by that document's rules from the layout the row recorded, never by
/// joining strings here.
///
/// **A server whose row records no layout yields nothing**, and that is stated
/// rather than papered over: without the layout no job's output folder can be
/// composed, so no file can be judged to lie inside one, so `Download` J9 makes
/// no claim — which is that clause's own answer outside a job's output folder.
fn the_jobs_on(
    connection: &rusqlite::Connection,
    row: &smoozy_servers::store::ServerRow,
    server_row: &Identity,
) -> Vec<AJobOnThisServer> {
    let Some(layout) = crate::servers::installation::the_layout_recorded_for(row) else {
        return Vec::new();
    };
    let records = crate::record::job_records::JobRecords::over(connection);
    // **Both lists, because neither alone is every job.** A job with no learned
    // ending is in the first and may be in no history at all; a job with a
    // recorded ending has started, so it is in the second. `Download` J9
    // marks on both.
    let mut held: Vec<smoozy_queue_history::record::LocalJobRecord> = records
        .with_no_learned_ending_for(server_row)
        .unwrap_or_default();
    if let Ok(history) = records.in_history() {
        for entry in history.entries {
            if entry.record.server.local_server.as_text() != server_row.as_text() {
                continue;
            }
            if held
                .iter()
                .any(|one| one.job.as_text() == entry.record.job.as_text())
            {
                continue;
            }
            held.push(entry.record);
        }
    }
    held.into_iter()
        .filter_map(|one| {
            let paths = smoozy_job_files::publication::AJobsPaths::composed(
                &layout,
                &one.job,
                one.label.as_deref(),
            )
            .ok()?;
            // **The job's folder as the local record holds it** (`Queue &
            // History` Y8) — empty where the render has not chosen it yet,
            // which no pick can reach. The Creator's second notes, piece 4.
            let output_folder = one.the_job_output_folder(&layout).unwrap_or_default().into_bytes();
            let lands_under = Some(records.the_job_whose_folder_it_lands_in(&one.job))
                .filter(|range| range.as_text() != one.job.as_text());
            Some(AJobOnThisServer {
                ending: one.the_ending_learned(),
                attempts: one.what_the_note_carried.attempts,
                lands_under,
                job: one.job,
                label: one.label,
                output_folder,
                record_directory: paths.job_record_directory.into_bytes(),
            })
        })
        .collect()
}

/// **`Download` J9's reading for a request made in the file browser**, off the
/// local job records alone and with no read of the machine.
///
/// **A browser request may reach files of more than one job, and files of no
/// job at all**, which is why the trait is asked per file. The whole-job route's
/// own reader knows one job; this one holds every job of the server and finds
/// the folder a path lies in.
struct TheJobsOnThisServer {
    jobs: Vec<AJobOnThisServer>,
    /// **`Download` N5a's answer for every job the request reaches**: each
    /// file Smoozy placed in a job's folder before the render that nothing
    /// has touched since, by whole machine path. **Empty where nothing was
    /// placed, or where it could not be read** — and then everything comes
    /// home, the clause's own safe direction.
    smoozys_own_copies: Vec<Vec<u8>>,
}

/// **The jobs a browser request reaches, with `Download` N5a's copies read
/// for each** — a job whose output folder is picked, lies inside a picked
/// folder, or holds a picked thing. `Download` N5a, as written 2026-10-02:
/// no route brings a copy home, *File Browser*'s *Download* of a job folder
/// included. After 0.1.8, the checks' leftovers, item 4: until then this
/// route read no list, and a job folder downloaded from Files brought the
/// copies home.
async fn the_jobs_a_browser_request_reaches<R>(
    reader: &mut R,
    jobs: Vec<AJobOnThisServer>,
    picked: &[Vec<u8>],
) -> TheJobsOnThisServer
where
    R: smoozy_download::acting::record_directory::ReadsOneFileOnTheMachine,
{
    let mut smoozys_own_copies = Vec::new();
    for one in &jobs {
        let folder = trim_the_trailing_slash(&one.output_folder);
        if folder.is_empty() {
            continue;
        }
        let reaches = picked.iter().any(|picked| {
            let picked = trim_the_trailing_slash(picked);
            is_at_or_below(picked, folder) || is_at_or_below(folder, picked)
        });
        if !reaches {
            continue;
        }
        // **A list that could not be read leaves everything to come home**,
        // `Download` N5a's own safe direction.
        if let Ok(Some(placed)) = smoozy_download::acting::record_directory::read_what_smoozy_placed(
            reader,
            &one.record_directory,
            one.attempts,
        )
        .await
        {
            smoozys_own_copies.extend(placed.the_copies_still_as_placed());
        }
    }
    TheJobsOnThisServer {
        jobs,
        smoozys_own_copies,
    }
}

/// **Whether `path` is `folder` or lies below it** — the folder and a
/// separator, so `/jobs/abc` never answers for `/jobs/abcdef`.
fn is_at_or_below(path: &[u8], folder: &[u8]) -> bool {
    path == folder
        || (folder == b"/" && path.starts_with(b"/"))
        || (path.len() > folder.len() && path.starts_with(folder) && path[folder.len()] == b'/')
}

impl TheJobsOnThisServer {
    /// The job whose output folder holds this path, where one does.
    ///
    /// **The test is the folder and a separator**, so `/jobs/abc-outputs` never
    /// answers for `/jobs/abcdef`.
    fn holding(&self, source_path: &[u8]) -> Option<&AJobOnThisServer> {
        self.jobs.iter().find(|one| {
            let folder = trim_the_trailing_slash(&one.output_folder);
            !folder.is_empty()
                && source_path.len() > folder.len()
                && source_path.starts_with(folder)
                && source_path[folder.len()] == b'/'
        })
    }
}

fn trim_the_trailing_slash(path: &[u8]) -> &[u8] {
    let mut end = path.len();
    while end > 1 && path[end - 1] == b'/' {
        end -= 1;
    }
    &path[..end]
}

impl smoozy_download::acting::enumerating::ReadsTheLocalJobRecords for TheJobsOnThisServer {
    fn is_a_copy_smoozy_placed(&self, source_path: &[u8]) -> bool {
        self.smoozys_own_copies
            .iter()
            .any(|placed| placed.as_slice() == source_path)
    }

    fn what_the_local_record_says(
        &self,
        source_path: &[u8],
    ) -> smoozy_download::vouching::WhatTheLocalRecordSays {
        let found = self.holding(source_path);
        smoozy_download::vouching::WhatTheLocalRecordSays {
            the_file_is_inside_the_jobs_output_folder: found.is_some(),
            the_recorded_ending: found.and_then(|one| one.ending),
            // **`Rendering` K18's per-frame reckoning is not in the local job
            // record**, and `Rendering` K22's rule is that an outcome carrying
            // no statement is never read as an empty one — so the absence is
            // carried as an absence and `Download` J9b lays the mark over the
            // whole folder. **This is the whole-job route's own answer and not
            // a second copy of the judgement**: `Download` J9's marking itself is
            // `App/Download/src/vouching.rs`'s and nothing here repeats it.
            the_completeness_statement: None,
        }
    }

    fn the_files_frame(&self, _source_path: &[u8]) -> Option<i64> {
        // **The inventory is `Job files and outputs` §7's and lives on the
        // machine**; nothing on this computer names a file's frame for a job
        // that was not read. **Absent is a value** and `Download` J9b's second
        // bullet gives such a file the whole-folder mark.
        None
    }

    fn the_job_whose_output_folder_this_is(
        &self,
        picked: &[u8],
    ) -> Option<smoozy_download::acting::enumerating::AJobsOwnFolder> {
        // **M9 and P10: a whole job picked in the browser is still a whole job**
        // — picking a job's own output folder in Files lands it on that job's
        // folder under P10's name, exactly as *Download all* does. The
        // comparison is of the picked thing and of nothing else, which is that
        // clause's own bound.
        let picked = trim_the_trailing_slash(picked);
        self.jobs
            .iter()
            .find(|one| trim_the_trailing_slash(&one.output_folder) == picked)
            .map(|one| smoozy_download::acting::enumerating::AJobsOwnFolder {
                // **`Download` P10's dated note**: a first-frame check and its
                // range job share the folder, and land under the range's name.
                job: one.lands_under.clone().unwrap_or_else(|| one.job.clone()),
                label: one.label.clone(),
            })
    }
}

/// **A queue or history entry's *Download all* and *Download all to…*, run** —
/// `Download` M1's request from its enumeration to its pre-flight.
///
/// **Nothing of `Download` R1 is touched.** This is step 10b's own route and it
/// is separate by construction: no arrangement is read, no window opens, no walk
/// series starts.
///
/// **`to` is the *…to* half**: the destination is the folder the user chose for
/// this request rather than `Download` H2's standing one. **Until step 10c's
/// *Files' own Download* this command took no such value at all, so *Download
/// all to…* quietly ran as *Download all*** and the frames landed somewhere the
/// user had not been shown.
#[tauri::command]
pub async fn a_whole_job_download_is_asked_for<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    job: String,
    to: bool,
) -> TheDownloadAnswer {
    let Some(job) = Identity::read_back(IdentityKind::Job, &job) else {
        return refused(words::THE_JOB_WOULD_NOT_READ);
    };
    let is = match what_the_job_is(&app, &job) {
        Ok(is) => is,
        Err(said) => return TheDownloadAnswer::DidNotStart { said },
    };
    let Some(point) = the_live_point_of(&app, &is.server) else {
        return refused(words::THE_SERVER_IS_NOT_CONNECTED);
    };
    // **The destination is settled before the request is composed** — piece 9's
    // ruling, and everything that could refuse this request has answered above.
    let destination = match the_destination_chosen(&app, to, is.destination.clone()).await {
        TheDestination::It(destination) => destination,
        TheDestination::TheUserChoseNoFolder => return TheDownloadAnswer::TheUserChoseNoFolder,
        TheDestination::NoPanelOpened => return refused(words::THE_FOLDER_PANEL_DID_NOT_OPEN),
    };
    if destination.is_empty() {
        return refused(words::THERE_IS_NO_DOWNLOAD_FOLDER);
    }

    let door = TheApplicationsRecord { app: app.clone() };
    // **`Download` N5a: what Smoozy itself placed in the job's folder before
    // the render**, read off the job's record directory before the walk —
    // the fixes before 0.1.8, fault 3. *Download all* brought home the
    // baked cache Smoozy had copied in, 2.8 GB on the check before 0.1.8.
    let smoozys_own_copies = match smoozy_download::acting::record_directory::read_what_smoozy_placed(
        &mut crate::servers::a_period::TheRecordDirectorysReadingRoute { sessions: &point },
        &is.the_jobs_record_directory,
        is.attempts,
    )
    .await
    {
        Ok(Some(placed)) => placed.the_copies_still_as_placed(),
        Ok(None) | Err(()) => Vec::new(),
    };
    // **The job records are read through the same door**, one short act at a
    // time, so nothing holds the one local record while the machine is read.
    let records = TheJobRecordsHere {
        record: &door,
        job: job.clone(),
        the_jobs_output_folder: is.the_jobs_output_folder.clone(),
        label: is.label.clone(),
        smoozys_own_copies,
    };
    // **The counting row is named as the job's rows are** — its label, else
    // its file's name (`an_arrangement_known`; step 10c's Vast-pass fix piece
    // 1, where it was the identity).
    let named = is.called.clone();
    the_request_is_put_to_the_user(
        &app,
        point,
        &is.server,
        named,
        crate::downloading::WhatIsBeingDownloaded {
            server_row: is.server_row.clone(),
            installation: is.installation.clone(),
            destination,
            picked: vec![is.the_jobs_output_folder.clone()],
            run: is.run.clone(),
        },
        &door,
        &records,
    )
    .await
}

/// **Files' own *Download* and *Download to…*, run** — the same `Download` M1
/// request, entered from a server's file browser instead of from a job.
///
/// **`Download` M2 is written for this case in as many words**: *"anything the
/// login can read can be requested — files, folders, any mix, in any number —
/// and the enumeration is a remote walk on the subsystem route … descending
/// from what the user picked."* So this is a second way in to one route and
/// never a second route: the walk, the durable record, M6's question, the
/// `Download` J9 marks and P11's landing structure are all the ones piece 10
/// built.
///
/// **`picked` is the path on the machine as the listing gave it, and never a
/// display name** (M1, `File Browser` N2, `File Browser` L1a). The bytes are
/// what the page holds and what it sends back untouched.
#[tauri::command]
pub async fn a_file_download_is_asked_for<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    server: String,
    picked: Vec<Vec<u8>>,
    to: bool,
) -> TheDownloadAnswer {
    // **Nothing chosen is answered before anything else is read**, because it
    // is the one refusal the user can act on by themselves.
    if picked.is_empty() || picked.iter().all(|one| one.is_empty()) {
        return refused(words::NOTHING_IS_SELECTED);
    }
    let is = match what_this_server_is(&app, &server) {
        Ok(is) => is,
        Err(said) => return TheDownloadAnswer::DidNotStart { said },
    };
    let Some(point) = the_live_point_of(&app, &server) else {
        return refused(words::THE_SERVER_IS_NOT_CONNECTED);
    };
    // **Piece 9's ruling, kept: the folder panel opens only once everything
    // that could refuse this request has passed.** A pick that is then refused
    // should not have been offered.
    let destination = match the_destination_chosen(&app, to, is.destination.clone()).await {
        TheDestination::It(destination) => destination,
        TheDestination::TheUserChoseNoFolder => return TheDownloadAnswer::TheUserChoseNoFolder,
        TheDestination::NoPanelOpened => return refused(words::THE_FOLDER_PANEL_DID_NOT_OPEN),
    };
    if destination.is_empty() {
        return refused(words::THERE_IS_NO_DOWNLOAD_FOLDER);
    }

    let door = TheApplicationsRecord { app: app.clone() };
    let records = the_jobs_a_browser_request_reaches(
        &mut crate::servers::a_period::TheRecordDirectorysReadingRoute { sessions: &point },
        is.jobs,
        &picked,
    )
    .await;
    let named = what_the_counting_row_names(&picked);
    the_request_is_put_to_the_user(
        &app,
        point,
        &server,
        named,
        crate::downloading::WhatIsBeingDownloaded {
            server_row: is.server_row,
            installation: is.installation,
            destination,
            picked,
            run: is.run,
        },
        &door,
        &records,
    )
    .await
}

// ════════════════════════════════════════════════════════════════════════════
// The queue entry's two Transfers controls, after Submit — the 38's piece 9,
// item 9
// ════════════════════════════════════════════════════════════════════════════

/// **A queue or History entry's *Files on your computer*** — `Download` H5 to
/// `Download` H8, which `Queue & History` N18's second to fourth bullets consume whole:
/// the job's set of landing places, composed when the control is used and
/// never cached, each opened selected in the folder that holds it
/// ([`TheReveal`]), and where the set is empty, which of `Download` H7a's two
/// readings applies. **It reads no machine and needs no connection** (`Download`
/// H11), so it works on an entry whose server is gone.
///
/// **Answers the sentence to say, or nothing where every place opened.**
///
/// **The job's output folder is composed from the server row's recorded
/// layout** (`Job files and outputs` N1, as *Download all* composes it); where
/// the row is gone it cannot be, and `Download` H5's second source then reaches no
/// request — the arrangement's places still open. Said in the piece's account.
pub(crate) fn the_jobs_files_are_shown<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    job: &str,
) -> Option<&'static str> {
    use smoozy_download::arrangement::KeepsTheArrangement as _;
    use smoozy_download::landing::the_set_of_landing_places;
    use smoozy_download::request::KeepsTheRequest as _;
    use tauri::Manager as _;
    let job = Identity::read_back(IdentityKind::Job, job)?;
    let output_folder = what_the_job_is(app, &job)
        .map(|is| is.the_jobs_output_folder)
        .unwrap_or_default();
    let (set, requests) = {
        let started = app.try_state::<Mutex<Started>>()?;
        let started = started.lock().ok()?;
        let connection = started.record.as_ref()?.connection();
        let held = crate::record::job_records::JobRecords::over(connection).read(&job).ok().flatten()?;
        // **`Download` P10's dated note: a first-frame check's files landed
        // under its range job's name**, so the place is composed with that
        // name (`Job files and outputs` P18a). The arrangement is read here
        // for its places alone and written nowhere.
        let lands_under =
            crate::record::job_records::JobRecords::over(connection).the_job_whose_folder_it_lands_in(&job);
        let arrangement = DownloadArrangements::over(connection)
            .the_arrangement_for(&job)
            .ok()
            .flatten()
            .map(|mut arrangement| {
                arrangement.job = lands_under.clone();
                arrangement
            });
        let requests = DownloadRequests::over(connection)
            .every_request_for(&held.server.local_server)
            .unwrap_or_default();
        let set = the_set_of_landing_places(
            arrangement.as_ref(),
            &crate::downloading::TheReceivingSystemsRulesHere,
            held.label.as_deref(),
            &requests,
            &output_folder,
        );
        (set, requests)
    };
    the_set_is_opened(&set, &requests, &output_folder, &mut TheReveal)
}

/// [`the_jobs_files_are_shown`]'s opening and its words, over an opener a test
/// can stand in for.
pub(crate) fn the_set_is_opened(
    set: &[Vec<u8>],
    requests: &[smoozy_download::request::AManualRequest],
    output_folder: &[u8],
    opener: &mut dyn OpensAFolder,
) -> Option<&'static str> {
    use smoozy_download::landing::{what_the_control_is_handed, WhatTheControlIsHanded};
    match what_the_control_is_handed(set, requests, output_folder, |_| true) {
        // **`Download` H7: every place opens and none is picked**, and H8: a
        // place that will not open is said, never dropped. **Note 4: a place
        // that is gone is said gone**, and the others in the set still open.
        WhatTheControlIsHanded::ThePlaces(_) => {
            let not_opened: Vec<Vec<u8>> = set
                .iter()
                .zip(open_every_place(&mut OnlyWhatIsThere(opener), set))
                .filter(|(_, one)| !one.it_opened)
                .map(|(place, _)| place.clone())
                .collect();
            what_is_said_of(&not_opened)
        }
        WhatTheControlIsHanded::NothingOfTheJobCameHome => Some(words::NOTHING_OF_THIS_JOB_IS_DOWNLOADED),
        WhatTheControlIsHanded::WhatCameHomeWasPickedOutFileByFile => Some(words::ONLY_PARTS_WERE_DOWNLOADED),
    }
}

/// **The entry's switch, off** — `Download` R3 and R11's first half: the
/// desired state and nothing else; every fetch already arranged runs to its
/// ending and nothing is removed. **Answers whether it was written.** Switching
/// on asks where again (`Download` R11) and so goes through the format box
/// ([`the_formats_are_chosen_for_a_job`]), never through this.
pub(crate) fn the_entrys_switch_is_turned_off<R: tauri::Runtime>(app: &tauri::AppHandle<R>, job: &str) -> bool {
    use tauri::Manager as _;
    let Some(started) = app.try_state::<Mutex<Started>>() else {
        return false;
    };
    the_switch_off_is_taken(started.inner(), job).is_none()
}

/// **What the box after Submit is composed from**, read now.
struct TheJobsBox {
    is: WhatTheJobIs,
    arrangement: Option<smoozy_download::arrangement::AnArrangement>,
    outputs: Option<smoozy_job_files::reading::TheOutputsHalf>,
    knows: WhatTheBoxKnows,
    record: crate::surface::render_settings::WhatTheRecordHolds,
}

/// **The box after Submit, read** — `Download` N1 and `Download` N2: the
/// formats are read
/// off the job's recorded reading, which reaches the desktop as a reading of
/// the job's record directory over the connection (the manifest where the file
/// was read before submitting, and otherwise the reading the render worker
/// wrote at render start); **until it lands there is no list**, and the box
/// says everything downloads. What the arrangement already holds — its
/// narrowing and its folder — is what the box opens on (`Download` R1).
async fn the_jobs_box_is_read<R: tauri::Runtime>(app: &tauri::AppHandle<R>, job: &Identity) -> Result<TheJobsBox, Prose> {
    use smoozy_download::acting::record_directory::{read_the_narrowing_material, read_the_recorded_reading_the_machine_wrote};
    use smoozy_download::arrangement::KeepsTheArrangement as _;
    use smoozy_download::narrowing::TheNarrowingMaterial;
    use tauri::Manager as _;
    let is = what_the_job_is(app, job)?;
    let (arrangement, record) = {
        let nothing = || Prose::new(words::THE_JOB_WOULD_NOT_READ);
        let started = app.try_state::<Mutex<Started>>().ok_or_else(nothing)?;
        let started = started.lock().map_err(|_| nothing())?;
        let connection = started.record.as_ref().ok_or_else(nothing)?.connection();
        (
            DownloadArrangements::over(connection).the_arrangement_for(job).ok().flatten(),
            crate::surface::render_settings::WhatTheRecordHolds::read_now(started.record.as_ref()),
        )
    };
    let mut outputs = None;
    if let Some(point) = the_live_point_of(app, &is.server) {
        let mut route = crate::servers::a_period::TheRecordDirectorysReadingRoute { sessions: &point };
        let read = read_the_narrowing_material(&mut route, &is.the_jobs_record_directory).await;
        let read = match (&read.narrowing_material, &read.job_output_folder) {
            (TheNarrowingMaterial::ItHasNotLanded, Some(folder)) if is.attempts > 0 => {
                read_the_recorded_reading_the_machine_wrote(&mut route, &is.the_jobs_record_directory, is.attempts, folder)
                    .await
                    .unwrap_or(read)
            }
            _ => read,
        };
        if let TheNarrowingMaterial::OnTheDesktop(reading) = read.narrowing_material {
            outputs = Some(*reading);
        }
    }
    let knows = what_the_jobs_box_knows(&is, arrangement.as_ref(), outputs.as_ref(), &record);
    Ok(TheJobsBox { is, arrangement, outputs, knows, record })
}

/// **What the box after Submit knows, composed** from what
/// [`the_jobs_box_is_read`] read: the job's reading, the arrangement's
/// narrowing and folder, and the job's own ground — its `.blend` and the View
/// Layer pins it was submitted with — so the queue's box counts the files the
/// job writes, as Render Settings did for the same job (the Creator's second
/// notes, piece 2b).
fn what_the_jobs_box_knows(
    is: &WhatTheJobIs,
    arrangement: Option<&smoozy_download::arrangement::AnArrangement>,
    outputs: Option<&smoozy_job_files::reading::TheOutputsHalf>,
    record: &crate::surface::render_settings::WhatTheRecordHolds,
) -> WhatTheBoxKnows {
    let choices = crate::surface::render_settings::TheSubmissionChoices {
        the_formats_chosen: arrangement.and_then(|one| match &one.narrowing {
            TheNarrowing::Everything => None,
            TheNarrowing::TheseFormats(formats) => Some(formats.clone()),
        }),
        where_they_go: arrangement.map(|one| one.destination.clone()),
        ..Default::default()
    };
    let mut knows = crate::surface::render_settings::what_the_format_box_knows(
        outputs,
        &choices,
        record,
        &crate::surface::render_settings::the_ground_of_a_submitted_job(
            is.the_blend_path.as_deref(),
            is.the_setup_it_was_submitted_with.as_ref(),
        ),
    );
    // **The narrowing that stands is an arrangement's that is on** (the box ruling of 2026-10-17). One
    // that is off is reset to everything by the switch-on this box's Done
    // makes (`Download` R11), so there everything is what goes.
    knows.the_formats_chosen = arrangement
        .filter(|one| one.desired_state == DesiredState::On)
        .and_then(|one| match &one.narrowing {
            TheNarrowing::Everything => None,
            TheNarrowing::TheseFormats(formats) => Some(formats.clone()),
        });
    knows
}

/// **Whether the arrangement is on**, so the folder is the one fixed when it was
/// switched on (`Download` R1: *"fixed for the life of the job … moved only by
/// the user switching the arrangement off and on again"*).
fn the_folder_is_fixed(arrangement: Option<&smoozy_download::arrangement::AnArrangement>) -> bool {
    arrangement.is_some_and(|one| one.desired_state == DesiredState::On)
}

/// **The box after Submit, composed** from what [`the_jobs_box_is_read`] read.
fn the_jobs_box_is_composed(read: &TheJobsBox) -> TheFormatBox {
    let mut the_box = the_format_box_from(&read.knows, &read.record.the_proposal());
    // **The box is the queue's here, so nothing says the formats can be
    // changed later from the queue** (`Design/Render Settings § Submitting`,
    // the Creator's note 16 on their second notes).
    the_box.fixed = Some(Words::plain(words::FIXED_ONCE_SUBMITTED));
    if the_folder_is_fixed(read.arrangement.as_ref()) {
        // **The one folder it was switched on with**, and no other row — kept
        // for *Done*, which hands back row 0 (must-do 18).
        the_box.folders.truncate(1);
        the_box.the_folder_is_fixed = true;
        // **A fact, not a choice: one plain line, *To ~/Renders/kitchen***,
        // and no sentence about where it goes — it went there already.
        the_box.the_fixed_folder = Some(Words::plain(words::TO).and(
            the_box.folders.first().map(|one| one.folder.as_str().to_string()).unwrap_or_default(),
        ));
        the_box.fixed = None;
    } else if let Some(arrangement) = &read.arrangement {
        // **`Download` R11's telling, said before the switch-on proceeds**:
        // where the arrangement has already downloaded files, what is already
        // downloaded stays where it is and only the rest comes to a folder
        // chosen now (`AnArrangement::what_the_switch_on_has_to_say_first`).
        let fetched = arrangement
            .reckoning
            .iter()
            .any(|entry| entry.account.state == smoozy_download::arrangement::WhatSmoozyDid::Fetched);
        if fetched {
            the_box.before_it_moves = Some(Words::around(
                words::FILES_ALREADY_DOWNLOADED_STAY_IN,
                String::from_utf8_lossy(&arrangement.destination).into_owned(),
                words::ONLY_THE_REST_GO_TO_ANOTHER,
            ));
        }
    }
    the_box
}

/// **What the user chose in the box after Submit, taken** — `Download` R1,
/// `Download` R3,
/// R11 and `Download` N3 on the queue entry (`Queue & History` L1a). The 38's piece 9,
/// item 9.
///
/// - **The arrangement is on**: the narrowing applies from this moment
///   (`Download` N3) —
///   files already downloaded stay, files not yet downloaded outside it stop
///   being owed, and a widened narrowing re-opens what it had passed over. The
///   folder is the one it was switched on with, which the box offered alone.
/// - **It is off**: it is switched on again with the folder and formats chosen
///   — `Download` R11's reset of both, the reckoning kept, so what is already downloaded
///   is not downloaded again.
/// - **There is none**: it is made now, from the queue entry (`Download` R1: *"or
///   afterwards from the queue entry"*).
///
/// **Every one of the three is `AnArrangement`'s own act**, written by the one
/// keeper. **A format the reading does not offer, or a folder the box did not
/// offer, changes nothing** (H34), and the answer says the choice did not save.
#[tauri::command]
pub async fn the_formats_are_chosen_for_a_job<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    job: String,
    formats: Vec<String>,
    folder: crate::surface::render_settings::TheFolderLeftChosen,
) -> Option<Prose> {
    use smoozy_download::arrangement::KeepsTheArrangement as _;
    use tauri::Manager as _;
    let did_not_save = || Some(Prose::new(THE_SWITCH_DID_NOT_SAVE));
    let Some(job) = Identity::read_back(IdentityKind::Job, &job) else {
        return did_not_save();
    };
    let read = match the_jobs_box_is_read(&app, &job).await {
        Ok(read) => read,
        Err(said) => return Some(said),
    };
    let Some(narrowing) = the_narrowing_chosen(&read, &formats) else {
        return did_not_save();
    };
    let where_they_go = match folder {
        crate::surface::render_settings::TheFolderLeftChosen::ThisOne { at } => {
            let mut offered = the_folders_offered(&read.knows, &read.record.the_proposal());
            if the_folder_is_fixed(read.arrangement.as_ref()) {
                offered.truncate(1);
            }
            match offered.into_iter().nth(at) {
                Some((folder, _)) => folder,
                None => return did_not_save(),
            }
        }
        crate::surface::render_settings::TheFolderLeftChosen::SomewhereElse => {
            if the_folder_is_fixed(read.arrangement.as_ref()) {
                return did_not_save();
            }
            use crate::picker::{self, Asking, HowMany, Picked, WhatMayBeChosen};
            let picked = picker::asked(
                &app,
                Asking {
                    what: WhatMayBeChosen::Folders,
                    how_many: HowMany::One,
                    starting_at: None,
                    title: Prose::new(words::WHERE_THE_DOWNLOAD_GOES),
                },
            )
            .await;
            let Picked::TheUserChose(chosen) = picked else {
                return None;
            };
            match chosen.first() {
                Some(one) => one.as_the_user_gave_it().as_os_str().as_encoded_bytes().to_vec(),
                None => return None,
            }
        }
    };
    // **`Download` H3: no folder is written into that nobody named.**
    if where_they_go.is_empty() {
        return Some(Prose::new(words::THERE_IS_NO_DOWNLOAD_FOLDER));
    }
    let arrangement = the_arrangement_after_the_choice(read.arrangement, &job, &read.is.server_row, where_they_go, narrowing);
    let written = match app.try_state::<Mutex<Started>>() {
        Some(started) => started.lock().ok().and_then(|started| {
            let connection = started.record.as_ref()?.connection();
            DownloadArrangements::over(connection).write_it_down(&arrangement).ok()
        }),
        None => None,
    };
    if written.is_none() {
        return did_not_save();
    }
    crate::surface::the_queue::the_queue_is_told_again(&app);
    the_transfers_moved(&app);
    None
}

/// **The arrangement the box's choice leaves**, each case `AnArrangement`'s own
/// act — see [`the_formats_are_chosen_for_a_job`]. On: the narrowing from now,
/// the folder kept (`Download` N3, R1). Off: switched on again with both reset
/// and the reckoning kept (`Download` R11). None: made now (`Download` R1).
fn the_arrangement_after_the_choice(
    arrangement: Option<smoozy_download::arrangement::AnArrangement>,
    job: &Identity,
    server_row: &Identity,
    where_they_go: Vec<u8>,
    narrowing: TheNarrowing,
) -> smoozy_download::arrangement::AnArrangement {
    match arrangement {
        Some(mut arrangement) => {
            let destination = if arrangement.desired_state == DesiredState::On {
                arrangement.destination.clone()
            } else {
                where_they_go
            };
            let _ = arrangement.switched_on_again(destination, narrowing);
            arrangement
        }
        None => smoozy_download::arrangement::AnArrangement::switched_on(
            job.clone(),
            server_row.clone(),
            where_they_go,
            narrowing,
        ),
    }
}

/// **The narrowing the box's ticks make**, over the formats the reading offers
/// — every one ticked is *everything* (`Download` N3: no filter rather than a
/// frozen list); a name the reading does not offer is no narrowing at all.
/// **With nothing read, nothing can be ticked, and Done keeps the narrowing
/// the box named** (the box ruling of 2026-10-17) — everything where none stands. **A list of one has
/// nothing to narrow** (`Design/Render Settings`), so it is everything. **Each
/// chosen format is kept as the reading recorded its name** (`Download` N1),
/// in the order the box offered them.
fn the_narrowing_chosen(read: &TheJobsBox, formats: &[String]) -> Option<TheNarrowing> {
    the_narrowing_the_ticks_make(
        read.knows.formats.as_deref(),
        read.outputs.as_ref(),
        read.knows.the_formats_chosen.as_deref(),
        formats,
    )
}

/// [`the_narrowing_chosen`] over its readings, so a test stands where it
/// stands. `standing` is [`WhatTheBoxKnows::the_formats_chosen`].
fn the_narrowing_the_ticks_make(
    offered: Option<&[AFormatKnown]>,
    outputs: Option<&smoozy_job_files::reading::TheOutputsHalf>,
    standing: Option<&[Vec<u8>]>,
    formats: &[String],
) -> Option<TheNarrowing> {
    let Some(offered) = offered else {
        return formats.is_empty().then(|| match standing {
            Some(chosen) => TheNarrowing::TheseFormats(chosen.to_vec()),
            None => TheNarrowing::Everything,
        });
    };
    if formats.iter().any(|one| !offered.iter().any(|format| format.name == *one)) {
        return None;
    }
    // **A list of one draws no row to tick**, so the page sends none — which
    // was kept as a narrowing to no format at all (found building the box ruling).
    if offered.len() < 2 || offered.iter().all(|format| formats.contains(&format.name)) {
        return Some(TheNarrowing::Everything);
    }
    let as_recorded = |name: &str| -> Vec<u8> {
        outputs
            .into_iter()
            .flat_map(|outputs| outputs.destinations.iter())
            .filter_map(|destination| destination.format.as_ref())
            .map(|format| format.as_the_file_records_it.as_bytes().to_vec())
            .find(|bytes| String::from_utf8_lossy(bytes) == name)
            .unwrap_or_else(|| name.as_bytes().to_vec())
    };
    Some(TheNarrowing::TheseFormats(
        offered
            .iter()
            .filter(|format| formats.contains(&format.name))
            .map(|format| as_recorded(&format.name))
            .collect(),
    ))
}

/// **The files an amber line counts, named** — *Which ones* on `Design/Transfers
/// § When a download meets trouble`'s *left alone* line and `§ When the job
/// ended badly`'s *may be incomplete* line: *"the count on the row, the names
/// one click away"*. `Specs/Download`: an auto-download *"tells the user which
/// files it left behind"*. The 38's piece 9, item 32.
///
/// **Read from the request's own records** (`Download` N7, `Download` J9), at the moment
/// the link is pressed and never kept: an auto-download's names are its
/// reckoning's entries standing at *left alone* (`Download` R4, P6), relative
/// to the job's folder as the reckoning keys them; a manual download's, under
/// its own *Which ones* (`which-ones-cannot-land`), are the landing paths its
/// name check refused (`Download` P4, P8), which is what that line counts —
/// never under *left alone*, since the two transfer faults after the 38's
/// pieces 7 and 9. **The *may be incomplete* line is composed by
/// nothing** — `Download` E1 step 2's mark is the ending walk's verdict and no
/// record keeps it — so its line is never drawn and this names nothing for it.
#[tauri::command]
pub async fn the_files_a_line_names<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    request: String,
    which: String,
) -> Vec<Verbatim> {
    use tauri::Manager as _;
    let the_run = app
        .try_state::<Arc<TheRunsTransfers>>()
        .map(|the_run| Arc::clone(the_run.inner()));
    let Some(started) = app.try_state::<Mutex<Started>>() else {
        return Vec::new();
    };
    let Ok(started) = started.lock() else {
        return Vec::new();
    };
    let known = what_is_known_now(
        &started,
        &the_run.as_ref().map(|the_run| the_run.what_is_under_way()).unwrap_or_default(),
        the_run.as_deref(),
    );
    let (Some(found), Some(record)) = (known.requests.iter().find(|one| one.id == request), started.record.as_ref())
    else {
        return Vec::new();
    };
    the_files_the_line_names(record.connection(), found, &which)
        .into_iter()
        .map(|name| Verbatim::new(String::from_utf8_lossy(&name).into_owned()))
        .collect()
}

/// [`the_files_a_line_names`]' whole but the reading of the application's
/// state, so a test stands where it stands.
pub(crate) fn the_files_the_line_names(
    connection: &rusqlite::Connection,
    found: &ARequestKnown,
    which: &str,
) -> Vec<Vec<u8>> {
    match (found.kind, which) {
        (TheKind::AutoDownload, "which-ones-were-left-alone" | "which-ones-cannot-land") => {
            let state = if which == "which-ones-cannot-land" {
                smoozy_download::arrangement::WhatSmoozyDid::CannotLand
            } else { smoozy_download::arrangement::WhatSmoozyDid::LeftAlone };
            let Some(job) = Identity::read_back(IdentityKind::Job, &found.id) else {
                return Vec::new();
            };
            DownloadArrangements::over(connection)
                .the_arrangement_for(&job)
                .ok()
                .flatten()
                .map(|arrangement| {
                    arrangement
                        .reckoning
                        .into_iter()
                        .filter(|entry| entry.account.state == state)
                        .map(|entry| entry.predicted_path)
                        .collect()
                })
                .unwrap_or_default()
        }
        (TheKind::Download, "which-ones-cannot-land") => the_request_held(connection, &found.server, &found.id)
            .map(|held| {
                held.entries
                    .into_iter()
                    .filter(|entry| entry.refusal.is_some())
                    .map(|entry| entry.landing_path)
                    .collect()
            })
            .unwrap_or_default(),
        _ => Vec::new(),
    }
}

/// **A failed file's *Try again*, and a changed file's *Upload it again*** —
/// `Download` P3 (*"a failed unit stays failed with its per-file Try again
/// being the user's act"*), `File Browser` K8's offer to send the new version,
/// and `Design/Transfers § A file that failed` and `§ A file that changed
/// while it was going up`. The 38's piece 9, item 17.
///
/// **Each is a new request for that one file, with a new identity** — hole 10's
/// ruling, on `State, identity and recovery` P7: *"Trying again is a new action
/// with a new operation identity."* The first request is left as it ended; its
/// outcome records are immutable. **The new request lands where the file was
/// going**: its destination is the folder its landing path was in, and the
/// file's own name lands under it (`Download` P11, `File Browser` N3).
///
/// - **A download** goes the way Files' *Download* goes
///   ([`the_request_is_put_to_the_user`]): enumerated, written down, and
///   `Download` M6's question put where something already stands — which the
///   page draws here as it draws any download's.
/// - **An upload** waits out `File Browser` K11's quiet period first (*"before
///   any send of a file that changed"*), then goes the way Files' upload goes
///   ([`crate::surface::files::one_file_goes_up_again`]).
///
/// `file` is the landing path the file's row names, as the row was drawn.
#[tauri::command]
pub async fn a_file_is_sent_again<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    request: String,
    file: String,
) -> TheDownloadAnswer {
    use tauri::Manager as _;
    let the_run = app
        .try_state::<Arc<TheRunsTransfers>>()
        .map(|the_run| Arc::clone(the_run.inner()));
    let (known, held) = match app.try_state::<Mutex<Started>>() {
        Some(started) => match started.lock() {
            Ok(started) => {
                let known = what_is_known_now(
                    &started,
                    &the_run.as_ref().map(|the_run| the_run.what_is_under_way()).unwrap_or_default(),
                    the_run.as_deref(),
                );
                let held = known
                    .requests
                    .iter()
                    .find(|one| one.id == request)
                    .and_then(|found| {
                        let connection = started.record.as_ref()?.connection();
                        the_entry_sent_again(connection, found, &request, file.as_bytes())
                    });
                (known, held)
            }
            Err(_) => return refused(words::THE_RECORD_WOULD_NOT_TAKE_IT),
        },
        None => return refused(words::THE_RECORD_WOULD_NOT_TAKE_IT),
    };
    let (Some(found), Some(held)) = (known.requests.iter().find(|one| one.id == request), held) else {
        return refused(words::THE_REQUEST_IS_NO_LONGER_THERE);
    };
    let server = found.server.clone();
    let folder = the_folder_holding_it(&held.landing_path);
    match held.direction {
        Direction::Down => {
            let is = match what_this_server_is(&app, &server) {
                Ok(is) => is,
                Err(said) => return TheDownloadAnswer::DidNotStart { said },
            };
            let Some(point) = the_live_point_of(&app, &server) else {
                return refused(words::THE_SERVER_IS_NOT_CONNECTED);
            };
            let door = TheApplicationsRecord { app: app.clone() };
            let picked = vec![held.source_path.clone()];
            let records = the_jobs_a_browser_request_reaches(
                &mut crate::servers::a_period::TheRecordDirectorysReadingRoute { sessions: &point },
                is.jobs,
                &picked,
            )
            .await;
            let named = what_the_counting_row_names(&picked);
            the_request_is_put_to_the_user(
                &app,
                point,
                &server,
                named,
                crate::downloading::WhatIsBeingDownloaded {
                    server_row: is.server_row,
                    installation: is.installation,
                    destination: folder,
                    picked,
                    run: is.run,
                },
                &door,
                &records,
            )
            .await
        }
        Direction::Up => {
            the_quiet_period_is_waited_out(&held.source_path).await;
            match crate::surface::files::one_file_goes_up_again(
                app.clone(),
                server,
                folder,
                held.source_path,
                held.the_earlier_answer,
            )
            .await
            {
                crate::surface::files::TheUploadGoes::Underway { request } => {
                    TheDownloadAnswer::ItStarted { request }
                }
                crate::surface::files::TheUploadGoes::DidNotStart { said } => {
                    TheDownloadAnswer::TheUploadDidNotStart { said }
                }
                _ => TheDownloadAnswer::TheUserStoppedIt,
            }
        }
    }
}

/// **The one entry a *Try again* or *Upload it again* is about**, read from
/// the request's own record: where it came from, where it was landing, and —
/// for an upload — the answer the first request gave `File Browser` U4.
struct AnEntrySentAgain {
    direction: Direction,
    source_path: Vec<u8>,
    landing_path: Vec<u8>,
    the_earlier_answer: Option<CollisionAnswer>,
}

/// **The entry whose landing path is the one the row names**, from the
/// request's own record — and only an entry whose unit failed, which is the
/// one kind of row that offers either act.
fn the_entry_sent_again(
    connection: &rusqlite::Connection,
    found: &ARequestKnown,
    request: &str,
    landing: &[u8],
) -> Option<AnEntrySentAgain> {
    use smoozy_state::outcomes::UnitOutcome;
    let row = Identity::read_back(IdentityKind::LocalServer, &found.server)?;
    match found.kind {
        TheKind::Download => {
            let held = the_request_held(connection, &found.server, request)?;
            let entry = held
                .entries
                .iter()
                .find(|entry| entry.landing_path == landing && entry.outcome == Some(UnitOutcome::Failed))?;
            Some(AnEntrySentAgain {
                direction: Direction::Down,
                source_path: entry.source_path.clone(),
                landing_path: entry.landing_path.clone(),
                the_earlier_answer: None,
            })
        }
        TheKind::Upload => {
            use smoozy_file_browser::request::KeepsTheRequest as _;
            let held = crate::record::upload_requests::UploadRequests::over(connection)
                .every_request_for(&row)
                .ok()?
                .into_iter()
                .find(|one| one.record.as_text() == request)?;
            let entry = held
                .entries
                .iter()
                .find(|entry| entry.landing_path == landing && entry.outcome == Some(UnitOutcome::Failed))?;
            Some(AnEntrySentAgain {
                direction: Direction::Up,
                source_path: entry.source_path.clone(),
                landing_path: entry.landing_path.clone(),
                the_earlier_answer: held.collision_answer.clone(),
            })
        }
        _ => None,
    }
}

/// The folder a landing path is in, as bytes.
fn the_folder_holding_it(landing: &[u8]) -> Vec<u8> {
    match landing.iter().rposition(|byte| *byte == b'/') {
        Some(0) => b"/".to_vec(),
        Some(at) => landing[..at].to_vec(),
        None => landing.to_vec(),
    }
}

/// **`File Browser` K11: the send of a file that changed starts when its path
/// identity has held unchanged for two seconds**, polled at the accounting
/// period — `smoozy_file_browser::continuation::TheQuietPeriod`, which had no
/// caller until the 38's piece 9. **It waits no longer than `Server protocol
/// and transfers` C19's deadline**: a file still being written past that is
/// sent anyway, and `File Browser` K8's watch and K9's gate stop it again if it
/// moves — which is the protection that matters; `File Browser` K11 only spares the user a
/// second stop.
async fn the_quiet_period_is_waited_out(source: &[u8]) {
    use smoozy_file_browser::continuation::{ReadsAPathIdentity as _, TheQuietPeriod, WhetherTheSendMayStart};
    let Ok(path) = std::str::from_utf8(source).map(str::to_string) else {
        return;
    };
    let mut quiet = TheQuietPeriod::new();
    let mut adapter = crate::transfers::ThePlatformFileAdapter;
    let began = std::time::Instant::now();
    while began.elapsed() < crate::downloading::THE_INACTIVITY_DEADLINE {
        if quiet.a_reading(adapter.read_by_path(&path), std::time::SystemTime::now()) == WhetherTheSendMayStart::Yes {
            return;
        }
        tokio::time::sleep(smoozy_server_protocol::transfers::moving::THE_ACCOUNTING_PERIOD).await;
    }
}

/// **What a counting row names** — the folder picked, by its own name, and
/// how many more where several were picked (`Design/Transfers`' paragraph of
/// 2026-10-01). Step 10c's twelfth-pass fix piece 3.
fn what_the_counting_row_names(picked: &[Vec<u8>]) -> String {
    let Some(first) = picked.iter().find(|one| !one.is_empty()) else {
        return String::new();
    };
    let first = trim_the_trailing_slash(first);
    let name = first.rsplit(|byte| *byte == b'/').next().unwrap_or(first);
    let name = String::from_utf8_lossy(if name.is_empty() { first } else { name }).into_owned();
    match picked.iter().filter(|one| !one.is_empty()).count() {
        0 | 1 => name,
        many => format!("{name} {} {} {}", words::AND, many - 1, words::MORE),
    }
}

/// A refusal in one sentence, through the typography point.
fn refused(said: &'static str) -> TheDownloadAnswer {
    TheDownloadAnswer::DidNotStart {
        said: Prose::new(said),
    }
}

/// **The server's own point, where a connection is live** — `Server protocol
/// and transfers` T5, lent by `src/servers/sessions.rs`.
fn the_live_point_of<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    server: &str,
) -> Option<crate::servers::sessions::ThisServersSessions> {
    use tauri::Manager;
    let periods = app
        .try_state::<Arc<crate::servers::a_period::ThePeriodsThisRun>>()
        .map(|periods| Arc::clone(periods.inner()));
    periods
        .as_deref()
        .and_then(|periods| periods.the_sessions_of(server))
        .filter(crate::servers::sessions::ThisServersSessions::a_connection_is_live)
}

/// Where this request lands.
enum TheDestination {
    It(Vec<u8>),
    /// The user closed the folder panel, which is an ordinary answer.
    TheUserChoseNoFolder,
    /// **No panel was opened at all**, which is [`crate::picker`]'s own F-11
    /// separation and never read as a user who changed their mind.
    NoPanelOpened,
}

/// **`Download` H2's destination: the standing folder, or the per-request
/// override the spec grants.**
///
/// **The panel is put up off the async runtime's own threads**, because
/// [`crate::picker::ask`] blocks the thread it is called on until the user has
/// acted (this crate's §3.2).
async fn the_destination_chosen<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    to: bool,
    standing: Vec<u8>,
) -> TheDestination {
    if !to {
        return TheDestination::It(standing);
    }
    let the_panels = app.clone();
    let picked = tauri::async_runtime::spawn_blocking(move || {
        crate::picker::ask(
            &the_panels,
            &crate::picker::Asking {
                what: crate::picker::WhatMayBeChosen::Folders,
                how_many: crate::picker::HowMany::One,
                starting_at: None,
                title: Prose::new(words::WHERE_THE_DOWNLOAD_GOES),
            },
        )
    })
    .await;
    let destination = match picked {
        Ok(crate::picker::Picked::TheUserChose(chosen)) => chosen
            .first()
            .map(|one| {
                TheDestination::It(
                    one.as_the_user_gave_it()
                        .as_os_str()
                        .as_encoded_bytes()
                        .to_vec(),
                )
            })
            .unwrap_or(TheDestination::TheUserChoseNoFolder),
        Ok(crate::picker::Picked::TheUserChoseNothing) => TheDestination::TheUserChoseNoFolder,
        Ok(crate::picker::Picked::ThereIsNoPanelForThisAsk(_)) | Err(_) => {
            TheDestination::NoPanelOpened
        }
    };
    {
        use tauri::Manager as _;
        if let Some(queue) = app.try_state::<Arc<TheOneQueue>>() {
            the_chosen_folder_is_told(&queue, &destination);
        }
    }
    destination
}

/// **[`A_DOWNLOAD_FOLDER_WAS_CHOSEN`], posted only where the panel gave a
/// folder** — the Creator's fifth note 7, piece Q. A panel closed without one,
/// or no panel at all, tells the page nothing: the first leaves the user where
/// they pressed, and the second is said on Transfers when the request answers.
fn the_chosen_folder_is_told(queue: &TheOneQueue, destination: &TheDestination) {
    if let TheDestination::It(_) = destination {
        let _ = queue.read_then_post::<(), std::convert::Infallible>(
            || Ok(()),
            |_| Event::about(Subject::TheRun, A_DOWNLOAD_FOLDER_WAS_CHOSEN),
        );
    }
}

/// **The tail both manual downloads share** — `Download` M1's durable record
/// and M2's walk behind it, M6's question in front of it.
///
/// **One composition for both ways in**, so the browser's request and the job's
/// cannot drift apart in what they write down or what they ask.
async fn the_request_is_put_to_the_user<R, J>(
    app: &tauri::AppHandle<R>,
    point: crate::servers::sessions::ThisServersSessions,
    server: &str,
    named: String,
    asked: crate::downloading::WhatIsBeingDownloaded,
    door: &TheApplicationsRecord<R>,
    records: &J,
) -> TheDownloadAnswer
where
    R: tauri::Runtime,
    J: smoozy_download::acting::enumerating::ReadsTheLocalJobRecords,
{
    use tauri::Manager as _;
    // **On Transfers from the press** — step 10c's twelfth-pass fix piece 3,
    // `Design/Transfers`' paragraph of 2026-10-01. The run holds the count
    // until the request is written; the request carries the count's identity.
    let the_run = app
        .try_state::<Arc<TheRunsTransfers>>()
        .map(|the_run| Arc::clone(the_run.inner()));
    let ready = match (&the_run, Identity::issue(IdentityKind::Operation)) {
        (Some(the_run), Ok(request)) => {
            let id = request.as_text();
            let ready = crate::downloading::a_download_is_counted_and_prepared(
                asked,
                crate::downloading::TheCountIsShown { the_run, request, named },
                &point,
                door,
                records,
            )
            .await;
            if matches!(ready, Err(crate::downloading::WhyTheDownloadDidNotStart::TheUserStoppedTheCount)) {
                if let Ok(mut stopped) = downloads_stopped_before_their_dispositions().lock() {
                    stopped.remove(&id);
                }
            }
            ready
        }
        _ => crate::downloading::a_download_is_prepared(asked, &point, door, records).await,
    };
    let mut ready = match ready {
        Ok(ready) => ready,
        Err(crate::downloading::WhyTheDownloadDidNotStart::TheUserStoppedTheCount) => {
            return TheDownloadAnswer::TheUserStoppedIt;
        }
        Err(why) => return refused(why_it_did_not_start(&why)),
    };

    let id = ready.request.record.as_text();
    if let Some(answer) = what_a_stop_before_its_dispositions_answers(&mut ready, door) {
        return answer;
    }
    if ready.the_pre_flight.the_request_stops_and_asks() {
        let asking = the_question(&id, &ready.the_pre_flight);
        if let Ok(mut waiting) = downloads_waiting_on_their_question().lock() {
            waiting.push((id, server.to_string(), ready));
        }
        return asking;
    }
    // **M6's narrowing: no question, no surface, and the transfers start.**
    if crate::downloading::the_request_goes_without_a_question(&mut ready, door).is_err() {
        return refused(words::THE_RECORD_WOULD_NOT_TAKE_IT);
    }
    the_files_come_down(app.clone(), point, ready);
    TheDownloadAnswer::ItStarted {
        request: Verbatim::new(id),
    }
}

/// **Stopped from Transfers while its first read of the destination had not
/// returned** — step 10c's eleventh-pass fix piece 1: written as `Download`
/// M6's *Stop* the moment that read returns, and nothing is asked or fetched.
/// `None` where the user did not stop it.
fn what_a_stop_before_its_dispositions_answers(
    ready: &mut crate::downloading::ADownloadReadyToRun,
    door: &dyn crate::downloading::TheRecordADownloadWrites,
) -> Option<TheDownloadAnswer> {
    let id = ready.request.record.as_text();
    let stopped = downloads_stopped_before_their_dispositions()
        .lock()
        .is_ok_and(|mut stopped| stopped.remove(&id));
    if !stopped {
        return None;
    }
    if crate::downloading::the_answer_is_applied(ready, door, CollisionAnswer::Stop).is_err() {
        return Some(refused(words::THE_RECORD_WOULD_NOT_TAKE_IT));
    }
    Some(TheDownloadAnswer::ItStarted {
        request: Verbatim::new(id),
    })
}

/// **Why a download did not start, in words** — each names what did not happen,
/// which is the shape `src/surface/files.rs` already says an upload's refusals
/// in. **None of them says nothing moves a transfer yet**, because one does.
fn why_it_did_not_start(why: &crate::downloading::WhyTheDownloadDidNotStart) -> &'static str {
    use crate::downloading::WhyTheDownloadDidNotStart as Why;
    match why {
        Why::ThereIsNothingToFetch => words::NOTHING_IS_SELECTED,
        Why::ThereIsNowhereToPutIt(_) => words::THERE_IS_NO_DOWNLOAD_FOLDER,
        Why::TheServerLentNothing(_) => words::THE_SERVER_IS_NOT_CONNECTED,
        Why::NothingPickedCouldBeRead => words::NOTHING_PICKED_COULD_BE_READ,
        Why::TheRecordWouldNotTakeIt(_) => words::THE_RECORD_WOULD_NOT_TAKE_IT,
        // **Answered as `TheUserStoppedIt` before it reaches here**, and never
        // said: the user asked for it.
        Why::TheUserStoppedTheCount => words::NOTHING_IS_DOWNLOADING_NOW,
    }
}

/// **`Download` M6's four answers, given once and applied to the enumeration
/// once.**
#[tauri::command]
pub async fn the_download_question_was_answered<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    request: String,
    answer: String,
) -> TheDownloadAnswer {
    let taken = downloads_waiting_on_their_question()
        .lock()
        .ok()
        .and_then(|mut waiting| {
            waiting
                .iter()
                .position(|(id, _, _)| *id == request)
                .map(|at| waiting.remove(at))
        });
    let Some((id, server, mut ready)) = taken else {
        return refused(words::THE_REQUEST_IS_NO_LONGER_WAITING);
    };
    let answer = match answer.as_str() {
        "replace" => Some(CollisionAnswer::Replace),
        "keep-both" => Some(CollisionAnswer::KeepBoth),
        "skip-existing" => Some(CollisionAnswer::SkipExisting),
        // **`Download` J10's go-ahead, offered only where nothing is already
        // there** — so nothing it fetches is written over.
        THE_GO_AHEAD if ready.the_pre_flight.the_headline() == 0 => None,
        _ => Some(CollisionAnswer::Stop),
    };
    let door = TheApplicationsRecord { app: app.clone() };
    let written = match answer {
        Some(answer) => crate::downloading::the_answer_is_applied(&mut ready, &door, answer),
        // **No collision to answer**, so the dispositions are M6's narrowing's.
        None => crate::downloading::the_request_goes_without_a_question(&mut ready, &door),
    };
    if written.is_err() {
        return refused(words::THE_RECORD_WOULD_NOT_TAKE_IT);
    }
    if answer == Some(CollisionAnswer::Stop) {
        // **Stop is the request not proceeding at all**: no entry takes a
        // disposition and nothing is fetched.
        return TheDownloadAnswer::ItStarted {
            request: Verbatim::new(id),
        };
    }
    let Some(point) = the_live_point_of(&app, &server) else {
        return refused(words::THE_SERVER_IS_NOT_CONNECTED);
    };
    the_files_come_down(app, point, ready);
    TheDownloadAnswer::ItStarted {
        request: Verbatim::new(id),
    }
}

/// **M6's question, composed on the application side** — A6 and X1: every
/// string of prose passes through the typography point and the interface
/// accepts prose from nowhere else.
///
/// **The three are named beside one another**, because M6 says the user answers
/// once knowing all of it.
///
/// **Where nothing is already there, the box asks nothing about it** —
/// `Design/Transfers § A download meeting files already there`, the paragraph
/// of 2026-10-01: no count, and *Download* and *Stop* in place of the four
/// answers, *Download* being `Download` J10's go-ahead. **The case this was
/// first written for** was a collision, which M6 stops for. **The case it now
/// also meets** is a request M6 puts in front of the user for a cannot-vouch
/// mark or a refusal alone; on the Vast pass it read *0 files are already
/// there* above *Replace · Keep both · Skip existing* (step 10c's Vast-pass
/// fix piece 1).
fn the_question(id: &str, said: &WhatThePreFlightHasToSay) -> TheDownloadAnswer {
    let an_answer = |label: &'static str, act: &'static str| AControl {
        label: Prose::new(label),
        act: Verbatim::new(act.to_string()),
    };
    let something_is_already_there = said.the_headline() > 0;
    TheDownloadAnswer::ItAsks {
        request: Verbatim::new(id.to_string()),
        headline: something_is_already_there.then(|| {
            Words::after(
                said.the_headline().to_string(),
                by_count(
                    said.the_headline(),
                    words::FILE_IS_ALREADY_THERE,
                    words::FILES_ARE_ALREADY_THERE,
                ),
            )
        }),
        cannot_vouch_for: (said.cannot_vouch_for > 0).then(|| {
            Words::after(
                said.cannot_vouch_for.to_string(),
                by_count(
                    said.cannot_vouch_for,
                    words::MAY_BE_INCOMPLETE_ONE,
                    words::MAY_BE_INCOMPLETE,
                ),
            )
        }),
        refusals: (said.refusals > 0).then(|| {
            Words::after(
                said.refusals.to_string(),
                by_count(
                    said.refusals,
                    words::COULD_NOT_BE_READ_ONE,
                    words::COULD_NOT_BE_READ,
                ),
            )
        }),
        answers: if something_is_already_there {
            vec![
                an_answer(words::REPLACE, "replace"),
                an_answer(words::KEEP_BOTH, "keep-both"),
                an_answer(words::SKIP_EXISTING, "skip-existing"),
                // `web/transfers.js` sets this one apart by its act
                // (`STOP_THE_DOWNLOAD`). Copy carefully.
                an_answer(words::STOP, "stop"),
            ]
        } else {
            vec![
                an_answer(words::DOWNLOAD, THE_GO_AHEAD),
                an_answer(words::STOP, "stop"),
            ]
        },
    }
}

/// **The act `Download` J10's go-ahead sends**, offered where nothing is
/// already there. Step 10c's Vast-pass fix piece 1.
const THE_GO_AHEAD: &str = "download";

/// **The files, off the drawing thread** — P5: nothing that takes time runs on
/// it.
fn the_files_come_down<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    point: crate::servers::sessions::ThisServersSessions,
    mut ready: crate::downloading::ADownloadReadyToRun,
) {
    use tauri::Manager;
    let transfers = app
        .try_state::<Arc<TheRunsTransfers>>()
        .map(|held| Arc::clone(held.inner()));
    let Some(transfers) = transfers else {
        return;
    };
    let door = TheApplicationsRecord { app: app.clone() };
    tauri::async_runtime::spawn(async move {
        let _ = crate::downloading::the_download_runs(&mut ready, &point, &transfers, &door).await;
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::downloading::AFileMoving;
    use std::path::Path;

    // ── The 38's piece 9 ────────────────────────────────────────────────────

    /// **An opener that answers what it is told and keeps what it was asked.**
    struct AnOpener {
        opens: bool,
        asked: Vec<Vec<u8>>,
    }

    impl OpensAFolder for AnOpener {
        fn open_this_folder(&mut self, folder: &[u8]) -> bool {
            self.asked.push(folder.to_vec());
            self.opens
        }
    }

    /// **A queue or History entry's *Files on your computer*: `Download` H5's
    /// set opened, every place, and `Download` H7a's two readings said where
    /// it is empty** (`Queue & History` N18). Item 9: until then the control
    /// answered *not finished*.
    #[test]
    fn an_entrys_files_on_your_computer_opens_the_set_or_says_which_empty_reading() {
        let folder: &[u8] = b"/smoozy/jobs/abc/outputs";
        // **The place is really there** since the Creator's thirteenth notes,
        // note 4: a place that is gone is never handed to the opener.
        let x = a_real_folder("entry");
        std::fs::create_dir_all(x.join("abc night")).expect("the job's folder");
        let x_bytes = in_bytes(&x);
        // Record the same mixed-separator path a real Windows download writes.
        let landed = smoozy_download::fetching::the_landing_path(&x_bytes, b"abc night/f_0001.exr");
        let mut whole = a_download_that_landed(
            &x_bytes,
            &[(b"/smoozy/jobs/abc/outputs/f_0001.exr", &landed)],
        );
        whole.server_row = Identity::issue(IdentityKind::LocalServer).expect("random bytes");
        let picked = a_download_that_landed(
            b"/Users/artist/y",
            &[(b"/smoozy/jobs/abc/outputs/f_0002.exr", b"/Users/artist/y/f_0002.exr")],
        );

        // **A whole-job download puts its folder in the set, and it opens.**
        let set = smoozy_download::landing::the_set_of_landing_places(
            None,
            &crate::downloading::TheReceivingSystemsRulesHere,
            None,
            std::slice::from_ref(&whole),
            folder,
        );
        assert_eq!(set.iter().map(|place| the_stored_folder(place)).collect::<Vec<_>>(), vec![x.join("abc night")]);
        let mut opener = AnOpener { opens: true, asked: Vec::new() };
        assert_eq!(the_set_is_opened(&set, std::slice::from_ref(&whole), folder, &mut opener), None);
        assert_eq!(opener.asked, set);

        // **`Download` H8: a place that will not open is said**, never dropped.
        let mut refusing = AnOpener { opens: false, asked: Vec::new() };
        assert_eq!(
            the_set_is_opened(&set, std::slice::from_ref(&whole), folder, &mut refusing),
            Some(words::COULD_NOT_OPEN_THE_FOLDER)
        );

        // **`Download` H7a: nothing came home, or only what was picked out.**
        let mut nothing = AnOpener { opens: true, asked: Vec::new() };
        assert_eq!(
            the_set_is_opened(&[], &[], folder, &mut nothing),
            Some(words::NOTHING_OF_THIS_JOB_IS_DOWNLOADED)
        );
        assert_eq!(
            the_set_is_opened(&[], std::slice::from_ref(&picked), folder, &mut nothing),
            Some(words::ONLY_PARTS_WERE_DOWNLOADED)
        );
        assert!(nothing.asked.is_empty(), "an empty set opened something");
        let _ = std::fs::remove_dir_all(x);
    }

    /// A folder of this test's own under the system's temporary folder, made
    /// empty.
    fn a_real_folder(name: &str) -> PathBuf {
        let mut folder = std::env::temp_dir();
        folder.push(format!("smoozy-transfers-test-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a folder for the test");
        folder
    }

    fn in_bytes(path: &Path) -> Vec<u8> {
        path.as_os_str().as_encoded_bytes().to_vec()
    }

    /// **The Creator's thirteenth notes, note 4: a queue or History entry's
    /// *Files on your computer* on a download folder deleted by hand.** The
    /// place that is gone is not handed to the opener and is said gone; the
    /// other place in the job's set still opens. Until then the reveal opened
    /// the deleted folder's parent and nothing was said.
    #[test]
    fn a_deleted_folder_in_a_jobs_set_is_not_opened_and_is_said_gone() {
        let root = a_real_folder("note-4-set");
        let kept = root.join("first folder/abc night");
        let deleted = root.join("second folder/abc night");
        std::fs::create_dir_all(&kept).expect("the first place");
        std::fs::create_dir_all(&deleted).expect("the second place");
        std::fs::remove_dir_all(&deleted).expect("the user deletes the second place");
        let set = vec![in_bytes(&kept), in_bytes(&deleted)];

        let mut opener = AnOpener { opens: true, asked: Vec::new() };
        assert_eq!(
            the_set_is_opened(&set, &[], b"/smoozy/jobs/abc/outputs", &mut opener),
            Some(words::THE_FOLDER_IS_NO_LONGER_THERE)
        );
        assert_eq!(opener.asked, vec![in_bytes(&kept)], "the place that is gone was opened, or the other was not");

        // **A place that is there and will not open is still *could not
        // open***, never *no longer there*.
        let mut refusing = AnOpener { opens: false, asked: Vec::new() };
        assert_eq!(
            the_set_is_opened(&set[..1], &[], b"/smoozy/jobs/abc/outputs", &mut refusing),
            Some(words::COULD_NOT_OPEN_THE_FOLDER)
        );
        let _ = std::fs::remove_dir_all(root);
    }

    /// **Note 4, the reveal's own fallback**: a place that is gone is refused
    /// by the first act, and its parent is not opened instead.
    #[test]
    fn a_place_that_is_gone_does_not_open_its_parent() {
        use crate::reveal::Revealed;
        let root = a_real_folder("note-4-shown");
        let gone = root.join("abc night");
        std::fs::create_dir_all(&gone).expect("the place");
        std::fs::remove_dir_all(&gone).expect("the user deletes it");
        let shown = a_place_is_shown(&gone, |_| Revealed::ItCouldNotBeOpened, |_| {
            panic!("the parent of a place that is gone was opened")
        });
        assert_eq!(shown, Revealed::ItCouldNotBeOpened);
        let _ = std::fs::remove_dir_all(root);
    }

    /// **Note 4 on Transfers' *Files on your computer***: the one thing a
    /// download brought, gone, is said gone — a file as a file, a folder as a
    /// folder — and so is a destination that is gone. **Nothing reaches the
    /// system here**: the reveal refuses a path that is not there before it
    /// asks, and the parent is no longer opened.
    #[test]
    fn transfers_files_on_your_computer_says_a_deleted_place_is_gone() {
        let root = a_real_folder("note-4-transfers");
        let x = root.join("x");
        std::fs::create_dir_all(&x).expect("the destination");
        let mut row = a_request(TheKind::Download, WhatItIsOf::AFolder { folder: "x".into() });
        row.destination = x.clone();
        let known = known(vec![row]);

        let one_file = a_download_that_landed(
            &in_bytes(&x),
            &[(b"/smoozy/jobs/abc/outputs/f_0001.exr", &smoozy_download::fetching::the_landing_path(&in_bytes(&x), b"f_0001.exr"))],
        );
        assert_eq!(
            the_files_are_shown(&known, "one", Some(&one_file)),
            Some(Prose::new(words::THE_FILE_IS_NO_LONGER_THERE))
        );

        let whole_job = a_download_that_landed(
            &in_bytes(&x),
            &[(b"/smoozy/jobs/abc/outputs/f_0001.exr", &smoozy_download::fetching::the_landing_path(&in_bytes(&x), b"abc night/f_0001.exr"))],
        );
        assert_eq!(
            the_files_are_shown(&known, "one", Some(&whole_job)),
            Some(Prose::new(words::THE_FOLDER_IS_NO_LONGER_THERE))
        );

        std::fs::remove_dir_all(&x).expect("the user deletes the destination");
        assert_eq!(
            the_files_are_shown(&known, "one", None),
            Some(Prose::new(words::THE_FOLDER_IS_NO_LONGER_THERE))
        );
        let _ = std::fs::remove_dir_all(root);
    }

    /// **The Creator's eighteenth notes, note 15: Transfers' *Files on your
    /// computer* knows when the files are gone, as Queue and History do.**
    /// Two cases, each with openers that would open anything they were asked
    /// for, so only Smoozy's own look decides:
    ///
    /// - the one thing a download brought, deleted: the system is never
    ///   asked for it (Queue and History's own check, `OnlyWhatIsThere`), and
    ///   it is said gone;
    /// - several things, every one deleted, in a destination that is still
    ///   there: the destination is not opened, and the line says *These files
    ///   are no longer there.* (`Design/Queue & History`, from 2026-10-25; it
    ///   read *This folder is no longer there.*, naming a folder that is there).
    ///   Where one of them is still there, the destination opens as before.
    ///
    /// Until then the first asked the system first, and the second opened the
    /// existing folder and said nothing.
    #[test]
    fn eighteenth_notes_15_transfers_knows_when_what_a_download_brought_is_gone() {
        let root = a_real_folder("eighteenth-15");
        let x = root.join("x");
        std::fs::create_dir_all(&x).expect("the destination");
        let mut row = a_request(TheKind::Download, WhatItIsOf::AFolder { folder: "x".into() });
        row.destination = x.clone();
        let known = known(vec![row]);
        let mut folders_opened: Vec<PathBuf> = Vec::new();

        let one_file = a_download_that_landed(
            &in_bytes(&x),
            &[(b"/smoozy/jobs/abc/outputs/f_0001.exr", &smoozy_download::fetching::the_landing_path(&in_bytes(&x), b"f_0001.exr"))],
        );
        let mut select = AnOpener { opens: true, asked: Vec::new() };
        assert_eq!(
            the_files_are_shown_through(&known, "one", Some(&one_file), &mut select, &mut |at| {
                folders_opened.push(at.to_path_buf());
                true
            }),
            Some(Prose::new(words::THE_FILE_IS_NO_LONGER_THERE)),
            "a deleted file was not said gone"
        );
        assert!(select.asked.is_empty(), "the system was asked for a file that is gone: {:?}", select.asked);

        let b = x.join("b.exr");
        let several = a_download_that_landed(
            &in_bytes(&x),
            &[
                (b"/smoozy/jobs/abc/outputs/a.exr", &smoozy_download::fetching::the_landing_path(&in_bytes(&x), b"a.exr")),
                (b"/smoozy/jobs/abc/outputs/b.exr", &smoozy_download::fetching::the_landing_path(&in_bytes(&x), b"b.exr")),
            ],
        );
        let mut select = AnOpener { opens: true, asked: Vec::new() };
        assert_eq!(
            the_files_are_shown_through(&known, "one", Some(&several), &mut select, &mut |at| {
                folders_opened.push(at.to_path_buf());
                true
            }),
            Some(Prose::new(words::THE_FILES_ARE_NO_LONGER_THERE)),
            "several deleted things in a folder still there were not said gone, as files (the small owed fixes, item 6)"
        );
        assert!(folders_opened.is_empty(), "the destination was opened over files that are gone: {folders_opened:?}");

        std::fs::write(&b, b"frame").expect("one of them is there");
        assert_eq!(
            the_files_are_shown_through(&known, "one", Some(&several), &mut select, &mut |at| {
                folders_opened.push(at.to_path_buf());
                true
            }),
            None,
            "a destination still holding what the download brought did not open"
        );
        assert_eq!(folders_opened, vec![x.clone()]);
        assert!(select.asked.is_empty());
        let _ = std::fs::remove_dir_all(root);
    }

    /// **The box after Submit, read**, for a job on `row` whose reading offers
    /// `formats` — what [`the_jobs_box_is_read`] would have read, with a
    /// standing folder and no proposal.
    fn a_jobs_box(
        row: &Identity,
        formats: Vec<AFormatKnown>,
        arrangement: Option<smoozy_download::arrangement::AnArrangement>,
    ) -> TheJobsBox {
        let mut record = crate::surface::render_settings::WhatTheRecordHolds::read_now(None);
        record.the_proposed_folder = None;
        record.the_standing_folder = Some(b"/Users/artist/Renders".to_vec());
        let files_in_the_job = formats.iter().map(|one| one.files).sum();
        TheJobsBox {
            is: WhatTheJobIs {
                server: row.as_text(),
                server_row: row.clone(),
                the_jobs_output_folder: b"/smoozy/jobs/abc/outputs".to_vec(),
                label: None,
                called: "abc".into(),
                destination: Vec::new(),
                installation: None,
                run: Identity::issue(IdentityKind::Run).expect("random bytes"),
                the_jobs_record_directory: Vec::new(),
                attempts: 0,
                the_blend_path: None,
                the_setup_it_was_submitted_with: None,
            },
            knows: WhatTheBoxKnows {
                formats: Some(formats),
                files_in_the_job,
                the_standing_folder: record.the_standing_folder.clone(),
                the_formats_chosen: None,
                the_folder_chosen_for_this_job: arrangement.as_ref().map(|one| one.destination.clone()),
            },
            arrangement,
            outputs: None,
            record,
        }
    }

    /// **The box after Submit: the folder fixed while the arrangement is on,
    /// `Download` R11's telling before a switch-on moves it, and the narrowing
    /// the ticks make** — item 9.
    #[test]
    fn the_box_after_submit_keeps_the_folder_and_says_what_a_switch_on_costs() {
        use smoozy_download::arrangement::{AReckoningEntry, AnArrangement, WhatSmoozyDid};
        let job = Identity::issue(IdentityKind::Job).expect("random bytes");
        let row = Identity::issue(IdentityKind::LocalServer).expect("random bytes");
        let formats = vec![
            AFormatKnown { name: "OPEN_EXR".into(), where_in_the_job: "f_".into(), at_the_top: false, files: 240, chosen: true },
            AFormatKnown { name: "PNG".into(), where_in_the_job: "p_".into(), at_the_top: false, files: 240, chosen: true },
        ];
        let a_box = |arrangement: Option<AnArrangement>| a_jobs_box(&row, formats.clone(), arrangement);
        let mut on = AnArrangement::switched_on(job.clone(), row.clone(), b"/Volumes/Work/kitchen".to_vec(), TheNarrowing::Everything);
        on.reckoning.push(AReckoningEntry::acted_on(b"f_0001.exr".to_vec(), Some(1), WhatSmoozyDid::Fetched, b"/Volumes/Work/kitchen".to_vec()));

        // **On: one folder, the one it was switched on with, and no other.**
        let the_box = the_jobs_box_is_composed(&a_box(Some(on.clone())));
        assert!(the_box.the_folder_is_fixed);
        assert_eq!(the_box.folders.len(), 1);
        assert_eq!(the_box.folders[0].folder.as_str(), "/Volumes/Work/kitchen");
        assert!(the_box.before_it_moves.is_none());

        // **Off, with files already downloaded: R11's telling, before.**
        let mut off = on.clone();
        off.switched_off();
        let the_box = the_jobs_box_is_composed(&a_box(Some(off.clone())));
        assert!(!the_box.the_folder_is_fixed);
        assert!(the_box.folders.len() >= 2, "the standing folder is offered too");
        let told = the_box.before_it_moves.expect("the telling");
        assert_eq!(told.values(), vec!["/Volumes/Work/kitchen"]);
        // **None, or off with nothing downloaded: nothing to tell.**
        assert!(the_jobs_box_is_composed(&a_box(None)).before_it_moves.is_none());

        // **The ticks: all of them is everything; some are kept as recorded,
        // in the box's order; a name not offered is no narrowing at all.**
        let every = vec!["PNG".to_string(), "OPEN_EXR".to_string()];
        assert_eq!(the_narrowing_the_ticks_make(Some(&formats), None, None, &every), Some(TheNarrowing::Everything));
        assert_eq!(
            the_narrowing_the_ticks_make(Some(&formats), None, None, &["PNG".to_string()]),
            Some(TheNarrowing::TheseFormats(vec![b"PNG".to_vec()]))
        );
        assert_eq!(the_narrowing_the_ticks_make(Some(&formats), None, None, &["TIFF".to_string()]), None);
        assert_eq!(the_narrowing_the_ticks_make(None, None, None, &[]), Some(TheNarrowing::Everything));
        assert_eq!(the_narrowing_the_ticks_make(None, None, None, &["PNG".to_string()]), None);

        // **What the choice leaves**: on, the folder is kept and the narrowing
        // is new; off, both are reset and it is on; none, it is made.
        let narrowed = TheNarrowing::TheseFormats(vec![b"PNG".to_vec()]);
        let after = the_arrangement_after_the_choice(Some(on.clone()), &job, &row, b"/elsewhere".to_vec(), narrowed.clone());
        assert_eq!(after.destination, b"/Volumes/Work/kitchen".to_vec());
        assert_eq!(after.narrowing, narrowed);
        assert_eq!(after.reckoning.len(), 1, "the reckoning is kept");
        let after = the_arrangement_after_the_choice(Some(off), &job, &row, b"/elsewhere".to_vec(), narrowed.clone());
        assert_eq!(after.desired_state, DesiredState::On);
        assert_eq!(after.destination, b"/elsewhere".to_vec());
        assert_eq!(after.reckoning.len(), 1, "what is already downloaded is not downloaded again");
        let made = the_arrangement_after_the_choice(None, &job, &row, b"/elsewhere".to_vec(), TheNarrowing::Everything);
        assert_eq!((made.job, made.server_row, made.desired_state), (job, row, DesiredState::On));
    }

    /// **The box ruling of 2026-10-17, in the queue's box: with no list, it names the narrowing that
    /// goes on, and Done keeps it.** The queue's box has no list until the
    /// job's reading lands — before the render starts, or with the server not
    /// connected — and it said *Everything downloads.* while an arrangement
    /// that is on went on downloading only what it was narrowed to; Done then
    /// widened it to everything, which nobody chose. **An arrangement that is
    /// off is reset by the switch-on Done makes** (`Download` R11), so there
    /// *Everything downloads.* is what Done does and it stays. **A box of one
    /// format narrows nothing** (`Design/Render Settings`): its Done made a
    /// narrowing to no format at all. **Each fails against the box as it
    /// stood.**
    #[test]
    fn the_queues_box_with_no_list_names_the_narrowing_that_stands() {
        use smoozy_download::arrangement::AnArrangement;
        let job = Identity::issue(IdentityKind::Job).expect("random bytes");
        let row = Identity::issue(IdentityKind::LocalServer).expect("random bytes");
        let exr = TheNarrowing::TheseFormats(vec![b"OPEN_EXR".to_vec()]);
        let on = AnArrangement::switched_on(job.clone(), row.clone(), b"/Volumes/Work/kitchen".to_vec(), exr.clone());
        let mut off = on.clone();
        off.switched_off();
        let unread = |arrangement: Option<AnArrangement>| {
            let mut read = a_jobs_box(&row, Vec::new(), arrangement.clone());
            read.knows = what_the_jobs_box_knows(&read.is, arrangement.as_ref(), None, &read.record);
            read
        };
        let says = |read: &TheJobsBox| {
            crate::surface::render_settings::tests::what_a_reading_says(
                &serde_json::to_value(&the_jobs_box_is_composed(read).reading).expect("drawn"),
            )
        };

        // **On, narrowed: named.**
        let (drawn, said) = says(&unread(Some(on.clone())));
        assert_eq!(drawn, "not-read", "{said:?}");
        assert!(!said.iter().any(|one| one == words::EVERYTHING_DOWNLOADS), "{said:?}");
        assert!(said.iter().any(|one| one == "EXR files are downloaded."), "{said:?}");
        assert_eq!(the_narrowing_chosen(&unread(Some(on.clone())), &[]), Some(exr.clone()), "Done widened it");

        // **Off, or none: everything, and Done makes it everything.**
        for arrangement in [Some(off), None] {
            let (_, said) = says(&unread(arrangement.clone()));
            assert!(said.iter().any(|one| one == words::EVERYTHING_DOWNLOADS), "{said:?}");
            assert_eq!(the_narrowing_chosen(&unread(arrangement), &[]), Some(TheNarrowing::Everything));
        }

        // **One format: nothing to narrow.**
        let one = vec![AFormatKnown { name: "PNG".into(), where_in_the_job: "p_".into(), at_the_top: false, files: 240, chosen: true }];
        let read = a_jobs_box(&row, one, Some(on));
        assert_eq!(says(&read).0, "one-format");
        assert_eq!(the_narrowing_chosen(&read, &[]), Some(TheNarrowing::Everything), "Done narrowed it to no format");
    }

    /// **A failed download's *Try again* reads the one entry it is about** —
    /// a new request for that file, into the folder its landing was in (item
    /// 17) — and a file that did not fail, or another request, reads nothing.
    #[test]
    fn try_again_reads_the_failed_entry_and_lands_it_where_it_was_going() {
        use smoozy_download::request::KeepsTheRequest as _;
        let (_folder, record, server) = a_record_with_one_server("try-again");
        let mut request = a_download_that_landed(
            b"/Users/artist/x",
            &[(b"/smoozy/jobs/abc/outputs/f_0001.exr", b"/Users/artist/x/abc night/f_0001.exr")],
        );
        let mut failed = smoozy_download::request::EnumerationEntry::enumerated(
            b"/smoozy/jobs/abc/outputs/f_0002.exr".to_vec(),
            b"/Users/artist/x/abc night/f_0002.exr".to_vec(),
            10,
            smoozy_download::vouching::AVouchingMark::NoClaimIsMade,
        );
        failed.disposition = Some(smoozy_download::request::Disposition::ToFetch);
        failed.its_outcome_became(smoozy_state::outcomes::UnitOutcome::Failed).expect("first write");
        request.entries.push(failed);
        request.server_row = server.clone();
        DownloadRequests::over(record.connection()).write_it_down(&request).expect("written");
        let mut found = a_request(TheKind::Download, WhatItIsOf::AFolder { folder: "x".into() });
        found.id = request.record.as_text();
        found.server = server.as_text();

        let again = the_entry_sent_again(record.connection(), &found, &found.id, b"/Users/artist/x/abc night/f_0002.exr")
            .expect("the failed entry");
        assert_eq!(again.direction, Direction::Down);
        assert_eq!(again.source_path, b"/smoozy/jobs/abc/outputs/f_0002.exr".to_vec());
        assert_eq!(the_folder_holding_it(&again.landing_path), b"/Users/artist/x/abc night".to_vec());
        assert!(
            the_entry_sent_again(record.connection(), &found, &found.id, b"/Users/artist/x/abc night/f_0001.exr").is_none(),
            "a file that landed is not sent again"
        );
        assert!(the_entry_sent_again(record.connection(), &found, "another", b"/Users/artist/x/abc night/f_0002.exr").is_none());
        assert_eq!(the_folder_holding_it(b"/f.exr"), b"/".to_vec());
    }

    /// **An upload of the user's own Blender build is named by the file the
    /// user picked** — not *1 file into* the staging folder, and not the
    /// landing name the machine reads it under — and still says it did not
    /// finish where it did not (`Design/Server Setup § Pointing at a build on
    /// your own computer`). Any other upload reads as it did. Step 10c's owed
    /// piece 5, 2026-10-16.
    #[test]
    fn a_users_build_going_up_is_named_by_its_own_file() {
        let counted = |failed: bool| crate::record::upload_requests::AnUploadRequestCounted {
            changed_landings: Vec::new(),
            record: Identity::issue(crate::record::upload_requests::THE_RECORD_IDENTITY_KIND).expect("random bytes"),
            destination: b"/home/ubuntu/smoozy/staging".to_vec(),
            files_in_the_whole: 1,
            files_done: usize::from(!failed),
            bytes_in_the_whole: 10,
            bytes_done: if failed { 0 } else { 10 },
            every_unit_has_ended: true,
            failed_landings: if failed {
                vec![(b"/home/ubuntu/smoozy/staging/run-blender-4.2-linux.tar.xz".to_vec(), 10)]
            } else {
                Vec::new()
            },
            nothing_is_decided: false,
            stopped_at_the_question: false,
            opened_by_run: Identity::issue(IdentityKind::Run).expect("random bytes"),
        };
        let the_name = |request: &crate::record::upload_requests::AnUploadRequestCounted| {
            let screen = the_screen_from(&known(vec![an_upload_request_known("row", request)]), None);
            let name = &screen.requests[0].name;
            (every_word(name), name.values().concat())
        };
        let build = counted(false);
        crate::setting_up::a_build_goes_up(&build.record.as_text(), "blender-4.2-linux.tar.xz");
        let (said, values) = the_name(&build);
        assert_eq!(values, "blender-4.2-linux.tar.xz", "the row is not named by the user's file: {said}");
        assert!(!said.contains(words::FILE_INTO), "the row reads as a folder upload: {said}");

        let short = counted(true);
        crate::setting_up::a_build_goes_up(&short.record.as_text(), "blender-4.2-linux.tar.xz");
        let (said, values) = the_name(&short);
        assert_eq!(values, "blender-4.2-linux.tar.xz");
        assert!(said.contains(words::DID_NOT_FINISH.trim_start_matches("— ")), "a build that did not land reads done: {said}");

        let ordinary = counted(false);
        let (said, values) = the_name(&ordinary);
        assert!(said.contains(words::FILE_INTO) && values.contains("/home/ubuntu/smoozy/staging"), "{said} {values}");
    }

    /// **A file that changed while it went up is on its own row, with *Upload
    /// it again*** (`Design/Transfers § A file that changed while it was going
    /// up`; `File Browser` K8) — item 17. Until then it was counted as failed
    /// and drawn nowhere.
    #[test]
    fn a_changed_upload_is_drawn_with_upload_it_again() {
        let landing = b"/home/ubuntu/kitchen/wood.png".to_vec();
        let request = crate::record::upload_requests::AnUploadRequestCounted {
            changed_landings: vec![landing.clone()],
            record: Identity::issue(IdentityKind::Operation).expect("random bytes"),
            destination: b"/home/ubuntu/kitchen".to_vec(),
            files_in_the_whole: 1,
            files_done: 0,
            bytes_in_the_whole: 10,
            bytes_done: 0,
            every_unit_has_ended: true,
            failed_landings: vec![(landing, 10)],
            nothing_is_decided: false,
            stopped_at_the_question: false,
            opened_by_run: Identity::issue(IdentityKind::Run).expect("random bytes"),
        };
        let screen = the_screen_from(&known(vec![an_upload_request_known("row", &request)]), None);
        let drawn = &screen.requests[0];
        assert_eq!(drawn.files.len(), 1);
        assert_eq!(drawn.files[0].state, "changed");
        let note = drawn.files[0].note.as_ref().expect("the amber line");
        assert_eq!(note.link.as_ref().expect("the offer").act.as_str(), "upload-it-again");
        assert!(drawn.files[0].failure.is_none(), "a changed file is not a failure");
    }

    /// **The four acts that send a file again or name an amber line's files
    /// take their own commands on the page**, and the application's
    /// prose route says nothing for them — so the silence in
    /// [`what_an_act_answers`] is unreachable rather than remembered. Items
    /// 17 and 32.
    #[test]
    fn the_acts_that_send_or_name_take_their_own_commands() {
        let page = the_page();
        for named in [
            "a_file_is_sent_again",
            "the_files_a_line_names",
            "the_formats_are_chosen_for_a_job",
            "\"try-again\", \"upload-it-again\"",
            "const WHICH_ONES = \"which-ones\"",
        ] {
            assert!(page.contains(named), "the page does not name {named}");
        }
        // **And each press is routed to them**: an amber line's link by its
        // act, and a file's failure controls and its note through the one
        // function that sends a file again.
        let routed = page.split_whitespace().collect::<Vec<_>>().join(" ");
        for route in [
            "line.link.act.startsWith(WHICH_ONES) ? theNamesAreShownUnder(p, request.id, line.link.act)",
            "onAct: (act) => theActOnAFileIsTaken(act, request.id, file.path)",
            "theActOnAFileIsTaken(file.note.link.act, request.id, file.path)",
            "if (!THE_ACTS_THAT_SEND_A_FILE_AGAIN.includes(act)) return theActIsTaken(act, request, file);",
        ] {
            assert!(routed.contains(route), "the page does not route: {route}");
        }
        for act in ["try-again", "upload-it-again", "which-ones-were-left-alone", "which-ones-cannot-land", "which-ones-may-be-incomplete"] {
            assert!(what_an_act_answers(&known(Vec::new()), the_act_named(act), "one").is_none(), "{act}");
        }
        let list = include_str!("mod.rs");
        for command in [
            "transfers::a_file_is_sent_again",
            "transfers::the_files_a_line_names",
            "transfers::the_formats_are_chosen_for_a_job",
        ] {
            assert!(list.contains(command), "{command} is not registered");
        }
    }

    /// ***Which ones* names the files the line counts, from the request's own
    /// records** — an auto-download's *left alone* entries, and a manual
    /// download's refused names — and names nothing for any other line. Item
    /// 32.
    #[test]
    fn which_ones_names_what_the_line_counts() {
        use smoozy_download::arrangement::{AReckoningEntry, AnArrangement, KeepsTheArrangement as _, WhatSmoozyDid};
        use smoozy_download::request::KeepsTheRequest as _;
        let (_folder, record, server) = a_record_with_one_server("which-ones");
        let job = Identity::issue(IdentityKind::Job).expect("random bytes");
        let mut arrangement = AnArrangement::switched_on(job.clone(), server.clone(), b"/Users/artist/x".to_vec(), TheNarrowing::Everything);
        for (name, state) in [
            (b"f_0001.exr".as_slice(), WhatSmoozyDid::Fetched),
            (b"f_0002.exr".as_slice(), WhatSmoozyDid::LeftAlone),
            (b"f_0003.exr".as_slice(), WhatSmoozyDid::LeftAlone),
        ] {
            arrangement
                .reckoning
                .push(AReckoningEntry::acted_on(name.to_vec(), None, state, b"/Users/artist/x".to_vec()));
        }
        DownloadArrangements::over(record.connection()).write_it_down(&arrangement).expect("written");
        let mut found = a_request(TheKind::AutoDownload, WhatItIsOf::AJob { name: "abc".into(), job: job.as_text() });
        found.id = job.as_text();
        found.server = server.as_text();
        assert_eq!(
            the_files_the_line_names(record.connection(), &found, "which-ones-were-left-alone"),
            vec![b"f_0002.exr".to_vec(), b"f_0003.exr".to_vec()]
        );
        assert!(the_files_the_line_names(record.connection(), &found, "which-ones-may-be-incomplete").is_empty());

        let mut request = a_download_that_landed(b"/Users/artist/y", &[(b"/m/a.exr", b"/Users/artist/y/a.exr")]);
        let mut refused = smoozy_download::request::EnumerationEntry::enumerated(
            b"/m/b:c.exr".to_vec(),
            b"/Users/artist/y/b:c.exr".to_vec(),
            10,
            smoozy_download::vouching::AVouchingMark::NoClaimIsMade,
        );
        refused.refusal = Some(smoozy_download::fetching::WhichRefusalItWas::NotRepresentable);
        refused.disposition = Some(smoozy_download::request::Disposition::CannotLand(
            smoozy_download::fetching::WhichRefusalItWas::NotRepresentable,
        ));
        request.entries.push(refused);
        request.server_row = server.clone();
        DownloadRequests::over(record.connection()).write_it_down(&request).expect("written");
        let mut found = a_request(TheKind::Download, WhatItIsOf::AFolder { folder: "y".into() });
        found.id = request.record.as_text();
        found.server = server.as_text();
        assert_eq!(
            the_files_the_line_names(record.connection(), &found, "which-ones-cannot-land"),
            vec![b"/Users/artist/y/b:c.exr".to_vec()]
        );
        // **Each line names only what it counts**: a manual download's refused
        // names are not *left alone*. The two transfer faults, fault 2.
        assert!(the_files_the_line_names(record.connection(), &found, "which-ones-were-left-alone").is_empty());
    }

    /// **`File Browser` K11: a file that holds still is sent once it has held
    /// for two seconds**, and no sooner — item 17's *Upload it again*.
    #[tokio::test]
    async fn the_quiet_period_waits_two_seconds_on_a_file_that_holds_still() {
        let at = std::env::temp_dir().join(format!("smoozy-quiet-{}", std::process::id()));
        std::fs::write(&at, b"still").expect("a file");
        let began = std::time::Instant::now();
        the_quiet_period_is_waited_out(at.as_os_str().as_encoded_bytes()).await;
        let waited = began.elapsed();
        let _ = std::fs::remove_file(&at);
        assert!(waited >= smoozy_file_browser::continuation::THE_QUIET_PERIOD, "{waited:?}");
        assert!(waited < std::time::Duration::from_secs(5), "{waited:?}");
    }

    fn the_mockup() -> String {
        std::fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../Preparation/Design/Mockup/Smoozy.html"),
        )
        .expect("the drawing is readable")
    }

    fn the_page() -> &'static str {
        include_str!("../../web/transfers.js")
    }

    /// **One act of the closed set, by the string the page sends.**
    fn the_act_named(act: &str) -> TheAct {
        let reading = serde::de::value::StrDeserializer::<serde::de::value::Error>::new(act);
        serde::Deserialize::deserialize(reading)
            .unwrap_or_else(|_| panic!("{act} is not a closed act"))
    }

    fn the_working_half() -> &'static str {
        include_str!("transfers.rs")
            .split("#[cfg(test)]\nmod tests {")
            .next()
            .expect("the working half")
    }

    /// A finished manual download into `destination`, every entry published.
    fn a_download_that_landed(
        destination: &[u8],
        entries: &[(&[u8], &[u8])],
    ) -> smoozy_download::request::AManualRequest {
        let entries = entries
            .iter()
            .map(|(source, landing)| {
                let mut entry = smoozy_download::request::EnumerationEntry::enumerated(
                    source.to_vec(),
                    landing.to_vec(),
                    10,
                    smoozy_download::vouching::AVouchingMark::NoClaimIsMade,
                );
                entry.disposition = Some(smoozy_download::request::Disposition::ToFetch);
                entry
                    .its_outcome_became(smoozy_state::outcomes::UnitOutcome::Done)
                    .expect("first write");
                entry
            })
            .collect();
        smoozy_download::request::AManualRequest::confirmed(
            Identity::issue(IdentityKind::Operation).expect("random bytes"),
            Identity::issue(IdentityKind::LocalServer).expect("random bytes"),
            None,
            destination.to_vec(),
            entries,
            Identity::issue(IdentityKind::Run).expect("random bytes"),
        )
    }

    /// **Note 15 on the test copy: *Files on your computer* shows what the
    /// download brought, selected** — `Design/Transfers`' paragraph of
    /// 2026-10-03, over `Download` H5 and H7. A whole job into `~/x` hands
    /// `~/x/<job folder>`; a picked single file hands its own path; several
    /// picked things open the destination with none selected.
    #[test]
    fn files_on_your_computer_selects_what_the_download_brought() {
        let whole_job = a_download_that_landed(
            b"/Users/artist/x",
            &[
                (b"/smoozy/jobs/abc/outputs/beauty/f_0001.exr", b"/Users/artist/x/abc night/beauty/f_0001.exr"),
                (b"/smoozy/jobs/abc/outputs/beauty/f_0002.exr", b"/Users/artist/x/abc night/beauty/f_0002.exr"),
            ],
        );
        assert_eq!(
            what_the_files_show(&whole_job),
            WhatTheFilesShow::ThisSelected(b"/Users/artist/x/abc night".to_vec())
        );

        let one_file = a_download_that_landed(
            b"/Users/artist/x",
            &[(b"/smoozy/jobs/abc/outputs/beauty/f_0001.exr", b"/Users/artist/x/f_0001.exr")],
        );
        assert_eq!(
            what_the_files_show(&one_file),
            WhatTheFilesShow::ThisSelected(b"/Users/artist/x/f_0001.exr".to_vec())
        );

        let two_picked = a_download_that_landed(
            b"/Users/artist/x",
            &[
                (b"/smoozy/a.png", b"/Users/artist/x/a.png"),
                (b"/smoozy/b.png", b"/Users/artist/x/b.png"),
            ],
        );
        assert_eq!(
            what_the_files_show(&two_picked),
            WhatTheFilesShow::TheDestination(b"/Users/artist/x".to_vec())
        );
    }

    /// **The act reads the request whole from the record**, so what it
    /// selects comes from the entries as written down, and the press takes
    /// that route ([`an_act_on_transfers`]).
    #[test]
    fn files_on_your_computer_reads_the_request_from_the_record() {
        use smoozy_download::request::KeepsTheRequest as _;
        let (_folder, record, server) = a_record_with_one_server("files-on-your-computer");
        let mut request = a_download_that_landed(
            b"/Users/artist/x",
            &[(b"/smoozy/jobs/abc/outputs/f_0001.exr", b"/Users/artist/x/abc night/f_0001.exr")],
        );
        request.server_row = server.clone();
        DownloadRequests::over(record.connection())
            .write_it_down(&request)
            .expect("the request is written");

        let held = the_request_held(record.connection(), &server.as_text(), &request.record.as_text())
            .expect("the request reads back");
        assert_eq!(
            what_the_files_show(&held),
            WhatTheFilesShow::ThisSelected(b"/Users/artist/x/abc night".to_vec())
        );
        assert!(the_request_held(record.connection(), &server.as_text(), "another").is_none());

        let (working, _) = the_working_half()
            .split_once("pub async fn an_act_on_transfers")
            .map(|(_, after)| (after, ()))
            .expect("the command");
        let the_arm = working
            .split_once("TheAct::FilesOnYourComputer =>")
            .map(|(_, after)| after.split("_ => what_an_act_answers").next().unwrap_or(""))
            .expect("the press has its own arm");
        assert!(the_arm.contains("the_request_held("), "the press does not read the request");
    }

    /// **`Download` H7's fallback**: where the first act refuses, the folder
    /// that holds the place is opened, and only a refusal of that is H8's
    /// *cannot open*. Where it is shown selected, nothing else is opened.
    #[test]
    fn a_place_the_reveal_refuses_opens_the_folder_that_holds_it() {
        use crate::reveal::Revealed;
        // **The place is really there**: since note 4 the fallback is taken
        // only for a place that is.
        let root = a_real_folder("fallback");
        let there = root.join("abc night");
        std::fs::create_dir_all(&there).expect("the place");
        let place = there.as_path();

        let mut opened = None;
        let shown = a_place_is_shown(place, |_| Revealed::ItCouldNotBeOpened, |folder| {
            opened = Some(folder.to_path_buf());
            Revealed::ItOpened
        });
        assert_eq!(shown, Revealed::ItOpened);
        assert_eq!(opened.as_deref(), Some(root.as_path()));

        let mut asked = None;
        let shown = a_place_is_shown(place, |at| {
            asked = Some(at.to_path_buf());
            Revealed::ItOpened
        }, |_| panic!("the folder that holds it was opened as well"));
        assert_eq!(shown, Revealed::ItOpened);
        assert_eq!(asked.as_deref(), Some(place));

        assert_eq!(
            a_place_is_shown(place, |_| Revealed::ItCouldNotBeOpened, |_| Revealed::ItCouldNotBeOpened),
            Revealed::ItCouldNotBeOpened
        );
        let _ = std::fs::remove_dir_all(root);
    }

    fn a_server(row: &str, name: &str) -> AServerKnown {
        AServerKnown { row: row.into(), name: name.into(), badge: Some("b1") }
    }

    fn a_file(path: &str, happened: WhatHappenedToTheFile) -> AFileKnown {
        AFileKnown {
            path: path.into(),
            record_ending: None,
            report: Report { bytes_staged: 10, bytes_expected: 100, started_over: None, verifying_a_prefix_of: None },
            happened,
        }
    }

    #[test]
    fn seventeenth_job_ended_while_its_download_runs_says_downloading() {
        let mut auto = a_request(TheKind::AutoDownload, WhatItIsOf::AJob { name: "liquid first frame".into(), job: "video-check".into() });
        auto.the_job_ended = true;
        auto.files_done = 1;
        auto.files_in_the_whole = 2;
        let drawn = &the_screen_from(&known(vec![auto]), None).requests[0];
        assert!(every_word(&drawn.name).contains("downloading"), "{}", every_word(&drawn.name));
        assert!(!every_word(&drawn.name).contains(words::ENDED));
        assert!(drawn.bar.is_some());
    }

    #[test]
    fn seventeenth_terminal_short_video_keeps_its_count_without_a_live_bar() {
        let mut auto = a_request(TheKind::AutoDownload, WhatItIsOf::AJob { name: "liquid first frame".into(), job: "video-check".into() });
        auto.the_job_ended = true;
        auto.the_arrangement_is_done = true;
        auto.files_done = 1;
        auto.files_in_the_whole = 2;
        let drawn = &the_screen_from(&known(vec![auto]), None).requests[0];
        assert!(every_word(&drawn.name).contains(words::ENDED));
        assert_eq!(drawn.count.values(), vec!["1 / 2"]);
        assert!(!drawn.collapsed, "the shortfall remains open");
        assert_eq!(drawn.bar, None, "a terminal shortfall is not still downloading");
    }

    #[test]
    fn seventeenth_starts_order_all_kinds_and_servers_survive_reopening() {
        use crate::record::{LocalRecord, transfer_records::parent_start};
        use smoozy_file_browser::request::{UploadRequest, EnumerationEntry as UpEntry, Disposition as UpDisposition, KeepsTheRequest as _};
        use smoozy_download::request::{AManualRequest, EnumerationEntry as DownEntry, Disposition as DownDisposition, KeepsTheRequest as _};
        use smoozy_download::arrangement::AnArrangement;
        use crate::the_found_files_sent::{ASend, AFileSent, HowItEnded};
        let folder = std::env::temp_dir().join(format!("smoozy-piece3-order-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).unwrap();
        let path = folder.join("record.sqlite3");
        let issue = |kind| Identity::issue(kind).unwrap();
        let servers: Vec<_> = (0..2).map(|_| AServerKnown {
            row: issue(IdentityKind::LocalServer).as_text(), name: "server".into(), badge: None,
        }).collect();
        let server_ids: Vec<_> = servers.iter().map(|s| Identity::read_back(IdentityKind::LocalServer, &s.row).unwrap()).collect();
        let run = issue(IdentityKind::Run);
        let mut ids = Vec::new();
        let mut original_starts = Vec::new();
        {
            let record = LocalRecord::open_at(&path).unwrap();
            let c = record.connection();
            for terminal in [true, false] {
                let id = issue(IdentityKind::Operation);
                let mut entry = UpEntry::enumerated(b"/source".to_vec(), b"/target/source".to_vec(), 10);
                entry.disposition = Some(UpDisposition::ToSend);
                if terminal { entry.its_outcome_became(smoozy_state::outcomes::UnitOutcome::Done, std::time::SystemTime::now()).unwrap(); }
                let up = UploadRequest::confirmed(smoozy_file_browser::request::open_an_upload_request().unwrap(), id.clone(), server_ids[1].clone(), None, b"/target".to_vec(), vec![entry], run.clone());
                let mut uploads = crate::record::upload_requests::UploadRequests::over(c);
                uploads.write_it_down(&up).unwrap();
                let start = parent_start(c, "upload_request", "record", &id.as_text()).unwrap().unwrap();
                uploads.write_it_down(&up).unwrap();
                assert_eq!(parent_start(c, "upload_request", "record", &id.as_text()).unwrap(), Some(start));
                ids.push(id.as_text()); original_starts.push(start);

                let id = issue(IdentityKind::Operation);
                let send = ASend { request: id.clone(), server: servers[0].row.clone(), file: "/scene.blend".into(), files: vec![AFileSent {
                    landing: b"/mirror/texture".to_vec(), size: 20, transfer: None, ended: terminal.then_some(HowItEnded::Published),
                }] };
                crate::record::dependency_records::save_dependency_send(c, &send).unwrap();
                let start = parent_start(c, "dependency_upload_request", "request", &id.as_text()).unwrap().unwrap();
                crate::record::dependency_records::save_dependency_send(c, &send).unwrap();
                assert_eq!(parent_start(c, "dependency_upload_request", "request", &id.as_text()).unwrap(), Some(start));
                ids.push(id.as_text()); original_starts.push(start);

                let id = issue(IdentityKind::Operation);
                let mut entry = DownEntry::enumerated(b"/server/frame".to_vec(), b"/downloads/frame".to_vec(), 30, smoozy_download::vouching::AVouchingMark::NoClaimIsMade);
                entry.disposition = Some(DownDisposition::ToFetch);
                if terminal { entry.its_outcome_became(smoozy_state::outcomes::UnitOutcome::Done).unwrap(); }
                let down = AManualRequest::confirmed(id.clone(), server_ids[1].clone(), None, b"/downloads".to_vec(), vec![entry], run.clone());
                let mut downloads = DownloadRequests::over(c);
                downloads.write_it_down(&down).unwrap();
                let start = parent_start(c, "download_request", "record", &id.as_text()).unwrap().unwrap();
                downloads.write_it_down(&down).unwrap();
                assert_eq!(parent_start(c, "download_request", "record", &id.as_text()).unwrap(), Some(start));
                ids.push(id.as_text()); original_starts.push(start);

                let id = issue(IdentityKind::Job);
                let mut auto = AnArrangement::switched_on(id.clone(), server_ids[0].clone(), b"/downloads".to_vec(), TheNarrowing::Everything);
                if terminal { auto.desired_state = DesiredState::Off; }
                let mut arrangements = DownloadArrangements::over(c);
                arrangements.write_it_down(&auto).unwrap();
                let start = parent_start(c, "download_arrangement", "job", &id.as_text()).unwrap().unwrap();
                arrangements.write_it_down(&auto).unwrap();
                assert_eq!(parent_start(c, "download_arrangement", "job", &id.as_text()).unwrap(), Some(start));
                ids.push(id.as_text()); original_starts.push(start);
            }
            assert!(original_starts.windows(2).all(|s| s[0].start_order < s[1].start_order));
            // Equal wall times still use the ONE saved order, never each table's rowid.
            for table in ["upload_request", "download_request", "dependency_upload_request", "download_arrangement"] {
                c.execute(&format!("UPDATE {table} SET started_at_utc = 1792500000123"), []).unwrap();
            }
            // Existing parents without stamps remain readable after reopening,
            // following all dated rows inside their respective groups.
            for id in [&ids[0], &ids[4]] {
                c.execute("UPDATE upload_request SET started_at_utc = NULL, start_order = NULL WHERE record = ?1", [id]).unwrap();
            }
        }
        let record = LocalRecord::open_at(&path).unwrap();
        let c = record.connection();
        let rows = the_requests_the_two_stores_hold(c, &servers);
        assert_eq!(rows.len(), 8);
        assert_eq!(rows.iter().filter(|r| r.start.is_none()).count(), 2);
        let known = WhatIsKnown { servers: servers.clone(), requests: rows, ..WhatIsKnown::default() };
        let screen = the_screen_from(&known, None);
        let expected: Vec<_> = ids[4..8].iter().rev().chain(ids[0..4].iter().rev()).map(String::as_str).collect();
        assert_eq!(screen.requests.iter().map(|r| r.id.as_str()).collect::<Vec<_>>(), expected);
        assert_eq!(screen.requests.iter().filter(|r| r.group_heading.is_some()).count(), 2);
        assert!(screen.requests.iter().filter(|r| r.started_at_millis.is_some()).all(|r| r.started_label.as_str() == words::STARTED && r.started_at_millis.as_ref().unwrap().as_str() == "1792500000123"));
        assert!(screen.requests.iter().filter(|r| r.started_at_millis.is_none()).all(|r| r.started_label == Prose::new(words::START_TIME_NOT_RECORDED)));
        let filtered = the_screen_from(&known, Some(&servers[0].row));
        assert_eq!(filtered.requests.iter().map(|r| r.id.as_str()).collect::<Vec<_>>(), vec![ids[7].as_str(), ids[5].as_str(), ids[3].as_str(), ids[1].as_str()]);
        // The saved clock wins over sequence; a clock correction is never hidden.
        let mut changed_clock = WhatIsKnown { servers: known.servers.clone(), requests: known.requests.clone(), ..WhatIsKnown::default() };
        changed_clock.requests.iter_mut().find(|r| r.id == ids[5]).unwrap().start.as_mut().unwrap().started_at_utc += 1;
        assert_eq!(the_screen_from(&changed_clock, None).requests[0].id.as_str(), ids[5]);
        // Reopening never allocates; another parent continues beyond every existing order.
        let high: i64 = c.query_row("SELECT last_order FROM transfer_start_order", [], |r| r.get(0)).unwrap();
        assert_eq!(high, original_starts.last().unwrap().start_order);
        drop(record);
        let _ = std::fs::remove_dir_all(folder);
    }

    #[test]
    fn seventeenth_unknown_starts_follow_known_starts_in_each_group() {
        use crate::record::transfer_records::TransferStart;
        let make = |id: &str, terminal, start| {
            let mut r = a_request(TheKind::Upload, WhatItIsOf::AFolder { folder: id.into() });
            r.id = id.into(); r.start = start;
            if terminal { r.stands = WhereTheRequestStands::Finished { at: String::new() }; }
            r
        };
        let stamp = Some(TransferStart { started_at_utc: 10, start_order: 1 });
        let screen = the_screen_from(&known(vec![make("old-z", false, None), make("dated-finished", true, stamp), make("old-b", false, None), make("dated-running", false, stamp), make("old-a", true, None)]), None);
        assert_eq!(screen.requests.iter().map(|r| r.id.as_str()).collect::<Vec<_>>(), ["dated-running", "old-b", "old-z", "dated-finished", "old-a"]);
        for r in screen.requests.iter().filter(|r| r.id.as_str().starts_with("old-")) {
            assert_eq!(r.started_label, Prose::new(words::START_TIME_NOT_RECORDED));
            assert!(r.started_at_millis.is_none());
        }
    }

    #[test]
    fn seventeenth_video_per_file_outcomes_explain_two_and_survive_reopening() {
        use crate::record::LocalRecord;
        use smoozy_download::arrangement::{AnArrangement, AReckoningEntry, WhatSmoozyDid};
        use smoozy_job_files::inventory::{TheInventory, AnEntry};
        use smoozy_download::acting::record_directory::what_the_narrowing_leaves;
        let folder = std::env::temp_dir().join(format!("smoozy-piece3-video-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).unwrap();
        let path = folder.join("record.sqlite3");
        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        let job;
        {
            let record = LocalRecord::open_at(&path).unwrap();
            let c = record.connection();
            let mut held = crate::record::deletions::tests::a_record_in_history(&server, "liquid first frame");
            held.what_the_note_carried.ending = Some(smoozy_queue_history::acting::note::TheEndingOnTheNote {
                outcome: smoozy_queue_history::acting::note::TheOutcomeAsWritten::Finished,
                exit_status: None, the_attempt_whose_capture_record_holds_the_words: 1,
                the_reason_the_machine_recorded: None, the_device_line_as_observed: None,
            });
            job = held.job.clone();
            crate::record::job_records::JobRecords::over(c).write_the_provisional_record(&held).unwrap();
            let mut auto = AnArrangement::switched_on(job.clone(), server.clone(), b"/downloads".to_vec(), TheNarrowing::Everything);
            auto.reckoning.push(AReckoningEntry::acted_on(b"liquid0001-0001.mp4".to_vec(), None, WhatSmoozyDid::Fetched, b"/downloads".to_vec()));
            auto.reckoning.push(AReckoningEntry::acted_on(b"expected-extra.mp4".to_vec(), None, WhatSmoozyDid::NeverComing, b"/downloads".to_vec()));
            DownloadArrangements::over(c).write_it_down(&auto).unwrap();
        }
        let record = LocalRecord::open_at(&path).unwrap();
        let c = record.connection();
        let auto = DownloadArrangements::over(c).the_arrangement_for(&job).unwrap().unwrap();
        assert_eq!(auto.reckoning.len(), 2);
        assert_eq!(auto.reckoning[0].account.state, WhatSmoozyDid::Fetched);
        assert_eq!(auto.reckoning[1].account.state, WhatSmoozyDid::NeverComing);
        // The test's two-file inventory has two actual expected video paths,
        // never two frames inside one video. The missing file stays a shortfall.
        let inventory = TheInventory { entries: vec![
            AnEntry { predicted_path: b"/job/liquid0001-0001.mp4".to_vec(), frame: None, the_route_that_produced_the_name: smoozy_job_files::inventory::WhichRouteProducedTheName::ItWasAskedOfBlender, the_destinations_aiming_at_it: vec![], may_grow: true },
            AnEntry { predicted_path: b"/job/expected-extra.mp4".to_vec(), frame: None, the_route_that_produced_the_name: smoozy_job_files::inventory::WhichRouteProducedTheName::ItWasAskedOfBlender, the_destinations_aiming_at_it: vec![], may_grow: true },
        ], added_write_place_folders: Vec::new(), collision_record: Vec::new(), a_destination_is_video_shaped: true, why_it_owes_nothing: None };
        let cut = what_the_narrowing_leaves(&inventory, &[], &TheNarrowing::Everything, b"/job");
        assert_eq!(cut.narrowed.len(), 2);
        let server_known = AServerKnown { row: server.as_text(), name: "server".into(), badge: None };
        let mut rows = the_requests_the_two_stores_hold(c, &[server_known]);
        the_wholes_this_run_has_read(&mut rows, &|_| Some(crate::servers::the_machines_work::TheJobsWhole { files: cut.narrowed.len(), it_may_grow: cut.it_may_grow }));
        assert!(rows[0].the_job_ended && rows[0].the_arrangement_is_done);
        let drawn = a_request_drawn(&known(Vec::new()), &rows[0]);
        assert_eq!(drawn.count.values(), vec!["1 / 2"]);
        assert!(drawn.terminal && !drawn.collapsed && drawn.bar.is_none());
        // A second file left alone is expressly OUT of `Download` N7's count. It is
        // reported separately, rather than looking like an outstanding file.
        let mut auto = auto;
        auto.reckoning[1].account.state = WhatSmoozyDid::LeftAlone;
        DownloadArrangements::over(c).write_it_down(&auto).unwrap();
        let mut rows = the_requests_the_two_stores_hold(c, &[AServerKnown { row: server.as_text(), name: "server".into(), badge: None }]);
        the_wholes_this_run_has_read(&mut rows, &|_| Some(crate::servers::the_machines_work::TheJobsWhole { files: 2, it_may_grow: true }));
        let drawn = a_request_drawn(&known(Vec::new()), &rows[0]);
        assert_eq!(drawn.count.values(), vec!["1 / 1"]);
        assert!(drawn.terminal && drawn.bar.is_none() && !drawn.collapsed);
        assert_eq!(rows[0].left_alone, 1);
        auto.reckoning[1].account.state = WhatSmoozyDid::CannotLand;
        DownloadArrangements::over(c).write_it_down(&auto).unwrap();
        let mut rows = the_requests_the_two_stores_hold(c, &[AServerKnown { row: server.as_text(), name: "server".into(), badge: None }]);
        the_wholes_this_run_has_read(&mut rows, &|_| Some(crate::servers::the_machines_work::TheJobsWhole { files: 2, it_may_grow: true }));
        assert_eq!(rows[0].files_in_the_whole, 1);
        assert_eq!(rows[0].cannot_land, 1);
        assert_eq!(the_files_the_line_names(c, &rows[0], "which-ones-cannot-land"), vec![b"expected-extra.mp4".to_vec()]);
        // With only the one video in the inventory and reckoning, it finishes
        // as one file and collapses. It never counts the video's frames.
        auto.reckoning.truncate(1);
        DownloadArrangements::over(c).write_it_down(&auto).unwrap();
        let rows = the_requests_the_two_stores_hold(c, &[AServerKnown { row: server.as_text(), name: "server".into(), badge: None }]);
        let drawn = a_request_drawn(&known(Vec::new()), &rows[0]);
        assert_eq!(drawn.count.values(), vec!["1 / 1"]);
        assert!(drawn.terminal && drawn.collapsed && drawn.bar.is_none());
        drop(record);
        let _ = std::fs::remove_dir_all(folder);
    }

    #[test]
    fn seventeenth_page_shows_local_start_groups_and_dims_a_terminal_shortfall() {
        use crate::record::transfer_records::TransferStart;
        let mut live = a_request(TheKind::AutoDownload, WhatItIsOf::AJob { name: "liquid first frame".into(), job: "video".into() });
        live.id = "live".into(); live.the_job_ended = true;
        live.files_done = 1; live.files_in_the_whole = 2;
        live.start = Some(TransferStart { started_at_utc: 1792592528000, start_order: 2 });
        let mut short = live.clone(); short.id = "short".into(); short.the_arrangement_is_done = true;
        short.start.as_mut().unwrap().start_order = 1;
        let mut old = a_request(TheKind::Upload, WhatItIsOf::AFolder { folder: "old".into() });
        old.id = "old".into(); old.stands = WhereTheRequestStands::Ended;
        let screen = the_screen_from(&known(vec![short, old, live]), None);
        let folder = std::env::temp_dir().join(format!("smoozy-piece3-page-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder); std::fs::create_dir_all(&folder).unwrap();
        let web = Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        for page in ["transfers.js", "the-value-drawn.js"] { std::fs::copy(web.join(page), folder.join(page)).unwrap(); }
        let mut document = "";
        for (name, source) in THE_QUESTIONS_STAND_INS {
            if name == "harness.js" { document = source.split("const answers = JSON.parse").next().unwrap(); }
            else { std::fs::write(folder.join(name), source).unwrap(); }
        }
        let harness = r#"
const screenSaid = JSON.parse(readFileSync("answers.json", "utf8"));
globalThis.window = { __TAURI__: { core: { invoke(command) { return Promise.resolve(command === "the_transfers_screen" ? screenSaid : null); } } } };
await import("./transfers.js");
const host = anElement("div"); body.appendChild(host);
const transfers = globalThis.screens.get("transfersview");
transfers.itIsDrawn(host); transfers.itIsCurrentNow();
await new Promise((done) => setImmediate(done));
console.log(JSON.stringify({groups: host.querySelectorAll("h2.reqgroup").map(r => r.textContent),
rows: host.querySelectorAll("div.req").map(r => ({id:r.dataset.request, done:r.classList.contains("done"),
start:r.querySelector("time.rqstarted").textContent, instant:r.querySelector("time.rqstarted").dateTime || null,
bars:r.querySelectorAll("progress.transfers-bar").length, count:r.querySelector("span.rqm").textContent}))}));
"#;
        std::fs::write(folder.join("harness.js"), format!("{document}{harness}")).unwrap();
        std::fs::write(folder.join("answers.json"), serde_json::to_string(&screen).unwrap()).unwrap();
        for (zone, date) in [("UTC", "21 Oct 2026, 14:22:08"), ("Asia/Nicosia", "21 Oct 2026, 17:22:08")] {
            let result = std::process::Command::new("node").env("TZ", zone).arg("harness.js").current_dir(&folder).output().expect("Node is needed for the page check");
            assert!(result.status.success(), "{}", String::from_utf8_lossy(&result.stderr));
            let got: serde_json::Value = serde_json::from_slice(&result.stdout).unwrap();
            assert_eq!(got["groups"], serde_json::json!([Prose::new(words::RUNNING_NEWEST_FIRST), Prose::new(words::FINISHED_NEWEST_FIRST)]));
            assert_eq!(got["rows"], serde_json::json!([
                {"id":"live", "done":false, "start":format!("{} {date}", Prose::new(words::STARTED).as_str()), "instant":"2026-10-21T14:22:08.000Z", "bars":1, "count":"1 / 2"},
                {"id":"short", "done":true, "start":format!("{} {date}", Prose::new(words::STARTED).as_str()), "instant":"2026-10-21T14:22:08.000Z", "bars":0, "count":"1 / 2"},
                {"id":"old", "done":true, "start":Prose::new(words::START_TIME_NOT_RECORDED), "instant":null, "bars":0, "count":"3 / 10"}
            ]));
        }
        let _ = std::fs::remove_dir_all(folder);
        assert!(the_page().contains("if (request.terminal) box.classList.add(\"done\");"));
    }

    #[test]
    fn seventeenth_terminal_accounts_wait_for_the_final_scan_and_new_arrangements_recheck() {
        use crate::record::transfer_records::TransferStart;
        let mut row = a_request(TheKind::AutoDownload, WhatItIsOf::AJob { name: "liquid first frame".into(), job: "video".into() });
        row.id = "video".into(); row.the_job_ended = true; row.the_arrangement_is_done = true;
        row.files_done = 1; row.files_in_the_whole = 2;
        row.start = Some(TransferStart { started_at_utc: 1, start_order: 10 });
        let run = TheRunsTransfers::for_this_run();
        let draw = |run: &TheRunsTransfers, row: &ARequestKnown| {
            let mut rows = vec![row.clone()];
            the_ending_walks_this_run_has_read(&mut rows, Some(run), &|_| false);
            a_request_drawn(&known(Vec::new()), &rows[0])
        };
        let drawn = draw(&run, &row);
        assert!(!drawn.terminal && drawn.bar.is_some());
        assert!(every_word(&drawn.name).contains(words::DOWNLOADING));
        run.an_auto_ending_walk_was_read("video", Some(10), false);
        assert!(!draw(&run, &row).terminal);
        run.an_auto_ending_walk_was_read("video", Some(10), true);
        assert!(draw(&run, &row).terminal);
        row.start.as_mut().unwrap().start_order = 11;
        assert!(!draw(&run, &row).terminal, "a new arrangement must not inherit the old scan");
        run.an_auto_ending_walk_was_read("video", Some(11), true);
        assert!(draw(&run, &row).terminal);
        assert!(!draw(&TheRunsTransfers::for_this_run(), &row).terminal, "reopening must derive the final scan again");
        // An in-flight file is not terminal even when the ending scan ran.
        row.the_arrangement_is_done = false;
        assert!(!draw(&run, &row).terminal);
    }

    /// **An ended job's record, with an auto-download holding `entries`, on
    /// the one server of a fresh record** — for the twenty-sixth notes, note
    /// 4's two tests below.
    fn an_ended_job_with_an_auto_download(
        record: &crate::record::LocalRecord,
        server: &Identity,
        label: &str,
        entries: &[(&[u8], smoozy_download::arrangement::WhatSmoozyDid)],
    ) -> Identity {
        use smoozy_download::arrangement::{AReckoningEntry, AnArrangement};
        let c = record.connection();
        let mut held = crate::record::deletions::tests::a_record_in_history(server, label);
        held.what_the_note_carried.ending = Some(smoozy_queue_history::acting::note::TheEndingOnTheNote {
            outcome: smoozy_queue_history::acting::note::TheOutcomeAsWritten::Finished,
            exit_status: None, the_attempt_whose_capture_record_holds_the_words: 1,
            the_reason_the_machine_recorded: None, the_device_line_as_observed: None,
        });
        let job = held.job.clone();
        crate::record::job_records::JobRecords::over(c).write_the_provisional_record(&held).unwrap();
        let mut auto = AnArrangement::switched_on(job.clone(), server.clone(), b"/downloads".to_vec(), TheNarrowing::Everything);
        for (path, state) in entries {
            auto.reckoning.push(AReckoningEntry::acted_on(path.to_vec(), None, state.clone(), b"/downloads".to_vec()));
        }
        DownloadArrangements::over(c).write_it_down(&auto).unwrap();
        job
    }

    /// **`Download` R5's dated note of 2026-10-31: a finished auto-download
    /// stays finished across a restart.** A fresh run holds no proof that the
    /// ending walk ran; the record's written completion is that proof, so the
    /// row is under *Finished* and off the rail without a connection. The
    /// twenty-sixth notes, note 4, case 8: every ended auto-download came back
    /// as *downloading*, and stayed so on a server never connected again.
    #[test]
    fn twenty_sixth_4_a_finished_auto_download_stays_finished_after_a_restart() {
        use smoozy_download::arrangement::WhatSmoozyDid;
        let (folder, record, server) = a_record_with_one_server("finished-after-a-restart");
        let job = an_ended_job_with_an_auto_download(
            &record,
            &server,
            "night png",
            &[(b"0001.png", WhatSmoozyDid::Fetched), (b"0002.png", WhatSmoozyDid::Fetched)],
        );
        let a_fresh_run = TheRunsTransfers::for_this_run();
        let before = the_screen_the_record_answers(Some(&record), &a_fresh_run);
        assert!(!before.requests[0].terminal, "the walk has not been written down yet");
        DownloadArrangements::over(record.connection()).the_ending_walk_completed(&job).unwrap();
        let after = the_screen_the_record_answers(Some(&record), &a_fresh_run);
        assert!(after.requests[0].terminal, "a finished row came back as downloading");
        assert_eq!(after.the_rails_figure, None, "a finished row is counted on the rail");
        assert_eq!(after.requests[0].count.values(), vec!["2 / 2"]);
        // **A new start asks for a new walk**: a switch-on is not the same
        // arrangement.
        record
            .connection()
            .execute("UPDATE download_arrangement SET start_order = start_order + 1 WHERE job = ?1", [job.as_text()])
            .unwrap();
        assert!(!the_screen_the_record_answers(Some(&record), &a_fresh_run).requests[0].terminal);
        drop(record);
        let _ = std::fs::remove_dir_all(folder);
    }

    /// **`Design/Transfers § An auto-download is not a request`, its bullet of
    /// 2026-10-31: a first-frame check and its full range are one row** — the
    /// range's name, the check's file counted beside the range's frames. A
    /// check whose range is gone keeps its own row. The twenty-sixth notes,
    /// note 4, case 5: the pair was two rows of one name.
    #[test]
    fn twenty_sixth_4_a_first_frame_pair_is_one_transfers_row() {
        use smoozy_download::arrangement::WhatSmoozyDid;
        let (folder, record, server) = a_record_with_one_server("one-row-for-a-pair");
        let the_check = an_ended_job_with_an_auto_download(&record, &server, "night control", &[(b"check_0001.png", WhatSmoozyDid::Fetched)]);
        let the_range = an_ended_job_with_an_auto_download(
            &record,
            &server,
            "night control",
            &[(b"0001.png", WhatSmoozyDid::Fetched), (b"0002.png", WhatSmoozyDid::Fetched), (b"0003.png", WhatSmoozyDid::Fetched)],
        );
        for (job, role, pair) in [(&the_check, "check", &the_range), (&the_range, "range", &the_check)] {
            record
                .connection()
                .execute(
                    "UPDATE job_record SET first_frame_purpose = ?1, first_frame_pair = ?2, first_frame_number = 1,
                            first_frame_range = '1-3' WHERE job = ?3",
                    rusqlite::params![role, pair.as_text(), job.as_text()],
                )
                .unwrap();
        }
        for job in [&the_check, &the_range] {
            DownloadArrangements::over(record.connection()).the_ending_walk_completed(job).unwrap();
        }
        let screen = the_screen_the_record_answers(Some(&record), &TheRunsTransfers::for_this_run());
        assert_eq!(screen.requests.len(), 1, "the pair is two rows");
        let row = &screen.requests[0];
        assert_eq!(row.id.as_str(), the_range.as_text());
        assert_eq!(row.count.values(), vec!["4 / 4"]);
        assert!(row.terminal);
        // **A check whose range is gone keeps its own row.**
        crate::record::job_records::JobRecords::over(record.connection()).remove(&the_range).unwrap();
        let screen = the_screen_the_record_answers(Some(&record), &TheRunsTransfers::for_this_run());
        assert_eq!(screen.requests.len(), 1);
        assert_eq!(screen.requests[0].id.as_str(), the_check.as_text());
        drop(record);
        let _ = std::fs::remove_dir_all(folder);
    }

    #[test]
    fn seventeenth_a_count_interrupted_by_closing_is_not_a_successful_empty_download() {
        use crate::record::LocalRecord;
        use smoozy_download::request::{AManualRequest, KeepsTheRequest as _, EnumerationEntry, Disposition};
        let (folder, record, server) = a_record_with_one_server("piece3-count-interrupted");
        let path = record.path().to_path_buf();
        let run = Identity::issue(IdentityKind::Run).unwrap();
        let mut parent = AManualRequest::confirmed(Identity::issue(IdentityKind::Operation).unwrap(), server,
            None, b"/downloads".to_vec(), Vec::new(), run);
        DownloadRequests::over(record.connection()).begin_counting(&parent, "many frames").unwrap();
        let start = crate::record::transfer_records::parent_start(record.connection(), "download_request", "record", &parent.record.as_text()).unwrap();
        assert!(start.is_some());
        drop(record);
        let record = LocalRecord::open_at(&path).unwrap();
        let screen = the_screen_the_record_answers(Some(&record), &TheRunsTransfers::for_this_run());
        assert_eq!(screen.requests.len(), 1);
        let drawn = &screen.requests[0];
        assert_eq!(drawn.count, Words::plain(words::DID_NOT_FINISH));
        assert!(drawn.terminal && drawn.bar.is_none());
        assert!(screen.the_rails_figure.is_none());
        assert_eq!(drawn.started_at_millis.as_ref().unwrap().as_str(), start.unwrap().started_at_utc.to_string());
        // A completed count clears its phase and preserves the original start.
        let mut file = EnumerationEntry::enumerated(b"/frames/one.png".to_vec(), b"/downloads/one.png".to_vec(), 10, smoozy_download::vouching::AVouchingMark::NoClaimIsMade);
        file.disposition = Some(Disposition::ToFetch);
        parent.entries.push(file);
        DownloadRequests::over(record.connection()).write_it_down(&parent).unwrap();
        let phase: Option<String> = record.connection().query_row("SELECT counting_name FROM download_request WHERE record = ?1", [parent.record.as_text()], |r| r.get(0)).unwrap();
        assert!(phase.is_none());
        assert_eq!(crate::record::transfer_records::parent_start(record.connection(), "download_request", "record", &parent.record.as_text()).unwrap(), start);
        drop(record);
        let _ = std::fs::remove_dir_all(folder);
    }

    fn a_request(kind: TheKind, of: WhatItIsOf) -> ARequestKnown {
        ARequestKnown {
            start: None,
            id: "one".into(),
            kind,
            server: "row".into(),
            of,
            files_done: 3,
            files_in_the_whole: 10,
            bytes: Some((30, 100)),
            seconds_left: None,
            destination: PathBuf::from("/Users/artist/Renders"),
            narrowed_to: Vec::new(),
            reconnecting: None,
            waits_for_the_connection: false,
            its_period_ended_it: false,
            trouble: None,
            left_alone: 0,
            cannot_land: 0,
            may_be_incomplete: 0,
            split_between: None,
            the_job_ended: false,
            the_job_ended_short: false,
            the_arrangement_is_done: false,
            the_whole_was_read: false,
            stands: WhereTheRequestStands::Running,
            files: Vec::new(),
        }
    }

    fn known(requests: Vec<ARequestKnown>) -> WhatIsKnown {
        WhatIsKnown { servers: vec![a_server("row", "RunPod 4090")], requests, ..WhatIsKnown::default() }
    }

    /// **What a server's Files bar counts** — every kind, running or waiting,
    /// on that server alone, and nothing that has ended or was left over
    /// (`Design/File Browser § The list`, the Creator's notes piece 6).
    #[test]
    fn a_servers_count_is_its_own_requests_running_or_waiting() {
        let mut waiting = a_request(TheKind::Upload, WhatItIsOf::AFolder { folder: "kitchen".into() });
        waiting.waits_for_the_connection = true;
        let mut elsewhere = a_request(TheKind::Download, WhatItIsOf::AFolder { folder: "renders".into() });
        elsewhere.server = "another row".into();
        let mut ended = a_request(TheKind::Download, WhatItIsOf::AFolder { folder: "old".into() });
        ended.stands = WhereTheRequestStands::Ended;
        let mut left = a_request(TheKind::Upload, WhatItIsOf::AFolder { folder: "cut".into() });
        left.stands = WhereTheRequestStands::LeftOver { removable: true, by_an_earlier_run: false };
        let known = known(vec![
            a_request(TheKind::AutoDownload, WhatItIsOf::AJob { name: "shot".into(), job: "j".into() }),
            waiting,
            elsewhere,
            ended,
            left,
        ]);
        assert_eq!(the_requests_running_on(&known, "row"), 2);
        assert_eq!(the_requests_running_on(&known, "another row"), 1);
        assert_eq!(the_requests_running_on(&known, "nobody"), 0);
    }

    /// **An auto-download whose job has ended and whose files are all down is
    /// neither running nor waiting**, so neither Files' bar nor the rail counts
    /// it — while one whose job has ended and whose files are still coming, or
    /// still waiting their turn, is counted. The pass before 0.1.9 on Vast, new
    /// fault 2: the bar read *6 transfers* with nothing moving, the six being
    /// auto-download rows that all read *ended*.
    #[test]
    fn an_ended_auto_download_is_not_counted_as_running() {
        let job = |name: &str| WhatItIsOf::AJob { name: name.into(), job: name.into() };
        let mut ended = a_request(TheKind::AutoDownload, job("ended"));
        ended.the_job_ended = true;
        ended.the_arrangement_is_done = true;
        let mut still_coming = a_request(TheKind::AutoDownload, job("coming"));
        still_coming.the_job_ended = true;
        let mut queued = a_request(TheKind::AutoDownload, job("queued"));
        queued.the_job_ended = true;
        queued.files_done = 0;
        queued.files_in_the_whole = 0;
        let rendering = a_request(TheKind::AutoDownload, job("rendering"));

        let only_ended = known(vec![ended.clone(), ended.clone()]);
        assert_eq!(the_requests_running_on(&only_ended, "row"), 0);
        assert!(the_screen_from(&only_ended, None).the_rails_figure.is_none());

        let all = known(vec![ended, still_coming, queued, rendering]);
        assert_eq!(the_requests_running_on(&all, "row"), 3);
        assert_eq!(the_screen_from(&all, None).the_rails_figure.expect("three").as_str(), "3");
    }

    /// **The typography point turns the last inter-word space of a piece of
    /// prose into a non-breaking one**, so words are compared with those taken
    /// out.
    fn every_word(said: &Words) -> String {
        said.said().replace('\u{a0}', " ")
    }

    /// **Step 10c's transfer wave piece 6: a request waiting for its
    /// connection reads as the server's own row does** — amber, *Reconnecting…*
    /// with when the next attempt falls, no *Try again*, Stop still on it —
    /// **and its cut file reads part-sent and waiting**, its figure where it
    /// stood (`Design/Transfers § Waiting for the connection to come back`,
    /// `§ A part-sent file that can be carried on from`; `File Browser` U5,
    /// `Download` M11 and `Download` M11a). **The line comes from `Servers`' reading and
    /// nowhere else**: where the server's row reads anything but
    /// *Reconnecting…*, the request carries no such line. **And a request
    /// whose server's period has ended reads ended at once** (`File Browser`
    /// U7's paragraph of 2026-09-28).
    #[test]
    fn a_request_waiting_for_its_connection_reads_as_its_servers_row_does() {
        let waiting = |its_period_ended: bool| {
            let mut up = a_request(TheKind::Upload, WhatItIsOf::AFolder { folder: "kitchen/".into() });
            up.files = Vec::new();
            let mut under_way = HashMap::new();
            under_way.insert(
                up.id.clone(),
                ARequestUnderWay {
                    moving: vec![AFileMoving {
                        entry: 3,
                        path: b"/home/a/kitchen/water.vdb".to_vec(),
                        report: Report {
                            bytes_staged: 118_000_000,
                            bytes_expected: 312_000_000,
                            started_over: None,
                            verifying_a_prefix_of: None,
                        },
                        waits_for_the_connection: true,
                    }],
                    server: "row".into(),
                    waits_for_the_connection: true,
                    its_period_ended,
                    ..ARequestUnderWay::default()
                },
            );
            let mut requests = vec![up];
            what_this_run_is_moving(&mut requests, &under_way);
            requests
        };

        let mut requests = waiting(false);
        the_repairs_the_servers_read(&mut requests, &|server| {
            assert_eq!(server, "row", "the request's own server is the one read");
            Some(Some(1_900_000_000))
        });
        let drawn = the_screen_from(&known(requests), None);
        let row = &drawn.requests[0];
        let reconnecting = row
            .lines
            .iter()
            .find(|line| every_word(&line.words) == words::RECONNECTING)
            .expect("the line the server's own row carries");
        assert_eq!(reconnecting.tone, "amber2");
        assert_eq!(reconnecting.due_at.as_ref().map(|at| at.as_str()), Some("1900000000"));
        assert!(row.lines.iter().all(|line| line.link.is_none()), "no Try again");
        assert_eq!(row.control.as_ref().map(|one| one.act.as_str()), Some("stop"));
        assert_eq!(row.files[0].state, "waitpart");
        assert_eq!(row.files[0].tail.as_ref().map(|tail| tail.as_str()), Some(words::SENT));
        assert_eq!(row.files[0].staged.as_ref().map(|one| one.as_str()), Some("118000000"));

        // **The server's own row reads something else**, so no line here.
        let mut requests = waiting(false);
        the_repairs_the_servers_read(&mut requests, &|_| None);
        let drawn = the_screen_from(&known(requests), None);
        assert!(drawn.requests[0]
            .lines
            .iter()
            .all(|line| every_word(&line.words) != words::RECONNECTING));
        assert_eq!(drawn.requests[0].files[0].state, "waitpart");
        let TheStopAnswer::TheBox(the_box) = the_stop_box_for(&waiting(false)[0]) else {
            panic!("a waiting request is stoppable");
        };
        assert!(the_box.nothing_is_thrown_away.is_some(), "nothing is in flight while it waits");

        // **Its period ended**: it reads ended at once.
        let mut requests = waiting(true);
        the_repairs_the_servers_read(&mut requests, &|_| Some(None));
        assert_eq!(requests[0].stands, WhereTheRequestStands::Ended);
        assert!(requests[0].reconnecting.is_none());
    }

    /// `File Browser` U5 and `Download` M11: waiting is stated even while the
    /// primary is healthy, and upload and download use the same words.
    #[test]
    fn a_cut_transfer_says_it_waits_even_when_the_server_is_connected() {
        for kind in [TheKind::Upload, TheKind::Download, TheKind::Dependencies] {
            let mut request = a_request(kind, WhatItIsOf::AFolder { folder: "scene/".into() });
            request.waits_for_the_connection = true;
            request.reconnecting = None;
            let drawn = the_screen_from(&known(vec![request]), None);
            let row = &drawn.requests[0];
            assert!(row.lines.iter().any(|line| every_word(&line.words) == "Waiting for connection…"),
                "a waiting transfer offered Stop but never said it was waiting");
            assert!(row.lines.iter().all(|line| every_word(&line.words) != words::RECONNECTING),
                "the primary is connected, so the server is not reconnecting");
            assert_eq!(row.control.as_ref().map(|one| one.act.as_str()), Some("stop"));
        }
    }

    /// **Step 10c's transfer wave piece 5: a request this run is moving draws
    /// its files in flight with their own bars, and its bar is the finished
    /// files plus the bytes in flight** (`File Browser` U11, `Download` M16).
    #[test]
    fn a_request_under_way_draws_what_is_moving_and_its_bar_counts_it() {
        let mut up = a_request(TheKind::Upload, WhatItIsOf::AFolder { folder: "kitchen/".into() });
        up.files = Vec::new();
        up.files_done = 1;
        up.files_in_the_whole = 3;
        up.bytes = Some((100, 400));
        let mut under_way = HashMap::new();
        under_way.insert(
            up.id.clone(),
            ARequestUnderWay {
                moving: vec![AFileMoving {
                    entry: 1,
                    path: b"/home/a/kitchen/wood.png".to_vec(),
                    report: Report {
                        bytes_staged: 60,
                        bytes_expected: 200,
                        started_over: None,
                        verifying_a_prefix_of: None,
                    },
                    waits_for_the_connection: false,
                }],
                waiting: vec![b"/home/a/kitchen/bark.png".to_vec()],
                ..ARequestUnderWay::default()
            },
        );
        let mut requests = vec![up];
        what_this_run_is_moving(&mut requests, &under_way);
        let drawn = the_screen_from(&known(requests), None);
        let row = &drawn.requests[0];
        assert_eq!(row.bar, Some(ABar { value: 160, max: 400 }));
        assert_eq!(row.control.as_ref().map(|one| one.act.as_str()), Some("stop"));
        assert_eq!(row.files.len(), 2);
        assert_eq!(row.files[0].staged.as_ref().map(|one| one.as_str()), Some("60"));
        assert_eq!(row.files[0].expected.as_ref().map(|one| one.as_str()), Some("200"));
        assert_eq!(row.files[1].state, "waiting");
        assert_eq!(drawn.the_rails_figure.as_ref().map(|one| one.as_str()), Some("1"));
    }

    /// **A request nothing in this run is moving, with files owed, reads as
    /// ended** — no Stop, no bar, not on the rail, *did not finish* — in both
    /// directions (`File Browser` U7 and its paragraph of 2026-09-28,
    /// `Download` M13's; step 10c's transfer wave piece 5).
    #[test]
    fn a_request_nothing_is_moving_reads_ended_and_never_running() {
        let mut up = a_request(TheKind::Upload, WhatItIsOf::AFolder { folder: "kitchen/".into() });
        up.files = Vec::new();
        up.files_done = 1;
        up.files_in_the_whole = 3;
        let mut down = a_request(TheKind::Download, WhatItIsOf::AFolder { folder: "~/Renders".into() });
        down.id = "another".into();
        down.files = Vec::new();
        let mut requests = vec![up, down];
        what_this_run_is_moving(&mut requests, &HashMap::new());
        for request in &requests {
            assert_eq!(request.stands, WhereTheRequestStands::Ended);
            assert!(matches!(the_stop_box_for(request), TheStopAnswer::NothingToStop));
        }
        let drawn = the_screen_from(&known(requests), None);
        assert!(drawn.the_rails_figure.is_none());
        for row in &drawn.requests {
            assert!(row.bar.is_none());
            assert!(row.collapsed);
            assert!(every_word(&row.name).contains(words::DID_NOT_FINISH));
            assert_ne!(row.control.as_ref().map(|one| one.act.as_str()), Some("stop"));
        }
        assert_eq!(drawn.requests.iter().find(|r| r.id.as_str() == "one").unwrap().count, Words::plain("").and("1 / 3".to_string()));
        assert_eq!(
            drawn.requests.iter().find(|r| r.id.as_str() == "another").unwrap().control.as_ref().map(|one| one.act.as_str()),
            Some("files-on-your-computer")
        );
    }

    /// **No act on Transfers says a reason that is no longer true** — step
    /// 10c's transfer wave piece 7, item 3. *"…because nothing in Smoozy moves a
    /// transfer yet"* stopped being true at step 10c piece 10, and *"…because
    /// nothing in Smoozy keeps an auto-download yet"* with it. **Where an act
    /// works it answers nothing of the kind, and where it does not it says
    /// `Design`'s own *not finished* words.**
    #[test]
    fn no_act_on_transfers_names_a_reason_that_is_no_longer_true() {
        let stale = [
            "nothing in Smoozy moves a transfer yet",
            "nothing in Smoozy keeps an auto-download yet",
        ];
        let known = known(Vec::new());
        for act in [
            "stop-the-request", "stop-one-file", "remove", "try-again", "admin-access",
            "upload-it-again", "what-the-server-said", "which-ones-were-left-alone", "which-ones-cannot-land",
            "which-ones-may-be-incomplete", "files-on-your-computer", "choose-the-formats",
            "upload", "upload-the-found",
        ] {
            let said = what_an_act_answers(&known, the_act_named(act), "one");
            if let Some(said) = said {
                let said = said.as_str().replace('\u{a0}', " ");
                for reason in stale {
                    assert!(!said.contains(reason), "{act} answers «{said}»");
                }
            }
        }
        // **The three that act answer nothing in prose**, and the two Stops
        // mark the run's register for the request's own run to perform.
        for act in ["stop-the-request", "stop-one-file", "remove"] {
            assert!(what_an_act_answers(&known, the_act_named(act), "one").is_none(), "{act}");
        }
        let the_run = TheRunsTransfers::for_this_run();
        let request = Identity::issue(IdentityKind::Operation).expect("random bytes");
        let server = Identity::issue(IdentityKind::LocalServer).expect("random bytes");
        assert!(!the_stop_is_taken(&the_run, TheAct::StopTheRequest, &request.as_text(), None));
        let _under_way = the_run.a_request_is_under_way(&request, &server);
        assert!(!the_stop_is_taken(&the_run, TheAct::StopOneFile, &request.as_text(), None));
        assert!(the_stop_is_taken(&the_run, TheAct::StopOneFile, &request.as_text(), Some("/a/b.png")));
        assert!(!the_run.what_is_under_way()[&request.as_text()].the_user_stopped_it);
        assert!(the_stop_is_taken(&the_run, TheAct::StopTheRequest, &request.as_text(), None));
        assert!(the_run.what_is_under_way()[&request.as_text()].the_user_stopped_it);
        for reason in stale {
            assert!(
                !crate::surface::the_queue::TheActOnAJob::ALL
                    .iter()
                    .any(|act| act.answers().contains(reason)),
                "a queue entry's act still answers «{reason}»"
            );
        }
    }

    /// **The ten states are the drawing's own**, read out of its state list.
    #[test]
    fn the_ten_states_are_the_drawings_own() {
        let drawing = the_mockup();
        let list = drawing
            .split("<select id=\"state\"")
            .nth(1)
            .and_then(|rest| rest.split("</select>").next())
            .expect("the drawing has a state list");
        let mut drawn: Vec<&str> = Vec::new();
        for (at, _) in list.match_indices("<option value=\"") {
            let rest = &list[at + "<option value=\"".len()..];
            let value = &rest[..rest.find('"').expect("the value closes")];
            if value == "t" || value.starts_with("t-") || value.starts_with("fmt") {
                drawn.push(value);
            }
        }
        let mut ours = THE_STATES_THIS_PIECE_DRAWS.to_vec();
        drawn.sort_unstable();
        ours.sort_unstable();
        assert_eq!(drawn, ours);
    }

    #[test]
    fn the_page_and_this_file_name_the_same_things() {
        let page = the_page();
        for named in [
            "the_transfers_screen",
            "the_stop_box",
            "the_format_box",
            "an_act_on_transfers",
            AN_ACT_ON_TRANSFERS,
            TRANSFERS_MOVED,
            "TRANSFERS",
            "theMarkOnARow",
            "theFailureShape",
            "theCountdown",
            "theCheckIsShownOn",
            "theSettingsSectionIsOpened",
            "TRANSFERS_SECTION",
        ] {
            assert!(page.contains(named), "the page does not name {named}");
        }
        for act in [
            "stop-the-request", "stop-one-file", "turn-off", "remove", "try-again", "admin-access",
            "upload-it-again", "what-the-server-said", "details", "which-ones-were-left-alone", "which-ones-cannot-land",
            "which-ones-may-be-incomplete", "files-on-your-computer", "choose-the-formats",
        ] {
            let parsed: Result<TheAct, serde::de::value::Error> = serde::Deserialize::deserialize(
                serde::de::value::StrDeserializer::<serde::de::value::Error>::new(act),
            );
            assert!(parsed.is_ok(), "{act} is not a closed act");
        }
    }

    /// **An empty list says so rather than showing a blank screen**, and names
    /// the server where it is narrowed to one.
    #[test]
    fn an_empty_list_says_so() {
        let screen = the_screen_from(&known(Vec::new()), None);
        assert_eq!(
            every_word(screen.nothing_here.as_ref().expect("said")),
            words::NOTHING_IS_MOVING
        );
        let narrowed = the_screen_from(&known(Vec::new()), Some("row"));
        assert_eq!(
            narrowed.nothing_here.expect("said").values(),
            vec!["RunPod 4090"]
        );
        assert!(screen.the_rails_figure.is_none());
    }

    /// **The Creator's seventh notes, note 9: a finished line says at a glance
    /// how it ended** — green *done* where every file arrived, red where a file
    /// failed, grey where the user stopped it, where Smoozy closing stopped it,
    /// and for an auto-download's *ended*. Nothing still running is coloured.
    #[test]
    fn each_ending_takes_its_own_colour() {
        let class_of = |request: ARequestKnown| the_screen_from(&known(vec![request]), None).requests[0].ending_class;

        let mut done = a_request(TheKind::Download, WhatItIsOf::AJob { name: "Hallway".into(), job: "job-hallway".into() });
        done.stands = WhereTheRequestStands::Finished { at: "04:19".into() };
        assert_eq!(class_of(done), Some("e-ok"), "a download whose every file arrived");

        let mut upload_done = a_request(TheKind::Upload, WhatItIsOf::AFolder { folder: "hallway/".into() });
        upload_done.stands = WhereTheRequestStands::Finished { at: "15:02".into() };
        assert_eq!(class_of(upload_done), Some("e-ok"), "an upload whose every file arrived");

        let mut failed = a_request(TheKind::Upload, WhatItIsOf::AFolder { folder: "hallway/".into() });
        failed.stands = WhereTheRequestStands::Ended;
        failed.files = vec![a_file("hallway/a.exr", WhatHappenedToTheFile::RefusedByTheServer)];
        assert_eq!(class_of(failed), Some("e-fail"), "a file that failed");

        let mut not_put_in_place = a_request(TheKind::Download, WhatItIsOf::AJob { name: "Hallway".into(), job: "job-hallway".into() });
        not_put_in_place.stands = WhereTheRequestStands::Ended;
        not_put_in_place.files = vec![a_file("a.exr", WhatHappenedToTheFile::CouldNotBeReadHere)];
        assert_eq!(class_of(not_put_in_place), Some("e-fail"), "a download with a file that failed");

        let mut stopped = a_request(TheKind::Upload, WhatItIsOf::AFolder { folder: "kitchen/".into() });
        stopped.stands = WhereTheRequestStands::Ended;
        assert_eq!(class_of(stopped), None, "a request the user stopped stays grey");

        let mut left_over = a_request(TheKind::Upload, WhatItIsOf::AFolder { folder: "kitchen/".into() });
        left_over.stands = WhereTheRequestStands::LeftOver { removable: true, by_an_earlier_run: true };
        assert_eq!(class_of(left_over), None, "one Smoozy closing stopped stays grey");

        let mut auto = a_request(TheKind::AutoDownload, WhatItIsOf::AJob { name: "Turntable".into(), job: "job-turntable".into() });
        auto.the_job_ended = true;
        auto.the_arrangement_is_done = true;
        auto.stands = WhereTheRequestStands::Finished { at: "13:40".into() };
        assert_eq!(class_of(auto), None, "an auto-download's ended says nothing about the render");

        let running = a_request(TheKind::Upload, WhatItIsOf::AFolder { folder: "kitchen/".into() });
        assert_eq!(class_of(running), None, "nothing running is coloured");
    }

    /// **The Creator's seventh notes, piece Z5b: a transfer its line or its
    /// server's period ended reads red** — the Creator, 2026-10-10, asked
    /// whether such a transfer should show red: *"Yes, it should."* Grey stays
    /// for the user's own *Stop*, and green for one that finished whole. Each
    /// is drawn the way the screen draws it: the run's register, then the
    /// leftovers' reading over a throwaway record.
    #[test]
    fn a_transfer_its_line_or_period_ended_ends_red() {
        let the_run = TheRunsTransfers::for_this_run();
        let issue = |kind| Identity::issue(kind).expect("random bytes");
        let record = rusqlite::Connection::open_in_memory().expect("a throwaway record");
        let class_of = |request: &Identity, server: &Identity, stands: WhereTheRequestStands| {
            let mut one = a_request(TheKind::Upload, WhatItIsOf::AFolder { folder: "kitchen/".into() });
            one.id = request.as_text();
            one.server = server.as_text();
            one.stands = stands;
            let mut requests = vec![one];
            let under_way = the_run.what_is_under_way();
            what_this_run_is_moving(&mut requests, &under_way);
            the_leftovers_are_told(&record, &mut requests, &under_way, Some(&the_run), &HashMap::new());
            the_screen_from(&known(requests), None).requests[0].ending_class
        };

        // **Its line was cut**: it waited for the connection, the repair never
        // came, the period ended, and its run returned with files owed.
        let (cut, its_server) = (issue(IdentityKind::Operation), issue(IdentityKind::LocalServer));
        {
            let _under_way = the_run.a_request_is_under_way(&cut, &its_server);
            the_run.it_waits_for_the_connection(&cut.as_text(), None, true);
            the_run.a_servers_period_ended(&its_server.as_text());
        }
        assert_eq!(
            class_of(&cut, &its_server, WhereTheRequestStands::Running),
            Some("e-fail"),
            "a transfer its line cut"
        );

        // **Its server's period ended**, its run not yet returned.
        let (ended, its_server) = (issue(IdentityKind::Operation), issue(IdentityKind::LocalServer));
        let _held = the_run.a_request_is_under_way(&ended, &its_server);
        the_run.a_servers_period_ended(&its_server.as_text());
        assert_eq!(
            class_of(&ended, &its_server, WhereTheRequestStands::Running),
            Some("e-fail"),
            "a transfer its server's period ended"
        );

        // **The user stopped it** — and the period that ended after does not
        // make it red.
        let (stopped, its_server) = (issue(IdentityKind::Operation), issue(IdentityKind::LocalServer));
        {
            let _under_way = the_run.a_request_is_under_way(&stopped, &its_server);
            assert!(the_run.the_user_stops(&stopped.as_text(), None));
            the_run.a_servers_period_ended(&its_server.as_text());
        }
        assert_eq!(
            class_of(&stopped, &its_server, WhereTheRequestStands::Running),
            None,
            "a transfer the user stopped stays grey"
        );

        // **It finished whole.**
        let (whole, its_server) = (issue(IdentityKind::Operation), issue(IdentityKind::LocalServer));
        drop(the_run.a_request_is_under_way(&whole, &its_server));
        assert_eq!(
            class_of(&whole, &its_server, WhereTheRequestStands::Finished { at: "15:02".into() }),
            Some("e-ok"),
            "a transfer that finished whole"
        );
    }

    /// **The ending word is drawn with its class, and escapes the dimming** —
    /// the drawing's own rules (`Smoozy.html`, `.rqm.e-ok`, `.rqm.e-fail`): a
    /// dimmed whole line would dim the word with it, whatever its colour.
    #[test]
    fn the_ending_word_is_coloured_and_not_dimmed() {
        assert!(
            the_page().contains("if (request.ending_class) count.classList.add(request.ending_class);"),
            "the page does not put the ending's class on the ending word"
        );
        let sheet = include_str!("../../web/the-drawing.css");
        assert!(!sheet.contains(".req.done { opacity"), "the whole finished line is dimmed, the word with it");
        for rule in [
            ".req.done > :not(.rqh), .req.done .rqh > :not(.rqm.e-ok):not(.rqm.e-fail) { opacity: .72; }",
            ".rqm.e-ok { color: var(--ok); font-weight: 600; }",
            ".rqm.e-fail { color: var(--bad); font-weight: 600; }",
        ] {
            assert!(sheet.contains(rule), "the stylesheet lacks {rule}");
        }
    }

    /// **A finished request never pushes a running one down**, and collapses
    /// unless it carries an amber line.
    #[test]
    fn finished_requests_sit_below_and_collapse_unless_amber() {
        let mut done = a_request(TheKind::Download, WhatItIsOf::AJob { name: "Hallway".into(), job: "job-hallway".into() });
        done.id = "done".into();
        done.stands = WhereTheRequestStands::Finished { at: "04:19".into() };
        let mut amber = a_request(TheKind::AutoDownload, WhatItIsOf::AJob { name: "Turntable".into(), job: "job-turntable".into() });
        amber.id = "amber".into();
        amber.the_job_ended = true;
        amber.left_alone = 7;
        amber.stands = WhereTheRequestStands::Finished { at: "13:40".into() };
        let running = a_request(TheKind::Upload, WhatItIsOf::AFolder { folder: "kitchen/".into() });
        let screen = the_screen_from(&known(vec![done, amber, running]), None);
        assert_eq!(screen.requests[0].id.as_str(), "one");
        assert_eq!(screen.requests.iter().map(|r| r.id.as_str()).collect::<Vec<_>>(), ["one", "amber", "done"]);
        assert!(screen.requests[2].collapsed);
        assert!(!screen.requests[1].collapsed);
        assert_eq!(screen.the_rails_figure.expect("one running").as_str(), "1");
        // The way back replaces Stop once there is nothing to stop.
        assert_eq!(
            screen.requests.iter().find(|r| r.id.as_str() == "done").unwrap().control.as_ref().expect("a control").act.as_str(),
            "files-on-your-computer"
        );
        assert_eq!(screen.requests.iter().find(|r| r.id.as_str() == "amber").unwrap().control.as_ref().expect("a control").act.as_str(), "open-the-job");
    }

    /// **An auto-download reads in files, is turned off rather than stopped, and
    /// reads *ended* and never *complete*.**
    #[test]
    fn an_auto_download_reads_in_files_and_ends() {
        let mut auto = a_request(TheKind::AutoDownload, WhatItIsOf::AJob { name: "Bedroom test".into(), job: "job-bedroom".into() });
        auto.bytes = None;
        auto.files_done = 412;
        auto.files_in_the_whole = 720;
        let screen = the_screen_from(&known(vec![auto.clone()]), None);
        assert_eq!(screen.requests[0].control.as_ref().expect("control").act.as_str(), "turn-off");
        assert_eq!(screen.requests[0].bar, Some(ABar { value: 412, max: 720 }));
        auto.the_job_ended = true;
        auto.the_arrangement_is_done = true;
        let ended = the_screen_from(&known(vec![auto]), None);
        let said = every_word(&ended.requests[0].where_it_goes);
        assert!(said.contains(words::ENDED_DASH), "{said}");
        assert!(!said.to_lowercase().contains("complete"));
        assert_eq!(ended.requests[0].control.as_ref().expect("control").act.as_str(), "open-the-job");
    }

    /// **A finished auto-download collapses** — `Design/Transfers`: *Finished
    /// requests collapse to one line each* and sit below whatever is still
    /// running. The Creator's third notes, note 12: an auto-download that
    /// brought all 5 of its 5 pictures home stood *running*, read *ended*, and
    /// was never greyed. One with a file short, or a file that failed, is not
    /// collapsed.
    #[test]
    fn a_finished_auto_download_collapses() {
        let mut auto = a_request(TheKind::AutoDownload, WhatItIsOf::AJob { name: "note 12".into(), job: "job-12".into() });
        auto.id = "auto".into();
        auto.bytes = None;
        auto.files_done = 5;
        auto.files_in_the_whole = 5;
        auto.the_job_ended = true;
        auto.the_arrangement_is_done = true;
        auto.the_whole_was_read = true;
        auto.files = vec![a_file("frame_0001.png", WhatHappenedToTheFile::Moving)];
        let mut upload = a_request(TheKind::Upload, WhatItIsOf::AFolder { folder: "kitchen".into() });
        upload.id = "upload".into();
        let screen = the_screen_from(&known(vec![auto.clone(), upload]), None);
        assert_eq!(screen.requests[0].id.as_str(), "upload", "a finished request sits below what still runs");
        let drawn = &screen.requests[1];
        assert!(drawn.collapsed, "**5 of 5 home, and the row was not greyed**");
        assert_eq!(drawn.bar, None);
        assert!(drawn.files.is_empty());
        assert!(every_word(&drawn.where_it_goes).contains(words::ENDED_DASH));

        let mut short = auto.clone();
        short.files_done = 4;
        assert!(!the_screen_from(&known(vec![short]), None).requests[0].collapsed, "a file short is not finished");
        let mut failed = auto;
        failed.files = vec![a_file("frame_0002.png", WhatHappenedToTheFile::NotPutInPlace { has_details: false })];
        assert!(!the_screen_from(&known(vec![failed]), None).requests[0].collapsed, "a failed file keeps its row");
    }

    /// **`Download` N6: an auto-download reads against the job's whole once a
    /// pass in this run has read it, and against the reckoning's length until
    /// then** — never below the files already counted, and a request that is
    /// not an auto-download is not touched. Step 10c's transfer wave piece 4:
    /// the tenth pass on Hyperstack read *6 / 73* and *36 / 188* for a
    /// 240-frame job.
    #[test]
    fn an_auto_download_reads_against_the_jobs_whole_once_a_pass_has_read_it() {
        use crate::servers::the_machines_work::TheJobsWhole;
        let mut auto = a_request(TheKind::AutoDownload, WhatItIsOf::AJob { name: "p12h10 long".into(), job: "job-long".into() });
        auto.bytes = None;
        auto.files_done = 6;
        auto.files_in_the_whole = 6;
        let mut unread = a_request(TheKind::AutoDownload, WhatItIsOf::AJob { name: "p12h10 long2".into(), job: "job-long2".into() });
        unread.bytes = None;
        unread.files_done = 3;
        unread.files_in_the_whole = 4;
        let mut grew = a_request(TheKind::AutoDownload, WhatItIsOf::AJob { name: "p12h10 video".into(), job: "job-video".into() });
        grew.bytes = None;
        grew.files_done = 3;
        grew.files_in_the_whole = 3;
        let mut upload = a_request(TheKind::Upload, WhatItIsOf::AFolder { folder: "kitchen/".into() });
        upload.files_in_the_whole = 2;
        let mut requests = vec![auto, unread, grew, upload];
        the_wholes_this_run_has_read(&mut requests, &|job| match job {
            "job-long" => Some(TheJobsWhole { files: 240, it_may_grow: false }),
            "job-video" => Some(TheJobsWhole { files: 1, it_may_grow: true }),
            "kitchen/" => Some(TheJobsWhole { files: 99, it_may_grow: false }),
            _ => None,
        });
        assert_eq!(requests[0].files_in_the_whole, 240, "**the row read the reckoning's length, not the job's whole**");
        assert_eq!(requests[1].files_in_the_whole, 4, "no pass read this job's whole, so the reckoning's length stands");
        assert_eq!(requests[2].files_in_the_whole, 3, "never below the files already counted");
        assert_eq!(requests[3].files_in_the_whole, 2, "an upload is not an auto-download");
        let screen = the_screen_from(&known(vec![requests[0].clone()]), None);
        assert_eq!(screen.requests[0].bar, Some(ABar { value: 6, max: 240 }));
    }

    /// **`Download` N6's last sentence, 2026-10-19: once the job has ended
    /// short of a clean finish, the whole is what the ending's walk found** —
    /// the reckoning's own length, with what the user's narrowing cut out
    /// left out of it (`Download` N7's third bullet) — and the asked range's count read
    /// by a pass never takes its place. A clean finish still reads against the
    /// asked whole. Out of the live pass on Vast for the thirteenth and
    /// fourteenth notes: a job stopped at 52 of 150 frames, three files each,
    /// read *ended 100 / 450*.
    ///
    /// **The reckoning holds both cases `Download` N6 names**: the ending walk enters a
    /// file it found at zero bytes as *never coming*, and enters nothing for a
    /// file it did not find (`smoozy_download::acting::running`'s
    /// `a_short_ending_leaves_an_entry_the_walk_did_not_find_out_of_the_whole`),
    /// so 156 here is 150 found whole and six found at zero bytes.
    #[test]
    fn a_job_that_ended_short_reads_against_what_the_ending_walk_found() {
        use smoozy_queue_history::acting::note::{TheEndingOnTheNote, TheOutcomeAsWritten};
        let folder = std::env::temp_dir().join(format!("smoozy-transfers-ended-short-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a scratch folder");
        let record = crate::record::LocalRecord::open_at(&folder.join("local-record.sqlite3")).expect("the record");
        let server = Identity::issue(IdentityKind::LocalServer).expect("randomness");
        let store = crate::record::job_records::JobRecords::over(record.connection());
        let ended = |outcome: TheOutcomeAsWritten| {
            let mut job = crate::record::deletions::tests::a_record_in_history(&server, "pv long");
            job.what_the_note_carried.ending = Some(TheEndingOnTheNote {
                outcome,
                exit_status: None,
                the_attempt_whose_capture_record_holds_the_words: 1,
                the_reason_the_machine_recorded: None,
                the_device_line_as_observed: None,
            });
            store.write_the_provisional_record(&job).expect("written");
            job.job
        };
        let stopped = ended(TheOutcomeAsWritten::LeftStopped);
        let failed = ended(TheOutcomeAsWritten::Failed);
        let finished = ended(TheOutcomeAsWritten::Finished);
        let counted = |job: &Identity, entries: usize, skipped: usize| {
            crate::record::download_records::AnArrangementCounted {
                job: job.clone(),
                destination: b"/Users/artist/Downloads".to_vec(),
                narrowing: TheNarrowing::Everything,
                desired_state: DesiredState::On,
                entries,
                fetched: 100,
                left_alone: 0,
                owed: entries - skipped - 100,
                skipped_by_narrowing: skipped,
                cannot_land: 0,
            }
        };
        let mut requests = vec![
            an_arrangement_known(record.connection(), "row", &counted(&stopped, 156, 0)),
            an_arrangement_known(record.connection(), "row", &counted(&failed, 156 + 104, 104)),
            an_arrangement_known(record.connection(), "row", &counted(&finished, 156, 0)),
        ];
        drop(store);
        drop(record);
        let _ = std::fs::remove_dir_all(&folder);
        assert!(requests[0].the_job_ended_short && requests[1].the_job_ended_short);
        assert!(!requests[2].the_job_ended_short && requests[2].the_job_ended);
        assert_eq!(requests[1].files_in_the_whole, 156, "what the narrowing cut out is out of the whole");

        the_wholes_this_run_has_read(&mut requests, &|_| {
            Some(crate::servers::the_machines_work::TheJobsWhole { files: 450, it_may_grow: false })
        });
        assert_eq!(
            requests[0].files_in_the_whole, 156,
            "**a stopped job read against the asked range's 450, not what the ending's walk found**"
        );
        assert_eq!(requests[1].files_in_the_whole, 156, "a failed job too");
        assert!(requests[0].the_whole_was_read);
        assert_eq!(requests[2].files_in_the_whole, 450, "a clean finish reads against the asked whole");
        let screen = the_screen_from(&known(vec![requests[0].clone()]), None);
        assert_eq!(screen.requests[0].bar, Some(ABar { value: 100, max: 156 }));
    }

    /// **`Download` E2: between the job's ending and done, E1 step 4's *ended —
    /// 0 of 3 files* is not said.** The seventh pass on Hyperstack saw it on a
    /// row whose three files had not yet started to land.
    #[test]
    fn an_auto_download_says_no_ended_count_before_it_is_done() {
        let mut auto = a_request(TheKind::AutoDownload, WhatItIsOf::AJob { name: "p12h7 cube".into(), job: "job-p12h7".into() });
        auto.bytes = None;
        auto.files_done = 0;
        auto.files_in_the_whole = 3;
        auto.the_job_ended = true;
        auto.the_arrangement_is_done = false;
        let screen = the_screen_from(&known(vec![auto]), None);
        let said = every_word(&screen.requests[0].where_it_goes);
        assert!(!said.contains(words::ENDED_DASH), "{said}");
        assert_eq!(screen.requests[0].bar, Some(ABar { value: 0, max: 3 }));
    }

    /// **An auto-download waiting its turn reads *queued*, never *ended 0 /
    /// 0*** — after 0.1.8, the checks' leftovers, item 5, ruled a fault by the
    /// build runner: the check before 0.1.8 saw an auto-download waiting
    /// behind another read *ended 0 / 0* until it started, which says it ended
    /// with nothing. The word is the one this screen already says for a file
    /// waiting its turn. **Once a pass has read the whole**, a job that wrote
    /// nothing to fetch reads *ended* as before.
    #[test]
    fn an_auto_download_waiting_its_turn_reads_queued() {
        let mut auto = a_request(TheKind::AutoDownload, WhatItIsOf::AJob { name: "c18 cube".into(), job: "job-cube".into() });
        auto.bytes = None;
        auto.files_done = 0;
        auto.files_in_the_whole = 0;
        auto.the_job_ended = true;
        auto.the_arrangement_is_done = false;
        let screen = the_screen_from(&known(vec![auto.clone()]), None);
        let name = every_word(&screen.requests[0].name);
        let count = every_word(&screen.requests[0].count);
        assert!(name.contains(words::QUEUED), "{name}");
        assert!(!name.contains(words::ENDED), "{name}");
        assert!(!count.contains("0 / 0"), "{count}");
        assert_eq!(screen.requests[0].bar, None);
        // **The control is the ended job's, as `Design/Transfers` has it**: *Open
        // the job*.
        assert_eq!(screen.requests[0].control.as_ref().expect("control").act.as_str(), "open-the-job");

        let mut read = vec![auto];
        crate::surface::transfers::the_wholes_this_run_has_read(&mut read, &|_| {
            Some(crate::servers::the_machines_work::TheJobsWhole { files: 0, it_may_grow: false })
        });
        let screen = the_screen_from(&known(read), None);
        let name = every_word(&screen.requests[0].name);
        assert!(name.contains(words::DOWNLOADING), "{name}");
    }

    /// **The row reads the mark and never the figure** — a held figure with a
    /// start-over mark says it started over; with no mark on an interrupted
    /// record it carries on; a verifying mark gets its own sentence and no
    /// time left.
    #[test]
    fn the_row_reads_the_mark_and_never_the_figure() {
        let request = a_request(TheKind::Upload, WhatItIsOf::AFolder { folder: "x/".into() });
        let mut restarted = a_file("sim/water.vdb", WhatHappenedToTheFile::Moving);
        restarted.report.started_over = Some(StartOverCause::TheStagedPrefixDidNotVerify);
        assert_eq!(
            every_word(&a_file_drawn(&request, &restarted).note.expect("a line").words),
            words::UPLOADING_FROM_THE_START
        );

        let mut carries_on = a_file("sim/water.vdb", WhatHappenedToTheFile::Moving);
        carries_on.record_ending = Some(Ending::Interrupted);
        let drawn = a_file_drawn(&request, &carries_on);
        assert_eq!(drawn.state, "waitpart");
        assert!(drawn.stop.is_none(), "it carries on by itself and offers nothing");

        let mut verifying = a_file("sim/water.vdb", WhatHappenedToTheFile::Moving);
        verifying.report.verifying_a_prefix_of = Some(3_900_000_000);
        let drawn = a_file_drawn(&request, &verifying);
        let note = drawn.note.expect("a line");
        assert_eq!(note.tone, "amber");
        assert_eq!(note.words.values(), vec!["3.9\u{a0}GB"]);
    }

    /// **No transfer's permission refusal carries the admin-access offer, in
    /// either direction** — `File Browser` P6a (*there is no elevated
    /// upload*) and `Download` M5. It offered one on an upload until
    /// 2026-09-27, which `Design/Transfers` asks for and the architecture
    /// rules out (Rule 10). **Nor a plain *Try again***, which that design's
    /// own sentence says would retry the same refused write for ever.
    #[test]
    fn no_transfer_refusal_offers_admin_access() {
        let up = a_request(TheKind::Upload, WhatItIsOf::AFolder { folder: "x/".into() });
        let down = a_request(TheKind::Download, WhatItIsOf::AJob { name: "x".into(), job: "job-x".into() });
        let refused = a_file("a.exr", WhatHappenedToTheFile::RefusedByTheServer);
        let acts = |request: &ARequestKnown| -> Vec<String> {
            a_file_drawn(request, &refused)
                .failure
                .expect("the failure shape")
                .controls
                .iter()
                .map(|control| control.act.as_str().to_string())
                .collect()
        };
        for request in [&up, &down] {
            let acts = acts(request);
            assert!(!acts.iter().any(|act| act == "admin-access" || act == "try-again"), "{acts:?}");
            assert!(acts.iter().any(|act| act == "what-the-server-said"), "{acts:?}");
        }
        // A file that never reached the server offers no server words.
        let unread = a_file("a.png", WhatHappenedToTheFile::CouldNotBeReadHere);
        assert!(!a_file_drawn(&up, &unread)
            .failure
            .expect("shape")
            .controls
            .iter()
            .any(|control| control.is.as_str() == "raw-text"));
    }

    /// **An upload request is read onto Transfers, and a file whose write the
    /// machine refused is drawn failed on its own row with the server's words
    /// behind *What the server said*** — `File Browser` P1 and `File Browser` P6a,
    /// `Design/File Browser`'s *a refused write on an upload surfaces on the
    /// transfer row*. **The request has ended and it does not collapse**, the
    /// failed row being the part that still needs reading. The seventh pass on
    /// Hyperstack saw *Nothing is uploading or downloading.* instead.
    #[test]
    fn a_refused_upload_is_drawn_on_its_row_with_what_the_server_said() {
        let issue = |kind| Identity::issue(kind).expect("randomness");
        // **As the store counts it**: one file to send, and it failed.
        let request = crate::record::upload_requests::AnUploadRequestCounted {
            changed_landings: Vec::new(),
            record: issue(crate::record::upload_requests::THE_RECORD_IDENTITY_KIND),
            destination: b"/".to_vec(),
            files_in_the_whole: 1,
            files_done: 0,
            bytes_in_the_whole: 93_790,
            bytes_done: 0,
            every_unit_has_ended: true,
            failed_landings: vec![(b"/cube.blend".to_vec(), 93_790)],
            nothing_is_decided: false,
            stopped_at_the_question: false,
            opened_by_run: issue(IdentityKind::Run),
        };
        crate::uploading::the_refusals_this_run().lock().expect("held").insert(
            (request.record.as_text(), b"/cube.blend".to_vec()),
            "the server said permission was denied".to_string(),
        );
        let known_here = known(vec![an_upload_request_known("row", &request)]);
        let screen = the_screen_from(&known_here, None);
        let drawn = &screen.requests[0];
        assert_eq!(drawn.tag_class, "up");
        assert!(!drawn.collapsed, "an ended request with a refused file collapsed");
        assert_eq!(drawn.files.len(), 1);
        assert_eq!(drawn.files[0].state, "failed");
        assert_eq!(
            drawn.files[0].failure.as_ref().expect("the shape").sentence.as_str().replace('\u{a0}', " "),
            words::CANNOT_WRITE_THERE
        );
        let said = what_an_act_answers(&known_here, TheAct::WhatTheServerSaid, &request.record.as_text())
            .expect("an answer");
        assert_eq!(said.as_str().replace('\u{a0}', " "), "The server said permission was denied.");
    }

    /// **An upload with a file that failed or was refused does not read
    /// *done*** — Rule 8, `Design/Transfers § A file that failed`: *"A
    /// request with a file that failed never reads done; it reads — did not
    /// finish with its count."* **Whether or not this run holds the server's
    /// words**: a later sitting draws no row for the file and still counts it
    /// failed. **A dependency check's send is the same**, its rows being the
    /// same rows. **A request whose every file went up still reads *done***.
    /// Step 10c's thirteenth-pass fix piece 3b: before it, an upload with a
    /// refused file read *1 file into / · done* above its red row.
    #[test]
    fn an_upload_with_a_refused_file_reads_did_not_finish() {
        // **The count as it reads**: *done* is a word, *1 / 2* a figure.
        let the_count = |count: &Words| format!("{}{}", every_word(count), count.values().concat());
        let issue = |kind| Identity::issue(kind).expect("randomness");
        let counted = |failed: Vec<(Vec<u8>, u64)>| crate::record::upload_requests::AnUploadRequestCounted {
            changed_landings: Vec::new(),
            record: issue(crate::record::upload_requests::THE_RECORD_IDENTITY_KIND),
            destination: b"/".to_vec(),
            files_in_the_whole: 2,
            files_done: 2 - failed.len(),
            bytes_in_the_whole: 187_580,
            bytes_done: 93_790 * (2 - failed.len() as u64),
            every_unit_has_ended: true,
            failed_landings: failed,
            nothing_is_decided: false,
            stopped_at_the_question: false,
            opened_by_run: issue(IdentityKind::Run),
        };
        let refused = counted(vec![(b"/cube.blend".to_vec(), 93_790)]);
        let unsaid = counted(vec![(b"/cube.blend".to_vec(), 93_790)]);
        let done = counted(Vec::new());
        crate::uploading::the_refusals_this_run().lock().expect("held").insert(
            (refused.record.as_text(), b"/cube.blend".to_vec()),
            "the server said permission was denied".to_string(),
        );
        let screen = the_screen_from(
            &known(vec![
                an_upload_request_known("row", &refused),
                an_upload_request_known("row", &unsaid),
                an_upload_request_known("row", &done),
            ]),
            None,
        );
        let requests: Vec<_> = [refused.record.as_text(), unsaid.record.as_text(), done.record.as_text()].into_iter().map(|id| screen.requests.iter().find(|r| r.id.as_str() == id).unwrap()).collect();
        for (drawn, rows) in [(&requests[0], 1), (&requests[1], 0)] {
            assert_eq!(drawn.tag_class, "up");
            assert_eq!(the_count(&drawn.count), "1 / 2");
            assert!(every_word(&drawn.name).ends_with(words::DID_NOT_FINISH), "{}", every_word(&drawn.name));
            assert!(drawn.bar.is_none());
            assert_eq!(drawn.files.len(), rows);
        }
        assert!(screen.the_rails_figure.is_none(), "an ended request is counted as running");
        assert_eq!(requests[0].files[0].state, "failed");
        assert_eq!(the_count(&requests[2].count), words::DONE);

        // **A dependency check's send with a refused or failed file**, against
        // one whose every file was published.
        let a_file = |landing: &[u8], ended| crate::the_found_files_sent::AFileSent {
            landing: landing.to_vec(),
            size: 10,
            transfer: None,
            ended: Some(ended),
        };
        use crate::the_found_files_sent::HowItEnded::{Failed, Published, Refused};
        let row = "p13-fix-3b-dependencies";
        let sends = [
            vec![a_file(b"/p/a.png", Published), a_file(b"/p/b.png", Refused)],
            vec![a_file(b"/p/a.png", Published), a_file(b"/p/b.png", Failed)],
            vec![a_file(b"/p/a.png", Published), a_file(b"/p/b.png", Published)],
        ]
        .map(|files| crate::the_found_files_sent::ASend {
            request: issue(IdentityKind::Operation),
            server: row.to_string(),
            file: "/p/scene.blend".to_string(),
            files,
        });
        crate::the_found_files_sent::the_sends_this_run()
            .lock()
            .expect("held")
            .extend(sends.iter().cloned());
        let servers = vec![a_server(row, "Desk")];
        let held = the_requests_the_sends_hold(&servers);
        let drawn = the_screen_from(&WhatIsKnown { servers, requests: held, ..WhatIsKnown::default() }, None);
        let count_of = |send: &crate::the_found_files_sent::ASend| {
            let id = send.request.as_text();
            let one = drawn.requests.iter().find(|one| one.id.as_str() == id).expect("the send is drawn");
            the_count(&one.count)
        };
        assert_eq!(count_of(&sends[0]), "1 / 2");
        assert_eq!(count_of(&sends[1]), "1 / 2");
        assert_eq!(count_of(&sends[2]), words::DONE);
    }

    /// **An upload that sent nothing it was asked for never reads *done***
    /// — `Design/Transfers § A file that failed`, the sentence of 2026-10-02,
    /// kept for an upload as the download keeps it. **Stopped at `File
    /// Browser` U4's question** reads *— did not finish · 0 / 1*. **Nothing
    /// decided** is not ended by its units: it reads as under way while this
    /// run holds it before its answer, and ended — *— did not finish* — once
    /// it does not. Step 10c's check-before-0.1.7 fix piece 1: on the
    /// Hyperstack check, a file dragged in over a connection that was gone
    /// read *UPLOAD 0 files into /home/ubuntu · done* at once.
    #[test]
    fn an_upload_that_sent_nothing_it_was_asked_for_does_not_read_done() {
        let the_count = |count: &Words| format!("{}{}", every_word(count), count.values().concat());
        let issue = |kind| Identity::issue(kind).expect("randomness");
        let asked = |nothing_is_decided: bool, stopped_at_the_question: bool| {
            crate::record::upload_requests::AnUploadRequestCounted {
            changed_landings: Vec::new(),
                record: issue(crate::record::upload_requests::THE_RECORD_IDENTITY_KIND),
                destination: b"/home/ubuntu".to_vec(),
                files_in_the_whole: 1,
                files_done: 0,
                bytes_in_the_whole: 1_073_741_824,
                bytes_done: 0,
                every_unit_has_ended: !nothing_is_decided,
                failed_landings: Vec::new(),
                nothing_is_decided,
                stopped_at_the_question,
                opened_by_run: issue(IdentityKind::Run),
            }
        };
        let stopped = asked(false, true);
        let lent_nothing = asked(true, false);
        let still_asking = asked(true, false);
        let mut requests = vec![
            an_upload_request_known("row", &stopped),
            an_upload_request_known("row", &lent_nothing),
            an_upload_request_known("row", &still_asking),
        ];
        assert_eq!(requests[0].stands, WhereTheRequestStands::Ended);
        assert_eq!(requests[1].stands, WhereTheRequestStands::Running);
        // **What this run holds**: only the request whose question is open.
        let mut under_way = HashMap::new();
        under_way.insert(
            still_asking.record.as_text(),
            ARequestUnderWay {
                server: "row".to_string(),
                ..ARequestUnderWay::default()
            },
        );
        what_this_run_is_moving(&mut requests, &under_way);
        let screen = the_screen_from(&known(requests), None);
        let drawn = |request: &crate::record::upload_requests::AnUploadRequestCounted| {
            let id = request.record.as_text();
            screen
                .requests
                .iter()
                .find(|one| one.id.as_str() == id)
                .expect("the request is drawn")
        };
        for ended in [drawn(&stopped), drawn(&lent_nothing)] {
            assert_eq!(ended.tag_class, "up");
            assert_eq!(the_count(&ended.count), "0 / 1");
            assert!(
                every_word(&ended.name).ends_with(words::DID_NOT_FINISH),
                "{}",
                every_word(&ended.name)
            );
        }
        let asking = drawn(&still_asking);
        assert_ne!(the_count(&asking.count), words::DONE);
        assert!(
            !every_word(&asking.name).ends_with(words::DID_NOT_FINISH),
            "a request whose question is open read as ended"
        );
    }

    /// **A manual download with a file that failed does not read *done*** —
    /// Rule 8, `Design/Transfers § A file that failed` with E2-1. It reads *—
    /// did not finish* with its count, its failed file is on its own row with
    /// *Download failed.*, *Details* is drawn only where this run holds what
    /// this computer said, there is no *Try again*, and the request does not
    /// collapse. **A request whose every file came down still reads *done***.
    /// Step 10c's thirteenth-pass fix piece 3: before it, the thirteenth pass's
    /// *Replace* that wrote nothing read *1 file into … · done*.
    #[test]
    fn a_download_with_a_failed_file_reads_did_not_finish_and_draws_the_file() {
        let issue = |kind| Identity::issue(kind).expect("randomness");
        let counted = |failed: Vec<(Vec<u8>, u64)>| crate::record::download_records::AManualRequestCounted {
            record: issue(IdentityKind::Operation),
            destination: b"/Users/artist/Downloads".to_vec(),
            files_in_the_whole: 1,
            files_done: if failed.is_empty() { 1 } else { 0 },
            bytes_in_the_whole: 93_790,
            bytes_done: if failed.is_empty() { 93_790 } else { 0 },
            every_unit_has_ended: true,
            cannot_land: 0,
            nothing_is_decided: false,
            opened_by_run: issue(IdentityKind::Run),
            failed_landings: failed,
            stopped_at_the_question: false,
            could_not_be_read: 0,
        };
        let landing = b"/Users/artist/Downloads/p13-cube.blend".to_vec();
        let failed = counted(vec![(landing.clone(), 93_790)]);
        let quiet = counted(vec![(landing.clone(), 93_790)]);
        let done = counted(Vec::new());
        crate::downloading::the_failures_this_run().lock().expect("held").insert(
            (failed.record.as_text(), landing.clone()),
            "File already exists. Not overwritten.".to_string(),
        );
        let screen = the_screen_from(
            &known(vec![
                a_manual_request_known("row", &failed),
                a_manual_request_known("row", &quiet),
                a_manual_request_known("row", &done),
            ]),
            None,
        );
        let requests: Vec<_> = [failed.record.as_text(), quiet.record.as_text(), done.record.as_text()]
            .into_iter().map(|id| screen.requests.iter().find(|r| r.id.as_str() == id).unwrap()).collect();
        let drawn = &requests[0];
        assert_eq!(drawn.tag_class, "dn");
        assert!(every_word(&drawn.name).ends_with(words::DID_NOT_FINISH), "{}", every_word(&drawn.name));
        assert_eq!(drawn.count.values(), vec!["0 / 1"]);
        assert!(!drawn.collapsed, "a request with a failed file collapsed");
        assert!(drawn.bar.is_none());
        assert!(screen.the_rails_figure.is_none(), "an ended request is counted as running");
        assert_eq!(drawn.files.len(), 1);
        assert_eq!(drawn.files[0].state, "failed");
        let failure = drawn.files[0].failure.as_ref().expect("the failure shape");
        assert_eq!(failure.sentence.as_str().replace('\u{a0}', " "), words::DOWNLOAD_FAILED);
        let acts: Vec<&str> = failure.controls.iter().map(|control| control.act.as_str()).collect();
        // **`Download` P3's per-file *Try again*, first after the sentence**
        // — the whole-app failure shape's order. The 38's piece 9, item 17.
        assert_eq!(acts, vec!["try-again", "details", "clear-a-failure"]);
        assert_eq!(failure.controls[0].label.as_str().replace('\u{a0}', " "), words::TRY_AGAIN);
        assert_eq!(failure.controls[1].label.as_str(), words::DETAILS);
        assert_eq!(
            the_details_of(&failed.record.as_text(), Some("/Users/artist/Downloads/p13-cube.blend"))
                .map(|said| said.as_str().replace('\u{a0}', " ")),
            Some("File already exists. Not overwritten.".to_string())
        );
        assert_eq!(the_details_of(&failed.record.as_text(), None), None);
        // **Where this run holds nothing, *Details* is absent** rather than
        // opening an empty panel.
        let acts: Vec<&str> = requests[1].files[0]
            .failure
            .as_ref()
            .expect("the failure shape")
            .controls
            .iter()
            .map(|control| control.act.as_str())
            .collect();
        assert_eq!(acts, vec!["try-again", "clear-a-failure"]);
        // **Every file down still reads done.**
        assert_eq!(every_word(&requests[2].count), words::DONE);
        assert!(requests[2].files.is_empty());
    }

    /// **A manual download's names its check refused are said as what
    /// happened to them** — `Download` P8 (*"the file is marked cannot land …
    /// said in the request … outcome; the file stays on the server"*) and `Download` N7
    /// (*"out, reported with the refusal"*), never under P6's *left alone*
    /// words, which say something stood in the folder. The two transfer faults
    /// after the 38's pieces 7 and 9, fault 2.
    #[test]
    fn a_manual_downloads_refused_names_are_said_as_not_downloaded() {
        let issue = |kind| Identity::issue(kind).expect("randomness");
        let counted = |cannot_land: usize| crate::record::download_records::AManualRequestCounted {
            record: issue(IdentityKind::Operation),
            destination: b"/Users/artist/Downloads".to_vec(),
            files_in_the_whole: 3,
            files_done: 3 - cannot_land,
            bytes_in_the_whole: 30,
            bytes_done: 30,
            every_unit_has_ended: true,
            cannot_land,
            nothing_is_decided: false,
            opened_by_run: issue(IdentityKind::Run),
            failed_landings: Vec::new(),
            stopped_at_the_question: false,
            could_not_be_read: 0,
        };
        let two = counted(2);
        let one = counted(1);
        let screen = the_screen_from(
            &known(vec![
                a_manual_request_known("row", &two),
                a_manual_request_known("row", &one),
            ]),
            None,
        );
        let requests: Vec<_> = [two.record.as_text(), one.record.as_text()].into_iter().map(|id| screen.requests.iter().find(|r| r.id.as_str() == id).unwrap()).collect();
        for (drawn, count, said) in [
            (&requests[0], "2", "files not downloaded: names not valid on this computer."),
            (&requests[1], "1", "file not downloaded: name not valid on this computer."),
        ] {
            let lines: Vec<String> = drawn
                .lines
                .iter()
                .map(|line| every_word(&line.words).replace('\u{a0}', " "))
                .collect();
            assert!(lines.iter().all(|line| !line.contains("already in this folder")), "{lines:?}");
            let line = drawn
                .lines
                .iter()
                .find(|line| every_word(&line.words).replace('\u{a0}', " ") == said)
                .unwrap_or_else(|| panic!("no line says «{said}»: {lines:?}"));
            assert_eq!(line.words.values(), vec![count]);
            assert_eq!(line.tone, "amber");
            assert_eq!(line.link.as_ref().map(|link| link.act.as_str()), Some("which-ones-cannot-land"));
        }
    }

    /// **A leftover says which disk and never how much**, and offers Remove.
    ///
    /// **Step 10c's transfer wave piece 7**: *stopped when Smoozy closed* only
    /// where Smoozy closing ended it, and no *Remove* where only an owed
    /// cleanup is left — the files are still there and a *Remove* could do
    /// nothing more.
    #[test]
    fn a_leftover_names_its_disk_and_carries_no_size() {
        let mut up = a_request(TheKind::Upload, WhatItIsOf::AFolder { folder: "kitchen/".into() });
        up.stands = WhereTheRequestStands::LeftOver { removable: true, by_an_earlier_run: true };
        let mut down = a_request(TheKind::Download, WhatItIsOf::AJob { name: "Hallway".into(), job: "job-hallway".into() });
        down.id = "down".into();
        down.stands = WhereTheRequestStands::LeftOver { removable: true, by_an_earlier_run: false };
        let mut owed = a_request(TheKind::Upload, WhatItIsOf::AFolder { folder: "attic/".into() });
        owed.id = "owed".into();
        owed.stands = WhereTheRequestStands::LeftOver { removable: false, by_an_earlier_run: false };
        let screen = the_screen_from(&known(vec![up, down, owed]), None);
        let requests: Vec<_> = ["one", "down", "owed"].into_iter().map(|id| screen.requests.iter().find(|r| r.id.as_str() == id).unwrap()).collect();
        let upload_line = every_word(&requests[0].where_it_goes);
        assert!(upload_line.contains(words::STILL_USING_SPACE_ON_THE_SERVER));
        assert!(upload_line.contains(words::STOPPED_WHEN_SMOOZY_CLOSED));
        assert!(requests[0].where_it_goes.values().iter().all(|value| !value.contains("B")));
        assert_eq!(requests[0].control.as_ref().expect("remove").act.as_str(), "remove");
        assert!(requests[0].bar.is_none());
        assert!(requests[1].where_it_goes.values().contains(&"/Users/artist/Renders"));
        assert!(!every_word(&requests[1].where_it_goes).contains(words::STOPPED_WHEN_SMOOZY_CLOSED));
        assert!(every_word(&requests[2].where_it_goes).contains(words::STILL_USING_SPACE_ON_THE_SERVER));
        assert!(requests[2].control.is_none(), "nothing more a Remove could do");
        assert!(screen.the_rails_figure.is_none(), "a leftover is not running");
    }

    /// **The stop box names what is in flight, and on a waiting request names
    /// nothing in flight and says nothing is discarded.**
    #[test]
    fn the_stop_box_says_what_is_across_what_never_goes_and_what_is_in_flight() {
        let mut request = a_request(TheKind::Upload, WhatItIsOf::AFolder { folder: "kitchen/".into() });
        request.files_done = 104;
        request.files_in_the_whole = 312;
        request.files = vec![
            a_file("textures/wood_oak.png", WhatHappenedToTheFile::Moving),
            a_file("textures/bark_rough.png", WhatHappenedToTheFile::Moving),
            a_file("textures/floor_tile.png", WhatHappenedToTheFile::WaitingItsTurn),
        ];
        let TheStopAnswer::TheBox(live) = the_stop_box_for(&request) else {
            panic!("a box");
        };
        assert_eq!(live.bullets.len(), 4);
        assert_eq!(live.bullets[1].bold.values(), vec!["206"]);
        assert!(live.nothing_is_thrown_away.is_none());

        request.reconnecting = Some(None);
        let TheStopAnswer::TheBox(waiting) = the_stop_box_for(&request) else {
            panic!("a box");
        };
        assert_eq!(waiting.bullets.len(), 2);
        assert!(waiting.nothing_is_thrown_away.is_some());
    }

    /// **The Creator's note 16 on their second notes: from a queue entry, a
    /// fixed folder is one plain line and not a choice.** With the arrangement
    /// on, the box says *To /Volumes/Work/kitchen* and nothing about where it
    /// goes being fixed; with it off, the folder is a choice again and the
    /// sentence says so, but **never that the formats can be changed later from
    /// the queue** — this is the queue. Render Settings' own box keeps both.
    /// **It fails against the box as it stood**, which drew the one folder as a
    /// radio button under *Where they go is fixed once the job is submitted.
    /// The formats can be changed later from the queue.*
    #[test]
    fn from_a_queue_entry_a_fixed_folder_is_a_line_and_not_a_choice() {
        use smoozy_download::arrangement::AnArrangement;
        let job = Identity::issue(IdentityKind::Job).expect("random bytes");
        let row = Identity::issue(IdentityKind::LocalServer).expect("random bytes");
        let formats = vec![
            AFormatKnown { name: "OPEN_EXR_MULTILAYER".into(), where_in_the_job: "EXR/".into(), at_the_top: false, files: 2, chosen: true },
            AFormatKnown { name: "FFMPEG".into(), where_in_the_job: String::new(), at_the_top: true, files: 1, chosen: true },
        ];
        let on = AnArrangement::switched_on(job.clone(), row.clone(), b"/Volumes/Work/kitchen".to_vec(), TheNarrowing::Everything);
        let fixed = the_jobs_box_is_composed(&a_jobs_box(&row, formats.clone(), Some(on.clone())));
        let line = fixed.the_fixed_folder.as_ref().expect("the folder as a line");
        assert_eq!(line.values(), vec!["/Volumes/Work/kitchen"]);
        assert!(fixed.fixed.is_none(), "a sentence about where it goes, where it went already");
        assert_eq!(fixed.folders.len(), 1, "Done still hands back the one row");

        let mut off = on;
        off.switched_off();
        let open = the_jobs_box_is_composed(&a_jobs_box(&row, formats.clone(), Some(off)));
        assert!(open.the_fixed_folder.is_none());
        let said = serde_json::to_string(&open.fixed).expect("it serialises").replace('\u{a0}', " ");
        assert!(said.contains("fixed once the job is submitted"), "{said}");
        assert!(!said.contains("changed later from the queue"), "the queue's box says the queue is ahead: {said}");

        let at_render_settings = the_format_box_from(&a_jobs_box(&row, formats, None).knows, &TheProposal::TheSystemNamesNoFolder(String::new()));
        let said = serde_json::to_string(&at_render_settings.fixed).expect("it serialises").replace('\u{a0}', " ");
        assert!(said.contains("changed later from the queue"), "{said}");
        assert!(at_render_settings.the_fixed_folder.is_none());

        // **The words are the Design's and the drawing's.**
        let design = include_str!("../../../../Preparation/Design/Render Settings.md");
        assert!(design.contains("*To ~/Renders/kitchen*"));
        let drawing = include_str!("../../../../Preparation/Design/Mockup/Smoozy.html");
        assert!(drawing.contains("id=\"fmtqmodal\""));
    }

    /// **Note 16, the names: the box shows each format in Smoozy's words and
    /// hands back the identifier**, and says *top of the job's folder* in words
    /// where a format is written there. **It fails against the box as it
    /// stood**, which printed `OPEN_EXR_MULTILAYER`.
    #[test]
    fn the_box_names_formats_in_smoozys_words_and_hands_back_the_identifier() {
        let several = the_format_box_from(
            &WhatTheBoxKnows {
                formats: Some(vec![
                    AFormatKnown { name: "OPEN_EXR_MULTILAYER".into(), where_in_the_job: "EXR/".into(), at_the_top: false, files: 2, chosen: true },
                    AFormatKnown { name: "FFMPEG".into(), where_in_the_job: String::new(), at_the_top: true, files: 1, chosen: true },
                ]),
                files_in_the_job: 3,
                the_standing_folder: Some(b"/Users/artist/Renders".to_vec()),
                the_formats_chosen: None,
                the_folder_chosen_for_this_job: None,
            },
            &TheProposal::TheSystemNamesNoFolder(String::new()),
        );
        let TheFormatReading::SeveralFormats { formats, .. } = &several.reading else {
            panic!("several");
        };
        let drawn: Vec<(&str, &str, bool)> =
            formats.iter().map(|one| (one.name.as_str(), one.said.as_str(), one.the_top.is_some())).collect();
        assert_eq!(drawn, vec![("OPEN_EXR_MULTILAYER", "Multilayer EXR", false), ("FFMPEG", "Video", true)]);
        let one = the_format_box_from(
            &WhatTheBoxKnows {
                formats: Some(vec![AFormatKnown {
                    name: "OPEN_EXR_MULTILAYER".into(),
                    where_in_the_job: "EXR/".into(),
                    at_the_top: false,
                    files: 2,
                    chosen: true,
                }]),
                files_in_the_job: 2,
                the_standing_folder: None,
                the_formats_chosen: None,
                the_folder_chosen_for_this_job: None,
            },
            &TheProposal::TheSystemNamesNoFolder(String::new()),
        );
        let said = serde_json::to_string(&one.reading).expect("it serialises");
        assert!(said.contains("Multilayer EXR") && !said.contains("OPEN_EXR"), "{said}");
        // The page draws the word and keeps the identifier on the row.
        let page = the_page();
        assert!(page.contains("fmtName.textContent = format.said;"));
        assert!(page.contains("one.dataset.format = format.name;"));
    }

    // ── The Creator's second notes, piece 2b: the queue's box counts the job's own layers ──

    /// **A local record holding one server whose row records its machine
    /// root**, so a job on it reads the way the box's act reads it.
    fn a_record_with_a_rooted_server() -> (rusqlite::Connection, Identity) {
        use smoozy_servers::identity::{Host, KeyReference, Port, Username};
        let connection = crate::record::deletions::tests::a_record_store();
        let server = Identity::issue(IdentityKind::LocalServer).expect("random bytes");
        let mut row = smoozy_servers::store::ServerRow::saved(
            server.as_text(),
            Host::what_the_user_typed("127.0.0.1").expect("a host"),
            Port::what_the_user_typed(22).expect("a port"),
            Username::what_the_user_typed("root").expect("a username"),
            KeyReference { local_key_identifier: "key-1".into(), fingerprint: "SHA256:aaa".into() },
        );
        row.recorded_machine_root = Some(smoozy_servers::store::RecordedMachineRoot {
            path: "/root/smoozy".into(),
            the_package_areas_own_location: None,
            read_under: "inst-1".into(),
        });
        server_rows::add(&connection, &row).expect("the row");
        (connection, server)
    }

    /// **A job submitted the way Submit submits it**: the page's payload
    /// composed for the scene *Interior* with `pins`, and the whole record and
    /// that payload written by Submit's own writer, the `.blend` one folder
    /// below `Render/` as the Creator's was.
    fn a_job_submitted_with(
        connection: &rusqlite::Connection,
        server: &Identity,
        pins: &[(&str, bool)],
    ) -> Identity {
        use crate::record::job_records::TheSubmissionsWriter;
        use smoozy_queue_history::acting::submission::{AProvisionalRecord, WritesTheProvisionalRecord};
        use smoozy_render_settings::names::{ExactName, NamedAnswer, Provenance};
        use smoozy_render_settings::payload::{compose, Composition};
        use smoozy_render_settings::store::{Pin, SceneEntries};
        let mut record = crate::record::deletions::tests::a_record_in_history(server, "Hero");
        record.blend_path_on_the_machine = Some("/home/ubuntu/rendering/hero.blend".to_string());
        let named = |name: &str| NamedAnswer {
            name: ExactName::as_the_reading_produced_it(name),
            provenance: Provenance::AKeptSetup { the_old_job: "an earlier job".into() },
        };
        let answers = SceneEntries {
            pins: pins.iter().map(|(layer, on)| Pin { layer: named(layer), on: *on }).collect(),
            ..Default::default()
        };
        let payload = compose(Composition {
            version_pick: &smoozy_render_settings::file_half::BlenderIdentity {
                path: "/root/smoozy/blender/blender-4.5.12/blender".to_string(),
                version: smoozy_server_setup::blender::Version { major: 4, minor: 5, patch: 12 },
            },
            scene: Some(&named("Interior")),
            answers: &answers,
            standing: None,
            stale: false,
        });
        TheSubmissionsWriter::of(connection, record.clone(), payload)
            .write_it(&AProvisionalRecord {
                job: record.job.clone(),
                server: server.clone(),
                group: None,
                label: "Hero".to_string(),
            })
            .expect("the record and its kept setup are written");
        record.job
    }

    /// **The queue's box over one job, as its act composes it**, on piece 2's
    /// stand-in for the Creator's file: the job read out of the record the
    /// way the act reads it, and the box composed from it with no arrangement.
    fn the_queues_box_over(connection: &rusqlite::Connection, job: &Identity) -> (Vec<(String, usize)>, usize) {
        let is = what_the_job_is_in(connection, Identity::issue(IdentityKind::Run).expect("random bytes"), job)
            .expect("the job reads");
        let knows = what_the_jobs_box_knows(
            &is,
            None,
            Some(&crate::surface::render_settings::tests::the_creators_shape()),
            &crate::surface::render_settings::WhatTheRecordHolds::default(),
        );
        let formats = knows
            .formats
            .expect("formats")
            .into_iter()
            .map(|format| (format.name, format.files))
            .collect();
        (formats, knows.files_in_the_job)
    }

    /// **The Creator's second notes, piece 2b: the queue's box counts the
    /// layers the job was submitted with.** The Creator's shape submitted with
    /// *Box AO* and *Mattress AO* switched off writes 2 PNG and 5 files, and
    /// the box a queue entry opens says so — as Render Settings said before
    /// Submit, for the same job. **It fails against the box as it stood**,
    /// which composed with no pins and said PNG 4 and 7 files.
    #[test]
    fn the_queues_box_counts_the_layers_the_job_was_submitted_with() {
        let (connection, server) = a_record_with_a_rooted_server();
        let job = a_job_submitted_with(&connection, &server, &[("Box AO", false), ("Mattress AO", false)]);
        let (formats, files) = the_queues_box_over(&connection, &job);
        assert_eq!(
            formats,
            vec![("FFMPEG".to_string(), 1), ("PNG".to_string(), 2), ("OPEN_EXR_MULTILAYER".to_string(), 2)]
        );
        assert_eq!(files, 5);

        // **The same job's pins on Render Settings say the same**: one rule,
        // one count.
        let on_the_page = crate::surface::render_settings::what_the_format_box_knows(
            Some(&crate::surface::render_settings::tests::the_creators_shape()),
            &crate::surface::render_settings::TheSubmissionChoices::default(),
            &crate::surface::render_settings::WhatTheRecordHolds::default(),
            &crate::surface::render_settings::tests::the_creators_ground(true),
        );
        assert_eq!(on_the_page.files_in_the_job, files);

        // **A pin switched on counts too**: *Box AO* on, *Mattress AO* off — 3 PNG.
        let job = a_job_submitted_with(&connection, &server, &[("Box AO", true), ("Mattress AO", false)]);
        let (formats, files) = the_queues_box_over(&connection, &job);
        assert_eq!(formats[1], ("PNG".to_string(), 3));
        assert_eq!(files, 6);
    }

    /// **A job whose record kept no setup has no pins, and its box says what
    /// it said before** — the file's own four layers, PNG 4 and 7 files.
    /// Nothing is made up for it: not for a record written before record
    /// version 25, and not for a kept setup that will not read back.
    #[test]
    fn a_job_with_no_kept_setup_counts_the_files_own_layers() {
        let (connection, server) = a_record_with_a_rooted_server();
        let the_files_own = vec![
            ("FFMPEG".to_string(), 1),
            ("PNG".to_string(), 4),
            ("OPEN_EXR_MULTILAYER".to_string(), 2),
        ];

        let mut written_before = crate::record::deletions::tests::a_record_in_history(&server, "Hero");
        written_before.blend_path_on_the_machine = Some("/home/ubuntu/rendering/hero.blend".to_string());
        crate::record::job_records::JobRecords::over(&connection)
            .write_the_provisional_record(&written_before)
            .expect("written with no kept setup");
        assert_eq!(the_queues_box_over(&connection, &written_before.job), (the_files_own.clone(), 7));

        let unreadable = a_job_submitted_with(&connection, &server, &[("Box AO", false), ("Mattress AO", false)]);
        connection
            .execute("UPDATE job_record SET kept_setup = '{\"not\": \"a payload\"}' WHERE job = ?1", [unreadable.as_text()])
            .expect("the column is spoiled");
        assert_eq!(the_queues_box_over(&connection, &unreadable), (the_files_own, 7));
    }

    /// **The box's name column takes its name's width**, as its folder rows
    /// already do. **It fails against the sheet as it stood**: the drawing's
    /// 58-pixel column held `OPEN_EXR_MULTILAYER` — 159 pixels of it, measured
    /// in a browser — and it ran into the folder beside it.
    #[test]
    fn the_boxs_name_column_takes_its_names_width() {
        let sheet = include_str!("../../web/transfers.css");
        let rule = sheet
            .split(".transfers-formats .fmt {")
            .nth(1)
            .expect("the format rows' own columns")
            .split('}')
            .next()
            .expect("the rule closes");
        assert!(rule.contains("grid-template-columns: 14px minmax(58px, max-content) 1fr auto;"), "{rule}");
        assert!(the_page().contains("el(\"div\", \"fmts\", \"transfers-formats\")"));
    }

    /// **The fixed folder, drawn and pressed, under Node**: one plain line and
    /// no radio rows, no sentence under it, the names in Smoozy's words, and
    /// *Done* handing back the one row. **Skipped, and says so, where no Node
    /// is installed.**
    #[test]
    fn the_fixed_folder_is_drawn_as_a_line_and_done_hands_back_its_row() {
        use smoozy_download::arrangement::AnArrangement;
        let job = Identity::issue(IdentityKind::Job).expect("random bytes");
        let row = Identity::issue(IdentityKind::LocalServer).expect("random bytes");
        let formats = vec![
            AFormatKnown { name: "OPEN_EXR_MULTILAYER".into(), where_in_the_job: "EXR/".into(), at_the_top: false, files: 2, chosen: true },
            AFormatKnown { name: "FFMPEG".into(), where_in_the_job: String::new(), at_the_top: true, files: 1, chosen: true },
        ];
        let on = AnArrangement::switched_on(job, row.clone(), b"/Volumes/Work/kitchen".to_vec(), TheNarrowing::Everything);
        let the_box = TheFormatAnswer::TheBox(the_jobs_box_is_composed(&a_jobs_box(&row, formats, Some(on))));
        let answers = serde_json::json!({ "box": the_box }).to_string();

        let folder = std::env::temp_dir().join(format!("smoozy-the-fixed-folder-harness-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a scratch folder");
        let web = Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        for page in ["transfers.js", "the-value-drawn.js"] {
            std::fs::copy(web.join(page), folder.join(page)).expect("the page is copied");
        }
        for (name, text) in THE_QUESTIONS_STAND_INS {
            let text = if name == "harness.js" { THE_FIXED_FOLDER_HARNESS } else { text };
            std::fs::write(folder.join(name), text).expect("a stand-in is written");
        }
        std::fs::write(folder.join("answers.json"), answers).expect("the answers are written");
        let ran = std::process::Command::new("node").arg(folder.join("harness.js")).current_dir(&folder).output();
        let _ = std::fs::remove_dir_all(&folder);
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the fixed folder's page was not run");
            return;
        };
        assert!(ran.status.success(), "the page did not run: {}", String::from_utf8_lossy(&ran.stderr));
        let got: serde_json::Value =
            serde_json::from_str(String::from_utf8_lossy(&ran.stdout).trim()).expect("the harness prints one answer");
        assert_eq!(got["radios"], 0, "{got}");
        assert_eq!(got["fixedLine"], "To /Volumes/Work/kitchen", "{got}");
        assert_eq!(got["sentencesUnder"], serde_json::json!([]), "{got}");
        assert_eq!(got["names"], serde_json::json!(["Multilayer EXR", "Video"]), "{got}");
        // **As it travels**: the typography point binds the last two words.
        assert_eq!(got["where"], serde_json::json!(["EXR/", "top of the job’s\u{a0}folder"]), "{got}");
        assert_eq!(
            got["done"],
            serde_json::json!({ "formats": ["OPEN_EXR_MULTILAYER", "FFMPEG"], "folder": { "is": "this-one", "at": 0 } }),
            "{got}"
        );
    }

    /// The harness [`the_fixed_folder_is_drawn_as_a_line_and_done_hands_back_its_row`]
    /// runs, over the least of a document the download question's harness uses.
    const THE_FIXED_FOLDER_HARNESS: &str = r#"import { readFileSync } from "node:fs";
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
  const element = {
    tag, children: [], parentNode: null, dataset: {}, hidden: false, text: "", type: "", id: "", title: "",
    classList: {
      add: (...more) => more.forEach((one) => names.add(one)),
      remove: (...less) => less.forEach((one) => names.delete(one)),
      toggle: (one, on) => { if (on === undefined ? !names.has(one) : on) names.add(one); else names.delete(one); },
      contains: (one) => names.has(one),
    },
    set className(value) { names.clear(); String(value).split(" ").filter(Boolean).forEach((one) => names.add(one)); },
    get firstChild() { return element.children[0]; },
    get className() { return Array.from(names).join(" "); },
    set textContent(value) { element.children = []; element.text = value === undefined || value === null ? "" : String(value); },
    get textContent() { return element.text + element.children.map((one) => one.textContent || one.text || "").join(""); },
    appendChild(child) {
      if (child.parentNode && child.parentNode.removeChild) child.parentNode.removeChild(child);
      child.parentNode = element;
      element.children.push(child);
      return child;
    },
    removeChild(child) { element.children = element.children.filter((one) => one !== child); child.parentNode = null; return child; },
    replaceChildren(...fresh) { for (const one of element.children) one.parentNode = null; element.children = []; fresh.forEach((one) => element.appendChild(one)); },
    remove() { if (element.parentNode) element.parentNode.removeChild(element); },
    setAttribute() {},
    addEventListener(kind, handler) { const held = listeners.get(kind) || []; held.push(handler); listeners.set(kind, held); },
    click() { for (const handler of listeners.get("click") || []) handler({ target: element, preventDefault() {} }); },
    focus() {},
    contains(other) { let at = other; while (at) { if (at === element) return true; at = at.parentNode; } return false; },
    querySelector(selector) { return everyUnder(element, aTest(selector), [])[0] || null; },
    querySelectorAll(selector) { return everyUnder(element, aTest(selector), []); },
  };
  return element;
}
globalThis.screens = new Map();
const body = anElement("body");
globalThis.document = {
  createElement: anElement,
  createTextNode: (text) => ({ text, children: [] }),
  addEventListener() {},
  body,
};
const answers = JSON.parse(readFileSync("answers.json", "utf8"));
const sent = [];
globalThis.window = {
  __TAURI__: {
    core: {
      invoke(command, asked) {
        if (command === "the_format_box") return Promise.resolve(answers.box);
        if (command === "the_transfers_screen") return new Promise(() => {});
        sent.push([command, asked]);
        return Promise.resolve(null);
      },
    },
  },
};
const page = await import("./transfers.js");
await page.theFormatBoxIsOpenedFor("job-1");
const box = body.children.find((one) => one.classList && one.classList.contains("modal"));
const out = {};
out.radios = box.querySelectorAll("label.choice3").length;
out.fixedLine = box.querySelector("p.transfers-fixed-folder").textContent;
out.sentencesUnder = box
  .querySelectorAll("p.msub")
  .filter((one) => !one.classList.contains("transfers-fixed-folder") && !one.classList.contains("transfers-answer"))
  .slice(1)
  .map((one) => one.textContent);
out.names = box.querySelectorAll("span.fmn").map((one) => one.textContent);
out.where = box.querySelectorAll("span.fmw").map((one) => one.textContent);
const buttons = box.querySelector("div.mrow").querySelectorAll("button");
buttons[buttons.length - 1].click();
await new Promise((done) => setTimeout(done, 0));
const chosen = sent.find(([command]) => command === "the_formats_are_chosen_for_a_job");
out.done = chosen ? { formats: chosen[1].formats, folder: chosen[1].folder } : null;
console.log(JSON.stringify(out));
"#;

    /// **The three format boxes, and §8's proposal is offered as a proposal.**
    #[test]
    fn the_format_boxes_read_the_reading_and_offer_the_proposal() {
        let proposal = TheProposal::TheSystemNamesThisFolder(PathBuf::from("/Users/artist/Downloads"));
        let several = the_format_box_from(
            &WhatTheBoxKnows {
                formats: Some(vec![
                    AFormatKnown { name: "EXR".into(), where_in_the_job: "beauty/".into(), at_the_top: false, files: 480, chosen: true },
                    AFormatKnown { name: "JPEG".into(), where_in_the_job: "previews/".into(), at_the_top: false, files: 120, chosen: false },
                ]),
                files_in_the_job: 600,
                the_standing_folder: Some("/Users/artist/Renders/kitchen".into()),
                the_formats_chosen: None,
                the_folder_chosen_for_this_job: None,
            },
            &proposal,
        );
        let TheFormatReading::SeveralFormats { total, .. } = &several.reading else {
            panic!("several");
        };
        assert_eq!(total.values(), vec!["480", "600"]);
        assert_eq!(several.folders.len(), 2);
        assert_eq!(several.folders[1].folder.as_str(), "/Users/artist/Downloads");

        let unread = the_format_box_from(
            &WhatTheBoxKnows {
                formats: None,
                files_in_the_job: 0,
                the_standing_folder: None,
                the_formats_chosen: None,
                the_folder_chosen_for_this_job: None,
            },
            &TheProposal::TheSystemNamesNoFolder("nothing".into()),
        );
        assert!(matches!(unread.reading, TheFormatReading::NotRead { .. }));
        assert!(unread.folders.is_empty(), "Download H3: no folder nobody named is offered");
    }

    /// **Must-do 18: a folder keeps its exact name, and the page hands back
    /// which row it was rather than what the row read** — `Server protocol and
    /// transfers` P7's *text only at the point a name is shown*. Step 10a, the
    /// screens round's loose ends, L6.
    ///
    /// **It fails against the box as it stood**, where the offered list was
    /// built with `display().to_string()`, every row carried its own text on
    /// the page, and the application matched the answer against that text.
    #[test]
    fn a_folder_keeps_its_exact_name_and_the_page_hands_back_which_row_it_was() {
        // A name the system holds and no text can: a lone continuation byte.
        let odd = b"/Users/artist/Renders/\xffodd".to_vec();
        let knows = WhatTheBoxKnows {
            formats: None,
            files_in_the_job: 0,
            the_standing_folder: Some(odd.clone()),
            the_formats_chosen: None,
            the_folder_chosen_for_this_job: None,
        };
        let proposal = TheProposal::TheSystemNamesNoFolder("nowhere".to_string());
        let offered = the_folders_offered(&knows, &proposal);
        assert_eq!(
            offered,
            vec![(odd.clone(), false)],
            "the folder did not come back as the bytes the system holds"
        );
        // The box draws it, lossily and for showing only.
        let the_box = the_format_box_from(&knows, &proposal);
        assert_eq!(the_box.folders.len(), 1);
        assert!(the_box.folders[0].folder.as_str().contains("\u{fffd}"));

        // And the page hands back the row, never its text.
        let page = the_page();
        assert!(page.contains("is: \"this-one\", at: Number(picked.dataset.at)"));
        assert!(!page.contains("dataset.folder"), "the page still carries a folder's text back");
    }

    /// **The folder already chosen for a job comes first, each folder once, and
    /// only the proposal says *your download folder*.**
    #[test]
    fn a_folder_already_chosen_is_offered_first_and_once() {
        let proposal = TheProposal::TheSystemNamesThisFolder(PathBuf::from("/Users/artist/Downloads"));
        let knows = WhatTheBoxKnows {
            formats: None,
            files_in_the_job: 0,
            the_standing_folder: Some(b"/Users/artist/Renders".to_vec()),
            the_formats_chosen: None,
            the_folder_chosen_for_this_job: Some(b"/Users/artist/Downloads".to_vec()),
        };
        assert_eq!(
            the_folders_offered(&knows, &proposal),
            vec![
                (b"/Users/artist/Downloads".to_vec(), true),
                (b"/Users/artist/Renders".to_vec(), false)
            ]
        );
        let formats = vec![
            AFormatKnown { name: "EXR".into(), where_in_the_job: String::new(), at_the_top: false, files: 480, chosen: true },
            AFormatKnown { name: "PNG".into(), where_in_the_job: String::new(), at_the_top: false, files: 120, chosen: false },
        ];
        assert_eq!(the_files_the_choice_brings(&formats), 480);
    }

    /// **Nothing here spends the diary capture** — see the head's
    /// `WhoseSentenceItIs`.
    /// **The readings put in by hand for the photographs are gone**, and this
    /// fails if they return.
    /// **The way in from the three screens that start a transfer goes here
    /// first and then asks** (a *…to* act goes when its folder is chosen —
    /// piece Q, [`tests::a_cancelled_folder_panel_stays_where_it_was_pressed`]),
    /// every act it carries is in the closed set, and
    /// each one is refused in one honest sentence while nothing connects.
    #[test]
    fn a_transfer_asked_for_elsewhere_comes_here_and_is_refused_honestly() {
        let page = the_page();
        let way_in = page
            .split("export function aTransferIsAskedFor(act, whose, picked) {")
            .nth(1)
            .expect("the page exports the one way in")
            .split("\n}")
            .next()
            .expect("the way in closes");
        let going = way_in
            .find("theCurrentPlaceIs(TRANSFERS)")
            .expect("the way in goes to Transfers");
        let asking = way_in
            .find("theActIsTaken(act")
            .expect("the way in asks the application");
        assert!(
            going < asking,
            "the user is sent to Transfers before the answer can arrive"
        );

        // **The act that reaches this arm from nowhere** says so rather than
        // going quiet; the dependency check's upload is taken on the check
        // and only shows this screen (step 10c's dependency wave piece 6).
        assert!(what_an_act_answers(&WhatIsKnown::default(), the_act_named("upload-the-found"), "whose").is_none());
        for act in ["upload"] {
            let parsed = the_act_named(act);
            let said = what_an_act_answers(&WhatIsKnown::default(), parsed, "whose");
            // Compared through the typography point, which binds the sentence's
            // last words together and so never returns the bare constant.
            assert_eq!(
                said.as_ref().map(|one| one.as_str().to_string()),
                Some(Prose::new(words::NOT_FINISHED).as_str().to_string()),
                "{act} must say it cannot happen rather than go quiet"
            );
        }
    }

    /// **Every download act takes its own command, and none of the four is
    /// answered in prose here.**
    ///
    /// **This is what makes the silent arm unreachable rather than remembered.**
    /// [`what_an_act_answers`] answers `None` for the four, which on the
    /// synchronous route would draw nothing at all; the page never takes that
    /// route for them, and this test is the whole of why that is safe.
    #[test]
    fn every_download_act_takes_its_own_command() {
        let page = the_page();
        let way_in = page
            .split("export function aTransferIsAskedFor(act, whose, picked) {")
            .nth(1)
            .expect("the page exports the one way in")
            .split("\n}")
            .next()
            .expect("the way in closes");
        for named in [
            "a_whole_job_download_is_asked_for",
            "a_file_download_is_asked_for",
            "DOWNLOAD_ALL",
            "DOWNLOAD_ALL_TO",
            "DOWNLOAD",
            "DOWNLOAD_TO",
        ] {
            assert!(way_in.contains(named), "the way in does not route {named}");
        }
        for act in ["download-all", "download-all-to", "download", "download-to"] {
            let parsed = the_act_named(act);
            assert!(
                what_an_act_answers(&WhatIsKnown::default(), parsed, "whose").is_none(),
                "{act} has its own command and must say nothing here"
            );
        }
        // **The four strings the two pages send are the four the closed set
        // parses**, and nothing holds the two sides together but this.
        let queue = include_str!("../../web/the-queue.js");
        let files = include_str!("../../web/files.js");
        assert!(queue.contains("const DOWNLOAD_ALL = \"download-all\";"));
        assert!(queue.contains("const DOWNLOAD_ALL_TO = \"download-all-to\";"));
        assert!(files
            .contains("const THE_ACTS_THAT_START_A_TRANSFER = [\"download\", \"download-to\"];"));
    }

    /// **The two screens a transfer contends with read the record off the
    /// thread that draws** — `Desktop app and delivery` P4 and P5, step 10c's
    /// transfer wave piece 1. A plain command runs on the thread that draws,
    /// so a Transfers or a Files reading that waited for the record's lock
    /// while a transfer held it was the window frozen for that long.
    ///
    /// **Read off the two files' own text**, because running either command
    /// needs a running shell to say which thread it landed on.
    #[test]
    fn the_two_screens_a_transfer_contends_with_are_asked_off_the_thread_that_draws() {
        let transfers = the_working_half();
        let files = include_str!("files.rs");
        assert!(
            transfers.contains("pub async fn the_transfers_screen<"),
            "the Transfers screen is asked on the thread that draws"
        );
        assert!(
            files.contains("pub async fn the_files_tab<"),
            "the Files tab is asked on the thread that draws"
        );
    }

    /// **The folder panel opens only after everything that could refuse the
    /// request has passed** — piece 9's ruling: a pick that is then refused
    /// should not have been offered.
    ///
    /// **It is read off this file's own text**, because the ordering is the
    /// whole of the rule and a test that ran the command would need a running
    /// shell to put a panel up.
    #[test]
    fn the_folder_panel_opens_after_the_refusals_and_before_the_request() {
        let working = the_working_half();
        for command in [
            "pub async fn a_whole_job_download_is_asked_for",
            "pub async fn a_file_download_is_asked_for",
        ] {
            let body = working
                .split(command)
                .nth(1)
                .unwrap_or_else(|| panic!("{command} is in this file"))
                .split("\n}\n")
                .next()
                .expect("the command closes");
            let connected = body
                .find("the_live_point_of")
                .unwrap_or_else(|| panic!("{command} asks whether the server is connected"));
            let panel = body
                .find("the_destination_chosen")
                .unwrap_or_else(|| panic!("{command} settles the destination"));
            let composed = body
                .find("the_request_is_put_to_the_user")
                .unwrap_or_else(|| panic!("{command} composes the request"));
            assert!(
                connected < panel,
                "{command} offers a pick it may then refuse"
            );
            assert!(
                panel < composed,
                "{command} composes the request before the pick"
            );
        }
        // **The panel itself is `crate::picker`'s and there is no second route
        // to one** (K11), and it is put up off the runtime's own threads.
        assert!(working.contains("tauri::async_runtime::spawn_blocking"));
        assert!(working.contains("crate::picker::ask("));
    }

    /// **`Download` J9's reading over a browser request, which may reach files
    /// of several jobs and files of no job at all.**
    ///
    /// **The folder test is the folder and a separator.** Without the
    /// separator, a job whose folder is `/jobs/abc` would answer for every file
    /// of `/jobs/abcdef` — and the mark it lends is *Smoozy cannot vouch for
    /// this*, which would then be said about another job's frames.
    #[test]
    fn a_browser_requests_vouching_reads_the_right_jobs_folder() {
        use smoozy_download::acting::enumerating::ReadsTheLocalJobRecords;
        use smoozy_queue_history::ending::Ending;
        let a_job = |folder: &str, ending: Option<Ending>| AJobOnThisServer {
            job: Identity::issue(IdentityKind::Job).expect("random bytes"),
            lands_under: None,
            label: None,
            output_folder: folder.as_bytes().to_vec(),
            ending,
            record_directory: Vec::new(),
            attempts: 0,
        };
        let reading = TheJobsOnThisServer {
            smoozys_own_copies: Vec::new(),
            jobs: vec![
                a_job("/jobs/abc/outputs", Some(Ending::Failed)),
                a_job("/jobs/abcdef/outputs", None),
                a_job("/jobs/done/outputs", Some(Ending::Finished)),
            ],
        };
        let says = |path: &str| reading.what_the_local_record_says(path.as_bytes());

        // Inside, and it is the right job's ending.
        let inside = says("/jobs/abc/outputs/frame_0001.png");
        assert!(inside.the_file_is_inside_the_jobs_output_folder);
        assert_eq!(inside.the_recorded_ending, Some(Ending::Failed));
        // **The longer name is a different job**, not a file of the shorter.
        let neighbour = says("/jobs/abcdef/outputs/frame_0001.png");
        assert!(neighbour.the_file_is_inside_the_jobs_output_folder);
        assert_eq!(neighbour.the_recorded_ending, None);
        // The folder itself is not a file inside itself.
        assert!(!says("/jobs/abc/outputs").the_file_is_inside_the_jobs_output_folder);
        // Outside every job, Smoozy has no standing to judge.
        let outside = says("/home/artist/kitchen.blend");
        assert!(!outside.the_file_is_inside_the_jobs_output_folder);
        assert_eq!(outside.the_recorded_ending, None);
        // **`Rendering` K18's reckoning is never in the local job record**, so
        // the absence is carried as an absence (`Download` J9b).
        assert!(inside.the_completeness_statement.is_none());
        assert!(reading
            .the_files_frame(b"/jobs/abc/outputs/frame_0001.png")
            .is_none());
    }

    /// **Files' own *Download* of a job folder leaves Smoozy's copies on the
    /// machine** — after 0.1.8, the checks' leftovers, item 4, and `Download`
    /// N5a as written 2026-10-02. The request reads each reached job's
    /// placed-copy list and the reading after the render: a copy nothing
    /// touched is not enumerated, one the render wrote over still is, and a
    /// job the request does not reach is not read at all.
    #[test]
    fn files_own_download_of_a_job_folder_leaves_smoozys_copies_behind() {
        use smoozy_dependencies::acting::placing::{
            the_placed_copies_after_the_render_file_name, the_placed_copies_file_name,
            APlacedFile, ThePlacedCopies,
        };
        use smoozy_download::acting::enumerating::ReadsTheLocalJobRecords;
        use smoozy_server_protocol::subsystem::{StatusCode, SubsystemFailure};

        struct ARecordDirectory {
            files: std::collections::BTreeMap<Vec<u8>, Vec<u8>>,
            asked: Vec<Vec<u8>>,
        }
        impl smoozy_download::acting::record_directory::ReadsOneFileOnTheMachine for ARecordDirectory {
            async fn read_one_whole_file(&mut self, path: &[u8]) -> Result<Vec<u8>, SubsystemFailure> {
                self.asked.push(path.to_vec());
                self.files
                    .get(path)
                    .cloned()
                    .ok_or(SubsystemFailure::TheServerRefused(StatusCode::NoSuchFile))
            }
        }
        let a_file = |path: &str, size: u64| APlacedFile {
            job_folder_path: smoozy_dependencies::bytes::MachineLocation::composed(
                path.as_bytes().to_vec(),
            ),
            size,
            inode: "7".into(),
            modification_time: 100,
            change_time: 100,
        };
        let made = ThePlacedCopies {
            files: vec![
                a_file("/jobs/abc/outputs/pointcache/c_000001.bphys", 9),
                a_file("/jobs/abc/outputs/pointcache/c_000002.bphys", 9),
            ],
        };
        // **The render wrote over the second.**
        let mut after = made.clone();
        after.files[1].size = 12;
        let mut files = std::collections::BTreeMap::new();
        files.insert(
            format!("/jobs/abc/record/{}", the_placed_copies_file_name(1)).into_bytes(),
            made.as_written(),
        );
        files.insert(
            format!("/jobs/abc/record/{}", the_placed_copies_after_the_render_file_name(1))
                .into_bytes(),
            after.as_written(),
        );
        let a_job = |at: &str| AJobOnThisServer {
            job: Identity::issue(IdentityKind::Job).expect("random bytes"),
            lands_under: None,
            label: None,
            output_folder: format!("/jobs/{at}/outputs").into_bytes(),
            ending: None,
            record_directory: format!("/jobs/{at}/record").into_bytes(),
            attempts: 1,
        };
        let mut reader = ARecordDirectory {
            files,
            asked: Vec::new(),
        };
        let reading = tokio::runtime::Builder::new_current_thread()
            .build()
            .expect("a runtime")
            .block_on(the_jobs_a_browser_request_reaches(
                &mut reader,
                vec![a_job("abc"), a_job("other")],
                &[b"/jobs/abc/outputs".to_vec()],
            ));
        assert!(reading.is_a_copy_smoozy_placed(b"/jobs/abc/outputs/pointcache/c_000001.bphys"));
        assert!(
            !reading.is_a_copy_smoozy_placed(b"/jobs/abc/outputs/pointcache/c_000002.bphys"),
            "a copy the render wrote over comes home"
        );
        assert!(!reading.is_a_copy_smoozy_placed(b"/jobs/abc/outputs/f_0001.png"));
        assert!(
            reader.asked.iter().all(|path| path.starts_with(b"/jobs/abc/record/")),
            "a job the request does not reach was read"
        );

        // **A folder picked above the job reaches it, and one inside it does
        // too**; a neighbour with a longer name does not.
        for picked in [&b"/jobs"[..], b"/jobs/abc/outputs/pointcache", b"/jobs/abc/outputs/"] {
            let mut again = ARecordDirectory {
                files: reader.files.clone(),
                asked: Vec::new(),
            };
            let reading = tokio::runtime::Builder::new_current_thread()
                .build()
                .expect("a runtime")
                .block_on(the_jobs_a_browser_request_reaches(
                    &mut again,
                    vec![a_job("abc")],
                    &[picked.to_vec()],
                ));
            assert!(
                reading.is_a_copy_smoozy_placed(b"/jobs/abc/outputs/pointcache/c_000001.bphys"),
                "{}",
                String::from_utf8_lossy(picked)
            );
        }
        let mut again = ARecordDirectory {
            files: reader.files.clone(),
            asked: Vec::new(),
        };
        tokio::runtime::Builder::new_current_thread()
            .build()
            .expect("a runtime")
            .block_on(the_jobs_a_browser_request_reaches(
                &mut again,
                vec![a_job("abc")],
                &[b"/jobs/abc/outputsX".to_vec()],
            ));
        assert!(again.asked.is_empty(), "a neighbour's pick read the job");
    }

    /// **`Download` M9 and P10: a job's own output folder picked in the browser
    /// is still a whole job**, and it is the picked thing that is compared and
    /// nothing else.
    #[test]
    fn a_jobs_own_output_folder_picked_in_the_browser_is_the_job() {
        use smoozy_download::acting::enumerating::ReadsTheLocalJobRecords;
        let job = Identity::issue(IdentityKind::Job).expect("random bytes");
        let reading = TheJobsOnThisServer {
            jobs: vec![AJobOnThisServer {
                job: job.clone(),
                lands_under: None,
                label: Some("Kitchen — final".into()),
                output_folder: b"/jobs/abc/outputs".to_vec(),
                ending: None,
                record_directory: Vec::new(),
                attempts: 0,
            }],
            smoozys_own_copies: Vec::new(),
        };
        let found = reading
            .the_job_whose_output_folder_this_is(b"/jobs/abc/outputs")
            .expect("the picked folder is the job's own");
        assert_eq!(found.job.as_text(), job.as_text());
        assert_eq!(found.label.as_deref(), Some("Kitchen — final"));
        // A trailing separator is the same folder.
        assert!(reading
            .the_job_whose_output_folder_this_is(b"/jobs/abc/outputs/")
            .is_some());
        // **A folder below it is not the job's own folder**, which is H5a's
        // own arithmetic: only the first puts the job's folder on the disk.
        assert!(reading
            .the_job_whose_output_folder_this_is(b"/jobs/abc/outputs/shot")
            .is_none());
        assert!(reading
            .the_job_whose_output_folder_this_is(b"/jobs")
            .is_none());
    }

    /// **No sentence in this file says nothing moves a transfer yet where one
    /// does.** The four downloads run, so every refusal on their route names
    /// what did not happen instead.
    #[test]
    fn a_downloads_refusals_name_what_did_not_happen() {
        use crate::downloading::{WhereItLands, WhyTheDownloadDidNotStart as Why};
        for (why, said) in [
            (Why::ThereIsNothingToFetch, words::NOTHING_IS_SELECTED),
            (
                Why::ThereIsNowhereToPutIt(WhereItLands::TheSettingsWouldNotRead),
                words::THERE_IS_NO_DOWNLOAD_FOLDER,
            ),
            (
                Why::NothingPickedCouldBeRead,
                words::NOTHING_PICKED_COULD_BE_READ,
            ),
        ] {
            assert_eq!(why_it_did_not_start(&why), said);
            assert_ne!(why_it_did_not_start(&why), words::NOT_FINISHED);
        }
    }

    /// **No command asks the shell for a queue it does not manage.** `main.rs`
    /// manages the one queue shared, so a command asking for the plain type is
    /// refused before it runs and the page hears nothing. This screen's act
    /// command did exactly that, and no test could see it.
    #[test]
    fn no_command_asks_for_a_queue_the_shell_does_not_manage() {
        let main = include_str!("../main.rs");
        assert!(main.contains("let queue = Arc::new(events::TheOneQueue::new());"));
        assert!(main.contains("app.manage(queue.clone());"));
        let asks_for_the_plain_one = ["State<'_, ", "TheOneQueue>"].concat();
        for (name, source) in [
            ("transfers.rs", include_str!("transfers.rs")),
            ("files.rs", include_str!("files.rs")),
            ("the_queue.rs", include_str!("the_queue.rs")),
            ("the_history.rs", include_str!("the_history.rs")),
            ("the_dependency_check.rs", include_str!("the_dependency_check.rs")),
        ] {
            assert!(
                !source.contains(&asks_for_the_plain_one),
                "{name} asks the shell for the plain queue, which it does not manage"
            );
        }
    }

    /// **The proposed folder's words wrap inside their row.** Piece 12's first
    /// rule lost to the drawing's `.fmt.choice3 .fmn { white-space: nowrap }`,
    /// which is more specific, and the path ran past the row's edge.
    #[test]
    fn the_folder_rows_words_wrap_inside_the_row() {
        let sheet = include_str!("../../web/transfers.css");
        let rule = sheet
            .split(".transfers-folders .fmt.choice3 .fmn {")
            .nth(1)
            .expect("the words' rule is as specific as the drawing's")
            .split('}')
            .next()
            .expect("the rule closes");
        assert!(rule.contains("white-space: normal;"));
        assert!(rule.contains("min-width: 0;"));
        let row = sheet
            .split(".transfers-folders .fmt.choice3 {")
            .nth(1)
            .expect("the row's own columns")
            .split('}')
            .next()
            .expect("the rule closes");
        assert!(
            row.contains("grid-template-columns: 14px minmax(0, 1fr);"),
            "the words take the rest of the row and no more"
        );
        assert!(the_page().contains("el(\"label\", \"fmt\", \"choice3\")"), "the rows are still the drawing's choice rows");
    }

    #[test]
    fn nothing_seeded_is_left() {
        let working = the_working_half();
        for named in [["SMOOZY_PIECE", "12_SEED"].concat(), ["mod ", "seeded"].concat(), ["seeded", "_box"].concat()] {
            assert!(!working.contains(&named), "the seeding came back: {named}");
        }
        assert!(!the_page().contains(&["seeded", "_box"].concat()));
    }

    #[test]
    fn nothing_here_raises_a_failure_record() {
        let working: String = the_working_half()
            .lines()
            .filter(|line| !line.trim_start().starts_with("//"))
            .collect::<Vec<_>>()
            .join("\n");
        for named in ["WhoseSentenceItIs", "failure_records", "TheFailureHandover"] {
            assert!(!working.contains(named), "this file names {named} in its code");
        }
    }

    /// **Must-do 4: Transfers shows the caps Settings saved**, in Settings' own
    /// figures, and *no cap set* only for a direction with none. Before the fix
    /// this screen said *no cap set* whatever the record held.
    #[test]
    fn transfers_shows_the_caps_settings_saved() {
        let folder = std::env::temp_dir().join(format!("smoozy-transfers-caps-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a folder");
        let record = crate::record::LocalRecord::open_at(&folder.join("local-record.sqlite3")).expect("a record");
        let nothing = the_screen_from(&what_is_known_from(Some(&record), &HashMap::new(), None, None), None);
        assert_eq!(every_word(&nothing.caps), words::NO_CAP_SET);

        crate::record::settings::a_cap_was_set(
            record.connection(),
            smoozy_server_protocol::transfers::Direction::Down,
            TheCap::Of(crate::scheduling::BytesPerSecond(12_500_000)),
        )
        .expect("the cap is saved");
        let screen = the_screen_from(&what_is_known_from(Some(&record), &HashMap::new(), None, None), None);
        assert_eq!(screen.caps.values(), vec!["12.5"], "the saved cap is not drawn");
        let said = every_word(&screen.caps);
        assert!(said.contains(words::NO_UPLOAD_CAP_SET), "{said}");
        assert!(!said.contains(words::NO_DOWNLOAD_CAP_SET), "{said}");
        assert!(said.contains(crate::surface::settings::words::MB_PER_SECOND));
        drop(record);
        let _ = std::fs::remove_dir_all(&folder);

        // **A settings read that failed is said**, never drawn as no cap.
        let unread = the_screen_from(&WhatIsKnown { the_caps: None, ..known(Vec::new()) }, None);
        assert_eq!(every_word(&unread.caps), crate::surface::settings::words::THE_SETTINGS_WOULD_NOT_READ);
        assert!(the_working_half().contains("fn the_caps_line("));
        assert!(!the_working_half().contains("Nothing in this application stores a cap yet"));
        assert!(the_page().contains("saidInto(screen.caps, said.caps)"));
    }

    /// **Must-do 2 of step 10b round 1 of *jobs, history and downloads*: the
    /// live speed goes through the one conversion, and a moving channel never
    /// reads *0.0 MB/s*.**
    ///
    /// **This failed before the fix**: the floor, 16,384 bytes a second, and
    /// 10,000 both read *0.0 MB/s*, and the figure came from a division this
    /// file did itself.
    #[test]
    fn a_moving_channel_never_reads_nought_and_the_conversion_is_the_one() {
        let mb = crate::surface::settings::words::MB_PER_SECOND;
        for (bytes, reads) in [
            (16_384, "<0.1"),
            (10_000, "<0.1"),
            (99_999, "<0.1"),
            (100_000, "0.1"),
            (4_149_999, "4.1"),
            (4_150_000, "4.2"),
            (12_400_000, "12.4"),
            (9_960_000, "10.0"),
            (0, "0.0"),
        ] {
            assert_eq!(a_live_rate_reads(BytesPerSecond(bytes)), format!("{reads}\u{a0}{mb}"), "{bytes}");
        }
        let body = the_working_half()
            .split("fn a_live_rate_reads")
            .nth(1)
            .expect("the reading is in the working half")
            .split("\n}")
            .next()
            .expect("it has a body");
        assert!(body.contains("megabytes_per_second("), "the one conversion is not used");
        assert!(!body.contains("100_000") && !body.contains("1_000_000"), "a second conversion");
    }

    /// **Must-do 14: servers that would not read are said**, in place of the
    /// list and the filter, and never as *nothing is uploading or downloading*.
    #[test]
    fn servers_that_would_not_read_are_said_and_not_drawn_empty() {
        let screen = the_screen_from(&WhatIsKnown::nothing_would_read(), None);
        assert!(screen.nothing_here.is_none(), "an unread screen said nothing is moving");
        assert_eq!(
            screen.could_not_read.as_ref().map(|said| said.as_str().to_string()),
            Some(Prose::new(words::THE_SERVERS_WOULD_NOT_READ).as_str().to_string())
        );
        let from_nothing = the_screen_from(&what_is_known_from(None, &HashMap::new(), None, None), None);
        assert!(from_nothing.could_not_read.is_some());
        // **One wording for one fact, and now one constant**: this screen's
        // sentence *is* the Servers screen's (L6). The two cannot drift.
        assert_eq!(
            words::THE_SERVERS_WOULD_NOT_READ,
            crate::surface::servers::words::THE_LIST_COULD_NOT_BE_READ
        );
        assert!(
            !the_working_half().contains("\"Could not read your servers.\""),
            "this file writes the sentence out again instead of taking it from \
             its one home"
        );
        let page = the_page();
        assert!(page.contains("said.could_not_read"));
        assert!(page.contains("screen.filter.hidden = Boolean(said.could_not_read)"));
    }

    /// **Must-do 17: a running dependency upload offers Stop**, and the way
    /// back to the check only once there is nothing left to stop.
    #[test]
    fn a_running_dependency_upload_offers_stop() {
        let running = a_request(TheKind::Dependencies, WhatItIsOf::ABlend { file: "kitchen.blend".into() });
        let screen = the_screen_from(&known(vec![running.clone()]), None);
        assert_eq!(screen.requests[0].control.as_ref().expect("a control").act.as_str(), "stop");
        let mut done = running;
        done.stands = WhereTheRequestStands::Finished { at: "04:19".into() };
        let screen = the_screen_from(&known(vec![done]), None);
        assert_eq!(screen.requests[0].control.as_ref().expect("a control").act.as_str(), "open-the-check");
    }

    /// **Must-do 1: *Open the job* goes to History and chooses that job by its
    /// identity.**
    #[test]
    fn open_the_job_hands_history_the_jobs_identity() {
        let mut auto = a_request(TheKind::AutoDownload, WhatItIsOf::AJob { name: "Turntable".into(), job: "job-7f3a".into() });
        auto.the_job_ended = true;
        let screen = the_screen_from(&known(vec![auto]), None);
        assert_eq!(screen.requests[0].job.as_ref().map(Verbatim::as_str), Some("job-7f3a"));
        let page = the_page();
        assert!(page.contains("import { theHistoryEntryIsOpened } from \"./the-history.js\";"));
        assert!(page.contains("if (act === \"open-the-job\") return theHistoryEntryIsOpened(request.job || \"\");"));
    }

    /// **Must-do 12: one of anything is singular.** Every place this file joins
    /// a count to a counted word picks the word by the count.
    #[test]
    fn one_file_is_a_file_and_one_minute_is_a_minute() {
        let mut one = a_request(TheKind::Upload, WhatItIsOf::AFolder { folder: "kitchen/".into() });
        one.files_in_the_whole = 1;
        one.files_done = 1;
        one.seconds_left = Some(30);
        one.bytes = Some((1, 2));
        let screen = the_screen_from(&known(vec![one.clone()]), None);
        assert!(every_word(&screen.requests[0].name).contains(words::FILE_INTO));
        assert!(every_word(&screen.requests[0].where_it_goes).contains(words::MINUTE_LEFT));
        assert!(!every_word(&screen.requests[0].where_it_goes).contains(words::MINUTES_LEFT));
        let TheStopAnswer::TheBox(the_box) = the_stop_box_for(&one) else {
            panic!("a box");
        };
        assert!(every_word(&the_box.bullets[0].bold).contains(words::FILE_IS_ALREADY_THERE));

        let mut down = a_request(TheKind::AutoDownload, WhatItIsOf::AJob { name: "x".into(), job: "j".into() });
        down.the_job_ended = true;
        down.left_alone = 1;
        down.may_be_incomplete = 1;
        down.split_between = Some((1, "/a".into(), 1, "/b".into()));
        down.stands = WhereTheRequestStands::Finished { at: "04:19".into() };
        let lines: Vec<String> = the_screen_from(&known(vec![down]), None).requests[0]
            .lines
            .iter()
            .map(|line| every_word(&line.words))
            .collect();
        let all = lines.join(" | ");
        for singular in [words::FILE_WAS_ALREADY_HERE, words::MAY_BE_INCOMPLETE_ONE, words::FILE_IS_IN, words::IS_IN] {
            assert!(all.contains(singular), "{singular:?} missing from {all}");
        }
        for plural in [words::FILES_WERE_ALREADY_HERE, words::MAY_BE_INCOMPLETE, words::FILES_ARE_IN] {
            assert!(!all.contains(plural), "{plural:?} drawn for one: {all}");
        }
        let one_format = the_format_box_from(
            &WhatTheBoxKnows {
                formats: Some(vec![AFormatKnown { name: "EXR".into(), where_in_the_job: String::new(), at_the_top: false, files: 1, chosen: true }]),
                files_in_the_job: 1,
                the_standing_folder: None,
                the_formats_chosen: None,
                the_folder_chosen_for_this_job: None,
            },
            &TheProposal::TheSystemNamesNoFolder("nothing".into()),
        );
        let TheFormatReading::OneFormat { said } = &one_format.reading else {
            panic!("one format");
        };
        assert!(every_word(said).contains(words::FILE_IS_DOWNLOADED));
    }

    /// **Must-do 18, the reveal's half: the stored folder is opened as it was
    /// stored**, with no text conversion between — a name that is not valid
    /// text survives the trip.
    #[test]
    fn the_stored_folder_is_opened_without_passing_through_text() {
        #[cfg(unix)]
        {
            use std::os::unix::ffi::OsStrExt;
            let odd = std::ffi::OsStr::from_bytes(b"/tmp/renders-\xff");
            let stored = odd.as_encoded_bytes().to_vec();
            assert_eq!(the_stored_folder(&stored).as_os_str(), odd);
        }
        let plain = PathBuf::from("/Users/artist/Renders");
        assert_eq!(the_stored_folder(plain.as_os_str().as_encoded_bytes()), plain);
        assert!(!the_working_half().contains("String::from_utf8_lossy(folder)"));
    }

    /// **A record with one saved server, and a manual download written down
    /// for it the way `a_download_is_prepared` writes it** — its enumeration
    /// and nothing else: no disposition on any entry, no collision answer.
    /// Step 10c's eleventh-pass fix piece 1.
    fn a_download_written_and_not_yet_decided(
        name: &str,
        run: &Identity,
    ) -> (PathBuf, crate::record::LocalRecord, smoozy_download::request::AManualRequest) {
        use smoozy_download::request::{AManualRequest, EnumerationEntry, KeepsTheRequest};
        use smoozy_download::vouching::AVouchingMark;
        use smoozy_servers::identity::{Host, KeyReference, Port, Username};
        use smoozy_servers::store::ServerRow;
        let folder = crate::scratch_folders::a_folder(format!("smoozy-transfers-{name}-{}", std::process::id()));
        let record = crate::record::LocalRecord::open_at(&folder.join("local-record.sqlite3")).expect("a record");
        let row = ServerRow::saved(
            Identity::issue(IdentityKind::LocalServer).expect("randomness").as_text(),
            Host::what_the_user_typed("192.0.2.40").expect("a host"),
            Port::what_the_user_typed(22).expect("a port"),
            Username::what_the_user_typed("root").expect("a username"),
            KeyReference { local_key_identifier: "key-1".into(), fingerprint: "SHA256:aaa".into() },
        );
        server_rows::add(record.connection(), &row).expect("the row is added");
        server_rows::write(record.connection(), &row).expect("the row is written");
        let server = Identity::read_back(IdentityKind::LocalServer, &row.local_server_identifier).expect("the row");
        let request = AManualRequest::confirmed(
            Identity::issue(IdentityKind::Operation).expect("randomness"),
            server,
            None,
            b"/Users/artist/Downloads".to_vec(),
            vec![EnumerationEntry::enumerated(
                b"/root/renders/p12h11/frame_0001.png".to_vec(),
                b"/Users/artist/Downloads/frame_0001.png".to_vec(),
                4_096,
                AVouchingMark::NoClaimIsMade,
            )],
            run.clone(),
        );
        DownloadRequests::over(record.connection()).write_it_down(&request).expect("it is written down");
        (folder, record, request)
    }

    /// **Step 10c's eleventh-pass fix piece 1, as the tester met it: a download
    /// that has not started never reads *done*.** `Download` M1, M6 and M16;
    /// `Design/Transfers § A request shows the whole and the files under it`;
    /// Rule 8.
    ///
    /// **The eleventh pass on Hyperstack**: a *Download* from Files was written
    /// down, then waited inside its first read of the Downloads folder on the
    /// system's own permission box, which the tester could not see — and for
    /// the whole of that wait the row read *1 file into … · done*, with *Files
    /// on your computer* where Stop belongs. The same held for every request
    /// while M6's question was open.
    #[test]
    fn a_download_nothing_has_decided_reads_under_way_and_never_done() {
        let run = Identity::issue(IdentityKind::Run).expect("randomness");
        let (folder, record, request) = a_download_written_and_not_yet_decided("undecided", &run);
        let screen = the_screen_from(&what_is_known_from(Some(&record), &HashMap::new(), None, Some(&run)), None);

        // **An earlier run's request left that way reads as a leftover
        // already does, and has nothing staged** (`Download` M13): ended,
        // *— did not finish*, no Stop, not counted on the rail.
        let later = Identity::issue(IdentityKind::Run).expect("randomness");
        let after_a_restart = the_screen_from(&what_is_known_from(Some(&record), &HashMap::new(), None, Some(&later)), None);
        drop(record);
        let _ = std::fs::remove_dir_all(&folder);
        let left = &after_a_restart.requests[0];
        assert!(every_word(&left.name).contains(words::DID_NOT_FINISH), "{}", every_word(&left.name));
        assert_ne!(every_word(&left.count), words::DONE);
        assert_ne!(left.control.as_ref().map(|control| control.act.as_str()), Some("stop"));
        assert!(left.bar.is_none());
        assert!(after_a_restart.the_rails_figure.is_none());

        let drawn = &screen.requests[0];
        assert_eq!(drawn.id.as_str(), request.record.as_text());
        let count = every_word(&drawn.count);
        assert_ne!(count, words::DONE, "a download that has not started reads done");
        assert_eq!(drawn.count.values(), vec!["0 / 1"], "the whole is not the enumeration");
        assert_eq!(every_word(&drawn.name).trim(), words::FILE_INTO);
        assert_eq!(drawn.name.values()[0], "1");
        assert!(!every_word(&drawn.name).contains(words::DID_NOT_FINISH));
        assert_eq!(drawn.control.as_ref().expect("a control").act.as_str(), "stop");
        assert_eq!(drawn.bar.as_ref().map(|bar| (bar.value, bar.max)), Some((0, 4_096)));
        assert_eq!(screen.the_rails_figure.as_ref().map(|figure| figure.as_str()), Some("1"));
        assert!(drawn.files.is_empty(), "a file is drawn that nothing has arranged");
    }

    /// **A record door for these tests**, over one record.
    struct ARecordDoor(Mutex<crate::record::LocalRecord>);

    impl crate::downloading::TheRecordADownloadWrites for ARecordDoor {
        fn with_the_record(&self, act: &mut dyn FnMut(&rusqlite::Connection)) -> bool {
            match self.0.lock() {
                Ok(record) => {
                    act(record.connection());
                    true
                }
                Err(_) => false,
            }
        }
    }

    /// **Step 10c's eleventh-pass fix piece 1: the row's Stop acts on a
    /// download nothing has decided yet**, and it is written as `Download`
    /// M6's own *Stop* — at once where M6's question is open, and when the
    /// first read of the destination returns where it has not. `Download`
    /// M12; `Design/Transfers § What Stop says before it does it`: *"The
    /// request is still Smoozy's to stop even while nothing is crossing."*
    ///
    /// ~~**What the row then reads is what a request answered *Stop* at M6 has
    /// always read** — nothing arranged, so none of the whole, and every unit
    /// ended: *0 files into …*, *done*, *Files on your computer*. No word is
    /// added.~~ **Changed 2026-10-01, step 10c's Vast-pass fix piece 1: a
    /// request answered *Stop* at M6 reads *— did not finish*, counted over
    /// what it asked for**, since *done* said of it is false (Rule 8,
    /// `Design/Transfers`, the sentence of 2026-10-01 after E2-1); the Vast
    /// pass read *0 files into … · done* after its *Stop*. No word is added.
    #[test]
    fn stop_on_a_download_nothing_has_decided_is_m6s_own_stop() {
        let run = Identity::issue(IdentityKind::Run).expect("randomness");
        let (folder, record, request) = a_download_written_and_not_yet_decided("undecided-stop", &run);
        let id = request.record.as_text();
        let door = ARecordDoor(Mutex::new(record));
        let known_now = || {
            let record = door.0.lock().expect("the record");
            what_is_known_from(Some(&record), &HashMap::new(), None, Some(&run))
        };

        // **M6's question is open**: the request waits on it.
        downloads_waiting_on_their_question().lock().expect("held").push((
            id.clone(),
            "row".to_string(),
            crate::downloading::ADownloadReadyToRun {
                request: request.clone(),
                colliding: vec![b"/Users/artist/Downloads/frame_0001.png".to_vec()],
                the_names_already_standing: vec![b"frame_0001.png".to_vec()],
                the_pre_flight: WhatThePreFlightHasToSay { collisions: 1, cannot_vouch_for: 0, refusals: 0 },
                landing_folders: Vec::new(),
            },
        ));
        let before = known_now();
        assert_eq!(before.requests[0].stands, WhereTheRequestStands::Running);
        // **Through the command's own route**: the run's register does not
        // hold the request, so the stop is taken as M6's.
        let the_run = TheRunsTransfers::for_this_run();
        a_stop_is_taken_wherever_the_request_stands(&the_run, &before, &door, TheAct::StopTheRequest, &id, None);
        assert!(
            !downloads_waiting_on_their_question().lock().expect("held").iter().any(|(one, _, _)| *one == id),
            "the question still waits"
        );
        let after = the_screen_from(&known_now(), None);
        let drawn = &after.requests[0];
        let whole = before.requests[0].files_in_the_whole;
        assert_eq!(drawn.count.values(), vec![format!("0 / {whole}")]);
        assert!(every_word(&drawn.name).ends_with(words::DID_NOT_FINISH), "{}", every_word(&drawn.name));
        assert_eq!(drawn.name.values()[0], whole.to_string());
        assert_eq!(drawn.control.as_ref().expect("a control").act.as_str(), "files-on-your-computer");
        assert!(after.the_rails_figure.is_none());

        // **The first read of the destination has not returned**: the stop is
        // held until it does, and the request is never asked about.
        let (other_folder, other_record, other) = a_download_written_and_not_yet_decided("undecided-reading", &run);
        let other_id = other.record.as_text();
        let reading = what_is_known_from(Some(&other_record), &HashMap::new(), None, Some(&run));
        let other_door = ARecordDoor(Mutex::new(other_record));
        a_stop_is_taken_wherever_the_request_stands(&the_run, &reading, &other_door, TheAct::StopTheRequest, &other_id, None);
        assert!(downloads_stopped_before_their_dispositions().lock().expect("held").contains(&other_id));
        // **The read returns**: the stop is written as M6's, nothing is asked,
        // and the stop is not held any longer.
        let mut returned = crate::downloading::ADownloadReadyToRun {
            request: other.clone(),
            colliding: Vec::new(),
            the_names_already_standing: Vec::new(),
            the_pre_flight: WhatThePreFlightHasToSay::default(),
            landing_folders: Vec::new(),
        };
        assert_eq!(
            what_a_stop_before_its_dispositions_answers(&mut returned, &other_door),
            Some(TheDownloadAnswer::ItStarted { request: Verbatim::new(other_id.clone()) })
        );
        assert!(!downloads_stopped_before_their_dispositions().lock().expect("held").contains(&other_id));
        assert_eq!(returned.request.collision_answer, Some(CollisionAnswer::Stop));
        let written = {
            let record = other_door.0.lock().expect("the record");
            the_screen_from(&what_is_known_from(Some(&record), &HashMap::new(), None, Some(&run)), None)
        };
        assert!(
            every_word(&written.requests[0].name).ends_with(words::DID_NOT_FINISH),
            "{}",
            every_word(&written.requests[0].name)
        );
        assert_ne!(every_word(&written.requests[0].count), words::DONE);
        // **A request nobody stopped goes on as before.**
        assert_eq!(what_a_stop_before_its_dispositions_answers(&mut returned, &other_door), None);

        // **A request that is not running is not this act's**: nothing is held
        // for it.
        let mut moving = reading.clone();
        moving.requests[0].stands = WhereTheRequestStands::Ended;
        a_download_not_yet_decided_is_stopped(&moving, &door, &other_id);
        assert!(!downloads_stopped_before_their_dispositions().lock().expect("held").contains(&other_id));

        drop(door);
        drop(other_door);
        let _ = std::fs::remove_dir_all(&folder);
        let _ = std::fs::remove_dir_all(&other_folder);
    }

    // ════════════════════════════════════════════════════════════════════
    // Step 10c's twelfth-pass fix piece 3 — a manual download is on
    // Transfers from the press
    // ════════════════════════════════════════════════════════════════════

    /// **A record with one saved server and nothing else**, and that server's
    /// identity.
    fn a_record_with_one_server(name: &str) -> (PathBuf, crate::record::LocalRecord, Identity) {
        use smoozy_servers::identity::{Host, KeyReference, Port, Username};
        use smoozy_servers::store::ServerRow;
        let folder = crate::scratch_folders::a_folder(format!("smoozy-transfers-{name}-{}", std::process::id()));
        let record = crate::record::LocalRecord::open_at(&folder.join("local-record.sqlite3")).expect("a record");
        let row = ServerRow::saved(
            Identity::issue(IdentityKind::LocalServer).expect("randomness").as_text(),
            Host::what_the_user_typed("192.0.2.41").expect("a host"),
            Port::what_the_user_typed(22).expect("a port"),
            Username::what_the_user_typed("root").expect("a username"),
            KeyReference { local_key_identifier: "key-1".into(), fingerprint: "SHA256:aaa".into() },
        );
        server_rows::add(record.connection(), &row).expect("the row is added");
        server_rows::write(record.connection(), &row).expect("the row is written");
        let server = Identity::read_back(IdentityKind::LocalServer, &row.local_server_identifier).expect("the row");
        (folder, record, server)
    }

    /// **The running application's record door, and every time it tells the
    /// page a transfer moved** — which is the only thing that makes a
    /// Transfers page already on screen ask again (`web/transfers.js`,
    /// `TRANSFERS_MOVED`; `Desktop app and delivery` P6, P8).
    ///
    /// **It keeps the page a person sees**: drawn at the press
    /// ([`ADoorThatCounts::the_press`]), and drawn again at the moment of
    /// every tell, as `web/transfers.js` asks again when told — never at any
    /// other moment.
    struct ADoorThatCounts<'a> {
        record: Mutex<crate::record::LocalRecord>,
        told: std::sync::atomic::AtomicUsize,
        the_run: &'a TheRunsTransfers,
        run: &'a Identity,
        the_page: Mutex<Option<TheTransfersScreen>>,
    }

    impl crate::downloading::TheRecordADownloadWrites for ADoorThatCounts<'_> {
        fn with_the_record(&self, act: &mut dyn FnMut(&rusqlite::Connection)) -> bool {
            match self.record.lock() {
                Ok(record) => {
                    act(record.connection());
                    true
                }
                Err(_) => false,
            }
        }

        fn a_transfer_moved(&self) {
            self.told.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            let drawn = self.the_screen_now();
            *self.the_page.lock().expect("the page") = Some(drawn);
        }
    }

    impl<'a> ADoorThatCounts<'a> {
        fn over(record: crate::record::LocalRecord, the_run: &'a TheRunsTransfers, run: &'a Identity) -> Self {
            ADoorThatCounts {
                record: Mutex::new(record),
                told: Default::default(),
                the_run,
                run,
                the_page: Mutex::new(None),
            }
        }

        fn told(&self) -> usize {
            self.told.load(std::sync::atomic::Ordering::SeqCst)
        }

        /// **What the Transfers command answers at this moment**, for this
        /// run, with what the run is moving and counting.
        fn the_screen_now(&self) -> TheTransfersScreen {
            let record = self.record.lock().expect("the record");
            the_screen_from(
                &what_is_known_from(
                    Some(&record),
                    &self.the_run.what_is_under_way(),
                    Some(self.the_run),
                    Some(self.run),
                ),
                None,
            )
        }

        /// **The press**: Transfers is put on screen and asks once.
        fn the_press(&self) -> TheTransfersScreen {
            let drawn = self.the_screen_now();
            *self.the_page.lock().expect("the page") = Some(drawn.clone());
            drawn
        }

        /// **What the page shows**: what it drew when it last asked.
        fn the_page(&self) -> TheTransfersScreen {
            self.the_page.lock().expect("the page").clone().expect("the page was drawn")
        }
    }

    /// **A folder of files on the machine**, read one folder at a time as the
    /// walk reads it. **Where `held` is given, the first reading waits on it**
    /// — a count still going.
    struct AFolderOnTheMachine {
        folder: Vec<u8>,
        files: usize,
        held: Option<Arc<TheHold>>,
    }

    impl smoozy_download::acting::walking::ReadsOneDirectoryOnTheMachine for AFolderOnTheMachine {
        async fn read_one_directory(
            &mut self,
            path: &[u8],
        ) -> Result<Vec<smoozy_download::acting::walking::ADirectoryEntry>, smoozy_server_protocol::subsystem::SubsystemFailure>
        {
            if let Some(held) = self.held.take() {
                held.reached.store(true, std::sync::atomic::Ordering::SeqCst);
                while !held.let_go.load(std::sync::atomic::Ordering::SeqCst) {
                    tokio::time::sleep(std::time::Duration::from_millis(5)).await;
                }
            }
            if path != self.folder.as_slice() {
                return Err(smoozy_server_protocol::subsystem::SubsystemFailure::TheServerRefused(
                    smoozy_server_protocol::subsystem::StatusCode::NoSuchFile,
                ));
            }
            Ok((0..self.files)
                .map(|at| smoozy_download::acting::walking::ADirectoryEntry {
                    name: format!("file_{at:04}.bin").into_bytes(),
                    kind: smoozy_download::acting::walking::TheEntrysKind::APlainFile,
                    size: 1_024,
                    modification_time: 0,
                })
                .collect())
        }
    }

    /// **A wait a test holds and lets go of**, and whether it has been reached.
    #[derive(Default)]
    struct TheHold {
        reached: std::sync::atomic::AtomicBool,
        let_go: std::sync::atomic::AtomicBool,
    }

    /// **Lets a hold go when dropped**, so a test that fails while something
    /// is held ends rather than waiting on it for good.
    struct LetGoAtTheEnd(Arc<TheHold>);

    impl Drop for LetGoAtTheEnd {
        fn drop(&mut self) {
            self.0.goes();
        }
    }

    impl TheHold {
        fn until_reached(&self) {
            let began = std::time::Instant::now();
            while !self.reached.load(std::sync::atomic::Ordering::SeqCst) {
                assert!(began.elapsed() < std::time::Duration::from_secs(10), "the hold was never reached");
                std::thread::sleep(std::time::Duration::from_millis(5));
            }
        }

        fn goes(&self) {
            self.let_go.store(true, std::sync::atomic::Ordering::SeqCst);
        }
    }

    /// **The user's Downloads folder, whose first read waits on the system's
    /// permission box** — the twelfth pass's *Downloads* box on a fresh copy,
    /// which the tester's Transfers page sat under reading *Nothing is
    /// uploading or downloading.* **The box is `held`**: the first read of
    /// the destination, whichever it is, does not return until the test lets
    /// it go.
    struct TheDownloadsFolderBehindTheBox {
        held: Arc<TheHold>,
    }

    impl TheDownloadsFolderBehindTheBox {
        fn the_box_stands(&self) {
            if !self.held.reached.swap(true, std::sync::atomic::Ordering::SeqCst) {
                while !self.held.let_go.load(std::sync::atomic::Ordering::SeqCst) {
                    std::thread::sleep(std::time::Duration::from_millis(5));
                }
            }
        }
    }

    impl smoozy_download::acting::enumerating::MakesTheLandingFolder for TheDownloadsFolderBehindTheBox {
        fn create_the_landing_folder(
            &mut self,
            _the_landing_folder: &[u8],
        ) -> Result<(), smoozy_download::destination::WhatTheAdapterSaid> {
            self.the_box_stands();
            Ok(())
        }
    }

    impl smoozy_download::request::ReadsTheDestinationsListing for TheDownloadsFolderBehindTheBox {
        fn what_is_standing_in(&mut self, _the_folder: &[u8]) -> Vec<Vec<u8>> {
            self.the_box_stands();
            Vec::new()
        }
    }

    /// No job is known on this server.
    struct NoJobHere;

    impl smoozy_download::acting::enumerating::ReadsTheLocalJobRecords for NoJobHere {
        fn what_the_local_record_says(&self, _source_path: &[u8]) -> smoozy_download::vouching::WhatTheLocalRecordSays {
            smoozy_download::vouching::WhatTheLocalRecordSays {
                the_file_is_inside_the_jobs_output_folder: false,
                the_recorded_ending: None,
                the_completeness_statement: None,
            }
        }

        fn the_files_frame(&self, _source_path: &[u8]) -> Option<i64> {
            None
        }

        fn the_job_whose_output_folder_this_is(
            &self,
            _picked: &[u8],
        ) -> Option<smoozy_download::acting::enumerating::AJobsOwnFolder> {
            None
        }
    }

    /// **Step 10c's twelfth-pass fix piece 3, fault 4 as the tester met it: a
    /// download whose first read of the Downloads folder waits on the
    /// system's permission box is on the Transfers page already open.**
    /// `Download` M1 and M16; `Desktop app and delivery` P6 and P7;
    /// `Design/Transfers § A request shows the whole and the files under it`;
    /// Rule 8.
    ///
    /// **The twelfth pass on Hyperstack**: the first *Download* on a fresh copy
    /// wrote its request at 19:23:48 and raised the system's box, and for the
    /// thirty seconds until *Allow* the Transfers page — put on screen by the
    /// press itself — read *Nothing is uploading or downloading.* The request
    /// was written; eleventh-pass fix piece 1 composes it under way whenever
    /// the screen is asked for; **and nothing asked**, because the page asks
    /// again only when it is told a transfer moved, and the write told it
    /// nothing.
    ///
    /// **So the page here is what a person sees**: drawn at the press, and
    /// drawn again only when the door tells it. **Both ways in**: the
    /// application's, with the count shown from the press, and the one the
    /// kept harness takes, with none.
    #[test]
    fn a_download_behind_the_permission_box_is_on_the_transfers_page_already_open() {
        for with_the_count in [true, false] {
            the_permission_box_stands_over_a_download(with_the_count);
        }
    }

    fn the_permission_box_stands_over_a_download(with_the_count: bool) {
        let (folder, record, server) = a_record_with_one_server(&format!("behind-the-box-{with_the_count}"));
        let run = Identity::issue(IdentityKind::Run).expect("randomness");
        let the_run = TheRunsTransfers::for_this_run();
        let door = ADoorThatCounts::over(record, &the_run, &run);
        let the_box = Arc::new(TheHold::default());
        let asked = crate::downloading::WhatIsBeingDownloaded {
            server_row: server.clone(),
            installation: None,
            destination: b"/Users/artist/Downloads".to_vec(),
            picked: vec![b"/home/ubuntu/p12-many".to_vec()],
            run: run.clone(),
        };
        let shown = with_the_count.then(|| crate::downloading::TheCountIsShown {
            the_run: &the_run,
            request: Identity::issue(IdentityKind::Operation).expect("randomness"),
            named: "p12-many".into(),
        });

        // **The press**: Transfers is put on screen and asks once.
        door.the_press();

        std::thread::scope(|scope| {
            let preparing = scope.spawn(|| {
                let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build().expect("a runtime");
                runtime.block_on(crate::downloading::a_download_is_prepared_over(
                    asked,
                    shown,
                    &mut AFolderOnTheMachine { folder: b"/home/ubuntu/p12-many".to_vec(), files: 3, held: None },
                    &door,
                    &NoJobHere,
                    &mut TheDownloadsFolderBehindTheBox { held: Arc::clone(&the_box) },
                    &mut TheDownloadsFolderBehindTheBox { held: Arc::clone(&the_box) },
                ))
            });

            let _ends = LetGoAtTheEnd(Arc::clone(&the_box));
            // **The box stands.** The page shows what it drew when it was last
            // told.
            the_box.until_reached();
            let told = door.told();
            let while_the_box_stands = door.the_page();
            the_box.goes();
            let ready = preparing.join().expect("the preparing thread").expect("the request is ready");

            assert_eq!(
                while_the_box_stands.nothing_here.as_ref().map(every_word),
                None,
                "the page read Nothing is uploading or downloading while the box stood (told {told} times, \
                 with the count {with_the_count})"
            );
            assert_eq!(while_the_box_stands.requests.len(), 1, "one row, never the count and the request both");
            let drawn = &while_the_box_stands.requests[0];
            assert_eq!(drawn.id.as_str(), ready.request.record.as_text());
            assert_eq!(drawn.count.values(), vec!["0 / 3"], "the row is not the request written");
            assert_eq!(drawn.control.as_ref().expect("a control").act.as_str(), "stop");
            assert_eq!(while_the_box_stands.the_rails_figure.as_ref().map(|one| one.as_str()), Some("1"));
        });
        drop(door);
        let _ = std::fs::remove_dir_all(&folder);
    }

    /// **Step 10c's twelfth-pass fix piece 3, fault 9: a folder download is on
    /// Transfers from the press, reading *Counting files…* with *Stop*, and a
    /// *Stop* during the count leaves nothing.** `Design/Transfers § A request
    /// shows the whole and the files under it`, its paragraph of 2026-10-01;
    /// `Download` M1, M12; Rule 8.
    ///
    /// **The twelfth pass on Hyperstack**: a right-click *Download* of a
    /// 1,000-file folder at 20:47:29, and for three minutes nothing on
    /// Transfers and no request in the record, because `Download` M1 writes
    /// the request only once the count is done. **The count here is held**
    /// at its first folder, which is the whole of that silence.
    #[test]
    fn a_folder_download_is_on_transfers_from_the_press_and_its_stop_leaves_nothing() {
        let (folder, record, server) = a_record_with_one_server("counting");
        let run = Identity::issue(IdentityKind::Run).expect("randomness");
        let the_run = TheRunsTransfers::for_this_run();
        let door = ADoorThatCounts::over(record, &the_run, &run);
        let the_count = Arc::new(TheHold::default());
        let the_box = Arc::new(TheHold::default());
        let request = Identity::issue(IdentityKind::Operation).expect("randomness");
        let id = request.as_text();
        let asked = crate::downloading::WhatIsBeingDownloaded {
            server_row: server.clone(),
            installation: None,
            destination: b"/Users/artist/Downloads".to_vec(),
            picked: vec![b"/home/ubuntu/p12-many".to_vec()],
            run: run.clone(),
        };

        // **The press**: nothing is on the page yet.
        let mut the_page = door.the_press();
        assert!(the_page.requests.is_empty());

        std::thread::scope(|scope| {
            let preparing = scope.spawn(|| {
                let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build().expect("a runtime");
                runtime.block_on(crate::downloading::a_download_is_prepared_over(
                    asked,
                    Some(crate::downloading::TheCountIsShown { the_run: &the_run, request, named: "p12-many".into() }),
                    &mut AFolderOnTheMachine {
                        folder: b"/home/ubuntu/p12-many".to_vec(),
                        files: 1_000,
                        held: Some(Arc::clone(&the_count)),
                    },
                    &door,
                    &NoJobHere,
                    &mut TheDownloadsFolderBehindTheBox { held: Arc::clone(&the_box) },
                    &mut TheDownloadsFolderBehindTheBox { held: Arc::clone(&the_box) },
                ))
            });

            let _ends = (LetGoAtTheEnd(Arc::clone(&the_count)), LetGoAtTheEnd(Arc::clone(&the_box)));
            // **The count is going**, and the page was told.
            the_count.until_reached();
            let told = door.told();
            assert!(told > 0, "the page was not told the count began");
            the_page = door.the_page();
            assert_eq!(the_page.nothing_here, None, "the page reads nothing while the files are counted");
            let drawn = &the_page.requests[0];
            assert_eq!(drawn.id.as_str(), id);
            assert_eq!(drawn.name.values(), vec!["p12-many"], "the row names the folder");
            assert_eq!(every_word(&drawn.count), words::COUNTING_FILES);
            assert_eq!(drawn.control.as_ref().expect("a control").act.as_str(), "stop");
            assert_eq!(drawn.tag_class, "dn");
            assert!(drawn.bar.is_none(), "a bar is drawn over a whole nobody knows");
            assert_eq!(the_page.the_rails_figure.as_ref().map(|one| one.as_str()), Some("1"));
            // **Its Stop box asks, and says nothing is discarded.**
            let known = {
                let record = door.record.lock().expect("the record");
                what_is_known_from(Some(&record), &the_run.what_is_under_way(), Some(&the_run), Some(&run))
            };
            let counted = known.requests.iter().find(|one| one.id == id).expect("the count is known");
            match the_stop_box_for(counted) {
                TheStopAnswer::TheBox(the_box) => {
                    assert!(the_box.bullets.is_empty());
                    assert_eq!(
                        the_box.nothing_is_thrown_away.as_ref().map(|one| one.as_str().replace('\u{a0}', " ")),
                        Some(words::NOTHING_IS_DOWNLOADING_NOW.to_string())
                    );
                }
                TheStopAnswer::NothingToStop => panic!("a count has nothing to stop"),
            }

            // **Stop, through the command's own route.** The count ends within
            // moments, or the stop did not reach it.
            a_stop_is_taken_wherever_the_request_stands(&the_run, &known, &door, TheAct::StopTheRequest, &id, None);
            let began = std::time::Instant::now();
            while !preparing.is_finished() {
                assert!(began.elapsed() < std::time::Duration::from_secs(10), "the count went on after its Stop");
                std::thread::sleep(std::time::Duration::from_millis(5));
            }
            let ended = preparing.join().expect("the preparing thread");
            assert!(
                matches!(ended, Err(crate::downloading::WhyTheDownloadDidNotStart::TheUserStoppedTheCount)),
                "{:?}",
                ended.map(|ready| ready.request.entries.len())
            );
            // **The command lets its hold go** (`the_request_is_put_to_the_user`).
            assert!(downloads_stopped_before_their_dispositions().lock().expect("held").remove(&id));

            // **Nothing is left**: no request written, the user's disk not
            // touched, and the page told the row went.
            assert!(door.told() > told, "the page was not told the count went");
            the_page = door.the_page();
            assert!(the_page.requests.is_empty(), "a row stands after the count was stopped");
            assert_eq!(the_page.nothing_here.as_ref().map(every_word).as_deref(), Some(words::NOTHING_IS_MOVING));
            let record = door.record.lock().expect("the record");
            use smoozy_download::request::KeepsTheRequest as _;
            assert!(
                DownloadRequests::over(record.connection())
                    .every_request_for(&server)
                    .expect("the requests read")
                    .is_empty(),
                "a stopped count wrote a request"
            );
            assert!(!the_box.reached.load(std::sync::atomic::Ordering::SeqCst), "the destination was read");
            assert!(the_run.what_is_being_counted().is_empty());
        });
        drop(door);
        let _ = std::fs::remove_dir_all(&folder);
    }

    /// **A count whose request is already written is one row, not two** — the
    /// request is written a moment before the count lets go. Step 10c's
    /// twelfth-pass fix piece 3.
    #[test]
    fn a_count_whose_request_is_written_is_drawn_once() {
        let run = Identity::issue(IdentityKind::Run).expect("randomness");
        let (folder, record, request) = a_download_written_and_not_yet_decided("counted-once", &run);
        let the_run = TheRunsTransfers::for_this_run();
        let door = ADoorThatCounts::over(record, &the_run, &run);
        let count = the_run.a_count_begins(
            &request.record,
            &request.server_row,
            "p12-many".into(),
            request.destination.clone(),
            &door,
        );
        let screen = door.the_screen_now();
        drop(count);
        drop(door);
        let _ = std::fs::remove_dir_all(&folder);
        assert_eq!(screen.requests.len(), 1);
        assert_eq!(screen.requests[0].count.values(), vec!["0 / 1"]);
    }

    /// **What a counting row names**: the folder by its own name, and how
    /// many more where several were picked. Step 10c's twelfth-pass fix piece
    /// 3.
    #[test]
    fn a_counting_row_names_the_folder_picked() {
        assert_eq!(what_the_counting_row_names(&[b"/home/ubuntu/p12-many".to_vec()]), "p12-many");
        assert_eq!(what_the_counting_row_names(&[b"/home/ubuntu/p12-many/".to_vec()]), "p12-many");
        assert_eq!(
            what_the_counting_row_names(&[b"/home/ubuntu/a".to_vec(), b"/home/ubuntu/b".to_vec(), b"/c".to_vec()]),
            "a and 2 more"
        );
        assert_eq!(what_the_counting_row_names(&[b"/".to_vec()]), "/");
    }

    /// **A manual download stopped at `Download` M6's question, or one that
    /// met something on the server it could not read, does not read *done*** —
    /// Rule 8, `Design/Transfers § A file that failed`, the sentence of
    /// 2026-10-01. Step 10c's Vast-pass fix piece 1: after *Stop* the Vast
    /// pass's row read *0 files into … · done*.
    #[test]
    fn a_download_stopped_at_the_question_or_that_could_not_read_something_does_not_read_done() {
        let issue = |kind| Identity::issue(kind).expect("randomness");
        let counted = |done: usize, stopped: bool, could_not_be_read: usize| {
            crate::record::download_records::AManualRequestCounted {
                record: issue(IdentityKind::Operation),
                destination: b"/Users/artist/Downloads".to_vec(),
                files_in_the_whole: 1,
                files_done: done,
                bytes_in_the_whole: 93_790,
                bytes_done: 93_790 * done as u64,
                every_unit_has_ended: true,
                cannot_land: 0,
                nothing_is_decided: false,
                opened_by_run: issue(IdentityKind::Run),
                failed_landings: Vec::new(),
                stopped_at_the_question: stopped,
                could_not_be_read,
            }
        };
        let stopped = counted(0, true, 0);
        let refused = counted(1, false, 1);
        let done = counted(1, false, 0);
        let screen = the_screen_from(
            &known(vec![
                a_manual_request_known("row", &stopped),
                a_manual_request_known("row", &refused),
                a_manual_request_known("row", &done),
            ]),
            None,
        );
        let requests: Vec<_> = [stopped.record.as_text(), refused.record.as_text(), done.record.as_text()].into_iter().map(|id| screen.requests.iter().find(|r| r.id.as_str() == id).unwrap()).collect();
        for (drawn, count) in requests.iter().zip(["0 / 1", "1 / 1"]) {
            assert!(every_word(&drawn.name).ends_with(words::DID_NOT_FINISH), "{}", every_word(&drawn.name));
            assert_eq!(drawn.count.values(), vec![count]);
        }
        assert_eq!(every_word(&requests[2].count), words::DONE);
    }

    /// **An auto-download of a job sent with no name is named by its file**, as
    /// Queue and History name it (`crate::surface::the_queue::what_the_job_is_called`,
    /// `Design/Queue & History § A rendering entry`). Step 10c's Vast-pass fix
    /// piece 1, at the runner's word after piece 2: it read as the job's
    /// identity. A named job keeps its name.
    #[test]
    fn an_auto_download_of_a_job_with_no_name_is_named_by_its_file() {
        let folder = std::env::temp_dir().join(format!("smoozy-transfers-no-name-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a scratch folder");
        let record = crate::record::LocalRecord::open_at(&folder.join("local-record.sqlite3")).expect("the record");
        let server = Identity::issue(IdentityKind::LocalServer).expect("randomness");
        let store = crate::record::job_records::JobRecords::over(record.connection());
        let mut unnamed = crate::record::deletions::tests::a_record_in_history(&server, "");
        unnamed.label = None;
        unnamed.blend_path_on_the_machine = Some("/pv-huge/blend/pv-anim.blend".to_string());
        store.write_the_provisional_record(&unnamed).expect("written");
        let named = crate::record::deletions::tests::a_record_in_history(&server, "pv long2");
        store.write_the_provisional_record(&named).expect("written");
        let an_arrangement = |job: &Identity| crate::record::download_records::AnArrangementCounted {
            job: job.clone(),
            destination: b"/Users/artist/Downloads".to_vec(),
            narrowing: TheNarrowing::Everything,
            desired_state: DesiredState::On,
            entries: 0,
            fetched: 0,
            left_alone: 0,
            owed: 0,
            skipped_by_narrowing: 0,
        cannot_land: 0,
        };
        let name_of = |known: ARequestKnown| match known.of {
            WhatItIsOf::AJob { name, .. } => name,
            _ => panic!("an auto-download is of a job"),
        };
        let unnamed_is = name_of(an_arrangement_known(record.connection(), "row", &an_arrangement(&unnamed.job)));
        let named_is = name_of(an_arrangement_known(record.connection(), "row", &an_arrangement(&named.job)));
        drop(record);
        let _ = std::fs::remove_dir_all(&folder);
        assert_eq!(unnamed_is, "pv-anim.blend");
        assert_eq!(named_is, "pv long2");
    }

    /// **The Transfers row names an auto-download's narrowing in Smoozy's
    /// words** — *Multilayer EXR only*, never `OPEN_EXR_MULTILAYER` (Rule 9;
    /// the Creator's second notes, piece 2). The arrangement keeps the
    /// identifier.
    #[test]
    fn a_narrowed_auto_download_names_its_formats_in_smoozys_words() {
        let folder = std::env::temp_dir().join(format!("smoozy-transfers-narrowed-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a scratch folder");
        let record = crate::record::LocalRecord::open_at(&folder.join("local-record.sqlite3")).expect("the record");
        let server = Identity::issue(IdentityKind::LocalServer).expect("randomness");
        let job = crate::record::deletions::tests::a_record_in_history(&server, "Hero");
        crate::record::job_records::JobRecords::over(record.connection())
            .write_the_provisional_record(&job)
            .expect("written");
        let known = an_arrangement_known(
            record.connection(),
            "row",
            &crate::record::download_records::AnArrangementCounted {
                job: job.job.clone(),
                destination: b"/Users/artist/Downloads".to_vec(),
                narrowing: TheNarrowing::TheseFormats(vec![b"OPEN_EXR_MULTILAYER".to_vec(), b"FFMPEG".to_vec()]),
                desired_state: DesiredState::On,
                entries: 0,
                fetched: 0,
                left_alone: 0,
                owed: 0,
                skipped_by_narrowing: 0,
            cannot_land: 0,
            },
        );
        drop(record);
        let _ = std::fs::remove_dir_all(&folder);
        assert_eq!(known.narrowed_to, vec!["Multilayer EXR".to_string(), "Video".to_string()]);
    }

    /// **Where nothing is already there, `Download` M6's box asks nothing about
    /// it** — `Design/Transfers § A download meeting files already there`, the
    /// paragraph of 2026-10-01: no count, and *Download* and *Stop*. Step 10c's
    /// Vast-pass fix piece 1: the Vast pass read *0 files are already there*
    /// above *Replace · Keep both · Skip existing · Stop*.
    #[test]
    fn where_nothing_is_already_there_the_question_asks_nothing_about_it() {
        let the_box = |collisions| match the_question(
            "operation:1",
            &WhatThePreFlightHasToSay { collisions, cannot_vouch_for: 1, refusals: 1 },
        ) {
            TheDownloadAnswer::ItAsks { headline, cannot_vouch_for, refusals, answers, .. } => (
                headline.is_some(),
                cannot_vouch_for.is_some() && refusals.is_some(),
                answers
                    .iter()
                    .map(|answer| format!("{}={}", answer.label.as_str(), answer.act.as_str()))
                    .collect::<Vec<_>>(),
            ),
            other => panic!("it does not ask: {other:?}"),
        };
        assert_eq!(
            the_box(0),
            (false, true, vec![format!("{}=download", words::DOWNLOAD), format!("{}=stop", words::STOP)])
        );
        let (headline, beside, answers) = the_box(2);
        assert!(headline && beside);
        assert_eq!(answers.len(), 4);
    }

    /// **Step 10c's twelfth-pass fix piece 2: `Download` M6's question is drawn
    /// and answered from exactly what the application sends — and this runs
    /// the page rather than reading it.** `Design/Transfers § A download meeting
    /// files already there`; Rule 8.
    ///
    /// **The twelfth pass on Hyperstack** met *1 file is already there* with four
    /// blank 16 × 16 buttons, and *Download did not start* said nothing. The page
    /// read the answers' words as `answer.label.said`, their act as
    /// `answer.act.value` and the request as `request.value`, and the sentence
    /// as `said.said`, while [`Prose`] and [`Verbatim`] each go over as the
    /// plain string they hold. **So this test writes the two answers out through
    /// the same serialisation the command's reply takes**, and hands the page
    /// that text and nothing else: a shape changed on either side fails here.
    ///
    /// `web/transfers.js` is loaded under Node beside stand-ins for the modules
    /// it imports and the least of a document it needs. **Skipped, and says so,
    /// where no Node is installed.**
    #[test]
    fn the_download_question_is_drawn_and_answered_from_what_the_application_sends() {
        let id = "operation:5f1c0a52-0000-4000-8000-000000000001";
        let asks = the_question(id, &WhatThePreFlightHasToSay { collisions: 1, cannot_vouch_for: 0, refusals: 0 });
        let did_not_start = refused(words::THE_SERVER_IS_NOT_CONNECTED);
        // **What the answer's own command replies**: it started, or — asked
        // after the request went — that the answer changed nothing.
        let started = TheDownloadAnswer::ItStarted { request: Verbatim::new(id) };
        let no_longer_waiting = refused(words::THE_REQUEST_IS_NO_LONGER_WAITING);
        let one = |answer: &TheDownloadAnswer| serde_json::to_string(answer).expect("the answer serialises");
        // **A count the user stopped says nothing**, and takes an older answer
        // away — step 10c's twelfth-pass fix piece 3.
        let stopped = TheDownloadAnswer::TheUserStoppedIt;
        // **Nothing already there, one thing that could not be read** — step
        // 10c's Vast-pass fix piece 1.
        let nothing_there =
            the_question(id, &WhatThePreFlightHasToSay { collisions: 0, cannot_vouch_for: 0, refusals: 1 });
        let answers = format!(
            r#"{{ "asks": {}, "did_not_start": {}, "started": {}, "no_longer_waiting": {}, "stopped": {}, "nothing_there": {} }}"#,
            one(&asks),
            one(&did_not_start),
            one(&started),
            one(&no_longer_waiting),
            one(&stopped),
            one(&nothing_there),
        );

        let folder = std::env::temp_dir().join(format!("smoozy-transfers-page-harness-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a scratch folder");
        let web = Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        for page in ["transfers.js", "the-value-drawn.js"] {
            std::fs::copy(web.join(page), folder.join(page)).expect("the page is copied");
        }
        for (name, text) in THE_QUESTIONS_STAND_INS {
            std::fs::write(folder.join(name), text).expect("a stand-in is written");
        }
        std::fs::write(folder.join("answers.json"), answers).expect("the answers are written");
        let ran = std::process::Command::new("node").arg(folder.join("harness.js")).current_dir(&folder).output();
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the download question's page was not run");
            let _ = std::fs::remove_dir_all(&folder);
            return;
        };
        let _ = std::fs::remove_dir_all(&folder);
        assert!(ran.status.success(), "the page did not run: {}", String::from_utf8_lossy(&ran.stderr));
        let said = String::from_utf8_lossy(&ran.stdout).trim().to_string();

        let label = |words: &str| Prose::new(words).as_str().to_string();
        let sent = |answer: &str| {
            format!(r#"the_download_question_was_answered {{"request":"{id}","answer":"{answer}"}}"#)
        };
        let expected = [
            format!("heading 1\u{a0}{}", label(words::FILE_IS_ALREADY_THERE)),
            // **The four sit level** — nothing is put forward on a download —
            // **and Stop is set apart**, as the upload's box sets it apart.
            format!(
                "buttons {}|btn quiet · {}|btn quiet · {}|btn quiet · grow · {}|btn quiet",
                label(words::REPLACE),
                label(words::KEEP_BOTH),
                label(words::SKIP_EXISTING),
                label(words::STOP),
            ),
            format!("pressed {} -> {} · boxes after 0", label(words::REPLACE), sent("replace")),
            format!("pressed {} -> {} · boxes after 0", label(words::KEEP_BOTH), sent("keep-both")),
            format!("pressed {} -> {} · boxes after 0", label(words::SKIP_EXISTING), sent("skip-existing")),
            format!("pressed {} -> {} · boxes after 0", label(words::STOP), sent("stop")),
            "after four answers that started: said nothing".to_string(),
            format!("answered after the request went: {} · shown yes", label(words::THE_REQUEST_IS_NO_LONGER_WAITING)),
            format!("did not start: {} · shown yes", label(words::THE_SERVER_IS_NOT_CONNECTED)),
            "stopped while counted: shown no".to_string(),
            format!(
                "nothing there: heading none · amber 1\u{a0}{} · buttons {}|btn quiet · grow · {}|btn quiet",
                label(words::COULD_NOT_BE_READ_ONE),
                label(words::DOWNLOAD),
                label(words::STOP),
            ),
            format!("pressed {} -> {}", label(words::DOWNLOAD), sent("download")),
        ]
        .join("\n");
        assert_eq!(said, expected);
    }

    /// **A *…to* act whose folder panel is closed without a folder leaves the
    /// user where they pressed, with nothing said** — the Creator's fifth note
    /// 7, piece Q: *"Download all to... jumps right into transfers, even if I
    /// cancel out of the folder picker."* **Once a folder is chosen it goes to
    /// Transfers at that moment, before the request answers**, because the
    /// count's row and its *Stop* are drawn there (`Design/Transfers`' paragraph
    /// of 2026-10-01). *Download all* and *Download* go there on the press, as
    /// before; a *…to* act refused before its panel opened is said on
    /// Transfers, as before.
    ///
    /// **This runs the page**, as the question's test above does, with the
    /// rail standing in so every move is counted. Skipped, and says so, where no
    /// Node is installed.
    #[test]
    fn a_cancelled_folder_panel_stays_where_it_was_pressed() {
        let one = |answer: &TheDownloadAnswer| serde_json::to_string(answer).expect("the answer serialises");
        let answers = format!(
            r#"{{ "no_folder": {}, "started": {}, "not_connected": {}, "no_download_folder": {}, "chosen": "{}" }}"#,
            one(&TheDownloadAnswer::TheUserChoseNoFolder),
            one(&TheDownloadAnswer::ItStarted { request: Verbatim::new("operation:1") }),
            one(&refused(words::THE_SERVER_IS_NOT_CONNECTED)),
            one(&refused(words::THERE_IS_NO_DOWNLOAD_FOLDER)),
            A_DOWNLOAD_FOLDER_WAS_CHOSEN,
        );
        let folder = std::env::temp_dir().join(format!("smoozy-transfers-piece-q-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a scratch folder");
        let web = Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        for page in ["transfers.js", "the-value-drawn.js"] {
            std::fs::copy(web.join(page), folder.join(page)).expect("the page is copied");
        }
        // **The question's stand-ins, with the rail and the route counting**,
        // and its document taken whole from its harness.
        let mut the_document = "";
        for (name, text) in THE_QUESTIONS_STAND_INS {
            match name {
                "harness.js" => {
                    the_document = text.split("const answers = JSON.parse").next().expect("the document");
                }
                "event-route.js" | "the-rail.js" => {}
                _ => std::fs::write(folder.join(name), text).expect("a stand-in is written"),
            }
        }
        std::fs::write(
            folder.join("event-route.js"),
            r#"export function whenTold(kind, handler) { (globalThis.told[kind] = globalThis.told[kind] || []).push(handler); }"#,
        )
        .expect("the route is written");
        std::fs::write(
            folder.join("the-rail.js"),
            r#"export const TRANSFERS = "transfersview";
export const A_SERVER = "machineview";
export function aScreenIsRegistered(screen) { globalThis.screens.set(screen.place, screen); }
export function theCurrentPlaceIs(place) { globalThis.went.push(place); }
export function theRowsFigureIs() {}"#,
        )
        .expect("the rail is written");
        std::fs::write(folder.join("harness.js"), format!("{the_document}{THE_PIECE_Q_HARNESS}")).expect("the harness");
        std::fs::write(folder.join("answers.json"), answers).expect("the answers are written");
        let ran = std::process::Command::new("node").arg(folder.join("harness.js")).current_dir(&folder).output();
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the folder panel's page was not run");
            let _ = std::fs::remove_dir_all(&folder);
            return;
        };
        let _ = std::fs::remove_dir_all(&folder);
        assert!(ran.status.success(), "the page did not run: {}", String::from_utf8_lossy(&ran.stderr));
        let said = String::from_utf8_lossy(&ran.stdout).trim().to_string();
        let label = |words: &str| Prose::new(words).as_str().to_string();
        let expected = [
            "Download all to, panel closed: pressed nowhere · chosen - · answered nowhere · line hidden".to_string(),
            "Download to, panel closed: pressed nowhere · chosen - · answered nowhere · line hidden".to_string(),
            "Download all to, folder chosen: pressed nowhere · chosen transfersview · answered transfersview · line hidden"
                .to_string(),
            format!(
                "Download to, folder chosen, refused: pressed nowhere · chosen transfersview · answered transfersview · line {}",
                label(words::THERE_IS_NO_DOWNLOAD_FOLDER)
            ),
            format!(
                "Download all to, refused before the panel: pressed nowhere · chosen - · answered transfersview · line {}",
                label(words::THE_SERVER_IS_NOT_CONNECTED)
            ),
            "Download all: pressed transfersview · chosen - · answered transfersview · line hidden".to_string(),
            "Download: pressed transfersview · chosen - · answered transfersview · line hidden".to_string(),
            "a folder told with nothing waiting: went nowhere".to_string(),
        ]
        .join("\n");
        assert_eq!(said, expected);
    }

    /// **Only a folder chosen is told** — piece Q's application half. A panel
    /// closed without a folder, and no panel at all, tell the page nothing.
    /// **And the folder panel is the one place that tells it**, read off this
    /// file's own text, because a test that ran the panel would need a shell.
    #[test]
    fn only_a_chosen_folder_is_told_to_the_page() {
        struct ThePage(Arc<Mutex<Vec<&'static str>>>);
        impl crate::events::Delivers for ThePage {
            fn deliver(&self, delivery: &crate::events::Delivery) {
                self.0.lock().expect("the page").push(delivery.kind);
            }
        }
        let got = Arc::new(Mutex::new(Vec::new()));
        let queue = TheOneQueue::new();
        queue.the_page_is_ready(Box::new(ThePage(Arc::clone(&got))));
        the_chosen_folder_is_told(&queue, &TheDestination::TheUserChoseNoFolder);
        the_chosen_folder_is_told(&queue, &TheDestination::NoPanelOpened);
        assert!(got.lock().expect("the page").is_empty());
        the_chosen_folder_is_told(&queue, &TheDestination::It(b"/Users/artist/Renders".to_vec()));
        assert_eq!(*got.lock().expect("the page"), vec![A_DOWNLOAD_FOLDER_WAS_CHOSEN]);

        let working = the_working_half();
        let chosen = working
            .split("async fn the_destination_chosen<")
            .nth(1)
            .expect("the destination is chosen in this file")
            .split("\n}\n")
            .next()
            .expect("it closes");
        let asked = chosen.find("crate::picker::ask").expect("it puts the panel up");
        let told = chosen.find("the_chosen_folder_is_told").expect("it tells the page");
        assert!(asked < told, "the page is told before the panel has answered");
        assert_eq!(working.matches("the_chosen_folder_is_told(").count(), 2, "one call and its definition");
        assert!(the_page().contains(A_DOWNLOAD_FOLDER_WAS_CHOSEN));
    }

    /// **Piece Q's presses**, after the question's document. Each download
    /// command waits until the harness answers it, so what the page did on the
    /// press, on the folder being chosen, and on the answer are read apart.
    const THE_PIECE_Q_HARNESS: &str = r#"const answers = JSON.parse(readFileSync("answers.json", "utf8"));
globalThis.told = {};
globalThis.went = [];
let release = null;
globalThis.window = {
  __TAURI__: {
    core: {
      invoke(command) {
        if (command === "a_whole_job_download_is_asked_for" || command === "a_file_download_is_asked_for") {
          return new Promise((done) => { release = done; });
        }
        return new Promise(() => {});
      },
    },
  },
};
const page = await import("./transfers.js");
const host = anElement("div");
body.appendChild(host);
globalThis.screens.get("transfersview").itIsDrawn(host);
const line = host.querySelector("p.transfers-answer");
const where = () => (globalThis.went.length ? globalThis.went.join(",") : "nowhere");
const told = (kind) => { for (const handler of globalThis.told[kind] || []) handler({ kind, subject: "run", said: {} }); };
async function aPress(named, act, whose, picked, folderChosen, reply) {
  globalThis.went.length = 0;
  line.hidden = true;
  const asked = page.aTransferIsAskedFor(act, whose, picked);
  const pressed = where();
  let chosen = "-";
  if (folderChosen) { told(answers.chosen); chosen = where(); }
  release(reply);
  await asked;
  console.log(named + ": pressed " + pressed + " · chosen " + chosen + " · answered " + where() + " · line " + (line.hidden ? "hidden" : line.textContent));
}
await aPress("Download all to, panel closed", "download-all-to", "job:1", undefined, false, answers.no_folder);
await aPress("Download to, panel closed", "download-to", "row", ["/home/ubuntu/shots"], false, answers.no_folder);
await aPress("Download all to, folder chosen", "download-all-to", "job:1", undefined, true, answers.started);
await aPress("Download to, folder chosen, refused", "download-to", "row", ["/home/ubuntu/shots"], true, answers.no_download_folder);
await aPress("Download all to, refused before the panel", "download-all-to", "job:1", undefined, false, answers.not_connected);
await aPress("Download all", "download-all", "job:1", undefined, false, answers.started);
await aPress("Download", "download", "row", ["/home/ubuntu/shots"], false, answers.started);
globalThis.went.length = 0;
told(answers.chosen);
console.log("a folder told with nothing waiting: went " + where());
"#;

    /// **What `web/transfers.js` imports, standing in**, and the harness that
    /// asks for a download twice over. The document is the least the page needs
    /// — the shape `src/surface/the_queue.rs`'s page harness uses — with one
    /// thing true to a browser that the page's fault turned on: **text set to
    /// nothing reads as nothing**, as `textContent` does for `undefined`.
    const THE_QUESTIONS_STAND_INS: [(&str, &str); 9] = [
        ("package.json", r#"{ "type": "module" }"#),
        ("event-route.js", r#"export function whenTold() {}"#),
        (
            "the-rail.js",
            r#"export const TRANSFERS = "transfersview";
export const A_SERVER = "machineview";
export function aScreenIsRegistered(screen) { globalThis.screens.set(screen.place, screen); }
export function theCurrentPlaceIs() {}
export function theRowsFigureIs() {}"#,
        ),
        (
            "the-failure-shape.js",
            r#"export const A_COLLAPSED_ROW = "collapsed-row";
export const EXPANDED = "expanded";
export function theFailureShape() { return document.createElement("span"); }"#,
        ),
        (
            "the-underway-shape.js",
            r#"export function theMarkOnARow() { return document.createElement("span"); }
export function theCountdown() { return document.createElement("span"); }"#,
        ),
        ("the-dependency-check.js", r#"export function theCheckIsShownOn() {}"#),
        (
            "settings.js",
            r#"export const TRANSFERS_SECTION = "transfers";
export function theSettingsSectionIsOpened() {}"#,
        ),
        ("the-history.js", r#"export function theHistoryEntryIsOpened() {}"#),
        (
            "harness.js",
            r#"import { readFileSync } from "node:fs";
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
  const element = {
    tag, children: [], parentNode: null, dataset: {}, hidden: false, text: "", type: "", id: "", title: "",
    classList: {
      add: (...more) => more.forEach((one) => names.add(one)),
      remove: (...less) => less.forEach((one) => names.delete(one)),
      toggle: (one, on) => { if (on === undefined ? !names.has(one) : on) names.add(one); else names.delete(one); },
      contains: (one) => names.has(one),
    },
    set className(value) { names.clear(); String(value).split(" ").filter(Boolean).forEach((one) => names.add(one)); },
    get className() { return Array.from(names).join(" "); },
    set textContent(value) { element.children = []; element.text = value === undefined || value === null ? "" : String(value); },
    get textContent() { return element.text + element.children.map((one) => one.textContent || one.text || "").join(""); },
    appendChild(child) {
      if (child.parentNode && child.parentNode.removeChild) child.parentNode.removeChild(child);
      child.parentNode = element;
      element.children.push(child);
      return child;
    },
    removeChild(child) { element.children = element.children.filter((one) => one !== child); child.parentNode = null; return child; },
    replaceChildren(...fresh) { for (const one of element.children) one.parentNode = null; element.children = []; fresh.forEach((one) => element.appendChild(one)); },
    remove() { if (element.parentNode) element.parentNode.removeChild(element); },
    setAttribute() {},
    addEventListener(kind, handler) { const held = listeners.get(kind) || []; held.push(handler); listeners.set(kind, held); },
    click() { for (const handler of listeners.get("click") || []) handler({ target: element, preventDefault() {} }); },
    focus() {},
    contains(other) { let at = other; while (at) { if (at === element) return true; at = at.parentNode; } return false; },
    querySelector(selector) { return everyUnder(element, aTest(selector), [])[0] || null; },
    querySelectorAll(selector) { return everyUnder(element, aTest(selector), []); },
  };
  return element;
}
globalThis.screens = new Map();
const body = anElement("body");
globalThis.document = {
  createElement: anElement,
  createTextNode: (text) => ({ text, children: [] }),
  addEventListener() {},
  body,
};
const answers = JSON.parse(readFileSync("answers.json", "utf8"));
let next = null;
let reply = answers.started;
const sent = [];
globalThis.window = {
  __TAURI__: {
    core: {
      invoke(command, asked) {
        if (command === "a_file_download_is_asked_for") return Promise.resolve(next);
        // **The screen asked again after the answer** never comes back here:
        // what it draws is not this test's.
        if (command === "the_transfers_screen") return new Promise(() => {});
        sent.push(command + " " + JSON.stringify(asked));
        if (command === "the_download_question_was_answered") return Promise.resolve(reply);
        return Promise.resolve(null);
      },
    },
  },
};

const page = await import("./transfers.js");
const host = anElement("div");
body.appendChild(host);
globalThis.screens.get("transfersview").itIsDrawn(host);
const boxes = () => body.children.filter((one) => one.classList && one.classList.contains("modal"));

for (let at = 0; at < 4; at++) {
  next = answers.asks;
  await page.aTransferIsAskedFor("download", "row", ["/home/ubuntu/frame_0001.png"]);
  const box = boxes()[0];
  if (!box) { console.log("no box"); break; }
  const row = box.querySelector("div.mrow");
  if (at === 0) {
    console.log("heading " + box.querySelector("h2").textContent);
    console.log("buttons " + row.children.map((one) => one.tag === "button" ? one.textContent + "|" + one.className : one.className).join(" · "));
  }
  const pressed = row.querySelectorAll("button")[at];
  pressed.click();
  console.log("pressed " + pressed.textContent + " -> " + (sent.pop() || "nothing") + " · boxes after " + boxes().length);
}

const settled = () => new Promise((done) => setTimeout(done, 0));
await settled();
const line = host.querySelector("p.transfers-answer");
console.log("after four answers that started: " + (line.hidden ? "said nothing" : "said " + line.textContent));

next = answers.asks;
reply = answers.no_longer_waiting;
await page.aTransferIsAskedFor("download", "row", ["/home/ubuntu/frame_0001.png"]);
boxes()[0].querySelector("div.mrow").querySelectorAll("button")[0].click();
await settled();
console.log("answered after the request went: " + line.textContent + " · shown " + (line.hidden ? "no" : "yes"));

next = answers.did_not_start;
await page.aTransferIsAskedFor("download", "row", ["/home/ubuntu/frame_0001.png"]);
console.log("did not start: " + line.textContent + " · shown " + (line.hidden ? "no" : "yes"));

next = answers.stopped;
await page.aTransferIsAskedFor("download", "row", ["/home/ubuntu/frame_0001.png"]);
console.log("stopped while counted: shown " + (line.hidden ? "no" : "yes"));

next = answers.nothing_there;
reply = answers.started;
await page.aTransferIsAskedFor("download", "row", ["/home/ubuntu/frame_0001.png"]);
{
  const box = boxes()[0];
  const row = box.querySelector("div.mrow");
  console.log(
    "nothing there: heading " + (box.querySelector("h2") ? "drawn" : "none") +
    " · amber " + box.querySelectorAll("p.amber").map((one) => one.textContent).join("|") +
    " · buttons " + row.children.map((one) => one.tag === "button" ? one.textContent + "|" + one.className : one.className).join(" · ")
  );
  const pressed = row.querySelectorAll("button")[0];
  pressed.click();
  console.log("pressed " + pressed.textContent + " -> " + (sent.pop() || "nothing"));
}
"#,
        ),
    ];

    /// **The Creator's fourteenth notes, note 4, on the page: a dependency
    /// upload's *For* leads its file's name in its own span.** The request is
    /// composed here, finished, and handed to `web/transfers.js` under Node
    /// through the same serialisation the screen's command replies with: the
    /// name is `span.rqlead` reading *For*, a space, and the file's name in
    /// the mono face — the drawing's own markup. A job's download, whose name
    /// starts with its value, takes no lead. `Design/Transfers § The
    /// dependency check's own transfers`.
    ///
    /// **What it bites on**: *For* in a plain span, which reads as one with
    /// the name, or a lead put on a name that has none. **Skipped, and says
    /// so, where no Node is installed.**
    #[test]
    fn fourteenth_notes_4_a_dependency_uploads_for_leads_its_files_name_on_the_page() {
        let mut dependencies =
            a_request(TheKind::Dependencies, WhatItIsOf::ABlend { file: "/home/ubuntu/kitchen_v14.blend".into() });
        dependencies.stands = WhereTheRequestStands::Finished { at: "04:19".into() };
        let mut download = a_request(TheKind::Download, WhatItIsOf::AJob { name: "Hallway — final".into(), job: "j".into() });
        download.id = "two".into();
        let screen = the_screen_from(&known(vec![dependencies, download]), None);
        let leads: Vec<(&str, bool)> = screen.requests.iter().map(|one| (one.tag_class, one.name_leads)).collect();
        assert_eq!(leads.iter().filter(|(_, leads)| *leads).collect::<Vec<_>>(), vec![&("dep", true)], "{leads:?}");
        let dep = leads.iter().position(|(class, _)| *class == "dep").expect("the dependency upload");
        let answers = serde_json::to_string(&screen).expect("the screen serialises");

        let folder = std::env::temp_dir().join(format!("smoozy-transfers-for-leads-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a scratch folder");
        let web = Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        for page in ["transfers.js", "the-value-drawn.js"] {
            std::fs::copy(web.join(page), folder.join(page)).expect("the page is copied");
        }
        let mut the_document = "";
        for (name, text) in THE_QUESTIONS_STAND_INS {
            if name == "harness.js" {
                the_document = text.split("const answers = JSON.parse").next().expect("the document");
            } else {
                std::fs::write(folder.join(name), text).expect("a stand-in is written");
            }
        }
        let body = r#"const screenSaid = JSON.parse(readFileSync("answers.json", "utf8"));
globalThis.window = { __TAURI__: { core: { invoke(command) {
  return Promise.resolve(command === "the_transfers_screen" ? screenSaid : null);
} } } };
await import("./transfers.js");
const host = anElement("div");
body.appendChild(host);
const transfers = globalThis.screens.get("transfersview");
transfers.itIsDrawn(host);
transfers.itIsCurrentNow();
for (let at = 0; at < 8; at += 1) await new Promise((go) => setImmediate(go));
const names = host.querySelectorAll("span.rqn").map((name) => name.children.map((one) =>
  one.tag ? one.tag + (one.className ? "." + one.className.split(" ").sort().join(".") : "") + " " + one.textContent : "text " + JSON.stringify(one.text)));
console.log(JSON.stringify(names));
"#;
        std::fs::write(folder.join("harness.js"), format!("{the_document}{body}")).expect("the harness");
        std::fs::write(folder.join("answers.json"), answers).expect("the answers are written");
        let ran = std::process::Command::new("node").arg(folder.join("harness.js")).current_dir(&folder).output();
        let _ = std::fs::remove_dir_all(&folder);
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the request's name was not drawn");
            return;
        };
        assert!(ran.status.success(), "the page did not run: {}", String::from_utf8_lossy(&ran.stderr));
        let got: serde_json::Value =
            serde_json::from_str(String::from_utf8_lossy(&ran.stdout).trim()).expect("the harness prints one answer");
        let for_ = Prose::new(words::FOR).as_str().to_string();
        assert_eq!(
            got[dep],
            serde_json::json!([format!("span.rqlead {for_}"), "text \" \"", "span.mono kitchen_v14.blend"])
        );
        // **A name that starts with its value takes no lead.**
        let download = &got[1 - dep];
        assert_eq!(download[0], "span.mono Hallway — final", "{got}");
        assert!(!download.to_string().contains("rqlead"), "{got}");
        // **The drawing draws the same markup.**
        let drawing = std::fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../Preparation/Design/Mockup/Smoozy.html"),
        )
        .expect("the drawing is readable");
        assert!(drawing.contains(&format!(
            "<span class=\"rqn\"><span class=\"rqlead\">{for_}</span> <span class=\"mono\">kitchen_v14.blend</span></span>"
        )));
    }
}

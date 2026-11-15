//! **An upload, run: from the files the user picked to published bytes on the
//! machine.** Built at step 10c piece 4, beside `src/downloading.rs` and in its
//! shape.
//!
//! **Nothing here decides what `File Browser` decides.** What an upload request
//! is, how the already-exists walk reads the machine, what an answer does to
//! the enumeration, what a unit's publication intent is, how the source is
//! watched while it goes up and what a publication writes into its neighbours
//! are all that module's, in `App/File Browser/`, built and met against a real
//! file-transfer server at step 5. **The transfer itself is `Server protocol
//! and transfers` §7's loop taken entire.** What is here is the order the acts
//! are taken in, the sessions they ride, and the one desktop-side admission
//! point each file is admitted at.
//!
//! # The order, and it is the plan's
//!
//! 2. **The enumeration, on the user's own computer** — every file inside every
//!    folder the user picked, names read as stored on disk (`File Browser` N2,
//!    `File Browser` X1's local side).
//! 3. **`File Browser` U1's write**: the request is durable before the walk
//!    and before any byte moves.
//! 4. **`File Browser` X1's walk**, on the connection's reading session, one
//!    directory reading per receiving folder.
//! 5. **`File Browser` U4's answer**, where something collided, written
//!    durably before any transfer is arranged.
//! 6. **The landing folders**, made plainly (`File Browser` N3), on the reading
//!    session.
//! 7. **Up to `File Browser` O1's four files at once**, each channel taking
//!    the next when its own ends (step 10c's eleventh-pass fix piece 4):
//!    `File Browser` N2 and `File Browser` N4's pre-flight, then the
//!    desktop point's admission, a transfer's channel from the server's own
//!    point, the byte loop under `File Browser` K8's watch and `File Browser`
//!    K9's gate, the
//!    unit's outcome written, and `File Browser` X7a's writes.
//!
//! # The clause this file makes true or false, and it is the whole risk of it
//!
//! **R17.** At a low upload cap the admissible count is one, and the guarantee
//! there is turn-taking between the upward classes with work waiting. **An
//! upload that took the single slot and held it for its whole request would
//! starve every other upward class behind it** — a submission, a dependency
//! check's own sends — **and the point would still look as though it works**:
//! one channel moving, the cap held, nothing failing, and R13 making every
//! refusal behind it silent.
//!
//! **So the slot is asked for per file and given back at that file's ending**,
//! exactly as `src/downloading.rs` does, and the draw is a local of the call
//! that admitted it. **The transfer's seat at the server's own point is the
//! same**: it is asked for per file and given back when the file ends, so at a
//! machine's limit of one a download waiting for that seat is seated between
//! two of this upload's files rather than after all of them.
//!
//! **What this file cannot keep alone, found by building it.** The point's
//! turn-taking is between the classes a caller *names* as having work waiting,
//! and **nothing tells an upload which other classes are waiting**: the point
//! keeps no register of them, and each caller knows only its own queue. So this
//! file names its own class, and turn-taking between an upload and a
//! submission holds only where the submission asks while the slot is free —
//! between two of the upload's files. **Today no other upward class runs in the
//! application**, so nothing is starved; the first that does (step 10c piece
//! 6's submission) meets it, and the repair is a register of waiting classes at
//! the point itself, which is `src/scheduling.rs`'s.
//!
//! **What R17 is not about, found by building it.** Uploads and downloads draw
//! on two budgets (R1), so at the desktop point an upload holding the upward
//! slot cannot starve a download at all. **Where the two do meet is the
//! server's own point**: at a limit of one, every transfer in either direction
//! rides the one additional connection one at a time, and that seat is what a
//! whole-request hold would take from a download. Both are driven by name in
//! this crate's tests.
//!
//! # What is not done here, said rather than left to be found
//!
//! - ~~**One file moves at a time.** `File Browser` O1's pool is four channels
//!   per server and it is a ceiling; this request fills one of them.~~
//!   **Up to four files move at once**, since step 10c's eleventh-pass fix
//!   piece 4 (2026-09-30) — see [`the_upload_runs`]. Until then a thousand
//!   small files took 46 minutes at a round trip of 0.2 s.
//! - ~~**Nothing reports progress while a file moves.**~~ **The byte loop's
//!   reports reach the Transfers row** (`Server protocol and transfers` G4,
//!   `File Browser` U11) through the run's register,
//!   `crate::downloading::WhatIsMovingNow`, since step 10c's transfer wave
//!   piece 5 (2026-09-29). Until then they went to nobody and a 4 GB upload's
//!   row read *0 of 1* until the file was across.
//! - ~~**A transfer that was interrupted is left for a later carry-on**, which
//!   is `File Browser` §9's and not built into the running application.~~
//!   **A file the line cuts carries on across a repair** (`File Browser` K1,
//!   K2, K4 and K10), with the same record handed back to the loop, since step
//!   10c's transfer wave piece 6 (2026-09-29) — see
//!   [`carried_across_repairs`]. Until then a one-minute drop ended the
//!   request with the line healthy again a moment later.
//! - **The warnings of `File Browser` §8 are not read**: the records they read
//!   have no reader in the running application.
//!
//! # How a clause marker is written in this file
//!
//! A marker with no document beside it means a clause of this crate's own plan,
//! `Preparation/Architecture/Desktop app and delivery.md`. **Every marker of
//! another document is written out with that document at its own site and this
//! file keeps no header list**, which is the shape `src/downloading.rs` took:
//! `File Browser` collides with this plan on nearly every letter.

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};

use smoozy_file_browser::already_exists::{
    self, components, is_inside, paths_are_equal, AComparison, Arriving, CollisionAnswer,
    TheAnswer, TheQuestionStopped,
};
use smoozy_file_browser::continuation::{TheSourceUnderWatch, TheSourcesPublicationGate};
use smoozy_file_browser::keep_both;
use smoozy_file_browser::listing::{
    CompleteListing, DirectoryAnswer, EntryName, Lease, ReadsADirectory,
};
use smoozy_file_browser::partial::{
    would_land_on_the_reserved_folder, THE_RESERVED_STAGING_FOLDER,
};
use smoozy_file_browser::permission::WhatSmoozyMet;
use smoozy_file_browser::pool::THE_UPLOAD_DIRECTIONS_CEILING;
use smoozy_file_browser::request::{
    self, apply_the_collision_answer, arrange_a_transfer, intent_for, Disposition,
    EnumerationEntry, KeepsTheRequest, TheReasonLine, UploadRequest,
};
use smoozy_file_browser::uploading::{
    check_the_length, check_the_name_can_be_carried, what_to_do_with, MakesTheLandingFolders,
    TheLengthBound, WhatTheMachineSaidAboutAFolder, WhatToDoWithALandingFolder,
};
use smoozy_file_browser::wiring::TheReadingChannel;
use smoozy_server_protocol::subsystem::client::Subsystem;
use smoozy_server_protocol::subsystem::SubsystemFailure;
use smoozy_server_protocol::transfers::moving::{
    carry_on_under_a_publication_gate_asking_the_machine,
    carry_out_a_fresh_file_under_a_publication_gate, carry_out_under_a_publication_gate,
    KeepsTheRecord, LocalFailure, TheMachineDigestsAStagedPart, TheUsersOwnComputer, Transfer,
};
use smoozy_server_protocol::transfers::publication::Intent;
use smoozy_server_protocol::transfers::{Direction, Ending, TransferRecord};
use smoozy_state::identities::{Identity, IdentityKind};
use smoozy_state::outcomes::UnitOutcome;

use crate::downloading::{
    a_part_is_abandoned, every_one_of, the_line_went_down, the_next_waiting,
    the_parts_a_new_request_may_take_up, the_record_is_carried_on, the_stop_lands_or,
    the_user_stopped_this_ending, what_went_down, ItWaitsForItsOwnConnection,
    while_the_line_is_down, TheNextFile, TheRowIsTold, TheRunsTransfers, TheStopHoldsTheDraw,
    WhatWentDown, THE_INACTIVITY_DEADLINE, THE_USERS_STOP,
};
use crate::record::upload_requests::UploadRequests;
use crate::scheduling::TheChannelsDraw;
use crate::servers::sessions::{AHeldConnection, ALentChannel, NotLent, ThisServersSessions};
use crate::transfers::ThePlatformFileAdapter;

/// **How long an upload waits before asking the desktop point again**, where
/// the point had no room for its next file.
///
/// **R13: a refusal is a reason to open fewer and never a reason to fail the
/// transfer**, so the file is offered again rather than failed. The point has
/// no way to call a caller back, so the caller asks again; **the figure is this
/// build's and no promise rests on it** — it is short beside a file's own
/// transfer and long beside a decision, which is all it has to be.
pub(crate) const THE_PAUSE_BEFORE_ASKING_AGAIN: Duration = Duration::from_millis(100);

/// The one lease the reading session is read on here. **`File Browser` L3a's
/// lease change cannot happen inside one of these readings**: the server's
/// point runs one reading to its end before it lets a command close the session.
const ONE_READING: Lease = Lease(1);

// ---------------------------------------------------------------------------
// What the upload is given.
// ---------------------------------------------------------------------------

/// **The one local record, as an upload reaches it.**
///
/// `State, identity and recovery` §1.3 rule 1's one writer per durable store
/// is why this is a door rather than a connection: **every write an upload
/// makes — the request, each transfer record, each unit's outcome — goes
/// through the application's own record, one short act at a time**, and never
/// through a second connection to the same file. The running application's
/// door takes the application's own lock for each act and lets it go at once.
pub trait TheRecordAnUploadWrites: Send + Sync {
    /// Runs `act` against the record, and says whether the record was there to
    /// run it against.
    fn with_the_record(&self, act: &mut dyn FnMut(&rusqlite::Connection)) -> bool;

    /// **A unit ended and it is written down** — the same moment, and the same
    /// one event, as `crate::downloading::TheRecordADownloadWrites::a_transfer_moved`:
    /// uploads draw on the same Transfers page, so a page that is showing asks
    /// again (`Desktop app and delivery` P7 and P8). A door with nobody to tell
    /// does nothing.
    fn a_transfer_moved(&self) {}

    /// **Bytes moved inside a file, and nothing durable was written** —
    /// `Server protocol and transfers` G4's report reached the run's register.
    /// Step 10c's transfer wave piece 5. **The page is told the same way**, by
    /// the one event, which carries nothing (P7, P8): it asks, and the row it
    /// composes reads the register. Called no more often than
    /// [`crate::downloading::THE_PAGE_IS_TOLD_AT_MOST_EVERY`]. A door that
    /// tells the page anything tells it this too, which is the default.
    fn a_transfer_is_moving(&self) {
        self.a_transfer_moved();
    }

    /// **The machine's last queue reading the run holds for `server`** —
    /// `Queue & History` C6's period-held record, read and never changed —
    /// which a warning composed while the act runs reads to tell a rendering
    /// job from a waiting one (`File Browser` W5a). **A door with no run behind
    /// it holds none**, which is the default, and the warning then reads every
    /// job with no learned ending as waiting. Added 2026-10-02, step 10c's
    /// Files' owed acts piece 1.
    fn the_last_queue_reading(
        &self,
        server: &str,
    ) -> Option<smoozy_queue_history::acting::adoption::TheMachinesWorkingState> {
        let _ = server;
        None
    }
}

/// **`File Browser` X7a's writes, performed where the running application
/// keeps the records they reach.** The composition is that module's own
/// `publication::at_this_publication`; the door is the caller's, because the
/// stores it reaches are not this file's.
pub trait WhenAFileIsPublished: Send + Sync {
    fn a_file_was_published(&self, landing_path: &[u8], outcome: &UnitOutcome);

    /// **`Dependency Upload & Remap` G17b: a file begins going up to this machine**,
    /// to this landing place. A reading of the check that rests on that place,
    /// taken while it does, is not kept.
    fn a_file_begins_going_up(&self, _landing: &[u8]) {}

    /// **It has ended, however it ended.**
    fn a_file_stopped_going_up(&self, _landing: &[u8]) {}
}

/// **What the user asked to upload, and where.**
#[derive(Debug, Clone)]
pub struct WhatIsBeingUploaded {
    /// `File Browser` N1: the transfer is bound to the server row it was
    /// arranged against and to nothing else.
    pub server_row: Identity,
    /// The installation identity it was arranged under, where one is known.
    pub installation: Option<Identity>,
    /// **The destination folder — where the user was browsing.** Bytes.
    pub destination: Vec<u8>,
    /// What the picker or the drop handed over, and nothing else
    /// (`File Browser` H3a).
    pub chosen: Vec<PathBuf>,
    /// The application run this upload is opened in.
    pub run: Identity,
}

/// **Why an upload did not start.** None of these moved a byte.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhyTheUploadDidNotStart {
    /// Legacy closed-act result; free-software upload composition does not raise it.
    SendingFilesUpIsClosed,
    /// Nothing was picked, or everything picked was an empty folder.
    ThereIsNothingToSend,
    /// **A file or folder on the user's own computer could not be read**, so
    /// the enumeration is not complete and `File Browser` U1 forbids a request
    /// written from part of one.
    SomethingPickedCouldNotBeRead(PathBuf),
    /// **The local record would not take the request**, so `File Browser` U1's
    /// durable half could not be written and nothing may move.
    TheRecordWouldNotTakeIt,
    /// **The server's point lent nothing** — no connection is live, or it
    /// ended, or the machine allows no session.
    TheServerLentNothing(NotLent),
    /// **`File Browser` X2: a receiving folder could not be read**, so the
    /// already-exists count would be wrong in the quiet direction. The upload
    /// does not start until the user has an answer.
    AFolderCouldNotBeRead { folder: Vec<u8>, met: WhatSmoozyMet },
}

/// **An upload whose request is written and whose walk is done** — everything
/// up to `File Browser` U4's question.
#[derive(Debug, Clone)]
pub struct AnUploadReadyToRun {
    pub request: UploadRequest,
    /// **`File Browser` X1's answer**, kept for the question and for each
    /// unit's publication intent.
    pub the_walk: TheAnswer,
    /// Every arriving thing, folders among them, in the enumeration's order.
    pub arriving: Vec<Arriving>,
    /// **`File Browser` U9b's reason line**, where an earlier request left the
    /// files the walk found.
    pub the_reason_line: Option<TheReasonLine>,
    /// **The jobs `File Browser` W8's telling named**, written by
    /// [`the_telling_stands_first`] before any file moves, and read at the
    /// run's end to finish or remove each job's mark (`File Browser` W9).
    /// Empty where the user pushed past no live-job warning. Step 10c's Files'
    /// owed acts piece 2.
    pub the_jobs_told: Vec<smoozy_file_browser::warnings::AJobNamed>,
    /// **Where Smoozy's own program sits on the server** — the package area
    /// of an installation the server row confirmed — so a file carried on
    /// across a repair has the part the server holds checked there (`Server
    /// protocol and transfers` S12b). **`None` reads the part back over the
    /// line, as before.** Uploads carry on, 2026-11-14.
    pub the_servers_program: Option<String>,
}

impl AnUploadReadyToRun {
    /// **Whether `File Browser` U4's question is asked at all.** It is asked
    /// where something collides and nowhere else.
    pub fn something_collides(&self) -> bool {
        self.the_walk.colliding > 0
    }

    /// How many files this request will send, once its dispositions are
    /// written — `File Browser` U11's count, over the units whose disposition
    /// is to send.
    pub fn the_files_it_sends(&self) -> usize {
        self.request
            .entries
            .iter()
            .filter(|entry| {
                entry.outcome.is_none()
                    && entry
                        .disposition
                        .as_ref()
                        .is_some_and(Disposition::a_transfer_is_arranged)
            })
            .count()
    }
}

/// **What the answer came to.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheAnswerLeft {
    /// The dispositions are written and the files go.
    ItGoes,
    /// **Stop.** Nothing is sent, and the request's ending is *stopped by the
    /// user* (`File Browser` U1).
    TheUserStoppedIt,
    /// The answer could not be written down, so nothing may move.
    TheRecordWouldNotTakeIt,
    /// The folders a Keep-both name is composed against could not be read.
    TheServerLentNothing(NotLent),
}

/// **What the upload came to, file by file, counted the way `File Browser`
/// U11 counts it.**
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct WhatTheUploadCameTo {
    /// Files at their final names.
    pub uploaded: usize,
    /// **`File Browser` U11: skipped units are reported separately, as what
    /// Skip existing did.**
    pub skipped: usize,
    /// **Units that ended as failures, by their landing paths** — `File
    /// Browser` N5: the report names the file.
    pub failed: Vec<Vec<u8>>,
    /// **Units whose transfer ended interrupted or stalled.** Their records
    /// stand for a later carry-on; nothing about them failed.
    pub did_not_finish: usize,
    /// **The failed units whose write the machine refused for permission**,
    /// by landing path, with what the server said — also in `failed`.
    /// `File Browser` P1 and `File Browser` P6a: a refused write on an upload is a failure
    /// and is reported as it stands, never *did not finish*.
    pub refused: Vec<(Vec<u8>, String)>,
}

/// **What the server said when it refused an upload's write, held for the
/// run** — keyed by the request's record identity and the landing path.
///
/// **Run-held, because the record has no column for it** and no record
/// version is this piece's (the fix map gives the next one to the note's two
/// fields). The unit's durable outcome is *failed*, which is what a later
/// sitting reads; the machine's own words are for the Transfers row's *What
/// the server said* while this run lasts, and a later sitting's row says the
/// file failed without them.
pub fn the_refusals_this_run(
) -> &'static std::sync::Mutex<std::collections::HashMap<(String, Vec<u8>), String>> {
    static HELD: std::sync::OnceLock<
        std::sync::Mutex<std::collections::HashMap<(String, Vec<u8>), String>>,
    > = std::sync::OnceLock::new();
    HELD.get_or_init(Default::default)
}

/// What the server said for one refused file of one request, where this run
/// holds it.
pub fn what_the_server_said_refusing(request: &Identity, landing: &[u8]) -> Option<String> {
    the_refusals_this_run()
        .lock()
        .ok()?
        .get(&(request.as_text(), landing.to_vec()))
        .cloned()
}

/// **The files whose landing folder could not be made, held for the run** —
/// keyed as [`the_refusals_this_run`] is, and for the same reason: the record
/// holds only *failed*. `Design/Transfers § A file that failed`, the paragraph
/// of 2026-11-11 (the four questions, batch 2, fix 1).
fn the_folders_not_made_this_run() -> &'static Mutex<std::collections::HashSet<(String, Vec<u8>)>> {
    static HELD: std::sync::OnceLock<Mutex<std::collections::HashSet<(String, Vec<u8>)>>> =
        std::sync::OnceLock::new();
    HELD.get_or_init(Default::default)
}

/// **One file of one request failed because its landing folder was not made**
/// (`File Browser` N3's failed folder).
pub fn a_folder_was_not_made_for(request: &Identity, landing: &[u8]) {
    if let Ok(mut held) = the_folders_not_made_this_run().lock() {
        held.insert((request.as_text(), landing.to_vec()));
    }
}

/// **Why one upload file failed, as far as Smoozy knows** — `Design/Transfers
/// § A file that failed`, the paragraph of 2026-11-11. **The name reasons are
/// `File Browser` N2, `File Browser` N4 and `File Browser` X12's pre-flight checks read again from the
/// landing path**, so a later sitting draws them as this one does; the folder
/// is this run's alone. A refusal and a change have their own rows and are
/// read before this.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhyTheUploadFailed {
    /// `File Browser`'s upload-refusal rule: the server refused the write and
    /// a reading taken at that failure showed no space left where it lands.
    ServerDiskFull,
    /// `File Browser` N2: the landing path is not text.
    TheNameIsNotValid,
    /// `File Browser` N4: one name is over the bound.
    TheNameIsTooLong,
    /// `File Browser` N4: the whole path is over the bound.
    ThePathIsTooLong,
    /// `File Browser` X12: the name is the staging folder's own.
    TheNameIsReserved,
    /// `File Browser` N3: its folder could not be made, in this run.
    ItsFolderWasNotMade,
    /// **The file could not be read on this computer at the failure** — a
    /// drive unplugged under it, say (the Rough what-if answers, share 2, 188).
    ItCouldNotBeReadHere,
    /// Nothing Smoozy holds says why.
    NoReasonIsKnown,
}

impl WhyTheUploadFailed {
    /// **Whether the same file sent again could go**: a name the server
    /// cannot take fails the same way every time.
    pub fn a_retry_could_help(self) -> bool {
        matches!(
            self,
            WhyTheUploadFailed::ItsFolderWasNotMade
                | WhyTheUploadFailed::NoReasonIsKnown
                | WhyTheUploadFailed::ServerDiskFull
                | WhyTheUploadFailed::ItCouldNotBeReadHere
        )
    }
}

/// **The files of this run whose upload failed where their source could not
/// be read on this computer**, by request and landing path — held for this
/// run only, as the full-disk reading is (the Rough what-if answers, share 2,
/// 188).
fn the_unreadable_sources() -> &'static Mutex<std::collections::HashSet<(String, Vec<u8>)>> {
    static UNREAD: std::sync::OnceLock<Mutex<std::collections::HashSet<(String, Vec<u8>)>>> =
        std::sync::OnceLock::new();
    UNREAD.get_or_init(Default::default)
}

/// **One read of the source at the failure**: a file this computer cannot open
/// and read from is the reason, whatever the transfer's ending says.
fn the_source_could_not_be_read_here(source: &str) -> bool {
    use std::io::Read;
    std::fs::File::open(source).and_then(|mut file| file.read(&mut [0u8; 1])).is_err()
}

fn the_source_was_unreadable_for(request: &str, landing: &[u8], unread: bool) {
    if let Ok(mut held) = the_unreadable_sources().lock() {
        if unread {
            held.insert((request.to_string(), landing.to_vec()));
        } else {
            held.remove(&(request.to_string(), landing.to_vec()));
        }
    }
}

/// **The files of this run whose upload failed on a full server disk**, by
/// request and landing path — held for this run only, as a refusal's words
/// are ([`the_refusals_this_run`]); a later sitting draws the file as
/// *Upload failed.* with no reason.
fn the_full_upload_destinations() -> &'static Mutex<std::collections::HashSet<(String, Vec<u8>)>> {
    static FULL: std::sync::OnceLock<Mutex<std::collections::HashSet<(String, Vec<u8>)>>> =
        std::sync::OnceLock::new();
    FULL.get_or_init(Default::default)
}

/// One file's failure-time reading, kept or let go: a file that later goes up
/// clears it.
fn the_server_disk_was_full_for(request: &str, landing: &[u8], full: bool) {
    if let Ok(mut held) = the_full_upload_destinations().lock() {
        if full {
            held.insert((request.to_string(), landing.to_vec()));
        } else {
            held.remove(&(request.to_string(), landing.to_vec()));
        }
    }
}

/// The reason for one failed file of one request.
pub fn why_this_upload_failed(request: &Identity, landing: &[u8]) -> WhyTheUploadFailed {
    if the_full_upload_destinations().lock().is_ok_and(|held| held.contains(&(request.as_text(), landing.to_vec()))) {
        return WhyTheUploadFailed::ServerDiskFull;
    }
    if the_unreadable_sources().lock().is_ok_and(|held| held.contains(&(request.as_text(), landing.to_vec()))) {
        return WhyTheUploadFailed::ItCouldNotBeReadHere;
    }
    if std::str::from_utf8(landing).is_err() {
        return WhyTheUploadFailed::TheNameIsNotValid;
    }
    match check_the_length(landing) {
        TheLengthBound::AComponentIsTooLong { .. } => return WhyTheUploadFailed::TheNameIsTooLong,
        TheLengthBound::ThePathIsTooLong { .. } => return WhyTheUploadFailed::ThePathIsTooLong,
        TheLengthBound::Inside => {}
    }
    if would_land_on_the_reserved_folder(landing) {
        return WhyTheUploadFailed::TheNameIsReserved;
    }
    let not_made = the_folders_not_made_this_run()
        .lock()
        .map(|held| held.contains(&(request.as_text(), landing.to_vec())))
        .unwrap_or(false);
    if not_made {
        WhyTheUploadFailed::ItsFolderWasNotMade
    } else {
        WhyTheUploadFailed::NoReasonIsKnown
    }
}

/// **The upload requests this run is still holding before their answer** —
/// written down (`File Browser` U1), and their walk under way or their
/// question open (`File Browser` U4). Keyed by the request's record identity. Added
/// 2026-10-02, step 10c's check-before-0.1.7 fix piece 1. **An answer that goes
/// keeps the request here until the run's register holds it**
/// ([`the_hold_passes_to_the_run`]), so a Stop always finds it in one or the
/// other; its dispositions are written by then, so its row does not read from
/// here.
///
/// **Run-held, because what it says is about this run's activity** (`File
/// Browser` U7) and not about the record: a request nothing has decided reads
/// as under way while it is here, and as having ended without sending anything
/// once it is not — the way a manual download nothing has decided reads under
/// way only while this run holds it (`crate::surface::transfers`, step 10c's
/// eleventh-pass fix piece 1). **The case it was written for** is the
/// Hyperstack check before 0.1.7: a file dragged in while the connection was
/// gone was written down, its walk was lent nothing, and its row read *0 files
/// into /home/ubuntu · done* at once.
fn the_requests_held_before_their_answer() -> &'static Mutex<std::collections::HashSet<String>> {
    static HELD: std::sync::OnceLock<Mutex<std::collections::HashSet<String>>> =
        std::sync::OnceLock::new();
    HELD.get_or_init(Default::default)
}

/// **Whether this run is still holding this request before its answer** —
/// its walk under way or its question open.
pub fn this_run_holds_it_before_its_answer(request: &Identity) -> bool {
    the_requests_held_before_their_answer()
        .lock()
        .map(|held| held.contains(&request.as_text()))
        .unwrap_or(false)
}

/// One request held before its answer, let go when this is dropped unless it
/// is kept for its question.
struct HeldBeforeItsAnswer(Option<String>);

impl HeldBeforeItsAnswer {
    fn from(request: &Identity) -> Self {
        let key = request.as_text();
        if let Ok(mut held) = the_requests_held_before_their_answer().lock() {
            held.insert(key.clone());
        }
        HeldBeforeItsAnswer(Some(key))
    }

    /// **Kept for its question**: let go when the answer is applied.
    fn kept_for_its_answer(mut self) {
        self.0 = None;
    }
}

impl Drop for HeldBeforeItsAnswer {
    fn drop(&mut self) {
        if let Some(key) = self.0.take() {
            if let Ok(mut held) = the_requests_held_before_their_answer().lock() {
                held.remove(&key);
            }
        }
    }
}

/// Let go of a request held before its answer.
fn its_answer_is_applied(request: &Identity) {
    drop(HeldBeforeItsAnswer(Some(request.as_text())));
}

/// **The uploads the user stopped while this run held them before their
/// answer** — `File Browser` U6 before the request is under way. Run-held, and
/// let go of when the stop is written. Sol's fresh look on the twenty-eighth
/// notes, piece 12: until then a Stop pressed during the walk, or while the
/// question was open, reached nothing, and the upload went on once the walk
/// ended or the question was answered.
fn the_uploads_stopped_before_they_went() -> &'static Mutex<std::collections::HashSet<String>> {
    static STOPPED: std::sync::OnceLock<Mutex<std::collections::HashSet<String>>> = std::sync::OnceLock::new();
    STOPPED.get_or_init(Default::default)
}

/// **The user's Stop, on an upload this run holds before its answer** — its
/// walk under way, its question open, or its answer applied and its run not yet
/// begun. The walk ends at the press and the stop is written as the request's
/// answer, whatever answer comes after. Answers whether the request was held.
///
/// **The look at the hold and the mark of the press are one step**, under the
/// hold's own lock (Sol's one look at the fixes after 0.4.0's second try). They
/// were two: where the comparison ended and the upload went under way between
/// them, the press was marked where nothing read it again, and the files went
/// on. The hold is let go under the same lock as the run takes the request
/// ([`the_hold_passes_to_the_run`]), so a press marked here is read there, and
/// a press that finds no hold finds the request in the run's register.
pub fn an_upload_held_before_its_answer_is_stopped(request: &str) -> bool {
    let held = the_requests_held_before_their_answer()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    if !held.contains(request) {
        return false;
    }
    a_test_may_hold_this_moment(BETWEEN_THE_LOOK_AND_THE_MARK);
    the_uploads_stopped_before_they_went()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .insert(request.to_string());
    true
}

/// **The hold passes to the run's register**, once the run holds the request
/// as under way: the hold is let go and a Stop pressed while it stood is
/// taken, in one step under the hold's lock (`File Browser` U6). Until then
/// the hold was let go as the answer was applied and the press was read after
/// the run began, so a Stop pressed between the two, or between a Stop's look
/// and its mark, reached nothing (Sol's one look at the fixes after 0.4.0's
/// second try).
pub(crate) fn the_hold_passes_to_the_run(the_run: &TheRunsTransfers, request: &Identity) {
    let key = request.as_text();
    let stopped = {
        let mut held = the_requests_held_before_their_answer()
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        held.remove(&key);
        the_uploads_stopped_before_they_went()
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .remove(&key)
    };
    if stopped {
        the_run.the_user_stops(&key, None);
    }
}

/// **A request whose answer let it go but whose run will not begin** is held no
/// longer, and a Stop marked on it is dropped.
pub(crate) fn it_will_not_go_under_way(request: &Identity) {
    its_answer_is_applied(request);
    the_uploads_stopped_before_they_went()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .remove(&request.as_text());
}

/// **The moment inside a Stop between its look at the hold and its mark** —
/// one a test may hold ([`a_test_may_hold_this_moment`]).
pub(crate) const BETWEEN_THE_LOOK_AND_THE_MARK: &str = "between the look at the hold and the mark";

/// **The moment inside a Stop between its look at the run's register and its
/// look at the hold** — one a test may hold.
pub(crate) const BETWEEN_THE_RUNS_LOOK_AND_THE_HOLDS: &str = "between the run's look and the hold's";

#[cfg(test)]
thread_local! {
    /// The one moment a test on this thread holds, and what it does there.
    static A_TEST_HOLDS: std::cell::RefCell<Option<(&'static str, Box<dyn FnMut()>)>> =
        const { std::cell::RefCell::new(None) };
}

/// **A moment a test may hold**, so it can force what another thread does
/// there. Only this crate's own tests ever hold one; outside them this does
/// nothing.
#[inline]
pub(crate) fn a_test_may_hold_this_moment(moment: &'static str) {
    #[cfg(test)]
    A_TEST_HOLDS.with(|held| {
        if let Some((at, hold)) = held.borrow_mut().as_mut() {
            if *at == moment {
                hold();
            }
        }
    });
    #[cfg(not(test))]
    let _ = moment;
}

/// **Whether the user stopped this upload while this run held it before its
/// answer**, and the stop is not written yet.
pub fn it_was_stopped_before_it_went(request: &str) -> bool {
    the_uploads_stopped_before_they_went()
        .lock()
        .map(|stopped| stopped.contains(request))
        .unwrap_or(false)
}

/// **The stop written as the request's `File Browser` U4 answer, at once** — for
/// an upload whose question is open when the user presses Stop in Transfers, so
/// its row ends there rather than when the question is answered. Nothing is
/// arranged; the row then reads what a request stopped at its question reads.
pub fn the_stop_is_written_before_it_went(
    request: &mut UploadRequest,
    with_the_record: &dyn Fn(&mut dyn FnMut(&rusqlite::Connection)) -> bool,
) -> bool {
    let mut applied = None;
    let wrote = with_the_record(&mut |connection| {
        let mut keeper = UploadRequests::over(connection);
        applied = Some(apply_the_collision_answer(
            request,
            &[],
            CollisionAnswer::Stop,
            &mut std::iter::empty(),
            &mut keeper,
            SystemTime::now(),
        ));
    });
    its_answer_is_applied(&request.record);
    if let Ok(mut stopped) = the_uploads_stopped_before_they_went().lock() {
        stopped.remove(&request.record.as_text());
    }
    wrote && matches!(applied, Some(Ok(request::WhatTheAnswerDid::TheRequestStops)))
}

// ---------------------------------------------------------------------------
// The enumeration, on the user's own computer.
// ---------------------------------------------------------------------------

/// **Every file inside every folder the user pointed at, and every folder on
/// the way** — `File Browser` X1's local side and `File Browser` N2's
/// structure.
///
/// **Names are read as stored on disk and carried as their bytes**
/// (`File Browser` N2); nothing is rewritten, normalised or re-encoded. The
/// order is the picked order, and inside a folder its entries by their bytes, so
/// two runs over one folder enumerate it alike.
pub fn what_was_picked(
    chosen: &[PathBuf],
    destination: &[u8],
) -> Result<(Vec<Arriving>, Vec<EnumerationEntry>), PathBuf> {
    let mut arriving = Vec::new();
    let mut entries = Vec::new();
    for picked in chosen {
        let found = std::fs::metadata(picked).map_err(|_| picked.clone())?;
        let name = EntryName::as_stored_on_disk(the_last_component_of(picked));
        if found.is_dir() {
            let mut seen = Vec::new();
            the_folder_is_walked(
                picked,
                vec![name],
                destination,
                &mut arriving,
                &mut entries,
                &mut seen,
            )?;
        } else {
            let thing = Arriving {
                relative: vec![name],
                is_a_folder: false,
            };
            entries.push(EnumerationEntry::enumerated(
                the_bytes_of(picked),
                thing.landing_path(destination),
                found.len(),
            ));
            arriving.push(thing);
        }
    }
    Ok((arriving, entries))
}

fn the_folder_is_walked(
    folder: &Path,
    relative: Vec<EntryName>,
    destination: &[u8],
    arriving: &mut Vec<Arriving>,
    entries: &mut Vec<EnumerationEntry>,
    seen: &mut Vec<PathBuf>,
) -> Result<(), PathBuf> {
    // **A folder reached twice through a link is walked once**, so a link that
    // points back up the tree cannot make the enumeration endless.
    let canonical = std::fs::canonicalize(folder).map_err(|_| folder.to_path_buf())?;
    if seen.contains(&canonical) {
        return Ok(());
    }
    seen.push(canonical);
    arriving.push(Arriving {
        relative: relative.clone(),
        is_a_folder: true,
    });
    let mut inside: Vec<PathBuf> = std::fs::read_dir(folder)
        .map_err(|_| folder.to_path_buf())?
        .map(|entry| entry.map(|entry| entry.path()))
        .collect::<Result<_, _>>()
        .map_err(|_| folder.to_path_buf())?;
    inside.sort_by_key(|path| the_bytes_of(path));
    for path in inside {
        let found = std::fs::metadata(&path).map_err(|_| path.clone())?;
        let mut beneath = relative.clone();
        beneath.push(EntryName::as_stored_on_disk(the_last_component_of(&path)));
        if found.is_dir() {
            the_folder_is_walked(&path, beneath, destination, arriving, entries, seen)?;
        } else {
            let thing = Arriving {
                relative: beneath,
                is_a_folder: false,
            };
            entries.push(EnumerationEntry::enumerated(
                the_bytes_of(&path),
                thing.landing_path(destination),
                found.len(),
            ));
            arriving.push(thing);
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Up to the question.
// ---------------------------------------------------------------------------

/// **Everything up to `File Browser` U4's question**: the capability read, the
/// enumeration, the request written down, the walk, and `File Browser` U9b's
/// reason line.
pub async fn an_upload_is_prepared<C: AHeldConnection>(
    asked: &WhatIsBeingUploaded,
    sessions: &ThisServersSessions<C>,
    record: &dyn TheRecordAnUploadWrites,
) -> Result<AnUploadReadyToRun, WhyTheUploadDidNotStart> {
    // The request witness is composed before any files are enumerated.
    let open = request::open_an_upload_request()
        .map_err(|_| WhyTheUploadDidNotStart::SendingFilesUpIsClosed)?;

    let (arriving, entries) = what_was_picked(&asked.chosen, &asked.destination)
        .map_err(WhyTheUploadDidNotStart::SomethingPickedCouldNotBeRead)?;
    if entries.is_empty() {
        return Err(WhyTheUploadDidNotStart::ThereIsNothingToSend);
    }

    let identity = Identity::issue(IdentityKind::Operation)
        .map_err(|_| WhyTheUploadDidNotStart::TheRecordWouldNotTakeIt)?;
    let request = UploadRequest::confirmed(
        open,
        identity,
        asked.server_row.clone(),
        asked.installation.clone(),
        asked.destination.clone(),
        entries,
        asked.run.clone(),
    );
    // **`File Browser` U1: written when the user confirms the upload, before
    // any transfer is arranged**, and the enumeration is durable before the
    // first byte moves (`Server protocol and transfers` S8's ordering).
    if !the_request_is_written_down(record, &request) {
        return Err(WhyTheUploadDidNotStart::TheRecordWouldNotTakeIt);
    }
    // **Held from here until its answer** — and, where it goes, until its
    // run holds it — and let go at once wherever it goes no further, so its
    // row never reads as under way for a walk that is not running.
    let held = HeldBeforeItsAnswer::from(&request.record);

    // **`File Browser` X1: both sides are walked before a byte moves**, and the
    // machine's side is one directory reading per receiving folder on the
    // connection's own reading session.
    let destination = asked.destination.clone();
    let walking = arriving.clone();
    let given_up = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let the_walk_is_given_up = Arc::clone(&given_up);
    let stopped_as = request.record.as_text();
    let walk = sessions.a_reading(true, move |subsystem| {
        Box::pin(async move {
            let mut channel = TheWalkEndsWhenGivenUp {
                reading: TheReadingChannel::on(subsystem, ONE_READING),
                given_up: the_walk_is_given_up,
                stopped_as,
            };
            Ok(
                already_exists::walk(&destination, &walking, &mut channel, SystemTime::now())
                    .await,
            )
        })
    });
    // **`File Browser` U6: the user's Stop ends the walk at the press** (Sol's
    // fresh look on the twenty-eighth notes, piece 12). Nothing collides in a
    // walk not taken, so no question is asked, and the answer applied next is
    // the stop ([`the_answer_is_applied`]). The readings already asked of the
    // machine finish there and change nothing.
    //
    // **A Stop pressed while the walk ran is the answer even where the walk
    // ended between two of the wait's looks** (hosted run 43746222478: on a
    // fast machine 2,000 folders were walked inside one look, and the walk
    // stood as though Stop had not been pressed). The walk itself reads no
    // further folder once Stop is pressed ([`TheWalkEndsWhenGivenUp`]).
    let stopped_here = request.record.as_text();
    let Some(walked) = crate::the_found_files_sent::unless_a_stop_is_asked(walk, &mut || {
        it_was_stopped_before_it_went(&stopped_here)
    })
    .await
    .filter(|_| !it_was_stopped_before_it_went(&stopped_here))
    else {
        // **And the walk on the machine reads no further folder** (Sol's second
        // look on piece 12), so the reading session is not held by it.
        given_up.store(true, std::sync::atomic::Ordering::SeqCst);
        held.kept_for_its_answer();
        return Ok(AnUploadReadyToRun {
            request,
            the_walk: TheAnswer { comparisons: Vec::new(), colliding: 0, arriving_files: 0, as_of: SystemTime::now(), readings_taken: 0 },
            arriving,
            the_reason_line: None,
            the_jobs_told: Vec::new(),
            the_servers_program: None,
        });
    };
    let walked = walked.map_err(WhyTheUploadDidNotStart::TheServerLentNothing)?;
    let the_walk = match walked {
        Ok(Ok(answer)) => answer,
        Ok(Err(TheQuestionStopped::AFolderCouldNotBeRead { folder, met })) => {
            return Err(WhyTheUploadDidNotStart::AFolderCouldNotBeRead { folder, met })
        }
        // **The reading session went under during the walk**: the question
        // stops, not the world (`File Browser` X4).
        Err(_) => {
            return Err(WhyTheUploadDidNotStart::TheServerLentNothing(
                NotLent::TheConnectionEnded,
            ))
        }
    };

    // **`File Browser` U9b, composed at the pre-flight from the local record
    // and never from the machine.**
    let occupied: Vec<Vec<u8>> = the_walk
        .comparisons
        .iter()
        .filter(|one| one.what.is_a_collision())
        .map(|one| one.landing_path.clone())
        .collect();
    let mut the_reason_line = None;
    if !occupied.is_empty() {
        record.with_the_record(&mut |connection| {
            if let Ok(earlier) =
                UploadRequests::over(connection).every_request_for(&asked.server_row)
            {
                the_reason_line = request::the_reason_line(
                    &earlier,
                    &asked.run,
                    &asked.server_row,
                    asked.installation.as_ref(),
                    &occupied,
                );
            }
        });
    }

    held.kept_for_its_answer();
    Ok(AnUploadReadyToRun {
        request,
        the_walk,
        arriving,
        the_reason_line,
        the_jobs_told: Vec::new(),
        the_servers_program: None,
    })
}

/// **`File Browser` X1's walk, ended at the user's Stop** — each further
/// receiving folder is answered as not read, so the walk stops at once (X2) and
/// its answer, which nothing waits for, is dropped. Sol's second look on the
/// twenty-eighth notes, piece 12: the walk given up went on reading every
/// receiving folder, and the readings after it waited. **It is given up at the
/// press itself**, the request's Stop read before each folder, and not only
/// when the wait next looks.
struct TheWalkEndsWhenGivenUp<R> {
    reading: R,
    given_up: Arc<std::sync::atomic::AtomicBool>,
    /// The request, as its Stop is kept.
    stopped_as: String,
}

impl<R: already_exists::ReadsAReceivingFolder + Send> already_exists::ReadsAReceivingFolder for TheWalkEndsWhenGivenUp<R> {
    fn read(&mut self, path: &[u8]) -> impl std::future::Future<Output = already_exists::ReceivingFolder> + Send {
        let given_up = self.given_up.load(std::sync::atomic::Ordering::SeqCst)
            || it_was_stopped_before_it_went(&self.stopped_as);
        let read = self.reading.read(path);
        async move {
            if given_up {
                already_exists::ReceivingFolder::CouldNotBeRead(WhatSmoozyMet::Gone)
            } else {
                read.await
            }
        }
    }
}

/// **What an upload's *Replace* says it did to a job's file** — `Queue &
/// History` N21's act intended, from which the mark's verb is read
/// (`Design/Queue & History § The marks`: *the verb is the act's*). Step 10c's
/// Files' owed acts piece 2.
pub const AN_UPLOADS_REPLACE: &str = "replace";

/// **`File Browser` W8 for an upload: the telling stands before the first
/// file moves.** Where the user answered *Replace* over files the live-job
/// warning named jobs for, the request's telling and each job's mark go down in
/// one write ([`UploadRequests::the_telling_is_written`]), every mark reading
/// *may have been disturbed* (`Queue & History` N21); **where that write cannot
/// be made the act is not attempted, naming the telling** (`File Browser` W8,
/// §11's ordered pair). Step 10c's Files' owed acts piece 2.
///
/// **One mark per member each job's set had reached**, naming that member —
/// the file the job depends on, or its own folder where the upload lands
/// inside it — so `File Browser` W9 can finish each from the units that touched it.
///
/// **Nothing is written where nothing was named**: there is no telling where
/// no live-job warning was pushed past, and the act goes ahead as any upload
/// does.
pub fn the_telling_stands_first(
    ready: &mut AnUploadReadyToRun,
    named: &[smoozy_file_browser::warnings::AJobNamed],
    record: &dyn TheRecordAnUploadWrites,
) -> Result<smoozy_file_browser::warnings::TheTellingStoodFirst, smoozy_file_browser::warnings::TheActIsNotAttempted> {
    if named.is_empty() {
        return smoozy_file_browser::warnings::the_act_may_be_attempted(true);
    }
    ready.request.telling = Some(smoozy_file_browser::request::TheTelling {
        the_jobs: named.iter().map(|job| job.job.clone()).collect(),
    });
    let marks: Vec<(Identity, smoozy_queue_history::record::ADisturbance)> = named
        .iter()
        .flat_map(|job| {
            job.the_members_reached.iter().map(|member| {
                (
                    job.job.clone(),
                    smoozy_queue_history::record::ADisturbance {
                        the_member: String::from_utf8_lossy(member).into_owned(),
                        the_act_intended: AN_UPLOADS_REPLACE.to_string(),
                        reading: smoozy_queue_history::record::DisturbanceReading::MayHaveBeenDisturbed,
                        how_many: None,
                    },
                )
            })
        })
        .collect();
    let mut written = false;
    let request = &ready.request;
    let reached = record.with_the_record(&mut |connection| {
        written = UploadRequests::over(connection)
            .the_telling_is_written(request, &marks)
            .is_ok();
    });
    let stood = smoozy_file_browser::warnings::the_act_may_be_attempted(reached && written);
    if stood.is_ok() {
        ready.the_jobs_told = named.to_vec();
    } else {
        ready.request.telling = None;
    }
    stood
}

/// **`File Browser` W9 for an upload: each job's mark is finished from the
/// units that touched its own members.** A member a unit published over reads
/// *disturbed*; a member whose every unit ended without taking effect has its
/// mark removed; a member with a unit whose ending was never learned keeps
/// *may have been disturbed*. **The composition is `File Browser`'s own**
/// ([`smoozy_file_browser::warnings::what_is_written_against`]), asked one
/// member at a time so each mark is finished from its own units. Step 10c's
/// Files' owed acts piece 2.
///
/// **A run that ends before this is reached leaves every mark reading *may
/// have been disturbed***, which is the truth: the act's outcome was never
/// learned.
pub fn the_marks_are_finished(
    request: &UploadRequest,
    told: &[smoozy_file_browser::warnings::AJobNamed],
    record: &dyn TheRecordAnUploadWrites,
) {
    let units: Vec<(Vec<u8>, Option<UnitOutcome>)> = request
        .entries
        .iter()
        .map(|entry| (entry.landing_path.clone(), entry.outcome.clone()))
        .collect();
    crate::managing_files::the_marks_are_finished(record, told, AN_UPLOADS_REPLACE, &units);
}

/// **`File Browser` U4: the answer, written durably before any transfer whose
/// arrangement depends on it is arranged** — or, where nothing collided, the
/// dispositions alone.
///
/// **Where nothing collides, no question is asked and no answer is recorded.**
/// `File Browser` U4's answer is the user's, and a request the user was never
/// asked about
/// holds none; every entry takes *to send*, which is what every answer makes of
/// a unit that does not collide.
pub async fn the_answer_is_applied<C: AHeldConnection>(
    ready: &mut AnUploadReadyToRun,
    answer: Option<CollisionAnswer>,
    sessions: &ThisServersSessions<C>,
    record: &dyn TheRecordAnUploadWrites,
) -> WhatTheAnswerLeft {
    // **Whatever else the answer comes to, this run no longer holds the
    // request before it** — its *Stop* is written below, or the record would
    // not take the answer and nothing moves. **Where it goes, the hold stands
    // until the run holds the request** ([`the_hold_passes_to_the_run`]), so a
    // Stop pressed between the two finds it (Sol's one look at the fixes after
    // 0.4.0's second try).
    let mut let_go = LetGoOnceAnswered(Some(ready.request.record.clone()));
    let left = the_answer_is_applied_while_held(ready, answer, sessions, record).await;
    if left == WhatTheAnswerLeft::ItGoes {
        let_go.0 = None;
    }
    left
}

/// [`the_answer_is_applied`], while the hold stands.
async fn the_answer_is_applied_while_held<C: AHeldConnection>(
    ready: &mut AnUploadReadyToRun,
    answer: Option<CollisionAnswer>,
    sessions: &ThisServersSessions<C>,
    record: &dyn TheRecordAnUploadWrites,
) -> WhatTheAnswerLeft {
    // **A Stop pressed before the request was under way is its answer**,
    // whatever answer comes now (`File Browser` U6; Sol's fresh look on the
    // twenty-eighth notes, piece 12).
    let stopped_before = the_uploads_stopped_before_they_went()
        .lock()
        .map(|mut stopped| stopped.remove(&ready.request.record.as_text()))
        .unwrap_or(false);
    let answer = if stopped_before { Some(CollisionAnswer::Stop) } else { answer.filter(|_| ready.something_collides()) };
    let Some(answer) = answer else {
        for entry in &mut ready.request.entries {
            // **`File Browser` U13: a unit handed to `Dependency Upload &
            // Remap` keeps its disposition** (the 38's piece 7).
            if !matches!(entry.disposition, Some(Disposition::HandedOver { .. })) {
                entry.disposition = Some(Disposition::ToSend);
            }
        }
        return if the_request_is_written_down(record, &ready.request) {
            WhatTheAnswerLeft::ItGoes
        } else {
            WhatTheAnswerLeft::TheRecordWouldNotTakeIt
        };
    };

    // **`File Browser` X9's generated names, in the enumeration's own order**,
    // composed against the receiving folders' own listings.
    let names = if answer == CollisionAnswer::KeepBoth {
        match the_keep_both_names(ready, sessions).await {
            Ok(names) => names,
            Err(why) => return WhatTheAnswerLeft::TheServerLentNothing(why),
        }
    } else {
        Vec::new()
    };

    let mut applied = None;
    let comparisons = ready.the_walk.comparisons.clone();
    let request = &mut ready.request;
    let wrote = record.with_the_record(&mut |connection| {
        let mut keeper = UploadRequests::over(connection);
        let mut names = names.clone().into_iter();
        applied = Some(apply_the_collision_answer(
            request,
            &comparisons,
            answer,
            &mut names,
            &mut keeper,
            SystemTime::now(),
        ));
    });
    match applied {
        Some(Ok(request::WhatTheAnswerDid::TheRequestStops)) if wrote => {
            WhatTheAnswerLeft::TheUserStoppedIt
        }
        Some(Ok(request::WhatTheAnswerDid::TheDispositionsAreWritten { .. })) if wrote => {
            WhatTheAnswerLeft::ItGoes
        }
        _ => WhatTheAnswerLeft::TheRecordWouldNotTakeIt,
    }
}

/// Lets go of a request held before its answer when the answer has been
/// applied, however it went — unless it goes, and the hold is kept for its run.
struct LetGoOnceAnswered(Option<Identity>);

impl Drop for LetGoOnceAnswered {
    fn drop(&mut self) {
        if let Some(request) = self.0.take() {
            its_answer_is_applied(&request);
        }
    }
}

/// **`File Browser` X9, for each colliding file in the enumeration's order.**
///
/// **The name is composed by `File Browser`'s own `keep_both::compose`**,
/// against the receiving folder's listing and every other name arriving in it.
/// **Where one cannot be composed the list stops there**, and every colliding
/// file from that one on ends as a failure naming it — `File Browser` U4's own
/// shape for a
/// name that will not fit, and never a silent send under the colliding name.
async fn the_keep_both_names<C: AHeldConnection>(
    ready: &AnUploadReadyToRun,
    sessions: &ThisServersSessions<C>,
) -> Result<Vec<Vec<u8>>, NotLent> {
    let colliding: Vec<&EnumerationEntry> = ready
        .request
        .entries
        .iter()
        .filter(|entry| {
            the_comparison_for(&ready.the_walk.comparisons, &entry.landing_path)
                .is_some_and(|one| one.what.is_a_collision())
        })
        .collect();
    let mut folders: Vec<Vec<u8>> = Vec::new();
    for entry in &colliding {
        let folder = the_folder_holding(&entry.landing_path);
        if !folders.iter().any(|held| paths_are_equal(held, &folder)) {
            folders.push(folder);
        }
    }
    let reading = folders.clone();
    let listings = sessions
        .a_reading(true, move |subsystem| {
            Box::pin(async move {
                let mut channel = TheReadingChannel::on(subsystem, ONE_READING);
                let mut out = Vec::new();
                for folder in reading {
                    let text = String::from_utf8_lossy(&folder).into_owned();
                    let listed = match channel.read_a_directory(&text, ONE_READING).await {
                        DirectoryAnswer::Entries {
                            entries,
                            the_subsystem_reported_the_end: true,
                        } => Some(CompleteListing::of(entries, SystemTime::now(), false)),
                        _ => None,
                    };
                    out.push((folder, listed));
                }
                Ok(out)
            })
        })
        .await?
        .map_err(|_| NotLent::TheConnectionEnded)?;

    let mut composed: Vec<Vec<u8>> = Vec::new();
    for entry in colliding {
        let folder = the_folder_holding(&entry.landing_path);
        let Some(listing) = listings
            .iter()
            .find(|(held, _)| paths_are_equal(held, &folder))
            .and_then(|(_, listed)| listed.as_ref())
        else {
            break;
        };
        // Every other name arriving in the same folder, and every name composed
        // already, so two generated names never land on one another.
        let mut also: Vec<EntryName> = ready
            .request
            .entries
            .iter()
            .filter(|other| paths_are_equal(&the_folder_holding(&other.landing_path), &folder))
            .map(|other| EntryName::from_the_machine(the_last_name(&other.landing_path).to_vec()))
            .collect();
        also.extend(
            composed
                .iter()
                .filter(|path| paths_are_equal(&the_folder_holding(path), &folder))
                .map(|path| EntryName::from_the_machine(the_last_name(path).to_vec())),
        );
        let original = EntryName::from_the_machine(the_last_name(&entry.landing_path).to_vec());
        let Some(fresh) = keep_both::compose(&original, false, &folder, listing, &also) else {
            break;
        };
        let mut landing = folder.clone();
        landing.push(b'/');
        landing.extend_from_slice(fresh.as_bytes());
        composed.push(landing);
    }
    Ok(composed)
}

// ---------------------------------------------------------------------------
// The files go.
// ---------------------------------------------------------------------------

/// **The upload, from its landing folders to its last file.**
///
/// `the_run` is the application's one desktop-side admission point, and
/// `sessions` the server's own point. **Neither is held past the file it was
/// asked for.**
///
/// **Up to `File Browser` O1's four files at once** (`File Browser` O2's upward
/// ceiling), each channel taking the next file when its own ends
/// ([`crate::downloading::every_one_of`]) — step 10c's eleventh-pass fix piece
/// 4. **The case this was first written for** is a few files, one after
/// another. **The case it now also meets** is a folder of a thousand small
/// files at a round trip of 0.2 s, where one file at a time took 46 minutes and
/// the pool of four had been there all along. **The files start in the
/// enumeration's order** (`File Browser` O7). **What then lands out of order,
/// and why it is harmless**: a small file that starts after a large one may
/// land first. Each file is its own transfer with its own staging name and its
/// own publication (`Server protocol and transfers` S1 and `Server protocol and transfers` S4), the landing
/// folders are all made before any file goes (`File Browser` N3), and `File
/// Browser` U11 counts outcomes, not their order — so nothing reads the order a
/// file landed in, and the report still names failures in the enumeration's.
pub async fn the_upload_runs<C: AHeldConnection>(
    ready: AnUploadReadyToRun,
    sessions: &ThisServersSessions<C>,
    the_run: &TheRunsTransfers,
    record: Arc<dyn TheRecordAnUploadWrites>,
    publishing: &dyn WhenAFileIsPublished,
) -> WhatTheUploadCameTo {
    // **The request is under way while this runs, and not a moment longer** —
    // step 10c's transfer wave piece 5, `File Browser` U7 and its paragraph
    // of 2026-09-28. Whatever ends this run — every unit ended, a period's
    // end that leaves each remaining file unable to start, or an unwinding —
    // the register lets it go, and the Transfers row then reads what the
    // record says of it and never *running*.
    let under_way = the_run.a_request_goes_under_way(
        &ready.request.record,
        &ready.request.server_row,
        sessions,
    );
    // **A Stop pressed while its answer was being applied** — *Keep both*'s
    // names being read, say — **or since, is taken now it is under way**
    // (`File Browser` U6; Sol's second look on the twenty-eighth notes, piece
    // 12), as the hold passes to the run (Sol's one look at the fixes after
    // 0.4.0's second try).
    the_hold_passes_to_the_run(the_run, &ready.request.record);

    // **`File Browser` N3: the landing folders, made plainly**, and a folder the
    // walk saw is a merge and is not re-made.
    //
    // **`File Browser` U6: the user's Stop reaches this step too**, read before
    // each folder (the twenty-eighth notes, piece 12: *Stop it* on a folder of
    // 156,557 files left the row *In progress* while the server kept getting
    // folders until Smoozy quit).
    let request_id = ready.request.record.as_text();
    let the_folders_that_failed =
        the_landing_folders_are_made(&ready, sessions, &|| the_run.the_user_stopped_it(&request_id)).await;

    // **`File Browser` U8's second and third bullets: what an earlier request
    // left on this machine, which this one may take up** — read once as the
    // run begins. Step 10c's transfer wave piece 7.
    let mut the_parts_left: Vec<TransferRecord> = Vec::new();
    record.with_the_record(&mut |connection| {
        the_parts_left = the_parts_a_new_request_may_take_up(
            connection,
            the_run,
            &ready.request.server_row,
            ready.request.installation.as_ref(),
            Direction::Up,
            &ready.request.record,
        );
    });

    let AnUploadReadyToRun {
        request,
        the_walk,
        the_jobs_told,
        the_servers_program,
        ..
    } = ready;
    let how_many = request.entries.len();
    let running = TheUploadRunning {
        request_id: request.record.as_text(),
        request_record: request.record.clone(),
        this_run: request.opened_by_run.clone(),
        request: Mutex::new(request),
        the_walk,
        the_folders_that_failed,
        the_parts_left: Mutex::new(the_parts_left),
        next: TheNextFile::default(),
        the_staging_folders: TheStagingFoldersSeen::default(),
        came_to: Mutex::new(Vec::new()),
        the_servers_program,
    };
    let workers = (0..THE_UPLOAD_DIRECTIONS_CEILING)
        .map(|_| {
            a_channel_of_the_upload(&running, how_many, sessions, the_run, &record, publishing)
        })
        .collect();
    every_one_of(workers).await;

    // **`File Browser` W9: each job the telling named has its mark finished
    // from its own units**, now that every unit this run could end has ended.
    {
        let request = running
            .request
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .clone();
        the_marks_are_finished(&request, &the_jobs_told, record.as_ref());
    }

    // **The run is over, so the request is no longer under way**, and the page
    // is told so it stops drawing it as moving. **Told as a move and not as a
    // unit's ending**, since no outcome was written here.
    drop(under_way);
    record.a_transfer_is_moving();
    running.what_it_came_to()
}

/// **An upload request while its files run**, shared by its channels. Added
/// 2026-09-30, step 10c's eleventh-pass fix piece 4.
///
/// **The request is behind a lock that is never held across a wait, and never
/// taken before the record's**: a channel writing an entry takes the record,
/// then the request inside that hold, so two channels cannot each hold one and
/// wait on the other. **Each entry is moved only by the channel that took
/// it**, and the request's own row — its identity, server, installation,
/// destination, answer and run — does not move while it runs, so an entry
/// written from the one request carries nothing another channel wrote.
struct TheUploadRunning {
    request_id: String,
    request_record: Identity,
    /// The run that opened the request (`File Browser` U1).
    this_run: Identity,
    request: Mutex<UploadRequest>,
    /// **`File Browser` X1's answer**, read and never moved.
    the_walk: TheAnswer,
    the_folders_that_failed: Vec<Vec<u8>>,
    the_parts_left: Mutex<Vec<TransferRecord>>,
    next: TheNextFile,
    the_staging_folders: TheStagingFoldersSeen,
    /// What each entry came to, by its place in the enumeration.
    came_to: Mutex<Vec<(usize, WhatOneUnitCameTo)>>,
    /// [`AnUploadReadyToRun::the_servers_program`], for every file of it.
    the_servers_program: Option<String>,
}

/// **What one unit of an upload came to in this run.**
#[derive(Debug, Clone, PartialEq, Eq)]
enum WhatOneUnitCameTo {
    Uploaded,
    Skipped,
    Failed(Vec<u8>),
    /// A failure whose write the machine refused for permission, with what it
    /// said (`File Browser` P1 and `File Browser` P6a).
    Refused(Vec<u8>, String),
    DidNotFinish,
    /// Nothing of it was this run's to send.
    Nothing,
}

impl TheUploadRunning {
    fn it_came_to(&self, at: usize, what: WhatOneUnitCameTo) {
        self.came_to
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .push((at, what));
    }

    /// **Counted the way `File Browser` U11 counts it**, and the files that
    /// failed named in the enumeration's order, whatever order they ended in.
    fn what_it_came_to(&self) -> WhatTheUploadCameTo {
        let mut each = std::mem::take(
            &mut *self
                .came_to
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner()),
        );
        each.sort_by_key(|(at, _)| *at);
        let mut came_to = WhatTheUploadCameTo::default();
        for (_, what) in each {
            match what {
                WhatOneUnitCameTo::Uploaded => came_to.uploaded += 1,
                WhatOneUnitCameTo::Skipped => came_to.skipped += 1,
                WhatOneUnitCameTo::Failed(landing) => came_to.failed.push(landing),
                WhatOneUnitCameTo::Refused(landing, said) => {
                    came_to.refused.push((landing.clone(), said));
                    came_to.failed.push(landing);
                }
                WhatOneUnitCameTo::DidNotFinish => came_to.did_not_finish += 1,
                WhatOneUnitCameTo::Nothing => {}
            }
        }
        came_to
    }
}

/// **The staging folders one request has seen made or standing** — `Server
/// protocol and transfers` S1's Smoozy-named folder beside the final name,
/// `File Browser` X12's reserved folder. Added 2026-09-30, step 10c's
/// eleventh-pass fix piece 4.
///
/// **Why it is asked for before each file at all**: a folder this login may
/// not write refuses the staging folder, and the refusal is the one answer that
/// says so (`File Browser` P1 and `File Browser` P6a, the seventh pass's fault 6). **Why only
/// once per folder per request**: a thousand files into one folder asked the
/// same folder a thousand times — one round trip each, at 0.2 s over three
/// minutes of a forty-six-minute upload (step 10c's transfer wave piece 13,
/// *Found on the way* 2). **An answer that says the login may not is not
/// kept**, so each file of a refused folder is told as the failure it is.
///
/// **Which folders this request made, and one ask at a time.** Added
/// 2026-10-16, step 10c's owed piece 1. A folder whose make answered that it
/// made it is kept as **made by this request**, and a file going into it with
/// no staged part on record is a fresh file: `Server protocol and transfers`
/// §7's loop then asks for none of the staging folder, the staged part's size
/// or its removal (`Server protocol and transfers` S12a, on the build runner's
/// ruling of that day).
/// **So the asks are taken one at a time**: with four files of a request
/// starting at once, four channels used to ask for one new folder together,
/// and only one of them could hear that it made it.
///
/// **What it does not remove**: for a file that is not fresh, `Server protocol
/// and transfers` §7's loop makes the folder again before the file's staging
/// object (`moving.rs`, S1's *made rather than assumed*) — and a folder removed
/// by hand between two files is made again there, as it always was.
#[derive(Debug, Default)]
pub(crate) struct TheStagingFoldersSeen {
    /// Each folder kept, and whether this request's own make made it.
    seen: Mutex<Vec<(Vec<u8>, bool)>>,
    /// **Held across one ask on the machine**, so a folder is asked for once
    /// per request and the one channel that made it is the one that knows.
    asking: tokio::sync::Mutex<()>,
}

impl TheStagingFoldersSeen {
    fn held(&self) -> std::sync::MutexGuard<'_, Vec<(Vec<u8>, bool)>> {
        self.seen
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    /// **Whether the folder is kept, and if so whether this request made it.**
    fn kept(&self, folder: &[u8]) -> Option<bool> {
        self.held()
            .iter()
            .find(|(seen, _)| seen == folder)
            .map(|(_, made)| *made)
    }

    /// **What the folder's asking answered**, kept where it says the folder
    /// is there: made, or refused for any reason but the login's permission
    /// and the path's absence — the plain make-directory's refusal of a folder
    /// that already stands (`Server protocol and transfers` §1.3: no message
    /// is mined, so the status is all there is).
    fn it_answered(&self, folder: &[u8], answered: &Result<(), SubsystemFailure>) {
        let it_stands = match answered {
            Ok(()) => true,
            Err(refused) => {
                matches!(refused, SubsystemFailure::TheServerRefused(_))
                    && !refused.is_the_login_not_being_allowed()
                    && !refused.is_the_path_not_being_there()
            }
        };
        if it_stands && self.kept(folder).is_none() {
            self.held().push((folder.to_vec(), answered.is_ok()));
        }
    }

    /// **The folder asked for, unless this request already has** — one ask at
    /// a time. Answers whether this request's own make made it, which is what
    /// makes a file going into it fresh; **`Err` is the one answer that says
    /// the login may not**, with what the server said.
    pub(crate) async fn asked_for<S>(
        &self,
        subsystem: &mut Subsystem<S>,
        folder: &[u8],
    ) -> Result<bool, SubsystemFailure>
    where
        S: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin + Send,
    {
        // **A folder already kept waits for nobody's ask.**
        if let Some(made) = self.kept(folder) {
            return Ok(made);
        }
        let _one_at_a_time = self.asking.lock().await;
        if let Some(made) = self.kept(folder) {
            return Ok(made);
        }
        let answered = subsystem.make_directory(folder).await;
        if let Err(refused) = &answered {
            if refused.is_the_login_not_being_allowed() {
                return Err(refused.clone());
            }
        }
        self.it_answered(folder, &answered);
        Ok(self.kept(folder).unwrap_or(false))
    }
}

/// **One of `File Browser` O1's channels, for one upload request**: the next
/// file, to its ending, then the next.
async fn a_channel_of_the_upload<C: AHeldConnection>(
    running: &TheUploadRunning,
    how_many: usize,
    sessions: &ThisServersSessions<C>,
    the_run: &TheRunsTransfers,
    record: &Arc<dyn TheRecordAnUploadWrites>,
    publishing: &dyn WhenAFileIsPublished,
) {
    while let Some(at) = running.next.of(how_many) {
        let what = one_unit_of_the_upload(running, at, sessions, the_run, record, publishing).await;
        running.it_came_to(at, what);
    }
}

/// **One unit of `File Browser` U1's request, from its disposition to its
/// ending** — what the upload did for each entry in turn before step 10c's
/// eleventh-pass fix piece 4, now done for whichever entry a channel takes.
async fn one_unit_of_the_upload<C: AHeldConnection>(
    running: &TheUploadRunning,
    at: usize,
    sessions: &ThisServersSessions<C>,
    the_run: &TheRunsTransfers,
    record: &Arc<dyn TheRecordAnUploadWrites>,
    publishing: &dyn WhenAFileIsPublished,
) -> WhatOneUnitCameTo {
    let request_id = &running.request_id;
    let entry = the_request(&running.request).entries[at].clone();
    // **The user's Stop on the request: unarranged files are never
    // arranged** (`File Browser` U6). Step 10c's transfer wave piece 7.
    if the_run.the_user_stopped_it(request_id) {
        let owed = entry.outcome.is_none()
            && entry.disposition.as_ref().is_some_and(|disposition| {
                disposition.a_transfer_is_arranged() && *disposition != Disposition::SkipExisting
            });
        return if owed {
            WhatOneUnitCameTo::DidNotFinish
        } else {
            WhatOneUnitCameTo::Nothing
        };
    }
    let Some(disposition) = entry.disposition.clone() else {
        return WhatOneUnitCameTo::Nothing;
    };
    if entry.outcome.is_some() {
        return if entry.outcome == Some(UnitOutcome::Failed) {
            WhatOneUnitCameTo::Failed(entry.landing_path.clone())
        } else {
            WhatOneUnitCameTo::Nothing
        };
    }
    // **`File Browser` U8's own reading of what the new request does with
    // a part left at this file's name** — its source path and its landing
    // path matched, the new request's disposition and collision answer
    // deciding.
    let comparison =
        the_comparison_for(&running.the_walk.comparisons, &entry.landing_path).map(|one| one.what);
    let mut taken_up = None;
    let mut abandoned = None;
    {
        let mut the_parts_left = running
            .the_parts_left
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let what_it_does = comparison.map(|comparison| {
            request::what_the_new_request_does(&entry, comparison, &the_parts_left)
        });
        match what_it_does {
            Some(request::WhatTheNewRequestDoes::TakeUpTheInterruptedTransfer {
                transfer, ..
            }) => {
                if let Some(found) = the_parts_left
                    .iter()
                    .position(|part| part.transfer.as_text() == transfer.as_text())
                {
                    taken_up = Some(the_parts_left.remove(found));
                }
            }
            // **`Server protocol and transfers` S28a: a part the new request
            // does not take up is written *stopped, abandoned by its request*
            // before anything else**, or no sweep ever reaches it.
            Some(request::WhatTheNewRequestDoes::SkipItAndAbandonTheStaging)
            | Some(request::WhatTheNewRequestDoes::StartFreshAndAbandonTheOldStaging) => {
                let matched = the_parts_left.iter().position(|part| {
                    paths_are_equal(part.source_path.as_bytes(), &entry.source_path)
                        && paths_are_equal(part.final_path.as_bytes(), &entry.landing_path)
                });
                if let Some(found) = matched {
                    abandoned = Some(the_parts_left.remove(found));
                }
            }
            _ => {}
        }
    }
    if taken_up.is_some() {
        the_run.every_leftover_is_read_again();
    }
    if let Some(part) = abandoned {
        let the_door = |act: &mut dyn FnMut(&rusqlite::Connection)| record.with_the_record(act);
        let _ = a_part_is_abandoned(&the_door, the_run, &part);
    }
    if disposition == Disposition::SkipExisting {
        return WhatOneUnitCameTo::Skipped;
    }
    if !disposition.a_transfer_is_arranged() {
        return WhatOneUnitCameTo::Nothing;
    }
    let landing = match &disposition {
        Disposition::RenamedUnderKeepBoth(fresh) => fresh.clone(),
        _ => entry.landing_path.clone(),
    };
    // **One file the user stopped by itself, before it started** — it is
    // never arranged, and the rest of the request goes on (`Design/Transfers
    // § Stopping one file`). Step 10c's transfer wave piece 7.
    if the_run.the_user_stopped_this(request_id, &landing) {
        return WhatOneUnitCameTo::DidNotFinish;
    }

    // **The pre-flight, per file** — `File Browser` N3's failed folder,
    // `File Browser` N2's carriage, `File Browser` N4's length bound and
    // `File Browser` X12's reserved name. **Each is a
    // per-file failure and never sent** (`File Browser` N5).
    let the_folder_failed = running
        .the_folders_that_failed
        .iter()
        .any(|folder| is_inside(&landing, folder));
    let will_not_go = the_folder_failed
        || check_the_name_can_be_carried(&entry.source_path, &landing).is_err()
        || check_the_length(&landing) != TheLengthBound::Inside
        || would_land_on_the_reserved_folder(&landing);
    if will_not_go {
        // **The folder's reason is kept for the Transfers row**; the name
        // reasons are read again from the path there (`Design/Transfers § A
        // file that failed`, 2026-11-11).
        if the_folder_failed {
            a_folder_was_not_made_for(&running.request_record, &entry.landing_path);
        }
        the_unit_ended(&running.request, at, UnitOutcome::Failed, record.as_ref());
        return WhatOneUnitCameTo::Failed(landing);
    }

    let intent = {
        let request = the_request(&running.request);
        the_intent_for(&request, &running.the_walk, at)
    };
    publishing.a_file_begins_going_up(&landing);
    let went = one_file_goes_up(
        running, at, &landing, intent, sessions, the_run, record, taken_up,
    )
    .await;
    publishing.a_file_stopped_going_up(&landing);
    match went {
        Some(UnitOutcome::Done) => {
            publishing.a_file_was_published(&landing, &UnitOutcome::Done);
            WhatOneUnitCameTo::Uploaded
        }
        Some(UnitOutcome::Failed) => {
            match what_the_server_said_refusing(&running.request_record, &landing) {
                Some(said) => WhatOneUnitCameTo::Refused(landing, said),
                None => WhatOneUnitCameTo::Failed(landing),
            }
        }
        Some(UnitOutcome::NotAttempted { .. }) | None => WhatOneUnitCameTo::DidNotFinish,
    }
}

/// **The request, held for one short act** — never across a wait.
fn the_request(request: &Mutex<UploadRequest>) -> std::sync::MutexGuard<'_, UploadRequest> {
    request
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

/// **`File Browser` X7: the unit's publication intent, taken from the walk and
/// the recorded answer.**
///
/// **Where no answer is recorded, nothing collided** and `File Browser` X7's
/// first bullet applies to every unit: the walk found the landing name absent,
/// so the publication is create-intended. A collision with no answer is never
/// published.
fn the_intent_for(request: &UploadRequest, the_walk: &TheAnswer, at: usize) -> Option<Intent> {
    let entry = &request.entries[at];
    let comparison = the_comparison_for(&the_walk.comparisons, &entry.landing_path)?.what;
    match request.collision_answer {
        Some(_) => intent_for(request, entry, comparison),
        None => (!comparison.is_a_collision()).then_some(Intent::CreateIntended),
    }
}

/// **One file up**: admitted at the desktop point, seated at the server's
/// point, carried by `Server protocol and transfers` §7's loop under
/// `File Browser` K8's watch and `File Browser` K9's gate, and its outcome
/// written.
///
/// **`None` where the file never started** — the connection went, or no seat
/// could be had — which leaves the unit owed rather than ended.
///
/// **`taken_up` is a part an earlier request left at this file's name** —
/// `File Browser` U8's second bullet: the unit continues that interrupted
/// transfer, its identity, its record and its staging path, through `Server
/// protocol and transfers` S12 entire and `File Browser` K10's gates, governed
/// by this request's disposition and answer throughout (`File Browser` U8's
/// third bullet).
/// Step 10c's transfer wave piece 7.
///
/// **`None` too where the user stopped the file**: `File Browser` U6's *the one
/// in flight is stopped* leaves the unit owed, and nothing about it failed.
///
/// **It is one of up to four at once** (step 10c's eleventh-pass fix piece 4),
/// so the request is reached through its lock, one short act at a time.
#[allow(clippy::too_many_arguments)]
async fn one_file_goes_up<C: AHeldConnection>(
    running: &TheUploadRunning,
    at: usize,
    landing: &[u8],
    intent: Option<Intent>,
    sessions: &ThisServersSessions<C>,
    the_run: &TheRunsTransfers,
    record: &Arc<dyn TheRecordAnUploadWrites>,
    taken_up: Option<TransferRecord>,
) -> Option<UnitOutcome> {
    // **`Server protocol and transfers` S15a: where the caller declares no
    // intent, publication does not begin** — so the file is not sent.
    let intent = intent?;
    let class = request::the_priority_class();
    let request = &running.request;

    // **R17: the slot is asked for per file**, and the seat with it — see
    // [`a_slot_and_a_seat`], which waits while the line is down and the
    // period stands (step 10c's transfer wave piece 6).
    let request_id = running.request_id.clone();
    let tell = || record.a_transfer_is_moving();
    let (mut draw, mut subsystem) =
        a_slot_and_a_seat(class, sessions, the_run, (&request_id, None, &tell)).await?;
    // **Stopped while it waited for its turn** (`File Browser` U6): with four
    // files of a request at once, a file may wait for the slot while another
    // is stopped, and a file the user stopped is never arranged. Step 10c's
    // eleventh-pass fix piece 4.
    if the_run.the_user_stopped_this(&request_id, landing) {
        let _ = draw.the_channel_ended(std::time::Instant::now());
        return None;
    }

    // **`File Browser` P1 and `File Browser` P6a: a write the machine refuses for permission
    // is a failure, told as one.** `Server protocol and transfers` S1 puts the
    // staging folder beside the landing, inside the folder the user chose, so
    // a folder this login cannot write refuses the staging before any byte
    // moves. **The transfer's own loop reads every refusal of that folder as
    // *already there*** — which is right for the folder that is there — and
    // then meets the refused staging file as an interruption, so until
    // 2026-09-27 the upload said only *did not finish*, with no reason
    // anywhere (the seventh pass on Hyperstack, fault 6: a first drag onto `/`
    // with the `ubuntu` login). **So the folder is asked for here first, and
    // the one answer that says the login may not is taken as the failure it
    // is**, with the server's own words kept for the Transfers row. Every other
    // answer is left to the loop exactly as before. **Once per folder per
    // request** ([`TheStagingFoldersSeen`], step 10c's eleventh-pass fix
    // piece 4).
    let the_staging_folder = the_staging_folder_beside(landing);
    let made_by_this_request = match running
        .the_staging_folders
        .asked_for(&mut subsystem, &the_staging_folder)
        .await
    {
        Ok(made) => made,
        Err(refused) => {
            let _ = draw.the_channel_ended(std::time::Instant::now());
            drop(subsystem);
            if let Ok(mut held) = the_refusals_this_run().lock() {
                held.insert((request_id.clone(), landing.to_vec()), refused.in_words());
            }
            the_unit_ended(request, at, UnitOutcome::Failed, record.as_ref());
            return Some(UnitOutcome::Failed);
        }
    };
    // **A fresh file** (`Server protocol and transfers` S12a): its staging
    // folder made by this request, and no part an
    // earlier request left taken up.
    let fresh = made_by_this_request && taken_up.is_none();

    let entry = the_request(request).entries[at].clone();
    let mut started_over = None;
    let (transfer, mut arranged) = match taken_up {
        // **The part is taken up, and the record is this run's from here on** —
        // which is what `Server protocol and transfers` C11 step 4 reads if the
        // line then goes under it.
        //
        // **`File Browser` K10 clause 2 first, and K12 makes it this upload's
        // too**: *"the one unchanged-source answer, for every upload of a
        // local source"*. Where the file on this computer, read up to the
        // part's highest checkpoint, no longer digests to that checkpoint's
        // running digest, the transfer **starts over from zero into a fresh
        // staging path** (`Server protocol and transfers` S12 step 5) and the
        // row says so. **Until the two transfer faults after the 38's pieces 7
        // and 9 nothing asked it**: the loop's own resume checks the bytes on
        // the server against the record and never the file here, so a file
        // saved between the cut and the new request was carried on over its
        // old first part, crossed whole, and was then refused by `File
        // Browser` K9's gate — *The file changed during upload* — on the
        // very request that asked for the file as it is now. **K10 clause 1
        // has nothing to compare against here**: the arrangement identity
        // (K7) is held by the watch of the request that arranged the part,
        // and no record keeps it, so clause 2 decides, as it does for the
        // check's own send ([`crate::the_found_files_sent`]).
        Some(mut part) => {
            part.written_by_run = running.this_run.clone();
            part.ending = None;
            started_over = a_part_taken_up_is_made_ready(&mut part, entry.size_as_enumerated);
            (part.transfer.clone(), part)
        }
        None => {
            let Ok(transfer) = Identity::issue(IdentityKind::Transfer) else {
                let _ = draw.the_channel_ended(std::time::Instant::now());
                return None;
            };
            // **`Server protocol and transfers` S1: staging sits beside the
            // final name**, in the reserved folder `File Browser` X12 names.
            let staging = String::from_utf8_lossy(&the_staging_folder).into_owned();
            let arranged = arrange_a_transfer(
                &the_request(request),
                &entry,
                transfer.clone(),
                &staging,
                running.this_run.clone(),
            );
            let Ok(Some(arranged)) = arranged else {
                let _ = draw.the_channel_ended(std::time::Instant::now());
                drop(subsystem);
                the_unit_ended(request, at, UnitOutcome::Failed, record.as_ref());
                return Some(UnitOutcome::Failed);
            };
            (transfer, arranged)
        }
    };
    // **`File Browser` U1: the entry carries its transfer identity once a
    // transfer is arranged**, and that one entry goes down **in the same save
    // as the transfer record's first**, which `Server protocol and transfers`
    // S8 puts before the first byte — see [`TheTransferRecords`].
    //
    // **`Server protocol and transfers` G4's report reaches the row** —
    // `File Browser` U11's bar in bytes — with the next files waiting their
    // turn behind this one. Step 10c's transfer wave piece 5.
    let waiting = {
        let mut held = the_request(request);
        held.entries[at].transfer = Some(transfer);
        the_next_waiting(&held.entries, at, |entry| {
            let disposition = entry.disposition.as_ref()?;
            (entry.outcome.is_none()
                && disposition.a_transfer_is_arranged()
                && *disposition != Disposition::SkipExisting)
                .then(|| match disposition {
                    Disposition::RenamedUnderKeepBoth(fresh) => fresh.clone(),
                    _ => entry.landing_path.clone(),
                })
        })
    };

    let source = arranged.source_path.clone();
    let size = arranged.total_size_expected;
    the_run.a_file_goes(&request_id, at, landing.to_vec(), size, waiting);
    let mut watcher = TheRowIsTold {
        the_run,
        request: request_id.clone(),
        entry: at,
        tell: &tell,
        started_over,
    };
    let mut keeper = TheTransferRecords {
        record: Arc::clone(record),
        request,
        at,
        the_entry_is_down: false,
        the_unit_ended_in_the_save: None,
        the_run,
    };
    let mut reading_for_the_gate = ThePlatformFileAdapter;
    let mut gate = TheSourcesPublicationGate::over(&mut reading_for_the_gate, source.clone(), size);
    let mut local = ThePlatformFileAdapter;
    let mut identities = ThePlatformFileAdapter;

    // **`File Browser` K7 and `File Browser` K8: the source is watched where its
    // identity can
    // be read**, and where it cannot, `File Browser` K9's gate still covers
    // it — nothing is
    // guessed.
    //
    // **The same watch is kept across a repair**, so its arrangement identity
    // is the one `File Browser` K10 clause 1 compares a resume against: a
    // file saved while the line was down is caught at the resume's first
    // reading of it and stopped, and the user is told it changed.
    let across = TheWayBack {
        sessions,
        the_run,
        class,
        request: &request_id,
        entry: at,
        tell: &tell,
        this_run: running.this_run.clone(),
        landing,
        record,
        the_servers_program: running.the_servers_program.as_deref(),
    };
    let happened = match TheSourceUnderWatch::arranged(&mut local, &mut identities, source.clone())
    {
        Some(mut watched) => {
            carried_across_repairs(
                &across,
                (&mut subsystem, &mut draw),
                &mut watched,
                &mut keeper,
                &mut watcher,
                intent,
                &mut arranged,
                &mut gate,
                fresh,
            )
            .await
        }
        None => {
            let mut plain = ThePlatformFileAdapter;
            carried_across_repairs(
                &across,
                (&mut subsystem, &mut draw),
                &mut plain,
                &mut keeper,
                &mut watcher,
                intent,
                &mut arranged,
                &mut gate,
                fresh,
            )
            .await
        }
    };

    // **`File Browser`'s upload-refusal rule: one free-space reading where the
    // file lands, taken at the failure on the same subsystem.** No space left
    // names a full disk; a reading that does not come back names nothing.
    // It is not a fit estimate, and nothing is refused before sending.
    let full = if happened == Ending::Failed {
        let parent = landing.rsplitn(2, |byte| *byte == b'/').nth(1).unwrap_or(b"/").to_vec();
        let path = String::from_utf8_lossy(&parent);
        tokio::time::timeout(
            std::time::Duration::from_secs(30),
            smoozy_server_protocol::free_space::read_it_at_the_deepest_ancestor_that_exists(&mut subsystem, &path),
        )
        .await
        .ok()
        .and_then(Result::ok)
        .is_some_and(|read| read.available_bytes == 0)
    } else {
        false
    };
    the_server_disk_was_full_for(&request_id, landing, full);
    // **And one read of the file on this computer, at the same failure**: a
    // drive unplugged under the upload is said as this computer's, not as an
    // upload that failed for no reason (the Rough what-if answers, share 2,
    // 188).
    the_source_was_unreadable_for(
        &request_id,
        landing,
        happened == Ending::Failed && !full && the_source_could_not_be_read_here(&source),
    );

    // **R5's *at once*, and R17's guarantee with it**: the draw and the seat go
    // back at this file's ending, whatever happened.
    let _ = draw.the_channel_ended(std::time::Instant::now());
    drop(draw);
    drop(subsystem);
    drop(watcher);
    // **Whatever ended it, the file is no longer moving.** Where its outcome
    // went down in the ending's save, it stopped in that same hold already.
    the_run.a_file_stopped(&request_id, at);
    let the_entry_is_down = keeper.the_entry_is_down;
    let the_unit_ended_in_the_save = keeper.the_unit_ended_in_the_save.take();
    drop(keeper);

    // **A transfer whose record was never written carried no entry with it**,
    // so the entry's transfer identity goes down on its own, as it did before
    // the two were one save.
    if !the_entry_is_down {
        the_entry_is_written_down(record.as_ref(), request, at);
    }
    // **The user's own Stop leaves the unit owed** — `File Browser` U6's *the
    // one in flight is stopped*; nothing about it failed, and no outcome is
    // written for it. Step 10c's transfer wave piece 7.
    if the_user_stopped_this_ending(the_run, &request_id, landing, &happened) {
        record.a_transfer_moved();
        return None;
    }
    let outcome = the_units_outcome_after(&happened)?;
    match the_unit_ended_in_the_save {
        // **The outcome went down in the ending's own save**, so this copy of
        // the request takes the same outcome at the same moment, and the page
        // is told — nothing is written twice.
        Some((written, moment)) if written == outcome => {
            let moved = the_request(request).entries[at]
                .its_outcome_became(outcome.clone(), moment)
                .is_ok();
            if moved {
                record.a_transfer_moved();
            }
        }
        _ => the_unit_ended(request, at, outcome.clone(), record.as_ref()),
    }
    Some(outcome)
}

/// **`File Browser` K10 clause 2 — `Dependency Upload & Remap` A10, adopted —
/// over a part an earlier request left**: the file on this computer, read up
/// to the part's highest checkpoint, digests to that checkpoint's recorded
/// running digest. The built test, the same one the check's own send asks
/// (`smoozy_dependencies::landing::a_resume_of_this_modules_upload`), and never
/// a second. A part with no checkpoint, or a file that cannot be read, does not
/// pass: `Server protocol and transfers` S12 step 2 would start it over anyway, and starting over is safe
/// under every reading. The two transfer faults after the 38's pieces 7 and
/// 9, fault 1.
/// **Whether a part an earlier request left may be carried on** — `File
/// Browser` U8's paragraph of 2026-11-12 and `File Browser` K10 clause 2: the
/// file's size now is the size the part was arranged at, and its first part
/// is the file's own ([`why_the_part_cannot_be_carried_on`]). The Fable
/// review's seventh sitting, fault 2.
///
/// **The case that clause was first read for** is a file changed somewhere in
/// what was already sent. **The case it now also meets** is a file that only
/// grew at its end: every checkpoint still digests, the last one covering
/// the whole file as it was, so a part left in publication pending was
/// published as it stood, shorter and older than the file, and read as
/// uploaded. The dependency check's send asks this too
/// (`crate::the_found_files_sent`).
pub(crate) fn the_part_carries_on(part: &TransferRecord, the_size_now: u64) -> bool {
    why_the_part_cannot_be_carried_on(part, the_size_now).is_none()
}

/// **A part taken up, made ready for the loop**: carried on as it stands, or
/// started over from zero into a fresh staging path (`Server protocol and
/// transfers` S12 step 5) **with the file's size as it now stands as its
/// expected total** — `File Browser` U8's paragraph of 2026-11-12. Until then
/// a start-over kept the size the part was arranged at, so a file saved at
/// another size ended *the file changed during upload* on the request that
/// asked for it as it is now. Answers the cause, for the row.
fn a_part_taken_up_is_made_ready(
    part: &mut TransferRecord,
    the_size_now: u64,
) -> Option<smoozy_server_protocol::transfers::StartOverCause> {
    let cause = why_the_part_cannot_be_carried_on(part, the_size_now)?;
    part.start_over();
    part.total_size_expected = the_size_now;
    Some(cause)
}

/// **`File Browser` K10 clause 2 — `Dependency Upload & Remap` A10's test —
/// over a part taken up, read once, and why it fails where it does** (`File
/// Browser` K10a, uploads carry on). `None` where the part is the file's own:
/// the same size, and the file read up to the part's highest checkpoint
/// digests to that checkpoint's digest. **Another size, or a file that reads
/// and digests otherwise, is the file on this computer changed**; no
/// checkpoint to go on from, or a file that could not be read that far, is a
/// resume that could not be made. The one read is the outcome both answers
/// come from (after Sol's look: a second, shorter read could not tell a file
/// that changed from one that failed part-way).
fn why_the_part_cannot_be_carried_on(
    part: &TransferRecord,
    the_size_now: u64,
) -> Option<smoozy_server_protocol::transfers::StartOverCause> {
    use smoozy_dependencies::readings::Readings as _;
    use smoozy_server_protocol::transfers::StartOverCause;
    if part.total_size_expected != the_size_now {
        return Some(StartOverCause::TheFileChangedOnThisComputer);
    }
    let Some(checkpoint) = part.highest_checkpoint_at_or_below(u64::MAX) else {
        return Some(StartOverCause::TheStagedPrefixDidNotVerify);
    };
    let read = crate::checking_a_file::TheUsersOwnComputer.the_sources_prefix_digest(
        &smoozy_dependencies::bytes::StoredLocation::as_the_system_returned_it(part.source_path.as_bytes()),
        checkpoint.offset,
    );
    match read {
        Some(digest) if digest == checkpoint.digest_of_everything_up_to_it => None,
        Some(_) => Some(StartOverCause::TheFileChangedOnThisComputer),
        None => Some(StartOverCause::TheStagedPrefixDidNotVerify),
    }
}

/// **The unit's outcome a transfer's ending makes**, and `None` where it makes
/// none — one reading, used where the ending's save writes it and where the
/// file's end reads it.
fn the_units_outcome_after(ending: &Ending) -> Option<UnitOutcome> {
    match ending {
        Ending::Published => Some(UnitOutcome::Done),
        // **Smoozy's own stops — the source changed under it (`File Browser`
        // K8, `File Browser` K9), the machine already holds something under the
        // name
        // (`File Browser` X7) — and a digest that disagreed, all end the unit
        // as a failure that names the file.**
        Ending::Stopped(_) | Ending::Failed => Some(UnitOutcome::Failed),
        // **Interrupted or stalled: the record stands for a later carry-on**,
        // and no unit outcome is written (`File Browser` U5's *waiting*).
        Ending::Interrupted | Ending::Stalled => None,
    }
}

/// **What a file needs to find its way back to the machine after the line
/// was cut** — the server's point, the run's register, the class it is
/// admitted under, which request and entry it is, how the page is told, and
/// this application run. Step 10c's transfer wave piece 6.
pub(crate) struct TheWayBack<'a, C: AHeldConnection> {
    pub(crate) sessions: &'a ThisServersSessions<C>,
    pub(crate) the_run: &'a TheRunsTransfers,
    pub(crate) class: &'static str,
    pub(crate) request: &'a str,
    pub(crate) entry: usize,
    pub(crate) tell: &'a (dyn Fn() + Sync),
    pub(crate) this_run: Identity,
    /// **Where the file lands, which is what a stop of this one file names**,
    /// and the record door a stop is written through. Step 10c's transfer
    /// wave piece 7.
    pub(crate) landing: &'a [u8],
    pub(crate) record: &'a Arc<dyn TheRecordAnUploadWrites>,
    /// **Where Smoozy's own program sits on the server**, so a carry-on has
    /// the part checked there (`Server protocol and transfers` S12b); `None`
    /// reads it back as before.
    pub(crate) the_servers_program: Option<&'a str>,
}

/// **`Server protocol and transfers` S26's deliberate stop, for an upload the
/// user stopped** — `File Browser` U6: *"the record is written* stopped
/// *before anything else, the staged material's removal happens on the
/// connection where one is up and is otherwise the owed cleanup of* `Server
/// protocol and transfers` S27*."*
/// Added 2026-09-29, step 10c's transfer wave piece 7.
///
/// **The record goes down first and on its own** (`Server protocol and
/// transfers` C23), straight to the transfer records and not through the
/// file's own keeper, which would write the unit *failed*: nothing about a
/// stopped file failed.
///
/// **Then the part, over the file's own channel where it is still up** — the
/// one this transfer made and whose name it synthesised (`Server protocol and
/// transfers` S4), which is what `Server protocol and transfers` S26's *removed
/// at once* is and what the byte loop's own ending removes by. **Where there
/// is no channel — the line was down when the stop came — the removal is
/// owed**, and the next connection's sweep is where it is taken up.
async fn an_upload_the_user_stopped_is_written_down<S>(
    arranged: &mut TransferRecord,
    subsystem: Option<&mut Subsystem<S>>,
    record: &dyn TheRecordAnUploadWrites,
) where
    S: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin + Send,
{
    arranged.ending = Some(THE_USERS_STOP);
    arranged.cleanup_is_owed = true;
    let put = |arranged: &TransferRecord| {
        let mut written = false;
        record.with_the_record(&mut |connection| {
            written = crate::record::transfer_records::put(connection, arranged).is_ok();
        });
        written
    };
    if !put(arranged) {
        return;
    }
    let Some(subsystem) = subsystem else {
        return;
    };
    let removed = matches!(
        tokio::time::timeout(
            THE_INACTIVITY_DEADLINE,
            subsystem.remove(&arranged.staging_path.as_bytes()),
        )
        .await,
        Ok(Ok(()))
    );
    if removed {
        arranged.cleanup_is_owed = false;
        let _ = put(arranged);
    }
}

/// **The desktop point's slot and the server point's seat, for one attempt of
/// one file** — R17's per-file slot and `Server protocol and transfers` T5 and
/// T1's channel of its own. **`None` where the file cannot start**: the period
/// ended, or the point refused a seat on a line that is up.
///
/// **While the line is down and the period stands, it waits** (`File Browser`
/// K2 and `File Browser` U5, and `crate::downloading::while_the_line_is_down`)
/// and asks again
/// when the point lends. **The slot is never held across that wait**: it goes
/// back at once (R5), so an upload waiting for its connection holds nothing
/// another class could use. **R13**: a refusal on a line that is up is a
/// reason to wait a moment and ask again, never to fail.
///
/// **The case this was first written for** is a line that stays up, where a
/// point lending nothing meant the file could not start and was left owed.
/// **The case it now also meets** is a line cut and repaired inside the period,
/// where the point lends nothing for the length of the repair and then lends
/// again. Until step 10c's transfer wave piece 6 every file that met the
/// repair was skipped, so a one-minute drop ended the whole request.
pub(crate) async fn a_slot_and_a_seat<C: AHeldConnection>(
    class: &'static str,
    sessions: &ThisServersSessions<C>,
    the_run: &TheRunsTransfers,
    (request, entry, tell): (&str, Option<usize>, &(dyn Fn() + Sync)),
) -> Option<(TheChannelsDraw, Subsystem<ALentChannel<C>>)> {
    loop {
        let mut draw = match the_run.admit_to_carry(Direction::Up, &[class], class) {
            Ok(draw) => draw,
            Err(_) if sessions.a_connection_is_live() => {
                tokio::time::sleep(THE_PAUSE_BEFORE_ASKING_AGAIN).await;
                continue;
            }
            Err(_) => {
                if while_the_line_is_down(sessions, the_run, request, entry, tell).await {
                    continue;
                }
                return None;
            }
        };
        let seated = match sessions.a_transfer_channel().await {
            Ok(channel) => Subsystem::negotiate(channel).await.ok(),
            Err(_) => None,
        };
        if let Some(subsystem) = seated {
            return Some((draw, subsystem));
        }
        let _ = draw.the_channel_ended(std::time::Instant::now());
        drop(draw);
        // **Whether the line went down is looked for, never glanced at** —
        // `File Browser` K2 and `File Browser` U5, the same look an interrupted file makes
        // ([`the_line_went_down`]). Added 2026-10-02, out of the hosted run
        // that was red on Linux at `d7a25677`. **The case this was first
        // written for** is a seat refused on a line that stays up, where the
        // point is lending and the file cannot start. **The case it now also
        // meets** is a seat asked for at the moment the line goes: the point
        // answers *the connection is gone* — a channel's opening fails at
        // once, and the transport reports the ending a moment later — or its
        // own ending drops the ask before it stops saying it lends. **Either
        // way the point still reads as lending for a moment**, and one glance
        // ended a file the repair would have carried on.
        if !the_line_went_down(sessions).await
            || !while_the_line_is_down(sessions, the_run, request, entry, tell).await
        {
            return None;
        }
    }
}

/// **One file carried, and carried on across a repair** — `File Browser` K1,
/// K2, K4 and K10, `Server protocol and transfers` C11 step 4 and
/// `Server protocol and transfers` S12 entire.
/// Added 2026-09-29, step 10c's transfer wave piece 6.
///
/// **The loop's own resume is already built** (`Server protocol and transfers`
/// S12: read the staged size, take the highest checkpoint at or below it,
/// verify the staged prefix against the record, carry on or start over). **What
/// was missing was the caller**: nothing handed an *interrupted* record back to
/// the loop, so a file the line cut stayed cut, though the period stood and
/// the line came back.
///
/// **So an *interrupted* ending is carried on** where the line went down under
/// it and came back under the same period, through
/// `crate::downloading::the_record_is_carried_on`'s gate, with a fresh slot and
/// a fresh seat and **the same record, the same watch and the same gate**.
/// Every other ending is the file's ending, and so is an interruption on a
/// line that stayed up, a period that ended while it waited (`File Browser`
/// U7's paragraph of 2026-09-28), and a point that would not seat it again.
///
/// **And an *interrupted* ending is carried on where `Servers` S14's
/// additional connection died under it alone**, the primary still up — the
/// dead-line sweep's extra-connection repair, 2026-10-19. Nothing is waited for first: the
/// next seat is on a fresh additional connection (`Servers` S14 condition 6),
/// and where that cannot be made yet the file waits, reading *waiting for the
/// connection*, and asks again.
///
/// **`File Browser` K10's two gates are the loop's and the watch's, and both
/// run**: clause 1 is the watch kept from the arrangement, which reads the
/// source at the resume's first bite and stops a changed file; clause 2's
/// contents test is the loop's verification of the staged prefix against the
/// checkpoint's recorded digest, and `File Browser` K9's gate still compares
/// the whole file before publication.
#[allow(clippy::too_many_arguments)]
pub(crate) async fn carried_across_repairs<C, L, K>(
    way_back: &TheWayBack<'_, C>,
    (subsystem, draw): (
        &mut Subsystem<ALentChannel<C>>,
        &mut crate::scheduling::TheChannelsDraw,
    ),
    local: &mut L,
    keeper: &mut K,
    watcher: &mut TheRowIsTold<'_>,
    intent: Intent,
    arranged: &mut TransferRecord,
    gate: &mut TheSourcesPublicationGate<'_, ThePlatformFileAdapter>,
    fresh: bool,
) -> Ending
where
    C: AHeldConnection,
    L: TheUsersOwnComputer,
    K: KeepsTheRecord,
{
    // **`fresh` is the first carry's alone**
    // (`Server protocol and transfers` S12a): a carry-on after a repair is never fresh,
    // since the cut may have left a part at the staging name.
    let mut fresh = fresh;
    loop {
        let TheWayBack {
            sessions,
            the_run,
            class,
            request,
            entry,
            tell,
            landing,
            record,
            ..
        } = *way_back;
        let Some(ending) = carried(
            subsystem,
            local,
            draw,
            keeper,
            watcher,
            intent,
            arranged,
            gate,
            (the_run, request, landing),
            std::mem::take(&mut fresh),
            // **`Server protocol and transfers` S12b: the server's own program
            // checks the part it holds**, where the server row names one.
            way_back.the_servers_program.map(|package_area| (sessions, package_area)),
        )
        .await
        else {
            // **The user stopped it, and the loop is at its pause** — the stop
            // is written, and the part removed over this same channel.
            an_upload_the_user_stopped_is_written_down(
                arranged,
                Some(&mut *subsystem),
                record.as_ref(),
            )
            .await;
            return THE_USERS_STOP;
        };
        if ending != Ending::Interrupted {
            return ending;
        }
        // **R5: the slot goes back at once**, and is not held across the wait.
        let _ = draw.the_channel_ended(std::time::Instant::now());
        // **What went down under it: the line, or `Servers` S14's additional
        // connection alone** — the dead-line sweep's extra-connection repair, 2026-10-19.
        // Until then only the first was looked for, so a file the additional
        // connection's own death cut ended though the primary stood and the
        // next transfer would have opened a fresh one (`Servers` S14
        // condition 6).
        let its_connection = subsystem.the_channel().the_connection_it_sits_on();
        let its_own_connection_went = match what_went_down(sessions, &its_connection).await {
            WhatWentDown::Nothing => return ending,
            WhatWentDown::ItsOwnConnection => true,
            WhatWentDown::TheLine => {
                let back =
                    while_the_line_is_down(sessions, the_run, request, Some(entry), tell).await;
                // **Stopped while it waited for the connection** (`File
                // Browser` U6): the record goes from *interrupted* to
                // *stopped*, and with no channel the part's removal is owed.
                // Step 10c's transfer wave piece 7.
                if the_run.the_user_stopped_this(request, landing) {
                    an_upload_the_user_stopped_is_written_down::<ALentChannel<C>>(
                        arranged,
                        None,
                        record.as_ref(),
                    )
                    .await;
                    return THE_USERS_STOP;
                }
                if !back {
                    return ending;
                }
                false
            }
        };
        // **Waiting for its own connection from the moment it was cut**,
        // until it is seated again or ends.
        let waiting = its_own_connection_went
            .then(|| ItWaitsForItsOwnConnection::begins(the_run, request, Some(entry), tell));
        let (fresh, seated) = loop {
            if let Some(seated) =
                a_slot_and_a_seat(class, sessions, the_run, (request, Some(entry), tell)).await
            {
                break seated;
            }
            if the_run.the_user_stopped_this(request, landing) {
                an_upload_the_user_stopped_is_written_down::<ALentChannel<C>>(
                    arranged,
                    None,
                    record.as_ref(),
                )
                .await;
                return THE_USERS_STOP;
            }
            // **The fresh additional connection could not be made yet**, the
            // primary still up: asked again after a pause, the file reading
            // *waiting for the connection* meanwhile (the dead-line sweep's
            // extra-connection repair). A seat refused after a repair of the line ends the
            // file, as before.
            if let Some(waiting) = &waiting {
                if sessions.a_connection_is_live() && waiting.pauses().await {
                    continue;
                }
            }
            if the_run.the_user_stopped_this(request, landing) {
                an_upload_the_user_stopped_is_written_down::<ALentChannel<C>>(
                    arranged,
                    None,
                    record.as_ref(),
                )
                .await;
                return THE_USERS_STOP;
            }
            return ending;
        };
        drop(waiting);
        if !the_record_is_carried_on(arranged, &way_back.this_run) {
            return ending;
        }
        // **A start-over made before the first carry is that carry's mark
        // and no later one's**, as the loop's own `Server protocol and transfers`
        // G4a mark is: a repair carries
        // on from the fresh attempt's checkpoint.
        watcher.started_over = None;
        *draw = fresh;
        *subsystem = seated;
    }
}

/// **`Server protocol and transfers` S12b: the server's own program, asked for
/// the digest of the part an upload staged** — §1.1 item 1's digest over a
/// byte range, the request the dependency check's contents reading already
/// sends, answered by a file-inspection worker at the server's disk speed.
/// Uploads carry on, 2026-11-14.
///
/// **Bounded as that reading is**: a minute on each ask, half an hour over the
/// whole while the server says it is still working. **Anything but a digest of
/// exactly the length asked is no answer**, and the loop then reads the part
/// back as before — which is what a server whose program predates the request
/// gets, since it says it does not take it.
pub(crate) struct TheServersOwnProgram<'s, C: AHeldConnection> {
    pub(crate) sessions: &'s ThisServersSessions<C>,
    pub(crate) package_area: &'s str,
    pub(crate) the_stop: TheStopOfThisFile<'s>,
}

/// **The user's Stop of one file, seen while the server checks its part.**
/// The check draws nothing, so the draw's own stop
/// ([`crate::downloading::TheStopHoldsTheDraw`]) would land only once it was
/// over. Seen here, the check's operation is stopped on the server, the loop
/// is marked as at its pause, and the check waits there to be ended — the
/// same pause a stopped draw makes, with nothing half-asked on the transfer's
/// own channel.
pub(crate) struct TheStopOfThisFile<'s> {
    pub(crate) the_run: &'s TheRunsTransfers,
    pub(crate) request: &'s str,
    pub(crate) landing: &'s [u8],
    pub(crate) parked: &'s std::sync::atomic::AtomicBool,
}

impl TheStopOfThisFile<'_> {
    fn it_was_stopped(&self) -> bool {
        let stopped = self.the_run.the_user_stopped_this(self.request, self.landing);
        if stopped {
            self.parked.store(true, std::sync::atomic::Ordering::SeqCst);
        }
        stopped
    }
}

impl<C: AHeldConnection> TheMachineDigestsAStagedPart for TheServersOwnProgram<'_, C> {
    fn the_digest_of_the_first(
        &mut self,
        path: &[u8],
        up_to: u64,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Option<smoozy_server_protocol::transfers::integrity::Digest>> + Send + '_>>
    {
        let asked = the_parts_digest_asked(path, up_to);
        Box::pin(async move {
            use smoozy_dependencies::acting::checking;
            let asked = asked?;
            let mut machine = crate::reading_the_file::ThisServersMachine {
                sessions: self.sessions,
                package_area: self.package_area,
                server: "",
            };
            let ended = checking::asked_until_it_ends(
                &mut machine,
                &asked,
                checking::THE_LONGEST_AN_ASK_WAITS,
                Some(checking::THE_OUTER_LIMIT_ON_A_CONTENTS_READING_OR_A_FIXING),
                checking::BETWEEN_TWO_ASKS_OF_ONE_READING,
                &mut || self.the_stop.it_was_stopped(),
            )
            .await;
            // **Stopped: wait at the pause** to be ended by
            // [`crate::downloading::the_stop_lands_or`], rather than go on to a
            // read-back the user has already stopped.
            if matches!(ended, Err(checking::TheReadingDidNotRun::TheUserStoppedIt)) {
                std::future::pending::<()>().await;
            }
            the_parts_digest_in(&ended.ok()?.fields, up_to)
        })
    }
}

/// **The one request**: the staged part by its path, as a protocol field and
/// never a word of a command, from its first byte for `up_to` bytes.
fn the_parts_digest_asked(path: &[u8], up_to: u64) -> Option<smoozy_server_protocol::framing::Request> {
    use smoozy_server_protocol::framing::{Fields, Request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS};
    use smoozy_server_protocol::requests as asked;
    Some(
        Request::speaking(
            THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
            Identity::issue(IdentityKind::Operation).ok()?,
            asked::THE_DIGEST_OVER_A_BYTE_RANGE_REQUEST,
        )
        .carrying(
            Fields::none()
                .and(asked::THE_DIGESTED_FILE_FIELD, path)
                .and(asked::THE_RANGE_STARTS_AT_FIELD, "0")
                .and(asked::THE_RANGE_LENGTH_FIELD, up_to.to_string()),
        ),
    )
}

/// **The answer, read** — a digest of exactly the length asked, and nothing
/// else counts.
fn the_parts_digest_in(
    fields: &smoozy_server_protocol::framing::Fields,
    up_to: u64,
) -> Option<smoozy_server_protocol::transfers::integrity::Digest> {
    match smoozy_dependencies::acting::machine::what_the_digest_came_to(fields) {
        smoozy_dependencies::readings::MachineContents::Read(read) if read.size == up_to => Some(read.digest),
        _ => None,
    }
}

/// **One attempt of the loop — or `None` where the user stopped the file and
/// the loop came to its pause** (`Server protocol and transfers` C22, through
/// `crate::downloading::TheStopHoldsTheDraw`). Step 10c's transfer wave piece 7.
#[allow(clippy::too_many_arguments)]
async fn carried<S, L, K, C>(
    subsystem: &mut Subsystem<S>,
    local: &mut L,
    draw: &mut crate::scheduling::TheChannelsDraw,
    keeper: &mut K,
    watcher: &mut TheRowIsTold<'_>,
    intent: Intent,
    arranged: &mut TransferRecord,
    gate: &mut TheSourcesPublicationGate<'_, ThePlatformFileAdapter>,
    (the_run, request, landing): (&TheRunsTransfers, &str, &[u8]),
    fresh: bool,
    the_servers_program: Option<(&ThisServersSessions<C>, &str)>,
) -> Option<Ending>
where
    S: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin + Send,
    L: TheUsersOwnComputer,
    K: KeepsTheRecord,
    C: AHeldConnection,
{
    let parked = std::sync::atomic::AtomicBool::new(false);
    // **The user's Stop reaches the server's check too** (after Sol's look):
    // the check draws nothing, so the draw's own stop would wait for it.
    let mut the_servers_own = the_servers_program.map(|(sessions, package_area)| TheServersOwnProgram {
        sessions,
        package_area,
        the_stop: TheStopOfThisFile { the_run, request, landing, parked: &parked },
    });
    let mut budget = TheStopHoldsTheDraw {
        draw,
        the_run,
        request,
        landing,
        parked: &parked,
    };
    let transfer = Transfer {
        subsystem,
        local,
        budget: &mut budget,
        keeper,
        watcher,
        inactivity_deadline: THE_INACTIVITY_DEADLINE,
        intent,
    };
    // **A fresh file takes `Server protocol and transfers` §7's loop without
    // its three asks before the staging object** —
    // `Server protocol and transfers` S12a, step 10c's owed piece 1.
    let happened = if fresh {
        the_stop_lands_or(
            carry_out_a_fresh_file_under_a_publication_gate(transfer, arranged, gate),
            &parked,
        )
        .await
    } else if let Some(machine) = the_servers_own.as_mut() {
        // **`Server protocol and transfers` S12b**: a carry-on whose part the
        // server's own program checks, reading none of it back.
        the_stop_lands_or(
            carry_on_under_a_publication_gate_asking_the_machine(transfer, arranged, gate, machine),
            &parked,
        )
        .await
    } else {
        the_stop_lands_or(
            carry_out_under_a_publication_gate(transfer, arranged, gate),
            &parked,
        )
        .await
    };
    happened.map(|happened| happened.ending)
}

/// **`File Browser` N3, over the reading session: every arriving folder the
/// walk did not see is made plainly, parent first.** Answers the folders that
/// could not be made, so the files beneath them fail by name and the rest of
/// the request lands (`File Browser` N5).
///
/// **`File Browser` U6: the user's Stop ends the making at the next folder**,
/// and the folders already made stay, as files already landed do. **So each
/// folder is its own reading**, and the stop is read before each. The case this
/// was first written for was a few folders made in one reading; the case it now
/// also meets is a folder of 156,557 files, whose folders took minutes at a
/// round trip of 0.2 s, with nothing able to stop them (the twenty-eighth notes,
/// piece 12). Where Stop ends the making, the folders not made are not said to
/// have failed: every file of the request is then never arranged, as `File Browser` U6 says.
async fn the_landing_folders_are_made<C: AHeldConnection>(
    ready: &AnUploadReadyToRun,
    sessions: &ThisServersSessions<C>,
    stopped: &(dyn Fn() -> bool + Sync),
) -> Vec<Vec<u8>> {
    let seen: Vec<Vec<u8>> = ready
        .the_walk
        .comparisons
        .iter()
        .filter(|one| one.what == already_exists::Comparison::Merges)
        .map(|one| one.landing_path.clone())
        .collect();
    let to_make: Vec<Vec<u8>> = ready
        .arriving
        .iter()
        .filter(|thing| thing.is_a_folder)
        .map(|thing| thing.landing_path(&ready.request.destination))
        .filter(|folder| {
            what_to_do_with(folder, &seen) == WhatToDoWithALandingFolder::MakeItPlainly
        })
        .collect();
    let mut failed: Vec<Vec<u8>> = Vec::new();
    for folder in &to_make {
        if stopped() {
            break;
        }
        if failed.iter().any(|above| is_inside(folder, above)) {
            failed.push(folder.clone());
            continue;
        }
        let making = folder.clone();
        // **A folder whose reading waits behind another's when Stop is pressed
        // is given up** — the wait ends at the press, and the reading, when its
        // turn comes, makes nothing (Sol's fresh look on piece 12).
        let given_up = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let it_is_given_up = Arc::clone(&given_up);
        let reading = sessions.a_reading(true, move |subsystem| {
            Box::pin(async move {
                if it_is_given_up.load(std::sync::atomic::Ordering::SeqCst) {
                    return Ok(None);
                }
                match subsystem.make_one_folder(&making).await {
                    Err(WhatTheMachineSaidAboutAFolder::TheChannelIsGone) => Err(
                        smoozy_server_protocol::subsystem::SubsystemFailure::TheChannelIsGone,
                    ),
                    said => Ok(Some(said)),
                }
            })
        });
        let Some(said) = crate::the_found_files_sent::unless_a_stop_is_asked(reading, &mut || stopped()).await else {
            given_up.store(true, std::sync::atomic::Ordering::SeqCst);
            break;
        };
        let said = match said {
            Ok(Ok(Some(said))) => Ok(Ok(said)),
            Ok(Ok(None)) => break,
            Ok(Err(failure)) => Ok(Err(failure)),
            Err(not_lent) => Err(not_lent),
        };
        match said {
            Ok(Ok(Ok(()))) => {}
            // **The race of `File Browser` X4 surfacing early**: reported on
            // that branch's files, nothing overwritten.
            Ok(Ok(Err(
                WhatTheMachineSaidAboutAFolder::ItIsAlreadyThere
                | WhatTheMachineSaidAboutAFolder::TheLoginMayNot
                | WhatTheMachineSaidAboutAFolder::ItRefused
                | WhatTheMachineSaidAboutAFolder::TheChannelIsGone,
            ))) => failed.push(folder.clone()),
            // **No folder could be made at all**, so every one of them failed,
            // as when they were made in one reading.
            _ => return to_make,
        }
    }
    failed
}

// ---------------------------------------------------------------------------
// The record.
// ---------------------------------------------------------------------------

/// **`Server protocol and transfers` S8's writer, through the application's own
/// record** — one short act per write, and never a second connection.
///
/// **It also carries the request's own entry for this file, at the two saves
/// where that entry moves**, so each is one save rather than two. Added
/// 2026-09-28, step 10c's transfer wave piece 1.
///
/// **The case this was first written for** is a request of a few files, where
/// a file cost its transfer record's saves and two more for its entry — one
/// when its transfer was arranged, one when it ended — and nobody could tell.
/// **The case it now also meets** is a folder of 10,000 small files, where a
/// file is little more than its saves: seven of them, at about 13 ms each at
/// the record's own settings on the desk, and the folder took 24 minutes.
/// **The entry's two moments are already saves of the transfer record**:
///
/// - **its transfer identity** (`File Browser` U1) goes down in the transfer
///   record's first save, which `Server protocol and transfers` S8 puts before
///   the first byte — the moment the old separate write stood just ahead of;
/// - **its outcome, and `File Browser` U9a's moment with it**, go down in the
///   save that writes the transfer's ending (`Server protocol and transfers`
///   C23), which is the moment the outcome became true — read through
///   [`the_units_outcome_after`], the same reading the file's end makes.
///
/// **The transfer record is never held back by the entry**: the entry's write
/// is nested inside the save and rolls back alone if it fails, and the file's
/// end then writes it the way it always did.
struct TheTransferRecords<'a> {
    record: Arc<dyn TheRecordAnUploadWrites>,
    /// **The request, behind its lock** — taken inside the record's hold and
    /// never the other way round (step 10c's eleventh-pass fix piece 4).
    request: &'a Mutex<UploadRequest>,
    /// Which of its entries this file is.
    at: usize,
    /// **The entry, with its transfer identity, is down.**
    the_entry_is_down: bool,
    /// **The outcome the ending's save wrote, and its moment.**
    the_unit_ended_in_the_save: Option<(UnitOutcome, SystemTime)>,
    /// **The run whose register holds this file as moving**, let go of in the
    /// same hold of the record as the outcome goes down — so a Transfers row
    /// composed in between never counts the file both as finished and as in
    /// flight. Step 10c's transfer wave piece 5.
    the_run: &'a TheRunsTransfers,
}

impl KeepsTheRecord for TheTransferRecords<'_> {
    fn write_it_down(&mut self, record: &TransferRecord) -> Result<(), LocalFailure> {
        // **What of the entry this save carries**, where it carries anything.
        let mut the_entry = None;
        let mut the_outcome = None;
        if self.the_unit_ended_in_the_save.is_none() {
            let ended = record.ending.as_ref().and_then(the_units_outcome_after);
            if !self.the_entry_is_down || ended.is_some() {
                let mut entry = the_request(self.request).entries[self.at].clone();
                if let Some(outcome) = ended {
                    let moment = SystemTime::now();
                    if entry.its_outcome_became(outcome.clone(), moment).is_ok() {
                        the_outcome = Some((outcome, moment));
                    }
                }
                the_entry = Some(entry);
            }
        }

        let mut written = Err(LocalFailure("the local record would not open".into()));
        let mut the_entry_went_down = false;
        let (request, at, the_run) = (self.request, self.at, self.the_run);
        self.record.with_the_record(&mut |connection| {
            let failed = |failure: String| LocalFailure(failure);
            if let Err(error) = connection.execute_batch("SAVEPOINT a_file_moved") {
                written = Err(failed(format!("{error:?}")));
                return;
            }
            match crate::record::transfer_records::put(connection, record) {
                Ok(()) => {
                    let request = the_request(request);
                    if let Some(entry) = &the_entry {
                        the_entry_went_down = UploadRequests::over(connection)
                            .this_entry_moved(&request, at, entry)
                            .is_ok();
                    }
                    if the_entry_went_down && the_outcome.is_some() {
                        the_run.a_file_stopped(&request.record.as_text(), at);
                    }
                    written = connection
                        .execute_batch("RELEASE a_file_moved")
                        .map_err(|error| failed(format!("{error:?}")));
                }
                Err(failure) => written = Err(failed(format!("{failure:?}"))),
            }
            if written.is_err() {
                the_entry_went_down = false;
                let _ = connection.execute_batch("ROLLBACK TO a_file_moved; RELEASE a_file_moved");
            }
        });
        if the_entry_went_down {
            self.the_entry_is_down = true;
            if the_outcome.is_some() {
                self.the_unit_ended_in_the_save = the_outcome;
            }
        }
        written
    }
}

/// `File Browser` U1's write, and every later change to the request goes down
/// through the same call.
fn the_request_is_written_down(
    record: &dyn TheRecordAnUploadWrites,
    request: &UploadRequest,
) -> bool {
    let mut written = false;
    let reached = record.with_the_record(&mut |connection| {
        written = UploadRequests::over(connection)
            .write_it_down(request)
            .is_ok();
    });
    reached && written
}

/// **One entry of the request, and the request's own row, written down — and
/// nothing else of it.** Added 2026-09-28, step 10c's transfer wave piece 1.
///
/// **The case this file was first written for** is a request of a few files,
/// and it wrote the whole request through [`the_request_is_written_down`] at
/// both of a file's moments — when its transfer is arranged and when it ends.
/// **The case it now also meets** is a folder of 10,000 files, where each of
/// those writes was 10,000 entries of about three statements each: twice per
/// file, the square of the folder's size, and all of it under the record's
/// lock. **Only the one entry has moved at either moment**, so only it goes
/// down (`UploadRequests::one_entry_moved`), in one transaction with the
/// request's own row. The whole write stays for the moments the whole request
/// changes: `File Browser` U1's first write and `File Browser` U4's answer.
///
/// **The request is taken inside the record's hold and never the other way
/// round** (see [`TheUploadRunning`]).
fn the_entry_is_written_down(
    record: &dyn TheRecordAnUploadWrites,
    request: &Mutex<UploadRequest>,
    at: usize,
) -> bool {
    let mut written = false;
    let reached = record.with_the_record(&mut |connection| {
        written = UploadRequests::over(connection)
            .one_entry_moved(&the_request(request), at)
            .is_ok();
    });
    reached && written
}

/// **A unit's outcome, and `File Browser` U9a's moment with it**, written in
/// the same act — the one entry, not the whole request.
fn the_unit_ended(
    request: &Mutex<UploadRequest>,
    at: usize,
    outcome: UnitOutcome,
    record: &dyn TheRecordAnUploadWrites,
) {
    let moved = the_request(request).entries[at]
        .its_outcome_became(outcome, SystemTime::now())
        .is_ok();
    if moved {
        the_entry_is_written_down(record, request, at);
        record.a_transfer_moved();
    }
}

// ---------------------------------------------------------------------------
// Small path work, on bytes.
// ---------------------------------------------------------------------------

fn the_comparison_for<'a>(
    comparisons: &'a [AComparison],
    landing: &[u8],
) -> Option<&'a AComparison> {
    comparisons
        .iter()
        .find(|one| paths_are_equal(&one.landing_path, landing))
}

/// The folder a landing path lands in, composed on the bytes.
/// **The staging folder beside a landing path** — `Server protocol and
/// transfers` S1, in `File Browser` X12's reserved folder.
///
/// **A file landing directly in `/` is staged in `/.smoozy-staging`.** Until
/// 2026-11-12 the folder and the name were joined with a slash whatever the
/// folder was, so an upload into `/` recorded `//.smoozy-staging/…`, a
/// leftover's *Remove* refused that path as stepping outside the staging
/// folder, and the part stayed on the server with nothing said (the live pass
/// of that date, new fault 5).
pub(crate) fn the_staging_folder_beside(landing: &[u8]) -> Vec<u8> {
    let mut staging = the_folder_holding(landing);
    if staging != b"/" {
        staging.push(b'/');
    }
    staging.extend_from_slice(THE_RESERVED_STAGING_FOLDER.as_bytes());
    staging
}

pub(crate) fn the_folder_holding(landing: &[u8]) -> Vec<u8> {
    let parts = components(landing);
    let mut folder = Vec::new();
    for part in &parts[..parts.len().saturating_sub(1)] {
        folder.push(b'/');
        folder.extend_from_slice(part);
    }
    if folder.is_empty() {
        folder.push(b'/');
    }
    folder
}

fn the_last_name(path: &[u8]) -> &[u8] {
    components(path).last().copied().unwrap_or(path)
}

fn the_last_component_of(path: &Path) -> Vec<u8> {
    path.file_name()
        .map(|name| the_bytes_of(Path::new(name)))
        .unwrap_or_else(|| the_bytes_of(path))
}

/// **A path as stored on disk** — `File Browser` N2.
#[cfg(unix)]
fn the_bytes_of(path: &Path) -> Vec<u8> {
    use std::os::unix::ffi::OsStrExt;
    path.as_os_str().as_bytes().to_vec()
}

#[cfg(not(unix))]
fn the_bytes_of(path: &Path) -> Vec<u8> {
    path.to_string_lossy().into_owned().into_bytes()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scheduling::{BytesPerSecond, TheAdmissibleCount, TheCap};

    const KIB: u64 = 1024;

    /// **`File Browser`'s upload-refusal rule, 189**: a failed file whose
    /// failure-time reading showed no space says the server disk is full and
    /// may be retried; a later success, or a reading that showed space, says
    /// nothing of the disk. Batch 2 of the Bad what-if answers.
    #[test]
    fn bad_batch_2_a_failed_upload_on_a_full_disk_says_so() {
        let request = Identity::issue(IdentityKind::Operation).unwrap();
        let landing = b"/home/user/scenes/big.blend";
        assert_ne!(why_this_upload_failed(&request, landing), WhyTheUploadFailed::ServerDiskFull);
        the_server_disk_was_full_for(&request.as_text(), landing, true);
        let why = why_this_upload_failed(&request, landing);
        assert_eq!(why, WhyTheUploadFailed::ServerDiskFull);
        assert!(why.a_retry_could_help());
        the_server_disk_was_full_for(&request.as_text(), landing, false);
        assert_eq!(why_this_upload_failed(&request, landing), WhyTheUploadFailed::NoReasonIsKnown);
    }

    /// **The Rough what-if answers, share 2 (188): a file that could not be
    /// read on this computer at its failure says so**, and may be retried once
    /// the drive is back. A source that reads is no reason.
    #[test]
    fn rough_share_2_a_source_unreadable_at_the_failure_is_said_as_this_computers() {
        let folder = std::env::temp_dir().join(format!("smoozy-rough-2-unread-{}", std::process::id()));
        std::fs::create_dir_all(&folder).expect("a scratch folder");
        let there = folder.join("scene.blend");
        std::fs::write(&there, b"BLENDER").expect("a source");
        assert!(!the_source_could_not_be_read_here(&there.display().to_string()));
        assert!(the_source_could_not_be_read_here(&folder.join("gone.blend").display().to_string()));
        let _ = std::fs::remove_dir_all(&folder);

        let request = Identity::issue(IdentityKind::Operation).unwrap();
        let landing = b"/home/user/scenes/from-a-stick.blend";
        the_source_was_unreadable_for(&request.as_text(), landing, true);
        let why = why_this_upload_failed(&request, landing);
        assert_eq!(why, WhyTheUploadFailed::ItCouldNotBeReadHere);
        assert!(why.a_retry_could_help());
        the_source_was_unreadable_for(&request.as_text(), landing, false);
        assert_eq!(why_this_upload_failed(&request, landing), WhyTheUploadFailed::NoReasonIsKnown);
    }

    /// **R17, by name: an upload holding the single upward slot at a low cap
    /// starves the class behind it — and this file never holds it.**
    ///
    /// The brief that asked for this said *"starves every download behind
    /// it"*. **At the desktop point that cannot happen**, and the second half
    /// of this test says so: R1 gives uploads and downloads two budgets, and a
    /// held upward draw leaves the downward slot free. **The class an upload can
    /// starve here is another upward one** — a submission, which `Rendering` §7
    /// declares against the same list — so that is the class driven.
    #[test]
    fn an_upload_holding_the_single_slot_at_a_low_cap_starves_the_class_behind_it() {
        let upload = request::the_priority_class();
        let submission = "submission";
        let run = TheRunsTransfers::for_this_run();
        run.a_cap_was_saved(Direction::Up, TheCap::Of(BytesPerSecond(16 * KIB)));
        run.a_cap_was_saved(Direction::Down, TheCap::Of(BytesPerSecond(16 * KIB)));
        assert_eq!(
            run.the_admissible_count(Direction::Up),
            TheAdmissibleCount::Of(1)
        );

        // **The failure, watched.** An upload that took the slot for its whole
        // request: the submission is refused at every decision, and nothing
        // anywhere says so (R13).
        let held = run
            .admit_to_carry(Direction::Up, &[upload], upload)
            .expect("the first file is admitted");
        for _ in 0..5 {
            assert!(
                run.admit(Direction::Up, &[submission]).is_none(),
                "a held slot let a second channel in, so this is not the failure R17 names"
            );
        }
        // **And a download is not what it starves**: the downward budget is a
        // different budget (R1).
        let a_download = run
            .admit(Direction::Down, &["manual-download"])
            .expect("a held upward draw leaves the downward slot free");
        drop(a_download);
        drop(held);

        // **What this file does instead: the slot per file, and a grant made
        // to the class behind is handed back unspent.** With the slot given
        // back, the submission — which waited through five decisions — has the
        // turn, and the upload's own ask for its next file is refused with the
        // class named rather than served.
        let handed_back = run.admit_to_carry(Direction::Up, &[upload, submission], upload);
        assert_eq!(handed_back.err(), Some(Some(submission)));
        let next = run
            .admit(Direction::Up, &[upload, submission])
            .expect("the slot is free again");
        assert_eq!(
            next.the_class(),
            submission,
            "at a count of one the class behind the upload takes the next turn"
        );
        drop(next);
        let again = run
            .admit_to_carry(Direction::Up, &[upload, submission], upload)
            .expect("and then the upload's next file");
        assert_eq!(again.the_class(), upload);
    }

    /// **R17 kept structurally: this file asks for the slot inside the call
    /// for one file, and gives it back there.** A draw anywhere else would be
    /// the request holding the slot, whatever its comments say.
    /// **`File Browser` U8's paragraph of 2026-11-12: a part is carried on
    /// only where the file's size now is the size the part was arranged at,
    /// and a start-over takes the size as it now stands.** The Fable review's
    /// seventh sitting, fault 2.
    ///
    /// **The fault as it stood.** The upload was cut inside its publication,
    /// so the part's record says *publication pending* and its last
    /// checkpoint covers the whole file as it was. The file on this computer
    /// then grew at its end. Its first bytes still digest to that checkpoint,
    /// so the part was carried on, the pending publication was retried with
    /// no check of the whole file, and the older, shorter copy was published
    /// and read as uploaded. **And where a changed file did start over, the
    /// old size went with it**, so the upload of a file saved at another size
    /// ended *the file changed during upload*.
    /// **`File Browser` K10a: a part a later request takes up and cannot
    /// carry on says why** — the file on this computer changed (another size,
    /// or the same size with other bytes), or the part could not be carried on
    /// for another reason (no checkpoint to go on from). Uploads carry on.
    #[test]
    fn uploads_carry_on_a_part_whose_file_changed_says_the_file_changed() {
        use smoozy_server_protocol::transfers::integrity::Digest;
        use smoozy_server_protocol::transfers::{Checkpoint, StartOverCause};

        let folder = std::env::temp_dir().join(format!("smoozy-carry-on-cause-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a scratch folder");
        let file = folder.join("scene.blend");
        let as_sent = vec![7u8; 100_000];
        std::fs::write(&file, &as_sent).expect("the file as it was sent");
        let issue = |kind| Identity::issue(kind).expect("random bytes");
        let mut part = TransferRecord::arranged(
            issue(IdentityKind::Transfer),
            Direction::Up,
            issue(IdentityKind::LocalServer),
            Some(issue(IdentityKind::Installation)),
            file.to_string_lossy().into_owned(),
            "/srv/job/scene.blend".to_string(),
            "/srv/job/.smoozy-staging",
            as_sent.len() as u64,
            issue(IdentityKind::Run),
        );
        let mut none_to_go_on_from = part.clone();
        part.checkpoints.push(Checkpoint {
            offset: 65_536,
            digest_of_everything_up_to_it: Digest::of(&as_sent[..65_536]),
            running_state: None,
        });
        part.ending = Some(Ending::Interrupted);

        // Saved at the same size with other bytes.
        std::fs::write(&file, vec![8u8; 100_000]).expect("saved again");
        let mut changed = part.clone();
        assert_eq!(
            a_part_taken_up_is_made_ready(&mut changed, 100_000),
            Some(StartOverCause::TheFileChangedOnThisComputer)
        );
        // Saved at another size.
        let mut grown = part.clone();
        assert_eq!(
            a_part_taken_up_is_made_ready(&mut grown, 120_000),
            Some(StartOverCause::TheFileChangedOnThisComputer)
        );
        // Nothing to go on from: not a change anybody saw.
        std::fs::write(&file, &as_sent).expect("as it was");
        assert_eq!(
            a_part_taken_up_is_made_ready(&mut none_to_go_on_from, 100_000),
            Some(StartOverCause::TheStagedPrefixDidNotVerify)
        );
        let _ = std::fs::remove_dir_all(&folder);
    }

    /// **`Server protocol and transfers` S12b: the server's own program is
    /// asked for the part from its first byte to the checkpoint**, the path as
    /// a field, and only a digest of exactly that length is an answer.
    #[test]
    fn uploads_carry_on_the_servers_program_is_asked_for_the_part_from_its_start() {
        use smoozy_server_protocol::framing::Fields;
        use smoozy_server_protocol::requests as asked;
        use smoozy_server_protocol::transfers::integrity::Digest;

        let request = the_parts_digest_asked(b"/home/ubuntu/jobs/.smoozy-staging/t-1.part", 8_388_608)
            .expect("composed");
        assert_eq!(request.name, asked::THE_DIGEST_OVER_A_BYTE_RANGE_REQUEST);
        assert_eq!(
            request.fields.value(asked::THE_DIGESTED_FILE_FIELD),
            Some(&b"/home/ubuntu/jobs/.smoozy-staging/t-1.part"[..])
        );
        assert_eq!(request.fields.text(asked::THE_RANGE_STARTS_AT_FIELD), Some("0"));
        assert_eq!(request.fields.text(asked::THE_RANGE_LENGTH_FIELD), Some("8388608"));

        let digest = Digest::of(b"the part");
        let digested = |bytes: &str| {
            Fields::none()
                .and(asked::WHAT_BECAME_OF_IT_FIELD, asked::IT_HAS_A_RECORDED_OUTCOME)
                .and(asked::THE_DIGESTS_OUTCOME_FIELD, asked::IT_WAS_DIGESTED)
                .and(asked::THE_DIGEST_FIELD, digest.as_text())
                .and(asked::THE_BYTES_DIGESTED_FIELD, bytes.to_string())
        };
        assert_eq!(the_parts_digest_in(&digested("8388608"), 8_388_608), Some(digest.clone()));
        assert_eq!(the_parts_digest_in(&digested("100"), 8_388_608), None, "another length is no answer");
        let not = Fields::none()
            .and(asked::WHAT_BECAME_OF_IT_FIELD, asked::IT_HAS_A_RECORDED_OUTCOME)
            .and(asked::THE_DIGESTS_OUTCOME_FIELD, asked::IT_WAS_NOT_DIGESTED);
        assert_eq!(the_parts_digest_in(&not, 8_388_608), None);

        // **The user's Stop reaches the check** (after Sol's look): the ask
        // hears it, and a stopped check waits at the pause the stop ends
        // rather than going on to a read-back.
        let working = include_str!("uploading.rs")
            .split("#[cfg(test)]\nmod tests {")
            .next()
            .expect("the working half");
        assert!(working.contains("&mut || self.the_stop.it_was_stopped(),"));
        assert!(working.contains("TheReadingDidNotRun::TheUserStoppedIt)) {\n                std::future::pending::<()>().await;"));
    }

    #[test]
    fn a_part_is_carried_on_only_where_the_file_is_the_size_it_was_arranged_at() {
        use smoozy_server_protocol::transfers::integrity::Digest;
        use smoozy_server_protocol::transfers::publication::PublicationPreState;
        use smoozy_server_protocol::transfers::{Checkpoint, StableFileIdentity};

        let folder = std::env::temp_dir().join(format!("smoozy-sitting7-a-part-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a scratch folder");
        let file = folder.join("cache.abc");
        let as_sent = vec![7u8; 100_000];
        std::fs::write(&file, &as_sent).expect("the file as it was sent");

        let issue = |kind| Identity::issue(kind).expect("random bytes");
        let mut part = TransferRecord::arranged(
            issue(IdentityKind::Transfer),
            Direction::Up,
            issue(IdentityKind::LocalServer),
            Some(issue(IdentityKind::Installation)),
            file.to_string_lossy().into_owned(),
            "/srv/job/cache.abc".to_string(),
            "/srv/job/.smoozy-staging",
            as_sent.len() as u64,
            issue(IdentityKind::Run),
        );
        part.checkpoints.push(Checkpoint {
            offset: as_sent.len() as u64,
            digest_of_everything_up_to_it: Digest::of(&as_sent),
            running_state: None,
        });
        part.publication_pending = Some(PublicationPreState {
            intent: Intent::CreateIntended,
            staging_path: part.staging_path.as_text(),
            final_path: part.final_path.clone(),
            staged_identity: StableFileIdentity::TheSupplierCouldNotAnswer,
            staged_byte_length: as_sent.len() as u64,
            staged_digest: Digest::of(&as_sent),
        });
        part.ending = Some(Ending::Interrupted);

        // The file as it was: the part is carried on, and nothing moves.
        let mut unchanged = part.clone();
        assert_eq!(a_part_taken_up_is_made_ready(&mut unchanged, as_sent.len() as u64), None);
        assert_eq!(unchanged, part);

        // The file grew at its end.
        let mut grown = as_sent.clone();
        grown.extend_from_slice(&[9u8; 50_000]);
        std::fs::write(&file, &grown).expect("the file as it is now");
        let mut taken_up = part.clone();
        let started_over = a_part_taken_up_is_made_ready(&mut taken_up, grown.len() as u64);
        let _ = std::fs::remove_dir_all(&folder);
        assert!(
            started_over.is_some(),
            "a part left in publication pending was carried on though the file has grown: the \
             older, shorter copy is published and reads as uploaded"
        );
        assert_eq!(taken_up.publication_pending, None);
        assert_eq!(taken_up.attempt, 2);
        assert!(taken_up.checkpoints.is_empty());
        assert_eq!(
            taken_up.total_size_expected,
            grown.len() as u64,
            "the start-over kept the size the part was arranged at"
        );
    }

    #[test]
    fn the_slot_is_asked_for_inside_one_files_call_and_nowhere_else() {
        let working = include_str!("uploading.rs")
            .split("#[cfg(test)]\nmod tests {")
            .next()
            .expect("the working half");
        let asked: Vec<usize> = working
            .match_indices(".admit_to_carry(")
            .map(|(at, _)| at)
            .collect();
        assert_eq!(asked.len(), 1, "the slot is asked for in one place");
        let the_call = working
            .find("async fn one_file_goes_up<")
            .expect("one file's own call");
        let after_it = working[the_call..]
            .find("\nasync fn carried<")
            .map(|end| the_call + end)
            .expect("the call ends");
        assert!(
            the_call < asked[0] && asked[0] < after_it,
            "the slot is asked for outside the call for one file"
        );
        assert!(working[the_call..after_it].contains("drop(draw);"));
    }

    /// **`File Browser` N2 and `File Browser` X1's local side: a folder goes up
    /// as a folder,
    /// names as their bytes, and every file lands under the destination
    /// component for component.**
    #[test]
    fn a_picked_folder_is_enumerated_file_by_file_with_its_structure() {
        let at = std::env::temp_dir().join(format!("smoozy-uploading-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&at);
        std::fs::create_dir_all(at.join("kitchen/textures")).expect("made");
        std::fs::write(at.join("kitchen/scene.blend"), b"1234").expect("written");
        std::fs::write(at.join("kitchen/textures/wood.png"), b"12").expect("written");
        std::fs::write(at.join("notes.txt"), b"1").expect("written");

        let (arriving, entries) =
            what_was_picked(&[at.join("kitchen"), at.join("notes.txt")], b"/home/artist")
                .expect("it is all readable");
        let landings: Vec<String> = entries
            .iter()
            .map(|entry| String::from_utf8_lossy(&entry.landing_path).into_owned())
            .collect();
        assert_eq!(
            landings,
            vec![
                "/home/artist/kitchen/scene.blend",
                "/home/artist/kitchen/textures/wood.png",
                "/home/artist/notes.txt",
            ]
        );
        assert_eq!(entries[0].size_as_enumerated, 4);
        assert_eq!(arriving.iter().filter(|thing| thing.is_a_folder).count(), 2);
        assert!(
            arriving[0].is_a_folder,
            "a folder comes before what is inside it"
        );
        let _ = std::fs::remove_dir_all(&at);
    }

    /// **Test 5 of the *Show hidden files* plan text, the local half: an
    /// uploaded folder sends its dot-files and dot-folders like anything
    /// else** — `File Browser` L21: *"an upload sends a hidden local file like
    /// any other (`File Browser` N2)"*, whatever the box says, because nothing here reads it.
    #[test]
    fn an_uploaded_folder_sends_its_hidden_files() {
        let at = std::env::temp_dir().join(format!("smoozy-uploading-hidden-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&at);
        std::fs::create_dir_all(at.join("kitchen/.cache")).expect("made");
        std::fs::write(at.join("kitchen/.env"), b"12").expect("written");
        std::fs::write(at.join("kitchen/.cache/bake.bin"), b"123").expect("written");
        std::fs::write(at.join("kitchen/scene.blend"), b"1234").expect("written");

        let (arriving, entries) = what_was_picked(&[at.join("kitchen")], b"/home/artist").expect("readable");
        let landings: Vec<String> = entries
            .iter()
            .map(|entry| String::from_utf8_lossy(&entry.landing_path).into_owned())
            .collect();
        assert_eq!(
            landings,
            vec![
                "/home/artist/kitchen/.cache/bake.bin",
                "/home/artist/kitchen/.env",
                "/home/artist/kitchen/scene.blend",
            ]
        );
        assert_eq!(arriving.iter().filter(|thing| thing.is_a_folder).count(), 2, "the hidden folder went missing");
        let _ = std::fs::remove_dir_all(&at);
    }

    #[test]
    fn the_folder_a_path_lands_in_is_composed_on_the_bytes() {
        assert_eq!(the_folder_holding(b"/home/a/b.blend"), b"/home/a".to_vec());
        assert_eq!(the_folder_holding(b"/b.blend"), b"/".to_vec());
        // **Staged beside it, and never under a doubled slash** (the live
        // pass of 2026-11-12, new fault 5).
        assert_eq!(the_staging_folder_beside(b"/home/a/b.blend"), b"/home/a/.smoozy-staging".to_vec());
        assert_eq!(the_staging_folder_beside(b"/b.blend"), b"/.smoozy-staging".to_vec());
        assert_eq!(the_last_name(b"/home/a/b.blend"), b"b.blend");
    }

    /// **A Stop pressed at any moment either stops the comparison or reaches
    /// the running upload** — Sol's one look at the fixes after 0.4.0's second
    /// try. A Stop looked at the hold and then, as a second step, marked the
    /// press; where the comparison ended and the upload went under way between
    /// the two, the press was marked where nothing read it again, and the Stop
    /// did not look again at the run's register, so the files went on. **Each
    /// of the Stop's two gaps is held here** while another thread does what the
    /// upload does as it goes under way: registered in the run, then its hold
    /// passed to the run ([`the_hold_passes_to_the_run`]).
    #[test]
    fn a_stop_pressed_as_the_comparison_ends_reaches_the_running_upload() {
        use std::sync::mpsc;
        use crate::surface::transfers::{a_stop_is_taken_wherever_the_request_stands, TheAct, WhatIsKnown};

        struct NoRecord;
        impl crate::downloading::TheRecordADownloadWrites for NoRecord {
            fn with_the_record(&self, _act: &mut dyn FnMut(&rusqlite::Connection)) -> bool {
                false
            }
        }

        let the_run = TheRunsTransfers::for_this_run();
        let server = Identity::issue(IdentityKind::LocalServer).expect("randomness");
        for moment in [BETWEEN_THE_LOOK_AND_THE_MARK, BETWEEN_THE_RUNS_LOOK_AND_THE_HOLDS] {
            let request = Identity::issue(IdentityKind::Operation).expect("randomness");
            HeldBeforeItsAnswer::from(&request).kept_for_its_answer();
            let (go, goes) = mpsc::channel::<()>();
            let (said, hears) = mpsc::channel::<&'static str>();
            let (done, its_done) = mpsc::channel::<()>();
            let passed_while_held = std::rc::Rc::new(std::cell::Cell::new(false));
            std::thread::scope(|scope| {
                // **The upload, going under way** once the Stop is held.
                let (the_run, request, server) = (&the_run, &request, &server);
                let upload = scope.spawn(move || {
                    goes.recv().expect("go");
                    let _under_way = the_run.a_request_is_under_way(request, server);
                    said.send("registered").expect("said");
                    the_hold_passes_to_the_run(the_run, request);
                    // Unheard where the Stop kept the hold until it marked
                    // the press: the moment's listener is gone by then.
                    let _ = said.send("passed");
                    its_done.recv().expect("the Stop is done");
                    the_run.the_user_stopped_it(&request.as_text())
                });
                let seen = std::rc::Rc::clone(&passed_while_held);
                A_TEST_HOLDS.with(|held| {
                    *held.borrow_mut() = Some((
                        moment,
                        Box::new(move || {
                            go.send(()).expect("go");
                            assert_eq!(hears.recv_timeout(Duration::from_secs(10)), Ok("registered"));
                            seen.set(hears.recv_timeout(Duration::from_millis(500)) == Ok("passed"));
                        }),
                    ));
                });
                a_stop_is_taken_wherever_the_request_stands(
                    the_run,
                    &WhatIsKnown::default(),
                    &NoRecord,
                    TheAct::StopTheRequest,
                    &request.as_text(),
                    None,
                );
                A_TEST_HOLDS.with(|held| *held.borrow_mut() = None);
                done.send(()).expect("done");
                let stopped = upload.join().expect("the upload");
                assert!(stopped, "a Stop held {moment:?} did not reach the running upload");
            });
            if moment == BETWEEN_THE_LOOK_AND_THE_MARK {
                assert!(
                    !passed_while_held.get(),
                    "the upload went under way between the Stop's look at the hold and its mark"
                );
            }
        }
    }
}

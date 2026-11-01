//! **The first caller of the two rate budgets and the one desktop-side
//! admission point, and the first reader of what Settings saves.**
//!
//! `src/scheduling.rs` holds the budgets, the floor, the admissible count, the
//! admission point, the turn-taking where that count is one, and the
//! achieved-rate reading. Its own header says what was missing: *"nothing in
//! the running application arranges an upload yet, so what is built is the
//! budget object and the admission point with their real behaviour, and who
//! draws on them arrives with the screens."* **This file is who draws on
//! them.**
//!
//! `src/record/settings.rs` reads and writes the two caps and the chosen
//! download folder. **Until this file nothing asked it for one**, so a cap the
//! user set was saved, drawn back on Settings and on Transfers, and bound
//! nothing. R3 says the setting is durable and user-held and the budget object
//! is not, so what joins them is a read at the start and a write at the moment
//! Settings saves — never a read at the moment a channel is admitted, which
//! would leave a cap changed mid-transfer reaching nothing already running
//! (`Server protocol and transfers` R8).
//!
//! # The clause this file makes true or false, and it is the whole risk of it
//!
//! **R17.** At a low cap the admissible count is one, and the guarantee there
//! is turn-taking between the classes with work waiting. **A first caller that
//! takes the single slot and holds it makes the point look like it works while
//! starving everything behind it** — the exact failure that clause was written
//! for, an auto-download arrangement that *"keeps trying, and never switches
//! itself off"* holding the one slot for ever while R13 makes every refusal
//! behind it silent.
//!
//! **So the slot is taken per file and given back at that file's ending, and
//! there is no route here to hold one.** [`one_file_comes_down`] admits,
//! transfers and returns; the draw is a local of that call and nothing in this
//! crate holds one in a field. A test below reads this file's own text and
//! fails if one ever does.
//!
//! # The two routes, and they are genuinely two — step 10c piece 10
//!
//! **An arrangement and a manual request are not one thing with a switch.**
//! `Download` R1's arrangement is a standing intention: made at submission
//! where the submit strip's choice is on, durable, resumed on any later
//! connection (`Download` R12), walking the job's output folder and fetching
//! what it judges finished, asking nothing of a user who is not there
//! (`Download` P7). `Download` M1's manual request is one multi-part action the
//! user confirms: enumerated, asked about once where something collides
//! (`Download` M6), run to its units' endings, and **ended by the application
//! run ending** (`Download` M13). **Nothing of the arrangement appears anywhere
//! in the manual route** — no reckoning is consulted, no window is opened, no
//! walk is started, and a test in this file reads this file's own text and
//! fails if one ever is.
//!
//! # What is not built here, said rather than left to be discovered
//!
//! **The adapter's own two judgements are still `Desktop app and delivery`'s
//! and are still unbuilt.** C10's exclusive create answers *is this name
//! already taken*, and C15 and C19 say **what** it is taken by; neither is
//! written in this crate. So a transfer that ends *stopped* reaches
//! [`the_state_a_stop_leaves`](smoozy_download::acting::running) with no taken
//! name, which makes the entry **left alone** — the file stays on the machine,
//! nothing of the user's is written over, and `Download` P6's amber line names
//! it. **That is the safe direction and it is the one this file takes
//! deliberately**, but it means *cannot land* and `Download` R7's
//! newer-version telling are never reached from here.
//!
//! **`Server protocol and transfers` S13's four values are read for a manual
//! download's file large enough to carry on from, when it is arranged and again
//! before it carries on** ([`TheFourValuesOverThePoint`], the 38's piece 9, item
//! 18), so a file cut by a dropped line resumes where the file on the server has
//! not changed. **An auto-download's fetch reads none**: a cut fetch of an
//! arrangement is given up and fetched again under `Download` P3, never carried
//! on, so a reckoning entry written here carries the account half and no
//! machine-derived half.
//!
//! **Nothing calls [`the_arrangement_is_made_at_submission`] from the submit
//! act yet.** The arrangement's own making, its durability and its three
//! refusals are built and driven here; the one line that calls it sits in
//! `src/submitting.rs`, which another agent held for the whole of this piece's
//! sitting, and it is owed.
//!
//! # How a clause marker is written in this file
//!
//! A marker with no document beside it means a clause of this crate's own
//! plan, `Preparation/Architecture/Desktop app and delivery.md`. **Every
//! marker of another document is written out with that document at its own
//! site and this file keeps no header list**, which is the shape
//! `src/servers/a_period.rs` settled: a list cannot tell a marker both plans
//! define from one only the other does, and `Download` collides with this plan
//! on almost every letter of its own series.

use std::collections::{HashMap, HashSet};
use std::future::Future;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, MutexGuard};
use std::time::{Duration, Instant};

use smoozy_download::acting::enumerating::{
    enumerate, make_the_landing_folders, MakesTheLandingFolder, ReadsTheLocalJobRecords,
};
use smoozy_download::acting::fetching::{
    arrange_one_fetch, carry_out_one_fetch, the_intent, AUnitOwed, TheFetchQueue, TheFourValues,
};
use smoozy_download::acting::probing::TheProbesCadence;
use smoozy_download::acting::running::{
    TheArrangementsRun, WhatIsInHand, WhatTheTurnDid, WhatTheUnitsFetchCameTo,
};
use smoozy_download::acting::walking::{ADirectoryEntry, ReadsOneDirectoryOnTheMachine};
use smoozy_download::acting::wiring::TheReadingRoute;
use smoozy_download::arrangement::{AnArrangement, KeepsTheArrangement, TheNarrowing, WhatEndsIt};
use smoozy_download::classes::{the_class, WhatArrangedIt};
use smoozy_download::destination::WhatTheAdapterSaid;
use smoozy_download::diary::AnAct;
use smoozy_download::fetching::{
    the_landing_path, TheNameCheckSaid, TheReceivingSystemsRules,
};
use smoozy_download::request::{
    apply_the_collision_answer, what_the_comparison_reads, what_the_pre_flight_has_to_say,
    AManualRequest, Disposition, KeepsTheRequest, ReadsTheDestinationsListing,
    WhatThePreFlightHasToSay,
};
use smoozy_file_browser::already_exists::CollisionAnswer;
use smoozy_file_browser::partial::THE_RESERVED_STAGING_FOLDER;
use smoozy_file_browser::pool::{with_a_class_a_later_document_declares, DeclaredClass};
use smoozy_server_protocol::subsystem::client::Subsystem;
use smoozy_server_protocol::subsystem::SubsystemFailure;
use smoozy_server_protocol::transfers::moving::{
    DrawsFromTheSharedBudget, KeepsTheRecord, LocalFailure, NobodyIsWatching, TheUsersOwnComputer,
    Transfer, WatchesTheTransfer,
};
use smoozy_server_protocol::transfers::publication::Intent;
use smoozy_server_protocol::transfers::{
    Direction, Ending, Report, StoppedReason, TransferRecord,
};
use smoozy_state::identities::{Identity, IdentityKind};
use smoozy_state::outcomes::{Refused, UnitOutcome};
use tokio::io::{AsyncRead, AsyncWrite};

use crate::platform::downloads_folder::{self, TheProposal};
use crate::record::download_records::{DownloadArrangements, DownloadRequests};
use crate::record::settings;
use crate::scheduling::{
    the_declared_order_with_the_update_fetch, TheAchievedRate, TheAdmissibleCount, TheCap,
    TheChannelsDraw, TheDesktopSideAdmissionPoint,
};
use crate::servers::sessions::{AHeldConnection, NotLent, TheConnectionItSatOn, ThisServersSessions};
use crate::transfers::{TheLocalRecord, ThePlatformFileAdapter};

// ---------------------------------------------------------------------------
// The declarations this point runs over.
// ---------------------------------------------------------------------------

/// **`Rendering` §7's submission class, by name.**
///
/// That document declares the class and ranks it *"below manual download,
/// above dependency-upload"*, and its own `units::THE_SUBMISSION_CLASS` holds
/// the two relations — **but it publishes no name string**, so there is
/// nothing to read the name from. It is spelled here for the same reason and
/// in the same way `src/scheduling.rs` spells the preview fetch's: the rank is
/// taken from that document's declaration and only the spelling is written
/// down.
const THE_SUBMISSION_CLASS: &str = "submission";

/// **Every class declared against `File Browser` O4's list, in one order.**
///
/// **The declarations are the caller's** — `File Browser` O4's closing
/// sentence hands the rank to the declaring document, and this is the caller —
/// so the five that document declares are taken as they stand, `Rendering`
/// §7's submission class is placed by its own relation, and R19's update fetch
/// is placed by `src/scheduling.rs`'s own route.
///
/// **`File Browser` O5's N is computed from what comes back and never read off
/// a worked figure.** With those two joined it is seven; **eight from
/// 2026-10-31**, when `File Browser` L29's picture view joined, third — below
/// manual download, above dependency-upload — placed by that document's own
/// [`smoozy_file_browser::pool::the_declared_order_with_the_picture_view`].
pub fn the_declared_order() -> Vec<DeclaredClass> {
    // The six are spaced so `Rendering` §7's relation has somewhere to land in
    // a list with no gap in it. **Nothing of `File Browser` O4's list is
    // edited and no rank of it is restated** — the spacing is undone by
    // `the_declared_order_with_the_update_fetch`, which re-reads the ranks from
    // the order it hands back.
    let spaced: Vec<DeclaredClass> = smoozy_file_browser::pool::the_declared_order_with_the_picture_view()
        .iter()
        .map(|class| DeclaredClass {
            rank: class.rank * 2,
            ..*class
        })
        .collect();
    let below_manual_download = spaced[1].rank + 1;
    let with_the_submission = with_a_class_a_later_document_declares(
        &spaced,
        DeclaredClass {
            name: THE_SUBMISSION_CLASS,
            declared_by: "Rendering",
            rank: below_manual_download,
        },
    );
    the_declared_order_with_the_update_fetch(&with_the_submission)
        .expect("R19 places the update fetch against `File Browser` O4's own list")
}

// ---------------------------------------------------------------------------
// The one point of the run, and the caps it is set from.
// ---------------------------------------------------------------------------

/// **The run's one desktop-side admission point, with the two budgets inside
/// it** — R10's *"one point that knows every server at once"*.
///
/// **R3: it is one object in process memory for the life of the application
/// run and it is not state.** Nothing durable is written from it and there is
/// no route here to write one. It is made once at the application's start and
/// reached through the shell's own managed state, exactly as the one ordered
/// queue and the run's periods are — **a second one anywhere would be a second
/// answer to *how much of the line is left*, which is the whole thing R10
/// exists to prevent.**
///
/// **It also holds what the run's uploads and manual downloads are moving
/// now** ([`WhatIsMovingNow`]), which is run-held for the same reason and is
/// the run's own answer to *what is moving*, as the point is to *how much of
/// the line is left*. Added 2026-09-29, step 10c's transfer wave piece 5.
#[derive(Debug)]
pub struct TheRunsTransfers {
    point: Mutex<TheDesktopSideAdmissionPoint>,
    moving: Mutex<WhatIsMovingNow>,
    /// **What each ended upload and manual download left staged, as the
    /// record read it** — step 10c's transfer wave piece 7. Run-held and never
    /// durable; see [`WhatARequestLeft`].
    left: Mutex<HashMap<String, WhatARequestLeft>>,
    /// **The manual downloads being counted**, in the order they were asked
    /// for — step 10c's twelfth-pass fix piece 3. Run-held and never durable;
    /// see [`ACountUnderWay`].
    counting: Mutex<Vec<(String, ACountUnderWay)>>,
    // `Server protocol and transfers` S0a's generation prevents a new arrangement inheriting an old walk.
    auto_ending_walks: Mutex<HashMap<String, (Option<i64>, bool)>>,
}

impl Default for TheRunsTransfers {
    fn default() -> TheRunsTransfers {
        TheRunsTransfers::for_this_run()
    }
}

impl TheRunsTransfers {
    /// One point over every declared class.
    pub fn for_this_run() -> TheRunsTransfers {
        TheRunsTransfers {
            point: Mutex::new(TheDesktopSideAdmissionPoint::over(the_declared_order())),
            moving: Mutex::new(WhatIsMovingNow::default()),
            left: Mutex::new(HashMap::new()),
            counting: Mutex::new(Vec::new()),
            auto_ending_walks: Mutex::new(HashMap::new()),
        }
    }

    pub(crate) fn an_auto_ending_walk_was_read(&self, job: &str, generation: Option<i64>, completed: bool) -> bool {
        let mut held = self.auto_ending_walks.lock().unwrap_or_else(|p| p.into_inner());
        held.insert(job.to_string(), (generation, completed)) != Some((generation, completed))
    }

    pub(crate) fn the_auto_ending_walk_completed(&self, job: &str, generation: Option<i64>) -> bool {
        self.auto_ending_walks.lock().unwrap_or_else(|p| p.into_inner()).get(job) == Some(&(generation, true))
    }

    fn counting(&self) -> MutexGuard<'_, Vec<(String, ACountUnderWay)>> {
        self.counting
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    /// **A manual download's count begins, and it is on Transfers until what
    /// this answers is dropped** — `Design/Transfers § A request shows the
    /// whole and the files under it`, its paragraph of 2026-10-01: *"A
    /// download of a folder is on Transfers from the moment it is asked
    /// for."* Step 10c's twelfth-pass fix piece 3.
    ///
    /// **`Download` M1 writes the request once the enumeration is complete**,
    /// so between the press and that write there is nothing durable to draw.
    /// What is held here is that moment, for the run, and nothing of it is
    /// written down. **Dropping it tells the page** through `record`, so a
    /// row that goes — because the request was written, or the count was
    /// stopped, or it ended in a refusal — is never left standing.
    pub fn a_count_begins<'a>(
        &'a self,
        request: &Identity,
        server: &Identity,
        named: String,
        destination: Vec<u8>,
        record: &'a dyn TheRecordADownloadWrites,
    ) -> TheCountIsUnderWay<'a> {
        let request = request.as_text();
        let count = ACountUnderWay {
            server: server.as_text(),
            named,
            destination,
            the_user_stopped_it: std::sync::Arc::new(AtomicBool::new(false)),
        };
        let stopped = std::sync::Arc::clone(&count.the_user_stopped_it);
        self.counting().push((request.clone(), count));
        record.a_transfer_moved();
        TheCountIsUnderWay {
            the_run: self,
            request,
            stopped,
            record,
        }
    }

    /// **Every count under way, in the order they were asked for.**
    pub fn what_is_being_counted(&self) -> Vec<(String, ACountUnderWay)> {
        self.counting().clone()
    }

    /// **The user's *Stop*, on a count under way** — `Design/Transfers`'
    /// paragraph of 2026-10-01: the counting row carries *Stop*. **Answers
    /// whether the request was being counted**; the count itself notices at
    /// its next look ([`the_stop_lands_or`]) and ends with nothing fetched
    /// and nothing written.
    pub fn the_user_stops_the_count(&self, request: &str) -> bool {
        match self.counting().iter().find(|(one, _)| one == request) {
            Some((_, count)) => {
                count.the_user_stopped_it.store(true, Ordering::SeqCst);
                true
            }
            None => false,
        }
    }

    fn held(&self) -> MutexGuard<'_, TheDesktopSideAdmissionPoint> {
        self.point
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    fn moving(&self) -> MutexGuard<'_, WhatIsMovingNow> {
        self.moving
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    /// **An upload's or a manual download's run begins**, and the request is
    /// under way until what this answers is dropped — step 10c's transfer
    /// wave piece 5. `File Browser` U7: a request's activity is run-held.
    ///
    /// **The server it moves to or from is held beside it**, so the moment
    /// that server's period ends reaches it ([`Self::a_servers_period_ended`]).
    /// Added 2026-09-29, step 10c's transfer wave piece 6.
    pub fn a_request_is_under_way(&self, request: &Identity, server: &Identity) -> TheRequestIsUnderWay<'_> {
        let request = request.as_text();
        let server = server.as_text();
        self.moving().begins(&request, &server);
        // **`Server Setup` R1b counts a transfer under way as live work**, so
        // the server's count of them moves with the request. Added
        // 2026-10-12.
        crate::servers::the_driver_swap::a_transfer_begins(&server);
        TheRequestIsUnderWay {
            the_run: self,
            request,
            server,
        }
    }

    /// **The same, over the server's own point — and where that point lends
    /// nothing at the moment the request is held, its period is taken as
    /// ended.** Added 2026-09-29, step 10c's transfer wave piece 6.
    ///
    /// **Why the point is read after the request is held and not before.** A
    /// run is only begun on a point that was lending a moment earlier
    /// (`src/surface/files.rs` and `src/surface/transfers.rs` both refuse to
    /// begin one otherwise). The one gap is a period that ends between that
    /// moment and this: its end was told to this register before the request
    /// was in it, so it reached nothing. **Reading the point after the request
    /// is held closes it**: a point still lending then belongs to a period
    /// whose end, when it comes, finds this request here; a point lending
    /// nothing may be a period that has already ended, whose point will never
    /// lend again, and waiting on it would be waiting for good. **So the
    /// request does not wait on it**, and a file the line cuts ends there,
    /// exactly as before this piece.
    pub fn a_request_goes_under_way<C: AHeldConnection>(
        &self,
        request: &Identity,
        server: &Identity,
        sessions: &ThisServersSessions<C>,
    ) -> TheRequestIsUnderWay<'_> {
        let under_way = self.a_request_is_under_way(request, server);
        if !sessions.a_connection_is_live() {
            self.moving().its_period_ended(&under_way.request);
        }
        under_way
    }

    /// **`File Browser` U7's paragraph of 2026-09-28, and `Download` M13's: a
    /// period's end does to that server's requests what closing Smoozy does.**
    /// Every upload and manual download of that server under way in this run
    /// ends there — nothing of it is arranged again and its *interrupted*
    /// records stay *interrupted*. Added 2026-09-29, step 10c's transfer wave
    /// piece 6.
    ///
    /// **Told by the period, at the moment the run's register of periods lets
    /// it go** (`crate::servers::a_period`, on the period's own thread, right
    /// after `ThePeriodsThisRun::a_period_has_ended`). **This holds no second
    /// copy of whether a period stands** — that is `Servers` K8a's one boolean,
    /// and it stays in that register. What is held here is each request's own
    /// run-held activity (`File Browser` U7), which the period's end ends.
    ///
    /// **Why it is told rather than read**, which is the one thing piece 6 had
    /// to establish first. An upload's run holds this server's point, this
    /// register and two doors, and nothing else. The point is made with the
    /// period and survives a repair, but it knows only whether a connection is
    /// live, and between a break and a repair it lends nothing exactly as it
    /// does after the period has ended. The two registers that can tell the
    /// two apart — `ThePeriodsThisRun::a_period_stands_for` and
    /// `WhatThisRunHolds::the_connection_now` — are not reachable from the
    /// run, and the first answers for the server rather than for the period:
    /// after a *Disconnect* and a fresh *Connect* it says *a period stands* of
    /// a new period whose point is another object, and an upload asking it
    /// would wait for good on a point that will never lend again.
    pub fn a_servers_period_ended(&self, server: &str) {
        self.moving().a_servers_period_ended(server);
    }

    /// **Whether this request's server's period has ended since the request
    /// began**, or the request is no longer held at all.
    pub(crate) fn its_period_has_ended(&self, request: &str) -> bool {
        self.moving()
            .requests
            .get(request)
            .is_none_or(|under_way| under_way.its_period_ended)
    }

    /// **The request waits for the connection to come back**, and where a
    /// file of it is in flight, that file waits with it — `File Browser` U5,
    /// `Download` M11. Said once as the wait begins and once as it ends.
    pub(crate) fn it_waits_for_the_connection(&self, request: &str, entry: Option<usize>, waits: bool) {
        self.moving().it_waits_for_the_connection(request, entry, waits);
    }

    /// **A file of a request under way starts moving**, with the files
    /// waiting their turn behind it.
    pub(crate) fn a_file_goes(
        &self,
        request: &str,
        entry: usize,
        path: Vec<u8>,
        expected: u64,
        waiting: Vec<Vec<u8>>,
    ) {
        self.moving().a_file_goes(request, entry, path, expected, waiting);
    }

    /// **A file stopped moving**, whatever ended it. Saying it twice is
    /// saying it once.
    pub(crate) fn a_file_stopped(&self, request: &str, entry: usize) {
        self.moving().a_file_stopped(request, entry);
    }

    /// **The user's *Stop*, on a request or on one of its files** —
    /// `Server protocol and transfers` C22: *"A stop closes the channel and
    /// stops the loop"*, and it is desktop-local. `File Browser` U6 and
    /// `Download` M12: the whole request's Stop keeps what landed, never
    /// arranges what waits, and stops what is in flight; one file's is the
    /// same act on one unit, the rest of the request untouched. Added
    /// 2026-09-29, step 10c's transfer wave piece 7.
    ///
    /// **This marks the stop in the run's register and does nothing else.**
    /// The request's own run reads it — at the next file, at the byte loop's
    /// next accounting interval ([`TheStopHoldsTheDraw`]), and while it waits
    /// for the connection — and performs `Server protocol and transfers` S26's
    /// deliberate stop there, the record written *stopped* before the channel
    /// is closed (`Server protocol and transfers` C23). **Answers whether the
    /// request was under way in this run**: one that is not has nothing to
    /// stop.
    ///
    /// `file` is the landing path as the row names it.
    pub fn the_user_stops(&self, request: &str, file: Option<&[u8]>) -> bool {
        let mut moving = self.moving();
        let Some(under_way) = moving.requests.get_mut(request) else {
            return false;
        };
        match file {
            None => under_way.the_user_stopped_it = true,
            Some(file) => under_way.files_the_user_stopped.push(file.to_vec()),
        }
        true
    }

    /// **Whether the user stopped this request as a whole.**
    pub(crate) fn the_user_stopped_it(&self, request: &str) -> bool {
        self.moving()
            .requests
            .get(request)
            .is_some_and(|under_way| under_way.the_user_stopped_it)
    }

    /// **Whether the user stopped this request, or this one file of it** —
    /// the file by its landing path, compared as the row shows it.
    pub(crate) fn the_user_stopped_this(&self, request: &str, landing: &[u8]) -> bool {
        self.moving().requests.get(request).is_some_and(|under_way| {
            under_way.the_user_stopped_it
                || under_way
                    .files_the_user_stopped
                    .iter()
                    .any(|file| String::from_utf8_lossy(file) == String::from_utf8_lossy(landing))
        })
    }

    /// **Whether this request was under way at any moment of this run** — so
    /// a leftover can say *"stopped when Smoozy closed"* only where Smoozy
    /// closing is what ended it (`Design/Transfers § What a past sitting left
    /// behind`). A request that ran in this run and ended with its server's
    /// period (`File Browser` U7's paragraph of 2026-09-28) was not stopped by
    /// Smoozy closing, and its row does not say it was. Step 10c's transfer
    /// wave piece 7.
    pub fn it_ran_in_this_run(&self, request: &str) -> bool {
        self.moving().ran.contains(request)
    }

    /// **Whether this request ended in this run because its server's period
    /// ended — its line cut and not repaired, or the period ended under it —
    /// and not by the user's *Stop*** (`Design/Transfers`, the finished line's
    /// ending word: the Creator, 2026-10-10, *red also where the transfer
    /// ended because its line was cut or its server's period ended*). Read
    /// while the request is still held, and remembered after its run returns.
    /// Run-held and never durable, as every request's activity is (`File
    /// Browser` U7). The Creator's seventh notes, piece Z5b.
    pub fn its_period_ended_it(&self, request: &str) -> bool {
        let moving = self.moving();
        match moving.requests.get(request) {
            Some(under_way) => under_way.its_period_ended && !under_way.the_user_stopped_it,
            None => moving.ended_by_their_period.contains(request),
        }
    }

    /// **What an ended request left staged, where this run has read it.**
    pub(crate) fn what_it_left(&self, request: &str) -> Option<WhatARequestLeft> {
        self.left
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .get(request)
            .cloned()
    }

    /// **What an ended request left, as the record read it**, kept for the run.
    pub(crate) fn it_left(&self, request: &str, left: WhatARequestLeft) {
        self.left
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .insert(request.to_string(), left);
    }

    /// **Something moved what a leftover is** — a *Remove*, a take-up, a
    /// *Skip existing* that abandoned a part, a sweep that cleared a debt — so
    /// every ended request is read again the next time the screen asks.
    pub fn every_leftover_is_read_again(&self) {
        self.left
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .clear();
    }

    /// **Every request under way, as it stands at this moment** — what the
    /// Transfers screen reads. Taken while the caller holds the record, so a
    /// file whose outcome goes down in the same hold is counted once.
    pub fn what_is_under_way(&self) -> HashMap<String, ARequestUnderWay> {
        self.moving().requests.clone()
    }

    /// **The two caps Settings holds, put into the budgets** — R2's unit and
    /// R3's split between the durable setting and the object.
    ///
    /// **`Server protocol and transfers` R8 is why this is a push and not a
    /// read at admission**: a cap set or changed while transfers are in flight
    /// takes effect on each running channel at that channel's next accounting
    /// interval, and a value only read when a channel is admitted would reach
    /// nothing already moving.
    pub fn the_caps_saved_are(&self, upload: TheCap, download: TheCap) {
        let mut point = self.held();
        point.set_the_cap(Direction::Up, upload);
        point.set_the_cap(Direction::Down, download);
    }

    /// **One direction's cap, at the moment Settings saved it.**
    pub fn a_cap_was_saved(&self, direction: Direction, cap: TheCap) {
        self.held().set_the_cap(direction, cap);
    }

    /// **Every cap the record holds, read once.** A record that would not open
    /// or would not answer moves neither budget: R8's *no cap set* is the
    /// ordinary state and is what the point already holds, and taking a failed
    /// read as *no cap* would lift a limit the user set — which is the fault
    /// `src/record/settings.rs` is already careful about on its own side.
    pub fn the_caps_are_read_from(&self, record: Option<&crate::record::LocalRecord>) {
        let Some(record) = record else {
            return;
        };
        let Ok(held) = settings::read(record.connection()) else {
            return;
        };
        self.the_caps_saved_are(held.the_upload_cap, held.the_download_cap);
    }

    pub fn the_cap(&self, direction: Direction) -> TheCap {
        self.held().the_cap(direction)
    }

    /// **R7 and R8's admissible count**, for a caller that wants to say how
    /// many channels it may open rather than open them one at a time.
    pub fn the_admissible_count(&self, direction: Direction) -> TheAdmissibleCount {
        self.held().the_admissible_count(direction)
    }

    /// **R15's achieved-rate reading, per direction** — `Server protocol and
    /// transfers` R12a's window, taken at the same accounting point the
    /// enforcement decisions are taken at.
    ///
    /// **Nothing where that direction has no channel**, because that clause's
    /// first publication moment is an accounting interval and one exists only
    /// while a channel is running.
    pub fn the_reading(&self, direction: Direction) -> Option<TheAchievedRate> {
        self.held()
            .the_reading_at_this_accounting_interval(direction, std::time::Instant::now())
    }

    /// **How many grants this point has made in a direction.** Read by R17's
    /// own test, and by nothing that decides anything.
    pub fn grants_made(&self, direction: Direction) -> usize {
        self.held().grants_made(direction)
    }

    /// **One admission decision — R7, R9, R13 and R17, over `File Browser`
    /// O5's share rule applied at this point by `File Browser` O8.**
    ///
    /// `waiting` is the classes with work waiting in that direction at this
    /// moment, which is the caller's to say. **R13: nothing is a reason to
    /// open fewer and never a reason to fail the transfer**, so an absence
    /// here is a refusal and never an error.
    pub fn admit(&self, direction: Direction, waiting: &[&'static str]) -> Option<TheChannelsDraw> {
        self.held()
            .admit(direction, waiting, std::time::Instant::now())
    }

    /// **The same decision for a caller that can spend a grant made to one
    /// class only**, and a grant it cannot spend leaves no mark on the turn or
    /// on the share rule's memory (must-do 1) — see
    /// [`crate::scheduling::TheDesktopSideAdmissionPoint::admit_to_carry`].
    pub fn admit_to_carry(
        &self,
        direction: Direction,
        waiting: &[&'static str],
        carries: &'static str,
    ) -> Result<TheChannelsDraw, Option<&'static str>> {
        self.held()
            .admit_to_carry(direction, waiting, carries, std::time::Instant::now())
    }
}

// ---------------------------------------------------------------------------
// What is moving now — the run-held half of a Transfers row.
// ---------------------------------------------------------------------------

/// **At most how often the page is told a transfer's bytes moved.** Added
/// 2026-09-29, step 10c's transfer wave piece 5.
///
/// `Server protocol and transfers` G4 has the byte loop report at least once
/// per accounting period — a tenth of a second — and every report asking the
/// page to compose the whole screen again would be ten asks a second for each
/// file in flight. **So the page is told no more than four times a second,
/// over every transfer of the run together**, which is more than the once a
/// second a person needs to see a row move. **It is not a timer** (`Desktop
/// app and delivery` P10b): nothing fires with nothing under way; a report
/// arriving inside the running byte loop either tells the page or does not.
/// A unit's ending and a request's ending are told whatever this says, so the
/// last figure a row shows is never one this held back.
pub const THE_PAGE_IS_TOLD_AT_MOST_EVERY: Duration = Duration::from_millis(250);

/// **How many of a request's files its row draws at once** — the files in
/// flight, then the next ones waiting their turn. `Design/Transfers § A request
/// shows the whole and the files under it`: *the files — a few at a time*.
/// **Four is `File Browser` O1's upward pool**, the most that could ever be in
/// flight for one server, so a row never hides a file that is moving.
pub const THE_FILES_A_ROW_DRAWS: usize = 4;

/// **One file in flight, and the last report its byte loop made** —
/// `Server protocol and transfers` G4's count, with `Server protocol and
/// transfers` G4a's mark and `Server protocol and transfers` G4b's.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AFileMoving {
    /// Which entry of its request it is.
    pub entry: usize,
    /// **Where it lands**, the path its row names.
    pub path: Vec<u8>,
    pub report: Report,
    /// **The line cut it and it waits for the connection to come back** —
    /// its record reads *interrupted*, its request is open and its server's
    /// period stands, which is `File Browser` U5's and `Download` M11's
    /// *waiting*. Its report is the last one its byte loop made, so its row
    /// keeps its figure where it stood. Step 10c's transfer wave piece 6.
    pub waits_for_the_connection: bool,
}

/// **What one request has moving in this run, and what waits behind it.**
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ARequestUnderWay {
    pub moving: Vec<AFileMoving>,
    /// The landing paths of the next files waiting their turn.
    pub waiting: Vec<Vec<u8>>,
    /// **The server it moves to or from**, its local identifier. Step 10c's
    /// transfer wave piece 6.
    pub server: String,
    /// **The request waits for its server's connection to come back** —
    /// whether or not a file of it is in flight. Step 10c's transfer wave
    /// piece 6.
    pub waits_for_the_connection: bool,
    /// **Its server's period has ended** (`File Browser` U7's paragraph of
    /// 2026-09-28), so the request has ended there, though its run may take a
    /// moment more to return. Step 10c's transfer wave piece 6.
    pub its_period_ended: bool,
    /// **The user pressed the request's *Stop*** (`File Browser` U6,
    /// `Download` M12, `Server protocol and transfers` C22). Step 10c's
    /// transfer wave piece 7.
    pub the_user_stopped_it: bool,
    /// **The files the user stopped one at a time**, by their landing paths —
    /// `Design/Transfers § Stopping one file`. Step 10c's transfer wave piece
    /// 7.
    pub files_the_user_stopped: Vec<Vec<u8>>,
}

/// **What an ended upload or manual download left staged, as the record
/// reads it** — `File Browser` U8's first bullet and `Download` M13's second.
/// Added 2026-09-29, step 10c's transfer wave piece 7.
///
/// **Counted, and read once per request per run**: reading an ended request
/// means reading its server's requests whole, which is the cost step 10c's
/// transfer wave piece 1 took off every Transfers ask. So it is read when a
/// request is first seen ended with its run returned, kept here, and read
/// again only when something this piece does moves it
/// ([`TheRunsTransfers::every_leftover_is_read_again`]).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct WhatARequestLeft {
    /// **Transfer records reading *interrupted* under the server's current
    /// installation** — the leftover the user is told of and may remove.
    pub interrupted: usize,
    /// **Records whose staged material was to be removed and was not** —
    /// `Server protocol and transfers` S27's owed cleanup — under the current
    /// installation. The part-sent files are still there, so the row still
    /// says so, and there is nothing more a *Remove* can do about them.
    pub owed: usize,
    /// The installation the reading was taken under, so a machine that has
    /// since been set up again is read again.
    pub under: Option<String>,
}

/// **The run's register of what its uploads and manual downloads are moving
/// now.** Added 2026-09-29, step 10c's transfer wave piece 5.
///
/// **Run-held and never durable** (`File Browser` U7: a request's activity
/// lives in process memory and ends with the run; `Download` M11a's fourth
/// bullet: nothing durable is written from a transfer's report). **A request is
/// in it for exactly as long as something in this run is moving it** — from the
/// moment its run begins to the moment it returns, whatever ended it — so a
/// request the record holds with a unit still owed and this register does not
/// hold is one nothing is moving and nothing will: it ended with an earlier
/// run, or with its server's period (`File Browser` U7's paragraph of
/// 2026-09-28, `Download` M13's), or its own run returned.
#[derive(Debug, Default)]
pub struct WhatIsMovingNow {
    requests: HashMap<String, ARequestUnderWay>,
    the_page_was_told_at: Option<Instant>,
    /// **Every request that was under way at any moment of this run.** Step
    /// 10c's transfer wave piece 7.
    ran: HashSet<String>,
    /// **Every request whose run returned after its server's period ended
    /// under it, and that the user had not stopped** — what
    /// [`TheRunsTransfers::its_period_ended_it`] remembers. The Creator's
    /// seventh notes, piece Z5b.
    ended_by_their_period: HashSet<String>,
}

impl WhatIsMovingNow {
    fn begins(&mut self, request: &str, server: &str) {
        self.requests.entry(request.to_string()).or_default().server = server.to_string();
        self.ran.insert(request.to_string());
        self.ended_by_their_period.remove(request);
    }

    /// **Whether a stop reaches this request, or its file in flight at
    /// `entry`.**
    fn a_stop_reaches(&self, request: &str, entry: Option<usize>) -> bool {
        let Some(under_way) = self.requests.get(request) else {
            return false;
        };
        if under_way.the_user_stopped_it {
            return true;
        }
        let Some(entry) = entry else {
            return false;
        };
        under_way.moving.iter().any(|file| {
            file.entry == entry
                && under_way
                    .files_the_user_stopped
                    .iter()
                    .any(|stopped| String::from_utf8_lossy(stopped) == String::from_utf8_lossy(&file.path))
        })
    }

    fn its_period_ended(&mut self, request: &str) {
        if let Some(under_way) = self.requests.get_mut(request) {
            under_way.its_period_ended = true;
        }
    }

    fn a_servers_period_ended(&mut self, server: &str) {
        for under_way in self.requests.values_mut() {
            if under_way.server == server {
                under_way.its_period_ended = true;
            }
        }
    }

    fn it_waits_for_the_connection(&mut self, request: &str, entry: Option<usize>, waits: bool) {
        let Some(under_way) = self.requests.get_mut(request) else {
            return;
        };
        under_way.waits_for_the_connection = waits;
        if let Some(entry) = entry {
            if let Some(file) = under_way.moving.iter_mut().find(|file| file.entry == entry) {
                file.waits_for_the_connection = waits;
            }
        }
    }

    fn ends(&mut self, request: &str) {
        if let Some(under_way) = self.requests.remove(request) {
            if under_way.its_period_ended && !under_way.the_user_stopped_it {
                self.ended_by_their_period.insert(request.to_string());
            }
        }
    }

    /// **The files waiting their turn are the next ones not already moving.**
    /// Step 10c's eleventh-pass fix piece 4: a request now has several files in
    /// flight, which start in the enumeration's order but do not always say so
    /// in it, so a file said to be waiting may already be moving beside this
    /// one. **A file is never drawn both moving and waiting.**
    fn a_file_goes(&mut self, request: &str, entry: usize, path: Vec<u8>, expected: u64, waiting: Vec<Vec<u8>>) {
        let Some(under_way) = self.requests.get_mut(request) else {
            return;
        };
        under_way.moving.retain(|file| file.entry != entry);
        under_way.moving.push(AFileMoving {
            entry,
            path,
            report: Report {
                bytes_staged: 0,
                bytes_expected: expected,
                started_over: None,
                verifying_a_prefix_of: None,
            },
            waits_for_the_connection: false,
        });
        let moving = &under_way.moving;
        under_way.waiting = waiting
            .into_iter()
            .filter(|path| !moving.iter().any(|file| file.path == *path))
            .take(THE_FILES_A_ROW_DRAWS - 1)
            .collect();
    }

    /// **A report arrived — and whether the page is to be told.**
    fn a_file_reported(&mut self, request: &str, entry: usize, report: &Report, now: Instant) -> bool {
        let Some(file) = self
            .requests
            .get_mut(request)
            .and_then(|under_way| under_way.moving.iter_mut().find(|file| file.entry == entry))
        else {
            return false;
        };
        file.report = report.clone();
        let tell = self
            .the_page_was_told_at
            .is_none_or(|at| now.saturating_duration_since(at) >= THE_PAGE_IS_TOLD_AT_MOST_EVERY);
        if tell {
            self.the_page_was_told_at = Some(now);
        }
        tell
    }

    fn a_file_stopped(&mut self, request: &str, entry: usize) {
        if let Some(under_way) = self.requests.get_mut(request) {
            under_way.moving.retain(|file| file.entry != entry);
        }
    }
}

/// **A manual download being counted** — the moment between the press and
/// `Download` M1's write, held for the run. Step 10c's twelfth-pass fix piece
/// 3; `Design/Transfers`' paragraph of 2026-10-01.
#[derive(Debug, Clone, Default)]
pub struct ACountUnderWay {
    /// The server it comes from, its local identifier.
    pub server: String,
    /// **What the row names**: the folder being counted.
    pub named: String,
    /// Where it goes.
    pub destination: Vec<u8>,
    /// **The user pressed the row's *Stop*.**
    pub the_user_stopped_it: std::sync::Arc<AtomicBool>,
}

/// **A count is on Transfers for as long as this is held**, and goes when it
/// is dropped — the page told as it goes.
pub struct TheCountIsUnderWay<'a> {
    the_run: &'a TheRunsTransfers,
    request: String,
    stopped: std::sync::Arc<AtomicBool>,
    record: &'a dyn TheRecordADownloadWrites,
}

impl TheCountIsUnderWay<'_> {
    fn the_user_stopped_it(&self) -> bool {
        self.stopped.load(Ordering::SeqCst)
    }
}

impl Drop for TheCountIsUnderWay<'_> {
    fn drop(&mut self) {
        self.the_run.counting().retain(|(one, _)| *one != self.request);
        self.record.a_transfer_moved();
    }
}

/// **A request is under way for as long as this is held**, and ends when it
/// is dropped — whatever ended the run that holds it, an unwinding among them.
pub struct TheRequestIsUnderWay<'a> {
    the_run: &'a TheRunsTransfers,
    request: String,
    /// **The server it moves to or from**, so its end reaches `Server Setup`
    /// R1b's count of the transfers with that server. Added 2026-10-12.
    server: String,
}

impl Drop for TheRequestIsUnderWay<'_> {
    fn drop(&mut self) {
        self.the_run.moving().ends(&self.request);
        // **After the register's own lock is let go**: where this was the last
        // transfer with the server, an owed swap is tried (`Server Setup`
        // R1b, `State, identity and recovery` F15's first moment).
        crate::servers::the_driver_swap::a_transfer_ends(&self.server);
    }
}

/// **The watcher an upload or a manual download hands its byte loop** —
/// `Server protocol and transfers` G4: the loop's report goes into the run's
/// register, and the page is told, no more often than
/// [`THE_PAGE_IS_TOLD_AT_MOST_EVERY`]. Nothing durable is written from it.
pub(crate) struct TheRowIsTold<'a> {
    pub the_run: &'a TheRunsTransfers,
    pub request: String,
    pub entry: usize,
    pub tell: &'a (dyn Fn() + Sync),
    /// **A start-over made before the loop was handed the record** — `File
    /// Browser` K10 clause 2's, where an upload takes up a part whose first
    /// bytes are no longer the file's own. It is `Server protocol and
    /// transfers` G4a's mark exactly as the loop sets it for its own
    /// start-over, carried on every report of that carry, so the row says
    /// *Could not resume — uploading from the beginning* rather than
    /// nothing. `None`
    /// everywhere else. The two transfer faults after the 38's pieces 7 and
    /// 9, fault 1.
    pub started_over: Option<smoozy_server_protocol::transfers::StartOverCause>,
}

impl WatchesTheTransfer for TheRowIsTold<'_> {
    fn report(&mut self, report: &Report) {
        let mut report = report.clone();
        if report.started_over.is_none() {
            report.started_over = self.started_over;
        }
        let tell = self
            .the_run
            .moving()
            .a_file_reported(&self.request, self.entry, &report, std::time::Instant::now());
        if tell {
            (self.tell)();
        }
    }
}

// ---------------------------------------------------------------------------
// Carrying on across a repair — step 10c's transfer wave piece 6.
// ---------------------------------------------------------------------------

/// **How often a request waiting for the connection looks again** at its
/// server's point and at whether its period has ended. **The figure is this
/// build's and no promise rests on it**: it is short beside a repair, whose
/// every attempt is an SSH connection made afresh, and long beside a look,
/// which reads two values held in memory.
pub(crate) const THE_LOOK_WHILE_THE_LINE_IS_DOWN: Duration = Duration::from_millis(250);

/// **How long after an *interrupted* ending the point is watched for the
/// break that caused it.** A line that goes takes every channel on it and the
/// connection with them, and the point stops lending within moments of the
/// channel failing. **A channel that failed on a connection that stayed up was
/// not cut by the line**: `Server protocol and transfers` C11 step 4 carries a
/// transfer on only across a **replaced** connection, and none is coming. So a
/// point still lending when this has passed ends the file where it stood, as
/// before this piece, and nothing is tried again on a line that is up — which
/// is what keeps an ending the line did not cause from being retried for ever.
/// **The figure is this build's and no promise rests on it.**
///
/// **The same look follows a seat that was not had** — added 2026-10-02, out
/// of the hosted run that was red on Linux at `d7a25677`: a seat asked for as
/// the line goes is answered while the point still reads as lending, so a
/// refusal on a line that stays up now ends its file up to this long later
/// rather than at once.
pub(crate) const THE_BREAK_IS_LOOKED_FOR_FOR: Duration = Duration::from_secs(2);

/// **Whether the line went down under a transfer that ended *interrupted***,
/// or under a seat that was not had — the point stopped lending within
/// [`THE_BREAK_IS_LOOKED_FOR_FOR`].
pub(crate) async fn the_line_went_down<C: AHeldConnection>(sessions: &ThisServersSessions<C>) -> bool {
    let began = Instant::now();
    loop {
        if !sessions.a_connection_is_live() {
            return true;
        }
        if began.elapsed() >= THE_BREAK_IS_LOOKED_FOR_FOR {
            return false;
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}

/// **The request waits for its server's connection to come back** — `File
/// Browser` K1 and K2, `Download` M11, `Server protocol and transfers` C11 and
/// `Server protocol and transfers` S12 — and answers **`true` when the point lends again, `false` when the
/// period ended instead** (`File Browser` U7's paragraph of 2026-09-28,
/// `Download` M13's) **or the user stopped the request, or the file in flight
/// at `entry`** (`File Browser` U6, `Download` M12; step 10c's transfer wave
/// piece 7). A third end, and it is as certain as the user's own press.
///
/// **The wait has two ends and both are certain to come.** The point lends
/// again when `Servers`' repair replaces the connection under the same period
/// — the point is the period's and survives a replacement
/// (`ThisServersSessions`) — or the period ends: the user's *Disconnect*, an
/// edit's save, the row removed, or the application run ending (`Servers`
/// S10), each of which is told to the run's register
/// ([`TheRunsTransfers::a_servers_period_ended`]). **A point that lends again
/// is always this period's**: a new period makes a new point, so this one
/// never lends for anybody else.
///
/// **While it waits, the request and the file in flight read *waiting for the
/// connection*** in the register (`File Browser` U5, `Download` M11), and the
/// page is told as the wait begins and as it ends. **Nothing here says when the
/// next attempt falls**: that is `Servers`' own reading, and the Transfers
/// screen reads it there (`Design/Transfers § Waiting for the connection to
/// come back`).
pub(crate) async fn while_the_line_is_down<C: AHeldConnection>(
    sessions: &ThisServersSessions<C>,
    the_run: &TheRunsTransfers,
    request: &str,
    entry: Option<usize>,
    tell: &(dyn Fn() + Sync),
) -> bool {
    let it_is_over = || {
        the_run.its_period_has_ended(request) || the_run.moving().a_stop_reaches(request, entry)
    };
    if it_is_over() {
        return false;
    }
    the_run.it_waits_for_the_connection(request, entry, true);
    tell();
    let back = loop {
        // **Or the user stopped it** — `File Browser` U6: *a waiting upload is
        // stoppable exactly as a moving one*, and `Download` M12 the same. The
        // caller then writes the stop (step 10c's transfer wave piece 7).
        if it_is_over() {
            break false;
        }
        if sessions.a_connection_is_live() {
            break true;
        }
        tokio::time::sleep(THE_LOOK_WHILE_THE_LINE_IS_DOWN).await;
    };
    the_run.it_waits_for_the_connection(request, entry, false);
    tell();
    back
}

/// **What went down under a transfer that ended *interrupted*.** Added
/// 2026-10-19, the dead-line sweep's extra-connection repair.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum WhatWentDown {
    /// **The line, and the primary connection with it** — the point stopped
    /// lending ([`the_line_went_down`]'s case).
    TheLine,
    /// **`Servers` S14's additional connection the channel sat on, and that
    /// alone** — the primary still answering. *A transfer interruption and
    /// never a connection break* (`Servers` S14 condition 6), and the next
    /// seat is on a fresh additional connection.
    ItsOwnConnection,
    /// **Neither, within [`THE_BREAK_IS_LOOKED_FOR_FOR`]** — the channel
    /// failed on a connection that stayed up, and nothing is tried again.
    Nothing,
}

/// **What went down under a transfer whose channel sat on `its_connection`**
/// — looked for, never glanced at, as [`the_line_went_down`] looks.
///
/// **Why the look is two looks.** Until the dead-line sweep's extra-connection repair
/// (2026-10-19) this asked only whether the point stopped lending — whether the
/// **primary** went down. **An additional connection that died on its own left
/// the point lending**, so a transfer it cut was read as cut on a line that
/// stayed up and ended where `Servers` S14 condition 6 has the next transfer
/// open a fresh connection and carry on. **The line going takes the additional
/// connection a moment before the point stops lending**, so where the
/// channel's own connection is seen ended first, the point is looked at once
/// more a moment later before the loss is called the additional's alone.
pub(crate) async fn what_went_down<C: AHeldConnection>(
    sessions: &ThisServersSessions<C>,
    its_connection: &TheConnectionItSatOn,
) -> WhatWentDown {
    let began = Instant::now();
    loop {
        if !sessions.a_connection_is_live() {
            return WhatWentDown::TheLine;
        }
        if its_connection.has_ended() {
            tokio::time::sleep(Duration::from_millis(50)).await;
            return if sessions.a_connection_is_live() {
                WhatWentDown::ItsOwnConnection
            } else {
                WhatWentDown::TheLine
            };
        }
        if began.elapsed() >= THE_BREAK_IS_LOOKED_FOR_FOR {
            return WhatWentDown::Nothing;
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}

/// **How long a transfer whose additional connection died waits before asking
/// for a seat again**, where the fresh additional connection could not be made
/// — the route to the machine still down for a new connection, the primary
/// still up. **The figure is this build's and no promise rests on it**: each
/// ask is one connection made afresh, `Servers` Q16's three steps, so it is
/// long beside a refusal and short beside the user's own wait. Added
/// 2026-10-19, the dead-line sweep's extra-connection repair.
pub(crate) const THE_PAUSE_BEFORE_A_FRESH_ADDITIONAL_CONNECTION: Duration = Duration::from_secs(5);

/// **A transfer whose additional connection died alone reads *waiting for the
/// connection* for as long as this stands** — from the moment it is cut until
/// it is seated again or ends — in the register (`File Browser` U5,
/// `Download` M11), which is what it is doing, on the one connection that
/// carries it. Added 2026-10-19, the dead-line sweep's extra-connection repair.
pub(crate) struct ItWaitsForItsOwnConnection<'a> {
    the_run: &'a TheRunsTransfers,
    request: &'a str,
    entry: Option<usize>,
    tell: &'a (dyn Fn() + Sync),
}

impl<'a> ItWaitsForItsOwnConnection<'a> {
    pub(crate) fn begins(
        the_run: &'a TheRunsTransfers,
        request: &'a str,
        entry: Option<usize>,
        tell: &'a (dyn Fn() + Sync),
    ) -> Self {
        the_run.it_waits_for_the_connection(request, entry, true);
        tell();
        ItWaitsForItsOwnConnection {
            the_run,
            request,
            entry,
            tell,
        }
    }

    /// **The pause before a seat is asked for again**, where the fresh
    /// additional connection could not be made — the route to the machine
    /// still down for a new connection, the primary still up. **`true` to ask
    /// again, `false` where the period ended or the user stopped the request
    /// or the file**: the same two certain ends [`while_the_line_is_down`]
    /// has. **There is no final deadline while the period stands**, as there
    /// is none on the primary's own repair: Stop and Disconnect are the ways
    /// out, as there.
    pub(crate) async fn pauses(&self) -> bool {
        let it_is_over = || {
            self.the_run.its_period_has_ended(self.request)
                || self.the_run.moving().a_stop_reaches(self.request, self.entry)
        };
        let began = Instant::now();
        loop {
            if it_is_over() {
                return false;
            }
            if began.elapsed() >= THE_PAUSE_BEFORE_A_FRESH_ADDITIONAL_CONNECTION {
                return true;
            }
            tokio::time::sleep(THE_LOOK_WHILE_THE_LINE_IS_DOWN).await;
        }
    }
}

impl Drop for ItWaitsForItsOwnConnection<'_> {
    fn drop(&mut self) {
        self.the_run
            .it_waits_for_the_connection(self.request, self.entry, false);
        (self.tell)();
    }
}

/// **`Server protocol and transfers` C11 step 4's gate, on the one record a
/// carry-on hands back** — `File Browser` K1's filter, which is that step
/// taken whole: the record says *interrupted* and was written by this
/// application run. **It is the same filter in both directions**, because the
/// step is one step: `Download` M11 reads the same two labels and the same
/// ending.
///
/// **Where it passes, the record's ending is taken off before the loop is
/// handed it again**: the transfer is moving once more and the record says so,
/// which is the `Server protocol and transfers` S9 record's own shape for a
/// transfer under way. **A carried-on record then stands exactly as a first
/// attempt's does while it moves**, so nothing that reads records later —
/// the next start's settling among them — can tell the two apart or has to.
pub(crate) fn the_record_is_carried_on(record: &mut TransferRecord, this_run: &Identity) -> bool {
    if !smoozy_file_browser::continuation::c11_step_four_resumes(record, this_run) {
        return false;
    }
    record.ending = None;
    true
}

/// **The next files of a request waiting their turn**, after the one at
/// `at`, as many as its row draws beside the file in flight — **and as many
/// again**, since up to that many after it may already be moving beside it
/// (step 10c's eleventh-pass fix piece 4). The run's register keeps the ones
/// not moving ([`TheRunsTransfers::a_file_goes`]).
pub(crate) fn the_next_waiting<T>(
    entries: &[T],
    at: usize,
    goes: impl Fn(&T) -> Option<Vec<u8>>,
) -> Vec<Vec<u8>> {
    entries
        .iter()
        .skip(at + 1)
        .filter_map(goes)
        .take(2 * (THE_FILES_A_ROW_DRAWS - 1))
        .collect()
}

/// **Several of one request's files at once, each channel taking the next
/// file when its own ends.** Added 2026-09-30, step 10c's eleventh-pass fix
/// piece 4.
///
/// **What it is for.** `File Browser` O1's pool is four channels a server and
/// its upward ceiling four (`File Browser` O2); `Download` Q1's downward
/// ceiling is two. **Until this piece an upload request, a manual download
/// request and the dependency check's send each filled one of them**, one
/// file after another, so a thousand small files paid their round trips one
/// file at a time: 46 minutes up at a 0.2 s round trip where four at once took
/// 11.6 (step 10c's transfer wave piece 13, at the desk).
///
/// **How.** Each worker is one channel's worth of the request: it takes the
/// next file in the enumeration's order (`File Browser` O7, `Download` P2),
/// carries it to its ending, and takes the next — so a small file never waits
/// for a large one beside it, which a batch of four would make it do. **They
/// run inside the caller's own task**, polled together here, so nothing is
/// spawned, nothing outlives the request's run, and every worker may borrow
/// what the run holds. **Every file is still admitted in its own turn at both
/// points** — `Desktop app and delivery` R7 and R17 at the desktop point,
/// `Server protocol and transfers` T5 at the server's — so a worker that
/// cannot be admitted waits there, and **the ceiling is the most that may
/// move, never a promise that many will.**
pub(crate) async fn every_one_of<F: Future<Output = ()>>(workers: Vec<F>) {
    let mut workers: Vec<Option<std::pin::Pin<Box<F>>>> = workers
        .into_iter()
        .map(|worker| Some(Box::pin(worker)))
        .collect();
    std::future::poll_fn(|context| {
        let mut every_one_ended = true;
        for slot in workers.iter_mut() {
            if let Some(worker) = slot {
                if worker.as_mut().poll(context).is_ready() {
                    *slot = None;
                } else {
                    every_one_ended = false;
                }
            }
        }
        if every_one_ended {
            std::task::Poll::Ready(())
        } else {
            std::task::Poll::Pending
        }
    })
    .await
}

/// **The next file of a request for a worker of [`every_one_of`]**, in the
/// enumeration's order, and nothing once every file has been taken. Added
/// 2026-09-30, step 10c's eleventh-pass fix piece 4.
#[derive(Debug, Default)]
pub(crate) struct TheNextFile(std::sync::atomic::AtomicUsize);

impl TheNextFile {
    pub(crate) fn of(&self, how_many: usize) -> Option<usize> {
        let at = self.0.fetch_add(1, Ordering::SeqCst);
        (at < how_many).then_some(at)
    }
}

// ---------------------------------------------------------------------------
// The user's Stop, and what a leftover leaves — step 10c's transfer wave piece 7.
// ---------------------------------------------------------------------------

/// **The byte loop's draw, held at nothing once the user has stopped** —
/// `Server protocol and transfers` C22: *"A stop closes the channel and stops
/// the loop"*, and nothing on the machine is asked to stop anything. Added
/// 2026-09-29, step 10c's transfer wave piece 7.
///
/// **Why the stop lands here and not anywhere in the middle of the loop.** The
/// loop is `Server protocol and transfers` §7's and is not this file's; the one
/// place it asks this side anything at every accounting interval is its draw
/// (`Server protocol and transfers` R2), and a draw of nothing makes it wait —
/// a plain pause, `Server protocol and transfers` R7, with nothing half-sent on
/// the channel and no read half-answered. **So a stopped draw answers nothing
/// and says it has**, and [`the_stop_lands_or`] ends the loop while it waits
/// there. `Server protocol and transfers` C19's deadline does not run on a
/// channel that drew nothing, so the pause is never read as a stall.
///
/// **A stop that comes while the loop is not drawing — verifying a staged
/// prefix, publishing — lands at its next draw, or not at all where the file
/// then publishes**, which is `File Browser` U6's *finished files stay*: a file
/// that reached its name in the moment of the press is a finished file.
pub(crate) struct TheStopHoldsTheDraw<'a, B: DrawsFromTheSharedBudget> {
    pub draw: &'a mut B,
    pub the_run: &'a TheRunsTransfers,
    pub request: &'a str,
    pub landing: &'a [u8],
    /// **Set the first time a draw is refused for the stop** — the loop is
    /// then waiting, and may be ended.
    pub parked: &'a AtomicBool,
}

impl<B: DrawsFromTheSharedBudget> DrawsFromTheSharedBudget for TheStopHoldsTheDraw<'_, B> {
    fn draw(&mut self, wanted: usize) -> usize {
        if self.the_run.the_user_stopped_this(self.request, self.landing) {
            self.parked.store(true, Ordering::SeqCst);
            return 0;
        }
        self.draw.draw(wanted)
    }

    fn give_back(&mut self, unused: usize) {
        self.draw.give_back(unused);
    }
}

/// **How often a stopped transfer is looked at to see whether its loop has
/// come to its pause.** The figure is this build's and no promise rests on it:
/// the loop reaches its draw at least once per `Server protocol and transfers`
/// R6 accounting period, a tenth of a second.
const THE_LOOK_FOR_A_STOP: Duration = Duration::from_millis(50);

/// **The loop, run to its ending — or `None` where the user stopped it and it
/// has come to its pause** ([`TheStopHoldsTheDraw`]). The loop is dropped
/// there, waiting, and the caller writes the stop.
pub(crate) async fn the_stop_lands_or<F: Future>(carrying: F, parked: &AtomicBool) -> Option<F::Output> {
    tokio::pin!(carrying);
    loop {
        tokio::select! {
            biased;
            done = &mut carrying => return Some(done),
            () = tokio::time::sleep(THE_LOOK_FOR_A_STOP) => {
                if parked.load(Ordering::SeqCst) {
                    return None;
                }
            }
        }
    }
}

/// **The ending the user's Stop writes** — `Server protocol and transfers`
/// S26's second row, *stopped, deliberately*, whose reason names who stopped
/// it.
///
/// **The reason is *staging removed by user*, and that is a choice between the
/// three this build can write**, recorded in the decisions record. `Server
/// protocol and transfers` S26 lists the reasons with *"including"*, so the
/// plan's set is open, and the protocol crate's closed set has no word for the
/// user's plain Stop. **Of the three, it is the one whose both halves are
/// true**: the user did it, and it removes the staged material. *Abandoned by
/// its request* is Smoozy's own act (`Server protocol and transfers` S28a) and
/// *the source changed* is `File Browser` K8's. Every reader of the reason
/// treats the three alike but for that one's `File Browser` X12 recovery
/// branch, which the stop never reaches: it lands only while the loop is
/// drawing, before any publication begins.
pub(crate) const THE_USERS_STOP: Ending = Ending::Stopped(StoppedReason::StagingRemovedByUser);

/// **Whether a transfer's ending is the user's own Stop in this run** — the
/// ending [`THE_USERS_STOP`] and the stop marked in the register — so the unit
/// is left owed rather than failed: *finished files stay, unarranged files are
/// never arranged, the one in flight is stopped* (`File Browser` U6), and
/// nothing about it failed.
pub(crate) fn the_user_stopped_this_ending(
    the_run: &TheRunsTransfers,
    request: &str,
    landing: &[u8],
    ending: &Ending,
) -> bool {
    *ending == THE_USERS_STOP && the_run.the_user_stopped_this(request, landing)
}

/// **The staging a record names, removed here on the user's own computer on
/// `Server protocol and transfers` S28's exact match** — the record's ending in
/// a removable state for its live attempt, every superseded attempt always,
/// and each object removed only where its stable identity now matches the one
/// recorded for that attempt (`Server protocol and transfers` S29: nothing is
/// found by looking in a folder). **Answers whether every derived path was
/// confirmed removed or already gone** — `Server protocol and transfers` S27's
/// test for clearing a debt. Added 2026-09-29, step 10c's transfer wave piece
/// 7, in the shape of the start's own sweep (`src/start.rs`), for one record.
///
/// **A download's staging only** (`Server protocol and transfers` S1): an
/// upload's is on the machine, where no identity can be read (`Server protocol
/// and transfers` §12 item 15).
pub(crate) fn the_staging_it_names_is_removed_here(record: &TransferRecord) -> bool {
    use smoozy_server_protocol::transfers::cleanup::{
        what_the_sweep_derives, what_the_sweep_does, WhatTheSweepDoes,
    };
    if record.direction != Direction::Down {
        return false;
    }
    let mut adapter = ThePlatformFileAdapter;
    let mut confirmed = true;
    for derived in what_the_sweep_derives(std::slice::from_ref(record)) {
        let at = derived.path.as_text();
        // **A path that cannot be looked at is not a path that is gone.**
        let identity_now = match std::fs::symlink_metadata(&at) {
            Ok(_) => Some(adapter.stable_identity_of(&at)),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => None,
            Err(_) => {
                confirmed = false;
                continue;
            }
        };
        match what_the_sweep_does(&derived, identity_now.as_ref()) {
            WhatTheSweepDoes::Remove => {
                if adapter.remove(&at).is_err() {
                    confirmed = false;
                }
            }
            WhatTheSweepDoes::NothingIsThere => {}
            WhatTheSweepDoes::LeaveItAndReportAnOwnershipMismatch
            | WhatTheSweepDoes::LeaveItBecauseTheIdentityWasNeverRecorded => confirmed = false,
        }
    }
    confirmed
}

/// **A leftover, removed** — `File Browser` U8's first bullet and `Download`
/// M13's second: *"Removing one first changes its transfer record durably to*
/// stopped: staging removed by user*, then removes only the exact staging
/// object whose synthesized name and stable file identity match that record (or
/// records it as an owed cleanup,* `Server protocol and transfers` S27*'s
/// shape)."* Added 2026-09-29, step 10c's
/// transfer wave piece 7.
///
/// **Every transfer record of the request that reads *interrupted* under the
/// server's current installation is written *stopped* first**, one at a time,
/// and only then is its material touched — `Server protocol and transfers`
/// C23's order, so a Smoozy that dies between the two leaves a record the next
/// sweep acts on rather than one a take-up would carry on.
///
/// - **A download's part is on this computer**, in the request's own
///   destination (`Server protocol and transfers` S1), and it is removed at
///   once on `Server protocol and transfers` S28's exact match
///   ([`the_staging_it_names_is_removed_here`]).
/// - **An upload's part is on the machine, and this does not reach it.** The
///   match `File Browser` U8 asks for needs the staging object's stable
///   identity, and on the machine the supplier cannot answer one — `Server
///   protocol and transfers` §12 item 15. So here the removal is recorded owed
///   (`Server protocol and transfers` S27), and **the transfer window's
///   *Remove* takes `File Browser` X12's explicit removal of the named entry
///   instead**, over the server's own point — `File Browser` U8 as amended
///   2026-10-03 (the 38's piece 9, item 33):
///   [`an_upload_leftover_is_removed_on_the_machine`]. This half is what a
///   *Remove* with no connection would be, and the Transfers screen does not
///   take it: it says the server is not connected instead, so the row keeps
///   its *Remove* for when it is.
///
/// **No connection is needed for a download's**, whose material is on this
/// computer.
pub fn a_leftover_is_removed(
    record: &dyn TheRecordADownloadWrites,
    the_run: &TheRunsTransfers,
    request: &Identity,
    server_row: &Identity,
    direction: Direction,
) -> TheLeftoverRemoved {
    let mut removed = TheLeftoverRemoved::default();
    let mut found: Vec<TransferRecord> = Vec::new();
    let mut installation_now: Option<String> = None;
    record.with_the_record(&mut |connection| {
        installation_now = crate::record::server_rows::one(connection, &server_row.as_text())
            .ok()
            .flatten()
            .and_then(|row| row.last_confirmed_installation_identity);
        let transfers = the_transfers_a_request_left(connection, request, server_row, direction);
        found = transfers
            .into_iter()
            .filter(|one| {
                one.ending == Some(Ending::Interrupted)
                    && one.installation.as_ref().map(Identity::as_text) == installation_now
            })
            .collect();
    });
    for mut one in found {
        // **Durably *stopped* first**, and only then the material.
        one.ending = Some(THE_USERS_STOP);
        one.cleanup_is_owed = true;
        let mut written = false;
        record.with_the_record(&mut |connection| {
            written = crate::record::transfer_records::put(connection, &one).is_ok();
        });
        if !written {
            removed.not_written += 1;
            continue;
        }
        let gone = the_staging_it_names_is_removed_here(&one);
        if gone {
            one.cleanup_is_owed = false;
            record.with_the_record(&mut |connection| {
                let _ = crate::record::transfer_records::put(connection, &one);
            });
            removed.removed += 1;
        } else {
            removed.owed += 1;
        }
    }
    the_run.every_leftover_is_read_again();
    record.a_transfer_moved();
    removed
}

/// **An upload's leftover removed on the machine** — `File Browser` U8's first
/// bullet as amended 2026-10-03 (the 38's piece 9, item 33, hole 12): *"the
/// transfer window's* Remove *takes* `File Browser` X12*'s explicit removal of a named entry
/// instead: it writes* stopped: staging removed by user *to every record naming
/// that staging path, then removes the named entry by path, only where the path
/// lies inside Smoozy's own staging directory on that machine; a path outside
/// it is refused and nothing is removed."* **A dependency send's leftover is
/// the same** (`Dependency Upload & Remap` A12), handed its records the same
/// way.
///
/// **`records` are the leftover's transfer records**, read by the caller under
/// the server's current installation (`File Browser` U8: *"under the current
/// installation identity"*): those reading *interrupted*, and those whose
/// cleanup is owed (`Server protocol and transfers` S27) — the second kind is
/// what an earlier *Remove* with no connection, or the sweep, left.
///
/// **In `Server protocol and transfers` C23's order**: a record reading
/// *interrupted* is written *stopped: staging removed by user* with its cleanup
/// owed, durably, before its material is touched; then each staging path the
/// record names (the live attempt and every superseded one, `Server protocol
/// and transfers` S4) is removed by path. **The stable file identity is not
/// asked** — it is the gate on *automatic* removal only (`File Browser` X12,
/// `Server protocol and transfers` S28), and the machine cannot answer one. A
/// path the machine answers *no such file* for is already gone. **The debt is
/// cleared only once every path is gone** (`Server protocol and transfers`
/// S27's *confirmed*).
///
/// **The bound is the path's own**: a staging path is `Server protocol and
/// transfers` S4's synthesis inside the
/// reserved folder `File Browser` X12 names, so a path whose folder is not that
/// folder — or that steps out of it — is refused, counted, and not sent.
pub async fn an_upload_leftover_is_removed_on_the_machine<C: AHeldConnection>(
    record: &dyn TheRecordADownloadWrites,
    the_run: &TheRunsTransfers,
    sessions: &ThisServersSessions<C>,
    records: Vec<TransferRecord>,
) -> TheLeftoverRemoved {
    use smoozy_server_protocol::subsystem::StatusCode;
    let mut removed = TheLeftoverRemoved::default();
    for mut one in records {
        if one.direction != Direction::Up {
            continue;
        }
        let interrupted = one.ending == Some(Ending::Interrupted);
        let owed = one.cleanup_is_owed
            && matches!(one.ending, Some(Ending::Stopped(_) | Ending::Failed | Ending::Stalled));
        if !interrupted && !owed {
            continue;
        }
        // **Durably *stopped* first**, and only then the material.
        if interrupted {
            one.ending = Some(THE_USERS_STOP);
            one.cleanup_is_owed = true;
            let mut written = false;
            record.with_the_record(&mut |connection| {
                written = crate::record::transfer_records::put(connection, &one).is_ok();
            });
            if !written {
                removed.not_written += 1;
                continue;
            }
        }
        let mut every_path_is_gone = true;
        for path in one.every_staging_path_it_has_named() {
            let path = path.as_bytes();
            if !it_lies_inside_a_staging_folder(&path) {
                removed.refused += 1;
                every_path_is_gone = false;
                continue;
            }
            let removing = path.clone();
            let answered = sessions
                .a_reading(false, move |subsystem| {
                    Box::pin(async move { subsystem.remove(&removing).await })
                })
                .await;
            match answered {
                Ok(Ok(())) | Ok(Err(SubsystemFailure::TheServerRefused(StatusCode::NoSuchFile))) => {}
                _ => every_path_is_gone = false,
            }
        }
        if every_path_is_gone {
            one.cleanup_is_owed = false;
            let mut written = false;
            record.with_the_record(&mut |connection| {
                written = crate::record::transfer_records::put(connection, &one).is_ok();
            });
            if written {
                removed.removed += 1;
            } else {
                removed.owed += 1;
            }
        } else {
            removed.owed += 1;
        }
    }
    the_run.every_leftover_is_read_again();
    record.a_transfer_moved();
    removed
}

/// **Whether a path lies inside Smoozy's own staging folder** — `File Browser`
/// `File Browser` X12's reserved folder, named by [`THE_RESERVED_STAGING_FOLDER`], holding
/// the entry directly: an absolute path whose folder's last component is that
/// name, with no component that steps anywhere (`.` or `..`) and a name of its
/// own. The bound `File Browser` U8's amended *Remove* is held to.
pub(crate) fn it_lies_inside_a_staging_folder(path: &[u8]) -> bool {
    if !path.starts_with(b"/") {
        return false;
    }
    let components: Vec<&[u8]> = path[1..].split(|byte| *byte == b'/').collect();
    if components.len() < 2
        || components
            .iter()
            .any(|component| component.is_empty() || *component == b"." || *component == b"..")
    {
        return false;
    }
    components[components.len() - 2] == THE_RESERVED_STAGING_FOLDER.as_bytes()
}

/// **What a leftover's *Remove* came to.**
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct TheLeftoverRemoved {
    /// Parts removed on the exact match.
    pub removed: usize,
    /// Parts whose removal is owed (`Server protocol and transfers` S27) —
    /// every upload's, and a download's whose object could not be claimed.
    pub owed: usize,
    /// Records the local record would not take, left as they were.
    pub not_written: usize,
    /// **Staging paths refused because they do not lie inside Smoozy's own
    /// staging folder** — `File Browser` U8's amended bound; nothing is sent
    /// for them. The 38's piece 9, item 33.
    pub refused: usize,
}

/// **Every transfer record an upload or manual download request's owed units
/// name**, each read whole — the entries with a transfer identity and no
/// outcome, which are the units a leftover is made of (`File Browser` U8,
/// `Download` M13). Step 10c's transfer wave piece 7.
///
/// **It reads the server's requests whole**, which is the one reading the
/// stores offer that carries an entry's transfer identity; so it is taken
/// once for a request and kept ([`WhatARequestLeft`]), never on every
/// Transfers ask.
pub(crate) fn the_transfers_a_request_left(
    connection: &rusqlite::Connection,
    request: &Identity,
    server_row: &Identity,
    direction: Direction,
) -> Vec<TransferRecord> {
    use smoozy_file_browser::request::KeepsTheRequest as _;
    let owed: Vec<Identity> = match direction {
        Direction::Up => crate::record::upload_requests::UploadRequests::over(connection)
            .every_request_for(server_row)
            .unwrap_or_default()
            .into_iter()
            .filter(|one| one.record.as_text() == request.as_text())
            .flat_map(|one| one.entries)
            .filter(|entry| entry.outcome.is_none())
            .filter_map(|entry| entry.transfer)
            .collect(),
        Direction::Down => DownloadRequests::over(connection)
            .every_request_for(server_row)
            .unwrap_or_default()
            .into_iter()
            .filter(|one| one.record.as_text() == request.as_text())
            .flat_map(|one| one.entries)
            .filter(|entry| entry.outcome.is_none())
            .filter_map(|entry| entry.transfer)
            .collect(),
    };
    owed.iter()
        .filter_map(|transfer| {
            crate::record::transfer_records::read(connection, transfer)
                .ok()
                .flatten()
        })
        .filter(|one| one.direction == direction)
        .collect()
}

/// **The interrupted transfers a fresh request of this server may take up** —
/// `File Browser` U8's second bullet and `Download` M13's third: records
/// reading *interrupted*, in this direction, under the installation the new
/// request is arranged under, whose own request's activity has ended (not under
/// way in this run). Read once, as the new request's run begins. Step 10c's
/// transfer wave piece 7.
pub(crate) fn the_parts_a_new_request_may_take_up(
    connection: &rusqlite::Connection,
    the_run: &TheRunsTransfers,
    server_row: &Identity,
    installation: Option<&Identity>,
    direction: Direction,
    the_new_request: &Identity,
) -> Vec<TransferRecord> {
    use smoozy_file_browser::request::KeepsTheRequest as _;
    let requests: Vec<(Identity, Vec<Identity>)> = match direction {
        Direction::Up => crate::record::upload_requests::UploadRequests::over(connection)
            .every_request_for(server_row)
            .unwrap_or_default()
            .into_iter()
            .map(|one| {
                let owed = one
                    .entries
                    .into_iter()
                    .filter(|entry| entry.outcome.is_none())
                    .filter_map(|entry| entry.transfer)
                    .collect();
                (one.record, owed)
            })
            .collect(),
        Direction::Down => DownloadRequests::over(connection)
            .every_request_for(server_row)
            .unwrap_or_default()
            .into_iter()
            .map(|one| {
                let owed = one
                    .entries
                    .into_iter()
                    .filter(|entry| entry.outcome.is_none())
                    .filter_map(|entry| entry.transfer)
                    .collect();
                (one.record, owed)
            })
            .collect(),
    };
    let under_way = the_run.what_is_under_way();
    requests
        .into_iter()
        .filter(|(request, _)| {
            request.as_text() != the_new_request.as_text()
                && !under_way.contains_key(&request.as_text())
        })
        .flat_map(|(_, owed)| owed)
        .filter_map(|transfer| {
            crate::record::transfer_records::read(connection, &transfer)
                .ok()
                .flatten()
        })
        .filter(|one| {
            one.direction == direction
                && one.ending == Some(Ending::Interrupted)
                && one.installation.as_ref() == installation
        })
        .collect()
}

/// **A part a fresh request will not take up, abandoned in the record first**
/// — `Server protocol and transfers` S28a: *"it first writes that transfer
/// record durably to* stopped*, with the reason* abandoned by its request*, and
/// only then arranges the replacement unit."* `File Browser` U8's third bullet
/// and `Download` M13's third: *Skip existing does not take it up at all.* Step
/// 10c's transfer wave piece 7.
///
/// **What becomes of the material is then the sweep's**: a download's at the
/// next start (`Server protocol and transfers` S28's second bullet), an
/// upload's at the next connection's — which, on the machine, can remove only
/// what an identity can be matched for (`Server protocol and transfers` §12
/// item 15).
pub(crate) fn a_part_is_abandoned(
    record: &dyn Fn(&mut dyn FnMut(&rusqlite::Connection)) -> bool,
    the_run: &TheRunsTransfers,
    part: &TransferRecord,
) -> bool {
    let mut abandoned = part.clone();
    abandoned.ending = Some(Ending::Stopped(StoppedReason::AbandonedByItsRequest));
    let mut written = false;
    record(&mut |connection| {
        written = crate::record::transfer_records::put(connection, &abandoned).is_ok();
    });
    the_run.every_leftover_is_read_again();
    written
}

// ---------------------------------------------------------------------------
// Where a download lands — the other thing Settings saves.
// ---------------------------------------------------------------------------

/// **Where a download is proposed to land** — `Download` H2 over this crate's
/// §8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhereItLands {
    /// **The folder the user pointed at**, which `Download` H2 makes the
    /// default the request or the switch-on moment shows.
    TheFolderTheUserChose(PathBuf),
    /// **The folder the system proposes**, where the user has chosen none.
    /// W3's *nothing caches it* holds: it is read fresh here every time.
    TheFolderTheSystemProposes(PathBuf),
    /// **W5: the route returned nothing, so nothing is proposed.** No composed
    /// path and no folder created.
    TheSystemNamesNoFolder(String),
    /// **The settings would not read.** Said as a failed read rather than
    /// taken as *the user chose none*, which would send a download somewhere
    /// they were never shown.
    TheSettingsWouldNotRead,
}

impl WhereItLands {
    /// The folder, where there is one.
    pub fn folder(&self) -> Option<&Path> {
        match self {
            WhereItLands::TheFolderTheUserChose(at)
            | WhereItLands::TheFolderTheSystemProposes(at) => Some(at),
            _ => None,
        }
    }
}

/// **`Download` H2's standing setting, read.**
///
/// The chosen folder where there is one; otherwise §8's proposal, read fresh
/// (W3); and where the system names none, nothing (W5).
///
/// **W4 is kept by this function holding no write of any kind.** The folder is
/// proposed and never made here, which is Apple's own instruction rather than
/// a preference.
pub fn where_a_download_lands(connection: &rusqlite::Connection) -> WhereItLands {
    let Ok(held) = settings::read(connection) else {
        return WhereItLands::TheSettingsWouldNotRead;
    };
    if let Some(chosen) = held.the_chosen_download_folder {
        return WhereItLands::TheFolderTheUserChose(the_path_of(&chosen));
    }
    match downloads_folder::the_folder_the_system_proposes() {
        TheProposal::TheSystemNamesThisFolder(at) => WhereItLands::TheFolderTheSystemProposes(at),
        TheProposal::TheSystemNamesNoFolder(said) => WhereItLands::TheSystemNamesNoFolder(said),
    }
}

/// The stored folder, back as a path. **The bytes are the ones the picker gave
/// and nothing is rewritten** — `File Browser` L1a's rule on this side.
#[cfg(unix)]
fn the_path_of(stored: &[u8]) -> PathBuf {
    use std::os::unix::ffi::OsStrExt;
    PathBuf::from(std::ffi::OsStr::from_bytes(stored))
}

#[cfg(not(unix))]
fn the_path_of(stored: &[u8]) -> PathBuf {
    PathBuf::from(String::from_utf8_lossy(stored).into_owned())
}

// ---------------------------------------------------------------------------
// One download.
// ---------------------------------------------------------------------------

/// **C19's inactivity deadline, which the caller declares and this is this
/// caller's.**
///
/// `Server protocol and transfers` C19 has it *"set well above the longest
/// legitimate stall observed"*, and the observation behind that clause is a
/// frozen transfer that sat at 17,233,920 bytes for **11.1 seconds** while
/// unfinished. Two minutes is an order of magnitude above it, and it is the
/// figure `App/Servers/tests/against_a_rented_machine.rs` already drives real
/// transfers at, so this caller does not introduce a second one.
///
/// **A capped channel never meets it.** The deadline runs while a channel
/// holds an allowance it cannot spend; a channel held to a low cap draws
/// nothing and waits, which is `Server protocol and transfers` R7 rather than
/// a stall.
pub const THE_INACTIVITY_DEADLINE: Duration = Duration::from_secs(120);

/// **Where one file lands, and which of the two the caller is.**
///
/// **`Download` R1 and `Download` M1 both fix a destination of their own** — an
/// arrangement's at the moment it was made, a request's at the moment it was
/// confirmed — and neither may drift to whatever the standing setting reads
/// later. **The standing setting is for a caller that has no record behind
/// it**, which is how `Download` H2's own default reaches a fetch.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhereTheCallerSaysItLands {
    /// **`Download` H2's standing setting, read fresh at this moment (W3)**,
    /// with this path composed beneath it — `Download` P11's *the structure the
    /// files had on the server is the structure they have when they land*.
    UnderTheStandingFolder(Vec<u8>),
    /// **The folder the caller already holds, with the landing path already
    /// composed.** An arrangement's destination is fixed for the life of the
    /// job (`Download` R1) and a request's is the one the user confirmed
    /// (`Download` M1); **the path the name check judged is the path the fetch
    /// lands at**, so neither is recomposed here.
    TheFolderTheCallerHolds {
        destination: Vec<u8>,
        landing: Vec<u8>,
    },
}

/// One file to bring down, as the caller names it.
#[derive(Debug, Clone)]
pub struct AFileToBringDown {
    /// The whole path on the machine. **Bytes, because a machine's names are
    /// bytes** (`Job files and outputs` N15).
    pub on_the_machine: Vec<u8>,
    /// Where it lands.
    pub lands: WhereTheCallerSaysItLands,
    /// The byte whole, from the enumeration.
    pub size: u64,
    /// Which of `Download` Q3's two classes this carries, which falls out of
    /// what arranged it and is never decided here.
    pub arranged_by: WhatArrangedIt,
    /// The server row the machine is, and the installation identity it was
    /// read under where there is one.
    pub server_row: Identity,
    pub installation: Option<Identity>,
    /// The application run this was arranged in.
    pub run: Identity,
    /// **The transfer identity, where the caller issued one.**
    ///
    /// **A fresh fetch is a new transfer with a new identity** (`State,
    /// identity and recovery` §9.5), and issuing one is that document's act.
    /// A caller that has to write the identity into its own record — an
    /// arrangement's reckoning (`Download` R4) or a request's entry
    /// (`Download` M1) — issues it first and hands it in, so the record and the
    /// transfer name one thing. **Absent, one is issued here.**
    pub transfer: Option<Identity>,
}

/// What one download came to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatTheDownloadCameTo {
    /// **`Server protocol and transfers` S26's ending, read as that document
    /// writes it and never re-derived.**
    pub ending: Ending,
    /// Where it was published, or would have been.
    pub lands_at: PathBuf,
    pub bytes_staged: u64,
    /// The class the desktop point admitted it under.
    pub admitted_as: &'static str,
    /// **The transfer identity this fetch ran under**, so the caller can write
    /// it into its own record — `Download` R4's account half, and `Download`
    /// M1's per-entry field.
    pub transfer: Identity,
    /// **What this computer said where it refused the file** — the first
    /// failure the platform file adapter returned while the file was staged
    /// or put in place, in its own words, and nothing where it refused
    /// nothing. It is what `Design/Transfers § A file that failed` puts behind
    /// *Details* (E2-1). Step 10c's thirteenth-pass fix piece 3.
    pub what_it_met: Option<String>,
    /// **The publication's exclusive create found the final name taken** —
    /// `Download` P6's case, which an auto-download writes at its entry
    /// rather than owing the file again. Added 2026-10-31, the twenty-sixth
    /// notes, note 4.
    pub the_name_was_taken: bool,
}

/// Why no download ran at all. **None of these is a transfer that failed.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhyNothingCameDown {
    /// There is nowhere to put it — W5's *the system names no folder*, or a
    /// settings read that did not answer.
    ThereIsNowhereToPutIt(WhereItLands),
    /// **R13: a refused channel is a reason to open fewer and never a reason
    /// to fail the transfer.** Nothing about the transfer failed; it was not
    /// admitted now, and the caller offers it again.
    TheDesktopPointAdmittedNothing,
    /// **The slot went to another class, so this unit is not the one to
    /// spend it on.**
    ///
    /// **A grant is never spent on a class that cannot take it**, which is the
    /// rule `Download`'s own per-server queue already keeps on its side. At an
    /// admissible count of one, spending a grant made to the waiting class on
    /// this caller's own unit is precisely how R17's turn-taking is defeated
    /// while the point still reads as though it were working. **The grant is
    /// handed back unspent and leaves no mark** — no channel is admitted, and
    /// the point's turn and share memory are as they stood (must-do 1) — and
    /// the caller offers the unit of the class named.
    TheSlotWentToAnotherClass(&'static str),
    /// A path that is not text, which the transfer record cannot carry. **The
    /// same refusal `Download`'s own arrangement takes**, taken here rather
    /// than converted lossily.
    TheNameCannotBeCarried,
}

/// **One file downloaded: admitted at the desktop point, carried out under
/// `Server protocol and transfers` §7, published into the folder the user
/// chose.**
///
/// **Three of this piece's clauses meet here.** R10 and R7's admission, which
/// is asked once per file. R13's refusal, which comes back as an absence.
/// **And R17's no-starving guarantee, which is kept by the draw being a local
/// of this call**: it is given back at the transfer's ending, and at an
/// admissible count of one the next grant goes to whichever class has waited
/// longest rather than back to this one.
///
/// **The transfer itself is `Server protocol and transfers` §7's loop taken
/// entire**, through `Download`'s own `acting::fetching`, and nothing about
/// the staging, the resume, the integrity split, the publication or the ending
/// is decided here. **The publication is create-intended** — `Download` P5 —
/// so C10's exclusive create is the refusal and nothing of the user's is
/// written over.
pub async fn one_file_comes_down<S>(
    subsystem: &mut Subsystem<S>,
    the_run: &TheRunsTransfers,
    connection: &mut rusqlite::Connection,
    asked: &AFileToBringDown,
    waiting: &[&'static str],
) -> Result<WhatTheDownloadCameTo, WhyNothingCameDown>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    let (folder, landing) = match &asked.lands {
        WhereTheCallerSaysItLands::TheFolderTheCallerHolds {
            destination,
            landing,
        } => (destination.clone(), landing.clone()),
        WhereTheCallerSaysItLands::UnderTheStandingFolder(under) => {
            let lands_in = where_a_download_lands(connection);
            let Some(folder) = lands_in.folder() else {
                return Err(WhyNothingCameDown::ThereIsNowhereToPutIt(lands_in));
            };
            let folder = the_bytes_of(folder);
            let landing = the_landing_path(&folder, under);
            (folder, landing)
        }
    };
    let mut keeper = TheLocalRecord { connection };
    the_file_comes_down(
        subsystem,
        the_run,
        &mut keeper,
        &mut NobodyIsWatching,
        &folder,
        &landing,
        asked,
        waiting,
    )
    .await
}

/// **The same file, for a caller whose record is a door rather than a
/// connection** — `src/uploading.rs`'s own shape, and the route both of this
/// piece's drivers take.
///
/// **The landing path is the caller's**, because `Download` R1 and `Download`
/// M1 each fix a destination of their own and neither may drift to what the
/// standing setting reads later.
///
/// **`watcher` is who the byte loop's reports go to** (`Server protocol and
/// transfers` G4): a manual download's row, or nobody.
pub async fn one_file_comes_down_for<S, K, W>(
    subsystem: &mut Subsystem<S>,
    the_run: &TheRunsTransfers,
    keeper: &mut K,
    watcher: &mut W,
    asked: &AFileToBringDown,
    waiting: &[&'static str],
) -> Result<WhatTheDownloadCameTo, WhyNothingCameDown>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
    K: KeepsTheRecord,
    W: WatchesTheTransfer,
{
    let WhereTheCallerSaysItLands::TheFolderTheCallerHolds {
        destination,
        landing,
    } = &asked.lands
    else {
        // **A caller with no connection cannot read `Download` H2's standing
        // setting**, and inventing a folder would send a download somewhere the
        // user was never shown.
        return Err(WhyNothingCameDown::ThereIsNowhereToPutIt(
            WhereItLands::TheSettingsWouldNotRead,
        ));
    };
    let (destination, landing) = (destination.clone(), landing.clone());
    the_file_comes_down(
        subsystem, the_run, keeper, watcher, &destination, &landing, asked, waiting,
    )
    .await
}

#[allow(clippy::too_many_arguments)]
async fn the_file_comes_down<S, K, W>(
    subsystem: &mut Subsystem<S>,
    the_run: &TheRunsTransfers,
    keeper: &mut K,
    watcher: &mut W,
    folder: &[u8],
    landing: &[u8],
    asked: &AFileToBringDown,
    waiting: &[&'static str],
) -> Result<WhatTheDownloadCameTo, WhyNothingCameDown>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
    K: KeepsTheRecord,
    W: WatchesTheTransfer,
{
    let (transfer_identity, mut record) = the_fetch_is_arranged(asked, folder, landing)?;
    the_arranged_fetch_comes_down(
        subsystem,
        the_run,
        keeper,
        watcher,
        landing,
        &mut record,
        transfer_identity,
        the_class(asked.arranged_by),
        waiting,
        None,
        None,
        // **`Download` P5: an arrangement has no disposition to read**, and
        // this route carries no manual request's answer: create-intended.
        the_intent(false),
        // **A first attempt resumes nothing**, so nothing is compared.
        None,
    )
    .await
}

/// **`Server protocol and transfers` S9's record for one fetch, arranged** —
/// with the transfer identity the caller issued, or one issued here.
fn the_fetch_is_arranged(
    asked: &AFileToBringDown,
    folder: &[u8],
    landing: &[u8],
) -> Result<(Identity, TransferRecord), WhyNothingCameDown> {
    let transfer_identity = match asked.transfer.clone() {
        Some(issued) => issued,
        None => Identity::issue(IdentityKind::Transfer)
            .map_err(|_| WhyNothingCameDown::TheNameCannotBeCarried)?,
    };
    let record = arrange_one_fetch(
        transfer_identity.clone(),
        asked.server_row.clone(),
        asked.installation.clone(),
        &asked.on_the_machine,
        landing,
        folder,
        asked.size,
        asked.run.clone(),
    )
    .map_err(|_| WhyNothingCameDown::TheNameCannotBeCarried)?;
    Ok((transfer_identity, record))
}

/// **One arranged fetch carried out**: admitted at the desktop point for this
/// one file, carried by `Server protocol and transfers` §7's loop, and the
/// slot given back at its ending. **Split from the arranging on 2026-09-29**
/// (step 10c's transfer wave piece 6), so a manual download's carry-on hands
/// the same record to the loop again rather than arranging a second.
///
/// **`the_user_may_stop` is the manual request the file belongs to**, whose
/// *Stop* — on the request or on this file — lands at the loop's next draw
/// ([`TheStopHoldsTheDraw`]) and is written here as `Server protocol and
/// transfers` S26's deliberate stop: the record *stopped* before anything
/// else (`Server protocol and transfers` C23), then the staged part removed on
/// this computer at once, or owed (`Server protocol and transfers` S27). An
/// auto-download is turned off rather than stopped (`Design/Transfers`) and
/// hands nothing. Step 10c's transfer wave piece 7.
#[allow(clippy::too_many_arguments)]
async fn the_arranged_fetch_comes_down<S, K, W>(
    subsystem: &mut Subsystem<S>,
    the_run: &TheRunsTransfers,
    keeper: &mut K,
    watcher: &mut W,
    landing: &[u8],
    record: &mut TransferRecord,
    transfer_identity: Identity,
    // **The class this file carries** — `Download` Q3's, which falls out of
    // what arranged it, or `Queue & History` P6's for a preview fetch. Handed
    // in rather than read off a request, the 38's piece 3.
    carries: &'static str,
    waiting: &[&'static str],
    the_user_may_stop: Option<&str>,
    // **A parameter, handed in for this one file and dropped at its end** —
    // never a field; written by its path so R17's own test below, which reads
    // field shapes off this crate's text, does not take it for one.
    admitted: Option<crate::scheduling::TheChannelsDraw>,
    // **`Server protocol and transfers` S15a: the publication's intent, which
    // the caller declares** — `Download` P5 and `Desktop app and delivery`
    // C14 through `Download`'s own `the_intent`.
    intent: Intent,
    // **`Server protocol and transfers` S13's four values as they stand now**,
    // read by the caller just before a resume ([`TheFourValuesOverThePoint`]);
    // nothing where no resume is in question. The 38's piece 9, item 18.
    the_source_now: Option<smoozy_server_protocol::transfers::SourceReading>,
) -> Result<WhatTheDownloadCameTo, WhyNothingCameDown>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
    K: KeepsTheRecord,
    W: WatchesTheTransfer,
{
    // **The admission, and it is asked for this one file.** The class this
    // file carries is `Download` Q3's, which falls out of what arranged it.
    // **A manual request's file is admitted before its seat is asked for**
    // ([`a_downward_slot`], step 10c's eleventh-pass fix piece 4) and hands
    // its draw in; everything else is admitted here, as it always was.
    // **A grant made to another class is handed back unspent, and it leaves no
    // mark** — step 10b round 1 of *jobs, history and downloads*, must-do 1.
    // The point decides over every class waiting and then, where the slot went
    // to another class, puts its turn and the share rule's memory back and
    // admits nothing, so the class that was granted is still the one whose
    // turn it is when its own file is offered. *(This read "the grant stands
    // for the class it was made to", and that was the fault: the turn moved on
    // with nothing spent, and at a count of one every offer was then refused.)*
    let mut draw = match admitted {
        Some(draw) => draw,
        None => the_run
            .admit_to_carry(Direction::Down, waiting, carries)
            .map_err(|granted| match granted {
                Some(another) => WhyNothingCameDown::TheSlotWentToAnotherClass(another),
                None => WhyNothingCameDown::TheDesktopPointAdmittedNothing,
            })?,
    };
    let admitted_as = draw.the_class();

    let mut local = TheAdapterKeepsWhatItMet::new();
    let parked = AtomicBool::new(false);
    let request = the_user_may_stop.unwrap_or_default();
    let mut budget = TheStopHoldsTheDraw {
        draw: &mut draw,
        the_run,
        request,
        landing,
        parked: &parked,
    };
    let carrying = carry_out_one_fetch(
        Transfer {
            subsystem,
            local: &mut local,
            budget: &mut budget,
            keeper,
            watcher,
            inactivity_deadline: THE_INACTIVITY_DEADLINE,
            // **`Download` P5 and `Desktop app and delivery` C14: every
            // publication is create-intended except a manual request's
            // user-answered *Replace*, which publishes with the replacing
            // rename** — read off the entry's disposition by the caller,
            // through `Download`'s own `the_intent`. Step 10c's thirteenth-pass
            // fix piece 3. *(This was fixed at create-intended, with the note
            // "this caller answers no replace of its own"; so the answer
            // reached nothing, the publication met the user's own file and
            // refused, and* Replace *never replaced.)*
            intent,
        },
        record,
        the_source_now,
    );
    let happened = if the_user_may_stop.is_some() {
        the_stop_lands_or(carrying, &parked).await
    } else {
        Some(carrying.await)
    };

    // **R5's *at once*, and R17's guarantee with it.** The channel's ending
    // returns its share and its place in the admissible count; the draw is
    // dropped at the end of this call whatever happened, so nothing of this
    // caller's survives the file it was admitted for.
    let _ = draw.the_channel_ended(std::time::Instant::now());
    drop(draw);

    let (ending, bytes_staged) = match happened {
        Some(happened) => (happened.ending, happened.report.bytes_staged),
        // **The user stopped it, and the loop is at its pause**: `Server
        // protocol and transfers` S26's deliberate stop, written here.
        None => {
            a_download_the_user_stopped_is_written_down(record, keeper);
            (THE_USERS_STOP, 0)
        }
    };
    Ok(WhatTheDownloadCameTo {
        ending,
        lands_at: the_path_of(landing),
        bytes_staged,
        admitted_as,
        transfer: transfer_identity,
        what_it_met: local.met,
        the_name_was_taken: local.the_name_was_taken,
    })
}

/// **The platform file adapter, keeping the first refusal it returned** — what
/// `Design/Transfers § A file that failed` puts behind a failed download's
/// *Details* (E2-1). Added 2026-10-01, step 10c's thirteenth-pass fix piece 3.
///
/// **Every act is the adapter's own, unchanged**: the outcome is still the
/// first error any call returned (`Desktop app and delivery` §6), and the loop
/// reads it exactly as it did. **What is added is that the words are kept**:
/// `Server protocol and transfers` §7's loop turns a refusal into the ending
/// *failed* and the words went nowhere, so a file this computer would not take
/// failed with no reason anywhere — the thirteenth pass's *"No reason was
/// shown anywhere"*. **Only the acts that write the file are kept**: the
/// staging's create, its writes, making it durable and putting it in place. A
/// removal is cleanup after the ending and is never why a file failed.
struct TheAdapterKeepsWhatItMet {
    adapter: ThePlatformFileAdapter,
    met: Option<String>,
    /// **The publication's exclusive create found the final name taken** —
    /// `Download` P6's case, kept beside the words. Added 2026-10-31, the
    /// twenty-sixth notes, note 4.
    the_name_was_taken: bool,
}

impl TheAdapterKeepsWhatItMet {
    fn new() -> TheAdapterKeepsWhatItMet {
        TheAdapterKeepsWhatItMet {
            adapter: ThePlatformFileAdapter,
            met: None,
            the_name_was_taken: false,
        }
    }

    fn kept<T>(&mut self, said: Result<T, LocalFailure>) -> Result<T, LocalFailure> {
        if let Err(LocalFailure(words)) = &said {
            self.met.get_or_insert_with(|| words.clone());
        }
        said
    }
}

impl TheUsersOwnComputer for TheAdapterKeepsWhatItMet {
    fn staged_size(&mut self, path: &str) -> Option<u64> {
        self.adapter.staged_size(path)
    }
    fn read_at(&mut self, path: &str, offset: u64, length: usize) -> Result<Vec<u8>, LocalFailure> {
        self.adapter.read_at(path, offset, length)
    }
    fn create_staging_exclusively(&mut self, path: &str) -> Result<(), LocalFailure> {
        let said = self.adapter.create_staging_exclusively(path);
        self.kept(said)
    }
    fn write_at(&mut self, path: &str, offset: u64, bytes: &[u8]) -> Result<(), LocalFailure> {
        let said = self.adapter.write_at(path, offset, bytes);
        self.kept(said)
    }
    fn make_durable(&mut self, path: &str) -> Result<(), LocalFailure> {
        let said = self.adapter.make_durable(path);
        self.kept(said)
    }
    fn stable_identity_of(
        &mut self,
        path: &str,
    ) -> smoozy_server_protocol::transfers::StableFileIdentity {
        self.adapter.stable_identity_of(path)
    }
    fn publish(&mut self, staging: &str, final_path: &str, intent: Intent) -> Result<(), LocalFailure> {
        let said = self
            .adapter
            .publish_saying_whether_the_name_was_taken(staging, final_path, intent)
            .map_err(|(failure, taken)| {
                self.the_name_was_taken |= taken;
                failure
            });
        self.kept(said)
    }
    fn remove(&mut self, path: &str) -> Result<(), LocalFailure> {
        self.adapter.remove(path)
    }
    fn read_the_final_object(
        &mut self,
        path: &str,
    ) -> smoozy_server_protocol::transfers::publication::WhatTheFinalObjectReadsAs {
        self.adapter.read_the_final_object(path)
    }
    fn the_sources_time(&mut self, source_path: &str) -> Option<i64> {
        self.adapter.the_sources_time(source_path)
    }
    /// **Not kept as what it met**: `Server protocol and transfers` S15c's
    /// refusal does not fail the transfer, so it is not a failure to tell.
    fn keep_the_time(&mut self, staging: &str, seconds: i64) -> Result<(), LocalFailure> {
        self.adapter.keep_the_time(staging, seconds)
    }
}

/// **What this computer said for each manual download's file that failed, in
/// this run** — keyed by the request and the file's landing path, which is the
/// shape `crate::uploading::the_refusals_this_run` holds an upload's refusals
/// in. **Held for this run only**: the record keeps the unit's outcome and not
/// its words, so a later sitting draws the file failed with no *Details*
/// (`Design/Transfers`: *where nothing was said, that control is absent*).
/// Step 10c's thirteenth-pass fix piece 3.
pub fn the_failures_this_run() -> &'static Mutex<HashMap<(String, Vec<u8>), String>> {
    static HELD: std::sync::OnceLock<Mutex<HashMap<(String, Vec<u8>), String>>> =
        std::sync::OnceLock::new();
    HELD.get_or_init(Default::default)
}

/// What this computer said for one failed file of one manual download, where
/// this run holds it — with a capital to open it and a full stop to close it,
/// as `Design/Whole app`'s failure shape shows another party's words.
pub fn what_this_computer_said_for(request: &str, landing: &[u8]) -> Option<String> {
    let said = the_failures_this_run()
        .lock()
        .ok()?
        .get(&(request.to_string(), landing.to_vec()))
        .cloned()?;
    let mut letters = said.chars();
    let first = letters.next()?;
    let rest = letters.as_str().trim_end_matches('.');
    Some(format!("{}{rest}.", first.to_uppercase()))
}

/// **`Server protocol and transfers` S26's deliberate stop, for a download the
/// user stopped** — the record *stopped* and durable before anything else
/// (`Server protocol and transfers` C23), then its staged part removed on this
/// computer on the exact match, and the debt written where it was not
/// (`Server protocol and transfers` S27). The channel is the caller's and goes
/// when its attempt ends. Step 10c's transfer wave piece 7.
///
/// **No unit outcome is written**: the unit is left owed, which is what `File
/// Browser` U6's *the one in flight is stopped* leaves — it did not fail.
fn a_download_the_user_stopped_is_written_down<K: KeepsTheRecord + ?Sized>(
    record: &mut TransferRecord,
    keeper: &mut K,
) {
    record.ending = Some(THE_USERS_STOP);
    record.cleanup_is_owed = true;
    if keeper.write_it_down(record).is_err() {
        return;
    }
    if the_staging_it_names_is_removed_here(record) {
        record.cleanup_is_owed = false;
        let _ = keeper.write_it_down(record);
    }
}

#[cfg(unix)]
fn the_bytes_of(path: &Path) -> Vec<u8> {
    use std::os::unix::ffi::OsStrExt;
    path.as_os_str().as_bytes().to_vec()
}

#[cfg(not(unix))]
fn the_bytes_of(path: &Path) -> Vec<u8> {
    path.to_string_lossy().into_owned().into_bytes()
}


// ---------------------------------------------------------------------------
// The receiving system's own rules — C3, C4, C5 and C7.
// ---------------------------------------------------------------------------

/// **C3's name check and C4's four outcomes, against the system this copy is
/// running on** — the far side of `Download`'s
/// [`TheReceivingSystemsRules`] seam, which until step 10c piece 10 had no
/// implementation outside that crate's own tests.
///
/// **C3 fixes the moment and the reason.** Fact 24: macOS refuses invalid
/// UTF-8 outright while Linux permits any byte but `/` and NUL, *"so a frame
/// can come back carrying a name that cannot be written to an APFS volume at
/// all, and that must be caught before the transfer rather than during it."*
///
/// **C5 fixes the unit, and it is two units rather than three.** Linux is 255
/// bytes; Windows and macOS are 255 UTF-16 code units, the macOS figure
/// measured rather than documented. **The check is per component, in the
/// receiving system's own unit, and never once in characters.**
///
/// **C7's three silent failures are refusals and not successes**, because a
/// silent transformation is exactly what Row 303 forbids: Windows strips
/// trailing periods and spaces, and collapses a reserved stem to a device.
///
/// **What this does not do, said rather than left to be found.** It does not
/// ask the receiving **volume** anything: C3's second half wants the check made
/// against the volume the file is landing on, and nothing here reads one.
/// **What stands without it is the system's own rule**, which is the floor
/// every volume of that system is at least as strict as for the cases C4 names
/// — so the check refuses what the system refuses and may accept what one
/// unusual volume would not, and that name then meets `Server protocol and
/// transfers` S26's own failure at the write rather than a silence. **C9's
/// thirty-one combining marks are kept on macOS** because that limit is a
/// property of the encoding rather than of a volume.
#[derive(Debug, Clone, Copy, Default)]
pub struct TheReceivingSystemsRulesHere;

/// **C5's bound, in whichever unit this system counts in.** One number, not
/// three: every system the release matrix names takes 255 of its own unit.
const THE_LONGEST_COMPONENT: usize = 255;

/// **C9, measured and undocumented, on one volume format**, and carried as a
/// limit of the check rather than as a rule about the system.
#[cfg(target_os = "macos")]
const THE_MOST_COMBINING_MARKS_A_BASE_CARRIES: usize = 31;

impl TheReceivingSystemsRules for TheReceivingSystemsRulesHere {
    fn check(&self, component: &[u8]) -> TheNameCheckSaid {
        // **An empty component is not a name.** It is never a render output's
        // name either — the bytes come from a listing — so this is the caller
        // handing in nothing rather than a system's limit.
        if component.is_empty() {
            return TheNameCheckSaid::NotRepresentable;
        }
        // **`/` and NUL are the two bytes no system takes inside a
        // component**, on C3's own reading of Fact 24.
        if component.contains(&b'/') || component.contains(&0) {
            return TheNameCheckSaid::NotRepresentable;
        }
        the_systems_own_check(component)
    }

    /// **What this system puts in place of what cannot enter a name**, used
    /// only where P10 composes a name of Smoozy's own.
    ///
    /// **P8 is why it is never applied to a render output's name**: substituting
    /// an artist's output name would be the silent transformation Row 303
    /// forbids arriving by Smoozy's own hand. A test below reads this file and
    /// fails if the fetch path ever reaches it.
    fn substitute(&self, component: &[u8]) -> Vec<u8> {
        let mut out: Vec<u8> = component
            .iter()
            .map(|byte| {
                if the_byte_may_enter_a_name(*byte) {
                    *byte
                } else {
                    b'-'
                }
            })
            .collect();
        // **C7's trailing periods and spaces**, taken off rather than left for
        // Windows to take off silently. It is done on every system so that one
        // job's folder is named the same wherever it lands.
        while matches!(out.last(), Some(b'.') | Some(b' ')) {
            out.pop();
        }
        if out.is_empty() {
            out.push(b'-');
        }
        out
    }

    fn length_in_this_systems_own_unit(&self, component: &[u8]) -> usize {
        the_systems_own_length(component)
    }

    fn the_longest_component(&self) -> usize {
        THE_LONGEST_COMPONENT
    }
}

/// **C5's unit on this system**: UTF-16 code units on Windows and macOS, bytes
/// on Linux.
#[cfg(any(target_os = "macos", target_os = "windows"))]
fn the_systems_own_length(component: &[u8]) -> usize {
    String::from_utf8_lossy(component).encode_utf16().count()
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
fn the_systems_own_length(component: &[u8]) -> usize {
    component.len()
}

/// Whether one byte may stand in a name Smoozy itself composes.
///
/// **The set is the union of the three systems' own refusals**, because P10's
/// folder name is Smoozy's composition and the same job should land in the same
/// folder name on every system.
fn the_byte_may_enter_a_name(byte: u8) -> bool {
    !matches!(
        byte,
        b'/' | b'\\' | b':' | b'*' | b'?' | b'"' | b'<' | b'>' | b'|' | 0..=31
    )
}

#[cfg(target_os = "macos")]
fn the_systems_own_check(component: &[u8]) -> TheNameCheckSaid {
    // **Fact 24: macOS refuses invalid UTF-8 outright with `EILSEQ`.**
    let Ok(text) = std::str::from_utf8(component) else {
        return TheNameCheckSaid::NotRepresentable;
    };
    if the_systems_own_length(component) > THE_LONGEST_COMPONENT {
        return TheNameCheckSaid::TooLongInThisSystemsOwnUnit;
    }
    // **C9: a base character carries at most thirty-one combining marks before
    // `EILSEQ`.**
    let mut since_a_base = 0usize;
    for character in text.chars() {
        if a_combining_mark(character) {
            since_a_base += 1;
            if since_a_base > THE_MOST_COMBINING_MARKS_A_BASE_CARRIES {
                return TheNameCheckSaid::NotRepresentable;
            }
        } else {
            since_a_base = 0;
        }
    }
    TheNameCheckSaid::TheNameMayBeWrittenAsGiven
}

/// The combining ranges Unicode defines, read off the standard's own block
/// names. **Nothing here is a judgement about a system**; it is what C9's
/// measured limit counts.
#[cfg(target_os = "macos")]
fn a_combining_mark(character: char) -> bool {
    let at = character as u32;
    (0x0300..=0x036F).contains(&at)
        || (0x1AB0..=0x1AFF).contains(&at)
        || (0x1DC0..=0x1DFF).contains(&at)
        || (0x20D0..=0x20FF).contains(&at)
        || (0xFE20..=0xFE2F).contains(&at)
}

#[cfg(target_os = "windows")]
fn the_systems_own_check(component: &[u8]) -> TheNameCheckSaid {
    // **A Windows path is UTF-16**, so a component whose bytes are not text
    // cannot be named to the system at all.
    let Ok(text) = std::str::from_utf8(component) else {
        return TheNameCheckSaid::NotRepresentable;
    };
    if text
        .chars()
        .any(|character| matches!(character, '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|')
            || (character as u32) < 32)
    {
        return TheNameCheckSaid::NotRepresentable;
    }
    if the_systems_own_length(component) > THE_LONGEST_COMPONENT {
        return TheNameCheckSaid::TooLongInThisSystemsOwnUnit;
    }
    // **C7, both halves, and both are silent transformations rather than
    // errors** — the file lands under a different name, or is discarded into a
    // device.
    if text.ends_with('.') || text.ends_with(' ') {
        return TheNameCheckSaid::SilentlyTransformed;
    }
    let stem = text.split('.').next().unwrap_or(text).to_ascii_uppercase();
    let reserved = matches!(
        stem.as_str(),
        "CON" | "PRN" | "AUX" | "NUL" | "COM1" | "COM2" | "COM3" | "COM4" | "COM5" | "COM6"
            | "COM7" | "COM8" | "COM9" | "LPT1" | "LPT2" | "LPT3" | "LPT4" | "LPT5" | "LPT6"
            | "LPT7" | "LPT8" | "LPT9"
    );
    if reserved {
        return TheNameCheckSaid::SilentlyTransformed;
    }
    TheNameCheckSaid::TheNameMayBeWrittenAsGiven
}

/// **Linux permits any byte but `/` and NUL**, which C3's own reading of
/// Fact 24 states, so the only remaining question is C5's length in bytes.
#[cfg(not(any(target_os = "macos", target_os = "windows")))]
fn the_systems_own_check(component: &[u8]) -> TheNameCheckSaid {
    if the_systems_own_length(component) > THE_LONGEST_COMPONENT {
        return TheNameCheckSaid::TooLongInThisSystemsOwnUnit;
    }
    TheNameCheckSaid::TheNameMayBeWrittenAsGiven
}

// ---------------------------------------------------------------------------
// The one local record, as a download reaches it.
// ---------------------------------------------------------------------------

/// **The one local record, as a download reaches it.**
///
/// `State, identity and recovery` §1.3 rule 1's one writer per durable store is
/// why this is a door rather than a connection: **every write a download makes
/// — the arrangement, the request, each transfer record, each unit's outcome —
/// goes through the application's own record, one short act at a time**, and
/// never through a second connection to the same file. It is
/// `src/uploading.rs`'s own shape in the other direction, and the two are
/// deliberately identical.
pub trait TheRecordADownloadWrites: Send + Sync {
    /// Runs `act` against the record, and says whether the record was there to
    /// run it against.
    fn with_the_record(&self, act: &mut dyn FnMut(&rusqlite::Connection)) -> bool;

    /// **A unit ended, or the whole grew, and it is written down.** Called
    /// straight after the durable write that made it true, and never before
    /// (`Desktop app and delivery` P7). **The running application's doors post
    /// `crate::surface::transfers::TRANSFERS_MOVED` here**, so a Transfers page
    /// that is showing asks for its state again — `Download` M16's count and
    /// `Design/Whole app § Anything that isn't instant`'s *on the row* moving
    /// while files land rather than when the page is next opened. **The event
    /// carries nothing**: `Desktop app and delivery` P8 makes events volatile,
    /// so the page asks rather than being handed the state.
    ///
    /// *Until 2026-09-27 nothing told the page, and a Transfers row sat at
    /// 0 / 3 with every file already on disk (the seventh passes, Vast fault 7
    /// and Hyperstack fault 8).* A door with nobody to tell does nothing.
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
}

/// **`Download` R1's keeper over the application's own record door.**
///
/// **A record that is not there is a refusal and never a silent success**: an
/// arrangement whose write did not land would fetch files nobody could account
/// for afterwards, which is the one thing R4 exists to prevent.
pub struct TheArrangementsKeeper<'a> {
    pub record: &'a dyn TheRecordADownloadWrites,
}

impl KeepsTheArrangement for TheArrangementsKeeper<'_> {
    fn write_it_down(&mut self, arrangement: &AnArrangement) -> Result<(), Refused> {
        let mut wrote: Result<(), Refused> = Err(the_record_was_not_there());
        let there = self.record.with_the_record(&mut |connection| {
            wrote = DownloadArrangements::over(connection).write_it_down(arrangement);
        });
        if !there {
            return Err(the_record_was_not_there());
        }
        wrote
    }

    fn the_arrangement_for(&self, job: &Identity) -> Result<Option<AnArrangement>, Refused> {
        let mut read: Result<Option<AnArrangement>, Refused> = Err(the_record_was_not_there());
        let there = self.record.with_the_record(&mut |connection| {
            read = DownloadArrangements::over(connection).the_arrangement_for(job);
        });
        if !there {
            return Err(the_record_was_not_there());
        }
        read
    }

    fn it_ended(&mut self, job: &Identity, how: WhatEndsIt) -> Result<(), Refused> {
        let mut ended: Result<(), Refused> = Err(the_record_was_not_there());
        let there = self.record.with_the_record(&mut |connection| {
            ended = DownloadArrangements::over(connection).it_ended(job, how);
        });
        if !there {
            return Err(the_record_was_not_there());
        }
        ended
    }
}

/// **`Download` M1's keeper over the same door.**
pub struct TheRequestsKeeper<'a> {
    pub record: &'a dyn TheRecordADownloadWrites,
}

impl KeepsTheRequest for TheRequestsKeeper<'_> {
    fn write_it_down(&mut self, request: &AManualRequest) -> Result<(), Refused> {
        let mut wrote: Result<(), Refused> = Err(the_record_was_not_there());
        let there = self.record.with_the_record(&mut |connection| {
            wrote = DownloadRequests::over(connection).write_it_down(request);
        });
        if !there {
            return Err(the_record_was_not_there());
        }
        wrote
    }

    fn every_request_for(&self, server_row: &Identity) -> Result<Vec<AManualRequest>, Refused> {
        let mut read: Result<Vec<AManualRequest>, Refused> = Err(the_record_was_not_there());
        let there = self.record.with_the_record(&mut |connection| {
            read = DownloadRequests::over(connection).every_request_for(server_row);
        });
        if !there {
            return Err(the_record_was_not_there());
        }
        read
    }

    fn the_user_removed_it(&mut self, record: &Identity) -> Result<(), Refused> {
        let mut removed: Result<(), Refused> = Err(the_record_was_not_there());
        let there = self.record.with_the_record(&mut |connection| {
            removed = DownloadRequests::over(connection).the_user_removed_it(record);
        });
        if !there {
            return Err(the_record_was_not_there());
        }
        removed
    }
}

fn the_record_was_not_there() -> Refused {
    Refused::TheStore("the local record is not open, so nothing was written".into())
}

// ---------------------------------------------------------------------------
// The reading route a walk rides.
// ---------------------------------------------------------------------------

/// **W1's route, over `Server protocol and transfers` T5's per-server point.**
///
/// **One directory reading per folder, each on the connection's one long-lived
/// reading session**, which is what W2's aggregate bound is arithmetic about.
/// `Download`'s own `acting::wiring::TheReadingRoute` is what performs each
/// reading; what this adds is **where the subsystem comes from**, which is the
/// thing that crate said it had no producer for: *"Nobody in this workspace
/// opens, holds or reopens a reading channel for a connected period."* Piece 2
/// built the holder and this is the first caller.
///
/// **Nobody is waiting on a walk.** `Server protocol and transfers` §1.1 item 2
/// puts a reading the user is waiting on before one nobody is waiting on, and a
/// walk is the second kind — so a user's own listing on the Files screen goes
/// first, every time.
pub struct TheWalksReadingRoute<'a, C: AHeldConnection> {
    sessions: &'a ThisServersSessions<C>,
}

impl<'a, C: AHeldConnection> TheWalksReadingRoute<'a, C> {
    pub fn on(sessions: &'a ThisServersSessions<C>) -> TheWalksReadingRoute<'a, C> {
        TheWalksReadingRoute { sessions }
    }
}

impl<C: AHeldConnection> ReadsOneDirectoryOnTheMachine for TheWalksReadingRoute<'_, C> {
    /// **One folder's reading, asked of the point and performed by
    /// `Download`'s own route.**
    ///
    /// **A point that lends nothing is `SubsystemFailure::TheChannelIsGone`**,
    /// and that is the honest value rather than a convenience: the walk has no
    /// channel, W5 discards it whole, the next one runs at the cadence, and
    /// `Servers`' own connection condition is what covers a machine that has
    /// gone. **Nothing here composes a sentence about it** — the cause travels
    /// as the protocol's own value.
    async fn read_one_directory(
        &mut self,
        path: &[u8],
    ) -> Result<Vec<ADirectoryEntry>, SubsystemFailure> {
        let path = path.to_vec();
        self.sessions
            .a_reading(false, move |subsystem| {
                Box::pin(async move { TheReadingRoute::on(subsystem).read_one_directory(&path).await })
            })
            .await
            .unwrap_or(Err(SubsystemFailure::TheChannelIsGone))
    }
}

// ---------------------------------------------------------------------------
// An arrangement, made at submission.
// ---------------------------------------------------------------------------

/// **What a submission hands the arrangement**, and every one of them is a value
/// the submit act already holds.
#[derive(Debug, Clone)]
pub struct WhatTheSubmissionChose {
    pub job: Identity,
    pub server_row: Identity,
    /// **`Download` R1's destination**: the folder chosen or accepted at the
    /// moment the arrangement was made, **fixed for the life of the job**.
    /// Nothing can be asked of a user who is not there.
    pub destination: Vec<u8>,
    /// **`Download` N3's narrowing**, as the submit strip's own boxes composed
    /// it — a chosen list, or *everything*, which is no filter rather than a
    /// frozen list.
    pub narrowing: TheNarrowing,
}

/// **Why no arrangement was made.** None of these is a submission that failed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhyNoArrangementWasMade {
    /// **The submit strip's choice was off.** `Rendering` puts the moment at
    /// submitting, and an arrangement is made where the choice is on and not
    /// otherwise — step 10b's own scenario is the other route and says outright
    /// *"auto-download was never switched on."*
    TheChoiceWasOff,
    /// **There is nowhere to put it** — no folder was chosen and the system
    /// names none (W5), or the settings would not read. **`Download` R1 fixes
    /// the destination at the moment the arrangement is made**, so an
    /// arrangement with no destination would have to ask a user who is not
    /// there.
    ThereIsNowhereToPutIt(WhereItLands),
    /// **The record would not take it.** `Download` R1's arrangement is one
    /// durable record, so an arrangement that is not written down does not
    /// exist — and a fetch made under one that was never written would arrive
    /// with nothing to account for it.
    TheRecordWouldNotTakeIt(Refused),
}

/// **`Download` R1, performed at the moment `Rendering` puts it: an
/// auto-download arrangement is created when auto-download is switched on at
/// submission.**
///
/// **Its desired state is on, its destination and narrowing are fixed here, and
/// its reckoning is empty** — nothing has been acted on yet, and `Download` R5
/// makes the reckoning an account of Smoozy's own acts rather than of what is
/// on the machine.
///
/// **It is durable before it is anything else.** The write is the whole act
/// (`Download` §12: *switching an arrangement on or off* is atomic entire, one
/// durable write of the arrangement record), so a crash a moment later leaves
/// an arrangement that resumes on the next connection rather than a job whose
/// frames nobody is fetching.
///
/// **Where the choice was off, nothing is written and no arrangement exists.**
/// That is not an arrangement switched off: a switched-off arrangement is a
/// record the user can switch on again from the queue entry, and a job
/// submitted with the choice off has never had one.
pub fn the_arrangement_is_made_at_submission<K: KeepsTheArrangement + ?Sized>(
    keeper: &mut K,
    the_choice_is_on: bool,
    chose: WhatTheSubmissionChose,
) -> Result<AnArrangement, WhyNoArrangementWasMade> {
    if !the_choice_is_on {
        return Err(WhyNoArrangementWasMade::TheChoiceWasOff);
    }
    if chose.destination.is_empty() {
        return Err(WhyNoArrangementWasMade::ThereIsNowhereToPutIt(
            WhereItLands::TheSystemNamesNoFolder(
                "the submission named no folder for the arrangement".into(),
            ),
        ));
    }
    let arrangement = AnArrangement::switched_on(
        chose.job,
        chose.server_row,
        chose.destination,
        chose.narrowing,
    );
    keeper
        .write_it_down(&arrangement)
        .map_err(WhyNoArrangementWasMade::TheRecordWouldNotTakeIt)?;
    Ok(arrangement)
}

// ---------------------------------------------------------------------------
// An arrangement, run while a period stands.
// ---------------------------------------------------------------------------

/// **How long a fetch waits before asking the desktop point again**, where the
/// point had no room for it.
///
/// **R13: a refusal is a reason to open fewer and never a reason to fail the
/// transfer**, so the unit is offered again rather than failed. The figure is
/// `src/uploading.rs`'s and is not a second one.
const THE_PAUSE_BEFORE_ASKING_AGAIN: Duration = Duration::from_millis(100);

/// **One turn of one arrangement: its walk, and what the walk made owed** —
/// `Download` §7 E1's steps 1, 2 and 4 with `Server protocol and transfers`
/// T5's sessions under it.
///
/// **The order is `Download`'s and nothing here re-decides it.** The walk is
/// [`TheArrangementsRun::one_turn`]'s, over [`TheWalksReadingRoute`], and the
/// units it makes owed wait in the arrangement's queue.
///
/// **`Download` E1 step 3, the fetches, is not in the turn any more: it is
/// [`the_owed_fetches_come_down`], run once a pass has walked every
/// arrangement.** Changed 2026-09-28, step 10c's ninth-pass fix piece 1. This
/// function used to drain the whole queue before it returned, so on the ninth
/// pass one job's 240-file ending drain held every other arrangement's walk
/// for seven minutes, and a second job's frames came only after the first
/// job's had all landed. **`Download` W2 and W3 give each arrangement its own
/// walk at its own pause**, which a turn that fetches until its queue is empty
/// cannot keep. **The case this was written for** was one arrangement, whose
/// own walks a drain only delays; **the case it also meets now** is several
/// on one server.
///
/// **`Download` W5 is kept by this function doing nothing with a discarded walk.** A walk
/// that did not complete is not a reading, so its verdict is never applied —
/// which is what stops an arrangement that met a refused listing marking every
/// frame of the job *never coming*. The turn says it was discarded, with the
/// protocol's own cause, and the next one runs at the cadence.
pub async fn one_turn_of_an_arrangement<C: AHeldConnection>(
    running: &mut TheArrangementsRun,
    sessions: &ThisServersSessions<C>,
    record: &dyn TheRecordADownloadWrites,
    queue: &mut TheFetchQueue,
    arrangement: &mut AnArrangement,
    in_hand: WhatIsInHand<'_>,
) -> Result<WhatTheTurnDid, Refused> {
    let rules = TheReceivingSystemsRulesHere;
    let the_whole_before = arrangement.reckoning.len();
    let the_turn = {
        let mut reader = TheWalksReadingRoute::on(sessions);
        let mut keeper = TheArrangementsKeeper { record };
        running
            .one_turn(
                &mut reader,
                &rules,
                &mut keeper,
                queue,
                arrangement,
                in_hand,
            )
            .await?
    };
    // **The walk wrote entries the row counts** — the ending walk's *never
    // coming*, or the whole growing — so the page is told before any fetch.
    if arrangement.reckoning.len() != the_whole_before {
        record.a_transfer_moved();
    }
    Ok(the_turn)
}

/// **How many of one server's auto-download fetches run at once: `Download`
/// Q1's ceiling of two.**
///
/// **Measured before it was taken**, as the brief for step 10c's ninth-pass fix
/// piece 1 asked: see `Audits/Build/Step 10c/Ninth-pass fix piece 1 — what
/// landed.md`. It is `Download` Q1's own figure, read from that crate's own
/// declaration and never written a second time, and it is a ceiling over the
/// whole server — the fetch queues are held per arrangement, so it is counted
/// here, across them.
const AT_ONCE: usize = smoozy_download::classes::THE_DOWNLOAD_DIRECTIONS_CEILING;

/// **One arrangement whose owed units may come down in this pass**, with
/// everything its fetches and their endings are handed.
pub struct AnArrangementOwingFetches<'a> {
    pub running: &'a mut TheArrangementsRun,
    pub queue: &'a mut TheFetchQueue,
    pub probing: &'a mut TheProbesCadence,
    pub arrangement: &'a mut AnArrangement,
    pub in_hand: WhatIsInHand<'a>,
}

/// **What the fetches of one pass came to.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatTheFetchesCameTo {
    /// **Files at their final names**, which is `Download` R4's *fetched*.
    pub fetched: usize,
    /// **`Download` §12's diary act, composed once at an arrangement's done
    /// transition** and at no other moment.
    pub the_diary_acts: Vec<AnAct>,
    /// **What is left of the pause the pass was given**, which is what the
    /// sequence then waits before the next pass's walks.
    pub what_is_left_of_the_pause: Duration,
}

/// **`Download` E1 step 3, for every arrangement of one server at once: the
/// owed units come down, in turn, until the pause before the next walks is
/// spent or nothing more can be fetched.** Added 2026-09-28, step 10c's
/// ninth-pass fix piece 1.
///
/// **`Download` W2 and W3: each arrangement walks at its own pause, and no
/// arrangement's walk waits on another's fetches.** So the fetches of a pass
/// run inside the pause `Download` W2 sets between two walks, and **a unit
/// not started before that pause is spent waits for the next pass** — whose
/// walks come first. At worst a walk is late by the files in flight when the
/// pause ran out.
///
/// **The arrangements take turns, one unit each**, so one job's long ending
/// drain does not hold a second job's frames until it is over — the ninth
/// pass's fault, where *p12h9 long2*'s first frame came 2 min 54 s after its
/// own render ended. **Within an arrangement the order is still `Download`
/// P2's**, the queue's own offer. **Nothing here re-derives `File Browser` O5's share**:
/// every unit here is one class, and the desktop point still grants per file.
///
/// **Two at once, and never more: `Download` Q1's ceiling** — see [`AT_ONCE`].
/// **Each of the two takes the next unit when its own ends.** Changed
/// 2026-10-16, step 10c's owed piece 3. **The case this was first written for**
/// was two units taken together and both waited for before the next two, which
/// costs nothing where the two take the same time. **The case it now also
/// meets** is a server where each file's channel is seated in its turn at the
/// one point (`Server protocol and transfers` T5), so the second of a pair
/// always starts and ends a seat later than the first, and the first then
/// stood idle for that seat every pair — one channel waiting on the other.
/// The manual request's two channels were made this way at step 10c's
/// eleventh-pass fix piece 4 ([`every_one_of`]).
///
/// **R17 is kept as it was.** The desktop point's slot and the server's seat
/// are asked for per file and given back at that file's end, so an upload or a
/// manual download waiting for either is seated between two of these files.
///
/// **An arrangement whose unit could not start leaves the rest of this pass's
/// fetches to the others**: a unit whose size could not be established, or
/// that the point admitted nothing for, goes back to *owed* (R13: a reason to
/// open fewer, never to fail) and is offered again next pass. So the loop
/// never spins on a unit it cannot start.
pub async fn the_owed_fetches_come_down<C: AHeldConnection>(
    owing: &mut [AnArrangementOwingFetches<'_>],
    sessions: &ThisServersSessions<C>,
    the_run: &TheRunsTransfers,
    record: &dyn TheRecordADownloadWrites,
    the_run_this_is: &Identity,
    the_pause: Duration,
) -> WhatTheFetchesCameTo {
    let began = std::time::Instant::now();
    let pass = ThePassUnderWay {
        held: Mutex::new(WhatThePassHolds {
            it_cannot_start: vec![false; owing.len()],
            owing,
            next: 0,
            under_way: 0,
            came_to: WhatTheFetchesCameTo {
                fetched: 0,
                the_diary_acts: Vec::new(),
                what_is_left_of_the_pause: the_pause,
            },
            anything_moved: false,
        }),
        a_unit_ended: tokio::sync::Notify::new(),
        began,
        the_pause,
    };
    let channels = (0..AT_ONCE)
        .map(|_| a_channel_of_the_pass(&pass, sessions, the_run, record, the_run_this_is))
        .collect();
    every_one_of(channels).await;
    let held = pass
        .held
        .into_inner()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let mut came_to = held.came_to;
    // **Nothing fetched is nothing spent**: the pass hands the sequence the
    // whole pause, which is `Download` W2's own figure unchanged.
    if held.anything_moved {
        came_to.what_is_left_of_the_pause = the_pause.saturating_sub(began.elapsed());
    }
    came_to
}

/// **One pass's fetches while they are under way**, shared by its two
/// channels. Added 2026-10-16, step 10c's owed piece 3.
///
/// **The lock is never held across a wait**: a unit is taken, arranged and
/// its ending applied under it, and its bytes come down outside it.
struct ThePassUnderWay<'o, 'a> {
    held: Mutex<WhatThePassHolds<'o, 'a>>,
    /// **Told each time a unit's ending is applied**, so a channel that found
    /// nothing to take while the other was moving a file asks again — the
    /// queue offers nothing while a publication is pending
    /// (`Download`'s `transfers_may_be_admitted`), and a unit that failed
    /// comes back to *owed* (`Download` P3).
    a_unit_ended: tokio::sync::Notify,
    began: std::time::Instant,
    the_pause: Duration,
}

struct WhatThePassHolds<'o, 'a> {
    owing: &'o mut [AnArrangementOwingFetches<'a>],
    it_cannot_start: Vec<bool>,
    /// The arrangement whose turn it is next.
    next: usize,
    /// Units taken and not yet ended.
    under_way: usize,
    came_to: WhatTheFetchesCameTo,
    anything_moved: bool,
}

/// **What a channel of the pass was handed when it asked for the next unit.**
enum WhatTheChannelTook {
    /// **A unit, and the size a walk read of it** — none where a walk read
    /// none, and the file's own attributes are then read on the machine.
    AUnit {
        at: usize,
        unit: AUnitOwed,
        the_walks_size: Option<u64>,
        the_job_output_folder: Vec<u8>,
    },
    /// **Nothing now, but the other channel is moving a file**, whose ending
    /// may make a unit offerable.
    NothingYet,
    /// **Nothing, and nothing is moving**: this channel's part of the pass is
    /// over.
    Nothing,
}

impl WhatThePassHolds<'_, '_> {
    /// **The next unit, the arrangements taking turns.**
    fn the_next_unit(&mut self, record: &dyn TheRecordADownloadWrites) -> WhatTheChannelTook {
        let count = self.owing.len();
        for step in 0..count {
            let at = (self.next + step) % count;
            if self.it_cannot_start[at] {
                continue;
            }
            let one = &mut self.owing[at];
            let standing = the_transfer_records_standing(record, &one.arrangement.server_row);
            let Some(unit) = one.queue.offer(&standing) else {
                continue;
            };
            // **The size the fetch is arranged against, from the walk that
            // made the file fetchable** — `Server protocol and transfers` S9's
            // record carries the expected total, and a fetch arranged with
            // nothing would publish an empty file over the frame's name.
            //
            // **The size a walk already read is the answer: the complete
            // ending walk's, kept beside the series, or the last complete
            // walk's.** Only where neither read the key are the file's own
            // attributes read on the reading session at the moment the fetch
            // is arranged.
            //
            // **The case this was first written for, and the case it now also
            // meets.** The ending walk is taken outside the series (`Download`
            // W5 keeps the series for the change detection), and its sizes
            // used to go with it — so the case met was a job whose frames were
            // walked while it rendered. **The case met now is a job that had
            // already ended when the connection began**, where every unit came
            // from the ending walk and each paid a size read, one round trip
            // inside the server's point, for a number the walk had just read:
            // one of the nine round trips a frame cost on the eleventh pass.
            // Step 10c's eleventh-pass fix piece 2. **A nought a walk read is
            // read again on the machine**, and so is a symbolic link, whose
            // listed size is the link's own (`Download` keeps only plain
            // files from the ending walk).
            let the_walks_size = one
                .running
                .what_a_walk_read_of(&unit.key)
                .map(|entry| entry.size)
                .filter(|size| *size > 0);
            let the_job_output_folder = one.in_hand.the_job_output_folder.to_vec();
            self.next = (at + 1) % count;
            self.under_way += 1;
            return WhatTheChannelTook::AUnit {
                at,
                unit,
                the_walks_size,
                the_job_output_folder,
            };
        }
        if self.under_way > 0 {
            WhatTheChannelTook::NothingYet
        } else {
            WhatTheChannelTook::Nothing
        }
    }

    /// **A unit that never started goes back to *owed***, and its
    /// arrangement stops offering for this pass. R13: a reason to open fewer,
    /// never to fail.
    fn it_could_not_start(&mut self, at: usize, unit: AUnitOwed) {
        self.under_way -= 1;
        self.owing[at].queue.it_failed_and_may_come_back(unit);
        self.it_cannot_start[at] = true;
    }

    /// **A unit's fetch ended, and its arrangement's account follows it.**
    fn its_ending_is_applied(
        &mut self,
        at: usize,
        unit: AUnitOwed,
        came: WhatTheUnitsFetchCameTo,
        record: &dyn TheRecordADownloadWrites,
    ) {
        self.under_way -= 1;
        self.anything_moved = true;
        let it_was_fetched = matches!(
            &came,
            WhatTheUnitsFetchCameTo::ItEnded { ending, .. }
                if *ending == smoozy_server_protocol::transfers::Ending::Published
        );
        let one = &mut self.owing[at];
        let mut keeper = TheArrangementsKeeper { record };
        let Ok(followed) = one.running.a_fetch_ended(
            &mut keeper,
            one.queue,
            one.probing,
            one.arrangement,
            &in_hand_for(&one.in_hand),
            unit,
            came,
        ) else {
            // **The record would not take the ending**, so this
            // arrangement's account cannot move; the next pass re-derives
            // what is owed from the reckoning (`Download` R5).
            self.it_cannot_start[at] = true;
            return;
        };
        if followed.the_entry_now_stands_at.is_some() {
            // **The reckoning's entry is written**, so the row can move.
            record.a_transfer_moved();
        }
        if it_was_fetched {
            self.came_to.fetched += 1;
        }
        if let Some(act) = followed.the_diary_act {
            self.came_to.the_diary_acts.push(act);
        }
    }
}

impl<'o, 'a> ThePassUnderWay<'o, 'a> {
    fn holds(&self) -> MutexGuard<'_, WhatThePassHolds<'o, 'a>> {
        self.held
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }
}

/// **One of `Download` Q1's two channels, for one pass**: the next unit, to
/// its ending, then the next — until the pause is spent or nothing more can
/// be fetched. Added 2026-10-16, step 10c's owed piece 3.
async fn a_channel_of_the_pass<C: AHeldConnection>(
    pass: &ThePassUnderWay<'_, '_>,
    sessions: &ThisServersSessions<C>,
    the_run: &TheRunsTransfers,
    record: &dyn TheRecordADownloadWrites,
    the_run_this_is: &Identity,
) {
    loop {
        if pass.began.elapsed() >= pass.the_pause {
            return;
        }
        // **Listening before asking**, so an ending applied between the
        // asking and the waiting is not missed.
        let a_unit_ended = pass.a_unit_ended.notified();
        tokio::pin!(a_unit_ended);
        a_unit_ended.as_mut().enable();
        let took = pass.holds().the_next_unit(record);
        let (at, unit, the_walks_size, the_job_output_folder) = match took {
            WhatTheChannelTook::AUnit {
                at,
                unit,
                the_walks_size,
                the_job_output_folder,
            } => (at, unit, the_walks_size, the_job_output_folder),
            WhatTheChannelTook::NothingYet => {
                a_unit_ended.await;
                continue;
            }
            WhatTheChannelTook::Nothing => return,
        };
        let size = match the_walks_size {
            Some(size) => Some(size),
            None => the_size_of(sessions, &the_job_output_folder, &unit.key).await,
        };
        // **A fetch is never arranged against no size, and this is the
        // sharpest thing the real server found.** A total of nothing
        // publishes a zero-byte file at the frame's own name, the ending
        // reads *published*, and the reckoning writes *fetched*, which
        // `Download` R9 makes final. **So a unit whose size could not be
        // established stays owed.**
        let Some(size) = size else {
            pass.holds().it_could_not_start(at, unit);
            pass.a_unit_ended.notify_waiters();
            continue;
        };
        let arranged = {
            let held = pass.holds();
            let one = &held.owing[at];
            the_units_fetch_is_arranged(
                &*one.running,
                record,
                &*one.arrangement,
                &unit,
                &the_job_output_folder,
                size,
                the_run_this_is,
            )
        };
        let came = match arranged {
            TheFetchArranged::AlreadyOver(came) => came,
            TheFetchArranged::ToComeDown { file, transfer } => {
                the_units_bytes_come_down(sessions, the_run, record, file, transfer).await
            }
        };
        match came {
            // **Nothing was admitted and nothing was seated**, so the unit is
            // still owed and this arrangement stops offering for this pass.
            None => pass.holds().it_could_not_start(at, unit),
            Some(came) => pass.holds().its_ending_is_applied(at, unit, came, record),
        }
        pass.a_unit_ended.notify_waiters();
    }
}

/// **One file's size, read on the connection's own reading session.**
///
/// **It is a reading the nobody is waiting on**, like the walk, so a user's own
/// listing goes before it (`Server protocol and transfers` §1.1 item 2).
/// **Nothing where the reading did not answer**, and the caller then arranges
/// the fetch against no size rather than against an invented one.
async fn the_size_of<C: AHeldConnection>(
    sessions: &ThisServersSessions<C>,
    the_job_output_folder: &[u8],
    key: &[u8],
) -> Option<u64> {
    let mut path = the_job_output_folder.to_vec();
    if !path.ends_with(b"/") {
        path.push(b'/');
    }
    path.extend_from_slice(key);
    a_size_read_within(sessions, path, THE_INACTIVITY_DEADLINE).await
}

/// **One file's size on the reading session, inside `Server protocol and
/// transfers` C19's deadline** — [`the_size_of`]'s and
/// [`the_size_on_the_machine`]'s one reading.
///
/// **Bounded twice, and each bound is for a different waiter.** The attribute
/// request itself runs **inside the server's point**, which serves every
/// reading, command and seat on the connection one thing at a time — so a
/// request the far end never answers holds all of them, the walks and every
/// other fetch included. Bounded there, it gives the point back, and it answers
/// *the channel is gone* so the point lets that reading session go and the next
/// reading opens a fresh one (`Server protocol and transfers` §1.1 item 2's
/// re-open rule). **The fetch's own wait is bounded too**, since a reading
/// waits its turn behind others before it runs at all.
///
/// **The case this was first written for** was a reading session whose server
/// answers at once. **The case it now also meets** is one that does not
/// answer, where the wait had no end. Step 10c's transfer wave piece 3 (the
/// tenth-pass map's piece 1, item (iii)). **Nothing where the deadline
/// passed**: the unit stays owed and is offered again at the next pass
/// (R13), and no transfer record exists yet to be ended.
async fn a_size_read_within<C: AHeldConnection>(
    sessions: &ThisServersSessions<C>,
    path: Vec<u8>,
    deadline: Duration,
) -> Option<u64> {
    let asked = sessions.a_reading(false, move |subsystem| {
        Box::pin(async move {
            tokio::time::timeout(deadline, subsystem.stat(&path))
                .await
                .unwrap_or(Err(SubsystemFailure::TheChannelIsGone))
        })
    });
    tokio::time::timeout(deadline, asked)
        .await
        .ok()?
        .ok()?
        .ok()?
        .size
}

/// **One file's size, by its whole path on the machine, on the connection's own
/// reading session.**
///
/// **Nothing where the reading did not answer, and nothing where it answered
/// without a size** — `Server protocol and transfers`' own attributes structure
/// makes the field optional, so an absence is carried as an absence and the
/// caller leaves the unit owed rather than fetching against an invented total.
///
/// It is [`the_size_of`]'s reading for a path that is already whole, which a
/// manual request's enumeration holds and an arrangement's unit key does not.
async fn the_size_on_the_machine<C: AHeldConnection>(
    sessions: &ThisServersSessions<C>,
    path: &[u8],
) -> Option<u64> {
    a_size_read_within(sessions, path.to_vec(), THE_INACTIVITY_DEADLINE).await
}

/// **The transfer records standing for a server**, which the queue reads before
/// it offers so `Server protocol and transfers` T2's ceiling is counted against
/// what is really open.
///
/// **A record that will not read is no record**, and the queue then offers
/// against what it can see, which is the direction that fetches late rather than
/// the one that fetches twice.
///
/// **It asks the store for exactly what the question reads, in one query and
/// with no marks.** Changed 2026-09-28, step 10c's transfer wave piece 1.
/// **The case this was first written for** was a record holding a few
/// transfers, and it read every transfer record ever written — each with all
/// its checkpoints and superseded attempts — then kept this server's. **The
/// case it now also meets** is a 5,000-frame job, where that read ran before
/// every offer and grew with every frame fetched, to learn one thing:
/// whether any of this server's records has a publication pending
/// (`Download`'s `transfers_may_be_admitted`, unchanged). **So only this
/// server's records that have not ended or hold a publication pending are
/// read**, which are the only ones that can make that answer *no*.
fn the_transfer_records_standing(
    record: &dyn TheRecordADownloadWrites,
    server_row: &Identity,
) -> Vec<smoozy_server_protocol::transfers::TransferRecord> {
    let mut standing = Vec::new();
    record.with_the_record(&mut |connection| {
        standing = crate::record::transfer_records::the_unended_and_the_pending_without_their_marks(
            connection, server_row,
        )
        .unwrap_or_default();
    });
    standing
}

/// `WhatIsInHand` holds borrows, so each call takes its own copy of the same
/// borrows rather than the value being moved into the first one.
fn in_hand_for<'a>(in_hand: &WhatIsInHand<'a>) -> WhatIsInHand<'a> {
    WhatIsInHand {
        the_job_output_folder: in_hand.the_job_output_folder,
        narrowed_entries: in_hand.narrowed_entries,
        the_whole_may_grow: in_hand.the_whole_may_grow,
        the_entries_the_narrowing_excludes: in_hand.the_entries_the_narrowing_excludes,
        order: in_hand.order,
        the_jobs_label: in_hand.the_jobs_label,
        the_first_frame_pair: in_hand.the_first_frame_pair,
        the_pairs_other_half_fetched: in_hand.the_pairs_other_half_fetched,
        window: in_hand.window,
        the_placed_copies: in_hand.the_placed_copies,
        the_four_values_of_the_placed_copies: in_hand.the_four_values_of_the_placed_copies,
        arrangements_walking_this_server: in_hand.arrangements_walking_this_server,
    }
}

/// **What arranging one owed unit's fetch came to.** Split from the fetch
/// itself on 2026-10-16, step 10c's owed piece 3, so a pass's two channels
/// arrange under the pass's lock and move their bytes outside it.
enum TheFetchArranged {
    /// **It ended before anything was asked of the machine**: `None` where the
    /// record would not take the write, which leaves the unit owed.
    AlreadyOver(Option<WhatTheUnitsFetchCameTo>),
    /// **Its bytes are to come down**, under the transfer identity the
    /// reckoning was handed.
    ToComeDown {
        file: AFileToBringDown,
        transfer: Identity,
    },
}

/// **One owed unit's fetch arranged**: the last one given up where it was
/// interrupted, the transfer identity issued, the arrangement's account told,
/// and the folder it lands in made.
#[allow(clippy::too_many_arguments)]
fn the_units_fetch_is_arranged(
    running: &TheArrangementsRun,
    record: &dyn TheRecordADownloadWrites,
    arrangement: &AnArrangement,
    unit: &AUnitOwed,
    the_job_output_folder: &[u8],
    size: u64,
    the_run_this_is: &Identity,
) -> TheFetchArranged {
    // **`Server protocol and transfers` S28a: the fetch this unit's last one
    // was is given up before its replacement is arranged**, or its staging is
    // never swept. A record that would not take the write arranges nothing,
    // and the unit stays owed.
    if the_interrupted_fetch_is_given_up(record, arrangement, unit, the_run_this_is).is_err() {
        return TheFetchArranged::AlreadyOver(None);
    }
    let Ok(transfer) = Identity::issue(IdentityKind::Transfer) else {
        return TheFetchArranged::AlreadyOver(None);
    };
    let arranged = running.arrange_the_fetch_for(
        arrangement,
        unit,
        the_job_output_folder,
        transfer.clone(),
        None,
        size,
        the_run_this_is.clone(),
    );
    let arranged = match arranged {
        Ok(arranged) => arranged,
        // **A path that is not valid text is never mangled into the transfer
        // record**, and the refusal has a state to land in.
        Err(which) => {
            return TheFetchArranged::AlreadyOver(Some(
                WhatTheUnitsFetchCameTo::TheNameCouldNotBeCarried(which),
            ))
        }
    };

    // **P11: the structure the files had on the server is the structure they
    // have when they land** — so the folder the file lands in has to exist
    // before the publication's rename, and `Download` M9's own maker is the
    // manual half's. **An arrangement's landing folders had no maker anywhere,
    // and this is where that was found**: the publication rename lands on a
    // path whose parent is not there and the transfer ends *failed*, with the
    // frame still on the machine and nothing saying why.
    //
    // **It is `Desktop app and delivery` F10's one route to the filesystem**,
    // the same call M9's maker makes, and a folder that already stands merges
    // rather than colliding.
    let _ = the_folder_a_file_lands_in(&unit.the_landing_path);

    TheFetchArranged::ToComeDown {
        file: AFileToBringDown {
            on_the_machine: arranged.source_path.clone().into_bytes(),
            lands: WhereTheCallerSaysItLands::TheFolderTheCallerHolds {
                destination: unit.the_destination_normalised.clone(),
                landing: unit.the_landing_path.clone(),
            },
            size,
            arranged_by: WhatArrangedIt::AnAutoDownloadArrangement,
            server_row: arrangement.server_row.clone(),
            installation: None,
            run: the_run_this_is.clone(),
            transfer: Some(transfer.clone()),
        },
        transfer,
    }
}

/// **An arranged unit's bytes, through [`one_file_comes_down`].**
///
/// **`None` where the file never started** — nothing was admitted, or the
/// server's point seated nothing — which leaves the unit owed rather than ended.
async fn the_units_bytes_come_down<C: AHeldConnection>(
    sessions: &ThisServersSessions<C>,
    the_run: &TheRunsTransfers,
    record: &dyn TheRecordADownloadWrites,
    file: AFileToBringDown,
    transfer: Identity,
) -> Option<WhatTheUnitsFetchCameTo> {
    // **An auto-download's row is drawn in files and not in bytes**
    // (`Download` N8), so its byte loop's reports go to nobody.
    let came = one_file_comes_down_over(
        sessions,
        the_run,
        record,
        &mut NobodyIsWatching,
        file,
        &[the_class(WhatArrangedIt::AnAutoDownloadArrangement)],
    )
    .await
    .ok()?;

    Some(WhatTheUnitsFetchCameTo::ItEnded {
        transfer,
        ending: came.ending,
        // **`Desktop app and delivery` C15 and C19 are unbuilt in this crate**,
        // so nothing here can say what a taken name is taken by, beyond that it
        // is taken. **Where C10's exclusive create found it taken, it is taken
        // by nothing the record names** — the pair's other half's files are
        // taken before any fetch (`Download` E1 step 2's dated note) — and
        // `Download` P6 makes the entry *left alone*, so the file is not owed
        // again. `Download` P6's built note of 2026-10-31, the twenty-sixth
        // notes, note 4: until then this was absent, the transfer read
        // *failed*, and the same file crossed whole at every pass.
        taken_by: came
            .the_name_was_taken
            .then_some(smoozy_download::fetching::WhatTheTakenNameIs::NothingTheRecordNames),
        // **`Download` D1's write-failure classes are the adapter's** and are unbuilt
        // beside C15, so no destination condition is raised from here.
        the_write_failed: None,
    })
}

/// **`Server protocol and transfers` S28a, kept by the arrangement: a unit whose
/// last fetch ended *interrupted* gives that fetch up before it is fetched
/// again under a new transfer identity.** Added 2026-09-28, step 10c's
/// ninth-pass fix piece 6.
///
/// **What the ninth pass found.** A frame's download was cut off by
/// *Disconnect*; on the next *Connect* the arrangement resumed (`Download` R12)
/// and fetched the frame again, whole, as `Download` P3's new transfer. The
/// first transfer's record still said *interrupted*, and `Server protocol and
/// transfers` S28a keeps that ending off `Server protocol and transfers` S28's
/// list, since the work may be carried on. So its `…-1.smoozy-part` stayed in
/// the user's `.smoozy-staging` after all 240 files had landed and the app had
/// quit.
///
/// **The case this code was first written for** is the one `Download` P3
/// names: a fetch that *failed* or *stalled*, whose staging the byte loop has
/// already removed (`Server protocol and transfers` S26), so the new transfer
/// leaves nothing behind. **The case it now also meets** is a fetch ended
/// *interrupted*, whose staging `Server protocol and transfers` S26 keeps for
/// a carry-on that nothing here performs: `Server protocol and transfers` C11
/// step 4's resume is for a replaced connection alone, a *Connect* after a
/// *Disconnect* is a new period and not a replacement, and a record an earlier
/// run wrote is never resumed automatically (`Download` R12, M13). So
/// re-fetching **is** abandoning it, and `Server protocol and transfers` S28a
/// says what an abandonment owes.
///
/// **In that clause's order**: the record is written durably *stopped*, reason
/// *abandoned by its request* (`Server protocol and transfers` S26's second
/// row); **then** the staging is removed at once — a download's staging is on
/// the user's own computer, so no connection is needed for it — by `Server
/// protocol and transfers` S28's own judgement, the path and the stable file
/// identity both matching the record; **then** the caller arranges the
/// replacement. **A removal that was tried and not confirmed is an owed
/// cleanup** (`Server protocol and transfers` S27), written on the record. **A
/// record that would not take *stopped* is an `Err`**, and the caller arranges
/// nothing: *"the record is written before the replacement is arranged, or the
/// material is not swept."*
///
/// **Nothing is inferred** (`Server protocol and transfers` S29): the transfer
/// given up is the one the reckoning entry's own account names (`Download`
/// R4), never one found by looking in the staging folder, and only a download
/// of this server's that ended *interrupted*.
fn the_interrupted_fetch_is_given_up(
    record: &dyn TheRecordADownloadWrites,
    arrangement: &AnArrangement,
    unit: &AUnitOwed,
    the_run_this_is: &Identity,
) -> Result<(), Refused> {
    use smoozy_server_protocol::transfers::cleanup::{
        what_the_sweep_derives, what_the_sweep_does, WhatTheSweepDoes,
    };
    use smoozy_server_protocol::transfers::moving::TheUsersOwnComputer;
    use smoozy_server_protocol::transfers::StoppedReason;

    let Some(the_last_fetch) = arrangement
        .reckoning
        .iter()
        .find(|entry| entry.predicted_path == unit.key)
        .and_then(|entry| entry.account.transfer.clone())
    else {
        return Ok(());
    };
    let mut held = None;
    let there = record.with_the_record(&mut |connection| {
        held = crate::record::transfer_records::read(connection, &the_last_fetch)
            .ok()
            .flatten();
    });
    if !there {
        return Err(the_record_was_not_there());
    }
    let Some(mut given_up) = held else {
        return Ok(());
    };
    // **A fetch an earlier run left with no ending is given up the same way**
    // — its run died with it in flight (Smoozy quit mid-file), so no transfer
    // will ever finish it, and `Server protocol and transfers` C11 step 4
    // never resumes a record an earlier run wrote. Added 2026-10-31, the
    // twenty-sixth notes, note 4, *Also seen*: a frame cut by quitting was
    // fetched afresh after reopening, and its part stayed in `.smoozy-staging`
    // for good. **One of this run's own is never taken**: it may still be
    // moving.
    // A record still waiting on its publication is the next start's (C13),
    // not this.
    let an_earlier_run_left_it_unended = given_up.ending.is_none()
        && given_up.publication_pending.is_none()
        && given_up.written_by_run != *the_run_this_is;
    if given_up.direction != Direction::Down
        || given_up.server != arrangement.server_row
        || !(given_up.ending == Some(Ending::Interrupted) || an_earlier_run_left_it_unended)
    {
        return Ok(());
    }

    // **Durably *stopped*, and before anything else.**
    given_up.ending = Some(Ending::Stopped(StoppedReason::AbandonedByItsRequest));
    let mut keeper = TheTransferRecordsOverTheDoor { record };
    keeper
        .write_it_down(&given_up)
        .map_err(|failure| Refused::TheStore(failure.0))?;

    // **Removed at once, by `Server protocol and transfers` S28's judgement
    // over this one record.**
    let mut adapter = ThePlatformFileAdapter;
    let mut owed = false;
    for derived in what_the_sweep_derives(std::slice::from_ref(&given_up)) {
        let at = derived.path.as_text();
        let identity_now = Path::new(&at)
            .exists()
            .then(|| adapter.stable_identity_of(&at));
        if what_the_sweep_does(&derived, identity_now.as_ref()) == WhatTheSweepDoes::Remove
            && adapter.remove(&at).is_err()
        {
            owed = true;
        }
    }
    if owed {
        given_up.cleanup_is_owed = true;
        let _ = keeper.write_it_down(&given_up);
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// A manual download, by hand and on its own.
// ---------------------------------------------------------------------------

/// **What the user picked, and where it goes** — `Download` M1's two, as the
/// screen hands them over.
#[derive(Debug, Clone)]
pub struct WhatIsBeingDownloaded {
    pub server_row: Identity,
    pub installation: Option<Identity>,
    /// **The destination — the standing folder, or the per-request override the
    /// spec grants.**
    pub destination: Vec<u8>,
    /// The whole paths on the machine the user picked, as the listing gave
    /// them.
    pub picked: Vec<Vec<u8>>,
    pub run: Identity,
}

/// **Why a manual download did not start.** None of these moved a byte.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhyTheDownloadDidNotStart {
    /// Nothing was picked.
    ThereIsNothingToFetch,
    /// **There is nowhere to put it** — W5's *the system names no folder*, or a
    /// settings read that did not answer.
    ThereIsNowhereToPutIt(WhereItLands),
    /// **The server's point lent nothing** — no connection is live, or it
    /// ended, or the machine allows no session.
    TheServerLentNothing(NotLent),
    /// **`Download` M4: the enumeration is not complete**, because nothing at
    /// all could be read of what the user picked. A request written from part
    /// of one would have a count that is wrong in the quiet direction.
    NothingPickedCouldBeRead,
    /// **The local record would not take the request**, so M1's durable half
    /// could not be written and nothing may move.
    TheRecordWouldNotTakeIt(Refused),
    /// **The user pressed *Stop* on the counting row** — nothing was fetched,
    /// nothing written, nothing made on their disk. Step 10c's twelfth-pass
    /// fix piece 3.
    TheUserStoppedTheCount,
}

/// **A manual request whose record is written and whose enumeration is done** —
/// everything up to `Download` M6's question.
#[derive(Debug, Clone)]
pub struct ADownloadReadyToRun {
    pub request: AManualRequest,
    /// **M6's own reading**: which of the request's landing paths are already
    /// standing, with the staging directory and every reservation-record path
    /// subtracted first (`Download` R6).
    pub colliding: Vec<Vec<u8>>,
    /// The names standing beside them, which M8's counted suffix is composed
    /// against.
    pub the_names_already_standing: Vec<Vec<u8>>,
    /// **M6's three, counted**: a collision to answer, a cannot-vouch mark to
    /// name, a refusal to say. **A request that finds none of the three starts
    /// without a question**, and no empty surface is composed for it.
    pub the_pre_flight: WhatThePreFlightHasToSay,
    /// **`Download` M9's landing folders, as the enumeration named them** —
    /// kept so that *Keep both*, which renames the arriving folder after they
    /// were made (M8), makes them again under the name it chose. Step 10c's
    /// Vast-pass fix piece 1.
    pub landing_folders: Vec<Vec<u8>>,
}

/// **`Download` M1 and M2, performed: the request enumerated on the machine and
/// written down before any byte moves.**
///
/// **The enumeration is a reading over the connection's reading session**, and
/// it is a reading the user *is* waiting on — they pressed Download and are
/// looking at the pre-flight — which is `Server protocol and transfers` §1.1
/// item 2's ordering.
///
/// **No arrangement is touched anywhere in here.** `Download` M1's request is
/// its own route: no reckoning is read, no window is opened, no walk series is
/// started, and nothing of `Download` R1 appears. A test below reads this file's
/// own text and fails if one ever does.
pub async fn a_download_is_prepared<C: AHeldConnection, J>(
    asked: WhatIsBeingDownloaded,
    sessions: &ThisServersSessions<C>,
    record: &dyn TheRecordADownloadWrites,
    records: &J,
) -> Result<ADownloadReadyToRun, WhyTheDownloadDidNotStart>
where
    J: ReadsTheLocalJobRecords,
{
    if let Some(why) = what_refuses_before_anything_is_read(&asked, sessions) {
        return Err(why);
    }
    let mut reader = TheWalksReadingRoute::on(sessions);
    a_download_is_prepared_over(
        asked,
        None,
        &mut reader,
        record,
        records,
        &mut TheLandingFolders,
        &mut TheDestinationsListing,
    )
    .await
}

/// **Where the count is shown on Transfers while it runs** — the run holding
/// it, the request's own identity, and what the row names. Step 10c's
/// twelfth-pass fix piece 3.
///
/// **The identity is the caller's**, so the row that reads *Counting files…*
/// and the request written after it are one row on the page, and so the
/// caller knows which request a *Stop* it took was about.
pub struct TheCountIsShown<'a> {
    pub the_run: &'a TheRunsTransfers,
    pub request: Identity,
    pub named: String,
}

/// **[`a_download_is_prepared`], on Transfers from the press** —
/// `Design/Transfers`' paragraph of 2026-10-01: *"A download of a folder is on
/// Transfers from the moment it is asked for. Until Smoozy knows what is in
/// the folder, its row names the folder and reads Counting files…, with
/// Stop."* Step 10c's twelfth-pass fix piece 3.
///
/// **The case [`a_download_is_prepared`] was first written for** is a request
/// of a few files, whose walk ends before anybody could look. **The case it
/// now also meets** is a folder of a thousand files in forty-five folders,
/// whose walk is four round trips a folder, one after another (28 s at a
/// round trip of 0.15 s at the desk; three minutes on the twelfth pass), and
/// whose request is written only when the walk ends (`Download` M1). **Until
/// then the page had nothing to draw.** The count is now held for the run
/// from the press ([`TheRunsTransfers::a_count_begins`]), with *Stop*, and
/// `Server protocol and transfers` S0a saves the start on an empty parent before the count. A *Stop* removes
/// that parent with nothing fetched or made at the destination
/// ([`WhyTheDownloadDidNotStart::TheUserStoppedTheCount`]).
pub async fn a_download_is_counted_and_prepared<C: AHeldConnection, J>(
    asked: WhatIsBeingDownloaded,
    shown: TheCountIsShown<'_>,
    sessions: &ThisServersSessions<C>,
    record: &dyn TheRecordADownloadWrites,
    records: &J,
) -> Result<ADownloadReadyToRun, WhyTheDownloadDidNotStart>
where
    J: ReadsTheLocalJobRecords,
{
    if let Some(why) = what_refuses_before_anything_is_read(&asked, sessions) {
        return Err(why);
    }
    let mut reader = TheWalksReadingRoute::on(sessions);
    a_download_is_prepared_over(
        asked,
        Some(shown),
        &mut reader,
        record,
        records,
        &mut TheLandingFolders,
        &mut TheDestinationsListing,
    )
    .await
}

// `Server protocol and transfers` S0a's counting parent is saved before the first walk. A refused/stopped
// count removes it; an interrupted application leaves an honest ended request.
struct TheCountingParent<'a> {
    record: &'a dyn TheRecordADownloadWrites,
    request: Identity,
    kept: bool,
}
impl Drop for TheCountingParent<'_> {
    fn drop(&mut self) {
        if !self.kept {
            let _ = TheRequestsKeeper { record: self.record }.the_user_removed_it(&self.request);
            self.record.a_transfer_moved();
        }
    }
}

/// **What refuses a manual download before anything is read** — nothing
/// picked, nowhere to put it, no connection lent.
fn what_refuses_before_anything_is_read<C: AHeldConnection>(
    asked: &WhatIsBeingDownloaded,
    sessions: &ThisServersSessions<C>,
) -> Option<WhyTheDownloadDidNotStart> {
    if asked.picked.is_empty() {
        return Some(WhyTheDownloadDidNotStart::ThereIsNothingToFetch);
    }
    if asked.destination.is_empty() {
        return Some(WhyTheDownloadDidNotStart::ThereIsNowhereToPutIt(
            WhereItLands::TheSystemNamesNoFolder(
                "the request named no folder to land in".into(),
            ),
        ));
    }
    if !sessions.a_connection_is_live() {
        return Some(WhyTheDownloadDidNotStart::TheServerLentNothing(
            NotLent::NoConnectionIsLive,
        ));
    }
    None
}

/// **[`a_download_is_prepared`] over a machine reading and a destination
/// handed in**, so a test can stand where the machine and the user's disk
/// stand — the destination's first read held, as the system's permission box
/// holds it. Step 10c's twelfth-pass fix piece 3.
pub(crate) async fn a_download_is_prepared_over<R, J, M, L>(
    asked: WhatIsBeingDownloaded,
    shown: Option<TheCountIsShown<'_>>,
    reader: &mut R,
    record: &dyn TheRecordADownloadWrites,
    records: &J,
    maker: &mut M,
    listing: &mut L,
) -> Result<ADownloadReadyToRun, WhyTheDownloadDidNotStart>
where
    R: ReadsOneDirectoryOnTheMachine + Send,
    J: ReadsTheLocalJobRecords,
    M: MakesTheLandingFolder + ?Sized,
    L: ReadsTheDestinationsListing + ?Sized,
{
    let identity = match &shown {
        Some(shown) => shown.request.clone(),
        None => Identity::issue(IdentityKind::Operation)
            .map_err(|_| WhyTheDownloadDidNotStart::ThereIsNothingToFetch)?,
    };
    let mut counting_parent = if shown.is_some() {
        let parent = AManualRequest::confirmed(identity.clone(), asked.server_row.clone(),
            asked.installation.clone(), asked.destination.clone(), Vec::new(), asked.run.clone());
        let mut saved = Err(the_record_was_not_there());
        record.with_the_record(&mut |connection| {
            saved = DownloadRequests::over(connection).begin_counting(&parent, &shown.as_ref().expect("the count is shown").named);
        });
        saved.map_err(WhyTheDownloadDidNotStart::TheRecordWouldNotTakeIt)?;
        Some(TheCountingParent { record, request: identity.clone(), kept: false })
    } else { None };
    // **On Transfers from the press** (`Design/Transfers`' paragraph of
    // 2026-10-01), where the caller shows it: held for the run until the
    // request is written, and gone — the page told — on every way out.
    let counting = shown.map(|shown| {
        shown.the_run.a_count_begins(
            &identity,
            &asked.server_row,
            shown.named,
            asked.destination.clone(),
            record,
        )
    });
    let rules = TheReceivingSystemsRulesHere;
    let walking = enumerate(
        reader,
        &rules,
        records,
        &asked.picked,
        &asked.destination,
    );
    let enumeration = match &counting {
        // **The row's *Stop* ends the walk where it stands**, with nothing
        // fetched and nothing written: the count is let go of, and the page
        // is told.
        Some(count) => match the_stop_lands_or(walking, &count.stopped).await {
            Some(enumeration) => enumeration,
            None => return Err(WhyTheDownloadDidNotStart::TheUserStoppedTheCount),
        },
        None => walking.await,
    };
    if counting.as_ref().is_some_and(TheCountIsUnderWay::the_user_stopped_it) {
        return Err(WhyTheDownloadDidNotStart::TheUserStoppedTheCount);
    }
    // **A branch the machine did not answer for is the connection's case and
    // never a recorded refusal** — `File Browser` P1's third answer, *"not a
    // statement about the folder at all"*. Added 2026-10-19, the dead-line
    // desk coverage, first half: a line that went silent while the picked
    // things were read ended every branch that way, and the download said
    // *Smoozy could not read what you chose on the server*; cut part-way, the
    // request would be written from part of the enumeration, its count wrong
    // in the quiet direction (M4). **So nothing is written, and the download
    // does not start for the reason it did not**: the server is not
    // connected.
    if enumeration.refusals.iter().any(|refusal| {
        matches!(
            refusal.what_smoozy_met,
            smoozy_file_browser::permission::WhatSmoozyMet::TheMachineDidNotAnswer
        )
    }) {
        return Err(WhyTheDownloadDidNotStart::TheServerLentNothing(
            NotLent::TheConnectionEnded,
        ));
    }
    // **M4: a branch that ends in a recorded refusal keeps the enumeration
    // complete**, so a request with some files and some refused subtrees goes
    // ahead. **A request with neither is a reading that answered nothing**, and
    // nothing is written for it.
    //
    // **Nor is one that met only what it could not read** — no file, and no
    // folder to make (M9). It has nothing to download, so M6 has nothing to
    // ask it: the refusal is said as the download not starting. **The case
    // this was first written for** was a reading that answered nothing at
    // all. **The case it now also meets** is the Vast pass's (step 10c's
    // Vast-pass fix piece 1): one file it could not read put up the collision
    // question with *0 files are already there*, and its row read *done*.
    //
    // **An empty folder picked alone is neither**: it was read, and it holds
    // nothing, so it goes ahead and arrives as an empty folder (M9). It used
    // to be refused with the sentence that Smoozy could not read it, which is
    // false (step 10c's Vast-pass fix piece 1b). So the test is *no file and
    // no folder to make*, whatever was refused.
    if enumeration.entries.is_empty() && enumeration.landing_folders.is_empty() {
        return Err(WhyTheDownloadDidNotStart::NothingPickedCouldBeRead);
    }

    let mut request = AManualRequest::confirmed(
        identity,
        asked.server_row.clone(),
        asked.installation.clone(),
        asked.destination.clone(),
        enumeration.entries.clone(),
        asked.run.clone(),
    );
    request.subtrees_that_could_not_be_read = enumeration.refusals.clone();

    // **M1: the enumeration is durable before the collision question is asked
    // and before the first byte moves.**
    let mut keeper = TheRequestsKeeper { record };
    keeper
        .write_it_down(&request)
        .map_err(WhyTheDownloadDidNotStart::TheRecordWouldNotTakeIt)?;
    if let Some(parent) = &mut counting_parent { parent.kept = true; }
    // **The page is told the request is written**, so a Transfers page
    // already on screen draws it under way before the first read of the
    // destination — which is where the system's permission box stands on a
    // fresh computer — and before `Download` M6's question. Step 10c's
    // twelfth-pass fix piece 3, the twelfth pass's fault 4: the request was
    // written at 19:23:48, the page asks again only when told, nothing told
    // it, and it read *Nothing is uploading or downloading* for as long as
    // the box stood. **The count going tells it**, where there was one, so
    // the counting row and the request are never both drawn.
    match counting {
        Some(count) => drop(count),
        None => record.a_transfer_moved(),
    }

    // **M9's landing folders**, made plainly through the platform file adapter,
    // because *a downloaded folder arrives as a folder* and a folder with no
    // files in it has no transfer to make one for it.
    let _ = make_the_landing_folders(maker, &enumeration);

    // **M6's already-exists comparison, by name and place only.**
    let the_names_already_standing = the_names_standing_beside(listing, &request);
    let colliding = the_colliding_landing_paths(listing, &request);
    let the_pre_flight = what_the_pre_flight_has_to_say(&request, &colliding);

    Ok(ADownloadReadyToRun {
        request,
        colliding,
        the_names_already_standing,
        the_pre_flight,
        landing_folders: enumeration.landing_folders,
    })
}

/// **M6's four answers, applied to the enumeration once and written down at the
/// moment they are given.**
///
/// **The answer is never re-asked for the life of the request**, which is what
/// its durability here makes true by construction.
pub fn the_answer_is_applied(
    ready: &mut ADownloadReadyToRun,
    record: &dyn TheRecordADownloadWrites,
    answer: CollisionAnswer,
) -> Result<(), Refused> {
    let rules = TheReceivingSystemsRulesHere;
    let landing_before: Vec<Vec<u8>> = ready
        .request
        .entries
        .iter()
        .map(|entry| entry.landing_path.clone())
        .collect();
    apply_the_collision_answer(
        &rules,
        &mut ready.request,
        answer,
        &ready.colliding,
        &ready.the_names_already_standing,
    );
    TheRequestsKeeper { record }.write_it_down(&ready.request)?;
    // **M8 and M9: *Keep both* renamed the arriving folder, so its landing
    // folders are made under the name it chose.** They were made before the
    // question, under the old name, and nothing made them again: on the Vast
    // pass every frame of a job's second *Download all* failed, because
    // `… pv long2 (2)/frames/` was never made (step 10c's Vast-pass fix piece
    // 1). *Replace* and *Skip existing* land in the folders already made.
    let _ = make_the_landing_folders(
        &mut TheLandingFolders,
        &smoozy_download::acting::enumerating::TheEnumeration {
            landing_folders: the_landing_folders_keep_both_moved(
                &ready.landing_folders,
                &landing_before,
                &ready.request,
            ),
            ..Default::default()
        },
    );
    Ok(())
}

/// **`Download` M8's renamed top-level things, carried to M9's landing
/// folders**: each landing folder under a top-level thing *Keep both* renamed,
/// under its new name. **Read off the entries the answer moved**, so nothing
/// here composes a name. Step 10c's Vast-pass fix piece 1.
fn the_landing_folders_keep_both_moved(
    landing_folders: &[Vec<u8>],
    landing_before: &[Vec<u8>],
    request: &AManualRequest,
) -> Vec<Vec<u8>> {
    let destination = request.destination.as_slice();
    let mut renamed: Vec<(&[u8], &[u8])> = Vec::new();
    for (before, entry) in landing_before.iter().zip(&request.entries) {
        if *before == entry.landing_path {
            continue;
        }
        let (Some((was, _)), Some((now, _))) = (
            the_top_under(before, destination),
            the_top_under(&entry.landing_path, destination),
        ) else {
            continue;
        };
        if !renamed.iter().any(|(already, _)| *already == was) {
            renamed.push((was, now));
        }
    }
    landing_folders
        .iter()
        .filter_map(|folder| {
            let (top, rest) = the_top_under(folder, destination)?;
            let (_, now) = renamed.iter().find(|(was, _)| *was == top)?;
            let mut moved = destination.to_vec();
            if !moved.ends_with(b"/") {
                moved.push(b'/');
            }
            moved.extend_from_slice(now);
            moved.extend_from_slice(rest);
            Some(moved)
        })
        .collect()
}

/// **The component a path puts directly under the destination, and what
/// follows it** (its leading separator kept), by components and never by a
/// byte prefix.
fn the_top_under<'p>(path: &'p [u8], destination: &[u8]) -> Option<(&'p [u8], &'p [u8])> {
    let under = path.strip_prefix(destination)?;
    let under = if destination.ends_with(b"/") {
        under
    } else {
        under.strip_prefix(b"/".as_slice())?
    };
    let end = under.iter().position(|byte| *byte == b'/').unwrap_or(under.len());
    (end > 0).then(|| (&under[..end], &under[end..]))
}

/// **A request that asks nothing takes its dispositions without a question** —
/// M6's narrowing of 2026-08-29, which a scenario replay found by playing a
/// clean whole-job download through a confirmation with nothing in it.
pub fn the_request_goes_without_a_question(
    ready: &mut ADownloadReadyToRun,
    record: &dyn TheRecordADownloadWrites,
) -> Result<(), Refused> {
    for entry in &mut ready.request.entries {
        entry.disposition = Some(match entry.refusal {
            Some(which) => Disposition::CannotLand(which),
            None => Disposition::ToFetch,
        });
    }
    TheRequestsKeeper { record }.write_it_down(&ready.request)
}

/// **What the manual download came to, counted the way `Download` M16 counts
/// it.**
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct WhatTheDownloadCameToInTheEnd {
    /// Files at their final names.
    pub fetched: usize,
    /// **Skipped units are in neither total and are reported separately.**
    pub skipped: usize,
    /// **Units the name check refused**, which is `Download` P8's per-file
    /// refusal.
    pub cannot_land: usize,
    /// Units that ended as failures, by their landing paths.
    pub failed: Vec<Vec<u8>>,
    /// **Units whose transfer ended interrupted or stalled.** Their records
    /// stand; nothing about them failed.
    pub did_not_finish: usize,
}

/// **`Download` M1's request run to its units' endings** — the manual route,
/// whole, with no arrangement anywhere in it.
///
/// **P2's order inside a request is the enumeration's own** (`File Browser`
/// O7's shape), and it is the order the files start in.
///
/// **Up to `Download` Q1's two files at once**, each channel taking the next
/// when its own ends ([`every_one_of`]) — step 10c's eleventh-pass fix piece
/// 4. **The case this was first written for** is a request of a few files, one
/// after another. **The case it now also meets** is a *Download all* of a
/// thousand small files at a round trip of 0.2 s, where one file at a time
/// spent 22.5 minutes and Q1's two had been there all along. **What then lands
/// out of order, and why it is harmless**: a small file that starts after a
/// large one may finish before it. Each file lands at its own name through its
/// own publication, and `Download` M16 counts outcomes, not their order, so
/// nothing reads the order a file landed in.
///
/// **R17 again, the same way:** the desktop point's slot and the server's seat
/// are both asked for per file and given back at that file's end.
pub async fn the_download_runs<C: AHeldConnection>(
    ready: &mut ADownloadReadyToRun,
    sessions: &ThisServersSessions<C>,
    the_run: &TheRunsTransfers,
    record: &dyn TheRecordADownloadWrites,
) -> WhatTheDownloadCameToInTheEnd {
    // **The request is under way while this runs, and not a moment longer**
    // — step 10c's transfer wave piece 5, `Download` M13's paragraph of
    // 2026-09-28 read with `File Browser` U7. Whatever ends this run — every
    // unit done, a period's end that leaves each remaining file unable to
    // start, or an unwinding — the register lets it go, and the row then reads
    // what the record says of it, never *running*.
    let under_way = the_run.a_request_goes_under_way(
        &ready.request.record,
        &ready.request.server_row,
        sessions,
    );
    let request_id = ready.request.record.as_text();

    // **`Download` M13's third bullet: what an earlier request left, which this
    // one may take up** — read once as the run begins. Step 10c's transfer
    // wave piece 7.
    let mut the_parts_left: Vec<TransferRecord> = Vec::new();
    record.with_the_record(&mut |connection| {
        the_parts_left = the_parts_a_new_request_may_take_up(
            connection,
            the_run,
            &ready.request.server_row,
            ready.request.installation.as_ref(),
            Direction::Down,
            &ready.request.record,
        );
    });

    // **`Download` Q1's two, filled** — step 10c's eleventh-pass fix piece 4.
    // See [`every_one_of`]: each worker takes the next file of the
    // enumeration when its own ends, and each file is admitted in its own turn.
    let how_many = ready.request.entries.len();
    let running = TheManualRequestRunning {
        request: Mutex::new(&mut ready.request),
        the_parts_left: Mutex::new(the_parts_left),
        next: TheNextFile::default(),
        came_to: Mutex::new(Vec::new()),
    };
    let workers = (0..AT_ONCE)
        .map(|_| {
            a_channel_of_the_manual_request(
                &running,
                how_many,
                sessions,
                the_run,
                record,
                &request_id,
            )
        })
        .collect();
    every_one_of(workers).await;

    // **The run is over, so the request is no longer under way**, and the
    // page is told so it stops drawing it as moving. **Told as a move and not
    // as a unit's ending**, since no outcome was written here.
    drop(under_way);
    record.a_transfer_is_moving();
    running.what_it_came_to()
}

/// **A manual request while its files run**, shared by its workers. Added
/// 2026-09-30, step 10c's eleventh-pass fix piece 4.
///
/// **The request is behind a lock that is never held across a wait**, and
/// never taken before the record's: a worker that writes an entry takes the
/// record, then the request inside it, so two workers cannot hold one each and
/// wait on the other. **Each entry is moved only by the worker that took it**,
/// and the request's own row — its identity, server, installation, destination
/// and answer — does not move while it runs, so an entry written from the one
/// request carries nothing another worker wrote.
struct TheManualRequestRunning<'r> {
    request: Mutex<&'r mut AManualRequest>,
    the_parts_left: Mutex<Vec<TransferRecord>>,
    next: TheNextFile,
    /// What each entry came to, by its place in the enumeration.
    came_to: Mutex<Vec<(usize, WhatOneUnitCameTo)>>,
}

/// **What one unit of a manual request came to in this run.**
#[derive(Debug, Clone, PartialEq, Eq)]
enum WhatOneUnitCameTo {
    Fetched,
    Skipped,
    CannotLand,
    Failed(Vec<u8>),
    DidNotFinish,
    /// Nothing of it was this run's: no disposition, or already ended.
    Nothing,
}

impl<'r> TheManualRequestRunning<'r> {
    fn the_request(&self) -> MutexGuard<'_, &'r mut AManualRequest> {
        self.request
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    fn it_came_to(&self, at: usize, what: WhatOneUnitCameTo) {
        self.came_to
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .push((at, what));
    }

    /// **Counted the way `Download` M16 counts it**, and the files that failed
    /// named in the enumeration's order, whatever order they ended in.
    fn what_it_came_to(&self) -> WhatTheDownloadCameToInTheEnd {
        let mut each = std::mem::take(
            &mut *self
                .came_to
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner()),
        );
        each.sort_by_key(|(at, _)| *at);
        let mut came_to = WhatTheDownloadCameToInTheEnd::default();
        for (_, what) in each {
            match what {
                WhatOneUnitCameTo::Fetched => came_to.fetched += 1,
                WhatOneUnitCameTo::Skipped => came_to.skipped += 1,
                WhatOneUnitCameTo::CannotLand => came_to.cannot_land += 1,
                WhatOneUnitCameTo::Failed(landing) => came_to.failed.push(landing),
                WhatOneUnitCameTo::DidNotFinish => came_to.did_not_finish += 1,
                WhatOneUnitCameTo::Nothing => {}
            }
        }
        came_to
    }
}

/// **One of `Download` Q1's channels, for one manual request**: the next file,
/// to its ending, then the next.
async fn a_channel_of_the_manual_request<C: AHeldConnection>(
    running: &TheManualRequestRunning<'_>,
    how_many: usize,
    sessions: &ThisServersSessions<C>,
    the_run: &TheRunsTransfers,
    record: &dyn TheRecordADownloadWrites,
    request_id: &str,
) {
    while let Some(at) = running.next.of(how_many) {
        let what =
            one_unit_of_the_manual_request(running, at, sessions, the_run, record, request_id)
                .await;
        running.it_came_to(at, what);
    }
}

/// **One unit of `Download` M1's request, from its disposition to its
/// ending** — what the manual route did for each entry in turn before step 10c's
/// eleventh-pass fix piece 4, now done for whichever entry a channel takes.
async fn one_unit_of_the_manual_request<C: AHeldConnection>(
    running: &TheManualRequestRunning<'_>,
    at: usize,
    sessions: &ThisServersSessions<C>,
    the_run: &TheRunsTransfers,
    record: &dyn TheRecordADownloadWrites,
    request_id: &str,
) -> WhatOneUnitCameTo {
    let class = the_class(WhatArrangedIt::AManualRequest);
    let tell = || record.a_transfer_is_moving();
    let the_door = |act: &mut dyn FnMut(&rusqlite::Connection)| record.with_the_record(act);
    let entry = running.the_request().entries[at].clone();

    // **The user's Stop on the request: unarranged files are never arranged**
    // (`Download` M12, `File Browser` U6). Step 10c's transfer wave piece 7.
    if the_run.the_user_stopped_it(request_id) {
        let owed = entry.outcome.is_none()
            && entry
                .disposition
                .as_ref()
                .is_some_and(Disposition::a_transfer_is_arranged);
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
        return WhatOneUnitCameTo::Nothing;
    }
    // **The part an earlier request left at this file's name, where one
    // is**: `Download` M13's match is the machine path and the landing
    // path, and the new request's disposition decides.
    let the_part_here = {
        let mut the_parts_left = running
            .the_parts_left
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        the_parts_left
            .iter()
            .position(|part| {
                part.source_path.as_bytes() == entry.source_path.as_slice()
                    && part.final_path.as_bytes() == entry.landing_path.as_slice()
            })
            .map(|found| the_parts_left.remove(found))
    };
    match &disposition {
        Disposition::SkipExisting => {
            // **`Server protocol and transfers` S28a: skip existing does
            // not take the part up**, and the record says so before
            // anything else, or no sweep ever reaches it.
            if let Some(part) = &the_part_here {
                let _ = a_part_is_abandoned(&the_door, the_run, part);
            }
            return WhatOneUnitCameTo::Skipped;
        }
        Disposition::CannotLand(_) => return WhatOneUnitCameTo::CannotLand,
        _ => {}
    }
    // **One file the user stopped by itself, before it started** — it is
    // never arranged, and the rest of the request goes on (`Download`
    // M12's single-transfer stop, `Design/Transfers § Stopping one
    // file`).
    if the_run.the_user_stopped_this(request_id, &entry.landing_path) {
        return WhatOneUnitCameTo::DidNotFinish;
    }
    let taken_up = the_part_here.filter(|part| {
        smoozy_download::request::may_take_up_the_staged_material(
            part,
            &entry.source_path,
            &entry.landing_path,
            &disposition,
        )
    });

    // **Piece 10's sharpest finding, kept on this route as well.**
    // `Server protocol and transfers` S9's record carries the expected
    // total, and its loop treats a total of nothing as a transfer with
    // nothing left to do: it **publishes**, so an empty file lands at the
    // file's own name, the ending reads *published*, and `Download` R9
    // makes the unit outcome final. On this route the name it lands at is a
    // file in the user's own folder, and an answer of *Replace* has already
    // taken what was there.
    //
    // **The walk's size is the ordinary answer.** What the walk cannot do
    // is tell *a file of nought bytes* from *the listing carried no size*:
    // the protocol makes the field optional, the reading route reads an
    // absent one as nought, and `Download` M1's record holds one number.
    // **So a unit the walk sized at nought is confirmed on the reading
    // session before any byte moves**, and one whose size cannot be
    // established **stays owed** — no transfer identity, no disposition
    // spent, nothing published. A file that really is nought bytes answers
    // nought and goes.
    let size = match entry.size_as_enumerated {
        0 => the_size_on_the_machine(sessions, &entry.source_path).await,
        size => Some(size),
    };
    let Some(size) = size else {
        return WhatOneUnitCameTo::DidNotFinish;
    };

    // **A taken-up unit carries the part's own transfer identity**
    // (`Download` M13's third bullet); every other unit a new one.
    let transfer = match &taken_up {
        Some(part) => part.transfer.clone(),
        None => match Identity::issue(IdentityKind::Transfer) {
            Ok(transfer) => transfer,
            Err(_) => return WhatOneUnitCameTo::Failed(entry.landing_path.clone()),
        },
    };
    if taken_up.is_some() {
        the_run.every_leftover_is_read_again();
    }
    // **`Server protocol and transfers` G4's report reaches the row**
    // (`Download` M16's bar in bytes), with the next files waiting their
    // turn behind this one — **once the desktop point has admitted the file**
    // ([`a_requested_file_comes_down`]; step 10c's eleventh-pass fix piece 4).
    let (waiting, destination, server_row, installation, run) = {
        let mut request = running.the_request();
        request.entries[at].transfer = Some(transfer.clone());
        let waiting = the_next_waiting(&request.entries, at, |entry| {
            (entry.outcome.is_none()
                && entry
                    .disposition
                    .as_ref()
                    .is_some_and(Disposition::a_transfer_is_arranged))
            .then(|| entry.landing_path.clone())
        });
        (
            waiting,
            request.destination.clone(),
            request.server_row.clone(),
            request.installation.clone(),
            request.opened_by_run.clone(),
        )
    };
    the_entry_is_written_down(record, &running.request, at);
    let mut watcher = TheRowIsTold {
        the_run,
        request: request_id.to_string(),
        entry: at,
        tell: &tell,
        started_over: None,
    };
    // **Carried on across a repair** — `Download` M11, step 10c's
    // transfer wave piece 6: a file the line cuts waits for the
    // connection and carries on with the same record, and a file that has
    // not yet started waits its turn, for as long as the period stands.
    let came = a_requested_file_comes_down(
        sessions,
        the_run,
        record,
        &mut watcher,
        (request_id, at, &tell),
        AFileToBringDown {
            on_the_machine: entry.source_path.clone(),
            lands: WhereTheCallerSaysItLands::TheFolderTheCallerHolds {
                destination,
                landing: entry.landing_path.clone(),
            },
            size,
            arranged_by: WhatArrangedIt::AManualRequest,
            server_row,
            installation,
            run,
            transfer: Some(transfer),
        },
        &[class],
        taken_up,
        waiting,
        // **`Download` P5 and `Desktop app and delivery` C14: the entry's own
        // disposition decides the publication's intent**, and only *Replace*
        // replaces. Step 10c's thirteenth-pass fix piece 3.
        the_intent(disposition == Disposition::Replace),
    )
    .await;

    let Ok(came) = came else {
        the_run.a_file_stopped(request_id, at);
        return WhatOneUnitCameTo::DidNotFinish;
    };
    // **The user's own Stop is not a failure**: the unit is left owed and
    // nothing is written for it (`File Browser` U6, `Download` M12). Step
    // 10c's transfer wave piece 7.
    if the_user_stopped_this_ending(the_run, request_id, &entry.landing_path, &came.ending) {
        the_run.a_file_stopped(request_id, at);
        record.a_transfer_moved();
        return WhatOneUnitCameTo::DidNotFinish;
    }
    let (outcome, what) = match came.ending {
        smoozy_server_protocol::transfers::Ending::Published => {
            (UnitOutcome::Done, WhatOneUnitCameTo::Fetched)
        }
        // **Smoozy's own stop, and a digest that disagreed, end the unit as
        // a failure that names the file.**
        smoozy_server_protocol::transfers::Ending::Stopped(_)
        | smoozy_server_protocol::transfers::Ending::Failed => {
            // **What this computer said, kept for the file's *Details*** —
            // `Design/Transfers § A file that failed`, E2-1. Held for this run
            // only, as an upload's refusal is.
            if let Some(met) = &came.what_it_met {
                if let Ok(mut held) = the_failures_this_run().lock() {
                    held.insert((request_id.to_string(), entry.landing_path.clone()), met.clone());
                }
            }
            (
                UnitOutcome::Failed,
                WhatOneUnitCameTo::Failed(entry.landing_path.clone()),
            )
        }
        // **Interrupted or stalled: the record stands for a later carry-on**
        // (M11), and no unit outcome is written.
        smoozy_server_protocol::transfers::Ending::Interrupted
        | smoozy_server_protocol::transfers::Ending::Stalled => {
            the_run.a_file_stopped(request_id, at);
            return WhatOneUnitCameTo::DidNotFinish;
        }
    };
    let _ = running.the_request().entries[at].its_outcome_became(outcome);
    // **The file stops moving in the same hold of the record as its
    // outcome goes down**, so a row composed in between never counts it
    // both as finished and as in flight.
    record.with_the_record(&mut |connection| {
        let request = running.the_request();
        let _ = DownloadRequests::over(connection).one_entry_moved(&request, at);
        the_run.a_file_stopped(request_id, at);
    });
    // **The unit's outcome is written**, so the row can move.
    record.a_transfer_moved();
    what
}

/// **One entry of a manual request, and the request's own row, written down —
/// and nothing else of it.** Added 2026-09-28, step 10c's transfer wave piece
/// 1, the same repair `src/uploading.rs` took in the other direction.
///
/// **The case [`the_download_runs`] was first written for** is a request of a
/// few files, and it wrote the whole request at both of a file's moments —
/// when its transfer is arranged and when it ends. **The case it now also
/// meets** is a *Download all* of a long job, where each of those writes was
/// every entry of the request. **Only the one entry has moved**, so only it
/// goes down (`DownloadRequests::one_entry_moved`). **`Download`'s
/// `KeepsTheRequest` is not changed for it**: this door reaches the store
/// directly, which is `src/uploading.rs`'s shape.
///
/// **A write that did not land is not a reason to stop the request**, which
/// is what the whole write's ignored answer already said here: the unit's
/// transfer record is `Server protocol and transfers` S8's own and is written
/// before any byte whatever this write does.
///
/// **The request is taken inside the record's hold and never the other way
/// round** (see [`TheManualRequestRunning`]).
fn the_entry_is_written_down(
    record: &dyn TheRecordADownloadWrites,
    request: &Mutex<&mut AManualRequest>,
    at: usize,
) {
    record.with_the_record(&mut |connection| {
        let request = request
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let _ = DownloadRequests::over(connection).one_entry_moved(&request, at);
    });
}

/// **The folder one file lands in, made where it is absent** — `Desktop app and
/// delivery` F10's one route.
///
/// **A folder that already stands is not a failure**, which is the spec's own
/// rule on the manual half and holds the same way here. **The failure is
/// answered rather than swallowed**: the caller's fetch then ends at the
/// publication and says so, which is `Desktop app and delivery` F2's rule that
/// a failure is reported and never read as a clean finish.
pub fn the_folder_a_file_lands_in(landing: &[u8]) -> Result<(), WhatTheAdapterSaid> {
    let folder = the_folder_holding(landing);
    if folder.is_empty() {
        return Ok(());
    }
    crate::platform::files::ensure_directory(&the_path_of(&folder))
        .map(|_| ())
        .map_err(|_| WhatTheAdapterSaid::Unclassified)
}

/// **`Download` M6's read of what is standing in each destination folder**, over
/// the platform file adapter's one route to the filesystem.
///
/// **Names are read as the filesystem stores them and compared byte for byte,
/// with no folding modelled** — M6's own rule, and `Desktop app and delivery`
/// C11's reason: the filesystem is asked, nothing is modelled. **A near-miss
/// this comparison cannot see is caught at publication** by C10's exclusive
/// create (M7).
///
/// **A folder that is not there answers nothing**, which is the ordinary case
/// for a fresh destination and not a failure.
pub struct TheDestinationsListing;

impl ReadsTheDestinationsListing for TheDestinationsListing {
    fn what_is_standing_in(&mut self, the_folder: &[u8]) -> Vec<Vec<u8>> {
        let Ok(entries) = std::fs::read_dir(the_path_of(the_folder)) else {
            return Vec::new();
        };
        entries
            .filter_map(|entry| entry.ok())
            .map(|entry| the_bytes_of(&entry.path()))
            .collect()
    }
}

/// **M9's act: each landing folder made plainly**, through the same route.
///
/// **A folder that already stands is not a failure**: the spec has a folder that
/// exists merge rather than collide.
pub struct TheLandingFolders;

impl MakesTheLandingFolder for TheLandingFolders {
    fn create_the_landing_folder(
        &mut self,
        the_landing_folder: &[u8],
    ) -> Result<(), WhatTheAdapterSaid> {
        crate::platform::files::ensure_directory(&the_path_of(the_landing_folder))
            .map(|_| ())
            // **`Download` D1's classes are the adapter's and none is composed here.** A
            // folder that would not be made is the one class this route can
            // reach without reading an error number, and *unclassified* is
            // that clause's own word for a refusal the adapter does not map —
            // never folded into one of the four named causes.
            .map_err(|_| WhatTheAdapterSaid::Unclassified)
    }
}

/// **The names standing in every folder this request would land in**, which is
/// what M8's counted suffix is composed against.
///
/// **Names, and never whole paths.** The listing answers whole paths, and
/// `Download` M8's composition compares a composed *name* against what stands;
/// handed the paths, it matched nothing, so a second *Keep both* composed
/// `name (2)` again and the create-intended publication refused it. Step 10c's
/// thirteenth-pass fix piece 3.
fn the_names_standing_beside<R: ReadsTheDestinationsListing + ?Sized>(
    listing: &mut R,
    request: &AManualRequest,
) -> Vec<Vec<u8>> {
    listing
        .what_is_standing_in(&request.destination)
        .into_iter()
        .map(|path| match path
            .iter()
            .rposition(|byte| *byte == b'/' || (cfg!(windows) && *byte == b'\\'))
        {
            Some(at) => path[at + 1..].to_vec(),
            None => path,
        })
        .collect()
}

/// **M6's comparison, by name and place only, file by file.**
fn the_colliding_landing_paths<R: ReadsTheDestinationsListing + ?Sized>(
    listing: &mut R,
    request: &AManualRequest,
) -> Vec<Vec<u8>> {
    let mut folders: Vec<Vec<u8>> = Vec::new();
    for entry in &request.entries {
        let folder = the_folder_holding(&entry.landing_path);
        if !folders.contains(&folder) {
            folders.push(folder);
        }
    }
    let mut standing: Vec<Vec<u8>> = Vec::new();
    for folder in &folders {
        // **`Download` R6: the staging directory and every reservation-record
        // path are subtracted first.** The staging half mirrors `File Browser`
        // X10 — *a staging path is never a landing place and never compared as
        // one*.
        standing.extend(what_the_comparison_reads(
            &listing.what_is_standing_in(folder),
            &the_reserved_staging_directory_in(folder),
            &[],
        ));
    }
    request
        .entries
        .iter()
        .filter(|entry| standing.contains(&entry.landing_path))
        .map(|entry| entry.landing_path.clone())
        .collect()
}

fn the_folder_holding(landing: &[u8]) -> Vec<u8> {
    match landing.iter().rposition(|byte| *byte == b'/') {
        Some(at) if at > 0 => landing[..at].to_vec(),
        Some(_) => b"/".to_vec(),
        None => Vec::new(),
    }
}

/// **`Server protocol and transfers` S1: the staging directory sits inside the
/// user's own destination folder**, under the reserved name.
fn the_reserved_staging_directory_in(folder: &[u8]) -> Vec<u8> {
    let mut out = folder.to_vec();
    if !out.ends_with(b"/") {
        out.push(b'/');
    }
    out.extend_from_slice(THE_RESERVED_STAGING_FOLDER.as_bytes());
    out
}

/// **One file down, over the server's own point** — the seat asked for per file
/// and given back at that file's end, which is the second half of R17.
///
/// **`None` where the file never started**: the point lent no seat, or the
/// channel would not negotiate. Neither is a transfer that failed.
async fn one_file_comes_down_over<C: AHeldConnection, W: WatchesTheTransfer>(
    sessions: &ThisServersSessions<C>,
    the_run: &TheRunsTransfers,
    record: &dyn TheRecordADownloadWrites,
    watcher: &mut W,
    asked: AFileToBringDown,
    waiting: &[&'static str],
) -> Result<WhatTheDownloadCameTo, WhyNothingCameDown> {
    // **`Server protocol and transfers` T5 and T1: a channel of its own, from
    // the server's own point**, given back when this file ends.
    let channel = sessions
        .a_transfer_channel()
        .await
        .map_err(|_| WhyNothingCameDown::TheDesktopPointAdmittedNothing)?;
    let mut subsystem = negotiated_within(channel, THE_INACTIVITY_DEADLINE)
        .await
        .ok_or(WhyNothingCameDown::TheDesktopPointAdmittedNothing)?;

    let mut keeper = TheTransferRecordsOverTheDoor { record };
    let came =
        one_file_comes_down_for(&mut subsystem, the_run, &mut keeper, watcher, &asked, waiting)
            .await;
    // **The seat goes back at this file's ending, whatever happened**, which is
    // R17's second half and is why nothing holds one across a turn.
    drop(subsystem);
    came
}

/// **`Queue & History` P6's class, by the name `File Browser` O4's list
/// carries for it** — read from that list by its declaring document rather
/// than spelled again here.
pub(crate) fn the_preview_fetchs_class() -> &'static str {
    smoozy_file_browser::pool::THE_DECLARED_ORDER
        .iter()
        .find(|class| class.declared_by == "Queue & History")
        .map(|class| class.name)
        .expect("`File Browser` O4's list carries `Queue & History` P6's class")
}

/// **One derivative down, over the server's own point** — `Queue & History`
/// P4: *"one fetch per derivative, through the transfer contract — staging,
/// integrity, publication last"*, admitted at the desktop point under P6's
/// class, the lowest, and never starved (R17). The 38's piece 3.
///
/// **The record is arranged by `Queue & History`'s own
/// `acting::previews::arrange_one_derivatives_fetch`** and handed in whole;
/// the seat, the admission, the loop and the slot given back at the end are
/// this file's, exactly as a download's are. **The publication is
/// create-intended**: the copy's name is issued fresh for each fetch, so
/// nothing stands at it to be replaced.
///
/// **`None` of a seat or a negotiation is not a fetch that failed**, as for a
/// download: the derivative stays owed.
pub(crate) async fn a_derivative_comes_down_over<C: AHeldConnection>(
    sessions: &ThisServersSessions<C>,
    the_run: &TheRunsTransfers,
    record: &dyn TheRecordADownloadWrites,
    arranged: &mut TransferRecord,
) -> Result<WhatTheDownloadCameTo, WhyNothingCameDown> {
    let channel = sessions
        .a_transfer_channel()
        .await
        .map_err(|_| WhyNothingCameDown::TheDesktopPointAdmittedNothing)?;
    let mut subsystem = negotiated_within(channel, THE_INACTIVITY_DEADLINE)
        .await
        .ok_or(WhyNothingCameDown::TheDesktopPointAdmittedNothing)?;
    let mut keeper = TheTransferRecordsOverTheDoor { record };
    let landing = arranged.final_path.clone().into_bytes();
    let transfer = arranged.transfer.clone();
    let class = the_preview_fetchs_class();
    let came = the_arranged_fetch_comes_down(
        &mut subsystem,
        the_run,
        &mut keeper,
        &mut NobodyIsWatching,
        &landing,
        arranged,
        transfer,
        class,
        &[class],
        None,
        None,
        the_intent(false),
        None,
    )
    .await;
    drop(subsystem);
    came
}

/// **One file of a manual request down, carried on across a repair** —
/// `Download` M11, with `Server protocol and transfers` C11 step 4 and
/// `Server protocol and transfers` S12 entire. Added 2026-09-29, step 10c's transfer wave piece 6.
///
/// **The case the manual route was first written for** is a line that stays
/// up: a file was seated, carried and ended, and a file the line cut ended
/// *interrupted* and was left owed. **The case it now also meets** is a line
/// that goes and comes back inside the period, which `Download` M11 promises
/// the user carries on *"without the user asking again"*. Until this piece
/// nothing handed the cut file back to the loop, and every file after it met a
/// point lending nothing and was skipped, so a one-minute drop ended the whole
/// request with the line healthy again a moment later.
///
/// **What it does, in order.**
///
/// - **The record is arranged once** and every attempt carries that same
///   record, so the loop's own resume (`Server protocol and transfers` S12)
///   reads its checkpoints. **A download's resume always starts over**:
///   `Server protocol and transfers` S13's four values have no reading taken
///   now, so the loop's source gate cannot confirm the file and starts it from
///   zero, which is honest and never splices, and the row says so through
///   `Server protocol and transfers` G4a's mark.
/// - **A seat the point cannot lend because the line is down is waited for**
///   while the period stands ([`while_the_line_is_down`]); one refused on a
///   live line leaves the unit owed, as before.
/// - **An *interrupted* ending is carried on only where the line went down**
///   ([`the_line_went_down`]) **and came back under the same period**, and
///   only through [`the_record_is_carried_on`]'s gate. *Stalled* is never
///   carried on — `Server protocol and transfers` C11 step 4 refuses it.
/// - **After a repair, the desktop point's refusal is asked again** rather than
///   ending the file — R13, a reason to open fewer and never to fail. The
///   first attempt keeps the route's own answer to one, which is not this
///   piece's.
async fn a_requested_file_comes_down<C: AHeldConnection, W: WatchesTheTransfer>(
    sessions: &ThisServersSessions<C>,
    the_run: &TheRunsTransfers,
    record: &dyn TheRecordADownloadWrites,
    watcher: &mut W,
    (request, entry, tell): (&str, usize, &(dyn Fn() + Sync)),
    asked: AFileToBringDown,
    waiting: &[&'static str],
    taken_up: Option<TransferRecord>,
    the_files_behind_it: Vec<Vec<u8>>,
    intent: Intent,
) -> Result<WhatTheDownloadCameTo, WhyNothingCameDown> {
    let WhereTheCallerSaysItLands::TheFolderTheCallerHolds {
        destination,
        landing,
    } = &asked.lands
    else {
        return Err(WhyNothingCameDown::ThereIsNowhereToPutIt(
            WhereItLands::TheSettingsWouldNotRead,
        ));
    };
    // **`Download` M13's third bullet: a part a *Disconnect* or an earlier
    // sitting left is taken up** — its transfer identity, its record and its
    // staging path — and carried on under this request, through `Server
    // protocol and transfers` S12 entire and `Server protocol and transfers` S13's
    // four-value comparison. The
    // record is this run's from here on, which is what `Server protocol and
    // transfers` C11 step 4 reads if the line then goes under it. Step 10c's
    // transfer wave piece 7.
    let mut the_record_stands = taken_up.is_some();
    // **A part taken up is a resume from its first attempt here**, so
    // `Server protocol and transfers` S13's four values are compared before
    // it adds a byte; a fresh file resumes only once the line has cut it.
    let mut a_resume_is_in_question = taken_up.is_some();
    let mut the_four_values = TheFourValuesOverThePoint {
        sessions,
        server: asked.server_row.as_text(),
    };
    let (transfer, mut arranged) = match taken_up {
        Some(mut part) => {
            part.written_by_run = asked.run.clone();
            part.ending = None;
            // **The taken-up unit is governed by the new request's disposition
            // throughout** (`Download` M13's third bullet: *replace publishes
            // replace-intended, an ordinary send publishes create-intended*),
            // so a publication the part left pending goes ahead with the new
            // request's intent and not the one it was staged under. Step 10c's
            // thirteenth-pass fix piece 3.
            if let Some(pending) = part.publication_pending.as_mut() {
                pending.intent = intent;
            }
            (part.transfer.clone(), part)
        }
        None => {
            let (transfer, mut arranged) = the_fetch_is_arranged(&asked, destination, landing)?;
            // **`Server protocol and transfers` S13: all four values are
            // recorded when a download is arranged** — for a file large
            // enough to reach its first checkpoint, the one kind a resume can
            // carry on from (`Server protocol and transfers` S12 step 2). A
            // smaller file always starts over, so a reading of it would be a
            // round trip spent on nothing. **A reading that cannot be taken
            // records nothing**, and a resume then starts the file over. The
            // 38's piece 9, item 18.
            if asked.size > smoozy_server_protocol::transfers::moving::THE_CHECKPOINT_INTERVAL {
                arranged.source_reading = the_four_values.read_the_four_values(&asked.on_the_machine).await;
            }
            (transfer, arranged)
        }
    };
    let stopped_here = |arranged: &mut TransferRecord| {
        let mut keeper = TheTransferRecordsOverTheDoor { record };
        a_download_the_user_stopped_is_written_down(arranged, &mut keeper);
        WhatTheDownloadCameTo {
            ending: THE_USERS_STOP,
            lands_at: the_path_of(landing),
            bytes_staged: 0,
            admitted_as: the_class(asked.arranged_by),
            transfer: transfer.clone(),
            what_it_met: None,
            the_name_was_taken: false,
        }
    };
    let mut carried_on = false;
    // **The additional connection the last attempt sat on died alone, and a
    // fresh one is being asked for** — so a seat not had on a line that is up
    // is waited for and asked again rather than ending the file. The dead-line
    // sweep's extra-connection repair, 2026-10-19.
    let mut its_own_connection_is_replaced: Option<ItWaitsForItsOwnConnection<'_>> = None;
    // **The file reads as moving once the desktop point has admitted it**,
    // and not while it waits for its turn there — step 10c's eleventh-pass
    // fix piece 4, where a second file of the request can wait at a low cap.
    let mut the_row_says_it_moves = false;
    loop {
        // **R17's slot first, and then the seat** — step 10c's eleventh-pass
        // fix piece 4. With `Download` Q1's two files of one request moving
        // at once, a cap low enough to admit one channel refuses the second
        // file while the first moves; **R13 makes that a reason to wait, never
        // to fail**, and a seat held while it waited would be a session of the
        // machine's held for nothing.
        let Some(draw) = a_downward_slot(
            sessions,
            the_run,
            waiting,
            &asked,
            (request, entry, landing, tell),
        )
        .await
        else {
            if the_record_stands && the_run.the_user_stopped_this(request, landing) {
                return Ok(stopped_here(&mut arranged));
            }
            return Err(WhyNothingCameDown::TheDesktopPointAdmittedNothing);
        };
        if !the_row_says_it_moves {
            the_row_says_it_moves = true;
            the_run.a_file_goes(
                request,
                entry,
                landing.to_vec(),
                asked.size,
                the_files_behind_it.clone(),
            );
        }
        // **`Server protocol and transfers` T5 and T1: a channel of its own,
        // from the server's own point**, given back when this attempt ends.
        let seated = match sessions.a_transfer_channel().await {
            Ok(channel) => negotiated_within(channel, THE_INACTIVITY_DEADLINE).await,
            Err(_) => None,
        };
        let Some(mut subsystem) = seated else {
            // **R5: the slot goes back at once**, and is never held across
            // the wait for a seat.
            let mut draw = draw;
            let _ = draw.the_channel_ended(std::time::Instant::now());
            drop(draw);
            // **Whether the line went down is looked for, never glanced at**
            // — `Download` M11 and `File Browser` K2, the same look an
            // interrupted file makes below. Added 2026-10-02, out of the
            // hosted run that was red on Linux at `d7a25677`, where one glance
            // ended a file the repair would have carried on: a seat asked for
            // at the moment the line goes is answered while the point still
            // reads as lending (`crate::uploading::a_slot_and_a_seat` says
            // how).
            if the_line_went_down(sessions).await {
                if while_the_line_is_down(sessions, the_run, request, Some(entry), tell).await {
                    continue;
                }
            } else if let Some(waiting) = &its_own_connection_is_replaced {
                // **The fresh additional connection could not be made yet**,
                // the primary still up: asked again after a pause (the
                // dead-line sweep's extra-connection repair).
                if waiting.pauses().await {
                    continue;
                }
            }
            // **The user stopped it while it waited for its seat** (`File
            // Browser` U6, `Download` M12), and a record of it stands: the
            // stop is written. Where none stands the file never started, and
            // the unit is simply left owed.
            if the_record_stands && the_run.the_user_stopped_this(request, landing) {
                return Ok(stopped_here(&mut arranged));
            }
            return Err(WhyNothingCameDown::TheDesktopPointAdmittedNothing);
        };
        // **Seated again: no longer waiting for its own connection.**
        its_own_connection_is_replaced = None;
        // **`Server protocol and transfers` S13's four values as they stand
        // now, just before the resume** —
        // only where one is in question and the arrangement recorded a
        // reading to compare with; nothing otherwise, which `Server protocol
        // and transfers` reads as *could not be confirmed*. The 38's piece 9,
        // item 18.
        let the_source_now = if a_resume_is_in_question && arranged.source_reading.is_some() {
            the_four_values
                .read_the_four_values(arranged.source_path.as_bytes())
                .await
        } else {
            None
        };
        let mut keeper = TheTransferRecordsOverTheDoor { record };
        let came = the_arranged_fetch_comes_down(
            &mut subsystem,
            the_run,
            &mut keeper,
            watcher,
            landing,
            &mut arranged,
            transfer.clone(),
            the_class(asked.arranged_by),
            waiting,
            Some(request),
            Some(draw),
            intent,
            the_source_now,
        )
        .await;
        // **The seat goes back at this attempt's ending, whatever happened** —
        // and the connection it sat on is kept, to be asked after.
        let its_connection = subsystem.the_channel().the_connection_it_sits_on();
        drop(subsystem);
        if came.is_ok() {
            the_record_stands = true;
        }
        let went_down = match &came {
            Ok(came) if came.ending == Ending::Interrupted => {
                Some(what_went_down(sessions, &its_connection).await)
            }
            _ => None,
        };
        match came {
            Err(_) if carried_on && sessions.a_connection_is_live() => {
                tokio::time::sleep(THE_LOOK_WHILE_THE_LINE_IS_DOWN).await;
            }
            Ok(came) if went_down == Some(WhatWentDown::TheLine) => {
                let back =
                    while_the_line_is_down(sessions, the_run, request, Some(entry), tell).await;
                // **Stopped while it waited for the connection** — the record
                // reads *interrupted*, and the stop makes it *stopped* before
                // anything else; its part is on this computer, so it goes at
                // once (step 10c's transfer wave piece 7).
                if the_run.the_user_stopped_this(request, landing) {
                    return Ok(stopped_here(&mut arranged));
                }
                if back && the_record_is_carried_on(&mut arranged, &asked.run) {
                    carried_on = true;
                    a_resume_is_in_question = true;
                    continue;
                }
                return Ok(came);
            }
            // **`Servers` S14's additional connection died under it alone**,
            // the primary still up — the dead-line sweep's extra-connection repair. **Not a
            // break, and nothing to wait for first** (`Servers` S14 condition
            // 6): the next seat is on a fresh additional connection, and the
            // record is carried on through the same gate as after a repair
            // (`Server protocol and transfers` C11 step 4), its part verified
            // or started over (`Server protocol and transfers` S12,
            // `Server protocol and transfers` S13).
            Ok(came) if went_down == Some(WhatWentDown::ItsOwnConnection) => {
                if the_run.the_user_stopped_this(request, landing) {
                    return Ok(stopped_here(&mut arranged));
                }
                if the_record_is_carried_on(&mut arranged, &asked.run) {
                    carried_on = true;
                    a_resume_is_in_question = true;
                    its_own_connection_is_replaced = Some(ItWaitsForItsOwnConnection::begins(
                        the_run,
                        request,
                        Some(entry),
                        tell,
                    ));
                    continue;
                }
                return Ok(came);
            }
            Ok(came) => {
                // **Item 4 of step 10c's transfer wave piece 7: an attempt the
                // fresh one superseded is removed once the fresh one has
                // published** — by its record and the stable identity recorded
                // for that attempt (`Server protocol and transfers` S4, `Server protocol and
                // transfers` S9, `Server protocol and transfers` S12 step 5 and `Server protocol and
                // transfers` S28's exact match), never on a path alone. A
                // carry-on of a download starts over where `Server protocol
                // and transfers` S13's four values differ or could not be
                // read, and until this the superseded part sat in the user's
                // download folder until the next start's sweep. **Nothing that cannot
                // be matched is touched**, and that sweep is still there for it.
                if came.ending == Ending::Published && !arranged.superseded_attempts.is_empty() {
                    let _ = the_staging_it_names_is_removed_here(&arranged);
                }
                return Ok(came);
            }
            Err(why) => return Err(why),
        }
    }
}

/// **The desktop point's slot for one attempt of one file of a manual
/// request, asked for before its seat** — `Desktop app and delivery` R17's
/// per-file slot, and R13: a refusal on a line that is up is a reason to wait
/// a moment and ask again, never to end the file. Added 2026-09-30, step
/// 10c's eleventh-pass fix piece 4.
///
/// **The case this route was first written for** is one file of the request
/// at a time, where the one channel admissible whatever the budget (R7) was
/// always this request's, and a refusal meant another class held the only
/// slot: the file was left owed and never tried again in the run. **The case
/// it now also meets** is `Download` Q1's two files of one request at once
/// under a cap that admits one: the second file would have been refused at
/// every such moment and ended *did not finish* with nothing wrong.
///
/// **It stops waiting** where the user stops the request or this file (`File
/// Browser` U6, `Download` M12), where the period ends (`Download` M13's
/// paragraph of 2026-09-28), and where the line went down and did not come back
/// under the period ([`while_the_line_is_down`]) — each a certain end, so the
/// wait always ends.
async fn a_downward_slot<C: AHeldConnection>(
    sessions: &ThisServersSessions<C>,
    the_run: &TheRunsTransfers,
    waiting: &[&'static str],
    asked: &AFileToBringDown,
    (request, entry, landing, tell): (&str, usize, &[u8], &(dyn Fn() + Sync)),
) -> Option<TheChannelsDraw> {
    let carries = the_class(asked.arranged_by);
    loop {
        if the_run.the_user_stopped_this(request, landing) || the_run.its_period_has_ended(request)
        {
            return None;
        }
        match the_run.admit_to_carry(Direction::Down, waiting, carries) {
            Ok(draw) => return Some(draw),
            Err(_) if sessions.a_connection_is_live() => {
                tokio::time::sleep(crate::uploading::THE_PAUSE_BEFORE_ASKING_AGAIN).await;
            }
            Err(_) => {
                if !while_the_line_is_down(sessions, the_run, request, Some(entry), tell).await {
                    return None;
                }
            }
        }
    }
}

/// **`Server protocol and transfers` S13's four-value reading of a file on the
/// machine, over the server's own point** — `Download`'s seam
/// ([`smoozy_download::acting::fetching::TheFourValues`]), which had no
/// producer until the 38's piece 9 (item 18).
///
/// **It is the application's one reading of the four values**,
/// [`crate::submitting::the_four_values_of`] — `Render Settings` Q14 item 2's,
/// taken on every rental at submission and in the dependency check — and it
/// takes its turn at the one link site on this server like every other reader.
/// **It is bounded by `Server protocol and transfers` C19's deadline**
/// ([`THE_INACTIVITY_DEADLINE`]), so a line that goes quiet under it answers
/// *nothing* rather than holding the file. **Every way it can fail is
/// nothing**, which `Server protocol and transfers` S13 reads as *could not be
/// confirmed* and never as *the
/// file changed*.
pub(crate) struct TheFourValuesOverThePoint<'a, C: AHeldConnection> {
    pub(crate) sessions: &'a ThisServersSessions<C>,
    /// The local server identifier `sessions` lends for, which the reading
    /// takes its turn by.
    pub(crate) server: String,
}

impl<C: AHeldConnection> smoozy_download::acting::fetching::TheFourValues
    for TheFourValuesOverThePoint<'_, C>
{
    fn read_the_four_values(
        &mut self,
        path: &[u8],
    ) -> impl Future<Output = Option<smoozy_server_protocol::transfers::SourceReading>> + Send {
        let path = std::str::from_utf8(path).ok().map(str::to_string);
        async move {
            let path = path?;
            match tokio::time::timeout(
                THE_INACTIVITY_DEADLINE,
                crate::submitting::the_four_values_of(self.sessions, &self.server, &path),
            )
            .await
            {
                Ok(smoozy_render_settings::readings::FourValuesNow::Answered(values)) => Some(values),
                _ => None,
            }
        }
    }
}

/// **A transfer channel's file-transfer session, negotiated inside `Server
/// protocol and transfers` C19's deadline** — or nothing, where the far end did
/// not answer the opening in time or answered something that is not an opening.
///
/// **The case this was first written for** was a channel the point had just
/// seated on a live connection, whose server answers the opening at once.
/// **The case it now also meets** is a channel whose far end never answers —
/// and a negotiation with no deadline then holds the fetch, the pass and every
/// arrangement of the server behind it for as long as the channel stays open,
/// with nothing said. Step 10c's transfer wave piece 3 (the tenth-pass map's
/// piece 1, item (iii)): `Server protocol and transfers` C19 has every waiter
/// on a transfer channel bounded, and until this piece only the reads and the
/// settles were.
///
/// **No transfer record exists yet**, so there is nothing to end *interrupted*
/// or *stalled*: the file never started, the seat goes back as the channel is
/// dropped, and the unit stays owed (R13 — a reason to open fewer,
/// never to fail). **The figure is C19's own** ([`THE_INACTIVITY_DEADLINE`]),
/// and no second one is written.
async fn negotiated_within<S>(channel: S, deadline: Duration) -> Option<Subsystem<S>>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    tokio::time::timeout(deadline, Subsystem::negotiate(channel))
        .await
        .ok()?
        .ok()
}

/// **`Server protocol and transfers` S9's record, written through the
/// application's own record door.**
///
/// `State, identity and recovery` §1.3 rule 1: one writer per durable store, one
/// short act at a time, never a second connection to the same file.
struct TheTransferRecordsOverTheDoor<'a> {
    record: &'a dyn TheRecordADownloadWrites,
}

impl KeepsTheRecord for TheTransferRecordsOverTheDoor<'_> {
    fn write_it_down(
        &mut self,
        record: &smoozy_server_protocol::transfers::TransferRecord,
    ) -> Result<(), LocalFailure> {
        let mut written = Err(LocalFailure("the local record would not open".into()));
        self.record.with_the_record(&mut |connection| {
            written = crate::record::transfer_records::put(connection, record)
                .map_err(|failure| LocalFailure(format!("{failure:?}")));
        });
        written
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **`File Browser` U8's amended bound on an upload leftover's *Remove***:
    /// only an entry directly inside Smoozy's own staging folder, and nothing
    /// that steps out of it. The 38's piece 9, item 33.
    #[test]
    fn a_leftovers_remove_reaches_only_inside_a_staging_folder() {
        for inside in [
            b"/home/ubuntu/kitchen/.smoozy-staging/0dd7caa5-1.smoozy-part".as_slice(),
            b"/.smoozy-staging/a-1.smoozy-part",
        ] {
            assert!(it_lies_inside_a_staging_folder(inside), "{}", String::from_utf8_lossy(inside));
        }
        for outside in [
            b"/home/ubuntu/kitchen/wood.png".as_slice(),
            b"/home/ubuntu/.smoozy-staging/../kitchen/wood.png",
            b"/home/ubuntu/.smoozy-staging/./a-1.smoozy-part",
            b"/home/ubuntu/.smoozy-staging/",
            b"/home/ubuntu/.smoozy-staging",
            b"relative/.smoozy-staging/a-1.smoozy-part",
            b"/home/ubuntu/.smoozy-staging/deeper/a-1.smoozy-part",
            b"/home/ubuntu//.smoozy-staging/a",
        ] {
            assert!(!it_lies_inside_a_staging_folder(outside), "{}", String::from_utf8_lossy(outside));
        }
    }
    use crate::scheduling::BytesPerSecond;

    use smoozy_download::acting::walking::TheEntrysKind;
    use smoozy_download::arrangement::{DesiredState, WhatSmoozyDid};
    use smoozy_download::ending::ANarrowedEntry;
    use smoozy_download::vouching::TheOrder;
    use smoozy_download::watch::TheWindow;

    const KIB: u64 = 1024;

    /// **Step 10c's eleventh-pass fix piece 4: a file is never drawn both
    /// moving and waiting.** A request's files start in the enumeration's
    /// order but say so in whatever order their channels get there, so the
    /// files a starting file names as waiting behind it may already be moving
    /// beside it. The register keeps the next ones not moving, as many as a
    /// row draws.
    #[test]
    fn the_files_waiting_are_the_next_ones_not_already_moving() {
        let the_run = TheRunsTransfers::for_this_run();
        let issue = |kind| Identity::issue(kind).expect("random bytes");
        let (request, server) = (
            issue(IdentityKind::Operation),
            issue(IdentityKind::LocalServer),
        );
        let _under_way = the_run.a_request_is_under_way(&request, &server);
        let path = |at: usize| format!("/home/a/file_{at}.bin").into_bytes();
        let entries: Vec<usize> = (0..10).collect();
        let behind = |at: usize| the_next_waiting(&entries, at, |one| Some(path(*one)));
        // **Files 1 and 2 said they move before file 0 did.**
        the_run.a_file_goes(&request.as_text(), 1, path(1), 1, behind(1));
        the_run.a_file_goes(&request.as_text(), 2, path(2), 1, behind(2));
        the_run.a_file_goes(&request.as_text(), 0, path(0), 1, behind(0));
        let under_way = the_run.what_is_under_way()[&request.as_text()].clone();
        assert_eq!(under_way.moving.len(), 3);
        assert_eq!(
            under_way.waiting,
            vec![path(3), path(4), path(5)],
            "the files waiting are the next three not already moving"
        );
    }

    /// **Step 10c's transfer wave piece 7: the user's Stop is marked on a
    /// request under way, on the request or on one file of it, and on nothing
    /// that is not under way** (`Server protocol and transfers` C22, `File
    /// Browser` U6, `Download` M12). **The draw is held at nothing once it is
    /// marked, and the loop is ended at that pause and not before.**
    #[tokio::test]
    async fn a_stop_is_marked_where_it_is_pressed_and_holds_the_draw_at_nothing() {
        let the_run = TheRunsTransfers::for_this_run();
        let issue = |kind| Identity::issue(kind).expect("random bytes");
        let (one, two) = (issue(IdentityKind::Operation), issue(IdentityKind::Operation));
        let server = issue(IdentityKind::LocalServer);
        assert!(!the_run.the_user_stops(&one.as_text(), None), "nothing under way, nothing to stop");
        let _one = the_run.a_request_is_under_way(&one, &server);
        let _two = the_run.a_request_is_under_way(&two, &server);
        the_run.a_file_goes(&two.as_text(), 3, b"/home/a/wood.png".to_vec(), 100, Vec::new());

        // **One file of `two`**, by the landing path its row names.
        assert!(the_run.the_user_stops(&two.as_text(), Some(b"/home/a/wood.png")));
        assert!(the_run.the_user_stopped_this(&two.as_text(), b"/home/a/wood.png"));
        assert!(!the_run.the_user_stopped_this(&two.as_text(), b"/home/a/bark.png"));
        assert!(!the_run.the_user_stopped_it(&two.as_text()), "one file's stop is not the request's");
        assert!(the_run.moving().a_stop_reaches(&two.as_text(), Some(3)));
        assert!(!the_run.moving().a_stop_reaches(&two.as_text(), None));

        // **The whole of `one`.**
        assert!(!the_run.the_user_stopped_this(&one.as_text(), b"/anything"));
        assert!(the_run.the_user_stops(&one.as_text(), None));
        assert!(the_run.the_user_stopped_it(&one.as_text()));
        assert!(the_run.the_user_stopped_this(&one.as_text(), b"/anything"));
        assert!(the_run.it_ran_in_this_run(&one.as_text()));

        // **The draw is held at nothing, and says so.**
        struct Everything;
        impl DrawsFromTheSharedBudget for Everything {
            fn draw(&mut self, wanted: usize) -> usize {
                wanted
            }
            fn give_back(&mut self, _unused: usize) {}
        }
        let parked = AtomicBool::new(false);
        let mut everything = Everything;
        let request = two.as_text();
        let mut held = TheStopHoldsTheDraw {
            draw: &mut everything,
            the_run: &the_run,
            request: &request,
            landing: b"/home/a/bark.png",
            parked: &parked,
        };
        assert_eq!(held.draw(10), 10, "another file of the request goes on");
        held.landing = b"/home/a/wood.png";
        assert_eq!(held.draw(10), 0);
        assert!(parked.load(Ordering::SeqCst));

        // **The loop is ended at its pause, and a loop that ends by itself
        // first is let end.**
        let never = std::future::pending::<()>();
        assert_eq!(the_stop_lands_or(never, &parked).await, None);
        let ends = async { 7 };
        assert_eq!(the_stop_lands_or(ends, &AtomicBool::new(false)).await, Some(7));

        // **A request waiting for its connection stops waiting when it is
        // stopped.**
        let lends_nothing: ThisServersSessions = ThisServersSessions::for_one_server();
        let waited = tokio::time::timeout(
            Duration::from_secs(5),
            while_the_line_is_down(&lends_nothing, &the_run, &one.as_text(), None, &|| ()),
        )
        .await;
        assert_eq!(waited, Ok(false), "a stopped request does not wait for its connection");
    }

    /// **Step 10c's transfer wave piece 6: a period's end ends that server's
    /// requests and no other's** (`File Browser` U7's paragraph of
    /// 2026-09-28, `Download` M13's) — **and a request begun on a point that
    /// lends nothing is not waited on**, since its period may already have
    /// ended and its point will never lend again.
    #[tokio::test]
    async fn a_period_that_ends_ends_its_servers_requests_and_no_others() {
        let the_run = TheRunsTransfers::for_this_run();
        let issue = |kind| Identity::issue(kind).expect("random bytes");
        let (one, two) = (issue(IdentityKind::Operation), issue(IdentityKind::Operation));
        let (here, there) = (issue(IdentityKind::LocalServer), issue(IdentityKind::LocalServer));
        let _one = the_run.a_request_is_under_way(&one, &here);
        let _two = the_run.a_request_is_under_way(&two, &there);
        assert!(!the_run.its_period_has_ended(&one.as_text()));
        the_run.a_servers_period_ended(&here.as_text());
        assert!(the_run.its_period_has_ended(&one.as_text()));
        assert!(!the_run.its_period_has_ended(&two.as_text()), "another server's request goes on");
        assert!(the_run.what_is_under_way()[&one.as_text()].its_period_ended);
        // **And it is remembered as what ended the request** — the Creator's
        // seventh notes, piece Z5b — while it is held and after its run
        // returns; the other server's request was not ended by it.
        assert!(the_run.its_period_ended_it(&one.as_text()));
        assert!(!the_run.its_period_ended_it(&two.as_text()));

        // **A point lending nothing when the request is held.**
        let three = issue(IdentityKind::Operation);
        let lends_nothing: ThisServersSessions = ThisServersSessions::for_one_server();
        let _three = the_run.a_request_goes_under_way(&three, &there, &lends_nothing);
        assert!(the_run.its_period_has_ended(&three.as_text()));
        let told = std::sync::atomic::AtomicUsize::new(0);
        let tell = || {
            told.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        };
        assert_eq!(
            tokio::time::timeout(
                Duration::from_secs(5),
                while_the_line_is_down(&lends_nothing, &the_run, &three.as_text(), None, &tell),
            )
            .await,
            Ok(false),
            "a request whose period has ended does not wait"
        );
        assert!(!the_run.what_is_under_way()[&three.as_text()].waits_for_the_connection);
    }

    /// **Step 10c's transfer wave piece 6: `Server protocol and transfers`
    /// C11 step 4's gate on the one record a carry-on hands back** — only
    /// *interrupted*, and only written by this run; and where it passes, the
    /// record is moving again and says so.
    #[test]
    fn a_record_is_carried_on_only_where_c11_step_four_would() {
        let issue = |kind| Identity::issue(kind).expect("random bytes");
        let this_run = issue(IdentityKind::Run);
        let a_record = |ending: Option<Ending>, run: &Identity| {
            let mut record = TransferRecord::arranged(
                issue(IdentityKind::Transfer),
                Direction::Down,
                issue(IdentityKind::LocalServer),
                None,
                "/jobs/frame.exr".to_string(),
                "/Users/artist/frame.exr".to_string(),
                "/Users/artist/.smoozy-staging",
                1_000,
                run.clone(),
            );
            record.ending = ending;
            record
        };
        let mut cut = a_record(Some(Ending::Interrupted), &this_run);
        assert!(the_record_is_carried_on(&mut cut, &this_run));
        assert_eq!(cut.ending, None, "a transfer moving again has no ending");
        for refused in [Ending::Stalled, Ending::Published, Ending::Failed] {
            let mut record = a_record(Some(refused.clone()), &this_run);
            assert!(!the_record_is_carried_on(&mut record, &this_run), "{refused:?} is carried on");
            assert_eq!(record.ending, Some(refused));
        }
        let mut earlier = a_record(Some(Ending::Interrupted), &issue(IdentityKind::Run));
        assert!(
            !the_record_is_carried_on(&mut earlier, &this_run),
            "a record an earlier run wrote is not carried on"
        );
    }

    /// **The Creator's seventh notes, piece Z5b: a request remembers that its
    /// server's period ended it** — its line cut and not repaired, or the
    /// period ended under it — after its run returns, so its finished line
    /// reads red. One the user stopped, one that returned with its period
    /// standing, and one begun again are not remembered so.
    #[test]
    fn a_request_its_period_ended_is_remembered_so_after_its_run_returns() {
        let the_run = TheRunsTransfers::for_this_run();
        let issue = |kind| Identity::issue(kind).expect("random bytes");
        let server = issue(IdentityKind::LocalServer);
        let (cut, stopped) = (issue(IdentityKind::Operation), issue(IdentityKind::Operation));
        {
            let _cut = the_run.a_request_is_under_way(&cut, &server);
            let _stopped = the_run.a_request_is_under_way(&stopped, &server);
            assert!(!the_run.its_period_ended_it(&cut.as_text()), "nothing has ended it yet");
            assert!(the_run.the_user_stops(&stopped.as_text(), None));
            the_run.a_servers_period_ended(&server.as_text());
            assert!(the_run.its_period_ended_it(&cut.as_text()), "while its run still holds it");
        }
        assert!(the_run.its_period_ended_it(&cut.as_text()), "after its run returned");
        assert!(!the_run.its_period_ended_it(&stopped.as_text()), "the user's stop came first");

        let other = issue(IdentityKind::LocalServer);
        let finished = issue(IdentityKind::Operation);
        drop(the_run.a_request_is_under_way(&finished, &other));
        assert!(!the_run.its_period_ended_it(&finished.as_text()), "its period still stands");

        let _again = the_run.a_request_is_under_way(&cut, &other);
        assert!(!the_run.its_period_ended_it(&cut.as_text()), "begun again, it is under way again");
    }

    /// **Step 10c's transfer wave piece 5: a request is under way exactly
    /// while its run holds it**, and a file's report reaches the register.
    #[test]
    fn a_request_is_under_way_while_its_run_holds_it_and_its_files_report_there() {
        let the_run = TheRunsTransfers::for_this_run();
        let request = Identity::issue(IdentityKind::Operation).expect("random bytes");
        let id = request.as_text();
        {
            let server = Identity::issue(IdentityKind::LocalServer).expect("random bytes");
            let _under_way = the_run.a_request_is_under_way(&request, &server);
            the_run.a_file_goes(&id, 2, b"/a/b.png".to_vec(), 100, vec![b"/a/c.png".to_vec()]);
            let told = std::sync::atomic::AtomicUsize::new(0);
            let tell = || {
                told.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            };
            let mut watcher = TheRowIsTold {
                the_run: &the_run,
                request: id.clone(),
                entry: 2,
                tell: &tell,
                started_over: None,
            };
            watcher.report(&Report {
                bytes_staged: 40,
                bytes_expected: 100,
                started_over: None,
                verifying_a_prefix_of: None,
            });
            let held = the_run.what_is_under_way();
            let moving = &held[&id];
            assert_eq!(moving.moving[0].report.bytes_staged, 40);
            assert_eq!(moving.waiting, vec![b"/a/c.png".to_vec()]);
            assert_eq!(told.load(std::sync::atomic::Ordering::SeqCst), 1);
            the_run.a_file_stopped(&id, 2);
            assert!(the_run.what_is_under_way()[&id].moving.is_empty());
        }
        assert!(
            !the_run.what_is_under_way().contains_key(&id),
            "a request whose run let it go is no longer under way"
        );
    }

    /// **The page is told no more often than a few times a second**, however
    /// many reports arrive, over every transfer of the run together — and a
    /// report after the bound tells it again.
    #[test]
    fn the_page_is_told_a_few_times_a_second_and_no_more() {
        let mut moving = WhatIsMovingNow::default();
        moving.begins("one", "server-1");
        moving.begins("two", "server-1");
        moving.a_file_goes("one", 0, b"/x".to_vec(), 10, Vec::new());
        moving.a_file_goes("two", 0, b"/y".to_vec(), 10, Vec::new());
        let report = Report {
            bytes_staged: 1,
            bytes_expected: 10,
            started_over: None,
            verifying_a_prefix_of: None,
        };
        let began = Instant::now();
        let mut told = 0;
        for tenth in 0..20u32 {
            for request in ["one", "two"] {
                if moving.a_file_reported(request, 0, &report, began + Duration::from_millis(100) * tenth) {
                    told += 1;
                }
            }
        }
        // Two seconds of reports from two files, ten a second each: forty
        // reports, and the page told at the first and then at the first report
        // a quarter of a second or more after the last telling — at 0, 0.3,
        // 0.6, 0.9, 1.2, 1.5 and 1.8 s.
        assert_eq!(told, 7);
        assert!(!moving.a_file_reported("three", 0, &report, began + Duration::from_secs(9)));
    }

    fn the_two_download_classes() -> (&'static str, &'static str) {
        (
            the_class(WhatArrangedIt::AnAutoDownloadArrangement),
            the_class(WhatArrangedIt::AManualRequest),
        )
    }

    /// **`File Browser` O5's N, computed from the declarations.** Eight, with
    /// `Rendering` §7's submission class, R19's update fetch and `File Browser`
    /// L29's picture view joined (the picture view from 2026-10-31).
    #[test]
    fn every_declared_class_is_in_the_order_this_point_runs_over() {
        let order = the_declared_order();
        assert_eq!(order.len(), 8, "N is the count of declared classes");
        let names: Vec<&str> = order.iter().map(|class| class.name).collect();
        assert_eq!(
            names,
            vec![
                "auto-download",
                "manual-download",
                THE_SUBMISSION_CLASS,
                smoozy_file_browser::declarations::THE_PICTURE_VIEW_CLASS,
                "dependency-upload",
                smoozy_file_browser::declarations::THE_TRANSFER_PRIORITY_CLASS,
                crate::scheduling::THE_UPDATE_FETCHS_CLASS,
                "preview-fetch",
            ],
            "the order is that document's with the two later declarations placed by their own \
             relations"
        );
    }

    /// **R3 and R8: a run starts with no cap set, and what Settings saved is
    /// what the budgets hold.**
    #[test]
    fn the_caps_settings_saved_are_the_budgets_this_run_holds() {
        let run = TheRunsTransfers::for_this_run();
        assert_eq!(run.the_cap(Direction::Up), TheCap::NoCapIsSet);
        assert_eq!(run.the_cap(Direction::Down), TheCap::NoCapIsSet);
        assert_eq!(
            run.the_admissible_count(Direction::Down),
            TheAdmissibleCount::TheBudgetBindsNothing
        );

        run.the_caps_saved_are(
            TheCap::Of(BytesPerSecond(64 * KIB)),
            TheCap::Of(BytesPerSecond(16 * KIB)),
        );
        assert_eq!(
            run.the_cap(Direction::Up),
            TheCap::Of(BytesPerSecond(64 * KIB))
        );
        // **R7: the budget divided by the floor**, which at the floor itself is
        // one.
        assert_eq!(
            run.the_admissible_count(Direction::Down),
            TheAdmissibleCount::Of(1)
        );
        assert_eq!(
            run.the_admissible_count(Direction::Up),
            TheAdmissibleCount::Of(4)
        );
    }

    /// **R17 at an admissible count of one, with a second class waiting — and
    /// this is the test the piece exists for.**
    ///
    /// **The failure it is written against is not a crash.** A first caller
    /// that takes the single slot and holds it leaves the point looking
    /// perfect: one channel moving, the cap held, nothing failing. R13 makes
    /// every refusal behind it silent, so the user's own manual download shows
    /// *underway* for ever and nothing is ever reported.
    ///
    /// So the drive below is the shape that fails: an auto-download
    /// arrangement that keeps asking, with a manual download waiting at every
    /// decision. **Both classes take turns**, and the assertion is on the
    /// sequence rather than on a count.
    #[test]
    fn at_a_count_of_one_two_waiting_classes_take_turns_and_neither_is_starved() {
        let (auto, manual) = the_two_download_classes();
        let run = TheRunsTransfers::for_this_run();
        run.a_cap_was_saved(Direction::Down, TheCap::Of(BytesPerSecond(16 * KIB)));
        assert_eq!(
            run.the_admissible_count(Direction::Down),
            TheAdmissibleCount::Of(1),
            "R17's case is the admissible count being one"
        );

        // Both classes hold work waiting at every decision, which is the
        // arrangement Row 219 describes: it keeps trying and never switches
        // itself off.
        let waiting = [auto, manual];
        let mut granted = Vec::new();
        for _ in 0..6 {
            let draw = run
                .admit(Direction::Down, &waiting)
                .expect("one channel is always admissible whatever the budget is");
            granted.push(draw.the_class());
            // **The slot is given back at the file's ending, which is what the
            // caller does.** Holding it here is what the next test drives.
            drop(draw);
        }

        assert_eq!(
            granted,
            vec![auto, manual, auto, manual, auto, manual],
            "at a count of one the guarantee is turn-taking between the classes with work \
             waiting, and the highest-ranked class took every slot instead"
        );
    }

    /// **The same drive with the first caller holding its slot, so the failure
    /// R17 names is watched rather than described.**
    ///
    /// **This is what the test above fails into.** With the draw held, the
    /// budget cannot afford a second channel's floor, so the manual download
    /// is refused at every decision — and R13 makes that refusal not a
    /// failure, so nothing anywhere would say so.
    #[test]
    fn a_caller_that_holds_the_one_slot_starves_the_class_behind_it() {
        let (auto, manual) = the_two_download_classes();
        let run = TheRunsTransfers::for_this_run();
        run.a_cap_was_saved(Direction::Down, TheCap::Of(BytesPerSecond(16 * KIB)));

        let held = run
            .admit(Direction::Down, &[auto, manual])
            .expect("the first is admitted");
        assert_eq!(held.the_class(), auto);

        for _ in 0..5 {
            assert!(
                run.admit(Direction::Down, &[manual]).is_none(),
                "a held slot let a second channel in, so this is not the failure R17 names"
            );
        }
        // **One grant was made and the class behind it never had one**, which
        // is the starvation stated as a count.
        assert_eq!(run.grants_made(Direction::Down), 1);
        drop(held);

        // And with it given back the class behind it takes the next one.
        let next = run
            .admit(Direction::Down, &[auto, manual])
            .expect("the slot is free again");
        assert_eq!(next.the_class(), manual);
    }

    /// **R17 kept structurally: nothing in this crate holds a draw in a
    /// field.**
    ///
    /// A draw held anywhere but a local of the call that admitted it is the
    /// first caller holding the slot, whatever the code around it says it
    /// does. **This reads every file of the crate rather than this one**,
    /// because the hold could be put anywhere.
    #[test]
    fn no_field_anywhere_in_this_crate_holds_an_admitted_channel() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let mut holders = Vec::new();
        for (name, text) in every_rust_file_under(&root) {
            for line in text.lines() {
                let line = line.trim();
                if line.starts_with("//") || line.starts_with("///") {
                    continue;
                }
                // A field declaration: a name, a colon, and the type. A local
                // is `let ... =` and a parameter is inside brackets on a
                // signature line, neither of which reads this way.
                //
                // **The shapes are joined at run time rather than written
                // out**, so that this test's own lines are not what it finds.
                let held = "TheChannels";
                let draw = "Draw";
                let looks_like_a_field = [
                    format!(": {held}{draw}"),
                    format!(": Option<{held}{draw}>"),
                    format!(": Vec<{held}{draw}>"),
                    format!(": Mutex<{held}{draw}>"),
                ]
                .iter()
                .any(|shape| line.contains(shape.as_str()));
                if looks_like_a_field {
                    holders.push(format!("{name}: {line}"));
                }
            }
        }
        assert!(
            holders.is_empty(),
            "an admitted channel is held in a field, which is the first caller holding the one \
             slot R17's turn-taking is about: {holders:?}"
        );
    }

    fn every_rust_file_under(folder: &std::path::Path) -> Vec<(String, String)> {
        let mut out = Vec::new();
        let Ok(entries) = std::fs::read_dir(folder) else {
            return out;
        };
        for entry in entries {
            let path = entry.expect("the entry is readable").path();
            if path.is_dir() {
                out.extend(every_rust_file_under(&path));
            } else if path.extension().and_then(|e| e.to_str()) == Some("rs") {
                out.push((
                    path.display().to_string(),
                    std::fs::read_to_string(&path).expect("the file is readable"),
                ));
            }
        }
        out
    }

    /// **W4, kept structurally: this file writes nothing into the folder it
    /// proposes.** The download's own writes are the transfer's, inside the
    /// reserved staging directory, and the publication is one rename.
    #[test]
    fn nothing_here_writes_into_the_folder_it_proposes() {
        // **The working half alone**, because this test's own list of writes
        // is written in this same file and would otherwise be what it finds.
        let whole = include_str!("downloading.rs");
        let working = whole
            .split_once("#[cfg(test)]")
            .map(|(before, _)| before)
            .expect("this file has a test module");
        // The names are joined at run time for the same reason.
        for (holder, act) in [
            ("fs::", "create_dir_all"),
            ("fs::", "write"),
            ("File::", "create"),
        ] {
            let shape = format!("{holder}{act}(");
            assert!(
                !working.contains(shape.as_str()),
                "W4: this file writes into the folder it proposes ({shape})"
            );
        }
    }

    /// **`Download` H2: the folder the user chose is the one a download lands
    /// in, and the proposal is what stands where they chose none.**
    #[test]
    fn the_chosen_folder_is_what_a_download_lands_in() {
        let folder = std::env::temp_dir().join("smoozy-downloading-chosen");
        let connection = rusqlite::Connection::open_in_memory().expect("a record");
        crate::record::migrations::bring_up_to_date(
            &connection,
            std::path::Path::new("in-memory"),
            false,
        )
        .expect("it migrates");

        // Nothing chosen: §8's proposal, or W5's nothing. **Both are honest
        // answers and which one this machine gives is not this test's
        // business** — what is asserted is that the chosen folder is never
        // invented.
        assert!(!matches!(
            where_a_download_lands(&connection),
            WhereItLands::TheFolderTheUserChose(_)
        ));

        settings::the_download_folder_was_chosen(&connection, &the_bytes_of(&folder))
            .expect("it saves");
        assert_eq!(
            where_a_download_lands(&connection),
            WhereItLands::TheFolderTheUserChose(folder),
            "a download lands where the user pointed and nowhere else"
        );
    }

    // ────────────────────────────────────────────────────────────────────
    // The arrangement, made at submission — step 10c piece 10
    // ────────────────────────────────────────────────────────────────────

    /// A record door over one open in-memory record, which is the shape the
    /// running application's own door has.
    struct ARecordForTheseTests {
        connection: std::sync::Mutex<rusqlite::Connection>,
    }

    impl ARecordForTheseTests {
        fn fresh() -> ARecordForTheseTests {
            let connection = rusqlite::Connection::open_in_memory().expect("a record");
            crate::record::migrations::bring_up_to_date(
                &connection,
                std::path::Path::new("in-memory"),
                false,
            )
            .expect("it migrates");
            ARecordForTheseTests {
                connection: std::sync::Mutex::new(connection),
            }
        }
    }

    impl TheRecordADownloadWrites for ARecordForTheseTests {
        fn with_the_record(&self, act: &mut dyn FnMut(&rusqlite::Connection)) -> bool {
            let held = self
                .connection
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            act(&held);
            true
        }
    }

    fn an_identity(kind: IdentityKind) -> Identity {
        Identity::issue(kind).expect("the desk has randomness")
    }

    fn what_the_submission_chose(job: &Identity, server_row: &Identity) -> WhatTheSubmissionChose {
        WhatTheSubmissionChose {
            job: job.clone(),
            server_row: server_row.clone(),
            destination: b"/Users/artist/Renders".to_vec(),
            narrowing: TheNarrowing::TheseFormats(vec![b"PNG".to_vec()]),
        }
    }

    /// **A fetch an earlier run left with no ending is given up before the
    /// file is fetched again, and its part goes with it** — `Server protocol
    /// and transfers` S28a as the arrangement keeps it. The twenty-sixth notes,
    /// note 4, *Also seen*: quitting Smoozy mid-file left the record unended
    /// and its `…-1.smoozy-part` in `.smoozy-staging` four restarts later.
    /// **One this run wrote is left alone**, since it may still be moving.
    #[test]
    fn twenty_sixth_4_a_fetch_an_earlier_run_left_unended_is_given_up_and_its_part_removed() {
        use smoozy_download::arrangement::{AReckoningEntry, WhatSmoozyDid};
        use smoozy_server_protocol::transfers::moving::TheUsersOwnComputer as _;
        use smoozy_server_protocol::transfers::{Direction, StoppedReason, TransferRecord};
        let record = ARecordForTheseTests::fresh();
        let folder = crate::scratch_folders::a_folder(format!("smoozy-unended-{}", std::process::id()));
        let staging_folder = folder.join(".smoozy-staging");
        std::fs::create_dir_all(&staging_folder).unwrap();
        let server = an_identity(IdentityKind::LocalServer);
        let this_run = an_identity(IdentityKind::Run);
        let a_fetch = |name: &str, run: &Identity| {
            let mut fetch = TransferRecord::arranged(
                an_identity(IdentityKind::Transfer),
                Direction::Down,
                server.clone(),
                None,
                format!("/machine/{name}"),
                folder.join(name).to_string_lossy().into_owned(),
                &staging_folder.to_string_lossy(),
                12,
                run.clone(),
            );
            let mut adapter = ThePlatformFileAdapter;
            let at = fetch.staging_path.as_text();
            adapter.create_staging_exclusively(&at).unwrap();
            adapter.write_at(&at, 0, b"part").unwrap();
            fetch.staging_identity = Some(adapter.stable_identity_of(&at));
            record.with_the_record(&mut |connection| {
                crate::record::transfer_records::put(connection, &fetch).unwrap();
            });
            (fetch.transfer.clone(), PathBuf::from(at))
        };
        let (earlier, earlier_part) = a_fetch("0003.exr", &an_identity(IdentityKind::Run));
        let (ours, our_part) = a_fetch("0004.exr", &this_run);
        let mut arrangement =
            AnArrangement::switched_on(an_identity(IdentityKind::Job), server.clone(), folder.as_os_str().as_encoded_bytes().to_vec(), TheNarrowing::Everything);
        for (key, transfer) in [(b"0003.exr", &earlier), (b"0004.exr", &ours)] {
            let mut entry = AReckoningEntry::acted_on(key.to_vec(), Some(3), WhatSmoozyDid::Owed, arrangement.destination.clone());
            entry.account.transfer = Some(transfer.clone());
            arrangement.reckoning.push(entry);
        }
        let unit = |key: &[u8]| AUnitOwed {
            key: key.to_vec(),
            in_order_at: 0,
            arranged_by: WhatArrangedIt::AnAutoDownloadArrangement,
            the_destination_normalised: arrangement.destination.clone(),
            the_landing_path: key.to_vec(),
        };
        the_interrupted_fetch_is_given_up(&record, &arrangement, &unit(b"0003.exr"), &this_run).unwrap();
        the_interrupted_fetch_is_given_up(&record, &arrangement, &unit(b"0004.exr"), &this_run).unwrap();
        let ending_of = |transfer: &Identity| {
            let mut ending = None;
            record.with_the_record(&mut |connection| {
                ending = crate::record::transfer_records::read(connection, transfer).unwrap().unwrap().ending;
            });
            ending
        };
        assert!(!earlier_part.exists(), "the part an earlier run left stayed in the staging folder");
        assert_eq!(ending_of(&earlier), Some(Ending::Stopped(StoppedReason::AbandonedByItsRequest)));
        assert!(our_part.exists(), "a fetch this run holds was given up");
        assert_eq!(ending_of(&ours), None);
        let _ = std::fs::remove_dir_all(folder);
    }

    /// **The publication's exclusive create found the name taken, and the
    /// fetch says so** — `Download` P6's built note of 2026-10-31. The words
    /// are the same as ever; what is new is that an auto-download can tell
    /// this refusal from every other one. The twenty-sixth notes, note 4.
    #[test]
    fn twenty_sixth_4_a_publication_over_a_taken_name_says_the_name_was_taken() {
        use smoozy_server_protocol::transfers::moving::TheUsersOwnComputer as _;
        let folder = crate::scratch_folders::a_folder(format!("smoozy-taken-{}", std::process::id()));
        let staged = folder.join("staged.part");
        let taken = folder.join("0001.png");
        let free = folder.join("0002.png");
        std::fs::write(&staged, b"new").unwrap();
        std::fs::write(&taken, b"already here").unwrap();
        let mut local = TheAdapterKeepsWhatItMet::new();
        assert!(local
            .publish(&staged.to_string_lossy(), &taken.to_string_lossy(), Intent::CreateIntended)
            .is_err());
        assert!(local.the_name_was_taken);
        assert_eq!(std::fs::read(&taken).unwrap(), b"already here");
        let mut local = TheAdapterKeepsWhatItMet::new();
        local
            .publish(&staged.to_string_lossy(), &free.to_string_lossy(), Intent::CreateIntended)
            .unwrap();
        assert!(!local.the_name_was_taken);
        let _ = std::fs::remove_dir_all(folder);
    }

    /// **`Download` R1: the arrangement is made at submission where the submit
    /// strip's choice is on, and it is durable from that moment.**
    ///
    /// The record is written, then read back through a **second** keeper over
    /// the same record, which is what a fresh application run does
    /// (`Download` R12).
    #[test]
    fn an_arrangement_is_made_where_the_submit_strips_choice_is_on() {
        let record = ARecordForTheseTests::fresh();
        let job = an_identity(IdentityKind::Job);
        let server_row = an_identity(IdentityKind::LocalServer);

        let made = the_arrangement_is_made_at_submission(
            &mut TheArrangementsKeeper { record: &record },
            true,
            what_the_submission_chose(&job, &server_row),
        )
        .expect("the arrangement is made");
        assert_eq!(made.desired_state, DesiredState::On);
        assert!(
            made.reckoning.is_empty(),
            "R5: the reckoning is an account of Smoozy's own acts and nothing has been acted on"
        );

        let read = TheArrangementsKeeper { record: &record }
            .the_arrangement_for(&job)
            .expect("it reads")
            .expect("it is durable from the moment it is made");
        assert_eq!(read, made);
    }

    /// **The choice being off is not an arrangement switched off — it is no
    /// arrangement at all**, which is step 10b's scenario: its situation says
    /// outright *"auto-download was never switched on."*
    #[test]
    fn a_submission_with_the_choice_off_writes_no_arrangement_at_all() {
        let record = ARecordForTheseTests::fresh();
        let job = an_identity(IdentityKind::Job);
        let server_row = an_identity(IdentityKind::LocalServer);

        let why = the_arrangement_is_made_at_submission(
            &mut TheArrangementsKeeper { record: &record },
            false,
            what_the_submission_chose(&job, &server_row),
        )
        .expect_err("no arrangement is made");
        assert_eq!(why, WhyNoArrangementWasMade::TheChoiceWasOff);
        assert!(
            TheArrangementsKeeper { record: &record }
                .the_arrangement_for(&job)
                .unwrap()
                .is_none(),
            "a job submitted with the choice off has never had an arrangement"
        );
    }

    /// **`Download` R1 fixes the destination at the moment the arrangement is
    /// made**, so a submission that names none makes nothing — an arrangement
    /// with no destination would have to ask a user who is not there.
    #[test]
    fn a_submission_that_names_no_folder_makes_no_arrangement() {
        let record = ARecordForTheseTests::fresh();
        let job = an_identity(IdentityKind::Job);
        let mut chose = what_the_submission_chose(&job, &an_identity(IdentityKind::LocalServer));
        chose.destination = Vec::new();
        let why = the_arrangement_is_made_at_submission(
            &mut TheArrangementsKeeper { record: &record },
            true,
            chose,
        )
        .expect_err("nothing is made");
        assert!(matches!(
            why,
            WhyNoArrangementWasMade::ThereIsNowhereToPutIt(_)
        ));
    }

    // ────────────────────────────────────────────────────────────────────
    // `Download` W5 — the window rule, and the fault it makes impossible
    // ────────────────────────────────────────────────────────────────────

    /// A machine whose folder reading refuses, which is `File Browser` P1's
    /// first of three arriving as its own protocol status.
    struct AMachineThatRefusesTheListing;

    impl ReadsOneDirectoryOnTheMachine for AMachineThatRefusesTheListing {
        async fn read_one_directory(
            &mut self,
            _path: &[u8],
        ) -> Result<Vec<ADirectoryEntry>, SubsystemFailure> {
            Err(SubsystemFailure::TheServerRefused(
                smoozy_server_protocol::subsystem::StatusCode::PermissionDenied,
            ))
        }
    }

    /// A machine that lists one folder with nothing in it, which is a
    /// **complete** short reading and is separable from a cut one by status
    /// (Fact 7).
    struct AMachineWithAnEmptyFolder;

    impl ReadsOneDirectoryOnTheMachine for AMachineWithAnEmptyFolder {
        async fn read_one_directory(
            &mut self,
            _path: &[u8],
        ) -> Result<Vec<ADirectoryEntry>, SubsystemFailure> {
            Ok(Vec::new())
        }
    }

    fn an_arrangement_with_frames(job: &Identity, server_row: &Identity) -> AnArrangement {
        AnArrangement::switched_on(
            job.clone(),
            server_row.clone(),
            b"/Users/artist/Renders".to_vec(),
            TheNarrowing::Everything,
        )
    }

    fn three_frames() -> Vec<ANarrowedEntry> {
        (1..=3)
            .map(|frame| ANarrowedEntry {
                predicted_path: format!("frame_{frame:04}.png").into_bytes(),
                frame: Some(frame),
            })
            .collect()
    }

    fn what_is_in_hand<'a>(
        folder: &'a [u8],
        entries: &'a [ANarrowedEntry],
        order: &'a TheOrder,
        excluded: &'a [ANarrowedEntry],
    ) -> WhatIsInHand<'a> {
        WhatIsInHand {
            the_job_output_folder: folder,
            narrowed_entries: entries,
            the_whole_may_grow: false,
            the_entries_the_narrowing_excludes: excluded,
            order,
            the_jobs_label: None,
            the_first_frame_pair: None,
            the_pairs_other_half_fetched: &[],
            window: TheWindow {
                the_desired_state_is_on: true,
                a_connection_to_the_server_stands: true,
                the_attempts_window_is_open: true,
            },
            the_placed_copies: None,
            the_four_values_of_the_placed_copies: &[],
            arrangements_walking_this_server: 1,
        }
    }

    /// **`Download` W5, and this is the fault the piece was told to make
    /// impossible.**
    ///
    /// *"A walk that did not complete is not a reading."* **An arrangement that
    /// applied a refused listing as its ending walk would mark every frame of
    /// the job as never coming** — the job would read as having produced
    /// nothing, on a machine holding every frame. So the ending walk is retried
    /// at the cadence, the arrangement is not done in the meantime, and the
    /// reckoning is untouched.
    ///
    /// **The second half of the test is what makes the first half mean
    /// anything**: the same arrangement, given a walk that *did* complete and
    /// found the folder empty, marks the frames *never coming* — so the refusal
    /// is being told apart from an empty folder rather than nothing being
    /// applied at all.
    #[tokio::test]
    async fn an_arrangement_that_meets_a_refused_listing_marks_no_frame_never_coming() {
        let record = ARecordForTheseTests::fresh();
        let rules = TheReceivingSystemsRulesHere;
        let job = an_identity(IdentityKind::Job);
        let server_row = an_identity(IdentityKind::LocalServer);
        let entries = three_frames();
        let order = TheOrder::ItCouldNotBeRead;
        let folder = b"/smoozy/jobs/a/outputs".to_vec();

        let mut arrangement = an_arrangement_with_frames(&job, &server_row);
        let mut running = TheArrangementsRun::fresh();
        let mut queue = TheFetchQueue::at_a_server();
        running.a_connection_began();
        running.the_ending_arrived(smoozy_queue_history::ending::Ending::Finished);

        // **A refused listing, twice**, because the retry is at the cadence and
        // a second refusal must change nothing either.
        for _ in 0..2 {
            let came = running
                .one_turn(
                    &mut AMachineThatRefusesTheListing,
                    &rules,
                    &mut TheArrangementsKeeper { record: &record },
                    &mut queue,
                    &mut arrangement,
                    what_is_in_hand(&folder, &entries, &order, &[]),
                )
                .await
                .expect("the turn ran");
            assert!(
                matches!(came, WhatTheTurnDid::TheWalkWasDiscarded { .. }),
                "a refused listing was taken as a reading: {came:?}"
            );
        }
        assert!(
            !running.the_ending_walk_has_completed(),
            "a discarded walk completed the ending walk"
        );
        assert!(
            arrangement
                .reckoning
                .iter()
                .all(|entry| entry.account.state != WhatSmoozyDid::NeverComing),
            "a refused listing marked a frame as never coming: {:?}",
            arrangement.reckoning
        );

        // And the same arrangement, on a walk that completed, does mark them.
        let came = running
            .one_turn(
                &mut AMachineWithAnEmptyFolder,
                &rules,
                &mut TheArrangementsKeeper { record: &record },
                &mut queue,
                &mut arrangement,
                what_is_in_hand(&folder, &entries, &order, &[]),
            )
            .await
            .expect("the turn ran");
        assert!(matches!(came, WhatTheTurnDid::TheEndingWalkRan(_)));
        assert_eq!(
            arrangement
                .reckoning
                .iter()
                .filter(|entry| entry.account.state == WhatSmoozyDid::NeverComing)
                .count(),
            3,
            "a complete ending walk over an empty folder owes three never-coming entries"
        );

        // **And it is durable**, which is what the next run reads.
        let read = TheArrangementsKeeper { record: &record }
            .the_arrangement_for(&job)
            .unwrap()
            .expect("the arrangement is in the record");
        assert_eq!(read.reckoning.len(), 3);
    }

    // ────────────────────────────────────────────────────────────────────
    // R17 a third time, and the two routes staying two
    // ────────────────────────────────────────────────────────────────────

    /// **R17's second half, kept structurally: nothing in this crate holds a
    /// lent transfer channel in a field.**
    ///
    /// The desktop point's slot is already held to this by
    /// [`no_field_anywhere_in_this_crate_holds_an_admitted_channel`]; **the
    /// server's own seat is the other half**, and at a machine's limit of one it
    /// is the one a whole-request hold would take from every other transfer in
    /// both directions. A seat held anywhere but a local of the call that took
    /// it is that hold, whatever the code around it says it does.
    #[test]
    fn no_field_anywhere_in_this_crate_holds_a_lent_transfer_seat() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let mut holders = Vec::new();
        for (name, text) in every_rust_file_under(&root) {
            for line in text.lines() {
                let line = line.trim();
                if line.starts_with("//") {
                    continue;
                }
                // The shapes are joined at run time so this test's own lines
                // are not what it finds.
                let lent = "ALent";
                let channel = "Channel";
                let looks_like_a_field = [
                    format!(": {lent}{channel}"),
                    format!(": Option<{lent}{channel}"),
                    format!(": Vec<{lent}{channel}"),
                    format!(": Mutex<{lent}{channel}"),
                ]
                .iter()
                .any(|shape| line.contains(shape.as_str()));
                if looks_like_a_field {
                    holders.push(format!("{name}: {line}"));
                }
            }
        }
        assert!(
            holders.is_empty(),
            "a lent transfer seat is held in a field, which at a machine's limit of one is the \
             hold R17's turn-taking is about: {holders:?}"
        );
    }

    /// **The two routes are genuinely two, and this reads the file to say so.**
    ///
    /// The brief's own words: *both are yours and they are genuinely two
    /// routes, not one with a switch*. **Nothing of `Download` R1's arrangement
    /// appears in the manual route** — no reckoning, no window, no walk series,
    /// no arrangement record — because a manual download is one action the user
    /// confirmed and an arrangement is a standing intention.
    #[test]
    fn the_manual_route_holds_no_arrangement_anywhere_in_it() {
        let whole = include_str!("downloading.rs");
        let working = whole
            .split_once("#[cfg(test)]")
            .map(|(before, _)| before)
            .expect("this file has a test module");
        let manual = working
            .split_once("// A manual download, by hand and on its own.")
            .map(|(_, after)| after)
            .expect("the manual route has its own section");
        // The names are joined at run time for the same reason as above.
        for (holder, act) in [
            ("AnArrange", "ment"),
            ("TheArrangements", "Run"),
            ("TheArrangements", "Keeper"),
            ("the_arrangement_is_made_at_", "submission"),
        ] {
            let shape = format!("{holder}{act}");
            assert!(
                !manual.contains(shape.as_str()),
                "the manual route reaches for an arrangement ({shape}), and the two routes are \
                 two rather than one with a switch"
            );
        }
    }

    // ────────────────────────────────────────────────────────────────────
    // C3, C4, C5 and C7 — the receiving system's own rules
    // ────────────────────────────────────────────────────────────────────

    /// **C4's four outcomes over the system this test is running on.**
    ///
    /// The cases that hold on every system are driven here; the per-system ones
    /// are driven in the two tests below, each behind its own system.
    #[test]
    fn the_name_check_refuses_what_no_system_takes() {
        let rules = TheReceivingSystemsRulesHere;
        assert_eq!(
            rules.check(b"frame_0001.png"),
            TheNameCheckSaid::TheNameMayBeWrittenAsGiven
        );
        for refused in [b"a/b".as_ref(), b"a\0b".as_ref(), b"".as_ref()] {
            assert_eq!(
                rules.check(refused),
                TheNameCheckSaid::NotRepresentable,
                "{refused:?} was accepted"
            );
        }
        // **C5's bound is per component in this system's own unit**, and it is
        // the same number in both units.
        assert_eq!(rules.the_longest_component(), 255);
        let too_long = vec![b'a'; 256];
        assert_eq!(
            rules.check(&too_long),
            TheNameCheckSaid::TooLongInThisSystemsOwnUnit
        );
    }

    /// **Fact 24, which is C3's own reason for the moment the check runs:
    /// macOS refuses invalid UTF-8 outright while Linux permits any byte but
    /// `/` and NUL.**
    #[test]
    fn a_name_that_is_not_text_is_refused_on_a_mac_and_taken_on_linux() {
        let rules = TheReceivingSystemsRulesHere;
        let not_text = [b'f', 0xff, 0xfe, b'.', b'p', b'n', b'g'];
        let said = rules.check(&not_text);
        if cfg!(any(target_os = "macos", target_os = "windows")) {
            assert_eq!(said, TheNameCheckSaid::NotRepresentable);
        } else {
            assert_eq!(said, TheNameCheckSaid::TheNameMayBeWrittenAsGiven);
        }
    }

    /// **P8: a render output's name is never repaired, and the substitution is
    /// only ever P10's own composed name.**
    ///
    /// Substituting an artist's output name would be the silent transformation
    /// Row 303 forbids arriving by Smoozy's own hand.
    #[test]
    fn nothing_on_the_fetch_path_substitutes_a_name() {
        let working = include_str!("downloading.rs")
            .split_once("#[cfg(test)]")
            .map(|(before, _)| before)
            .expect("this file has a test module");
        // **A call and never the definition**: the rules type below has to
        // answer P10's caller inside the `Download` crate, and what this test
        // is about is whether anything on the fetch path *reaches* for it.
        let calls = [".sub", "stitute("].concat();
        assert!(
            !working.contains(calls.as_str()),
            "this file substitutes into a name, and P8 refuses per file rather than repairing"
        );
        // And the substitution itself still works, for P10's caller inside the
        // `Download` crate.
        let rules = TheReceivingSystemsRulesHere;
        assert_eq!(rules.substitute(b"a:b*c"), b"a-b-c".to_vec());
        assert_eq!(rules.substitute(b"trailing. "), b"trailing".to_vec());
    }

    /// **`Download` M8's composition is handed the names standing beside the
    /// request, never their whole paths** — step 10c's thirteenth-pass fix
    /// piece 3. The destination's listing answers whole paths; handed those,
    /// the composition matched nothing, so a second *Keep both* of the same
    /// file composed `name (2)` again and its create-intended publication
    /// refused it.
    #[test]
    fn the_names_standing_are_names_and_never_whole_paths() {
        struct TwoFilesStand;
        impl ReadsTheDestinationsListing for TwoFilesStand {
            fn what_is_standing_in(&mut self, the_folder: &[u8]) -> Vec<Vec<u8>> {
                [&b"a.blend"[..], &b"a (2).blend"[..]]
                    .iter()
                    .map(|name| [the_folder, b"/", name].concat())
                    .collect()
            }
        }
        let request = AManualRequest::confirmed(
            Identity::issue(IdentityKind::Operation).expect("random bytes"),
            Identity::issue(IdentityKind::LocalServer).expect("random bytes"),
            None,
            b"/Users/artist/Downloads".to_vec(),
            Vec::new(),
            Identity::issue(IdentityKind::Run).expect("random bytes"),
        );
        let standing = the_names_standing_beside(&mut TwoFilesStand, &request);
        assert_eq!(standing, vec![b"a.blend".to_vec(), b"a (2).blend".to_vec()]);
        assert_eq!(
            smoozy_download::request::the_kept_both_name(
                &TheReceivingSystemsRulesHere,
                b"a.blend",
                false,
                &standing
            ),
            Some(b"a (3).blend".to_vec())
        );
    }

    /// **Keep both's landing folders move with the folder it renamed, an empty
    /// one too** (`Download` M8, M9) — step 10c's Vast-pass fix piece 1. A
    /// folder beside it that was not renamed does not move.
    #[test]
    fn keep_boths_landing_folders_move_with_the_folder_it_renamed() {
        let mut request = AManualRequest::confirmed(
            Identity::issue(IdentityKind::Operation).expect("random bytes"),
            Identity::issue(IdentityKind::LocalServer).expect("random bytes"),
            None,
            b"/Users/artist/Downloads".to_vec(),
            vec![
                smoozy_download::request::EnumerationEntry::enumerated(
                    b"/jobs/abc/frames/0001.png".to_vec(),
                    b"/Users/artist/Downloads/abc night/frames/0001.png".to_vec(),
                    3,
                    smoozy_download::vouching::AVouchingMark::NoClaimIsMade,
                ),
                smoozy_download::request::EnumerationEntry::enumerated(
                    b"/other/a.png".to_vec(),
                    b"/Users/artist/Downloads/other/a.png".to_vec(),
                    3,
                    smoozy_download::vouching::AVouchingMark::NoClaimIsMade,
                ),
            ],
            Identity::issue(IdentityKind::Run).expect("random bytes"),
        );
        let before: Vec<Vec<u8>> = request.entries.iter().map(|entry| entry.landing_path.clone()).collect();
        request.entries[0].landing_path = b"/Users/artist/Downloads/abc night (2)/frames/0001.png".to_vec();
        let folders = [
            b"/Users/artist/Downloads/abc night".to_vec(),
            b"/Users/artist/Downloads/abc night/frames".to_vec(),
            b"/Users/artist/Downloads/abc night/empty".to_vec(),
            b"/Users/artist/Downloads/other".to_vec(),
        ];
        assert_eq!(
            the_landing_folders_keep_both_moved(&folders, &before, &request),
            vec![
                b"/Users/artist/Downloads/abc night (2)".to_vec(),
                b"/Users/artist/Downloads/abc night (2)/frames".to_vec(),
                b"/Users/artist/Downloads/abc night (2)/empty".to_vec(),
            ]
        );
    }

    /// The kind a listing may not carry is never read as *this is not a
    /// folder*, which is `Download`'s own route rule; this holds the value is
    /// still reachable from here so the walk's reader keeps it.
    #[test]
    fn an_entrys_kind_that_was_not_carried_is_a_value() {
        assert_eq!(TheEntrysKind::ItWasNotCarried.is_a_folder(), None);
    }

    // ═══════════════════════════════════════════════════════════════════════
    // Step 10c's transfer wave piece 3 — the waits in the fetch path that had
    // no deadline (the tenth-pass map's piece 1, item (iii))
    // ═══════════════════════════════════════════════════════════════════════

    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    use smoozy_server_protocol::subsystem::wire::kind;
    use smoozy_server_protocol::transport::{CommandResult, CommandString};
    use tokio::io::{AsyncReadExt, AsyncWriteExt, DuplexStream};

    use crate::servers::sessions::{lend_until_it_ends, NoSession, TheAdditionalConnection};

    /// **How long a test lets a bounded wait take before calling it
    /// unbounded.** Far above the short deadlines these tests hand in, and far
    /// below anything a person would sit through.
    const THE_TESTS_OWN_BOUND: Duration = Duration::from_secs(10);

    /// **A machine whose file-transfer server goes quiet** — either before it
    /// answers the opening, or straight after. **Nothing it is asked after
    /// that is ever answered**, and it reads on until the desktop's end of
    /// the session goes.
    struct AMachineThatGoesQuiet {
        it_answers_the_opening: bool,
        sessions_opened: Arc<AtomicUsize>,
    }

    impl AHeldConnection for AMachineThatGoesQuiet {
        type Channel = DuplexStream;

        async fn open_a_session(&mut self) -> Result<DuplexStream, NoSession> {
            let (ours, mut theirs) = tokio::io::duplex(64 * 1024);
            self.sessions_opened.fetch_add(1, Ordering::SeqCst);
            let it_answers_the_opening = self.it_answers_the_opening;
            tokio::spawn(async move {
                let mut length = [0u8; 4];
                if theirs.read_exact(&mut length).await.is_err() {
                    return;
                }
                let mut opening = vec![0u8; u32::from_be_bytes(length) as usize];
                if theirs.read_exact(&mut opening).await.is_err() {
                    return;
                }
                if it_answers_the_opening {
                    // **Revision 3, nothing announced** — the shape the
                    // desktop's own stand-ins answer the opening with.
                    let mut version = 5u32.to_be_bytes().to_vec();
                    version.push(kind::VERSION);
                    version.extend(3u32.to_be_bytes());
                    if theirs.write_all(&version).await.is_err() {
                        return;
                    }
                }
                let mut sink = [0u8; 1024];
                while matches!(theirs.read(&mut sink).await, Ok(read) if read > 0) {}
            });
            Ok(ours)
        }

        /// **Nothing here asks for a command.**
        async fn a_command(
            &mut self,
            _command: CommandString,
            _frame: Option<Vec<u8>>,
        ) -> Result<CommandResult, NoSession> {
            Err(NoSession::Refused)
        }

        async fn until_it_ends(&mut self) -> smoozy_server_protocol::transport::Ending {
            std::future::pending().await
        }

        async fn torn_down(self) -> smoozy_server_protocol::transport::Ending {
            smoozy_server_protocol::transport::Ending::Announced
        }
    }

    /// **A point lending one connection to a machine that goes quiet**, on a
    /// task of its own, and how many sessions the machine has opened.
    async fn a_point_to_a_quiet_machine(
        it_answers_the_opening: bool,
    ) -> (
        ThisServersSessions<AMachineThatGoesQuiet>,
        Arc<AtomicUsize>,
        tokio::task::JoinHandle<()>,
    ) {
        let opened = Arc::new(AtomicUsize::new(0));
        let mut machine = AMachineThatGoesQuiet {
            it_answers_the_opening,
            sessions_opened: Arc::clone(&opened),
        };
        let sessions: ThisServersSessions<AMachineThatGoesQuiet> =
            ThisServersSessions::for_one_server();
        let nothing_more: TheAdditionalConnection<AMachineThatGoesQuiet> =
            Arc::new(|| Box::pin(async { None }));
        let lending = tokio::spawn({
            let sessions = sessions.clone();
            async move {
                let _ = lend_until_it_ends(&sessions, &mut machine, &nothing_more).await;
            }
        });
        let began = std::time::Instant::now();
        while !sessions.a_connection_is_live() {
            assert!(began.elapsed() < THE_TESTS_OWN_BOUND, "the point was never lent");
            tokio::task::yield_now().await;
        }
        (sessions, opened, lending)
    }

    /// **`Server protocol and transfers` C19 on the size a fetch is arranged
    /// against: a machine that answers the opening and then never answers the
    /// attribute request** — the fetch gets nothing inside the deadline, and
    /// the server's point is given back, so the next reading is served on a
    /// fresh session. **Before this piece the request had no deadline**, and
    /// it held the fetch, the pass and every reading, command and seat the
    /// point serves, for as long as the channel stayed open.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_size_the_machine_never_answers_gives_the_fetch_and_the_point_back() {
        let (sessions, opened, lending) = a_point_to_a_quiet_machine(true).await;
        let deadline = Duration::from_millis(300);
        let first = tokio::time::timeout(
            THE_TESTS_OWN_BOUND,
            a_size_read_within(&sessions, b"/job/out/0001.png".to_vec(), deadline),
        )
        .await;
        assert_eq!(
            first,
            Ok(None),
            "**a size the machine never answered held the fetch past its deadline**"
        );
        let second = tokio::time::timeout(
            THE_TESTS_OWN_BOUND,
            a_size_read_within(&sessions, b"/job/out/0002.png".to_vec(), deadline),
        )
        .await;
        assert_eq!(second, Ok(None));
        assert_eq!(
            opened.load(Ordering::SeqCst),
            2,
            "**the server's point was still held by the first attribute request**, so the \
             second reading was never served"
        );
        lending.abort();
    }

    /// **The same, where the machine never answers the reading session's own
    /// opening** — which is the point's own wait (`crate::servers::sessions`),
    /// so the fetch's wait is the one bounded here.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_size_behind_an_opening_the_machine_never_answers_gives_the_fetch_back() {
        let (sessions, _opened, lending) = a_point_to_a_quiet_machine(false).await;
        let asked = tokio::time::timeout(
            THE_TESTS_OWN_BOUND,
            a_size_read_within(&sessions, b"/job/out/0001.png".to_vec(), Duration::from_millis(300)),
        )
        .await;
        assert_eq!(
            asked,
            Ok(None),
            "**the fetch waited past its deadline for a reading session that never opened**"
        );
        lending.abort();
    }

    /// **`Server protocol and transfers` C19 on a transfer channel's own
    /// opening: a far end that never answers it** gives the seat back inside
    /// the deadline, and the file never started. Before this piece the
    /// negotiation had no deadline.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_transfer_channel_whose_opening_is_never_answered_gives_its_seat_back() {
        let (ours, _theirs) = tokio::io::duplex(64 * 1024);
        let negotiated = tokio::time::timeout(
            THE_TESTS_OWN_BOUND,
            negotiated_within(ours, Duration::from_millis(300)),
        )
        .await;
        assert!(
            matches!(negotiated, Ok(None)),
            "**the negotiation waited past its deadline on a far end that never answered**"
        );
    }
}

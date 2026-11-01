//! **`Queue & History` §15's desktop half: the fetch, the holding, the age
//! sweep and the picture's bytes** — the 38's piece 3, desktop half
//! (2026-10-03).
//!
//! **The machine half makes the pictures** (`Rented-server runtime — program`'s
//! preview worker) and journals *a preview was published* about the job for
//! each one (`Queue & History` P3). **This file is what the desktop does with
//! that event**, and every rule it keeps is a clause already written:
//!
//! - **P3: the fetch triggers on the journalled event.** The read forward —
//!   §7 step 3 on a connection and P10d's pass after it, the one mechanism
//!   (`Queue & History` C1a) — puts a fetch on the job through
//!   [`the_published_are_owed`], durably, before the cursor moves past the
//!   event (`State, identity and recovery` E3). **P5**: catching up is the same
//!   trigger on the next connection.
//! - **P2 and P4: one fetch per derivative, newest first, one in flight.**
//!   [`the_owed_are_fetched`], on a thread of its own beside step 7 and P10d
//!   and never in front of either ([`are_fetched_beside`]), takes the newest
//!   owed through the built `TheLivePreviewsOneFetch`, arranges it through the
//!   built `arrange_one_derivatives_fetch`, and carries it through the transfer
//!   contract at P6's class ([`crate::downloading::a_derivative_comes_down_over`]).
//! - **P4 and `State, identity and recovery` R13: the copy is held on the job
//!   record with one moment** — when it was published here, by this computer's
//!   clock ([`crate::record::previews`]).
//! - **`State, identity and recovery` R18 to `State, identity and recovery` R21: the age sweep** — [`the_age_sweep`], at application start
//!   and whenever a fetch publishes a copy, over `smoozy_state::lifetimes`'
//!   own comparison. **`Queue & History` P4a**: nothing re-fetches a cleared copy, by
//!   construction — the cursor has passed its event.
//! - **`Desktop app and delivery` H32: the picture reaches the page as bytes**
//!   ([`the_pictures_bytes`]) and the page makes a `blob:` address of them;
//!   no path is ever made a web-view address.
//! - **T15 to T17: the finished notification carries the held copy**
//!   ([`the_picture_held_for`]), or none where none is held yet.
//!
//! **Nothing here makes a picture** (T15), walks a folder, or reads a file's
//! presence as state (`State, identity and recovery` §1.3 rule 5): what is
//! owed and what is held are records Smoozy wrote.
//!
//! **Markers of other documents are written out with that document at every
//! site.** A bare marker means a clause of `Preparation/Architecture/Desktop
//! app and delivery.md`.

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::{Arc, Mutex, OnceLock};
use std::time::SystemTime;

use tauri::{AppHandle, Manager, Runtime};

use smoozy_queue_history::acting::previews::{
    arrange_one_derivatives_fetch, the_fetches_the_read_forward_queues, ADerivativeWasPublished,
    TheLivePreviewsOneFetch, WhatTheLivePreviewDid,
};
use smoozy_queue_history::previews::ThePublishedPreview;
use smoozy_server_protocol::requests::THE_PREVIEW_PUBLISHED_EVENT;
use smoozy_server_protocol::transfers::{Ending, TransferRecord};
use smoozy_state::identities::{Identity, IdentityKind};
use smoozy_state::lifetimes::{a_pass, FetchedDerivative, SweepMoment};

use crate::keys::LendsTheRecord;
use crate::record::job_records::JobRecords;
use crate::record::previews::{self, AFrameOfTheSet, WhatThePublicationDid};
use crate::record::RecordFailure;
use crate::servers::sessions::ThisServersSessions;
use crate::servers::the_machines_work::{with_the_record, AnEventRead};
use crate::start::Started;

// ════════════════════════════════════════════════════════════════════════════
// `Queue & History` P3: what the read forward applies
// ════════════════════════════════════════════════════════════════════════════

/// **`Queue & History` P3 and P5: every *a preview was published* this read
/// carried, about a job of this server the record holds, owes a fetch** —
/// written in the read's own application, before the cursor moves.
///
/// **An event whose words will not read back is applied as nothing**, as a
/// kind this build does not know is (`State, identity and recovery` E4): the
/// picture is one this desktop cannot name. **A job of another server, or one
/// whose record is gone, owes nothing.** Returns how many fetches were newly
/// owed; an error is a write the record would not take, and the caller moves
/// no cursor past it.
pub fn the_published_are_owed(
    connection: &rusqlite::Connection,
    server: &Identity,
    events: &[AnEventRead],
) -> Result<usize, RecordFailure> {
    let records = JobRecords::over(connection);
    let mut applied: Vec<ADerivativeWasPublished> = Vec::new();
    let mut saids: Vec<ThePublishedPreview> = Vec::new();
    for event in events.iter().filter(|event| event.kind == THE_PREVIEW_PUBLISHED_EVENT) {
        let Some(job) = event
            .about
            .as_deref()
            .and_then(|text| Identity::read_back(IdentityKind::Job, text))
        else {
            continue;
        };
        let Some(said) = event.says.as_deref().and_then(ThePublishedPreview::read_back) else {
            continue;
        };
        let Some(derivative) = Identity::read_back(IdentityKind::Operation, &said.operation) else {
            continue;
        };
        if !matches!(records.read(&job)?, Some(held) if &held.server.local_server == server) {
            continue;
        }
        // **Another View Layer's copy whose name the event could not carry**
        // (`ThePublishedPreview::as_says` leaves a long name to the record)
        // has no key of its own here (P4), so it owes nothing rather than
        // taking the frame's unnamed place. The Creator's seventh notes,
        // note 4.
        if !said.main && said.view_layer.is_none() {
            continue;
        }
        applied.push(ADerivativeWasPublished {
            // **A copy carrying a source is no layer's**, so it is found as the
            // frame's main copy and never by its place (the Creator's tenth
            // notes, note 2).
            it_replaces_one_already_held: previews::the_copy_of(
                connection,
                &job,
                said.frame,
                if said.source.is_some() { None } else { Some(i64::from(said.place)) },
            )?
            .is_some(),
            job,
            derivative,
            frame: said.frame,
        });
        saids.push(said);
    }
    let mut owed = 0;
    // `the_fetches_the_read_forward_queues` hands back every one applied, in
    // order, so each keeps its own event's words.
    for (one, said) in the_fetches_the_read_forward_queues(&applied).iter().zip(&saids) {
        let operation = one.derivative.as_text();
        if previews::a_derivative_is_owed(
            connection,
            &one.job,
            one.frame,
            &operation,
            said.view_layer.as_deref(),
            said.main,
            i64::from(said.place),
            said.source.as_ref(),
        )? == previews::WhatTheEventPut::AFetchIsOwed
        {
            owed += 1;
        }
    }
    Ok(owed)
}

// ════════════════════════════════════════════════════════════════════════════
// `Queue & History` P2 and P4: the fetches
// ════════════════════════════════════════════════════════════════════════════

/// **What the machine has at a derivative's path.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatTheMachineHas {
    /// The file, this long.
    ItsSize(u64),
    /// **No such file**: the machine no longer holds the production.
    NotThere,
    /// No answer that says either.
    NoAnswer,
}

/// **The machine and the transfer contract, as a fetch reaches them.** One
/// implementation in the running application, over the server's own point;
/// the tests answer it from a script.
pub trait ReachesTheDerivatives {
    /// **One attribute read of the derivative's path**, nobody waiting on it
    /// (`Server protocol and transfers` §1.1 item 2) — the size the transfer
    /// record is arranged with.
    fn what_the_machine_has(&mut self, path: &str) -> WhatTheMachineHas;
    /// **The arranged fetch carried out**, and the ending the contract wrote;
    /// nothing where it never started.
    fn it_comes_down(&mut self, arranged: &mut TransferRecord) -> Option<Ending>;
    /// **A part no later fetch will take up is abandoned in the record** —
    /// `Server protocol and transfers` S28a — so the start's sweep removes its
    /// staging. A fresh fetch is always a new transfer.
    fn its_part_is_abandoned(&mut self, arranged: &TransferRecord);
}

/// **Where the copies land, and what every fetch of this server is arranged
/// under.**
#[derive(Debug, Clone)]
pub struct WhereTheyLand {
    /// `Desktop app and delivery`'s fetched-derivative area.
    pub area: PathBuf,
    pub server: Identity,
    /// The installation every fetch of this connection is arranged under —
    /// the one step 7 confirmed.
    pub installation: Identity,
    pub run: Identity,
    pub layout: smoozy_job_files::layout::TheLayout,
}

/// **What one pass did.**
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct WhatThePassDid {
    /// Copies published and held.
    pub published: usize,
    /// Debts settled with nothing: the machine no longer holds the production.
    pub settled: usize,
    /// Fetches still owed after this pass.
    pub still_owed: usize,
    /// **Each derivative asked for in this sequence**, by job, frame and
    /// production, so this sequence asks for each at most once.
    pub asked: Vec<String>,
}

/// One fetch's key: the job, the frame, the copy's place, whether it is the
/// main copy, and its production — so a frame's View Layers' copies, which
/// share one production, are each asked for once. **The main copy is told
/// apart from the first layer's own copy**, both being at place nought where
/// the main copy is no layer's (the Creator's tenth notes, note 2).
fn the_key_of(one: &AFrameOfTheSet) -> String {
    format!(
        "{} {} {} {} {}",
        one.job.as_text(),
        one.frame,
        one.place,
        if one.the_main_copy { "main" } else { "layer" },
        one.operation
    )
}

/// **A name issued for the copy alone**, so no field of the record is ever
/// placed in a filename (L4).
fn a_fresh_name() -> Option<String> {
    Identity::issue(IdentityKind::Transfer)
        .ok()
        .map(|name| format!("{}.png", name.as_text()))
}

/// **`Queue & History` P2 and P4, performed for one server**: every owed
/// derivative not in `passed_over`, newest frame first, **one in flight at a
/// time** (`TheLivePreviewsOneFetch`), each arranged and carried through the
/// transfer contract and held on its job record when it publishes.
///
/// **A derivative is asked for once per pass**, so a machine that cannot
/// answer costs one ask, and the debt stands for the next live pass; one the
/// machine no longer holds settles with nothing. **After every publication the
/// age sweep runs** (`State, identity and recovery` R18's second moment).
pub fn the_owed_are_fetched(
    record: &dyn LendsTheRecord,
    reaches: &mut dyn ReachesTheDerivatives,
    lands: &WhereTheyLand,
    passed_over: &HashSet<String>,
) -> WhatThePassDid {
    let mut did = WhatThePassDid::default();
    let mut flight = TheLivePreviewsOneFetch::default();
    loop {
        let owed: Vec<AFrameOfTheSet> = with_the_record(record, |connection| {
            previews::the_owed_on(connection, &lands.server).unwrap_or_default()
        })
        .unwrap_or_default()
        .into_iter()
        .filter(|one| {
            let key = the_key_of(one);
            !passed_over.contains(&key) && !did.asked.contains(&key)
        })
        .collect();
        let newest_first: Vec<Identity> = owed
            .iter()
            .filter_map(|one| Identity::read_back(IdentityKind::Operation, &one.operation))
            .collect();
        let taken = match flight.admit(&newest_first) {
            WhatTheLivePreviewDid::ItTook(taken) => taken,
            WhatTheLivePreviewDid::TheSetHoldsNothing | WhatTheLivePreviewDid::OneIsAlreadyInFlight => break,
        };
        // **The first owed row of that production** — `the_owed_on`'s order puts
        // a frame's main copy before its other View Layers' (P10).
        let Some(one) = owed.iter().find(|one| one.operation == taken.as_text()).cloned() else {
            break;
        };
        did.asked.push(the_key_of(&one));
        let came_to = one_derivative_comes_down(record, reaches, lands, &one);
        flight.it_ended();
        match came_to {
            WhatOneFetchCameTo::Published => did.published += 1,
            WhatOneFetchCameTo::Settled => did.settled += 1,
            WhatOneFetchCameTo::StillOwed => did.still_owed += 1,
        }
    }
    did
}

enum WhatOneFetchCameTo {
    Published,
    Settled,
    StillOwed,
}

fn one_derivative_comes_down(
    record: &dyn LendsTheRecord,
    reaches: &mut dyn ReachesTheDerivatives,
    lands: &WhereTheyLand,
    one: &AFrameOfTheSet,
) -> WhatOneFetchCameTo {
    // **Where the machine wrote it**: `Job files and outputs` B29 and `Job files and outputs` B30's
    // one composition, from the job's own paths.
    let label = with_the_record(record, |connection| {
        JobRecords::over(connection).read(&one.job).ok().flatten()
    })
    .flatten()
    .and_then(|held| held.label);
    let Ok(paths) = smoozy_job_files::publication::AJobsPaths::composed(&lands.layout, &one.job, label.as_deref())
    else {
        return WhatOneFetchCameTo::StillOwed;
    };
    // **The main copy at the frame's own name, another View Layer's at its
    // place** (`Queue & History` P4, the Creator's seventh notes, note 4).
    let on_the_machine = smoozy_job_files::main_output::where_this_copy_is_written_in(
        &paths.job_record_directory,
        one.frame,
        one.the_main_copy,
        u32::try_from(one.place).unwrap_or(0),
    )
    .published_at;
    let size = match reaches.what_the_machine_has(&on_the_machine) {
        WhatTheMachineHas::ItsSize(size) => size,
        WhatTheMachineHas::NotThere => {
            let _ = with_the_record(record, |connection| {
                previews::the_production_is_gone(
                    connection,
                    &one.job,
                    one.frame,
                    one.view_layer.as_deref(),
                    &one.operation,
                )
            });
            return WhatOneFetchCameTo::Settled;
        }
        WhatTheMachineHas::NoAnswer => return WhatOneFetchCameTo::StillOwed,
    };

    let (Some(name), Ok(transfer)) = (a_fresh_name(), Identity::issue(IdentityKind::Transfer)) else {
        return WhatOneFetchCameTo::StillOwed;
    };
    if std::fs::create_dir_all(&lands.area).is_err() {
        return WhatOneFetchCameTo::StillOwed;
    }
    let landing = lands.area.join(name);
    let staging =
        lands.area.join(smoozy_file_browser::partial::THE_RESERVED_STAGING_FOLDER);
    let (Some(landing_text), Some(staging_text)) = (landing.to_str(), staging.to_str()) else {
        return WhatOneFetchCameTo::StillOwed;
    };
    let mut arranged = arrange_one_derivatives_fetch(
        transfer,
        lands.server.clone(),
        lands.installation.clone(),
        lands.run.clone(),
        &on_the_machine,
        landing_text,
        staging_text,
        size,
    );
    match reaches.it_comes_down(&mut arranged) {
        Some(Ending::Published) => {}
        Some(Ending::Interrupted) => {
            reaches.its_part_is_abandoned(&arranged);
            return WhatOneFetchCameTo::StillOwed;
        }
        _ => return WhatOneFetchCameTo::StillOwed,
    }

    // **`State, identity and recovery` R13's one moment: when the copy was published here, by this
    // computer's clock.**
    let landed = with_the_record(record, |connection| {
        previews::the_copy_was_published(
            connection,
            &one.job,
            one.frame,
            one.view_layer.as_deref(),
            &one.operation,
            &landing,
            SystemTime::now(),
        )
    });
    match landed {
        Some(Ok(WhatThePublicationDid::ItIsHeld { replaced })) => {
            if let Some(replaced) = replaced {
                previews::the_copies_go(&[replaced]);
            }
        }
        // **Nothing holds it** — the entry went while it was on its way — so
        // the copy outlives nothing.
        Some(Ok(WhatThePublicationDid::NothingHoldsIt)) => previews::the_copies_go(&[landing]),
        _ => return WhatOneFetchCameTo::StillOwed,
    }
    // **`State, identity and recovery` R18: a sweep whenever a preview fetch publishes a copy.**
    record.lend(&mut |connection| {
        the_age_sweep(connection, SweepMoment::APreviewFetchPublishedACopy, SystemTime::now());
    });
    WhatOneFetchCameTo::Published
}

// ════════════════════════════════════════════════════════════════════════════
// `State, identity and recovery` R18 to `State, identity and recovery` R21: the age sweep
// ════════════════════════════════════════════════════════════════════════════

/// **One pass of the age sweep**, at one of R18's two moments, judged against
/// one reading of this computer's clock (`State, identity and recovery` R19). **The copy alone** goes (`State, identity and recovery` R20):
/// the file, then the frame from the set — and **a copy whose removal did not
/// complete stays in the record**, to be met again at the next pass. Returns
/// how many were cleared.
pub fn the_age_sweep(connection: &rusqlite::Connection, at: SweepMoment, reading: SystemTime) -> usize {
    let held: Vec<FetchedDerivative<(Identity, i64, PathBuf)>> = previews::every_copy_held(connection)
        .unwrap_or_default()
        .into_iter()
        .filter_map(|one| {
            Some(FetchedDerivative {
                published_on_this_computer_at: one.published_on_this_computer_at?,
                copy: (one.job, one.frame, one.copy?),
            })
        })
        .collect();
    let mut cleared = 0;
    for (job, frame, copy) in a_pass(at, reading, &held) {
        let gone = match std::fs::remove_file(&copy) {
            Ok(()) => true,
            Err(why) => why.kind() == std::io::ErrorKind::NotFound,
        };
        if gone && previews::the_copy_is_cleared(connection, &job, frame, &copy).is_ok() {
            cleared += 1;
        }
    }
    cleared
}

// ════════════════════════════════════════════════════════════════════════════
// What the entry and the notification read
// ════════════════════════════════════════════════════════════════════════════

/// **T15 and T16: the held copy a finished notification attaches** — the
/// entry's own picture, the newest frame held, as a file on disk and its
/// length. **None where none is held yet** (T17).
pub fn the_picture_held_for(
    connection: &rusqlite::Connection,
    job: &Identity,
) -> Option<crate::notifications::picture::TheFetchedDerivative> {
    let held = previews::the_newest_held(connection, job).ok().flatten()?;
    let path = held.copy?;
    let length_in_bytes = std::fs::metadata(&path).ok()?.len();
    Some(crate::notifications::picture::TheFetchedDerivative { path, length_in_bytes })
}

/// **H32: the picture's bytes, for the typed application service the page
/// asks** (`crate::surface::the_queue::the_preview_picture`). **Only a copy
/// the job record holds is read**, by the path the record holds; nothing the
/// page sends is made a path. **Empty where no copy is held or it would not
/// read**, and the page then draws no picture.
///
/// **`place` is the View Layer's place in the frame's set** (P4), as the
/// surface handed it; absent asks for the frame's main copy (P10).
pub fn the_pictures_bytes(connection: &rusqlite::Connection, job: &str, frame: i64, place: Option<i64>) -> Vec<u8> {
    Identity::read_back(IdentityKind::Job, job)
        .and_then(|job| previews::the_copy_of(connection, &job, frame, place).ok().flatten())
        .and_then(|held| held.copy)
        .and_then(|copy| std::fs::read(copy).ok())
        .unwrap_or_default()
}

// ════════════════════════════════════════════════════════════════════════════
// On the running application
// ════════════════════════════════════════════════════════════════════════════

/// **Each server's own state for this run**: whether a pass is in hand, and
/// what was asked for on which connection. Run-held, as the kept words' are.
#[derive(Default)]
struct ThisServersPasses {
    in_hand: bool,
    look_again: bool,
    connection: u64,
    asked: HashSet<String>,
}

impl ThisServersPasses {
    fn begin(&mut self, connection: u64) -> bool {
        if self.connection != connection {
            self.connection = connection;
            self.asked.clear();
        }
        if self.in_hand {
            self.look_again = true;
            return false;
        }
        // `Queue & History` P4: a new live pass retries failed catch-up copies.
        // Inside this sequence the set still prevents immediate retries.
        self.asked.clear();
        self.in_hand = true;
        self.look_again = false;
        true
    }
}

fn the_passes_this_run() -> &'static Mutex<HashMap<String, ThisServersPasses>> {
    static HELD: OnceLock<Mutex<HashMap<String, ThisServersPasses>>> = OnceLock::new();
    HELD.get_or_init(Default::default)
}

/// **The application's own record, as the fetches write it**: the run's
/// record, reached through the application's state.
struct TheApplicationsRecord<R: Runtime> {
    app: AppHandle<R>,
}

impl<R: Runtime> crate::downloading::TheRecordADownloadWrites for TheApplicationsRecord<R> {
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
}

/// **The machine and the contract over the server's own point**, the record
/// reached through `record`. **Public so the desk can drive the fetch over a
/// real line** (the dead-line desk coverage, 2026-10-19); the running
/// application hands it its own record, and nothing else differs.
pub struct OverThisServersPoint<D: crate::downloading::TheRecordADownloadWrites> {
    pub record: D,
    pub sessions: ThisServersSessions,
    pub the_run: Arc<crate::downloading::TheRunsTransfers>,
}

impl<D: crate::downloading::TheRecordADownloadWrites + Sync> ReachesTheDerivatives for OverThisServersPoint<D> {
    fn what_the_machine_has(&mut self, path: &str) -> WhatTheMachineHas {
        use smoozy_server_protocol::subsystem::{StatusCode, SubsystemFailure};
        let path = path.as_bytes().to_vec();
        let sessions = &self.sessions;
        let answered = crate::servers::the_machines_work::waited_for(|| async move {
            sessions
                .a_reading(false, move |subsystem| Box::pin(async move { subsystem.stat(&path).await }))
                .await
        });
        match answered {
            Ok(Ok(attributes)) => attributes
                .size
                .map_or(WhatTheMachineHas::NoAnswer, WhatTheMachineHas::ItsSize),
            Ok(Err(SubsystemFailure::TheServerRefused(StatusCode::NoSuchFile))) => WhatTheMachineHas::NotThere,
            _ => WhatTheMachineHas::NoAnswer,
        }
    }

    fn it_comes_down(&mut self, arranged: &mut TransferRecord) -> Option<Ending> {
        let (sessions, the_run, door) = (&self.sessions, &*self.the_run, &self.record);
        crate::servers::the_machines_work::waited_for(|| async move {
            crate::downloading::a_derivative_comes_down_over(sessions, the_run, door, arranged)
                .await
                .ok()
                .map(|came| came.ending)
        })
    }

    fn its_part_is_abandoned(&mut self, arranged: &TransferRecord) {
        let door = |act: &mut dyn FnMut(&rusqlite::Connection)| self.record.with_the_record(act);
        crate::downloading::a_part_is_abandoned(&door, &self.the_run, arranged);
    }
}

/// **Started beside step 7 and every P10d read forward** — `Servers` Q1 step 7,
/// and `Queue & History` P4's retry of owed fetches on each live pass even
/// when no new event was applied. Returns at
/// once; **a pass already in hand for this server is not doubled**, and is
/// told to look again before it ends, so a fetch owed meanwhile is not left
/// for the next connection.
pub fn are_fetched_beside<R: Runtime>(
    app: &AppHandle<R>,
    server: &Identity,
    connection: u64,
    sessions: &ThisServersSessions,
) {
    let server_text = server.as_text();
    {
        let Ok(mut passes) = the_passes_this_run().lock() else {
            return;
        };
        let this = passes.entry(server_text.clone()).or_default();
        if !this.begin(connection) {
            return;
        }
    }
    let (app, server, sessions) = (app.clone(), server.clone(), sessions.clone());
    let spawned = std::thread::Builder::new()
        .name("smoozy previews".to_string())
        .spawn(move || {
            the_passes(&app, &server, connection, sessions);
            if let Ok(mut passes) = the_passes_this_run().lock() {
                if let Some(this) = passes.get_mut(&server.as_text()) {
                    this.in_hand = false;
                }
            }
        });
    if spawned.is_err() {
        if let Ok(mut passes) = the_passes_this_run().lock() {
            if let Some(this) = passes.get_mut(&server_text) {
                this.in_hand = false;
            }
        }
    }
}

fn where_they_land<R: Runtime>(app: &AppHandle<R>, server: &Identity) -> Option<WhereTheyLand> {
    let started = app.try_state::<Mutex<Started>>()?;
    let record = crate::keys::TheRunsRecord {
        started: started.inner(),
    };
    let run = started.lock().ok()?.run.clone()?;
    let row = with_the_record(&record, |connection| {
        crate::record::server_rows::one(connection, &server.as_text()).ok().flatten()
    })
    .flatten()?;
    Some(WhereTheyLand {
        area: crate::platform::locations::resolve(crate::platform::locations::Location::FetchedDerivativeArea)
            .ok()?,
        server: server.clone(),
        installation: row
            .last_confirmed_installation_identity
            .as_deref()
            .and_then(|text| Identity::read_back(IdentityKind::Installation, text))?,
        run,
        layout: crate::servers::installation::the_layout_recorded_for(&row)?,
    })
}

fn the_passes<R: Runtime>(app: &AppHandle<R>, server: &Identity, connection: u64, sessions: ThisServersSessions) {
    let (Some(periods), Some(started), Some(the_run)) = (
        app.try_state::<Arc<crate::servers::a_period::ThePeriodsThisRun>>(),
        app.try_state::<Mutex<Started>>(),
        app.try_state::<Arc<crate::downloading::TheRunsTransfers>>(),
    ) else {
        return;
    };
    let Some(lands) = where_they_land(app, server) else {
        return;
    };
    let record = crate::keys::TheRunsRecord {
        started: started.inner(),
    };
    let server_text = server.as_text();
    let holds = periods.the_machines_work();
    let mut reaches = OverThisServersPoint {
        record: TheApplicationsRecord { app: app.clone() },
        sessions: sessions.clone(),
        the_run: the_run.inner().clone(),
    };
    loop {
        if !(sessions.a_connection_is_live() && holds.is_current(&server_text, connection)) {
            return;
        }
        let passed_over = the_passes_this_run()
            .lock()
            .ok()
            .and_then(|passes| passes.get(&server_text).map(|this| this.asked.clone()))
            .unwrap_or_default();
        let did = the_owed_are_fetched(&record, &mut reaches, &lands, &passed_over);
        let look_again = match the_passes_this_run().lock() {
            Ok(mut passes) => match passes.get_mut(&server_text) {
                Some(this) => {
                    this.asked.extend(did.asked.iter().cloned());
                    std::mem::take(&mut this.look_again)
                }
                None => false,
            },
            Err(_) => false,
        };
        if did.published > 0 || did.settled > 0 {
            crate::surface::the_queue::the_queue_is_told_again(app);
            crate::surface::the_history::the_history_is_told_again(app);
        }
        if did.asked.is_empty() && !look_again {
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::record::previews::tests::a_record;
    use smoozy_server_protocol::transfers::StoppedReason;
    use std::cell::RefCell;

    struct ARecordInHand(rusqlite::Connection);

    impl LendsTheRecord for ARecordInHand {
        fn lend(&self, act: &mut dyn FnMut(&rusqlite::Connection)) {
            act(&self.0)
        }
        fn write_a_diary_line(&self, _act: &smoozy_keys::diary::Act) {}
    }

    /// **A machine and a contract, scripted**: what each path holds, and what
    /// each fetch ends as — writing the bytes at the landing on *published*,
    /// as the contract's publication would.
    struct AMachineInHand {
        sizes: HashMap<String, WhatTheMachineHas>,
        endings: RefCell<Vec<Option<Ending>>>,
        fetched: Vec<TransferRecord>,
        abandoned: Vec<TransferRecord>,
    }

    impl AMachineInHand {
        fn holding(frames: &[(String, WhatTheMachineHas)]) -> Self {
            AMachineInHand {
                sizes: frames.iter().cloned().collect(),
                endings: RefCell::new(Vec::new()),
                fetched: Vec::new(),
                abandoned: Vec::new(),
            }
        }
    }

    impl ReachesTheDerivatives for AMachineInHand {
        fn what_the_machine_has(&mut self, path: &str) -> WhatTheMachineHas {
            self.sizes.get(path).copied().unwrap_or(WhatTheMachineHas::NoAnswer)
        }
        fn it_comes_down(&mut self, arranged: &mut TransferRecord) -> Option<Ending> {
            self.fetched.push(arranged.clone());
            let ending = if self.endings.borrow().is_empty() {
                Some(Ending::Published)
            } else {
                self.endings.borrow_mut().remove(0)
            };
            if ending == Some(Ending::Published) {
                std::fs::write(&arranged.final_path, b"png").unwrap();
            }
            ending
        }
        fn its_part_is_abandoned(&mut self, arranged: &TransferRecord) {
            self.abandoned.push(arranged.clone());
        }
    }

    fn an_area() -> PathBuf {
        std::env::temp_dir().join(format!(
            "smoozy-previews-{}",
            Identity::issue(IdentityKind::Operation).unwrap().as_text()
        ))
    }

    fn a_layout() -> smoozy_job_files::layout::TheLayout {
        smoozy_job_files::layout::TheLayout::of(
            smoozy_job_files::layout::MachineRoot::chosen("/ephemeral/smoozy").expect("a root"),
            smoozy_job_files::layout::WhereThePackageAreaSits::UnderTheRoot,
        )
    }

    struct AServerWithAJob {
        record: ARecordInHand,
        server: Identity,
        job: Identity,
        lands: WhereTheyLand,
    }

    fn a_server_with_a_job() -> AServerWithAJob {
        let connection = a_record();
        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        let made = crate::record::deletions::tests::a_record_in_history(&server, "Kitchen");
        JobRecords::over(&connection).write_the_provisional_record(&made).unwrap();
        let lands = WhereTheyLand {
            area: an_area(),
            server: server.clone(),
            installation: Identity::issue(IdentityKind::Installation).unwrap(),
            run: Identity::issue(IdentityKind::Run).unwrap(),
            layout: a_layout(),
        };
        AServerWithAJob {
            record: ARecordInHand(connection),
            server,
            job: made.job,
            lands,
        }
    }

    fn where_the_machine_wrote(job: &Identity, frame: i64, lands: &WhereTheyLand) -> String {
        let paths = smoozy_job_files::publication::AJobsPaths::composed(&lands.layout, job, Some("Kitchen")).unwrap();
        smoozy_job_files::main_output::where_a_derivative_is_written_in(&paths.job_record_directory, frame)
            .published_at
    }

    fn an_event(sequence: u64, job: &Identity, frame: i64, operation: &str, layer: Option<&[u8]>) -> AnEventRead {
        AnEventRead {
            sequence,
            kind: THE_PREVIEW_PUBLISHED_EVENT.to_string(),
            about: Some(job.as_text()),
            says: Some(
                ThePublishedPreview {
                    frame,
                    operation: operation.to_string(),
                    view_layer: layer.map(<[u8]>::to_vec),
                    main: true,
                    place: 0,
                    source: None,
                }
                .as_says(),
            ),
        }
    }

    /// Another View Layer's copy's event, as the supervisor journals it.
    fn another_layers_event(
        sequence: u64,
        job: &Identity,
        frame: i64,
        operation: &str,
        layer: Option<&[u8]>,
        place: u32,
    ) -> AnEventRead {
        AnEventRead {
            says: Some(
                ThePublishedPreview {
                    frame,
                    operation: operation.to_string(),
                    view_layer: layer.map(<[u8]>::to_vec),
                    main: false,
                    place,
                    source: None,
                }
                .as_says(),
            ),
            ..an_event(sequence, job, frame, operation, layer)
        }
    }

    /// **The Creator's seventh notes, note 4 — P4, P10: a two-layer frame's
    /// two copies are held on their own rows, keyed by View Layer; the main
    /// copy is fetched first, at the frame's own name, and is the only one the
    /// entry and the notification read; the other comes down behind it from
    /// its place's name and is read only when asked for by place.** A layer
    /// copy whose name the event could not carry owes nothing.
    #[test]
    fn a_frames_view_layers_are_held_apart_and_only_the_main_copy_is_fetched_live() {
        let at = a_server_with_a_job();
        let production = an_operation();
        let owed = the_published_are_owed(
            &at.record.0,
            &at.server,
            &[
                an_event(0, &at.job, 5, &production, Some(b"Box")),
                another_layers_event(1, &at.job, 5, &production, Some(b"Mattress"), 1),
                another_layers_event(2, &at.job, 5, &production, None, 2),
            ],
        )
        .unwrap();
        assert_eq!(owed, 2, "one fetch per named copy; an unnamed other layer owes nothing");
        let rows = previews::the_owed_on(&at.record.0, &at.server).unwrap();
        assert_eq!(rows.len(), 2);
        assert!(rows[0].the_main_copy, "the main copy is owed first");
        assert_eq!(rows[0].view_layer.as_deref(), Some(&b"Box"[..]));
        assert_eq!((rows[1].the_main_copy, rows[1].place), (false, 1));

        let paths = smoozy_job_files::publication::AJobsPaths::composed(&at.lands.layout, &at.job, Some("Kitchen")).unwrap();
        let the_others = smoozy_job_files::main_output::where_this_copy_is_written_in(&paths.job_record_directory, 5, false, 1)
            .published_at;
        let mut machine = AMachineInHand::holding(&[
            (where_the_machine_wrote(&at.job, 5, &at.lands), WhatTheMachineHas::ItsSize(3)),
            (the_others.clone(), WhatTheMachineHas::ItsSize(3)),
        ]);
        let did = the_owed_are_fetched(&at.record, &mut machine, &at.lands, &HashSet::new());
        assert_eq!(did.published, 2);
        assert_eq!(machine.fetched[0].source_path, where_the_machine_wrote(&at.job, 5, &at.lands), "the live fetch takes the main copy");
        assert_eq!(machine.fetched[1].source_path, the_others);

        let held = previews::every_copy_held_for(&at.record.0, &at.job).unwrap();
        assert_eq!(held.len(), 2, "two layers, two rows, neither replacing the other");
        let main = previews::the_newest_held(&at.record.0, &at.job).unwrap().unwrap();
        assert!(main.the_main_copy);
        assert_eq!(the_picture_held_for(&at.record.0, &at.job).unwrap().path, main.copy.clone().unwrap());
        let by_place = previews::the_copy_of(&at.record.0, &at.job, 5, Some(1)).unwrap().unwrap();
        assert_eq!(by_place.view_layer.as_deref(), Some(&b"Mattress"[..]));
        assert_ne!(by_place.copy, main.copy);
        assert_eq!(previews::the_copy_of(&at.record.0, &at.job, 5, None).unwrap().unwrap().copy, main.copy);
        std::fs::remove_dir_all(&at.lands.area).unwrap();
    }

    fn an_operation() -> String {
        Identity::issue(IdentityKind::Operation).unwrap().as_text()
    }

    /// **The Creator's shape, on the desktop** (the tenth notes, note 2): the
    /// main copy's event carries the output path as its source and the
    /// layers' copies come behind it, the first layer's at place nought too.
    /// **All three are owed and all three come down** — the main copy at the
    /// frame's own name, each layer at its place's — and the source is held
    /// first in the frame. **What it bites on**: the main copy and the first
    /// layer's copy taken for one fetch, both being at place nought; the
    /// source not kept from the event.
    #[test]
    fn tenth_notes_2_a_sourced_main_copy_and_the_first_layers_own_copy_both_come_down() {
        use smoozy_job_files::main_output::WhereThePictureCameFrom;
        let at = a_server_with_a_job();
        let production = an_operation();
        let main = AnEventRead {
            says: Some(
                ThePublishedPreview {
                    frame: 5,
                    operation: production.clone(),
                    view_layer: None,
                    main: true,
                    place: 0,
                    source: Some(WhereThePictureCameFrom::TheOutputPath),
                }
                .as_says(),
            ),
            ..an_event(0, &at.job, 5, &production, None)
        };
        let owed = the_published_are_owed(
            &at.record.0,
            &at.server,
            &[
                main,
                another_layers_event(1, &at.job, 5, &production, Some(b"Box"), 0),
                another_layers_event(2, &at.job, 5, &production, Some(b"Mattress"), 1),
            ],
        )
        .unwrap();
        assert_eq!(owed, 3, "the main copy and both layers");
        let paths = smoozy_job_files::publication::AJobsPaths::composed(&at.lands.layout, &at.job, Some("Kitchen")).unwrap();
        let a_layers = |place: u32| {
            smoozy_job_files::main_output::where_this_copy_is_written_in(&paths.job_record_directory, 5, false, place)
                .published_at
        };
        let mut machine = AMachineInHand::holding(&[
            (where_the_machine_wrote(&at.job, 5, &at.lands), WhatTheMachineHas::ItsSize(3)),
            (a_layers(0), WhatTheMachineHas::ItsSize(3)),
            (a_layers(1), WhatTheMachineHas::ItsSize(3)),
        ]);
        let did = the_owed_are_fetched(&at.record, &mut machine, &at.lands, &HashSet::new());
        assert_eq!(did.published, 3, "{did:?}");
        assert_eq!(machine.fetched[0].source_path, where_the_machine_wrote(&at.job, 5, &at.lands), "the main copy first");
        let held = previews::every_copy_held_for(&at.record.0, &at.job).unwrap();
        assert_eq!(
            held.iter().map(|one| (one.view_layer.clone(), one.source.clone())).collect::<Vec<_>>(),
            vec![
                (None, Some(WhereThePictureCameFrom::TheOutputPath)),
                (Some(b"Box".to_vec()), None),
                (Some(b"Mattress".to_vec()), None)
            ]
        );
        let box_copy = previews::the_copy_of(&at.record.0, &at.job, 5, Some(0)).unwrap().unwrap();
        assert_eq!(box_copy.view_layer.as_deref(), Some(&b"Box"[..]));
        std::fs::remove_dir_all(&at.lands.area).unwrap();
    }


    /// **P3 and E4: each published event about a job of this server owes one
    /// fetch, the same event again owes nothing, and an event about another
    /// server's job, or with words that will not read back, owes nothing.**
    #[test]
    fn a_published_event_owes_one_fetch_for_a_job_of_this_server() {
        let at = a_server_with_a_job();
        let one = an_operation();
        let events = vec![
            an_event(0, &at.job, 5, &one, Some(b"Reflections")),
            AnEventRead { says: Some("frame 6".into()), ..an_event(1, &at.job, 6, &an_operation(), None) },
        ];
        assert_eq!(the_published_are_owed(&at.record.0, &at.server, &events).unwrap(), 1);
        assert_eq!(the_published_are_owed(&at.record.0, &at.server, &events).unwrap(), 0);
        let elsewhere = Identity::issue(IdentityKind::LocalServer).unwrap();
        assert_eq!(the_published_are_owed(&at.record.0, &elsewhere, &[an_event(2, &at.job, 7, &an_operation(), None)]).unwrap(), 0);
        let owed = previews::the_owed_on(&at.record.0, &at.server).unwrap();
        assert_eq!(owed.len(), 1);
        assert_eq!(owed[0].frame, 5);
        assert_eq!(owed[0].view_layer.as_deref(), Some(&b"Reflections"[..]));
    }

    /// **P2 and P4: newest first, one fetch per derivative, arranged at the
    /// path the machine wrote, landing under a fresh name in the area, and
    /// held with its moment** — then a frame the machine no longer holds
    /// settles, and one it does not answer for stays owed and is not asked for
    /// again in the same pass.
    #[test]
    fn the_owed_come_down_newest_first_and_are_held() {
        let at = a_server_with_a_job();
        let (one, two, three) = (an_operation(), an_operation(), an_operation());
        the_published_are_owed(
            &at.record.0,
            &at.server,
            &[
                an_event(0, &at.job, 1, &one, None),
                an_event(1, &at.job, 2, &two, None),
                an_event(2, &at.job, 3, &three, None),
            ],
        )
        .unwrap();
        let mut machine = AMachineInHand::holding(&[
            (where_the_machine_wrote(&at.job, 3, &at.lands), WhatTheMachineHas::ItsSize(3)),
            (where_the_machine_wrote(&at.job, 2, &at.lands), WhatTheMachineHas::NotThere),
        ]);
        let did = the_owed_are_fetched(&at.record, &mut machine, &at.lands, &HashSet::new());
        assert_eq!((did.published, did.settled, did.still_owed), (1, 1, 1));
        assert_eq!(machine.fetched.len(), 1, "one fetch per derivative, and none for an absent one");
        let fetched = &machine.fetched[0];
        assert_eq!(fetched.source_path, where_the_machine_wrote(&at.job, 3, &at.lands));
        assert_eq!(fetched.total_size_expected, 3);
        assert!(fetched.final_path.starts_with(at.lands.area.to_str().unwrap()));
        assert!(!fetched.final_path.contains(&at.job.as_text()), "no field of the record is in the name (L4)");
        let held = previews::the_newest_held(&at.record.0, &at.job).unwrap().unwrap();
        assert_eq!(held.frame, 3);
        assert!(held.published_on_this_computer_at.is_some());
        let owed: Vec<i64> = previews::the_owed_on(&at.record.0, &at.server).unwrap().iter().map(|one| one.frame).collect();
        assert_eq!(owed, vec![1], "frame 2 settled with nothing; frame 1 still owed");
        assert_eq!(did.asked.len(), 3);

        // **The next pass on the same connection asks for nothing it asked.**
        let again = the_owed_are_fetched(&at.record, &mut machine, &at.lands, &did.asked.iter().cloned().collect());
        assert!(again.asked.is_empty());
        std::fs::remove_dir_all(&at.lands.area).unwrap();
    }

    #[test]
    fn seventeenth_reopening_recovers_every_layer_after_a_failed_copy() {
        let mut at = a_server_with_a_job();
        let mut events = Vec::new();
        let mut paths = Vec::new();
        for frame in 1..=3 {
            let operation = an_operation();
            events.push(an_event(events.len() as u64, &at.job, frame, &operation, Some(b"Beauty")));
            events.push(another_layers_event(events.len() as u64, &at.job, frame, &operation, Some(b"Mist"), 1));
            let job_paths = smoozy_job_files::publication::AJobsPaths::composed(
                &at.lands.layout, &at.job, Some("Kitchen"),
            ).unwrap();
            for main in [true, false] {
                paths.push((
                    smoozy_job_files::main_output::where_this_copy_is_written_in(
                        &job_paths.job_record_directory, frame, main, if main { 0 } else { 1 },
                    ).published_at,
                    WhatTheMachineHas::ItsSize(3),
                ));
            }
        }
        assert_eq!(the_published_are_owed(&at.record.0, &at.server, &events).unwrap(), 6);
        // Saved debts survive closing the record and opening it in a fresh run.
        std::fs::create_dir_all(&at.lands.area).unwrap();
        let saved = at.lands.area.join("record.sqlite");
        at.record.0.execute("VACUUM INTO ?1", [saved.to_str().unwrap()]).unwrap();
        at.record = ARecordInHand(rusqlite::Connection::open(&saved).unwrap());
        at.lands.run = Identity::issue(IdentityKind::Run).unwrap();
        let mut machine = AMachineInHand::holding(&paths);
        machine.endings = RefCell::new(vec![None]);
        let mut passes = ThisServersPasses::default();
        assert!(passes.begin(1));
        let first = the_owed_are_fetched(&at.record, &mut machine, &at.lands, &passes.asked);
        assert_eq!((first.published, first.still_owed), (5, 1));
        passes.asked.extend(first.asked);
        passes.in_hand = false;
        // The next live read carries no new event. It still owes the failed copy.
        assert!(passes.begin(1));
        let second = the_owed_are_fetched(&at.record, &mut machine, &at.lands, &passes.asked);
        assert_eq!(second.published, 1, "one failed copy must not wait for another connection");
        assert!(previews::the_owed_on(&at.record.0, &at.server).unwrap().is_empty());
        for frame in 1..=3 {
            for place in [0, 1] {
                assert_eq!(the_pictures_bytes(&at.record.0, &at.job.as_text(), frame, Some(place)), b"png");
            }
        }
        drop(at.record);
        std::fs::remove_dir_all(&at.lands.area).unwrap();
    }

    #[test]
    fn seventeenth_recovery_never_fetches_a_copy_cleared_by_age() {
        let at = a_server_with_a_job();
        let event = an_event(0, &at.job, 1, &an_operation(), None);
        the_published_are_owed(&at.record.0, &at.server, &[event.clone()]).unwrap();
        let mut machine = AMachineInHand::holding(&[
            (where_the_machine_wrote(&at.job, 1, &at.lands), WhatTheMachineHas::ItsSize(3)),
        ]);
        the_owed_are_fetched(&at.record, &mut machine, &at.lands, &HashSet::new());
        assert_eq!(the_age_sweep(
            &at.record.0, SweepMoment::ApplicationStart,
            SystemTime::now() + smoozy_state::lifetimes::PREVIEW_AGE_PERIOD + std::time::Duration::from_secs(1),
        ), 1);
        // The cursor stays past this event: recovery reads debts, never old events.
        let mut passes = ThisServersPasses::default();
        assert!(passes.begin(2));
        assert!(the_owed_are_fetched(&at.record, &mut machine, &at.lands, &passes.asked).asked.is_empty());
        assert_eq!(machine.fetched.len(), 1);
        assert!(the_pictures_bytes(&at.record.0, &at.job.as_text(), 1, None).is_empty());
        std::fs::remove_dir_all(&at.lands.area).unwrap();
    }

    /// **A fetch the line cut leaves the derivative owed and its part
    /// abandoned** (`Server protocol and transfers` S28a); one that ended any
    /// other way short of publication leaves it owed.
    #[test]
    fn a_cut_fetch_leaves_the_frame_owed_and_its_part_abandoned() {
        let at = a_server_with_a_job();
        the_published_are_owed(&at.record.0, &at.server, &[an_event(0, &at.job, 4, &an_operation(), None)]).unwrap();
        let mut machine =
            AMachineInHand::holding(&[(where_the_machine_wrote(&at.job, 4, &at.lands), WhatTheMachineHas::ItsSize(9))]);
        machine.endings = RefCell::new(vec![Some(Ending::Interrupted)]);
        let did = the_owed_are_fetched(&at.record, &mut machine, &at.lands, &HashSet::new());
        assert_eq!(did.still_owed, 1);
        assert_eq!(machine.abandoned.len(), 1);
        assert!(previews::the_newest_held(&at.record.0, &at.job).unwrap().is_none());

        machine.endings = RefCell::new(vec![Some(Ending::Stopped(StoppedReason::AbandonedByItsRequest))]);
        let did = the_owed_are_fetched(&at.record, &mut machine, &at.lands, &HashSet::new());
        assert_eq!(did.still_owed, 1);
        assert_eq!(machine.abandoned.len(), 1);
        let _ = std::fs::remove_dir_all(&at.lands.area);
    }

    /// **`Job files and outputs` B29 and `Queue & History` P4: a newer production of a held frame is fetched again and
    /// replaces the held copy**, which is taken off this computer.
    #[test]
    fn a_newer_production_replaces_the_held_copy() {
        let at = a_server_with_a_job();
        let path = where_the_machine_wrote(&at.job, 2, &at.lands);
        let mut machine = AMachineInHand::holding(&[(path, WhatTheMachineHas::ItsSize(3))]);
        the_published_are_owed(&at.record.0, &at.server, &[an_event(0, &at.job, 2, &an_operation(), None)]).unwrap();
        the_owed_are_fetched(&at.record, &mut machine, &at.lands, &HashSet::new());
        let first = previews::the_newest_held(&at.record.0, &at.job).unwrap().unwrap().copy.unwrap();
        the_published_are_owed(&at.record.0, &at.server, &[an_event(1, &at.job, 2, &an_operation(), None)]).unwrap();
        the_owed_are_fetched(&at.record, &mut machine, &at.lands, &HashSet::new());
        let second = previews::the_newest_held(&at.record.0, &at.job).unwrap().unwrap().copy.unwrap();
        assert_ne!(first, second);
        assert!(!first.exists(), "the replaced copy is gone");
        assert!(second.exists());
        std::fs::remove_dir_all(&at.lands.area).unwrap();
    }

    /// **`State, identity and recovery` R21, in the plan's own words**: a copy published thirty days and one
    /// second before the pass's reading is gone and the job record is whole;
    /// one published thirty days exactly before it stands. Both moments.
    #[test]
    fn thirty_days_and_one_second_is_cleared_and_thirty_days_exactly_stands() {
        use smoozy_state::lifetimes::PREVIEW_AGE_PERIOD;
        use std::time::Duration;
        for moment in [SweepMoment::ApplicationStart, SweepMoment::APreviewFetchPublishedACopy] {
            let at = a_server_with_a_job();
            std::fs::create_dir_all(&at.lands.area).unwrap();
            // **A reading on a whole millisecond**, the record's own spelling
            // of a moment, so *thirty days exactly* is exactly that.
            let reading = std::time::UNIX_EPOCH + Duration::from_millis(1_759_500_000_123);
            let old = at.lands.area.join("old.png");
            let kept = at.lands.area.join("kept.png");
            std::fs::write(&old, b"png").unwrap();
            std::fs::write(&kept, b"png").unwrap();
            for (frame, copy, ago) in [
                (1, &old, PREVIEW_AGE_PERIOD + Duration::from_secs(1)),
                (2, &kept, PREVIEW_AGE_PERIOD),
            ] {
                previews::a_derivative_is_owed(&at.record.0, &at.job, frame, "op", None, true, 0, None).unwrap();
                previews::the_copy_was_published(&at.record.0, &at.job, frame, None, "op", copy, reading - ago).unwrap();
            }
            assert_eq!(the_age_sweep(&at.record.0, moment, reading), 1);
            assert!(!old.exists());
            assert!(kept.exists());
            assert!(previews::the_copy_of(&at.record.0, &at.job, 1, None).unwrap().is_none());
            assert!(previews::the_copy_of(&at.record.0, &at.job, 2, None).unwrap().is_some());
            assert!(JobRecords::over(&at.record.0).read(&at.job).unwrap().is_some(), "the job record is whole");
            std::fs::remove_dir_all(&at.lands.area).unwrap();
        }
    }

    /// **T15 and T17: the notification attaches the newest copy held, and
    /// none where none is held yet.**
    #[test]
    fn the_notification_takes_the_newest_copy_held_or_none() {
        let at = a_server_with_a_job();
        assert!(the_picture_held_for(&at.record.0, &at.job).is_none());
        std::fs::create_dir_all(&at.lands.area).unwrap();
        let copy = at.lands.area.join("a.png");
        std::fs::write(&copy, b"12345").unwrap();
        previews::a_derivative_is_owed(&at.record.0, &at.job, 9, "op", None, true, 0, None).unwrap();
        previews::the_copy_was_published(&at.record.0, &at.job, 9, None, "op", &copy, SystemTime::now()).unwrap();
        let picture = the_picture_held_for(&at.record.0, &at.job).unwrap();
        assert_eq!(picture.path, copy);
        assert_eq!(picture.length_in_bytes, 5);
        std::fs::remove_dir_all(&at.lands.area).unwrap();
    }

    /// **`State, identity and recovery` R18's second moment: a fetch that publishes runs the sweep**, so a
    /// copy past its period goes the moment the held set grows.
    #[test]
    fn a_publication_runs_the_age_sweep() {
        use smoozy_state::lifetimes::PREVIEW_AGE_PERIOD;
        use std::time::Duration;
        let at = a_server_with_a_job();
        std::fs::create_dir_all(&at.lands.area).unwrap();
        let old = at.lands.area.join("old.png");
        std::fs::write(&old, b"png").unwrap();
        previews::a_derivative_is_owed(&at.record.0, &at.job, 1, "op", None, true, 0, None).unwrap();
        previews::the_copy_was_published(
            &at.record.0,
            &at.job,
            1,
            None,
            "op",
            &old,
            SystemTime::now() - PREVIEW_AGE_PERIOD - Duration::from_secs(60),
        )
        .unwrap();
        the_published_are_owed(&at.record.0, &at.server, &[an_event(0, &at.job, 2, &an_operation(), None)]).unwrap();
        let mut machine = AMachineInHand::holding(&[(
            where_the_machine_wrote(&at.job, 2, &at.lands),
            WhatTheMachineHas::ItsSize(3),
        )]);
        the_owed_are_fetched(&at.record, &mut machine, &at.lands, &HashSet::new());
        assert!(!old.exists(), "the old copy went at the publication");
        assert_eq!(previews::the_newest_held(&at.record.0, &at.job).unwrap().unwrap().frame, 2);
        std::fs::remove_dir_all(&at.lands.area).unwrap();
    }

    /// **`State, identity and recovery` R18's first moment: the start runs the sweep once the record is
    /// open.** Asserted over the start's own text, as the start's other acts
    /// are: the whole start is reached only in a running application.
    #[test]
    fn the_start_runs_the_age_sweep_once_the_record_is_open() {
        let working = include_str!("start.rs")
            .split("#[cfg(test)]")
            .next()
            .expect("the working half");
        let start = working.find("pub fn start(").expect("the start");
        let body = &working[start..];
        let body = &body[..body.find("\n}\n").expect("the end of the start")];
        let opened = body.find("LocalRecord::open()").expect("the record is opened");
        let swept = body
            .find("the_previews::the_age_sweep(")
            .expect("the start never runs the age sweep");
        assert!(opened < swept);
        assert!(body[swept..].contains("SweepMoment::ApplicationStart"));
    }

    /// **The Creator's nineteenth notes, note 10 — `Queue & History` P2 and
    /// `Queue & History` P4**: on one server, an ended first-frame check owes
    /// frame 1 while a running job owes frames 1 to 20 (with a second View
    /// Layer on frame 20). The running job's newest main copy comes down first
    /// (`Queue & History` P2), the ended check's frame 1 comes down next, and
    /// the running job's other layer and passed-over frames follow behind.
    #[test]
    fn nineteenth_notes_10_an_ended_checks_first_frame_comes_down_before_passed_over_frames() {
        use smoozy_queue_history::acting::note::{TheEndingOnTheNote, TheOutcomeAsWritten};

        let at = a_server_with_a_job();
        let mut check_record = crate::record::deletions::tests::a_record_in_history(&at.server, "Kitchen");
        check_record.what_the_note_carried.ending = Some(TheEndingOnTheNote {
            outcome: TheOutcomeAsWritten::Finished,
            exit_status: Some(0),
            the_attempt_whose_capture_record_holds_the_words: 1,
            the_reason_the_machine_recorded: None,
            the_device_line_as_observed: None,
        });
        check_record.what_the_note_carried.last_frame_figure = Some(1);
        let the_check = check_record.job.clone();
        JobRecords::over(&at.record.0)
            .write_the_provisional_record(&check_record)
            .unwrap();

        let mut events = vec![an_event(0, &the_check, 1, &an_operation(), Some(b"Beauty"))];
        let mut machine_files = vec![(
            where_the_machine_wrote(&the_check, 1, &at.lands),
            WhatTheMachineHas::ItsSize(3),
        )];
        for frame in 1..=20 {
            let production = an_operation();
            events.push(an_event(events.len() as u64, &at.job, frame, &production, Some(b"Beauty")));
            machine_files.push((
                where_the_machine_wrote(&at.job, frame, &at.lands),
                WhatTheMachineHas::ItsSize(3),
            ));
            if frame == 20 {
                events.push(another_layers_event(
                    events.len() as u64,
                    &at.job,
                    frame,
                    &production,
                    Some(b"Mist"),
                    1,
                ));
                let running_paths = smoozy_job_files::publication::AJobsPaths::composed(
                    &at.lands.layout,
                    &at.job,
                    Some("Kitchen"),
                )
                .unwrap();
                machine_files.push((
                    smoozy_job_files::main_output::where_this_copy_is_written_in(
                        &running_paths.job_record_directory,
                        20,
                        false,
                        1,
                    )
                    .published_at,
                    WhatTheMachineHas::ItsSize(3),
                ));
            }
        }
        assert_eq!(the_published_are_owed(&at.record.0, &at.server, &events).unwrap(), 22);

        let mut machine = AMachineInHand::holding(&machine_files);
        let did = the_owed_are_fetched(&at.record, &mut machine, &at.lands, &HashSet::new());
        assert_eq!(did.published, 22);

        let check_frame_1_path = where_the_machine_wrote(&the_check, 1, &at.lands);
        let running_frame_20_path = where_the_machine_wrote(&at.job, 20, &at.lands);
        let check_position = machine
            .fetched
            .iter()
            .position(|arranged| arranged.source_path == check_frame_1_path)
            .expect("the check's frame 1 was fetched");
        assert_eq!(
            machine.fetched[0].source_path, running_frame_20_path,
            "the running job's newest main picture still comes first"
        );
        assert_eq!(
            check_position,
            1,
            "the ended check's frame 1 was fetched at index {check_position} of {} instead of right after the running job's newest main picture",
            machine.fetched.len()
        );
        assert_eq!(previews::the_newest_held(&at.record.0, &the_check).unwrap().unwrap().frame, 1);
        std::fs::remove_dir_all(&at.lands.area).unwrap();

        // **The map's second question checked at the desk**: on a server where
        // nothing else renders after the check, a failed fetch of frame 1 is
        // retried on the next live pass without waiting for another picture
        // event or connection (`Queue & History` P4).
        let quiet = a_server_with_a_job();
        the_published_are_owed(
            &quiet.record.0,
            &quiet.server,
            &[an_event(0, &quiet.job, 1, &an_operation(), Some(b"Beauty"))],
        )
        .unwrap();
        let mut quiet_machine = AMachineInHand::holding(&[(
            where_the_machine_wrote(&quiet.job, 1, &quiet.lands),
            WhatTheMachineHas::ItsSize(3),
        )]);
        quiet_machine.endings = RefCell::new(vec![None]);
        let mut passes = ThisServersPasses::default();
        assert!(passes.begin(1));
        let first_try = the_owed_are_fetched(&quiet.record, &mut quiet_machine, &quiet.lands, &passes.asked);
        assert_eq!((first_try.published, first_try.still_owed), (0, 1));
        passes.asked.extend(first_try.asked);
        passes.in_hand = false;

        assert!(passes.begin(1));
        let retry_on_next_live_pass =
            the_owed_are_fetched(&quiet.record, &mut quiet_machine, &quiet.lands, &passes.asked);
        assert_eq!((retry_on_next_live_pass.published, retry_on_next_live_pass.still_owed), (1, 0));
        assert_eq!(previews::the_newest_held(&quiet.record.0, &quiet.job).unwrap().unwrap().frame, 1);
        std::fs::remove_dir_all(&quiet.lands.area).unwrap();
    }
}

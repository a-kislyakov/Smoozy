//! **`Servers` Q1 step 7: a connection reads the machine's work** — step 10c
//! piece 7.
//!
//! **What step 7 is, in the table's own words**: *"Desktop reconciliation
//! steps 2 to 5, then the queue reading and the adoption offer"*, on every
//! primary connection, **only on the confirmed branch**. `Servers` names it as
//! one of the two steps its own sequence does not build
//! (`smoozy_servers::sequence::STEPS_NOT_BUILT`), and the hand-over runs from
//! whoever drives that sequence — which is `src/servers/a_period.rs`, at the
//! moment the connection is first lent. **This file is what runs then.**
//!
//! # The order, and every step is a clause
//!
//! `State, identity and recovery` V1, at that document's §7, in its own order:
//!
//! - **Step 1 is not here.** It is the installation identity's branch, taken
//!   inside `Servers`' own sequence at its step 5. **Its one write** — every
//!   record of the server whose render start was never learned marked *start
//!   never heard* where the branch is absent or different, the Creator's
//!   ruling C1 of 2026-09-18 — **is made by `src/servers/a_period.rs` the
//!   moment the branch comes back**, which is the first caller that write has
//!   ever had. **Only the confirmed branch reaches this file.**
//! - **Step 2**: every provisional local job record of this server is asked
//!   about, and [`crate::record::job_records::JobRecords::the_resolution`]
//!   performs what the answer means. **The third branch — absent, and the
//!   machine cannot say — leaves the record provisional, and no absence of any
//!   length removes it.** Nothing in this file holds a clock, a count of
//!   connections or an age a timeout could be built from; a record the machine
//!   cannot speak for is simply asked about again on the next connection, and
//!   [`tests::a_record_the_machine_cannot_speak_for_is_never_removed`] is the
//!   test that holds it by name.
//! - **Step 3**: the read forward from the durable event cursor, each event
//!   applied idempotently, **the cursor advanced only after the application is
//!   durable**, and a gap said rather than smoothed
//!   (`State, identity and recovery` E3, E4, E5).
//! - **Step 4**: every record the machine now has, and whose ending Smoozy has
//!   not learned, filled in from the machine's own note — through the store's
//!   own fill-in, **whose tombstone refusal is unconditional**.
//! - **Step 5**: the queue reading, `Queue & History` A1's one request,
//!   **period-held** and written nowhere durable.
//! - **Step 6**: the adoption offer, `Queue & History` A5, composed once from
//!   that reading and **never recomposed by a pass** (`Servers` Q18).
//!
//! **Beside it, on a thread of its own, `Server protocol and transfers` S28's
//! machine-side sweep** — [`the_staging_left_on_this_machine_is_swept`], step
//! 10c's transfer wave piece 7: *"The sweep runs at the next connection to that
//! server … beside admission rather than in front of it."*
//!
//! **Then P10d starts**, on the same thread: `src/servers/live_reading.rs`'s
//! sequence, whose three reads get their ends here — the read forward is step
//! 3's own pass, the queue reading is step 5's own request, and the running
//! entry's commentary read is the reading session's ranged read of a sink
//! (see [`ThePassesOverThisConnection`], piece 6c).
//!
//! # Where it runs, and why not on the period's own thread
//!
//! **Every request here goes through `Server protocol and transfers` T5's
//! per-server point**, and that point serves from the period's own thread,
//! inside the wait for the connection to end. **A request made from that
//! thread would wait for itself.** So step 7 runs on a thread of its own,
//! started the moment the point goes live, and ends with the connection it was
//! started for: [`WhatThisRunHolds::is_current`] answers false the moment that
//! connection ends, whatever else has begun since.
//!
//! # Three things this file does not do, each said where it would have been
//!
//! - ~~**The catch-up notice is composed and held for the run, not written
//!   durably.**~~ **Since the 38's piece 15 it is written durably**, into the
//!   tables record version 38 made (`src/record/catch_up_notices.rs`), before
//!   the cursor advances past the events it names (`Queue & History` C2) —
//!   so it survives a restart and stands until the user's *Dismiss*. That
//!   bullet is struck rather than removed because it named an owed item.
//! - **`Servers` W41a's running-time request is not made, and the reason is
//!   now only the second one.** Until 2026-09-21 its request name and its
//!   answer's fields were spelled only in the machine's own runtime library,
//!   whose acting half is built for the machine alone and which this crate —
//!   which must also build for Windows — cannot reach at all. **They are in
//!   `smoozy-framing` now**, so this side can name them, and the last test in
//!   this file is what says so. **What still holds the request back is that no screen of this
//!   piece's reads the figure**, which is `Desktop app and delivery`'s own work
//!   and not this file's.
//! - **Taking a submission's leftovers as a debt at a later connection
//!   (`Rendering` S4b) is not here.** It is not one of step 7's clauses.
//!
//! # How a clause marker is written in this file
//!
//! **This file keeps no header list** and writes every other document's name
//! at its own site. **A bare marker below means a clause of this crate's own
//! plan, `Preparation/Architecture/Desktop app and delivery.md`.**

use std::collections::{HashMap, HashSet};
use std::future::Future;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;

use tauri::{AppHandle, Manager, Runtime};

use smoozy_queue_history::acting::adoption::{
    the_adoption_offer, AsksAMachineForItsQueue, TheMachinesWorkingState, WhatTheRequestGave,
};
use smoozy_queue_history::acting::note::TheNote;
use smoozy_queue_history::catch_up::{compose_the_account, AnAppliedEvent};
use smoozy_queue_history::line::QueueState;
use smoozy_queue_history::readings::{AJobAsTheMachineReadsIt, WhatTheEntryHasLive};
use smoozy_queue_history::record::{Standing, WhatTheMachineSaidAboutTheJob};
use smoozy_server_protocol::asking_the_supervisor::WhatCameBack;
use smoozy_server_protocol::framing::{Fields, Request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS};
use smoozy_server_protocol::requests::{
    the_field_of_item, AN_EVENT, HOW_MANY_EVENTS_FIELD, HOW_MANY_JOBS_FIELD, IT_HAS_ENDED,
    IT_IS_ABSENT, IT_IS_HELD, IT_IS_PRESENT, IT_IS_RENDERING, IT_IS_WAITING, ON_THIS_MACHINE_FIELD,
    THE_ABOUT_FIELD, THE_CURSOR_FIELD, THE_CURSOR_MOVES_FORWARD_REQUEST, THE_DISPOSITION_EVENT,
    THE_DOES_THE_MACHINE_HOLD_THIS_JOB_REQUEST, THE_HELD_BEHIND_FIELD,
    THE_HOLD_IT_IS_HELD_UNDER_FIELD, THE_HOLD_PLACED_EVENT, THE_HOLD_REASON_FIELD, THE_JOB_FIELD,
    THE_JOB_RECORD_REMOVED_EVENT, THE_JOB_RECORD_WRITTEN_EVENT, THE_JOB_THAT_STARTS_NEXT_FIELD,
    THE_KIND_FIELD, THE_NEXT_SEQUENCE_FIELD, THE_OLDEST_RETAINED_FIELD,
    THE_OUTCOME_IT_ENDED_WITH_FIELD, THE_PER_JOB_NOTE_READING_REQUEST, THE_PREVIEW_PUBLISHED_EVENT,
    THE_QUEUE_READING_REQUEST, THE_QUEUE_STATE_FIELD, THE_READ_FORWARD_REQUEST,
    THE_RECORDED_OUTCOME_WRITTEN_EVENT, THE_RELEASE_EVENT, THE_RENDER_ATTEMPT_STARTED_EVENT,
    THE_PHONES_CHOICE_MOMENT_FIELD, THE_PHONES_CHOICE_OPERATION_FIELD, THE_PHONES_REPORTING_CHOICE_FIELD,
    THE_RENDER_OPERATION_FIELD, THE_REPORTING_CHOICE_FIELD, THE_SAYS_FIELD, THE_SEQUENCE_FIELD,
};
use smoozy_servers::wiring::TheDesktopsClock;
use smoozy_state::identities::{Identity, IdentityKind, InstallationCheck};
use smoozy_state::observed::{Clock, Reading};

use crate::keys::LendsTheRecord;
use crate::notifications::when::{HowItArrived, NowOrCatchingUp, WhereTheEventsAreComingFrom};
use crate::record::job_records::JobRecords;
use crate::record::server_rows;
use crate::servers::a_period::ThePeriodsThisRun;
use crate::servers::installation::{where_the_package_area_is, WhereThePackageAreaIs};
use crate::servers::live_reading::{
    the_live_reading_sequence, WhatAPassReads, WhatAPassTakes, WhatIsOnScreen,
};
use crate::servers::sessions::{AHeldConnection, ThisServersSessions};
use crate::servers::the_idle_recheck::TheIdleNudges;
use crate::start::Started;

// ════════════════════════════════════════════════════════════════════════════
// The kinds the reader reads
// ════════════════════════════════════════════════════════════════════════════

/// **The kinds whose application moves a job the local record may hold**, and
/// they are every kind the machine journals about the line. **A kind not here
/// is applied as nothing** — `State, identity and recovery` E4's idempotence
/// makes that safe — and a kind the machine adds later is read once it is
/// added here.
const THE_KINDS_THAT_MOVE_A_JOB: [&str; 6] = [
    THE_JOB_RECORD_WRITTEN_EVENT,
    THE_HOLD_PLACED_EVENT,
    THE_RENDER_ATTEMPT_STARTED_EVENT,
    THE_RECORDED_OUTCOME_WRITTEN_EVENT,
    THE_DISPOSITION_EVENT,
    THE_RELEASE_EVENT,
];

/// **The kinds whose event names a job whose reading may now stand** —
/// `Queue & History` N9's journalled event, a render attempt's start (where a
/// job read before submitting carries its reading in the manifest), and a
/// recorded outcome (where every attempt record stands). The 38's piece 10.
const THE_KINDS_THAT_NAME_A_READING: [&str; 3] = [
    smoozy_server_protocol::requests::THE_RECORDED_READING_WRITTEN_EVENT,
    THE_RENDER_ATTEMPT_STARTED_EVENT,
    THE_RECORDED_OUTCOME_WRITTEN_EVENT,
];

// ════════════════════════════════════════════════════════════════════════════
// Asking the machine's supervisor
// ════════════════════════════════════════════════════════════════════════════

/// **One request to the machine's supervisor, and what came back.**
///
/// **A seam and not a stand-in**: its one implementation in the running
/// application is [`OverThisServersPoint`], over `Server protocol and
/// transfers` T5's per-server point; the tests answer it from a scripted
/// machine.
pub trait AsksTheSupervisor {
    fn ask(&mut self, request: &Request) -> WhatCameBack;

    /// **One ask, waited on for no longer than `within`** — the time a waiter
    /// has left of its own deadline (`Server protocol and transfers` C16).
    /// Added 2026-10-19, the dead-line sweep's two-minute-wait repair: until then a waiter
    /// checked its deadline only after an ask returned, so an ask begun just
    /// before it on a line gone silent waited a whole break window past it.
    ///
    /// **An ask given up this way is *nothing correlated came back***: the
    /// request may have reached the machine, and nothing is claimed either
    /// way (`Server protocol and transfers` C17). **Nothing is cancelled** —
    /// the machine records before it acts (C4), and a late outcome is read
    /// later where every outcome is read.
    ///
    /// **By default the ask is made whole**, which is what a seam that answers
    /// at once does; the application's own point gives it up on time.
    fn ask_within(&mut self, request: &Request, within: std::time::Duration) -> WhatCameBack {
        let _ = within;
        self.ask(request)
    }

    /// **One file of a job's record directory, read whole over the reading
    /// session** — `Queue & History` N9's recorded reading *"sits in the job
    /// record directory for §7 step 4's fill-in"* (`Job files and outputs` H9),
    /// and it is read where the manifest and the attempt records are read,
    /// on the route `Download` N2 already takes. The 38's piece 10.
    ///
    /// **On this seam rather than a second one** because every caller of the
    /// fill-in holds this one handle on the machine. **Nothing by default**: a
    /// seam that reads no file reads the reading as not landed, which is the
    /// honest state of every job before its render's step 4.
    fn a_file_of_the_job_record_directory(&mut self, path: &str) -> Option<Vec<u8>> {
        let _ = path;
        None
    }
}

/// **The supervisor, asked through this server's own point**, with the package
/// area the server row records.
///
/// **Over any held connection, the real one by default**, so the point's own
/// asking is driven at the desk too (the dead-line sweep's two-minute-wait repair).
pub struct OverThisServersPoint<
    C: AHeldConnection = smoozy_server_protocol::transport::ssh::Connection,
> {
    pub sessions: ThisServersSessions<C>,
    pub package_area: String,
}

impl<C: AHeldConnection> AsksTheSupervisor for OverThisServersPoint<C> {
    fn ask(&mut self, request: &Request) -> WhatCameBack {
        let sessions = &self.sessions;
        let package_area = self.package_area.as_str();
        waited_for(|| async move { sessions.ask_the_supervisor(package_area, request).await })
    }

    /// **Given up at `within`**, wherever the ask stands — waiting its turn,
    /// planting its link or waiting on the command's answer. **What a
    /// give-up leaves is what the point already does with an asker that
    /// stopped waiting**: a command under way has its channel closed
    /// (`Server Setup` Y8's route, `crate::servers::sessions`), and a link
    /// left planted is replaced by the next ask's own planting.
    fn ask_within(&mut self, request: &Request, within: std::time::Duration) -> WhatCameBack {
        let sessions = &self.sessions;
        let package_area = self.package_area.as_str();
        waited_for(|| async move {
            tokio::time::timeout(within, sessions.ask_the_supervisor(package_area, request))
                .await
                .unwrap_or(WhatCameBack::NothingCorrelatedCameBack)
        })
    }

    /// **Nobody is waiting on it** (`Server protocol and transfers` §1.1 item
    /// 2): a user's own listing goes first.
    fn a_file_of_the_job_record_directory(&mut self, path: &str) -> Option<Vec<u8>> {
        use smoozy_download::acting::record_directory::ReadsOneFileOnTheMachine;
        use smoozy_download::acting::wiring::TheReadingRoute;
        let sessions = &self.sessions;
        let path = path.as_bytes().to_vec();
        waited_for(|| async move {
            sessions
                .a_reading(false, move |subsystem| {
                    Box::pin(async move {
                        TheReadingRoute::on(subsystem)
                            .read_one_whole_file(&path)
                            .await
                    })
                })
                .await
        })
        .ok()?
        .ok()
    }
}

/// **Every step here is a synchronous seam and every ask is not**, so an ask
/// is waited on from a thread of its own, on a runtime made for the one wait
/// and dropped with it. **The pass that makes an ask is itself inside a
/// runtime** — P10d's sequence is asynchronous — and a runtime blocked from
/// inside itself stops; this is how `src/submitting.rs` waits on the same point
/// and for the same reason.
pub(crate) fn waited_for<M, F>(make: M) -> F::Output
where
    M: FnOnce() -> F + Send,
    F: Future,
    F::Output: Send,
{
    std::thread::scope(|scope| {
        scope
            .spawn(move || {
                let runtime = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .expect("a runtime for one wait");
                runtime.block_on(make())
            })
            .join()
            .expect("the wait did not panic")
    })
}

/// One request under step 10c piece 1's name, with a fresh operation identity
/// (`Server protocol and transfers` C1). **Nothing** where this computer
/// offered no randomness for one.
pub(crate) fn a_request(name: &str, fields: Fields) -> Option<Request> {
    let identity = Identity::issue(IdentityKind::Operation).ok()?;
    Some(Request::speaking(THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, identity, name).carrying(fields))
}

/// **The answer's fields, where the supervisor answered this request.**
/// Nothing where it refused the request by name, or where nothing that
/// answers it came back.
pub(crate) fn the_answer_to(
    machine: &mut dyn AsksTheSupervisor,
    request: Option<Request>,
) -> Option<Fields> {
    match machine.ask(&request?) {
        WhatCameBack::ItAnswered(answered)
            if answered
                .the_request_name_it_says_it_does_not_answer()
                .is_none() =>
        {
            Some(answered.fields)
        }
        _ => None,
    }
}

pub(crate) fn an_item<'f>(
    fields: &'f Fields,
    item: &str,
    at: usize,
    field: &str,
) -> Option<&'f str> {
    fields.text(&the_field_of_item(item, at, field))
}

pub(crate) fn a_number<T: std::str::FromStr>(text: Option<&str>) -> Option<T> {
    text?.parse().ok()
}

// ── `State, identity and recovery` V1 step 2's ask ─────────────────────────

/// **Does the machine hold this job** — step 10c piece 1's ask, read as
/// `State, identity and recovery` V1 step 2's three branches.
///
/// **Only *absent* removes a record, and only an answer that says so is
/// *absent*.** Every other outcome — the machine's own *cannot say*, a request
/// it refused, an answer with no word in it, a silence, an ask never made — is
/// the third branch, which is the one the spec makes safe.
pub fn what_the_machine_says_about(
    machine: &mut dyn AsksTheSupervisor,
    job: &Identity,
) -> WhatTheMachineSaidAboutTheJob {
    let asked = a_request(
        THE_DOES_THE_MACHINE_HOLD_THIS_JOB_REQUEST,
        Fields::none().and(THE_JOB_FIELD, job.as_text()),
    );
    match the_answer_to(machine, asked)
        .as_ref()
        .and_then(|fields| fields.text(ON_THIS_MACHINE_FIELD))
    {
        Some(IT_IS_PRESENT) => WhatTheMachineSaidAboutTheJob::ItHoldsTheJob,
        Some(IT_IS_ABSENT) => {
            WhatTheMachineSaidAboutTheJob::ItsOperationRecordShowsTheSubmissionWasRefusedOrNeverTaken
        }
        _ => WhatTheMachineSaidAboutTheJob::TheMachineCannotSay,
    }
}

// ── `Queue & History` N1's note ──────────────────────────────────────────────

/// **One job's note, read off the machine** — `Queue & History` N1, *"the read
/// surface an entry's lasting facts come from"*.
///
/// **Nothing where the note is not there, or cannot be read whole.** A note is
/// never half-read: an ending whose outcome word this build does not know is a
/// note it cannot read, and the fill-in is then not made at all, rather than
/// made with the ending left out.
///
/// **The answer is read by the note's own crate**, which also composes it on
/// the machine, so the two ends have one home (step 10c's fix piece 2). **The
/// ending now brings the reason the machine recorded and the device line as
/// the worker observed it** (the paragraph after `Queue & History` N3),
/// and the fill-in keeps both beside the record.
pub fn the_note_of(machine: &mut dyn AsksTheSupervisor, job: &Identity) -> Option<TheNote> {
    let asked = a_request(
        THE_PER_JOB_NOTE_READING_REQUEST,
        Fields::none().and(THE_JOB_FIELD, job.as_text()),
    );
    let fields = the_answer_to(machine, asked)?;
    TheNote::read_from_the_answer(&fields)
}

// ── `Queue & History` A1's queue reading ─────────────────────────────────────

/// **`Queue & History` A1's one request, read into that module's own shape.**
///
/// **A rendering job's live half is read here** — `Queue & History` R5's
/// figures, which the render worker reports every two seconds and the
/// supervisor carries in that job's part of the answer (step 10c's notes on
/// 0.1.8, piece 6a), with `Queue & History` B1's total and `Queue & History` B6's finished part,
/// and `Queue & History` E8's two sinks with their lengths. **Period-held
/// with the rest of the reading and written nowhere durable** (`Queue &
/// History` R5); a part that carries neither reads as none, which `Queue &
/// History` R4 makes an ordinary entry. *(This said until 2026-10-03, piece
/// 6b, that no render worker reported the figures; 6a made that untrue.)*
///
/// **What the machine's answer does not carry, and so this does not either.**
/// The notes:
/// each has its own reading, one job at a time. **The machine's own history
/// copy**: it has no store on the machine (step 10c piece 5, sitting 1, item
/// 4), so its count is nought and its entries none — which the Queue screen
/// never draws, so nothing on it reads as though the copy were empty.
///
/// **An answer with no count of jobs is a store the machine could not read**,
/// and that is not an empty line: it is taken as no reading at all.
pub fn the_queue_reading_from(
    machine: &mut dyn AsksTheSupervisor,
) -> Result<TheMachinesWorkingState, WhyThereIsNoReading> {
    let Some(request) = the_queue_reading_request() else {
        return Err(WhyThereIsNoReading::ItCouldNotBeAsked);
    };
    the_queue_reading_in(machine.ask(&request))
}

/// **`Queue & History` A1's one request**, with a fresh operation identity, or
/// nothing where this computer offered no randomness for one. Split out on
/// 2026-09-28 so an asker that is not synchronous — the ready action's —
/// sends the same request and reads it with [`the_queue_reading_in`].
pub fn the_queue_reading_request() -> Option<Request> {
    a_request(THE_QUEUE_READING_REQUEST, Fields::none())
}

/// **What came back for [`the_queue_reading_request`], read into `Queue &
/// History`'s own shape.**
pub fn the_queue_reading_in(
    came_back: WhatCameBack,
) -> Result<TheMachinesWorkingState, WhyThereIsNoReading> {
    the_queue_reading_and_its_reporting_in(came_back).map(|(state, _)| state)
}

/// **What `Queue & History` A1's reading carries for `Telegram` V1d**, read
/// beside the working state: each job's current reporting choice where the
/// machine holds one, and once the machine's phone's choice. **Both are absent
/// on a machine that holds neither**, and on a program from before them.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct WhatTheReadingSaysOfReporting {
    /// Each job's current choice, in the manifest's own form as JSON text
    /// (`Telegram` V1c's record value). A value that is not one of the five
    /// is not carried here.
    pub each_jobs_choice: Vec<(Identity, String)>,
    pub the_phones_choice: Option<ThePhonesChoiceRead>,
}

/// **The machine's phone's choice** (`Telegram` V1d): the value, the operation
/// identity, and Telegram's own `date` for it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThePhonesChoiceRead {
    pub choice: smoozy_render_settings::reporting::ReportingChoice,
    pub operation: String,
    pub moment: i64,
}

/// **[`the_queue_reading_in`], and what the same answer says of reporting.**
/// Split out on 2026-10-31 for `Telegram` V1d, so every caller that wants only
/// the working state is unchanged.
pub fn the_queue_reading_and_its_reporting_in(
    came_back: WhatCameBack,
) -> Result<(TheMachinesWorkingState, WhatTheReadingSaysOfReporting), WhyThereIsNoReading> {
    use smoozy_render_settings::reporting::ReportingChoice;
    let fields = match came_back {
        WhatCameBack::ItAnswered(answered)
            if answered
                .the_request_name_it_says_it_does_not_answer()
                .is_none() =>
        {
            answered.fields
        }
        WhatCameBack::ItAnswered(_) => return Err(WhyThereIsNoReading::TheMachineSaidItCouldNot),
        WhatCameBack::NothingCorrelatedCameBack => {
            return Err(WhyThereIsNoReading::NothingThatAnswersItCameBack)
        }
        WhatCameBack::ItWasNeverAsked => return Err(WhyThereIsNoReading::ItCouldNotBeAsked),
    };
    let Some(how_many) = a_number::<usize>(fields.text(HOW_MANY_JOBS_FIELD)) else {
        return Err(WhyThereIsNoReading::TheMachineSaidItCouldNot);
    };
    let mut jobs = Vec::with_capacity(how_many);
    let mut reporting = WhatTheReadingSaysOfReporting::default();
    for at in 0..how_many {
        let Some(job) = an_item(&fields, THE_JOB_FIELD, at, THE_JOB_FIELD)
            .and_then(|text| Identity::read_back(IdentityKind::Job, text))
        else {
            continue;
        };
        let an_operation = |field: &str| {
            an_item(&fields, THE_JOB_FIELD, at, field)
                .and_then(|text| Identity::read_back(IdentityKind::Operation, text))
        };
        let state = match an_item(&fields, THE_JOB_FIELD, at, THE_QUEUE_STATE_FIELD) {
            Some(IT_IS_WAITING) => QueueState::Waiting,
            Some(IT_IS_HELD) => match an_operation(THE_HOLD_IT_IS_HELD_UNDER_FIELD) {
                Some(hold) => QueueState::Held { hold },
                None => continue,
            },
            Some(IT_IS_RENDERING) => match an_operation(THE_RENDER_OPERATION_FIELD) {
                Some(operation) => QueueState::Rendering { operation },
                None => continue,
            },
            Some(IT_HAS_ENDED) => match an_operation(THE_OUTCOME_IT_ENDED_WITH_FIELD) {
                Some(outcome) => QueueState::Ended { outcome },
                None => continue,
            },
            _ => continue,
        };
        // **`Queue & History` A1's *its unreleased hold*: why a held job is
        // held and what it waits behind**, read off the machine's hold record
        // (`Queue & History` H1, `Queue & History` H6). Absent from a program
        // older than step 10c's eleventh-pass fix piece 8b, and then nothing
        // is made up here.
        let the_hold = match &state {
            QueueState::Held { .. } => an_item(&fields, THE_JOB_FIELD, at, THE_HOLD_REASON_FIELD)
                .map(|reason| smoozy_queue_history::readings::WhatTheHoldSays {
                    reason: reason.to_string(),
                    held_behind: an_item(&fields, THE_JOB_FIELD, at, THE_HELD_BEHIND_FIELD)
                        .and_then(|text| Identity::read_back(IdentityKind::Job, text)),
                }),
            _ => None,
        };
        // **`Queue & History` R5 and E8: a rendering job's figures and sinks**,
        // read back by the machine crate's own reader so this side spells no
        // name. Only a rendering job carries them. Piece 6b.
        // **`Rented-server runtime` S14 and `Rented-server runtime` S14a: an ended job's part carries
        // its ending while that ending is owed to a full store** — the
        // outcome, the owed mark and `Rented-server runtime` V12's reading,
        // read back by the machine crate's own reader. History draws the job
        // from it until the landing (`Queue & History` C11). Step 12's SC4
        // follow-up.
        let live = match &state {
            QueueState::Rendering { .. } => {
                WhatTheEntryHasLive::read_back(|name| an_item(&fields, THE_JOB_FIELD, at, name))
            }
            QueueState::Ended { .. } => {
                smoozy_queue_history::acting::note::TheEndingOwed::read_back(|name| {
                    an_item(&fields, THE_JOB_FIELD, at, name)
                })
                .map_or(
                    WhatTheEntryHasLive::NoReadings,
                    WhatTheEntryHasLive::ItsEndingIsOwed,
                )
            }
            _ => WhatTheEntryHasLive::NoReadings,
        };
        // **`Queue & History` A1 for `Telegram` V1d: the job's current
        // reporting choice**, where the machine's record holds one. Read back
        // into one of the five and written again in the one form, so the
        // desk's record never keeps a spelling it cannot read.
        if let Some(choice) = an_item(&fields, THE_JOB_FIELD, at, THE_REPORTING_CHOICE_FIELD)
            .and_then(ReportingChoice::from_the_manifests_form)
        {
            reporting.each_jobs_choice.push((job.clone(), choice.in_the_manifests_form()));
        }
        jobs.push(AJobAsTheMachineReadsIt {
            job,
            state,
            live,
            the_hold,
        });
    }
    // **And once, the machine's phone's choice** — all three or none.
    reporting.the_phones_choice = match (
        fields
            .text(THE_PHONES_REPORTING_CHOICE_FIELD)
            .and_then(ReportingChoice::from_the_manifests_form),
        fields.text(THE_PHONES_CHOICE_OPERATION_FIELD).filter(|text| !text.is_empty()),
        a_number::<i64>(fields.text(THE_PHONES_CHOICE_MOMENT_FIELD)),
    ) {
        (Some(choice), Some(operation), Some(moment)) => Some(ThePhonesChoiceRead {
            choice,
            operation: operation.to_string(),
            moment,
        }),
        _ => None,
    };
    Ok((
        TheMachinesWorkingState {
            jobs,
            the_admission_point_stands_at: fields
                .text(THE_JOB_THAT_STARTS_NEXT_FIELD)
                .and_then(|text| Identity::read_back(IdentityKind::Job, text)),
            notes: Vec::new(),
            the_history_copys_count: 0,
            the_history_copys_entries: Vec::new(),
        },
        reporting,
    ))
}

/// **What the desk does with [`WhatTheReadingSaysOfReporting`]** — `Telegram`
/// V1d's *what the desktop learns*: each tracked job's current choice written
/// into its own record, so the Queue's control (`Telegram` V1c) shows a change
/// made from the phone; and the phone's choice taken as the app-wide default
/// where it is newer than the one last taken
/// ([`crate::record::settings::the_phones_choice_is_taken_where_newer`]). A
/// job the desk does not track has no record and nothing is written for it.
/// **Says whether the default was taken**, so Settings can be drawn again.
pub fn the_reporting_the_reading_carried_is_written(
    connection: &rusqlite::Connection,
    said: &WhatTheReadingSaysOfReporting,
) -> bool {
    let records = JobRecords::over(connection);
    for (job, choice) in &said.each_jobs_choice {
        let _ = records.the_reporting_choice_is_written(job, choice);
    }
    let Some(phone) = said.the_phones_choice.as_ref() else {
        return false;
    };
    let (id, minutes) = crate::surface::render_settings::the_page_identifier_of(phone.choice);
    crate::record::settings::the_phones_choice_is_taken_where_newer(
        connection,
        id,
        minutes,
        &phone.operation,
        phone.moment,
    )
    .unwrap_or(false)
}

/// Why a queue reading was not had.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhyThereIsNoReading {
    /// The supervisor answered and said it could not — a request it does not
    /// answer, or a store it could not read.
    TheMachineSaidItCouldNot,
    /// **The command ran and nothing that answers it came back.**
    NothingThatAnswersItCameBack,
    /// The request never reached the supervisor.
    ItCouldNotBeAsked,
}

/// **`Queue & History`'s own seam, given its end** — the one step 10b's piece
/// 8 found the queue's controls waiting on.
///
/// **`Queue & History` A3's narrowing is kept by what is asked of the point,
/// not by a guess here.** A request that ran and got nothing correlated back
/// on a connection that is still live is the supervisor not answering; the
/// same silence on a connection that has since gone is the connection's, and
/// raises nothing (`Server protocol and transfers` C17a).
pub struct TheQueueOverThisServersPoint<'a> {
    pub machine: std::cell::RefCell<&'a mut dyn AsksTheSupervisor>,
    pub the_connection_is_live: &'a dyn Fn() -> bool,
    /// **What the answer said of reporting** (`Telegram` V1d), kept from the
    /// last answer for the caller, which holds the record. Start it empty.
    pub the_reporting_read: std::cell::RefCell<Option<WhatTheReadingSaysOfReporting>>,
}

impl AsksAMachineForItsQueue for TheQueueOverThisServersPoint<'_> {
    fn ask(&self, _server: &Identity) -> WhatTheRequestGave {
        let read = match the_queue_reading_request() {
            Some(request) => the_queue_reading_and_its_reporting_in(self.machine.borrow_mut().ask(&request)),
            None => Err(WhyThereIsNoReading::ItCouldNotBeAsked),
        };
        let read = read.map(|(state, reporting)| {
            *self.the_reporting_read.borrow_mut() = Some(reporting);
            state
        });
        match read {
            Ok(state) => WhatTheRequestGave::ItAnswered(Reading::taken(
                state,
                SystemTime::now(),
                Clock::ThisComputer,
            )),
            Err(WhyThereIsNoReading::NothingThatAnswersItCameBack)
                if (self.the_connection_is_live)() =>
            {
                WhatTheRequestGave::TheSupervisorIsUnavailable
            }
            Err(WhyThereIsNoReading::TheMachineSaidItCouldNot) => {
                WhatTheRequestGave::TheSupervisorIsUnavailable
            }
            Err(_) => WhatTheRequestGave::ItWasLostInsideADeclaredBreakAndRaisesNothing,
        }
    }
}

// ── `State, identity and recovery` §4's read forward ─────────────────────────

/// One event, as the read forward carried it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnEventRead {
    pub sequence: u64,
    pub kind: String,
    pub about: Option<String>,
    /// **The machine's own words for the event**, which is where a hold's
    /// release says its outcome (`Rented-server runtime` U20).
    pub says: Option<String>,
}

/// What one read forward carried.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatTheReadForwardCarried {
    pub events: Vec<AnEventRead>,
    /// **One past the newest sequence the journal holds.**
    pub next: u64,
    /// **`State, identity and recovery` E5's number**, absent where the
    /// journal holds nothing.
    pub oldest_retained: Option<u64>,
}

/// **The read forward from `from`** — the first sequence number the desktop
/// has not yet done the work for. **A read advances nothing** (`State, identity
/// and recovery` E3).
pub fn the_read_forward_from(
    machine: &mut dyn AsksTheSupervisor,
    from: u64,
) -> Option<WhatTheReadForwardCarried> {
    let asked = a_request(
        THE_READ_FORWARD_REQUEST,
        Fields::none().and(THE_CURSOR_FIELD, from.to_string()),
    );
    let fields = the_answer_to(machine, asked)?;
    let how_many: usize = a_number(fields.text(HOW_MANY_EVENTS_FIELD))?;
    let next: u64 = a_number(fields.text(THE_NEXT_SEQUENCE_FIELD))?;
    let mut events = Vec::with_capacity(how_many);
    for at in 0..how_many {
        // **An event that will not read back makes the whole read unreadable.**
        // Applying the ones around it and advancing past it would lose it for
        // good, which is what `State, identity and recovery` E3 exists to stop.
        events.push(AnEventRead {
            sequence: a_number(an_item(&fields, AN_EVENT, at, THE_SEQUENCE_FIELD))?,
            kind: an_item(&fields, AN_EVENT, at, THE_KIND_FIELD)?.to_string(),
            about: an_item(&fields, AN_EVENT, at, THE_ABOUT_FIELD).map(str::to_string),
            says: an_item(&fields, AN_EVENT, at, THE_SAYS_FIELD).map(str::to_string),
        });
    }
    Some(WhatTheReadForwardCarried {
        events,
        next,
        oldest_retained: a_number(fields.text(THE_OLDEST_RETAINED_FIELD)),
    })
}

/// **The acknowledgement that moves the desktop's registration on the
/// machine** — `State, identity and recovery` E6's retention floor.
///
/// **Its answer is not read, and that is `Rented-server runtime` R1's row for
/// this command rather than a shortcut**: the command record ends as the
/// command closes, so an ask about it afterwards finds no record, and nothing
/// the desktop holds depends on it. **The desktop's own cursor is the durable
/// one** (`State, identity and recovery` E2), already written by the time this
/// is sent; a move that is lost is simply made again by the next pass.
fn the_machine_is_told_the_cursor_moved(machine: &mut dyn AsksTheSupervisor, to: u64) {
    let _ = the_answer_to(
        machine,
        a_request(
            THE_CURSOR_MOVES_FORWARD_REQUEST,
            Fields::none().and(THE_CURSOR_FIELD, to.to_string()),
        ),
    );
}

// ── `Rented-server runtime` U9 and `Rented-server runtime` U10: the desk's
// answer to a hold ──
//
// Step 10c's eleventh-pass fix piece 8. **Until then nothing on this side
// answered a hold at all**, and the held entry's two answers said *"Sending
// instructions to the server is not built"*, though the machine has answered
// this request since step 10c piece 5.

/// **What the desk's answer to a hold came to.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatTheAnswerCameTo {
    /// **`Rented-server runtime` U14: the hold's recorded release outcome** —
    /// which answer was applied, this one or one recorded before it (`Rented-server runtime` U13).
    /// **A second answer is not an error**: it is told what stood.
    TheHoldIsAnswered(smoozy_runtime::holds::ReleaseOutcome),
    /// **`Rented-server runtime` U12a: the *no* was written *stopped* and the
    /// machine removed the job in the same act** — its result carries the
    /// cancel's own *removed* (`Queue & History` L2). A program older than
    /// step 10c's eleventh-pass fix piece 8b never says this, and its *no*
    /// reads [`Self::TheHoldIsAnswered`] *stopped* with the job still there.
    TheNoRemovedTheJob,
    /// **The machine did not take it** — an older program that answers no such
    /// request, or one that speaks another protocol. Nothing was recorded.
    NotTaken,
    /// **Nothing reached the machine** — no live connection, or the request
    /// never went out.
    NotReached,
    /// **The waiter's own deadline passed** (`Server protocol and transfers`
    /// C17) with the answer still in hand, or nothing readable back. Nothing
    /// is claimed either way.
    TheWaiterGaveUp,
}

/// **`Rented-server runtime` U9: an answer to a hold is an ordinary command
/// with its own operation identity, naming the hold identity and carrying the
/// answer** — and `Queue & History` H2: *"the desk's answer … submitted over
/// the connection"*. **`Rented-server runtime` U10: the desk's route**, over SSH through the command
/// adapter, carried in the field the machine reads it from.
///
/// **Asked again under the same identity until the release outcome comes
/// back** (`Server protocol and transfers` C8 and `Server protocol and
/// transfers` C9): the machine records the identity before it acts (`Server
/// protocol and transfers` C4), so the same answer arriving twice acts once,
/// and *still in hand* is an answer rather than a silence. **The answer's own
/// C8 word is not read**: the machine spells that field in its own program and
/// nowhere both ends share, so what is read is the release outcome itself —
/// `Server protocol and transfers`' `THE_RELEASE_OUTCOME_FIELD`, which both
/// ends take from one file — and anything without one is asked again.
pub fn the_answer_to_a_hold_is_sent(
    machine: &mut dyn AsksTheSupervisor,
    hold: &Identity,
    says: smoozy_runtime::holds::WhatTheAnswerSays,
    wait: crate::servers::stop_and_clear::TheWait,
) -> WhatTheAnswerCameTo {
    use smoozy_runtime::holds::{ReleaseOutcome, WhatTheAnswerSays};
    use smoozy_server_protocol::requests::{
        IT_WAS_REMOVED, THE_ANSWER_FIELD, THE_ANSWER_IS_NO, THE_ANSWER_IS_YES,
        THE_ANSWER_TO_A_HOLD_REQUEST, THE_CANCELS_OUTCOME_FIELD, THE_HOLD_FIELD,
        THE_RELEASE_OUTCOME_FIELD, THE_ROUTE_FIELD, THE_ROUTE_IS_THE_DESK,
    };
    let Some(request) = a_request(
        THE_ANSWER_TO_A_HOLD_REQUEST,
        Fields::none()
            .and(THE_HOLD_FIELD, hold.as_text())
            .and(
                THE_ANSWER_FIELD,
                match says {
                    WhatTheAnswerSays::Yes => THE_ANSWER_IS_YES,
                    WhatTheAnswerSays::No => THE_ANSWER_IS_NO,
                },
            )
            .and(THE_ROUTE_FIELD, THE_ROUTE_IS_THE_DESK),
    ) else {
        return WhatTheAnswerCameTo::NotReached;
    };
    // **The machine's own word for each outcome is the outcome's own name,
    // lower case** — the same reading `the_held_jobs_dropped_in` takes of the
    // release event, from the one type both ends compile.
    let the_word_for = |outcome: ReleaseOutcome| format!("{outcome:?}").to_lowercase();
    let began = std::time::Instant::now();
    let mut it_may_have_arrived = false;
    loop {
        // **Every ask is given what is left of the waiter's own deadline, and
        // none is begun once nothing is** — the dead-line sweep's two-minute-wait repair,
        // 2026-10-19, as `crate::servers::the_queues_own_acts` asks. The
        // give-up writes nothing and cancels nothing (`Server protocol and
        // transfers` C17); a late release is read off the journal.
        let Some(left) = wait.what_is_left(began) else {
            return WhatTheAnswerCameTo::TheWaiterGaveUp;
        };
        match machine.ask_within(&request, left) {
            WhatCameBack::ItAnswered(answered) => {
                if answered
                    .the_request_name_it_says_it_does_not_answer()
                    .is_some()
                    || answered
                        .the_protocol_version_it_says_it_does_not_speak()
                        .is_some()
                {
                    return WhatTheAnswerCameTo::NotTaken;
                }
                it_may_have_arrived = true;
                if let Some(word) = answered.fields.text(THE_RELEASE_OUTCOME_FIELD) {
                    if let Some(outcome) = [
                        ReleaseOutcome::Released,
                        ReleaseOutcome::Stopped,
                        ReleaseOutcome::Dropped,
                    ]
                    .into_iter()
                    .find(|outcome| the_word_for(*outcome) == word)
                    {
                        // **`Rented-server runtime` U12a**: a *no* that
                        // removed its job says so in the cancel's own field.
                        if outcome == ReleaseOutcome::Stopped
                            && answered.fields.text(THE_CANCELS_OUTCOME_FIELD)
                                == Some(IT_WAS_REMOVED)
                        {
                            return WhatTheAnswerCameTo::TheNoRemovedTheJob;
                        }
                        return WhatTheAnswerCameTo::TheHoldIsAnswered(outcome);
                    }
                }
            }
            WhatCameBack::NothingCorrelatedCameBack => it_may_have_arrived = true,
            WhatCameBack::ItWasNeverAsked if !it_may_have_arrived => {
                return WhatTheAnswerCameTo::NotReached
            }
            WhatCameBack::ItWasNeverAsked => {}
        }
        if wait.what_is_left(began).is_none() {
            return WhatTheAnswerCameTo::TheWaiterGaveUp;
        }
        wait.pauses(began);
    }
}

/// **The held entry's two answers** — `Design/Queue & History § What the job
/// writes`: *Render all — 240 frames* and *Cancel* (*Render the rest — 239
/// frames* until decision C, 2026-10-02).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheHeldJobsAnswer {
    RenderAll,
    CancelIt,
}

/// **What one of the held entry's answers left, for the entry to say.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheHeldJobsAnswerLeft {
    /// ***Render all*: the release is written and the job is in the
    /// machine's line** (`Rented-server runtime` U12, `Queue & History` H3).
    ItIsInTheLine,
    /// ***Cancel*: the *no* is written and the job is gone** — the
    /// machine's *no* removed it (`Rented-server runtime` U12a), or, on a
    /// program older than that, the cancel after it came back *removed* — so
    /// the local record may go (`Queue & History` L2, `Queue & History` L5).
    ItWasRemoved,
    /// **Another answer was recorded first** (`Rented-server runtime` U13,
    /// `Rented-server runtime` U14), or the hold was dropped behind a first job
    /// that produced nothing (`Rented-server runtime` U20) — told what stood rather than that it failed. **A *no* that meets
    /// a *yes* already written makes no stop and no cancel**
    /// (`Rented-server runtime` U15).
    AnotherAnswerStood(smoozy_runtime::holds::ReleaseOutcome),
    /// **The answer itself did not come back answered**: never reached, not
    /// taken, or the waiter gave up.
    TheAnswer(WhatTheAnswerCameTo),
    /// ***Cancel*: the *no* is written, and the cancel after it stands
    /// here** — step 10c's S3's own reading of a cancel.
    TheCancel(crate::servers::the_queues_own_acts::WhereTheActStands),
}

/// **One of the held entry's answers, sent** — step 10c's eleventh-pass fix
/// piece 8.
///
/// - ***Render the rest*** is the answer *yes*, naming the hold
///   (`Rented-server runtime` U9, `Rented-server runtime` U12; `Queue &
///   History` H2). The machine writes the release and puts the job at the end
///   of its line in the same act (`Queue & History` H3).
/// - ***Cancel*** is the answer *no*, naming the hold, **and nothing
///   else** — `Specs/Queue & History`: *"Cancelling it and saying no are one
///   act, not two"* and *"Saying no leaves nothing behind."* **The machine's
///   *no* removes its job in the same act** (`Rented-server runtime` U12a)
///   and says so, so a *yes* already recorded from a phone is found rather
///   than overridden (`Rented-server runtime` U13 to `Rented-server runtime`
///   U15) and nothing is sent after it.
///
/// **Written for one case and now meeting two.** Step 10c's eleventh-pass
/// fix piece 8 wrote *Cancel* as the *no* and then the cancel of the job
/// (`Queue & History` L5), **because the machine's *no* wrote *stopped* and
/// removed nothing**, and a *no* alone would have left the job held for ever.
/// **Piece 8b moved that fact to the machine**, one rule for both routes, and
/// the desk's cancel went with it. **It stays for one case only: a program
/// older than piece 8b**, whose *no* comes back *stopped* without saying the
/// job went — told apart by the answer itself rather than by a version. The
/// record and the world parted company wherever a *no* came from a phone: the
/// desk's route emptied the machine and the phone's did not.
pub fn the_held_jobs_answer_is_sent(
    machine: &mut dyn AsksTheSupervisor,
    job: &Identity,
    hold: &Identity,
    answer: TheHeldJobsAnswer,
    wait: crate::servers::stop_and_clear::TheWait,
) -> WhatTheHeldJobsAnswerLeft {
    use smoozy_runtime::holds::{ReleaseOutcome, WhatTheAnswerSays};
    let says = match answer {
        TheHeldJobsAnswer::RenderAll => WhatTheAnswerSays::Yes,
        TheHeldJobsAnswer::CancelIt => WhatTheAnswerSays::No,
    };
    // **One press, one wait**: the cancel after an older program's *no* is
    // given what is left of the answer's own deadline, not a fresh one — the
    // dead-line sweep's two-minute-wait repair, 2026-10-19.
    let began = std::time::Instant::now();
    match (
        answer,
        the_answer_to_a_hold_is_sent(machine, hold, says, wait),
    ) {
        (
            TheHeldJobsAnswer::RenderAll,
            WhatTheAnswerCameTo::TheHoldIsAnswered(ReleaseOutcome::Released),
        ) => WhatTheHeldJobsAnswerLeft::ItIsInTheLine,
        (TheHeldJobsAnswer::CancelIt, WhatTheAnswerCameTo::TheNoRemovedTheJob) => {
            WhatTheHeldJobsAnswerLeft::ItWasRemoved
        }
        // **A program older than `Rented-server runtime` U12a** — or one whose
        // store refused the removal, which says why and leaves the job held —
        // still holds the job after its *no*, so the cancel goes after it.
        (
            TheHeldJobsAnswer::CancelIt,
            WhatTheAnswerCameTo::TheHoldIsAnswered(ReleaseOutcome::Stopped),
        ) => {
            use crate::servers::the_queues_own_acts::{TheQueuesOwnAct, WhereTheActStands};
            // **With nothing left of the wait the cancel is not sent at all**,
            // and that is said as what it is: it did not reach the machine.
            if wait.what_is_left(began).is_none() {
                return WhatTheHeldJobsAnswerLeft::TheCancel(WhereTheActStands::NotReached(
                    TheQueuesOwnAct::TheCancel,
                ));
            }
            match crate::servers::the_queues_own_acts::the_cancel_is_sent(
                machine,
                job,
                wait.the_rest_of(began),
            ) {
                None => WhatTheHeldJobsAnswerLeft::ItWasRemoved,
                Some(stands) => WhatTheHeldJobsAnswerLeft::TheCancel(stands),
            }
        }
        // **A *yes* is never answered *removed*** — only a *no* removes — so
        // this is a *no* recorded first: what stood was *stopped*.
        (TheHeldJobsAnswer::RenderAll, WhatTheAnswerCameTo::TheNoRemovedTheJob) => {
            WhatTheHeldJobsAnswerLeft::AnotherAnswerStood(ReleaseOutcome::Stopped)
        }
        (_, WhatTheAnswerCameTo::TheHoldIsAnswered(stood)) => {
            WhatTheHeldJobsAnswerLeft::AnotherAnswerStood(stood)
        }
        (_, came_to) => WhatTheHeldJobsAnswerLeft::TheAnswer(came_to),
    }
}

// ════════════════════════════════════════════════════════════════════════════
// What the run holds about each server's work
// ════════════════════════════════════════════════════════════════════════════

/// **What a standing period holds of its machine's work**, and none of it is
/// durable.
#[derive(Debug, Clone, Default)]
pub struct WhatAPeriodHolds {
    /// **`Queue & History` C6: the last-known queue reading, period-held.**
    pub reading: Option<Reading<TheMachinesWorkingState>>,
    /// **`Queue & History` A5's offer, composed once at step 7** and never by a
    /// pass (`Servers` Q18).
    pub the_offer: Vec<Identity>,
    /// **`Queue & History` C7's trigger: the break is declared**, and the
    /// reading reads as not updating from that moment until a later
    /// connection's step 7 takes a fresh one.
    pub the_break_is_declared: bool,
    /// Which live connection of this period the reading is for.
    connection: u64,
    /// The connection that actually took `reading`, rather than one that
    /// inherited its last-known display (`Download` J6).
    reading_connection: Option<u64>,
    /// **When each rendering job's figures last moved**, keyed by the job:
    /// the figures' own moment on the machine's clock, and the moment, on this
    /// computer's clock, of the first reading that carried it. **Two clocks,
    /// never compared with each other** (`State, identity and recovery` D5):
    /// the machine's moments are only ever compared for equality, and the
    /// span is measured on this computer's. See
    /// [`WhatAPeriodHolds::the_figures_to_draw`]. Period-held. Piece 6b.
    the_figures_unmoved_since: HashMap<String, (SystemTime, SystemTime)>,
}

/// **The build runner's ruling 3 of 2026-10-03 on the Creator's notes on
/// 0.1.8, piece 6a's account §6: old figures after a worker dies are not
/// drawn.** Three of the worker's two-second report intervals (`Queue &
/// History` R5): figures that have not moved for longer than this, as the
/// queue readings on this computer saw them, read as none — `Queue & History`
/// R4's *absent stays absent*, never a stale bar.
pub const FIGURES_THAT_HAVE_NOT_MOVED_FOR_THIS_LONG_ARE_NOT_DRAWN: std::time::Duration =
    std::time::Duration::from_secs(6);

impl WhatAPeriodHolds {
    /// **A rendering job's figures, where they are to be drawn** — `Queue &
    /// History` R5's, from the period-held reading. **None** while the
    /// reading is not updating (`Queue & History` C7: never a bar after *Not
    /// updating*), where the job carries none (R4), and where they have not
    /// moved for longer than
    /// [`FIGURES_THAT_HAVE_NOT_MOVED_FOR_THIS_LONG_ARE_NOT_DRAWN`] by the time
    /// of this reading — the worker that sent them has stopped reporting, and
    /// the job still reads *rendering* until reconciliation says otherwise
    /// (piece 6a's account §6). Piece 6b.
    ///
    /// **How the ruling is kept without comparing two clocks.** The ruling
    /// reads *older than six seconds beyond the queue reading's own*; the
    /// figures' moment is on the machine's clock and the reading's on this
    /// computer's, and `State, identity and recovery` D5 forbids taking one from the other. So *older* is
    /// measured as *unchanged*: the figures' moment is compared only with
    /// itself, and the span from the first reading that carried it to this
    /// one is measured on this computer's clock.
    pub fn the_figures_to_draw<'r>(
        &self,
        job: &'r AJobAsTheMachineReadsIt,
    ) -> Option<&'r smoozy_queue_history::readings::AWorkersReport> {
        if self.the_break_is_declared {
            return None;
        }
        let figures = job.live.the_figures()?;
        let reading_at = self.reading.as_ref()?.taken_at();
        match self.the_figures_unmoved_since.get(&job.job.as_text()) {
            Some((moment, since)) if *moment == figures.readings.taken_at() => {
                let unmoved = reading_at.duration_since(*since).unwrap_or_default();
                (unmoved <= FIGURES_THAT_HAVE_NOT_MOVED_FOR_THIS_LONG_ARE_NOT_DRAWN)
                    .then_some(figures)
            }
            _ => Some(figures),
        }
    }

    /// **Each rendering job's figures, against the reading before**: a moment
    /// that moved starts its span again at this reading; one that did not
    /// keeps the span it had. Jobs no longer carrying figures are dropped.
    fn the_figures_moved_or_not(&mut self, reading: &Reading<TheMachinesWorkingState>) {
        let mut next = HashMap::new();
        for job in &reading.value().jobs {
            let Some(figures) = job.live.the_figures() else {
                continue;
            };
            let moment = figures.readings.taken_at();
            let since = match self.the_figures_unmoved_since.get(&job.job.as_text()) {
                Some((held, since)) if *held == moment => *since,
                _ => reading.taken_at(),
            };
            next.insert(job.job.as_text(), (moment, since));
        }
        self.the_figures_unmoved_since = next;
    }
}

/// **Every server's work, as this run holds it.** Kept on the run's register
/// of periods (`crate::servers::a_period::ThePeriodsThisRun::the_machines_work`).
#[derive(Debug, Default)]
pub struct WhatThisRunHolds {
    per_server: Mutex<HashMap<String, WhatAPeriodHolds>>,
    /// **P10d: whether a surface showing the queue is on screen.** The Queue
    /// screen says when it is shown and when it is put away.
    the_queue_is_on_screen: Mutex<bool>,
    /// **P10d, as the runner ruled it on 2026-09-28: the server list is such a
    /// surface too**, because `Design/Servers § A row` draws each server's
    /// rendering job and how many wait. It says when it is shown and put away.
    the_server_list_is_on_screen: Mutex<bool>,
    /// **And so is a server's own page, for that server only** — its head takes
    /// the row's work column. Which server's page is showing, where one is.
    the_servers_own_page_is_on_screen: Mutex<Option<String>>,
    /// The job whose entry the Queue screen last asked for, so a pass that
    /// draws the queue again draws the same entry.
    the_chosen_job: Mutex<Option<String>>,
    /// **Which connection each server's arrangements are being driven on**, so
    /// they are driven once per connection and never twice.
    the_arrangements_driven_on: Mutex<HashMap<String, u64>>,
    /// **The last connection number given out for each server, held for the
    /// run and never dropped with a period** — so a number is never given
    /// out twice in one run. See [`WhatThisRunHolds::a_connection_begins`].
    /// Added 2026-09-29, step 10c's transfer wave piece 3.
    the_last_number_given: Mutex<HashMap<String, u64>>,
    /// **`Download` N6's whole for each job, as the last pass in this run that
    /// had the job's inventory in hand read it**, keyed by the job identity.
    /// **Held for the run and written nowhere** — `Download` R5 makes the whole
    /// a reading of the machine rather than a durable value — and **not
    /// dropped with a period**, because it is the job's whole and not the
    /// connection's. Added 2026-09-29, step 10c's transfer wave piece 4.
    the_wholes: Mutex<HashMap<String, TheJobsWhole>>,
    /// **`Download` E2b: the jobs whose download a connection's passes have
    /// seen stand still for a minute.** Run-held, as the wholes are.
    the_downloads_not_moving: Mutex<std::collections::HashSet<String>>,
    /// **The name of each held job the machine dropped, read before the
    /// removal took its local record**, keyed by the job identity. `Queue &
    /// History` H11: the user is told, and by name, and nothing else is left
    /// that holds the name. **Held for the run so a pass read again after an
    /// interruption still names the drop**; the name the user reads is the
    /// one the notice record keeps beside the line (record version 38, the
    /// 38's piece 15). Added 2026-09-29, S3's loose ends.
    the_names_of_dropped_jobs: Mutex<HashMap<String, String>>,
    /// **`Desktop app and delivery` T18's boundary: which servers' §7 step 3
    /// has completed on the connection now live.** Process memory, written
    /// nowhere (T21). A connection beginning or ending takes the server out
    /// again, so the next step 3 is catch-up (T20). Added 2026-10-03, the 38's
    /// piece 1 — the boundary's first caller.
    the_events_come_from: Mutex<WhereTheEventsAreComingFrom>,
    /// **The jobs whose endings a live read forward applied, not yet posted**,
    /// per server. **Any read forward inside a standing period may apply one**
    /// — P10d's own pass, and the passes a stop, a clear or a cancel takes
    /// after its act — and **P10d's next pass is what posts them**, because it
    /// is the one that holds the way to the notification service. Held for the
    /// run and never written: losing one costs the reaching alone (T22). Added
    /// 2026-10-03, the 38's piece 1.
    the_endings_to_tell: Mutex<HashMap<String, Vec<Identity>>>,
    /// **What wakes `Queue & History` W4's re-check before its time** — an
    /// event applied, or a connection beginning or ending. See
    /// [`crate::servers::the_idle_recheck`]. Added 2026-10-03, the 38's piece 1.
    idle_nudges: TheIdleNudges,
    /// **`Queue & History` E8: the running entry's commentary on screen, where
    /// one is** — P10d repeats the commentary read only while it is. One panel
    /// shows at a time. Run-held, written nowhere. Piece 6c.
    the_words_on_screen: Mutex<Option<TheWordsOnScreen>>,
    /// **One commentary read at a time**, so the read a press takes at once and
    /// the read a pass takes never ask for the same bytes twice.
    one_words_read_at_a_time: Mutex<()>,
    /// **The rendering entry's own three lines, where the queue draws one** —
    /// `Desktop app and delivery` M13b's one narrowing and `Queue & History`
    /// E8's *"the entry's own three lines are on screen too"*. Run-held,
    /// written nowhere, and gone when the queue is put away. The Creator's
    /// notes on the test copy, piece 8 (note 12).
    the_entrys_lines: Mutex<Option<TheEntrysLines>>,
    /// **What each job's reading says that only its queue entry draws** —
    /// `Job files and outputs` H8's six things, keyed by the job. **Held for the
    /// run and written nowhere**: History draws none of them (`Design/Queue &
    /// History § The rest of the entry`), the queue entry exists only beside a
    /// reading of the machine, and every connection's fill-in reads them again.
    /// The 38's piece 10.
    what_each_reading_says: Mutex<HashMap<String, WhatTheReadingSays>>,
}

/// **`Download` N6's whole for one job: the narrowed inventory's count, and
/// the video statement beside it** — a whole that may grow, which is
/// `Job files and outputs` I19 and `Job files and outputs` I20. Added
/// 2026-09-29, step 10c's transfer wave piece 4.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TheJobsWhole {
    /// How many narrowed inventory entries — every entry where the
    /// arrangement is unnarrowed.
    pub files: usize,
    /// **The video statement**: the whole is stated as possibly growing.
    pub it_may_grow: bool,
}

impl WhatThisRunHolds {
    /// **A connection under this server's period is lent from now on**, and
    /// every earlier one's step 7 and passes stop being current. Answers which
    /// connection this is.
    ///
    /// **The number is the run's, not the period's, and it never starts
    /// again.** Changed 2026-09-29, step 10c's transfer wave piece 3 (the
    /// tenth-pass map's piece 1, item (ii)). **The case the count was first
    /// written for** was repairs inside one period, where it moved from one
    /// connection to the next. **The case it now also meets** is a second
    /// period in the same run: the period's end took the count with it, so
    /// the next period's first connection was number 1 again — the number the
    /// last period's own connection had had. **The last period's downloads
    /// can still be asleep in `Download` W2's pause at that moment**, holding
    /// the guard that drives them once per connection under that number
    /// ([`WhatThisRunHolds::the_arrangements_begin`]), so the new connection's
    /// downloads were refused; the old driving then woke, ended, and took the
    /// guard with it, and nothing drove a download for the rest of the period.
    /// **A pass of the last period finishing late could also land its reading
    /// on the new period's connection 1** ([`WhatThisRunHolds::a_reading_was_taken`]).
    /// Keeping the number for the run closes both; the period's reading and
    /// offer are still dropped at its end (`Queue & History` C6).
    pub fn a_connection_begins(&self, server: &str) -> u64 {
        // **T20: this connection's step 3 is catch-up again**, whatever the
        // last connection's had come to; and a re-check of the last connection
        // wakes to find it gone.
        self.the_server_is_catching_up_again(server);
        let number = self.the_next_number_for(server);
        let Ok(mut held) = self.per_server.lock() else {
            return number;
        };
        let period = held.entry(server.to_string()).or_default();
        period.connection = number;
        period.connection
    }

    /// **`Servers` K3 and K5: the break is declared**, so the reading reads as
    /// not updating (`Queue & History` C7) and the connection's passes stop.
    pub fn the_connection_ended(&self, server: &str) {
        self.the_server_is_catching_up_again(server);
        let number = self.the_next_number_for(server);
        if let Ok(mut held) = self.per_server.lock() {
            if let Some(period) = held.get_mut(server) {
                period.the_break_is_declared = true;
                period.connection = number;
            }
        }
    }

    /// **The next number for this server, never given out before in this
    /// run** — a connection's own, or the one a break moves the count to so
    /// that no connection is current until the next one is lent.
    fn the_next_number_for(&self, server: &str) -> u64 {
        let Ok(mut given) = self.the_last_number_given.lock() else {
            return 0;
        };
        let last = given.entry(server.to_string()).or_insert(0);
        *last = last.wrapping_add(1);
        *last
    }

    /// **`Queue & History` C6: the period ending drops the reading entirely** —
    /// not stamped, not kept, not written anywhere. **The connection number is
    /// not the period's and is kept** (see [`WhatThisRunHolds::a_connection_begins`]).
    pub fn the_period_ended(&self, server: &str) {
        if let Ok(mut held) = self.per_server.lock() {
            held.remove(server);
        }
        self.the_server_is_catching_up_again(server);
    }

    /// **T20, and W4's *dies with the connected period*.** The server's §7
    /// step 3 is catch-up again, and anything waiting on its re-check wakes to
    /// find its connection gone.
    fn the_server_is_catching_up_again(&self, server: &str) {
        if let (Ok(mut from), Some(identity)) = (
            self.the_events_come_from.lock(),
            Identity::read_back(IdentityKind::LocalServer, server),
        ) {
            from.the_connected_period_ended(&identity);
        }
        self.idle_nudges.nudge(server);
    }

    /// **T18: whether an event read forward now, on this route, is *now* or
    /// catch-up.**
    pub fn how_an_event_read_now_arrived(
        &self,
        server: &Identity,
        arrived: HowItArrived,
    ) -> NowOrCatchingUp {
        self.the_events_come_from
            .lock()
            .map(|from| from.how_this_event_arrived(server, arrived))
            .unwrap_or(NowOrCatchingUp::CatchingUp)
    }

    /// **§7 step 3 has completed for this server on the connection now live.**
    fn the_reconciliation_read_completed(&self, server: &Identity) {
        if let Ok(mut from) = self.the_events_come_from.lock() {
            from.the_reconciliation_read_completed(server);
        }
    }

    /// **The endings a live read forward applied for this server, taken** —
    /// each once, by whichever P10d pass takes them first.
    pub fn the_endings_to_tell_are_taken(&self, server: &str) -> Vec<Identity> {
        self.the_endings_to_tell
            .lock()
            .ok()
            .and_then(|mut held| held.remove(server))
            .unwrap_or_default()
    }

    fn an_ending_to_tell(&self, server: &str, job: Identity) {
        if let Ok(mut held) = self.the_endings_to_tell.lock() {
            let jobs = held.entry(server.to_string()).or_default();
            if !jobs.contains(&job) {
                jobs.push(job);
            }
        }
    }

    /// **What wakes W4's re-check.**
    pub fn idle_nudges(&self) -> &TheIdleNudges {
        &self.idle_nudges
    }

    /// **Which connection of this server's period is the live one**, and
    /// nothing where none is — no connection lent yet, or the break declared.
    /// **It moves nothing**: `Servers` Q19a's re-take is taken on the
    /// connection that is already live, so its step 7 is that connection's and
    /// not a new one's, and a count moved here would end the arrangements the
    /// connection is already driving. Added 2026-09-27, step 10c's eighth-pass
    /// fix piece 3.
    pub fn the_connection_now(&self, server: &str) -> Option<u64> {
        self.per_server.lock().ok().and_then(|held| {
            held.get(server)
                .filter(|period| !period.the_break_is_declared)
                .map(|period| period.connection)
        })
    }

    /// Whether `connection` is still the live one under this server's period.
    ///
    /// **The connection's own count says it, and the break's mark does not.**
    /// A break moves the count ([`WhatThisRunHolds::the_connection_ended`]), so
    /// a connection that has ended is never current. **The mark is the
    /// reading's** — `Queue & History` C7's *not updating*, which stands until
    /// the next connection's step 7 takes a fresh reading. Until 2026-09-27 it
    /// was read here as well, so **a connection that followed a break was not
    /// current until its own step 7's reading landed**: the arrangements it is
    /// handed at the same moment (`Download` R12) found it not live and ended
    /// at once, and nothing drove a download again on that connection — the
    /// return after the ready action's restart among them. Step 10c's
    /// eighth-pass fix piece 3.
    pub fn is_current(&self, server: &str, connection: u64) -> bool {
        self.per_server
            .lock()
            .map(|held| {
                held.get(server)
                    .is_some_and(|period| period.connection == connection)
            })
            .unwrap_or(false)
    }

    /// **`Download` R12's arrangements begin to be driven on this connection**,
    /// and nothing where they already are — so `Servers` Q19a's re-take, which
    /// hands them the connection they were denied on a machine with no
    /// placement recorded, never starts a second driving beside the first.
    /// Added 2026-09-27, step 10c's eighth-pass fix piece 3.
    pub fn the_arrangements_begin(&self, server: &str, connection: u64) -> bool {
        let Ok(mut held) = self.the_arrangements_driven_on.lock() else {
            return false;
        };
        if held.get(server) == Some(&connection) {
            return false;
        }
        held.insert(server.to_string(), connection);
        true
    }

    /// The arrangements driven on `connection` have stopped.
    pub fn the_arrangements_ended(&self, server: &str, connection: u64) {
        if let Ok(mut held) = self.the_arrangements_driven_on.lock() {
            if held.get(server) == Some(&connection) {
                held.remove(server);
            }
        }
    }

    /// **A fresh reading, taken on `connection`.** Where the offer is handed in
    /// it is step 7's and replaces the held one; a pass hands none in and the
    /// held offer stands (`Servers` Q18).
    ///
    /// **A reading for a server with no period standing lands nowhere and
    /// makes nothing** — a pass of a period that has ended finishing late.
    /// Until 2026-09-29 it made an empty entry for the server, whose
    /// connection then read as lent with no period standing. Step 10c's
    /// transfer wave piece 3.
    pub fn a_reading_was_taken(
        &self,
        server: &str,
        connection: u64,
        reading: Reading<TheMachinesWorkingState>,
        the_offer: Option<Vec<Identity>>,
    ) {
        if let Ok(mut held) = self.per_server.lock() {
            let Some(period) = held.get_mut(server) else {
                return;
            };
            if period.connection != connection {
                return;
            }
            period.the_figures_moved_or_not(&reading);
            period.reading = Some(reading);
            period.reading_connection = Some(connection);
            period.the_break_is_declared = false;
            if let Some(offer) = the_offer {
                period.the_offer = offer;
            }
        }
    }

    /// `Download` J6: only reports actually read on the live connection.
    /// A previous connection's last-known display is not recovery evidence.
    pub fn the_saved_prefixes_reported_on_this_connection(
        &self,
        server: &str,
    ) -> HashMap<String, (u32, i64)> {
        let Ok(held) = self.per_server.lock() else {
            return HashMap::new();
        };
        let Some(period) = held.get(server).filter(|period| {
            !period.the_break_is_declared && period.reading_connection == Some(period.connection)
        }) else {
            return HashMap::new();
        };
        let Some(reading) = &period.reading else {
            return HashMap::new();
        };
        reading
            .value()
            .jobs
            .iter()
            .filter_map(|job| {
                if !matches!(job.state, QueueState::Rendering { .. }) {
                    return None;
                }
                let WhatTheEntryHasLive::Readings {
                    figures: Some(report),
                    ..
                } = &job.live
                else {
                    return None;
                };
                Some((
                    job.job.as_text(),
                    (report.attempt, report.newest_frame_saved?),
                ))
            })
            .collect()
    }

    /// **`Queue & History` A6: the offer was accepted**, so the jobs it
    /// adopted leave the held offer. **The offer is still never recomposed by
    /// a pass** (`Servers` Q18): this takes jobs out of it and puts none in.
    /// Owed piece 10.
    pub fn the_offer_was_accepted(&self, server: &str, adopted: &[Identity]) {
        if let Ok(mut held) = self.per_server.lock() {
            if let Some(period) = held.get_mut(server) {
                period.the_offer.retain(|job| !adopted.contains(job));
            }
        }
    }

    /// Every server's held work, as it stands now.
    pub fn every_server(&self) -> Vec<(String, WhatAPeriodHolds)> {
        let mut out: Vec<(String, WhatAPeriodHolds)> = self
            .per_server
            .lock()
            .map(|held| {
                held.iter()
                    .filter(|(_, period)| period.reading.is_some())
                    .map(|(server, period)| (server.clone(), period.clone()))
                    .collect()
            })
            .unwrap_or_default();
        out.sort_by(|a, b| a.0.cmp(&b.0));
        out
    }

    /// **The name a dropped held job is told by** in the account, where one
    /// was read before its local record went.
    pub fn the_name_of_a_dropped_job(&self, job: &Identity) -> Option<String> {
        self.the_names_of_dropped_jobs
            .lock()
            .ok()
            .and_then(|held| held.get(&job.as_text()).cloned())
    }

    fn a_dropped_job_is_named(&self, job: &Identity, name: String) {
        if let Ok(mut held) = self.the_names_of_dropped_jobs.lock() {
            held.insert(job.as_text(), name);
        }
    }

    /// P10d's own condition for the queue reading, for the Queue screen.
    pub fn the_queue_is_on_screen(&self) -> bool {
        self.the_queue_is_on_screen
            .lock()
            .map(|on| *on)
            .unwrap_or(false)
    }

    /// **P10d's condition for the queue reading, whole: whether a surface
    /// showing `server`'s queue or its running entry is on screen.**
    ///
    /// **Written for one case and now meeting three.** It was the Queue screen
    /// alone, so while Queue was away no reading was taken and the server
    /// list's work column — drawn from the same reading — went stale: step 10c's
    /// ninth pass read *Idle* on the list while Queue said *1 rendering*. **The
    /// runner ruled on 2026-09-28** that the server list and a server's own page
    /// are such surfaces, since `Design/Servers § A row` draws that server's
    /// rendering job and how many wait, and the page's head takes the row's
    /// words. The list shows every server, so it asks for each; a server's page
    /// shows one, so it asks for that one.
    pub fn a_surface_showing_its_work_is_on_screen(&self, server: &str) -> bool {
        self.the_queue_is_on_screen()
            || self
                .the_server_list_is_on_screen
                .lock()
                .map(|on| *on)
                .unwrap_or(false)
            || self
                .the_servers_own_page_is_on_screen
                .lock()
                .map(|on| on.as_deref() == Some(server))
                .unwrap_or(false)
    }

    /// The server list was shown, or put away.
    pub fn the_server_list_is(&self, on_screen: bool) {
        if let Ok(mut on) = self.the_server_list_is_on_screen.lock() {
            *on = on_screen;
        }
    }

    /// A server's own page was shown for `server`, or — with nothing — put
    /// away. **One page shows at a time**, so the one held is replaced.
    pub fn a_servers_own_page_is(&self, server: Option<&str>) {
        if let Ok(mut on) = self.the_servers_own_page_is_on_screen.lock() {
            *on = server.map(str::to_string);
        }
    }

    /// The Queue screen was shown, asking for this job's entry.
    pub fn the_queue_is_shown(&self, chosen: Option<&str>) {
        if let Ok(mut on) = self.the_queue_is_on_screen.lock() {
            *on = true;
        }
        if let Ok(mut held) = self.the_chosen_job.lock() {
            *held = chosen.map(str::to_string);
        }
    }

    /// The Queue screen was put away. **The words' panel goes with it**, and
    /// the commentary read stops (piece 6c). **So do the entry's three lines**
    /// (the Creator's notes on the test copy, piece 8).
    pub fn the_queue_is_put_away(&self) {
        if let Ok(mut on) = self.the_queue_is_on_screen.lock() {
            *on = false;
        }
        self.the_words_are_put_away();
        if let Ok(mut held) = self.the_entrys_lines.lock() {
            *held = None;
        }
    }

    /// **`Queue & History` E8: a rendering job's words were opened** — the
    /// read starts from nought on each sink. `name` is what the job is called,
    /// for the panel's head. Piece 6c.
    pub fn the_words_are_opened(&self, server: &str, job: &str, name: &str) {
        if let Ok(mut held) = self.the_words_on_screen.lock() {
            *held = Some(TheWordsOnScreen {
                server: server.to_string(),
                job: job.to_string(),
                name: name.to_string(),
                sinks: Default::default(),
            });
        }
    }

    /// The words' panel was closed, or the screen it sits on put away.
    pub fn the_words_are_put_away(&self) {
        if let Ok(mut held) = self.the_words_on_screen.lock() {
            *held = None;
        }
    }

    /// **The queue drew `job`'s entry, rendering on `server` with its words to
    /// read — or, with nothing, an entry that shows no lines.** Answers the
    /// newest three lines held for that job, which are none until the first
    /// read lands. A different job, or none, starts over: the next entry's
    /// first read takes the stream's last 4 KB again (`Queue & History` E8).
    /// The Creator's notes on the test copy, piece 8.
    pub fn the_entry_is_drawn_for(&self, rendering: Option<(&str, &str)>) -> Vec<String> {
        let Ok(mut held) = self.the_entrys_lines.lock() else {
            return Vec::new();
        };
        let Some((server, job)) = rendering else {
            *held = None;
            return Vec::new();
        };
        match held.as_ref() {
            Some(lines) if lines.server == server && lines.job == job => lines.lines.clone(),
            _ => {
                *held = Some(TheEntrysLines {
                    server: server.to_string(),
                    job: job.to_string(),
                    ..TheEntrysLines::default()
                });
                Vec::new()
            }
        }
    }

    /// **P10d's condition for its third read, whole**: the words' panel, or
    /// the queue's rendering entry with its three lines — both are commentary
    /// on screen (`Queue & History` E8; the Creator's notes on the test copy,
    /// piece 8).
    pub fn the_commentary_is_on_screen_for(&self, server: &str) -> bool {
        self.the_words_are_on_screen_for(server) || self.the_entrys_lines_are_on_screen_for(server)
    }

    /// **P10d's condition for the entry's lines**: the queue is on screen and
    /// its entry is a job rendering on `server`.
    pub fn the_entrys_lines_are_on_screen_for(&self, server: &str) -> bool {
        self.the_queue_is_on_screen()
            && self
                .the_entrys_lines
                .lock()
                .map(|held| held.as_ref().is_some_and(|lines| lines.server == server))
                .unwrap_or(false)
    }

    /// **P10d's condition for the commentary read**: whether `server`'s
    /// running entry's words are on screen.
    pub fn the_words_are_on_screen_for(&self, server: &str) -> bool {
        self.the_words_on_screen
            .lock()
            .map(|held| held.as_ref().is_some_and(|words| words.server == server))
            .unwrap_or(false)
    }

    /// **Which server is rendering `job`, with its words to read** — a
    /// period-held reading that is updating, in which the job is rendering and
    /// names its two sinks (`Queue & History` E8). Nothing otherwise, and then
    /// there is nothing live to open. Piece 6c.
    pub fn where_a_rendering_jobs_words_are(&self, job: &str) -> Option<String> {
        self.every_server()
            .into_iter()
            .find_map(|(server, period)| {
                if period.the_break_is_declared {
                    return None;
                }
                let reading = period.reading?;
                reading
                    .value()
                    .jobs
                    .iter()
                    .any(|one| {
                        one.job.as_text() == job
                            && matches!(one.state, QueueState::Rendering { .. })
                            && one.live.the_sinks().is_some()
                    })
                    .then_some(server)
            })
    }

    /// The job whose entry the Queue screen last asked for.
    pub fn the_chosen_job(&self) -> Option<String> {
        self.the_chosen_job
            .lock()
            .ok()
            .and_then(|held| held.clone())
    }

    /// **A pass read this job's whole** (`Download` N6), from the inventory it
    /// had in hand. The latest reading replaces the one before — a new attempt
    /// reads its own inventory (`Job files and outputs` R13). Added 2026-09-29,
    /// step 10c's transfer wave piece 4.
    pub fn a_jobs_whole_was_read(&self, job: &str, whole: TheJobsWhole) {
        if let Ok(mut held) = self.the_wholes.lock() {
            held.insert(job.to_string(), whole);
        }
    }

    /// **`Download` E2b: what the passes on a connection make of a job's
    /// download, handed over after every pass.**
    pub fn a_jobs_download_was_read(&self, job: &str, it_is_not_moving: bool) {
        if let Ok(mut held) = self.the_downloads_not_moving.lock() {
            if it_is_not_moving {
                held.insert(job.to_string());
            } else {
                held.remove(job);
            }
        }
    }

    /// **`Download` E2b's reading of a job's download.**
    pub fn the_download_is_not_moving(&self, job: &str) -> bool {
        self.the_downloads_not_moving
            .lock()
            .map(|held| held.contains(job))
            .unwrap_or(false)
    }

    /// **The whole this run has read for a job**, and nothing where no pass in
    /// this run has had its inventory in hand.
    pub fn the_whole_of(&self, job: &str) -> Option<TheJobsWhole> {
        self.the_wholes
            .lock()
            .ok()
            .and_then(|held| held.get(job).copied())
    }

    /// **A job's reading, as the fill-in read it** — the 38's piece 10.
    pub fn a_reading_was_filled_in(&self, job: &str, says: WhatTheReadingSays) {
        if let Ok(mut held) = self.what_each_reading_says.lock() {
            held.insert(job.to_string(), says);
        }
    }

    /// **What a job's reading says**, and nothing where no fill-in in this run
    /// has read one.
    pub fn what_the_reading_says(&self, job: &str) -> Option<WhatTheReadingSays> {
        self.what_each_reading_says
            .lock()
            .ok()
            .and_then(|held| held.get(job).cloned())
    }
}

// ════════════════════════════════════════════════════════════════════════════
// `Queue & History` N9 — what the render's file read produces reaches the
// entry, by the two standing routes and no third one. The 38's piece 10.
// ════════════════════════════════════════════════════════════════════════════

/// **What a job's reading says that only its queue entry draws** — `Job files
/// and outputs` H8's six things, the count they come to, and whether the frames
/// can be counted at all (`Job files and outputs` I9a).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatTheReadingSays {
    pub six: smoozy_job_files::holds::TheSixThings,
    /// **`Job files and outputs` I13's count**, where the inventory was
    /// composed.
    pub how_many_files: Option<u64>,
    /// **`Job files and outputs` I9a's two-view case**: the inventory is not
    /// composed, and the entry says so in one line.
    pub the_frames_cannot_be_counted: bool,
    /// **The job's own folder** (`Job files and outputs` M1), so a path the
    /// entry names is named beneath it, as Files shows it.
    pub job_output_folder: String,
    /// **At each frame the reading covers, whether more than one View Layer
    /// renders there** — the reading's per-frame set (`Job files and outputs`
    /// R4 item 5) with the job's pins applied by that crate's one rule, the
    /// pins read off the kept setup. The entry names the layer only where this
    /// says more than one (`Design/Queue & History`, *A rendering entry*); a
    /// frame the reading does not cover keeps naming it. The Creator's second
    /// notes, piece 6b.
    pub more_than_one_view_layer_at: std::collections::BTreeMap<i64, bool>,
}

impl WhatTheReadingSays {
    /// **Whether the entry names the View Layer at this frame**: only where
    /// more than one renders there, and — the safe side — wherever the reading
    /// does not say.
    pub fn the_view_layer_is_named_at(&self, frame: i64) -> bool {
        self.more_than_one_view_layer_at
            .get(&frame)
            .copied()
            .unwrap_or(true)
    }
}

/// **The reading of one job, out of its record directory**, and what was read
/// to compose it.
struct OneJobsRecordDirectory {
    /// The newest recorded reading the render worker wrote, where one stands.
    the_attempts_own: Option<smoozy_job_files::composing::TheReadingAndWhatFollows>,
    /// The manifest, where it reads back.
    manifest: Option<smoozy_job_files::manifest::SubmissionManifest>,
    /// Every attempt record that reads back, in attempt order.
    attempts: Vec<smoozy_job_files::attempts::AttemptRecord>,
    /// **Where the render worker recorded the job's folder at render start**
    /// (`Job files and outputs` P21), where it did — read beside the reading,
    /// on `Queue & History` N9's route, as its field of 2026-10-04.
    the_folder_chosen_at_render_start:
        Option<smoozy_job_files::where_the_folder_sits::WhereTheJobOutputFolderSits>,
}

impl OneJobsRecordDirectory {
    /// **The job output folder in force**: the manifest's, or the one recorded
    /// at render start — and nothing before either stands (`Job files and
    /// outputs` P21, `Queue & History` Y8). Never a path composed here.
    fn the_job_output_folder(&self) -> Option<String> {
        smoozy_job_files::where_the_folder_sits::the_job_output_folder_in_force(
            self.manifest
                .as_ref()
                .and_then(|manifest| manifest.job_output_folder.the_path_it_names()),
            self.the_folder_chosen_at_render_start.as_ref(),
        )
    }
}

fn one_jobs_record_directory(
    machine: &mut dyn AsksTheSupervisor,
    directory: &str,
    attempts: u32,
) -> OneJobsRecordDirectory {
    use smoozy_job_files::composing::TheReadingAndWhatFollows;
    use smoozy_job_files::manifest::{
        the_recorded_readings_file_name, SubmissionManifest, MANIFEST_FILE_NAME,
    };
    let under = |name: &str| format!("{directory}/{name}");
    let manifest = machine
        .a_file_of_the_job_record_directory(&under(MANIFEST_FILE_NAME))
        .and_then(|bytes| SubmissionManifest::read_back(&bytes));
    // **`Job files and outputs` R13 keeps one per attempt**, and an attempt
    // that kept the manifest's map writes none — so the newest that stands is
    // the reading in force.
    let the_attempts_own = (1..=attempts).rev().find_map(|attempt| {
        machine
            .a_file_of_the_job_record_directory(&under(&the_recorded_readings_file_name(attempt)))
            .and_then(|bytes| TheReadingAndWhatFollows::read_back(&bytes))
    });
    let attempts = (1..=attempts)
        .filter_map(|attempt| the_attempt_record_of(machine, directory, attempt))
        .collect();
    // **Read only where the manifest names no folder**: a manifest that names
    // one was chosen for at submission, and no record stands beside it.
    let the_folder_chosen_at_render_start = manifest
        .as_ref()
        .filter(|manifest| manifest.job_output_folder.the_path_it_names().is_none())
        .and_then(|_| {
            machine.a_file_of_the_job_record_directory(&under(
                smoozy_job_files::where_the_folder_sits::THE_RECORD_OF_WHERE_THE_FOLDER_SITS,
            ))
        })
        .and_then(|bytes| {
            smoozy_job_files::where_the_folder_sits::WhereTheJobOutputFolderSits::read_back(&bytes)
        });
    OneJobsRecordDirectory {
        the_attempts_own,
        manifest,
        attempts,
        the_folder_chosen_at_render_start,
    }
}

/// **One attempt record of a job, out of its record directory** — `Job files
/// and outputs` M8's, the one the render worker writes before it launches
/// Blender. `None` where it does not stand or will not read back.
fn the_attempt_record_of(
    machine: &mut dyn AsksTheSupervisor,
    directory: &str,
    attempt: u32,
) -> Option<smoozy_job_files::attempts::AttemptRecord> {
    machine
        .a_file_of_the_job_record_directory(&format!("{directory}/attempt-{attempt}.json"))
        .and_then(|bytes| smoozy_job_files::attempts::AttemptRecord::read_back(&bytes))
}

/// **The attempt number a render attempt's start names** — the machine's own
/// words for the event, *attempt N* (`Rented-server runtime — program`'s
/// `start_a_render_attempt`; [`tests::the_machine_names_an_attempt_the_way_it_is_read_here`]
/// fails if they move). `None` where the words say nothing else.
fn the_attempt_a_start_names(says: Option<&str>) -> Option<u32> {
    says?.strip_prefix("attempt ")?.trim().parse().ok()
}

/// **`Queue & History` N1: the frame a re-run proceeded from, which is the
/// first frame of the re-run attempt's list** — read off that attempt's own
/// record, never computed. `None` where the record does not stand yet (the
/// render worker writes it after the start is journalled), where the server
/// row records no layout, or where the list is empty, which is not a zero.
fn the_first_frame_of_an_attempt(
    row: &smoozy_servers::store::ServerRow,
    machine: &mut dyn AsksTheSupervisor,
    held: &smoozy_queue_history::record::LocalJobRecord,
    attempt: u32,
) -> Option<i64> {
    let layout = crate::servers::installation::the_layout_recorded_for(row)?;
    let paths = smoozy_job_files::publication::AJobsPaths::composed(
        &layout,
        &held.job,
        held.label.as_deref(),
    )
    .ok()?;
    the_attempt_record_of(machine, &paths.job_record_directory, attempt)?
        .frame_list_in_the_order_it_was_written
        .first()
        .copied()
}

/// **A re-run's frame, read after its event** — the 38's piece 15.
///
/// The render worker writes the re-run attempt's record after the supervisor
/// journals the attempt's start, so a pass that applies the start before the
/// record stands composes the line with no frame ([`the_read_forward_applied`]).
/// **Every pass after it reads that attempt's record again** and, once it
/// stands, writes the frame into the standing notice — the one write a line
/// takes after it was composed, and only a line with no frame yet takes it
/// (`NoticeRecord::the_re_runs_frame_is_known`). **Nothing is read where the
/// notice names no such line**, and the record is read again before the write,
/// so a *Dismiss* that landed while the machine was asked is not undone.
fn the_re_runs_frames_are_read(
    server: &Identity,
    record: &dyn LendsTheRecord,
    machine: &mut dyn AsksTheSupervisor,
) {
    use crate::record::catch_up_notices::{the_notice_is_written, the_standing_notice};
    let Some(Some(standing)) = with_the_record(record, |connection| {
        the_standing_notice(connection, server).ok().flatten()
    }) else {
        return;
    };
    let owed = standing.the_re_runs_whose_frame_is_not_known();
    if owed.is_empty() {
        return;
    }
    let Some(Some(row)) = with_the_record(record, |connection| {
        server_rows::one(connection, &server.as_text())
            .ok()
            .flatten()
    }) else {
        return;
    };
    let mut read = Vec::new();
    for (job, attempt) in owed {
        let Some(Some(held)) = with_the_record(record, |connection| {
            JobRecords::over(connection).read(&job).ok().flatten()
        }) else {
            continue;
        };
        if let Some(first) = the_first_frame_of_an_attempt(&row, machine, &held, attempt) {
            read.push((job, attempt, first));
        }
    }
    if read.is_empty() {
        return;
    }
    with_the_record(record, |connection| {
        let Some(mut now) = the_standing_notice(connection, server).ok().flatten() else {
            return;
        };
        let mut changed = false;
        for (job, attempt, first) in &read {
            changed |= now.the_re_runs_frame_is_known(job, *attempt, *first);
        }
        if changed {
            let _ = the_notice_is_written(connection, &now);
        }
    });
}

/// **`Queue & History` N1: the frame a re-run proceeded from, landed on the
/// job record's own mark** — the mark Queue and History draw (`Design/Queue &
/// History`, *The marks*, the fifth; `Specs/Queue & History`, *interrupted and
/// re-run from a given frame*). Added 2026-10-24, on the second finding of
/// step 12's reboot scenario.
///
/// **The machine cannot write it**: its note takes the mark when the re-run
/// attempt starts, and the attempt's list is the render worker's, composed
/// after that (`Rented-server runtime — program`'s `start_a_render_attempt`).
/// So the desktop reads it where it is written, off the re-run attempt's own
/// record, exactly as the catch-up notice's line does
/// ([`the_first_frame_of_an_attempt`]), and lands it on the job record's mark.
/// **A frame once known is never changed, and a later fill-in from the note
/// does not take it away** (`JobRecords::the_re_runs_frame_is_known`).
///
/// **Only the marks still without a frame are read**, and the render worker
/// writes the attempt's record before it launches Blender, so in the ordinary
/// case a mark is read once, at the first pass after the re-run began.
fn the_re_run_marks_are_given_their_frames(
    server: &Identity,
    record: &dyn LendsTheRecord,
    machine: &mut dyn AsksTheSupervisor,
) {
    let Some(Ok(owed)) = with_the_record(record, |connection| {
        JobRecords::over(connection).the_re_runs_whose_frame_is_not_known(server)
    }) else {
        return;
    };
    if owed.is_empty() {
        return;
    }
    let Some(Some(row)) = with_the_record(record, |connection| {
        server_rows::one(connection, &server.as_text())
            .ok()
            .flatten()
    }) else {
        return;
    };
    let mut read = Vec::new();
    for (job, attempt) in owed {
        let Some(Some(held)) = with_the_record(record, |connection| {
            JobRecords::over(connection).read(&job).ok().flatten()
        }) else {
            continue;
        };
        if let Some(first) = the_first_frame_of_an_attempt(&row, machine, &held, attempt) {
            read.push((job, attempt, first));
        }
    }
    if read.is_empty() {
        return;
    }
    with_the_record(record, |connection| {
        let records = JobRecords::over(connection);
        for (job, attempt, first) in &read {
            let _ = records.the_re_runs_frame_is_known(job, *attempt, *first);
        }
    });
}

/// **`Rendering` A4's read of the machine's own attempt record, out of the job's
/// record directory** — the latest attempt record that carries a frame list
/// (`Job files and outputs` C18a's rule, the record P1 step 6 reads the list
/// from), and the inventory for the map that attempt held in force: the
/// newest recorded reading the render worker wrote, or where none stands the
/// one the manifest carries. Read through [`one_jobs_record_directory`], the
/// same reading the fill-in takes, and **nothing durable is written from it**.
/// `None` where no attempt record carries a list or no reading stands to
/// give the inventory — `Rendering` A4 sends both to `Rendering` A9's could-not-check case. The job
/// output folder the manifest names comes back beside it (`Job files and
/// outputs` M1). The 38's piece 2.
pub(crate) fn the_map_in_force_on_the_machine(
    machine: &mut dyn AsksTheSupervisor,
    record_directory: &str,
    attempts: u32,
) -> Option<(
    smoozy_rendering::acting::page_readings::TheMapInForceRead,
    Option<String>,
)> {
    let read = one_jobs_record_directory(machine, record_directory, attempts);
    let the_frame_list_asked_for = read
        .attempts
        .iter()
        .rev()
        .find(|attempt| !attempt.frame_list_in_the_order_it_was_written.is_empty())?
        .frame_list_in_the_order_it_was_written
        .clone();
    let the_inventory = match (&read.the_attempts_own, &read.manifest) {
        (Some(own), _) => own.inventory.clone(),
        (None, Some(manifest)) => match &manifest.recorded_reading {
            smoozy_job_files::manifest::TheRecordedReading::ItWasReadBeforeSubmitting(derived) => {
                derived.inventory.clone()
            }
            smoozy_job_files::manifest::TheRecordedReading::ThereIsNone => return None,
        },
        (None, None) => return None,
    };
    Some((
        smoozy_rendering::acting::page_readings::TheMapInForceRead {
            the_frame_list_asked_for,
            the_inventory,
        },
        read.the_job_output_folder(),
    ))
}

/// **`Queue & History` N9, performed for the jobs named**: each one's recorded
/// reading read from its record directory, landed on the local record by
/// [`JobRecords::the_read_is_filled_in`] — `Queue & History` N10's merge rule
/// is that call's and is not applied here — and its six things held for the
/// queue entry. Returns how many records it filled in.
///
/// **Who calls it, and it is two routes and no third** (`Queue & History` N9,
/// `Job files and outputs` H9):
/// - **the connection's fill-in** — [`the_step_seven`], beside §7 step 4, for
///   every record of this server still in the queue or with no reading yet;
/// - **the journalled event** — [`the_read_forward_pass`], on the reconciliation
///   route and on P10d's alike, for the jobs a recorded reading's event, a
///   render attempt's start or a recorded outcome names.
///
/// **The live figures the queue reading carries are not a route for it**
/// (notes piece 6a): they are period-held and say nothing the reading says.
///
/// **What is read, and from where.** The newest recorded reading the render
/// worker wrote (`Job files and outputs` H9), or where none stands the one the
/// manifest carries (a job read before submitting, `Job files and outputs`
/// M1); and every attempt record (`Job files and outputs` M8), for
/// `Queue & History` N9a's two facts and the asked order. **A job before its
/// render's step 4 has no reading** and nothing is filled from one.
pub fn the_readings_are_filled_in(
    server: &Identity,
    record: &dyn LendsTheRecord,
    machine: &mut dyn AsksTheSupervisor,
    holds: &WhatThisRunHolds,
    jobs: &[Identity],
) -> usize {
    use smoozy_queue_history::record::{
        AnAttemptWhereTheFileChanged, TheFramesAsked, TheFramesTheReadFound, WhatTheReadProduced,
    };
    let Some(Some(row)) = with_the_record(record, |connection| {
        server_rows::one(connection, &server.as_text())
            .ok()
            .flatten()
    }) else {
        return 0;
    };
    let Some(layout) = crate::servers::installation::the_layout_recorded_for(&row) else {
        return 0;
    };
    let mut filled = 0;
    for job in jobs {
        let Some(Some(held)) = with_the_record(record, |connection| {
            JobRecords::over(connection).read(job).ok().flatten()
        }) else {
            continue;
        };
        let attempts = held.what_the_note_carried.attempts;
        if &held.server.local_server != server
            || held.standing != Standing::Confirmed
            || !held.the_render_start_has_been_learned
            || attempts == 0
        {
            continue;
        }
        let Ok(paths) = smoozy_job_files::publication::AJobsPaths::composed(
            &layout,
            job,
            held.label.as_deref(),
        ) else {
            continue;
        };
        let read = one_jobs_record_directory(machine, &paths.job_record_directory, attempts);
        // **`Job files and outputs` M1: the folder the manifest names and the
        // folder made are one value** — and since 2026-10-04 (`Job files and
        // outputs` P21) the folder the render worker chose at render start and
        // recorded. **The layout's own composition stands in only where no
        // manifest reads back**, which is a job submitted before §6.6 or a
        // record directory that cannot be read; **where the manifest says the
        // folder is chosen at render start and none is recorded yet, nothing
        // is filled in** — the job has no folder (`Queue & History` Y8).
        let job_output_folder = match (&read.manifest, read.the_job_output_folder()) {
            (_, Some(folder)) => folder,
            (None, None) => paths.job_output_folder.clone(),
            (Some(_), None) => continue,
        };
        let derived = match (&read.the_attempts_own, &read.manifest) {
            (Some(own), _) => Some(own.as_the_manifest_carries_it()),
            (None, Some(manifest)) => match &manifest.recorded_reading {
                smoozy_job_files::manifest::TheRecordedReading::ItWasReadBeforeSubmitting(
                    derived,
                ) => Some((**derived).clone()),
                smoozy_job_files::manifest::TheRecordedReading::ThereIsNone => None,
            },
            (None, None) => None,
        };
        // **The scene the reading read**: the job's own where it named one
        // (Smoozy's own record wins, `Queue & History` N10), otherwise the one
        // the reading names — never the file's first scene, which nothing says
        // is the one rendered (`Rendering` L16).
        let the_scene_it_read = read
            .the_attempts_own
            .as_ref()
            .and_then(|own| own.reading.the_scene_it_read.as_ref())
            .and_then(|name| String::from_utf8(name.as_bytes().to_vec()).ok());
        let scene_name = held.scene_name.clone().or(the_scene_it_read);
        let picture_size = read.the_attempts_own.as_ref().and_then(|own| {
            let settings =
                <smoozy_render_settings::reading::SettingsHalf as serde::Deserialize>::deserialize(
                    &own.reading.settings.0,
                )
                .ok()?;
            let named = scene_name.as_deref()?;
            let scene = settings
                .scenes
                .iter()
                .find(|scene| scene.name.as_bytes() == named.as_bytes())?;
            smoozy_render_settings::reading::the_size_at(
                scene.full_size,
                held.resolution_percentage_typed
                    .unwrap_or(scene.resolution_percentage),
            )
        });
        let main_output = derived
            .as_ref()
            .map(|derived| the_main_outputs_folder(derived, &job_output_folder));
        let the_frames_cannot_be_counted = derived.as_ref().is_some_and(|derived| {
            matches!(
                derived.inventory,
                smoozy_job_files::inventory::WhatTheInventoryIs::TheFrameSetIsNotPredictable(
                    smoozy_job_files::inventory::WhyTheFrameSetIsNotPredictable::TheSceneWritesMoreThanOneViewSeparately
                )
            )
        });
        let produced = WhatTheReadProduced {
            // **`Queue & History` N9's field of 2026-10-04**: where the render
            // worker chose the job's folder, with its case.
            the_job_output_folder: read.the_folder_chosen_at_render_start.clone(),
            scene_name: derived.as_ref().and(scene_name),
            picture_size,
            main_output_or_the_jobs_folder: main_output,
            attempts_where_the_file_changed: read
                .attempts
                .iter()
                .filter_map(AnAttemptWhereTheFileChanged::read_from)
                .collect(),
            the_frames: TheFramesTheReadFound {
                // **The first attempt record that stands is every frame the job
                // asked for**: its folder is its own and nothing in it was
                // already there. A held attempt writes none and renders
                // nothing, so the next one is that list.
                asked: read.attempts.first().and_then(|first| {
                    TheFramesAsked::from_the_order(&first.frame_list_in_the_order_it_was_written)
                }),
                they_cannot_be_counted: the_frames_cannot_be_counted,
            },
        };
        if produced == WhatTheReadProduced::default() {
            continue;
        }
        let written = with_the_record(record, |connection| {
            JobRecords::over(connection).the_read_is_filled_in(job, &produced)
        });
        if matches!(
            written,
            Some(Ok(
                crate::record::job_records::WhatTheFillInDid::ItCarriedTheNotesHalfOver
            ))
        ) {
            filled += 1;
        }
        if let Some(derived) = &derived {
            // **The job's pins, off the kept setup Submit wrote beside the
            // record** — the same payload the machine renders by, in `Job files
            // and outputs`' shape. No kept setup, no pins: the file's own
            // switches stand. The Creator's second notes, piece 6b.
            let kept = with_the_record(record, |connection| {
                JobRecords::over(connection)
                    .the_kept_setup_of(job)
                    .ok()
                    .flatten()
            })
            .flatten();
            let the_jobs_pins = crate::surface::render_settings::the_ground_of_a_submitted_job(
                None,
                match &kept {
                    Some(crate::record::job_records::TheKeptSetup::Kept(payload)) => Some(payload),
                    _ => None,
                },
            )
            .the_jobs_pins;
            let more_than_one_view_layer_at =
                smoozy_job_files::reading::the_per_frame_set_with_the_jobs_pins(
                    &derived.reading.per_frame,
                    &the_jobs_pins,
                )
                .iter()
                .map(|at| (at.frame, at.view_layers_that_render.len() > 1))
                .collect();
            holds.a_reading_was_filled_in(
                &job.as_text(),
                WhatTheReadingSays {
                    six: smoozy_job_files::holds::the_six_things(
                        &derived.reading,
                        &derived.map,
                        &derived.inventory,
                        &job_output_folder,
                    ),
                    how_many_files: match &derived.inventory {
                        smoozy_job_files::inventory::WhatTheInventoryIs::It(inventory) => {
                            Some(inventory.the_count() as u64)
                        }
                        _ => None,
                    },
                    the_frames_cannot_be_counted,
                    job_output_folder: job_output_folder.clone(),
                    more_than_one_view_layer_at,
                },
            );
        }
    }
    filled
}

/// **What *show outputs* opens, and the entry's *Writing to***: the main
/// output's own folder where the beauty reached a destination inside a file
/// output node (`Job files and outputs` B7), and the job's own folder
/// otherwise — the regular output path's case and `Job files and outputs`
/// B8's *no main output* alike,
/// which that crate's composition says are *"told apart by the folder show
/// outputs opens, which is the job's own folder in both"*. **The pick is read
/// and never made again** (`Job files and outputs` B15): it is the one the
/// reading carries.
fn the_main_outputs_folder(
    derived: &smoozy_job_files::manifest::TheReadingAndEverythingDerivedFromIt,
    job_output_folder: &str,
) -> String {
    use smoozy_job_files::mapping::HowItMapped;
    use smoozy_job_files::reading::DestinationKind;
    let Some(main) = derived.picks.the_main_output.as_ref() else {
        return job_output_folder.to_string();
    };
    derived
        .map
        .pairs
        .iter()
        .find(|pair| match &pair.destination.kind {
            // **`Job files and outputs` R4 item 2**: from 5.3 by the node's
            // effect as well.
            DestinationKind::AnOutputOfAFileOutputNode { node, output, the_effect, .. } => {
                node == &main.node && main.output.as_ref() == Some(output) && the_effect == &main.the_effect
            }
            DestinationKind::AWholeFileOutputNodeGatheringIntoOneFile { node, the_effect, .. } => {
                node == &main.node && main.output.is_none() && the_effect == &main.the_effect
            }
            _ => false,
        })
        .and_then(|pair| match &pair.outcome {
            HowItMapped::UnderTheJobsFolder { new_folder, .. } => {
                String::from_utf8(new_folder.clone()).ok()
            }
            HowItMapped::NotMappable(_) => None,
        })
        .unwrap_or_else(|| job_output_folder.to_string())
}

// ════════════════════════════════════════════════════════════════════════════
// Step 7
// ════════════════════════════════════════════════════════════════════════════

/// One read or one write against the local record, answered as a value.
/// **L4: the record is lent for one act and never held across a request.**
pub(crate) fn with_the_record<T>(
    record: &dyn LendsTheRecord,
    act: impl FnOnce(&rusqlite::Connection) -> T,
) -> Option<T> {
    let mut act = Some(act);
    let mut out = None;
    record.lend(&mut |connection| {
        if let Some(act) = act.take() {
            out = Some(act(connection));
        }
    });
    out
}

/// **What step 7 did on one connection**, as much as a test or the account
/// wants to see.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct WhatStepSevenDid {
    pub confirmed: usize,
    pub removed: usize,
    pub still_provisional: usize,
    pub events_applied: usize,
    pub filled_in: usize,
    pub the_queue_was_read: bool,
    pub offered: usize,
}

/// **`State, identity and recovery` V1 step 2, at that document's §7**: every
/// provisional record of this server, asked about and resolved.
///
/// **The branch is `Queue & History`'s own `what_step_2_does`, reached through
/// the store**, so there is nowhere in this function a timeout could be put.
pub fn every_provisional_record_is_resolved(
    server: &Identity,
    record: &dyn LendsTheRecord,
    machine: &mut dyn AsksTheSupervisor,
    did: &mut WhatStepSevenDid,
) {
    let provisional: Vec<Identity> = with_the_record(record, |connection| {
        JobRecords::over(connection)
            .provisional_for(server)
            .unwrap_or_default()
    })
    .unwrap_or_default()
    .into_iter()
    .map(|one| one.job)
    .collect();
    the_provisional_records_named_are_resolved(record, machine, &provisional, did);
}

/// **`State, identity and recovery` V1 step 2's ask and its branch, for the
/// provisional records named** — every one of this server's at a connection's
/// step 7, and only those an event names at a pass
/// ([`a_read_forward_pass`]; step 10c's twelfth-pass fix piece 4). A record
/// that is no longer provisional when its turn comes is left alone: the branch
/// is `JobRecords::the_resolution`'s, which acts only on a provisional record.
fn the_provisional_records_named_are_resolved(
    record: &dyn LendsTheRecord,
    machine: &mut dyn AsksTheSupervisor,
    named: &[Identity],
    did: &mut WhatStepSevenDid,
) {
    for job in named {
        let said = what_the_machine_says_about(machine, job);
        match said {
            WhatTheMachineSaidAboutTheJob::ItHoldsTheJob => did.confirmed += 1,
            WhatTheMachineSaidAboutTheJob::ItsOperationRecordShowsTheSubmissionWasRefusedOrNeverTaken => {
                did.removed += 1
            }
            WhatTheMachineSaidAboutTheJob::TheMachineCannotSay => did.still_provisional += 1,
        }
        let _ = with_the_record(record, |connection| {
            JobRecords::over(connection).the_resolution(job, said)
        });
    }
}

/// **`State, identity and recovery` V1 step 4**: every record of this server
/// the machine has and whose ending Smoozy has not learned, filled in from the
/// machine's own note. **A provisional record is not asked about here** — it
/// is not known to be the machine's — and a record marked *start never heard*
/// is not among them, its machine being gone.
pub fn every_record_is_filled_in(
    server: &Identity,
    record: &dyn LendsTheRecord,
    machine: &mut dyn AsksTheSupervisor,
) -> usize {
    let unfinished = with_the_record(record, |connection| {
        JobRecords::over(connection)
            .with_no_learned_ending_for(server)
            .unwrap_or_default()
    })
    .unwrap_or_default();
    let mut filled = 0;
    for one in unfinished
        .into_iter()
        .filter(|one| one.standing == Standing::Confirmed)
    {
        let Some(note) = the_note_of(machine, &one.job) else {
            continue;
        };
        let written = with_the_record(record, |connection| {
            JobRecords::over(connection).the_fill_in(&one.job, &note)
        });
        if matches!(
            written,
            Some(Ok(
                crate::record::job_records::WhatTheFillInDid::ItCarriedTheNotesHalfOver
            ))
        ) {
            filled += 1;
        }
    }
    filled
}

/// **How one read-forward pass ended.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HowThePassEnded {
    /// Every event the journal holds past the cursor was applied, and the
    /// cursor stands past them.
    ItReadToTheEnd { applied: usize },
    /// **`State, identity and recovery` V2: a read that could not complete is
    /// said, not shown as a whole account.** The cursor did not move past
    /// anything that was not applied.
    TheReadCouldNotComplete { applied: usize },
}

/// **`State, identity and recovery` V1 step 3 — and P10d's read forward, which
/// is the same mechanism read at another moment** (`Queue & History` C1a).
///
/// For each read the machine answers, in this order and never another:
///
/// 1. **the events are applied** — every record of this server that the
///    machine has and whose ending is not learned is filled in from its note,
///    which is idempotent by construction (`State, identity and recovery` E4);
/// 2. **the account is composed** from what was applied (`Queue & History`
///    C1, `Queue & History` C1a), with a gap said where the journal no longer holds the cursor
///    (`Queue & History` C4), **and written into the local record** (`Queue &
///    History` C2, record version 38);
/// 3. **the cursor is written to the server row**, durably, only now —
///    `State, identity and recovery` E3's *after the work is done*;
/// 4. **the machine is told**, for its retention floor alone.
///
/// **This is the read forward taken inside a standing period** — P10d's own
/// pass, and the passes a stop, a clear or a cancel takes after its act — so
/// **an ending it applies after §7 step 3 has completed is *now*** (`Desktop
/// app and delivery` T18) and is put out for P10d's next pass to post. Step 3
/// itself is [`the_read_forward_pass`] on the reconciliation route, and posts
/// nothing.
pub fn a_read_forward_pass(
    server: &Identity,
    record: &dyn LendsTheRecord,
    machine: &mut dyn AsksTheSupervisor,
    holds: &WhatThisRunHolds,
) -> HowThePassEnded {
    the_read_forward_pass(
        server,
        record,
        machine,
        holds,
        HowItArrived::OnTheLiveReading,
    )
}

/// **The read forward, on the route it was taken by** — `Desktop app and
/// delivery` T18's two, and there is no third.
///
/// **Only the classification differs between them; the application is one
/// mechanism** (`Queue & History` C1a). An ending applied on the live route
/// once step 3 has completed is put out to be posted — **only after the
/// cursor stands past it**, so a read that is taken again after an
/// interruption never puts the same ending out twice. **A live pass that reads
/// to the end on a connection whose step 3 did not complete completes it**: what
/// it read was step 3's remainder, and is catch-up (T20); what comes after is
/// *now*. The 38's piece 1, 2026-10-03.
pub fn the_read_forward_pass(
    server: &Identity,
    record: &dyn LendsTheRecord,
    machine: &mut dyn AsksTheSupervisor,
    holds: &WhatThisRunHolds,
    arrived: HowItArrived,
) -> HowThePassEnded {
    let server_text = server.as_text();
    let now_or_catching_up = holds.how_an_event_read_now_arrived(server, arrived);
    let ended = the_read_forward_applied(server, record, machine, holds, now_or_catching_up);
    the_re_runs_frames_are_read(server, record, machine);
    the_re_run_marks_are_given_their_frames(server, record, machine);
    if arrived == HowItArrived::OnTheLiveReading
        && now_or_catching_up == NowOrCatchingUp::CatchingUp
        && matches!(ended, HowThePassEnded::ItReadToTheEnd { .. })
        && holds.the_connection_now(&server_text).is_some()
    {
        holds.the_reconciliation_read_completed(server);
    }
    ended
}

fn the_read_forward_applied(
    server: &Identity,
    record: &dyn LendsTheRecord,
    machine: &mut dyn AsksTheSupervisor,
    holds: &WhatThisRunHolds,
    now_or_catching_up: NowOrCatchingUp,
) -> HowThePassEnded {
    let server_text = server.as_text();
    let mut applied_in_all = 0usize;
    let Some(Some(row)) = with_the_record(record, |connection| {
        server_rows::one(connection, &server_text).ok().flatten()
    }) else {
        return HowThePassEnded::TheReadCouldNotComplete { applied: 0 };
    };
    // **A cursor the desktop never held is a reader starting at the beginning
    // of this installation's own journal**, and missing nothing it tracked.
    let the_desktop_held_a_cursor = row.event_cursor.is_some();
    let mut from = row.event_cursor.unwrap_or(0);
    let mut the_first_read = true;
    loop {
        let Some(read) = the_read_forward_from(machine, from) else {
            return HowThePassEnded::TheReadCouldNotComplete {
                applied: applied_in_all,
            };
        };
        let gap = the_first_read
            && the_desktop_held_a_cursor
            && read.oldest_retained.is_some_and(|oldest| oldest > from);
        the_first_read = false;
        if read.events.is_empty() && !gap {
            return HowThePassEnded::ItReadToTheEnd {
                applied: applied_in_all,
            };
        }

        // **`Queue & History` H11: a held job the machine dropped is told, by
        // name** — so the name is read here, before the removal below takes
        // the local record that holds it. S3's loose ends, 2026-09-29.
        let dropped = the_held_jobs_dropped_in(&read.events, server, record, holds);
        // **`Queue & History` L22: a waiting job dropped for a removed Blender
        // is told the same way**, named before the removal below takes its
        // local record. The 38's piece 14c.
        let dropped_for_a_removed_blender =
            the_waiting_jobs_dropped_for_a_removed_blender_in(&read.events, server, record, holds);

        // 1. The application.
        //
        // **A job record the machine removed takes the local record with it**
        // — `Queue & History` L23's last bullet (the stop and clear plan
        // pass's SC-1), which is `State, identity and recovery` H8's first
        // bullet applied at that document's §7 step 3: provisional or
        // confirmed alike, as `Queue & History` L2 does for a cancel. **Only a
        // job of this server, and only the one the event names** — nothing is
        // inferred from a job missing from a reading (`State, identity and
        // recovery` D2). **A second arrival for the same job finds nothing and
        // is nothing** (`State, identity and recovery` E4). Applied first, so a
        // removed provisional record is not asked about below. Step 10c's S2.
        for event in read
            .events
            .iter()
            .filter(|event| event.kind == THE_JOB_RECORD_REMOVED_EVENT)
        {
            let Some(job) = event
                .about
                .as_deref()
                .and_then(|text| Identity::read_back(IdentityKind::Job, text))
            else {
                continue;
            };
            let removed = with_the_record(record, |connection| {
                let records = JobRecords::over(connection);
                match records.read(&job) {
                    Ok(Some(held)) if &held.server.local_server == server => {
                        records.remove(&job).is_ok()
                    }
                    Ok(_) => true,
                    Err(_) => false,
                }
            });
            // **A removal the record would not take moves no cursor**, so the
            // event is read again at the next pass (`State, identity and
            // recovery` E3).
            if removed != Some(true) {
                return HowThePassEnded::TheReadCouldNotComplete {
                    applied: applied_in_all,
                };
            }
        }

        // **An event about a job this desktop still holds as provisional is
        // answered by `State, identity and recovery` V1 step 2's own ask**, made
        // again: the submission's acknowledgement confirms nothing
        // (`Queue & History`'s `resolve` asks only after a lost one), so a job
        // submitted inside a standing period would otherwise stay provisional —
        // never filled in, never in history — until the next connection. The
        // ask and its three branches are step 2's, unchanged, and the third
        // still removes nothing.
        //
        // **Only the provisional records an event names are asked about** —
        // step 10c's twelfth-pass fix piece 4. **The case this was first
        // written for** was one job submitted inside a standing period, whose
        // own event named it. **The case it now also meets** is an act that
        // sends more than one job — a first-frame check's rest, a group's
        // later scenes — where each job's record is written provisional
        // before its own request goes out (`Queue & History` Y3). Until this
        // piece an event about any provisional record asked about *every*
        // one of this server's, so a pass landing between the rest's record
        // and its request asked the machine about a job it did not hold yet.
        // The machine truly answered *absent*, step 2's branch removed the
        // record leaving no trace, and a moment later the machine took the
        // job: **from then on it held a job the desktop had no record of**,
        // which the Queue drops at its join (`Queue & History` C10) and the
        // server row still counts. Step 10c's twelfth pass, fault 5. **An
        // event names only a job the machine has already taken**, so a job
        // whose request is still on its way is never asked about here; its
        // own event, a pass later, is what asks.
        let mut named: Vec<Identity> = Vec::new();
        for job in read.events.iter().filter_map(|event| {
            event
                .about
                .as_deref()
                .and_then(|text| Identity::read_back(IdentityKind::Job, text))
        }) {
            let provisional_here = with_the_record(record, |connection| {
                JobRecords::over(connection).read(&job).ok().flatten()
            })
            .flatten()
            .is_some_and(|held| {
                held.standing == Standing::Provisional && &held.server.local_server == server
            });
            if provisional_here && !named.contains(&job) {
                named.push(job);
            }
        }
        if !named.is_empty() {
            the_provisional_records_named_are_resolved(
                record,
                machine,
                &named,
                &mut WhatStepSevenDid::default(),
            );
        }
        if read
            .events
            .iter()
            .any(|event| THE_KINDS_THAT_MOVE_A_JOB.contains(&event.kind.as_str()))
        {
            every_record_is_filled_in(server, record, machine);
        }
        // **`Queue & History` N9's second route: the journalled event.** The
        // jobs a recorded reading's event names, and those a render attempt's
        // start or a recorded outcome names — a job read before submitting
        // writes no reading of its own, and an ending is when every attempt
        // record stands — have their reading filled in, after the note's
        // half, on whichever route this read was taken (`Queue & History`
        // C1a). The 38's piece 10.
        let mut the_readings_named: Vec<Identity> = Vec::new();
        for job in read
            .events
            .iter()
            .filter(|event| THE_KINDS_THAT_NAME_A_READING.contains(&event.kind.as_str()))
            .filter_map(|event| {
                event
                    .about
                    .as_deref()
                    .and_then(|text| Identity::read_back(IdentityKind::Job, text))
            })
        {
            if !the_readings_named.contains(&job) {
                the_readings_named.push(job);
            }
        }
        if !the_readings_named.is_empty() {
            the_readings_are_filled_in(server, record, machine, holds, &the_readings_named);
        }
        // **`Queue & History` P3 and P5: a published preview puts a fetch on
        // its job**, durably, before the cursor moves past the event (`State,
        // identity and recovery` E3) — on whichever route this read was taken,
        // since catching up is the same trigger. **A write the record would
        // not take moves no cursor**, so the event is read again. The fetch
        // itself runs beside the pass, never in it (`crate::the_previews`).
        // The 38's piece 3.
        if read
            .events
            .iter()
            .any(|event| event.kind == THE_PREVIEW_PUBLISHED_EVENT)
        {
            let owed = with_the_record(record, |connection| {
                crate::the_previews::the_published_are_owed(connection, server, &read.events)
                    .is_ok()
            });
            if owed != Some(true) {
                return HowThePassEnded::TheReadCouldNotComplete {
                    applied: applied_in_all,
                };
            }
        }

        // 2. The account, over what was applied — **written into the local
        // record before the cursor moves past the events it names** (`Queue &
        // History` C2), so it survives a restart and stands until the user's
        // *Dismiss*. The 38's piece 15.
        let mut tracked = HashSet::new();
        let mut applied = Vec::with_capacity(read.events.len());
        // **The endings this read applied**, put out to be posted once the
        // cursor stands past them — and only where they are *now* (T18).
        let mut ended_in_this_read: Vec<Identity> = Vec::new();
        for event in &read.events {
            let about = event
                .about
                .as_deref()
                .and_then(|text| Identity::read_back(IdentityKind::Job, text));
            let held = about.as_ref().and_then(|job| {
                with_the_record(record, |connection| {
                    JobRecords::over(connection).read(job).ok().flatten()
                })
                .flatten()
                .filter(|held| &held.server.local_server == server)
            });
            if let Some(held) = &held {
                tracked.insert(held.job.clone());
            }
            applied.push(match (&event.kind, held) {
                (kind, Some(held)) if kind == THE_RECORDED_OUTCOME_WRITTEN_EVENT => {
                    match held.the_ending_learned() {
                        Some(ending) => {
                            ended_in_this_read.push(held.job.clone());
                            AnAppliedEvent::AJobEnded {
                                job: held.job.clone(),
                                ending,
                                sequence: event.sequence,
                            }
                        }
                        None => AnAppliedEvent::SomethingTheNoticeDoesNotName,
                    }
                }
                // **`Queue & History` C1: interrupted and re-run from a given
                // frame.** The machine journals a render attempt's start, about
                // the job, saying its number; **an attempt past the first is the
                // re-run `Queue & History` D4 makes of an interrupted render**
                // and nothing else starts one. The frame is the first of that
                // attempt's own list (`Queue & History` N1), read from its
                // attempt record — **outside the lend**, since the record is
                // never held across a request (L4) — and absent where the render
                // worker has not written it yet: [`the_re_runs_frames_are_read`]
                // fills it in on a later pass.
                (kind, Some(held)) if kind == THE_RENDER_ATTEMPT_STARTED_EVENT => {
                    match the_attempt_a_start_names(event.says.as_deref()) {
                        Some(attempt) if attempt > 1 => AnAppliedEvent::InterruptedAndReRunFrom {
                            frame: the_first_frame_of_an_attempt(&row, machine, &held, attempt),
                            job: held.job.clone(),
                            attempt,
                            sequence: event.sequence,
                        },
                        _ => AnAppliedEvent::SomethingTheNoticeDoesNotName,
                    }
                }
                _ => AnAppliedEvent::SomethingTheNoticeDoesNotName,
            });
        }
        for (job, name, why, sequence) in dropped {
            tracked.insert(job.clone());
            applied.push(AnAppliedEvent::AHeldJobWasDropped {
                job,
                name,
                why,
                sequence,
            });
        }
        for (job, name, the_version, sequence) in dropped_for_a_removed_blender {
            tracked.insert(job.clone());
            applied.push(AnAppliedEvent::AWaitingJobWasDroppedForARemovedBlender {
                job,
                a_bounded_rendering_of_the_label: name,
                the_version,
                sequence,
            });
        }
        // **The standing record is read and the grown one written in one
        // lend**, so a *Dismiss* cannot land between the two and be undone by
        // the write. **A read the record refuses composes nothing**: composing
        // over nothing would write a fresh record over the standing one.
        let the_account_stands = with_the_record(record, |connection| {
            let standing =
                crate::record::catch_up_notices::the_standing_notice(connection, server).ok()?;
            match compose_the_account(standing, server, &applied, &tracked, gap) {
                Some(account) => {
                    crate::record::catch_up_notices::the_notice_is_written(connection, &account)
                        .ok()
                }
                None => Some(()),
            }
        })
        .flatten();
        // **A notice the record would not take moves no cursor**, so the
        // events it names are read again (`Queue & History` C2, `State,
        // identity and recovery` E3).
        if the_account_stands.is_none() {
            return HowThePassEnded::TheReadCouldNotComplete {
                applied: applied_in_all,
            };
        }

        // 3. The cursor, durably, only once the work above is done.
        //
        // **One past the last event this read carried, never the journal's
        // own next number** — the machine carries a bounded number of events
        // a read (`THE_MOST_EVENTS_ONE_READ_CARRIES`, in `Rented-server
        // runtime — program/src/main.rs`) and says *"a reader that got fewer
        // than there are asks again from past the last"*. **The case this was
        // first written for** was a read that carried every event the journal
        // held, where the two numbers are the same. **The case it now also
        // meets** is a machine that ran long with the app closed: until
        // 2026-09-29 the cursor went to the newest after the first read, so
        // every event between the 512th and the newest — an ending, a removal,
        // a drop — was never applied, on this connection or any later one
        // (`State, identity and recovery` E3). Step 10c's dependency wave piece
        // 10b, on the runner's ruling of that day. **A read that carried
        // nothing** — a gap past everything the journal still holds — moves to
        // the journal's next number, as before.
        let one_past_the_last_carried = read
            .events
            .last()
            .map(|last| last.sequence.saturating_add(1));
        let next = one_past_the_last_carried.unwrap_or(read.next).max(from);
        let written = with_the_record(record, |connection| {
            let Ok(Some(mut row)) = server_rows::one(connection, &server_text) else {
                return false;
            };
            row.event_cursor = Some(next);
            server_rows::write(connection, &row).is_ok()
        });
        if written != Some(true) {
            return HowThePassEnded::TheReadCouldNotComplete {
                applied: applied_in_all,
            };
        }
        applied_in_all += read.events.len();

        // **T18: an ending applied after step 3 completed is *now*, and goes
        // out to be posted — only now that the cursor stands past it.** Step 3
        // itself and its remainder are catch-up and put nothing out.
        if now_or_catching_up == NowOrCatchingUp::Now {
            for job in ended_in_this_read {
                holds.an_ending_to_tell(&server_text, job);
            }
        }
        // **`Queue & History` W4: an event applied may be the one that left
        // the line quiet**, so the re-check is woken to look.
        if !read.events.is_empty() {
            holds.idle_nudges().nudge(&server_text);
        }

        // 4. The machine's registration.
        the_machine_is_told_the_cursor_moved(machine, next);

        // **The read came back short of the newest, so it is asked again from
        // past the last** — and a read that reached the newest ends the pass
        // without asking once more for nothing.
        if read.events.is_empty() || next == from || next >= read.next {
            return HowThePassEnded::ItReadToTheEnd {
                applied: applied_in_all,
            };
        }
        from = next;
    }
}

/// **`Queue & History` H9 and H11: the held jobs this read says the machine
/// dropped**, each with the name it is told by, the release's own words as the
/// why and the removal's sequence number — **the name read now, while its local
/// record still stands**, held for the run and kept on the notice's line.
///
/// **How a drop is known.** The machine performs H11's order in one turn: the
/// job's record goes, then its hold's release is written *dropped* and
/// journalled (`Rented-server runtime` U20), then the removal is journalled.
/// **So a drop is a release whose words end in the outcome *dropped*, and the
/// very next event, the removal of a job.** The release is about the hold and
/// the desktop holds nothing linking a hold to its job, so the next event is
/// what names it. **A cancel, a clear and a held job's *no* are never read as
/// one**: none writes a release *dropped*. **What this does not catch**: a
/// removal the machine's journal refused and wrote later, out of that order.
/// The job then leaves in silence, as it did before.
///
/// **Only a job this desktop tracks on this server is named** (`Queue &
/// History` L22, A2). A job whose record a pass in this run already took is
/// named by what this run held, so an interrupted pass read again still tells.
fn the_held_jobs_dropped_in(
    events: &[AnEventRead],
    server: &Identity,
    record: &dyn LendsTheRecord,
    holds: &WhatThisRunHolds,
) -> Vec<(Identity, String, String, u64)> {
    let the_word_for_dropped =
        format!("{:?}", smoozy_runtime::holds::ReleaseOutcome::Dropped).to_lowercase();
    let mut dropped = Vec::new();
    for pair in events.windows(2) {
        let (release, removal) = (&pair[0], &pair[1]);
        let Some(says) = release.says.as_deref() else {
            continue;
        };
        if release.kind != THE_RELEASE_EVENT
            || says.rsplit(": ").next() != Some(the_word_for_dropped.as_str())
            || removal.kind != THE_JOB_RECORD_REMOVED_EVENT
            || removal.sequence != release.sequence + 1
        {
            continue;
        }
        let Some(job) = removal
            .about
            .as_deref()
            .and_then(|text| Identity::read_back(IdentityKind::Job, text))
        else {
            continue;
        };
        let held = with_the_record(record, |connection| {
            JobRecords::over(connection).read(&job).ok().flatten()
        })
        .flatten();
        let name = match held {
            // **Called what Queue calls it** — a job sent with no name by its
            // file (step 10c's Vast pass, fault 3).
            Some(held) if &held.server.local_server == server => {
                crate::surface::the_queue::what_the_job_is_called(&held)
            }
            Some(_) => continue,
            None => match holds.the_name_of_a_dropped_job(&job) {
                Some(name) => name,
                None => continue,
            },
        };
        holds.a_dropped_job_is_named(&job, name.clone());
        dropped.push((job, name, says.to_string(), removal.sequence));
    }
    dropped
}

/// **`Queue & History` L22: each waiting job the machine dropped at its turn
/// because the Blender it was submitted with was removed** — a job record's
/// removal whose words are the requests crate's own drop words
/// ([`smoozy_server_protocol::requests::what_a_drop_says`]), with the version
/// they name. The 38's piece 14c.
///
/// **Named as Queue names it**, from the local record read before the removal
/// takes it; where the record already went in an earlier pass of this period,
/// by the name held for it then. **A job of another server, or one this copy
/// never knew, is told to nobody** (`Queue & History` L22: the desktop never having known of
/// it). The event's own label is not drawn: it is the machine's bounded copy
/// of a name this copy holds whole.
fn the_waiting_jobs_dropped_for_a_removed_blender_in(
    events: &[AnEventRead],
    server: &Identity,
    record: &dyn LendsTheRecord,
    holds: &WhatThisRunHolds,
) -> Vec<(Identity, String, String, u64)> {
    let mut dropped = Vec::new();
    for removal in events
        .iter()
        .filter(|event| event.kind == THE_JOB_RECORD_REMOVED_EVENT)
    {
        let Some((the_version, _)) = removal
            .says
            .as_deref()
            .map(|says| says.strip_prefix("Telegram off: ").unwrap_or(says))
            .and_then(smoozy_server_protocol::requests::what_a_drop_says)
        else {
            continue;
        };
        let Some(job) = removal
            .about
            .as_deref()
            .and_then(|text| Identity::read_back(IdentityKind::Job, text))
        else {
            continue;
        };
        let held = with_the_record(record, |connection| {
            JobRecords::over(connection).read(&job).ok().flatten()
        })
        .flatten();
        let name = match held {
            Some(held) if &held.server.local_server == server => {
                crate::surface::the_queue::what_the_job_is_called(&held)
            }
            Some(_) => continue,
            None => match holds.the_name_of_a_dropped_job(&job) {
                Some(name) => name,
                None => continue,
            },
        };
        holds.a_dropped_job_is_named(&job, name.clone());
        dropped.push((job, name, the_version, removal.sequence));
    }
    dropped
}

/// **What a queue reading came to** — [`the_queue_is_read`].
#[derive(Debug)]
pub struct WhatTheQueueReadingCameTo {
    /// **What the request gave** — `None` where it could not be asked, and
    /// where the answer was not held because an ending it names could not be
    /// learned first ([`a_reading_is_held_once_its_endings_are_learned`]).
    pub gave: Option<WhatTheRequestGave>,
    /// **The events a read forward applied before the reading was held**, for
    /// the caller to follow up as it follows up any read forward.
    pub applied_first: usize,
    /// **Whether a phone's choice read here became the default** (`Telegram`
    /// V1d), so the caller can draw Settings again.
    pub the_default_was_taken: bool,
}

/// **`Queue & History` A1's reading, taken and held** — step 5, and a P10d
/// pass's queue reading. `the_offer` is step 6's and only step 7 composes it.
/// **An ending the reading names is learned before it is held**
/// ([`a_reading_is_held_once_its_endings_are_learned`]).
pub fn the_queue_is_read(
    server: &Identity,
    connection: u64,
    record: &dyn LendsTheRecord,
    machine: &mut dyn AsksTheSupervisor,
    holds: &WhatThisRunHolds,
    the_connection_is_live: &dyn Fn() -> bool,
    compose_the_offer: bool,
) -> WhatTheQueueReadingCameTo {
    let asking = TheQueueOverThisServersPoint {
        machine: std::cell::RefCell::new(machine),
        the_connection_is_live,
        the_reporting_read: std::cell::RefCell::new(None),
    };
    let gave = smoozy_queue_history::acting::adoption::take_the_reading(
        &asking,
        server,
        smoozy_queue_history::acting::adoption::WhetherTheInstallationConfirmed::ItIsTheSameInstallation,
    );
    let the_reporting_read = asking.the_reporting_read.into_inner();
    let machine = asking.machine.into_inner();
    let mut came_to = WhatTheQueueReadingCameTo {
        gave,
        applied_first: 0,
        the_default_was_taken: false,
    };
    // **`Telegram` V1d: what the reading said of reporting is the desk's
    // before the reading is held**, so the Queue drawn from it shows each
    // job's choice as the machine holds it.
    if let (Some(WhatTheRequestGave::ItAnswered(_)), Some(said)) = (&came_to.gave, the_reporting_read.as_ref()) {
        came_to.the_default_was_taken =
            with_the_record(record, |connection| the_reporting_the_reading_carried_is_written(connection, said))
                .unwrap_or(false);
    }
    if let Some(WhatTheRequestGave::ItAnswered(reading)) = &came_to.gave {
        let offer = if compose_the_offer {
            Some(the_offer_in(reading.value(), record))
        } else {
            None
        };
        let held = a_reading_is_held_once_its_endings_are_learned(
            server,
            connection,
            record,
            machine,
            holds,
            reading.clone(),
            offer,
        );
        came_to.applied_first = held.applied_first;
        if !held.it_was_held {
            came_to.gave = None;
        }
    }
    came_to
}

/// **Whether a reading was held, and what was read forward before it** —
/// [`a_reading_is_held_once_its_endings_are_learned`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WhatHoldingTheReadingCameTo {
    pub it_was_held: bool,
    pub applied_first: usize,
}

/// **`Queue & History` C11: when the job ends it leaves the queue and the
/// history entry takes the ending Smoozy learns — from watching, or from the
/// note.** A reading that names a job as *ended* is that watching, **so the
/// ending is learned before the reading is held**, and the job is never in
/// neither list.
///
/// **The gap this closes.** The Queue lets a job go on the reading's word
/// ([`crate::surface::the_queue::it_has_left_the_queue`]), and History lists
/// it only once its record has an ending (`Queue & History` C9 as narrowed
/// 2026-10-04). A P10d pass takes the read forward first and the queue
/// reading after it, so **a job that ended between the two was read as
/// *ended* while its record still had no ending**, and stayed in neither list
/// until the next pass's read forward — the pause and a whole pass later. On
/// a machine 340 ms away that was at least 12 seconds (step 10c's worldwide
/// setup pass on Vast, new fault 5). Step 7's reading and a move's re-read met
/// the same gap.
///
/// **How it is learned: the read forward, taken once more at this moment** —
/// the same mechanism, the same cursor and the same idempotent application
/// (`Queue & History` C1a), so the account, the posted ending (`Desktop app
/// and delivery` T18) and the kept words follow as from any read forward.
/// **The machine journals the ending in the same turn it writes the record
/// *ended***, after the note's ending (`Rented-server runtime — program`,
/// `the_jobs_side_of_an_ending`), so a reading that says *ended* is never
/// ahead of the event that carries it. Taken only where the reading names
/// such a job: an ordinary reading asks nothing more.
///
/// **Not taken for an ending owed to a full store**: History draws that one
/// from the reading until the landing (step 12's SC4 follow-up). **A read
/// forward that could not complete, with the ending still not learned, leaves
/// the last reading standing** — as a reading that did not come back does —
/// so the job stays in the Queue as last read, marked not updating once the
/// break is declared (`Queue & History` C7), and the next pass tries again. A
/// read forward that read to the end holds the reading whatever it learned, so
/// a queue is never frozen on one job.
pub(crate) fn a_reading_is_held_once_its_endings_are_learned(
    server: &Identity,
    connection: u64,
    record: &dyn LendsTheRecord,
    machine: &mut dyn AsksTheSupervisor,
    holds: &WhatThisRunHolds,
    reading: Reading<TheMachinesWorkingState>,
    offer: Option<Vec<Identity>>,
) -> WhatHoldingTheReadingCameTo {
    let mut applied_first = 0;
    if an_ending_it_names_is_not_learned(server, reading.value(), record) {
        match a_read_forward_pass(server, record, machine, holds) {
            HowThePassEnded::ItReadToTheEnd { applied } => applied_first = applied,
            HowThePassEnded::TheReadCouldNotComplete { applied } => {
                applied_first = applied;
                if an_ending_it_names_is_not_learned(server, reading.value(), record) {
                    return WhatHoldingTheReadingCameTo {
                        it_was_held: false,
                        applied_first,
                    };
                }
            }
        }
    }
    holds.a_reading_was_taken(&server.as_text(), connection, reading, offer);
    WhatHoldingTheReadingCameTo {
        it_was_held: true,
        applied_first,
    }
}

/// **Whether the reading names a job as *ended* that this server's local
/// record holds with no ending learned**, its ending not carried as owed.
fn an_ending_it_names_is_not_learned(
    server: &Identity,
    state: &TheMachinesWorkingState,
    record: &dyn LendsTheRecord,
) -> bool {
    let ended: Vec<&Identity> = state
        .jobs
        .iter()
        .filter(|job| {
            matches!(job.state, QueueState::Ended { .. }) && job.live.the_ending_owed().is_none()
        })
        .map(|job| &job.job)
        .collect();
    if ended.is_empty() {
        return false;
    }
    with_the_record(record, |connection| {
        let store = JobRecords::over(connection);
        ended.iter().any(|job| {
            store.read(job).ok().flatten().is_some_and(|held| {
                &held.server.local_server == server && held.the_ending_learned().is_none()
            })
        })
    })
    .unwrap_or(false)
}

/// **`Queue & History` A5 and `Queue & History` A9: every job on the machine the local record
/// does not name, less any a history tombstone names.** The judgement is that
/// module's `the_adoption_offer`; what is here is the local record's two
/// answers to it.
fn the_offer_in(state: &TheMachinesWorkingState, record: &dyn LendsTheRecord) -> Vec<Identity> {
    with_the_record(record, |connection| {
        let store = JobRecords::over(connection);
        let tracked: HashSet<Identity> = state
            .jobs
            .iter()
            .filter(|job| matches!(store.read(&job.job), Ok(Some(_))))
            .map(|job| job.job.clone())
            .collect();
        match store.tombstones() {
            Ok(tombstones) => the_adoption_offer(state, &tracked, &tombstones),
            // **A tombstone store that will not read offers nothing**: an
            // offer made without it could put a deleted job back (`Queue & History` A9).
            Err(_) => Vec::new(),
        }
    })
    .unwrap_or_default()
}

/// **Step 7, whole, on one connection that settled on the confirmed branch.**
pub fn the_step_seven(
    server: &Identity,
    connection: u64,
    record: &dyn LendsTheRecord,
    machine: &mut dyn AsksTheSupervisor,
    holds: &WhatThisRunHolds,
    the_connection_is_live: &dyn Fn() -> bool,
) -> WhatStepSevenDid {
    let mut did = WhatStepSevenDid::default();
    // `State, identity and recovery` V1 step 2.
    every_provisional_record_is_resolved(server, record, machine, &mut did);
    // `State, identity and recovery` V1 step 3 — **the catch-up account**,
    // which posts nothing (`Desktop app and delivery` T18).
    let step_three = the_read_forward_pass(
        server,
        record,
        machine,
        holds,
        HowItArrived::AtTheReconciliationRead,
    );
    did.events_applied = match step_three {
        HowThePassEnded::ItReadToTheEnd { applied }
        | HowThePassEnded::TheReadCouldNotComplete { applied } => applied,
    };
    // **T18: what is read forward after this, while this connection is live,
    // is *now*.** A step 3 that could not complete leaves the server catching
    // up, and the first live pass that reads to the end completes it.
    if matches!(step_three, HowThePassEnded::ItReadToTheEnd { .. })
        && holds.is_current(&server.as_text(), connection)
    {
        holds.the_reconciliation_read_completed(server);
    }
    // `State, identity and recovery` V1 step 4.
    did.filled_in = every_record_is_filled_in(server, record, machine);
    // **`Queue & History` N9's first route: the connection's fill-in, beside
    // the note's** — every job of this server still in the queue, so its
    // entry has its reading on this connection however late it landed. A job
    // that ended has had its reading filled in by the event that carried its
    // ending, at step 3. The 38's piece 10.
    let still_in_the_queue: Vec<Identity> = with_the_record(record, |connection| {
        JobRecords::over(connection)
            .with_no_learned_ending_for(server)
            .unwrap_or_default()
    })
    .unwrap_or_default()
    .into_iter()
    .filter(|one| one.standing == Standing::Confirmed && one.the_render_start_has_been_learned)
    .map(|one| one.job)
    .collect();
    the_readings_are_filled_in(server, record, machine, holds, &still_in_the_queue);
    // `State, identity and recovery` V1 steps 5 and 6.
    if let Some(WhatTheRequestGave::ItAnswered(_)) = the_queue_is_read(
        server,
        connection,
        record,
        machine,
        holds,
        the_connection_is_live,
        true,
    )
    .gave
    {
        did.the_queue_was_read = true;
    }
    did.offered = holds
        .every_server()
        .into_iter()
        .find(|(held, _)| *held == server.as_text())
        .map(|(_, period)| period.the_offer.len())
        .unwrap_or(0);
    did
}

// ════════════════════════════════════════════════════════════════════════════
// `Queue & History` A6 to `Queue & History` A9 — *Track it*: the offer accepted
// ════════════════════════════════════════════════════════════════════════════

/// **Why pressing *Track it* adopted nothing**, said beside the control.
/// **Each is all or nothing** (`Queue & History` A6): no job of the offer
/// entered the local record.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhyNothingWasTracked {
    /// **No reading of that server is held** — its period ended, its row is
    /// gone — **or the machine did not answer** the reads each entry is
    /// composed from (A7).
    TheServerCouldNotBeReached,
    /// **The local record refused the one write.**
    TheRecordRefusedIt,
}

/// **`Queue & History` A6 to `Queue & History` A9, performed: *Track it* on one server's
/// offer.** Owed piece 10.
///
/// - **The offer is the one step 7 composed** (A5, `Servers` Q18), less
///   anything the local record names by now or a tombstone names (`Queue & History` A9) —
///   [`the_offer_in`], asked again at the press over the same reading. So a
///   second press, or a press on a banner drawn before the first went
///   through, finds nothing left and adopts nothing twice.
/// - **A7: each entry shows what the machine's records hold**: its note
///   (`Queue & History` N1 — the label, the moments, the ending) and its
///   manifest (`Job files and outputs` M1 — the file, the scene, the Blender,
///   the settings payload, the folder), read now, before anything is
///   written. **A record that will not read is carried as its absence**;
///   **a machine that does not answer adopts nothing** and says so, since a
///   record written with nothing in it could never be filled in later. A job
///   whose note says the machine no longer holds it is not work on the
///   machine, and is not adopted (A5's own set, read now).
/// - **A6: one durable write, every record confirmed, under the identity
///   the machine carries** — [`smoozy_queue_history::acting::adoption::accept`]
///   through [`JobRecords::the_adopted_are_written`].
/// - **A8: from then on the record is Smoozy's own note**: a job already
///   rendering is in History at once (its note's start is the learning,
///   `Queue & History` C9), and every later move — its reading, its frames,
///   its ending — comes by the ordinary routes. Its reading is filled in now,
///   by `Queue & History` N9's own route, where its render has started.
/// - **No auto-download arrangement is made.** `Download` R1 names the two
///   moments one is created — at submission, or from the queue entry's
///   switch — and adoption is neither; the entry's switch and History's
///   *Download* are the ordinary routes for its frames.
pub fn the_offer_is_accepted(
    server: &Identity,
    record: &dyn LendsTheRecord,
    machine: &mut dyn AsksTheSupervisor,
    holds: &WhatThisRunHolds,
) -> Result<Vec<Identity>, WhyNothingWasTracked> {
    use smoozy_job_files::manifest::{SubmissionManifest, MANIFEST_FILE_NAME};
    let server_text = server.as_text();
    let Some((_, period)) = holds
        .every_server()
        .into_iter()
        .find(|(held, _)| *held == server_text)
    else {
        return Err(WhyNothingWasTracked::TheServerCouldNotBeReached);
    };
    let Some(reading) = period.reading.as_ref() else {
        return Err(WhyNothingWasTracked::TheServerCouldNotBeReached);
    };
    let mut state = reading.value().clone();
    let still_offerable = the_offer_in(&state, record);
    let mut offer: Vec<Identity> = period
        .the_offer
        .iter()
        .filter(|job| still_offerable.contains(job))
        .cloned()
        .collect();
    if offer.is_empty() {
        return Ok(Vec::new());
    }
    let Some(Some(row)) = with_the_record(record, |connection| {
        server_rows::one(connection, &server_text).ok().flatten()
    }) else {
        return Err(WhyNothingWasTracked::TheServerCouldNotBeReached);
    };
    let layout = crate::servers::installation::the_layout_recorded_for(&row);
    let mut manifests: HashMap<Identity, Option<SubmissionManifest>> = HashMap::new();
    let mut no_longer_held: Vec<Identity> = Vec::new();
    for job in &offer {
        let note = match the_note_as_far_as_it_reads(machine, job) {
            WhatTheNoteReadCameTo::TheMachineDidNotAnswer => {
                return Err(WhyNothingWasTracked::TheServerCouldNotBeReached)
            }
            WhatTheNoteReadCameTo::TheMachineNoLongerHoldsTheJob => {
                no_longer_held.push(job.clone());
                continue;
            }
            WhatTheNoteReadCameTo::It(note) => note,
        };
        // **The job record directory is named for the identity alone**
        // (`Job files and outputs` N3), so the manifest is read whether or not
        // the note gave a label.
        let manifest = layout.as_ref().and_then(|layout| {
            let label = note
                .as_ref()
                .map(|note| note.label.as_str())
                .filter(|label| !label.is_empty());
            let paths =
                smoozy_job_files::publication::AJobsPaths::composed(layout, job, label).ok()?;
            machine
                .a_file_of_the_job_record_directory(&format!(
                    "{}/{MANIFEST_FILE_NAME}",
                    paths.job_record_directory
                ))
                .and_then(|bytes| SubmissionManifest::read_back(&bytes))
        });
        if let Some(note) = note {
            state.notes.push((job.clone(), note));
        }
        manifests.insert(job.clone(), manifest);
    }
    offer.retain(|job| !no_longer_held.contains(job));
    if offer.is_empty() {
        return Ok(Vec::new());
    }
    let mut writer = TheAdoptedAreWrittenDown {
        record,
        stood: the_server_as_it_stands(&row, server),
        manifests,
    };
    let adopted =
        smoozy_queue_history::acting::adoption::accept(&mut writer, server, &offer, &state)
            .map_err(|_| WhyNothingWasTracked::TheRecordRefusedIt)?;
    let jobs: Vec<Identity> = adopted.into_iter().map(|one| one.job).collect();
    holds.the_offer_was_accepted(&server_text, &jobs);
    the_readings_are_filled_in(server, record, machine, holds, &jobs);
    // **`Queue & History` §17: *untracked work taken over*, one line for the
    // act** — the server's label as `Queue & History` N12's copy carries it and the count, and
    // never a job's identity or label. Owed piece 11. **What the diary
    // answers changes nothing** (`Desktop app and delivery` L16).
    if !jobs.is_empty() {
        let act = smoozy_queue_history::diary::AnAct::UntrackedWorkTakenOver {
            // **Empty where the row has no label**, as the copy holds none.
            the_servers_label: row.label.clone().unwrap_or_default(),
            jobs_covered: jobs.len(),
        };
        let at = SystemTime::now();
        let wrote = record.write_a_modules_line(THE_QUEUES_MODULE, &|run| {
            act.line(
                &smoozy_queue_history::diary::Concerning {
                    run,
                    server_row: Some(server.clone()),
                    operations: Vec::new(),
                    group: None,
                },
                at,
            )
        });
        let _ = wrote.nothing_is_owed();
    }
    Ok(jobs)
}

/// **The name `Queue & History` declares its diary lines under**, registered
/// at start (`src/start.rs`, owed piece 11) and written under by *Track it*.
pub const THE_QUEUES_MODULE: &str = "Queue & History";

/// What one job's note read came to, at the press.
enum WhatTheNoteReadCameTo {
    /// The note, or `None` where it would not read whole (A7: the absence is
    /// carried).
    It(Option<TheNote>),
    /// **The machine answered that it does not hold the job.**
    TheMachineNoLongerHoldsTheJob,
    /// Nothing that answers the request came back, or it was never asked.
    TheMachineDidNotAnswer,
}

fn the_note_as_far_as_it_reads(
    machine: &mut dyn AsksTheSupervisor,
    job: &Identity,
) -> WhatTheNoteReadCameTo {
    let Some(request) = a_request(
        THE_PER_JOB_NOTE_READING_REQUEST,
        Fields::none().and(THE_JOB_FIELD, job.as_text()),
    ) else {
        return WhatTheNoteReadCameTo::TheMachineDidNotAnswer;
    };
    match machine.ask(&request) {
        WhatCameBack::ItAnswered(answered)
            if answered
                .the_request_name_it_says_it_does_not_answer()
                .is_none() =>
        {
            if answered.fields.text(ON_THIS_MACHINE_FIELD) == Some(IT_IS_ABSENT) {
                WhatTheNoteReadCameTo::TheMachineNoLongerHoldsTheJob
            } else {
                WhatTheNoteReadCameTo::It(TheNote::read_from_the_answer(&answered.fields))
            }
        }
        // **A program that does not answer the note reading** holds a note
        // this desk cannot read, which is A7's absence and not a silence.
        WhatCameBack::ItAnswered(_) => WhatTheNoteReadCameTo::It(None),
        _ => WhatTheNoteReadCameTo::TheMachineDidNotAnswer,
    }
}

/// **`Queue & History` N12's copy of the server, as its row stands at
/// adoption** — the label and the badge, in the form the submission's copy
/// takes them (`src/surface/render_settings.rs`).
fn the_server_as_it_stands(
    row: &smoozy_servers::store::ServerRow,
    server: &Identity,
) -> smoozy_queue_history::record::TheServerAsItStood {
    smoozy_queue_history::record::TheServerAsItStood {
        local_server: server.clone(),
        label: row.label.clone(),
        hue: row
            .colour
            .as_deref()
            .and_then(|colour| colour.strip_prefix('b'))
            .and_then(|number| number.parse::<u16>().ok())
            .filter(|number| (1..=8).contains(number))
            .map(smoozy_queue_history::record::Hue),
    }
}

/// **`Queue & History` A6's write, given its end on the desk.**
struct TheAdoptedAreWrittenDown<'a> {
    record: &'a dyn LendsTheRecord,
    stood: smoozy_queue_history::record::TheServerAsItStood,
    manifests: HashMap<Identity, Option<smoozy_job_files::manifest::SubmissionManifest>>,
}

impl smoozy_queue_history::acting::adoption::WritesAdoptedJobs for TheAdoptedAreWrittenDown<'_> {
    fn accept_them_all(
        &mut self,
        adopted: &[smoozy_queue_history::acting::adoption::AnAdoptedJob],
    ) -> Result<(), String> {
        let records: Vec<_> = adopted
            .iter()
            .map(|one| {
                an_adopted_record(
                    one,
                    &self.stood,
                    self.manifests.get(&one.job).and_then(Option::as_ref),
                )
            })
            .collect();
        match with_the_record(self.record, |connection| {
            JobRecords::over(connection).the_adopted_are_written(&records)
        }) {
            Some(Ok(())) => Ok(()),
            Some(Err(failure)) => Err(format!("{failure:?}")),
            None => Err("the record was not lent".to_string()),
        }
    }
}

/// **One adopted job's local record, and the kept setup beside it** —
/// `Queue & History` A6 and A7, composed from what the machine's records
/// hold and nothing else.
///
/// - **Confirmed, under the machine's identity** (A6, Y4), on this server.
/// - **From the note** (`Queue & History` N1): the label — none where it is
///   empty (`Queue & History` Y8) — and the note's carried half, through the record's own
///   fill-in (`LocalJobRecord::the_fill_in_from_the_note`), so a job already
///   rendering has its start learned and is in History at once (C9).
/// - **From the manifest** (`Job files and outputs` M1): the group, the
///   `.blend`'s path, the Blender pair, and through the settings payload the
///   scene, the typed percentage and the device asked for (`Queue & History`
///   N11, `Queue & History` N22, `Queue & History` E12a). The payload is kept beside the record, as a
///   submission's is (record version 25), so *Submit again* works from the
///   entry. **The folder** is the manifest's: chosen at render start is
///   learned by `Queue & History` N9's route; a named path is the job area's own, which is
///   every named path a manifest has ever carried (`Job files and outputs`
///   P21's first case is reached by no route).
/// - **Absent where it was never known here**: the submitted moment
///   (`Queue & History` N16: the desktop's clock, and this desktop never saw
///   the submission), the file's own values beside the overrides (`Queue & History` N14: no
///   reading stood here), the dependency check and the carry-on mark.
/// - **The Blender's origin is *resolved for them*** (`Queue & History` N10a):
///   the record cannot say the user picked it, and this reading makes
///   *Submit again* run the automatic match afresh rather than carry a build
///   pick into a page as the user's own.
/// - **Where the manifest will not read**, the entry states the absence
///   (A7): no file, no scene, an empty Blender pair, the file's own device
///   unread, and the job area's folder, which is where `Queue & History` N9's route then reads.
pub fn an_adopted_record(
    adopted: &smoozy_queue_history::acting::adoption::AnAdoptedJob,
    stood: &smoozy_queue_history::record::TheServerAsItStood,
    manifest: Option<&smoozy_job_files::manifest::SubmissionManifest>,
) -> (
    smoozy_queue_history::record::LocalJobRecord,
    Option<smoozy_render_settings::payload::Payload>,
) {
    use smoozy_queue_history::record::{
        AnswerOrigin, LocalJobRecord, TheBlenderTheEntryKeeps, TheDeviceAsk, TheFramesTheReadFound,
        TheJobOutputFolderTheRecordHolds, TheServerAsItStood, WhatTheNoteCarried,
    };
    use smoozy_render_settings::payload::{Payload, VersionPick};
    let payload = manifest.and_then(|manifest| Payload::read_back(&manifest.settings_payload));
    let scene_name = payload
        .as_ref()
        .and_then(|payload| {
            payload
                .scene
                .as_ref()
                .map(|scene| scene.name.as_text().to_string())
        })
        .or_else(|| {
            manifest
                .map(|manifest| manifest.scene.name.clone())
                .filter(|name| !name.is_empty())
        });
    let mut record = LocalJobRecord {
        job: adopted.job.clone(),
        group: manifest
            .and_then(|manifest| manifest.group.as_deref())
            .and_then(|text| Identity::read_back(IdentityKind::Group, text)),
        server: TheServerAsItStood {
            local_server: adopted.server.clone(),
            ..stood.clone()
        },
        submitted_at: None,
        label: adopted
            .the_note
            .as_ref()
            .map(|note| note.label.clone())
            .filter(|label| !label.is_empty()),
        blend_path_on_the_machine: manifest
            .map(|manifest| manifest.blend_path_on_the_machine.clone()),
        submitted_with_no_scene_named: manifest.is_some() && scene_name.is_none(),
        scene_name,
        resolution_percentage_typed: payload
            .as_ref()
            .and_then(|payload| payload.resolution_percentage),
        picture_size_from_the_read: None,
        main_output_from_the_read: None,
        blender: TheBlenderTheEntryKeeps {
            pick: VersionPick {
                build_path: manifest
                    .map(|manifest| manifest.blender.build_path.clone())
                    .unwrap_or_default(),
                version: manifest
                    .map(|manifest| manifest.blender.version.clone())
                    .unwrap_or_default(),
            },
            origin: AnswerOrigin::ResolvedForThem,
        },
        device_asked_for: match payload.as_ref().and_then(|payload| payload.device) {
            Some(choice) => TheDeviceAsk::TheUsersOwn(choice),
            None => TheDeviceAsk::TheFilesOwnUnread,
        },
        the_files_values_beside_the_overrides: None,
        dependency_check: None,
        carries_on_from: None,
        disturbances: Vec::new(),
        attempts_where_the_file_changed: Vec::new(),
        the_frames: TheFramesTheReadFound::default(),
        standing: Standing::Confirmed,
        the_render_start_has_been_learned: false,
        what_the_note_carried: WhatTheNoteCarried::default(),
        the_job_output_folder: match manifest.map(|manifest| &manifest.job_output_folder) {
            Some(smoozy_job_files::manifest::WhereTheJobOutputFolderIs::ChosenAtRenderStart {
                ..
            }) => TheJobOutputFolderTheRecordHolds::ChosenAtRenderStart,
            _ => TheJobOutputFolderTheRecordHolds::AsEveryJobBeforeItHadIt,
        },
        the_driver_was_below_the_bar: None,
    };
    if let Some(note) = adopted.the_note.as_ref() {
        record.the_fill_in_from_the_note(note);
    }
    (record, payload)
}

// ════════════════════════════════════════════════════════════════════════════
// `Servers` Q1 step 9 — the update of an older program
// ════════════════════════════════════════════════════════════════════════════

/// **`Rented-server runtime` G21's reading, out of `Queue & History` A1's queue
/// reading** — V11's *the queue reading of `Servers` Q1 step 7 where the
/// program can be spoken to*. Added 2026-09-28, step 10c's transfer wave piece
/// 8.
///
/// **The four standings, each from what the reading says of one job:**
///
/// - **waiting and rendering are live**, and **held is held** — work the
///   program has accepted and not finished;
/// - **an ended job is an uncollected result only where this copy tracks it
///   and has not yet learned its ending** — `State, identity and recovery` V1
///   step 4 is what collects it, and step 7 has just run it. `the_ending_is_learned`
///   answers that, and nothing where the local record does not track the job:
///   **a job this copy never tracked is not this copy's result to collect**,
///   and it is on `Queue & History` A5's offer instead. Holding every update
///   behind such a job would hold it for good on a machine another copy used,
///   and nothing is lost by not holding it — `State, identity and recovery`
///   §1.3 rule 7 never removes a result not yet acknowledged, whatever release
///   reads the store.
pub fn what_g21_reads(
    reading: &TheMachinesWorkingState,
    the_ending_is_learned: &dyn Fn(&Identity) -> Option<bool>,
) -> Vec<(String, smoozy_runtime::package::retry::AcceptedWorkStanding)> {
    use smoozy_runtime::package::retry::AcceptedWorkStanding as Standing;
    reading
        .jobs
        .iter()
        .map(|job| {
            let standing = match &job.state {
                QueueState::Waiting | QueueState::Rendering { .. } => Standing::Live,
                QueueState::Held { .. } => Standing::Held,
                QueueState::Ended { .. } => match the_ending_is_learned(&job.job) {
                    Some(false) => Standing::Uncollected,
                    Some(true) | None => Standing::Nothing,
                },
            };
            (job.job.as_text(), standing)
        })
        .collect()
}

/// **What V11 decides, from the two release sequences and the work reading.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatStepNineDecides {
    /// **No active package, or the machine's release is this application's
    /// own**: nothing is sent and none of V13's five stands.
    NothingIsSent,
    /// **V14: the machine is newer, and nothing this desktop sends places a
    /// release below it.** V13's *too new* stands.
    TheMachineIsTooNew,
    /// **V11: the machine is older and work stands on it, or its work could not
    /// be read.** Nothing is sent and nothing is staged; V13's *the update
    /// waits* stands, and V12 tries it again.
    TheUpdateWaits,
    /// **V11: the machine is older and nothing on it is live, held or
    /// uncollected** — the bootstrap-and-repair request is sent for this
    /// application's own release.
    TheUpdateIsSent,
}

/// **V11 and V14's judgement, with the readings handed in** — so the whole of it
/// is driven at the desk.
///
/// **A work reading that could not be had is not an empty line**: the update
/// waits, and V12 tries it again at the next connection.
pub fn what_step_nine_decides(
    the_machines_release: Option<u64>,
    this_applications_own: u64,
    work: Option<&[(String, smoozy_runtime::package::retry::AcceptedWorkStanding)]>,
) -> WhatStepNineDecides {
    use crate::servers::published::{the_direction, TheDirection};
    match the_direction(the_machines_release, this_applications_own) {
        None | Some(TheDirection::TheSame) => WhatStepNineDecides::NothingIsSent,
        Some(TheDirection::TheMachineIsNewer) => WhatStepNineDecides::TheMachineIsTooNew,
        Some(TheDirection::TheMachineIsOlder) => match work {
            Some(work) if work.iter().all(|(_, standing)| !standing.blocks()) => {
                WhatStepNineDecides::TheUpdateIsSent
            }
            _ => WhatStepNineDecides::TheUpdateWaits,
        },
    }
}

/// **How a sent update ended**, in V13's terms.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TheUpdatesEnding {
    /// `Rented-server runtime` G3's *verified*: the machine runs this
    /// application's own release.
    ItLanded,
    /// **`Rented-server runtime` G22: deferred**, an ordinary result — a job
    /// was accepted between the reading and the commit (V11's *the gate stays the
    /// gate*), or another transaction stands. The update waits.
    ItWaits,
    /// **Rolled back, failed before commit, or not carried out**, with the
    /// transaction's own words. The machine is on the release it had.
    ItFailed { said: String },
}

/// **Everything step 9 reaches that is not a judgement**, as a seam — its one
/// implementation in the running application is [`TheUpdateOverThisPoint`];
/// the tests stand a machine in that records every request sent to it.
pub trait TheUpdatesOwnEnds {
    /// **`Server protocol and transfers` B4's inspection of the row's package
    /// area.**
    fn what_the_package_area_holds(
        &mut self,
    ) -> crate::servers::installation::WhatThePackageAreaHolds;
    /// **This application's own release sequence** (`Server protocol and
    /// transfers` B8d), or why it could not be had.
    fn this_applications_own_release(&mut self) -> Result<u64, String>;
    /// **V11's request, sent**: under `under` where a deferred phase recorded
    /// one (V12), and under a fresh identity otherwise.
    fn the_update_is_sent(
        &mut self,
        under: Option<Identity>,
        held: &crate::servers::installation::WhatThePackageAreaHolds,
        accepted_work: Vec<(String, smoozy_runtime::package::retry::AcceptedWorkStanding)>,
    ) -> TheUpdatesEnding;
}

/// **What step 9 did on one reading**, as much as a test wants to see.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatStepNineDid {
    /// The package area could not be read, so nothing is known and nothing
    /// moved.
    ThePackageAreaCouldNotBeRead,
    /// This application's own release could not be had, so nothing was compared.
    TheOwnReleaseCouldNotBeHad { said: String },
    /// What V11 and V14 decided, and — where the update was sent — how it
    /// ended.
    Decided {
        decided: WhatStepNineDecides,
        ending: Option<TheUpdatesEnding>,
    },
    /// **V4: a request is already in hand for this server**, so none was
    /// composed.
    ARequestIsAlreadyInHand,
}

/// **`Servers` Q1 step 9, on one reading: V11, V12 and V14, with V13's
/// conditions raised and withdrawn as they move.** Added 2026-09-28, step 10c's
/// transfer wave piece 8.
///
/// **It runs where `Servers` Q1 puts it**: after step 7's queue reading, on the
/// confirmed branch only, and again — V12 — at the queue reading that shows
/// the last of the machine's work ended. **Nothing polls and nothing is
/// timed**: each call is a reading already being taken.
///
/// **What it was written for, and what it now also meets.** A placement was
/// written for a machine never prepared. It now meets a machine already
/// running Smoozy — every server a user set up before this release — and the
/// one rule it holds to is that a program with work on it is never replaced
/// from under that work.
pub fn the_step_nine(
    server: &str,
    reading: Option<&TheMachinesWorkingState>,
    the_ending_is_learned: &dyn Fn(&Identity) -> Option<bool>,
    ends: &mut dyn TheUpdatesOwnEnds,
) -> WhatStepNineDid {
    use crate::servers::published::{
        an_update_is_taken_in_hand, the_server_carries, WhatSmoozyOnTheServerIs as Carries,
    };
    let held = ends.what_the_package_area_holds();
    if !held.it_was_read {
        return WhatStepNineDid::ThePackageAreaCouldNotBeRead;
    }
    // **`Telegram` V1c**: the queue offers a live job's reporting change off
    // this same reading — **noted before this copy's own release is asked
    // for**, which it does not need. Moved up 2026-10-31, the twenty-fourth
    // notes, note 4: a copy whose own release could not be had left the
    // control grey.
    crate::servers::published::the_servers_release_is_read(server, held.its_release_sequence);
    let own = match ends.this_applications_own_release() {
        Ok(own) => own,
        Err(said) => return WhatStepNineDid::TheOwnReleaseCouldNotBeHad { said },
    };
    let work = reading.map(|reading| what_g21_reads(reading, the_ending_is_learned));
    let decided = what_step_nine_decides(held.its_release_sequence, own, work.as_deref());
    let ending = match decided {
        WhatStepNineDecides::NothingIsSent => {
            // **Every one of V13's five ends here**: the machine runs this
            // application's release, or none — *the update failed* included,
            // whose own ending is a later update landing.
            the_server_carries(server, None);
            None
        }
        WhatStepNineDecides::TheMachineIsTooNew => {
            the_server_carries(server, Some(Carries::TooNew));
            None
        }
        WhatStepNineDecides::TheUpdateWaits => {
            the_server_carries(server, Some(Carries::TheUpdateWaits));
            None
        }
        WhatStepNineDecides::TheUpdateIsSent => {
            // **V4: one request outstanding per server.**
            if !an_update_is_taken_in_hand(server) {
                return WhatStepNineDid::ARequestIsAlreadyInHand;
            }
            // **V12: a deferred phase found on the machine is re-sent under its
            // recorded operation identity**, read from the phase record, and
            // never a fresh one (`Server protocol and transfers` B6).
            let under = held
                .the_phase_record
                .as_ref()
                .filter(|record| record.phase == smoozy_runtime::package::phase::Phase::Deferred)
                .map(|record| record.attempt.clone());
            let ending = ends.the_update_is_sent(under, &held, work.unwrap_or_default());
            the_server_carries(
                server,
                match &ending {
                    TheUpdatesEnding::ItLanded => {
                        crate::servers::published::the_servers_release_is_read(server, Some(own));
                        None
                    }
                    TheUpdatesEnding::ItWaits => Some(Carries::TheUpdateWaits),
                    TheUpdatesEnding::ItFailed { said } => {
                        Some(Carries::TheUpdateFailed { said: said.clone() })
                    }
                },
            );
            Some(ending)
        }
    };
    WhatStepNineDid::Decided { decided, ending }
}

/// **Whether the local record has learned this job's ending** — `Some` where it
/// tracks the job, `None` where it does not — which is what
/// [`what_g21_reads`] asks of an ended job.
fn the_ending_is_learned_in(
    record: &dyn LendsTheRecord,
) -> impl Fn(&Identity) -> Option<bool> + '_ {
    move |job: &Identity| {
        with_the_record(record, |connection| {
            JobRecords::over(connection)
                .read(job)
                .ok()
                .flatten()
                .map(|held| held.the_ending_learned().is_some())
        })
        .flatten()
    }
}

/// **Step 9's ends over this server's own point** — package inspection on its
/// reading session, this copy's own release read once per run, and V11's
/// request over the connection the point lends whole.
pub struct TheUpdateOverThisPoint<'a> {
    pub sessions: ThisServersSessions,
    pub package_area: String,
    pub record: &'a dyn LendsTheRecord,
    /// **The installation this copy confirmed at step 5**, which the update is
    /// made under (`Rented-server runtime` G10: an update keeps the machine's
    /// installation).
    pub installation: Identity,
    /// **The page is told the update is under way** before the request goes
    /// out, since the transaction holds the connection for its whole length
    /// (Row 291's *anything that isn't instant shows that it's underway*).
    pub the_page_is_told: &'a dyn Fn(),
}

impl TheUpdatesOwnEnds for TheUpdateOverThisPoint<'_> {
    fn what_the_package_area_holds(
        &mut self,
    ) -> crate::servers::installation::WhatThePackageAreaHolds {
        let sessions = &self.sessions;
        let area = self.package_area.as_str();
        waited_for(|| async move {
            crate::servers::installation::what_the_package_area_holds(sessions, area).await
        })
    }

    fn this_applications_own_release(&mut self) -> Result<u64, String> {
        crate::setting_up::this_copys_own_release_sequence_here(self.record)
    }

    fn the_update_is_sent(
        &mut self,
        under: Option<Identity>,
        held: &crate::servers::installation::WhatThePackageAreaHolds,
        accepted_work: Vec<(String, smoozy_runtime::package::retry::AcceptedWorkStanding)>,
    ) -> TheUpdatesEnding {
        // **This application's own release and its program** — the desk
        // program in a copy given one (`Server protocol and transfers` B10a),
        // and otherwise this build's own release file and artifact
        // (`Server protocol and transfers` B8d). The tuple is the one the
        // machine's own phase record placed under.
        (self.the_page_is_told)();
        let tuple = smoozy_server_protocol::release::composing::the_tuple_this_build_is_made_for();
        let got = the_own_release_and_its_program(self.record, &tuple);
        let (verified, artifact) = match got {
            Ok(got) => got,
            Err(said) => return TheUpdatesEnding::ItFailed { said },
        };
        let sessions = &self.sessions;
        let area = self.package_area.as_str();
        let installation = self.installation.clone();
        let placed = waited_for(|| async move {
            crate::setting_up::the_update_is_placed(
                sessions,
                crate::setting_up::OverTheLentConnection,
                area,
                installation,
                held,
                under,
                verified,
                artifact,
                accepted_work,
            )
            .await
        });
        use smoozy_server_protocol::placement::performing::WhatThePlacementReached as Reached;
        match placed {
            Ok(Reached::Verified) => TheUpdatesEnding::ItLanded,
            Ok(Reached::Deferred { .. })
            | Ok(Reached::APackageTransactionIsInProgress { .. })
            | Ok(Reached::AClaimStandsWhoseOwnerCouldNotBeRead) => TheUpdatesEnding::ItWaits,
            Ok(Reached::RolledBack) => TheUpdatesEnding::ItFailed {
                said: "New version failed to start. Rolled back.".to_string(),
            },
            Ok(Reached::FailedBeforeCommit { because }) | Ok(Reached::RollingBack { because }) => {
                TheUpdatesEnding::ItFailed { said: because }
            }
            Err(said) => TheUpdatesEnding::ItFailed { said },
        }
    }
}

/// **This application's own release, and the program in it** — read the way
/// the ready action reads it (`crate::setting_up::the_ready_action_on`), so the
/// update and a setup place the same thing.
fn the_own_release_and_its_program(
    record: &dyn LendsTheRecord,
    tuple: &smoozy_server_protocol::release::manifest::TargetTuple,
) -> Result<
    (
        smoozy_server_protocol::release::verifying::AVerifiedRelease,
        Vec<u8>,
    ),
    String,
> {
    use crate::setting_up::ReadsTheCurrentRelease;
    #[cfg(debug_assertions)]
    if let Some(mut desk) = crate::the_desk_program::the_one_this_copy_was_given() {
        return desk.the_current_release(tuple);
    }
    let mut store = crate::setting_up::TheTrustRecordThroughTheSpine { record };
    let mut over_https = crate::setting_up::TheCurrentReleaseOverHttps {
        store: &mut store,
        from: crate::setting_up::WHERE_THE_CURRENT_RELEASE_IS_READ_FROM,
    };
    over_https.the_current_release(tuple)
}

/// **Step 9 in the running application, on one connection**, and nothing where
/// a setup this copy is driving holds the server (`Servers` Q1 puts step 9
/// after whatever step 8 did, and `Server Setup` Y9 holds one act of Smoozy's
/// own on a machine at a time) or the row confirms no installation.
fn the_step_nine_here<R: Runtime>(
    app: &AppHandle<R>,
    server: &str,
    connection: u64,
    sessions: &ThisServersSessions,
    package_area: &str,
    record: &dyn LendsTheRecord,
    reading: Option<&TheMachinesWorkingState>,
) {
    let confirmed = with_the_record(record, |connection| {
        server_rows::one(connection, server).ok().flatten()
    })
    .flatten()
    .and_then(|row| row.last_confirmed_installation_identity)
    .and_then(|text| Identity::read_back(IdentityKind::Installation, &text));
    let Some(installation) = step_nine_may_run_now(server, connection, confirmed) else {
        return;
    };
    let before = crate::servers::published::what_the_server_carries(server);
    let the_control_before = crate::servers::published::a_live_jobs_reporting_can_be_changed_on(server);
    let told = || crate::surface::servers::the_work_column_is_told_again(app);
    let mut ends = TheUpdateOverThisPoint {
        sessions: sessions.clone(),
        package_area: package_area.to_string(),
        record,
        installation,
        the_page_is_told: &told,
    };
    let learned = the_ending_is_learned_in(record);
    the_step_nine(server, reading, &learned, &mut ends);
    if crate::servers::published::what_the_server_carries(server) != before {
        crate::surface::servers::the_work_column_is_told_again(app);
    }
    // **The queue was drawn before this step read the release**, so it is
    // told again where the answer moved, and the control wakes without
    // waiting for the next drawing (the twenty-fourth notes, note 4).
    if crate::servers::published::a_live_jobs_reporting_can_be_changed_on(server) != the_control_before {
        crate::surface::the_queue::the_queue_is_told_again(app);
    }
}

/// **The connection step 9 was held back on, per server** — run-held, nothing
/// durable. Added 2026-10-31, the twenty-fourth notes, note 4 (the runner's
/// follow-up).
fn the_step_nines_held_back() -> &'static Mutex<std::collections::HashMap<String, u64>> {
    static HELD: std::sync::OnceLock<Mutex<std::collections::HashMap<String, u64>>> =
        std::sync::OnceLock::new();
    HELD.get_or_init(Default::default)
}

/// **Step 9's gate on one connection**: the confirmed installation it runs
/// under, or nothing where a setup this copy drives holds the server or the
/// row confirms no installation — **and then the connection is noted**, so
/// the run's ending takes step 9 after it (`Servers` Q1: step 9 follows
/// whatever step 8 did). A step 9 that runs settles what was owed. Added
/// 2026-10-31, the twenty-fourth notes, note 4: until then a server a setup
/// held on this connection was never read again before the next connection.
pub fn step_nine_may_run_now(server: &str, connection: u64, confirmed: Option<Identity>) -> Option<Identity> {
    let held_back = crate::setting_up::the_kind_being_driven_on(server).is_some() || confirmed.is_none();
    if let Ok(mut held) = the_step_nines_held_back().lock() {
        if held_back {
            held.insert(server.to_string(), connection);
        } else {
            held.remove(server);
        }
    }
    if held_back {
        return None;
    }
    confirmed
}

/// **A step 9 held back and now owed**: the connection it was held back on,
/// taken once, and nothing while a setup still drives the server.
pub fn a_held_back_step_nine_is_owed_on(server: &str) -> Option<u64> {
    if crate::setting_up::the_kind_being_driven_on(server).is_some() {
        return None;
    }
    the_step_nines_held_back().lock().ok().and_then(|mut held| held.remove(server))
}

/// **Every setup run's ending takes the step 9 it held back** — registered
/// once, at start, beside the periods it reads
/// ([`crate::setting_up::when_a_run_ends`]). **Nothing where none is owed**;
/// otherwise step 9 runs on a thread of its own, as at a connection, and only
/// where the connection it was held back on is still the live one.
pub fn a_held_back_step_nine_follows_every_run<R: Runtime>(app: AppHandle<R>) {
    crate::setting_up::when_a_run_ends(Box::new(move |server: &str| {
        let Some(connection) = a_held_back_step_nine_is_owed_on(server) else {
            return;
        };
        let (app, server) = (app.clone(), server.to_string());
        let _ = std::thread::Builder::new()
            .name("smoozy step 9 after a setup".to_string())
            .spawn(move || a_held_back_step_nine_is_taken(&app, &server, connection));
    }));
}

fn a_held_back_step_nine_is_taken<R: Runtime>(app: &AppHandle<R>, server: &str, connection: u64) {
    let Some(periods) = app.try_state::<Arc<ThePeriodsThisRun>>() else {
        return;
    };
    let periods = periods.inner().clone();
    let holds = periods.the_machines_work();
    let Some(sessions) = periods.the_sessions_of(server) else {
        return;
    };
    if !(sessions.a_connection_is_live() && holds.is_current(server, connection)) {
        return;
    }
    let Some(started) = app.try_state::<Mutex<Started>>() else {
        return;
    };
    let record = crate::keys::TheRunsRecord {
        started: started.inner(),
    };
    let Some(Some(row)) = with_the_record(&record, |connection| {
        server_rows::one(connection, server).ok().flatten()
    }) else {
        return;
    };
    let WhereThePackageAreaIs::At(package_area) = where_the_package_area_is(&row) else {
        return;
    };
    let reading = the_reading_held_for(holds, server);
    the_step_nine_here(app, server, connection, &sessions, &package_area, &record, reading.as_ref());
}

/// **The queue reading this run holds for one server**, taken out of its
/// moment.
fn the_reading_held_for(holds: &WhatThisRunHolds, server: &str) -> Option<TheMachinesWorkingState> {
    holds
        .every_server()
        .into_iter()
        .find(|(held, _)| held == server)
        .and_then(|(_, period)| period.reading)
        .map(|reading| reading.value().clone())
}

/// **V12: the queue reading shows the last of the machine's work ended while an
/// update waits**, so step 9 is taken again on it — and nothing where no update
/// waits, or the reading still shows work, which asks the machine nothing more.
fn a_waiting_update_is_tried_again<R: Runtime>(
    app: &AppHandle<R>,
    server: &str,
    connection: u64,
    sessions: &ThisServersSessions,
    package_area: &str,
    record: &dyn LendsTheRecord,
    holds: &WhatThisRunHolds,
) {
    if crate::servers::published::what_the_server_carries(server)
        != Some(crate::servers::published::WhatSmoozyOnTheServerIs::TheUpdateWaits)
    {
        return;
    }
    let Some(reading) = the_reading_held_for(holds, server) else {
        return;
    };
    let learned = the_ending_is_learned_in(record);
    if what_g21_reads(&reading, &learned)
        .iter()
        .any(|(_, standing)| standing.blocks())
    {
        return;
    }
    the_step_nine_here(app, server, connection, sessions, package_area, record, Some(&reading));
}

// ════════════════════════════════════════════════════════════════════════════
// Started from the period, and P10d after it
// ════════════════════════════════════════════════════════════════════════════

/// **The moment a primary connection is lent**: step 7 is started on a thread
/// of its own, if the connection settled on the confirmed branch, and P10d
/// after it on the same thread.
///
/// **Called from the period's own thread and returns at once.**
pub fn the_machines_work_is_read<R: Runtime>(
    app: AppHandle<R>,
    server: String,
    sessions: ThisServersSessions,
    connection: u64,
) {
    let _ = std::thread::Builder::new()
        .name("smoozy step 7".to_string())
        .spawn(move || the_work_on_one_connection(&app, &server, sessions, connection));
}

fn the_work_on_one_connection<R: Runtime>(
    app: &AppHandle<R>,
    server: &str,
    sessions: ThisServersSessions,
    connection: u64,
) {
    let Some(periods) = app.try_state::<Arc<ThePeriodsThisRun>>() else {
        return;
    };
    let periods = periods.inner().clone();
    // **`Servers` Q1 step 7 runs only on the confirmed branch**, and only for
    // the connection that read it.
    let Some(read) = periods.what_the_last_connection_read(server) else {
        return;
    };
    if read.check != InstallationCheck::PresentAndEqualConfirmed
        || !read.on_the_connection_that_is_live
    {
        return;
    }
    let Some(started) = app.try_state::<Mutex<Started>>() else {
        return;
    };
    let record = crate::keys::TheRunsRecord {
        started: started.inner(),
    };
    let Some(server_identity) = Identity::read_back(IdentityKind::LocalServer, server) else {
        return;
    };
    let Some(Some(row)) = with_the_record(&record, |connection| {
        server_rows::one(connection, server).ok().flatten()
    }) else {
        return;
    };
    let WhereThePackageAreaIs::At(package_area) = where_the_package_area_is(&row) else {
        return;
    };
    // **`Server protocol and transfers` S28's machine-side sweep, beside the
    // rest and never in front of it** — its own thread, its readings asked for
    // as nobody's waiting (`Server protocol and transfers` §1.1 item 2). Step
    // 10c's transfer wave piece 7.
    the_sweep_is_started(app, &server_identity, &sessions);

    let holds = periods.the_machines_work();
    let is_live = || sessions.a_connection_is_live() && holds.is_current(server, connection);
    let mut machine = OverThisServersPoint {
        sessions: sessions.clone(),
        package_area: package_area.clone(),
    };

    let did = the_step_seven(
        &server_identity,
        connection,
        &record,
        &mut machine,
        holds,
        &is_live,
    );
    // **`Queue & History` E9: the words of every ending step 7 learned**, on a
    // thread of their own beside the rest. The Creator's notes on 0.1.8, piece
    // 6d.
    crate::servers::the_words_kept::are_taken_in_beside(
        app,
        &server_identity,
        connection,
        &sessions,
        &package_area,
    );
    // **`Queue & History` P5: the previews step 3 learned of, and any a
    // closed app left owed, are fetched beside the rest.** The 38's piece 3.
    crate::the_previews::are_fetched_beside(app, &server_identity, connection, &sessions);
    crate::surface::the_queue::the_queue_is_told_again(app);
    crate::surface::the_history::the_history_is_told_again(app);
    crate::surface::servers::the_work_column_is_told_again(app);

    // **`Servers` Q1 step 9: the update of an older program** — V11, after
    // step 7's queue reading, on the confirmed branch only, which is the only
    // branch this thread runs on. **The reading is this connection's own**, or
    // none where step 7 could not take one, and then the update waits. Added
    // 2026-09-28, step 10c's transfer wave piece 8.
    let this_connections_reading = did
        .the_queue_was_read
        .then(|| the_reading_held_for(holds, server))
        .flatten();
    if is_live() {
        the_step_nine_here(
            app,
            server,
            connection,
            &sessions,
            &package_area,
            &record,
            this_connections_reading.as_ref(),
        );
        // **`Server Setup` R1c: a standing hold of the driver's reason is a
        // moment for `Server Setup` R1b's comparison on any connection**, and this
        // connection's own queue reading is the first that can find one — so a
        // hold placed in one connected period is picked up in the next. Where
        // it concludes a swap is owed, `Server Setup` R1b's starting rule follows at the same
        // reading. The queue-reading pass does the same while the queue is on
        // screen. Added 2026-10-12, held-job piece A.
        let the_server = server.to_string();
        crate::servers::the_driver_swap::a_standing_driver_hold_is_compared(
            app,
            &the_server,
            this_connections_reading.as_ref(),
        );
        if crate::servers::the_driver_swap::a_swap_is_owed_on(&the_server) {
            crate::servers::the_driver_swap::an_owed_swap_is_tried(
                app,
                &the_server,
                this_connections_reading.as_ref(),
            );
        }
    }

    // **`Queue & History` W4's re-check, beside P10d and never inside it** —
    // `Desktop app and delivery` P10e's one instance, on a thread of its own,
    // ending with this connection. A connection that finds the machine already
    // past the half hour says so at once (`Queue & History` W3; the 38's map,
    // hole 2). The 38's piece 1.
    crate::servers::the_idle_recheck::is_started(
        app,
        &server_identity,
        connection,
        &sessions,
        &package_area,
        row.last_confirmed_installation_identity.as_deref(),
    );

    // **P10d's own start is `Servers` Q1 step 7 completing for that server.**
    let mut passes = ThePassesOverThisConnection {
        app,
        periods: &periods,
        server: &server_identity,
        connection,
        record: &record,
        machine: &mut machine,
        is_live: &is_live,
        sessions: &sessions,
        package_area: &package_area,
    };
    let Ok(runtime) = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
    else {
        return;
    };
    runtime.block_on(the_live_reading_sequence(
        server,
        &mut passes,
        &mut TheDesktopsClock,
    ));
}

// ════════════════════════════════════════════════════════════════════════════
// `Server protocol and transfers` S28's machine-side sweep — step 10c's transfer
// wave piece 7
// ════════════════════════════════════════════════════════════════════════════

/// **The sweep, on a thread of its own, for the connection that was just
/// lent.** Nothing waits on it and it waits on nothing but its own readings.
fn the_sweep_is_started<R: Runtime>(
    app: &AppHandle<R>,
    server: &Identity,
    sessions: &ThisServersSessions,
) {
    let (app, server, sessions) = (app.clone(), server.clone(), sessions.clone());
    let _ = std::thread::Builder::new()
        .name("smoozy staging sweep".to_string())
        .spawn(move || {
            let Some(started) = app.try_state::<Mutex<Started>>() else {
                return;
            };
            let door = |act: &mut dyn FnMut(&rusqlite::Connection)| match started.lock() {
                Ok(held) => match held.record.as_ref() {
                    Some(record) => {
                        act(record.connection());
                        true
                    }
                    None => false,
                },
                Err(_) => false,
            };
            let Ok(runtime) = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
            else {
                return;
            };
            let came_to = runtime.block_on(the_staging_left_on_this_machine_is_swept(
                &sessions, &door, &server,
            ));
            if came_to.debts_cleared > 0 {
                if let Some(the_run) = app.try_state::<Arc<crate::downloading::TheRunsTransfers>>()
                {
                    the_run.every_leftover_is_read_again();
                }
                crate::surface::transfers::the_transfers_moved(&app);
            }
        });
}

/// **What the machine-side sweep came to on one connection.**
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct WhatTheSweepCameTo {
    /// Staged objects removed on `Server protocol and transfers` S28's exact
    /// match.
    pub removed: usize,
    /// Derived paths with nothing there — material already gone.
    pub gone: usize,
    /// **Objects left where they stand because the record cannot claim
    /// them** — on the machine, every one there is: the supplier cannot answer
    /// a stable identity (`Server protocol and transfers` §12 item 15).
    pub left: usize,
    /// Paths that could not be looked at, on a connection that went.
    pub not_looked_at: usize,
    /// **Owed cleanups confirmed and cleared** (`Server protocol and
    /// transfers` S27): every path the record derives is gone.
    pub debts_cleared: usize,
}

/// **`Server protocol and transfers` S28's first bullet: abandoned staging on
/// the machine is removed by a record-driven sweep at the next connection** —
/// *"It must complete before any transfer on that connection reuses a swept
/// staging path, and otherwise it runs beside admission rather than in front
/// of it."* Added 2026-09-29, step 10c's transfer wave piece 7; until then it
/// had no caller.
///
/// **What it reads**: this server's upload records under its current
/// installation that still have something the sweep could act on — an owed
/// cleanup (`Server protocol and transfers` S27), a part abandoned by its
/// request (`Server protocol and transfers` S28a), or a superseded attempt
/// (`Server protocol and transfers` S12 step 5). **A removable record whose
/// removal was confirmed has nothing left**, so it is not read again at every
/// connection. **No path is ever reused**: every staging path is synthesised
/// from its own transfer identity and attempt (`Server protocol and transfers`
/// S4), and the one kind of record a transfer on this connection carries on —
/// *interrupted* — is never on the sweep's list (`Server protocol and
/// transfers` S28a).
///
/// **What it can do, stated as it stands.** It removes an object only where
/// the path, the synthesised name and the stable file identity all match the
/// record (`Server protocol and transfers` S28, `Server protocol and
/// transfers` S29). **On the machine the
/// supplier cannot answer a stable identity** — the file-transfer surface
/// carries none, and a command-channel reading of one is `File Browser` §13
/// item 9's unestablished ground (`Server protocol and transfers` §12 item 15,
/// permanently unresolved) — **so every object still standing is left**, and
/// that item's *what holds* is what stands in for it: the material stays
/// visible as a reserved staging folder for the user's explicit removal
/// through `File Browser` X12. **What the sweep does do is clear a debt whose
/// material is already gone** — every derived path answering that nothing is
/// there — which is `Server protocol and transfers` S27's *confirmed*.
pub async fn the_staging_left_on_this_machine_is_swept<C: AHeldConnection>(
    sessions: &ThisServersSessions<C>,
    with_the_record: &(dyn Fn(&mut dyn FnMut(&rusqlite::Connection)) -> bool + Sync),
    server: &Identity,
) -> WhatTheSweepCameTo {
    use smoozy_server_protocol::subsystem::{StatusCode, SubsystemFailure};
    use smoozy_server_protocol::transfers::cleanup::{
        what_the_sweep_derives, what_the_sweep_does, WhatTheSweepDoes,
    };
    use smoozy_server_protocol::transfers::StableFileIdentity;

    let mut came_to = WhatTheSweepCameTo::default();
    let mut records = Vec::new();
    with_the_record(&mut |connection| {
        records = the_uploads_the_sweep_reads(connection, server);
    });
    for one in records {
        let mut every_path_is_gone = true;
        for derived in what_the_sweep_derives(std::slice::from_ref(&one)) {
            let path = derived.path.as_bytes();
            let looking = path.clone();
            let looked = sessions
                .a_reading(false, move |subsystem| {
                    Box::pin(async move {
                        match subsystem.lstat(&looking).await {
                            Ok(_) => Ok(true),
                            Err(SubsystemFailure::TheServerRefused(StatusCode::NoSuchFile)) => {
                                Ok(false)
                            }
                            Err(other) => Err(other),
                        }
                    })
                })
                .await;
            // **Something stands there, and the machine cannot say which
            // object it is.**
            let identity_now = match looked {
                Ok(Ok(true)) => Some(StableFileIdentity::TheSupplierCouldNotAnswer),
                Ok(Ok(false)) => None,
                _ => {
                    came_to.not_looked_at += 1;
                    every_path_is_gone = false;
                    continue;
                }
            };
            match what_the_sweep_does(&derived, identity_now.as_ref()) {
                WhatTheSweepDoes::Remove => {
                    let removing = path.clone();
                    let removed = sessions
                        .a_reading(false, move |subsystem| {
                            Box::pin(async move { subsystem.remove(&removing).await })
                        })
                        .await;
                    if matches!(removed, Ok(Ok(()))) {
                        came_to.removed += 1;
                    } else {
                        every_path_is_gone = false;
                    }
                }
                WhatTheSweepDoes::NothingIsThere => came_to.gone += 1,
                WhatTheSweepDoes::LeaveItAndReportAnOwnershipMismatch
                | WhatTheSweepDoes::LeaveItBecauseTheIdentityWasNeverRecorded => {
                    came_to.left += 1;
                    every_path_is_gone = false;
                }
            }
        }
        // **`Server protocol and transfers` S27: the debt is cleared only when
        // the removal is confirmed** — every path the record derives gone.
        if one.cleanup_is_owed && every_path_is_gone {
            let mut cleared = one.clone();
            cleared.cleanup_is_owed = false;
            let mut written = false;
            with_the_record(&mut |connection| {
                written = crate::record::transfer_records::put(connection, &cleared).is_ok();
            });
            if written {
                came_to.debts_cleared += 1;
            }
        }
    }
    came_to
}

/// **The upload records of one server the machine-side sweep reads** — under
/// the row's current installation, and only those with something the sweep
/// could act on (see [`the_staging_left_on_this_machine_is_swept`]). Each is
/// read whole, with its superseded attempts.
fn the_uploads_the_sweep_reads(
    connection: &rusqlite::Connection,
    server: &Identity,
) -> Vec<smoozy_server_protocol::transfers::TransferRecord> {
    let installation = server_rows::one(connection, &server.as_text())
        .ok()
        .flatten()
        .and_then(|row| row.last_confirmed_installation_identity);
    let Ok(mut statement) = connection.prepare(
        "SELECT transfer FROM transfer_record
         WHERE server = ?1 AND direction = 'up'
           AND (cleanup_is_owed != 0
                OR (ending = 'stopped' AND stopped_reason = 'abandoned by its request')
                OR EXISTS (SELECT 1 FROM transfer_superseded_attempt AS superseded
                           WHERE superseded.transfer = transfer_record.transfer))
         ORDER BY rowid",
    ) else {
        return Vec::new();
    };
    let Ok(rows) = statement.query_map([server.as_text()], |row| row.get::<_, String>(0)) else {
        return Vec::new();
    };
    let named: Vec<String> = rows.filter_map(Result::ok).collect();
    named
        .iter()
        .filter_map(|text| Identity::read_back(IdentityKind::Transfer, text))
        .filter_map(|transfer| {
            crate::record::transfer_records::read(connection, &transfer)
                .ok()
                .flatten()
        })
        .filter(|one| one.installation.as_ref().map(Identity::as_text) == installation)
        .collect()
}

// ════════════════════════════════════════════════════════════════════════════
// `Queue & History` E8: the running entry's commentary — piece 6c
// ════════════════════════════════════════════════════════════════════════════

/// **The words on screen**: whose they are, what the job is called, and how far
/// each of its two sinks has been read. Run-held.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheWordsOnScreen {
    pub server: String,
    pub job: String,
    pub name: String,
    /// The ordinary stream's sink and the complaints stream's, as read so far.
    sinks: [ASinkAsRead; 2],
}

/// **One sink as read so far**: its path, the bytes read from it, and the end
/// of a character the last read cut through, held for the next.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct ASinkAsRead {
    path: String,
    read_to: u64,
    held_back: Vec<u8>,
    /// Whether any of its words have reached the page since the panel opened.
    any_sent: bool,
}

/// **What reads a sink's bytes from one length to another** — the reading
/// session over `Server protocol and transfers` T5's per-server point
/// ([`TheSinksOverThisServersPoint`]); the tests answer it from bytes in hand.
pub trait ReadsASinksBytes {
    fn the_bytes(&mut self, path: &str, from: u64, to: u64) -> Result<Vec<u8>, String>;
}

/// **A sink's bytes over the server's own reading session** —
/// `smoozy_dependencies`' ranged read, beside its whole one.
pub struct TheSinksOverThisServersPoint {
    pub sessions: ThisServersSessions,
}

impl ReadsASinksBytes for TheSinksOverThisServersPoint {
    fn the_bytes(&mut self, path: &str, from: u64, to: u64) -> Result<Vec<u8>, String> {
        let sessions = &self.sessions;
        let path = path.as_bytes().to_vec();
        waited_for(|| async move {
            let read = sessions
                .a_reading(true, move |subsystem| {
                    Box::pin(async move {
                        Ok(smoozy_dependencies::acting::checking::read_a_streams_bytes_over_the_file_session(
                            subsystem, &path, from, to,
                        )
                        .await)
                    })
                })
                .await;
            match read {
                Ok(Ok(read)) => read,
                Ok(Err(failure)) => Err(format!("{failure:?}")),
                Err(not_lent) => Err(format!("{not_lent:?}")),
            }
        })
    }
}

/// **What one commentary read brought**: the text each stream printed since
/// the last read, whether this is the first since the panel opened (or since
/// a new attempt's sinks replaced the last), and whether each stream has sent
/// anything at all. Piece 6c.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheWordsThatArrived {
    pub job: String,
    pub name: String,
    pub first: bool,
    /// The ordinary stream's new text, then the complaints stream's.
    pub new: [String; 2],
    pub any_yet: [bool; 2],
}

/// **`Queue & History` E8's read, once**: for the words on screen on `server`,
/// each sink is read from the length last read to the length the newest
/// period-held queue reading states — **only the new bytes** — and what came
/// is handed back as text. Piece 6c.
///
/// - **Nothing where the words are not on screen for this server**, where the
///   reading is not updating, or where the job no longer renders with its
///   sinks named: the panel keeps what it has.
/// - **A new attempt's sinks start the read again from nought**, and the
///   arrival says it is first, so the page starts the panel over.
/// - **A character the read cut through is held back** until its end arrives,
///   and bytes that are not text at all are drawn as the replacement mark —
///   the one way a page's text can carry them. Nothing else is changed (`Desktop
///   app and delivery` M9).
/// - **A read that fails reads again next time** from where it was; nothing is
///   counted as read that did not arrive.
pub fn the_words_are_read(
    holds: &WhatThisRunHolds,
    server: &str,
    reader: &mut dyn ReadsASinksBytes,
) -> Option<TheWordsThatArrived> {
    let _one_at_a_time = holds.one_words_read_at_a_time.lock().ok()?;
    let mut words = holds.the_words_on_screen.lock().ok()?.clone()?;
    if words.server != server {
        return None;
    }
    let (_, period) = holds
        .every_server()
        .into_iter()
        .find(|(held, _)| held == server)?;
    if period.the_break_is_declared {
        return None;
    }
    let reading = period.reading?;
    let sinks = reading.value().jobs.iter().find_map(|one| {
        (one.job.as_text() == words.job && matches!(one.state, QueueState::Rendering { .. }))
            .then(|| one.live.the_sinks().cloned())
            .flatten()
    })?;
    let mut first = words.sinks.iter().all(|sink| sink.path.is_empty());
    let mut new: [String; 2] = Default::default();
    for (at, sink) in [&sinks.ordinary, &sinks.complaints].into_iter().enumerate() {
        let held = &mut words.sinks[at];
        if held.path != sink.path {
            if !held.path.is_empty() {
                first = true;
            }
            *held = ASinkAsRead {
                path: sink.path.clone(),
                ..ASinkAsRead::default()
            };
        }
    }
    if first {
        for held in words.sinks.iter_mut() {
            held.read_to = 0;
            held.held_back.clear();
            held.any_sent = false;
        }
    }
    for (at, sink) in [&sinks.ordinary, &sinks.complaints].into_iter().enumerate() {
        let held = &mut words.sinks[at];
        let Some(to) = sink.length else {
            continue;
        };
        if to <= held.read_to {
            continue;
        }
        if let Ok(bytes) = reader.the_bytes(&sink.path, held.read_to, to) {
            held.read_to = to;
            held.held_back.extend_from_slice(&bytes);
            new[at] = the_whole_characters(&mut held.held_back);
            held.any_sent |= !new[at].is_empty();
        }
    }
    // **The panel may have closed while the bytes were read**; then nothing is
    // kept and nothing is handed back.
    let mut on_screen = holds.the_words_on_screen.lock().ok()?;
    match on_screen.as_ref() {
        Some(still) if still.server == words.server && still.job == words.job => {}
        _ => return None,
    }
    let arrived = TheWordsThatArrived {
        job: words.job.clone(),
        name: words.name.clone(),
        first,
        any_yet: [words.sinks[0].any_sent, words.sinks[1].any_sent],
        new,
    };
    *on_screen = Some(words);
    (arrived.first || arrived.new.iter().any(|text| !text.is_empty())).then_some(arrived)
}

/// **The whole characters at the front of `held`, taken out of it as text**;
/// the start of a character the read cut through stays behind for the next.
fn the_whole_characters(held: &mut Vec<u8>) -> String {
    match std::str::from_utf8(held) {
        Ok(text) => {
            let text = text.to_string();
            held.clear();
            text
        }
        Err(cut) if cut.error_len().is_none() => {
            let rest = held.split_off(cut.valid_up_to());
            let text = String::from_utf8_lossy(held).into_owned();
            *held = rest;
            text
        }
        Err(_) => {
            let text = String::from_utf8_lossy(held).into_owned();
            held.clear();
            text
        }
    }
}

// ── The rendering entry's own three lines — the Creator's notes on the test
// copy, piece 8 (note 12) ─────────────────────────────────────────────────────

/// **What the entry's first read takes: the ordinary stream's last 4 KB**, and
/// the most any read after it takes (`Queue & History` E8; the map's hole 8).
/// More than this arriving between two reads starts the lines over from the
/// newest 4 KB, so no read of the entry's is ever larger.
pub const THE_ENTRYS_READ_AT_MOST: u64 = 4096;

/// **How many lines the entry shows** — `Design/Queue & History`, *A
/// rendering entry*: *"a small window of Blender's newest three lines"*.
pub const THE_ENTRYS_LINES: usize = 3;

/// **The rendering entry's own lines, as read so far.** Run-held.
///
/// The machine's own copy loop takes a line the same way: it ends at a line
/// feed, and a carriage return before the feed is not part of it
/// (`Rendering — machine`'s `offer_the_lines_in`).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TheEntrysLines {
    server: String,
    job: String,
    /// The ordinary stream's sink, as last read. A new attempt names another.
    path: String,
    /// How far the sink has been read; nothing before the first read.
    read_to: Option<u64>,
    /// The line Blender is printing, not ended yet, kept up to
    /// [`THE_ENTRYS_READ_AT_MOST`] bytes: past that the line is far wider
    /// than the entry and is cut with an ellipsis anyway.
    unfinished: Vec<u8>,
    /// **The read began inside a line**, so what comes before the next line
    /// end is the rest of a line whose start was not read, and is not shown.
    inside_a_line: bool,
    /// The newest complete lines, oldest first, at most [`THE_ENTRYS_LINES`].
    lines: Vec<String>,
}

/// **`bytes`, taken into the lines**: each line end completes a line, and only
/// the newest three are kept. Nothing in a line is changed but the carriage
/// return before its end; a byte that is not text is the replacement mark, the
/// one way a page's text can carry it.
fn the_bytes_are_taken_into(lines: &mut TheEntrysLines, bytes: &[u8]) {
    for &byte in bytes {
        if byte != b'\n' {
            if lines.unfinished.len() < THE_ENTRYS_READ_AT_MOST as usize {
                lines.unfinished.push(byte);
            }
            continue;
        }
        let line = std::mem::take(&mut lines.unfinished);
        if std::mem::replace(&mut lines.inside_a_line, false) {
            continue;
        }
        let line = match line.last() {
            Some(b'\r') => &line[..line.len() - 1],
            _ => &line[..],
        };
        lines.lines.push(String::from_utf8_lossy(line).into_owned());
        if lines.lines.len() > THE_ENTRYS_LINES {
            lines.lines.remove(0);
        }
    }
}

/// **`Queue & History` E8's read for the entry's own lines, once**: the
/// ordinary stream only, its last 4 KB the first time and only the new bytes
/// after. **True where the lines changed**, and then the queue is drawn again.
///
/// - **Nothing where the queue is not on screen or its entry is not a job
///   rendering on `server`**, where the reading is not updating, or where the job no longer renders
///   with its sinks named: the entry draws no lines then anyway.
/// - **A new attempt's sink starts the lines over**, from its last 4 KB.
/// - **A read that fails reads again next time** from where it was.
/// - **The errors stream is never read for them** (`Desktop app and delivery`
///   M13b's narrowing).
pub fn the_entrys_lines_are_read(
    holds: &WhatThisRunHolds,
    server: &str,
    reader: &mut dyn ReadsASinksBytes,
) -> bool {
    if !holds.the_entrys_lines_are_on_screen_for(server) {
        return false;
    }
    let Ok(_one_at_a_time) = holds.one_words_read_at_a_time.lock() else {
        return false;
    };
    let Some(mut lines) = holds
        .the_entrys_lines
        .lock()
        .ok()
        .and_then(|held| held.clone())
    else {
        return false;
    };
    let Some((_, period)) = holds
        .every_server()
        .into_iter()
        .find(|(held, _)| held == server)
    else {
        return false;
    };
    if period.the_break_is_declared {
        return false;
    }
    let Some(reading) = period.reading else {
        return false;
    };
    let Some(sink) = reading.value().jobs.iter().find_map(|one| {
        (one.job.as_text() == lines.job && matches!(one.state, QueueState::Rendering { .. }))
            .then(|| one.live.the_sinks().map(|sinks| sinks.ordinary.clone()))
            .flatten()
    }) else {
        return false;
    };
    let before = lines.lines.clone();
    if lines.path != sink.path {
        lines = TheEntrysLines {
            server: lines.server,
            job: lines.job,
            path: sink.path.clone(),
            ..TheEntrysLines::default()
        };
    }
    let Some(to) = sink.length else {
        return false;
    };
    let from = match lines.read_to {
        Some(at) if at <= to && to - at <= THE_ENTRYS_READ_AT_MOST => at,
        // **The first read, or more than one read takes since the last**:
        // the newest 4 KB, begun inside a line unless it is the stream's
        // start.
        _ => {
            let from = to.saturating_sub(THE_ENTRYS_READ_AT_MOST);
            lines.unfinished.clear();
            lines.lines.clear();
            lines.inside_a_line = from > 0;
            from
        }
    };
    if from < to {
        let Ok(bytes) = reader.the_bytes(&sink.path, from, to) else {
            return false;
        };
        the_bytes_are_taken_into(&mut lines, &bytes);
    }
    lines.read_to = Some(to);
    // **The queue may have been put away, or another entry chosen, while the
    // bytes were read**; then nothing is kept.
    let Ok(mut held) = holds.the_entrys_lines.lock() else {
        return false;
    };
    match held.as_ref() {
        Some(still) if still.server == lines.server && still.job == lines.job => {}
        _ => return false,
    }
    let changed = lines.lines != before;
    *held = Some(lines);
    changed
}

/// **P10d's three reads, given their ends on one connection.**
///
/// - **The read forward** is [`a_read_forward_pass`], step 3's own.
/// - **The queue reading** is step 5's own request, taken while the Queue
///   screen, the server list or this server's own page is on screen (the
///   runner's ruling on P10d of 2026-09-28); the offer is not recomposed
///   (`Servers` Q18).
/// - **The running entry's commentary read** is `Queue & History` E8's read of
///   a sink's bytes over the reading session, from the length last read to the
///   length the queue reading states, taken while the words' panel is open
///   ([`the_words_are_read`]). *(It was never taken until 2026-10-03, piece 6c:
///   no request named it and the Queue screen drew no commentary.)*
pub struct ThePassesOverThisConnection<'a, R: Runtime> {
    app: &'a AppHandle<R>,
    periods: &'a ThePeriodsThisRun,
    server: &'a Identity,
    connection: u64,
    record: &'a dyn LendsTheRecord,
    machine: &'a mut dyn AsksTheSupervisor,
    is_live: &'a dyn Fn() -> bool,
    /// **The point and the package area**, for V12's step 9 taken again at a
    /// queue reading.
    sessions: &'a ThisServersSessions,
    package_area: &'a str,
}

impl<R: Runtime> WhatAPassTakes for ThePassesOverThisConnection<'_, R> {
    fn is_on_screen(&mut self, server: &str, what: WhatIsOnScreen) -> bool {
        match what {
            WhatIsOnScreen::TheServersQueueOrRunningEntry => self
                .periods
                .the_machines_work()
                .a_surface_showing_its_work_is_on_screen(server),
            // **The words' panel, or the queue's rendering entry with its
            // three lines** — both are commentary on screen (`Queue & History`
            // E8; the Creator's notes on the test copy, piece 8).
            WhatIsOnScreen::TheRunningEntrysCommentary => self
                .periods
                .the_machines_work()
                .the_commentary_is_on_screen_for(server),
        }
    }

    fn the_period_stands_on_a_live_connection(&mut self, server: &str) -> bool {
        self.periods.a_period_stands_for(server) && (self.is_live)()
    }

    fn take(&mut self, _server: &str, read: WhatAPassReads) {
        let holds = self.periods.the_machines_work();
        match read {
            WhatAPassReads::TheReadForwardFromTheEventCursor => {
                let applied = match a_read_forward_pass(
                    self.server,
                    self.record,
                    &mut *self.machine,
                    holds,
                ) {
                    HowThePassEnded::ItReadToTheEnd { applied }
                    | HowThePassEnded::TheReadCouldNotComplete { applied } => applied,
                };
                self.a_read_forward_is_followed_up(applied);
            }
            WhatAPassReads::TheQueueReading => {
                // **An ending the reading names is learned before it is
                // held** ([`a_reading_is_held_once_its_endings_are_learned`]),
                // and what that read forward applied is followed up as the
                // pass's own read forward is — History drawn again with it.
                let read = the_queue_is_read(
                    self.server,
                    self.connection,
                    self.record,
                    &mut *self.machine,
                    holds,
                    self.is_live,
                    false,
                );
                if read.applied_first > 0 {
                    self.a_read_forward_is_followed_up(read.applied_first);
                }
                // **`Telegram` V1d: a phone's choice taken as the default** is
                // drawn in Settings' Telegram section at once.
                if read.the_default_was_taken {
                    crate::surface::telegram::the_section_is_told_again(self.app);
                }
                crate::surface::the_queue::the_queue_is_told_again(self.app);
                crate::surface::servers::the_work_column_is_told_again(self.app);
                // **V12: the reading that shows the last of the machine's work
                // ended is where a waiting update is tried again**, and only
                // while this connection is the live one.
                if (self.is_live)() {
                    a_waiting_update_is_tried_again(
                        self.app,
                        &self.server.as_text(),
                        self.connection,
                        self.sessions,
                        self.package_area,
                        self.record,
                        holds,
                    );
                    // **`Server Setup` R1b: the same reading is where an owed
                    // swap of the driver starts**, once it shows the last of
                    // the machine's live work ended and no transfer is under
                    // way — `State, identity and recovery` F15's third
                    // moment. Nothing where no swap is owed. Added 2026-10-12.
                    let server = self.server.as_text();
                    // **`Server Setup` R1c: a standing hold of the driver's
                    // reason is a moment for `Server Setup` R1b's comparison**, on any
                    // connection — it may conclude a swap is owed, which the
                    // lines below then start, or release the holds of a run
                    // that ended out of sight. Added 2026-10-12, held-job
                    // piece A.
                    crate::servers::the_driver_swap::a_standing_driver_hold_is_compared(
                        self.app,
                        &server,
                        the_reading_held_for(holds, &server).as_ref(),
                    );
                    if crate::servers::the_driver_swap::a_swap_is_owed_on(&server) {
                        let reading = the_reading_held_for(holds, &server);
                        crate::servers::the_driver_swap::an_owed_swap_is_tried(
                            self.app,
                            &server,
                            reading.as_ref(),
                        );
                    }
                }
            }
            WhatAPassReads::TheRunningEntrysCommentaryRead => {
                let mut reader = TheSinksOverThisServersPoint {
                    sessions: self.sessions.clone(),
                };
                if let Some(arrived) =
                    the_words_are_read(holds, &self.server.as_text(), &mut reader)
                {
                    crate::surface::the_queue::the_running_words_are_posted(self.app, &arrived);
                }
                // **The entry's own three lines**, drawn again where they
                // moved. The Creator's notes on the test copy, piece 8.
                if the_entrys_lines_are_read(holds, &self.server.as_text(), &mut reader) {
                    crate::surface::the_queue::the_queue_is_told_again(self.app);
                }
            }
        }
    }
}

impl<R: Runtime> ThePassesOverThisConnection<'_, R> {
    /// **What follows a read forward on this connection**, whichever read
    /// took it — the pass's own, or the one a queue reading takes before it
    /// is held.
    fn a_read_forward_is_followed_up(&mut self, applied: usize) {
        let holds = self.periods.the_machines_work();
        if applied > 0 {
            crate::surface::the_queue::the_queue_is_told_again(self.app);
            crate::surface::the_history::the_history_is_told_again(self.app);
            crate::surface::servers::the_work_column_is_told_again(self.app);
            // **`Queue & History` E9: an ending this pass learned owes its
            // words**, fetched beside the pass and never in it. The Creator's
            // notes on 0.1.8, piece 6d.
            crate::servers::the_words_kept::are_taken_in_beside(
                self.app,
                self.server,
                self.connection,
                self.sessions,
                self.package_area,
            );
        }
        // `Queue & History` P4: retry saved debts on the existing live pass,
        // even when there is no newly published event.
        crate::the_previews::are_fetched_beside(
            self.app,
            self.server,
            self.connection,
            self.sessions,
        );
        // **`Desktop app and delivery` T18's *now*: every ending a live read
        // forward applied since the last pass is posted**, this pass's own and
        // any a stop, a clear or a cancel read. The 38's piece 1.
        let endings = holds.the_endings_to_tell_are_taken(&self.server.as_text());
        if !endings.is_empty() {
            crate::servers::the_endings_posted::are_posted(
                self.app,
                self.record,
                self.server,
                &endings,
            );
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use std::collections::VecDeque;

    use super::*;
    use crate::record::deletions::tests::a_record_in_history;
    use smoozy_server_protocol::framing::Answered;
    use smoozy_server_protocol::requests::{
        HOW_MANY_RE_RUNS_FIELD, IT_FINISHED, THE_ATTEMPTS_FIELD,
        THE_ATTEMPT_THAT_HOLDS_THE_WORDS_FIELD, THE_ENDED_CLOCK_FIELD, THE_ENDED_FIELD,
        THE_ENDING_FIELD, THE_EXIT_STATUS_FIELD, THE_LABEL_FIELD, THE_LAST_FRAME_FIELD,
        THE_STARTED_CLOCK_FIELD, THE_STARTED_FIELD,
    };
    use smoozy_servers::identity::{Host, KeyReference, Port, Username};
    use smoozy_servers::store::ServerRow;
    use smoozy_state::observed::MachineClock;

    // ── A machine, scripted ──────────────────────────────────────────────────

    /// **A machine that answers by the request's name**, with every request
    /// it was asked written down in order.
    #[derive(Default)]
    struct AMachineInHand {
        /// What each request name answers, in turn; the last one repeats.
        answers: HashMap<&'static str, VecDeque<Option<Fields>>>,
        asked: Vec<(String, Fields)>,
    }

    impl AMachineInHand {
        fn answers(mut self, name: &'static str, fields: Option<Fields>) -> Self {
            self.answers.entry(name).or_default().push_back(fields);
            self
        }

        fn how_many_times(&self, name: &str) -> usize {
            self.asked.iter().filter(|(asked, _)| asked == name).count()
        }
    }

    impl AsksTheSupervisor for AMachineInHand {
        fn ask(&mut self, request: &Request) -> WhatCameBack {
            self.asked
                .push((request.name.clone(), request.fields.clone()));
            let queued = self
                .answers
                .iter_mut()
                .find(|(name, _)| **name == request.name)
                .map(|(_, queued)| queued);
            let Some(queued) = queued else {
                return WhatCameBack::ItAnswered(
                    Answered::to(request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, "test")
                        .saying_it_does_not_answer_that_request(&request.name),
                );
            };
            let next = if queued.len() > 1 {
                queued.pop_front().flatten()
            } else {
                queued.front().cloned().flatten()
            };
            match next {
                Some(fields) => WhatCameBack::ItAnswered(
                    Answered::to(request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, "test")
                        .carrying(fields),
                ),
                None => WhatCameBack::NothingCorrelatedCameBack,
            }
        }
    }

    // ── A record, in memory ──────────────────────────────────────────────────

    pub(crate) struct ARecordInHand(pub(crate) rusqlite::Connection);

    impl LendsTheRecord for ARecordInHand {
        fn lend(&self, act: &mut dyn FnMut(&rusqlite::Connection)) {
            act(&self.0)
        }
        fn write_a_diary_line(&self, _act: &smoozy_keys::diary::Act) {}
    }

    pub(crate) fn a_record_with_a_server() -> (ARecordInHand, Identity) {
        let connection = rusqlite::Connection::open_in_memory().unwrap();
        for migration in crate::record::migrations::ALL {
            if let crate::record::migrations::WhatItRuns::TheseStatements(statements) =
                migration.what_it_runs
            {
                for statement in statements {
                    connection.execute(statement, []).unwrap();
                }
            }
        }
        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        let row = ServerRow::saved(
            server.as_text(),
            Host::what_the_user_typed("127.0.0.1").expect("a host"),
            Port::what_the_user_typed(22).expect("a port"),
            Username::what_the_user_typed("root").expect("a username"),
            KeyReference {
                local_key_identifier: "key-1".into(),
                fingerprint: "SHA256:aaa".into(),
            },
        );
        server_rows::add(&connection, &row).unwrap();
        (ARecordInHand(connection), server)
    }

    pub(crate) fn a_reading_with_a_saved_prefix(
        job: &Identity,
        attempt: u32,
        saved: i64,
    ) -> Reading<TheMachinesWorkingState> {
        use smoozy_queue_history::readings::{AWorkersReport, LiveReadings};
        Reading::taken(
            TheMachinesWorkingState {
                jobs: vec![AJobAsTheMachineReadsIt {
                    job: job.clone(),
                    state: QueueState::Rendering {
                        operation: Identity::issue(IdentityKind::Operation).unwrap(),
                    },
                    live: WhatTheEntryHasLive::Readings {
                        figures: Some(AWorkersReport {
                            attempt,
                            readings: Reading::taken(
                                LiveReadings::default(),
                                SystemTime::now(),
                                Clock::AMachine(MachineClock("test".into())),
                            ),
                            newest_frame_saved: Some(saved),
                            the_plan_total: None,
                            the_units_finished: None,
                        }),
                        sinks: None,
                    },
                    the_hold: None,
                }],
                the_admission_point_stands_at: None,
                notes: Vec::new(),
                the_history_copys_count: 0,
                the_history_copys_entries: Vec::new(),
            },
            SystemTime::now(),
            Clock::ThisComputer,
        )
    }

    #[test]
    fn seventeenth_download_evidence_belongs_to_the_connection_that_read_it() {
        let holds = WhatThisRunHolds::default();
        let job = Identity::issue(IdentityKind::Job).unwrap();
        let first = holds.a_connection_begins("server");
        holds.a_reading_was_taken(
            "server",
            first,
            a_reading_with_a_saved_prefix(&job, 2, 10),
            None,
        );
        assert_eq!(
            holds
                .the_saved_prefixes_reported_on_this_connection("server")
                .get(&job.as_text()),
            Some(&(2, 10))
        );
        holds.the_connection_ended("server");
        assert!(holds
            .the_saved_prefixes_reported_on_this_connection("server")
            .is_empty());
        let second = holds.a_connection_begins("server");
        assert!(holds
            .the_saved_prefixes_reported_on_this_connection("server")
            .is_empty());
        holds.a_reading_was_taken(
            "server",
            first,
            a_reading_with_a_saved_prefix(&job, 2, 20),
            None,
        );
        assert!(holds
            .the_saved_prefixes_reported_on_this_connection("server")
            .is_empty());
        holds.a_reading_was_taken(
            "server",
            second,
            a_reading_with_a_saved_prefix(&job, 3, -4),
            None,
        );
        assert_eq!(
            holds
                .the_saved_prefixes_reported_on_this_connection("server")
                .get(&job.as_text()),
            Some(&(3, -4))
        );
    }

    fn a_provisional_record(
        server: &Identity,
        label: &str,
    ) -> smoozy_queue_history::record::LocalJobRecord {
        let mut made = a_record_in_history(server, label);
        made.standing = Standing::Provisional;
        made.the_render_start_has_been_learned = false;
        made
    }

    fn write(record: &ARecordInHand, made: &smoozy_queue_history::record::LocalJobRecord) {
        JobRecords::over(&record.0)
            .write_the_provisional_record(made)
            .unwrap();
    }

    fn on_the_machine(word: &str) -> Option<Fields> {
        Some(Fields::none().and(ON_THIS_MACHINE_FIELD, word))
    }

    fn nothing_is_live() -> bool {
        true
    }

    // ── `State, identity and recovery` V1 step 2 ────────────────────────────

    /// **`State, identity and recovery` V1 step 2's third branch, held by
    /// name: *absent, and the machine cannot say* leaves the record
    /// provisional, and no absence of any length removes it.**
    ///
    /// Every way the machine can fail to say is tried — its own *cannot say*,
    /// a silence, a refusal of the request by name, an answer with no word in
    /// it — **and each over a hundred connections**, because the fault this
    /// guards against is a count or an age turning the third branch into the
    /// second.
    #[test]
    fn a_record_the_machine_cannot_speak_for_is_never_removed() {
        let (record, server) = a_record_with_a_server();
        let kept = a_provisional_record(&server, "Kitchen");
        write(&record, &kept);
        let answers: [Option<Fields>; 4] = [
            on_the_machine(smoozy_server_protocol::requests::THE_MACHINE_CANNOT_SAY),
            None,
            Some(Fields::none()),
            Some(Fields::none().and(ON_THIS_MACHINE_FIELD, "a word this build does not know")),
        ];
        for answer in answers {
            for _connection in 0..100 {
                let mut machine = AMachineInHand::default()
                    .answers(THE_DOES_THE_MACHINE_HOLD_THIS_JOB_REQUEST, answer.clone());
                let mut did = WhatStepSevenDid::default();
                every_provisional_record_is_resolved(&server, &record, &mut machine, &mut did);
                assert_eq!(did.still_provisional, 1);
            }
        }
        // **And a machine that does not answer the request at all** — the
        // program before step 10c piece 5 — is the same branch.
        for _connection in 0..100 {
            let mut machine = AMachineInHand::default();
            let mut did = WhatStepSevenDid::default();
            every_provisional_record_is_resolved(&server, &record, &mut machine, &mut did);
            assert_eq!(did.still_provisional, 1);
        }
        let still = JobRecords::over(&record.0).read(&kept.job).unwrap();
        assert_eq!(
            still.map(|held| held.standing),
            Some(Standing::Provisional),
            "**`State, identity and recovery` V1 step 2: no absence, of any length, removes a \
             provisional record** — `Preparation/Specs/Whole app.md` forbids the \
             determination a timeout would need"
        );
    }

    /// **The other two branches**: *present* confirms, and only an answer that
    /// says *absent* removes.
    #[test]
    fn present_confirms_and_only_absent_removes() {
        let (record, server) = a_record_with_a_server();
        let held = a_provisional_record(&server, "Held");
        write(&record, &held);
        let mut machine = AMachineInHand::default().answers(
            THE_DOES_THE_MACHINE_HOLD_THIS_JOB_REQUEST,
            on_the_machine(IT_IS_PRESENT),
        );
        let mut did = WhatStepSevenDid::default();
        every_provisional_record_is_resolved(&server, &record, &mut machine, &mut did);
        assert_eq!(did.confirmed, 1);
        assert_eq!(
            JobRecords::over(&record.0)
                .read(&held.job)
                .unwrap()
                .map(|one| one.standing),
            Some(Standing::Confirmed)
        );

        let refused = a_provisional_record(&server, "Refused");
        write(&record, &refused);
        let mut machine = AMachineInHand::default().answers(
            THE_DOES_THE_MACHINE_HOLD_THIS_JOB_REQUEST,
            on_the_machine(IT_IS_ABSENT),
        );
        let mut did = WhatStepSevenDid::default();
        every_provisional_record_is_resolved(&server, &record, &mut machine, &mut did);
        assert_eq!(did.removed, 1);
        assert!(JobRecords::over(&record.0)
            .read(&refused.job)
            .unwrap()
            .is_none());
        assert!(
            JobRecords::over(&record.0)
                .read(&held.job)
                .unwrap()
                .is_some(),
            "a confirmed record is never asked about again"
        );
    }

    // ── `State, identity and recovery` V1 step 4 ────────────────────────────

    fn a_note_that_started_and_finished(label: &str) -> Fields {
        Fields::none()
            .and(ON_THIS_MACHINE_FIELD, IT_IS_PRESENT)
            .and(THE_LABEL_FIELD, label)
            .and(THE_ATTEMPTS_FIELD, "1")
            .and(HOW_MANY_RE_RUNS_FIELD, "0")
            .and(THE_STARTED_FIELD, "1754700000")
            .and(THE_STARTED_CLOCK_FIELD, "the machine")
            .and(THE_ENDED_FIELD, "1754703600")
            .and(THE_ENDED_CLOCK_FIELD, "the machine")
            .and(THE_ENDING_FIELD, IT_FINISHED)
            .and(THE_ATTEMPT_THAT_HOLDS_THE_WORDS_FIELD, "1")
            .and(THE_EXIT_STATUS_FIELD, "0")
            .and(THE_LAST_FRAME_FIELD, "240")
    }

    /// **Step 4: a confirmed record is filled in from the machine's note**, and
    /// from then on it is in history (`Queue & History` C9).
    #[test]
    fn a_confirmed_record_is_filled_in_from_the_note_and_enters_history() {
        let (record, server) = a_record_with_a_server();
        let mut confirmed = a_provisional_record(&server, "Kitchen");
        confirmed.standing = Standing::Confirmed;
        write(&record, &confirmed);
        let mut machine = AMachineInHand::default().answers(
            THE_PER_JOB_NOTE_READING_REQUEST,
            Some(a_note_that_started_and_finished("Kitchen")),
        );
        assert_eq!(every_record_is_filled_in(&server, &record, &mut machine), 1);
        let filled = JobRecords::over(&record.0)
            .read(&confirmed.job)
            .unwrap()
            .unwrap();
        assert!(filled.the_render_start_has_been_learned);
        assert_eq!(
            filled.the_ending_learned(),
            Some(smoozy_queue_history::ending::Ending::Finished)
        );
        assert_eq!(filled.what_the_note_carried.last_frame_figure, Some(240));
        assert_eq!(
            filled
                .what_the_note_carried
                .started
                .as_ref()
                .map(|at| at.clock().clone()),
            Some(Clock::AMachine(MachineClock("the machine".to_string()))),
            "**`Queue & History` N16: the note's moments are the machine's**, on its clock"
        );
        let history = JobRecords::over(&record.0).in_history().unwrap();
        assert_eq!(history.entries.len(), 1);

        // **An ended record is not asked about again.**
        let mut machine = AMachineInHand::default();
        assert_eq!(every_record_is_filled_in(&server, &record, &mut machine), 0);
        assert_eq!(machine.how_many_times(THE_PER_JOB_NOTE_READING_REQUEST), 0);
    }

    /// **Step 10c's fix piece 2, the desktop's whole half: the note reading's
    /// answer, composed by the very function the machine's program sends it
    /// with, is read, filled in, kept, and drawn** — a failed job's reason
    /// behind *What the server said*, and a finished job's device line as
    /// *Ran on* (the paragraph after `Queue & History` N3, and
    /// `Queue & History` E12).
    ///
    /// **The failed job printed a line too, and it still reads *not known***,
    /// because a line from a run that did not finish is not evidence. **Undone
    /// at either end** — the machine not sending the two, or this side not
    /// keeping them — the finished entry reads *Ran on — not known* and the
    /// failed one has no *What the server said*, which is what all four
    /// providers showed on the seventh passes.
    #[test]
    fn a_failed_and_a_finished_job_reach_history_with_the_reason_and_the_line() {
        use crate::surface::the_history::{the_entry_for, BesideTheRecord};
        use smoozy_queue_history::acting::note::{TheEndingOnTheNote, TheOutcomeAsWritten};

        let line = "Path tracing on: NVIDIA GeForce RTX 4090 (CUDA)";
        let reason = "the render ended with status 3";
        let the_answer = |outcome, the_reason: Option<&str>| {
            let on_the_machine = Clock::AMachine(MachineClock("the machine".to_string()));
            let mut note = TheNote::created_with_the_job_record("Kitchen");
            note.started = Some(Reading::taken(
                (),
                SystemTime::UNIX_EPOCH,
                on_the_machine.clone(),
            ));
            note.ended = Some(Reading::taken((), SystemTime::UNIX_EPOCH, on_the_machine));
            note.attempts = 1;
            note.ending = Some(TheEndingOnTheNote {
                outcome,
                exit_status: Some(if the_reason.is_some() { 3 } else { 0 }),
                the_attempt_whose_capture_record_holds_the_words: 1,
                the_reason_the_machine_recorded: the_reason.map(str::to_string),
                no_render_began: false,
                the_device_line_as_observed: Some(line.to_string()),
            });
            // **The machine's own composition**, which `smoozy-server` sends.
            note.as_the_answers_fields()
        };

        for (outcome, the_reason) in [
            (TheOutcomeAsWritten::Failed, Some(reason)),
            (TheOutcomeAsWritten::Finished, None),
        ] {
            let (record, server) = a_record_with_a_server();
            let mut confirmed = a_provisional_record(&server, "Kitchen");
            confirmed.standing = Standing::Confirmed;
            write(&record, &confirmed);
            let mut machine = AMachineInHand::default().answers(
                THE_PER_JOB_NOTE_READING_REQUEST,
                Some(the_answer(outcome, the_reason)),
            );
            assert_eq!(every_record_is_filled_in(&server, &record, &mut machine), 1);

            let filled = JobRecords::over(&record.0)
                .read(&confirmed.job)
                .unwrap()
                .unwrap();
            let entry = the_entry_for(
                &filled,
                &BesideTheRecord::read(&record.0, &confirmed.job),
                false,
                true,
            );
            let ran_on: Vec<String> = entry
                .rendered
                .iter()
                .find(|fact| fact.name == "Ran on")
                .expect("the line is always drawn")
                .parts
                .iter()
                .map(|part| format!("{}{}{}", part.lead, part.value, part.trail))
                .collect();
            match outcome {
                TheOutcomeAsWritten::Finished => {
                    assert_eq!(ran_on, vec![line.to_string()]);
                    assert!(entry.failure.is_none());
                }
                _ => {
                    assert_eq!(ran_on, vec!["not known".to_string()]);
                    assert_eq!(
                        entry
                            .failure
                            .expect("a failed ending carries the shape")
                            .what_the_server_said
                            .as_deref(),
                        Some(reason)
                    );
                }
            }
        }
    }

    /// **A provisional record is never filled in**: it is not known to be the
    /// machine's until step 2 says so.
    #[test]
    fn a_provisional_record_is_not_filled_in() {
        let (record, server) = a_record_with_a_server();
        let provisional = a_provisional_record(&server, "Kitchen");
        write(&record, &provisional);
        let mut machine = AMachineInHand::default().answers(
            THE_PER_JOB_NOTE_READING_REQUEST,
            Some(a_note_that_started_and_finished("Kitchen")),
        );
        assert_eq!(every_record_is_filled_in(&server, &record, &mut machine), 0);
        assert_eq!(machine.how_many_times(THE_PER_JOB_NOTE_READING_REQUEST), 0);
    }

    // ── `State, identity and recovery` V1 step 3 ────────────────────────────

    fn events(from: u64, kinds: &[(&str, Option<&Identity>)], oldest: Option<u64>) -> Fields {
        let mut fields = Fields::none()
            .and(HOW_MANY_EVENTS_FIELD, kinds.len().to_string())
            .and(
                THE_NEXT_SEQUENCE_FIELD,
                (from + kinds.len() as u64).to_string(),
            );
        if let Some(oldest) = oldest {
            fields = fields.and(THE_OLDEST_RETAINED_FIELD, oldest.to_string());
        }
        for (at, (kind, about)) in kinds.iter().enumerate() {
            fields = fields
                .and(
                    &the_field_of_item(AN_EVENT, at, THE_SEQUENCE_FIELD),
                    (from + at as u64).to_string(),
                )
                .and(&the_field_of_item(AN_EVENT, at, THE_KIND_FIELD), *kind);
            if let Some(about) = about {
                fields = fields.and(
                    &the_field_of_item(AN_EVENT, at, THE_ABOUT_FIELD),
                    about.as_text(),
                );
            }
        }
        fields
    }

    /// **`Queue & History` C2's record for one server, as the local record
    /// holds it** — the 38's piece 15; until then the account was the run's.
    fn the_account_of(
        record: &ARecordInHand,
        server: &Identity,
    ) -> Option<smoozy_queue_history::catch_up::NoticeRecord> {
        crate::record::catch_up_notices::the_standing_notice(&record.0, server).unwrap()
    }

    fn the_cursor_of(record: &ARecordInHand, server: &Identity) -> Option<u64> {
        server_rows::one(&record.0, &server.as_text())
            .unwrap()
            .unwrap()
            .event_cursor
    }

    /// **Step 3: the events are applied, the account composed, and only then
    /// the cursor written and the machine told** — in that order.
    #[test]
    fn events_are_applied_before_the_cursor_moves_and_the_account_names_the_ending() {
        let (record, server) = a_record_with_a_server();
        let mut confirmed = a_provisional_record(&server, "Kitchen");
        confirmed.standing = Standing::Confirmed;
        write(&record, &confirmed);
        let mut machine = AMachineInHand::default()
            .answers(
                THE_READ_FORWARD_REQUEST,
                Some(events(
                    0,
                    &[
                        (THE_RENDER_ATTEMPT_STARTED_EVENT, Some(&confirmed.job)),
                        (THE_RECORDED_OUTCOME_WRITTEN_EVENT, Some(&confirmed.job)),
                    ],
                    Some(0),
                )),
            )
            .answers(THE_READ_FORWARD_REQUEST, Some(events(2, &[], Some(0))))
            .answers(
                THE_PER_JOB_NOTE_READING_REQUEST,
                Some(a_note_that_started_and_finished("Kitchen")),
            )
            .answers(THE_CURSOR_MOVES_FORWARD_REQUEST, Some(Fields::none()));
        let holds = WhatThisRunHolds::default();
        assert_eq!(
            a_read_forward_pass(&server, &record, &mut machine, &holds),
            HowThePassEnded::ItReadToTheEnd { applied: 2 }
        );
        assert_eq!(the_cursor_of(&record, &server), Some(2));
        let order: Vec<&str> = machine
            .asked
            .iter()
            .map(|(name, _)| name.as_str())
            .collect();
        let note_at = order
            .iter()
            .position(|name| *name == THE_PER_JOB_NOTE_READING_REQUEST)
            .unwrap();
        let moved_at = order
            .iter()
            .position(|name| *name == THE_CURSOR_MOVES_FORWARD_REQUEST)
            .unwrap();
        assert!(
            note_at < moved_at,
            "**`State, identity and recovery` E3: the cursor advances after the work the event \
             asked for is done, never on receipt**: {order:?}"
        );
        let account = the_account_of(&record, &server).expect("an account");
        assert_eq!(account.entries.len(), 1, "the ending is named once");
        assert!(!account.the_account_is_incomplete);

        // **`State, identity and recovery` E4: the same event read again —
        // its own sequence number, 1 — doubles nothing.**
        let mut again = AMachineInHand::default()
            .answers(
                THE_READ_FORWARD_REQUEST,
                Some(events(
                    1,
                    &[(THE_RECORDED_OUTCOME_WRITTEN_EVENT, Some(&confirmed.job))],
                    Some(0),
                )),
            )
            .answers(THE_READ_FORWARD_REQUEST, Some(events(2, &[], Some(0))));
        a_read_forward_pass(&server, &record, &mut again, &holds);
        assert_eq!(the_account_of(&record, &server).unwrap().entries.len(), 1);
    }

    /// **A job submitted inside a standing period is confirmed by the pass
    /// that reads its events**, through step 2's own ask, and then filled in —
    /// so its start and ending reach history without waiting for a
    /// reconnection. **A machine that cannot say still removes nothing.**
    #[test]
    fn a_pass_confirms_a_record_its_events_name_and_fills_it_in() {
        let (record, server) = a_record_with_a_server();
        let submitted = a_provisional_record(&server, "Kitchen");
        write(&record, &submitted);
        let the_events = events(
            0,
            &[
                (THE_JOB_RECORD_WRITTEN_EVENT, Some(&submitted.job)),
                (THE_RECORDED_OUTCOME_WRITTEN_EVENT, Some(&submitted.job)),
            ],
            Some(0),
        );

        let mut cannot_say = AMachineInHand::default()
            .answers(THE_READ_FORWARD_REQUEST, Some(the_events.clone()))
            .answers(THE_READ_FORWARD_REQUEST, Some(events(2, &[], Some(0))))
            .answers(
                THE_DOES_THE_MACHINE_HOLD_THIS_JOB_REQUEST,
                on_the_machine(smoozy_server_protocol::requests::THE_MACHINE_CANNOT_SAY),
            );
        a_read_forward_pass(
            &server,
            &record,
            &mut cannot_say,
            &WhatThisRunHolds::default(),
        );
        assert_eq!(
            JobRecords::over(&record.0)
                .read(&submitted.job)
                .unwrap()
                .map(|held| held.standing),
            Some(Standing::Provisional)
        );

        let mut row = server_rows::one(&record.0, &server.as_text())
            .unwrap()
            .unwrap();
        row.event_cursor = None;
        server_rows::write(&record.0, &row).unwrap();
        let mut present = AMachineInHand::default()
            .answers(THE_READ_FORWARD_REQUEST, Some(the_events))
            .answers(THE_READ_FORWARD_REQUEST, Some(events(2, &[], Some(0))))
            .answers(
                THE_DOES_THE_MACHINE_HOLD_THIS_JOB_REQUEST,
                on_the_machine(IT_IS_PRESENT),
            )
            .answers(
                THE_PER_JOB_NOTE_READING_REQUEST,
                Some(a_note_that_started_and_finished("Kitchen")),
            );
        a_read_forward_pass(&server, &record, &mut present, &WhatThisRunHolds::default());
        let held = JobRecords::over(&record.0)
            .read(&submitted.job)
            .unwrap()
            .unwrap();
        assert_eq!(held.standing, Standing::Confirmed);
        assert!(held.the_render_start_has_been_learned);
    }

    /// **A pass asks only about the provisional records its events name** —
    /// step 10c's twelfth-pass fix piece 4, from that pass's fault 5. A
    /// first-frame check's rest is written provisional before its request
    /// goes out; a pass that reads the first job's event in that moment asks
    /// about the first job and not the rest, **so the machine's true *absent*
    /// for a job it has not been sent yet never removes the rest's record**.
    /// The rest is asked about when its own event is read.
    #[test]
    fn a_pass_asks_only_about_the_provisional_records_its_events_name() {
        let (record, server) = a_record_with_a_server();
        let first = a_provisional_record(&server, "p12 fc3");
        let rest = a_provisional_record(&server, "p12 fc3");
        write(&record, &first);
        write(&record, &rest);
        // **The machine holds the first job and not yet the rest**: the first
        // ask is answered *present*, and any ask after it *absent*.
        let mut machine = AMachineInHand::default()
            .answers(
                THE_READ_FORWARD_REQUEST,
                Some(events(
                    0,
                    &[(THE_JOB_RECORD_WRITTEN_EVENT, Some(&first.job))],
                    Some(0),
                )),
            )
            .answers(THE_READ_FORWARD_REQUEST, Some(events(1, &[], Some(0))))
            .answers(
                THE_DOES_THE_MACHINE_HOLD_THIS_JOB_REQUEST,
                on_the_machine(IT_IS_PRESENT),
            )
            .answers(
                THE_DOES_THE_MACHINE_HOLD_THIS_JOB_REQUEST,
                on_the_machine(IT_IS_ABSENT),
            );
        a_read_forward_pass(&server, &record, &mut machine, &WhatThisRunHolds::default());
        let asked_about: Vec<Option<&str>> = machine
            .asked
            .iter()
            .filter(|(name, _)| name == THE_DOES_THE_MACHINE_HOLD_THIS_JOB_REQUEST)
            .map(|(_, fields)| fields.text(THE_JOB_FIELD))
            .collect();
        assert_eq!(
            asked_about,
            vec![Some(first.job.as_text().as_str())],
            "only the job the event names"
        );
        let standing = |job: &Identity| {
            JobRecords::over(&record.0)
                .read(job)
                .unwrap()
                .map(|held| held.standing)
        };
        assert_eq!(standing(&first.job), Some(Standing::Confirmed));
        assert_eq!(
            standing(&rest.job),
            Some(Standing::Provisional),
            "the rest's record stands until its own event asks about it"
        );
    }

    /// **`Queue & History` L23's last bullet (the stop and clear plan pass's
    /// SC-1): each removal the machine journals takes that job's local record
    /// with it, provisional or confirmed alike** — `State, identity and
    /// recovery` H8, first bullet, applied at that document's §7 step 3. **A
    /// job the event does not name stays**, and nothing is inferred from a job
    /// missing from a reading. **The event may arrive twice for one job** (S1's
    /// account), and the second arrival is nothing (`State, identity and
    /// recovery` E4). Step 10c's S2.
    #[test]
    fn a_removal_the_machine_journalled_takes_the_local_record_and_a_second_arrival_is_nothing() {
        use smoozy_server_protocol::requests::THE_MACHINE_CANNOT_SAY;
        let (record, server) = a_record_with_a_server();
        let mut waiting = a_provisional_record(&server, "Kitchen");
        waiting.standing = Standing::Confirmed;
        write(&record, &waiting);
        let provisional = a_provisional_record(&server, "Hall");
        write(&record, &provisional);
        let mut kept = a_provisional_record(&server, "Stairs");
        kept.standing = Standing::Confirmed;
        write(&record, &kept);
        // **A job of another server, named by an event on this one**, is not
        // this machine's to remove.
        let elsewhere = Identity::issue(IdentityKind::LocalServer).unwrap();
        let mut theirs = a_provisional_record(&elsewhere, "Attic");
        theirs.standing = Standing::Confirmed;
        write(&record, &theirs);
        let the_events = [
            (THE_JOB_RECORD_REMOVED_EVENT, Some(&waiting.job)),
            (THE_JOB_RECORD_REMOVED_EVENT, Some(&provisional.job)),
            (THE_JOB_RECORD_REMOVED_EVENT, Some(&theirs.job)),
            (THE_JOB_RECORD_REMOVED_EVENT, Some(&waiting.job)),
        ];
        let mut machine = AMachineInHand::default()
            .answers(
                THE_READ_FORWARD_REQUEST,
                Some(events(0, &the_events, Some(0))),
            )
            .answers(THE_READ_FORWARD_REQUEST, Some(events(4, &[], Some(0))))
            .answers(
                THE_DOES_THE_MACHINE_HOLD_THIS_JOB_REQUEST,
                on_the_machine(THE_MACHINE_CANNOT_SAY),
            )
            .answers(THE_CURSOR_MOVES_FORWARD_REQUEST, Some(Fields::none()));
        let holds = WhatThisRunHolds::default();
        assert_eq!(
            a_read_forward_pass(&server, &record, &mut machine, &holds),
            HowThePassEnded::ItReadToTheEnd { applied: 4 }
        );
        let held = |job: &Identity| JobRecords::over(&record.0).read(job).unwrap();
        assert!(
            held(&waiting.job).is_none(),
            "a confirmed record the clear removed is still held"
        );
        assert!(
            held(&provisional.job).is_none(),
            "a provisional record the clear removed is still held"
        );
        assert!(
            held(&kept.job).is_some(),
            "a job no event named was removed"
        );
        assert!(
            held(&theirs.job).is_some(),
            "another server's job was removed"
        );
        assert_eq!(the_cursor_of(&record, &server), Some(4));

        // **The same events read again** — a cursor put back — remove nothing
        // more and fail nothing.
        let mut row = server_rows::one(&record.0, &server.as_text())
            .unwrap()
            .unwrap();
        row.event_cursor = None;
        server_rows::write(&record.0, &row).unwrap();
        let mut again = AMachineInHand::default()
            .answers(
                THE_READ_FORWARD_REQUEST,
                Some(events(0, &the_events, Some(0))),
            )
            .answers(THE_READ_FORWARD_REQUEST, Some(events(4, &[], Some(0))))
            .answers(THE_CURSOR_MOVES_FORWARD_REQUEST, Some(Fields::none()));
        assert_eq!(
            a_read_forward_pass(&server, &record, &mut again, &holds),
            HowThePassEnded::ItReadToTheEnd { applied: 4 }
        );
        assert!(held(&kept.job).is_some());
    }

    /// **`Queue & History` H9 and H11: only a release *dropped* followed at
    /// once by a removal is read as a drop.** A held job's *no* (a release
    /// *stopped*), a release and a removal with another event between them,
    /// and a job of another server are each named nowhere. S3's loose ends,
    /// 2026-09-29.
    #[test]
    fn only_a_release_dropped_then_the_removal_is_read_as_a_drop() {
        let (record, server) = a_record_with_a_server();
        let mut ours = a_provisional_record(&server, "Kitchen — final v4");
        ours.standing = Standing::Confirmed;
        write(&record, &ours);
        let elsewhere = Identity::issue(IdentityKind::LocalServer).unwrap();
        let mut theirs = a_provisional_record(&elsewhere, "Attic");
        theirs.standing = Standing::Confirmed;
        write(&record, &theirs);
        let hold = Identity::issue(IdentityKind::Operation).unwrap();
        let an_event = |sequence: u64, kind: &str, about: &Identity, says: &str| AnEventRead {
            sequence,
            kind: kind.to_string(),
            about: Some(about.as_text()),
            says: Some(says.to_string()),
        };
        let dropped = "the hold's release outcome was written: dropped";
        let holds = WhatThisRunHolds::default();
        let named =
            |events: &[AnEventRead]| the_held_jobs_dropped_in(events, &server, &record, &holds);

        let a_no = [
            an_event(
                4,
                THE_RELEASE_EVENT,
                &hold,
                "the hold's release outcome was written: stopped",
            ),
            an_event(5, THE_JOB_RECORD_REMOVED_EVENT, &ours.job, "removed"),
        ];
        assert!(
            named(&a_no).is_empty(),
            "a held job's no was read as a drop"
        );
        let apart = [
            an_event(4, THE_RELEASE_EVENT, &hold, dropped),
            an_event(6, THE_JOB_RECORD_REMOVED_EVENT, &ours.job, "removed"),
        ];
        assert!(
            named(&apart).is_empty(),
            "a removal not next to the release was read as its drop"
        );
        let another_servers = [
            an_event(4, THE_RELEASE_EVENT, &hold, dropped),
            an_event(5, THE_JOB_RECORD_REMOVED_EVENT, &theirs.job, "removed"),
        ];
        assert!(
            named(&another_servers).is_empty(),
            "another server's job was named"
        );
        assert_eq!(holds.the_name_of_a_dropped_job(&ours.job), None);

        let a_drop = [
            an_event(4, THE_RELEASE_EVENT, &hold, dropped),
            an_event(5, THE_JOB_RECORD_REMOVED_EVENT, &ours.job, "removed"),
        ];
        let told = || {
            vec![(
                ours.job.clone(),
                "Kitchen — final v4".to_string(),
                dropped.to_string(),
                5,
            )]
        };
        assert_eq!(named(&a_drop), told());
        assert_eq!(
            holds.the_name_of_a_dropped_job(&ours.job).as_deref(),
            Some("Kitchen — final v4")
        );

        // **A pass that took the record and then stopped short is read again,
        // and the drop is still named**, by what this run held.
        JobRecords::over(&record.0).remove(&ours.job).unwrap();
        assert_eq!(named(&a_drop), told());
    }

    #[test]
    fn twentieth_telegram_off_does_not_hide_a_removed_blender_at_the_desk() {
        let (record, server) = a_record_with_a_server();
        let mut job = a_provisional_record(&server, "Silent job");
        job.standing = Standing::Confirmed;
        write(&record, &job);
        let holds = WhatThisRunHolds::default();
        let words = smoozy_server_protocol::requests::the_drops_words("5.2.3", "Silent job");
        for says in [words.clone(), format!("Telegram off: {words}")] {
            let events = [AnEventRead {
                sequence: 9,
                kind: THE_JOB_RECORD_REMOVED_EVENT.to_string(),
                about: Some(job.job.as_text()),
                says: Some(says),
            }];
            assert_eq!(
                the_waiting_jobs_dropped_for_a_removed_blender_in(
                    &events, &server, &record, &holds
                ),
                vec![(
                    job.job.clone(),
                    "Silent job".to_string(),
                    "5.2.3".to_string(),
                    9
                )]
            );
        }
    }

    /// **The machine's half of what [`the_held_jobs_dropped_in`] reads**: its
    /// release says the outcome last, in the word read here, and `Queue &
    /// History` H9's drop journals the release and then the removal, with
    /// nothing between. This fails if either moves.
    ///
    /// **Since 2026-10-16 a drop's release carries what the phone is told
    /// from ahead of those words** (`Rented-server runtime` U20 as dated that
    /// day, the Telegram worker's piece 4), journalled by the machine's
    /// `journal_a_drop`; the outcome is still the last thing it says, and this
    /// reader ignores what comes before.
    #[test]
    fn the_machine_journals_a_drop_the_way_it_is_read_here() {
        let program = include_str!("../../../Rented-server runtime — program/src/main.rs");
        assert!(program.contains(
            "\"the hold's release outcome was written: {}\",\n                format!(\"{:?}\", release.outcome).to_lowercase()"
        ));
        assert!(include_str!("../../../Rented-server runtime — program/src/the_telegram_journal.rs")
            .contains("pub const THE_TAIL_OF_A_DROPS_ENTRY: &str = \"the hold's release outcome was written: dropped\";"));
        let h9 = program
            .split("fn the_holds_on_what_was_removed_with_silence")
            .nth(1)
            .expect("H9's drop");
        let h9 = &h9[..h9.find("\n        }\n").expect("its end")];
        let release = h9
            .find("self.journal_a_drop(hold, &release, &record);")
            .expect("the release journalled");
        let removal = h9
            .find("self.after_a_removal(&record.job")
            .expect("the removal");
        assert!(release < removal);
        let after = program
            .split("fn after_a_removal(")
            .nth(1)
            .expect("after_a_removal");
        let (_, body) = after.split_once(") {").expect("its body");
        assert!(
            body.split("self.journal_or_hold(")
                .nth(1)
                .is_some_and(|first| first
                    .trim_start()
                    .starts_with("asked::THE_JOB_RECORD_REMOVED_EVENT")),
            "the removal is no longer the first thing journalled after the release"
        );
    }

    /// **A read the machine does not answer moves nothing** (`State, identity
    /// and recovery` V2).
    #[test]
    fn a_read_that_is_not_answered_moves_no_cursor() {
        let (record, server) = a_record_with_a_server();
        let mut machine = AMachineInHand::default().answers(THE_READ_FORWARD_REQUEST, None);
        let holds = WhatThisRunHolds::default();
        assert_eq!(
            a_read_forward_pass(&server, &record, &mut machine, &holds),
            HowThePassEnded::TheReadCouldNotComplete { applied: 0 }
        );
        assert_eq!(the_cursor_of(&record, &server), None);
        assert_eq!(machine.how_many_times(THE_CURSOR_MOVES_FORWARD_REQUEST), 0);
    }

    /// **`Queue & History` C4: a gap is stated, never smoothed.** Where the
    /// journal's oldest retained number has passed the cursor the desktop
    /// held, the account says it is incomplete.
    #[test]
    fn a_gap_is_said() {
        let (record, server) = a_record_with_a_server();
        let mut row = server_rows::one(&record.0, &server.as_text())
            .unwrap()
            .unwrap();
        row.event_cursor = Some(3);
        server_rows::write(&record.0, &row).unwrap();
        let mut machine = AMachineInHand::default()
            .answers(
                THE_READ_FORWARD_REQUEST,
                Some(events(9, &[(THE_HOLD_PLACED_EVENT, None)], Some(9))),
            )
            .answers(THE_READ_FORWARD_REQUEST, Some(events(10, &[], Some(9))));
        let holds = WhatThisRunHolds::default();
        a_read_forward_pass(&server, &record, &mut machine, &holds);
        assert!(
            the_account_of(&record, &server)
                .unwrap()
                .the_account_is_incomplete
        );
        assert_eq!(the_cursor_of(&record, &server), Some(10));
    }

    /// **A journal the way the machine's program keeps one**: every event from
    /// sequence 0, **at most 512 carried by one read**, and the answer naming
    /// the journal's own next number — `THE_MOST_EVENTS_ONE_READ_CARRIES` and
    /// `the_read_forward` in `Rented-server runtime — program/src/main.rs`,
    /// over `read_forward_from` in `Rented-server runtime/src/acting/journal.rs`.
    /// Every cursor a read was asked from is written down.
    struct AJournalInHand {
        kinds: Vec<(&'static str, Option<Identity>)>,
        the_note: Fields,
        read_from: Vec<u64>,
    }

    /// The machine's own figure, copied rather than imported: the desktop does
    /// not depend on the program, and the read must never rely on it.
    const THE_MACHINES_LIMIT: usize = 512;

    impl AsksTheSupervisor for AJournalInHand {
        fn ask(&mut self, request: &Request) -> WhatCameBack {
            let answered = Answered::to(request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, "test");
            let fields = match request.name.as_str() {
                THE_READ_FORWARD_REQUEST => {
                    let from: u64 = request
                        .fields
                        .text(THE_CURSOR_FIELD)
                        .unwrap()
                        .parse()
                        .unwrap();
                    self.read_from.push(from);
                    let carried: Vec<_> = self
                        .kinds
                        .iter()
                        .enumerate()
                        .skip(from as usize)
                        .take(THE_MACHINES_LIMIT)
                        .collect();
                    let mut fields = Fields::none()
                        .and(HOW_MANY_EVENTS_FIELD, carried.len().to_string())
                        .and(THE_NEXT_SEQUENCE_FIELD, self.kinds.len().to_string())
                        .and(THE_OLDEST_RETAINED_FIELD, "0");
                    for (at, (sequence, (kind, about))) in carried.into_iter().enumerate() {
                        fields = fields
                            .and(
                                &the_field_of_item(AN_EVENT, at, THE_SEQUENCE_FIELD),
                                sequence.to_string(),
                            )
                            .and(&the_field_of_item(AN_EVENT, at, THE_KIND_FIELD), *kind);
                        if let Some(about) = about {
                            fields = fields.and(
                                &the_field_of_item(AN_EVENT, at, THE_ABOUT_FIELD),
                                about.as_text(),
                            );
                        }
                    }
                    fields
                }
                THE_PER_JOB_NOTE_READING_REQUEST => self.the_note.clone(),
                THE_CURSOR_MOVES_FORWARD_REQUEST => Fields::none(),
                _ => {
                    return WhatCameBack::ItAnswered(
                        answered.saying_it_does_not_answer_that_request(&request.name),
                    )
                }
            };
            WhatCameBack::ItAnswered(answered.carrying(fields))
        }
    }

    /// **A machine that ran long with the app closed: every event past the
    /// 512th is still applied** — `State, identity and recovery` V1 step 3 and
    /// E3, read against the machine's own rule that *a reader that got fewer
    /// than there are asks again from past the last*.
    ///
    /// **As a person meets it**: two jobs on one server, and the app closed
    /// while a thousand events are written. The first job is cleared from the
    /// queue at event 700; the second finishes at event 900. **Opening the
    /// app must take the first off the queue and tell the second's ending.**
    /// Until 2026-09-29 the cursor was written as the journal's own next number
    /// after the first read, so the read stopped at event 511 and neither was
    /// ever applied — nor would they be at any later connection, the cursor
    /// being past them.
    #[test]
    fn every_event_past_the_first_read_is_applied_on_a_machine_that_ran_long() {
        use smoozy_server_protocol::requests::THE_RECORDED_READING_WRITTEN_EVENT;
        let (record, server) = a_record_with_a_server();
        let mut cleared = a_provisional_record(&server, "Hall");
        cleared.standing = Standing::Confirmed;
        write(&record, &cleared);
        let mut finished = a_provisional_record(&server, "Kitchen");
        finished.standing = Standing::Confirmed;
        write(&record, &finished);

        // **A thousand events, none of which moves either job** but for the two.
        let mut kinds: Vec<(&'static str, Option<Identity>)> =
            vec![(THE_RECORDED_READING_WRITTEN_EVENT, None); 1000];
        kinds[700] = (THE_JOB_RECORD_REMOVED_EVENT, Some(cleared.job.clone()));
        kinds[900] = (
            THE_RECORDED_OUTCOME_WRITTEN_EVENT,
            Some(finished.job.clone()),
        );
        let mut machine = AJournalInHand {
            kinds,
            the_note: a_note_that_started_and_finished("Kitchen"),
            read_from: Vec::new(),
        };
        let holds = WhatThisRunHolds::default();
        let ended = a_read_forward_pass(&server, &record, &mut machine, &holds);

        let held = |job: &Identity| JobRecords::over(&record.0).read(job).unwrap();
        assert!(
            held(&cleared.job).is_none(),
            "the job cleared at event 700 is still on the queue; the reads began at {:?}",
            machine.read_from
        );
        let account = the_account_of(&record, &server);
        assert!(
            account
                .as_ref()
                .is_some_and(|account| account.entries.len() == 1),
            "the ending at event 900 is not told: {account:?}; the reads began at {:?}",
            machine.read_from
        );
        assert!(
            held(&finished.job).is_some_and(|held| held.the_ending_learned().is_some()),
            "the job that finished at event 900 was never filled in"
        );
        assert_eq!(ended, HowThePassEnded::ItReadToTheEnd { applied: 1000 });
        assert_eq!(
            the_cursor_of(&record, &server),
            Some(1000),
            "the cursor is not past the newest"
        );
        assert_eq!(
            machine.read_from,
            vec![0, 512],
            "**each read asks from one past the last event carried, and the pass stops at \
             the newest** rather than asking once more for nothing"
        );
    }

    // ── `Queue & History` A1 and A5 ─────────────────────────────────────────

    fn a_queue(jobs: &[(&Identity, &str)]) -> Fields {
        let mut fields = Fields::none().and(HOW_MANY_JOBS_FIELD, jobs.len().to_string());
        for (at, (job, state)) in jobs.iter().enumerate() {
            fields = fields
                .and(
                    &the_field_of_item(THE_JOB_FIELD, at, THE_JOB_FIELD),
                    job.as_text(),
                )
                .and(
                    &the_field_of_item(THE_JOB_FIELD, at, THE_QUEUE_STATE_FIELD),
                    *state,
                );
            if *state == IT_IS_RENDERING {
                fields = fields.and(
                    &the_field_of_item(THE_JOB_FIELD, at, THE_RENDER_OPERATION_FIELD),
                    Identity::issue(IdentityKind::Operation).unwrap().as_text(),
                );
            }
            // **An ended job carries its recorded outcome's identity**, as the
            // machine's reading does; without it the reader skips the job.
            if *state == IT_HAS_ENDED {
                fields = fields.and(
                    &the_field_of_item(THE_JOB_FIELD, at, THE_OUTCOME_IT_ENDED_WITH_FIELD),
                    Identity::issue(IdentityKind::Operation).unwrap().as_text(),
                );
            }
        }
        fields
    }

    /// **Steps 5 and 6: the reading is held for the period, and the offer is
    /// every job the local record does not name.**
    #[test]
    fn the_reading_is_held_and_the_untracked_work_is_offered() {
        let (record, server) = a_record_with_a_server();
        let mut ours = a_provisional_record(&server, "Kitchen");
        ours.standing = Standing::Confirmed;
        write(&record, &ours);
        let theirs = Identity::issue(IdentityKind::Job).unwrap();
        let mut machine = AMachineInHand::default()
            .answers(THE_READ_FORWARD_REQUEST, Some(events(0, &[], None)))
            .answers(
                THE_QUEUE_READING_REQUEST,
                Some(a_queue(&[
                    (&ours.job, IT_IS_RENDERING),
                    (&theirs, IT_IS_WAITING),
                ])),
            );
        let holds = WhatThisRunHolds::default();
        let connection = holds.a_connection_begins(&server.as_text());
        let did = the_step_seven(
            &server,
            connection,
            &record,
            &mut machine,
            &holds,
            &nothing_is_live,
        );
        assert!(did.the_queue_was_read);
        assert_eq!(did.offered, 1);
        let (_, period) = holds.every_server().into_iter().next().unwrap();
        assert_eq!(period.the_offer, vec![theirs]);
        assert_eq!(period.reading.unwrap().value().jobs.len(), 2);

        // **`Queue & History` C7 and C6.**
        holds.the_connection_ended(&server.as_text());
        assert!(holds.every_server()[0].1.the_break_is_declared);
        assert!(!holds.is_current(&server.as_text(), connection));
        holds.the_period_ended(&server.as_text());
        assert!(holds.every_server().is_empty());
    }

    /// **A connection that follows a break is current from the moment it is
    /// lent**, before its own step 7's reading lands — so the arrangements
    /// handed to it at that moment (`Download` R12) are driven rather than
    /// ending at once. **The break's mark stays the reading's** (`Queue &
    /// History` C7) until step 7 takes a fresh one. Step 10c's eighth-pass fix
    /// piece 3.
    #[test]
    fn a_connection_after_a_break_is_current_before_its_reading_lands() {
        let holds = WhatThisRunHolds::default();
        let first = holds.a_connection_begins("server-1");
        holds.the_connection_ended("server-1");
        let second = holds.a_connection_begins("server-1");
        assert!(
            !holds.is_current("server-1", first),
            "an ended connection is never current"
        );
        assert!(
            holds.is_current("server-1", second),
            "the return after a break was not current until a reading landed"
        );
        assert_eq!(
            holds.the_connection_now("server-1"),
            None,
            "the break's mark stands"
        );
    }

    /// **`Download` R12's arrangements are driven once per connection** —
    /// `Servers` Q19a's re-take hands them a connection they may already be
    /// driven on, and a second driving is never started beside the first.
    #[test]
    fn the_arrangements_are_driven_once_per_connection() {
        let holds = WhatThisRunHolds::default();
        let connection = holds.a_connection_begins("server-1");
        assert!(holds.the_arrangements_begin("server-1", connection));
        assert!(!holds.the_arrangements_begin("server-1", connection));
        holds.the_arrangements_ended("server-1", connection);
        assert!(holds.the_arrangements_begin("server-1", connection));
        let next = holds.a_connection_begins("server-1");
        assert!(holds.the_arrangements_begin("server-1", next));
    }

    /// **A second period in one run is a new connection, never the last
    /// period's connection again** — `Download` R12's *"resumes on any
    /// connection to its server — replacement or period-opening"*, over the
    /// guard that drives the arrangements once per connection. Step 10c's
    /// transfer wave piece 3 (the tenth-pass map's piece 1, item (ii)).
    ///
    /// **The fault, on `WhatThisRunHolds` alone, in the order a person makes
    /// it**: *Connect*, the downloads begin; *Disconnect*, and the period ends
    /// while the last period's driving is still in its pause; *Connect* again.
    /// Until this piece the period's end took the count with it, so the new
    /// connection was number 1 again, the guard still held 1, and the new
    /// connection's downloads were refused. The old driving then ended and took
    /// the guard with it, and nothing drove a download for the rest of the
    /// period, with nothing said.
    #[test]
    fn a_connect_after_a_disconnect_drives_its_downloads_whatever_the_last_period_is_doing() {
        let holds = WhatThisRunHolds::default();
        let the_first = holds.a_connection_begins("server-1");
        assert!(holds.the_arrangements_begin("server-1", the_first));
        // **The user's *Disconnect*: the break, then the period's end**, as
        // `crate::servers::a_period` calls them. The first period's driving is
        // still asleep in `Download` W2's pause and has not ended.
        holds.the_connection_ended("server-1");
        holds.the_period_ended("server-1");
        // **The user's *Connect*: a new period, and its first connection.**
        let the_second = holds.a_connection_begins("server-1");
        assert_ne!(
            the_second, the_first,
            "**a second period's connection took the first period's number**, so anything \
             keyed on the number mistakes one for the other"
        );
        assert!(
            holds.the_arrangements_begin("server-1", the_second),
            "**the new connection's downloads were refused** because the last period's \
             driving still held the guard"
        );
        // **The old driving wakes, finds its connection gone, and ends** — and
        // takes nothing of the new connection's with it.
        assert!(!holds.is_current("server-1", the_first));
        holds.the_arrangements_ended("server-1", the_first);
        assert!(
            !holds.the_arrangements_begin("server-1", the_second),
            "the new connection's driving still stands once the old one has ended"
        );
    }

    /// **A reading a last period's pass took lands nowhere in the next
    /// period** — `Queue & History` C6 drops a period's reading when the period
    /// ends, and a pass of that period finishing late must not put one back on
    /// the next period's first connection. Step 10c's transfer wave piece 3.
    #[test]
    fn a_reading_the_last_period_took_does_not_land_on_the_next_period() {
        let holds = WhatThisRunHolds::default();
        let the_first = holds.a_connection_begins("server-1");
        holds.the_connection_ended("server-1");
        holds.the_period_ended("server-1");
        let the_second = holds.a_connection_begins("server-1");
        // **A pass of the first period, finishing after the second began.**
        holds.a_reading_was_taken(
            "server-1",
            the_first,
            Reading::taken(
                TheMachinesWorkingState {
                    jobs: Vec::new(),
                    the_admission_point_stands_at: None,
                    notes: Vec::new(),
                    the_history_copys_count: 0,
                    the_history_copys_entries: Vec::new(),
                },
                SystemTime::UNIX_EPOCH,
                Clock::ThisComputer,
            ),
            Some(Vec::new()),
        );
        assert!(
            holds.every_server().is_empty(),
            "**the last period's reading landed on the next period's connection**"
        );
        assert_eq!(holds.the_connection_now("server-1"), Some(the_second));
        // **And a reading for a server with no period at all makes none.**
        holds.the_period_ended("server-1");
        holds.a_reading_was_taken(
            "server-1",
            the_second,
            Reading::taken(
                TheMachinesWorkingState {
                    jobs: Vec::new(),
                    the_admission_point_stands_at: None,
                    notes: Vec::new(),
                    the_history_copys_count: 0,
                    the_history_copys_entries: Vec::new(),
                },
                SystemTime::UNIX_EPOCH,
                Clock::ThisComputer,
            ),
            None,
        );
        assert_eq!(
            holds.the_connection_now("server-1"),
            None,
            "no period stands"
        );
    }

    /// **`Download` N6's whole, held for the run once a pass has read it** —
    /// and nothing where no pass in this run has. Step 10c's transfer wave
    /// piece 4.
    #[test]
    fn a_jobs_whole_is_held_for_the_run_and_outlives_the_period() {
        let holds = WhatThisRunHolds::default();
        assert_eq!(holds.the_whole_of("job-1"), None);
        holds.a_connection_begins("server-1");
        holds.a_jobs_whole_was_read(
            "job-1",
            TheJobsWhole {
                files: 240,
                it_may_grow: false,
            },
        );
        holds.the_connection_ended("server-1");
        holds.the_period_ended("server-1");
        assert_eq!(
            holds.the_whole_of("job-1"),
            Some(TheJobsWhole {
                files: 240,
                it_may_grow: false
            }),
            "the whole is the job's, not the period's"
        );
        // **A later pass's reading replaces it** — a new attempt reads its own.
        holds.a_jobs_whole_was_read(
            "job-1",
            TheJobsWhole {
                files: 1,
                it_may_grow: true,
            },
        );
        assert_eq!(
            holds.the_whole_of("job-1").map(|whole| whole.files),
            Some(1)
        );
    }

    /// **`Servers` Q19a, on the machine's work: the connection a user first
    /// makes to a bare machine reads nothing — and once the placement has
    /// committed and the re-take has read the confirmed branch, step 7 runs on
    /// that same connection**, so the Queue holds a reading, the job submitted
    /// in the same session is confirmed, and its ending reaches history —
    /// with no reconnect. The count is not moved: the connection is the one
    /// already lent. Step 10c's eighth-pass fix piece 3.
    #[test]
    fn step_seven_runs_on_the_connection_already_lent_once_the_placement_is_read() {
        let (record, server) = a_record_with_a_server();
        let holds = WhatThisRunHolds::default();
        // **The first connection, to a bare machine**: lent, and step 7 not
        // run on the absent branch — so the Queue holds nothing.
        let connection = holds.a_connection_begins(&server.as_text());
        assert!(
            holds.every_server().is_empty(),
            "the Queue reads empty on a bare machine"
        );

        // **Placed, and a job submitted in the same session** — its record is
        // provisional until the machine speaks for it.
        let submitted = a_provisional_record(&server, "Kitchen");
        write(&record, &submitted);

        // **At the commit**: the connection now is the one already lent.
        let now = holds
            .the_connection_now(&server.as_text())
            .expect("a connection is lent");
        assert_eq!(now, connection);
        let mut machine = AMachineInHand::default()
            .answers(
                THE_DOES_THE_MACHINE_HOLD_THIS_JOB_REQUEST,
                on_the_machine(IT_IS_PRESENT),
            )
            .answers(THE_READ_FORWARD_REQUEST, Some(events(0, &[], Some(0))))
            .answers(
                THE_PER_JOB_NOTE_READING_REQUEST,
                Some(a_note_that_started_and_finished("Kitchen")),
            )
            .answers(
                THE_QUEUE_READING_REQUEST,
                Some(a_queue(&[(&submitted.job, IT_HAS_ENDED)])),
            );
        let did = the_step_seven(
            &server,
            now,
            &record,
            &mut machine,
            &holds,
            &nothing_is_live,
        );
        assert!(did.the_queue_was_read, "the Queue was not read");
        assert_eq!(
            did.confirmed, 1,
            "the job submitted in the session was not confirmed"
        );
        assert_eq!(did.filled_in, 1, "its ending did not reach history");
        assert!(holds.is_current(&server.as_text(), connection));
        assert_eq!(holds.every_server().len(), 1, "the Queue holds no reading");
        let held = JobRecords::over(&record.0)
            .read(&submitted.job)
            .unwrap()
            .unwrap();
        assert_eq!(held.standing, Standing::Confirmed);
    }

    /// **Where a job stands, as the reading held and its record now are** —
    /// whether the Queue draws it ([`crate::surface::the_queue::it_has_left_the_queue`])
    /// and whether History lists it (an ending learned: `Queue & History` C9
    /// as narrowed 2026-10-04).
    fn the_lists_it_is_in(
        record: &ARecordInHand,
        holds: &WhatThisRunHolds,
        server: &Identity,
        job: &Identity,
    ) -> (bool, bool) {
        let the_record = JobRecords::over(&record.0).read(job).unwrap().unwrap();
        let state = holds
            .every_server()
            .into_iter()
            .find(|(held, _)| *held == server.as_text())
            .and_then(|(_, period)| period.reading)
            .and_then(|reading| {
                reading
                    .value()
                    .jobs
                    .iter()
                    .find(|one| &one.job == job)
                    .map(|one| one.state.clone())
            });
        let in_the_queue = state.is_some_and(|state| {
            !crate::surface::the_queue::it_has_left_the_queue(&state, Some(&the_record))
        });
        (in_the_queue, the_record.the_ending_learned().is_some())
    }

    /// **`Telegram` V1d, what the desktop learns from `Queue & History` A1's
    /// reading**: each tracked job's current choice is written to its record,
    /// so the Queue's control shows a change made from the phone; the phone's
    /// choice is taken once as Settings' default, minutes included; a later
    /// click at the desk stands when the same choice is read again, and an
    /// older choice another server still holds is never taken over a newer one.
    #[test]
    fn a_reading_writes_each_jobs_choice_and_takes_the_phones_choice_once() {
        use crate::record::settings::{the_reporting_default_was_set, the_telegram_settings};
        let (record, server) = a_record_with_a_server();
        let waiting = a_record_in_history(&server, "Kitchen");
        write(&record, &waiting);
        let untracked = Identity::issue(IdentityKind::Job).unwrap();
        let holds = WhatThisRunHolds::default();
        let connection = holds.a_connection_begins(&server.as_text());
        let operation = Identity::issue(IdentityKind::Operation).unwrap().as_text();
        let a_reading = |choice: &str, operation: &str, moment: &str| {
            a_queue(&[(&waiting.job, IT_IS_WAITING), (&untracked, IT_IS_WAITING)])
                .and(&the_field_of_item(THE_JOB_FIELD, 0, THE_REPORTING_CHOICE_FIELD), "\"OnlyWhatNeedsAnAnswer\"")
                .and(&the_field_of_item(THE_JOB_FIELD, 1, THE_REPORTING_CHOICE_FIELD), "\"Nothing\"")
                .and(THE_PHONES_REPORTING_CHOICE_FIELD, choice)
                .and(THE_PHONES_CHOICE_OPERATION_FIELD, operation)
                .and(THE_PHONES_CHOICE_MOMENT_FIELD, moment)
        };
        let read_once = |fields: Fields| {
            let mut machine = AMachineInHand::default().answers(THE_QUEUE_READING_REQUEST, Some(fields));
            the_queue_is_read(&server, connection, &record, &mut machine, &holds, &nothing_is_live, false)
        };

        // The first reading: the job's choice is the record's, and the phone's
        // choice of a picture every 10 minutes becomes the default.
        let read = read_once(a_reading("{\"APictureEveryFewMinutes\":{\"minutes\":10}}", &operation, "1790000000"));
        assert!(matches!(read.gave, Some(WhatTheRequestGave::ItAnswered(_))));
        assert!(read.the_default_was_taken);
        assert_eq!(
            JobRecords::over(&record.0).the_reporting_choice_of(&waiting.job).unwrap().as_deref(),
            Some("\"OnlyWhatNeedsAnAnswer\"")
        );
        let held = the_telegram_settings(&record.0).unwrap();
        assert_eq!(held.the_reporting_default.as_deref(), Some("a-picture-every-few-minutes"));
        assert_eq!(held.the_default_minutes, Some(10));

        // A click at the desk, then the same phone's choice read again: the
        // click stands.
        the_reporting_default_was_set(&record.0, "everything").unwrap();
        let read = read_once(a_reading("{\"APictureEveryFewMinutes\":{\"minutes\":10}}", &operation, "1790000000"));
        assert!(!read.the_default_was_taken);
        assert_eq!(the_telegram_settings(&record.0).unwrap().the_reporting_default.as_deref(), Some("everything"));

        // An older choice another server still holds is not taken.
        let older = Identity::issue(IdentityKind::Operation).unwrap().as_text();
        assert!(!read_once(a_reading("\"Nothing\"", &older, "1789999000")).the_default_was_taken);
        assert_eq!(the_telegram_settings(&record.0).unwrap().the_reporting_default.as_deref(), Some("everything"));

        // A new, later one is: Off.
        let newer = Identity::issue(IdentityKind::Operation).unwrap().as_text();
        assert!(read_once(a_reading("\"Nothing\"", &newer, "1790000060")).the_default_was_taken);
        assert_eq!(the_telegram_settings(&record.0).unwrap().the_reporting_default.as_deref(), Some("nothing"));

        // **A reading from a program before these fields** changes nothing.
        let mut machine = AMachineInHand::default()
            .answers(THE_QUEUE_READING_REQUEST, Some(a_queue(&[(&waiting.job, IT_IS_WAITING)])));
        let read = the_queue_is_read(&server, connection, &record, &mut machine, &holds, &nothing_is_live, false);
        assert!(!read.the_default_was_taken);
        assert_eq!(
            JobRecords::over(&record.0).the_reporting_choice_of(&waiting.job).unwrap().as_deref(),
            Some("\"OnlyWhatNeedsAnAnswer\"")
        );
    }

    /// **The three phone fields come together or not at all**, and a value
    /// that is not one of the five is not carried.
    #[test]
    fn the_phones_choice_is_read_only_whole_and_only_as_one_of_the_five() {
        let request = the_queue_reading_request().unwrap();
        let read = |fields: Fields| {
            the_queue_reading_and_its_reporting_in(WhatCameBack::ItAnswered(
                Answered::to(&request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, "test").carrying(fields),
            ))
            .expect("the answer reads")
            .1
        };
        let empty = Fields::none().and(HOW_MANY_JOBS_FIELD, "0");
        assert_eq!(read(empty.clone()), WhatTheReadingSaysOfReporting::default());
        let whole = empty
            .clone()
            .and(THE_PHONES_REPORTING_CHOICE_FIELD, "\"OnlyWhatNeedsAnAnswer\"")
            .and(THE_PHONES_CHOICE_OPERATION_FIELD, "op")
            .and(THE_PHONES_CHOICE_MOMENT_FIELD, "1790000000");
        assert_eq!(
            read(whole).the_phones_choice,
            Some(ThePhonesChoiceRead {
                choice: smoozy_render_settings::reporting::ReportingChoice::OnlyWhatNeedsAnAnswer,
                operation: "op".into(),
                moment: 1_790_000_000,
            })
        );
        let no_moment = empty
            .clone()
            .and(THE_PHONES_REPORTING_CHOICE_FIELD, "\"Nothing\"")
            .and(THE_PHONES_CHOICE_OPERATION_FIELD, "op");
        assert_eq!(read(no_moment).the_phones_choice, None);
        let not_one_of_the_five = empty
            .and(THE_PHONES_REPORTING_CHOICE_FIELD, "\"Loudly\"")
            .and(THE_PHONES_CHOICE_OPERATION_FIELD, "op")
            .and(THE_PHONES_CHOICE_MOMENT_FIELD, "1790000000");
        assert_eq!(read(not_one_of_the_five).the_phones_choice, None);
    }

    /// **Step 10c's worldwide setup pass on Vast, new fault 5: a finished job
    /// was in neither Queue nor History for at least 12 seconds.** A P10d pass
    /// reads forward and then takes the queue reading; a job that ended
    /// between the two was read as *ended*, so the Queue let it go, while its
    /// record had no ending, so History did not list it — until the next
    /// pass's read forward.
    ///
    /// **Now the ending is learned before the reading that names it is held**:
    /// the job goes from the Queue straight to History, with nothing drawn in
    /// between. **A reading that names no unlearned ending asks nothing
    /// more**, and **a read forward that cannot complete leaves the last
    /// reading standing**, so the job stays in the Queue as last read.
    ///
    /// **What it bites on**: the reading held as it came, which is how it was
    /// held before — *(false, false)*, the job in neither list.
    #[test]
    fn a_job_read_as_ended_is_in_history_before_the_reading_that_lets_it_go_is_held() {
        let (record, server) = a_record_with_a_server();
        let rendering = a_record_in_history(&server, "Kitchen");
        write(&record, &rendering);
        let holds = WhatThisRunHolds::default();
        let connection = holds.a_connection_begins(&server.as_text());

        // **A pass while it renders**: in the Queue alone.
        let mut machine = AMachineInHand::default().answers(
            THE_QUEUE_READING_REQUEST,
            Some(a_queue(&[(&rendering.job, IT_IS_RENDERING)])),
        );
        let read = the_queue_is_read(
            &server,
            connection,
            &record,
            &mut machine,
            &holds,
            &nothing_is_live,
            false,
        );
        assert!(matches!(read.gave, Some(WhatTheRequestGave::ItAnswered(_))));
        assert_eq!(
            machine.how_many_times(THE_READ_FORWARD_REQUEST),
            0,
            "a rendering job asks nothing more"
        );
        assert_eq!(
            the_lists_it_is_in(&record, &holds, &server, &rendering.job),
            (true, false)
        );

        // **The next pass's queue reading lands after the ending and before
        // that pass's read forward has seen it.**
        let mut machine = AMachineInHand::default()
            .answers(
                THE_QUEUE_READING_REQUEST,
                Some(a_queue(&[(&rendering.job, IT_HAS_ENDED)])),
            )
            .answers(
                THE_READ_FORWARD_REQUEST,
                Some(events(
                    0,
                    &[(THE_RECORDED_OUTCOME_WRITTEN_EVENT, Some(&rendering.job))],
                    Some(0),
                )),
            )
            .answers(THE_READ_FORWARD_REQUEST, Some(events(1, &[], Some(0))))
            .answers(
                THE_PER_JOB_NOTE_READING_REQUEST,
                Some(a_note_that_started_and_finished("Kitchen")),
            )
            .answers(THE_CURSOR_MOVES_FORWARD_REQUEST, Some(Fields::none()));
        let read = the_queue_is_read(
            &server,
            connection,
            &record,
            &mut machine,
            &holds,
            &nothing_is_live,
            false,
        );
        assert!(
            matches!(read.gave, Some(WhatTheRequestGave::ItAnswered(_))),
            "the reading is held"
        );
        assert_eq!(
            the_lists_it_is_in(&record, &holds, &server, &rendering.job),
            (false, true),
            "**`Queue & History` C11: when the job ends it leaves the queue and the history entry \
             takes the ending** — it is in History the moment the Queue lets it go"
        );
        assert_eq!(
            read.applied_first, 1,
            "the ending's event was applied before the reading was held"
        );
        let asked: Vec<&str> = machine
            .asked
            .iter()
            .map(|(name, _)| name.as_str())
            .collect();
        assert_eq!(asked.first(), Some(&THE_QUEUE_READING_REQUEST), "{asked:?}");
        assert_eq!(
            the_cursor_of(&record, &server),
            Some(1),
            "the read forward's own cursor moved"
        );

        // **Once learned, a reading that still says *ended* asks nothing more.**
        let mut machine = AMachineInHand::default().answers(
            THE_QUEUE_READING_REQUEST,
            Some(a_queue(&[(&rendering.job, IT_HAS_ENDED)])),
        );
        the_queue_is_read(
            &server,
            connection,
            &record,
            &mut machine,
            &holds,
            &nothing_is_live,
            false,
        );
        assert_eq!(machine.how_many_times(THE_READ_FORWARD_REQUEST), 0);
        assert_eq!(
            the_lists_it_is_in(&record, &holds, &server, &rendering.job),
            (false, true)
        );

        // **A read forward that cannot complete leaves the last reading
        // standing**: a second job, rendering, read as ended over a line that
        // then answers nothing, stays in the Queue as last read.
        let second = a_record_in_history(&server, "Hall");
        write(&record, &second);
        let mut machine = AMachineInHand::default().answers(
            THE_QUEUE_READING_REQUEST,
            Some(a_queue(&[(&second.job, IT_IS_RENDERING)])),
        );
        the_queue_is_read(
            &server,
            connection,
            &record,
            &mut machine,
            &holds,
            &nothing_is_live,
            false,
        );
        let mut machine = AMachineInHand::default()
            .answers(
                THE_QUEUE_READING_REQUEST,
                Some(a_queue(&[(&second.job, IT_HAS_ENDED)])),
            )
            .answers(THE_READ_FORWARD_REQUEST, None);
        let read = the_queue_is_read(
            &server,
            connection,
            &record,
            &mut machine,
            &holds,
            &nothing_is_live,
            false,
        );
        assert!(read.gave.is_none(), "the reading is not held");
        assert_eq!(machine.how_many_times(THE_READ_FORWARD_REQUEST), 1);
        assert_eq!(
            the_lists_it_is_in(&record, &holds, &server, &second.job),
            (true, false),
            "the job stays in the Queue as last read, never in neither list"
        );
    }

    /// **P10d's on-screen condition, as the runner ruled it on 2026-09-28**:
    /// the queue reading repeats while Queue, the server list, or that
    /// server's own page is on screen — and not for another server's page.
    /// Step 10c's ninth pass read *Idle* on the list while Queue was away,
    /// because only Queue counted.
    #[test]
    fn the_server_list_and_a_servers_page_keep_the_reading_repeating() {
        let holds = WhatThisRunHolds::default();
        let (this, that) = ("this-server", "that-server");
        assert!(
            !holds.a_surface_showing_its_work_is_on_screen(this),
            "nothing on screen"
        );

        holds.the_server_list_is(true);
        assert!(
            holds.a_surface_showing_its_work_is_on_screen(this),
            "the list shows every server"
        );
        assert!(holds.a_surface_showing_its_work_is_on_screen(that));
        holds.the_server_list_is(false);
        assert!(!holds.a_surface_showing_its_work_is_on_screen(this));

        holds.a_servers_own_page_is(Some(this));
        assert!(
            holds.a_surface_showing_its_work_is_on_screen(this),
            "its own page"
        );
        assert!(
            !holds.a_surface_showing_its_work_is_on_screen(that),
            "not another's"
        );
        holds.a_servers_own_page_is(None);
        assert!(!holds.a_surface_showing_its_work_is_on_screen(this));

        holds.the_queue_is_shown(None);
        assert!(
            holds.a_surface_showing_its_work_is_on_screen(that),
            "Queue shows every server"
        );
        holds.the_queue_is_put_away();
        assert!(!holds.a_surface_showing_its_work_is_on_screen(that));
    }

    /// **`Queue & History` A1's *its unreleased hold*, read: a held job's item
    /// says why it is held and the job it waits behind** (`Queue & History`
    /// H1, `Queue & History` H6), and **a reading from a program older than
    /// step 10c's eleventh-pass fix piece 8b, which says neither, reads as
    /// saying nothing** — never a reason made up here. A job that is not held
    /// carries no hold whatever its item says.
    #[test]
    fn a_held_jobs_item_says_why_it_is_held_and_what_it_waits_behind() {
        let (first, held, older, waiting) = (
            Identity::issue(IdentityKind::Job).unwrap(),
            Identity::issue(IdentityKind::Job).unwrap(),
            Identity::issue(IdentityKind::Job).unwrap(),
            Identity::issue(IdentityKind::Job).unwrap(),
        );
        let under = |at: usize| {
            (
                the_field_of_item(THE_JOB_FIELD, at, THE_HOLD_IT_IS_HELD_UNDER_FIELD),
                Identity::issue(IdentityKind::Operation).unwrap().as_text(),
            )
        };
        let (held_under, older_under) = (under(0), under(1));
        let fields = a_queue(&[
            (&held, IT_IS_HELD),
            (&older, IT_IS_HELD),
            (&waiting, IT_IS_WAITING),
        ])
        .and(&held_under.0, held_under.1)
        .and(&older_under.0, older_under.1)
        .and(
            &the_field_of_item(THE_JOB_FIELD, 0, THE_HOLD_REASON_FIELD),
            "the first-frame check",
        )
        .and(
            &the_field_of_item(THE_JOB_FIELD, 0, THE_HELD_BEHIND_FIELD),
            first.as_text(),
        )
        .and(
            &the_field_of_item(THE_JOB_FIELD, 2, THE_HOLD_REASON_FIELD),
            "the first-frame check",
        );
        let mut machine =
            AMachineInHand::default().answers(THE_QUEUE_READING_REQUEST, Some(fields));
        let read = the_queue_reading_from(&mut machine).expect("a reading");
        let the_hold_of = |job: &Identity| {
            read.jobs
                .iter()
                .find(|item| &item.job == job)
                .expect("its item")
                .the_hold
                .clone()
        };
        assert_eq!(
            the_hold_of(&held),
            Some(smoozy_queue_history::readings::WhatTheHoldSays {
                reason: "the first-frame check".to_string(),
                held_behind: Some(first),
            })
        );
        assert_eq!(
            the_hold_of(&older),
            None,
            "an older program's reading says nothing of the hold"
        );
        assert_eq!(
            the_hold_of(&waiting),
            None,
            "a job that is not held carries no hold"
        );
    }

    /// **`Queue & History` A1: a store the machine could not read is not an
    /// empty line.**
    #[test]
    fn a_reading_with_no_count_is_no_reading() {
        let mut machine =
            AMachineInHand::default().answers(THE_QUEUE_READING_REQUEST, Some(Fields::none()));
        assert_eq!(
            the_queue_reading_from(&mut machine).err(),
            Some(WhyThereIsNoReading::TheMachineSaidItCouldNot)
        );
    }

    /// **A quick waiter**, so a test that meets a silence ends in milliseconds.
    const QUICK: crate::servers::stop_and_clear::TheWait =
        crate::servers::stop_and_clear::TheWait {
            deadline: std::time::Duration::from_millis(200),
            pause: std::time::Duration::from_millis(1),
        };

    fn a_release(word: &str) -> Option<Fields> {
        Some(Fields::none().and(
            smoozy_server_protocol::requests::THE_RELEASE_OUTCOME_FIELD,
            word,
        ))
    }

    /// **`Rented-server runtime` U12a's result**: *stopped*, and the job gone
    /// in the cancel's own field and word.
    fn a_no_that_removed_its_job() -> Option<Fields> {
        a_release("stopped").map(|fields| {
            fields.and(
                smoozy_server_protocol::requests::THE_CANCELS_OUTCOME_FIELD,
                smoozy_server_protocol::requests::IT_WAS_REMOVED,
            )
        })
    }

    /// A cancel's *removed*, for a stand-in that must never be asked it.
    fn removed_again() -> Option<Fields> {
        Some(
            Fields::none()
                .and(
                    smoozy_server_protocol::requests::WHAT_BECAME_OF_IT_FIELD,
                    smoozy_server_protocol::requests::IT_HAS_A_RECORDED_OUTCOME,
                )
                .and(
                    smoozy_server_protocol::requests::THE_CANCELS_OUTCOME_FIELD,
                    smoozy_server_protocol::requests::IT_WAS_REMOVED,
                ),
        )
    }

    /// **`Rented-server runtime` U9 and `Rented-server runtime` U10, and `Queue &
    /// History` H2: the
    /// desk's answer names the hold, carries the answer and says it came from
    /// the desk** — and is asked again, while the machine has no outcome to
    /// give, until the release outcome comes back (`Server protocol and
    /// transfers` C8). Step 10c's eleventh-pass fix piece 8.
    #[test]
    fn the_desks_answer_names_the_hold_and_reads_the_release_that_stood() {
        use smoozy_runtime::holds::{ReleaseOutcome, WhatTheAnswerSays};
        use smoozy_server_protocol::requests::{
            THE_ANSWER_FIELD, THE_ANSWER_IS_NO, THE_ANSWER_IS_YES, THE_ANSWER_TO_A_HOLD_REQUEST,
            THE_HOLD_FIELD, THE_ROUTE_FIELD, THE_ROUTE_IS_THE_DESK,
        };
        let hold = Identity::issue(IdentityKind::Operation).unwrap();
        let mut machine = AMachineInHand::default()
            .answers(THE_ANSWER_TO_A_HOLD_REQUEST, Some(Fields::none()))
            .answers(THE_ANSWER_TO_A_HOLD_REQUEST, a_release("released"));
        assert_eq!(
            the_answer_to_a_hold_is_sent(&mut machine, &hold, WhatTheAnswerSays::Yes, QUICK),
            WhatTheAnswerCameTo::TheHoldIsAnswered(ReleaseOutcome::Released)
        );
        assert_eq!(
            machine.how_many_times(THE_ANSWER_TO_A_HOLD_REQUEST),
            2,
            "asked again while in hand"
        );
        let (_, sent) = &machine.asked[0];
        assert_eq!(sent.text(THE_HOLD_FIELD), Some(hold.as_text().as_str()));
        assert_eq!(sent.text(THE_ANSWER_FIELD), Some(THE_ANSWER_IS_YES));
        assert_eq!(sent.text(THE_ROUTE_FIELD), Some(THE_ROUTE_IS_THE_DESK));

        // **`Rented-server runtime` U14: a second answer is told what stood**, never failed.
        let mut machine =
            AMachineInHand::default().answers(THE_ANSWER_TO_A_HOLD_REQUEST, a_release("released"));
        assert_eq!(
            the_answer_to_a_hold_is_sent(&mut machine, &hold, WhatTheAnswerSays::No, QUICK),
            WhatTheAnswerCameTo::TheHoldIsAnswered(ReleaseOutcome::Released)
        );
        assert_eq!(
            machine.asked[0].1.text(THE_ANSWER_FIELD),
            Some(THE_ANSWER_IS_NO)
        );

        // **`Rented-server runtime` U12a: a *no* that removed its job is told
        // apart from one that did not** — by the answer, not by a version.
        let mut machine = AMachineInHand::default()
            .answers(THE_ANSWER_TO_A_HOLD_REQUEST, a_no_that_removed_its_job());
        assert_eq!(
            the_answer_to_a_hold_is_sent(&mut machine, &hold, WhatTheAnswerSays::No, QUICK),
            WhatTheAnswerCameTo::TheNoRemovedTheJob
        );
        let mut machine =
            AMachineInHand::default().answers(THE_ANSWER_TO_A_HOLD_REQUEST, a_release("stopped"));
        assert_eq!(
            the_answer_to_a_hold_is_sent(&mut machine, &hold, WhatTheAnswerSays::No, QUICK),
            WhatTheAnswerCameTo::TheHoldIsAnswered(ReleaseOutcome::Stopped),
            "an older program's no"
        );

        // **Every way it does not come back answered, each told apart.**
        let mut refused = AMachineInHand::default();
        assert_eq!(
            the_answer_to_a_hold_is_sent(&mut refused, &hold, WhatTheAnswerSays::Yes, QUICK),
            WhatTheAnswerCameTo::NotTaken
        );
        let mut silent = AMachineInHand::default().answers(THE_ANSWER_TO_A_HOLD_REQUEST, None);
        assert_eq!(
            the_answer_to_a_hold_is_sent(&mut silent, &hold, WhatTheAnswerSays::Yes, QUICK),
            WhatTheAnswerCameTo::TheWaiterGaveUp
        );
        struct Nowhere;
        impl AsksTheSupervisor for Nowhere {
            fn ask(&mut self, _: &Request) -> WhatCameBack {
                WhatCameBack::ItWasNeverAsked
            }
        }
        assert_eq!(
            the_answer_to_a_hold_is_sent(&mut Nowhere, &hold, WhatTheAnswerSays::Yes, QUICK),
            WhatTheAnswerCameTo::NotReached
        );
    }

    /// **The desk's answer to a hold ends at its waiter's own deadline though
    /// its last ask meets a silent line** — the dead-line sweep's two-minute-wait repair,
    /// over the application's own asking (`OverThisServersPoint`) and the
    /// point's stand-in. The fifth ask, begun just before the shortened
    /// deadline, meets the silence.
    ///
    /// **The case it bites on.** Until the repair every ask was made whole
    /// and the deadline looked at only after it returned, so the fifth ask
    /// held the answer's wait for as long as the line said nothing.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn the_desks_answer_to_a_hold_ends_at_its_own_deadline_when_an_ask_meets_a_silent_line() {
        use crate::servers::sessions::tests::{
            a_point_lending, additional_connections_to, AMachine,
        };
        use crate::servers::stop_and_clear::tests::{
            the_machine_over, watched_for, ON_TIME, SHORTENED,
        };
        use smoozy_runtime::holds::WhatTheAnswerSays;
        use std::sync::atomic::Ordering;
        let machine = AMachine::with_a_limit_of(10);
        let point = a_point_lending(&machine, additional_connections_to(&machine, false)).await;
        machine
            .the_supervisor_is_silent_from
            .store(5, Ordering::SeqCst);
        let hold = Identity::issue(IdentityKind::Operation).unwrap();
        let mut over = the_machine_over(&point);
        let began = std::time::Instant::now();
        let came = watched_for(std::time::Duration::from_secs(10), move || {
            the_answer_to_a_hold_is_sent(&mut over, &hold, WhatTheAnswerSays::Yes, SHORTENED)
        })
        .await;
        let (came_to, took) = came.expect("the answer's wait went on past its own deadline");
        assert!(
            took < SHORTENED.deadline + ON_TIME,
            "the answer's wait took {took:?}"
        );
        assert_eq!(came_to, WhatTheAnswerCameTo::TheWaiterGaveUp);
        let asked_at = machine.supervisor_asked_at.lock().unwrap().clone();
        assert_eq!(
            asked_at.len(),
            5,
            "the last ask, begun just before the deadline, met no silence"
        );
        assert!(
            asked_at
                .iter()
                .all(|at| at.duration_since(began) < SHORTENED.deadline),
            "an ask was begun after the deadline"
        );
        point.ends().await;
    }

    /// ***Cancel* is one press and one wait** — the dead-line sweep's
    /// two-minute-wait repair. Where an older program's *no* comes back late, the cancel
    /// after it is given what is left of the answer's own deadline, **not a
    /// fresh one**, and where nothing is left it is not sent and says so.
    #[test]
    fn cancel_it_after_a_late_no_has_only_what_is_left_of_the_wait() {
        use crate::servers::stop_and_clear::tests::{ON_TIME, SHORTENED};
        use crate::servers::the_queues_own_acts::{TheQueuesOwnAct, WhereTheActStands};
        use smoozy_server_protocol::requests::{THE_ANSWER_TO_A_HOLD_REQUEST, THE_CANCEL_REQUEST};

        /// **The *no* comes back *stopped* after `late`**, and the cancel is
        /// still in hand however often it is asked.
        struct ALateNo {
            late: std::time::Duration,
            inner: AMachineInHand,
        }
        impl AsksTheSupervisor for ALateNo {
            fn ask(&mut self, request: &Request) -> WhatCameBack {
                if request.name == THE_ANSWER_TO_A_HOLD_REQUEST {
                    std::thread::sleep(self.late);
                }
                self.inner.ask(request)
            }
        }
        let in_hand = Some(Fields::none().and(
            smoozy_server_protocol::requests::WHAT_BECAME_OF_IT_FIELD,
            smoozy_server_protocol::requests::IT_IS_STILL_IN_HAND,
        ));
        let (job, hold) = (
            Identity::issue(IdentityKind::Job).unwrap(),
            Identity::issue(IdentityKind::Operation).unwrap(),
        );
        let mut machine = ALateNo {
            late: std::time::Duration::from_millis(700),
            inner: AMachineInHand::default()
                .answers(THE_ANSWER_TO_A_HOLD_REQUEST, a_release("stopped"))
                .answers(THE_CANCEL_REQUEST, in_hand.clone()),
        };
        let began = std::time::Instant::now();
        let left = the_held_jobs_answer_is_sent(
            &mut machine,
            &job,
            &hold,
            TheHeldJobsAnswer::CancelIt,
            SHORTENED,
        );
        let took = began.elapsed();
        assert_eq!(
            left,
            WhatTheHeldJobsAnswerLeft::TheCancel(WhereTheActStands::TheWaiterGaveUp(
                TheQueuesOwnAct::TheCancel
            ))
        );
        assert!(
            took < SHORTENED.deadline + ON_TIME,
            "one press waited {took:?}"
        );
        assert!(
            machine.inner.how_many_times(THE_CANCEL_REQUEST) >= 1,
            "the cancel was never sent"
        );

        // **Nothing left of the wait**: the cancel is not sent.
        let mut machine = ALateNo {
            late: SHORTENED.deadline,
            inner: AMachineInHand::default()
                .answers(THE_ANSWER_TO_A_HOLD_REQUEST, a_release("stopped"))
                .answers(THE_CANCEL_REQUEST, in_hand),
        };
        assert_eq!(
            the_held_jobs_answer_is_sent(
                &mut machine,
                &job,
                &hold,
                TheHeldJobsAnswer::CancelIt,
                SHORTENED
            ),
            WhatTheHeldJobsAnswerLeft::TheCancel(WhereTheActStands::NotReached(
                TheQueuesOwnAct::TheCancel
            ))
        );
        assert_eq!(machine.inner.how_many_times(THE_CANCEL_REQUEST), 0);
    }

    /// **The held entry's two answers, sent.** *Render the rest* is one *yes*
    /// and nothing else. ***Cancel* is one *no* and then the cancel of the
    /// job** — `Specs/Queue & History`'s *"Saying no leaves nothing behind"*,
    /// which the machine's *no* alone does not do — **and a *no* that meets a
    /// *yes* already written sends no cancel** (`Rented-server runtime` U15).
    #[test]
    fn render_the_rest_is_a_yes_and_cancel_it_is_a_no_then_the_cancel() {
        use smoozy_runtime::holds::ReleaseOutcome;
        use smoozy_server_protocol::requests::{
            IT_HAS_A_RECORDED_OUTCOME, IT_WAS_REMOVED, THE_ANSWER_FIELD, THE_ANSWER_IS_NO,
            THE_ANSWER_TO_A_HOLD_REQUEST, THE_CANCELS_OUTCOME_FIELD, THE_CANCEL_REQUEST,
            WHAT_BECAME_OF_IT_FIELD,
        };
        let (job, hold) = (
            Identity::issue(IdentityKind::Job).unwrap(),
            Identity::issue(IdentityKind::Operation).unwrap(),
        );
        let removed = Some(
            Fields::none()
                .and(WHAT_BECAME_OF_IT_FIELD, IT_HAS_A_RECORDED_OUTCOME)
                .and(THE_CANCELS_OUTCOME_FIELD, IT_WAS_REMOVED),
        );

        let mut machine =
            AMachineInHand::default().answers(THE_ANSWER_TO_A_HOLD_REQUEST, a_release("released"));
        assert_eq!(
            the_held_jobs_answer_is_sent(
                &mut machine,
                &job,
                &hold,
                TheHeldJobsAnswer::RenderAll,
                QUICK
            ),
            WhatTheHeldJobsAnswerLeft::ItIsInTheLine
        );
        assert_eq!(
            machine.asked.len(),
            1,
            "one answer and nothing else: {:?}",
            machine.asked
        );

        let mut machine = AMachineInHand::default()
            .answers(THE_ANSWER_TO_A_HOLD_REQUEST, a_release("stopped"))
            .answers(THE_CANCEL_REQUEST, removed.clone());
        assert_eq!(
            the_held_jobs_answer_is_sent(
                &mut machine,
                &job,
                &hold,
                TheHeldJobsAnswer::CancelIt,
                QUICK
            ),
            WhatTheHeldJobsAnswerLeft::ItWasRemoved
        );
        let names: Vec<&str> = machine
            .asked
            .iter()
            .map(|(name, _)| name.as_str())
            .collect();
        assert_eq!(
            names,
            vec![THE_ANSWER_TO_A_HOLD_REQUEST, THE_CANCEL_REQUEST],
            "the no first, then the cancel"
        );
        assert_eq!(
            machine.asked[0].1.text(THE_ANSWER_FIELD),
            Some(THE_ANSWER_IS_NO)
        );
        assert_eq!(
            machine.asked[1].1.text(THE_JOB_FIELD),
            Some(job.as_text().as_str())
        );

        // **`Rented-server runtime` U15: a *no* that meets a *yes* already written
        // is told, and
        // nothing is cancelled.**
        let mut machine = AMachineInHand::default()
            .answers(THE_ANSWER_TO_A_HOLD_REQUEST, a_release("released"))
            .answers(THE_CANCEL_REQUEST, removed);
        assert_eq!(
            the_held_jobs_answer_is_sent(
                &mut machine,
                &job,
                &hold,
                TheHeldJobsAnswer::CancelIt,
                QUICK
            ),
            WhatTheHeldJobsAnswerLeft::AnotherAnswerStood(ReleaseOutcome::Released)
        );
        assert_eq!(machine.how_many_times(THE_CANCEL_REQUEST), 0);
        // **A *yes* that meets a *no* already written is told so.**
        let mut machine =
            AMachineInHand::default().answers(THE_ANSWER_TO_A_HOLD_REQUEST, a_release("stopped"));
        assert_eq!(
            the_held_jobs_answer_is_sent(
                &mut machine,
                &job,
                &hold,
                TheHeldJobsAnswer::RenderAll,
                QUICK
            ),
            WhatTheHeldJobsAnswerLeft::AnotherAnswerStood(ReleaseOutcome::Stopped)
        );
        // **`Rented-server runtime` U12a: a *no* that removed its job sends
        // nothing after it** — the machine said the job went, in the cancel's
        // own word. Step 10c's eleventh-pass fix piece 8b; above, a program
        // older than that answers *stopped* alone and the cancel still goes.
        let mut machine = AMachineInHand::default()
            .answers(THE_ANSWER_TO_A_HOLD_REQUEST, a_no_that_removed_its_job())
            .answers(THE_CANCEL_REQUEST, removed_again());
        assert_eq!(
            the_held_jobs_answer_is_sent(
                &mut machine,
                &job,
                &hold,
                TheHeldJobsAnswer::CancelIt,
                QUICK
            ),
            WhatTheHeldJobsAnswerLeft::ItWasRemoved
        );
        assert_eq!(
            machine.asked.len(),
            1,
            "one answer and nothing after it: {:?}",
            machine.asked
        );
        // **A *yes* that meets it is told a *no* stood.**
        let mut machine = AMachineInHand::default()
            .answers(THE_ANSWER_TO_A_HOLD_REQUEST, a_no_that_removed_its_job());
        assert_eq!(
            the_held_jobs_answer_is_sent(
                &mut machine,
                &job,
                &hold,
                TheHeldJobsAnswer::RenderAll,
                QUICK
            ),
            WhatTheHeldJobsAnswerLeft::AnotherAnswerStood(ReleaseOutcome::Stopped)
        );
        // **A *no* that reached nothing sends no cancel either.**
        let mut refused = AMachineInHand::default();
        assert_eq!(
            the_held_jobs_answer_is_sent(
                &mut refused,
                &job,
                &hold,
                TheHeldJobsAnswer::CancelIt,
                QUICK
            ),
            WhatTheHeldJobsAnswerLeft::TheAnswer(WhatTheAnswerCameTo::NotTaken)
        );
        assert_eq!(refused.how_many_times(THE_CANCEL_REQUEST), 0);
    }

    /// **The two things this crate must be able to name and could not, named
    /// from this crate** — which is the half of both clauses a compile settles
    /// and a reading cannot.
    ///
    /// **`Servers` W41a's health readings request and its answer's three
    /// fields**, and **`Rented-server runtime` U9's answer to a hold with the
    /// three fields it carries**, whose `Rented-server runtime` U10 puts the
    /// desk's route over SSH
    /// through the command adapter. Both were spelled only in the machine's own
    /// runtime library, whose acting half is built for the machine alone.
    ///
    /// **This is the fourth and fifth time a name has had to change crates to
    /// be sendable** — `Rented-server runtime` P3's two acts, the setup run's
    /// own field, and the two Telegram role requests went before them — and
    /// every one of those three was found by trying to send the request and
    /// watching the build fail. **These two were found by looking**, and this
    /// test is here so that a name sliding back into that acting half fails at
    /// the desk rather than on a hosted Windows run.
    ///
    /// **It asserts reachability and nothing else.** The file's own opening
    /// says what holds the first back. ~~Nothing on this side answers a hold at
    /// all~~ — true until step 10c's eleventh-pass fix piece 8, whose
    /// [`the_answer_to_a_hold_is_sent`] makes the second request and whose own
    /// tests say what it sends.
    #[test]
    fn the_two_moved_requests_and_their_fields_are_reachable_from_this_crate() {
        use smoozy_server_protocol::requests::{
            UnderC6, EVERY_REQUEST_NAMED_HERE, THE_ANSWER_FIELD, THE_ANSWER_IS_NO,
            THE_ANSWER_IS_YES, THE_ANSWER_TO_A_HOLD_REQUEST, THE_CARD_COUNT_FIELD,
            THE_HEALTH_READINGS_REQUEST, THE_HOLD_FIELD, THE_PRODUCED_BY_FIELD,
            THE_RELEASE_MOMENT_FIELD, THE_RELEASE_OUTCOME_FIELD, THE_ROUTE_FIELD,
            THE_ROUTE_IS_THE_DESK, THE_ROUTE_IS_THE_PHONE, THE_RUNNING_TIME_FIELD,
            THE_SETUP_IN_PROGRESS_FIELD,
        };
        let stands = |name: &str| {
            EVERY_REQUEST_NAMED_HERE
                .iter()
                .find(|request| request.name == name)
                .map(|request| (request.under_c6, request.carries))
        };
        // **V5 and V2: the readings change nothing on the machine.**
        let (health, health_fields) = stands(THE_HEALTH_READINGS_REQUEST)
            .expect("the health readings request is named in the crate both ends read");
        assert_eq!(health, UnderC6::APureReading);
        assert!(
            health_fields.is_empty(),
            "the readings ride the answer, not the ask"
        );
        // **`Rented-server runtime` U9 puts the answer under C4**, so it is
        // recorded before it acts, and it carries `Rented-server runtime` U9's
        // three parts.
        let (hold, hold_fields) = stands(THE_ANSWER_TO_A_HOLD_REQUEST)
            .expect("the answer to a hold is named in the crate both ends read");
        assert_eq!(hold, UnderC6::RecordedBeforeItActs);
        assert_eq!(
            hold_fields,
            &[THE_HOLD_FIELD, THE_ANSWER_FIELD, THE_ROUTE_FIELD]
        );
        // **The two closed sets moved whole**, which is what lets this side see
        // that `Rented-server runtime` U10 has no third route and
        // `Rented-server runtime` U9's answer no third word.
        assert_ne!(THE_ROUTE_IS_THE_DESK, THE_ROUTE_IS_THE_PHONE);
        assert_ne!(THE_ANSWER_IS_YES, THE_ANSWER_IS_NO);
        // Every other name this side must read back, each distinct.
        let read_back = [
            THE_CARD_COUNT_FIELD,
            THE_RUNNING_TIME_FIELD,
            THE_SETUP_IN_PROGRESS_FIELD,
            THE_RELEASE_OUTCOME_FIELD,
            THE_PRODUCED_BY_FIELD,
            THE_RELEASE_MOMENT_FIELD,
        ];
        let mut seen = std::collections::HashSet::new();
        for name in read_back {
            assert!(seen.insert(name), "{name} is named twice");
        }
    }

    // ── `Servers` Q1 step 9 ─────────────────────────────────────────────────

    use crate::servers::installation::WhatThePackageAreaHolds;
    use crate::servers::published::{what_the_server_carries, WhatSmoozyOnTheServerIs};
    use smoozy_runtime::package::retry::AcceptedWorkStanding;

    /// **A machine that records every update request sent to it**, holding a
    /// program of `its_release` and answering each request with `ending`.
    struct AMachineThatRecordsUpdates {
        holds: WhatThePackageAreaHolds,
        own: Result<u64, String>,
        ending: TheUpdatesEnding,
        sent: Vec<(Option<Identity>, Vec<(String, AcceptedWorkStanding)>)>,
    }

    impl AMachineThatRecordsUpdates {
        fn running(its_release: Option<u64>, own: u64) -> Self {
            AMachineThatRecordsUpdates {
                holds: WhatThePackageAreaHolds {
                    it_was_read: true,
                    its_release_sequence: its_release,
                    ..Default::default()
                },
                own: Ok(own),
                ending: TheUpdatesEnding::ItLanded,
                sent: Vec::new(),
            }
        }
    }

    impl TheUpdatesOwnEnds for AMachineThatRecordsUpdates {
        fn what_the_package_area_holds(&mut self) -> WhatThePackageAreaHolds {
            self.holds.clone()
        }
        fn this_applications_own_release(&mut self) -> Result<u64, String> {
            self.own.clone()
        }
        fn the_update_is_sent(
            &mut self,
            under: Option<Identity>,
            _held: &WhatThePackageAreaHolds,
            accepted_work: Vec<(String, AcceptedWorkStanding)>,
        ) -> TheUpdatesEnding {
            self.sent.push((under, accepted_work));
            self.ending.clone()
        }
    }

    fn a_line(jobs: &[(Identity, QueueState)]) -> TheMachinesWorkingState {
        TheMachinesWorkingState {
            jobs: jobs
                .iter()
                .map(|(job, state)| AJobAsTheMachineReadsIt {
                    job: job.clone(),
                    state: state.clone(),
                    live: WhatTheEntryHasLive::NoReadings,
                    the_hold: None,
                })
                .collect(),
            the_admission_point_stands_at: None,
            notes: Vec::new(),
            the_history_copys_count: 0,
            the_history_copys_entries: Vec::new(),
        }
    }

    fn a_job() -> Identity {
        Identity::issue(IdentityKind::Job).expect("randomness")
    }

    fn a_server() -> String {
        Identity::issue(IdentityKind::LocalServer)
            .expect("randomness")
            .as_text()
    }

    fn nothing_tracked(_: &Identity) -> Option<bool> {
        None
    }

    /// **V11: an older machine with nothing on it gets one request, for this
    /// application's own release, and the conditions end when it lands.**
    /// Step 10c's transfer wave piece 8; the gap it proves is that until then an
    /// older server with nothing running was never updated at connection.
    #[test]
    fn an_older_idle_machine_gets_one_request() {
        let server = a_server();
        let mut machine = AMachineThatRecordsUpdates::running(Some(6), 7);
        let did = the_step_nine(&server, Some(&a_line(&[])), &nothing_tracked, &mut machine);
        assert_eq!(
            machine.sent.len(),
            1,
            "an older idle machine was not updated"
        );
        assert_eq!(
            machine.sent[0].0, None,
            "a fresh request carried a recorded identity"
        );
        assert_eq!(
            did,
            WhatStepNineDid::Decided {
                decided: WhatStepNineDecides::TheUpdateIsSent,
                ending: Some(TheUpdatesEnding::ItLanded),
            }
        );
        assert_eq!(what_the_server_carries(&server), None);
    }

    /// **V11: an older machine with a waiting job gets nothing, and the update
    /// waits — then V12: the reading that shows the job ended sends it.** The
    /// gap proved first is that until 2026-09-28 a machine with work on it was
    /// not held at all: nothing read its work, and `Rented-server runtime`
    /// G21 was fed an empty list.
    #[test]
    fn a_waiting_job_holds_the_update_and_its_ending_sends_it() {
        let server = a_server();
        let job = a_job();
        let mut machine = AMachineThatRecordsUpdates::running(Some(6), 7);

        let waiting = a_line(&[(job.clone(), QueueState::Waiting)]);
        the_step_nine(&server, Some(&waiting), &nothing_tracked, &mut machine);
        assert!(
            machine.sent.is_empty(),
            "an update was sent over a waiting job"
        );
        assert_eq!(
            what_the_server_carries(&server),
            Some(WhatSmoozyOnTheServerIs::TheUpdateWaits)
        );

        // **Held and rendering hold it the same way.**
        for state in [
            QueueState::Held {
                hold: Identity::issue(IdentityKind::Operation).expect("randomness"),
            },
            QueueState::Rendering {
                operation: Identity::issue(IdentityKind::Operation).expect("randomness"),
            },
        ] {
            the_step_nine(
                &server,
                Some(&a_line(&[(job.clone(), state)])),
                &nothing_tracked,
                &mut machine,
            );
            assert!(machine.sent.is_empty());
        }
        // **No reading is not an empty line**: the update waits.
        the_step_nine(&server, None, &nothing_tracked, &mut machine);
        assert!(
            machine.sent.is_empty(),
            "an update was sent with no work reading at all"
        );

        // The job ends while connected, and this copy has learned its ending.
        let ended = a_line(&[(
            job.clone(),
            QueueState::Ended {
                outcome: Identity::issue(IdentityKind::Operation).expect("randomness"),
            },
        )]);
        let learned = |_: &Identity| Some(true);
        the_step_nine(&server, Some(&ended), &learned, &mut machine);
        assert_eq!(
            machine.sent.len(),
            1,
            "the job's ending did not send the waiting update"
        );
        assert_eq!(
            machine.sent[0].1,
            vec![(job.as_text(), AcceptedWorkStanding::Nothing)],
            "`Rented-server runtime` G21 was not fed the real reading"
        );
        assert_eq!(what_the_server_carries(&server), None);
    }

    /// **`Rented-server runtime` G21's reading: an ended job is an uncollected
    /// result only where this
    /// copy tracks it and has not learned its ending**, and a job it never
    /// tracked is not its result to collect.
    #[test]
    fn an_ended_job_holds_the_update_only_while_this_copy_has_not_collected_it() {
        let job = a_job();
        let ended = a_line(&[(
            job.clone(),
            QueueState::Ended {
                outcome: Identity::issue(IdentityKind::Operation).expect("randomness"),
            },
        )]);
        let not_learned = |_: &Identity| Some(false);
        assert_eq!(
            what_g21_reads(&ended, &not_learned),
            vec![(job.as_text(), AcceptedWorkStanding::Uncollected)]
        );
        assert_eq!(
            what_g21_reads(&ended, &nothing_tracked),
            vec![(job.as_text(), AcceptedWorkStanding::Nothing)]
        );
        let server = a_server();
        let mut machine = AMachineThatRecordsUpdates::running(Some(6), 7);
        the_step_nine(&server, Some(&ended), &not_learned, &mut machine);
        assert!(
            machine.sent.is_empty(),
            "an update was sent over an uncollected result"
        );
    }

    /// **V12: a deferred phase found on the machine is re-sent under its
    /// recorded operation identity**, and never a fresh one.
    #[test]
    fn a_deferred_phase_is_re_sent_under_its_recorded_identity() {
        use smoozy_runtime::package::phase::{a_fresh_attempts_first_write, Phase};
        let server = a_server();
        let recorded = Identity::issue(IdentityKind::Operation).expect("randomness");
        let mut record = a_fresh_attempts_first_write(
            recorded.clone(),
            smoozy_runtime::package::release::VerifiedReleaseDescriptor {
                manifest_format: "smoozy-server-package-manifest-1".into(),
                verified_release_sequence: 7,
                requested_package_version: "0.1.7".into(),
                manifest_digest: [1; 32],
                artifact_identity: "linux-x86_64-gnu".into(),
                artifact_byte_length: 11,
                artifact_digest: [2; 32],
            },
            None,
            smoozy_runtime::machine_kind::MachineKind::Systemd,
            "/a/staging".into(),
        );
        record.phase = Phase::Deferred;
        let mut machine = AMachineThatRecordsUpdates::running(Some(6), 7);
        machine.holds.the_phase_record = Some(record);
        the_step_nine(&server, Some(&a_line(&[])), &nothing_tracked, &mut machine);
        assert_eq!(machine.sent.len(), 1);
        assert_eq!(
            machine.sent[0].0,
            Some(recorded),
            "the re-send was not under the recorded identity"
        );
    }

    /// **The same release: nothing is sent, and none of V13's five stands** —
    /// *the update failed* included, whose ending is a later update landing.
    #[test]
    fn the_same_release_gets_nothing() {
        let server = a_server();
        crate::servers::published::the_server_carries(
            &server,
            Some(WhatSmoozyOnTheServerIs::TheUpdateFailed {
                said: "earlier".into(),
            }),
        );
        let mut machine = AMachineThatRecordsUpdates::running(Some(7), 7);
        let did = the_step_nine(&server, Some(&a_line(&[])), &nothing_tracked, &mut machine);
        assert!(machine.sent.is_empty());
        assert_eq!(
            did,
            WhatStepNineDid::Decided {
                decided: WhatStepNineDecides::NothingIsSent,
                ending: None
            }
        );
        assert_eq!(what_the_server_carries(&server), None);
        // **And a machine with no active package is not this step's.**
        let mut bare = AMachineThatRecordsUpdates::running(None, 7);
        the_step_nine(&server, Some(&a_line(&[])), &nothing_tracked, &mut bare);
        assert!(bare.sent.is_empty());
    }

    /// **V14: a newer machine gets no request from V11, and is marked too
    /// new.**
    #[test]
    fn a_newer_machine_gets_no_request_and_is_too_new() {
        let server = a_server();
        let mut machine = AMachineThatRecordsUpdates::running(Some(8), 7);
        the_step_nine(&server, Some(&a_line(&[])), &nothing_tracked, &mut machine);
        assert!(
            machine.sent.is_empty(),
            "a newer machine was sent an older release"
        );
        assert_eq!(
            what_the_server_carries(&server),
            Some(WhatSmoozyOnTheServerIs::TooNew)
        );
    }

    /// **W0-2, in `Server protocol and transfers` B10a: a copy given a desk
    /// program takes it as its own release, at nought.** A machine the shortcut
    /// placed reads the same and gets nothing; one a published release placed
    /// reads too new and gets nothing either.
    #[cfg(debug_assertions)]
    #[test]
    fn a_shortcut_copy_neither_updates_nor_refuses_its_own_desk_program() {
        let desk = crate::the_desk_program::THE_SEQUENCE_A_DESK_PROGRAM_IS_PLACED_UNDER;
        let server = a_server();
        let mut desk_placed = AMachineThatRecordsUpdates::running(Some(desk), desk);
        the_step_nine(
            &server,
            Some(&a_line(&[])),
            &nothing_tracked,
            &mut desk_placed,
        );
        assert!(
            desk_placed.sent.is_empty(),
            "a shortcut copy updated its own desk program"
        );
        assert_eq!(
            what_the_server_carries(&server),
            None,
            "a shortcut copy refused its own desk program"
        );
        let mut release_placed = AMachineThatRecordsUpdates::running(Some(6), desk);
        the_step_nine(
            &server,
            Some(&a_line(&[])),
            &nothing_tracked,
            &mut release_placed,
        );
        assert!(release_placed.sent.is_empty());
        assert_eq!(
            what_the_server_carries(&server),
            Some(WhatSmoozyOnTheServerIs::TooNew)
        );
    }

    /// **V4: one request outstanding per server**, and a failed update is said
    /// with the transaction's own words.
    #[test]
    fn one_request_is_outstanding_and_a_failure_keeps_its_words() {
        let server = a_server();
        assert!(crate::servers::published::an_update_is_taken_in_hand(
            &server
        ));
        let mut machine = AMachineThatRecordsUpdates::running(Some(6), 7);
        let did = the_step_nine(&server, Some(&a_line(&[])), &nothing_tracked, &mut machine);
        assert_eq!(did, WhatStepNineDid::ARequestIsAlreadyInHand);
        assert!(machine.sent.is_empty());

        crate::servers::published::the_server_carries(&server, None);
        machine.ending = TheUpdatesEnding::ItFailed {
            said: "the health exchange did not answer".into(),
        };
        the_step_nine(&server, Some(&a_line(&[])), &nothing_tracked, &mut machine);
        assert_eq!(
            what_the_server_carries(&server),
            Some(WhatSmoozyOnTheServerIs::TheUpdateFailed {
                said: "the health exchange did not answer".into()
            })
        );
        // **A package area that could not be read moves nothing.**
        let mut unread = AMachineThatRecordsUpdates::running(Some(6), 7);
        unread.holds.it_was_read = false;
        assert_eq!(
            the_step_nine(&server, Some(&a_line(&[])), &nothing_tracked, &mut unread),
            WhatStepNineDid::ThePackageAreaCouldNotBeRead
        );
        assert!(unread.sent.is_empty());
    }

    /// **The twenty-fourth notes, note 4: a server whose copy's own release
    /// cannot be read still offers the queue's Telegram control.** Step 9 used
    /// to return at the own-release check before the line that records the
    /// server's release, though that line needs only the package area. A
    /// desk-placed program and a published one at the first release that
    /// brings the change are both read; nothing is sent; an unread package
    /// area still records nothing.
    #[test]
    fn twenty_fourth_4_a_server_whose_own_release_cannot_be_read_still_offers_the_telegram_control() {
        use crate::servers::published::{a_live_jobs_reporting_can_be_changed_on, ALiveJobsReportingCanBeChanged};
        use smoozy_server_protocol::requests::THE_FIRST_RELEASE_THAT_CHANGES_A_LIVE_JOBS_REPORTING as FIRST;
        // **The desk-placed program is a development build's alone** (`Server
        // protocol and transfers` B10a); a release build reads the published one.
        #[cfg(debug_assertions)]
        let the_releases = vec![crate::the_desk_program::THE_SEQUENCE_A_DESK_PROGRAM_IS_PLACED_UNDER, FIRST];
        #[cfg(not(debug_assertions))]
        let the_releases = vec![FIRST];
        for its_release in the_releases {
            let server = a_server();
            let mut machine = AMachineThatRecordsUpdates::running(Some(its_release), FIRST);
            machine.own = Err("Could not read the release file.".to_string());
            assert!(matches!(
                the_step_nine(&server, Some(&a_line(&[])), &nothing_tracked, &mut machine),
                WhatStepNineDid::TheOwnReleaseCouldNotBeHad { .. }
            ));
            assert!(machine.sent.is_empty(), "nothing is compared, so nothing is sent");
            assert_eq!(
                a_live_jobs_reporting_can_be_changed_on(&server),
                Some(ALiveJobsReportingCanBeChanged::Yes),
                "the server's release, read, was not recorded"
            );
        }
        let server = a_server();
        let mut unread = AMachineThatRecordsUpdates::running(Some(FIRST), FIRST);
        unread.holds.it_was_read = false;
        the_step_nine(&server, Some(&a_line(&[])), &nothing_tracked, &mut unread);
        assert_eq!(a_live_jobs_reporting_can_be_changed_on(&server), None);
    }

    /// **The twenty-fourth notes, note 4, the runner's follow-up: a Blender
    /// install that held step 9 back has it taken when it ends, on the same
    /// connection**, and the queue's Telegram control is then offered. Until
    /// then step 9 ran once per connection and returned at once while a setup
    /// drove the server, so the release was never read before the next
    /// connection. **A row with no confirmed installation holds it back the
    /// same way**; what is owed is taken once.
    #[test]
    fn twenty_fourth_4_a_blender_install_that_held_step_nine_back_has_it_taken_when_it_ends() {
        use smoozy_server_setup::run::SetupKind;
        let server = a_server();
        let installation = Identity::issue(IdentityKind::Installation).expect("randomness");
        assert!(crate::setting_up::a_run_begins(&server, SetupKind::ABlenderInstall));
        assert_eq!(
            step_nine_may_run_now(&server, 7, Some(installation.clone())),
            None,
            "step 9 ran under a setup"
        );
        assert_eq!(a_held_back_step_nine_is_owed_on(&server), None, "owed while the run still drives");
        crate::setting_up::a_run_ends(&server);
        assert_eq!(
            a_held_back_step_nine_is_owed_on(&server),
            Some(7),
            "the run ended and the step 9 it held back on this connection was not owed"
        );
        assert_eq!(a_held_back_step_nine_is_owed_on(&server), None, "owed twice");
        assert_eq!(step_nine_may_run_now(&server, 7, Some(installation.clone())), Some(installation));
        // **The step 9 then taken is read on a desk-placed program**, which is
        // a development build's alone (`Server protocol and transfers` B10a).
        #[cfg(debug_assertions)]
        {
            use crate::servers::published::{a_live_jobs_reporting_can_be_changed_on, ALiveJobsReportingCanBeChanged};
            let desk = crate::the_desk_program::THE_SEQUENCE_A_DESK_PROGRAM_IS_PLACED_UNDER;
            let mut machine = AMachineThatRecordsUpdates::running(Some(desk), desk);
            the_step_nine(&server, Some(&a_line(&[])), &nothing_tracked, &mut machine);
            assert_eq!(
                a_live_jobs_reporting_can_be_changed_on(&server),
                Some(ALiveJobsReportingCanBeChanged::Yes),
                "a server a Blender install held back left the Telegram control unread"
            );
        }

        // **No confirmed installation yet** holds it back too; a step 9 that
        // then runs settles it.
        let other = a_server();
        assert_eq!(step_nine_may_run_now(&other, 9, None), None);
        let confirmed = Identity::issue(IdentityKind::Installation).expect("randomness");
        assert_eq!(step_nine_may_run_now(&other, 9, Some(confirmed.clone())), Some(confirmed));
        assert_eq!(a_held_back_step_nine_is_owed_on(&other), None, "a step 9 that ran left one owed");
    }

    /// **The app lets every setup run's ending take a held-back step 9**: the
    /// hook is registered at start, beside the periods it needs.
    #[test]
    fn twenty_fourth_4_the_app_registers_step_nine_after_a_run() {
        let main = include_str!("../main.rs");
        assert!(main.contains("the_machines_work::a_held_back_step_nine_follows_every_run("));
    }

    // ── The 38's piece 1: an ending read while connected is posted ─────────

    use crate::notifications::kinds::Kind;
    use crate::notifications::{APost, TheMachinery, TheNotificationService, ThePlatform};
    use crate::record::settings::TheSwitches;
    use crate::servers::the_endings_posted::{are_handed_to, what_these_endings_post};
    use smoozy_queue_history::acting::note::{TheEndingOnTheNote, TheOutcomeAsWritten};

    /// **A system that writes down what it was asked to post** — the key, the
    /// kind, and the two lines as the system would draw them.
    #[derive(Default)]
    pub(crate) struct WhatWasPosted(
        pub(crate) std::sync::Mutex<Vec<(String, Kind, String, String)>>,
    );

    impl ThePlatform for Arc<WhatWasPosted> {
        fn the_machinery(&self) -> TheMachinery {
            TheMachinery::ItIsInPlace
        }
        fn the_permission(&self) -> crate::notifications::permission::WhatTheSystemSays {
            crate::notifications::permission::WhatTheSystemSays::ItIsGranted
        }
        fn ask_for_permission_now(&self) {}
        fn post(&self, post: &APost) {
            self.0.lock().unwrap().push((
                post.key.as_text(),
                post.kind,
                post.headline(),
                post.the_second_line(),
            ));
        }
        fn take_down(&self, _key: &crate::presentation::the_presentation_key::ThePresentationKey) {}
    }

    pub(crate) fn a_service_that_writes_down(
        switches: TheSwitches,
    ) -> (Arc<WhatWasPosted>, TheNotificationService) {
        let system = Arc::new(WhatWasPosted::default());
        let service = TheNotificationService::on(Box::new(system.clone()));
        service.the_switches_are(Some(switches));
        (system, service)
    }

    /// A confirmed record of this server, written.
    fn a_confirmed_record(
        record: &ARecordInHand,
        server: &Identity,
        label: &str,
    ) -> smoozy_queue_history::record::LocalJobRecord {
        let mut made = a_provisional_record(server, label);
        made.standing = Standing::Confirmed;
        write(record, &made);
        made
    }

    /// **The note's half carried over, as the fill-in carries it** — an ending,
    /// its two machine-clock moments, and the frame a job short of a clean
    /// finish was on.
    pub(crate) fn it_ended(
        record: &ARecordInHand,
        job: &Identity,
        outcome: TheOutcomeAsWritten,
        took: u64,
        at_frame: Option<i64>,
    ) {
        let on_the_machine = Clock::AMachine(MachineClock("the machine".to_string()));
        let start = SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(1_754_700_000);
        let mut note = TheNote::created_with_the_job_record("whatever");
        note.started = Some(Reading::taken((), start, on_the_machine.clone()));
        note.ended = Some(Reading::taken(
            (),
            start + std::time::Duration::from_secs(took),
            on_the_machine,
        ));
        note.attempts = 1;
        note.last_frame_figure = at_frame;
        note.ending = Some(TheEndingOnTheNote {
            outcome,
            exit_status: Some(0),
            the_attempt_whose_capture_record_holds_the_words: 1,
            the_reason_the_machine_recorded: None,
            no_render_began: false,
            the_device_line_as_observed: None,
        });
        JobRecords::over(&record.0).the_fill_in(job, &note).unwrap();
    }

    /// **Step 7 on a fresh connection, its step 3 reading `read` and then
    /// nothing** — the catch-up account.
    fn step_seven_on_a_new_connection(
        record: &ARecordInHand,
        server: &Identity,
        holds: &WhatThisRunHolds,
        read: Fields,
        then_from: u64,
    ) {
        let connection = holds.a_connection_begins(&server.as_text());
        // **A note is answered only where step 3 reads an event**: a job still
        // rendering at the connection has no ending on its note.
        let a_note = (then_from > 0).then(|| a_note_that_started_and_finished("Kitchen"));
        let mut machine = AMachineInHand::default()
            .answers(THE_READ_FORWARD_REQUEST, Some(read))
            .answers(
                THE_READ_FORWARD_REQUEST,
                Some(events(then_from, &[], Some(0))),
            )
            .answers(THE_PER_JOB_NOTE_READING_REQUEST, a_note)
            .answers(THE_CURSOR_MOVES_FORWARD_REQUEST, Some(Fields::none()));
        the_step_seven(
            server,
            connection,
            record,
            &mut machine,
            holds,
            &nothing_is_live,
        );
    }

    /// **`Desktop app and delivery` T18 and T20, both halves, through the real
    /// read forward: an ending read on the live read forward after step 3 is
    /// posted once — and one read at a connection's catch-up posts nothing.**
    ///
    /// **This is the line that fails if a post is made from step 3**, which
    /// would put a burst of old endings on the screen at every reconnection —
    /// the thing T20 is written to forbid — and **if the live ending is not put
    /// out at all**, which is what every ending did until the 38's piece 1.
    #[test]
    fn an_ending_read_live_posts_once_and_one_read_at_a_catch_up_posts_nothing() {
        let (record, server) = a_record_with_a_server();
        let holds = WhatThisRunHolds::default();
        let kitchen = a_confirmed_record(&record, &server, "Kitchen");

        // The connection's step 3: nothing past the cursor.
        step_seven_on_a_new_connection(&record, &server, &holds, events(0, &[], Some(0)), 0);
        assert!(holds
            .the_endings_to_tell_are_taken(&server.as_text())
            .is_empty());

        // **Now**: the job finishes while connected, and P10d's pass reads it.
        let mut live = AMachineInHand::default()
            .answers(
                THE_READ_FORWARD_REQUEST,
                Some(events(
                    0,
                    &[
                        (THE_RENDER_ATTEMPT_STARTED_EVENT, Some(&kitchen.job)),
                        (THE_RECORDED_OUTCOME_WRITTEN_EVENT, Some(&kitchen.job)),
                    ],
                    Some(0),
                )),
            )
            .answers(THE_READ_FORWARD_REQUEST, Some(events(2, &[], Some(0))))
            .answers(
                THE_PER_JOB_NOTE_READING_REQUEST,
                Some(a_note_that_started_and_finished("Kitchen")),
            )
            .answers(THE_CURSOR_MOVES_FORWARD_REQUEST, Some(Fields::none()));
        let nudged_before = holds.idle_nudges().how_far(&server.as_text());
        a_read_forward_pass(&server, &record, &mut live, &holds);
        let told = holds.the_endings_to_tell_are_taken(&server.as_text());
        assert_eq!(told, vec![kitchen.job.clone()]);
        assert_ne!(
            holds.idle_nudges().how_far(&server.as_text()),
            nudged_before,
            "**`Queue & History` W4**: an ending applied wakes the idle re-check to look"
        );

        let (system, service) = a_service_that_writes_down(TheSwitches::nobody_has_turned_one());
        are_handed_to(&service, what_these_endings_post(&record, &server, &told));
        assert_eq!(
            system.0.lock().unwrap().clone(),
            vec![(
                format!("job:{}", kitchen.job.as_text()),
                Kind::AJobFinished,
                "Kitchen finished".to_string(),
                "1\u{00A0}hour \u{00B7} the big box".to_string(),
            )],
            "**Design's two lines**: the name and what happened, then the duration and the server"
        );
        // **Taken once**: a second pass finds nothing left to post.
        assert!(holds
            .the_endings_to_tell_are_taken(&server.as_text())
            .is_empty());

        // **T20: the line drops; a job ends during the outage; the next
        // connection's step 3 reads it.** Catch-up, and nothing is put out.
        let clay = a_confirmed_record(&record, &server, "Clay");
        holds.the_connection_ended(&server.as_text());
        step_seven_on_a_new_connection(
            &record,
            &server,
            &holds,
            events(
                2,
                &[
                    (THE_RENDER_ATTEMPT_STARTED_EVENT, Some(&clay.job)),
                    (THE_RECORDED_OUTCOME_WRITTEN_EVENT, Some(&clay.job)),
                ],
                Some(0),
            ),
            4,
        );
        assert_eq!(
            JobRecords::over(&record.0)
                .read(&clay.job)
                .unwrap()
                .unwrap()
                .the_ending_learned(),
            Some(smoozy_queue_history::ending::Ending::Finished),
            "the catch-up applied the ending"
        );
        assert!(
            holds.the_endings_to_tell_are_taken(&server.as_text()).is_empty(),
            "**T18: an ending read at a connection's catch-up posts nothing** — it is the catch-up \
             notice's"
        );
        assert!(
            the_account_of(&record, &server).is_some(),
            "and it is the catch-up account's"
        );
    }

    /// **A step 3 that could not complete leaves the server catching up**, and
    /// the live pass that reads its remainder to the end is catch-up too —
    /// T18's *now* is *after that step has completed*. **What it reads after
    /// that is now.** And **a read forward a stop, a clear or a cancel takes
    /// inside the period** is the same function, so its ending is put out just
    /// the same.
    #[test]
    fn the_remainder_of_an_unfinished_step_three_is_catch_up_and_what_follows_is_now() {
        let (record, server) = a_record_with_a_server();
        let holds = WhatThisRunHolds::default();
        let first = a_confirmed_record(&record, &server, "First");
        let second = a_confirmed_record(&record, &server, "Second");

        // Step 3 asks and nothing answers: it did not complete.
        let connection = holds.a_connection_begins(&server.as_text());
        let mut silent = AMachineInHand::default().answers(THE_READ_FORWARD_REQUEST, None);
        the_step_seven(
            &server,
            connection,
            &record,
            &mut silent,
            &holds,
            &nothing_is_live,
        );

        let one_ending = |job: &Identity, from: u64| {
            AMachineInHand::default()
                .answers(
                    THE_READ_FORWARD_REQUEST,
                    Some(events(
                        from,
                        &[(THE_RECORDED_OUTCOME_WRITTEN_EVENT, Some(job))],
                        Some(0),
                    )),
                )
                .answers(
                    THE_READ_FORWARD_REQUEST,
                    Some(events(from + 1, &[], Some(0))),
                )
                .answers(
                    THE_PER_JOB_NOTE_READING_REQUEST,
                    Some(a_note_that_started_and_finished("x")),
                )
                .answers(THE_CURSOR_MOVES_FORWARD_REQUEST, Some(Fields::none()))
        };
        a_read_forward_pass(&server, &record, &mut one_ending(&first.job, 0), &holds);
        assert!(
            holds
                .the_endings_to_tell_are_taken(&server.as_text())
                .is_empty(),
            "step 3's remainder, read by a live pass, is still catch-up"
        );
        a_read_forward_pass(&server, &record, &mut one_ending(&second.job, 1), &holds);
        assert_eq!(
            holds.the_endings_to_tell_are_taken(&server.as_text()),
            vec![second.job.clone()]
        );
    }

    /// **`Desktop app and delivery` T10 to T12 and T27, through the records**:
    /// a failed job speaks for itself inside its group, with the frame it was
    /// on; the last member carries the group's own finishing, with the scene
    /// count; a job the user stopped posts nothing.
    #[test]
    fn a_failure_speaks_for_itself_the_last_member_speaks_for_the_group_and_a_stop_says_nothing() {
        let (record, server) = a_record_with_a_server();
        let group = Identity::issue(IdentityKind::Group).unwrap();
        let mut members = Vec::new();
        for _ in 0..3 {
            let mut made = a_provisional_record(&server, "Product spins");
            made.standing = Standing::Confirmed;
            made.group = Some(group.clone());
            write(&record, &made);
            members.push(made.job);
        }
        let alone = a_confirmed_record(&record, &server, "Clay");

        it_ended(
            &record,
            &members[0],
            TheOutcomeAsWritten::Failed,
            600,
            Some(61),
        );
        let posts = what_these_endings_post(&record, &server, &[members[0].clone()]);
        assert_eq!(
            posts.len(),
            1,
            "a failure inside a group speaks for itself, and only it"
        );
        assert_eq!(posts[0].kind, Kind::AJobFailedOrWasGivenUp);
        assert_eq!(posts[0].headline(), "Product spins failed");
        assert_eq!(
            posts[0].the_second_line(),
            "at frame\u{00A0}61 \u{00B7} the big box"
        );

        it_ended(
            &record,
            &members[1],
            TheOutcomeAsWritten::Finished,
            600,
            None,
        );
        assert!(
            what_these_endings_post(&record, &server, &[members[1].clone()]).is_empty(),
            "**T11: no member posts its own finishing**, and the group is not done"
        );

        it_ended(
            &record,
            &members[2],
            TheOutcomeAsWritten::Finished,
            600,
            None,
        );
        let posts = what_these_endings_post(&record, &server, &[members[2].clone()]);
        assert_eq!(posts.len(), 1);
        assert_eq!(posts[0].kind, Kind::AJobFinished);
        assert_eq!(
            posts[0].key,
            crate::presentation::the_presentation_key::ThePresentationKey::AGroupsFinishing(
                group.clone()
            )
        );
        assert_eq!(posts[0].headline(), "Product spins finished");
        // All three started at one moment and ran ten minutes, so the batch's
        // first start to last end is ten minutes.
        assert_eq!(
            posts[0].the_second_line(),
            "3\u{00A0}scenes in 10\u{00A0}minutes \u{00B7} the big box"
        );

        // **Two members ending in one read close the group once.**
        let posts =
            what_these_endings_post(&record, &server, &[members[1].clone(), members[2].clone()]);
        assert_eq!(posts.len(), 1);

        it_ended(
            &record,
            &alone.job,
            TheOutcomeAsWritten::LeftStopped,
            600,
            Some(12),
        );
        assert!(
            what_these_endings_post(&record, &server, &[alone.job.clone()]).is_empty(),
            "**T10: a job the user stopped posts nothing**"
        );
    }

    /// **The four Settings switches each stop their kind** — `Desktop app and
    /// delivery` T8 and T17a, over the very posts this piece's two routes
    /// compose: a finished and a failed job from the records, the idle telling
    /// from `Queue & History`'s re-check, and the connection kind beside them.
    #[test]
    fn each_of_the_four_switches_stops_its_own_kind() {
        let (record, server) = a_record_with_a_server();
        let finished = a_confirmed_record(&record, &server, "Kitchen");
        let failed = a_confirmed_record(&record, &server, "Clay");
        it_ended(
            &record,
            &finished.job,
            TheOutcomeAsWritten::Finished,
            60,
            None,
        );
        it_ended(
            &record,
            &failed.job,
            TheOutcomeAsWritten::Failed,
            60,
            Some(3),
        );
        let every_kind = || {
            let mut posts = what_these_endings_post(
                &record,
                &server,
                &[finished.job.clone(), failed.job.clone()],
            );
            posts.push(APost::from(
                crate::notifications::kinds::what_an_idle_machine_posts(&server),
                crate::typography::Verbatim::new("the big box"),
                crate::notifications::picture::the_picture_for(std::env::consts::OS, None),
            ));
            posts.push(APost::from(
                crate::notifications::kinds::what_a_raised_connection_condition_posts(
                    &Identity::issue(IdentityKind::Condition).unwrap(),
                ),
                crate::typography::Verbatim::new("the big box"),
                crate::notifications::picture::the_picture_for(std::env::consts::OS, None),
            ));
            posts
        };
        for off in Kind::ALL {
            let (system, service) =
                a_service_that_writes_down(TheSwitches::nobody_has_turned_one().with(off, false));
            are_handed_to(&service, every_kind());
            let kinds: Vec<Kind> = system
                .0
                .lock()
                .unwrap()
                .iter()
                .map(|(_, kind, _, _)| *kind)
                .collect();
            assert!(
                !kinds.contains(&off),
                "{off:?} was switched off and still posted"
            );
            assert_eq!(
                kinds.len(),
                3,
                "{off:?} off stopped something else too: {kinds:?}"
            );
        }
    }

    // ── Piece 6b: a rendering job's figures, read and judged ────────────────

    /// **A queue answer as the supervisor composes it**: one rendering job
    /// carrying the worker's report and its two sinks under the names the
    /// machine's own `as_pairs` writes, and one waiting job carrying nothing.
    pub(crate) fn a_queue_answer_with_figures(
        rendering: &Identity,
        waiting: &Identity,
        report: Option<&smoozy_queue_history::readings::AWorkersReport>,
        sinks: Option<&smoozy_queue_history::readings::TheSinksOfARenderingJob>,
    ) -> Fields {
        let item = |at: usize, name: &str| the_field_of_item(THE_JOB_FIELD, at, name);
        let mut fields = Fields::none()
            .and(HOW_MANY_JOBS_FIELD, "2")
            .and(&item(0, THE_JOB_FIELD), rendering.as_text())
            .and(&item(0, THE_QUEUE_STATE_FIELD), IT_IS_RENDERING)
            .and(
                &item(0, THE_RENDER_OPERATION_FIELD),
                Identity::issue(IdentityKind::Operation).unwrap().as_text(),
            )
            .and(&item(1, THE_JOB_FIELD), waiting.as_text())
            .and(&item(1, THE_QUEUE_STATE_FIELD), IT_IS_WAITING);
        for (name, value) in report.map(|report| report.as_pairs()).unwrap_or_default() {
            fields = fields.and(&item(0, name), value);
        }
        for (name, value) in sinks.map(|sinks| sinks.as_pairs()).unwrap_or_default() {
            fields = fields.and(&item(0, name), value);
        }
        fields
    }

    /// The worker's report, taken at `seconds` on the machine's own clock.
    pub(crate) fn a_report_taken_at(
        seconds: u64,
    ) -> smoozy_queue_history::readings::AWorkersReport {
        use smoozy_queue_history::readings::{AWorkersReport, LiveReadings, SamplePair};
        AWorkersReport {
            attempt: 1,
            readings: Reading::taken(
                LiveReadings {
                    frame_in_hand: Some(1007),
                    samples: Some(SamplePair { seen: 64, of: 128 }),
                    blenders_own_estimate_for_the_frame: Some("00:12.58".to_string()),
                    frames_completed: 6,
                    ..LiveReadings::default()
                },
                SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(seconds),
                Clock::AMachine(MachineClock("installation-1".to_string())),
            ),
            newest_frame_saved: Some(1006),
            the_plan_total: Some(1_280),
            the_units_finished: Some(768),
        }
    }

    /// **Step 12's SC4 follow-up: an ended job's part carrying its ending owed
    /// to a full store reaches the desktop whole** — the outcome, the owed mark
    /// and `Rented-server runtime` V12's reading (`Rented-server runtime`
    /// S14a) — **and an ending that landed carries none**, so History reads
    /// the first from the reading and the second from the record.
    ///
    /// **What it bites on**: the reader dropping what rides beside *ended*,
    /// which is how it read before this piece — the job then on neither list
    /// for as long as the disk stayed full.
    #[test]
    fn an_ending_owed_to_a_full_store_reaches_the_desktop_beside_the_jobs_ended() {
        use smoozy_queue_history::acting::note::{
            TheEndingOwed, TheFreeSpaceAtTheEnding, TheFreeSpaceReadAtTheEnding,
            TheOutcomeAsWritten,
        };
        let (owed_job, landed_job) = (
            Identity::issue(IdentityKind::Job).unwrap(),
            Identity::issue(IdentityKind::Job).unwrap(),
        );
        let owed = TheEndingOwed {
            outcome: TheOutcomeAsWritten::Failed,
            the_free_space_at_the_ending: Some(TheFreeSpaceAtTheEnding::It(
                TheFreeSpaceReadAtTheEnding {
                    available_bytes: 0,
                    the_folder_it_was_taken_against: "/root/smoozy/out/shot-40".to_string(),
                    taken_at: Reading::taken(
                        (),
                        SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(1_790_000_000),
                        Clock::AMachine(MachineClock("installation-1".to_string())),
                    ),
                },
            )),
        };
        let item = |at: usize, name: &str| the_field_of_item(THE_JOB_FIELD, at, name);
        let an_operation = || Identity::issue(IdentityKind::Operation).unwrap().as_text();
        let mut fields = Fields::none()
            .and(HOW_MANY_JOBS_FIELD, "2")
            .and(&item(0, THE_JOB_FIELD), owed_job.as_text())
            .and(&item(0, THE_QUEUE_STATE_FIELD), IT_HAS_ENDED)
            .and(&item(0, THE_OUTCOME_IT_ENDED_WITH_FIELD), an_operation())
            .and(&item(1, THE_JOB_FIELD), landed_job.as_text())
            .and(&item(1, THE_QUEUE_STATE_FIELD), IT_HAS_ENDED)
            .and(&item(1, THE_OUTCOME_IT_ENDED_WITH_FIELD), an_operation());
        for (name, value) in owed.as_pairs() {
            fields = fields.and(&item(0, name), value);
        }
        let request = the_queue_reading_request().unwrap();
        let state = the_queue_reading_in(WhatCameBack::ItAnswered(
            Answered::to(&request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, "test").carrying(fields),
        ))
        .expect("the answer reads");
        assert_eq!(
            state.jobs[0].live.the_ending_owed(),
            Some(&owed),
            "`Rented-server runtime` S14a: everything beside the state arrives"
        );
        assert_eq!(
            state.jobs[1].live.the_ending_owed(),
            None,
            "an ending that landed is the record's"
        );
        let holds = WhatThisRunHolds::default();
        let connection = holds.a_connection_begins("server-owed");
        holds.a_reading_was_taken(
            "server-owed",
            connection,
            Reading::taken(state, SystemTime::now(), Clock::ThisComputer),
            None,
        );
        let carried = crate::surface::the_history::the_endings_owed_in(&holds);
        assert_eq!(
            carried,
            vec![crate::surface::the_history::AnEndingTheReadingCarries {
                server: "server-owed".to_string(),
                job: owed_job,
                owed,
            }]
        );
    }

    /// **Piece 6b: the queue reading's rendering job brings its figures and its
    /// sinks to the desktop, a waiting job brings none, and figures that stop
    /// moving for longer than six seconds of this computer's clock are not
    /// drawn** — the build runner's ruling 3 of 2026-10-03, kept without
    /// comparing the machine's clock with this one (`State, identity and
    /// recovery` D5). Nothing is drawn after *Not updating* either.
    ///
    /// **What it bites on**: the reader dropping the figures (the state before
    /// this piece), dropping the sinks, old figures drawn, and figures drawn
    /// beside *Not updating*.
    #[test]
    fn piece_6b_a_rendering_jobs_figures_reach_the_desktop_and_old_ones_are_not_drawn() {
        use smoozy_queue_history::readings::{ASink, TheSinksOfARenderingJob};
        let rendering = Identity::issue(IdentityKind::Job).unwrap();
        let waiting = Identity::issue(IdentityKind::Job).unwrap();
        let sinks = TheSinksOfARenderingJob {
            ordinary: ASink {
                path: "/root/smoozy/streams/job/1/ordinary".to_string(),
                length: Some(4_096),
            },
            complaints: ASink {
                path: "/root/smoozy/streams/job/1/complaints".to_string(),
                length: Some(0),
            },
        };
        let read = |report: &smoozy_queue_history::readings::AWorkersReport| {
            let request = the_queue_reading_request().unwrap();
            the_queue_reading_in(WhatCameBack::ItAnswered(
                Answered::to(&request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, "test").carrying(
                    a_queue_answer_with_figures(&rendering, &waiting, Some(report), Some(&sinks)),
                ),
            ))
            .expect("the answer reads")
        };
        let first = a_report_taken_at(1_790_000_000);
        let state = read(&first);
        assert_eq!(
            state.jobs[0].live.the_figures(),
            Some(&first),
            "the figures, whole"
        );
        assert_eq!(
            state.jobs[0].live.the_sinks(),
            Some(&sinks),
            "the two sinks, with their lengths"
        );
        assert_eq!(
            state.jobs[1].live,
            WhatTheEntryHasLive::NoReadings,
            "a waiting job carries none"
        );

        let server = "server-6b";
        let holds = WhatThisRunHolds::default();
        let connection = holds.a_connection_begins(server);
        let on_this_computer =
            |seconds: u64| SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(seconds);
        let taken = |state, at| {
            holds.a_reading_was_taken(
                server,
                connection,
                Reading::taken(state, on_this_computer(at), Clock::ThisComputer),
                None,
            );
        };
        let drawn = || {
            let (_, period) = holds.every_server().into_iter().next().expect("held");
            let reading = period.reading.clone().expect("a reading");
            period
                .the_figures_to_draw(&reading.value().jobs[0])
                .cloned()
        };
        // The desktop's clock is a day off the machine's, and it does not matter.
        taken(read(&first), 1_790_086_400);
        assert_eq!(drawn(), Some(first.clone()), "fresh figures are drawn");
        taken(read(&first), 1_790_086_405);
        assert_eq!(
            drawn(),
            Some(first.clone()),
            "unmoved for five seconds: still drawn"
        );
        taken(read(&first), 1_790_086_407);
        assert_eq!(
            drawn(),
            None,
            "unmoved for seven seconds: the worker stopped, and nothing is drawn"
        );
        let moved = a_report_taken_at(1_790_000_010);
        taken(read(&moved), 1_790_086_412);
        assert_eq!(drawn(), Some(moved), "moving again: drawn again");
        holds.the_connection_ended(server);
        assert_eq!(drawn(), None, "never a bar after Not updating");
    }

    // ── Piece 6c: the running entry's words, read only as they grow ─────────

    /// **A machine's sinks in hand**: each path's bytes, and every range asked.
    #[derive(Default)]
    pub(crate) struct SinksInHand {
        pub(crate) bytes: HashMap<String, Vec<u8>>,
        pub(crate) asked: Vec<(String, u64, u64)>,
    }

    impl ReadsASinksBytes for SinksInHand {
        fn the_bytes(&mut self, path: &str, from: u64, to: u64) -> Result<Vec<u8>, String> {
            self.asked.push((path.to_string(), from, to));
            let held = self
                .bytes
                .get(path)
                .ok_or_else(|| format!("{path}: no such file"))?;
            held.get(from as usize..to as usize)
                .map(<[u8]>::to_vec)
                .ok_or_else(|| format!("{path}: the stream ended short"))
        }
    }

    /// The two sinks of one attempt, at the lengths the queue reading states.
    pub(crate) fn the_sinks_at(
        attempt: u32,
        ordinary: u64,
        complaints: u64,
    ) -> smoozy_queue_history::readings::TheSinksOfARenderingJob {
        use smoozy_queue_history::readings::{ASink, TheSinksOfARenderingJob};
        TheSinksOfARenderingJob {
            ordinary: ASink {
                path: format!("/root/smoozy/streams/job/{attempt}/ordinary"),
                length: Some(ordinary),
            },
            complaints: ASink {
                path: format!("/root/smoozy/streams/job/{attempt}/complaints"),
                length: Some(complaints),
            },
        }
    }

    /// **Piece 6c, `Queue & History` E8: the words on screen are read from the
    /// length last read to the length the queue reading states — only the new
    /// bytes — and nothing is read while they are not on screen.** A
    /// character a read cut through waits for its end; a new attempt's sinks
    /// start the panel over; closing it stops the reads.
    ///
    /// **What it bites on**: a read that asks for the whole stream every time,
    /// a read taken with the panel closed, a cut character drawn broken, and a
    /// new attempt's words appended to the last one's.
    #[test]
    fn piece_6c_the_words_are_read_only_as_they_grow_and_only_while_on_screen() {
        let rendering = Identity::issue(IdentityKind::Job).unwrap();
        let waiting = Identity::issue(IdentityKind::Job).unwrap();
        let server = "server-6c";
        let holds = WhatThisRunHolds::default();
        let connection = holds.a_connection_begins(server);
        let mut clock = 1_790_086_400u64;
        let mut read_at = |sinks: &smoozy_queue_history::readings::TheSinksOfARenderingJob| {
            let request = the_queue_reading_request().unwrap();
            let state = the_queue_reading_in(WhatCameBack::ItAnswered(
                Answered::to(&request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, "test").carrying(
                    a_queue_answer_with_figures(&rendering, &waiting, None, Some(sinks)),
                ),
            ))
            .expect("the answer reads");
            clock += 5;
            holds.a_reading_was_taken(
                server,
                connection,
                Reading::taken(
                    state,
                    SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(clock),
                    Clock::ThisComputer,
                ),
                None,
            );
        };
        // "Fra:1 … Sample 1/128\n" then "Fra:1 … Saved: 'é'\n" — the é is two bytes.
        let printed = "Fra:1 Sample 1/128\nSaved: '/out/caf\u{e9}.png'\n"
            .as_bytes()
            .to_vec();
        let cut = printed
            .iter()
            .position(|byte| *byte == 0xC3)
            .expect("the first byte of é") as u64
            + 1;
        let mut machine = SinksInHand::default();
        machine
            .bytes
            .insert(the_sinks_at(1, 0, 0).ordinary.path, printed.clone());
        machine
            .bytes
            .insert(the_sinks_at(1, 0, 0).complaints.path, Vec::new());

        read_at(&the_sinks_at(1, 19, 0));
        assert_eq!(
            holds
                .where_a_rendering_jobs_words_are(&rendering.as_text())
                .as_deref(),
            Some(server)
        );
        assert_eq!(
            holds.where_a_rendering_jobs_words_are(&waiting.as_text()),
            None,
            "a waiting job has no words to open"
        );
        assert_eq!(
            the_words_are_read(&holds, server, &mut machine),
            None,
            "nothing is read while nothing is on screen"
        );
        assert!(machine.asked.is_empty());

        holds.the_words_are_opened(server, &rendering.as_text(), "Kitchen");
        assert!(holds.the_words_are_on_screen_for(server));
        let first = the_words_are_read(&holds, server, &mut machine).expect("the first arrival");
        assert!(first.first, "the panel starts from what was printed so far");
        assert_eq!(first.new[0], "Fra:1 Sample 1/128\n");
        assert_eq!(
            first.new[1], "",
            "the complaints stream has printed nothing"
        );
        assert_eq!(first.any_yet, [true, false]);
        assert_eq!(
            machine.asked,
            vec![(the_sinks_at(1, 0, 0).ordinary.path, 0, 19)],
            "an empty sink is not asked for"
        );

        // **The second refresh asks only for the new bytes**, and a character
        // the read cut through waits for its end.
        read_at(&the_sinks_at(1, cut, 0));
        let second = the_words_are_read(&holds, server, &mut machine).expect("more arrived");
        assert!(!second.first);
        assert_eq!(second.new[0], "Saved: '/out/caf");
        assert_eq!(
            machine.asked.last(),
            Some(&(the_sinks_at(1, 0, 0).ordinary.path, 19, cut))
        );
        read_at(&the_sinks_at(1, printed.len() as u64, 0));
        let third = the_words_are_read(&holds, server, &mut machine).expect("the rest arrived");
        assert_eq!(
            third.new[0], "\u{e9}.png'\n",
            "the cut character arrives whole"
        );
        let all: String = [
            first.new[0].as_str(),
            second.new[0].as_str(),
            third.new[0].as_str(),
        ]
        .concat();
        assert_eq!(
            all.as_bytes(),
            printed.as_slice(),
            "the words are the stream, unedited"
        );

        // Nothing new: nothing asked, nothing posted.
        let asked = machine.asked.len();
        read_at(&the_sinks_at(1, printed.len() as u64, 0));
        assert_eq!(the_words_are_read(&holds, server, &mut machine), None);
        assert_eq!(machine.asked.len(), asked);

        // **A new attempt's sinks start the panel over**, from nought.
        machine.bytes.insert(
            the_sinks_at(2, 0, 0).ordinary.path,
            b"Fra:1 again\n".to_vec(),
        );
        machine.bytes.insert(
            the_sinks_at(2, 0, 0).complaints.path,
            b"Error: no\n".to_vec(),
        );
        read_at(&the_sinks_at(2, 12, 10));
        let again =
            the_words_are_read(&holds, server, &mut machine).expect("the new attempt's words");
        assert!(again.first);
        assert_eq!(
            again.new,
            ["Fra:1 again\n".to_string(), "Error: no\n".to_string()]
        );
        assert_eq!(again.any_yet, [true, true]);

        // **Closed, the reads stop.**
        holds.the_words_are_put_away();
        assert!(!holds.the_words_are_on_screen_for(server));
        assert_eq!(the_words_are_read(&holds, server, &mut machine), None);
        // **Not updating: nothing opens.**
        holds.the_connection_ended(server);
        assert_eq!(
            holds.where_a_rendering_jobs_words_are(&rendering.as_text()),
            None
        );
    }

    // ── The Creator's notes on the test copy, piece 8: the entry's own lines ──

    /// **The entry's three lines, `Queue & History` E8 as narrowed by `Desktop
    /// app and delivery` M13b: the ordinary stream's last 4 KB first, then only
    /// the new bytes, three complete lines kept, and nothing read while the
    /// queue's entry is not that rendering job.** The errors stream is never
    /// asked for.
    ///
    /// **What it bites on**: the whole stream read the first time, the whole
    /// stream (or the last 4 KB) read again each time, a partial line shown, a
    /// fourth line kept, the errors stream read, reads taken with the queue put
    /// away, and a new attempt's lines appended to the last one's.
    #[test]
    fn piece_8_the_entrys_lines_are_the_newest_three_read_only_as_they_grow() {
        let rendering = Identity::issue(IdentityKind::Job).unwrap();
        let waiting = Identity::issue(IdentityKind::Job).unwrap();
        let server = "server-p8";
        let holds = WhatThisRunHolds::default();
        let connection = holds.a_connection_begins(server);
        let mut clock = 1_790_086_400u64;
        let mut read_at = |sinks: &smoozy_queue_history::readings::TheSinksOfARenderingJob| {
            let request = the_queue_reading_request().unwrap();
            let state = the_queue_reading_in(WhatCameBack::ItAnswered(
                Answered::to(&request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, "test").carrying(
                    a_queue_answer_with_figures(&rendering, &waiting, None, Some(sinks)),
                ),
            ))
            .expect("the answer reads");
            clock += 5;
            holds.a_reading_was_taken(
                server,
                connection,
                Reading::taken(
                    state,
                    SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(clock),
                    Clock::ThisComputer,
                ),
                None,
            );
        };
        // **Fifty lines, about 5 KB**, the last with a carriage return before
        // its end.
        let mut printed = Vec::new();
        for frame in 1..=49 {
            printed.extend_from_slice(format!("Fra:{frame} Mem:18.19G (Peak 19.02G) | Time:00:{frame:02}.10 | Sample {frame}/2048 | the long middle of a Cycles line\n").as_bytes());
        }
        printed.extend_from_slice(b"Fra:50 Saved: '/out/caf\xc3\xa9.png'\r\n");
        assert!(printed.len() > 4096);
        let ordinary = the_sinks_at(1, 0, 0).ordinary.path;
        let complaints = the_sinks_at(1, 0, 0).complaints.path;
        let mut machine = SinksInHand::default();
        machine.bytes.insert(ordinary.clone(), printed.clone());
        machine
            .bytes
            .insert(complaints.clone(), b"Error: one\n".to_vec());
        let length = printed.len() as u64;
        read_at(&the_sinks_at(1, length, 11));

        // **Not drawn, not read**: the queue is not on screen, or its entry is
        // not this job.
        assert!(!the_entrys_lines_are_read(&holds, server, &mut machine));
        assert_eq!(
            holds.the_entry_is_drawn_for(Some((server, &rendering.as_text()))),
            Vec::<String>::new()
        );
        assert!(
            !holds.the_entrys_lines_are_on_screen_for(server),
            "the queue itself is not on screen"
        );
        assert!(!the_entrys_lines_are_read(&holds, server, &mut machine));
        assert!(machine.asked.is_empty());
        holds.the_queue_is_shown(None);
        assert!(holds.the_entrys_lines_are_on_screen_for(server));
        assert!(!holds.the_entrys_lines_are_on_screen_for("another-server"));
        assert!(
            holds.the_commentary_is_on_screen_for(server),
            "P10d's third read is taken for the entry's lines with no panel open"
        );

        // **The first read is the last 4 KB**, and the line it began inside is
        // not shown.
        assert!(the_entrys_lines_are_read(&holds, server, &mut machine));
        assert_eq!(
            machine.asked,
            vec![(ordinary.clone(), length - THE_ENTRYS_READ_AT_MOST, length)]
        );
        assert_eq!(
            holds.the_entry_is_drawn_for(Some((server, &rendering.as_text()))),
            vec![
                "Fra:48 Mem:18.19G (Peak 19.02G) | Time:00:48.10 | Sample 48/2048 | the long middle of a Cycles line".to_string(),
                "Fra:49 Mem:18.19G (Peak 19.02G) | Time:00:49.10 | Sample 49/2048 | the long middle of a Cycles line".to_string(),
                "Fra:50 Saved: '/out/caf\u{e9}.png'".to_string(),
            ],
        );

        // **Nothing new: nothing asked.**
        read_at(&the_sinks_at(1, length, 11));
        assert!(!the_entrys_lines_are_read(&holds, server, &mut machine));
        assert_eq!(machine.asked.len(), 1);

        // **The next read asks only past the length**, and a line still being
        // printed waits for its end.
        printed.extend_from_slice(b"Fra:51 Sample 1/2048\nFra:51 Sam");
        machine.bytes.insert(ordinary.clone(), printed.clone());
        read_at(&the_sinks_at(1, printed.len() as u64, 11));
        assert!(the_entrys_lines_are_read(&holds, server, &mut machine));
        assert_eq!(
            machine.asked.last(),
            Some(&(ordinary.clone(), length, printed.len() as u64))
        );
        let three = holds.the_entry_is_drawn_for(Some((server, &rendering.as_text())));
        assert_eq!(three.len(), 3);
        assert_eq!(three[2], "Fra:51 Sample 1/2048");
        let before = printed.len() as u64;
        printed.extend_from_slice(b"ple 2/2048\n");
        machine.bytes.insert(ordinary.clone(), printed.clone());
        read_at(&the_sinks_at(1, printed.len() as u64, 11));
        assert!(the_entrys_lines_are_read(&holds, server, &mut machine));
        assert_eq!(
            machine.asked.last(),
            Some(&(ordinary.clone(), before, printed.len() as u64))
        );
        assert_eq!(
            holds.the_entry_is_drawn_for(Some((server, &rendering.as_text()))),
            vec![
                "Fra:50 Saved: '/out/caf\u{e9}.png'".to_string(),
                "Fra:51 Sample 1/2048".to_string(),
                "Fra:51 Sample 2/2048".to_string(),
            ],
        );
        assert!(
            machine.asked.iter().all(|(path, _, _)| *path == ordinary),
            "the errors stream is never read for the entry's lines"
        );

        // **More than one read takes since the last: the newest 4 KB**, begun
        // inside a line, never the whole gap.
        let gap = printed.len() as u64;
        for frame in 52..=150 {
            printed.extend_from_slice(
                format!("Fra:{frame} Sample 1/2048 | a line long enough to fill the gap\n")
                    .as_bytes(),
            );
        }
        machine.bytes.insert(ordinary.clone(), printed.clone());
        read_at(&the_sinks_at(1, printed.len() as u64, 11));
        assert!(printed.len() as u64 - gap > THE_ENTRYS_READ_AT_MOST);
        assert!(the_entrys_lines_are_read(&holds, server, &mut machine));
        assert_eq!(
            machine.asked.last(),
            Some(&(
                ordinary.clone(),
                printed.len() as u64 - THE_ENTRYS_READ_AT_MOST,
                printed.len() as u64
            ))
        );
        assert_eq!(
            holds.the_entry_is_drawn_for(Some((server, &rendering.as_text())))[2],
            "Fra:150 Sample 1/2048 | a line long enough to fill the gap"
        );

        // **A new attempt's sink starts the lines over**, from its own start.
        let again = the_sinks_at(2, 0, 0).ordinary.path;
        machine
            .bytes
            .insert(again.clone(), b"Fra:1 again\n".to_vec());
        read_at(&the_sinks_at(2, 12, 0));
        assert!(the_entrys_lines_are_read(&holds, server, &mut machine));
        assert_eq!(machine.asked.last(), Some(&(again, 0, 12)));
        assert_eq!(
            holds.the_entry_is_drawn_for(Some((server, &rendering.as_text()))),
            vec!["Fra:1 again".to_string()]
        );

        // **Long lines: the last 4 KB holds two whole lines and the end of a
        // third**, and the end of a line is never shown as a line.
        let long = the_sinks_at(3, 0, 0).ordinary.path;
        let a_long_line = |frame: u32| format!("Fra:{frame} {}\n", "x".repeat(1_800));
        let three: String = (1..=3).map(a_long_line).collect();
        machine
            .bytes
            .insert(long.clone(), three.clone().into_bytes());
        read_at(&the_sinks_at(3, three.len() as u64, 0));
        assert!(the_entrys_lines_are_read(&holds, server, &mut machine));
        assert_eq!(
            holds.the_entry_is_drawn_for(Some((server, &rendering.as_text()))),
            vec![
                a_long_line(2).trim_end().to_string(),
                a_long_line(3).trim_end().to_string()
            ]
        );

        // **A new attempt a few bytes longer than the last one's read is
        // still read from its own end**, never carried on from the last one's
        // place.
        let fourth = the_sinks_at(4, 0, 0).ordinary.path;
        let another_line = |frame: u32| format!("Fra:{frame} {}\n", "y".repeat(1_802));
        let longer: String = (1..=3).map(another_line).collect();
        assert!(
            longer.len() > three.len()
                && longer.len() - three.len() < THE_ENTRYS_READ_AT_MOST as usize
        );
        machine
            .bytes
            .insert(fourth.clone(), longer.clone().into_bytes());
        read_at(&the_sinks_at(4, longer.len() as u64, 0));
        assert!(the_entrys_lines_are_read(&holds, server, &mut machine));
        assert_eq!(
            machine.asked.last(),
            Some(&(
                fourth,
                longer.len() as u64 - THE_ENTRYS_READ_AT_MOST,
                longer.len() as u64
            ))
        );
        assert_eq!(
            holds.the_entry_is_drawn_for(Some((server, &rendering.as_text()))),
            vec![
                another_line(2).trim_end().to_string(),
                another_line(3).trim_end().to_string()
            ]
        );

        // **Another entry drawn, or the queue put away: the reads stop**, and
        // the next entry starts over with nothing held.
        assert_eq!(holds.the_entry_is_drawn_for(None), Vec::<String>::new());
        assert!(!holds.the_entrys_lines_are_on_screen_for(server));
        assert!(!holds.the_commentary_is_on_screen_for(server));
        let asked = machine.asked.len();
        assert!(!the_entrys_lines_are_read(&holds, server, &mut machine));
        assert_eq!(machine.asked.len(), asked);
        holds.the_entry_is_drawn_for(Some((server, &rendering.as_text())));
        assert!(
            the_entrys_lines_are_read(&holds, server, &mut machine),
            "drawn again, read again"
        );
        let asked = machine.asked.len();
        holds.the_queue_is_put_away();
        assert!(!holds.the_entrys_lines_are_on_screen_for(server));
        assert!(!the_entrys_lines_are_read(&holds, server, &mut machine));
        assert_eq!(machine.asked.len(), asked);
        assert_eq!(
            holds.the_entry_is_drawn_for(Some((server, &rendering.as_text()))),
            Vec::<String>::new(),
            "the lines went with the queue"
        );

        // **Not updating: nothing is read.**
        holds.the_queue_is_shown(None);
        holds.the_connection_ended(server);
        assert!(!the_entrys_lines_are_read(&holds, server, &mut machine));
        assert_eq!(machine.asked.len(), asked);
    }

    // ── The 38's piece 10: `Queue & History` N9's two routes ─────────────────

    /// **A machine whose job record directory holds files**, answering every
    /// request as [`AMachineInHand`] does.
    pub(crate) struct AMachineWithItsFiles {
        supervisor: AMachineInHand,
        pub(crate) files: HashMap<String, Vec<u8>>,
    }

    impl AsksTheSupervisor for AMachineWithItsFiles {
        fn ask(&mut self, request: &Request) -> WhatCameBack {
            self.supervisor.ask(request)
        }
        fn a_file_of_the_job_record_directory(&mut self, path: &str) -> Option<Vec<u8>> {
            self.files.get(path).cloned()
        }
    }

    const THE_ROOT: &str = "/ephemeral/smoozy";

    /// **A server whose row records its machine root**, so the job record
    /// directory can be composed (`Job files and outputs` N3).
    fn a_server_with_its_root() -> (ARecordInHand, Identity) {
        let (record, server) = a_record_with_a_server();
        let mut row = server_rows::one(&record.0, &server.as_text())
            .unwrap()
            .unwrap();
        row.recorded_machine_root = Some(smoozy_servers::store::RecordedMachineRoot {
            path: THE_ROOT.into(),
            the_package_areas_own_location: None,
            read_under: "inst-1".into(),
        });
        server_rows::write(&record.0, &row).unwrap();
        (record, server)
    }

    /// **A job submitted without *Read the file*, its render started**, with
    /// `attempts` attempts on its note.
    fn a_job_submitted_unread(
        record: &ARecordInHand,
        server: &Identity,
        label: &str,
        attempts: u32,
    ) -> Identity {
        let mut made = a_record_in_history(server, label);
        made.what_the_note_carried.attempts = attempts;
        write(record, &made);
        made.job
    }

    fn the_paths_of(job: &Identity, label: &str) -> smoozy_job_files::publication::AJobsPaths {
        let layout = smoozy_job_files::layout::TheLayout::of(
            smoozy_job_files::layout::MachineRoot::chosen(THE_ROOT).unwrap(),
            smoozy_job_files::layout::WhereThePackageAreaSits::UnderTheRoot,
        );
        smoozy_job_files::publication::AJobsPaths::composed(&layout, job, Some(label)).unwrap()
    }

    /// **What the render worker writes at its step 4** — the scene's output
    /// path, one PNG a frame, on the scene the reading names (`Job files and
    /// outputs` H9), composed by that crate's own composition.
    pub(crate) fn a_recorded_reading(
        job_output_folder: &str,
        frames: &[i64],
        views: smoozy_job_files::reading::HowManyViewsTheSceneWrites,
    ) -> smoozy_job_files::composing::TheReadingAndWhatFollows {
        use smoozy_job_files::composing::{
            the_reading_and_everything_derived_from, WhatTheReadingIsComposedAgainst,
        };
        use smoozy_job_files::main_output::WhereTheRenderEnters;
        use smoozy_job_files::manifest::CarriedAndNotInterpretedHere;
        use smoozy_job_files::reading::{
            ArtistsText, AtOneFrame, Destination, DestinationKind, Format, GatingConditions,
            RecordedLocation, TheVersionInHand, WhereItComesFrom, WhichScene,
        };
        use smoozy_job_files::reading_script::{TheObservedOutputs, ThePrintedReading};
        let layer = || ArtistsText::of(b"ViewLayer".to_vec());
        let gating = GatingConditions {
            the_effects_on_for_render: None,
            compositing_switch: true,
            a_compositing_tree_is_present: false,
            a_live_final_output_node: false,
            the_final_output_node_is_muted: false,
            four_fives_use_nodes_switch: None,
            save_output_switch: None,
            the_render_goes_out_through_a_sequencer_holding_a_scene_strip: false,
        };
        let printed = ThePrintedReading {
            outputs: TheObservedOutputs {
                taken_by: TheVersionInHand::of(5, 2),
                gating: gating.clone(),
                destinations: vec![Destination {
                    kind: DestinationKind::TheRegularOutputPath,
                    recorded_location: Some(RecordedLocation::of(b"//out/f_".to_vec())),
                    format: Some(Format {
                        as_the_file_records_it: ArtistsText::of(b"PNG".to_vec()),
                        the_ending_it_writes: Some(ArtistsText::of(b".png".to_vec())),
                        is_video_shaped: false,
                    }),
                    origin: WhereItComesFrom::TheScenesOutputPath,
                    scene: WhichScene::TheJobsOwnScene,
                    the_view_layers_that_feed_it: vec![layer()],
                    the_location_with_its_folder_replaced: None,
                }],
                scenes_reached_through_a_sequencer_strip: Vec::new(),
                the_view_transform: None,
                the_tree_is_shared_with: None,
                per_frame: frames
                    .iter()
                    .map(|frame| AtOneFrame {
                        frame: *frame,
                        view_layers_that_render: vec![layer()],
                        gating: gating.clone(),
                        the_regular_output_paths_answered_path: Some(RecordedLocation::of(
                            format!("/artist/out/f_{frame:04}.png").into_bytes(),
                        )),
                    })
                    .collect(),
                recorded_locations: Vec::new(),
                views,
                the_compositing_tree: None,
            },
            settings: CarriedAndNotInterpretedHere(serde_json::json!({})),
            dependencies: CarriedAndNotInterpretedHere(serde_json::json!({})),
            errors: std::collections::BTreeMap::new(),
            the_baked_caches_on_the_machine: Vec::new(),
            the_scene_holds_a_simulation_not_baked: false,
            the_scene_it_read: Some(ArtistsText::of(b"Interior".to_vec())),
            the_open_files_path: None,
            the_check_is_written_as_a_picture: false,
        };
        let enters = WhereTheRenderEnters {
            scene: ArtistsText::of(b"Interior".to_vec()),
            view_layer: ArtistsText::of(b"ViewLayer".to_vec()),
        };
        the_reading_and_everything_derived_from(
            printed,
            WhatTheReadingIsComposedAgainst {
                the_folder_the_blend_sits_in: b"/artist",
                job_output_folder,
                enters_at: &enters,
                the_jobs_pins: &[],
            },
        )
        .expect("a reading")
    }

    fn an_attempt_record(
        number: u32,
        frames: &[i64],
        the_file_changed: bool,
    ) -> smoozy_job_files::attempts::AttemptRecord {
        smoozy_job_files::attempts::AttemptRecord {
            number,
            frame_list_in_the_order_it_was_written: frames.to_vec(),
            map_in_force: smoozy_job_files::attempts::TheMapInForce::NoneWasComposedForThisAttempt,
            file_identity_at_this_attempt: smoozy_server_protocol::transfers::SourceReading {
                size: 1,
                inode: "1".into(),
                modification_time: 1,
                change_time: number as i64,
            },
            the_file_changed_before_this_attempt: the_file_changed,
            a_frame_an_earlier_attempt_wrote_was_kept: false,
        }
    }

    /// **The machine's job record directory for one job past its render's
    /// step 4**: the manifest (no reading), each attempt's recorded reading
    /// and each attempt record, under the one directory both ends compose.
    pub(crate) fn its_record_directory(
        files: &mut HashMap<String, Vec<u8>>,
        job: &Identity,
        label: &str,
        frames: &[i64],
        attempts: u32,
    ) -> String {
        use smoozy_job_files::manifest::{
            the_recorded_readings_file_name, AnchorState, CarriedAndNotInterpretedHere, FrameRange,
            SceneField, SubmissionManifest, TheBlenderToRenderBy, TheCardMakeAtSubmission,
            TheRecordedReading, MANIFEST_FILE_NAME,
        };
        let paths = the_paths_of(job, label);
        let manifest = SubmissionManifest::composed(
            job,
            None,
            TheBlenderToRenderBy {
                version: "5.2.2".into(),
                build_path: "/opt/blender".into(),
            },
            SceneField {
                name: String::new(),
                anchor: AnchorState::ToAnchor {
                    provenance: "none".into(),
                },
            },
            FrameRange::TheFilesOwn,
            CarriedAndNotInterpretedHere(serde_json::json!({})),
            CarriedAndNotInterpretedHere(serde_json::json!({})),
            CarriedAndNotInterpretedHere(serde_json::json!({})),
            "/artist/scene.blend".into(),
            smoozy_server_protocol::transfers::SourceReading {
                size: 1,
                inode: "1".into(),
                modification_time: 1,
                change_time: 1,
            },
            TheRecordedReading::ThereIsNone,
            TheCardMakeAtSubmission::NoReadingStood,
            smoozy_job_files::manifest::WhereTheJobOutputFolderIs::named_by(&paths),
        );
        let under = |name: &str| format!("{}/{name}", paths.job_record_directory);
        files.insert(under(MANIFEST_FILE_NAME), manifest.as_written());
        for attempt in 1..=attempts {
            files.insert(
                under(&the_recorded_readings_file_name(attempt)),
                serde_json::to_vec(&a_recorded_reading(
                    &paths.job_output_folder,
                    frames,
                    smoozy_job_files::reading::HowManyViewsTheSceneWrites::One,
                ))
                .unwrap(),
            );
            let record = an_attempt_record(
                attempt,
                if attempt == 1 { frames } else { &frames[..1] },
                attempt > 1,
            );
            files.insert(under(&record.file_name()), record.as_written());
        }
        paths.job_output_folder
    }

    // ── The 38's piece 15: the catch-up notice, kept ─────────────────────────

    /// **A read forward whose events carry the machine's own words**, then
    /// nothing more.
    fn a_journal_saying(from: u64, said: &[(&str, &Identity, &str)]) -> Fields {
        let mut fields = Fields::none()
            .and(HOW_MANY_EVENTS_FIELD, said.len().to_string())
            .and(
                THE_NEXT_SEQUENCE_FIELD,
                (from + said.len() as u64).to_string(),
            )
            .and(THE_OLDEST_RETAINED_FIELD, "0");
        for (at, (kind, about, says)) in said.iter().enumerate() {
            fields = fields
                .and(
                    &the_field_of_item(AN_EVENT, at, THE_SEQUENCE_FIELD),
                    (from + at as u64).to_string(),
                )
                .and(&the_field_of_item(AN_EVENT, at, THE_KIND_FIELD), *kind)
                .and(
                    &the_field_of_item(AN_EVENT, at, THE_ABOUT_FIELD),
                    about.as_text(),
                )
                .and(&the_field_of_item(AN_EVENT, at, THE_SAYS_FIELD), *says);
        }
        fields
    }

    /// **`Queue & History` C1 and `Queue & History` N1: a job interrupted and re-run from frame
    /// 61 is named so** — and the frame, written by the render worker after the
    /// start was journalled, is read on a later pass. **The first attempt's
    /// start names nothing**, and the line stands in the local record, so a
    /// new run reads it (`Queue & History` C2). The 38's piece 15.
    #[test]
    fn a_re_run_from_frame_61_is_named_and_its_frame_is_read_when_it_stands() {
        let (record, server) = a_server_with_its_root();
        let clay = a_job_submitted_unread(&record, &server, "Kitchen — clay", 2);
        let journal = AMachineInHand::default()
            .answers(
                THE_READ_FORWARD_REQUEST,
                Some(a_journal_saying(
                    0,
                    &[
                        (THE_RENDER_ATTEMPT_STARTED_EVENT, &clay, "attempt 1"),
                        (THE_RENDER_ATTEMPT_STARTED_EVENT, &clay, "attempt 2"),
                    ],
                )),
            )
            .answers(THE_READ_FORWARD_REQUEST, Some(events(2, &[], Some(0))))
            .answers(THE_CURSOR_MOVES_FORWARD_REQUEST, Some(Fields::none()));
        let mut machine = AMachineWithItsFiles {
            supervisor: journal,
            files: HashMap::new(),
        };
        // **The first attempt's record stands from the start**, so a frame read
        // from any attempt but the re-run's own would be its 1.
        let first = an_attempt_record(1, &(1..=240).collect::<Vec<i64>>(), false);
        let directory = the_paths_of(&clay, "Kitchen — clay").job_record_directory;
        machine.files.insert(
            format!("{directory}/{}", first.file_name()),
            first.as_written(),
        );
        let holds = WhatThisRunHolds::default();
        assert_eq!(
            a_read_forward_pass(&server, &record, &mut machine, &holds),
            HowThePassEnded::ItReadToTheEnd { applied: 2 }
        );
        let account = the_account_of(&record, &server).expect("the re-run is named");
        assert_eq!(
            account.entries,
            vec![
                smoozy_queue_history::catch_up::ANoticeEntry::InterruptedAndReRunFrom {
                    job: clay.clone(),
                    attempt: 2,
                    frame: None,
                    sequence: 1,
                }
            ],
            "only the second attempt is a re-run, and its list was not written yet"
        );

        // **The render worker writes the re-run attempt's record**, and the
        // next pass — with nothing new in the journal — reads its first frame.
        let attempt = an_attempt_record(2, &(61..=240).collect::<Vec<i64>>(), false);
        machine.files.insert(
            format!("{directory}/{}", attempt.file_name()),
            attempt.as_written(),
        );
        a_read_forward_pass(&server, &record, &mut machine, &holds);
        let account = the_account_of(&record, &server).expect("still standing");
        assert!(
            matches!(
                &account.entries[..],
                [
                    smoozy_queue_history::catch_up::ANoticeEntry::InterruptedAndReRunFrom {
                        frame: Some(61),
                        attempt: 2,
                        ..
                    }
                ]
            ),
            "{:?}",
            account.entries
        );
        // **A frame once said is kept**: the record gone from the machine
        // changes nothing on the next pass.
        machine.files.clear();
        a_read_forward_pass(&server, &record, &mut machine, &holds);
        assert!(matches!(
            &the_account_of(&record, &server).unwrap().entries[..],
            [
                smoozy_queue_history::catch_up::ANoticeEntry::InterruptedAndReRunFrom {
                    frame: Some(61),
                    ..
                }
            ]
        ));
    }

    /// **`Queue & History` N1, on the job record Queue and History draw**: a
    /// re-run's mark comes off the machine's note with its frame absent, and
    /// the first frame of the re-run attempt's own list lands on it once that
    /// attempt's record stands. **A later fill-in from the note, frame still
    /// absent, does not take it away.** Step 12's reboot scenario, finding 2.
    #[test]
    fn the_job_records_re_run_mark_is_given_the_frame_the_re_run_proceeded_from() {
        use smoozy_queue_history::acting::note::AnAttemptMark;
        let (record, server) = a_server_with_its_root();
        let mut made = a_record_in_history(&server, "Kitchen — clay");
        made.what_the_note_carried.attempts = 2;
        made.what_the_note_carried.re_run_marks = vec![AnAttemptMark::for_a_re_run(2, &[])];
        write(&record, &made);
        let clay = made.job.clone();
        let the_mark = || {
            JobRecords::over(&record.0)
                .read(&clay)
                .unwrap()
                .unwrap()
                .what_the_note_carried
                .re_run_marks
        };
        let journal = AMachineInHand::default()
            .answers(THE_READ_FORWARD_REQUEST, Some(events(0, &[], Some(0))))
            .answers(THE_CURSOR_MOVES_FORWARD_REQUEST, Some(Fields::none()));
        let mut machine = AMachineWithItsFiles {
            supervisor: journal,
            files: HashMap::new(),
        };
        let directory = the_paths_of(&clay, "Kitchen — clay").job_record_directory;
        let first = an_attempt_record(1, &(1..=240).collect::<Vec<i64>>(), false);
        machine.files.insert(
            format!("{directory}/{}", first.file_name()),
            first.as_written(),
        );
        let holds = WhatThisRunHolds::default();

        // **The render worker has not written the re-run's record yet**, so
        // nothing is guessed — and the first attempt's 1 is not taken for it.
        a_read_forward_pass(&server, &record, &mut machine, &holds);
        assert_eq!(
            the_mark(),
            vec![AnAttemptMark {
                attempt: 2,
                the_re_run_proceeded_from: None
            }]
        );

        let attempt = an_attempt_record(2, &(61..=240).collect::<Vec<i64>>(), false);
        machine.files.insert(
            format!("{directory}/{}", attempt.file_name()),
            attempt.as_written(),
        );
        a_read_forward_pass(&server, &record, &mut machine, &holds);
        assert_eq!(
            the_mark(),
            vec![AnAttemptMark {
                attempt: 2,
                the_re_run_proceeded_from: Some(61)
            }]
        );

        // **The note's mark carried again, frame absent**, as every fill-in
        // carries it: the frame stays.
        let mut note = TheNote::created_with_the_job_record("Kitchen — clay");
        note.attempts = 2;
        note.re_run_marks = vec![AnAttemptMark::for_a_re_run(2, &[])];
        JobRecords::over(&record.0)
            .the_fill_in(&clay, &note)
            .unwrap();
        assert_eq!(
            the_mark(),
            vec![AnAttemptMark {
                attempt: 2,
                the_re_run_proceeded_from: Some(61)
            }]
        );

        // **A frame once known is kept**: the record gone from the machine
        // changes nothing, and nothing is read for a mark that has its frame.
        machine.files.clear();
        a_read_forward_pass(&server, &record, &mut machine, &holds);
        assert_eq!(
            the_mark(),
            vec![AnAttemptMark {
                attempt: 2,
                the_re_run_proceeded_from: Some(61)
            }]
        );
    }

    /// **`Queue & History` C2: a notice the record would not take moves no
    /// cursor**, so the events it names are read again. The 38's piece 15.
    #[test]
    fn a_notice_the_record_refuses_moves_no_cursor() {
        let (record, server) = a_record_with_a_server();
        let mut confirmed = a_provisional_record(&server, "Kitchen");
        confirmed.standing = Standing::Confirmed;
        write(&record, &confirmed);
        record
            .0
            .execute("DROP TABLE catch_up_notice_entry", [])
            .unwrap();
        let mut machine = AMachineInHand::default()
            .answers(
                THE_READ_FORWARD_REQUEST,
                Some(events(
                    0,
                    &[(THE_RECORDED_OUTCOME_WRITTEN_EVENT, Some(&confirmed.job))],
                    Some(0),
                )),
            )
            .answers(
                THE_PER_JOB_NOTE_READING_REQUEST,
                Some(a_note_that_started_and_finished("Kitchen")),
            )
            .answers(THE_CURSOR_MOVES_FORWARD_REQUEST, Some(Fields::none()));
        assert_eq!(
            a_read_forward_pass(&server, &record, &mut machine, &WhatThisRunHolds::default()),
            HowThePassEnded::TheReadCouldNotComplete { applied: 0 }
        );
        assert_eq!(
            the_cursor_of(&record, &server),
            None,
            "the cursor moved past an account nobody kept"
        );
    }

    /// **The machine's half of what [`the_attempt_a_start_names`] reads**: a
    /// render attempt's start is journalled about the job, saying *attempt N*.
    /// This fails if the words move.
    #[test]
    fn the_machine_names_an_attempt_the_way_it_is_read_here() {
        let program = include_str!("../../../Rented-server runtime — program/src/main.rs");
        assert!(program.contains(
            "asked::THE_RENDER_ATTEMPT_STARTED_EVENT,\n                Some(job.clone()),\n                &format!(\"attempt {attempt}\"),"
        ));
        assert_eq!(the_attempt_a_start_names(Some("attempt 2")), Some(2));
        assert_eq!(the_attempt_a_start_names(Some("attempt 1")), Some(1));
        assert_eq!(the_attempt_a_start_names(Some("attempt two")), None);
        assert_eq!(the_attempt_a_start_names(None), None);
    }

    /// **`Rendering` A4's read of the attempt record, out of the job's record
    /// directory** (the 38's piece 2): the latest attempt record that carries
    /// a frame list, the inventory for that attempt's own recorded reading,
    /// and the folder the manifest names. **No attempt record, no read.**
    #[test]
    fn the_map_in_force_is_the_latest_attempt_record_with_a_frame_list() {
        let job = Identity::issue(IdentityKind::Job).unwrap();
        let mut files = HashMap::new();
        let folder = its_record_directory(&mut files, &job, "Kitchen", &[1, 2, 3], 2);
        let mut machine = AMachineWithItsFiles {
            supervisor: AMachineInHand::default(),
            files,
        };
        let directory = the_paths_of(&job, "Kitchen").job_record_directory;
        let (read, the_folder) =
            the_map_in_force_on_the_machine(&mut machine, &directory, 2).expect("a read");
        // **The second attempt's list**, which the fixture writes as `[1]`.
        assert_eq!(read.the_frame_list_asked_for, vec![1]);
        assert!(matches!(
            read.the_inventory,
            smoozy_job_files::inventory::WhatTheInventoryIs::It(_)
        ));
        assert_eq!(the_folder, Some(folder));
        // Read as of one attempt, the first attempt's whole list.
        let (read, _) = the_map_in_force_on_the_machine(&mut machine, &directory, 1).unwrap();
        assert_eq!(read.the_frame_list_asked_for, vec![1, 2, 3]);
        assert!(the_map_in_force_on_the_machine(&mut machine, &directory, 0).is_none());
    }

    /// **The 38's piece 10, item 10 — `Queue & History` N9's first route.** A job past its
    /// render's step 4 has its reading filled into the record on the next
    /// connection: *Writing to* the job's own folder (the regular output path
    /// is no node destination, `Job files and outputs` B8), the scene the
    /// reading names, the asked order — and its six things held for the queue
    /// entry. **A job before its step 4 has none**, and nothing is made up for
    /// it.
    #[test]
    fn piece_10_a_job_past_its_step_4_has_its_reading_filled_in_on_the_next_connection() {
        let (record, server) = a_server_with_its_root();
        let read = a_job_submitted_unread(&record, &server, "Kitchen", 1);
        let not_yet = a_job_submitted_unread(&record, &server, "Hall", 1);
        let mut files = HashMap::new();
        let folder = its_record_directory(&mut files, &read, "Kitchen", &[1, 2, 3], 1);
        let mut machine = AMachineWithItsFiles {
            supervisor: AMachineInHand::default(),
            files,
        };
        let holds = WhatThisRunHolds::default();
        the_step_seven(&server, 1, &record, &mut machine, &holds, &nothing_is_live);

        let filled = JobRecords::over(&record.0).read(&read).unwrap().unwrap();
        assert_eq!(
            filled.main_output_from_the_read.as_deref(),
            Some(folder.as_str())
        );
        assert_eq!(filled.scene_name.as_deref(), Some("Interior"));
        assert!(
            filled.submitted_with_no_scene_named,
            "the no-scene mark stays"
        );
        assert_eq!(
            filled.the_frames.asked,
            smoozy_queue_history::record::TheFramesAsked::from_the_order(&[1, 2, 3])
        );
        let says = holds
            .what_the_reading_says(&read.as_text())
            .expect("the six things are held");
        assert_eq!(says.how_many_files, Some(3));
        assert!(!says.six.the_folders_it_is_writing_into.is_empty());

        let none = JobRecords::over(&record.0).read(&not_yet).unwrap().unwrap();
        assert_eq!(none.main_output_from_the_read, None);
        assert_eq!(none.scene_name, None);
        assert_eq!(
            none.the_frames,
            smoozy_queue_history::record::TheFramesTheReadFound::default()
        );
        assert!(holds.what_the_reading_says(&not_yet.as_text()).is_none());
    }

    /// **`Job files and outputs` P21 on `Queue & History` N9's first route**: a job whose folder
    /// is chosen at render start, the render worker having chosen it where the
    /// file points and recorded it beside the reading, has **that folder**
    /// filled into the record and the entry's *Writing to* — never the job
    /// area's composition. **Before the worker records one, nothing is filled
    /// in**: the job has no folder (`Queue & History` Y8). The Creator's
    /// second notes, piece 4.
    #[test]
    fn a_folder_chosen_at_render_start_reaches_the_record_and_nothing_before_it() {
        use smoozy_job_files::where_the_folder_sits::{
            WhereItIs, WhereTheJobOutputFolderSits, THE_RECORD_OF_WHERE_THE_FOLDER_SITS,
        };
        let (record, server) = a_server_with_its_root();
        let job = a_job_submitted_unread(&record, &server, "Kitchen", 1);
        let mut held = JobRecords::over(&record.0).read(&job).unwrap().unwrap();
        held.the_job_output_folder =
            smoozy_queue_history::record::TheJobOutputFolderTheRecordHolds::ChosenAtRenderStart;
        JobRecords::over(&record.0).remove(&job).unwrap();
        write(&record, &held);
        let mut files = HashMap::new();
        its_record_directory(&mut files, &job, "Kitchen", &[1, 2, 3], 1);
        // **The manifest as this desktop now writes it**: chosen at render start.
        let directory = the_paths_of(&job, "Kitchen").job_record_directory;
        let at = format!(
            "{directory}/{}",
            smoozy_job_files::manifest::MANIFEST_FILE_NAME
        );
        let mut manifest =
            smoozy_job_files::manifest::SubmissionManifest::read_back(&files[&at]).unwrap();
        manifest.job_output_folder =
            smoozy_job_files::manifest::WhereTheJobOutputFolderIs::ChosenAtRenderStart {
                the_folders_name: "job-Kitchen".into(),
                the_servers_fallback_folder_at_submission: None,
            };
        files.insert(at, manifest.as_written());
        let mut machine = AMachineWithItsFiles {
            supervisor: AMachineInHand::default(),
            files,
        };
        let holds = WhatThisRunHolds::default();
        the_step_seven(&server, 1, &record, &mut machine, &holds, &nothing_is_live);
        let before = JobRecords::over(&record.0).read(&job).unwrap().unwrap();
        assert_eq!(
            before.main_output_from_the_read, None,
            "no folder, nothing filled in"
        );
        assert_eq!(
            before.the_job_output_folder,
            smoozy_queue_history::record::TheJobOutputFolderTheRecordHolds::ChosenAtRenderStart
        );

        let chosen = WhereTheJobOutputFolderSits {
            path: "/home/ubuntu/Render/job-Kitchen".into(),
            where_it_is: WhereItIs::TheFallback {
                why: Some(smoozy_job_files::where_the_folder_sits::WhyTheFilesOwnPlaceWasNotUsed::ItIsAnotherComputers),
                the_servers_fallback_folder_was_passed_over: true,
            },
        };
        machine.files.insert(
            format!("{directory}/{THE_RECORD_OF_WHERE_THE_FOLDER_SITS}"),
            chosen.as_written(),
        );
        the_step_seven(&server, 1, &record, &mut machine, &holds, &nothing_is_live);
        let filled = JobRecords::over(&record.0).read(&job).unwrap().unwrap();
        assert_eq!(
            filled.main_output_from_the_read.as_deref(),
            Some("/home/ubuntu/Render/job-Kitchen")
        );
        assert_eq!(
            filled.the_job_output_folder,
            smoozy_queue_history::record::TheJobOutputFolderTheRecordHolds::It(chosen),
            "the case and the passed-over mark reach the record"
        );
        let says = holds
            .what_the_reading_says(&job.as_text())
            .expect("the six things are held");
        assert_eq!(says.job_output_folder, "/home/ubuntu/Render/job-Kitchen");
    }

    /// **The Creator's second notes, piece 6b: the fill-in keeps, per frame,
    /// whether more than one View Layer renders there, with the job's pins
    /// applied** — the reading's per-frame set through `Job files and outputs`'
    /// one rule, the pins read off the kept setup. Frame 1 renders one layer;
    /// frame 2 renders two, one of them pinned off; frame 3 renders two.
    ///
    /// **What it bites on**: nothing kept (the state before this piece), the
    /// pins not applied (frame 2 would read *more than one*), and the count
    /// taken before the pins rather than after.
    #[test]
    fn piece_6b2_the_fill_in_keeps_which_frames_render_more_than_one_view_layer() {
        use smoozy_job_files::composing::TheReadingAndWhatFollows;
        use smoozy_job_files::manifest::the_recorded_readings_file_name;
        use smoozy_job_files::reading::ArtistsText;
        use smoozy_render_settings::names::{AnchorState, ExactName, Provenance};
        use smoozy_render_settings::payload::{Payload, VersionPick};
        use smoozy_render_settings::view_layers::PinEntry;
        let (record, server) = a_server_with_its_root();
        let job = a_job_submitted_unread(&record, &server, "Kitchen", 1);
        // **The kept setup pins `AO` off**, as Submit writes it.
        let held = JobRecords::over(&record.0).read(&job).unwrap().unwrap();
        JobRecords::over(&record.0).remove(&job).unwrap();
        let kept = Payload {
            prepare_simulations: true,
            version_pick: VersionPick {
                build_path: "/opt/blender".to_string(),
                version: "5.2.2".to_string(),
            },
            scene: None,
            frame_range: None,
            sample_count: None,
            resolution_percentage: None,
            engine: None,
            device: None,
            view_layer_pins: vec![PinEntry {
                layer: ExactName::as_the_reading_produced_it("AO"),
                anchor: AnchorState::ToAnchor {
                    provenance: Provenance::AKeptSetup {
                        the_old_job: "none".to_string(),
                    },
                },
                on: false,
            }],
            simplify: Default::default(),
            the_check_is_written_as_a_picture: false,
            the_check_writes_into: None,
        };
        JobRecords::over(&record.0)
            .write_the_provisional_record_and_its_kept_setup(&held, &kept)
            .unwrap();
        // **The reading the worker wrote**: frame 2 renders `AO` beside
        // `ViewLayer`, frame 3 renders `Hero` beside it.
        let mut files = HashMap::new();
        its_record_directory(&mut files, &job, "Kitchen", &[1, 2, 3], 1);
        let at = format!(
            "{}/{}",
            the_paths_of(&job, "Kitchen").job_record_directory,
            the_recorded_readings_file_name(1)
        );
        let mut reading: TheReadingAndWhatFollows = serde_json::from_slice(&files[&at]).unwrap();
        for one in reading.reading.outputs.per_frame.iter_mut() {
            match one.frame {
                2 => one
                    .view_layers_that_render
                    .push(ArtistsText::of(b"AO".to_vec())),
                3 => one
                    .view_layers_that_render
                    .push(ArtistsText::of(b"Hero".to_vec())),
                _ => {}
            }
        }
        files.insert(at, serde_json::to_vec(&reading).unwrap());
        let mut machine = AMachineWithItsFiles {
            supervisor: AMachineInHand::default(),
            files,
        };
        let holds = WhatThisRunHolds::default();
        the_step_seven(&server, 1, &record, &mut machine, &holds, &nothing_is_live);
        let says = holds
            .what_the_reading_says(&job.as_text())
            .expect("the reading is held");
        assert_eq!(
            says.more_than_one_view_layer_at,
            [(1, false), (2, false), (3, true)].into_iter().collect(),
        );
        assert!(!says.the_view_layer_is_named_at(2));
        assert!(says.the_view_layer_is_named_at(3));
        assert!(
            says.the_view_layer_is_named_at(4),
            "a frame the reading does not cover"
        );
    }

    /// **The 38's piece 10, item 10 — `Queue & History` N9's second route.** On a pass inside a
    /// standing period, the journalled event *a recorded reading was written*
    /// fills the job's reading in; a pass with no such event reads no file.
    #[test]
    fn piece_10_the_journalled_event_fills_the_reading_in_on_a_pass() {
        use smoozy_server_protocol::requests::THE_RECORDED_READING_WRITTEN_EVENT;
        let (record, server) = a_server_with_its_root();
        let job = a_job_submitted_unread(&record, &server, "Kitchen", 1);
        let mut files = HashMap::new();
        let folder = its_record_directory(&mut files, &job, "Kitchen", &[1, 2], 1);
        let mut quiet = AMachineWithItsFiles {
            supervisor: AMachineInHand::default()
                .answers(THE_READ_FORWARD_REQUEST, Some(events(0, &[], Some(0)))),
            files: files.clone(),
        };
        let holds = WhatThisRunHolds::default();
        a_read_forward_pass(&server, &record, &mut quiet, &holds);
        assert_eq!(
            JobRecords::over(&record.0)
                .read(&job)
                .unwrap()
                .unwrap()
                .main_output_from_the_read,
            None
        );

        let mut written = AMachineWithItsFiles {
            supervisor: AMachineInHand::default()
                .answers(
                    THE_READ_FORWARD_REQUEST,
                    Some(events(
                        0,
                        &[(THE_RECORDED_READING_WRITTEN_EVENT, Some(&job))],
                        Some(0),
                    )),
                )
                .answers(THE_READ_FORWARD_REQUEST, Some(events(1, &[], Some(0)))),
            files,
        };
        a_read_forward_pass(&server, &record, &mut written, &holds);
        let filled = JobRecords::over(&record.0).read(&job).unwrap().unwrap();
        assert_eq!(
            filled.main_output_from_the_read.as_deref(),
            Some(folder.as_str())
        );
        assert!(holds.what_the_reading_says(&job.as_text()).is_some());
    }

    /// **The 38's piece 10, `Queue & History` N9a.** A re-run whose attempt
    /// record says the file changed lands that attempt's statement — and that
    /// no frame was kept, so the frames were rendered again — and the entry
    /// says so; the asked order is the first attempt's whole list, not the
    /// re-run's remainder.
    #[test]
    fn piece_10_a_re_run_whose_file_changed_says_so_and_that_frames_were_rendered_again() {
        use smoozy_queue_history::record::WhatBecameOfTheFramesAlreadyFinished;
        let (record, server) = a_server_with_its_root();
        let job = a_job_submitted_unread(&record, &server, "Kitchen", 2);
        let mut files = HashMap::new();
        its_record_directory(&mut files, &job, "Kitchen", &[1, 2, 3], 2);
        let mut machine = AMachineWithItsFiles {
            supervisor: AMachineInHand::default(),
            files,
        };
        the_readings_are_filled_in(
            &server,
            &record,
            &mut machine,
            &WhatThisRunHolds::default(),
            &[job.clone()],
        );
        let filled = JobRecords::over(&record.0).read(&job).unwrap().unwrap();
        assert_eq!(filled.attempts_where_the_file_changed.len(), 1);
        assert_eq!(filled.attempts_where_the_file_changed[0].attempt, 2);
        assert_eq!(
            filled.attempts_where_the_file_changed[0].what_became_of_the_frames_already_finished,
            WhatBecameOfTheFramesAlreadyFinished::TheyWereRenderedAgain
        );
        assert_eq!(
            filled
                .the_frames
                .asked
                .as_ref()
                .map(|asked| asked.how_many()),
            Some(3)
        );
        let entry = crate::surface::the_history::the_entry_for(
            &filled,
            &crate::surface::the_history::BesideTheRecord::default(),
            false,
            true,
        )
        .entry;
        let said = format!("{:?}", entry.marks);
        assert!(said.contains("The file changed before attempt"), "{said}");
        assert!(
            said.contains("Frames already finished were rendered again."),
            "{said}"
        );
    }

    /// **The 38's piece 3 — `Queue & History` P3, on the read forward**: a
    /// published preview about a job of this server puts one fetch on the job
    /// in the read's own application, before the cursor moves past it; read
    /// again, it puts nothing more (`State, identity and recovery` E4).
    #[test]
    fn seventeenth_catch_up_saves_every_frames_layers_before_advancing_the_cursor() {
        let (record, server) = a_record_with_a_server();
        let job = a_confirmed_record(&record, &server, "Kitchen").job;
        let kinds = vec![(THE_PREVIEW_PUBLISHED_EVENT, Some(&job)); 6];
        let mut carried = events(0, &kinds, Some(0));
        for frame in 1..=3 {
            let production = Identity::issue(IdentityKind::Operation).unwrap().as_text();
            for (place, layer) in [(0, "Beauty"), (1, "Mist")] {
                let published = smoozy_queue_history::previews::ThePublishedPreview {
                    frame,
                    operation: production.clone(),
                    view_layer: Some(layer.as_bytes().to_vec()),
                    main: place == 0,
                    place,
                    source: None,
                };
                let at = ((frame - 1) * 2) as usize + place as usize;
                carried = carried.and(
                    &the_field_of_item(AN_EVENT, at, THE_SAYS_FIELD),
                    published.as_says(),
                );
            }
        }
        let mut machine = AMachineInHand::default()
            .answers(THE_READ_FORWARD_REQUEST, Some(carried))
            .answers(THE_CURSOR_MOVES_FORWARD_REQUEST, Some(Fields::none()));
        assert_eq!(
            the_read_forward_pass(
                &server,
                &record,
                &mut machine,
                &WhatThisRunHolds::default(),
                HowItArrived::AtTheReconciliationRead
            ),
            HowThePassEnded::ItReadToTheEnd { applied: 6 }
        );
        assert_eq!(the_cursor_of(&record, &server), Some(6));
        let owed = crate::record::previews::the_owed_on(&record.0, &server).unwrap();
        assert_eq!(owed.len(), 6);
        for frame in 1..=3 {
            assert_eq!(owed.iter().filter(|one| one.frame == frame).count(), 2);
        }
    }

    #[test]
    fn piece_3_a_published_preview_puts_a_fetch_on_its_job_before_the_cursor_moves() {
        let (record, server) = a_record_with_a_server();
        let job = a_confirmed_record(&record, &server, "Kitchen").job;
        let says = smoozy_queue_history::previews::ThePublishedPreview {
            frame: 5,
            operation: Identity::issue(IdentityKind::Operation).unwrap().as_text(),
            view_layer: Some(b"Reflections".to_vec()),
            main: true,
            place: 0,
            source: None,
        }
        .as_says();
        let one = |from: u64| {
            events(from, &[(THE_PREVIEW_PUBLISHED_EVENT, Some(&job))], Some(0)).and(
                &the_field_of_item(AN_EVENT, 0, THE_SAYS_FIELD),
                says.clone(),
            )
        };
        for from in [0, 0] {
            let mut machine = AMachineInHand::default()
                .answers(THE_READ_FORWARD_REQUEST, Some(one(from)))
                .answers(THE_READ_FORWARD_REQUEST, Some(events(1, &[], Some(0))))
                .answers(THE_CURSOR_MOVES_FORWARD_REQUEST, Some(Fields::none()));
            let mut row = server_rows::one(&record.0, &server.as_text())
                .unwrap()
                .unwrap();
            row.event_cursor = Some(from);
            server_rows::write(&record.0, &row).unwrap();
            a_read_forward_pass(&server, &record, &mut machine, &WhatThisRunHolds::default());
            assert_eq!(the_cursor_of(&record, &server), Some(1));
        }
        let owed = crate::record::previews::the_owed_on(&record.0, &server).unwrap();
        assert_eq!(
            owed.len(),
            1,
            "one fetch for one derivative, however often its event is read"
        );
        assert_eq!(owed[0].frame, 5);
        assert_eq!(owed[0].view_layer.as_deref(), Some(&b"Reflections"[..]));
    }

    /// **The 38's piece 3 — `Desktop app and delivery` T15 and T17**: the
    /// finished notification carries the entry's own picture where a copy is
    /// held, and goes without one where none is.
    #[test]
    fn piece_3_the_finished_notification_carries_the_held_picture_or_none() {
        use crate::notifications::picture::ThePicture;
        let (record, server) = a_record_with_a_server();
        let job = a_confirmed_record(&record, &server, "Kitchen").job;
        let mut machine = AMachineInHand::default().answers(
            THE_PER_JOB_NOTE_READING_REQUEST,
            Some(a_note_that_started_and_finished("Kitchen")),
        );
        every_record_is_filled_in(&server, &record, &mut machine);
        let posted = |record: &ARecordInHand| {
            crate::servers::the_endings_posted::what_these_endings_post(
                record,
                &server,
                &[job.clone()],
            )
        };
        let before = posted(&record);
        assert_eq!(before.len(), 1);
        assert!(
            matches!(before[0].picture, ThePicture::None(_)),
            "{:?}",
            before[0].picture
        );

        let folder = std::env::temp_dir().join(format!(
            "smoozy-previews-{}",
            Identity::issue(IdentityKind::Operation).unwrap().as_text()
        ));
        std::fs::create_dir_all(&folder).unwrap();
        let copy = folder.join("held.png");
        std::fs::write(&copy, b"png").unwrap();
        crate::record::previews::a_derivative_is_owed(
            &record.0, &job, 3, "op", None, true, 0, None,
        )
        .unwrap();
        crate::record::previews::the_copy_was_published(
            &record.0,
            &job,
            3,
            None,
            "op",
            &copy,
            SystemTime::now(),
        )
        .unwrap();
        let after = posted(&record);
        assert_eq!(
            crate::notifications::picture::as_path(&after[0].picture),
            Some(copy.as_path()),
            "every system takes the held picture: Linux by its file address in a hint"
        );
        std::fs::remove_dir_all(&folder).unwrap();
    }
}

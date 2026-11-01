//! **`Queue & History` W4's re-check, given its ends** — the idle half of the
//! 38's piece 1 (2026-10-03): `Queue & History`'s two traits implemented over
//! this desktop's record and this server's point, the sequence that runs the
//! re-check, and the telling.
//!
//! **What is not here, because it exists.** The judgement — idle, the quiet
//! moment, the half hour on the machine's clock, the remaining interval, the
//! mark looked up before speaking and written by the act — is
//! `smoozy_queue_history::acting::idle_recheck::the_recheck`, whole. The words
//! are [`crate::notifications::kinds::what_an_idle_machine_posts`]. **This file
//! takes the reading, keeps the mark, waits, and posts.**
//!
//! # The shape, and it is `Desktop app and delivery` P10e's one instance
//!
//! P10e: *"A wake-up scheduled from a durable fact the application already
//! holds, which re-verifies against the machine before it says anything, and
//! which dies with the connected period it belongs to."* So:
//!
//! - **It is started when `Servers` Q1 step 7 completes**, on a thread of its
//!   own beside P10d's sequence and never inside it — P10e: *"It is not a
//!   live-reading sequence and does not join P10d's closed set."* Its requests
//!   are ordinary requests on the primary connection, under `Server protocol
//!   and transfers`' own admission point.
//! - **It re-takes the claim from the machine every time** (`Queue & History` W4): the queue
//!   reading for `Queue & History` W1, and the journal read forward from its start for `Queue & History` W2's
//!   quiet moment, with the machine's own now riding the same answer for `Queue & History` W3.
//! - **It waits for one of two things and nothing else**: the remaining
//!   interval `Queue & History` W4's arithmetic gives, where the machine is short of the half
//!   hour; or **a nudge** — an event applied by any read forward, which may be
//!   the one that left the line quiet, or the connection beginning or ending.
//!   **With nothing under way it waits on a nudge alone**, so a machine
//!   rendering for six hours is asked nothing by this file for six hours.
//! - **It ends with the connection it was started for** (`Queue & History` W4: *"Disconnected,
//!   nothing fires"*); the next connection's step 7 starts a fresh one, which
//!   is how a machine found already past the half hour is told about at once
//!   (`Queue & History` W3; the 38's map, hole 2, ruled as it leans: idle is a reading, not a
//!   journalled event, so `Desktop app and delivery` T18 does not reach it).
//!
//! **This file keeps no header list**: every other document's marker is
//! written with that document at its own site, because this crate's own plan
//! defines clauses of the same short names.

use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, Condvar, Mutex};
use std::time::{Duration, SystemTime};

use smoozy_queue_history::acting::idle_recheck::{
    the_recheck, AFreshReading, KeepsTheIdleMarks, ReadsTheMachineFresh, WhatTheRecheckDid,
};
use smoozy_queue_history::idle::{ARecordedEvent, IdleSpokenMark};
use smoozy_queue_history::line::{JobRecord, QueueState, SequenceKey};
use smoozy_server_protocol::framing::Fields;
use smoozy_server_protocol::requests::{
    AN_EVENT, HOW_MANY_EVENTS_FIELD, THE_CURSOR_FIELD, THE_EVENT_IDENTITY_FIELD, THE_HAPPENED_AT_FIELD,
    THE_HAPPENED_ON_CLOCK_FIELD, THE_HOLD_PLACED_EVENT, THE_JOB_RECORD_REMOVED_EVENT,
    THE_JOB_RECORD_WRITTEN_EVENT, THE_KIND_FIELD, THE_NEXT_SEQUENCE_FIELD, THE_READ_AT_FIELD,
    THE_READ_FORWARD_REQUEST, THE_READ_ON_CLOCK_FIELD, THE_RECORDED_OUTCOME_WRITTEN_EVENT,
    THE_RELEASE_EVENT, THE_RENDER_ATTEMPT_STARTED_EVENT, THE_SEQUENCE_FIELD,
};
use smoozy_state::identities::{Identity, IdentityKind};
use smoozy_state::observed::{Clock, MachineClock, Reading};
use tauri::{AppHandle, Manager, Runtime};

use crate::keys::LendsTheRecord;
use crate::notifications::kinds::what_an_idle_machine_posts;
use crate::notifications::{picture, APost, TheNotificationService};
use crate::record::{idle_marks, server_rows};
use crate::servers::a_period::ThePeriodsThisRun;
use crate::servers::sessions::ThisServersSessions;
use crate::servers::the_machines_work::{
    a_number, a_request, an_item, the_answer_to, the_queue_reading_from, with_the_record,
    AsksTheSupervisor, OverThisServersPoint,
};
use crate::start::Started;
use crate::typography::Verbatim;

// ════════════════════════════════════════════════════════════════════════════
// The mark, in the local record
// ════════════════════════════════════════════════════════════════════════════

/// **`Queue & History` W5's mark, kept in this desktop's local record** —
/// [`crate::record::idle_marks`], record version 33.
pub struct TheIdleMarksInTheRecord<'a> {
    pub record: &'a dyn LendsTheRecord,
}

impl KeepsTheIdleMarks for TheIdleMarksInTheRecord<'_> {
    fn the_marks_for(&self, server: &Identity) -> Result<Vec<IdleSpokenMark>, String> {
        match with_the_record(self.record, |connection| idle_marks::the_marks_for(connection, server)) {
            Some(Ok(marks)) => Ok(marks),
            Some(Err(failure)) => Err(format!("{failure:?}")),
            None => Err("the local record was not lent".to_string()),
        }
    }

    fn write_the_mark(&mut self, mark: &IdleSpokenMark) -> Result<(), String> {
        match with_the_record(self.record, |connection| idle_marks::the_mark_is_written(connection, mark)) {
            Some(Ok(())) => Ok(()),
            Some(Err(failure)) => Err(format!("{failure:?}")),
            None => Err("the local record was not lent".to_string()),
        }
    }
}

// ════════════════════════════════════════════════════════════════════════════
// The machine, read fresh
// ════════════════════════════════════════════════════════════════════════════

/// **The kinds of event that move a job in the machine's line**, and nothing
/// else — a job written, held, started, ended, released or removed. **A
/// reconciliation disposition is not among them**: the machine journals one
/// for a setup run as well as for a render, and a render's disposition that
/// moves its job is followed by that job's own start or ending, which are.
const THE_KINDS_THAT_MOVE_THE_LINE: [&str; 6] = [
    THE_JOB_RECORD_WRITTEN_EVENT,
    THE_HOLD_PLACED_EVENT,
    THE_RENDER_ATTEMPT_STARTED_EVENT,
    THE_RECORDED_OUTCOME_WRITTEN_EVENT,
    THE_RELEASE_EVENT,
    THE_JOB_RECORD_REMOVED_EVENT,
];

/// **The most reads one re-check makes of the journal.** The journal is capped
/// on the machine (`State, identity and recovery` E6) and one read carries
/// hundreds of events, so this is never reached by a journal that is read to
/// its end; it is what stops a machine that keeps answering with more from
/// holding the re-check for good. A Build figure.
const THE_MOST_READS_OF_THE_JOURNAL: usize = 64;

/// **`Queue & History` W1 to `Queue & History` W3's reading, taken through this server's point.**
///
/// **Every part is taken now** (`Queue & History` W4: the claim re-taken from the machine), and
/// **a part that cannot be taken is no reading at all** — `Queue & History` W4's *disconnected,
/// nothing fires*, which is also what a program older than this piece gives,
/// its read forward carrying no now.
pub struct TheMachineReadFresh<'a> {
    machine: RefCell<&'a mut dyn AsksTheSupervisor>,
    /// **`Queue & History` W5's key's first half: the installation identity the server row
    /// records as last confirmed** — step 7 runs only on the confirmed branch,
    /// so it is the installation this connection reached.
    installation: Identity,
}

impl<'a> TheMachineReadFresh<'a> {
    pub fn over(machine: &'a mut dyn AsksTheSupervisor, installation: Identity) -> Self {
        TheMachineReadFresh {
            machine: RefCell::new(machine),
            installation,
        }
    }
}

impl ReadsTheMachineFresh for TheMachineReadFresh<'_> {
    fn take_a_fresh_reading(&self, _server: &Identity) -> Option<AFreshReading> {
        let mut machine = self.machine.borrow_mut();
        // **`Queue & History` W1: the line and the render operation, from `Queue & History` A1's
        // one request.** Untracked work counts as work: the reading is
        // the machine's whole working state.
        let state = the_queue_reading_from(&mut **machine).ok()?;
        let (retained, machine_now) = the_journal_as_it_stands(&mut **machine)?;
        let a_render_operation_is_running = state
            .jobs
            .iter()
            .any(|job| matches!(job.state, QueueState::Rendering { .. }));
        Some(AFreshReading {
            machine_now,
            installation: self.installation.clone(),
            a_render_operation_is_running,
            records: the_line_as_records(&state.jobs),
            retained,
        })
    }
}

/// **The reading's jobs, in the shape `Queue & History` W1's judgement reads.** A waiting job is
/// in the line, a held one is not (`Queue & History` Q5) — which is all `Queue & History` W1
/// asks of it; the place given is the reading's own order, and nothing reads
/// it as anything more.
fn the_line_as_records(
    jobs: &[smoozy_queue_history::readings::AJobAsTheMachineReadsIt],
) -> Vec<JobRecord> {
    jobs.iter()
        .enumerate()
        .map(|(at, job)| JobRecord {
            job: job.job.clone(),
            group: None,
            sequence_key: matches!(job.state, QueueState::Waiting).then(|| SequenceKey::read_back(at as u64 + 1)),
            state: job.state.clone(),
            record_directory: String::new(),
            removed_build: None,
            reporting_choice_now: None,
        })
        .collect()
}

/// **`Queue & History` W2 and `Queue & History` W3: every event the machine's journal retains, with its moment,
/// and the machine's own now** — read forward from the journal's start, which
/// is a reading that advances nothing (`State, identity and recovery` E3).
///
/// **Which events left the line without startable work.** `Queue & History` W2's rule is *"the
/// most recent recorded event that left the line without startable work"*, and
/// this is only asked of a machine `Queue & History` W1 has just found idle. **So the most
/// recent event that moved a job in the line is that event**: nothing moved
/// the line after it, and the line is quiet now. Each event of a kind in
/// [`THE_KINDS_THAT_MOVE_THE_LINE`] is marked; anything else, a setup run's
/// among them, is not, and dates nothing but `Queue & History` W2's floor. **An event that does not read back whole
/// makes no reading**: leaving it out could make the quiet moment earlier than
/// it is, and the statement wrong.
fn the_journal_as_it_stands(
    machine: &mut dyn AsksTheSupervisor,
) -> Option<(Vec<ARecordedEvent>, Reading<()>)> {
    let mut retained = Vec::new();
    let mut from = 0u64;
    for _ in 0..THE_MOST_READS_OF_THE_JOURNAL {
        let fields = the_answer_to(
            machine,
            a_request(
                THE_READ_FORWARD_REQUEST,
                Fields::none().and(THE_CURSOR_FIELD, from.to_string()),
            ),
        )?;
        // **`Queue & History` W3's now, from the read that ends the walk**: the newest of the
        // machine's answers, on the clock its events are on.
        let now = a_moment(fields.text(THE_READ_AT_FIELD), fields.text(THE_READ_ON_CLOCK_FIELD))?;
        let how_many: usize = a_number(fields.text(HOW_MANY_EVENTS_FIELD))?;
        let next: u64 = a_number(fields.text(THE_NEXT_SEQUENCE_FIELD))?;
        let mut last = None;
        for at in 0..how_many {
            let field = |name: &str| an_item(&fields, AN_EVENT, at, name);
            let sequence: u64 = a_number(field(THE_SEQUENCE_FIELD))?;
            let identity = Identity::read_back(IdentityKind::Event, field(THE_EVENT_IDENTITY_FIELD)?)?;
            let at_moment = a_moment(field(THE_HAPPENED_AT_FIELD), field(THE_HAPPENED_ON_CLOCK_FIELD))?;
            let kind = field(THE_KIND_FIELD)?;
            retained.push(ARecordedEvent {
                identity,
                at: at_moment,
                left_the_line_without_startable_work: THE_KINDS_THAT_MOVE_THE_LINE.contains(&kind),
            });
            last = Some(sequence);
        }
        match last {
            Some(last) if last.saturating_add(1) < next => from = last.saturating_add(1),
            _ => return Some((retained, now)),
        }
    }
    None
}

/// A moment as the machine's answer carries it: whole seconds since 1970, and
/// the clock it was taken on (`State, identity and recovery` D5).
fn a_moment(seconds: Option<&str>, clock: Option<&str>) -> Option<Reading<()>> {
    let seconds: u64 = a_number(seconds)?;
    let clock = clock.filter(|clock| !clock.is_empty())?;
    Some(Reading::taken(
        (),
        SystemTime::UNIX_EPOCH + Duration::from_secs(seconds),
        Clock::AMachine(MachineClock(clock.to_string())),
    ))
}

// ════════════════════════════════════════════════════════════════════════════
// What wakes it
// ════════════════════════════════════════════════════════════════════════════

/// **What wakes a waiting re-check before its time**, per server: a count that
/// moves on every nudge, and the one place a re-check waits on it.
///
/// **Run-held, and written nowhere.** A nudge is never a reading: what it
/// does is make the re-check take one.
#[derive(Debug, Default)]
pub struct TheIdleNudges {
    moved: Mutex<HashMap<String, u64>>,
    woken: Condvar,
}

impl TheIdleNudges {
    /// **Something happened that may change what the re-check would find** —
    /// an event applied, or a connection beginning or ending.
    pub fn nudge(&self, server: &str) {
        if let Ok(mut moved) = self.moved.lock() {
            let count = moved.entry(server.to_string()).or_insert(0);
            *count = count.wrapping_add(1);
        }
        self.woken.notify_all();
    }

    /// **How far this server's count stands**, read before a re-check so a
    /// nudge that lands while it runs is not missed.
    pub fn how_far(&self, server: &str) -> u64 {
        self.moved
            .lock()
            .map(|moved| moved.get(server).copied().unwrap_or(0))
            .unwrap_or(0)
    }

    /// **Wait until this server's count moves past `seen`**, or until
    /// `at_most` has passed where there is one. Answers whether a nudge came.
    pub fn a_nudge_or_the_time(&self, server: &str, seen: u64, at_most: Option<Duration>) -> bool {
        let Ok(mut moved) = self.moved.lock() else {
            return false;
        };
        let deadline = at_most.map(|wait| std::time::Instant::now() + wait);
        loop {
            if moved.get(server).copied().unwrap_or(0) != seen {
                return true;
            }
            match deadline {
                None => match self.woken.wait(moved) {
                    Ok(next) => moved = next,
                    Err(_) => return false,
                },
                Some(deadline) => {
                    let left = deadline.saturating_duration_since(std::time::Instant::now());
                    if left.is_zero() {
                        return false;
                    }
                    match self.woken.wait_timeout(moved, left) {
                        Ok((next, _)) => moved = next,
                        Err(_) => return false,
                    }
                }
            }
        }
    }
}

// ════════════════════════════════════════════════════════════════════════════
// One re-check, and the sequence
// ════════════════════════════════════════════════════════════════════════════

/// **What one re-check asks the sequence to do next.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatComesNext {
    /// **`Queue & History` W4: short of the half hour** — look again after this, or sooner on a
    /// nudge.
    LookAgainIn(Duration),
    /// Nothing is scheduled: the next look is on a nudge.
    WaitForANudge,
}

/// **One re-check, and the post it makes where it is told about.**
///
/// The judgement is `Queue & History`'s, whole. **The post is composed only
/// where `Queue & History` W5's mark was written by this very act**, so a telling that is posted
/// is a telling that is marked, and the next re-check finds it spoken.
pub fn one_recheck(
    machine: &dyn ReadsTheMachineFresh,
    marks: &mut dyn KeepsTheIdleMarks,
    server: &Identity,
    the_servers_own_name: &str,
) -> (WhatComesNext, Option<APost>) {
    match the_recheck(machine, marks, server) {
        Ok(WhatTheRecheckDid::ItWasToldAbout(_)) => (
            WhatComesNext::WaitForANudge,
            Some(APost::from(
                what_an_idle_machine_posts(server),
                Verbatim::new(the_servers_own_name),
                picture::the_picture_for(std::env::consts::OS, None),
            )),
        ),
        Ok(WhatTheRecheckDid::ScheduleTheRecheckIn(wait)) => (WhatComesNext::LookAgainIn(wait), None),
        // **Not idle, already told, no stretch to date, the machine not read,
        // or the record not answering**: nothing is said, and the next look is
        // on the next nudge.
        Ok(_) | Err(_) => (WhatComesNext::WaitForANudge, None),
    }
}

/// **Started when `Servers` Q1 step 7 completes for this connection**, on a
/// thread of its own. Returns at once.
///
/// **A server row with no confirmed installation recorded starts nothing**:
/// `Queue & History` W5's key has no first half, and nothing is made up for it.
pub fn is_started<R: Runtime>(
    app: &AppHandle<R>,
    server: &Identity,
    connection: u64,
    sessions: &ThisServersSessions,
    package_area: &str,
    installation: Option<&str>,
) {
    let Some(installation) = installation.and_then(|text| Identity::read_back(IdentityKind::Installation, text))
    else {
        return;
    };
    let (app, server, sessions, package_area) =
        (app.clone(), server.clone(), sessions.clone(), package_area.to_string());
    let _ = std::thread::Builder::new()
        .name("smoozy idle re-check".to_string())
        .spawn(move || the_sequence(&app, &server, connection, sessions, package_area, installation));
}

fn the_sequence<R: Runtime>(
    app: &AppHandle<R>,
    server: &Identity,
    connection: u64,
    sessions: ThisServersSessions,
    package_area: String,
    installation: Identity,
) {
    let (Some(periods), Some(started)) = (
        app.try_state::<Arc<ThePeriodsThisRun>>(),
        app.try_state::<Mutex<Started>>(),
    ) else {
        return;
    };
    let periods = periods.inner().clone();
    let record = crate::keys::TheRunsRecord {
        started: started.inner(),
    };
    let server_text = server.as_text();
    let holds = periods.the_machines_work();
    let is_live = || {
        periods.a_period_stands_for(&server_text)
            && sessions.a_connection_is_live()
            && holds.is_current(&server_text, connection)
    };
    loop {
        if !is_live() {
            return;
        }
        let seen = holds.idle_nudges().how_far(&server_text);
        let mut point = OverThisServersPoint {
            sessions: sessions.clone(),
            package_area: package_area.clone(),
        };
        let fresh = TheMachineReadFresh::over(&mut point, installation.clone());
        let mut marks = TheIdleMarksInTheRecord { record: &record };
        let name = with_the_record(&record, |connection| {
            server_rows::one(connection, &server_text).ok().flatten()
        })
        .flatten()
        .map(|row| crate::surface::servers::what_a_server_is_called(&row))
        .unwrap_or_default();
        let (next, post) = one_recheck(&fresh, &mut marks, server, &name);
        if let Some(post) = post {
            if let Some(service) = app.try_state::<Arc<TheNotificationService>>() {
                service.post(post);
            }
        }
        let at_most = match next {
            WhatComesNext::LookAgainIn(wait) => Some(wait),
            WhatComesNext::WaitForANudge => None,
        };
        holds.idle_nudges().a_nudge_or_the_time(&server_text, seen, at_most);
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use smoozy_server_protocol::asking_the_supervisor::WhatCameBack;
    use smoozy_server_protocol::framing::{Answered, Request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS};
    use smoozy_server_protocol::requests::{
        the_field_of_item, HOW_MANY_JOBS_FIELD, IT_IS_RENDERING, IT_IS_WAITING, THE_ABOUT_FIELD,
        THE_JOB_FIELD, THE_QUEUE_READING_REQUEST, THE_QUEUE_STATE_FIELD, THE_RENDER_OPERATION_FIELD,
    };

    // ── A machine, scripted ──────────────────────────────────────────────────

    /// **One event as the machine's journal holds it**: its kind, what it is
    /// about, and its moment in seconds on the machine's clock.
    struct AnEvent {
        identity: Identity,
        kind: &'static str,
        about: Option<Identity>,
        at: u64,
    }

    fn an_event(kind: &'static str, about: Option<Identity>, at: u64) -> AnEvent {
        AnEvent {
            identity: Identity::issue(IdentityKind::Event).unwrap(),
            kind,
            about,
            at,
        }
    }

    /// **A machine whose line, journal and own now are given**, answering the
    /// two requests a re-check makes. `now` absent is a program older than the
    /// 38's piece 1, whose read forward carries no now.
    struct AMachine {
        line: Vec<(Identity, &'static str)>,
        journal: Vec<AnEvent>,
        now: Option<u64>,
        asked: usize,
    }

    const THE_CLOCK: &str = "installation-of-the-box";

    impl AsksTheSupervisor for AMachine {
        fn ask(&mut self, request: &Request) -> WhatCameBack {
            self.asked += 1;
            let mut fields = Fields::none();
            if request.name == THE_QUEUE_READING_REQUEST {
                fields = fields.and(HOW_MANY_JOBS_FIELD, self.line.len().to_string());
                for (at, (job, state)) in self.line.iter().enumerate() {
                    fields = fields
                        .and(&the_field_of_item(THE_JOB_FIELD, at, THE_JOB_FIELD), job.as_text())
                        .and(&the_field_of_item(THE_JOB_FIELD, at, THE_QUEUE_STATE_FIELD), *state)
                        .and(
                            &the_field_of_item(THE_JOB_FIELD, at, THE_RENDER_OPERATION_FIELD),
                            Identity::issue(IdentityKind::Operation).unwrap().as_text(),
                        );
                }
            } else if request.name == THE_READ_FORWARD_REQUEST {
                let from: usize = request.fields.text(THE_CURSOR_FIELD).unwrap().parse().unwrap();
                let carried: Vec<&AnEvent> = self.journal.iter().skip(from).take(2).collect();
                fields = fields
                    .and(HOW_MANY_EVENTS_FIELD, carried.len().to_string())
                    .and(THE_NEXT_SEQUENCE_FIELD, self.journal.len().to_string());
                if let Some(now) = self.now {
                    fields = fields.and(THE_READ_AT_FIELD, now.to_string()).and(THE_READ_ON_CLOCK_FIELD, THE_CLOCK);
                }
                for (at, event) in carried.iter().enumerate() {
                    let item = |name: &str| the_field_of_item(AN_EVENT, at, name);
                    fields = fields
                        .and(&item(THE_SEQUENCE_FIELD), (from + at).to_string())
                        .and(&item(THE_EVENT_IDENTITY_FIELD), event.identity.as_text())
                        .and(&item(THE_KIND_FIELD), event.kind)
                        .and(&item(THE_HAPPENED_AT_FIELD), event.at.to_string())
                        .and(&item(THE_HAPPENED_ON_CLOCK_FIELD), THE_CLOCK);
                    if let Some(about) = &event.about {
                        fields = fields.and(&item(THE_ABOUT_FIELD), about.as_text());
                    }
                }
            } else {
                return WhatCameBack::ItAnswered(
                    Answered::to(request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, "test")
                        .saying_it_does_not_answer_that_request(&request.name),
                );
            }
            WhatCameBack::ItAnswered(
                Answered::to(request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, "test").carrying(fields),
            )
        }
    }

    /// **A machine that ran one job, which ended at `ended`**, and is quiet
    /// since, its own now being `now`.
    fn a_machine_that_ran_one_job(ended: u64, now: Option<u64>) -> AMachine {
        let job = Identity::issue(IdentityKind::Job).unwrap();
        AMachine {
            line: Vec::new(),
            journal: vec![
                an_event(THE_JOB_RECORD_WRITTEN_EVENT, Some(job.clone()), ended - 100),
                an_event(THE_RENDER_ATTEMPT_STARTED_EVENT, Some(job.clone()), ended - 90),
                an_event(THE_RECORDED_OUTCOME_WRITTEN_EVENT, Some(job), ended),
            ],
            now,
            asked: 0,
        }
    }

    // ── A record, in memory ──────────────────────────────────────────────────

    struct ARecord(rusqlite::Connection);

    impl LendsTheRecord for ARecord {
        fn lend(&self, act: &mut dyn FnMut(&rusqlite::Connection)) {
            act(&self.0)
        }
        fn write_a_diary_line(&self, _act: &smoozy_keys::diary::Act) {}
    }

    fn a_record() -> ARecord {
        let connection = rusqlite::Connection::open_in_memory().unwrap();
        for migration in crate::record::migrations::ALL {
            if let crate::record::migrations::WhatItRuns::TheseStatements(statements) = migration.what_it_runs {
                for statement in statements {
                    connection.execute(statement, []).unwrap();
                }
            }
        }
        ARecord(connection)
    }

    fn the_marks_in(record: &ARecord) -> i64 {
        record
            .0
            .query_row("SELECT count(*) FROM idle_spoken_mark", [], |row| row.get(0))
            .unwrap()
    }

    const HALF_AN_HOUR: u64 = 30 * 60;

    /// **`Queue & History` W3 and `Queue & History` W5, through this desktop's own ends: a
    /// machine quiet past the half hour is told about once, in Design's two
    /// lines, and the mark is in the local record — so the same stretch is not
    /// told about again, at the next re-check or after a restart.**
    ///
    /// **This is the line that fails if the re-check is not given its ends**,
    /// which is what the idle kind had until the 38's piece 1: built, and called
    /// only by its own tests.
    #[test]
    fn a_machine_quiet_past_the_half_hour_is_told_about_once() {
        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        let installation = Identity::issue(IdentityKind::Installation).unwrap();
        let record = a_record();
        let mut machine = a_machine_that_ran_one_job(10_000, Some(10_000 + HALF_AN_HOUR + 60));

        let (next, post) = one_recheck(
            &TheMachineReadFresh::over(&mut machine, installation.clone()),
            &mut TheIdleMarksInTheRecord { record: &record },
            &server,
            "Vast 5060",
        );
        let post = post.expect("told about");
        assert_eq!(post.headline(), "Vast 5060 is\u{00A0}idle");
        assert_eq!(post.the_second_line(), "No jobs for 30\u{00A0}minutes.");
        assert_eq!(next, WhatComesNext::WaitForANudge);
        assert_eq!(the_marks_in(&record), 1);

        // **Once per stretch**: the same machine, read again, and a record
        // opened afresh over the same store (a restart) say nothing.
        let (_, again) = one_recheck(
            &TheMachineReadFresh::over(&mut machine, installation),
            &mut TheIdleMarksInTheRecord { record: &record },
            &server,
            "Vast 5060",
        );
        assert!(again.is_none(), "a stretch already told about is not told about again");
        assert_eq!(the_marks_in(&record), 1);
    }

    /// **`Queue & History` W4: short of the half hour, nothing is said and the re-check is
    /// scheduled at the remainder — machine-clock arithmetic.** At the re-read,
    /// **new work means nothing is said**; **a second quiet stretch speaks
    /// again**.
    #[test]
    fn short_of_the_half_hour_it_waits_new_work_says_nothing_and_a_new_stretch_speaks_again() {
        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        let installation = Identity::issue(IdentityKind::Installation).unwrap();
        let record = a_record();
        let look = |machine: &mut AMachine| {
            one_recheck(
                &TheMachineReadFresh::over(machine, installation.clone()),
                &mut TheIdleMarksInTheRecord { record: &record },
                &server,
                "Vast 5060",
            )
        };

        let mut machine = a_machine_that_ran_one_job(10_000, Some(10_000 + 600));
        assert_eq!(look(&mut machine), (WhatComesNext::LookAgainIn(Duration::from_secs(1_200)), None));
        assert_eq!(the_marks_in(&record), 0);

        // At the re-read the machine has work: a waiting job, then a rendering
        // one. Nothing is said, whatever the clock says.
        machine.now = Some(10_000 + HALF_AN_HOUR + 1);
        machine.line = vec![(Identity::issue(IdentityKind::Job).unwrap(), IT_IS_WAITING)];
        assert_eq!(look(&mut machine).1, None);
        machine.line = vec![(Identity::issue(IdentityKind::Job).unwrap(), IT_IS_RENDERING)];
        assert_eq!(look(&mut machine).1, None);
        assert_eq!(the_marks_in(&record), 0);

        // It falls quiet, past the half hour: told.
        machine.line.clear();
        assert!(look(&mut machine).1.is_some());
        // **A second stretch** — another job ran and ended later.
        let later = Identity::issue(IdentityKind::Job).unwrap();
        machine.journal.push(an_event(THE_JOB_RECORD_WRITTEN_EVENT, Some(later.clone()), 20_000));
        machine.journal.push(an_event(THE_RECORDED_OUTCOME_WRITTEN_EVENT, Some(later), 21_000));
        machine.now = Some(21_000 + HALF_AN_HOUR);
        assert!(look(&mut machine).1.is_some(), "a new stretch speaks again");
        assert_eq!(the_marks_in(&record), 2);
    }

    /// **An event that moves no job in the line dates nothing but `Queue & History` W2's floor**
    /// — a setup run's disposition is not the line falling quiet.
    #[test]
    fn an_event_that_moves_no_job_does_not_move_the_quiet_moment() {
        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        let record = a_record();
        let mut machine = a_machine_that_ran_one_job(10_000, Some(10_000 + HALF_AN_HOUR + 5));
        machine.journal.push(an_event(
            smoozy_server_protocol::requests::THE_DISPOSITION_EVENT,
            Some(Identity::issue(IdentityKind::Operation).unwrap()),
            10_000 + HALF_AN_HOUR,
        ));
        let (_, post) = one_recheck(
            &TheMachineReadFresh::over(&mut machine, Identity::issue(IdentityKind::Installation).unwrap()),
            &mut TheIdleMarksInTheRecord { record: &record },
            &server,
            "Vast 5060",
        );
        assert!(post.is_some(), "the quiet moment is the job's ending, not the setup run's");
    }

    /// **`Queue & History` W4's *disconnected, nothing fires*, and a program older than this
    /// piece**: a read forward that carries no now is no reading, and nothing
    /// is said — never a half hour reckoned on this computer's clock.
    #[test]
    fn a_machine_that_says_no_now_is_not_told_about() {
        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        let record = a_record();
        let mut machine = a_machine_that_ran_one_job(10_000, None);
        let (next, post) = one_recheck(
            &TheMachineReadFresh::over(&mut machine, Identity::issue(IdentityKind::Installation).unwrap()),
            &mut TheIdleMarksInTheRecord { record: &record },
            &server,
            "Vast 5060",
        );
        assert_eq!((next, post), (WhatComesNext::WaitForANudge, None));
        assert_eq!(the_marks_in(&record), 0);
    }

    /// **The journal is read to its end however many reads that takes** — the
    /// scripted machine carries two events a read.
    #[test]
    fn the_journal_is_read_whole() {
        let mut machine = a_machine_that_ran_one_job(10_000, Some(20_000));
        for at in 0..5 {
            machine.journal.push(an_event("a test's own event", None, 10_001 + at));
        }
        let (retained, now) = the_journal_as_it_stands(&mut machine).expect("read");
        assert_eq!(retained.len(), 8);
        assert_eq!(machine.asked, 4);
        assert_eq!(
            now.taken_at(),
            SystemTime::UNIX_EPOCH + Duration::from_secs(20_000)
        );
    }

    /// **What wakes it: a nudge after the count it saw, or the time — and a
    /// nudge that landed while it was looking is not missed.**
    #[test]
    fn a_nudge_wakes_it_and_the_time_ends_its_wait() {
        let nudges = Arc::new(TheIdleNudges::default());
        let seen = nudges.how_far("s");
        assert!(!nudges.a_nudge_or_the_time("s", seen, Some(Duration::from_millis(20))));
        nudges.nudge("s");
        // **A wait with no time of its own is how the sequence waits**; the
        // test gives each one a long one so that a nudge that never lands
        // fails here rather than hanging the run.
        let long = Some(Duration::from_secs(10));
        assert!(nudges.a_nudge_or_the_time("s", seen, long), "a nudge already landed");
        let seen = nudges.how_far("s");
        let waiting = {
            let nudges = nudges.clone();
            std::thread::spawn(move || nudges.a_nudge_or_the_time("s", seen, long))
        };
        std::thread::sleep(Duration::from_millis(20));
        nudges.nudge("another server");
        nudges.nudge("s");
        assert!(waiting.join().unwrap());
    }
}

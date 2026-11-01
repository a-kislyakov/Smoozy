//! **The queue's own acts on the machine: the entry's *Stop*, *Cancel* on a
//! waiting or held job, and a group's *Cancel all*** — step 10c's S3, over the
//! machine's half S1 built and the pair's sender S2 built.
//!
//! **What each is, by its clauses.**
//!
//! - **The stop** — `Server protocol and transfers` C21's own command, with its
//!   own operation identity, naming the render's (`Queue & History` L10,
//!   `Rendering` K5). It is waited on by its own identity with the waiter's own
//!   deadline, `Rendering` K9's 120 seconds (`Rendering` K8, `Queue & History`
//!   L12), and **the deadline passing writes nothing and claims nothing**: it is
//!   `Rendering` K10's *the stop was sent, and the render had not ended when
//!   Smoozy stopped waiting*.
//! - **The cancel** — `Queue & History` L1's command naming the job, for a
//!   waiting job and a held one alike (`Queue & History` L5). **The machine's
//!   answer is the authority** (`Queue & History` L1, L4): a job that started
//!   rendering or ended while the cancel travelled comes back *not cancelled*
//!   with the machine's own words. **The local record goes only on *removed***
//!   (`Queue & History` L2): on the answer, or on the journal event S2's pass
//!   already applies.
//! - **A group's *Cancel all*** — `Queue & History` L19, the stop-and-clear
//!   machinery scoped to the group, **sent through S2's one sender**
//!   (`crate::servers::stop_and_clear::the_pair_is_sent`) with the group's
//!   identity. Nothing about the pair is composed twice.
//!
//! **While an act is in hand the entry says so** — `Queue & History` L12a's
//! *stopping* and L2's *cancelling*, **composed from the desktop's knowledge of
//! its own outstanding command** and never a sixth queue state: the machine's
//! record still reads what it reads. **That knowledge is held for the run**
//! ([`where_the_act_stands`]) and nothing durable copies it; an act a closed
//! Smoozy left in hand is learned from the machine's journal on the next
//! connection, as every other ending is.
//!
//! - **A move on the line** — `Queue & History` L6's reorder of one waiting
//!   job and `Queue & History` L7's block move of a group, each naming a
//!   position as *after job X* or *at the front of the waiting line* and never
//!   an index. **The machine applies it to the line as it is then**
//!   (`Queue & History` L6, `Queue & History` L8),
//!   so nothing here works out an order: the page names where the row was
//!   dropped, this file sends it, and **what the user sees afterwards is a
//!   fresh queue reading drawn** (`Queue & History` §16's *"A1's period-held
//!   reading of the machine's own line redrawn"*). No act is held for the run
//!   while a move travels, and nothing is written durably. The 38's piece 8.
//!
//! **What this file does not do.** It presents nothing — the words are
//! `src/surface/the_queue.rs`'s — and it forgets no job it did not hear go.
//!
//! **How a clause marker is written in this file.** No header list; every
//! document is written at its site, and a bare marker means a clause of
//! `Preparation/Architecture/Desktop app and delivery.md`.

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use std::time::Instant;

use smoozy_server_protocol::asking_the_supervisor::WhatCameBack;
use smoozy_server_protocol::framing::{Fields, Request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS};
use smoozy_server_protocol::requests::{
    AFTER_A_JOB, AT_THE_FRONT_OF_THE_WAITING_LINE, ITS_OWN_ENDING, IT_HAS_A_RECORDED_OUTCOME,
    IT_IS_STILL_IN_HAND, IT_WAS_LEFT_STOPPED, IT_WAS_NOT_CANCELLED, IT_WAS_REMOVED,
    NOTHING_WAS_RENDERING, THERE_IS_NO_RECORD_OF_IT, THE_CANCELS_OUTCOME_FIELD, THE_CANCEL_REQUEST,
    THE_GROUP_BLOCK_MOVE_REQUEST, THE_GROUP_FIELD, THE_JOB_FIELD, THE_JOB_IT_GOES_AFTER_FIELD,
    THE_MACHINES_OWN_WORDS_FIELD, THE_MOVES_OUTCOME_FIELD, THE_OPERATION_TO_STOP_FIELD,
    THE_POSITION_FIELD, THE_REORDER_REQUEST, THE_STOPS_OUTCOME_FIELD, THE_STOP_REQUEST,
    WHAT_BECAME_OF_IT_FIELD,
};
use smoozy_state::identities::{Identity, IdentityKind};

use crate::servers::stop_and_clear::{TheWait, WhatThePairCameTo};
use crate::servers::the_machines_work::AsksTheSupervisor;

/// **Which of the queue's three acts.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheQueuesOwnAct {
    TheStop,
    TheCancel,
    TheGroupsCancel,
}

/// **Where one job stands with an act of Smoozy's own**, for the entry to
/// say. **An act that did what it was for leaves nothing here** — the job's
/// record then says what happened, through the journal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhereTheActStands {
    /// **`Queue & History` L12a's *stopping* and L2's *cancelling*.**
    InHand(TheQueuesOwnAct),
    /// **Nothing reached the machine** — no live connection, or the request
    /// never went out.
    NotReached(TheQueuesOwnAct),
    /// **The machine did not take it**: an older program that answers no such
    /// request, or `Server protocol and transfers` C8's *no record of that
    /// identity* — refused before it was recorded, the work not done. **Its
    /// own words ride along** where it gave some.
    NotTaken(TheQueuesOwnAct, Option<String>),
    /// **The waiter's own deadline passed** (`Server protocol and transfers`
    /// C17): nothing was written, and for a stop it is `Rendering` K10's
    /// *the stop was sent, and the render had not ended when Smoozy stopped
    /// waiting*.
    TheWaiterGaveUp(TheQueuesOwnAct),
    /// **The stop's recorded outcome is *the stop failed*** — a durable write
    /// the stop needed was refused (`Rented-server runtime` X8), with the
    /// machine's own words.
    TheStopFailed(Option<String>),
    /// **`Queue & History` L1 and L4: *not cancelled*** — the job was not
    /// waiting or held when the cancel arrived, with the machine's own words
    /// saying why.
    NotCancelled(Option<String>),
    /// **`Queue & History` L16 in `Queue & History` L19's group form**: the
    /// group's clear was refused, so its stop was not attempted either.
    TheClearWasRefused,
}

// ════════════════════════════════════════════════════════════════════════════
// What the run knows of its own acts
// ════════════════════════════════════════════════════════════════════════════

fn the_acts_this_run() -> &'static Mutex<HashMap<String, WhereTheActStands>> {
    static HELD: OnceLock<Mutex<HashMap<String, WhereTheActStands>>> = OnceLock::new();
    HELD.get_or_init(|| Mutex::new(HashMap::new()))
}

/// **Where the job stands with an act of Smoozy's own**, where one is known.
pub fn where_the_act_stands(job: &str) -> Option<WhereTheActStands> {
    the_acts_this_run().lock().ok()?.get(job).cloned()
}

/// **The job now stands here, or — with `None` — with nothing to say.**
pub fn it_stands(job: &str, stands: Option<WhereTheActStands>) {
    if let Ok(mut held) = the_acts_this_run().lock() {
        match stands {
            Some(stands) => {
                held.insert(job.to_string(), stands);
            }
            None => {
                held.remove(job);
            }
        }
    }
}

/// **One press at a time**: an act already in hand for this job is not sent
/// again under a second identity.
fn it_is_in_hand(job: &str) -> bool {
    matches!(where_the_act_stands(job), Some(WhereTheActStands::InHand(_)))
}

// ════════════════════════════════════════════════════════════════════════════
// Asking, and waiting on one identity
// ════════════════════════════════════════════════════════════════════════════

/// **How asking one identity again and again ended.**
#[derive(Debug, Clone, PartialEq, Eq)]
enum HowTheAskEnded {
    /// C8's *a recorded outcome*: the act's own word, and the machine's words.
    AnOutcome(String, Option<String>),
    NotTaken(Option<String>),
    NotReached,
    TheWaiterGaveUp,
}

/// **One request, sent and then asked again under the same identity until its
/// outcome is recorded or the waiter's own deadline passes** —
/// `Server protocol and transfers` C8 and C9: the machine records before it
/// acts, so the same request arriving twice acts once, and *still in hand* is
/// an answer rather than a silence.
fn asked_until_it_ends(
    machine: &mut dyn AsksTheSupervisor,
    request: &Request,
    the_acts_own_field: &str,
    wait: TheWait,
) -> HowTheAskEnded {
    asked_until_it_ends_with_its_answer(machine, request, the_acts_own_field, wait).0
}

/// **The same, with the fields of the answer that recorded the outcome** —
/// `Telegram` V1c's change reads back the value the record now holds.
fn asked_until_it_ends_with_its_answer(
    machine: &mut dyn AsksTheSupervisor,
    request: &Request,
    the_acts_own_field: &str,
    wait: TheWait,
) -> (HowTheAskEnded, Option<Fields>) {
    let began = Instant::now();
    let mut it_may_have_arrived = false;
    loop {
        // **Every ask is given what is left of the waiter's own deadline, and
        // none is begun once nothing is** — the dead-line sweep's two-minute-wait repair,
        // 2026-10-19. Until then the deadline was looked at only after an ask
        // returned, so an ask begun just before it on a line gone silent held
        // the user's wait a whole break window past it. **The give-up writes
        // nothing and cancels nothing** (`Server protocol and transfers`
        // C17): the outcome is not yet known, and a late one is read where
        // every outcome is read, the machine's journal.
        let Some(left) = wait.what_is_left(began) else {
            return (HowTheAskEnded::TheWaiterGaveUp, None);
        };
        match machine.ask_within(request, left) {
            WhatCameBack::ItAnswered(answered) => {
                if answered.the_request_name_it_says_it_does_not_answer().is_some()
                    || answered.the_protocol_version_it_says_it_does_not_speak().is_some()
                {
                    return (HowTheAskEnded::NotTaken(None), None);
                }
                let words = answered
                    .fields
                    .text(THE_MACHINES_OWN_WORDS_FIELD)
                    .map(str::to_string);
                match answered.fields.text(WHAT_BECAME_OF_IT_FIELD) {
                    Some(IT_HAS_A_RECORDED_OUTCOME) => {
                        let word = answered.fields.text(the_acts_own_field).unwrap_or_default();
                        return (HowTheAskEnded::AnOutcome(word.to_string(), words), Some(answered.fields.clone()));
                    }
                    Some(THERE_IS_NO_RECORD_OF_IT) => return (HowTheAskEnded::NotTaken(words), None),
                    Some(IT_IS_STILL_IN_HAND) => it_may_have_arrived = true,
                    // **An answer that says none of the three is not one of
                    // them**, and nothing is guessed from it.
                    _ => it_may_have_arrived = true,
                }
            }
            WhatCameBack::NothingCorrelatedCameBack => it_may_have_arrived = true,
            WhatCameBack::ItWasNeverAsked if !it_may_have_arrived => return (HowTheAskEnded::NotReached, None),
            WhatCameBack::ItWasNeverAsked => {}
        }
        if wait.what_is_left(began).is_none() {
            return (HowTheAskEnded::TheWaiterGaveUp, None);
        }
        wait.pauses(began);
    }
}

/// **`Server protocol and transfers` C21: the stop, naming the render's
/// operation identity**, under a fresh identity of its own, waited on by that
/// identity (`Rendering` K8). **`None` is the stop done**: *left stopped*,
/// *its own ending* or *nothing was rendering* — the render is not running,
/// and the job's own record says which (`Queue & History` L11).
pub fn the_stop_is_sent(
    machine: &mut dyn AsksTheSupervisor,
    the_render: &Identity,
    wait: TheWait,
) -> Option<WhereTheActStands> {
    let act = TheQueuesOwnAct::TheStop;
    let Ok(command) = Identity::issue(IdentityKind::Operation) else {
        return Some(WhereTheActStands::NotReached(act));
    };
    let request = Request::speaking(THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, command, THE_STOP_REQUEST)
        .carrying(Fields::none().and(THE_OPERATION_TO_STOP_FIELD, the_render.as_text()));
    match asked_until_it_ends(machine, &request, THE_STOPS_OUTCOME_FIELD, wait) {
        HowTheAskEnded::AnOutcome(word, words) => match word.as_str() {
            IT_WAS_LEFT_STOPPED | ITS_OWN_ENDING | NOTHING_WAS_RENDERING => None,
            _ => Some(WhereTheActStands::TheStopFailed(words)),
        },
        HowTheAskEnded::NotTaken(words) => Some(WhereTheActStands::NotTaken(act, words)),
        HowTheAskEnded::NotReached => Some(WhereTheActStands::NotReached(act)),
        HowTheAskEnded::TheWaiterGaveUp => Some(WhereTheActStands::TheWaiterGaveUp(act)),
    }
}

/// **`Queue & History` L1 and L5: the cancel, naming the job.** **`None` is
/// *removed*** — the machine's copy is gone, and only then may the local one
/// go (`Queue & History` L2).
pub fn the_cancel_is_sent(
    machine: &mut dyn AsksTheSupervisor,
    job: &Identity,
    wait: TheWait,
) -> Option<WhereTheActStands> {
    let act = TheQueuesOwnAct::TheCancel;
    let Ok(command) = Identity::issue(IdentityKind::Operation) else {
        return Some(WhereTheActStands::NotReached(act));
    };
    let request = Request::speaking(THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, command, THE_CANCEL_REQUEST)
        .carrying(Fields::none().and(THE_JOB_FIELD, job.as_text()));
    match asked_until_it_ends(machine, &request, THE_CANCELS_OUTCOME_FIELD, wait) {
        HowTheAskEnded::AnOutcome(word, _) if word == IT_WAS_REMOVED => None,
        HowTheAskEnded::AnOutcome(word, words) if word == IT_WAS_NOT_CANCELLED => {
            Some(WhereTheActStands::NotCancelled(words))
        }
        HowTheAskEnded::AnOutcome(_, words) => Some(WhereTheActStands::NotCancelled(words)),
        HowTheAskEnded::NotTaken(words) => Some(WhereTheActStands::NotTaken(act, words)),
        HowTheAskEnded::NotReached => Some(WhereTheActStands::NotReached(act)),
        HowTheAskEnded::TheWaiterGaveUp => Some(WhereTheActStands::TheWaiterGaveUp(act)),
    }
}

/// **What a group's cancel leaves each member it was sent for**, read off
/// S2's result. `rendering` is whether this member was the one rendering.
///
/// - ***Cleared and stopped*** leaves nothing: the members are gone.
/// - ***Nothing was changed*** — refused by name, never reached, or the record
///   would not write — is *not taken*, for every member.
/// - ***The clear was refused*** is `Queue & History` L16's, for every member.
/// - ***The render had not stopped***: the waiting members were cleared, so
///   only the rendering one is left, and for it this is `Rendering` K10.
/// - ***Nothing came back*** is the waiter giving up, and never *nothing was
///   changed*: the machine may have carried the pair out with nobody
///   connected (`Queue & History` L13).
pub fn what_the_groups_cancel_leaves(came_to: WhatThePairCameTo, rendering: bool) -> Option<WhereTheActStands> {
    let act = TheQueuesOwnAct::TheGroupsCancel;
    match came_to {
        WhatThePairCameTo::ClearedAndStopped => None,
        WhatThePairCameTo::NothingWasChanged => Some(WhereTheActStands::NotTaken(act, None)),
        WhatThePairCameTo::TheClearWasRefused => Some(WhereTheActStands::TheClearWasRefused),
        WhatThePairCameTo::TheRenderHadNotStopped if rendering => {
            Some(WhereTheActStands::TheWaiterGaveUp(TheQueuesOwnAct::TheStop))
        }
        WhatThePairCameTo::TheRenderHadNotStopped => None,
        WhatThePairCameTo::NothingCameBack => Some(WhereTheActStands::TheWaiterGaveUp(act)),
    }
}

// ════════════════════════════════════════════════════════════════════════════
// A move on the line — `Queue & History` L6 and `Queue & History` L7, the
// 38's piece 8
// ════════════════════════════════════════════════════════════════════════════

/// **What moves**: one job, or a group's waiting members as one block.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatMoves {
    /// `Queue & History` L6.
    OneJob(Identity),
    /// `Queue & History` L7.
    AGroup(Identity),
}

/// **`Queue & History` L6's position: after a job, or the front of the
/// waiting line.** The page names the job it was dropped behind; nothing
/// here counts rows.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhereItGoes {
    AtTheFront,
    After(Identity),
}

/// **What a move came to.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheMoveCameTo {
    /// **The machine wrote it**, or wrote nothing because what was named was
    /// not waiting (`Queue & History` L9). **Either way the queue reading now
    /// says where everything is**, which is what the user reads; the machine's
    /// own words are not drawn.
    TheMachineAnswered,
    /// Nothing reached the machine.
    NotReached,
    /// **The machine was reached and did not take it** — an older program that
    /// answers no such request, or `Server protocol and transfers` C8's *no
    /// record of that identity*.
    NotTaken(Option<String>),
    /// **The waiter's own deadline passed** with the move still in hand: it
    /// may yet land, and the next reading says whether it did.
    TheWaiterGaveUp,
}

/// **The move, sent under a fresh identity of its own and asked again under
/// it until its outcome is recorded** — the same waiter the cancel uses, since
/// a move refused part-way is *still in hand* until the machine finishes it
/// (`Queue & History` L24, `Queue & History` D14).
pub fn the_move_is_sent(
    machine: &mut dyn AsksTheSupervisor,
    what: &WhatMoves,
    to: &WhereItGoes,
    wait: TheWait,
) -> WhatTheMoveCameTo {
    let Ok(command) = Identity::issue(IdentityKind::Operation) else {
        return WhatTheMoveCameTo::NotReached;
    };
    let (name, fields) = match what {
        WhatMoves::OneJob(job) => (THE_REORDER_REQUEST, Fields::none().and(THE_JOB_FIELD, job.as_text())),
        WhatMoves::AGroup(group) => (
            THE_GROUP_BLOCK_MOVE_REQUEST,
            Fields::none().and(THE_GROUP_FIELD, group.as_text()),
        ),
    };
    let fields = match to {
        WhereItGoes::AtTheFront => fields.and(THE_POSITION_FIELD, AT_THE_FRONT_OF_THE_WAITING_LINE),
        WhereItGoes::After(job) => fields
            .and(THE_POSITION_FIELD, AFTER_A_JOB)
            .and(THE_JOB_IT_GOES_AFTER_FIELD, job.as_text()),
    };
    let request = Request::speaking(THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, command, name).carrying(fields);
    match asked_until_it_ends(machine, &request, THE_MOVES_OUTCOME_FIELD, wait) {
        // **Moved, or not moved**: the reading drawn next says which, and
        // where everything stands.
        HowTheAskEnded::AnOutcome(_, _) => WhatTheMoveCameTo::TheMachineAnswered,
        HowTheAskEnded::NotTaken(words) => WhatTheMoveCameTo::NotTaken(words),
        HowTheAskEnded::NotReached => WhatTheMoveCameTo::NotReached,
        HowTheAskEnded::TheWaiterGaveUp => WhatTheMoveCameTo::TheWaiterGaveUp,
    }
}

/// **The move begun**: the server found in the period-held readings — the
/// same finding the cancel and the group's cancel make — then the move on a
/// thread of its own (P5), and, where it reached the machine, **a fresh queue
/// reading taken over the same connection** so the list is drawn in the
/// machine's new order now rather than at the next P10d pass.
///
/// `after` is the job the row was dropped behind, or nothing for the front.
pub fn the_move_is_begun<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    job: Option<&str>,
    group: Option<&str>,
    after: Option<&str>,
) -> Result<(), NothingWasSent> {
    let (what, found) = match group {
        Some(group) => (
            WhatMoves::AGroup(
                Identity::read_back(IdentityKind::Group, group).ok_or(NothingWasSent::ItIsNotInTheQueue)?,
            ),
            what_the_act_is_for(app, TheQueuesOwnAct::TheGroupsCancel, None, Some(group))?,
        ),
        None => {
            let found = what_the_act_is_for(app, TheQueuesOwnAct::TheCancel, job, None)?;
            let (job, _) = found.jobs[0].clone();
            (WhatMoves::OneJob(job), found)
        }
    };
    let to = match after {
        None => WhereItGoes::AtTheFront,
        Some(after) => WhereItGoes::After(
            Identity::read_back(IdentityKind::Job, after).ok_or(NothingWasSent::ItIsNotInTheQueue)?,
        ),
    };
    let app = app.clone();
    std::thread::spawn(move || {
        let mut machine = crate::surface::servers::TheRemovalsOwnEnds { app: &app }.the_machine(&found.server);
        let came_to = the_move_is_sent(machine.as_mut(), &what, &to, TheWait::THE_STOPS_OWN);
        if came_to != WhatTheMoveCameTo::NotReached {
            the_queue_is_read_again(&app, &found.server, machine.as_mut());
        }
        crate::surface::the_queue::the_move_came_to(&app, matches!(what, WhatMoves::AGroup(_)), came_to);
    });
    Ok(())
}

// ════════════════════════════════════════════════════════════════════════════
// A live job's Telegram reporting changed — `Telegram` V1c, the twenty-first
// notes, note 7
// ════════════════════════════════════════════════════════════════════════════

/// **What a change of a live job's reporting came to.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheChangeCameTo {
    /// *Changed*, with the value the job record now holds, read off the
    /// acknowledgement — what the desk's own record takes.
    Changed(String),
    /// *Not changed*: the job had ended, or the machine holds no such job; the
    /// machine's own words where it gave some.
    NotChanged(Option<String>),
    /// Refused before it was recorded, or refused by name.
    NotTaken(Option<String>),
    NotReached,
    TheWaiterGaveUp,
}

/// **`Telegram` V1c's request, sent under a fresh identity and asked again
/// under it until its outcome is recorded** — the cancel's waiter. A first-frame
/// pair is sent as its held range job; the machine writes both.
pub fn the_reporting_change_is_sent(
    machine: &mut dyn AsksTheSupervisor,
    job: &Identity,
    choice: &str,
    wait: TheWait,
) -> WhatTheChangeCameTo {
    use smoozy_server_protocol::requests::{
        IT_WAS_CHANGED, THE_CHANGES_OUTCOME_FIELD, THE_REPORTING_CHOICE_FIELD,
        THE_TELEGRAM_CHOICE_IS_CHANGED_REQUEST,
    };
    let Ok(command) = Identity::issue(IdentityKind::Operation) else {
        return WhatTheChangeCameTo::NotReached;
    };
    let request =
        Request::speaking(THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, command, THE_TELEGRAM_CHOICE_IS_CHANGED_REQUEST)
            .carrying(
                Fields::none()
                    .and(THE_JOB_FIELD, job.as_text())
                    .and(THE_REPORTING_CHOICE_FIELD, choice),
            );
    match asked_until_it_ends_with_its_answer(machine, &request, THE_CHANGES_OUTCOME_FIELD, wait) {
        (HowTheAskEnded::AnOutcome(word, words), fields) if word == IT_WAS_CHANGED => {
            match fields.as_ref().and_then(|fields| fields.text(THE_REPORTING_CHOICE_FIELD)) {
                Some(held) => WhatTheChangeCameTo::Changed(held.to_string()),
                None => WhatTheChangeCameTo::NotChanged(words),
            }
        }
        (HowTheAskEnded::AnOutcome(_, words), _) => WhatTheChangeCameTo::NotChanged(words),
        (HowTheAskEnded::NotTaken(words), _) => WhatTheChangeCameTo::NotTaken(words),
        (HowTheAskEnded::NotReached, _) => WhatTheChangeCameTo::NotReached,
        (HowTheAskEnded::TheWaiterGaveUp, _) => WhatTheChangeCameTo::TheWaiterGaveUp,
    }
}

/// **The change begun**: the server found in the period-held readings, as the
/// cancel finds it, then the change on a thread of its own (P5). **On
/// *changed* the desk's own record takes the value the acknowledgement
/// carries** (record version 55), and the queue is drawn again; otherwise the
/// entry says what came of it, and the control goes back to what the record
/// holds.
pub fn the_reporting_change_is_begun<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    job: Option<&str>,
    choice: Option<&str>,
) -> Result<(), NothingWasSent> {
    let found = what_the_act_is_for(app, TheQueuesOwnAct::TheCancel, job, None)?;
    let (job, _) = found.jobs[0].clone();
    let choice = choice.ok_or(NothingWasSent::ItIsNotInTheQueue)?.to_string();
    let app = app.clone();
    std::thread::spawn(move || {
        let mut machine = crate::surface::servers::TheRemovalsOwnEnds { app: &app }.the_machine(&found.server);
        let came_to = the_reporting_change_is_sent(machine.as_mut(), &job, &choice, TheWait::THE_STOPS_OWN);
        if let WhatTheChangeCameTo::Changed(held) = &came_to {
            if let Some(started) = tauri::Manager::try_state::<Mutex<crate::start::Started>>(&app) {
                if let Ok(started) = started.lock() {
                    if let Some(record) = started.record.as_ref() {
                        let _ = crate::record::job_records::JobRecords::over(record.connection())
                            .the_reporting_choice_is_written(&job, held);
                    }
                }
            }
        }
        crate::surface::the_queue::the_reporting_change_came_to(&app, came_to);
    });
    Ok(())
}

/// **`Queue & History` A1's reading taken again over the act's own
/// connection**, and held where every other reading of that server is held —
/// the same request, the same stamping and the same holding as a P10d pass
/// (`crate::servers::the_machines_work::TheQueueOverThisServersPoint`). A
/// reading that does not come back leaves the last one standing.
fn the_queue_is_read_again<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    server: &str,
    machine: &mut dyn AsksTheSupervisor,
) {
    use smoozy_queue_history::acting::adoption::{take_the_reading, WhatTheRequestGave, WhetherTheInstallationConfirmed};
    let (Some(periods), Some(identity)) = (
        crate::surface::servers::the_periods_of(app),
        Identity::read_back(IdentityKind::LocalServer, server),
    ) else {
        return;
    };
    let holds = periods.the_machines_work();
    let Some(connection) = holds.the_connection_now(server) else {
        return;
    };
    let asking = crate::servers::the_machines_work::TheQueueOverThisServersPoint {
        machine: std::cell::RefCell::new(machine),
        the_connection_is_live: &|| true,
        the_reporting_read: std::cell::RefCell::new(None),
    };
    let gave = take_the_reading(&asking, &identity, WhetherTheInstallationConfirmed::ItIsTheSameInstallation);
    let the_reporting_read = asking.the_reporting_read.into_inner();
    let machine = asking.machine.into_inner();
    // **An ending the reading names is learned before it is held**, as on a
    // P10d pass, so a job that ended during the move is never in neither
    // list (`Queue & History` C11). Step 10c's *a finished job never missing
    // from both lists*.
    let Some(WhatTheRequestGave::ItAnswered(reading)) = gave else {
        return;
    };
    match tauri::Manager::try_state::<Mutex<crate::start::Started>>(app) {
        Some(started) => {
            let record = crate::keys::TheRunsRecord { started: started.inner() };
            // **`Telegram` V1d: what the reading said of reporting**, the
            // desk's before the reading is held, as on a P10d pass.
            if let Some(said) = the_reporting_read.as_ref() {
                let taken = crate::servers::the_machines_work::with_the_record(&record, |connection| {
                    crate::servers::the_machines_work::the_reporting_the_reading_carried_is_written(connection, said)
                });
                if taken == Some(true) {
                    crate::surface::telegram::the_section_is_told_again(app);
                }
            }
            let held = crate::servers::the_machines_work::a_reading_is_held_once_its_endings_are_learned(
                &identity, connection, &record, machine, holds, reading, None,
            );
            if held.applied_first > 0 {
                crate::surface::the_history::the_history_is_told_again(app);
            }
        }
        // **No record to learn into**: held as it came, as before.
        None => holds.a_reading_was_taken(server, connection, reading, None),
    }
}

// ════════════════════════════════════════════════════════════════════════════
// The application's ends
// ════════════════════════════════════════════════════════════════════════════

/// **Why nothing was sent at all**, said beside the control that was pressed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NothingWasSent {
    /// The job, or every member of the group, is gone from the reading.
    ItIsNotInTheQueue,
    /// A stop names a render, and the reading shows this job not rendering.
    ItIsNotRendering,
}

/// **What one act is sent for**, found in the period-held readings (`Queue &
/// History` C6) — the only place the desktop learns what is running.
struct WhatTheActIsFor {
    server: String,
    /// The render's operation identity, for the stop.
    the_render: Option<Identity>,
    /// Every job this act marks, and whether it is the one rendering.
    jobs: Vec<(Identity, bool)>,
}

fn what_the_act_is_for<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    act: TheQueuesOwnAct,
    job: Option<&str>,
    group: Option<&str>,
) -> Result<WhatTheActIsFor, NothingWasSent> {
    use smoozy_queue_history::line::QueueState;
    let periods = crate::surface::servers::the_periods_of(app).ok_or(NothingWasSent::ItIsNotInTheQueue)?;
    let every = periods.the_machines_work().every_server();
    match act {
        TheQueuesOwnAct::TheStop | TheQueuesOwnAct::TheCancel => {
            let job = job
                .and_then(|text| Identity::read_back(IdentityKind::Job, text))
                .ok_or(NothingWasSent::ItIsNotInTheQueue)?;
            for (server, period) in &every {
                let Some(reading) = period.reading.as_ref() else {
                    continue;
                };
                let Some(found) = reading.value().jobs.iter().find(|held| held.job == job) else {
                    continue;
                };
                let rendering = match &found.state {
                    QueueState::Rendering { operation } => Some(operation.clone()),
                    _ => None,
                };
                if act == TheQueuesOwnAct::TheStop && rendering.is_none() {
                    return Err(NothingWasSent::ItIsNotRendering);
                }
                return Ok(WhatTheActIsFor {
                    server: server.clone(),
                    jobs: vec![(job, rendering.is_some())],
                    the_render: rendering,
                });
            }
            Err(NothingWasSent::ItIsNotInTheQueue)
        }
        TheQueuesOwnAct::TheGroupsCancel => {
            let group = group
                .and_then(|text| Identity::read_back(IdentityKind::Group, text))
                .ok_or(NothingWasSent::ItIsNotInTheQueue)?;
            let started = tauri::Manager::try_state::<Mutex<crate::start::Started>>(app)
                .ok_or(NothingWasSent::ItIsNotInTheQueue)?;
            let started = started.lock().map_err(|_| NothingWasSent::ItIsNotInTheQueue)?;
            let record = started.record.as_ref().ok_or(NothingWasSent::ItIsNotInTheQueue)?;
            let store = crate::record::job_records::JobRecords::over(record.connection());
            // **`Queue & History` L20: membership is read off the members' own
            // local records**, joined to the reading by job identity.
            for (server, period) in &every {
                let Some(reading) = period.reading.as_ref() else {
                    continue;
                };
                let members: Vec<(Identity, bool)> = reading
                    .value()
                    .jobs
                    .iter()
                    .filter(|held| !matches!(held.state, QueueState::Ended { .. }))
                    .filter(|held| {
                        store
                            .read(&held.job)
                            .ok()
                            .flatten()
                            .is_some_and(|local| local.group.as_ref() == Some(&group))
                    })
                    .map(|held| (held.job.clone(), matches!(held.state, QueueState::Rendering { .. })))
                    .collect();
                if members.is_empty() {
                    continue;
                }
                // **The render the reading last showed, never a target**
                // (`Queue & History` L13) — S2 carries it the same way.
                let the_render = reading.value().jobs.iter().find_map(|held| match &held.state {
                    QueueState::Rendering { operation } => Some(operation.clone()),
                    _ => None,
                });
                return Ok(WhatTheActIsFor {
                    server: server.clone(),
                    the_render,
                    jobs: members,
                });
            }
            Err(NothingWasSent::ItIsNotInTheQueue)
        }
    }
}

/// **The act begun**: each job it covers marked in hand before anything is
/// sent, then the act on a thread of its own — `Rendering` K9's wait is up to
/// two minutes, and P5 keeps the command that pressed it off it.
///
/// **When it ends**, where the act went out at all, one read-forward pass is
/// run over the same connection, so the job's ending or its removal reaches the
/// local record now rather than at the next pass (`State, identity and
/// recovery` §7 step 3, and S2's SC-1 event); **a cancel answered *removed*
/// takes the local record itself** (`Queue & History` L2's *on the
/// acknowledgement*). Then both screens and the work column are drawn again.
pub fn the_act_is_begun<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    act: TheQueuesOwnAct,
    job: Option<&str>,
    group: Option<&str>,
) -> Result<(), NothingWasSent> {
    let what = what_the_act_is_for(app, act, job, group)?;
    if what.jobs.iter().any(|(job, _)| it_is_in_hand(&job.as_text())) {
        return Ok(());
    }
    for (job, rendering) in &what.jobs {
        let in_hand = match act {
            TheQueuesOwnAct::TheGroupsCancel if *rendering => TheQueuesOwnAct::TheStop,
            TheQueuesOwnAct::TheGroupsCancel => TheQueuesOwnAct::TheCancel,
            other => other,
        };
        it_stands(&job.as_text(), Some(WhereTheActStands::InHand(in_hand)));
    }
    let app = app.clone();
    let group = group.and_then(|text| Identity::read_back(IdentityKind::Group, text));
    std::thread::spawn(move || {
        // **The machine, asked through the server's own point** — S2's one
        // composition of it, which the removal asks through too.
        let mut machine = crate::surface::servers::TheRemovalsOwnEnds { app: &app }.the_machine(&what.server);
        let mut removed: Vec<Identity> = Vec::new();
        let mut it_went_out = true;
        match act {
            TheQueuesOwnAct::TheStop => {
                let (job, _) = &what.jobs[0];
                let stands = match &what.the_render {
                    Some(render) => the_stop_is_sent(machine.as_mut(), render, TheWait::THE_STOPS_OWN),
                    None => Some(WhereTheActStands::NotReached(act)),
                };
                it_went_out = !matches!(stands, Some(WhereTheActStands::NotReached(_)));
                it_stands(&job.as_text(), stands);
            }
            TheQueuesOwnAct::TheCancel => {
                let (job, _) = &what.jobs[0];
                let stands = the_cancel_is_sent(machine.as_mut(), job, TheWait::THE_STOPS_OWN);
                it_went_out = !matches!(stands, Some(WhereTheActStands::NotReached(_)));
                if stands.is_none() {
                    removed.push(job.clone());
                }
                it_stands(&job.as_text(), stands);
            }
            TheQueuesOwnAct::TheGroupsCancel => {
                use crate::servers::stop_and_clear::{the_pair_is_sent, WhatThePairCarries};
                let came_to = match tauri::Manager::try_state::<Mutex<crate::start::Started>>(&app) {
                    Some(started) => {
                        let record = crate::keys::TheRunsRecord { started: started.inner() };
                        let mut keeper = crate::keys::TheOutcomeRecordsLent(&record);
                        let carries = WhatThePairCarries {
                            group: group.clone(),
                            the_render_the_desktop_last_saw: what.the_render.clone(),
                        };
                        the_pair_is_sent(&mut keeper, machine.as_mut(), &carries, &[], TheWait::THE_STOPS_OWN)
                            .came_to
                    }
                    None => WhatThePairCameTo::NothingWasChanged,
                };
                for (job, rendering) in &what.jobs {
                    it_stands(&job.as_text(), what_the_groups_cancel_leaves(came_to, *rendering));
                }
            }
        }
        the_result_is_presented(
            || {
                if it_went_out {
                    the_machines_journal_is_applied(&app, &what.server, machine.as_mut(), &removed);
                }
            },
            || {
                crate::surface::the_queue::the_queue_is_told_again(&app);
                crate::surface::the_history::the_history_is_told_again(&app);
                crate::surface::servers::the_work_column_is_told_again(&app);
            },
        );
    });
    Ok(())
}

// `Rendering` K10's deadline includes telling the screen. A journal read
// after the wait can meet the same silent line, so show the result first;
// then show any later outcome the journal supplied. Sitting B, 2026-10-21.
fn the_result_is_presented(follow_up: impl FnOnce(), tell: impl Fn()) {
    tell();
    follow_up();
    tell();
}

/// **`Queue & History` L2 on the acknowledgement, then one read-forward
/// pass.** A local record the answer said is gone is removed only where it is
/// this server's.
fn the_machines_journal_is_applied<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    server: &str,
    machine: &mut dyn AsksTheSupervisor,
    removed: &[Identity],
) {
    let (Some(started), Some(periods), Some(identity)) = (
        tauri::Manager::try_state::<Mutex<crate::start::Started>>(app),
        crate::surface::servers::the_periods_of(app),
        Identity::read_back(IdentityKind::LocalServer, server),
    ) else {
        return;
    };
    if !removed.is_empty() {
        if let Ok(started) = started.lock() {
            if let Some(record) = started.record.as_ref() {
                let store = crate::record::job_records::JobRecords::over(record.connection());
                for job in removed {
                    if let Ok(Some(held)) = store.read(job) {
                        if held.server.local_server == identity {
                            let _ = store.remove(job);
                        }
                    }
                }
            }
        }
    }
    let record = crate::keys::TheRunsRecord { started: started.inner() };
    let _ = crate::servers::the_machines_work::a_read_forward_pass(
        &identity,
        &record,
        machine,
        periods.the_machines_work(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::servers::stop_and_clear::tests::QUICK;
    use smoozy_server_protocol::framing::Answered;
    use smoozy_server_protocol::requests::THE_STOP_FAILED;

    /// **A stand-in supervisor answering one kind of request by identity**:
    /// *still in hand* for `in_hand` asks, then the scripted answer.
    struct AMachine {
        answers: Answers,
        in_hand: usize,
        asked: Vec<Request>,
    }

    #[derive(Clone)]
    enum Answers {
        AnOutcome(&'static str, &'static str, Option<&'static str>),
        RefusedByName,
        NoRecord(&'static str),
        Silence,
        NeverAsked,
    }

    impl AMachine {
        fn answering(answers: Answers, in_hand: usize) -> Self {
            AMachine { answers, in_hand, asked: Vec::new() }
        }
    }

    impl AsksTheSupervisor for AMachine {
        fn ask(&mut self, request: &Request) -> WhatCameBack {
            if let Some(first) = self.asked.first() {
                assert_eq!(first.identity, request.identity, "asked again under a fresh identity");
            }
            self.asked.push(request.clone());
            let answered = |fields: Fields| {
                WhatCameBack::ItAnswered(
                    Answered::to(request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, "test").carrying(fields),
                )
            };
            match self.answers.clone() {
                Answers::RefusedByName => WhatCameBack::ItAnswered(
                    Answered::to(request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, "0.1.6")
                        .saying_it_does_not_answer_that_request(&request.name),
                ),
                Answers::NoRecord(words) => answered(
                    Fields::none()
                        .and(WHAT_BECAME_OF_IT_FIELD, THERE_IS_NO_RECORD_OF_IT)
                        .and(THE_MACHINES_OWN_WORDS_FIELD, words),
                ),
                Answers::Silence => WhatCameBack::NothingCorrelatedCameBack,
                Answers::NeverAsked => WhatCameBack::ItWasNeverAsked,
                Answers::AnOutcome(_, _, _) if self.asked.len() <= self.in_hand => {
                    answered(Fields::none().and(WHAT_BECAME_OF_IT_FIELD, IT_IS_STILL_IN_HAND))
                }
                Answers::AnOutcome(field, word, words) => {
                    let mut fields = Fields::none()
                        .and(WHAT_BECAME_OF_IT_FIELD, IT_HAS_A_RECORDED_OUTCOME)
                        .and(field, word);
                    if let Some(words) = words {
                        fields = fields.and(THE_MACHINES_OWN_WORDS_FIELD, words);
                    }
                    answered(fields)
                }
            }
        }
    }

    fn a_render() -> Identity {
        Identity::issue(IdentityKind::Operation).unwrap()
    }

    /// **`Server protocol and transfers` C21 and `Rendering` K8: the stop
    /// names the render, is asked again under its own identity while it is
    /// in hand, and *left stopped*, *its own ending* and *nothing was
    /// rendering* are all the stop done.**
    #[test]
    fn the_stop_names_the_render_and_is_asked_again_until_its_outcome_lands() {
        let render = a_render();
        let mut machine = AMachine::answering(
            Answers::AnOutcome(THE_STOPS_OUTCOME_FIELD, IT_WAS_LEFT_STOPPED, None),
            3,
        );
        assert_eq!(the_stop_is_sent(&mut machine, &render, QUICK), None);
        assert_eq!(machine.asked.len(), 4, "asked again while in hand");
        assert_eq!(machine.asked[0].name, THE_STOP_REQUEST);
        assert_eq!(
            machine.asked[0].fields.text(THE_OPERATION_TO_STOP_FIELD),
            Some(render.as_text().as_str())
        );
        for word in [ITS_OWN_ENDING, NOTHING_WAS_RENDERING] {
            let mut machine = AMachine::answering(Answers::AnOutcome(THE_STOPS_OUTCOME_FIELD, word, None), 0);
            assert_eq!(the_stop_is_sent(&mut machine, &render, QUICK), None, "{word}");
        }
    }

    /// **Every way a stop does not end the render, each told apart.**
    /// `Rendering` K10: the waiter giving up is its own, never *failed*.
    #[test]
    fn a_stop_that_does_not_end_the_render_says_which_way() {
        let render = a_render();
        let run = |answers: Answers| the_stop_is_sent(&mut AMachine::answering(answers, usize::MAX), &render, QUICK);
        let act = TheQueuesOwnAct::TheStop;
        assert_eq!(run(Answers::AnOutcome(THE_STOPS_OUTCOME_FIELD, "x", None)), Some(WhereTheActStands::TheWaiterGaveUp(act)));
        let stands = the_stop_is_sent(
            &mut AMachine::answering(Answers::AnOutcome(THE_STOPS_OUTCOME_FIELD, THE_STOP_FAILED, Some("the disk is full")), 0),
            &render,
            QUICK,
        );
        assert_eq!(stands, Some(WhereTheActStands::TheStopFailed(Some("the disk is full".to_string()))));
        assert_eq!(run(Answers::RefusedByName), Some(WhereTheActStands::NotTaken(act, None)));
        assert_eq!(
            run(Answers::NoRecord("this machine holds no render under that identity")),
            Some(WhereTheActStands::NotTaken(
                act,
                Some("this machine holds no render under that identity".to_string())
            ))
        );
        assert_eq!(run(Answers::Silence), Some(WhereTheActStands::TheWaiterGaveUp(act)));
        assert_eq!(run(Answers::NeverAsked), Some(WhereTheActStands::NotReached(act)));
    }

    /// **`Queue & History` L1, L2 and L4: the cancel names the job; *removed*
    /// is the only answer that lets the local record go, and *not cancelled*
    /// carries the machine's own words.**
    #[test]
    fn a_cancel_is_removed_or_not_cancelled_with_the_machines_words() {
        let job = Identity::issue(IdentityKind::Job).unwrap();
        let mut machine = AMachine::answering(Answers::AnOutcome(THE_CANCELS_OUTCOME_FIELD, IT_WAS_REMOVED, None), 1);
        assert_eq!(the_cancel_is_sent(&mut machine, &job, QUICK), None);
        assert_eq!(machine.asked[0].name, THE_CANCEL_REQUEST);
        assert_eq!(machine.asked[0].fields.text(THE_JOB_FIELD), Some(job.as_text().as_str()));

        let words = "it is rendering, and a rendering job is stopped rather than cancelled";
        let mut machine = AMachine::answering(
            Answers::AnOutcome(THE_CANCELS_OUTCOME_FIELD, IT_WAS_NOT_CANCELLED, Some(words)),
            0,
        );
        assert_eq!(
            the_cancel_is_sent(&mut machine, &job, QUICK),
            Some(WhereTheActStands::NotCancelled(Some(words.to_string())))
        );
        let mut machine = AMachine::answering(Answers::RefusedByName, 0);
        assert_eq!(
            the_cancel_is_sent(&mut machine, &job, QUICK),
            Some(WhereTheActStands::NotTaken(TheQueuesOwnAct::TheCancel, None))
        );
    }

    /// **`Telegram` V1c: the change names the job and the value; *changed*
    /// carries the value the machine's record now holds, which is what the
    /// desk's record takes; an ended job is *not changed* with the machine's
    /// words; an older program refuses it by name.** The twenty-first notes,
    /// note 7.
    #[test]
    fn twenty_first_7_the_change_carries_back_what_the_record_holds() {
        use smoozy_server_protocol::requests::{
            IT_WAS_CHANGED, IT_WAS_NOT_CHANGED, THE_CHANGES_OUTCOME_FIELD, THE_REPORTING_CHOICE_FIELD,
            THE_TELEGRAM_CHOICE_IS_CHANGED_REQUEST,
        };
        struct Changing {
            word: &'static str,
            holds: Option<&'static str>,
            asked: Vec<Request>,
        }
        impl AsksTheSupervisor for Changing {
            fn ask(&mut self, request: &Request) -> WhatCameBack {
                self.asked.push(request.clone());
                let mut fields = Fields::none()
                    .and(WHAT_BECAME_OF_IT_FIELD, IT_HAS_A_RECORDED_OUTCOME)
                    .and(THE_CHANGES_OUTCOME_FIELD, self.word);
                if let Some(holds) = self.holds {
                    fields = fields.and(THE_REPORTING_CHOICE_FIELD, holds);
                } else {
                    fields = fields.and(THE_MACHINES_OWN_WORDS_FIELD, "it has already ended");
                }
                WhatCameBack::ItAnswered(
                    Answered::to(request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, "test").carrying(fields),
                )
            }
        }
        let job = Identity::issue(IdentityKind::Job).unwrap();
        let mut machine = Changing { word: IT_WAS_CHANGED, holds: Some("\"Nothing\""), asked: Vec::new() };
        assert_eq!(
            the_reporting_change_is_sent(&mut machine, &job, "\"Nothing\"", QUICK),
            WhatTheChangeCameTo::Changed("\"Nothing\"".to_string())
        );
        assert_eq!(machine.asked[0].name, THE_TELEGRAM_CHOICE_IS_CHANGED_REQUEST);
        assert_eq!(machine.asked[0].fields.text(THE_JOB_FIELD), Some(job.as_text().as_str()));
        assert_eq!(machine.asked[0].fields.text(THE_REPORTING_CHOICE_FIELD), Some("\"Nothing\""));
        let mut machine = Changing { word: IT_WAS_NOT_CHANGED, holds: None, asked: Vec::new() };
        assert_eq!(
            the_reporting_change_is_sent(&mut machine, &job, "\"Everything\"", QUICK),
            WhatTheChangeCameTo::NotChanged(Some("it has already ended".to_string()))
        );
        let mut machine = AMachine::answering(Answers::RefusedByName, 0);
        assert_eq!(
            the_reporting_change_is_sent(&mut machine, &job, "\"Everything\"", QUICK),
            WhatTheChangeCameTo::NotTaken(None)
        );
    }

    /// **`Queue & History` L19 through S2's result**, member by member.
    #[test]
    fn a_groups_cancel_leaves_each_member_where_the_pair_came_to() {
        use WhatThePairCameTo::*;
        assert_eq!(what_the_groups_cancel_leaves(ClearedAndStopped, true), None);
        assert_eq!(what_the_groups_cancel_leaves(ClearedAndStopped, false), None);
        assert_eq!(
            what_the_groups_cancel_leaves(TheRenderHadNotStopped, true),
            Some(WhereTheActStands::TheWaiterGaveUp(TheQueuesOwnAct::TheStop))
        );
        assert_eq!(what_the_groups_cancel_leaves(TheRenderHadNotStopped, false), None);
        assert_eq!(
            what_the_groups_cancel_leaves(NothingCameBack, false),
            Some(WhereTheActStands::TheWaiterGaveUp(TheQueuesOwnAct::TheGroupsCancel)),
            "nothing came back is never said as nothing changed"
        );
        assert_eq!(
            what_the_groups_cancel_leaves(TheClearWasRefused, false),
            Some(WhereTheActStands::TheClearWasRefused)
        );
        assert_eq!(
            what_the_groups_cancel_leaves(NothingWasChanged, true),
            Some(WhereTheActStands::NotTaken(TheQueuesOwnAct::TheGroupsCancel, None))
        );
    }

    /// **`Queue & History` L6 and L7, the 38's piece 8: a move names the job
    /// or the group and the position — after a job, or the front — and never
    /// an index**; it is asked again under its own identity while it is in
    /// hand, and every way it can end is told apart.
    #[test]
    fn a_move_names_what_moves_and_where_and_says_how_it_ended() {
        use smoozy_server_protocol::requests::{IT_WAS_MOVED, IT_WAS_NOT_MOVED};
        let job = Identity::issue(IdentityKind::Job).unwrap();
        let before = Identity::issue(IdentityKind::Job).unwrap();
        let group = Identity::issue(IdentityKind::Group).unwrap();

        let mut machine = AMachine::answering(Answers::AnOutcome(THE_MOVES_OUTCOME_FIELD, IT_WAS_MOVED, None), 2);
        let came_to = the_move_is_sent(&mut machine, &WhatMoves::OneJob(job.clone()), &WhereItGoes::AtTheFront, QUICK);
        assert_eq!(came_to, WhatTheMoveCameTo::TheMachineAnswered);
        assert_eq!(machine.asked.len(), 3, "asked again while in hand");
        let asked = &machine.asked[0];
        assert_eq!(asked.name, THE_REORDER_REQUEST);
        assert_eq!(asked.fields.text(THE_JOB_FIELD), Some(job.as_text().as_str()));
        assert_eq!(asked.fields.text(THE_POSITION_FIELD), Some(AT_THE_FRONT_OF_THE_WAITING_LINE));
        assert_eq!(asked.fields.text(THE_JOB_IT_GOES_AFTER_FIELD), None);

        let mut machine = AMachine::answering(Answers::AnOutcome(THE_MOVES_OUTCOME_FIELD, IT_WAS_NOT_MOVED, Some("it is rendering")), 0);
        let came_to = the_move_is_sent(&mut machine, &WhatMoves::AGroup(group.clone()), &WhereItGoes::After(before.clone()), QUICK);
        assert_eq!(came_to, WhatTheMoveCameTo::TheMachineAnswered, "the reading drawn next says it was not moved");
        let asked = &machine.asked[0];
        assert_eq!(asked.name, THE_GROUP_BLOCK_MOVE_REQUEST);
        assert_eq!(asked.fields.text(THE_GROUP_FIELD), Some(group.as_text().as_str()));
        assert_eq!(asked.fields.text(THE_POSITION_FIELD), Some(AFTER_A_JOB));
        assert_eq!(asked.fields.text(THE_JOB_IT_GOES_AFTER_FIELD), Some(before.as_text().as_str()));

        let run = |answers: Answers| {
            the_move_is_sent(&mut AMachine::answering(answers, usize::MAX), &WhatMoves::OneJob(job.clone()), &WhereItGoes::AtTheFront, QUICK)
        };
        assert_eq!(run(Answers::RefusedByName), WhatTheMoveCameTo::NotTaken(None));
        assert_eq!(run(Answers::NeverAsked), WhatTheMoveCameTo::NotReached);
        assert_eq!(run(Answers::Silence), WhatTheMoveCameTo::TheWaiterGaveUp);
        assert_eq!(
            run(Answers::AnOutcome(THE_MOVES_OUTCOME_FIELD, IT_WAS_MOVED, None)),
            WhatTheMoveCameTo::TheWaiterGaveUp,
            "still in hand when the waiter gives up is never said as moved"
        );
    }

    /// The shortened two-minute wait ends while the line is silent. Its
    /// screen event must precede the follow-up's unanswered journal request.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_stops_expired_wait_is_shown_before_a_silent_journal_read_ends() {
        use crate::servers::sessions::tests::{a_point_lending, additional_connections_to, AMachine as ThePointsMachine};
        use crate::servers::stop_and_clear::tests::{the_machine_over, SHORTENED, ON_TIME};
        use std::sync::atomic::Ordering;
        use std::time::Duration;

        let machine = ThePointsMachine::with_a_limit_of(10);
        let point = a_point_lending(&machine, additional_connections_to(&machine, false)).await;
        // Four replies, then silence just before the wait's own boundary.
        machine.the_supervisor_is_silent_from.store(5, Ordering::SeqCst);
        let mut over = the_machine_over(&point);
        let job = Identity::issue(IdentityKind::Job).unwrap();
        let render = Identity::issue(IdentityKind::Operation).unwrap();
        let (told, heard) = std::sync::mpsc::channel();
        let (release, released) = std::sync::mpsc::channel();
        let began = Instant::now();
        let running = tokio::task::spawn_blocking(move || {
            it_stands(&job.as_text(), Some(WhereTheActStands::InHand(TheQueuesOwnAct::TheStop)));
            let stands = the_stop_is_sent(&mut over, &render, SHORTENED);
            it_stands(&job.as_text(), stands);
            the_result_is_presented(
                || { let _ = released.recv_timeout(Duration::from_secs(3)); },
                || { let _ = told.send(where_the_act_stands(&job.as_text())); },
            );
            it_stands(&job.as_text(), None);
        });
        let got = tokio::task::spawn_blocking(move || heard.recv_timeout(SHORTENED.deadline + ON_TIME)).await.unwrap();
        let elapsed = began.elapsed();
        // Always release and join, including on the old ordering's failure.
        let _ = release.send(());
        running.await.unwrap();
        point.ends().await;
        assert_eq!(got.ok(), Some(Some(WhereTheActStands::TheWaiterGaveUp(TheQueuesOwnAct::TheStop))), "no screen event at the deadline ({elapsed:?})");
    }

    /// **`Rendering` K8 to K9 and `Server protocol and transfers` C16 to C17,
    /// under the dead-line sweep's two-minute-wait repair: the user's wait ends at its own
    /// deadline though the last ask meets a silent line, and a success that
    /// lands after it is still taken.**
    ///
    /// **The route is the application's own**: the queue's cancel, asked
    /// through the server's own point (`crate::servers::the_machines_work::OverThisServersPoint`)
    /// over the point's stand-in, whose supervisor answers each ask with
    /// nothing C8 names — *still in hand* — until the fifth, begun just before
    /// the shortened deadline, which meets a silent line.
    ///
    /// **The case it bites on.** Until the repair the deadline was looked at
    /// only after an ask returned and every ask was made whole, so the fifth
    /// ask held the user's wait for as long as the line said nothing — a
    /// whole break window on a rental, and for good here.
    ///
    /// **What a late success is then**: the cancel's waiter gave up, which
    /// writes nothing and removes nothing (`Server protocol and transfers`
    /// C17), so the job's local record stands; the machine removes the job
    /// afterwards, and the next read-forward pass reads that off the journal
    /// and takes the local record (`Queue & History` L2's *on the journal
    /// event*), exactly as it takes any removal.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn the_users_wait_ends_at_its_own_deadline_and_a_late_cancel_is_still_taken() {
        use crate::servers::sessions::tests::{a_point_lending, additional_connections_to, AMachine as ThePointsMachine};
        use crate::servers::stop_and_clear::tests::{the_machine_over, watched_for, ON_TIME, SHORTENED};
        use crate::servers::the_machines_work::{a_read_forward_pass, WhatThisRunHolds};
        use smoozy_queue_history::record::Standing;
        use smoozy_server_protocol::requests as asked;
        use std::sync::atomic::Ordering;

        let machine = ThePointsMachine::with_a_limit_of(10);
        let point = a_point_lending(&machine, additional_connections_to(&machine, false)).await;
        machine.the_supervisor_is_silent_from.store(5, Ordering::SeqCst);

        // **The job, in this computer's record**, on a server of its own.
        let mut path = std::env::temp_dir();
        path.push(format!("smoozy-a-late-cancel-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).unwrap();
        let record = crate::record::LocalRecord::open_at(&path.join("local-record.sqlite3")).unwrap();
        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        crate::record::server_rows::add(
            record.connection(),
            &smoozy_servers::store::ServerRow::saved(
                server.as_text(),
                smoozy_servers::identity::Host::what_the_user_typed("127.0.0.1").unwrap(),
                smoozy_servers::identity::Port::what_the_user_typed(22).unwrap(),
                smoozy_servers::identity::Username::what_the_user_typed("root").unwrap(),
                smoozy_servers::identity::KeyReference {
                    local_key_identifier: "key-1".into(),
                    fingerprint: "SHA256:aaa".into(),
                },
            ),
        )
        .unwrap();
        let store = crate::record::job_records::JobRecords::over(record.connection());
        let mut held = crate::record::deletions::tests::a_record_in_history(&server, "Kitchen — final v4");
        held.standing = Standing::Confirmed;
        held.the_render_start_has_been_learned = false;
        store.write_the_provisional_record(&held).unwrap();
        let job = held.job.clone();

        // **The cancel, waited on by a shortened waiter.**
        let began = std::time::Instant::now();
        let mut over = the_machine_over(&point);
        let cancelled = job.clone();
        let came = watched_for(std::time::Duration::from_secs(10), move || {
            the_cancel_is_sent(&mut over, &cancelled, SHORTENED)
        })
        .await;
        let (stands, took) = came.expect("the user's wait went on past its own deadline");
        assert!(took < SHORTENED.deadline + ON_TIME, "the user's wait took {took:?}");
        assert_eq!(stands, Some(WhereTheActStands::TheWaiterGaveUp(TheQueuesOwnAct::TheCancel)));
        let asked_at = machine.supervisor_asked_at.lock().unwrap().clone();
        assert_eq!(asked_at.len(), 5, "the last ask, begun just before the deadline, met no silence");
        assert!(
            asked_at.iter().all(|at| at.duration_since(began) < SHORTENED.deadline),
            "an ask was begun after the deadline"
        );
        // **Nothing was written or removed by the give-up**, and the primary
        // is still lending.
        assert!(store.read(&job).unwrap().is_some(), "the waiter's expiry took the local record");
        assert!(point.sessions.a_connection_is_live());
        point.ends().await;

        // **The machine carries the cancel out afterwards**, and the next
        // read-forward pass takes it from the journal.
        struct TheJournal {
            job: Identity,
            read: bool,
        }
        impl AsksTheSupervisor for TheJournal {
            fn ask(&mut self, request: &Request) -> WhatCameBack {
                let answered = Answered::to(request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, "test");
                let fields = match request.name.as_str() {
                    asked::THE_READ_FORWARD_REQUEST => {
                        let from: u64 = request.fields.text(asked::THE_CURSOR_FIELD).unwrap().parse().unwrap();
                        let events: &[()] = if self.read { &[] } else { &[()] };
                        self.read = true;
                        let mut fields = Fields::none()
                            .and(asked::HOW_MANY_EVENTS_FIELD, events.len().to_string())
                            .and(asked::THE_NEXT_SEQUENCE_FIELD, (from + events.len() as u64).to_string())
                            .and(asked::THE_OLDEST_RETAINED_FIELD, "0");
                        if !events.is_empty() {
                            let item = |field| asked::the_field_of_item(asked::AN_EVENT, 0, field);
                            fields = fields
                                .and(&item(asked::THE_SEQUENCE_FIELD), from.to_string())
                                .and(&item(asked::THE_KIND_FIELD), asked::THE_JOB_RECORD_REMOVED_EVENT)
                                .and(&item(asked::THE_ABOUT_FIELD), self.job.as_text())
                                .and(&item(asked::THE_SAYS_FIELD), asked::IT_WAS_REMOVED);
                        }
                        fields
                    }
                    asked::THE_CURSOR_MOVES_FORWARD_REQUEST => Fields::none(),
                    other => return WhatCameBack::ItAnswered(answered.saying_it_does_not_answer_that_request(other)),
                };
                WhatCameBack::ItAnswered(answered.carrying(fields))
            }
        }
        let holds = WhatThisRunHolds::default();
        let lent = crate::keys::TheRecordItself::over(record.connection());
        a_read_forward_pass(&server, &lent, &mut TheJournal { job: job.clone(), read: false }, &holds);
        assert!(store.read(&job).unwrap().is_none(), "the late cancel was not taken from the journal");
        drop(record);
        let _ = std::fs::remove_dir_all(&path);
    }

    /// **The run's knowledge of its own act, by job**, and one press at a
    /// time.
    #[test]
    fn an_act_in_hand_is_held_by_job_and_goes_when_it_ends() {
        let job = Identity::issue(IdentityKind::Job).unwrap().as_text();
        assert_eq!(where_the_act_stands(&job), None);
        it_stands(&job, Some(WhereTheActStands::InHand(TheQueuesOwnAct::TheStop)));
        assert!(it_is_in_hand(&job));
        it_stands(&job, Some(WhereTheActStands::TheWaiterGaveUp(TheQueuesOwnAct::TheStop)));
        assert!(!it_is_in_hand(&job), "a new stop can be sent once the waiter gave up");
        it_stands(&job, None);
        assert_eq!(where_the_act_stands(&job), None);
    }
}

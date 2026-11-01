//! **Routing, naming and the typed requests — `Telegram` §5, R1 to R15 with
//! R1a.** How a tap reaches the machine that owns the job, how a reply or a
//! typed request is answered, and how *how is it going* becomes one message.
//!
//! **Where it meets the slot and the board.** The listener (piece 2) hands
//! this module what the conversation sends and what the board addresses to
//! this machine, through [`TheRoutersHands`], which is its
//! [`crate::listener::HandsOn`]. This module hands back what this machine
//! proposes, replaces and replies through [`TheRouter::before_each_turn`],
//! called before every turn. **Nothing here edits the board**: every entry
//! reaches it through the listener's own proposal machinery (B8, B15).
//!
//! **Markers in this folder are all `Telegram`'s**: R1 to R15 and R1a, with B8,
//! B12, B15 and F3 as they bind here.
//!
//! - **R1** — the token (`token.rs`), the record and resolution that fails
//!   closed (`context.rs`), the expiry answered ([`Routing::ended`]).
//! - **R1a** — `context.rs`, [`resolve_a_reply`]; the fallback on expiry here.
//! - **R2** — `context.rs`: every instruction is addressed to the owning
//!   machine; *holder* never names the machine holding a job.
//! - **R3** — `typed.rs`'s answers, sent from here.
//! - **R4** — `context.rs`, [`a_taps_resolution`]: the press's own identifier
//!   seeds the operation.
//! - **R5** — stated, not promised: nothing here waits on a measured round
//!   trip, and no deadline is a multiple of one. Anything slower is B12's.
//! - **R6** — nothing here assumes the answer reaches the machine that asked:
//!   whichever machine holds composes it, from the board.
//! - **R7, R8, R9** — `gather.rs`: labels are display, identities route.
//! - **R10, R11, R12** — `gather.rs` and the gather here.
//! - **R13** — the disambiguation here; **R14** — the routed frame request
//!   (`context.rs`, [`a_frame_request`]); what the owner answers from what it
//!   holds is `Telegram` I2, I4 and I5, piece 6's and step 8b's.
//! - **R15** — `typed.rs`: four requests, each also a command (R17).
//! - **R13a** — `frame` alone and `stop`, over R13's disambiguation, here.
//! - **R1a, widened 2026-10-31, and R1b** — a reply's `frame`, `stop` and
//!   `reporting` boarded by the replied-to message, the at-once checks, the
//!   fallback for `frame` and the fail-closed answer for the other two, here;
//!   resolution in `context.rs`; every message this listener sends
//!   registered as about no job ([`Routing::send`]).
//! - **V1d** — the ask, the round (a gather first, then one instruction per
//!   machine that can take it) and its one reply, here.
//!
//! **F3 — an update is confirmed only after its work is done**, and for a
//! routed tap, a reply or a gather the work is its entries being present on a
//! board this machine has read. So [`TheRoutersHands`] gives the listener back
//! a refusal until a completed read shows them, and the update stays
//! unconfirmed and comes back. A truthful answer's work is its being sent.

pub mod context;
pub mod gather;
pub mod token;
pub mod typed;

use crate::job_messages::{AMarkedText, AServerLabel};
use std::collections::{BTreeMap, HashSet};
use std::sync::mpsc::TryRecvError;
use std::sync::{Arc, Mutex, MutexGuard};

use serde_json::{json, Value};

use crate::board::instructions::{AKindOfEntry, AnEntry, TheAddressee};
use crate::board::text::TheBoard;
use crate::listener::{HandsOn, Notes, ReadsItsOwnClock, TheListener, WhoThisMachineIs};
use crate::pacing::{AClassOfSend, HowASendEnded, TheSendQueue};
use crate::transport::{TheClient, WhyTheCallFailed};
use crate::worker_state::TheTelegramWorkersState;

pub use context::{a_frame_request, a_replys_resolution, a_taps_resolution, resolve_a_reply, resolve_a_tap};
use context::{
    an_action_on_one_job, ARepliedRequest, AReplysEvidence, ATapsEvidence, AnActionContextRecord, WhyItFailsClosed,
    CHOOSING_THE_PHONES_CHOICE, EVERY_LIVE_JOBS_REPORTING, SENDING_A_FRAME,
};
use gather::{a_state_request, how_it_is_going, the_candidates, the_gather_for, the_gather_of, AReply, ReadsWhatThisMachineHolds, WhatAGatherIsFor};
use token::WhatATokenNames;
use typed::{ATypedRequest, WhatAnUpdateAsks};
use crate::job_messages::TheReportingChoice;
use crate::worker_state::{ASentMessage, WhatAMessageIsAbout};

/// **`Telegram` V1d — the ask's words**, above its five buttons, and the line
/// under them.
pub const THE_PHONES_ASK: &str = "Reporting for every job and for new jobs:";
pub const FOR_OTHER_MINUTES: &str = "For other minutes, send /reporting 10.";

/// **`Telegram` V1d — the five choices, in the Render Settings list's order**,
/// the third at five minutes.
pub const THE_FIVE_CHOICES: [TheReportingChoice; 5] = [
    TheReportingChoice::Everything,
    TheReportingChoice::ThePicturesWithoutTheRunningCommentary,
    TheReportingChoice::APictureEveryFewMinutes { minutes: 5 },
    TheReportingChoice::OnlyWhatNeedsAnAnswer,
    TheReportingChoice::Nothing,
];

/// **`Telegram` V1d — the reply's words**: what was set, then a line for each
/// machine not reached and each that needs an update, only where any applies.
pub fn the_phones_choice_was_set(choice: TheReportingChoice, not_reached: &[AServerLabel], needs_an_update: &[AServerLabel]) -> String {
    let mut text = format!("Reporting set to \u{201c}{}\u{201d} for every job and new jobs.", choice.as_the_phone_says());
    let names = |labels: &[AServerLabel]| labels.iter().map(|l| l.to_string()).collect::<Vec<_>>().join(", ");
    if !not_reached.is_empty() {
        let theirs = if not_reached.len() == 1 { "Its jobs keep their choice." } else { "Their jobs keep their choice." };
        text.push_str(&format!("\nNot reached: {}. {theirs}", names(not_reached)));
    }
    for older in needs_an_update {
        text.push_str(&format!("\n{older} needs a Smoozy update for this."));
    }
    text
}

/// **One V1d round, as the machine that started it knows it.**
#[derive(Debug, Clone)]
struct AReportingRound {
    choice: TheReportingChoice,
    /// The operation's identity, the same on every machine told.
    operation: String,
    date: i64,
    update: i64,
    /// Machines that did not answer the gather, and those whose program is
    /// older than V1d: named in the reply, and never told.
    not_reached: Vec<AServerLabel>,
    needs_an_update: Vec<AServerLabel>,
    /// The instructions boarded, once the gather is composed.
    told: Vec<String>,
    /// When the instructions were first seen on a board, on this machine's
    /// own clock: past B12's reporting deadline the reply is composed, and an
    /// instruction still standing is B12's own *not confirmed* to tell.
    first_seen: Option<std::time::Duration>,
    composed: bool,
}

/// Everything the routing of one role worker is handed.
pub struct WhatTheRouterIsHanded {
    /// The proved conversation (`Telegram` P1).
    pub conversation: i64,
    pub me: WhoThisMachineIs,
    /// **A handle onto the same store the listener holds** (F2: one store).
    pub store: Box<dyn TheTelegramWorkersState>,
    pub holds: Box<dyn ReadsWhatThisMachineHolds>,
    /// The one send queue (`Telegram` C6).
    pub queue: Arc<TheSendQueue>,
    pub clock: Arc<dyn ReadsItsOwnClock>,
    /// What happened, in words that never carry a token or a message's content.
    pub notes: Notes,
}

/// One gather, as the machine that composes it knows it.
#[derive(Default)]
struct AGather {
    purpose: Option<WhatAGatherIsFor>,
    update: i64,
    /// Every machine asked, with the display the roster listed, in order.
    asked: Vec<(String, AServerLabel)>,
    /// **Whether at least one of its entries has left the board** — by its
    /// reply or at its deadline (B12). It is composed once none still stands.
    /// Until the round-1 code audit of the bot and the chat (fix-here item 17)
    /// only a deadline counted, so every gather waited its whole ten seconds.
    left: bool,
    composed: bool,
}

struct Routing {
    handed: WhatTheRouterIsHanded,
    /// The board as the listener last knew it, taken before every turn.
    board: Option<TheBoard>,
    to_propose: Vec<AnEntry>,
    to_replace: Vec<AnEntry>,
    /// R10: each reply this machine gave, by the state-request it answers, so a
    /// repeated delivery gives the same reply back.
    replied: BTreeMap<String, Value>,
    /// Operations proposed for an update, and whether a read has shown them.
    proposed: HashSet<String>,
    seen: HashSet<String>,
    /// Updates whose truthful answer has been sent.
    answered: HashSet<i64>,
    /// Entries addressed here that have been handled.
    handled: HashSet<String>,
    /// Resolution entries whose expiry has been answered.
    told: HashSet<String>,
    /// **F3 — proposals whose work is done without being seen on a board**:
    /// refused as not admitted (the truthful refusal sent), completed by a
    /// reply, or ended at a deadline before any read saw them. Fix-here item 1.
    finished: HashSet<String>,
    gathers: BTreeMap<String, AGather>,
    replies: BTreeMap<String, BTreeMap<String, AReply>>,
    /// R1a: typed frame requests whose reply resolution expired, as the
    /// gather each falls back to.
    fallbacks: Vec<(i64, WhatAGatherIsFor)>,
    /// V1d: each round by its gather's identity.
    rounds: BTreeMap<String, AReportingRound>,
    /// **A test's stand-in for a machine on a release before V1d**: its
    /// replies to a gather carry no `speaks_the_phones_choice`. Always false
    /// outside tests.
    older_than_the_phones_choice: bool,
}

/// **The routing of one role worker.** Clone it freely; every clone is the
/// same routing.
#[derive(Clone)]
pub struct TheRouter {
    inner: Arc<Mutex<Routing>>,
}

/// **What the listener hands on to the routing** (`crate::listener::HandsOn`).
pub struct TheRoutersHands(TheRouter);

impl TheRouter {
    pub fn new(handed: WhatTheRouterIsHanded) -> TheRouter {
        TheRouter {
            inner: Arc::new(Mutex::new(Routing {
                handed,
                board: None,
                to_propose: Vec::new(),
                to_replace: Vec::new(),
                replied: BTreeMap::new(),
                proposed: HashSet::new(),
                seen: HashSet::new(),
                answered: HashSet::new(),
                handled: HashSet::new(),
                told: HashSet::new(),
                finished: HashSet::new(),
                gathers: BTreeMap::new(),
                replies: BTreeMap::new(),
                fallbacks: Vec::new(),
                rounds: BTreeMap::new(),
                older_than_the_phones_choice: false,
            })),
        }
    }

    /// **A test's machine on a release before V1d** (see the field).
    #[cfg(test)]
    pub(crate) fn as_if_older_than_the_phones_choice(&self) {
        self.routing().older_than_the_phones_choice = true;
    }

    /// The listener's [`HandsOn`].
    pub fn hands(&self) -> TheRoutersHands {
        TheRoutersHands(self.clone())
    }

    fn routing(&self) -> MutexGuard<'_, Routing> {
        self.inner.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    /// **Before every turn**: read the board the listener last knew, compose
    /// what is ready, and hand the listener this machine's proposals,
    /// replacements and replies.
    pub fn before_each_turn(&self, listener: &mut TheListener) {
        let mut routing = self.routing();
        routing.board = listener.the_board_as_last_read();
        routing.see_what_the_board_holds();
        // F3: an entry accepted and gone again between two looks at the board
        // was still accepted.
        routing.see_what_was_accepted(&listener.the_instructions_accepted_lately());
        routing.compose_what_is_ready();
        routing.compose_the_rounds_that_are_done();
        routing.fall_back();
        for entry in std::mem::take(&mut routing.to_propose) {
            listener.propose(entry);
        }
        for entry in std::mem::take(&mut routing.to_replace) {
            listener.propose_a_replacement(entry);
        }
    }

    /// **`Telegram` R1 — remove the records of an ended subject**: the job or
    /// hold whose generation ended. A press on one of its buttons then resolves
    /// to nothing, and the user is told nothing was done.
    pub fn the_subject_ended(&self, subject: &str) -> usize {
        let mut routing = self.routing();
        routing.handed.store.remove_action_contexts_where(&|r| r.the_subject() == Some(subject)).unwrap_or(0)
    }

    /// **`Telegram` R1 — remove the records of a superseded message.**
    pub fn the_message_was_superseded(&self, message: i64) -> usize {
        let mut routing = self.routing();
        routing.handed.store.remove_action_contexts_where(&|r| r.source_message == Some(message)).unwrap_or(0)
    }
}

impl HandsOn for TheRoutersHands {
    fn an_update_from_the_conversation(&mut self, update: &Value) -> Result<(), String> {
        self.0.routing().an_update(update)
    }
    fn an_entry_addressed_here(&mut self, entry: &AnEntry) -> Option<Value> {
        self.0.routing().addressed_here(entry)
    }
    fn an_entry_ended_at_its_deadline(&mut self, entry: &AnEntry) {
        self.0.routing().ended(entry);
    }
    fn a_reply_to_a_gather(&mut self, _from: &str, reply: &Value) {
        let Ok(reply) = serde_json::from_value::<AReply>(reply.clone()) else { return };
        self.0.routing().replies.entry(reply.gather.clone()).or_default().insert(reply.machine.clone(), reply);
    }
    fn an_entry_was_completed(&mut self, entry: &AnEntry) {
        self.0.routing().completed(entry);
    }
    fn an_entry_was_not_admitted(&mut self, entry: &AnEntry) {
        self.0.routing().not_admitted(entry);
    }
}

fn on_the_board(board: &TheBoard, operation: &str) -> bool {
    board.instructions.iter().any(|e| e.operation == operation)
}

impl Routing {
    fn note(&mut self, what: &str) {
        (self.handed.notes)(what);
    }

    fn see_what_the_board_holds(&mut self) {
        let Some(board) = &self.board else { return };
        let now_seen: Vec<String> = self.proposed.iter().filter(|op| on_the_board(board, op)).cloned().collect();
        self.seen.extend(now_seen);
    }

    fn see_what_was_accepted(&mut self, accepted: &[AnEntry]) {
        let now_seen: Vec<String> = accepted.iter().map(|e| e.operation.clone()).filter(|op| self.proposed.contains(op)).collect();
        self.seen.extend(now_seen);
    }

    /// **F3's third kind, and B2 — an entry of this machine's that the board
    /// could not admit.** The holder has sent the truthful refusal, so the
    /// update behind it is done: it is confirmed when it next comes back, and
    /// the proposal is forgotten. Until the round-1 code audit of the bot and
    /// the chat (fix-here item 1) the proposal was remembered as pending for
    /// ever, and the listener never moved past that update: every later tap
    /// and question waited behind it.
    fn not_admitted(&mut self, entry: &AnEntry) {
        self.proposed.remove(&entry.operation);
        self.finished.insert(entry.operation.clone());
        self.note(&format!("{} was not admitted, and the refusal was sent", entry.operation));
    }

    /// **B12 and R10 — a state-request completed by its reply**, removed by the
    /// holder that took the reply in (fix-here item 17).
    fn completed(&mut self, entry: &AnEntry) {
        self.finished.insert(entry.operation.clone());
        self.a_gather_entry_left(entry);
    }

    /// **F3 — propose an entry, and count its work done only once a read has
    /// shown it on the board.**
    fn board_it(&mut self, entry: AnEntry) -> bool {
        if self.seen.contains(&entry.operation) || self.finished.contains(&entry.operation) {
            return true;
        }
        if self.board.as_ref().is_some_and(|b| on_the_board(b, &entry.operation)) {
            self.seen.insert(entry.operation);
            return true;
        }
        if self.proposed.insert(entry.operation.clone()) {
            self.note(&format!("proposed {}", entry.operation));
            self.to_propose.push(entry);
        }
        false
    }

    // ------------------------------------------------------------ updates

    fn an_update(&mut self, update: &Value) -> Result<(), String> {
        let is_a_tap = update.get("callback_query").is_some();
        match typed::what_an_update_asks(update) {
            None | Some(WhatAnUpdateAsks::Nothing { .. }) => Ok(()),
            Some(WhatAnUpdateAsks::NotUnderstood { update }) => {
                let answer = if is_a_tap { typed::A_BUTTON_THAT_EXPIRED } else { typed::NOT_A_REQUEST };
                self.answer_once(update, answer)
            }
            Some(WhatAnUpdateAsks::ATap { update, callback, data, label, conversation, source_message }) => {
                let Some(names) = data.as_deref().and_then(|d| WhatATokenNames::read(d).ok()) else {
                    // R1 and R3: an unknown format is rejected and answered.
                    return self.answer_once(update, typed::A_BUTTON_THAT_EXPIRED);
                };
                let evidence = ATapsEvidence { token: names.as_the_token(), callback, conversation, source_message, label };
                self.boarded(a_taps_resolution(&evidence, update, &names))
            }
            Some(WhatAnUpdateAsks::ARequest { update, request, conversation, replying_to, date, to_a_person }) => {
                // V1d: a number outside 1 to 60 is refused, reply or none.
                if let ATypedRequest::Reporting(Some(minutes)) = request {
                    if !(crate::job_messages::THE_FEWEST_MINUTES_BETWEEN_PICTURES..=crate::job_messages::THE_MOST_MINUTES_BETWEEN_PICTURES).contains(&minutes) {
                        return self.answer_once(update, typed::WHOLE_MINUTES);
                    }
                }
                let replied = replying_to.filter(|message| !to_a_person && !self.is_about_no_job(*message));
                match (request, replied) {
                    // R1a: *status* is R10's gather whatever it replies to.
                    (ATypedRequest::HowIsItGoing, _) => self.gathered(update, WhatAGatherIsFor::HowIsItGoing),
                    // R1a, widened: the other three, by the replied-to message.
                    (request, Some(source_message)) => {
                        let (frame, asked) = match request {
                            ATypedRequest::SendAFrame(frame) => (Some(frame), ARepliedRequest::Frame),
                            ATypedRequest::SendTheNewestFrame => (None, ARepliedRequest::Frame),
                            ATypedRequest::Stop => (None, ARepliedRequest::Stop),
                            ATypedRequest::Reporting(minutes) => (None, ARepliedRequest::Reporting { minutes: minutes.map(|m| m as u8) }),
                            ATypedRequest::HowIsItGoing => unreachable!("answered above"),
                        };
                        let evidence = AReplysEvidence { conversation, source_message, frame, request: Some(asked) };
                        self.boarded(a_replys_resolution(&evidence, update))
                    }
                    (ATypedRequest::SendAFrame(frame), None) => self.gathered(update, WhatAGatherIsFor::SendAFrame(frame)),
                    (ATypedRequest::SendTheNewestFrame, None) => self.gathered(update, WhatAGatherIsFor::SendTheNewestFrame),
                    (ATypedRequest::Stop, None) => self.gathered(update, WhatAGatherIsFor::Stop),
                    (ATypedRequest::Reporting(None), None) => self.ask_for_the_phones_choice(update, date),
                    (ATypedRequest::Reporting(Some(minutes)), None) => self.start_a_round(
                        update,
                        TheReportingChoice::APictureEveryFewMinutes { minutes: minutes as u8 },
                        format!("reporting-{update}"),
                        date,
                    ),
                }
            }
        }
    }

    fn boarded(&mut self, entry: AnEntry) -> Result<(), String> {
        if self.board_it(entry) {
            Ok(())
        } else {
            Err("waiting for the entry to be read on the board (F3)".into())
        }
    }

    /// **`Telegram` R10 — board one state-request per roster machine.**
    fn gathered(&mut self, update: i64, purpose: WhatAGatherIsFor) -> Result<(), String> {
        let gather = the_gather_for(update);
        if !self.gathers.contains_key(&gather) {
            let Some(board) = &self.board else {
                return Err("no board has been read yet".into());
            };
            let asked: Vec<(String, AServerLabel)> = board.roster.iter().map(|r| (r.machine.clone(), AServerLabel::with_colour(&r.display, r.colour.as_deref()))).collect();
            self.note(&format!("boarding {gather} for {} machines", asked.len()));
            self.gathers.insert(gather.clone(), AGather { purpose: Some(purpose), update, asked, ..AGather::default() });
        }
        let asked = self.gathers[&gather].asked.clone();
        let mut all = true;
        for (machine, _) in &asked {
            all &= self.board_it(a_state_request(&gather, machine, purpose, update));
        }
        if all {
            Ok(())
        } else {
            Err("waiting for the gather to be read on the board (F3)".into())
        }
    }

    /// **`Telegram` R3 — a truthful answer, sent once.** Its work is its being
    /// sent: a send the service refused is not sent again for ever, and one that
    /// reached nothing is tried again when the update comes back.
    fn answer_once(&mut self, update: i64, text: &str) -> Result<(), String> {
        if self.answered.contains(&update) {
            return Ok(());
        }
        match self.send(text.to_string(), None) {
            HowASendEnded::HandedBack(WhyTheCallFailed::NotReached(_)) => Err("the answer reached nothing".into()),
            _ => {
                self.answered.insert(update);
                self.note(&format!("answered update {update}"));
                Ok(())
            }
        }
    }

    /// **`Telegram` R1a, widened — a message the listener knows is about no
    /// job**: the board (B3), its own machinery message (B10), or one its own
    /// register marks so. A reply to it is handled at once as no reply.
    fn is_about_no_job(&self, message: i64) -> bool {
        let store = &self.handed.store;
        store.the_board().ok().flatten() == Some(message)
            || store.the_machinery_message().ok().flatten() == Some(message)
            || matches!(store.the_sent_message(message), Ok(Some(ASentMessage { about: WhatAMessageIsAbout::NoJob, .. })))
    }

    /// **`Telegram` V1d — the ask**: the question sent first, a record for
    /// each of the five buttons written against it, and only then the buttons
    /// (R1), the third at five minutes; the line for other minutes under them.
    /// Each record carries Telegram's `date` for the ask, which a tap's round
    /// takes as its own (a tap carries none).
    fn ask_for_the_phones_choice(&mut self, update: i64, _date: i64) -> Result<(), String> {
        if self.answered.contains(&update) {
            return Ok(());
        }
        let Ok(key) = self.handed.store.the_routing_key() else { return Err("the routing key could not be read".into()) };
        let text = format!("{THE_PHONES_ASK}\n{FOR_OTHER_MINUTES}");
        let sent = match self.send(text.clone(), None) {
            HowASendEnded::HandedBack(WhyTheCallFailed::NotReached(why)) => return Err(why),
            HowASendEnded::Accepted(sent) => sent,
            _ => {
                self.answered.insert(update);
                return Ok(());
            }
        };
        self.answered.insert(update);
        let (Some(message), date) = (sent.get("message_id").and_then(Value::as_i64), sent.get("date").and_then(Value::as_i64).unwrap_or_default()) else {
            return Ok(());
        };
        let me = self.handed.me.machine.clone();
        let mut rows = Vec::new();
        for choice in THE_FIVE_CHOICES {
            let Some(mut record) = AnActionContextRecord::issued_under(
                &key,
                self.handed.conversation,
                &me,
                None,
                None,
                CHOOSING_THE_PHONES_CHOICE,
                json!({ "value": choice.in_the_manifests_form(), "date": date, "labels": { "button": choice.as_the_phone_says() } }),
                "",
            ) else {
                return Ok(());
            };
            record.source_message = Some(message);
            let token = record.token.clone();
            if self.handed.store.record_an_action_context(record).is_err() {
                self.note("the phone's ask: a record could not be written, so no buttons were added");
                return Ok(());
            }
            rows.push(json!([{ "text": choice.as_the_phone_says(), "callback_data": token }]));
        }
        self.send(text, Some((message, json!({ "inline_keyboard": rows }))));
        self.note(&format!("sent the phone's ask as message {message}"));
        Ok(())
    }

    /// **`Telegram` V1d — a round begins with a gather**, so the machines whose
    /// program is older than V1d are known before anything is told: an older
    /// program never acknowledges an action it does not know, so telling it
    /// would leave the instruction standing for ever (B12) and the reply could
    /// never say it needs an update.
    fn start_a_round(&mut self, update: i64, choice: TheReportingChoice, operation: String, date: i64) -> Result<(), String> {
        let gather = the_gather_for(update);
        self.rounds.entry(gather).or_insert_with(|| AReportingRound {
            choice,
            operation,
            date,
            update,
            not_reached: Vec::new(),
            needs_an_update: Vec::new(),
            told: Vec::new(),
            first_seen: None,
            composed: false,
        });
        self.gathered(update, WhatAGatherIsFor::ThePhonesChoice)
    }

    /// **`Telegram` V1d — the gather is in: tell each machine that can take
    /// it**, one action instruction each, itself included.
    fn tell_the_phones_choice(&mut self, gather: &str, asked: &[(String, AServerLabel)], replies: &BTreeMap<String, AReply>) {
        let Some(mut round) = self.rounds.get(gather).cloned() else {
            self.note(&format!("{gather}: a round this machine did not start; nothing told"));
            return;
        };
        for (machine, display) in asked {
            match replies.get(machine) {
                None => round.not_reached.push(display.clone()),
                Some(reply) if !reply.speaks_the_phones_choice => round.needs_an_update.push(display.clone()),
                Some(_) => {
                    // Its own identity: `{gather}-{machine}` is the gather's
                    // state-request's.
                    let mut entry = an_action_on_one_job(
                        &format!("{}-to-{machine}", round.operation),
                        machine,
                        "",
                        EVERY_LIVE_JOBS_REPORTING,
                        json!({
                            "value": round.choice.in_the_manifests_form(),
                            "operation": round.operation,
                            "date": round.date,
                            "update": round.update,
                            "labels": { "server": display, "label": "", "button": "/reporting" },
                        }),
                        Some(round.update),
                    );
                    entry.subjects.clear();
                    round.told.push(entry.operation.clone());
                    self.board_it(entry);
                }
            }
        }
        self.note(&format!("{gather}: told {} machines the phone's choice", round.told.len()));
        self.rounds.insert(gather.to_string(), round);
    }

    /// **`Telegram` V1d — the one reply**, once every instruction told has
    /// left the board, or B12's reporting deadline has passed since they were
    /// first seen there (one still standing is B12's own *not confirmed*).
    fn compose_the_rounds_that_are_done(&mut self) {
        let Some(board) = self.board.clone() else { return };
        let now = self.handed.clock.now();
        let mut done = Vec::new();
        for (gather, round) in self.rounds.iter_mut() {
            if round.composed || !self.gathers.get(gather).is_some_and(|g| g.composed) {
                continue;
            }
            let standing = round.told.iter().filter(|op| on_the_board(&board, op)).count();
            if standing > 0 && round.first_seen.is_none() {
                round.first_seen = Some(now);
            }
            let past = round.first_seen.is_some_and(|seen| now.saturating_sub(seen) >= crate::board::instructions::THE_DEFAULT_REPORTING_DEADLINE);
            let all_seen = round.told.iter().all(|op| self.seen.contains(op) || self.finished.contains(op));
            if (standing == 0 && all_seen) || past {
                round.composed = true;
                done.push(round.clone());
            }
        }
        for round in done {
            let text = the_phones_choice_was_set(round.choice, &round.not_reached, &round.needs_an_update);
            self.send(text, None);
            self.note(&format!("the phone's choice {}: reply sent", round.operation));
        }
    }

    // ------------------------------------------------------------ entries addressed here

    fn live(&self) -> impl Fn(&str) -> Option<String> + '_ {
        |subject: &str| self.handed.holds.the_live_generation(subject)
    }

    /// **Gives back this machine's reply to a state-request** (R10), the same
    /// reply for a repeated delivery; the listener says it, or takes it in where
    /// this machine holds (B11, B12).
    fn addressed_here(&mut self, entry: &AnEntry) -> Option<Value> {
        if self.handled.contains(&entry.operation) {
            return self.replied.get(&entry.operation).cloned();
        }
        let me = self.handed.me.machine.clone();
        match (entry.kind, &entry.addressee) {
            (AKindOfEntry::StateRequest, _) => {
                let Some((gather, _)) = the_gather_of(entry) else { return None };
                let reply = AReply {
                    operation: entry.operation.clone(),
                    gather,
                    machine: me,
                    display: self.handed.me.display.to_string(),
                    colour: self.handed.me.display.colour().map(str::to_string),
                    jobs: self.handed.holds.the_jobs(),
                    moment: self.handed.clock.the_moment(),
                    // V1d: this release acts on the phone's choice.
                    speaks_the_phones_choice: !self.older_than_the_phones_choice,
                };
                self.replied.insert(entry.operation.clone(), reply.as_value());
                self.note(&format!("replying to {}", entry.operation));
            }
            (AKindOfEntry::ContextResolution, TheAddressee::RoutingKey(_)) => {
                let Ok(key) = self.handed.store.the_routing_key() else { return None };
                let resolved = resolve_a_tap(entry, &me, &key, self.handed.store.as_ref(), &self.live());
                // **V1d — a tap on the ask starts the round here**, and the
                // instruction it resolves to is only recorded by this
                // machine's acts, so the press is acknowledged (B9).
                if let Ok(full) = &resolved {
                    if full.action == CHOOSING_THE_PHONES_CHOICE {
                        if let Some(choice) = TheReportingChoice::exactly(&full.payload["value"]) {
                            let update = entry.source_update.unwrap_or_default();
                            let date = full.payload["date"].as_i64().unwrap_or_default();
                            let _ = self.start_a_round(update, choice, entry.operation.clone(), date);
                        }
                    }
                }
                if !self.took(entry, resolved) {
                    return None;
                }
            }
            (AKindOfEntry::ContextResolution, TheAddressee::SourceMessage(_)) => {
                let resolved = resolve_a_reply(entry, &me, self.handed.store.as_ref(), self.handed.holds.as_ref()).map(|mut full| {
                    // R7: this machine's own label, for what the user is told.
                    if full.addressee == TheAddressee::Machine(me.clone()) && full.payload["labels"].get("server").is_none() {
                        full.payload["labels"]["server"] = json!(self.handed.me.display);
                    }
                    full
                });
                if !self.took(entry, resolved) {
                    return None;
                }
            }
            _ => return None,
        }
        self.handled.insert(entry.operation.clone());
        self.replied.get(&entry.operation).cloned()
    }

    /// Gives back whether the entry is handled; a store that could not be read
    /// leaves it to be tried at the next read.
    fn took(&mut self, entry: &AnEntry, resolved: Result<AnEntry, WhyItFailsClosed>) -> bool {
        match resolved {
            Ok(full) => {
                self.note(&format!("resolved {} to its owner", entry.operation));
                self.to_replace.push(full);
                true
            }
            Err(WhyItFailsClosed::TheStoreFailed) => false,
            Err(why) => {
                // R1: rejected, and nothing is submitted. The user's answer is
                // the expiry's, from whichever machine holds the board.
                self.note(&format!("rejected {}: {why:?}", entry.operation));
                true
            }
        }
    }

    // ------------------------------------------------------------ deadlines

    /// **`Telegram` B12, R1, R1a and R12 — an entry this holder removed at its
    /// deadline.**
    fn ended(&mut self, entry: &AnEntry) {
        // F3 and fix-here item 1: an entry that left at its deadline is ended,
        // never pending, whether or not a read ever saw it.
        self.finished.insert(entry.operation.clone());
        if self.a_gather_entry_left(entry) {
            return;
        }
        if entry.kind != AKindOfEntry::ContextResolution || !self.told.insert(entry.operation.clone()) {
            return;
        }
        match &entry.addressee {
            TheAddressee::SourceMessage(_) => {
                let Ok(evidence) = serde_json::from_value::<AReplysEvidence>(entry.payload.clone()) else { return };
                let update = entry.source_update.unwrap_or_default();
                match (evidence.the_request(), evidence.frame) {
                    // R1a: a frame request's expiry falls back to the typed
                    // request's own handling.
                    (ARepliedRequest::Frame, Some(frame)) => self.fallbacks.push((update, WhatAGatherIsFor::SendAFrame(frame))),
                    (ARepliedRequest::Frame, None) => self.fallbacks.push((update, WhatAGatherIsFor::SendTheNewestFrame)),
                    // R1a, widened: `stop` and `reporting` fail closed.
                    (ARepliedRequest::Stop | ARepliedRequest::Reporting { .. }, _) => {
                        self.send(typed::COULD_NOT_TELL_WHICH_JOB.to_string(), None);
                        self.note(&format!("{}: nobody claimed the reply, so nothing was changed", entry.operation));
                    }
                }
            }
            _ => {
                // R1 and R3: answered, never guessed at.
                self.send(typed::A_BUTTON_THAT_EXPIRED.to_string(), None);
                self.note(&format!("told the user {} expired", entry.operation));
            }
        }
    }

    /// **R10 and B12 — a gather's entry has left the board**, by its reply or
    /// at its deadline; a holder that did not board the gather learns of it
    /// here. Gives back whether the entry was a gather's.
    fn a_gather_entry_left(&mut self, entry: &AnEntry) -> bool {
        let Some((gather, purpose)) = the_gather_of(entry) else { return false };
        let TheAddressee::Machine(machine) = &entry.addressee else { return true };
        let display = self.the_display_of(machine);
        let known = self.gathers.entry(gather).or_default();
        known.purpose.get_or_insert(purpose);
        if known.update == 0 {
            known.update = entry.source_update.unwrap_or_default();
        }
        if !known.asked.iter().any(|(m, _)| m == machine) {
            known.asked.push((machine.clone(), display));
        }
        known.left = true;
        true
    }

    fn the_display_of(&self, machine: &str) -> AServerLabel {
        self.board
            .as_ref()
            .and_then(|b| b.roster.iter().find(|r| r.machine == machine))
            .map_or_else(|| AServerLabel::from("A server"), |r| AServerLabel::with_colour(&r.display, r.colour.as_deref()))
    }

    fn fall_back(&mut self) {
        for (update, purpose) in std::mem::take(&mut self.fallbacks) {
            let _ = self.gathered(update, purpose);
        }
    }

    // ------------------------------------------------------------ composing

    /// **`Telegram` R10 and R13 — compose each gather some of whose entries
    /// have left the board, by reply or deadline, and none of which still
    /// stands** — as soon as every server asked has replied.
    fn compose_what_is_ready(&mut self) {
        let Some(board) = self.board.clone() else { return };
        let ready: Vec<String> = self
            .gathers
            .iter()
            .filter(|(gather, g)| {
                g.left && !g.composed && !board.instructions.iter().any(|e| e.operation.starts_with(&format!("{gather}-")))
            })
            .map(|(gather, _)| gather.clone())
            .collect();
        for gather in ready {
            let (purpose, update, asked) = {
                let g = self.gathers.get_mut(&gather).expect("a ready gather is known");
                g.composed = true;
                (g.purpose.unwrap_or(WhatAGatherIsFor::HowIsItGoing), g.update, g.asked.clone())
            };
            let replies = self.replies.remove(&gather).unwrap_or_default();
            let answered = asked.iter().filter(|(m, _)| replies.contains_key(m)).count();
            match purpose {
                WhatAGatherIsFor::HowIsItGoing => {
                    // **C1b and V1b, 2026-10-31 — the reply to *Status*
                    // offers no Stop**: it covers every server, and `/stop`
                    // stops a job from there. The twenty-sixth notes, note 3,
                    // Creator-stated: *"/status should not offer to stop
                    // jobs."* Until then a *Stop {server} — {job}* went under
                    // it for every live job (the seventeenth notes, piece 6).
                    self.send(how_it_is_going(&asked, &replies), None);
                    let silent: Vec<&str> = asked
                        .iter()
                        .filter(|(m, _)| !replies.contains_key(m))
                        .map(|(m, _)| m.as_str())
                        .collect();
                    self.note(&format!(
                        "sent how it is going for {gather}: {answered} answered, {} did not answer {silent:?}",
                        silent.len()
                    ));
                }
                WhatAGatherIsFor::SendAFrame(frame) => self.disambiguate(&gather, update, Some(frame), &the_candidates(&asked, &replies)),
                WhatAGatherIsFor::SendTheNewestFrame => self.disambiguate(&gather, update, None, &the_candidates(&asked, &replies)),
                WhatAGatherIsFor::Stop => self.offer_a_stop(&gather, update, &asked, &replies),
                WhatAGatherIsFor::ThePhonesChoice => self.tell_the_phones_choice(&gather, &asked, &replies),
            }
        }
    }

    /// **`Telegram` R13 — the tappable choice of job**, or the one job routed
    /// directly, or the truth that there is none.
    fn disambiguate(&mut self, gather: &str, update: i64, frame: Option<u64>, candidates: &[gather::ACandidate]) {
        match candidates {
            [] => {
                self.send(typed::NO_JOB_TO_SEND_A_FRAME_FROM.to_string(), None);
                self.note(&format!("{gather}: no job to send a frame from"));
            }
            [only] => {
                // R7: the labels ride the request, for what the user is told about it.
                let labels = json!({ "server": self.the_display_of(&only.owner), "label": only.label });
                let request = a_frame_request(&format!("frame-{update}"), &only.owner, &only.job, &only.generation, frame, Some(update), labels);
                self.note(&format!("{gather}: routed directly as {}", request.operation));
                self.to_propose.push(request);
            }
            several => self.offer_the_choice(gather, frame, several),
        }
    }

    /// **`Telegram` R1 and R13 — the question sent first, the records written
    /// against it, and only then the buttons**, so no button ever exists without
    /// its record.
    fn offer_the_choice(&mut self, gather: &str, frame: Option<u64>, candidates: &[gather::ACandidate]) {
        let Ok(key) = self.handed.store.the_routing_key() else { return };
        let displays: Vec<AServerLabel> = candidates.iter().map(|c| self.the_display_of(&c.owner)).collect();
        let HowASendEnded::Accepted(sent) = self.send(typed::WHICH_JOB.to_string(), None) else { return };
        let Some(message) = sent.get("message_id").and_then(Value::as_i64) else { return };
        let mut rows = Vec::new();
        for (candidate, display) in candidates.iter().zip(displays) {
            let Some(mut record) = AnActionContextRecord::issued_under(
                &key,
                self.handed.conversation,
                &candidate.owner,
                Some(&candidate.job),
                None,
                if frame.is_some() { SENDING_A_FRAME } else { context::SENDING_THE_NEWEST_FRAME },
                // R7 and fix-here item 13: the labels, and the button's own, for
                // what the user is told about the tap; nothing routes by them.
                json!({ "frame": frame, "job": candidate.job, "labels": { "server": display, "label": candidate.label, "button": candidate.label } }),
                &candidate.generation,
            ) else {
                return;
            };
            record.source_message = Some(message);
            let token = record.token.clone();
            if self.handed.store.record_an_action_context(record).is_err() {
                self.note(&format!("{gather}: a record could not be written, so no buttons were added"));
                return;
            }
            let choice = json!({ "text": candidate.label, "callback_data": token });
            // **C1b: a job that has ended is still a frame's candidate (R13),
            // and gets no Stop.**
            if !candidate.live {
                rows.push(json!([choice]));
                continue;
            }
            let subject = crate::actions::TheSubject {
                owning_machine: candidate.owner.clone(),
                job: Some(candidate.job.clone()),
                hold: None,
                generation: candidate.generation.clone(),
            };
            let Some(mut stop) = crate::actions::a_stop_button_on(
                self.handed.store.as_mut(),
                self.handed.conversation,
                message,
                &subject,
                &display,
                &candidate.label,
            ) else {
                return;
            };
            stop["text"] = json!(format!("Stop {}", candidate.label));
            rows.push(json!([choice, stop]));
        }
        let markup = json!({ "inline_keyboard": rows });
        let added = self.send(typed::WHICH_JOB.to_string(), Some((message, markup)));
        self.note(&format!(
            "{gather}: sent which job as message {message} with {} buttons, added = {}",
            candidates.len(),
            matches!(added, HowASendEnded::Accepted(_))
        ));
    }

    /// **`Telegram` R13a — `stop` over R13's disambiguation**: the jobs waiting,
    /// held or rendering on every machine that answered. None is answered in
    /// plain words; one goes straight to D0's confirmation on its owner; more
    /// are offered as buttons, each a press of *Stop this job* for its job.
    /// A machine that did not answer is named under them.
    fn offer_a_stop(&mut self, gather: &str, update: i64, asked: &[(String, AServerLabel)], replies: &BTreeMap<String, AReply>) {
        let candidates: Vec<gather::ACandidate> = the_candidates(asked, replies).into_iter().filter(|c| c.live).collect();
        let silent: Vec<String> = asked.iter().filter(|(m, _)| !replies.contains_key(m)).map(|(_, d)| d.to_string()).collect();
        let under = if silent.is_empty() { String::new() } else { format!("\n{} did not answer.", silent.join(", ")) };
        match candidates.as_slice() {
            [] => {
                self.send(format!("{}{under}", typed::NOTHING_TO_STOP), None);
                self.note(&format!("{gather}: nothing to stop"));
            }
            [only] => {
                let labels = json!({ "server": self.the_display_of(&only.owner), "label": only.label, "button": "Stop this job" });
                let request = an_action_on_one_job(
                    &format!("stop-{update}"),
                    &only.owner,
                    &only.job,
                    crate::job_messages::ASK_TO_STOP_THIS_JOB,
                    json!({ "job": only.job, "generation": only.generation, "labels": labels }),
                    Some(update),
                );
                self.note(&format!("{gather}: routed directly as {}", request.operation));
                self.to_propose.push(request);
            }
            several => {
                let Ok(key) = self.handed.store.the_routing_key() else { return };
                let owners: HashSet<&str> = several.iter().map(|c| c.owner.as_str()).collect();
                let HowASendEnded::Accepted(sent) = self.send(format!("{}{under}", typed::WHICH_JOB), None) else { return };
                let Some(message) = sent.get("message_id").and_then(Value::as_i64) else { return };
                let mut rows = Vec::new();
                for candidate in several {
                    let display = self.the_display_of(&candidate.owner);
                    // R13a: where two servers are offered, the server after it.
                    let text = if owners.len() > 1 { format!("{} — {display}", candidate.label) } else { candidate.label.clone() };
                    let Some(mut record) = AnActionContextRecord::issued_under(
                        &key,
                        self.handed.conversation,
                        &candidate.owner,
                        Some(&candidate.job),
                        None,
                        crate::job_messages::ASK_TO_STOP_THIS_JOB,
                        json!({ "job": candidate.job, "labels": { "server": display, "label": candidate.label, "button": "Stop this job" } }),
                        &candidate.generation,
                    ) else {
                        return;
                    };
                    record.source_message = Some(message);
                    let token = record.token.clone();
                    if self.handed.store.record_an_action_context(record).is_err() {
                        self.note(&format!("{gather}: a record could not be written, so no buttons were added"));
                        return;
                    }
                    rows.push(json!([{ "text": text, "callback_data": token }]));
                }
                self.send(format!("{}{under}", typed::WHICH_JOB), Some((message, json!({ "inline_keyboard": rows }))));
                self.note(&format!("{gather}: sent which job to stop as message {message}"));
            }
        }
    }

    /// **Every send through the one queue** (`Telegram` C6), waiting for its
    /// end and sending in its own place where no other thread is. `edit` adds
    /// buttons to a message already sent. **`Telegram` R1b: every new message
    /// the listener sends is registered as about no job** once Telegram
    /// accepts it, so a reply to it is handled at once as no reply.
    fn send(&mut self, text: impl Into<AMarkedText>, edit: Option<(i64, Value)>) -> HowASendEnded {
        let is_new = edit.is_none();
        let ended = self.send_through_the_queue(text, edit);
        if let (true, HowASendEnded::Accepted(sent)) = (is_new, &ended) {
            if let Some(message) = sent.get("message_id").and_then(Value::as_i64) {
                let line = ASentMessage { message, conversation: self.handed.conversation, about: WhatAMessageIsAbout::NoJob };
                // A line lost costs a reply to it the deadline, and nothing else.
                let _ = self.handed.store.register_a_sent_message(line);
            }
        }
        ended
    }

    fn send_through_the_queue(&self, text: impl Into<AMarkedText>, edit: Option<(i64, Value)>) -> HowASendEnded {
        let text = text.into();
        let chat = self.handed.conversation;
        let queue = &self.handed.queue;
        let told = match edit {
            None => queue.offer(
                AClassOfSend::RoutedAnswersAndAcknowledgements,
                Box::new(move |c: &TheClient| c.send_marked_message(chat, &text, None)),
            ),
            Some((message, buttons)) => queue.offer(
                AClassOfSend::RoutedAnswersAndAcknowledgements,
                Box::new(move |c: &TheClient| c.edit_marked_message(chat, message, &text, Some(buttons.clone()))),
            ),
        };
        loop {
            match told.try_recv() {
                Ok(ended) => return ended,
                Err(TryRecvError::Disconnected) => break,
                Err(TryRecvError::Empty) => {
                    if queue.send_the_next().is_none() {
                        if let Ok(ended) = told.recv() {
                            return ended;
                        }
                        break;
                    }
                }
            }
        }
        HowASendEnded::HandedBack(WhyTheCallFailed::NotReached("the send queue ended".into()))
    }
}

#[cfg(test)]
mod tests;

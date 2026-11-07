//! **The four phone actions, and a job's messages sent — `Telegram` §16 D1 to
//! D4, §17 A1 to A6, §18 Y1 to Y4b, and E3's tap and E4.**
//!
//! **What is here.** [`TheJobMessages`] sends a composed message
//! (`crate::job_messages`) and gives it its buttons, each an ordinary R1
//! button whose action-context record is written before the button is
//! attached. [`TheOwnersActs`] is what the owning machine does when a resolved
//! tap reaches it: it is the listener's [`ActsOnAnInstruction`], so B9's
//! outcome is recorded beside the identity and a repeat acts on nothing, and
//! piece 3's routing has already carried the tap to the owner (R1, R2).
//! [`TheListenersAcknowledgement`] is D2's first half.
//!
//! **The commands return once their submission is acknowledged** (`Telegram`
//! F3's work for an action on this machine's own job), and each recorded
//! outcome — the stop's counts, the answer applied, the job sent again — is
//! reported as its own message when it is journalled, by
//! [`TheOwnersActs::report`] (D2, A3, Y1). So a stop that takes D3's fourteen
//! seconds never holds the listener's turn, the heartbeat or a tap's answer.
//! Changed by the round-1 code audit of the bot and the chat, fix-here item 4.
//!
//! **What is not here, and whose it is.** Every act ends at
//! [`TheSupervisorsCommandPath`] — the stop, the release, the submission — at
//! [`ReadsTheWordsBack`], and the journal event
//! that carries each outcome is read elsewhere. **On a rented machine all of
//! it is step 10c's**: `Server protocol and transfers` C21's stop and
//! `Rented-server runtime` X8's steps, the release on a real hold record, a
//! machine minting a job identity under `State, identity and recovery`
//! Y7a, reading a captured stream whole, and the
//! journal reader (`Audits/Build/Step 10/Telegram — what each piece owns and in
//! what order.md` §8 items 4, 7, 8, 9 and 13). **So every act below keeps its
//! rule, and the acting itself is owed**; a desk hands in
//! [`a_stand_in::AStandInSupervisor`].
//!
//! **E4: all of it happens with nobody connected** — every value is read on
//! the machine and every send goes from it.
//!
//! **Markers in this file are all `Telegram`'s** unless another document is
//! named.

use crate::job_messages::{AServerLabel, TheServersName};
use crate::job_messages::{AMarkedText, a_user_message};
use std::collections::HashSet;
use std::sync::mpsc::{Receiver, TryRecvError};
use std::sync::Arc;
use std::time::SystemTime;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use smoozy_state::identities::{Identity, IdentityKind};

use crate::board::instructions::{
    an_older_version_could_not_do_it, AKindOfEntry, AnEntry, WHY_AN_ENTRY_OF_ANOTHER_VERSION_IS_REFUSED,
};
use crate::board::text::how_long_it_counts;
use crate::job_messages::{
    a_frame_cannot_be_sent_yet, a_stop_to_offer, the_act_was_not_done, the_answer_already_applied,
    the_answer_was_applied, the_job_already_ended, the_stop_question,
    the_stop_was_sent, the_stops_outcome, the_submissions_outcome, the_waiting_message,
    the_words_could_not_be_read, the_words_in_the_chat, AComposedMessage, AStopsOutcome, AStream,
    ASubmissionsOutcome, AWaitingJob, TheAnswer, WhatTheHoldWaitsFor, ANSWER_A_WAITING_JOB,
    ASK_TO_STOP_THIS_JOB, KEEP_RENDERING, SEND_THE_WORDS, STOP_THIS_JOB, STOP_AN_ENDED_JOB, TRY_IT_AGAIN,
};
use crate::listener::{ActsOnAnInstruction, Notes, TheListener};
use crate::pacing::{AClassOfSend, HowASendEnded, TheSendQueue};
use crate::pictures::ADerivative;
use crate::routing::context::{
    the_owner_finds_the_subject_live, AnActionContextRecord, SENDING_A_FRAME,
};
use crate::routing::gather::ReadsWhatThisMachineHolds;
use crate::routing::typed::A_BUTTON_THAT_EXPIRED;
use crate::transport::{TheClient, WhyTheCallFailed};
use crate::worker_state::TheTelegramWorkersState;

// ------------------------------------------------------------ the supervisor's side

/// **`Telegram` B9 and `Server protocol and transfers` C4 — the outcome
/// recorded beside an instruction's identity.** For a command, whether its
/// submission was acknowledged (`Telegram` F3); for an act that is not a
/// command, whether it was done.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ARecordedOutcome {
    Done,
    /// Not done, and the user was told in plain words.
    Failed(String),
}

pub const THE_JOB_OR_ATTEMPT_ENDED: &str = "the confirmed job or attempt has ended";

/// **What a release command's recorded outcome says** (`Rented-server runtime`
/// U13, U14).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AReleasesOutcome {
    /// This answer was the first recorded, and was applied.
    Applied,
    /// `Rented-server runtime` U13: another answer was recorded first; this one acted on nothing.
    /// `Rented-server runtime` U14: the loser is told which, and when.
    AnotherAnswerWasApplied { answer: TheAnswer, at: SystemTime },
}

/// **The one supervisor command path a role worker submits to** (`Telegram`
/// F1: *"it submits every action to the same supervisor command path the SSH
/// adapter uses"*; `Server protocol and transfers` C4, C8; `Rented-server
/// runtime` U10). **Step 10c's on a machine.**
///
/// **Its shape is `Telegram` B9's since the round-1 code audit of the bot and
/// the chat (U6, fix-here items 2 and 4)**: a command carries its operation
/// identity and records its outcome beside it, a repeat finds that outcome,
/// and each command returns once its submission is acknowledged. **`Err` means
/// nothing was recorded**, so the instruction stays unacknowledged and is tried
/// again.
pub trait TheSupervisorsCommandPath: Send {
    /// The same durable context store used by the listener. A path without
    /// it cannot offer a confirmation and must never submit an unconfirmed stop.
    fn the_action_context_store(&self) -> Option<Box<dyn TheTelegramWorkersState>> {
        None
    }

    /// **B9 and `Server protocol and transfers` C4 — the outcome recorded
    /// beside this identity, if any.**
    fn the_recorded_outcome(&mut self, operation: &str) -> Result<Option<ARecordedOutcome>, String>;
    /// **B9 — record an outcome beside an identity**, for an act that is not a
    /// command: the words sent in the chat, a lock's refusal, an ended subject.
    fn record_the_outcome(&mut self, operation: &str, outcome: &ARecordedOutcome) -> Result<(), String>;
    /// **D1 — a stop for the recorded render operation of this job**, under
    /// its own operation identity (`Server protocol and transfers` C21). The
    /// kind's termination is `Rendering`'s and `Rented-server runtime` X8's; its
    /// recorded outcome is journalled and reported by [`TheOwnersActs::report`].
    fn submit_a_stop(&mut self, operation: &str, job: &str) -> Result<ARecordedOutcome, String>;
    fn submit_a_confirmed_stop(
        &mut self,
        operation: &str,
        job: &str,
        generation: &str,
    ) -> Result<ARecordedOutcome, String> {
        let _ = generation;
        self.submit_a_stop(operation, job)
    }

    /// **A2 — one release command on the hold** (`Rented-server runtime` U9),
    /// first recorded wins (`Rented-server runtime` U13).
    fn release_a_hold(&mut self, operation: &str, hold: &str, answer: TheAnswer) -> Result<ARecordedOutcome, String>;
    /// **Y1 — the same job submitted again**, composed from its immutable
    /// manifest and kept records (`Job files and outputs` M2), **its new job
    /// identity issued by the supervisor before the submission is composed**
    /// (`State, identity and recovery` Y7a). `submission` is the
    /// submission's own operation identity, one per button, so a second press
    /// finds the recorded submission (`Server protocol and transfers` C4;
    /// `Telegram` R4); `operation` is the press's.
    fn submit_again(&mut self, operation: &str, submission: &str, failed_job: &str) -> Result<ARecordedOutcome, String>;
    /// **R14, I4 and I5 — a routed frame request, answered on the owning
    /// machine**: the picture sent from the derivative set, a cleared one made
    /// again through the preview route, or the negative case said from the
    /// records. **The path sends what it says itself** and gives back the
    /// outcome to record; `Err` where the line reached nothing, so the request
    /// is tried again (B9). **`None` where this path has no actor for a frame
    /// request** — a desk process, a test — and the user is told the frame
    /// cannot be sent from this server yet, as before. Added 2026-10-16, the
    /// Telegram worker's piece 6.
    fn answer_a_frame_request(&mut self, operation: &str, job: &str, frame: u64) -> Option<Result<ARecordedOutcome, String>> {
        let _ = (operation, job, frame);
        None
    }

    /// **R13a and C1c — the newest frame the job's derivative set holds**
    /// (I2): `Some(None)` where it holds none, and **`None` where this path
    /// has no actor for a frame request**, as [`Self::answer_a_frame_request`].
    /// Added 2026-10-31, the Telegram discussion, the server half.
    fn the_newest_frame_held(&mut self, job: &str) -> Option<Option<u64>> {
        let _ = job;
        None
    }

    /// **V1d — the phone's choice on this machine**: V1c's change for every
    /// job it holds that is waiting, held or rendering, both of a first-frame
    /// pair, each durably; then the phone's choice written into the Telegram
    /// worker's state store where it is later. Gives back how many jobs
    /// changed; `Err` where something was not written, so the instruction is
    /// tried again and a repeat finds each change already in force. **`None`
    /// where this path has no actor** — a desk process, a test.
    fn change_every_live_jobs_reporting(
        &mut self,
        operation: &str,
        choice: &crate::worker_state::ThePhonesChoice,
    ) -> Option<Result<usize, String>> {
        let _ = (operation, choice);
        None
    }

    /// **R1a, widened — V1c's change for one job, from a reply**: `Done` where
    /// written, `Failed` saying why where the job has ended or is not here.
    /// **`None` where this path has no actor.**
    fn change_one_jobs_reporting(
        &mut self,
        operation: &str,
        job: &str,
        choice: crate::job_messages::TheReportingChoice,
    ) -> Option<Result<ARecordedOutcome, String>> {
        let _ = (operation, job, choice);
        None
    }
}

/// **`Telegram` B9a, from 2026-10-31 for an action as well as a version** —
/// what an action this program does not know is recorded as: refused, never
/// acted on in part, and acknowledged. Until then it was left unrecorded and
/// tried again for ever (B12), which is how every release before this one
/// still meets V1d's instruction; `routing` asks first for that reason.
pub const AN_ACTION_THIS_SERVER_DOES_NOT_KNOW: &str = "an action this server's Smoozy does not know";

/// **R1a, widened — one job's reporting changed from a reply, said under the
/// job's head** (R7a).
pub fn one_jobs_reporting_was_set(server: &(impl TheServersName + ?Sized), label: &str, choice: crate::job_messages::TheReportingChoice) -> AMarkedText {
    a_user_message(server, label, "", &format!("Reporting set to \u{201c}{}\u{201d} for this job.", choice.as_the_phone_says()))
}

/// **R1a, widened — an ended job's reporting, asked from a reply.**
pub fn one_jobs_reporting_has_ended(server: &(impl TheServersName + ?Sized), label: &str) -> AMarkedText {
    a_user_message(server, label, "", "This job has already ended. Nothing was changed.")
}

/// **R13a and C1c — the newest frame asked of a job with none finished.**
pub fn no_frame_has_finished(server: &(impl TheServersName + ?Sized), label: &str) -> AMarkedText {
    a_user_message(server, label, "", "No finished frame yet.")
}

/// **A captured stream, read back whole or not at all** (`Rented-server
/// runtime` M11). Nothing is trimmed, prefixed or redacted (`Rented-server runtime` M8),
/// and nothing is mined for a cause (`Rented-server runtime` M24). **Step 10c's on a machine.**
pub trait ReadsTheWordsBack: Send {
    fn read_whole(&self, job: &str, attempt: u32, stream: AStream) -> Result<Vec<u8>, String>;
}

/// **A command's recorded outcome, as the owning machine's journal carries it**
/// — handed to [`TheOwnersActs::report`] by step 10c's journal reader, with the
/// labels the instruction carried (R7).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AJournalledOutcome {
    /// D2's second half.
    AStop { server: AServerLabel, label: String, outcome: AStopsOutcome },
    /// A3: the answer pressed, and what the release recorded. **`waits_for`
    /// is what the hold was for**, read off the hold record with the rest:
    /// a release of a hold for the driver update is told nothing (held-job
    /// piece B, 2026-10-12).
    ARelease { server: AServerLabel, label: String, answer: TheAnswer, frames_held: u64, waits_for: WhatTheHoldWaitsFor, outcome: AReleasesOutcome },
    /// Y1.
    ASubmission { server: AServerLabel, label: String, outcome: ASubmissionsOutcome },
}

// ------------------------------------------------------------ sending

/// **Every send through the one queue** (`Telegram` C6), waiting for its end
/// and sending in its own place where no other thread is — the shape piece 3's
/// routing uses.
fn send_and_wait(queue: &TheSendQueue, class: AClassOfSend, send: crate::pacing::ASend) -> HowASendEnded {
    wait_for(queue, queue.offer(class, send))
}

/// **Wait for a send already offered to end**, sending in its own place where
/// no other thread is. **Public since 2026-10-16** (the Telegram worker's
/// piece 6), for the machine's worker, which sends frame pictures and their
/// answers through the same queue.
pub fn wait_for(queue: &TheSendQueue, told: Receiver<HowASendEnded>) -> HowASendEnded {
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

/// **A text for the user, sent with its notification and no spoiler**
/// (`Telegram` B24, B25: nothing here is machinery).
fn send_text(queue: &TheSendQueue, conversation: i64, class: AClassOfSend, text: AMarkedText) -> HowASendEnded {
    send_and_wait(queue, class, Box::new(move |c: &TheClient| c.send_marked_message(conversation, &text, None)))
}

/// **What a message's buttons are about**: the owning machine, and the job or
/// hold with the generation they are valid in (R1).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheSubject {
    pub owning_machine: String,
    pub job: Option<String>,
    pub hold: Option<String>,
    pub generation: String,
}

/// **Why a message went without its buttons, or at all.**
#[derive(Debug, Clone, PartialEq)]
pub enum WhyAMessageWasNotSent {
    /// The send was not accepted; nothing was recorded.
    NotSent(HowASendEnded),
    /// The message went, and a record could not be written, so **no button
    /// was added**: no button ever exists without its record (R1).
    SentWithoutItsButtons { message: i64 },
    /// The message and its records went, and the call attaching the buttons
    /// was not accepted. The records stay, subject-held (R1).
    TheButtonsWereNotAdded { message: i64 },
}

/// **Why a message's buttons could not be taken off** — the worker's store,
/// or the edit, with how it ended.
#[derive(Debug, Clone, PartialEq)]
pub enum WhyTheButtonsStayed {
    TheStore(String),
    TheEdit { message: i64, ended: HowASendEnded },
}

impl WhyTheButtonsStayed {
    /// Whether the edit reached nothing, so trying again later may land it.
    pub fn the_line_reached_nothing(&self) -> bool {
        matches!(self, WhyTheButtonsStayed::TheEdit { ended: HowASendEnded::HandedBack(WhyTheCallFailed::NotReached(_)), .. })
    }
}

/// **On a companion's Stop record: the message holds nothing but this button**
/// (`Telegram` C1b's line of 2026-11-02).
const THE_MESSAGE_SAYS_ONLY_THIS: &str = "the_message_says_only_its_stop";

/// **Whether an edit taking buttons off is done.** A crash after the edit but
/// before its durable mark replays the same markup, which Telegram answers
/// *not modified*; a deleted message has no control left to remove.
fn the_edit_is_done(ended: &HowASendEnded) -> bool {
    matches!(ended, HowASendEnded::Accepted(_))
        || matches!(
            ended,
            HowASendEnded::HandedBack(WhyTheCallFailed::Refused(why))
                if why.description.as_deref().is_some_and(|text|
                    text.contains("message is not modified") || text.contains("message to edit not found"))
        )
}

/// **A machine's messages about its jobs** — the issuer of E1, A2, A4 and G1's
/// messages and of their buttons.
pub struct TheJobMessages {
    pub conversation: i64,
    /// **One store, shared with the listener** (F2).
    pub store: Box<dyn TheTelegramWorkersState>,
    pub queue: Arc<TheSendQueue>,
    pub notes: Notes,
}

impl TheJobMessages {
    fn note(&mut self, what: &str) {
        (self.notes)(what);
    }

    /// C1b, found by the rented pass: remove ended jobs' Stops from every
    /// earlier message. Keep the contexts for delayed taps, and mark each
    /// successful edit durably so a restart does not edit it again.
    pub fn remove_the_stops_where(
        &mut self,
        ended: &dyn Fn(&AnActionContextRecord) -> bool,
    ) -> Result<usize, String> {
        let records = self.store.the_action_contexts().map_err(|why| why.0)?;
        // *Keep rendering* is the stop question's other answer, so it goes
        // with the job's Stops: kept, it would be a tap that does nothing,
        // where the question's own clearing did not land (the live Telegram
        // pass, fault 6).
        // `Telegram` C1c: *Send a frame* on the status message goes with the
        // job's Stop too (2026-10-31).
        let is_stop = |r: &AnActionContextRecord| {
            [ASK_TO_STOP_THIS_JOB, STOP_THIS_JOB, STOP_AN_ENDED_JOB, KEEP_RENDERING, crate::routing::context::SENDING_THE_NEWEST_FRAME]
                .contains(&r.action.as_str())
        };
        let removing: Vec<_> = records.iter().filter(|r| {
            r.conversation == self.conversation && r.source_message.is_some()
                && is_stop(r) && r.payload["stop_removed"] != json!(true) && ended(r)
        }).collect();
        let messages: std::collections::BTreeSet<_> = removing.iter().filter_map(|r| r.source_message).collect();
        let mut removed = 0;
        for message in messages {
            let mut kept: Vec<_> = records.iter().filter(|r| {
                r.conversation == self.conversation && r.source_message == Some(message)
                    && r.payload["stop_removed"] != json!(true)
                    && !removing.iter().any(|gone| gone.identity == r.identity)
            }).collect();
            kept.sort_by_key(|r| r.payload["button_row"].as_u64().unwrap_or(u64::MAX));
            let rows: Vec<_> = kept.iter().map(|r| {
                let label = r.payload.pointer("/labels/button").and_then(Value::as_str)
                    .unwrap_or_else(|| the_words_of(&r.action));
                json!([if label == "Stop this job" { json!({"text": label, "callback_data": r.token, "style": "danger"}) } else { json!({"text": label, "callback_data": r.token}) }])
            }).collect();
            let conversation = self.conversation;
            // **`Telegram` C1b's line of 2026-11-02: a companion says nothing but
            // its Stop, so it goes when its Stop goes.** Read from its durable
            // record, so a restart deletes it too. **Where Telegram refuses, a
            // message 48 hours old or more (B18), its button still goes below.**
            let a_companion = kept.is_empty()
                && removing
                    .iter()
                    .any(|r| r.source_message == Some(message) && r.payload[THE_MESSAGE_SAYS_ONLY_THIS] == json!(true));
            let deleted = a_companion.then(|| {
                send_and_wait(
                    &self.queue,
                    AClassOfSend::EndingsHoldsAndDrops,
                    Box::new(move |c: &TheClient| c.delete_an_albums_companion(conversation, message)),
                )
            });
            match &deleted {
                None | Some(HowASendEnded::HandedBack(WhyTheCallFailed::Refused(_))) => {
                    let result = send_and_wait(
                        &self.queue,
                        AClassOfSend::EndingsHoldsAndDrops,
                        Box::new(move |c: &TheClient| c.edit_message_reply_markup(conversation, message, json!({ "inline_keyboard": rows }))),
                    );
                    if !the_edit_is_done(&result) {
                        return Err(format!("Stop controls on message {message} could not be removed: {result:?}"));
                    }
                }
                Some(HowASendEnded::Accepted(_)) => {}
                Some(other) => return Err(format!("The companion {message} could not be deleted: {other:?}")),
            }
            for record in removing.iter().filter(|r| r.source_message == Some(message)) {
                let mut marked = (*record).clone();
                marked.payload["stop_removed"] = json!(true);
                self.store.record_an_action_context(marked).map_err(|why| why.0)?;
                removed += 1;
            }
        }
        Ok(removed)
    }

    /// **Every button on one message goes** — `Telegram` C1b's *a tap is only
    /// ever there where it would do something*, for a message whose every
    /// button has stopped meaning anything: a stop question once it is
    /// answered, and a first-frame question once its hold is released (A3),
    /// from the desk or the phone. Found by the live Telegram pass (its fault
    /// 6). **Every record on the message is marked**, as
    /// [`Self::remove_the_stops_where`] marks its own, so no later rebuild of
    /// the message puts a button back; the records themselves stay, so a late
    /// tap is still answered. A message whose every record is already marked
    /// is not edited again; a failed edit marks nothing, so it is owed still.
    /// Gives back how many records were marked.
    pub fn remove_every_button_on(&mut self, message: i64, class: AClassOfSend) -> Result<usize, WhyTheButtonsStayed> {
        let records = self.store.the_action_contexts().map_err(|why| WhyTheButtonsStayed::TheStore(why.0))?;
        let on_it: Vec<_> = records
            .iter()
            .filter(|r| r.conversation == self.conversation && r.source_message == Some(message))
            .collect();
        if !on_it.is_empty() && on_it.iter().all(|r| r.payload["stop_removed"] == json!(true)) {
            return Ok(0);
        }
        let conversation = self.conversation;
        let ended = send_and_wait(
            &self.queue,
            class,
            Box::new(move |c: &TheClient| c.edit_message_reply_markup(conversation, message, json!({ "inline_keyboard": [] }))),
        );
        if !the_edit_is_done(&ended) {
            return Err(WhyTheButtonsStayed::TheEdit { message, ended });
        }
        let mut marked = 0;
        for record in on_it.into_iter().filter(|r| r.payload["stop_removed"] != json!(true)) {
            let mut record = record.clone();
            record.payload["stop_removed"] = json!(true);
            self.store.record_an_action_context(record).map_err(|why| WhyTheButtonsStayed::TheStore(why.0))?;
            marked += 1;
        }
        Ok(marked)
    }

    /// **A message with no buttons**: a finish (E1, G1), a drop (A4), the idle
    /// message (V3). **C6's class**: endings, holds and drops.
    pub fn send(&mut self, text: AMarkedText) -> HowASendEnded {
        let ended = send_text(&self.queue, self.conversation, AClassOfSend::EndingsHoldsAndDrops, text);
        let id = accepted_id(&ended);
        self.note(&format!("sent a job message {}", id.map_or("not accepted".into(), |id| id.to_string())));
        ended
    }

    /// **The same, for a message about one job**, registered as about it once
    /// Telegram accepts it (`Telegram` R1b) — an ending with no buttons.
    pub fn send_about_a_job(&mut self, text: AMarkedText, job: &str) -> HowASendEnded {
        let ended = self.send(text);
        if let Some(message) = accepted_id(&ended) {
            self.register(message, crate::worker_state::WhatAMessageIsAbout::AJob { job: job.to_string(), generation: None, frame: None, layer: None });
        }
        ended
    }

    /// **`Telegram` R1b — one line for a message Telegram accepted.** A line
    /// that is not written costs a reply to that message the deadline, and
    /// nothing else, so it never fails the send.
    pub fn register(&mut self, message: i64, about: crate::worker_state::WhatAMessageIsAbout) {
        let line = crate::worker_state::ASentMessage { message, conversation: self.conversation, about };
        if let Err(why) = self.store.register_a_sent_message(line) {
            self.note(&format!("message {message} was not registered: {}", why.0));
        }
    }

    /// **R1 — every button's record written against a message already sent**,
    /// and the buttons to attach, only once all are written. Each record's
    /// labels carry the button's own label, for what the user is told about it
    /// (R7, Rule 9; fix-here item 13).
    fn the_buttons_recorded_for(
        &mut self,
        message: i64,
        composed: &AComposedMessage,
        subject: &TheSubject,
    ) -> Result<Value, WhyAMessageWasNotSent> {
        let Ok(key) = self.store.the_routing_key() else {
            return Err(WhyAMessageWasNotSent::SentWithoutItsButtons { message });
        };
        let mut rows = Vec::new();
        for (row, button) in composed.buttons.iter().enumerate() {
            let mut payload = button.payload.clone();
            payload["button_row"] = json!(row);
            if button.action == KEEP_RENDERING || payload["confirmed"] == json!(true) {
                payload["confirmation_message"] = json!(message);
            }
            if let Some(fields) = payload.as_object_mut() {
                // Y1 and R4: the submission's own operation identity, one per
                // button, so a second press lands on the supervisor's idempotency.
                if button.action == TRY_IT_AGAIN {
                    let Ok(submission) = Identity::issue(IdentityKind::Operation) else {
                        return Err(WhyAMessageWasNotSent::SentWithoutItsButtons { message });
                    };
                    fields.insert("submission".into(), json!(submission.as_text()));
                }
                if button.action == ANSWER_A_WAITING_JOB {
                    fields.insert("job".into(), json!(subject.job));
                }
                if let Some(labels) = fields.get_mut("labels").and_then(Value::as_object_mut) {
                    labels.insert("button".into(), json!(button.label));
                }
            }
            let Some(mut record) = AnActionContextRecord::issued_under(
                &key,
                self.conversation,
                &subject.owning_machine,
                if button.action == ANSWER_A_WAITING_JOB {
                    None
                } else {
                    subject.job.as_deref()
                },
                if button.action == ANSWER_A_WAITING_JOB || button.action == STOP_AN_ENDED_JOB {
                    subject.hold.as_deref()
                } else {
                    None
                },
                button.action,
                payload,
                &subject.generation,
            ) else {
                return Err(WhyAMessageWasNotSent::SentWithoutItsButtons { message });
            };
            record.source_message = Some(message);
            let token = record.token.clone();
            if self.store.record_an_action_context(record).is_err() {
                self.note(&format!("a record for message {message} could not be written, so no buttons were added"));
                return Err(WhyAMessageWasNotSent::SentWithoutItsButtons { message });
            }
            let mut item = json!({"text": button.label, "callback_data": token});
            if button.label == "Stop this job" { item["style"] = json!("danger"); }
            rows.push(json!([item]));
        }
        Ok(json!({ "inline_keyboard": rows }))
    }

    /// **R1, E3, A2 and Y4a — a text message with buttons.** The message is
    /// sent first, because a record binds its source message's identity; then
    /// every record is written durably; **only then are the buttons attached**,
    /// by an edit that keeps the text (observed accepted, `Telegram` Sources
    /// entry 84), so no button ever exists without its record. Gives back the
    /// message identity.
    pub fn send_with_buttons(
        &mut self,
        composed: &AComposedMessage,
        subject: &TheSubject,
    ) -> Result<i64, WhyAMessageWasNotSent> {
        self.send_with_buttons_in(composed, subject, AClassOfSend::EndingsHoldsAndDrops)
    }

    pub fn send_with_buttons_in(
        &mut self,
        composed: &AComposedMessage,
        subject: &TheSubject,
        class: AClassOfSend,
    ) -> Result<i64, WhyAMessageWasNotSent> {
        let sent = send_text(&self.queue, self.conversation, class, composed.text.clone());
        let Some(message) = accepted_id(&sent) else {
            self.note("a job message was not accepted");
            return Err(WhyAMessageWasNotSent::NotSent(sent));
        };
        // R1b: a message about a job is registered as about it.
        if let Some(job) = &subject.job {
            self.register(
                message,
                crate::worker_state::WhatAMessageIsAbout::AJob { job: job.clone(), generation: Some(subject.generation.clone()), frame: None, layer: None },
            );
        }
        if composed.buttons.is_empty() {
            self.note(&format!("sent a job message {message} with no buttons"));
            return Ok(message);
        }
        let markup = self.the_buttons_recorded_for(message, composed, subject)?;
        let (conversation, text) = (self.conversation, composed.text.clone());
        let added = send_and_wait(
            &self.queue,
            class,
            Box::new(move |c: &TheClient| {
                c.edit_marked_message(conversation, message, &text, Some(markup.clone()))
            }),
        );
        let ok = matches!(added, HowASendEnded::Accepted(_));
        self.note(&format!("sent a job message {message} with {} buttons, added = {ok}", composed.buttons.len()));
        if ok {
            Ok(message)
        } else {
            Err(WhyAMessageWasNotSent::TheButtonsWereNotAdded { message })
        }
    }

    /// **R1 — attach buttons to a message already sent, changing nothing else**
    /// (`TheClient::edit_message_reply_markup`, U4's call).
    fn attach(&mut self, message: i64, markup: Value, class: AClassOfSend) -> Result<(), WhyAMessageWasNotSent> {
        let conversation = self.conversation;
        let added = send_and_wait(
            &self.queue,
            class,
            Box::new(move |c: &TheClient| c.edit_message_reply_markup(conversation, message, markup.clone())),
        );
        let ok = matches!(added, HowASendEnded::Accepted(_));
        self.note(&format!("attached buttons to message {message}, accepted = {ok}"));
        if ok {
            Ok(())
        } else {
            Err(WhyAMessageWasNotSent::TheButtonsWereNotAdded { message })
        }
    }

    /// **C1b — a message about a job still live when it is composed**: its
    /// other actions kept, and Stop added. **A subject with no job — an old
    /// hold whose job is gone — is never live**, so it gets no Stop. Every
    /// caller decides liveness from the journal first; a message about an
    /// ending goes through [`Self::send_with_buttons_in`] or [`Self::send`]
    /// instead, with no Stop (eighteenth notes, note 16, 2026-10-25).
    pub fn send_about_in(
        &mut self,
        composed: &AComposedMessage,
        subject: &TheSubject,
        server: &(impl TheServersName + ?Sized),
        label: &str,
        class: AClassOfSend,
    ) -> Result<i64, WhyAMessageWasNotSent> {
        let mut composed = composed.clone();
        if subject.job.is_some() {
            composed.buttons.push(a_stop_to_offer(server, label, false));
        }
        self.send_with_buttons_in(&composed, subject, class)
    }

    /// **C1b's short companion, after an album, which cannot carry a button
    /// itself**: the machine, the job and the frame, with *Stop this job*. **Its
    /// Stop's durable record says it holds nothing else**, which is how the
    /// removal knows to delete it (C1b's line of 2026-11-02). **A subject with
    /// no job is never live, so nothing is sent**: there would be nothing to say.
    pub fn send_a_companion(
        &mut self,
        text: AMarkedText,
        subject: &TheSubject,
        server: &(impl TheServersName + ?Sized),
        label: &str,
        class: AClassOfSend,
    ) -> Result<(), WhyAMessageWasNotSent> {
        if subject.job.is_none() {
            return Ok(());
        }
        let mut stop = a_stop_to_offer(server, label, false);
        stop.payload[THE_MESSAGE_SAYS_ONLY_THIS] = json!(true);
        self.send_with_buttons_in(&AComposedMessage { text, buttons: vec![stop] }, subject, class)
            .map(|_| ())
    }

    /// **C1b at composition**: Stop where `live`, none where the job has
    /// ended, the message's other actions kept either way.
    pub fn send_about_while(
        &mut self,
        live: bool,
        composed: &AComposedMessage,
        subject: &TheSubject,
        server: &(impl TheServersName + ?Sized),
        label: &str,
        class: AClassOfSend,
    ) -> Result<i64, WhyAMessageWasNotSent> {
        if live {
            self.send_about_in(composed, subject, server, label, class)
        } else {
            self.send_with_buttons_in(composed, subject, class)
        }
    }

    pub fn send_about_with_buttons(
        &mut self,
        composed: &AComposedMessage,
        subject: &TheSubject,
        server: &(impl TheServersName + ?Sized),
        label: &str,
    ) -> Result<i64, WhyAMessageWasNotSent> {
        self.send_about_in(
            composed,
            subject,
            server,
            label,
            AClassOfSend::EndingsHoldsAndDrops,
        )
    }

    pub fn send_about(
        &mut self,
        text: AMarkedText,
        subject: &TheSubject,
        server: &(impl TheServersName + ?Sized),
        label: &str,
    ) -> Result<i64, WhyAMessageWasNotSent> {
        self.send_about_with_buttons(
            &AComposedMessage {
                text,
                buttons: vec![],
            },
            subject,
            server,
            label,
        )
    }

    /// Attach to a lone accepted message; for an album use a separate message.
    pub fn control_the_accepted_message(
        &mut self,
        ended: &HowASendEnded,
        subject: &TheSubject,
        server: &(impl TheServersName + ?Sized),
        label: &str,
        companion: &str,
        class: AClassOfSend,
    ) -> Result<(), WhyAMessageWasNotSent> {
        match ended {
            HowASendEnded::Accepted(result) if result.is_array() => {
                self.send_a_companion(companion.into(), subject, server, label, class)
            }
            _ => match accepted_id(ended) {
                Some(message) => self.attach_a_stop(message, subject, server, label, class),
                None => Ok(()),
            },
        }
    }

    /// A3a: pictures belong to the check. Answers and Stop belong to the
    /// held range, on a separate message even when there is only one copy.
    pub fn send_the_waiting_set(
        &mut self,
        waiting: &AWaitingJob,
        copies: Vec<ADerivative>,
        subject: &TheSubject,
    ) -> Result<Option<i64>, WhyAMessageWasNotSent> {
        let Some(mut composed) = the_waiting_message(waiting) else {
            return Ok(None);
        };
        if copies.is_empty() {
            composed
                .text
                .push_str("\nNo preview available.");
        } else {
            let mut pictures =
                crate::pictures::ThePictures::new(self.conversation, &waiting.server);
            if let crate::pictures::APictureSent::NotAccepted(ended) = pictures
                .send_a_frame_with_its_stop(self, copies, AClassOfSend::EndingsHoldsAndDrops, None)
            {
                if matches!(
                    ended,
                    HowASendEnded::HandedBack(WhyTheCallFailed::NotReached(_))
                ) {
                    return Err(WhyAMessageWasNotSent::NotSent(ended));
                }
            }
        }
        self.send_the_waiting_question_composed(waiting, composed, subject)
    }

    pub fn send_the_waiting_question(
        &mut self,
        waiting: &AWaitingJob,
        no_picture: bool,
        subject: &TheSubject,
    ) -> Result<Option<i64>, WhyAMessageWasNotSent> {
        let Some(mut composed) = the_waiting_message(waiting) else {
            return Ok(None);
        };
        if no_picture {
            composed
                .text
                .push_str("\nNo preview available.");
        }
        self.send_the_waiting_question_composed(waiting, composed, subject)
    }

    fn send_the_waiting_question_composed(
        &mut self,
        waiting: &AWaitingJob,
        composed: AComposedMessage,
        subject: &TheSubject,
    ) -> Result<Option<i64>, WhyAMessageWasNotSent> {
        if subject.job.is_some() {
            self.send_about_with_buttons(&composed, subject, &waiting.server, &waiting.label)
                .map(Some)
        } else {
            self.send_with_buttons(&composed, subject).map(Some)
        }
    }

    pub fn send_the_waiting_message(
        &mut self,
        waiting: &AWaitingJob,
        picture: Option<&ADerivative>,
        subject: &TheSubject,
    ) -> Result<Option<i64>, WhyAMessageWasNotSent> {
        self.send_the_waiting_set(waiting, picture.cloned().into_iter().collect(), subject)
    }

    /// **D1's button, for a message another piece sends** — see
    /// [`a_stop_button_on`].
    pub fn a_stop_button(&mut self, message: i64, subject: &TheSubject, server: &(impl TheServersName + ?Sized), label: &str) -> Option<Value> {
        a_stop_button_on(self.store.as_mut(), self.conversation, message, subject, server, label)
    }

    /// **D1, R1 and fix-here item 6 — *Stop this job* attached to a picture or
    /// a moving picture already sent**: its record written against the
    /// message, then the button attached by the call that changes only a
    /// message's buttons. `class` is the class of the message it rides.
    /// **Needs the real service**: whether Telegram accepts that call on a photo
    /// and on a video is owed to the live pass (`Telegram` R1).
    pub fn attach_a_stop(
        &mut self,
        message: i64,
        subject: &TheSubject,
        server: &(impl TheServersName + ?Sized),
        label: &str,
        class: AClassOfSend,
    ) -> Result<(), WhyAMessageWasNotSent> {
        let Some(button) = self.a_stop_button(message, subject, server, label) else {
            return Err(WhyAMessageWasNotSent::SentWithoutItsButtons { message });
        };
        self.attach(message, json!({ "inline_keyboard": [[button]] }), class)
    }
}

/// D0/R1: an initial Stop control opens a question. Its durable record binds
/// this message and this job/attempt; only the question's affirmative action
/// uses the existing stop command. Albums use a separate companion.
pub(crate) fn a_stop_button_on(
    store: &mut dyn TheTelegramWorkersState,
    conversation: i64,
    message: i64,
    subject: &TheSubject,
    server: &(impl TheServersName + ?Sized),
    label: &str,
) -> Option<Value> {
    // C1b: a subject with no job is an old hold whose job is gone, never live.
    subject.job.as_ref()?;
    let key = store.the_routing_key().ok()?;
    let mut record = AnActionContextRecord::issued_under(
        &key,
        conversation,
        &subject.owning_machine,
        subject.job.as_deref(),
        None,
        ASK_TO_STOP_THIS_JOB,
        a_stop_to_offer(server, label, false).payload,
        &subject.generation,
    )?;
    record.source_message = Some(message);
    let token = record.token.clone();
    store.record_an_action_context(record).ok()?;
    Some(json!({ "text": "Stop this job", "callback_data": token, "style": "danger" }))
}

pub fn message_result(sent: Result<i64, WhyAMessageWasNotSent>) -> HowASendEnded {
    match sent {
        Ok(message) => HowASendEnded::Accepted(json!({"message_id": message})),
        Err(WhyAMessageWasNotSent::NotSent(ended)) => ended,
        Err(
            WhyAMessageWasNotSent::SentWithoutItsButtons { message }
            | WhyAMessageWasNotSent::TheButtonsWereNotAdded { message },
        ) => HowASendEnded::Accepted(json!({"message_id": message})),
    }
}

pub fn accepted_id_for_a_job_message(ended: &HowASendEnded) -> Option<i64> {
    accepted_id(ended)
}

pub(crate) fn accepted_id(ended: &HowASendEnded) -> Option<i64> {
    match ended {
        HowASendEnded::Accepted(result) => result.get("message_id").and_then(Value::as_i64),
        _ => None,
    }
}

// ------------------------------------------------------------ the owner's acts

/// The labels a record's payload carries for the answers (R7).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct TheLabels {
    #[serde(default)]
    server: AServerLabel,
    #[serde(default)]
    label: String,
    /// The button's own label, where the record carries it.
    #[serde(default)]
    button: String,
}

/// **What the owning machine does with a resolved tap** — `Telegram` D1, A2,
/// Y1 and E3, on B9.
pub struct TheOwnersActs {
    pub conversation: i64,
    pub supervisor: Box<dyn TheSupervisorsCommandPath>,
    pub words: Box<dyn ReadsTheWordsBack>,
    /// The live generation of each job and hold this machine owns (R1).
    pub holds: Box<dyn ReadsWhatThisMachineHolds>,
    pub queue: Arc<TheSendQueue>,
    pub notes: Notes,
}

impl TheOwnersActs {
    fn note(&mut self, what: &str) {
        (self.notes)(what);
    }

    fn answer(&mut self, class: AClassOfSend, text: AMarkedText) -> HowASendEnded {
        let ended = send_text(&self.queue, self.conversation, class, text);
        let ok = matches!(ended, HowASendEnded::Accepted(_));
        self.note(&format!("answered the user, accepted = {ok}"));
        ended
    }

    fn messages(&self) -> Option<TheJobMessages> {
        Some(TheJobMessages {
            conversation: self.conversation,
            store: self.supervisor.the_action_context_store()?,
            queue: self.queue.clone(),
            notes: Box::new(|_| {}),
        })
    }

    fn subject_of(entry: &AnEntry) -> Option<TheSubject> {
        let job = if entry.action == ANSWER_A_WAITING_JOB {
            entry.payload["job"].as_str()?
        } else {
            entry.subjects.first()?.as_str()
        };
        Some(TheSubject {
            owning_machine: match &entry.addressee {
                crate::board::instructions::TheAddressee::Machine(m) => m.clone(),
                _ => return None,
            },
            job: Some(job.into()),
            hold: None,
            generation: entry.payload["generation"]
                .as_str()
                .unwrap_or_default()
                .into(),
        })
    }

    /// **C1b — whether the job an entry names is live now**: waiting,
    /// rendering or held, as this machine reads it while the answer is
    /// composed.
    fn it_is_live(&self, subject: Option<&TheSubject>) -> bool {
        subject
            .and_then(|subject| subject.job.as_deref())
            .is_some_and(|job| self.holds.the_stop_state(job) != crate::routing::gather::AStopState::Ended)
    }

    /// **C1b — an answer about a job**, with Stop only while that job is live.
    fn answer_about(&mut self, entry: &AnEntry, labels: &TheLabels, text: AMarkedText) -> HowASendEnded {
        let subject = Self::subject_of(entry);
        if !self.it_is_live(subject.as_ref()) {
            return self.answer(AClassOfSend::RoutedAnswersAndAcknowledgements, text);
        }
        if let (Some(mut messages), Some(subject)) = (self.messages(), subject) {
            return message_result(messages.send_about_in(
                &AComposedMessage {
                    text,
                    buttons: vec![],
                },
                &subject,
                &labels.server,
                &labels.label,
                AClassOfSend::RoutedAnswersAndAcknowledgements,
            ));
        }
        self.answer(AClassOfSend::RoutedAnswersAndAcknowledgements, text)
    }

    /// **C1b — *This job has already ended.*, never with a Stop of its own**,
    /// and no earlier message edited. A line that reached nothing is `Err`,
    /// so the tap is tried again.
    fn say_it_has_already_ended(&mut self, labels: &TheLabels) -> Result<(), String> {
        let text = the_job_already_ended(&labels.server, &labels.label);
        match self.answer(AClassOfSend::RoutedAnswersAndAcknowledgements, text) {
            HowASendEnded::HandedBack(WhyTheCallFailed::NotReached(why)) => Err(why),
            _ => Ok(()),
        }
    }

    /// **`Telegram` V1d on this machine**: every live job changed and the
    /// phone's choice written, through the supervisor's path, then the
    /// outcome recorded with the count. Nothing is said here; the reply is the
    /// listener's.
    fn the_phones_choice(&mut self, entry: &AnEntry) -> Result<(), String> {
        let Some(value) = crate::job_messages::TheReportingChoice::exactly(&entry.payload["value"]) else {
            return self.supervisor.record_the_outcome(
                &entry.operation,
                &ARecordedOutcome::Failed("the phone's choice is not one a job can hold".into()),
            );
        };
        let choice = crate::worker_state::ThePhonesChoice {
            value: value.in_the_manifests_form(),
            operation: entry.payload["operation"].as_str().unwrap_or(&entry.operation).to_string(),
            date: entry.payload["date"].as_i64().unwrap_or_default(),
            update: entry.payload["update"].as_i64().or(entry.source_update).unwrap_or_default(),
        };
        match self.supervisor.change_every_live_jobs_reporting(&entry.operation, &choice) {
            Some(changed) => {
                let changed = changed?;
                self.note(&format!("{}: the phone's choice changed {changed} jobs", entry.operation));
                self.supervisor.record_the_outcome(&entry.operation, &ARecordedOutcome::Done)
            }
            None => self.supervisor.record_the_outcome(
                &entry.operation,
                &ARecordedOutcome::Failed("no actor changes a job's reporting here".into()),
            ),
        }
    }

    /// **`Telegram` R1a, widened — one job's reporting, from a reply**: with a
    /// value, V1c's change for that job alone, the default untouched; with
    /// none, that job's own ask of five buttons, each a record on this
    /// machine (R1).
    fn one_jobs_reporting(&mut self, entry: &AnEntry, job: &str, labels: &TheLabels) -> Result<(), String> {
        let ended = || matches!(self.holds.the_stop_state(job), crate::routing::gather::AStopState::Ended);
        let Some(value) = crate::job_messages::TheReportingChoice::exactly(&entry.payload["value"]) else {
            if ended() {
                self.answer(AClassOfSend::RoutedAnswersAndAcknowledgements, one_jobs_reporting_has_ended(&labels.server, &labels.label));
                return self.supervisor.record_the_outcome(&entry.operation, &ARecordedOutcome::Failed(THE_JOB_OR_ATTEMPT_ENDED.into()));
            }
            let owner = match &entry.addressee {
                crate::board::instructions::TheAddressee::Machine(m) => m.clone(),
                _ => return Err("a reporting change has no owner".into()),
            };
            let Some(mut messages) = self.messages() else {
                return self.supervisor.record_the_outcome(&entry.operation, &ARecordedOutcome::Failed("no context store for an ask".into()));
            };
            let generation = self.holds.the_live_generation(job).unwrap_or_default();
            let about = TheSubject { owning_machine: owner, job: Some(job.to_string()), hold: None, generation };
            let ask = AComposedMessage {
                text: a_user_message(&labels.server, &labels.label, "", &format!("Reporting for this job:\n{}", crate::routing::FOR_OTHER_MINUTES)),
                buttons: crate::routing::THE_FIVE_CHOICES
                    .iter()
                    .map(|choice| crate::job_messages::AButtonToOffer {
                        label: choice.as_the_phone_says(),
                        action: crate::routing::context::ONE_JOBS_REPORTING,
                        payload: json!({ "job": job, "value": choice.in_the_manifests_form(), "labels": { "server": labels.server, "label": labels.label } }),
                    })
                    .collect(),
            };
            return match messages.send_with_buttons_in(&ask, &about, AClassOfSend::RoutedAnswersAndAcknowledgements) {
                Err(WhyAMessageWasNotSent::NotSent(HowASendEnded::HandedBack(WhyTheCallFailed::NotReached(why)))) => Err(why),
                _ => self.supervisor.record_the_outcome(&entry.operation, &ARecordedOutcome::Done),
            };
        };
        match self.supervisor.change_one_jobs_reporting(&entry.operation, job, value) {
            Some(outcome) => {
                let outcome = outcome?;
                let text = match &outcome {
                    ARecordedOutcome::Done => one_jobs_reporting_was_set(&labels.server, &labels.label, value),
                    ARecordedOutcome::Failed(_) => one_jobs_reporting_has_ended(&labels.server, &labels.label),
                };
                self.answer(AClassOfSend::RoutedAnswersAndAcknowledgements, text);
                self.supervisor.record_the_outcome(&entry.operation, &outcome)
            }
            None => self.supervisor.record_the_outcome(
                &entry.operation,
                &ARecordedOutcome::Failed("no actor changes a job's reporting here".into()),
            ),
        }
    }

    /// **D2, A3 and Y1's report, with Stop only where C1b puts one**: never on
    /// a stop's outcome (D2), and on an answer or a submission only while the
    /// job it names is live when the report is composed — a held range
    /// released to render, or a job just sent again; not one cancelled.
    pub fn report_about(
        &mut self,
        journalled: AJournalledOutcome,
        subject: &TheSubject,
        server: &(impl TheServersName + ?Sized),
        label: &str,
    ) -> Option<HowASendEnded> {
        let a_stops_outcome = matches!(journalled, AJournalledOutcome::AStop { .. });
        let ended = self.report(journalled)?;
        if a_stops_outcome || !self.it_is_live(Some(subject)) {
            return Some(ended);
        }
        if let Some(mut messages) = self.messages() {
            let _ = messages.control_the_accepted_message(
                &ended,
                subject,
                server,
                label,
                &format!("{}\n{label}", server.label()),
                AClassOfSend::RoutedAnswersAndAcknowledgements,
            );
        }
        Some(ended)
    }

    /// **The stop question, answered, loses every button** — and, since the
    /// live Telegram pass (its fault 6), every record on it is marked, so the
    /// job's ending does not rebuild *Keep rendering* onto it
    /// ([`TheJobMessages::remove_every_button_on`]). Without the worker's
    /// store the buttons are cleared alone, as before.
    fn close_the_question(&self, entry: &AnEntry) {
        if let Some(message) = entry.payload["confirmation_message"].as_i64() {
            if let Some(mut messages) = self.messages() {
                let _ = messages.remove_every_button_on(message, AClassOfSend::RoutedAnswersAndAcknowledgements);
                return;
            }
            let conversation = self.conversation;
            send_and_wait(
                &self.queue,
                AClassOfSend::RoutedAnswersAndAcknowledgements,
                Box::new(move |c| {
                    c.edit_message_reply_markup(
                        conversation,
                        message,
                        json!({"inline_keyboard": []}),
                    )
                }),
            );
        }
    }

    /// **E3 and E3d — the words, read back whole and sent in the chat.** Gives
    /// back the outcome to record, or `Err` where the words message reached
    /// nothing, so the tap is tried again.
    fn the_words(&mut self, entry: &AnEntry, job: &str, labels: &TheLabels) -> Result<ARecordedOutcome, String> {
        let stream: AStream = serde_json::from_value(entry.payload["stream"].clone()).map_err(|e| e.to_string())?;
        let attempt = entry.payload["attempt"].as_u64().unwrap_or(1) as u32;
        let incomplete = entry.payload["incomplete"].as_bool().unwrap_or(false);
        let bytes = match self.words.read_whole(job, attempt, stream) {
            Err(why) => {
                self.note(&format!(
                    "the words for {} could not be read: {why}",
                    entry.operation
                ));
                self.answer_about(
                    entry,
                    labels,
                    the_words_could_not_be_read(&labels.server, &labels.label, stream),
                );
                return Ok(ARecordedOutcome::Failed(
                    "the words could not be read back whole".into(),
                ));
            }
            Ok(bytes) => bytes,
        };
        // **E3d/R7a: ordinary log text under marked names, never markup**, in C6's first class: it is the answer to a tap.
        let told = the_words_in_the_chat(&labels.server, &labels.label, stream, &bytes, incomplete);
        let (conversation, text) = (self.conversation, told.text.clone());
        let ended = send_and_wait(
            &self.queue,
            AClassOfSend::RoutedAnswersAndAcknowledgements,
            Box::new(move |c: &TheClient| c.send_marked_message(conversation, &text, None)),
        );
        let ok = matches!(ended, HowASendEnded::Accepted(_));
        self.note(&format!(
            "sent the words for {} in the chat, {} units, a file to follow = {}, accepted = {ok}",
            entry.operation,
            how_long_it_counts(&told.text),
            told.file.is_some()
        ));
        if let HowASendEnded::HandedBack(WhyTheCallFailed::NotReached(why)) = &ended {
            return Err(format!("the words message reached nothing: {why}"));
        }
        // **C1b: requested output carries Stop only while its job is live**;
        // the words of a job that has ended — the usual case, asked for after
        // a failure — carry none, and neither does the file after them.
        let live = self.it_is_live(Self::subject_of(entry).as_ref());
        if let (true, Some(mut messages), Some(subject)) = (live, self.messages(), Self::subject_of(entry)) {
            let _ = messages.control_the_accepted_message(
                &ended,
                &subject,
                &labels.server,
                &labels.label,
                "",
                AClassOfSend::RoutedAnswersAndAcknowledgements,
            );
        }
        // **E3d: the whole stream as a file, a second message straight after
        // the block**, and only once the block went, so no file arrives with
        // nothing above it. **C6b: in the upload lane**, which names the words
        // file, so it is never cut for the heartbeat and never delays an ending
        // or an answer. **It is offered and not waited for**: this runs inside
        // the listener's turn, and a turn held for an upload is a heartbeat held
        // too.
        if let (true, Some(name)) = (ok, told.file) {
            let (caption, length) = (a_user_message(&labels.server, &labels.label, "", ""), bytes.len());
            let mut store = self.supervisor.the_action_context_store();
            let subject = Self::subject_of(entry).filter(|_| live);
            let labels = labels.clone();
            drop(self.queue.offer_an_upload(Box::new(move |c: &TheClient| {
                let result =
                    c.send_document_marked(conversation, &name, bytes.clone(), Some(&caption), None)?;
                if let (Some(store), Some(subject), Some(id)) = (
                    store.as_mut(),
                    subject.as_ref(),
                    result.get("message_id").and_then(Value::as_i64),
                ) {
                    if let Some(button) = a_stop_button_on(
                        store.as_mut(),
                        conversation,
                        id,
                        subject,
                        &labels.server,
                        &labels.label,
                    ) {
                        c.edit_message_reply_markup(
                            conversation,
                            id,
                            json!({"inline_keyboard": [[button]]}),
                        )?;
                    }
                }
                Ok(result)
            })));
            self.note(&format!(
                "offered the whole words for {} as a file of {length} bytes (C6b)",
                entry.operation
            ));
        }
        Ok(ARecordedOutcome::Done)
    }

    /// **D2, A3 and Y1 — a command's recorded outcome, reported as its own
    /// message when it is journalled.** The rule that composes each message is
    /// here; reading the journal event on a machine is step 10c's (piece map §8
    /// item 13). **A3: the answer that won is told too** (fix-here item 10), and
    /// the one that lost is told which was applied and when.
    ///
    /// **A release of a hold for the driver update is told nothing** (A3), so
    /// `None`: no send was offered. Held-job piece B, 2026-10-12.
    pub fn report(&mut self, journalled: AJournalledOutcome) -> Option<HowASendEnded> {
        Some(match journalled {
            AJournalledOutcome::AStop { server, label, outcome } => {
                self.answer(AClassOfSend::EndingsHoldsAndDrops, the_stops_outcome(&server, &label, &outcome))
            }
            AJournalledOutcome::ARelease { waits_for, .. } if !waits_for.the_phone_is_told() => {
                self.note("the release of a hold for the driver update: nothing told (A3)");
                return None;
            }
            AJournalledOutcome::ARelease { server, label, answer, frames_held, outcome: AReleasesOutcome::Applied, .. } => {
                self.answer(AClassOfSend::RoutedAnswersAndAcknowledgements, the_answer_was_applied(&server, &label, answer, frames_held))
            }
            AJournalledOutcome::ARelease { server, label, frames_held, outcome: AReleasesOutcome::AnotherAnswerWasApplied { answer, at }, .. } => {
                // `Rented-server runtime` U14: told which answer won, and when.
                self.answer(
                    AClassOfSend::RoutedAnswersAndAcknowledgements,
                    the_answer_already_applied(&server, &label, answer, frames_held, at),
                )
            }
            AJournalledOutcome::ASubmission { server, label, outcome } => {
                self.answer(AClassOfSend::RoutedAnswersAndAcknowledgements, the_submissions_outcome(&server, &label, &outcome))
            }
        })
    }
}

/// **The button's own words where a record carries none** — only for records
/// written before the label was kept.
fn the_words_of(action: &str) -> &'static str {
    match action {
        STOP_THIS_JOB | ASK_TO_STOP_THIS_JOB => "Stop this job",
        TRY_IT_AGAIN => "Retry",
        ANSWER_A_WAITING_JOB => "the answer",
        _ => "the action",
    }
}

impl ActsOnAnInstruction for TheOwnersActs {
    fn act_once(&mut self, entry: &AnEntry) -> Result<(), String> {
        // **B9 and `Server protocol and transfers` C4: a repeated delivery finds
        // the recorded outcome and acts on nothing.**
        if let Some(recorded) = self.supervisor.the_recorded_outcome(&entry.operation)? {
            self.note(&format!("{} already has its outcome recorded: {recorded:?}", entry.operation));
            return Ok(());
        }
        let labels: TheLabels = serde_json::from_value(entry.payload["labels"].clone()).unwrap_or_default();
        // **`Telegram` B9a: an entry of a protocol version this server does not
        // speak is never acted on, never partly acted on and never guessed
        // at.** It is recorded as refused for that reason, so it is
        // acknowledged (B9), and the user is told the server's Smoozy is older
        // — never that it has not confirmed (B12).
        if !entry.is_spoken_here() {
            self.note(&format!("{} is of protocol version {}, which this server does not speak; refused", entry.operation, entry.version));
            self.answer(
                AClassOfSend::RoutedAnswersAndAcknowledgements,
                an_older_version_could_not_do_it(&labels.server, &labels.label, &labels.button),
            );
            return self
                .supervisor
                .record_the_outcome(&entry.operation, &ARecordedOutcome::Failed(WHY_AN_ENTRY_OF_ANOTHER_VERSION_IS_REFUSED.into()));
        }
        let subject = entry.subjects.first().cloned().unwrap_or_default();
        // **C1b: an old control whose job has gone** — kept on its message,
        // since no earlier message is edited — **answers that the job has
        // already ended, with no Stop of its own, and stops nothing.**
        if entry.action == STOP_AN_ENDED_JOB {
            self.say_it_has_already_ended(&labels)?;
            return self.supervisor.record_the_outcome(&entry.operation, &ARecordedOutcome::Done);
        }
        // **V1d — the tap on the ask started its round where it was resolved**
        // (`routing`); the press itself is recorded and nothing else.
        if entry.action == crate::routing::context::CHOOSING_THE_PHONES_CHOICE {
            return self.supervisor.record_the_outcome(&entry.operation, &ARecordedOutcome::Done);
        }
        if entry.action == crate::routing::context::EVERY_LIVE_JOBS_REPORTING {
            return self.the_phones_choice(entry);
        }
        if entry.action == crate::routing::context::ONE_JOBS_REPORTING {
            return self.one_jobs_reporting(entry, &subject, &labels);
        }
        let confirmation = entry.payload["confirmation_identity"]
            .as_str()
            .map(|id| format!("stop-question-{id}"))
            .or_else(|| entry.payload["confirmation_message"].as_i64().map(|id| format!("stop-question-{id}")));
        if [ASK_TO_STOP_THIS_JOB, STOP_THIS_JOB, KEEP_RENDERING].contains(&entry.action.as_str()) {
            let live = the_owner_finds_the_subject_live(entry, &|subject| {
                self.holds.the_live_generation(subject)
            });
            if !live
                || self.holds.the_stop_state(&subject) == crate::routing::gather::AStopState::Ended
            {
                self.say_it_has_already_ended(&labels)?;
                return self.supervisor.record_the_outcome(
                    &entry.operation,
                    &ARecordedOutcome::Failed("the job or attempt has ended".into()),
                );
            }
        }
        if (entry.action == KEEP_RENDERING
            || (entry.action == STOP_THIS_JOB && entry.payload["confirmed"] == json!(true)))
            && confirmation.is_some()
        {
            if self
                .supervisor
                .the_recorded_outcome(confirmation.as_deref().unwrap())?
                .is_some()
            {
                self.answer_about(
                    entry,
                    &labels,
                    a_user_message(&labels.server, &labels.label, "", "Already answered."),
                );
                return self
                    .supervisor
                    .record_the_outcome(&entry.operation, &ARecordedOutcome::Done);
            }
        }
        if entry.action == KEEP_RENDERING {
            self.close_the_question(entry);
            if let Some(key) = &confirmation {
                self.supervisor
                    .record_the_outcome(key, &ARecordedOutcome::Failed("kept rendering".into()))?;
            }
            return self
                .supervisor
                .record_the_outcome(&entry.operation, &ARecordedOutcome::Done);
        }
        if entry.action == ASK_TO_STOP_THIS_JOB || entry.action == STOP_THIS_JOB {
            if entry.action == ASK_TO_STOP_THIS_JOB || entry.payload["confirmed"] != json!(true) {
                let Some(mut messages) = self.messages() else {
                    self.answer(
                        AClassOfSend::RoutedAnswersAndAcknowledgements,
                        "Could not confirm the stop. Try again.".into(),
                    );
                    return self.supervisor.record_the_outcome(
                        &entry.operation,
                        &ARecordedOutcome::Failed("no context store for a confirmation".into()),
                    );
                };
                let owner = match &entry.addressee {
                    crate::board::instructions::TheAddressee::Machine(m) => m.clone(),
                    _ => return Err("a stop has no owner".into()),
                };
                // Read and check together: composing the question must never
                // rebind an old button to a successor observed on this read.
                let generation = self.holds.the_live_generation(&subject);
                if !the_owner_finds_the_subject_live(entry, &|_| generation.clone()) {
                    self.say_it_has_already_ended(&labels)?;
                    return self.supervisor.record_the_outcome(
                        &entry.operation,
                        &ARecordedOutcome::Failed("the job or attempt has ended".into()),
                    );
                }
                let about = TheSubject {
                    owning_machine: owner,
                    job: Some(subject.clone()),
                    hold: None,
                    generation: generation.ok_or("a stop has no live generation")?,
                };
                let mut question = the_stop_question(
                    &labels.server,
                    &labels.label,
                    self.holds.the_stop_state(&subject)
                        == crate::routing::gather::AStopState::Waiting,
                );
                let identity = Identity::issue(IdentityKind::Operation).map_err(|_| "a stop question could not be identified")?.as_text();
                for button in &mut question.buttons { button.payload["confirmation_identity"] = json!(identity); }
                match messages.send_with_buttons_in(
                    &question,
                    &about,
                    AClassOfSend::RoutedAnswersAndAcknowledgements,
                ) {
                    Ok(_) => {}
                    Err(WhyAMessageWasNotSent::NotSent(HowASendEnded::HandedBack(
                        WhyTheCallFailed::NotReached(why),
                    ))) => return Err(why),
                    Err(_) => {}
                }
                return self
                    .supervisor
                    .record_the_outcome(&entry.operation, &ARecordedOutcome::Done);
            }
            self.close_the_question(entry);
        }
        // R1, on the owner: a generation that has ended is acted on by nothing,
        // and the user is told nothing was done.
        let live = the_owner_finds_the_subject_live(entry, &|subject| self.holds.the_live_generation(subject));
        if !live {
            self.note(&format!("{} names a subject no longer live; nothing was done", entry.operation));
            self.answer(AClassOfSend::RoutedAnswersAndAcknowledgements, AMarkedText::plain(A_BUTTON_THAT_EXPIRED));
            return self.supervisor.record_the_outcome(&entry.operation, &ARecordedOutcome::Failed("the subject has ended".into()));
        }
        let outcome = match entry.action.as_str() {
            // D1: the owning machine's role worker submits the stop to its own
            // supervisor, and returns once the submission is acknowledged; D2's
            // second half is the journalled outcome, reported by `report`.
            STOP_THIS_JOB => self.supervisor.submit_a_confirmed_stop(
                &entry.operation,
                &subject,
                entry.payload["generation"]
                    .as_str()
                    .ok_or("the stop carries no generation")?,
            )?,
            // A2: the press becomes one release command on the owner.
            ANSWER_A_WAITING_JOB => {
                let answer: TheAnswer = serde_json::from_value(entry.payload["answer"].clone()).map_err(|e| e.to_string())?;
                self.supervisor.release_a_hold(&entry.operation, &subject, answer)?
            }
            TRY_IT_AGAIN => {
                let submission = entry.payload["submission"].as_str().ok_or("the record carries no submission identity")?.to_string();
                self.supervisor.submit_again(&entry.operation, &submission, &subject)?
            }
            SEND_THE_WORDS => {
                let outcome = self.the_words(entry, &subject, &labels)?;
                return self.supervisor.record_the_outcome(&entry.operation, &outcome);
            }
            // **R14, until step 10c composes the actor that answers a routed frame
            // request** (`Piece 7 — what landed.md`): the user is told
            // truthfully that the frame cannot be sent yet, and the outcome is
            // recorded, rather than an error that silently acknowledged it.
            SENDING_A_FRAME => {
                let frame = entry.payload["frame"].as_u64().unwrap_or_default();
                // **Since step 10c's Telegram worker, piece 6, the machine's
                // own path answers it** (I4, I5); a path with no actor keeps
                // the truthful answer below.
                if let Some(answered) = self.supervisor.answer_a_frame_request(&entry.operation, &subject, frame) {
                    let outcome = answered?;
                    return self
                        .supervisor
                        .record_the_outcome(&entry.operation, &outcome);
                }
                self.answer_about(
                    entry,
                    &labels,
                    a_frame_cannot_be_sent_yet(&labels.server, &labels.label, frame),
                );
                return self.supervisor.record_the_outcome(
                    &entry.operation,
                    &ARecordedOutcome::Failed("no actor answers a frame request yet".into()),
                );
            }
            crate::routing::context::SENDING_THE_NEWEST_FRAME => {
                let answered = match self.supervisor.the_newest_frame_held(&subject) {
                    Some(Some(frame)) => self.supervisor.answer_a_frame_request(&entry.operation, &subject, frame),
                    Some(None) => {
                        self.answer_about(entry, &labels, no_frame_has_finished(&labels.server, &labels.label));
                        Some(Ok(ARecordedOutcome::Done))
                    }
                    None => None,
                };
                if let Some(answered) = answered {
                    let outcome = answered?;
                    return self.supervisor.record_the_outcome(&entry.operation, &outcome);
                }
                self.answer_about(entry, &labels, a_user_message(&labels.server, &labels.label, "", "Not available on this server yet."));
                return self.supervisor.record_the_outcome(
                    &entry.operation,
                    &ARecordedOutcome::Failed("no actor answers a frame request yet".into()),
                );
            }
            // **B9a's manner, for an action** (2026-10-31): refused, never
            // acted on in part, and acknowledged.
            other => {
                self.note(&format!("{} names {other}, which this server does not know; refused", entry.operation));
                return self
                    .supervisor
                    .record_the_outcome(&entry.operation, &ARecordedOutcome::Failed(AN_ACTION_THIS_SERVER_DOES_NOT_KNOW.into()));
            }
        };
        if entry.action == STOP_THIS_JOB {
            if let Some(key) = &confirmation {
                self.supervisor.record_the_outcome(key, &outcome)?;
            }
        }
        self.note(&format!(
            "{} was submitted, and its submission recorded: {outcome:?}",
            entry.operation
        ));
        // **B9: a recorded failure is told to the user in plain words**, naming
        // the server and the job.
        if let ARecordedOutcome::Failed(why) = &outcome {
            if why == THE_JOB_OR_ATTEMPT_ENDED {
                self.say_it_has_already_ended(&labels)?;
                return Ok(());
            }
            let asked = if labels.button.is_empty() {
                the_words_of(&entry.action).to_string()
            } else {
                labels.button.clone()
            };
            self.answer_about(
                entry,
                &labels,
                the_act_was_not_done(&labels.server, &labels.label, &asked),
            );
        }
        Ok(())
    }
}

// ------------------------------------------------------------ D2's first half

/// **`Telegram` D2 — the listener says at once that the stop was sent to the
/// named machine for the named job.** The listener is the holder (only the
/// holder asks for updates), so this is said the moment its read-back accepts
/// the full stop instruction (B8), **whichever machine owns the job** — the
/// holder's own included, since the holder now acts only on an instruction a
/// read-back has accepted. Piece 7's ruling 9, *the first time its confirmed
/// board carries the full stop instruction*, is narrowed to that by the round-1
/// code audit of the bot and the chat, fix-here item 3. Kept in memory: a
/// hand-over may say it again, never not at all.
pub struct TheListenersAcknowledgement {
    pub conversation: i64,
    pub queue: Arc<TheSendQueue>,
    pub said: HashSet<String>,
}

/// **The stops among entries a holder has newly accepted that it has not yet
/// said were sent**, with the words for each.
pub fn the_stops_to_say(accepted: &[AnEntry], said: &HashSet<String>) -> Vec<(String, AMarkedText)> {
    accepted
        .iter()
        .filter(|e| {
            e.kind == AKindOfEntry::Action
                && e.action == STOP_THIS_JOB
                && e.payload["confirmed"] == json!(true)
                && !said.contains(&e.operation)
        })
        .map(|e| {
            let labels: TheLabels =
                serde_json::from_value(e.payload["labels"].clone()).unwrap_or_default();
            (
                e.operation.clone(),
                the_stop_was_sent(&labels.server, &labels.label),
            )
        })
        .collect()
}

impl TheListenersAcknowledgement {
    /// **C1b: the acknowledgement that a stop was sent carries no Stop.**
    pub fn before_each_turn(&mut self, listener: &TheListener) {
        let accepted = listener.the_instructions_accepted_lately();
        for (operation, text) in the_stops_to_say(&accepted, &self.said) {
            let ended = send_text(
                &self.queue,
                self.conversation,
                AClassOfSend::RoutedAnswersAndAcknowledgements,
                text,
            );
            if !matches!(
                ended,
                HowASendEnded::HandedBack(WhyTheCallFailed::NotReached(_))
            ) {
                self.said.insert(operation);
            }
        }
    }
}

// ------------------------------------------------------------ the stand-in

/// **A stand-in for the supervisor's command path, a machine's captured
/// streams**, for this crate's tests and a desk process.
/// It records every command, keeps each outcome beside its identity
/// (`Server protocol and transfers` C4) and `Rented-server runtime` U13's
/// first-recorded-wins, and journals each command's outcome for
/// [`TheOwnersActs::report`].
pub mod a_stand_in {
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

    use super::*;

    #[derive(Debug, Default)]
    pub struct WhatTheSupervisorHeard {
        /// The outcome recorded beside each instruction's identity.
        pub recorded: HashMap<String, ARecordedOutcome>,
        /// Every command submitted, in order, in words.
        pub commands: Vec<String>,
        pub stopped: HashMap<String, SystemTime>,
        pub released: HashMap<String, (TheAnswer, SystemTime)>,
        /// The submission identity, and the new job identity issued for it.
        pub submitted: HashMap<String, (String, SystemTime)>,
        pub words: HashMap<(String, AStream), Vec<u8>>,
        /// **The journal**: each stop's job and outcome, in order.
        pub stops: Vec<(String, AStopsOutcome)>,
        /// Each release's hold, the answer pressed, and its outcome, in order.
        pub releases: Vec<(String, TheAnswer, AReleasesOutcome)>,
        /// Each submission's failed job and outcome, in order.
        pub submissions: Vec<(String, ASubmissionsOutcome)>,
        /// **How many command submissions next reach nothing**, recording nothing.
        pub not_reached: usize,
        /// **Whether command submissions are refused**, recorded as failures.
        pub refuses: bool,
    }

    /// Clone it freely; every clone is the same supervisor.
    #[derive(Debug, Clone, Default)]
    pub struct AStandInSupervisor(pub Arc<Mutex<WhatTheSupervisorHeard>>);

    impl AStandInSupervisor {
        pub fn heard(&self) -> std::sync::MutexGuard<'_, WhatTheSupervisorHeard> {
            self.0.lock().unwrap_or_else(|p| p.into_inner())
        }
    }

    pub struct WithContexts {
        supervisor: AStandInSupervisor,
        store: Arc<Mutex<Box<dyn TheTelegramWorkersState>>>,
    }
    impl AStandInSupervisor {
        pub fn with_context_store(
            &self,
            store: Arc<Mutex<Box<dyn TheTelegramWorkersState>>>,
        ) -> WithContexts {
            WithContexts {
                supervisor: self.clone(),
                store,
            }
        }
    }
    impl TheSupervisorsCommandPath for WithContexts {
        fn the_action_context_store(&self) -> Option<Box<dyn TheTelegramWorkersState>> {
            Some(Box::new(self.store.clone()))
        }
        fn the_recorded_outcome(&mut self, id: &str) -> Result<Option<ARecordedOutcome>, String> {
            self.supervisor.the_recorded_outcome(id)
        }
        fn record_the_outcome(
            &mut self,
            id: &str,
            outcome: &ARecordedOutcome,
        ) -> Result<(), String> {
            self.supervisor.record_the_outcome(id, outcome)
        }
        fn submit_a_stop(&mut self, id: &str, job: &str) -> Result<ARecordedOutcome, String> {
            self.supervisor.submit_a_stop(id, job)
        }
        fn release_a_hold(
            &mut self,
            id: &str,
            hold: &str,
            answer: TheAnswer,
        ) -> Result<ARecordedOutcome, String> {
            self.supervisor.release_a_hold(id, hold, answer)
        }
        fn submit_again(
            &mut self,
            id: &str,
            submission: &str,
            job: &str,
        ) -> Result<ARecordedOutcome, String> {
            self.supervisor.submit_again(id, submission, job)
        }
    }

    /// **What happens before any command**: a command path that reaches
    /// nothing, a repeat finding its outcome, or a refusal recorded.
    fn before_a_command(heard: &mut WhatTheSupervisorHeard, operation: &str) -> Option<Result<ARecordedOutcome, String>> {
        if heard.not_reached > 0 {
            heard.not_reached -= 1;
            return Some(Err("the command path reached nothing".into()));
        }
        if let Some(recorded) = heard.recorded.get(operation) {
            return Some(Ok(recorded.clone()));
        }
        if heard.refuses {
            let refused = ARecordedOutcome::Failed("the stand-in refused the command".into());
            heard.recorded.insert(operation.to_string(), refused.clone());
            return Some(Ok(refused));
        }
        None
    }

    impl TheSupervisorsCommandPath for AStandInSupervisor {
        fn the_recorded_outcome(&mut self, operation: &str) -> Result<Option<ARecordedOutcome>, String> {
            Ok(self.heard().recorded.get(operation).cloned())
        }
        fn record_the_outcome(&mut self, operation: &str, outcome: &ARecordedOutcome) -> Result<(), String> {
            self.heard().recorded.insert(operation.to_string(), outcome.clone());
            Ok(())
        }
        fn submit_a_stop(&mut self, operation: &str, job: &str) -> Result<ARecordedOutcome, String> {
            let mut heard = self.heard();
            if let Some(ended) = before_a_command(&mut heard, operation) {
                return ended;
            }
            heard.commands.push(format!("stop {job} under {operation}"));
            let outcome = match heard.stopped.get(job) {
                Some(at) => AStopsOutcome::AlreadyStopped { at: *at },
                None => {
                    heard.stopped.insert(job.to_string(), SystemTime::now());
                    AStopsOutcome::Stopped { frames_on_the_server: 12, frames_not_rendered: 28 }
                }
            };
            heard.stops.push((job.to_string(), outcome));
            heard.recorded.insert(operation.to_string(), ARecordedOutcome::Done);
            Ok(ARecordedOutcome::Done)
        }
        fn release_a_hold(&mut self, operation: &str, hold: &str, answer: TheAnswer) -> Result<ARecordedOutcome, String> {
            let mut heard = self.heard();
            if let Some(ended) = before_a_command(&mut heard, operation) {
                return ended;
            }
            heard.commands.push(format!("release {hold} with {answer:?} under {operation}"));
            let outcome = match heard.released.get(hold) {
                Some((applied, at)) => AReleasesOutcome::AnotherAnswerWasApplied { answer: *applied, at: *at },
                None => {
                    heard.released.insert(hold.to_string(), (answer, SystemTime::now()));
                    AReleasesOutcome::Applied
                }
            };
            heard.releases.push((hold.to_string(), answer, outcome));
            heard.recorded.insert(operation.to_string(), ARecordedOutcome::Done);
            Ok(ARecordedOutcome::Done)
        }
        fn submit_again(&mut self, operation: &str, submission: &str, failed_job: &str) -> Result<ARecordedOutcome, String> {
            let mut heard = self.heard();
            if let Some(ended) = before_a_command(&mut heard, operation) {
                return ended;
            }
            let outcome = match heard.submitted.get(submission).map(|(_, at)| *at) {
                Some(at) => {
                    heard.commands.push(format!("submit {failed_job} again under {submission}: already recorded"));
                    ASubmissionsOutcome::AlreadySubmitted { at }
                }
                None => {
                    // `State, identity and recovery` Y7a: the new job's identity, issued before the
                    // submission is composed, by the same draw the desktop uses.
                    let new_job = Identity::issue(IdentityKind::Job).map_err(|e| format!("{e:?}"))?.as_text();
                    heard.commands.push(format!("submit {failed_job} again as {new_job} under {submission}"));
                    heard.submitted.insert(submission.to_string(), (new_job, SystemTime::now()));
                    ASubmissionsOutcome::Submitted
                }
            };
            heard.submissions.push((failed_job.to_string(), outcome));
            heard.recorded.insert(operation.to_string(), ARecordedOutcome::Done);
            Ok(ARecordedOutcome::Done)
        }
    }

    impl ReadsTheWordsBack for AStandInSupervisor {
        fn read_whole(&self, job: &str, _attempt: u32, stream: AStream) -> Result<Vec<u8>, String> {
            self.heard().words.get(&(job.to_string(), stream)).cloned().ok_or_else(|| "no such record".to_string())
        }
    }

}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;
    use std::time::Duration;

    use super::a_stand_in::AStandInSupervisor;
    use super::*;
    use crate::board::instructions::TheAddressee;
    use crate::job_messages::{
        the_failure_message, the_waiting_message, ACollectionOutcome, AFailedJob, AWaitingJob, TheCause, WhatTheHoldWaitsFor,
        THE_CAP_ON_A_FILE,
    };
    use crate::pacing::Waits;
    use crate::routing::context::{a_frame_request, a_taps_resolution, resolve_a_tap, ATapsEvidence};
    use crate::routing::gather::AJobAsHeld;
    use crate::routing::token::WhatATokenNames;
    use crate::transport::a_stand_in::AStandInService;
    use crate::transport::WhatACallCarries;
    use crate::worker_state::InMemory;

    const KEY: &str = "00112233445566778899aabbccddeeff";
    const CHAT: i64 = 9001;
    const ME: &str = "machine-1";

    struct NoWait;
    impl Waits for NoWait {
        fn wait(&self, _: Duration) {}
    }

    /// The jobs and holds a test says are live, at generation 1.
    struct Live(Vec<&'static str>);
    impl ReadsWhatThisMachineHolds for Live {
        fn the_jobs(&self) -> Vec<AJobAsHeld> {
            Vec::new()
        }
        fn the_live_generation(&self, subject: &str) -> Option<String> {
            self.0.contains(&subject).then(|| "1".to_string())
        }
    }

    fn quiet() -> Notes {
        Box::new(|_| {})
    }

    fn a_queue(service: &Arc<AStandInService>) -> Arc<TheSendQueue> {
        Arc::new(TheSendQueue::new(TheClient::over(service.clone()), Arc::new(NoWait)))
    }

    fn fields_of(carries: &WhatACallCarries) -> Value {
        match carries {
            WhatACallCarries::Fields(f) => Value::Object(f.clone()),
            WhatACallCarries::FieldsAndFiles { fields, .. } => Value::Object(fields.clone()),
            WhatACallCarries::Nothing => json!({}),
        }
    }

    fn a_failure(cause: TheCause) -> AFailedJob {
        AFailedJob {
            server: "Desk".into(),
            job: "job-1".into(),
            label: "Kitchen".into(),
            generation: "1".into(),
            attempt: 1,
            at_frame: Some((13, 40)),
            exit_status: Some(1),
            frames_on_the_server: 12,
            ordinary_output: ACollectionOutcome::Collected,
            complaints: ACollectionOutcome::CollectedAndTheStreamWasEmpty,
            cause,
            disk_free_at_the_ending: None,
            no_render_began: false,
            the_reason_the_machine_recorded: None,
        }
    }

    type Shared = Arc<Mutex<Box<dyn TheTelegramWorkersState>>>;

    fn issuer(service: &Arc<AStandInService>) -> (TheJobMessages, Shared) {
        let store: Shared = Arc::new(Mutex::new(Box::new(InMemory::with_routing_key(KEY))));
        (TheJobMessages { conversation: CHAT, store: Box::new(store.clone()), queue: a_queue(service), notes: quiet() }, store)
    }

    fn job_one() -> TheSubject {
        TheSubject { owning_machine: ME.into(), job: Some("job-1".into()), hold: None, generation: "1".into() }
    }

    #[test]
    fn rented_pass_stop_removal_keeps_other_buttons_retries_and_survives_reassembly() {
        let service = Arc::new(AStandInService::new());
        service.then_answers(r#"{"ok":true,"result":{"message_id":77}}"#);
        let (mut messages, store) = issuer(&service);
        let first = messages.send_about_with_buttons(
            &the_failure_message(&a_failure(TheCause::TheMachinesOwn)),
            &job_one(), "Desk", "Kitchen",
        ).unwrap();
        let second = first + 10;
        messages.attach_a_stop(second, &job_one(), "Desk", "Kitchen", AClassOfSend::EndingsHoldsAndDrops).unwrap();
        let original = store.the_action_contexts_bound_to(first).unwrap();
        assert_eq!(messages.remove_the_stops_where(&|_| false).unwrap(), 0, "live jobs keep Stop");
        service.then_is_not_reached("line cut");
        assert!(messages.remove_the_stops_where(&|_| true).is_err());
        assert!(store.the_action_contexts_bound_to(first).unwrap().iter()
            .all(|r| r.payload["stop_removed"] != json!(true)), "a failed edit is still owed");
        // A new issuer over the same durable records retries both old messages.
        let mut restarted = TheJobMessages {
            conversation: CHAT, store: Box::new(store.clone()), queue: a_queue(&service), notes: quiet(),
        };
        assert_eq!(restarted.remove_the_stops_where(&|_| true).unwrap(), 2);
        let edits: Vec<_> = service.heard().iter().filter(|c| c.method == "editMessageReplyMarkup")
            .map(|c| fields_of(&c.carries)).collect();
        let remaining = &edits.iter().rev().find(|f| f["message_id"] == json!(first)).unwrap()["reply_markup"]["inline_keyboard"];
        let labels: Vec<_> = remaining.as_array().unwrap().iter().map(|row| row[0]["text"].as_str().unwrap()).collect();
        assert_eq!(labels, ["Send Output log", "Retry"]);
        assert!(edits.iter().rev().find(|f| f["message_id"] == json!(second)).unwrap()["reply_markup"]["inline_keyboard"].as_array().unwrap().is_empty());
        let after = store.the_action_contexts_bound_to(first).unwrap();
        assert_eq!(after.len(), original.len(), "delayed taps retain every context");
        assert!(original.iter().all(|old| after.iter().any(|r| r.token == old.token)));
        let calls = service.heard().len();
        assert_eq!(restarted.remove_the_stops_where(&|_| true).unwrap(), 0);
        assert_eq!(service.heard().len(), calls, "successful edits are not repeated");
    }

    #[test]
    fn rented_pass_replayed_stop_removal_accepts_an_unchanged_or_deleted_message() {
        for description in ["Bad Request: message is not modified", "Bad Request: message to edit not found"] {
            let service = Arc::new(AStandInService::new());
            let (mut messages, _) = issuer(&service);
            messages.a_stop_button(90, &job_one(), "Desk", "Kitchen").unwrap();
            service.then_answers(&json!({"ok": false, "error_code": 400, "description": description}).to_string());
            assert_eq!(messages.remove_the_stops_where(&|_| true).unwrap(), 1);
            assert_eq!(messages.remove_the_stops_where(&|_| true).unwrap(), 0);
        }
    }

    #[test]
    fn rented_pass_an_ended_reply_that_reaches_nothing_is_not_recorded_as_done() {
        let service = Arc::new(AStandInService::new());
        let (mut messages, store) = issuer(&service);
        messages.a_stop_button(90, &job_one(), "Desk", "Kitchen").unwrap();
        let mut supervisor = AStandInSupervisor::default();
        let mut acts = owner(&service, &supervisor, vec![]);
        let entry = press(&store, 90, STOP_THIS_JOB, "late");
        service.then_is_not_reached("line cut");
        assert!(acts.act_once(&entry).is_err(), "the reply must be retried");
        assert!(supervisor.heard().commands.is_empty());
        assert!(supervisor.the_recorded_outcome(&entry.operation).unwrap().is_none());
        acts.act_once(&entry).unwrap();
        assert!(supervisor.the_recorded_outcome(&entry.operation).unwrap().is_some());
    }

    /// **R1 and E3 — the message first, then the records, then the buttons**;
    /// **B24 and B25 — for the user: it notifies, and carries no spoiler.**
    #[test]
    fn a_failure_message_is_sent_then_recorded_then_given_its_buttons() {
        let service = Arc::new(AStandInService::new());
        service.then_answers(r#"{"ok":true,"result":{"message_id":77}}"#);
        let (mut messages, store) = issuer(&service);
        let message = messages.send_with_buttons(&the_failure_message(&a_failure(TheCause::TheMachinesOwn)), &job_one()).unwrap();
        assert_eq!(message, 77);

        let heard = service.heard();
        assert_eq!(heard.iter().map(|c| c.method.as_str()).collect::<Vec<_>>(), ["sendMessage", "editMessageText"]);
        let sent = fields_of(&heard[0].carries);
        assert!(sent.get("reply_markup").is_none(), "no button before its record");
        for call in heard.iter() {
            let fields = fields_of(&call.carries);
            assert!(fields.get("disable_notification").is_none() && fields["entities"].as_array().unwrap().iter().all(|e| e["type"] != "spoiler") && fields.get("parse_mode").is_none());
        }
        let rows = fields_of(&heard[1].carries)["reply_markup"]["inline_keyboard"].clone();
        let tokens: Vec<String> = rows.as_array().unwrap().iter().map(|r| r[0]["callback_data"].as_str().unwrap().to_string()).collect();
        assert_eq!(tokens.len(), 2);
        let records = store.the_action_contexts_bound_to(77).unwrap();
        assert_eq!(records.len(), 2);
        for token in &tokens {
            assert_eq!(token.len(), 44, "R1: 44 bytes, inside Telegram's 64");
            let record = records.iter().find(|r| &r.token == token).expect("every button has its record");
            assert_eq!((record.owning_machine.as_str(), record.job.as_deref(), record.generation.as_str()), (ME, Some("job-1"), "1"));
        }
        let try_again = records.iter().find(|r| r.action == TRY_IT_AGAIN).unwrap();
        assert!(try_again.payload["submission"].is_string());
        // R7 and fix-here item 13: the button's own label rides its record.
        assert_eq!(try_again.payload["labels"]["button"], json!("Retry"));
    }

    /// **R1: a record that cannot be written means no button at all.**
    #[test]
    fn no_button_is_added_without_its_record() {
        struct Refuses;
        impl TheTelegramWorkersState for Refuses {
            fn the_update_position(&self) -> Result<Option<i64>, crate::worker_state::WhyTheStoreFailed> { Ok(None) }
            fn record_the_update_position(&mut self, _: i64) -> Result<(), crate::worker_state::WhyTheStoreFailed> { Ok(()) }
            fn the_board(&self) -> Result<Option<i64>, crate::worker_state::WhyTheStoreFailed> { Ok(None) }
            fn record_the_board(&mut self, _: i64) -> Result<(), crate::worker_state::WhyTheStoreFailed> { Ok(()) }
            fn the_machinery_message(&self) -> Result<Option<i64>, crate::worker_state::WhyTheStoreFailed> { Ok(None) }
            fn record_the_machinery_message(&mut self, _: i64) -> Result<(), crate::worker_state::WhyTheStoreFailed> { Ok(()) }
            fn the_routing_key(&self) -> Result<String, crate::worker_state::WhyTheStoreFailed> { Ok(KEY.into()) }
            fn holds_a_context_record_bound_to(&self, _: i64) -> Result<bool, crate::worker_state::WhyTheStoreFailed> { Ok(false) }
            fn record_an_action_context(&mut self, _: AnActionContextRecord) -> Result<(), crate::worker_state::WhyTheStoreFailed> {
                Err(crate::worker_state::WhyTheStoreFailed("refused".into()))
            }
            fn the_action_context(&self, _: &str) -> Result<Option<AnActionContextRecord>, crate::worker_state::WhyTheStoreFailed> { Ok(None) }
            fn the_action_contexts(&self) -> Result<Vec<AnActionContextRecord>, crate::worker_state::WhyTheStoreFailed> { Ok(Vec::new()) }
            fn the_action_contexts_bound_to(&self, _: i64) -> Result<Vec<AnActionContextRecord>, crate::worker_state::WhyTheStoreFailed> { Ok(Vec::new()) }
            fn remove_action_contexts_where(&mut self, _: &dyn Fn(&AnActionContextRecord) -> bool) -> Result<usize, crate::worker_state::WhyTheStoreFailed> { Ok(0) }
            fn the_journal_cursor(&self) -> Result<Option<u64>, crate::worker_state::WhyTheStoreFailed> { Ok(None) }
            fn record_the_journal_cursor(&mut self, _: u64) -> Result<(), crate::worker_state::WhyTheStoreFailed> { Ok(()) }
            fn the_status_message(&self) -> Result<Option<i64>, crate::worker_state::WhyTheStoreFailed> { Ok(None) }
            fn record_the_status_message(&mut self, _: i64) -> Result<(), crate::worker_state::WhyTheStoreFailed> { Ok(()) }
        }
        let service = Arc::new(AStandInService::new());
        service.then_answers(r#"{"ok":true,"result":{"message_id":5}}"#);
        let mut messages = TheJobMessages { conversation: CHAT, store: Box::new(Refuses), queue: a_queue(&service), notes: quiet() };
        let sent = messages.send_with_buttons(&the_failure_message(&a_failure(TheCause::TheFilesOwn)), &job_one());
        assert_eq!(sent, Err(WhyAMessageWasNotSent::SentWithoutItsButtons { message: 5 }));
        assert_eq!(service.heard().len(), 1, "the edit adding buttons was never made");
    }

    fn owner(service: &Arc<AStandInService>, supervisor: &AStandInSupervisor, live: Vec<&'static str>) -> TheOwnersActs {
        TheOwnersActs {
            conversation: CHAT,
            supervisor: Box::new(supervisor.clone()),
            words: Box::new(supervisor.clone()),
            holds: Box::new(Live(live)),
            queue: a_queue(service),
            notes: quiet(),
        }
    }

    /// **The whole path at the desk, without the board's round trip that piece
    /// 3 already watched**: a button's record, a press resolved on the issuer
    /// by R1, and the owner's B9.
    fn press(store: &Shared, message: i64, action: &str, callback: &str) -> AnEntry {
        let record = store
            .the_action_contexts_bound_to(message)
            .unwrap()
            .into_iter()
            .find(|r| {
                r.action == action || (action == STOP_THIS_JOB && r.action == ASK_TO_STOP_THIS_JOB)
            })
            .unwrap();
        let evidence = ATapsEvidence {
            token: record.token.clone(),
            callback: callback.into(),
            conversation: CHAT,
            source_message: message,
            label: None,
        };
        let names = WhatATokenNames::read(&record.token).unwrap();
        let entry = a_taps_resolution(&evidence, 1, &names);
        let full = resolve_a_tap(&entry, ME, KEY, store, &|s| (s == "job-1" || s == "hold-1").then(|| "1".to_string())).unwrap();
        assert_eq!(full.addressee, TheAddressee::Machine(ME.into()));
        full
    }

    /// Fixture for command-stage tests. The full two-tap path is exercised
    /// separately; this fixture is already an authoritative confirmation.
    fn confirmed_press(store: &Shared, message: i64, callback: &str) -> AnEntry {
        let mut entry = press(store, message, STOP_THIS_JOB, callback);
        entry.payload["confirmed"] = json!(true);
        entry.action = STOP_THIS_JOB.into();
        entry
    }

    fn act_once(owner: &mut TheOwnersActs, entry: &AnEntry) {
        owner.act_once(entry).unwrap();
    }

    fn texts_sent(service: &AStandInService) -> Vec<String> {
        service
            .heard()
            .iter()
            .filter(|c| c.method == "sendMessage")
            .filter_map(|c| fields_of(&c.carries)["text"].as_str().map(str::to_string))
            .collect()
    }

    #[test]
    fn twenty_first_only_the_stop_button_gets_the_danger_style() {
        let service = Arc::new(AStandInService::new());
        service.then_answers(r#"{"ok":true,"result":{"message_id":77}}"#);
        let (mut messages, _) = issuer(&service);
        messages.send_with_buttons(&the_stop_question("Desk", "Kitchen", false), &job_one()).unwrap();
        let calls = service.heard();
        let rows = &fields_of(&calls[1].carries)["reply_markup"]["inline_keyboard"];
        assert_eq!(rows[0][0]["text"], "Keep rendering");
        assert!(rows[0][0].get("style").is_none());
        assert_eq!(rows[1][0]["text"], "Stop this job");
        assert_eq!(rows[1][0]["style"], "danger");
        assert!(rows.as_array().unwrap().iter().all(|r| r[0].get("entities").is_none()));
    }

    /// **Y1, R4 and fix-here item 4 — *try it again* submits the same job once,
    /// with a new identity issued before it; the act says nothing itself, and
    /// each journalled outcome is reported; a second press finds the recorded
    /// submission.**
    #[test]
    fn try_it_again_submits_once_and_a_second_press_is_told() {
        let service = Arc::new(AStandInService::new());
        service.then_answers(r#"{"ok":true,"result":{"message_id":77}}"#);
        let (mut messages, store) = issuer(&service);
        messages.send_with_buttons(&the_failure_message(&a_failure(TheCause::TheMachinesOwn)), &job_one()).unwrap();

        let supervisor = AStandInSupervisor::default();
        let answers = Arc::new(AStandInService::new());
        let mut acts = owner(&answers, &supervisor, vec!["job-1"]);
        let first = press(&store, 77, TRY_IT_AGAIN, "cb-1");
        act_once(&mut acts, &first);
        act_once(&mut acts, &first); // B9: the same instruction delivered again.
        act_once(&mut acts, &press(&store, 77, TRY_IT_AGAIN, "cb-2"));
        assert!(texts_sent(&answers).is_empty(), "the act returns at the submission, and says nothing itself");

        let journal = supervisor.heard().submissions.clone();
        {
            let heard = supervisor.heard();
            assert_eq!(heard.submitted.len(), 1, "one submission");
            assert_eq!(heard.commands.len(), 2);
            assert!(heard.commands[0].starts_with("submit job-1 again as ") && heard.commands[1].ends_with("already recorded"));
        }
        for (_, outcome) in journal {
            acts.report(AJournalledOutcome::ASubmission { server: "Desk".into(), label: "Kitchen".into(), outcome });
        }
        let texts = texts_sent(&answers);
        assert_eq!(texts.len(), 2);
        assert_eq!(texts[0], "Desk\nKitchen\nResubmitted.");
        assert!(texts[1].starts_with("Desk\nKitchen\nAlready resubmitted at "));
    }

    /// **A3, held-job piece B: the release of a hold for the driver update is
    /// told nothing** — not *was answered*, not *was already answered* — and
    /// no send is offered. A release of a hold awaiting the user is still told.
    #[test]
    fn the_release_of_a_hold_for_the_driver_update_is_told_nothing() {
        let supervisor = AStandInSupervisor::default();
        let answers = Arc::new(AStandInService::new());
        let mut acts = owner(&answers, &supervisor, vec!["hold-7"]);
        for outcome in [
            AReleasesOutcome::Applied,
            AReleasesOutcome::AnotherAnswerWasApplied { answer: TheAnswer::Dont, at: std::time::SystemTime::UNIX_EPOCH },
        ] {
            let told = acts.report(AJournalledOutcome::ARelease {
                server: "Desk".into(),
                label: "Kitchen".into(),
                answer: TheAnswer::GoOn,
                frames_held: 240,
                waits_for: WhatTheHoldWaitsFor::TheDriverUpdate,
                outcome,
            });
            assert!(told.is_none(), "no send is offered for the driver's release");
        }
        assert!(texts_sent(&answers).is_empty(), "{:?}", texts_sent(&answers));
        answers.then_answers(r#"{"ok":true,"result":{"message_id":5}}"#);
        assert!(acts
            .report(AJournalledOutcome::ARelease {
                server: "Desk".into(),
                label: "Kitchen".into(),
                answer: TheAnswer::GoOn,
                frames_held: 240,
                waits_for: WhatTheHoldWaitsFor::TheUsersAnswer,
                outcome: AReleasesOutcome::Applied,
            })
            .is_some());
        assert_eq!(texts_sent(&answers), ["Desk\nKitchen\nAnswered “Render all — 240 frames”."]);
    }

    /// **A2, A3 and fix-here item 10 — each button becomes one release command;
    /// the first recorded wins, the winner is told which answer was applied,
    /// and the loser is told which answer was applied and when.**
    #[test]
    fn the_waiting_jobs_two_buttons_are_two_release_commands_and_both_are_told() {
        let service = Arc::new(AStandInService::new());
        service.then_answers(r#"{"ok":true,"result":{"message_id":80}}"#);
        let (mut messages, store) = issuer(&service);
        let waiting = AWaitingJob {
            server: "Desk".into(),
            label: "Kitchen".into(),
            hold: "hold-1".into(),
            generation: "1".into(),
            waits_for: WhatTheHoldWaitsFor::TheUsersAnswer,
            the_frame_shown: 1,
            frames_held: 39,
            the_frames: String::new(),
        };
        let subject = TheSubject { owning_machine: ME.into(), job: None, hold: Some("hold-1".into()), generation: "1".into() };
        messages.send_with_buttons(&the_waiting_message(&waiting).unwrap(), &subject).unwrap();

        let supervisor = AStandInSupervisor::default();
        let answers = Arc::new(AStandInService::new());
        let mut acts = owner(&answers, &supervisor, vec!["hold-1"]);
        // Both buttons carry the same action, so each is pressed by its answer.
        let records = store.the_action_contexts_bound_to(80).unwrap();
        for (answer, callback) in [("GoOn", "cb-go"), ("Dont", "cb-dont")] {
            let record = records.iter().find(|r| r.payload["answer"] == json!(answer)).unwrap();
            let evidence = ATapsEvidence { token: record.token.clone(), callback: callback.into(), conversation: CHAT, source_message: 80, label: None };
            let entry = a_taps_resolution(&evidence, 2, &WhatATokenNames::read(&record.token).unwrap());
            act_once(&mut acts, &resolve_a_tap(&entry, ME, KEY, &store, &|_| Some("1".into())).unwrap());
        }

        let journal = {
            let heard = supervisor.heard();
            assert_eq!(heard.commands.len(), 2, "one release command per press");
            assert!(heard.commands[0].starts_with("release hold-1 with GoOn") && heard.commands[1].starts_with("release hold-1 with Dont"));
            assert_eq!(heard.released["hold-1"].0, TheAnswer::GoOn);
            heard.releases.clone()
        };
        assert!(texts_sent(&answers).is_empty());
        for (_, answer, outcome) in journal {
            acts.report(AJournalledOutcome::ARelease {
                server: "Desk".into(),
                label: "Kitchen".into(),
                answer,
                frames_held: 39,
                waits_for: WhatTheHoldWaitsFor::TheUsersAnswer,
                outcome,
            });
        }
        let texts = texts_sent(&answers);
        assert_eq!(texts.len(), 2, "the winner and the loser are both told");
        assert_eq!(texts[0], "Desk\nKitchen\nAnswered “Render all — 39 frames”.");
        assert!(texts[1].starts_with("Desk\nKitchen\nAlready answered \u{201c}Render all \u{2014} 39 frames\u{201d} at "));
    }

    /// **B9, U6 and fix-here item 2 — an act whose command path reaches nothing
    /// records nothing and is tried again; a command refused is recorded as a
    /// failure and told once, naming the server, the button and the job.**
    #[test]
    fn a_failed_act_is_tried_again_or_its_recorded_failure_is_told_once() {
        let service = Arc::new(AStandInService::new());
        let (mut messages, store) = issuer(&service);
        messages.a_stop_button(90, &job_one(), "Desk", "Kitchen").unwrap();
        let supervisor = AStandInSupervisor::default();
        let answers = Arc::new(AStandInService::new());
        let mut acts = owner(&answers, &supervisor, vec!["job-1"]);

        supervisor.heard().not_reached = 1;
        let first = confirmed_press(&store, 90, "cb-1");
        assert!(
            acts.act_once(&first).is_err(),
            "nothing recorded, so not acknowledged"
        );
        assert!(supervisor.heard().recorded.is_empty());
        acts.act_once(&first).unwrap();
        assert_eq!(supervisor.heard().commands.len(), 1, "tried again, and submitted");

        supervisor.heard().refuses = true;
        let second = confirmed_press(&store, 90, "cb-2");
        acts.act_once(&second).unwrap();
        acts.act_once(&second).unwrap();
        assert_eq!(texts_sent(&answers), ["Desk\nKitchen\n“Stop this job” was rejected by this server."]);
    }

    /// **R14 until step 10c — a routed frame request is answered truthfully and
    /// its outcome recorded, never returned as an error.**
    #[test]
    fn a_routed_frame_request_is_answered_truthfully() {
        let supervisor = AStandInSupervisor::default();
        let answers = Arc::new(AStandInService::new());
        let mut acts = owner(&answers, &supervisor, vec!["job-1"]);
        let request = a_frame_request("frame-5", ME, "job-1", "1", Some(120), Some(5), json!({ "server": "Desk", "label": "Kitchen" }));
        acts.act_once(&request).unwrap();
        acts.act_once(&request).unwrap();
        assert_eq!(texts_sent(&answers), ["Desk\nKitchen · Frame 120\nNot available on this server yet."]);
        assert!(matches!(supervisor.heard().recorded.get("frame-5"), Some(ARecordedOutcome::Failed(_))));
    }

    /// **`Telegram` B9a's manner for an action (2026-10-31): an action this
    /// server does not know is recorded as refused and acknowledged**, and
    /// nothing is done. It was left unrecorded and tried again for ever.
    #[test]
    fn telegram_discussion_an_action_this_server_does_not_know_is_refused_and_acknowledged() {
        let supervisor = AStandInSupervisor::default();
        let answers = Arc::new(AStandInService::new());
        let mut acts = owner(&answers, &supervisor, vec!["job-1"]);
        let mut unknown = a_frame_request("later-1", ME, "job-1", "1", Some(1), Some(5), json!({}));
        unknown.action = "an action of a later release".into();
        assert_eq!(acts.act_once(&unknown), Ok(()), "acknowledged");
        assert_eq!(
            supervisor.heard().recorded.get("later-1"),
            Some(&ARecordedOutcome::Failed(AN_ACTION_THIS_SERVER_DOES_NOT_KNOW.into()))
        );
        assert!(supervisor.heard().commands.is_empty() && answers.heard().is_empty(), "nothing done, nothing said");
    }

    /// **The desk's stand-in path has no actor for the discussion's acts**:
    /// the newest frame says it cannot be sent here, and the phone's choice
    /// and one job's change are recorded as not done — never an error that
    /// would leave them standing.
    #[test]
    fn telegram_discussion_a_path_with_no_actor_records_the_new_acts_as_not_done() {
        let supervisor = AStandInSupervisor::default();
        let answers = Arc::new(AStandInService::new());
        let mut acts = owner(&answers, &supervisor, vec!["job-1"]);
        let newest = a_frame_request("newest-1", ME, "job-1", "1", None, Some(5), json!({ "server": "Desk", "label": "Kitchen" }));
        assert_eq!(newest.action, crate::routing::context::SENDING_THE_NEWEST_FRAME);
        acts.act_once(&newest).unwrap();
        assert_eq!(texts_sent(&answers), ["Desk\nKitchen\nNot available on this server yet."]);
        let mut every = crate::routing::context::an_action_on_one_job("every-1", ME, "", crate::routing::context::EVERY_LIVE_JOBS_REPORTING, json!({ "value": "Nothing", "operation": "op", "date": 1, "update": 2 }), Some(5));
        every.subjects.clear();
        acts.act_once(&every).unwrap();
        assert!(matches!(supervisor.heard().recorded.get("every-1"), Some(ARecordedOutcome::Failed(_))));
        let one = crate::routing::context::an_action_on_one_job("one-1", ME, "job-1", crate::routing::context::ONE_JOBS_REPORTING, json!({ "job": "job-1", "value": "Everything", "labels": { "server": "Desk", "label": "Kitchen" } }), Some(5));
        acts.act_once(&one).unwrap();
        assert!(matches!(supervisor.heard().recorded.get("one-1"), Some(ARecordedOutcome::Failed(_))));
    }

    /// **E3 — nothing goes unasked, and words that cannot be read back whole
    /// send nothing but the answer saying so.**
    #[test]
    fn the_words_go_only_on_a_tap_and_unreadable_words_send_nothing() {
        let service = Arc::new(AStandInService::new());
        service.then_answers(r#"{"ok":true,"result":{"message_id":77}}"#);
        let (mut messages, store) = issuer(&service);
        messages.send_with_buttons(&the_failure_message(&a_failure(TheCause::TheFilesOwn)), &job_one()).unwrap();
        assert!(service.heard().iter().all(|c| !matches!(c.carries, WhatACallCarries::FieldsAndFiles { .. })), "E3: nothing unasked");

        let supervisor = AStandInSupervisor::default();
        let answers = Arc::new(AStandInService::new());
        let mut acts = owner(&answers, &supervisor, vec!["job-1"]);
        act_once(&mut acts, &press(&store, 77, SEND_THE_WORDS, "cb-1"));
        let heard = answers.heard();
        assert_eq!(heard.len(), 1);
        assert!(fields_of(&heard[0].carries)["text"].as_str().unwrap().contains("Desk\nKitchen\nCould not read the Output log. Nothing was sent."));
        assert!(fields_of(&heard[0].carries)["entities"].as_array().unwrap().iter().all(|e| e["type"] != "pre"));
    }

    /// A3a: one picture still precedes a separate question with its answers.
    #[test]
    fn piece17_the_waiting_question_follows_the_picture_as_a_separate_message() {
        let service = Arc::new(AStandInService::new());
        service.then_answers(r#"{"ok":true,"result":{"message_id":80}}"#);
        service.then_answers(r#"{"ok":true,"result":{"message_id":81}}"#);
        let (mut messages, store) = issuer(&service);
        let waiting = AWaitingJob {
            server: "Desk".into(),
            label: "Kitchen".into(),
            hold: "hold-1".into(),
            generation: "1".into(),
            waits_for: WhatTheHoldWaitsFor::TheUsersAnswer,
            the_frame_shown: 1,
            frames_held: 39,
            the_frames: String::new(),
        };
        let picture = ADerivative {
            job: "job-1".into(),
            job_label: "Kitchen".into(),
            frame: 1,
            file_name: "frame1.png".into(),
            bytes: crate::pictures::tests::a_png_header(1280, 720),
            which_picture: None,
        };
        let subject = TheSubject { owning_machine: ME.into(), job: None, hold: Some("hold-1".into()), generation: "1".into() };
        assert_eq!(messages.send_the_waiting_message(&waiting, Some(&picture), &subject), Ok(Some(81)));

        let heard = service.heard();
        assert_eq!(
            heard.iter().map(|c| c.method.as_str()).collect::<Vec<_>>(),
            ["sendPhoto", "sendMessage", "editMessageText"]
        );
        let sent = fields_of(&heard[0].carries);
        assert!(
            sent.get("reply_markup").is_none(),
            "no button before its record"
        );
        assert!(sent["caption"].as_str().unwrap().contains("Frame 1"));
        assert_eq!(
            fields_of(&heard[1].carries)["text"],
            json!(the_waiting_message(&waiting).unwrap().text.text)
        );
        let attached = fields_of(&heard[2].carries);
        assert_eq!(attached["message_id"], json!(81));
        let tokens: Vec<String> = attached["reply_markup"]["inline_keyboard"]
            .as_array()
            .unwrap()
            .iter()
            .map(|r| r[0]["callback_data"].as_str().unwrap().to_string())
            .collect();
        let records = store.the_action_contexts_bound_to(81).unwrap();
        assert_eq!(records.len(), 2);
        assert!(tokens.iter().all(|t| records.iter().any(|r| &r.token == t)), "every button attached has its record");
    }

    /// **D1, R1 and fix-here item 6 — *Stop this job* attached to a picture
    /// already sent**, its record bound to that picture, so a tap on it resolves.
    #[test]
    fn a_stop_is_attached_to_a_picture_already_sent_and_resolves() {
        let service = Arc::new(AStandInService::new());
        let (mut messages, store) = issuer(&service);
        messages.attach_a_stop(90, &job_one(), "Desk", "Kitchen", AClassOfSend::FramePicturesAndMedia).unwrap();
        let heard = service.heard();
        assert_eq!(heard.iter().map(|c| c.method.as_str()).collect::<Vec<_>>(), ["editMessageReplyMarkup"]);
        let attached = fields_of(&heard[0].carries);
        assert_eq!(attached["message_id"], json!(90));
        assert_eq!(
            attached["reply_markup"]["inline_keyboard"][0][0]["text"],
            json!("Stop this job")
        );
        let full = press(&store, 90, STOP_THIS_JOB, "cb-1");
        assert_eq!(full.action, ASK_TO_STOP_THIS_JOB);
    }

    /// **A tap on a stop reaches the owner's stop**: the press resolved on
    /// the issuer, and the owner submitting the stop for the job.
    fn a_tap_reaches_the_owners_stop(store: &Shared, message: i64) {
        let supervisor = AStandInSupervisor::default();
        let answers = Arc::new(AStandInService::new());
        let mut acts = owner(&answers, &supervisor, vec!["job-1"]);
        act_once(&mut acts, &confirmed_press(store, message, "cb-stop"));
        let commands = supervisor.heard().commands.clone();
        assert_eq!(commands.len(), 1, "the stop was submitted once");
        assert!(commands[0].starts_with("stop job-1 under "), "{commands:?}");
    }

    /// **D1, R1 and fix-here item 6, finished — the status message carries a
    /// working *Stop this job*, attached by the clock itself**: created with no
    /// button, the record written against the new message, then the button on
    /// every edit, under one record.
    #[test]
    fn the_status_message_carries_a_stop_that_reaches_the_owner() {
        let service = Arc::new(AStandInService::new());
        service.then_answers(r#"{"ok":true,"result":{"message_id":31}}"#);
        let (messages, store) = issuer(&service);
        let mut the_clocks_store = store.clone();
        let mut clock = crate::status::TheClock::for_the_conversation(CHAT);
        let status = crate::status::tests::rendering();
        for _ in 0..3 {
            clock.a_tick(&mut the_clocks_store, &messages.queue, &status, Some(&job_one()), SystemTime::now()).unwrap();
            messages.queue.send_the_next();
        }
        clock.take_in_what_ended(&mut the_clocks_store).unwrap();

        let heard = service.heard();
        assert_eq!(heard.iter().map(|c| c.method.as_str()).collect::<Vec<_>>(), ["sendMessage", "editMessageText", "editMessageText"]);
        assert!(fields_of(&heard[0].carries).get("reply_markup").is_none(), "no button before its record");
        let records = store.the_action_contexts_bound_to(31).unwrap();
        assert_eq!(records.len(), 1, "one record for the message and its job, however many edits");
        assert_eq!(records[0].payload["labels"], json!({ "server": "RunPod 4090", "label": "Kitchen \u{2014} final", "button": "Stop this job" }));
        for edit in &heard[1..] {
            let fields = fields_of(&edit.carries);
            assert_eq!(fields["message_id"], json!(31));
            assert_eq!(fields["reply_markup"]["inline_keyboard"][0][0]["text"], json!("Stop this job"));
            assert_eq!(fields["reply_markup"]["inline_keyboard"][0][0]["callback_data"], json!(records[0].token));
        }
        a_tap_reaches_the_owners_stop(&store, 31);
    }

    /// **D1, R1 and fix-here item 6, finished — a frame picture carries a
    /// working *Stop this job***: sent with no button, the record written
    /// against it, then the button attached by the call that changes only it.
    #[test]
    fn a_frame_picture_carries_a_stop_that_reaches_the_owner() {
        let service = Arc::new(AStandInService::new());
        service.then_answers(r#"{"ok":true,"result":{"message_id":95}}"#);
        let (mut messages, store) = issuer(&service);
        let mut pictures = crate::pictures::ThePictures::new(CHAT, "Desk");
        let picture = ADerivative {
            job: "job-1".into(),
            job_label: "Kitchen".into(),
            frame: 140,
            file_name: "frame140.png".into(),
            bytes: crate::pictures::tests::a_png_header(1280, 720),
            which_picture: None,
        };
        let sent = pictures.send_with_its_stop(&mut messages, picture, AClassOfSend::FramePicturesAndMedia, Some(&job_one()));
        assert_eq!(sent, crate::pictures::APictureSent::Sent { message: 95, stop: Some(Ok(())) });

        let heard = service.heard();
        assert_eq!(heard.iter().map(|c| c.method.as_str()).collect::<Vec<_>>(), ["sendPhoto", "editMessageReplyMarkup"]);
        assert!(fields_of(&heard[0].carries).get("reply_markup").is_none(), "no button before its record");
        let records = store.the_action_contexts_bound_to(95).unwrap();
        assert_eq!(records.len(), 1);
        let attached = fields_of(&heard[1].carries);
        assert_eq!(attached["message_id"], json!(95));
        assert_eq!(attached["reply_markup"]["inline_keyboard"][0][0]["text"], json!("Stop this job"));
        assert_eq!(attached["reply_markup"]["inline_keyboard"][0][0]["callback_data"], json!(records[0].token));
        a_tap_reaches_the_owners_stop(&store, 95);
    }

    #[test]
    fn piece17_a_stop_press_opens_no_command_and_says_no_stop_was_sent() {
        let service = Arc::new(AStandInService::new());
        let (mut messages, store) = issuer(&service);
        messages
            .a_stop_button(90, &job_one(), "Desk", "Kitchen")
            .unwrap();
        let supervisor = AStandInSupervisor::default();
        let mut acts = owner(&service, &supervisor, vec!["job-1"]);
        let entry = press(&store, 90, STOP_THIS_JOB, "open");
        acts.act_once(&entry).unwrap();
        assert!(
            supervisor.heard().commands.is_empty(),
            "opening Stop must not submit a command"
        );
        assert!(
            the_stops_to_say(&[entry], &HashSet::new()).is_empty(),
            "no acknowledgement before confirmation"
        );
    }

    /// **An old drop's control, kept since no earlier message is edited,
    /// names only its hold** and can command no other job. Since the
    /// eighteenth notes, note 16 (2026-10-25), a drop carries no Stop at all;
    /// a control written before then still answers *This job has already
    /// ended.* once, **with no Stop of its own and no message edited**.
    #[test]
    fn eighteenth_6_an_old_drops_control_answers_that_the_job_ended_with_no_stop() {
        let service = Arc::new(AStandInService::new());
        service.then_answers(r#"{"ok":true,"result":{"message_id":95}}"#);
        service.then_answers(r#"{"ok":true,"result":{"message_id":96}}"#);
        let (_, store) = issuer(&service);
        let mut old = AnActionContextRecord::issued_under(
            KEY,
            CHAT,
            ME,
            None,
            Some("old-hold"),
            STOP_AN_ENDED_JOB,
            a_stop_to_offer("Desk", "Kitchen", false).payload,
            "old-hold",
        )
        .unwrap();
        old.source_message = Some(90);
        store.lock().unwrap().record_an_action_context(old).unwrap();
        let supervisor = AStandInSupervisor::default();
        let mut acts = owner(&service, &supervisor, vec!["old-hold", "job-1"]);
        acts.supervisor = Box::new(supervisor.with_context_store(store.clone()));
        let entry = press(&store, 90, STOP_AN_ENDED_JOB, "old-drop");
        acts.act_once(&entry).unwrap();
        acts.act_once(&entry).unwrap();
        assert!(supervisor.heard().commands.is_empty());
        assert_eq!(texts_sent(&service), ["Desk\nKitchen\nThis job has already ended."]);
        assert_eq!(
            service.heard().iter().map(|call| call.method.as_str()).collect::<Vec<_>>(),
            ["sendMessage"],
            "no Stop attached, and no earlier message edited"
        );
    }

    /// **The eighteenth notes, note 16: a Stop pressed after its job ended
    /// answers *This job has already ended.* as a new message with no Stop of
    /// its own, and edits no earlier message** (`Telegram` C1b as narrowed
    /// 2026-10-24). Before 2026-10-25 the answer carried a Stop for the
    /// same ended job.
    #[test]
    fn eighteenth_6_a_stop_pressed_after_the_job_ended_answers_with_no_stop() {
        for action in [ASK_TO_STOP_THIS_JOB, "confirmed"] {
            let service = Arc::new(AStandInService::new());
            let (mut messages, store) = issuer(&service);
            messages.a_stop_button(90, &job_one(), "Desk", "Kitchen").unwrap();
            let supervisor = AStandInSupervisor::default();
            let mut acts = owner(&service, &supervisor, vec!["job-1"]);
            acts.supervisor = Box::new(supervisor.with_context_store(store.clone()));
            let entry = if action == "confirmed" {
                service.then_answers(r#"{"ok":true,"result":{"message_id":101}}"#);
                service.then_answers(r#"{"ok":true,"result":{"message_id":101}}"#);
                acts.act_once(&press(&store, 90, STOP_THIS_JOB, "open")).unwrap();
                press(&store, 101, STOP_THIS_JOB, "yes")
            } else {
                press(&store, 90, STOP_THIS_JOB, "open")
            };
            let before = service.heard().len();
            service.then_answers(r#"{"ok":true,"result":{"message_id":120}}"#);
            service.then_answers(r#"{"ok":true,"result":{"message_id":120}}"#);
            acts.holds = Box::new(Live(vec![]));
            acts.act_once(&entry).unwrap();
            let after: Vec<String> = service.heard()[before..].iter().map(|call| call.method.clone()).collect();
            assert_eq!(after, ["sendMessage"], "{action}: one new message, nothing attached or edited");
            assert_eq!(texts_sent(&service).last().unwrap(), "Desk\nKitchen\nThis job has already ended.");
            assert!(supervisor.heard().commands.is_empty());
        }
    }

    /// **The eighteenth notes, note 16: a stop's outcome is reported with no
    /// Stop** (`Telegram` D2), **and an answer about a job carries one only
    /// while that job is live** when it is composed (`Telegram` C1b). Before
    /// 2026-10-25 every report had a Stop attached.
    #[test]
    fn eighteenth_6_a_report_carries_stop_only_while_its_job_is_live() {
        let reported = |live: Vec<&'static str>, journalled: AJournalledOutcome| {
            let service = Arc::new(AStandInService::new());
            service.then_answers(r#"{"ok":true,"result":{"message_id":130}}"#);
            let (_, store) = issuer(&service);
            let supervisor = AStandInSupervisor::default();
            let mut acts = owner(&service, &supervisor, live);
            acts.supervisor = Box::new(supervisor.with_context_store(store.clone()));
            acts.report_about(journalled, &job_one(), "Desk", "Kitchen").unwrap();
            service.heard().iter().map(|call| call.method.clone()).collect::<Vec<_>>()
        };
        let stopped = || AJournalledOutcome::AStop {
            server: "Desk".into(),
            label: "Kitchen".into(),
            outcome: AStopsOutcome::Stopped { frames_on_the_server: 2, frames_not_rendered: 1 },
        };
        let sent_again = || AJournalledOutcome::ASubmission {
            server: "Desk".into(),
            label: "Kitchen".into(),
            outcome: ASubmissionsOutcome::Submitted,
        };
        assert_eq!(reported(vec!["job-1"], stopped()), ["sendMessage"], "a stop's outcome, even before the record reads ended");
        assert_eq!(reported(vec![], stopped()), ["sendMessage"]);
        assert_eq!(reported(vec![], sent_again()), ["sendMessage"], "a job that has ended");
        assert_eq!(
            reported(vec!["job-1"], sent_again()),
            ["sendMessage", "editMessageReplyMarkup"],
            "a live job keeps its Stop"
        );
    }

    #[test]
    fn piece17_opening_stop_cannot_rebind_while_the_question_is_composed() {
        struct Changed(std::sync::atomic::AtomicUsize);
        impl ReadsWhatThisMachineHolds for Changed {
            fn the_jobs(&self) -> Vec<AJobAsHeld> { vec![] }
            fn the_live_generation(&self, _: &str) -> Option<String> {
                let read = self.0.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                Some(if read == 0 { "1" } else { "2" }.into())
            }
        }
        let service = Arc::new(AStandInService::new());
        let (mut messages, store) = issuer(&service);
        messages.a_stop_button(90, &job_one(), "Desk", "Kitchen").unwrap();
        let supervisor = AStandInSupervisor::default();
        let mut acts = owner(&service, &supervisor, vec![]);
        acts.supervisor = Box::new(supervisor.with_context_store(store.clone()));
        acts.holds = Box::new(Changed(std::sync::atomic::AtomicUsize::new(0)));
        acts.act_once(&press(&store, 90, STOP_THIS_JOB, "changed-while-opening")).unwrap();
        assert!(supervisor.heard().commands.is_empty());
        let texts = texts_sent(&service);
        assert!(!texts.iter().any(|text| text.starts_with("Desk\nKitchen\nStop this job?")));
        assert!(texts.iter().any(|text| text.contains("This job has already ended.")));
    }

    #[test]
    fn piece17_confirmation_yes_keep_repeated_and_ended_are_bound_to_the_question() {
        for decision in ["yes", "keep", "ended", "replaced"] {
            let service = Arc::new(AStandInService::new());
            service.then_answers(r#"{"ok":true,"result":{"message_id":101}}"#);
            let (mut messages, store) = issuer(&service);
            messages
                .a_stop_button(90, &job_one(), "Desk", "Kitchen")
                .unwrap();
            let supervisor = AStandInSupervisor::default();
            let mut acts = owner(&service, &supervisor, vec!["job-1"]);
            acts.supervisor = Box::new(supervisor.with_context_store(store.clone()));
            let opening = press(&store, 90, STOP_THIS_JOB, "open");
            acts.act_once(&opening).unwrap();
            acts.act_once(&opening).unwrap();
            assert!(supervisor.heard().commands.is_empty());
            assert_eq!(
                texts_sent(&service)
                    .iter()
                    .filter(|text| text.starts_with("Desk\nKitchen\nStop this job?"))
                    .count(),
                1
            );
            let confirmed = press(&store, 101, STOP_THIS_JOB, "yes");
            assert_eq!(confirmed.payload["confirmed"], json!(true));
            if decision == "keep" {
                let keep = press(&store, 101, KEEP_RENDERING, "keep");
                acts.act_once(&keep).unwrap();
            } else if decision == "ended" {
                acts.holds = Box::new(Live(vec![]));
            } else if decision == "replaced" {
                struct Next;
                impl ReadsWhatThisMachineHolds for Next {
                    fn the_jobs(&self) -> Vec<AJobAsHeld> {
                        vec![]
                    }
                    fn the_live_generation(&self, _: &str) -> Option<String> {
                        Some("2".into())
                    }
                }
                acts.holds = Box::new(Next);
            }
            acts.act_once(&confirmed).unwrap();
            acts.act_once(&confirmed).unwrap();
            let second_press = press(&store, 101, STOP_THIS_JOB, "yes-again");
            acts.act_once(&second_press).unwrap();
            assert_eq!(
                supervisor.heard().commands.len(),
                usize::from(decision == "yes"),
                "{decision}"
            );
            if decision == "ended" || decision == "replaced" {
                assert!(texts_sent(&service)
                    .iter()
                    .any(|text| text.contains("This job has already ended.")));
            }
        }
    }

    /// **`Telegram` C1b, the live Telegram pass's fault 6: a confirmed stop
    /// leaves no *Keep rendering* behind once its job ends.** Confirming
    /// clears the question's buttons and marks every record on it, so the
    /// ending's removal of the job's Stops rebuilds nothing on it. Before
    /// this, the removal found the question's *Stop this job* record unmarked
    /// and rebuilt the message from its other records — *Keep rendering*.
    #[test]
    fn live_pass_6_a_confirmed_stop_leaves_no_keep_rendering_once_the_job_ends() {
        let service = Arc::new(AStandInService::new());
        service.then_answers(r#"{"ok":true,"result":{"message_id":101}}"#);
        let (mut messages, store) = issuer(&service);
        messages.a_stop_button(90, &job_one(), "Desk", "Kitchen").unwrap();
        let supervisor = AStandInSupervisor::default();
        let mut acts = owner(&service, &supervisor, vec!["job-1"]);
        acts.supervisor = Box::new(supervisor.with_context_store(store.clone()));
        acts.act_once(&press(&store, 90, STOP_THIS_JOB, "open")).unwrap();
        let confirmed = press(&store, 101, STOP_THIS_JOB, "yes");
        acts.act_once(&confirmed).unwrap();
        assert_eq!(supervisor.heard().commands.len(), 1, "the stop was submitted");
        // **The job ends**: every earlier Stop of it goes.
        messages.remove_the_stops_where(&|_| true).unwrap();
        let the_last_keyboard_of = |message: i64| {
            service
                .heard()
                .iter()
                .filter(|c| c.method == "editMessageReplyMarkup" || c.method == "editMessageText")
                .map(|c| fields_of(&c.carries))
                .filter(|f| f["message_id"] == json!(message))
                .filter_map(|f| f["reply_markup"]["inline_keyboard"].as_array().cloned())
                .last()
                .unwrap_or_default()
        };
        assert!(the_last_keyboard_of(101).is_empty(), "{:?}", the_last_keyboard_of(101));
        assert!(the_last_keyboard_of(90).is_empty());
        // A second sweep edits nothing more.
        let calls = service.heard().len();
        assert_eq!(messages.remove_the_stops_where(&|_| true).unwrap(), 0);
        assert_eq!(service.heard().len(), calls);
    }

    /// **The live Telegram pass's fault 6: every button on one message goes,
    /// and every record on it is marked**, so nothing rebuilds it; the
    /// records stay for a late tap. A message already cleared is not edited
    /// again, and a line that reaches nothing leaves it owed.
    #[test]
    fn live_pass_6_every_button_on_a_message_goes_once() {
        let service = Arc::new(AStandInService::new());
        service.then_answers(r#"{"ok":true,"result":{"message_id":77}}"#);
        let (mut messages, store) = issuer(&service);
        let message = messages
            .send_about_with_buttons(&the_failure_message(&a_failure(TheCause::TheMachinesOwn)), &job_one(), "Desk", "Kitchen")
            .unwrap();
        let before = store.the_action_contexts_bound_to(message).unwrap().len();
        service.then_is_not_reached("line cut");
        let failed = messages.remove_every_button_on(message, AClassOfSend::EndingsHoldsAndDrops).unwrap_err();
        assert!(failed.the_line_reached_nothing(), "{failed:?}");
        assert!(store.the_action_contexts_bound_to(message).unwrap().iter().all(|r| r.payload["stop_removed"] != json!(true)));
        assert_eq!(messages.remove_every_button_on(message, AClassOfSend::EndingsHoldsAndDrops).unwrap(), before);
        let last = service.heard().last().map(|c| fields_of(&c.carries)).unwrap();
        assert_eq!(last["reply_markup"]["inline_keyboard"], json!([]));
        let after = store.the_action_contexts_bound_to(message).unwrap();
        assert_eq!(after.len(), before, "late taps keep their contexts");
        assert!(after.iter().all(|r| r.payload["stop_removed"] == json!(true)));
        let calls = service.heard().len();
        assert_eq!(messages.remove_every_button_on(message, AClassOfSend::EndingsHoldsAndDrops).unwrap(), 0);
        assert_eq!(messages.remove_the_stops_where(&|_| true).unwrap(), 0, "nothing rebuilds it");
        assert_eq!(service.heard().len(), calls);
    }

    #[test]
    fn piece17_first_frame_zero_one_many_and_eleven_keep_all_answers_on_the_range() {
        for number in [0usize, 1, 4, 11] {
            let service = Arc::new(AStandInService::new());
            for first in (0..number).step_by(10) {
                let count = (number - first).min(10);
                let result = if count == 1 {
                    json!({"message_id":100+first})
                } else {
                    json!((0..count)
                        .map(|n| json!({"message_id":100+first+n}))
                        .collect::<Vec<_>>())
                };
                service.then_answers(&json!({"ok":true,"result":result}).to_string());
            }
            service.then_answers(r#"{"ok":true,"result":{"message_id":300}}"#);
            let (mut messages, store) = issuer(&service);
            let waiting = AWaitingJob {
                server: "Desk".into(),
                label: "Range".into(),
                hold: "hold-1".into(),
                generation: "1".into(),
                waits_for: WhatTheHoldWaitsFor::TheUsersAnswer,
                the_frame_shown: 1,
                frames_held: 240,
                the_frames: String::new(),
            };
            let subject = TheSubject {
                owning_machine: ME.into(),
                job: Some("range-1".into()),
                hold: Some("hold-1".into()),
                generation: "1".into(),
            };
            let copies = (0..number)
                .map(|n| ADerivative {
                    job: "check-1".into(),
                    job_label: "Check".into(),
                    frame: 1,
                    file_name: format!("{n}.png"),
                    bytes: crate::pictures::tests::a_png_header(1280, 720),
                    which_picture: Some(crate::pictures::WhichPictureItIs::AViewLayer(format!(
                        "Layer {n}"
                    ))),
                })
                .collect();
            assert_eq!(
                messages.send_the_waiting_set(&waiting, copies, &subject),
                Ok(Some(300))
            );
            let records = store.the_action_contexts_bound_to(300).unwrap();
            assert_eq!(records.len(), 3);
            assert_eq!(
                records
                    .iter()
                    .filter(|r| r.action == ANSWER_A_WAITING_JOB
                        && r.hold.as_deref() == Some("hold-1")
                        && r.payload["job"] == json!("range-1"))
                    .count(),
                2
            );
            let stop = records
                .iter()
                .find(|r| r.action == ASK_TO_STOP_THIS_JOB)
                .unwrap();
            assert_eq!(stop.job.as_deref(), Some("range-1"));
            assert!(stop.hold.is_none());
            assert!(
                service
                    .heard()
                    .iter()
                    .filter(|c| c.method == "editMessageReplyMarkup")
                    .next()
                    .is_none(),
                "never attach to an album or picture"
            );
            let texts = texts_sent(&service);
            assert!(texts.last().unwrap().contains("Needs your answer."));
            if number == 0 {
                assert!(texts.last().unwrap().contains("No preview available."));
            }
        }
    }

    #[test]
    fn piece17_refused_first_frame_still_asks_both_answers() {
        let service = Arc::new(AStandInService::new());
        service.then_answers(r#"{"ok":false,"error_code":400,"description":"photo refused"}"#);
        service.then_answers(r#"{"ok":true,"result":{"message_id":200}}"#);
        service.then_answers(r#"{"ok":true,"result":{"message_id":300}}"#);
        let (mut messages, store) = issuer(&service);
        let waiting = AWaitingJob {
            server: "Desk".into(),
            label: "Range".into(),
            hold: "hold-1".into(),
            generation: "1".into(),
            waits_for: WhatTheHoldWaitsFor::TheUsersAnswer,
            the_frame_shown: 1,
            frames_held: 240,
            the_frames: String::new(),
        };
        let subject = TheSubject {
            owning_machine: ME.into(),
            job: Some("range-1".into()),
            hold: Some("hold-1".into()),
            generation: "1".into(),
        };
        let picture = ADerivative {
            job: "check".into(),
            job_label: "Check".into(),
            frame: 1,
            file_name: "1.png".into(),
            bytes: crate::pictures::tests::a_png_header(1280, 720),
            which_picture: None,
        };
        assert_eq!(
            messages.send_the_waiting_set(&waiting, vec![picture], &subject),
            Ok(Some(300))
        );
        assert_eq!(
            store
                .the_action_contexts_bound_to(300)
                .unwrap()
                .iter()
                .filter(|r| r.action == ANSWER_A_WAITING_JOB)
                .count(),
            2
        );
    }

    // ---------------------------------------------------------- E3d

    /// **Telegram's bound on an ordinary message's text** — 4,096, counted
    /// here in UTF-16 code units, the one unit Telegram's pages name for a
    /// message's text (`Long machine output in the chat — what Telegram's
    /// pages say.md` §1 and §3). Written out rather than borrowed, so a test
    /// would catch the code counting in another unit.
    const THE_BOUND: usize = 4096;

    fn units(text: &str) -> usize {
        text.encode_utf16().count()
    }

    /// The text an entity marks, cut out by its UTF-16 positions.
    fn marked_by(text: &str, entity: &Value) -> String {
        let (offset, length) = (entity["offset"].as_u64().unwrap() as usize, entity["length"].as_u64().unwrap() as usize);
        let utf16: Vec<u16> = text.encode_utf16().collect();
        String::from_utf16(&utf16[offset..offset + length]).expect("the positions fall on whole characters")
    }

    /// **A failure with words on both streams, one stream's words tapped**;
    /// gives back every call the owner made on the tap.
    fn tap_for_the_words(stream: AStream, words: Vec<u8>, complaints: ACollectionOutcome) -> Vec<crate::transport::a_stand_in::ACallItHeard> {
        let service = Arc::new(AStandInService::new());
        service.then_answers(r#"{"ok":true,"result":{"message_id":77}}"#);
        let (mut messages, store) = issuer(&service);
        let mut failure = a_failure(TheCause::TheMachinesOwn);
        failure.complaints = complaints;
        messages.send_with_buttons(&the_failure_message(&failure), &job_one()).unwrap();

        let supervisor = AStandInSupervisor::default();
        supervisor.heard().words.insert(("job-1".into(), stream), words);
        let answers = Arc::new(AStandInService::new());
        answers.then_answers(r#"{"ok":true,"result":{"message_id":78}}"#);
        let mut acts = owner(&answers, &supervisor, vec!["job-1"]);
        let record = store
            .the_action_contexts_bound_to(77)
            .unwrap()
            .into_iter()
            .find(|r| r.action == SEND_THE_WORDS && r.payload["stream"] == json!(stream))
            .expect("a button for this stream");
        let evidence = ATapsEvidence { token: record.token.clone(), callback: "cb-words".into(), conversation: CHAT, source_message: 77, label: None };
        let entry = a_taps_resolution(&evidence, 1, &WhatATokenNames::read(&record.token).unwrap());
        act_once(&mut acts, &resolve_a_tap(&entry, ME, KEY, &store, &|_| Some("1".into())).unwrap());
        // **C6b: the act ended with any file still waiting in the upload lane**,
        // so it did not hold the listener's turn for it; it goes when that lane
        // is sent.
        let files_waiting = acts.queue.how_many_uploads_are_waiting();
        assert_eq!(answers.heard().iter().filter(|c| c.method == "sendDocument").count(), 0, "C6b: the act waited for its file");
        while acts.queue.send_the_next_upload() {}
        let heard = answers.heard();
        assert_eq!(heard.iter().filter(|c| c.method == "sendDocument").count(), files_waiting);
        heard
    }

    /// The plain log body, bounded in UTF-16 units, under its marked names.
    fn the_block(call: &crate::transport::a_stand_in::ACallItHeard) -> (String, String) {
        assert_eq!(call.method, "sendMessage");
        let fields = fields_of(&call.carries);
        assert!(fields.get("parse_mode").is_none(), "never markup");
        let text = fields["text"].as_str().unwrap().to_string();
        assert!(units(&text) <= THE_BOUND, "over Telegram's bound: {} units", units(&text));
        let entities = fields["entities"].as_array().expect("an entity list");
        assert_eq!(entities.len(), 2);
        assert_eq!(entities[0]["type"], "bold");
        assert_eq!(marked_by(&text, &entities[0]), "Desk");
        assert_eq!(entities[1]["type"], "italic");
        assert_eq!(marked_by(&text, &entities[1]), "Kitchen");
        let block = text.splitn(4, '\n').nth(3).unwrap_or("").trim_end().to_string();
        (text, block)
    }

    /// Lines from many places: Cyrillic, Japanese, emoji, a Windows path full
    /// of backslashes, and every character markup would have wanted escaped.
    fn a_line(n: usize) -> String {
        format!("Кадр {n} 🎬 Fra:{n} Mem:1.2G | C:\\Users\\Анна\\シーン_[final]*(v2).blend <b>&copy;</b> `ok` ~#+-=|{{}}.!\n")
    }

    /// **E3d, whole: words that fit arrive whole in the chat, and no file.**
    #[test]
    fn words_that_fit_arrive_whole_in_the_chat_with_no_file() {
        let words: String = (1..=5).map(a_line).collect();
        let heard = tap_for_the_words(AStream::TheOrdinaryOutput, words.clone().into_bytes(), ACollectionOutcome::Collected);
        assert_eq!(heard.len(), 1, "one message and no file");
        let (text, block) = the_block(&heard[0]);
        assert_eq!(text, format!("Desk\nKitchen\nOutput log, in full.\n{words}"));
        assert_eq!(block, words.trim_end(), "the block is the words, its trailing newline outside the entity");
        assert!(text.len() > units(&text), "the test really holds letters that count differently");
    }

    /// **E3d, the end: words too long arrive as their end, cut at a line by
    /// length alone, and the whole stream follows as a file.**
    #[test]
    fn words_too_long_arrive_as_their_end_then_as_the_whole_file() {
        let words: String = (1..=300).map(a_line).collect();
        let heard = tap_for_the_words(AStream::TheOrdinaryOutput, words.clone().into_bytes(), ACollectionOutcome::Collected);
        assert_eq!(heard.iter().map(|c| c.method.as_str()).collect::<Vec<_>>(), ["sendMessage", "sendDocument"], "the block, then the file");
        let (text, block) = the_block(&heard[0]);

        let lines = block.split_inclusive('\n').count();
        let head = format!("Desk\nKitchen\nOutput log, last {lines} lines. Full log in the file below.\n");
        assert!(text.starts_with(&head), "{}", &text[..head.len().min(text.len())]);
        let shown = &text[head.len()..];
        assert!(words.ends_with(shown), "the block is the end of the words, untouched");
        assert!(words[..words.len() - shown.len()].ends_with('\n'), "cut at a line boundary");
        // Length alone: one more line would not have fitted.
        let one_more = a_line(300 - lines);
        let longer_head = format!("Desk\nKitchen\nOutput log, last {} lines. Full log in the file below.\n", lines + 1);
        assert!(units(&longer_head) + units(&one_more) + units(shown) > THE_BOUND);

        let WhatACallCarries::FieldsAndFiles { fields, files } = &heard[1].carries else { panic!("a file") };
        assert_eq!(files[0].bytes, words.as_bytes(), "the file is the whole stream: nothing trimmed");
        assert_eq!(files[0].file_name, "Kitchen - Output.txt");
        assert_eq!(fields["caption"], json!("Desk\nKitchen"));
    }

    /// **E3d, over the cap: the end, and the label saying the whole record is
    /// too large to send and is read from the desk. No file.**
    #[test]
    fn words_over_the_cap_arrive_as_their_end_and_say_so() {
        let line = b"Fra:1 Mem:1.2G | Rendering 1 / 40\n";
        let mut words = Vec::with_capacity(THE_CAP_ON_A_FILE as usize + line.len());
        while (words.len() as u64) <= THE_CAP_ON_A_FILE {
            words.extend_from_slice(line);
        }
        let heard = tap_for_the_words(AStream::TheComplaints, words, ACollectionOutcome::Collected);
        assert_eq!(heard.len(), 1, "no file over the cap");
        let (text, block) = the_block(&heard[0]);
        let lines = block.split_inclusive('\n').count();
        assert!(text.starts_with(&format!(
            "Desk\nKitchen\nErrors log, last {lines} lines. Full log over 50 MB, too large to send. Open it in Smoozy on your computer.\n"
        )));
        assert!(block.lines().all(|l| l == "Fra:1 Mem:1.2G | Rendering 1 / 40"));
    }

    /// **E3d, a line longer than the whole bound.** No line boundary can make
    /// it fit, so the block holds the end of that one line, cut between whole
    /// characters, and the label says so; the whole stream follows as a file.
    #[test]
    fn a_line_longer_than_the_bound_arrives_as_the_end_of_that_line() {
        let long = "é🎬ж".repeat(2000);
        let words = format!("an earlier line\n{long}\n");
        let heard = tap_for_the_words(AStream::TheOrdinaryOutput, words.clone().into_bytes(), ACollectionOutcome::Collected);
        assert_eq!(heard.iter().map(|c| c.method.as_str()).collect::<Vec<_>>(), ["sendMessage", "sendDocument"]);
        let (text, block) = the_block(&heard[0]);
        let head = "Desk\nKitchen\nOutput log, the end of the last line. Full log in the file below.\n";
        assert!(text.starts_with(head));
        let shown = &text[head.len()..];
        assert!(long.ends_with(shown.trim_end()) && shown.ends_with('\n'), "the end of the one long line");
        assert!(!block.contains("an earlier line"));
        assert!(units(&text) > THE_BOUND - 2, "as much as fits, the widest character being two units");
    }

    /// **E3d — every words button names its stream**, and each tap sends its
    /// own stream; **`Rented-server runtime` M26 — an incomplete record is marked on the label too.**
    #[test]
    fn a_two_stream_failure_offers_two_buttons_each_naming_its_stream() {
        let mut failure = a_failure(TheCause::TheMachinesOwn);
        failure.complaints = ACollectionOutcome::Collected;
        let labels: Vec<String> = the_failure_message(&failure).buttons.iter().map(|b| b.label.clone()).collect();
        assert_eq!(labels, ["Send Output log", "Send Errors log", "Retry"]);

        let heard = tap_for_the_words(AStream::TheComplaints, b"Error: out of memory\n".to_vec(), ACollectionOutcome::Collected);
        assert_eq!(the_block(&heard[0]).0, "Desk\nKitchen\nErrors log, in full.\nError: out of memory\n");
        let heard = tap_for_the_words(AStream::TheComplaints, b"Error: out of\n".to_vec(), ACollectionOutcome::CollectionIncomplete);
        assert_eq!(the_block(&heard[0]).0, "Desk\nKitchen\nErrors log, incomplete.\nError: out of\n");
    }

    /// **D1, D2 and D4 — the stop submitted to the owner's own supervisor, its
    /// journalled outcome told with the counts, and a second press told it was
    /// already stopped. The listener's first half is said once per stop.**
    #[test]
    fn the_stop_is_submitted_by_the_owner_and_acknowledged_twice() {
        let service = Arc::new(AStandInService::new());
        let (mut messages, store) = issuer(&service);
        let button = messages.a_stop_button(90, &job_one(), "Desk", "Kitchen").unwrap();
        assert_eq!(button["text"], json!("Stop this job"));

        let supervisor = AStandInSupervisor::default();
        let answers = Arc::new(AStandInService::new());
        let mut acts = owner(&answers, &supervisor, vec!["job-1"]);
        let first = confirmed_press(&store, 90, "cb-1");

        let mut said = HashSet::new();
        let to_say = the_stops_to_say(std::slice::from_ref(&first), &said);
        assert_eq!(to_say, vec![(first.operation.clone(), the_stop_was_sent("Desk", "Kitchen"))]);
        said.insert(first.operation.clone());
        assert!(the_stops_to_say(std::slice::from_ref(&first), &said).is_empty());

        act_once(&mut acts, &first);
        act_once(&mut acts, &confirmed_press(&store, 90, "cb-2"));
        let journal = supervisor.heard().stops.clone();
        assert_eq!(supervisor.heard().commands.len(), 2);
        for (_, outcome) in journal {
            acts.report(AJournalledOutcome::AStop { server: "Desk".into(), label: "Kitchen".into(), outcome });
        }
        let texts = texts_sent(&answers);
        assert!(texts[0].contains("Stopped. 12 frames on the server"));
        assert!(texts[1].starts_with("Desk\nKitchen\nAlready stopped at "));
    }

    /// **R1 on the owner — an ended generation submits nothing**, and the user
    /// is told nothing was done.
    #[test]
    fn nothing_is_submitted_for_a_subject_that_has_ended() {
        let service = Arc::new(AStandInService::new());
        service.then_answers(r#"{"ok":true,"result":{"message_id":77}}"#);
        let (mut messages, store) = issuer(&service);
        messages.send_with_buttons(&the_failure_message(&a_failure(TheCause::TheMachinesOwn)), &job_one()).unwrap();
        let supervisor = AStandInSupervisor::default();
        let answers = Arc::new(AStandInService::new());
        let mut acts = owner(&answers, &supervisor, vec![]);
        let tap = press(&store, 77, TRY_IT_AGAIN, "cb-1");
        act_once(&mut acts, &tap);
        act_once(&mut acts, &tap);
        assert!(supervisor.heard().commands.is_empty());
        assert_eq!(texts_sent(&answers), [A_BUTTON_THAT_EXPIRED]);
    }
}

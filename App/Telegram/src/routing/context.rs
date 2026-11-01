//! **The action-context record, and resolution that fails closed — `Telegram`
//! R1, R1a, R2 and R4.**
//!
//! **R1: the issuing machine first writes the action-context record durably
//! through its supervisor.** The record, not the token, binds the token to the
//! proved conversation, the source message, the owning machine, the job and
//! hold identities where present, the action, the complete payload, and the
//! subject generation in which the action is valid (`State, identity and
//! recovery` §2.2 Y4). **The store is F2's**: it is reached through
//! [`TheTelegramWorkersState`], which on a machine the supervisor writes.
//!
//! **Resolution fails closed**: a missing record, an unknown format, an ended
//! subject, a wrong conversation, a wrong source message or a mismatched
//! generation is rejected and nothing is submitted. A successful resolution
//! gives the full instruction that replaces the context-resolution entry under
//! the same operation identity (B8, B15).
//!
//! **R2: the listener routes and enforces nothing.** Every instruction here is
//! addressed to the **owning** machine by its identity, including where that is
//! the listener's own; *holder* is never used for the machine holding a job.
//!
//! **R4: each press is one instruction.** A tap's operation identity is seeded
//! by the `callback_query`'s own identifier, so two presses are two
//! instructions and a redelivered press is the same one.

use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::board::instructions::{
    AKindOfEntry, AnEntry, TheAddressee, THE_DEFAULT_REPORTING_DEADLINE, THE_DEFAULT_RESOLUTION_DEADLINE,
    THE_PROTOCOL_VERSION,
};
use crate::routing::token::{a_fresh_action_context, from_hexadecimal, WhatATokenNames};
use crate::worker_state::{ASentMessage, ThePhonesChoice, TheTelegramWorkersState, WhatAMessageIsAbout, WhyTheStoreFailed};

/// **`Telegram` R1 — the authoritative action-context record.**
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnActionContextRecord {
    /// The 16-byte action-context identity, written as text.
    pub identity: String,
    /// The whole token the button carries, compared exactly.
    pub token: String,
    pub conversation: i64,
    /// The message the button rides, once it is known. A record whose message
    /// is not yet known resolves nothing.
    pub source_message: Option<i64>,
    pub owning_machine: String,
    pub job: Option<String>,
    pub hold: Option<String>,
    pub action: String,
    pub payload: Value,
    /// The subject generation the action is valid in.
    pub generation: String,
}

impl AnActionContextRecord {
    /// **`Telegram` R1 — a fresh record and its token**, under this machine's
    /// routing key. Nothing is written by this; writing it is the issuer's
    /// first act.
    #[allow(clippy::too_many_arguments)]
    pub fn issued_under(
        routing_key: &str,
        conversation: i64,
        owning_machine: &str,
        job: Option<&str>,
        hold: Option<&str>,
        action: &str,
        payload: Value,
        generation: &str,
    ) -> Option<AnActionContextRecord> {
        let named = WhatATokenNames { routing_key: from_hexadecimal(routing_key)?, action_context: a_fresh_action_context()? };
        Some(AnActionContextRecord {
            identity: named.the_action_context(),
            token: named.as_the_token(),
            conversation,
            source_message: None,
            owning_machine: owning_machine.to_string(),
            job: job.map(str::to_string),
            hold: hold.map(str::to_string),
            action: action.to_string(),
            payload,
            generation: generation.to_string(),
        })
    }

    /// The subject the generation belongs to: the hold where there is one,
    /// otherwise the job.
    pub fn the_subject(&self) -> Option<&str> {
        self.hold.as_deref().or(self.job.as_deref())
    }
}

/// **What a tap gives the listener, carried to the issuing machine** as the
/// payload of the context-resolution instruction (B8: *the opaque action token
/// and callback evidence*).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ATapsEvidence {
    pub token: String,
    pub callback: String,
    pub conversation: i64,
    pub source_message: i64,
    /// The pressed button's own label, where the tap's message showed it — for
    /// what the user is told about this entry (R7, Rule 9), never for routing.
    #[serde(default)]
    pub label: Option<String>,
}

/// **What a reply gives the listener** (R1a): the typed request and the
/// reply evidence.
///
/// **Widened 2026-10-31 (R1a, widened): `frame`, `stop` and `reporting`.**
/// `frame` is written only for a frame request with a number, so a machine
/// on an older release — which reads this with `frame` required — cannot read
/// any other request's entry and rejects it, never half-reading a stop or a
/// reporting change as a frame request. `request` is absent on an older
/// listener's entry, which is a numbered frame request.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AReplysEvidence {
    pub conversation: i64,
    pub source_message: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frame: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request: Option<ARepliedRequest>,
}

/// **The request a reply carries**, beside a numbered frame's `frame`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ARepliedRequest {
    /// `frame`, numbered (the number in `frame`) or not.
    Frame,
    Stop,
    /// `reporting`, with the minutes where typed (range already checked).
    Reporting { minutes: Option<u8> },
}

impl AReplysEvidence {
    /// The request, an older listener's entry being a numbered frame's.
    pub fn the_request(&self) -> ARepliedRequest {
        self.request.unwrap_or(ARepliedRequest::Frame)
    }
}

/// The action a resolution entry names. Read by no rule; it says what the
/// entry is to whoever reads the board.
pub const RESOLVING_A_TAP: &str = "resolve a button press";
pub const RESOLVING_A_REPLY: &str = "resolve a reply";

/// **`Telegram` R13 and R14 — the routed frame request's action.**
pub const SENDING_A_FRAME: &str = "send a frame";

/// **`Telegram` R13a and C1c — the newest frame the job's derivative set
/// holds.** Its own action, so an owner on an older release refuses it as an
/// action it does not know rather than reading a missing number as frame 0.
pub const SENDING_THE_NEWEST_FRAME: &str = "send the newest frame";

/// **`Telegram` V1d — the phone's choice, told to one machine** for every job
/// it holds that is waiting, held or rendering.
pub const EVERY_LIVE_JOBS_REPORTING: &str = "every live job's reporting";

/// **`Telegram` V1d's ask, its five buttons** — issued and resolved by the
/// listener; a tap starts the round on the machine that resolves it.
pub const CHOOSING_THE_PHONES_CHOICE: &str = "choose the phone's choice";

/// **`Telegram` R1a, widened — one job's reporting, from a reply**: V1c's
/// change for that job alone (its value in the payload), or, with none, that
/// job's own ask of five buttons.
pub const ONE_JOBS_REPORTING: &str = "one job's reporting";

/// **`Telegram` R1 and R4 — the context-resolution instruction for one tap**,
/// addressed by the routing key alone, carrying R1's default deadline. Its
/// operation identity is seeded by the press's own identifier.
pub fn a_taps_resolution(evidence: &ATapsEvidence, update: i64, names: &WhatATokenNames) -> AnEntry {
    AnEntry {
        version: THE_PROTOCOL_VERSION,
        operation: format!("tap-{}", evidence.callback),
        addressee: TheAddressee::RoutingKey(names.the_routing_key()),
        kind: AKindOfEntry::ContextResolution,
        subjects: Vec::new(),
        action: RESOLVING_A_TAP.to_string(),
        payload: json!(evidence),
        source_update: Some(update),
        accepted_at: String::new(),
        deadline_seconds: THE_DEFAULT_RESOLUTION_DEADLINE.as_secs(),
    }
}

/// **`Telegram` R1a — the context-resolution instruction for one reply**,
/// addressed by the source message identity, the one address a reply supplies.
pub fn a_replys_resolution(evidence: &AReplysEvidence, update: i64) -> AnEntry {
    AnEntry {
        version: THE_PROTOCOL_VERSION,
        operation: format!("reply-{update}"),
        addressee: TheAddressee::SourceMessage(evidence.source_message),
        kind: AKindOfEntry::ContextResolution,
        subjects: Vec::new(),
        action: RESOLVING_A_REPLY.to_string(),
        payload: json!(evidence),
        source_update: Some(update),
        accepted_at: String::new(),
        deadline_seconds: THE_DEFAULT_RESOLUTION_DEADLINE.as_secs(),
    }
}

/// **`Telegram` R14 — the routed frame request**, a full action instruction to
/// the owning machine, carrying the generation the choice was made in so the
/// owner can compare it with its own, **and the server's and the job's labels**
/// for what the user is told about it (R7; never routed by). **No frame is
/// R13a's newest** ([`SENDING_THE_NEWEST_FRAME`]).
pub fn a_frame_request(
    operation: &str,
    owner: &str,
    job: &str,
    generation: &str,
    frame: Option<u64>,
    update: Option<i64>,
    labels: Value,
) -> AnEntry {
    let (action, payload) = match frame {
        Some(frame) => (SENDING_A_FRAME, json!({ "frame": frame, "job": job, "generation": generation, "labels": labels })),
        None => (SENDING_THE_NEWEST_FRAME, json!({ "job": job, "generation": generation, "labels": labels })),
    };
    an_action_on_one_job(operation, owner, job, action, payload, update)
}

/// **One full action instruction about one job, to the machine that owns it**
/// (R2), under the reporting deadline (B12).
pub fn an_action_on_one_job(operation: &str, owner: &str, job: &str, action: &str, payload: Value, update: Option<i64>) -> AnEntry {
    AnEntry {
        version: THE_PROTOCOL_VERSION,
        operation: operation.to_string(),
        addressee: TheAddressee::Machine(owner.to_string()),
        kind: AKindOfEntry::Action,
        subjects: vec![job.to_string()],
        action: action.to_string(),
        payload,
        source_update: update,
        accepted_at: String::new(),
        deadline_seconds: THE_DEFAULT_REPORTING_DEADLINE.as_secs(),
    }
}

/// **Why a resolution was rejected.** Nothing is submitted for any of them
/// (R1), and the user's answer is the expiry's (R1, R3).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhyItFailsClosed {
    TheEntryCannotBeRead,
    TheTokenIsRefused,
    NotThisMachinesKey,
    NoRecord,
    TheTokenDoesNotMatch,
    TheConversationDoesNotMatch,
    TheSourceMessageDoesNotMatch,
    TheSubjectHasEnded,
    TheStoreFailed,
}

/// **`Telegram` R1 — the generation check, and whose it is.** The issuing
/// machine compares the live generation of a subject **it owns**. Where the
/// issuer is not the owner — the listener's own disambiguation (R13) offers
/// other machines' jobs — it holds no live generation to compare, so the
/// generation rides the full instruction and the owner compares it on acting
/// ([`the_owner_finds_the_subject_live`]). Recorded at
/// `Audits/Build/Telegram/Decisions record.md`.
fn the_generation_holds(record: &AnActionContextRecord, me: &str, live: &dyn Fn(&str) -> Option<String>) -> bool {
    if record.owning_machine != me {
        return true;
    }
    match record.the_subject() {
        Some(subject) => live(subject).as_deref() == Some(record.generation.as_str()),
        None => true,
    }
}

/// **`Telegram` R1 — resolve one tap on the issuing machine.** Gives back the
/// full instruction that replaces the entry under the same operation identity.
pub fn resolve_a_tap(
    entry: &AnEntry,
    me: &str,
    my_routing_key: &str,
    store: &dyn TheTelegramWorkersState,
    live: &dyn Fn(&str) -> Option<String>,
) -> Result<AnEntry, WhyItFailsClosed> {
    let evidence: ATapsEvidence =
        serde_json::from_value(entry.payload.clone()).map_err(|_| WhyItFailsClosed::TheEntryCannotBeRead)?;
    let names = WhatATokenNames::read(&evidence.token).map_err(|_| WhyItFailsClosed::TheTokenIsRefused)?;
    if names.the_routing_key() != my_routing_key {
        return Err(WhyItFailsClosed::NotThisMachinesKey);
    }
    let record = store
        .the_action_context(&names.the_action_context())
        .map_err(|_| WhyItFailsClosed::TheStoreFailed)?
        .ok_or(WhyItFailsClosed::NoRecord)?;
    if record.token != evidence.token {
        return Err(WhyItFailsClosed::TheTokenDoesNotMatch);
    }
    if record.conversation != evidence.conversation {
        return Err(WhyItFailsClosed::TheConversationDoesNotMatch);
    }
    if record.source_message != Some(evidence.source_message) {
        return Err(WhyItFailsClosed::TheSourceMessageDoesNotMatch);
    }
    if ![
        crate::job_messages::ASK_TO_STOP_THIS_JOB,
        crate::job_messages::STOP_AN_ENDED_JOB,
        crate::job_messages::STOP_THIS_JOB,
        crate::job_messages::KEEP_RENDERING,
    ]
    .contains(&record.action.as_str())
        && !the_generation_holds(&record, me, live)
    {
        return Err(WhyItFailsClosed::TheSubjectHasEnded);
    }
    let mut payload = record.payload.clone();
    // A record about no subject — V1d's ask — has no generation to compare.
    if let (Value::Object(fields), Some(_)) = (&mut payload, record.the_subject()) {
        fields.insert("generation".into(), json!(record.generation));
    }
    Ok(AnEntry {
        version: THE_PROTOCOL_VERSION,
        operation: entry.operation.clone(),
        addressee: TheAddressee::Machine(record.owning_machine.clone()),
        kind: AKindOfEntry::Action,
        subjects: record.job.iter().chain(record.hold.iter()).cloned().collect(),
        action: record.action.clone(),
        payload,
        source_update: entry.source_update,
        accepted_at: String::new(),
        deadline_seconds: THE_DEFAULT_REPORTING_DEADLINE.as_secs(),
    })
}

/// **`Telegram` R1a, widened 2026-10-31, and R1b — resolve one reply on the
/// machine that speaks for the replied-to message.** The action is the typed
/// request over the message's job and this machine, **never a record's stored
/// action**:
///
/// - **R1b's register first**: a line about a job names it, with the frame of
///   a picture. **This machine's status message (C7) is about the job it shows
///   now** ([`the_job_the_status_shows`]), and about none when it shows none.
/// - **Otherwise the action-context records bound to the message**, for a
///   message sent before the register — **leaving out a record whose Stop was
///   taken off at its job's end** (C1b's `stop_removed`), which no longer
///   speaks for the message. Where the rest do not agree on one job — R13's
///   disambiguation is such a message — nothing replaces the entry.
///
/// What it becomes: `frame` R14's request (the number typed, else the
/// picture's frame, else the newest); `stop` D0's confirmation; `reporting`
/// that job's change, or its ask. An ended job's stop or change is the owner's
/// to answer (*This job has already ended.*), so it resolves all the same.
pub fn resolve_a_reply(
    entry: &AnEntry,
    me: &str,
    store: &dyn TheTelegramWorkersState,
    holds: &dyn crate::routing::gather::ReadsWhatThisMachineHolds,
) -> Result<AnEntry, WhyItFailsClosed> {
    let evidence: AReplysEvidence =
        serde_json::from_value(entry.payload.clone()).map_err(|_| WhyItFailsClosed::TheEntryCannotBeRead)?;
    let jobs = holds.the_jobs();
    let (owner, job, generation, pictured, labels) = match store.the_sent_message(evidence.source_message) {
        Err(_) => return Err(WhyItFailsClosed::TheStoreFailed),
        Ok(Some(ASentMessage { conversation, .. })) if conversation != evidence.conversation => {
            return Err(WhyItFailsClosed::TheConversationDoesNotMatch)
        }
        Ok(Some(ASentMessage { about: WhatAMessageIsAbout::NoJob, .. })) => return Err(WhyItFailsClosed::NoRecord),
        Ok(Some(ASentMessage { about: WhatAMessageIsAbout::AJob { job, generation, frame, .. }, .. })) => {
            (me.to_string(), job, generation, frame, json!({}))
        }
        Ok(None) if store.the_status_message().ok().flatten() == Some(evidence.source_message) => {
            let shown = the_job_the_status_shows(&jobs).ok_or(WhyItFailsClosed::NoRecord)?;
            (me.to_string(), shown.job.clone(), Some(shown.generation.clone()), None, json!({}))
        }
        Ok(None) => {
            let records: Vec<AnActionContextRecord> = store
                .the_action_contexts_bound_to(evidence.source_message)
                .map_err(|_| WhyItFailsClosed::TheStoreFailed)?
                .into_iter()
                .filter(|r| r.conversation == evidence.conversation && r.payload["stop_removed"] != json!(true))
                .collect();
            let first = records.first().ok_or(WhyItFailsClosed::NoRecord)?;
            let (owner, job) = (first.owning_machine.clone(), first.job.clone());
            if records.iter().any(|r| r.owning_machine != owner || r.job != job) {
                return Err(WhyItFailsClosed::TheSourceMessageDoesNotMatch);
            }
            let Some(job) = job else {
                return Err(WhyItFailsClosed::NoRecord);
            };
            let labels = first.payload.get("labels").cloned().unwrap_or_else(|| json!({}));
            (owner, job, Some(first.generation.clone()), None, labels)
        }
    };
    // R7: the labels ride the instruction for what the user is told; the
    // owner's own reading of the job wins over a record's.
    let held = jobs.iter().find(|j| j.job == job).filter(|_| owner == me);
    let mut labels = labels;
    if let Some(held) = held {
        labels["label"] = json!(held.label);
    }
    // R1: the owner compares the generation on acting; this machine's own
    // reading is the one to carry where it owns the job.
    let generation = if owner == me { holds.the_live_generation(&job).or(generation) } else { generation }.unwrap_or_default();
    let operation = entry.operation.clone();
    Ok(match evidence.the_request() {
        // A job this machine no longer holds gets R14's I5 truth from the
        // owner's frame answer, as before.
        ARepliedRequest::Frame => {
            a_frame_request(&operation, &owner, &job, &generation, evidence.frame.or(pictured), entry.source_update, labels)
        }
        ARepliedRequest::Stop => an_action_on_one_job(
            &operation,
            &owner,
            &job,
            crate::job_messages::ASK_TO_STOP_THIS_JOB,
            json!({ "job": job, "generation": generation, "labels": labels }),
            entry.source_update,
        ),
        ARepliedRequest::Reporting { minutes } => an_action_on_one_job(
            &operation,
            &owner,
            &job,
            ONE_JOBS_REPORTING,
            json!({
                "job": job,
                "generation": generation,
                "labels": labels,
                "value": minutes.map(|minutes| crate::job_messages::TheReportingChoice::APictureEveryFewMinutes { minutes }.in_the_manifests_form()),
            }),
            entry.source_update,
        ),
    })
}

/// **`Telegram` C1 — the job this machine's status message shows now**: the
/// rendering job, else one held for the user's answer, held for the driver
/// update, or queued, in the status message's own order; none when idle.
pub fn the_job_the_status_shows(jobs: &[crate::routing::gather::AJobAsHeld]) -> Option<&crate::routing::gather::AJobAsHeld> {
    use crate::routing::gather::AJobWaits;
    jobs.iter().find(|job| job.rendering.is_some()).or_else(|| {
        [AJobWaits::OnYourAnswer, AJobWaits::ForTheDriverUpdate, AJobWaits::Queued]
            .into_iter()
            .find_map(|waits| jobs.iter().find(|job| job.waiting == Some(waits)))
    })
}

/// **`Telegram` R1, on the owning machine — whether the subject an instruction
/// names is still live in the generation it carries.** An instruction whose
/// generation has ended is acted on by nothing (R1: *mismatched generation is
/// rejected and nothing is submitted*); what the owner then tells the user is
/// the owning act's, `Telegram` I5 for a frame.
pub fn the_owner_finds_the_subject_live(entry: &AnEntry, live: &dyn Fn(&str) -> Option<String>) -> bool {
    let Some(generation) = entry.payload.get("generation").and_then(Value::as_str) else {
        return true;
    };
    entry.subjects.iter().any(|subject| {
        let current = live(subject);
        current.as_deref() == Some(generation)
            || ([
                crate::job_messages::STOP_THIS_JOB,
                crate::job_messages::ASK_TO_STOP_THIS_JOB,
            ]
            .contains(&entry.action.as_str())
                && entry.payload["confirmed"] != json!(true)
                && [format!("{generation}:0"), format!("{generation}:1")]
                    .iter()
                    .any(|old| current.as_ref() == Some(old)))
    })
}

/// **One store, reached from two places in one role worker** — the listener,
/// and the routing that issues and resolves records. F2 has one store; this is
/// how two holders of it share it without a second copy.
impl<S: TheTelegramWorkersState + ?Sized> TheTelegramWorkersState for Arc<Mutex<Box<S>>> {
    fn the_update_position(&self) -> Result<Option<i64>, WhyTheStoreFailed> {
        held(self).the_update_position()
    }
    fn record_the_update_position(&mut self, next: i64) -> Result<(), WhyTheStoreFailed> {
        held(self).record_the_update_position(next)
    }
    fn the_board(&self) -> Result<Option<i64>, WhyTheStoreFailed> {
        held(self).the_board()
    }
    fn record_the_board(&mut self, message: i64) -> Result<(), WhyTheStoreFailed> {
        held(self).record_the_board(message)
    }
    fn the_machinery_message(&self) -> Result<Option<i64>, WhyTheStoreFailed> {
        held(self).the_machinery_message()
    }
    fn record_the_machinery_message(&mut self, message: i64) -> Result<(), WhyTheStoreFailed> {
        held(self).record_the_machinery_message(message)
    }
    fn the_routing_key(&self) -> Result<String, WhyTheStoreFailed> {
        held(self).the_routing_key()
    }
    fn holds_a_context_record_bound_to(&self, source_message: i64) -> Result<bool, WhyTheStoreFailed> {
        held(self).holds_a_context_record_bound_to(source_message)
    }
    fn record_an_action_context(&mut self, record: AnActionContextRecord) -> Result<(), WhyTheStoreFailed> {
        held(self).record_an_action_context(record)
    }
    fn the_action_context(&self, identity: &str) -> Result<Option<AnActionContextRecord>, WhyTheStoreFailed> {
        held(self).the_action_context(identity)
    }
    fn the_action_contexts(&self) -> Result<Vec<AnActionContextRecord>, WhyTheStoreFailed> {
        held(self).the_action_contexts()
    }
    fn the_action_contexts_bound_to(&self, source_message: i64) -> Result<Vec<AnActionContextRecord>, WhyTheStoreFailed> {
        held(self).the_action_contexts_bound_to(source_message)
    }
    fn remove_action_contexts_where(
        &mut self,
        which: &dyn Fn(&AnActionContextRecord) -> bool,
    ) -> Result<usize, WhyTheStoreFailed> {
        held(self).remove_action_contexts_where(which)
    }
    fn the_journal_cursor(&self) -> Result<Option<u64>, WhyTheStoreFailed> {
        held(self).the_journal_cursor()
    }
    fn record_the_journal_cursor(&mut self, next: u64) -> Result<(), WhyTheStoreFailed> {
        held(self).record_the_journal_cursor(next)
    }
    fn the_status_message(&self) -> Result<Option<i64>, WhyTheStoreFailed> {
        held(self).the_status_message()
    }
    fn record_the_status_message(&mut self, message: i64) -> Result<(), WhyTheStoreFailed> {
        held(self).record_the_status_message(message)
    }
    fn register_a_sent_message(&mut self, line: ASentMessage) -> Result<(), WhyTheStoreFailed> {
        held(self).register_a_sent_message(line)
    }
    fn the_sent_message(&self, message: i64) -> Result<Option<ASentMessage>, WhyTheStoreFailed> {
        held(self).the_sent_message(message)
    }
    fn the_sent_messages(&self) -> Result<Vec<ASentMessage>, WhyTheStoreFailed> {
        held(self).the_sent_messages()
    }
    fn forget_sent_messages_where(&mut self, which: &dyn Fn(&ASentMessage) -> bool) -> Result<usize, WhyTheStoreFailed> {
        held(self).forget_sent_messages_where(which)
    }
    fn the_phones_choice(&self) -> Result<Option<ThePhonesChoice>, WhyTheStoreFailed> {
        held(self).the_phones_choice()
    }
    fn record_the_phones_choice(&mut self, choice: ThePhonesChoice) -> Result<bool, WhyTheStoreFailed> {
        held(self).record_the_phones_choice(choice)
    }
    fn speaks_for_the_message(&self, message: i64) -> bool {
        held(self).speaks_for_the_message(message)
    }
}

fn held<S: ?Sized>(store: &Arc<Mutex<Box<S>>>) -> std::sync::MutexGuard<'_, Box<S>> {
    store.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::worker_state::InMemory;

    const KEY: &str = "00112233445566778899aabbccddeeff";
    const CHAT: i64 = 9001;

    fn a_record(owner: &str) -> AnActionContextRecord {
        let mut record =
            AnActionContextRecord::issued_under(KEY, CHAT, owner, Some("job-1"), None, "stop", json!({}), "g1").unwrap();
        record.source_message = Some(77);
        record
    }

    fn a_tap(record: &AnActionContextRecord, conversation: i64, message: i64) -> AnEntry {
        let evidence = ATapsEvidence { token: record.token.clone(), callback: "cb-1".into(), conversation, source_message: message, label: None };
        a_taps_resolution(&evidence, 500, &WhatATokenNames::read(&record.token).unwrap())
    }

    fn live_in(generation: &'static str) -> impl Fn(&str) -> Option<String> {
        move |subject: &str| (subject == "job-1").then(|| generation.to_string())
    }

    /// **`Telegram` R1: the token names the key and the record, the entry is
    /// addressed by the key alone, and a match gives the full instruction to
    /// the owner under the same operation.**
    #[test]
    fn a_tap_resolves_to_the_owner_under_the_same_operation() {
        let mut store = InMemory::with_routing_key(KEY);
        let record = a_record("m1");
        store.record_an_action_context(record.clone()).unwrap();
        let entry = a_tap(&record, CHAT, 77);
        assert_eq!(entry.addressee, TheAddressee::RoutingKey(KEY.into()));
        assert_eq!(entry.operation, "tap-cb-1");
        assert_eq!(entry.deadline_seconds, 10);
        let full = resolve_a_tap(&entry, "m1", KEY, &store, &live_in("g1")).unwrap();
        assert_eq!(full.operation, entry.operation);
        assert_eq!(full.kind, AKindOfEntry::Action);
        assert_eq!(full.addressee, TheAddressee::Machine("m1".into()));
        assert_eq!(full.action, "stop");
        assert_eq!(full.subjects, vec!["job-1".to_string()]);
        // Nothing but the two identities is in the token.
        assert!(!record.token.contains("stop") && !record.token.contains("job"));
    }

    /// **`Telegram` R1: resolution fails closed**, one case at a time.
    #[test]
    fn every_mismatch_is_rejected() {
        let mut store = InMemory::with_routing_key(KEY);
        let record = a_record("m1");
        let entry = a_tap(&record, CHAT, 77);
        assert_eq!(resolve_a_tap(&entry, "m1", KEY, &store, &live_in("g1")), Err(WhyItFailsClosed::NoRecord));
        store.record_an_action_context(record.clone()).unwrap();
        let other_key = "ffeeddccbbaa99887766554433221100";
        assert_eq!(resolve_a_tap(&entry, "m1", other_key, &store, &live_in("g1")), Err(WhyItFailsClosed::NotThisMachinesKey));
        assert_eq!(
            resolve_a_tap(&a_tap(&record, 42, 77), "m1", KEY, &store, &live_in("g1")),
            Err(WhyItFailsClosed::TheConversationDoesNotMatch)
        );
        assert_eq!(
            resolve_a_tap(&a_tap(&record, CHAT, 78), "m1", KEY, &store, &live_in("g1")),
            Err(WhyItFailsClosed::TheSourceMessageDoesNotMatch)
        );
        assert_eq!(resolve_a_tap(&entry, "m1", KEY, &store, &live_in("g2")), Err(WhyItFailsClosed::TheSubjectHasEnded));
        let mut forged = entry.clone();
        forged.payload["token"] = json!("not a token");
        assert_eq!(resolve_a_tap(&forged, "m1", KEY, &store, &live_in("g1")), Err(WhyItFailsClosed::TheTokenIsRefused));
        // A record removed when its subject ends resolves nothing again.
        assert_eq!(store.remove_action_contexts_where(&|r| r.job.as_deref() == Some("job-1")).unwrap(), 1);
        assert_eq!(resolve_a_tap(&entry, "m1", KEY, &store, &live_in("g1")), Err(WhyItFailsClosed::NoRecord));
    }

    /// **`Telegram` R4: two presses are two instructions.**
    #[test]
    fn two_presses_are_two_operations() {
        let record = a_record("m1");
        let names = WhatATokenNames::read(&record.token).unwrap();
        let one = ATapsEvidence { token: record.token.clone(), callback: "a".into(), conversation: CHAT, source_message: 77, label: None };
        let two = ATapsEvidence { callback: "b".into(), ..one.clone() };
        assert_ne!(a_taps_resolution(&one, 1, &names).operation, a_taps_resolution(&two, 2, &names).operation);
        assert_eq!(a_taps_resolution(&one, 1, &names).operation, a_taps_resolution(&one, 1, &names).operation);
    }

    /// **The generation check where the issuer is not the owner** rides the
    /// instruction to the owner.
    #[test]
    fn an_issuer_that_is_not_the_owner_passes_the_generation_on() {
        let mut store = InMemory::with_routing_key(KEY);
        let record = a_record("m2");
        store.record_an_action_context(record.clone()).unwrap();
        let full = resolve_a_tap(&a_tap(&record, CHAT, 77), "m1", KEY, &store, &|_| None).unwrap();
        assert_eq!(full.payload["generation"], json!("g1"));
        assert!(the_owner_finds_the_subject_live(&full, &live_in("g1")));
        assert!(!the_owner_finds_the_subject_live(&full, &live_in("g2")));
        assert!(!the_owner_finds_the_subject_live(&full, &|_| None));
    }

    /// **What a machine holds, for the reply tests**: jobs by identity, each
    /// rendering or ended, in generation `g1`.
    struct Holding(Vec<(&'static str, bool)>);
    impl crate::routing::gather::ReadsWhatThisMachineHolds for Holding {
        fn the_jobs(&self) -> Vec<crate::routing::gather::AJobAsHeld> {
            self.0
                .iter()
                .map(|(job, rendering)| crate::routing::gather::AJobAsHeld {
                    job: job.to_string(),
                    label: format!("{job} label"),
                    generation: "g1".into(),
                    record_directory: true,
                    rendering: rendering.then_some(crate::routing::gather::TheProgress { frame: 3, of: 9, percent: 33 }),
                    waiting: None,
                })
                .collect()
        }
        fn the_live_generation(&self, subject: &str) -> Option<String> {
            self.0.iter().any(|(job, rendering)| *job == subject && *rendering).then(|| "g1".to_string())
        }
    }

    fn a_reply(source_message: i64, frame: Option<u64>, request: Option<ARepliedRequest>, update: i64) -> AnEntry {
        a_replys_resolution(&AReplysEvidence { conversation: CHAT, source_message, frame, request }, update)
    }

    /// **`Telegram` R1a: a reply resolves only where the bound records agree on
    /// one subject, and to the typed request, never the stored action.**
    #[test]
    fn a_reply_resolves_by_the_message_and_only_to_one_subject() {
        let mut store = InMemory::with_routing_key(KEY);
        let record = a_record("m1");
        store.record_an_action_context(record.clone()).unwrap();
        assert!(store.holds_a_context_record_bound_to(77).unwrap());
        let entry = a_reply(77, Some(120), None, 600);
        assert_eq!(entry.addressee, TheAddressee::SourceMessage(77));
        let holds = Holding(vec![("job-1", true)]);
        let full = resolve_a_reply(&entry, "m1", &store, &holds).unwrap();
        assert_eq!(full.action, SENDING_A_FRAME);
        assert_eq!(full.payload["frame"], json!(120));
        assert_eq!(full.operation, "reply-600");

        let mut other = a_record("m1");
        other.job = Some("job-2".into());
        store.record_an_action_context(other).unwrap();
        assert_eq!(resolve_a_reply(&entry, "m1", &store, &holds), Err(WhyItFailsClosed::TheSourceMessageDoesNotMatch));
        let elsewhere = a_reply(5, Some(120), None, 601);
        assert_eq!(resolve_a_reply(&elsewhere, "m1", &store, &holds), Err(WhyItFailsClosed::NoRecord));
    }

    /// **R1a, widened: only a numbered frame request carries `frame`**, so an
    /// older machine, which reads the evidence with `frame` required, cannot
    /// read a stop, a reporting change or the newest frame at all.
    #[test]
    fn an_older_machine_cannot_half_read_a_stop_or_a_change_as_a_frame() {
        #[derive(Deserialize)]
        #[allow(dead_code)]
        struct TheOlderEvidence {
            conversation: i64,
            source_message: i64,
            frame: u64,
        }
        let older = |entry: AnEntry| serde_json::from_value::<TheOlderEvidence>(entry.payload).is_ok();
        assert!(older(a_reply(77, Some(4), Some(ARepliedRequest::Frame), 1)));
        assert!(!older(a_reply(77, None, Some(ARepliedRequest::Frame), 2)));
        assert!(!older(a_reply(77, None, Some(ARepliedRequest::Stop), 3)));
        assert!(!older(a_reply(77, None, Some(ARepliedRequest::Reporting { minutes: Some(10) }), 4)));
        // And an older listener's entry still reads here, as a numbered frame.
        let theirs = json!({ "conversation": CHAT, "source_message": 77, "frame": 9 });
        let read: AReplysEvidence = serde_json::from_value(theirs).unwrap();
        assert_eq!((read.the_request(), read.frame), (ARepliedRequest::Frame, Some(9)));
    }

    /// **R1a, widened, and R1b: a job's message, by its register line** — a
    /// reply's `frame`, `stop` and `reporting` each act on that job; a picture
    /// gives its own frame; a line about no job claims nothing.
    #[test]
    fn a_reply_to_a_registered_message_acts_on_its_job() {
        let mut store = InMemory::with_routing_key(KEY);
        let holds = Holding(vec![("job-1", true), ("job-2", false)]);
        let line = |message: i64, about: WhatAMessageIsAbout| ASentMessage { message, conversation: CHAT, about };
        store.register_a_sent_message(line(80, WhatAMessageIsAbout::AJob { job: "job-1".into(), generation: Some("g1".into()), frame: Some(42), layer: Some("Beauty".into()) })).unwrap();
        store.register_a_sent_message(line(81, WhatAMessageIsAbout::AJob { job: "job-2".into(), generation: Some("g0".into()), frame: None, layer: None })).unwrap();
        store.register_a_sent_message(line(82, WhatAMessageIsAbout::NoJob)).unwrap();

        let picture = resolve_a_reply(&a_reply(80, None, Some(ARepliedRequest::Frame), 1), "m1", &store, &holds).unwrap();
        assert_eq!((picture.action.as_str(), &picture.payload["frame"]), (SENDING_A_FRAME, &json!(42)), "a reply to a picture sends that frame");
        let numbered = resolve_a_reply(&a_reply(80, Some(7), Some(ARepliedRequest::Frame), 2), "m1", &store, &holds).unwrap();
        assert_eq!(numbered.payload["frame"], json!(7), "a number typed wins over the picture's");
        let newest = resolve_a_reply(&a_reply(81, None, Some(ARepliedRequest::Frame), 3), "m1", &store, &holds).unwrap();
        assert_eq!(newest.action, SENDING_THE_NEWEST_FRAME);
        assert_eq!(newest.subjects, vec!["job-2".to_string()]);

        let stop = resolve_a_reply(&a_reply(80, None, Some(ARepliedRequest::Stop), 4), "m1", &store, &holds).unwrap();
        assert_eq!(stop.action, crate::job_messages::ASK_TO_STOP_THIS_JOB, "straight to D0's confirmation");
        assert_eq!((stop.addressee.clone(), stop.payload["generation"].clone()), (TheAddressee::Machine("m1".into()), json!("g1")));
        let ended = resolve_a_reply(&a_reply(81, None, Some(ARepliedRequest::Stop), 5), "m1", &store, &holds).unwrap();
        assert_eq!(ended.subjects, vec!["job-2".to_string()], "an ended job's stop is the owner's to answer");

        let change = resolve_a_reply(&a_reply(80, None, Some(ARepliedRequest::Reporting { minutes: Some(10) }), 6), "m1", &store, &holds).unwrap();
        assert_eq!(change.action, ONE_JOBS_REPORTING);
        assert_eq!(change.payload["value"], json!({ "APictureEveryFewMinutes": { "minutes": 10 } }));
        let ask = resolve_a_reply(&a_reply(80, None, Some(ARepliedRequest::Reporting { minutes: None }), 7), "m1", &store, &holds).unwrap();
        assert_eq!(ask.payload["value"], Value::Null, "no value: that job's own ask");

        assert_eq!(resolve_a_reply(&a_reply(82, None, Some(ARepliedRequest::Stop), 8), "m1", &store, &holds), Err(WhyItFailsClosed::NoRecord));
        assert_eq!(resolve_a_reply(&a_reply(99, None, Some(ARepliedRequest::Stop), 9), "m1", &store, &holds), Err(WhyItFailsClosed::NoRecord));
    }

    /// **The replies exploration's second fault, reproduced: a status message
    /// that has shown two jobs.** C1 re-uses the one status message across
    /// jobs, and each job leaves its Stop record on it. **Before this change
    /// the records disagreed and nothing was resolved** — exactly the refusal
    /// `a_reply_resolves_by_the_message_and_only_to_one_subject` asserts for
    /// two jobs' records on one message, which was the code at `d3abb7ca7` —
    /// so every reply to such a status message cost the deadline. **Now the
    /// status message is about the job it shows now.**
    #[test]
    fn a_reply_to_a_status_message_that_has_shown_two_jobs_is_about_the_one_it_shows() {
        let mut store = InMemory::with_routing_key(KEY);
        store.record_the_status_message(77).unwrap();
        let mut first = a_record("m1");
        first.action = crate::job_messages::ASK_TO_STOP_THIS_JOB.into();
        first.payload = json!({ "stop_removed": true });
        let mut second = a_record("m1");
        second.job = Some("job-2".into());
        store.record_an_action_context(first).unwrap();
        store.record_an_action_context(second).unwrap();
        let holds = Holding(vec![("job-1", false), ("job-2", true)]);
        let full = resolve_a_reply(&a_reply(77, None, Some(ARepliedRequest::Frame), 1), "m1", &store, &holds).unwrap();
        assert_eq!(full.subjects, vec!["job-2".to_string()]);
        // Idle, the status message is about no job.
        let idle = Holding(vec![("job-1", false), ("job-2", false)]);
        assert_eq!(resolve_a_reply(&a_reply(77, None, Some(ARepliedRequest::Stop), 2), "m1", &store, &idle), Err(WhyItFailsClosed::NoRecord));
        // And an older message with no line: the record whose Stop was taken
        // off no longer speaks for it.
        let mut older = InMemory::with_routing_key(KEY);
        let mut gone = a_record("m1");
        gone.payload = json!({ "stop_removed": true });
        let mut kept = a_record("m1");
        kept.job = Some("job-2".into());
        older.record_an_action_context(gone).unwrap();
        older.record_an_action_context(kept).unwrap();
        let full = resolve_a_reply(&a_reply(77, Some(3), None, 3), "m1", &older, &holds).unwrap();
        assert_eq!(full.subjects, vec!["job-2".to_string()]);
    }

    #[test]
    fn a_shared_store_is_one_store() {
        let shared: Arc<Mutex<Box<dyn TheTelegramWorkersState>>> =
            Arc::new(Mutex::new(Box::new(InMemory::with_routing_key(KEY))));
        let mut one = shared.clone();
        one.record_an_action_context(a_record("m1")).unwrap();
        assert!(shared.holds_a_context_record_bound_to(77).unwrap());
    }
}

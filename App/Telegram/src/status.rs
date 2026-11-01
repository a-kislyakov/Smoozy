//! **The status message and its clock — `Telegram` §10 C1, C2, C2a, C3, C4,
//! C5 and C7 — and the footing every piece-6 rule stands on, §2 F1 to F5.**
//!
//! **Written as rules over values.** What a machine is doing, its readings and
//! its kind are handed in; this file composes the message, keeps its one
//! identity, and paces it. **Taking the readings on a machine, reading the note
//! and the journal, and running the clock with the app closed are step 10c's**
//! (`Audits/Build/Step 10/Telegram — what each piece owns and in what order.md`
//! §8 items 3, 4 and 6, which named step 8b until that step's own correction
//! moved its machine half to step 10c; corrected here 2026-10-16). A site
//! below that keeps a rule and not the act says so.
//!
//! **The footing, `Telegram` §2, cited and kept as the rules that bind the
//! worker:**
//!
//! - **F1** — the worker is the supervisor-hosted role worker; it reads the
//!   token from its file at start and never hands it to a child. *Rule kept
//!   here*: nothing in this crate starts a child. *Act owed*: the worker mode
//!   itself, step 10c — built 2026-10-16 by the second piece of the Telegram
//!   worker's map.
//! - **F2** — the worker writes no durable store itself. *Kept*: the journal
//!   cursor and the status-message identity are reached only through
//!   [`TheTelegramWorkersState`], whose real store is the supervisor's.
//! - **F3** — both cursors advance after the work is done and never on
//!   receipt. *Kept* for pictures at [`crate::pictures::the_work_is_done`];
//!   the journal reading itself is step 10c's.
//! - **F4** — the footing holds at the promise level (Fact 62, Fact 1). An
//!   observation; nothing to build.
//! - **F5** — on a full state mount the supervisor does not run, so nothing is
//!   edited and the user reads C5's silence. *Kept*: nothing here sends news of
//!   a machine that stopped.
//!
//! **C4 — the clock runs on the machine with nobody connected.** *Rule kept*:
//! nothing in this file reaches the desktop or needs a connection. *Act owed*:
//! the clock on a machine with the app closed, step 10c.
//!
//! **C5 — the silence is the mechanism.** When a machine dies, is cut off or
//! cannot start, its status message simply stops changing and the last
//! written moment is what the user reads. **Nothing here ever sends that news**:
//! a send that reached nothing is not retried and not reported
//! ([`TheClock::take_in_what_ended`]).
//!
//! **B16 — the status message is never pinned.** No call in this file pins,
//! and a test fails if one does.

use crate::job_messages::{AServerLabel, TheServersName};
use crate::job_messages::{AMarkedText, a_user_message};
use std::sync::mpsc::{Receiver, TryRecvError};
use std::time::{Duration, SystemTime};

use serde_json::{json, Value};

use crate::actions::{a_stop_button_on, TheSubject};
use crate::board::text::the_moment_in_utc;
use crate::pacing::{AClassOfSend, HowASendEnded, TheSendQueue};
use crate::transport::{TheClient, WhyTheCallFailed};
use crate::worker_state::{TheTelegramWorkersState, WhyTheStoreFailed};

/// **`Telegram` C2 and C3 — the base interval, 5 seconds**, the Creator's
/// ruling of 2026-08-19. **C3's label**: it rests on a burst — forty edits in
/// 9.3 seconds — and not on a day; a day at this rate is §21 item 4's.
pub const THE_BASE_INTERVAL: Duration = Duration::from_secs(5);

/// **`Telegram` C2 — the interval is 5 seconds times the number of enabled
/// machines**, so the fleet's status traffic stays at 0.2 edits a second
/// whatever its size (the Creator's ruling of 2026-08-25). A count of nought is
/// read as one: a machine running this clock is itself enabled.
pub fn the_interval_for(enabled_machines: usize) -> Duration {
    THE_BASE_INTERVAL.saturating_mul(u32::try_from(enabled_machines.max(1)).unwrap_or(u32::MAX))
}

/// **`Rented-server runtime` K1's two kinds a role worker ever runs on**,
/// detected there and never here (`Telegram` C1's second bullet).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheMachineKind {
    AContainer,
    ASystemdMachine,
}

/// **`Telegram` C1 — one reading, with the moment it was taken on the
/// machine's own clock, or said to be unavailable.** Never carried forward
/// from an earlier tick (C1's fifth bullet): each tick hands in fresh ones.
#[derive(Debug, Clone, PartialEq)]
pub enum AReading<T> {
    Taken { value: T, at: SystemTime },
    Unavailable,
}

/// The card's memory, as `Rented-server runtime` V6's mechanism reads it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CardMemory {
    pub used_gb: f64,
    pub total_gb: f64,
}

/// **`Telegram` C1's second bullet — how long the machine has been up.**
#[derive(Debug, Clone, PartialEq)]
pub enum TheUptime {
    /// On a container the machine's own uptime cannot be read, so no reading
    /// is taken and the host's figure is never shown.
    NotReadOnAContainer,
    Read(AReading<Duration>),
}

impl TheUptime {
    /// **The reading is taken only where the kind allows it.** On a container
    /// `read` is never called — *"No uptime reading is taken on that kind"*.
    pub fn on(kind: TheMachineKind, read: impl FnOnce() -> AReading<Duration>) -> TheUptime {
        match kind {
            TheMachineKind::AContainer => TheUptime::NotReadOnAContainer,
            TheMachineKind::ASystemdMachine => TheUptime::Read(read()),
        }
    }
}

/// **What the machine is doing** — the note's and the journal's facts, which
/// `Queue & History` owns and this module only carries to a phone (C1).
#[derive(Debug, Clone, PartialEq)]
pub enum WhatTheMachineIsDoing {
    /// V1b: machine readings only, without a job or an idle claim.
    WorkNotReported,
    Rendering {
        /// What the job is called (`Telegram` R7, `Telegram` R7a): the note's
        /// label whole (`Queue & History` N1), or the `.blend`'s name where
        /// there is none.
        job_label: String,
        /// What the design writes after the label, such as the scene.
        detail: Option<String>,
        frame: u64,
        of: u64,
        percent: u8,
        /// The design's *This frame* line, where the note has one.
        this_frame: Option<String>,
    },
    /// **`Telegram` C1: a rendering job a stop's interrupt has gone out on**,
    /// until the render ends. Blender goes on through the range after the
    /// first interrupt and saves nothing, so no frame or per cent is said; the
    /// job is still named, and *Stop this job* still rides the message
    /// (`Rendering` K10's second stop). Added 2026-10-19, the live pass's
    /// faults, piece E.
    Stopping { job_label: String },
    /// **A job held for the user's answer** — the first-frame check's second
    /// job (`Rented-server runtime` §8). Nothing renders while money is spent,
    /// so the message must never say *idle* (C1: the note's facts).
    WaitingOnYou { job_label: String },
    /// **A job held until the server's NVIDIA driver is updated** (`Rendering`
    /// S18a, `Server Setup` R1c). Nothing waits on the user, so it reads as
    /// its queue entry does and **never *waiting on you*** (`Telegram` A3,
    /// `Queue & History` H12). Held-job piece B, 2026-10-12.
    WaitingForTheDriverUpdate { job_label: String },
    /// **A job in the line, with nothing rendering yet.**
    Queued { job_label: String },
    /// Nothing rendering and nothing waiting; how long for is `Queue &
    /// History` W1 to W3's, computed on the machine, where it is known.
    NothingRendering { quiet_for: Option<Duration> },
}

impl WhatTheMachineIsDoing {
    /// The job the message names, if any — the one a stop on it would stop.
    fn the_job_label(&self) -> Option<&str> {
        match self {
            WhatTheMachineIsDoing::Rendering { job_label, .. }
            | WhatTheMachineIsDoing::Stopping { job_label }
            | WhatTheMachineIsDoing::WaitingOnYou { job_label }
            | WhatTheMachineIsDoing::WaitingForTheDriverUpdate { job_label }
            | WhatTheMachineIsDoing::Queued { job_label } => Some(job_label),
            WhatTheMachineIsDoing::NothingRendering { .. } | WhatTheMachineIsDoing::WorkNotReported => None,
        }
    }
}

/// **Everything one tick's message says.**
#[derive(Debug, Clone, PartialEq)]
pub struct TheStatus {
    /// The label this machine was last told (`Telegram` R7, R8).
    pub server_label: AServerLabel,
    pub doing: WhatTheMachineIsDoing,
    pub card_memory: AReading<CardMemory>,
    pub disk_free_gb: AReading<f64>,
    pub uptime: TheUptime,
}

/// **`Telegram` C1c — *Send a frame*, for the status message**: its record
/// written against the message first (R1), binding the job and attempt and
/// the action *the newest frame*, answered by R14 on this machine.
fn a_frame_button_on(
    store: &mut dyn TheTelegramWorkersState,
    conversation: i64,
    message: i64,
    subject: &TheSubject,
    server: &AServerLabel,
    label: &str,
) -> Option<Value> {
    let key = store.the_routing_key().ok()?;
    let mut record = crate::routing::context::AnActionContextRecord::issued_under(
        &key,
        conversation,
        &subject.owning_machine,
        subject.job.as_deref(),
        None,
        crate::routing::context::SENDING_THE_NEWEST_FRAME,
        json!({ "job": subject.job, "labels": { "server": server, "label": label, "button": "Send a frame" } }),
        &subject.generation,
    )?;
    record.source_message = Some(message);
    let token = record.token.clone();
    store.record_an_action_context(record).ok()?;
    Some(json!({ "text": "Send a frame", "callback_data": token }))
}

/// **`Telegram` C1, C2 and C2a — the message's text.** The words are the
/// design's (`Preparation/Design/Mockup/Smoozy.html`, `#tgview`, the kept-current
/// message): the server's label, the job, *Rendering*, *This frame*, *GPU
/// memory*, *Disk free*, *Uptime* (*Using* and *Up* until step 13's words
/// sweep), and **the moment written into the text as a complete UTC date and
/// 24-hour time with seconds, `UTC` said** (C2a). No time zone is converted and
/// nothing is subtracted across two clocks (C2; `State, identity and recovery` D5).
///
/// **A reading taken at a moment other than the tick's says its own moment**
/// beside it, because C1 has every reading shown with the moment it was taken.
pub fn compose(status: &TheStatus, now: SystemTime) -> AMarkedText {
    let updated = the_moment_in_utc(now);
    let mut lines = vec![status.server_label.to_string()];
    match &status.doing {
        WhatTheMachineIsDoing::WorkNotReported => {},
        WhatTheMachineIsDoing::Rendering { job_label, detail, frame, of, percent, this_frame } => {
            lines.push(match detail {
                Some(detail) => format!("{job_label} \u{b7} {detail}"),
                None => job_label.clone(),
            });
            // **A render whose worker has reported no figure yet, or whose
            // frames are not known, is said to be rendering and nothing
            // more** — `Queue & History` R4: absent, never a nought — the
            // rule the gather's line keeps. **And a job whose reporting
            // choice leaves out the running commentary** (`Telegram` V1) is
            // handed in the same way, so its progress is never said. Added
            // 2026-10-16, the Telegram worker's piece 6.
            if *frame == 0 || *of == 0 {
                lines.push("Rendering.".to_string());
            } else {
                lines.push(format!("Rendering: frame {frame} of {of} \u{b7} {percent}%"));
            }
            if let Some(this_frame) = this_frame {
                lines.push(format!("This frame: {this_frame}"));
            }
        }
        // **`Telegram` C1's own words**, in place of the frame line.
        WhatTheMachineIsDoing::Stopping { job_label } => {
            lines.push(job_label.clone());
            lines.push("Stopping.".to_string());
        }
        // *Needs your answer* is the drawing's phrase (`tg-check`; *Waiting on
        // you* until step 13's final words sweep); the rest are
        // standard words Design may replace. Added by the round-1 code audit of
        // the bot and the chat, fix-here item 5.
        WhatTheMachineIsDoing::WaitingOnYou { job_label } => {
            lines.push(job_label.clone());
            lines.push("Needs your answer. Nothing is rendering.".to_string());
        }
        // **The queue entry's own words** (`Design/Queue & History § A job
        // held for the driver update`): Design gives the phone none of its
        // own, and until it does the entry's stand (`Telegram` A3).
        WhatTheMachineIsDoing::WaitingForTheDriverUpdate { job_label } => {
            lines.push(job_label.clone());
            lines.push("Waiting for driver update.".to_string());
        }
        WhatTheMachineIsDoing::Queued { job_label } => {
            lines.push(job_label.clone());
            lines.push("Queued.".to_string());
        }
        WhatTheMachineIsDoing::NothingRendering { quiet_for } => lines.push(match quiet_for {
            Some(quiet_for) => format!("Idle for {}.", a_duration(*quiet_for)),
            None => "Idle.".to_string(),
        }),
    }
    let card = match &status.card_memory {
        AReading::Taken { value, at } => {
            with_its_moment(format!("{} of {} GB", gb(value.used_gb), gb(value.total_gb)), *at, &updated)
        }
        AReading::Unavailable => "unavailable".to_string(),
    };
    let disk = match &status.disk_free_gb {
        AReading::Taken { value, at } => {
            with_its_moment(format!("{} GB", value.max(0.0).floor() as u64), *at, &updated)
        }
        AReading::Unavailable => "unavailable".to_string(),
    };
    lines.push(format!("GPU memory: {card} \u{b7} Disk free: {disk}"));
    lines.push(format!(
        "Uptime: {}",
        match &status.uptime {
            TheUptime::NotReadOnAContainer => "unavailable".to_string(),
            TheUptime::Read(AReading::Taken { value, at }) => with_its_moment(a_duration(*value), *at, &updated),
            TheUptime::Read(AReading::Unavailable) => "unavailable".to_string(),
        }
    ));
    lines.push(format!("Updated {updated}"));
    let mut out = a_user_message(&status.server_label, "", "", "");
    for (at, line) in lines.iter().enumerate().skip(1) {
        out.append("\n", None);
        if at == 1 {
            if let Some(job) = status.doing.the_job_label() {
                out.append(job, Some("italic"));
                out.append(&line[job.len()..], None);
                continue;
            }
        }
        out.append(line, None);
    }
    out
}

/// **`Telegram` C7 — what an old status message is edited to say** once a new
/// one has had to be created. Rule 9 binds it; the drawing has no such state.
pub fn the_superseded_text(server_label: &(impl TheServersName + ?Sized)) -> AMarkedText {
    a_user_message(server_label, "", "", "No longer updated. See the newer status message.")
}

fn with_its_moment(text: String, at: SystemTime, updated: &str) -> String {
    let moment = the_moment_in_utc(at);
    if moment == updated {
        text
    } else {
        format!("{text} (read {moment})")
    }
}

fn gb(value: f64) -> String {
    let rounded = format!("{:.1}", value.max(0.0));
    rounded.strip_suffix(".0").map(str::to_string).unwrap_or(rounded)
}

/// *4 hours 12 minutes*, *30 minutes*, *2 days 3 hours*, *40 seconds* — the
/// design's form, the two largest units that are not nought.
fn a_duration(duration: Duration) -> String {
    let seconds = duration.as_secs();
    let units = [(seconds / 86_400, "day"), (seconds % 86_400 / 3600, "hour"), (seconds % 3600 / 60, "minute")];
    let named: Vec<String> = units
        .iter()
        .skip_while(|(n, _)| *n == 0)
        .take(2)
        .filter(|(n, _)| *n > 0)
        .map(|(n, unit)| format!("{n} {unit}{}", if *n == 1 { "" } else { "s" }))
        .collect();
    if named.is_empty() {
        let s = seconds % 60;
        format!("{s} second{}", if s == 1 { "" } else { "s" })
    } else {
        named.join(" ")
    }
}

/// What the clock last offered, so its ending can be read.
#[derive(Debug, Clone, Copy, PartialEq)]
enum WhatWasOffered {
    ACreation { replacing: Option<i64> },
    AnEdit(i64),
    MarkingSuperseded(i64),
}

/// **`Telegram` C6 and C7 — one machine's status message on its clock.**
///
/// **Every send goes through the one queue in the clock's class**, so at most
/// one edit is outstanding and a refused tick is dropped rather than re-sent
/// (piece 1's [`TheSendQueue`]). **C7**: the message's identity is read from
/// and written to the worker's state, so a restart re-uses it; a new message is
/// created only where the recorded one could not be edited, and the old one is
/// then edited to say it is superseded.
///
/// **B24**: the message is for the user, so its creation notifies and it
/// carries no spoiler. **B16**: it is never pinned.
#[derive(Debug)]
pub struct TheClock {
    chat: i64,
    outstanding: Option<(WhatWasOffered, Receiver<HowASendEnded>)>,
    could_not_be_edited: Option<i64>,
    to_mark_superseded: Option<i64>,
    /// The stop last written for a message, and the job it stops, so every
    /// edit carries the same button under one record. Held in memory: a restart
    /// writes one more record, never a button without one.
    the_stop: Option<TheStopOnAMessage>,
    /// **`Telegram` C1c — whether the job the message names has a finished
    /// frame**, handed in by the worker before each tick
    /// ([`TheClock::a_frame_can_be_sent`]).
    a_frame_is_finished: bool,
}

#[derive(Debug)]
struct TheStopOnAMessage {
    message: i64,
    job: Option<String>,
    generation: String,
    with_a_frame: bool,
    buttons: Value,
}

/// **What one tick did**, for the caller's log and for tests.
#[derive(Debug, Clone, PartialEq)]
pub enum WhatATickDid {
    /// A new status message was offered, replacing the one named, if any.
    OfferedACreation { replacing: Option<i64> },
    OfferedAnEdit(i64),
    /// This tick edited the old message to say it is superseded instead.
    OfferedTheSupersededMark(i64),
    /// The last send has not ended, so this tick is dropped (C6: at most one
    /// clock edit outstanding).
    DroppedWhileOneIsOutstanding,
}

impl TheClock {
    pub fn for_the_conversation(chat: i64) -> TheClock {
        TheClock { chat, outstanding: None, could_not_be_edited: None, to_mark_superseded: None, the_stop: None, a_frame_is_finished: false }
    }

    /// **`Telegram` C1c — the job the message names has a finished frame**, so
    /// *Send a frame* rides beside *Stop this job* while it renders. Added
    /// 2026-10-31, the Telegram discussion, the server half.
    pub fn a_frame_can_be_sent(&mut self, finished: bool) {
        self.a_frame_is_finished = finished;
    }

    /// **D1 and R1 — the stop's buttons for `message`**, its record written
    /// against that message the first time, and nothing where no job can be
    /// stopped or the record could not be written.
    fn the_stop_for(
        &mut self,
        store: &mut dyn TheTelegramWorkersState,
        message: i64,
        status: &TheStatus,
        stop: Option<&TheSubject>,
    ) -> Option<Value> {
        let (Some(subject), Some(label)) = (stop, status.doing.the_job_label()) else {
            return None;
        };
        // C1c: while it renders and has a finished frame — never while it is
        // being stopped, held or queued.
        let with_a_frame = self.a_frame_is_finished && matches!(status.doing, WhatTheMachineIsDoing::Rendering { .. });
        if let Some(kept) = &self.the_stop {
            if kept.message == message && kept.job == subject.job && kept.generation == subject.generation && kept.with_a_frame == with_a_frame {
                return Some(kept.buttons.clone());
            }
        }
        let mut row = Vec::new();
        if with_a_frame {
            row.push(a_frame_button_on(store, self.chat, message, subject, &status.server_label, label)?);
        }
        row.push(a_stop_button_on(store, self.chat, message, subject, &status.server_label, label)?);
        let buttons = json!({ "inline_keyboard": [row] });
        self.the_stop = Some(TheStopOnAMessage {
            message,
            job: subject.job.clone(),
            generation: subject.generation.clone(),
            with_a_frame,
            buttons: buttons.clone(),
        });
        Some(buttons)
    }

    /// **Read how the last send ended, and keep what it says.** Call it before
    /// every tick, and as often as the worker reads (B13's cadence), so a new
    /// message's identity is recorded soon after the service accepts it — **a
    /// death between that acceptance and this read leaves one frozen message
    /// unmarked**, the at-least-once cost F3 already accepts.
    ///
    /// Gives back whether a send is still outstanding.
    pub fn take_in_what_ended(&mut self, store: &mut dyn TheTelegramWorkersState) -> Result<bool, WhyTheStoreFailed> {
        let Some((offered, told)) = self.outstanding.take() else {
            return Ok(false);
        };
        let ended = match told.try_recv() {
            Ok(ended) => ended,
            Err(TryRecvError::Empty) => {
                self.outstanding = Some((offered, told));
                return Ok(true);
            }
            Err(TryRecvError::Disconnected) => return Ok(false),
        };
        match (offered, ended) {
            (WhatWasOffered::ACreation { replacing }, HowASendEnded::Accepted(result)) => {
                if let Some(id) = result.get("message_id").and_then(Value::as_i64) {
                    store.record_the_status_message(id)?;
                    self.could_not_be_edited = None;
                    self.to_mark_superseded = replacing;
                }
            }
            // **C7: only a *bad request* refusal — `error_code` 400 — means the
            // recorded message cannot be edited**, so the next tick creates
            // one. Any other refusal, such as a passing server error, changes
            // nothing that tick, as C5's silence does. The published number
            // is read, never the description text (J3 refuses to lean on it).
            // Narrowed by the round-1 code audit of the bot and the chat,
            // fix-here item 16. A send that reached nothing is C5's silence.
            (WhatWasOffered::AnEdit(id), HowASendEnded::HandedBack(WhyTheCallFailed::Refused(refusal)))
                if refusal.error_code == Some(400) =>
            {
                self.could_not_be_edited = Some(id);
            }
            (
                WhatWasOffered::MarkingSuperseded(_),
                HowASendEnded::Accepted(_) | HowASendEnded::HandedBack(WhyTheCallFailed::Refused(_)),
            ) => {
                self.to_mark_superseded = None;
            }
            _ => {}
        }
        Ok(false)
    }

    /// **One tick of `Telegram` C2's clock.** `stop` is the job the message
    /// names, handed in only while that job can still be stopped. **The clock
    /// attaches *Stop this job* itself** (D1; the drawing's `#tgview` draws it
    /// on the status message), in the Verdict's order: **`Telegram` R1 — a new
    /// status message is sent with no buttons**, because a button's record
    /// names its message and this one does not exist yet; once its identity is
    /// recorded, the next tick writes the stop's record against it and the
    /// button rides that edit and every one after. Changed by the round-1 code
    /// audit of the bot and the chat, fix-here item 6, finished in code: until
    /// then buttons were handed in and nothing outside tests handed any.
    pub fn a_tick(
        &mut self,
        store: &mut dyn TheTelegramWorkersState,
        queue: &TheSendQueue,
        status: &TheStatus,
        stop: Option<&TheSubject>,
        now: SystemTime,
    ) -> Result<WhatATickDid, WhyTheStoreFailed> {
        if self.take_in_what_ended(store)? {
            return Ok(WhatATickDid::DroppedWhileOneIsOutstanding);
        }
        let chat = self.chat;
        if let Some(old) = self.to_mark_superseded {
            let text = the_superseded_text(&status.server_label);
            let told = queue.offer(
                AClassOfSend::TheClocksEdit,
                Box::new(move |c: &TheClient| c.edit_marked_message(chat, old, &text, None)),
            );
            self.outstanding = Some((WhatWasOffered::MarkingSuperseded(old), told));
            return Ok(WhatATickDid::OfferedTheSupersededMark(old));
        }
        let text = compose(status, now);
        let (offered, did, send): (_, _, crate::pacing::ASend) = match store.the_status_message()? {
            Some(id) if self.could_not_be_edited != Some(id) => {
                let buttons = self.the_stop_for(store, id, status, stop);
                (
                    WhatWasOffered::AnEdit(id),
                    WhatATickDid::OfferedAnEdit(id),
                    Box::new(move |c: &TheClient| c.edit_marked_message(chat, id, &text, buttons.clone())),
                )
            }
            recorded => (
                WhatWasOffered::ACreation { replacing: recorded },
                WhatATickDid::OfferedACreation { replacing: recorded },
                Box::new(move |c: &TheClient| c.send_marked_message(chat, &text, None)),
            ),
        };
        self.outstanding = Some((offered, queue.offer(AClassOfSend::TheClocksEdit, send)));
        Ok(did)
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::pacing::OnThisComputersClock;
    use crate::transport::a_stand_in::AStandInService;
    use crate::transport::WhatACallCarries;
    use crate::worker_state::InMemory;
    use std::sync::Arc;
    use std::time::UNIX_EPOCH;

    const CHAT: i64 = 777;

    fn at(seconds: u64) -> SystemTime {
        UNIX_EPOCH + Duration::from_secs(seconds)
    }

    /// 12 Aug 2026, 14:22:07 UTC — the design's own moment.
    const THE_DESIGNS_MOMENT: u64 = 1_786_544_527;

    pub(crate) fn rendering() -> TheStatus {
        let now = at(THE_DESIGNS_MOMENT);
        TheStatus {
            server_label: "RunPod 4090".into(),
            doing: WhatTheMachineIsDoing::Rendering {
                job_label: "Kitchen \u{2014} final".into(),
                detail: Some("Interior".into()),
                frame: 148,
                of: 240,
                percent: 62,
                this_frame: Some("View Layer 2 of 2 \u{b7} 812 of 2048 samples".into()),
            },
            card_memory: AReading::Taken { value: CardMemory { used_gb: 18.2, total_gb: 24.0 }, at: now },
            disk_free_gb: AReading::Taken { value: 61.4, at: now },
            uptime: TheUptime::on(TheMachineKind::ASystemdMachine, || AReading::Taken {
                value: Duration::from_secs(4 * 3600 + 12 * 60 + 5),
                at: now,
            }),
        }
    }

    /// **`Telegram` C1, C2a — the design's message, word for word.**
    #[test]
    fn the_message_says_what_the_design_draws() {
        assert_eq!(
            compose(&rendering(), at(THE_DESIGNS_MOMENT)),
            "RunPod 4090\nKitchen \u{2014} final \u{b7} Interior\nRendering: frame 148 of 240 \u{b7} 62%\n\
             This frame: View Layer 2 of 2 \u{b7} 812 of 2048 samples\nGPU memory: 18.2 of 24 GB \u{b7} Disk free: 61 GB\n\
             Uptime: 4 hours 12 minutes\nUpdated 12 Aug 2026, 14:22:07 UTC"
        );
    }

    /// **`Telegram` C1's second bullet — on a container no uptime reading is
    /// taken, and the line says it is unavailable.**
    #[test]
    fn on_a_container_uptime_is_never_read_and_is_said_to_be_unavailable() {
        let mut status = rendering();
        status.uptime = TheUptime::on(TheMachineKind::AContainer, || panic!("no uptime reading is taken on a container"));
        assert!(compose(&status, at(THE_DESIGNS_MOMENT)).contains("\nUptime: unavailable\n"));
    }

    /// **`Telegram` C1's fifth bullet — a reading that could not be taken is
    /// said to be unavailable, never left out.**
    #[test]
    fn a_reading_not_taken_is_said_to_be_unavailable() {
        let mut status = rendering();
        status.card_memory = AReading::Unavailable;
        status.disk_free_gb = AReading::Unavailable;
        status.uptime = TheUptime::Read(AReading::Unavailable);
        let text = compose(&status, at(THE_DESIGNS_MOMENT));
        assert!(text.contains("\nGPU memory: unavailable \u{b7} Disk free: unavailable\n"), "{text}");
        assert!(text.contains("\nUptime: unavailable\n"), "{text}");
    }

    /// **`Telegram` C1 — every reading with its own moment**, said where it
    /// is not the tick's.
    #[test]
    fn a_reading_taken_at_another_moment_says_its_moment() {
        let mut status = rendering();
        status.disk_free_gb = AReading::Taken { value: 61.0, at: at(THE_DESIGNS_MOMENT - 4) };
        assert!(compose(&status, at(THE_DESIGNS_MOMENT)).contains("Disk free: 61 GB (read 12 Aug 2026, 14:22:03 UTC)"));
    }

    #[test]
    fn the_idle_state_takes_the_designs_words() {
        let mut status = rendering();
        status.doing = WhatTheMachineIsDoing::NothingRendering { quiet_for: Some(Duration::from_secs(30 * 60)) };
        assert!(compose(&status, at(THE_DESIGNS_MOMENT))
            .contains("RunPod 4090\nIdle for 30 minutes.\nGPU memory:"));
    }

    /// **`Queue & History` R4 on the status message: a render with no figure
    /// yet says *Rendering.* and no frame nought at nought per cent.**
    #[test]
    fn a_render_with_no_figure_yet_says_rendering_and_no_nought() {
        let mut status = rendering();
        status.doing = WhatTheMachineIsDoing::Rendering {
            job_label: "Kitchen".into(),
            detail: None,
            frame: 0,
            of: 240,
            percent: 0,
            this_frame: None,
        };
        let text = compose(&status, at(THE_DESIGNS_MOMENT));
        assert!(text.starts_with("RunPod 4090\nKitchen\nRendering.\nGPU memory:"), "{text}");
        assert!(!text.contains("Frame 0"), "{text}");
    }

    /// **`Telegram` C1, the live pass's faults piece E: once a stop has been
    /// sent, the message says *Stopping.* in place of the frame line**, the job
    /// still named, and no frame or per cent on it.
    #[test]
    fn a_job_being_stopped_says_stopping_and_no_figure() {
        let mut status = rendering();
        status.doing = WhatTheMachineIsDoing::Stopping { job_label: "Kitchen \u{2014} final".into() };
        let text = compose(&status, at(THE_DESIGNS_MOMENT));
        assert!(text.starts_with("RunPod 4090\nKitchen \u{2014} final\nStopping.\nGPU memory: 18.2 of 24 GB"), "{text}");
        assert!(!text.contains("frame") && !text.contains('%') && !text.contains("This frame"), "{text}");
        assert_eq!(status.doing.the_job_label(), Some("Kitchen \u{2014} final"));
    }

    /// **`Telegram` C1 and D1: *Stop this job* stays under the message from
    /// *Rendering* to *Stopping.*** — the same button under the one record, so
    /// the second stop is a tap away (`Rendering` K10).
    #[test]
    fn the_stop_stays_under_the_message_when_it_says_stopping() {
        let service = Arc::new(AStandInService::new());
        let queue = a_queue(&service);
        let mut store = InMemory::with_routing_key("00112233445566778899aabbccddeeff");
        store.record_the_status_message(88).unwrap();
        let mut clock = TheClock::for_the_conversation(CHAT);
        let job = TheSubject { owning_machine: "m".into(), job: Some("job-1".into()), hold: None, generation: "1".into() };
        let mut status = rendering();
        clock.a_tick(&mut store, &queue, &status, Some(&job), at(THE_DESIGNS_MOMENT)).unwrap();
        queue.send_the_next();
        clock.take_in_what_ended(&mut store).unwrap();
        status.doing = WhatTheMachineIsDoing::Stopping { job_label: "Kitchen \u{2014} final".into() };
        clock.a_tick(&mut store, &queue, &status, Some(&job), at(THE_DESIGNS_MOMENT + 5)).unwrap();
        queue.send_the_next();
        let heard = service.heard();
        assert_eq!(heard.len(), 2);
        let markup = |at: usize| {
            let WhatACallCarries::Fields(fields) = &heard[at].carries else { panic!("text calls carry fields") };
            (fields["text"].as_str().unwrap_or_default().to_string(), fields["reply_markup"].clone())
        };
        let ((rendering, before), (stopping, after)) = (markup(0), markup(1));
        assert!(rendering.contains("\nRendering: frame 148 of 240"), "{rendering}");
        assert!(stopping.contains("\nStopping.\n"), "{stopping}");
        assert!(before["inline_keyboard"][0][0]["text"] == "Stop this job", "{before}");
        assert_eq!(after, before, "the same stop, under the same record");
        assert_eq!(store.the_action_contexts_bound_to(88).unwrap().len(), 1);
    }

    /// **`Telegram` C1c: *Send a frame* beside *Stop this job* while the job
    /// renders and has a finished frame**, first on the one row, its record
    /// binding the job and the newest frame; gone while it is being stopped,
    /// and before the first frame.
    #[test]
    fn telegram_discussion_send_a_frame_rides_beside_stop_from_the_first_finished_frame() {
        let service = Arc::new(AStandInService::new());
        let queue = a_queue(&service);
        let mut store = InMemory::with_routing_key("00112233445566778899aabbccddeeff");
        store.record_the_status_message(88).unwrap();
        let mut clock = TheClock::for_the_conversation(CHAT);
        let job = TheSubject { owning_machine: "m".into(), job: Some("job-1".into()), hold: None, generation: "1".into() };
        let mut status = rendering();
        let tick = |clock: &mut TheClock, store: &mut InMemory, status: &TheStatus, at_second: u64| {
            clock.a_tick(store, &queue, status, Some(&job), at(THE_DESIGNS_MOMENT + at_second)).unwrap();
            queue.send_the_next();
            clock.take_in_what_ended(store).unwrap();
        };
        tick(&mut clock, &mut store, &status, 0);
        clock.a_frame_can_be_sent(true);
        tick(&mut clock, &mut store, &status, 5);
        status.doing = WhatTheMachineIsDoing::Stopping { job_label: "Kitchen \u{2014} final".into() };
        tick(&mut clock, &mut store, &status, 10);
        let heard = service.heard();
        let buttons = |at: usize| -> Vec<String> {
            let WhatACallCarries::Fields(fields) = &heard[at].carries else { panic!("text calls carry fields") };
            fields["reply_markup"]["inline_keyboard"][0].as_array().unwrap().iter().map(|b| b["text"].as_str().unwrap().to_string()).collect()
        };
        assert_eq!(buttons(0), ["Stop this job"], "no finished frame yet");
        assert_eq!(buttons(1), ["Send a frame", "Stop this job"], "one row, Send a frame first");
        assert_eq!(buttons(2), ["Stop this job"], "not while it is being stopped");
        let records = store.the_action_contexts_bound_to(88).unwrap();
        let frame = records.iter().find(|r| r.action == crate::routing::context::SENDING_THE_NEWEST_FRAME).expect("its record");
        assert_eq!((frame.job.as_deref(), frame.generation.as_str()), (Some("job-1"), "1"));
    }

    /// **`Telegram` C2 — 5 seconds times the enabled machines.**
    #[test]
    fn the_interval_grows_with_the_fleet() {
        assert_eq!(the_interval_for(0), Duration::from_secs(5));
        assert_eq!(the_interval_for(1), Duration::from_secs(5));
        assert_eq!(the_interval_for(3), Duration::from_secs(15));
        assert_eq!(the_interval_for(10), Duration::from_secs(50));
    }

    fn a_queue(service: &Arc<AStandInService>) -> TheSendQueue {
        TheSendQueue::new(TheClient::over(service.clone()), Arc::new(OnThisComputersClock))
    }

    fn methods(service: &AStandInService) -> Vec<String> {
        service.heard().iter().map(|c| c.method.clone()).collect()
    }

    /// **`Telegram` C7 and B16 — one message created, then edited in place,
    /// never pinned, notifying and never a spoiler (B24, B25).**
    #[test]
    fn one_message_is_created_then_edited_and_never_pinned() {
        let service = Arc::new(AStandInService::new());
        service.then_answers(r#"{"ok":true,"result":{"message_id":31}}"#);
        let queue = a_queue(&service);
        let mut store = InMemory::with_routing_key("k");
        let mut clock = TheClock::for_the_conversation(CHAT);

        for tick in 0..4 {
            clock.a_tick(&mut store, &queue, &rendering(), None, at(THE_DESIGNS_MOMENT + 5 * tick)).unwrap();
            queue.send_the_next();
        }
        clock.take_in_what_ended(&mut store).unwrap();

        assert_eq!(store.the_status_message().unwrap(), Some(31));
        assert_eq!(methods(&service), ["sendMessage", "editMessageText", "editMessageText", "editMessageText"]);
        for call in service.heard() {
            let WhatACallCarries::Fields(fields) = &call.carries else { panic!("text calls carry fields") };
            assert!(fields.get("disable_notification").is_none(), "the status message is for the user (B24)");
            assert!(fields["entities"].as_array().unwrap().iter().all(|e| e["type"] != "spoiler"), "no spoiler on anything for the user (B25)");
            if call.method == "editMessageText" {
                assert_eq!(fields["message_id"], 31);
            }
        }
    }

    /// **`Telegram` C7 — a restart re-uses the recorded message.**
    #[test]
    fn a_restart_edits_the_recorded_message() {
        let service = Arc::new(AStandInService::new());
        let queue = a_queue(&service);
        let mut store = InMemory::with_routing_key("k");
        store.record_the_status_message(88).unwrap();
        let mut clock = TheClock::for_the_conversation(CHAT);
        assert_eq!(
            clock.a_tick(&mut store, &queue, &rendering(), None, at(THE_DESIGNS_MOMENT)).unwrap(),
            WhatATickDid::OfferedAnEdit(88)
        );
    }

    /// **`Telegram` C6 — a tick while one is outstanding is dropped.**
    #[test]
    fn a_tick_while_one_is_outstanding_is_dropped() {
        let service = Arc::new(AStandInService::new());
        let queue = a_queue(&service);
        let mut store = InMemory::with_routing_key("k");
        store.record_the_status_message(88).unwrap();
        let mut clock = TheClock::for_the_conversation(CHAT);
        clock.a_tick(&mut store, &queue, &rendering(), None, at(THE_DESIGNS_MOMENT)).unwrap();
        assert_eq!(
            clock.a_tick(&mut store, &queue, &rendering(), None, at(THE_DESIGNS_MOMENT + 5)).unwrap(),
            WhatATickDid::DroppedWhileOneIsOutstanding
        );
        assert_eq!(queue.how_many_are_waiting(), 1);
    }

    /// **`Telegram` C7 — where the recorded message cannot be edited, a new one
    /// is created and the old one is edited to say it is superseded.**
    #[test]
    fn a_message_that_cannot_be_edited_is_replaced_and_marked_superseded() {
        let service = Arc::new(AStandInService::new());
        service
            .then_answers(r#"{"ok":false,"error_code":400,"description":"Bad Request: message to edit not found"}"#)
            .then_answers(r#"{"ok":true,"result":{"message_id":90}}"#);
        let queue = a_queue(&service);
        let mut store = InMemory::with_routing_key("k");
        store.record_the_status_message(88).unwrap();
        let mut clock = TheClock::for_the_conversation(CHAT);

        let mut did = Vec::new();
        for tick in 0..4 {
            did.push(clock.a_tick(&mut store, &queue, &rendering(), None, at(THE_DESIGNS_MOMENT + 5 * tick)).unwrap());
            queue.send_the_next();
        }

        assert_eq!(
            did,
            [
                WhatATickDid::OfferedAnEdit(88),
                WhatATickDid::OfferedACreation { replacing: Some(88) },
                WhatATickDid::OfferedTheSupersededMark(88),
                WhatATickDid::OfferedAnEdit(90),
            ]
        );
        assert_eq!(store.the_status_message().unwrap(), Some(90));
        let WhatACallCarries::Fields(marked) = &service.heard()[2].carries else { panic!() };
        assert_eq!(marked["message_id"], 88);
        assert_eq!(
            marked["text"],
            "RunPod 4090\nNo longer updated. See the newer status message."
        );
    }

    /// **`Telegram` C5 — a send that reached nothing is silence**: nothing is
    /// re-sent, reported, or replaced.
    #[test]
    fn a_send_that_reached_nothing_changes_nothing() {
        let service = Arc::new(AStandInService::new());
        service.then_is_not_reached("connection failed");
        let queue = a_queue(&service);
        let mut store = InMemory::with_routing_key("k");
        store.record_the_status_message(88).unwrap();
        let mut clock = TheClock::for_the_conversation(CHAT);
        clock.a_tick(&mut store, &queue, &rendering(), None, at(THE_DESIGNS_MOMENT)).unwrap();
        queue.send_the_next();
        assert_eq!(
            clock.a_tick(&mut store, &queue, &rendering(), None, at(THE_DESIGNS_MOMENT + 5)).unwrap(),
            WhatATickDid::OfferedAnEdit(88)
        );
    }

    /// **C1 and the round-1 code audit's fix-here item 5 — a job waiting on the
    /// user, and a queued one, are said; never *idle*.**
    #[test]
    fn a_waiting_job_and_a_queued_job_are_said() {
        let mut status = rendering();
        status.doing = WhatTheMachineIsDoing::WaitingOnYou { job_label: "Kitchen \u{2014} final".into() };
        let text = compose(&status, at(THE_DESIGNS_MOMENT));
        assert!(text.contains("RunPod 4090\nKitchen \u{2014} final\nNeeds your answer. Nothing is rendering.\nGPU memory:"), "{text}");
        assert!(!text.contains("nothing waiting") && !text.contains("Idle"));
        status.doing = WhatTheMachineIsDoing::Queued { job_label: "Turntable".into() };
        assert!(compose(&status, at(THE_DESIGNS_MOMENT)).contains("\nTurntable\nQueued.\n"));
    }

    /// **A3 and C1, held-job piece B: a job held for the driver update reads
    /// as its queue entry does, and never *waiting on you*** — nothing waits on
    /// the user. Its label stays the one a stop names.
    #[test]
    fn a_job_held_for_the_driver_update_is_never_waiting_on_you() {
        let mut status = rendering();
        status.doing = WhatTheMachineIsDoing::WaitingForTheDriverUpdate { job_label: "Kitchen \u{2014} night".into() };
        let text = compose(&status, at(THE_DESIGNS_MOMENT));
        assert!(text.contains("RunPod 4090\nKitchen \u{2014} night\nWaiting for driver update.\nGPU memory:"), "{text}");
        assert!(!text.to_lowercase().contains("waiting on you"), "{text}");
        assert!(!text.to_lowercase().contains("needs your answer"), "{text}");
        assert!(!text.contains("until you answer"), "{text}");
        assert_eq!(status.doing.the_job_label(), Some("Kitchen \u{2014} night"));
    }

    /// **C7 — only a 400 refusal means the message cannot be edited**: a
    /// passing server error leaves the recorded message in use, and posts
    /// nothing new.
    #[test]
    fn a_refusal_that_is_not_a_bad_request_changes_nothing() {
        let service = Arc::new(AStandInService::new());
        service.then_answers(r#"{"ok":false,"error_code":500,"description":"Internal Server Error"}"#);
        let queue = a_queue(&service);
        let mut store = InMemory::with_routing_key("k");
        store.record_the_status_message(88).unwrap();
        let mut clock = TheClock::for_the_conversation(CHAT);
        let mut did = Vec::new();
        for tick in 0..3 {
            did.push(clock.a_tick(&mut store, &queue, &rendering(), None, at(THE_DESIGNS_MOMENT + 5 * tick)).unwrap());
            queue.send_the_next();
        }
        assert_eq!(did, [WhatATickDid::OfferedAnEdit(88), WhatATickDid::OfferedAnEdit(88), WhatATickDid::OfferedAnEdit(88)]);
        assert_eq!(methods(&service), ["editMessageText", "editMessageText", "editMessageText"]);
    }

    /// **D1 — with no job named in the message, no stop rides it**, whatever
    /// is handed in, and no record is written.
    #[test]
    fn with_nothing_rendering_no_stop_rides_the_message() {
        let service = Arc::new(AStandInService::new());
        let queue = a_queue(&service);
        let mut store = InMemory::with_routing_key("00112233445566778899aabbccddeeff");
        store.record_the_status_message(88).unwrap();
        let mut clock = TheClock::for_the_conversation(CHAT);
        let mut status = rendering();
        status.doing = WhatTheMachineIsDoing::NothingRendering { quiet_for: None };
        let job = TheSubject { owning_machine: "m".into(), job: Some("job-1".into()), hold: None, generation: "1".into() };
        clock.a_tick(&mut store, &queue, &status, Some(&job), at(THE_DESIGNS_MOMENT)).unwrap();
        queue.send_the_next();
        let WhatACallCarries::Fields(fields) = &service.heard()[0].carries else { panic!() };
        assert!(fields.get("reply_markup").is_none());
        assert!(store.the_action_contexts_bound_to(88).unwrap().is_empty());
    }
}

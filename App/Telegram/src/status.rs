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

use crate::job_messages::{AButtonToOffer, AComposedMessage, AServerLabel, TheServersName};
use crate::job_messages::{AMarkedText, a_user_message};
use std::sync::mpsc::{Receiver, TryRecvError};
use std::sync::{Arc, Mutex};
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
        /// **The frame in hand has finished sampling** (`Telegram` C1's
        /// *Finishing* as changed later on 2026-11-13).
        finishing: bool,
    },
    /// **`Telegram` C1: a rendering job a stop's interrupt has gone out on**,
    /// until the render ends. Blender goes on through the range after the
    /// first interrupt and saves nothing, so no frame or per cent is said; the
    /// job is still named, and *Stop this job* still rides the message
    /// (`Rendering` K10's second stop). Added 2026-10-19, the live pass's
    /// faults, piece E.
    Stopping { job_label: String },
    /// `Telegram` C1: the second attempt has no render process yet.
    RunningAgain { job_label: String },
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
            | WhatTheMachineIsDoing::RunningAgain { job_label }
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
        WhatTheMachineIsDoing::Rendering { job_label, detail, .. } => {
            lines.push(match detail {
                Some(detail) => format!("{job_label} \u{b7} {detail}"),
                None => job_label.clone(),
            });
        }
        other => lines.extend(other.the_job_label().map(str::to_string)),
    }
    lines.extend(the_state_line(&status.doing));
    if let WhatTheMachineIsDoing::Rendering { this_frame: Some(this_frame), .. } = &status.doing {
        lines.push(format!("This frame: {this_frame}"));
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
    // **C2a as changed 2026-11-13 (the Creator's notes on the test copy, note
    // 2): the *Updated* line carries C1d's age entity**, so a phone counts it
    // up as it does the job's; its text is the same UTC moment as before.
    out.with_when_it_was_updated(now);
    out
}

/// **`Telegram` C1's line for what a job is doing** — one composer for C1's
/// status message and, from 2026-11-09, each job's own message (C1d: *"the job
/// line C1 shows for that job, in C1's words"*). Nothing where the machine
/// shows no job's state line (work not reported).
pub fn the_state_line(doing: &WhatTheMachineIsDoing) -> Option<String> {
    Some(match doing {
        WhatTheMachineIsDoing::WorkNotReported => return None,
        // **A render whose worker has reported no figure yet, or whose
        // frames are not known, is said to be rendering and nothing
        // more** — `Queue & History` R4: absent, never a nought — the
        // rule the gather's line keeps. **And a job whose reporting
        // choice leaves out the running commentary** (`Telegram` V1) is
        // handed in the same way, so its progress is never said. Added
        // 2026-10-16, the Telegram worker's piece 6.
        WhatTheMachineIsDoing::Rendering { frame, of, .. } if *frame == 0 || *of == 0 => "Rendering.".to_string(),
        // **C1's addition of 2026-11-13: every sample counted, Blender
        // finishing the frame** — denoising, compositing, writing — which
        // the research names no line for, so it is not named here either.
        // **Changed later that day (Sol's look at the scout's list, point
        // 5): the frame in hand, never the job's 100%**, so every frame's
        // finishing is said and a wrong plan total says nothing early.
        WhatTheMachineIsDoing::Rendering { frame, of, finishing: true, .. } => {
            format!("Finishing: frame {frame} of {of}.")
        }
        WhatTheMachineIsDoing::Rendering { frame, of, percent, .. } => {
            format!("Rendering: frame {frame} of {of} \u{b7} {percent}%")
        }
        // **`Telegram` C1's own words**, in place of the frame line.
        WhatTheMachineIsDoing::Stopping { .. } => "Stopping.".to_string(),
        WhatTheMachineIsDoing::RunningAgain { .. } => "Interrupted. Retrying.".to_string(),
        // *Needs your answer* is the drawing's phrase (`tg-check`; *Waiting on
        // you* until step 13's final words sweep); the rest are
        // standard words Design may replace. Added by the round-1 code audit of
        // the bot and the chat, fix-here item 5.
        WhatTheMachineIsDoing::WaitingOnYou { .. } => "Needs your answer. Nothing is rendering.".to_string(),
        // **The queue entry's own words** (`Design/Queue & History § A job
        // held for the driver update`): Design gives the phone none of its
        // own, and until it does the entry's stand (`Telegram` A3).
        WhatTheMachineIsDoing::WaitingForTheDriverUpdate { .. } => "Waiting for driver update.".to_string(),
        // **The app's own word** — the four questions, scout 3.29, 2026-11-11.
        WhatTheMachineIsDoing::Queued { .. } => "Waiting.".to_string(),
        WhatTheMachineIsDoing::NothingRendering { quiet_for } => match quiet_for {
            Some(quiet_for) => format!("Idle for {}.", a_duration(*quiet_for)),
            None => "Idle.".to_string(),
        },
    })
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
        format!("{text} (as of {moment})")
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

/// **`Telegram` C7's sentence of 2026-11-12 — how many refusals that are not
/// a bad request the superseded mark is tried through.** The third ends it,
/// the figure C1e's cut is given up at.
pub const THE_MOST_REFUSALS_A_SUPERSEDED_MARK_IS_TRIED_THROUGH: u8 = 3;

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
    /// **The old messages still owed their superseded mark, oldest first,
    /// each with how many refusals it has met.** More than one where a second
    /// replacement comes before the first mark has landed (Sol's look at
    /// C7's sentence of 2026-11-12: one slot dropped the older mark).
    to_mark_superseded: std::collections::VecDeque<(i64, u8)>,
    /// **`Telegram` C7's sentence of 2026-11-12 — the mark met a refusal
    /// that may pass, so the next tick is the live message's** and the mark
    /// is tried again on the one after.
    the_mark_waits_a_tick: bool,
    /// The stop last written for a message, and the job it stops, so every
    /// edit carries the same button under one record. Held in memory: a restart
    /// writes one more record, never a button without one.
    the_stop: Option<TheStopOnAMessage>,
    /// **`Telegram` C1c — whether the job the message names has a finished
    /// frame**, handed in by the worker before each tick
    /// ([`TheClock::a_frame_can_be_sent`]).
    a_frame_is_finished: bool,
    /// **`Telegram` F3c — the status message Telegram accepted whose
    /// identity the store could not write.** It is edited as the recorded
    /// one would be, and the write is tried again at each tick. Added
    /// 2026-11-12, the Fable review, sitting 6: without it every tick sent a
    /// new status message for as long as the store could not be written.
    not_yet_written: Option<i64>,
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
        TheClock { chat, outstanding: None, could_not_be_edited: None, to_mark_superseded: std::collections::VecDeque::new(), the_mark_waits_a_tick: false, the_stop: None, a_frame_is_finished: false, not_yet_written: None }
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
                    // F3c: the message stands whether or not its identity
                    // could be written, so it is kept either way.
                    let written = store.record_the_status_message(id);
                    self.could_not_be_edited = None;
                    if let Some(old) = replacing {
                        self.to_mark_superseded.push_back((old, 0));
                    }
                    self.not_yet_written = written.is_err().then_some(id);
                    written?;
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
            // **C7's sentence of 2026-11-12 (Sol's look at the double-check
            // ideas, finding 5): the mark ends where Telegram accepts it or
            // refuses it as a bad request** — the old message is gone or
            // cannot be edited. **Any other refusal keeps it owed**, and it
            // is tried again on every second tick, **until the third such
            // refusal ends it too**, so a refusal that never passes does not
            // take every second tick from the live message for good. Until then any refusal
            // ended it, so one that would have passed — a server error, say
            // — left the old message at its last words for good. *(A flood
            // refusal ends as dropped, C6a, and was always offered again.)*
            (WhatWasOffered::MarkingSuperseded(old), HowASendEnded::Accepted(_)) => {
                self.to_mark_superseded.retain(|(owed, _)| *owed != old);
            }
            (WhatWasOffered::MarkingSuperseded(old), HowASendEnded::HandedBack(WhyTheCallFailed::Refused(refusal))) => {
                let refusals = self
                    .to_mark_superseded
                    .iter_mut()
                    .find(|(owed, _)| *owed == old)
                    .map(|(_, refusals)| {
                        *refusals = refusals.saturating_add(1);
                        *refusals
                    })
                    .unwrap_or(THE_MOST_REFUSALS_A_SUPERSEDED_MARK_IS_TRIED_THROUGH);
                if refusal.error_code == Some(400) || refusals >= THE_MOST_REFUSALS_A_SUPERSEDED_MARK_IS_TRIED_THROUGH {
                    self.to_mark_superseded.retain(|(owed, _)| *owed != old);
                } else {
                    self.the_mark_waits_a_tick = true;
                }
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
        // **A mark that met a passing refusal lets this tick go to the live
        // message**, so that one still moves while the mark is owed.
        let the_mark_waits = std::mem::take(&mut self.the_mark_waits_a_tick);
        if let Some(old) = self.to_mark_superseded.front().map(|(old, _)| *old).filter(|_| !the_mark_waits) {
            let text = the_superseded_text(&status.server_label);
            let told = queue.offer(
                AClassOfSend::TheClocksEdit,
                Box::new(move |c: &TheClient| c.edit_marked_message(chat, old, &text, None)),
            );
            self.outstanding = Some((WhatWasOffered::MarkingSuperseded(old), told));
            return Ok(WhatATickDid::OfferedTheSupersededMark(old));
        }
        // C1d as changed 2026-11-13: the moment on Telegram's clock.
        let text = compose(status, queue.on_telegrams_clock(now));
        // F3c: an identity not yet written is written now where it can be,
        // and is the status message meanwhile.
        if let Some(id) = self.not_yet_written {
            if store.record_the_status_message(id).is_ok() {
                self.not_yet_written = None;
            }
        }
        let held = match self.not_yet_written {
            Some(id) => Some(id),
            None => store.the_status_message()?,
        };
        let (offered, did, send): (_, _, crate::pacing::ASend) = match held {
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

// ------------------------------------------------------------ C1d: each job's own message

/// **`Telegram` C1d's last bullet and §21 item 39 — the one switch.** Whether
/// a change to a message makes a sound or a notice on a phone is not
/// established; the live pass owed by item 39 settles it. **If it does, this
/// is set to `false`** and nothing else moves: the tick's edits stop, and the
/// job's message changes only by C1e's cut, by A3's answer and by D0's
/// question asked in place — each beside a send that already sounds under
/// V1e, or at a tap the user has just made — its progress then being C1's
/// alone. **The *Updated* line goes with it** (C1d as changed 2026-11-10):
/// a moment no tick moves would freeze and mislead.
pub const THE_TICK_EDITS_A_JOBS_MESSAGE: bool = true;

/// **What one live job's message should say now** — handed in by the worker
/// for each job it reads at a tick, from the same reading C1's message is
/// composed from (`Queue & History` owns it).
#[derive(Debug, Clone, PartialEq)]
pub struct WhatAJobsMessageSays {
    /// The job, the machine that owns it and the generation its buttons are
    /// valid in (R1).
    pub subject: TheSubject,
    pub server: AServerLabel,
    pub label: String,
    pub state: AJobsState,
    /// **The moment this was read, on the machine's own clock** — what C1d's
    /// *Updated* line holds while the job renders (the twenty-ninth notes,
    /// note 9). None where no tick read it, as at the start: the first tick
    /// adds the line.
    pub at: Option<SystemTime>,
}

/// **The job's state, as C1d shows it.** A job held on its question is not
/// handed in: the question is its words while it waits (A3a).
#[derive(Debug, Clone, PartialEq)]
pub enum AJobsState {
    /// Rendering; the frame figures are nought until the first progress, and
    /// whether a frame is finished, for C1c's *Send a frame*.
    Rendering { frame: u64, of: u64, percent: u8, a_frame_is_finished: bool, finishing: bool },
    /// `Telegram` C1d: the second attempt has no render process yet.
    RunningAgain,
    /// A stop's interrupt has gone out on the render (C1).
    Stopping,
    /// Waiting its turn. `the_frames` is the range in words where the job is a
    /// first-frame range released by *Render all*, whose job's message offers
    /// *Cancel this job* (C1d's buttons).
    Queued { the_frames: Option<String> },
}

impl AJobsState {
    fn the_line(&self, label: &str) -> Option<String> {
        let doing = match self {
            AJobsState::Rendering { frame, of, percent, finishing, .. } => {
                if *frame == 0 || *of == 0 {
                    return None;
                }
                WhatTheMachineIsDoing::Rendering {
                    job_label: label.into(),
                    detail: None,
                    frame: *frame,
                    of: *of,
                    percent: *percent,
                    this_frame: None,
                    finishing: *finishing,
                }
            }
            AJobsState::RunningAgain => WhatTheMachineIsDoing::RunningAgain { job_label: label.into() },
            AJobsState::Stopping => WhatTheMachineIsDoing::Stopping { job_label: label.into() },
            AJobsState::Queued { .. } => WhatTheMachineIsDoing::Queued { job_label: label.into() },
        };
        the_state_line(&doing)
    }
}

/// **C1d's buttons, as wanted** — *Send a frame* under C1c's own rule, then
/// *Stop this job*, or *Cancel this job* while a released range waits.
#[derive(Debug, Clone, PartialEq)]
struct TheWantedButtons {
    a_frame: bool,
    the_frames: Option<String>,
}

impl TheWantedButtons {
    fn for_the(state: &AJobsState) -> TheWantedButtons {
        match state {
            AJobsState::Rendering { a_frame_is_finished, .. } => {
                TheWantedButtons { a_frame: *a_frame_is_finished, the_frames: None }
            }
            AJobsState::Stopping | AJobsState::RunningAgain => TheWantedButtons { a_frame: false, the_frames: None },
            AJobsState::Queued { the_frames } => TheWantedButtons {
                a_frame: false,
                the_frames: Some(the_frames.clone().unwrap_or_default()),
            },
        }
    }

    /// **The buttons to offer**, each written as an R1 record against the
    /// message before it is attached. *Stop this job* carries the job's
    /// message's identity (`jobs_message`), so a press asks in place (D0).
    fn as_offered(&self, says: &WhatAJobsMessageSays) -> Vec<AButtonToOffer> {
        let mut buttons = Vec::new();
        if self.a_frame {
            buttons.push(AButtonToOffer {
                label: "Send a frame".into(),
                action: crate::routing::context::SENDING_THE_NEWEST_FRAME,
                payload: json!({ "job": says.subject.job, "labels": { "server": says.server, "label": says.label, "button": "Send a frame" } }),
            });
        }
        let mut stop = crate::job_messages::a_stop_to_offer(&says.server, &says.label, false);
        stop.payload[THE_JOBS_MESSAGE] = json!(true);
        if let Some(the_frames) = &self.the_frames {
            stop.label = "Cancel this job".into();
            stop.payload["labels"]["button"] = json!("Cancel this job");
            if !the_frames.is_empty() {
                stop.payload["the_frames"] = json!(the_frames);
            }
        }
        buttons.push(stop);
        buttons
    }
}

/// **On a Stop's record: the job's message it rides**, so a press asks in
/// place (`Telegram` D0). Written `true` until the message's identity is known,
/// then that identity (`crate::actions::TheJobMessages`).
pub const THE_JOBS_MESSAGE: &str = "jobs_message";
/// **On *Keep rendering*'s record: the question's own words**, so a tick after
/// a restart still shows the question asked in place (D0: *"that it is open is
/// read from this durable context, never kept twice"*).
pub const THE_QUESTIONS_WORDS: &str = "question_words";

/// What the worker remembers of one job's message, in memory only (C7a:
/// *"its last accepted words live in the worker's memory"*).
#[derive(Default)]
struct OneJobsMessage {
    /// **Every edit made at once goes through here**, so the cut, a question
    /// asked in place and an answer told in place never cross, and none of
    /// them is overtaken by a tick's edit offered before it.
    lane: Arc<Mutex<()>>,
    last_accepted: Option<(AMarkedText, Value)>,
    /// The tick's edit offered and not ended, with what it said.
    outstanding: Option<(AMarkedText, Value, Receiver<HowASendEnded>)>,
    /// **Its cut is under way or done**: the tick edits it no more.
    ending: bool,
    /// The last edit Telegram refused as impossible: the next tick sends the
    /// job's message again (C7a).
    cannot_be_edited: bool,
    /// The buttons last offered, by the wanted set and generation they were
    /// written for.
    buttons: Option<(TheWantedButtons, String, Value)>,
    /// What the worker last handed in about it.
    says: Option<WhatAJobsMessageSays>,
}

#[derive(Default)]
struct TheKept {
    by_message: std::collections::HashMap<i64, OneJobsMessage>,
    /// **I3's line, by job** — a last line on the job's message while it is
    /// live (C1d), held in memory: a restart may lose it, never repeat it.
    lines: std::collections::HashMap<String, Vec<String>>,
}

/// **`Telegram` C1d, C1e and C7a — every job's own message on this machine**,
/// shared by the journal's thread (the start, the cut, the answer in place,
/// pictures' replies), the clock's tick and the owner's acts (the question
/// asked in place). **Its durable half is C7a's record** in the worker's
/// store; this keeps only what the worker may lose. Added 2026-11-09, the
/// twenty-eighth notes, notes 18 and 19.
#[derive(Clone, Default)]
pub struct TheJobsOwnMessages(Arc<Mutex<TheKept>>);

/// **How a cut, an answer in place or a question in place went.**
#[derive(Debug, Clone, PartialEq)]
pub enum WhatTheEditCameTo {
    /// Accepted, or already as it was asked to be.
    Done,
    /// No job's message is recorded for the job, or the recorded one is not
    /// the one named: nothing was edited, and the caller sends as before.
    NoJobsMessage,
    /// The message can no longer be edited: its record ended, nothing sent.
    ItCannotBeEdited,
    /// The line reached nothing; try again on a later turn.
    TheLineReachedNothing,
    /// Refused for another reason; tried again on a later turn.
    Refused(HowASendEnded),
}

fn kept(keeper: &TheJobsOwnMessages) -> std::sync::MutexGuard<'_, TheKept> {
    keeper.0.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}

/// **An edit is done** where it was accepted or Telegram says the message is
/// already as asked (`crate::actions`' reading of a replayed edit).
fn the_edit_landed(ended: &HowASendEnded) -> bool {
    matches!(ended, HowASendEnded::Accepted(_))
        || matches!(ended, HowASendEnded::HandedBack(WhyTheCallFailed::Refused(why))
            if why.description.as_deref().is_some_and(|text| text.contains("message is not modified")))
}

/// **C7's own reading of *cannot be edited***: a bad request, `error_code`
/// 400, that is not *not modified*.
fn it_cannot_be_edited(ended: &HowASendEnded) -> bool {
    !the_edit_landed(ended)
        && matches!(ended, HowASendEnded::HandedBack(WhyTheCallFailed::Refused(why)) if why.error_code == Some(400))
}

/// **The rows a message's live records make**, each button on its row in the
/// order they were written — how a question asked in place is shown again.
fn the_rows_of(records: &[crate::routing::context::AnActionContextRecord]) -> Value {
    let mut live: Vec<_> = records.iter().filter(|r| r.payload["stop_removed"] != json!(true)).collect();
    live.sort_by_key(|r| r.payload["button_row"].as_u64().unwrap_or(u64::MAX));
    let rows: Vec<Value> = live
        .iter()
        .map(|r| {
            let label = r.payload.pointer("/labels/button").and_then(Value::as_str).unwrap_or("Stop this job");
            let mut item = json!({ "text": label, "callback_data": r.token });
            if r.payload["confirmed"] == json!(true) {
                item["style"] = json!("danger");
            }
            json!([item])
        })
        .collect();
    json!({ "inline_keyboard": rows })
}

/// Every button of a markup on one row (C1c: *"Buttons sit on one row"*).
pub(crate) fn on_one_row(markup: Value) -> Value {
    let row: Vec<Value> = markup["inline_keyboard"]
        .as_array()
        .into_iter()
        .flatten()
        .flat_map(|row| row.as_array().cloned().unwrap_or_default())
        .collect();
    json!({ "inline_keyboard": [row] })
}

impl TheJobsOwnMessages {
    /// **The live job's message of `job`**, where one is recorded (C7a) — what
    /// a frame's picture replies to (I2).
    pub fn the_message_of(&self, store: &dyn TheTelegramWorkersState, job: &str) -> Option<i64> {
        store.the_jobs_message_of(job).map(|record| record.message)
    }

    /// **The name the tick last gave a job's message**, from the worker's
    /// memory — for a cut once the job's own records have gone.
    pub fn the_label_last_shown_for(&self, job: &str) -> Option<String> {
        kept(self)
            .by_message
            .values()
            .filter_map(|one| one.says.as_ref())
            .find(|says| says.subject.job.as_deref() == Some(job))
            .map(|says| says.label.clone())
    }

    /// **I3's line on the job's message while it is live** (C1d): *Preview not
    /// sent: over Telegram's limits.*, shown at the next edit.
    pub fn a_last_line(&self, job: &str, line: &str) {
        let mut kept = kept(self);
        let lines = kept.lines.entry(job.to_string()).or_default();
        if !lines.iter().any(|held| held == line) {
            lines.push(line.to_string());
        }
    }

    /// **I3's lines no accepted edit of the job's message has shown** — said
    /// as a message of its own at the ending, since the cut takes them away
    /// (Sol's fresh look, round 1).
    pub fn the_lines_never_shown(&self, job: &str) -> Vec<String> {
        let kept = kept(self);
        let shown: Vec<String> = kept
            .by_message
            .values()
            .filter(|one| one.says.as_ref().is_some_and(|says| says.subject.job.as_deref() == Some(job)))
            .filter_map(|one| one.last_accepted.as_ref().map(|(text, _)| text.text.clone()))
            .collect();
        kept.lines
            .get(job)
            .into_iter()
            .flatten()
            .filter(|line| !shown.iter().any(|text| text.contains(line.as_str())))
            .cloned()
            .collect()
    }

    /// **I3's lines said as a message of their own**: gone from the job's
    /// message, so a cut tried again says them no more (Sol's fresh look,
    /// round 2).
    pub fn the_lines_are_said(&self, job: &str) {
        kept(self).lines.remove(job);
    }

    pub(crate) fn the_lines_of(&self, job: &str) -> Vec<String> {
        kept(self).lines.get(job).cloned().unwrap_or_default()
    }

    /// The words C1d composes now, with no question and no buttons. **While
    /// the job renders, they end with C1d's *Updated* line** holding the
    /// moment the tick read (the twenty-ninth notes, note 9) — where
    /// `with_its_moment` (no question asked in place), where a tick read a
    /// moment (not the start's), and only while the tick edits at all (the
    /// one switch: a moment no tick moves would freeze and mislead).
    fn the_words(&self, says: &WhatAJobsMessageSays, with_its_moment: bool, queue: &TheSendQueue) -> AMarkedText {
        let lines = self.the_lines_of(says.subject.job.as_deref().unwrap_or_default());
        let lines: Vec<&str> = lines.iter().map(String::as_str).collect();
        let mut words =
            crate::job_messages::the_jobs_message(&says.server, &says.label, says.state.the_line(&says.label).as_deref(), &lines);
        if let Some(at) = says.at.filter(|_| {
            with_its_moment && THE_TICK_EDITS_A_JOBS_MESSAGE && matches!(says.state, AJobsState::Rendering { .. })
        }) {
            // C1d as changed 2026-11-13: on Telegram's clock.
            words.with_when_it_was_updated(queue.on_telegrams_clock(at));
        }
        words
    }

    /// **C1a and C1d — the job's message at its start**: *Started.* with *Stop
    /// this job* where the job is live, its sound V1e's (`silent`), and C7a's
    /// record written once Telegram accepts it. **Where a job's message is
    /// already recorded for the job — a held range's question (C1a: *"a held
    /// range job's start sends none"*), an earlier attempt's, or one sent
    /// before a restart replayed this start — none is sent**, and the record
    /// takes this attempt's generation, so its buttons are written again
    /// under it at the next tick.
    pub fn the_start(
        &self,
        messages: &mut crate::actions::TheJobMessages,
        says: &WhatAJobsMessageSays,
        live: bool,
        silent: bool,
    ) -> Result<i64, crate::actions::WhyAMessageWasNotSent> {
        let job = says.subject.job.clone().unwrap_or_default();
        if let Some(mut record) = messages.store.the_jobs_message_of(&job) {
            if record.generation != says.subject.generation {
                record.generation = says.subject.generation.clone();
                let _ = messages.store.record_a_jobs_message(record.clone());
            }
            return Ok(record.message);
        }
        let composed = AComposedMessage {
            text: self.the_words(says, true, &messages.queue),
            buttons: if live { TheWantedButtons::for_the(&says.state).as_offered(says) } else { Vec::new() },
        };
        messages.send_the_jobs_message(&composed, &says.subject, crate::transport::HowItIsSent::silent(silent))
    }

    /// **Read how the tick's edits ended**, keeping what each says.
    fn take_in_what_ended(&self) {
        let mut kept = kept(self);
        for one in kept.by_message.values_mut() {
            let Some((text, markup, told)) = one.outstanding.take() else { continue };
            match told.try_recv() {
                Err(TryRecvError::Empty) => one.outstanding = Some((text, markup, told)),
                Err(TryRecvError::Disconnected) => {}
                Ok(ended) if the_edit_landed(&ended) => one.last_accepted = Some((text, markup)),
                Ok(ended) if it_cannot_be_edited(&ended) => one.cannot_be_edited = true,
                // C6's superseded rule and C5's silence: the next tick
                // carries it.
                Ok(_) => {}
            }
        }
    }

    /// **C1d's buttons for one message**: those written for it last where
    /// they still stand — the same wanted set, the same generation, and every
    /// record unmarked — and otherwise written again (R1).
    fn the_buttons_for(
        &self,
        messages: &mut crate::actions::TheJobMessages,
        message: i64,
        says: &WhatAJobsMessageSays,
    ) -> Option<Value> {
        let wanted = TheWantedButtons::for_the(&says.state);
        let held = kept(self)
            .by_message
            .get(&message)
            .and_then(|one| one.buttons.clone())
            .filter(|(was, generation, _)| *was == wanted && *generation == says.subject.generation);
        if let Some((_, _, markup)) = held {
            let records = messages.store.the_action_contexts_bound_to(message).unwrap_or_default();
            let tokens: Vec<String> = markup["inline_keyboard"]
                .as_array()
                .into_iter()
                .flatten()
                .flat_map(|row| row.as_array().cloned().unwrap_or_default().into_iter())
                .filter_map(|item| item["callback_data"].as_str().map(str::to_string))
                .collect();
            if tokens.iter().all(|token| records.iter().any(|r| r.token == *token && r.payload["stop_removed"] != json!(true))) {
                return Some(markup);
            }
        }
        let composed = AComposedMessage { text: AMarkedText::default(), buttons: wanted.as_offered(says) };
        let markup = on_one_row(messages.the_buttons_recorded_for(message, &composed, &says.subject).ok()?);
        kept(self).by_message.entry(message).or_default().buttons =
            Some((wanted, says.subject.generation.clone(), markup.clone()));
        Some(markup)
    }

    /// **What the job's message shows now**: C1d's words and buttons, or —
    /// while D0's question asked in place is open, read from its durable
    /// records — the words with the question's and its two buttons.
    fn what_it_shows(
        &self,
        messages: &mut crate::actions::TheJobMessages,
        message: i64,
        says: &WhatAJobsMessageSays,
    ) -> Option<(AMarkedText, Value)> {
        let records = messages.store.the_action_contexts_bound_to(message).ok()?;
        let open = records.iter().find(|r| {
            r.action == crate::job_messages::KEEP_RENDERING
                && r.payload["stop_removed"] != json!(true)
                && r.payload[THE_QUESTIONS_WORDS].is_string()
        });
        // **No *Updated* line while D0's question is asked in place**: the
        // question stays beside its buttons (C1d as changed 2026-11-10).
        let mut words = self.the_words(says, open.is_none(), &messages.queue);
        if let Some(keep) = open {
            words.push_str(&format!("\n{}", keep.payload[THE_QUESTIONS_WORDS].as_str().unwrap_or_default()));
            let question: Vec<_> = records
                .iter()
                .filter(|r| r.payload["confirmation_message"] == json!(message) && r.payload["stop_removed"] != json!(true))
                .cloned()
                .collect();
            return Some((words, the_rows_of(&question)));
        }
        Some((words, self.the_buttons_for(messages, message, says)?))
    }

    /// **One tick of C1d**, on the same worker turn as C1's edit. `says` is
    /// what the worker read for each live job; a job not in it — held on its
    /// question, ended, or unknown — is not edited. **Only a job's message
    /// whose words or buttons differ from what Telegram last accepted is
    /// edited, one a tick**, in its own class of C6 with its superseded rule.
    /// **One whose last edit Telegram refused as impossible** — the user
    /// deleted it — **is sent again, without a sound, and recorded in its
    /// place** (C7a, V1e).
    pub fn a_tick(&self, messages: &mut crate::actions::TheJobMessages, says: &[WhatAJobsMessageSays]) {
        self.take_in_what_ended();
        let Ok(records) = messages.store.the_jobs_messages() else { return };
        for record in records {
            let Some(says) = says.iter().find(|one| one.subject.job.as_deref() == Some(record.job.as_str())) else {
                continue;
            };
            let (ending, outstanding, cannot_be_edited, lane) = {
                let mut kept = kept(self);
                let one = kept.by_message.entry(record.message).or_default();
                one.says = Some(says.clone());
                (one.ending, one.outstanding.is_some(), one.cannot_be_edited, one.lane.clone())
            };
            if ending || outstanding || !THE_TICK_EDITS_A_JOBS_MESSAGE {
                continue;
            }
            if cannot_be_edited {
                self.sent_again(messages, record.message, says);
                return;
            }
            let Ok(_lane) = lane.try_lock() else { continue };
            // **Read again under the lane**: a cut that ended the record since
            // the read above has the last word (Sol's fresh look, round 1).
            if messages.store.the_jobs_message_of(&record.job).map(|now| now.message) != Some(record.message) {
                continue;
            }
            let Some((text, markup)) = self.what_it_shows(messages, record.message, says) else { continue };
            let mut kept = kept(self);
            let one = kept.by_message.entry(record.message).or_default();
            if one.ending || one.last_accepted.as_ref() == Some(&(text.clone(), markup.clone())) {
                continue;
            }
            let (chat, message, words, buttons) = (messages.conversation, record.message, text.clone(), markup.clone());
            let told = messages.queue.offer(
                AClassOfSend::TheJobsMessagesEdit,
                Box::new(move |c: &TheClient| c.edit_marked_message(chat, message, &words, Some(buttons.clone()))),
            );
            one.outstanding = Some((text, markup, told));
            return;
        }
    }

    /// **C7a — the job's message sent again**, with the current words and
    /// buttons, without a sound, and recorded in place of the one that could
    /// not be edited, which is not touched.
    fn sent_again(&self, messages: &mut crate::actions::TheJobMessages, old: i64, says: &WhatAJobsMessageSays) {
        // **Under the old message's lane, and only while it is still the
        // job's live message** — a cut since the tick's read has the last word
        // (Sol's fresh look, round 1).
        let lane = self.the_lane_of(old);
        let _lane = lane.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        let job = says.subject.job.clone().unwrap_or_default();
        let ended = kept(self).by_message.get(&old).is_some_and(|one| one.ending);
        if ended || messages.store.the_jobs_message_of(&job).map(|record| record.message) != Some(old) {
            return;
        }
        let composed = AComposedMessage {
            text: self.the_words(says, true, &messages.queue),
            buttons: TheWantedButtons::for_the(&says.state).as_offered(says),
        };
        let sent = messages.send_the_jobs_message(&composed, &says.subject, crate::transport::HowItIsSent::silent(true));
        if !matches!(sent, Err(crate::actions::WhyAMessageWasNotSent::NotSent(_))) {
            kept(self).by_message.remove(&old);
        }
    }

    /// **Wait out the tick's edit of `message`, where one is outstanding**, so
    /// an edit made at once is never overtaken by one offered before it.
    fn the_ticks_edit_is_waited_out(&self, queue: &TheSendQueue, message: i64) {
        let outstanding = kept(self).by_message.get_mut(&message).and_then(|one| one.outstanding.take());
        if let Some((_, _, told)) = outstanding {
            let _ = crate::actions::wait_for(queue, told);
        }
    }

    fn the_lane_of(&self, message: i64) -> Arc<Mutex<()>> {
        kept(self).by_message.entry(message).or_default().lane.clone()
    }

    /// **One edit made at once on `message`**, in `class`, through its lane,
    /// after any tick's edit of it. What Telegram accepted is kept.
    fn edited_at_once(
        &self,
        messages: &mut crate::actions::TheJobMessages,
        message: i64,
        text: AMarkedText,
        markup: Value,
        class: AClassOfSend,
    ) -> HowASendEnded {
        self.the_ticks_edit_is_waited_out(&messages.queue, message);
        let (chat, words, buttons) = (messages.conversation, text.clone(), markup.clone());
        let ended = crate::actions::wait_for(
            &messages.queue,
            messages.queue.offer(class, Box::new(move |c: &TheClient| c.edit_marked_message(chat, message, &words, Some(buttons.clone())))),
        );
        if the_edit_landed(&ended) {
            kept(self).by_message.entry(message).or_default().last_accepted = Some((text, markup));
        }
        ended
    }

    /// **`Telegram` C1e — the job's message cut to one line**, every button
    /// off in the same act, in the endings class, after its ending's own
    /// message went. **Its C7a record ends** once the cut is accepted, or once
    /// Telegram says the message can no longer be edited, and nothing else is
    /// sent. Its buttons' records are marked removed and kept, so a late press
    /// still answers *This job has already ended.* (C1b). No record, no cut.
    pub fn the_cut(
        &self,
        messages: &mut crate::actions::TheJobMessages,
        job: &str,
        server: &(impl TheServersName + ?Sized),
        label: &str,
        ended: &str,
    ) -> WhatTheEditCameTo {
        let Some(record) = messages.store.the_jobs_message_of(job) else {
            return WhatTheEditCameTo::NoJobsMessage;
        };
        let lane = self.the_lane_of(record.message);
        let _lane = lane.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        // **Read again under the lane**: a deleted message sent again while
        // this waited is the one to cut (Sol's fresh look, round 2).
        if messages.store.the_jobs_message_of(job).map(|now| now.message) != Some(record.message) {
            drop(_lane);
            return self.the_cut(messages, job, server, label, ended);
        }
        kept(self).by_message.entry(record.message).or_default().ending = true;
        let text = crate::job_messages::the_jobs_message_cut(server, label, ended);
        let markup = json!({ "inline_keyboard": [] });
        // **Where Telegram already accepted this very line** — the ending
        // told at once (C1e as changed 2026-11-10) — nothing is sent again;
        // the record ends. After a restart nothing is remembered, and
        // Telegram's *message is not modified* counts as landed.
        let already = kept(self).by_message.get(&record.message).and_then(|one| one.last_accepted.clone())
            == Some((text.clone(), markup.clone()));
        let came_to = if already {
            HowASendEnded::Accepted(Value::Null)
        } else {
            self.edited_at_once(messages, record.message, text, markup, AClassOfSend::EndingsHoldsAndDrops)
        };
        self.what_a_last_edit_came_to(messages, &record.job, record.message, came_to)
    }

    /// **`Telegram` C1e as changed 2026-11-12 — a cut given up**, after
    /// Telegram refused it three times for a reason other than *cannot be
    /// edited* (the Fable review, sitting 6): the message's Stop contexts are
    /// marked removed, as an accepted cut marks them, and its C7a record
    /// ends; nothing is sent. Gives back whether the record ended — a store
    /// that could not be written leaves it for a later turn. No record, and
    /// there is nothing to give up.
    pub fn the_cut_is_given_up(&self, messages: &mut crate::actions::TheJobMessages, job: &str) -> bool {
        let Some(record) = messages.store.the_jobs_message_of(job) else {
            return true;
        };
        let lane = self.the_lane_of(record.message);
        let _lane = lane.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        let _ = messages.every_record_on_it_is_marked_removed(record.message);
        if messages.store.the_jobs_message_ends(job).is_err() {
            return false;
        }
        let mut kept = kept(self);
        let one = kept.by_message.entry(record.message).or_default();
        one.ending = true;
        one.outstanding = None;
        one.buttons = None;
        kept.lines.remove(job);
        true
    }

    /// **`Telegram` C1e as changed 2026-11-10 — the ending's line shown at
    /// once**, when the ending is read and before the ending's own message,
    /// which for a finished job waits for its last frame's pictures (the
    /// twenty-ninth notes, note 11). The same line and the same act as the
    /// cut — every button off, its records marked removed and kept, any tick's
    /// edit waited out first, no tick after — **but the C7a record stays**,
    /// so the pictures still reply to the message (I2). The cut ends it. A
    /// refusal leaves the record as it was, for the cut to try.
    pub fn the_ending_told(
        &self,
        messages: &mut crate::actions::TheJobMessages,
        job: &str,
        server: &(impl TheServersName + ?Sized),
        label: &str,
        ended: &str,
    ) -> WhatTheEditCameTo {
        let Some(record) = messages.store.the_jobs_message_of(job) else {
            return WhatTheEditCameTo::NoJobsMessage;
        };
        let lane = self.the_lane_of(record.message);
        let _lane = lane.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        // **Read again under the lane**, as the cut does: a deleted message
        // sent again while this waited is the one to tell.
        if messages.store.the_jobs_message_of(job).map(|now| now.message) != Some(record.message) {
            drop(_lane);
            return self.the_ending_told(messages, job, server, label, ended);
        }
        kept(self).by_message.entry(record.message).or_default().ending = true;
        let text = crate::job_messages::the_jobs_message_cut(server, label, ended);
        let came_to = self.edited_at_once(
            messages,
            record.message,
            text,
            json!({ "inline_keyboard": [] }),
            AClassOfSend::EndingsHoldsAndDrops,
        );
        if the_edit_landed(&came_to) {
            let _ = messages.every_record_on_it_is_marked_removed(record.message);
            let mut kept = kept(self);
            let one = kept.by_message.entry(record.message).or_default();
            one.outstanding = None;
            one.buttons = None;
            return WhatTheEditCameTo::Done;
        }
        if it_cannot_be_edited(&came_to) {
            return WhatTheEditCameTo::ItCannotBeEdited;
        }
        match came_to {
            HowASendEnded::HandedBack(WhyTheCallFailed::NotReached(_)) => WhatTheEditCameTo::TheLineReachedNothing,
            other => WhatTheEditCameTo::Refused(other),
        }
    }

    /// The record ended where the edit landed or cannot land.
    fn what_a_last_edit_came_to(
        &self,
        messages: &mut crate::actions::TheJobMessages,
        job: &str,
        message: i64,
        came_to: HowASendEnded,
    ) -> WhatTheEditCameTo {
        if the_edit_landed(&came_to) || it_cannot_be_edited(&came_to) {
            let _ = messages.every_record_on_it_is_marked_removed(message);
            if messages.store.the_jobs_message_ends(job).is_err() {
                return WhatTheEditCameTo::Refused(came_to);
            }
            // **Kept as ended, never forgotten** (Sol's fresh look, round 1):
            // a tick that read the record before it ended would otherwise
            // make the message anew and edit it back.
            let mut kept = kept(self);
            let one = kept.by_message.entry(message).or_default();
            one.ending = true;
            one.outstanding = None;
            one.buttons = None;
            kept.lines.remove(job);
            return if the_edit_landed(&came_to) { WhatTheEditCameTo::Done } else { WhatTheEditCameTo::ItCannotBeEdited };
        }
        kept(self).by_message.entry(message).or_default().ending = true;
        match came_to {
            HowASendEnded::HandedBack(WhyTheCallFailed::NotReached(_)) => WhatTheEditCameTo::TheLineReachedNothing,
            other => WhatTheEditCameTo::Refused(other),
        }
    }

    /// **`Telegram` A3 — the answer applied, told by an edit of the question**,
    /// which is the held range's job's message: its words become the sentence
    /// and its buttons go. **On a *yes* it stays the job's message** and the
    /// tick shows the range's progress from then on; **on a *no*, or a drop,
    /// that edit is its last** and its record ends. No record for the job —
    /// a question an older release sent — and nothing is edited.
    pub fn the_answer_in_place(
        &self,
        messages: &mut crate::actions::TheJobMessages,
        job: &str,
        text: AMarkedText,
        it_stays: bool,
    ) -> WhatTheEditCameTo {
        let Some(record) = messages.store.the_jobs_message_of(job) else {
            return WhatTheEditCameTo::NoJobsMessage;
        };
        let lane = self.the_lane_of(record.message);
        let _lane = lane.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        if messages.store.the_jobs_message_of(job).map(|now| now.message) != Some(record.message) {
            drop(_lane);
            return self.the_answer_in_place(messages, job, text, it_stays);
        }
        if !it_stays {
            kept(self).by_message.entry(record.message).or_default().ending = true;
        }
        let came_to =
            self.edited_at_once(messages, record.message, text, json!({ "inline_keyboard": [] }), AClassOfSend::EndingsHoldsAndDrops);
        if !it_stays {
            return self.what_a_last_edit_came_to(messages, &record.job, record.message, came_to);
        }
        if the_edit_landed(&came_to) {
            WhatTheEditCameTo::Done
        } else if it_cannot_be_edited(&came_to) {
            kept(self).by_message.entry(record.message).or_default().cannot_be_edited = true;
            WhatTheEditCameTo::ItCannotBeEdited
        } else if matches!(came_to, HowASendEnded::HandedBack(WhyTheCallFailed::NotReached(_))) {
            WhatTheEditCameTo::TheLineReachedNothing
        } else {
            WhatTheEditCameTo::Refused(came_to)
        }
    }

    /// **`Telegram` D0 — a Stop pressed on the job's own message asks in
    /// place**: the message's words gain the question's, and its buttons
    /// become the question's two, each written as a record against it first
    /// (R1), *Keep rendering*'s carrying the question's words so a restart
    /// keeps it open. In the first class of C6, outside the tick. **Only where
    /// `message` is still the job's live message**; otherwise
    /// [`WhatTheEditCameTo::NoJobsMessage`], and the question goes as a new
    /// message, as before.
    pub fn the_question_in_place(
        &self,
        messages: &mut crate::actions::TheJobMessages,
        message: i64,
        question: &AComposedMessage,
        words: &str,
        subject: &TheSubject,
    ) -> WhatTheEditCameTo {
        let job = subject.job.clone().unwrap_or_default();
        let lane = self.the_lane_of(message);
        let _lane = lane.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        if messages.store.the_jobs_message_of(&job).map(|record| record.message) != Some(message) {
            return WhatTheEditCameTo::NoJobsMessage;
        }
        let says = {
            let kept = kept(self);
            let one = kept.by_message.get(&message);
            if one.is_some_and(|one| one.ending) {
                return WhatTheEditCameTo::NoJobsMessage;
            }
            one.and_then(|one| one.says.clone())
        };
        // **A question already open on it is retired first** — a retry, or
        // two quick presses — so its buttons are never shown twice (Sol's
        // fresh look, round 2). Its records stay, marked, for a late press.
        if let Ok(records) = messages.store.the_action_contexts_bound_to(message) {
            for mut record in records.into_iter().filter(|r| {
                r.payload["confirmation_message"] == json!(message) && r.payload["stop_removed"] != json!(true)
            }) {
                record.payload["stop_removed"] = json!(true);
                let _ = messages.store.record_an_action_context(record);
            }
        }
        let mut composed = question.clone();
        for button in &mut composed.buttons {
            button.payload[THE_QUESTIONS_WORDS] = json!(words);
        }
        let Ok(markup) = messages.the_buttons_recorded_for(message, &composed, subject) else {
            return WhatTheEditCameTo::Refused(HowASendEnded::DroppedAsSuperseded);
        };
        let mut text = match &says {
            Some(says) => self.the_words(says, false, &messages.queue),
            None => question.text.clone(),
        };
        if says.is_some() {
            text.push_str(&format!("\n{words}"));
        }
        let came_to = self.edited_at_once(messages, message, text, markup, AClassOfSend::RoutedAnswersAndAcknowledgements);
        if the_edit_landed(&came_to) {
            WhatTheEditCameTo::Done
        } else if matches!(came_to, HowASendEnded::HandedBack(WhyTheCallFailed::NotReached(_))) {
            WhatTheEditCameTo::TheLineReachedNothing
        } else {
            WhatTheEditCameTo::Refused(came_to)
        }
    }

    /// **D0 — the question asked in place, answered**: its buttons' records
    /// are marked removed and the job's message shows C1d's words and buttons
    /// again at once — *Keep rendering* puts them back; after *Stop this job*
    /// the tick then says *Stopping.* (C1). Gives back whether `message` was a
    /// job's message this did that for; where nothing is known of it yet —
    /// after a restart — the caller takes the buttons off as before, and the
    /// tick puts C1d's back.
    pub fn the_question_is_answered(&self, messages: &mut crate::actions::TheJobMessages, message: i64) -> bool {
        // **The lane first, then every check** — a cut that went while this
        // waited has the last word (Sol's fresh look, round 1).
        let lane = self.the_lane_of(message);
        let _lane = lane.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        let Some(says) = kept(self).by_message.get(&message).filter(|one| !one.ending).and_then(|one| one.says.clone()) else {
            return false;
        };
        let job = says.subject.job.clone().unwrap_or_default();
        if messages.store.the_jobs_message_of(&job).map(|record| record.message) != Some(message) {
            return false;
        }
        if messages.every_record_on_it_is_marked_removed(message).is_err() {
            return false;
        }
        if let Some(one) = kept(self).by_message.get_mut(&message) {
            one.last_accepted = None;
        }
        let Some((text, markup)) = self.what_it_shows(messages, message, &says) else { return true };
        let _ = self.edited_at_once(messages, message, text, markup, AClassOfSend::RoutedAnswersAndAcknowledgements);
        true
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    /// **`Telegram` C1's addition of 2026-11-13 (the scout's list, item 3):
    /// at 100% the job line says Blender is finishing the frame**, on the
    /// status message and the job's own alike — *Rendering 100%* stood for 12
    /// minutes on two live passes while Blender denoised and composited.
    #[test]
    fn at_a_hundred_percent_the_job_line_says_finishing() {
        let rendering = |frame, of, percent, finishing| WhatTheMachineIsDoing::Rendering {
            job_label: "kitchen_v14".into(),
            detail: None,
            frame,
            of,
            percent,
            this_frame: None,
            finishing,
        };
        // **Sol's look at the scout's list, point 5: the frame in hand.**
        // Frame 1 of 240 finishing at the job's 0%, and a full job whose frame
        // still samples (a wrong plan total) saying nothing early.
        assert_eq!(the_state_line(&rendering(1, 240, 0, true)).as_deref(), Some("Finishing: frame 1 of 240."));
        assert_eq!(the_state_line(&rendering(1, 1, 100, true)).as_deref(), Some("Finishing: frame 1 of 1."));
        assert_eq!(the_state_line(&rendering(1, 1, 100, false)).as_deref(), Some("Rendering: frame 1 of 1 \u{b7} 100%"));
        assert_eq!(the_state_line(&rendering(1, 1, 99, false)).as_deref(), Some("Rendering: frame 1 of 1 \u{b7} 99%"));
        assert_eq!(the_state_line(&rendering(0, 1, 100, true)).as_deref(), Some("Rendering."), "no figure, nothing more");
        // **The job's own message, by the same rule.**
        let job = AJobsState::Rendering { frame: 3, of: 240, percent: 1, a_frame_is_finished: true, finishing: true };
        assert_eq!(job.the_line("kitchen_v14").as_deref().map(|line| line.contains("Finishing: frame 3 of 240.")), Some(true));
    }
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
                finishing: false,
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

    /// **The Creator's notes on the test copy, note 2 (`Telegram` C2a as
    /// changed 2026-11-13): the server's message shows its age as the job's
    /// does** — its *Updated* line carries C1d's `date_time` entity, format
    /// `r`, holding the tick's moment, so a phone counts it up; the text is
    /// C2a's moment as before, and uptime stays a length of time. **Failed
    /// before the change**: the line was plain text, a clock time nothing
    /// counted.
    #[test]
    fn notes_2_the_servers_message_says_its_age_as_the_jobs_message_does() {
        let composed = compose(&rendering(), at(THE_DESIGNS_MOMENT));
        assert!(composed.text.ends_with("\nUptime: 4 hours 12 minutes\nUpdated 12 Aug 2026, 14:22:07 UTC"), "{}", composed.text);
        let moments: Vec<&Value> = composed.entities.iter().filter(|e| e["type"] == "date_time").collect();
        assert_eq!(moments.len(), 1, "{:#?}", composed.entities);
        assert_eq!(moments[0]["unix_time"], json!(THE_DESIGNS_MOMENT));
        assert_eq!(moments[0]["date_time_format"], json!("r"));
        let utf16: Vec<u16> = composed.text.encode_utf16().collect();
        let (offset, length) = (moments[0]["offset"].as_u64().unwrap() as usize, moments[0]["length"].as_u64().unwrap() as usize);
        assert_eq!(offset + length, utf16.len());
        assert_eq!(String::from_utf16(&utf16[offset..]).unwrap(), "12 Aug 2026, 14:22:07 UTC");
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
        assert!(compose(&status, at(THE_DESIGNS_MOMENT)).contains("Disk free: 61 GB (as of 12 Aug 2026, 14:22:03 UTC)"));
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
            finishing: false,
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

    /// **The Fable review, sitting 6 — `Telegram` F3c: a status message
    /// whose identity could not be written is not sent again.** Until
    /// 2026-11-12 a store that could not be written left the worker with no
    /// status message it knew of, so every tick sent a new one, each with its
    /// sound.
    #[test]
    fn sitting_6_a_status_message_whose_identity_cannot_be_written_is_sent_once() {
        let service = Arc::new(AStandInService::new());
        service.then_answers(r#"{"ok":true,"result":{"message_id":31}}"#);
        let queue = a_queue(&service);
        let (mut store, stops_writes) = crate::worker_state::a_file_store_whose_writes_can_be_stopped("status");
        std::fs::create_dir(&stops_writes).expect("writes stopped");
        let mut clock = TheClock::for_the_conversation(CHAT);
        for tick in 0..5 {
            let _ = clock.a_tick(&mut store, &queue, &rendering(), None, at(THE_DESIGNS_MOMENT + 5 * tick));
            queue.send_the_next();
        }
        let _ = clock.take_in_what_ended(&mut store);
        assert_eq!(store.the_status_message().unwrap(), None, "the store could not be written");
        // The tick that reads the failed write says so and offers nothing;
        // every tick after it edits the one message.
        assert_eq!(
            methods(&service),
            ["sendMessage", "editMessageText", "editMessageText", "editMessageText"],
            "one message, then edits of it"
        );
        for call in service.heard().iter().filter(|call| call.method == "editMessageText") {
            let WhatACallCarries::Fields(fields) = &call.carries else { panic!("text calls carry fields") };
            assert_eq!(fields["message_id"], 31);
        }
        // The store can be written again: the identity is written at the next tick.
        std::fs::remove_dir(&stops_writes).expect("writes let through");
        clock.a_tick(&mut store, &queue, &rendering(), None, at(THE_DESIGNS_MOMENT + 30)).unwrap();
        assert_eq!(store.the_status_message().unwrap(), Some(31));
        let _ = std::fs::remove_dir_all(stops_writes.parent().expect("its scratch folder"));
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

    /// **`Telegram` C7's sentence of 2026-11-12 (Sol's look at the
    /// double-check ideas, finding 5): the superseded mark is tried again
    /// until Telegram accepts it or refuses it as a bad request.** The old
    /// message reads *Rendering 100%*; its edit is refused as a bad request
    /// and a new message is created; the mark on the old one meets a passing
    /// non-flood refusal. The live message moves on the next tick and the mark is
    /// tried again on the one after, and lands. **It failed before the fix**:
    /// any refusal ended the mark, so the old message stayed at *Rendering
    /// 100%* for good beside the new one. *(A flood refusal never reached
    /// this: the send queue drops a flooded clock edit, C6a, and the mark was
    /// already offered again.)*
    #[test]
    fn double_check_sol_5_a_superseded_mark_that_meets_a_passing_refusal_is_tried_again() {
        let service = Arc::new(AStandInService::new());
        service
            .then_answers(r#"{"ok":false,"error_code":400,"description":"Bad Request: message to edit not found"}"#)
            .then_answers(r#"{"ok":true,"result":{"message_id":90}}"#)
            .then_answers(r#"{"ok":false,"error_code":403,"description":"Forbidden"}"#);
        let queue = a_queue(&service);
        let mut store = InMemory::with_routing_key("k");
        store.record_the_status_message(88).unwrap();
        let mut clock = TheClock::for_the_conversation(CHAT);

        let mut did = Vec::new();
        for tick in 0..7 {
            did.push(clock.a_tick(&mut store, &queue, &rendering(), None, at(THE_DESIGNS_MOMENT + 5 * tick)).unwrap());
            queue.send_the_next();
        }
        assert_eq!(
            did,
            [
                WhatATickDid::OfferedAnEdit(88),
                WhatATickDid::OfferedACreation { replacing: Some(88) },
                WhatATickDid::OfferedTheSupersededMark(88),
                // The live message moves between two tries of the mark.
                WhatATickDid::OfferedAnEdit(90),
                WhatATickDid::OfferedTheSupersededMark(88),
                // Accepted, so it is not offered again.
                WhatATickDid::OfferedAnEdit(90),
                WhatATickDid::OfferedAnEdit(90),
            ]
        );
    }

    /// **And the third refusal that is not a bad request ends the mark** (the
    /// same sentence), so one that never passes does not keep taking every
    /// second tick from the live message.
    #[test]
    fn double_check_sol_5_a_superseded_mark_refused_three_times_is_given_up() {
        let service = Arc::new(AStandInService::new());
        let queue = a_queue(&service);
        let mut store = InMemory::with_routing_key("k");
        store.record_the_status_message(88).unwrap();
        let mut clock = TheClock::for_the_conversation(CHAT);
        let mut did = Vec::new();
        for tick in 0..10 {
            let offered = clock.a_tick(&mut store, &queue, &rendering(), None, at(THE_DESIGNS_MOMENT + 5 * tick)).unwrap();
            // The old message cannot be edited, the new one is made and
            // edited, and every try of the mark meets a permanent refusal.
            service.then_answers(match offered {
                WhatATickDid::OfferedAnEdit(88) => r#"{"ok":false,"error_code":400,"description":"Bad Request: message to edit not found"}"#,
                WhatATickDid::OfferedTheSupersededMark(_) => r#"{"ok":false,"error_code":403,"description":"Forbidden"}"#,
                _ => r#"{"ok":true,"result":{"message_id":90}}"#,
            });
            did.push(offered);
            queue.send_the_next();
        }
        let marks = did.iter().filter(|one| matches!(one, WhatATickDid::OfferedTheSupersededMark(88))).count();
        assert_eq!(marks, 3, "{did:?}");
        assert_eq!(did[did.len() - 3..], [WhatATickDid::OfferedAnEdit(90), WhatATickDid::OfferedAnEdit(90), WhatATickDid::OfferedAnEdit(90)], "{did:?}");
    }

    #[test]
    fn bad_batch_1_service_errors_do_not_use_the_three_refusal_cap() {
        let service = Arc::new(AStandInService::new());
        let queue = a_queue(&service);
        let mut store = InMemory::with_routing_key("k");
        store.record_the_status_message(88).unwrap();
        let mut clock = TheClock::for_the_conversation(CHAT);
        let mut marks = 0;
        for tick in 0..12 {
            let offered = clock.a_tick(&mut store, &queue, &rendering(), None,
                at(THE_DESIGNS_MOMENT + 5 * tick)).unwrap();
            service.then_answers(match offered {
                WhatATickDid::OfferedAnEdit(88) => r#"{"ok":false,"error_code":400}"#,
                WhatATickDid::OfferedTheSupersededMark(_) => {
                    marks += 1;
                    if marks <= 4 { r#"{"ok":false,"error_code":503}"# }
                    else { r#"{"ok":true,"result":true}"# }
                }
                _ => r#"{"ok":true,"result":{"message_id":90}}"#,
            });
            queue.send_the_next();
        }
        assert_eq!(marks, 5, "temporary service errors must leave the mark owed until accepted");
    }

    /// **A second replacement keeps the first mark owed** (Sol's look at this
    /// fix): 88 is replaced by 90 and its mark meets a server error; 90 then
    /// cannot be edited and is replaced by 91. Both 88 and 90 are marked.
    /// With one slot, 90 took 88's place and 88 stayed at its last words.
    #[test]
    fn double_check_sol_5_a_second_replacement_keeps_the_first_mark_owed() {
        let service = Arc::new(AStandInService::new());
        let queue = a_queue(&service);
        let mut store = InMemory::with_routing_key("k");
        store.record_the_status_message(88).unwrap();
        let mut clock = TheClock::for_the_conversation(CHAT);
        let (mut did, mut the_first_mark_was_refused, mut made) = (Vec::new(), false, 89);
        for tick in 0..12 {
            let offered = clock.a_tick(&mut store, &queue, &rendering(), None, at(THE_DESIGNS_MOMENT + 5 * tick)).unwrap();
            let answer = match offered {
                // Neither 88 nor 90 can be edited; 91 can.
                WhatATickDid::OfferedAnEdit(88 | 90) => r#"{"ok":false,"error_code":400,"description":"Bad Request: message to edit not found"}"#.to_string(),
                WhatATickDid::OfferedACreation { .. } => {
                    made += 1;
                    format!(r#"{{"ok":true,"result":{{"message_id":{made}}}}}"#)
                }
                // The first try of 88's mark meets a server error.
                WhatATickDid::OfferedTheSupersededMark(88) if !the_first_mark_was_refused => {
                    the_first_mark_was_refused = true;
                    r#"{"ok":false,"error_code":500,"description":"Internal Server Error"}"#.to_string()
                }
                _ => r#"{"ok":true,"result":{"message_id":91}}"#.to_string(),
            };
            service.then_answers(&answer);
            did.push(offered);
            queue.send_the_next();
        }
        let marked = |which| did.iter().filter(|one| **one == WhatATickDid::OfferedTheSupersededMark(which)).count();
        assert_eq!((marked(88), marked(90)), (2, 1), "{did:?}");
        assert_eq!(did.last(), Some(&WhatATickDid::OfferedAnEdit(91)), "{did:?}");
    }

    /// **And a bad request still ends the mark** (the same sentence): the old
    /// message is gone or cannot be edited, and nothing more can be done.
    #[test]
    fn double_check_sol_5_a_superseded_mark_refused_as_a_bad_request_is_given_up() {
        let service = Arc::new(AStandInService::new());
        service
            .then_answers(r#"{"ok":false,"error_code":400,"description":"Bad Request: message to edit not found"}"#)
            .then_answers(r#"{"ok":true,"result":{"message_id":90}}"#)
            .then_answers(r#"{"ok":false,"error_code":400,"description":"Bad Request: message to edit not found"}"#);
        let queue = a_queue(&service);
        let mut store = InMemory::with_routing_key("k");
        store.record_the_status_message(88).unwrap();
        let mut clock = TheClock::for_the_conversation(CHAT);
        let mut did = Vec::new();
        for tick in 0..6 {
            did.push(clock.a_tick(&mut store, &queue, &rendering(), None, at(THE_DESIGNS_MOMENT + 5 * tick)).unwrap());
            queue.send_the_next();
        }
        assert_eq!(
            did[2..],
            [
                WhatATickDid::OfferedTheSupersededMark(88),
                WhatATickDid::OfferedAnEdit(90),
                WhatATickDid::OfferedAnEdit(90),
                WhatATickDid::OfferedAnEdit(90),
            ]
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
        // **One word for a waiting job, the app's own** — the four questions,
        // scout 3.29, 2026-11-11: the phone said *Queued.*
        assert!(compose(&status, at(THE_DESIGNS_MOMENT)).contains("\nTurntable\nWaiting.\n"));
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

    // ------------------------------------------------------------ C1d's keeper

    fn a_jobs_messages(service: &Arc<AStandInService>) -> (crate::actions::TheJobMessages, Arc<std::sync::Mutex<Box<dyn TheTelegramWorkersState>>>) {
        let store: Arc<std::sync::Mutex<Box<dyn TheTelegramWorkersState>>> =
            Arc::new(std::sync::Mutex::new(Box::new(InMemory::with_routing_key("00112233445566778899aabbccddeeff"))));
        let messages = crate::actions::TheJobMessages {
            conversation: CHAT,
            store: Box::new(store.clone()),
            queue: Arc::new(a_queue(service)),
            notes: Box::new(|_| {}),
        };
        (messages, store)
    }

    fn kitchen(state: AJobsState) -> WhatAJobsMessageSays {
        WhatAJobsMessageSays {
            subject: TheSubject { owning_machine: "m".into(), job: Some("job-1".into()), hold: None, generation: "op:1".into() },
            server: AServerLabel::with_colour("RunPod 4090", Some("red")),
            label: "Kitchen".into(),
            state,
            at: None,
        }
    }

    #[test]
    fn a_job_run_again_says_so_on_its_start_and_next_tick() {
        let service = Arc::new(AStandInService::new());
        service.then_answers(r#"{"ok":true,"result":{"message_id":40}}"#);
        let (mut messages, _) = a_jobs_messages(&service);
        let keeper = TheJobsOwnMessages::default();
        let says = kitchen(AJobsState::RunningAgain);
        assert_eq!(keeper.the_start(&mut messages, &says, true, true), Ok(40));
        let heard = service.heard();
        assert!(the_fields(&heard[0])["text"].as_str().unwrap().ends_with("Interrupted. Retrying."));
        assert_eq!(says.state.the_line("Kitchen").as_deref(), Some("Interrupted. Retrying."));
        let mut status = rendering();
        status.doing = WhatTheMachineIsDoing::RunningAgain { job_label: "Kitchen".into() };
        let text = compose(&status, at(THE_DESIGNS_MOMENT));
        assert!(text.contains("Kitchen\nInterrupted. Retrying.\nGPU memory:"), "{text}");
        assert!(!text.contains("frame") && !text.contains('%') && !text.contains("This frame"));
        assert_eq!(TheWantedButtons::for_the(&says.state).a_frame, false);
        assert!(kitchen_rendering(1, false).state.the_line("Kitchen").unwrap().starts_with("Rendering:"));
        assert_eq!(the_labels(&the_fields(&heard[1])["reply_markup"]), [["Stop this job"]]);
    }

    fn kitchen_rendering(frame: u64, a_frame_is_finished: bool) -> WhatAJobsMessageSays {
        kitchen(AJobsState::Rendering { frame, of: 240, percent: (frame * 100 / 240) as u8, a_frame_is_finished, finishing: false })
    }

    fn the_fields(call: &crate::transport::a_stand_in::ACallItHeard) -> serde_json::Map<String, Value> {
        match &call.carries {
            crate::transport::WhatACallCarries::Fields(fields) => fields.clone(),
            crate::transport::WhatACallCarries::FieldsAndFiles { fields, .. } => fields.clone(),
            _ => panic!("a call with no fields"),
        }
    }

    fn the_labels(markup: &Value) -> Vec<Vec<String>> {
        markup["inline_keyboard"]
            .as_array()
            .unwrap()
            .iter()
            .map(|row| row.as_array().unwrap().iter().map(|b| b["text"].as_str().unwrap().to_string()).collect())
            .collect()
    }

    /// **`Telegram` C1a, C1d, C7a and V1e — the job's message at its start**:
    /// *Started.* with *Stop this job*, without a sound unless the choice is
    /// `Everything`, its record written once Telegram accepts it, and **no
    /// second message where one is already recorded** (a restart replaying the
    /// start, a held range's question). The twenty-eighth notes, note 18.
    #[test]
    fn twenty_eighth_18_the_start_is_the_jobs_message_recorded_once() {
        let service = Arc::new(AStandInService::new());
        service.then_answers(r#"{"ok":true,"result":{"message_id":40}}"#);
        let (mut messages, store) = a_jobs_messages(&service);
        let keeper = TheJobsOwnMessages::default();
        assert_eq!(keeper.the_start(&mut messages, &kitchen_rendering(0, false), true, true), Ok(40));
        let heard = service.heard();
        assert_eq!(heard[0].method, "sendMessage");
        assert_eq!(the_fields(&heard[0])["text"], json!("🟥 RunPod 4090\nKitchen\nStarted."));
        assert_eq!(the_fields(&heard[0])["disable_notification"], json!(true));
        assert!(!the_fields(&heard[0]).contains_key("reply_markup"), "R1: no button before its record");
        assert_eq!(heard[1].method, "editMessageText");
        assert_eq!(the_labels(&the_fields(&heard[1])["reply_markup"]), [["Stop this job"]]);
        let record = store.the_jobs_message_of("job-1").expect("C7a's record");
        assert_eq!((record.message, record.generation.as_str()), (40, "op:1"));
        let stop = &store.the_action_contexts_bound_to(40).unwrap()[0];
        assert_eq!(stop.payload[THE_JOBS_MESSAGE], json!(40), "the Stop knows its job's message");
        // Replayed after a restart: nothing more is sent.
        assert_eq!(keeper.the_start(&mut messages, &kitchen_rendering(0, false), true, false), Ok(40));
        assert_eq!(service.heard().len(), 2);
    }

    /// **`Telegram` C1d — the tick edits the job's message only where it
    /// changed**: the job's line in C1's words, *Send a frame* beside *Stop
    /// this job* on one row from the first finished frame, one edit
    /// outstanding, and nothing sent while it says the same.
    #[test]
    fn twenty_eighth_18_the_tick_edits_only_what_changed() {
        let service = Arc::new(AStandInService::new());
        service.then_answers(r#"{"ok":true,"result":{"message_id":40}}"#);
        let (mut messages, _store) = a_jobs_messages(&service);
        let keeper = TheJobsOwnMessages::default();
        keeper.the_start(&mut messages, &kitchen_rendering(0, false), true, false).unwrap();
        let before = service.heard().len();
        keeper.a_tick(&mut messages, &[kitchen_rendering(31, true)]);
        keeper.a_tick(&mut messages, &[kitchen_rendering(31, true)]);
        assert_eq!(messages.queue.how_many_are_waiting(), 1, "one edit outstanding");
        while messages.queue.send_the_next().is_some() {}
        let heard = service.heard();
        assert_eq!(heard.len(), before + 1);
        let edit = the_fields(&heard[before]);
        assert_eq!(edit["text"], json!("🟥 RunPod 4090\nKitchen\nRendering: frame 31 of 240 \u{b7} 12%"));
        assert_eq!(the_labels(&edit["reply_markup"]), [["Send a frame", "Stop this job"]]);
        // Telegram accepted it: the same again sends nothing.
        keeper.a_tick(&mut messages, &[kitchen_rendering(31, true)]);
        keeper.a_tick(&mut messages, &[kitchen_rendering(31, true)]);
        while messages.queue.send_the_next().is_some() {}
        assert_eq!(service.heard().len(), before + 1, "nothing changed, nothing sent");
        // A job not handed in — held on its question, or ended — is not edited.
        keeper.a_tick(&mut messages, &[]);
        while messages.queue.send_the_next().is_some() {}
        assert_eq!(service.heard().len(), before + 1);
    }

    /// **`Telegram` C1e — the cut waits out a tick's edit offered before it**,
    /// so the ending is the last word: one line, no buttons, its record
    /// ended, its buttons' records marked and kept, and **no tick edits it
    /// after**. Sol's ask: an ending arriving while the job's message is
    /// being edited.
    #[test]
    fn twenty_eighth_18_the_cut_is_the_last_word() {
        let service = Arc::new(AStandInService::new());
        service.then_answers(r#"{"ok":true,"result":{"message_id":40}}"#);
        let (mut messages, store) = a_jobs_messages(&service);
        let keeper = TheJobsOwnMessages::default();
        keeper.the_start(&mut messages, &kitchen_rendering(0, false), true, false).unwrap();
        keeper.a_tick(&mut messages, &[kitchen_rendering(200, true)]);
        assert_eq!(messages.queue.how_many_are_waiting(), 1, "the tick's edit waits in the queue");
        let server = AServerLabel::with_colour("RunPod 4090", Some("red"));
        assert_eq!(
            keeper.the_cut(&mut messages, "job-1", &server, "Kitchen", crate::job_messages::A_FINISHED_ENDING),
            WhatTheEditCameTo::Done
        );
        let heard = service.heard();
        let last = the_fields(heard.last().unwrap());
        assert_eq!(last["text"], json!("🟥 RunPod 4090 \u{b7} Kitchen \u{b7} Finished."));
        assert_eq!(last["reply_markup"], json!({ "inline_keyboard": [] }));
        assert!(the_fields(&heard[heard.len() - 2])["text"].as_str().unwrap().contains("frame 200"), "the tick's went first");
        assert_eq!(store.the_jobs_message_of("job-1"), None, "C7a's record ended");
        let records = store.the_action_contexts_bound_to(40).unwrap();
        assert!(!records.is_empty() && records.iter().all(|r| r.payload["stop_removed"] == json!(true)));
        let after = service.heard().len();
        keeper.a_tick(&mut messages, &[kitchen_rendering(240, true)]);
        while messages.queue.send_the_next().is_some() {}
        assert_eq!(service.heard().len(), after, "nothing edits a cut message");
        assert_eq!(
            keeper.the_cut(&mut messages, "job-1", &server, "Kitchen", crate::job_messages::A_FINISHED_ENDING),
            WhatTheEditCameTo::NoJobsMessage,
            "a second cut finds no record and sends nothing"
        );
    }

    /// **Sol's fresh look, round 1: a tick that read the job's record just
    /// before the cut ended it must not edit the cut message after.** Posed
    /// here by the record still standing when the tick runs, as the tick's
    /// earlier read would have it. **It failed before the fix**: the cut
    /// forgot the message, so the tick made it anew and edited it back.
    #[test]
    fn twenty_eighth_18_a_tick_that_read_the_record_before_the_cut_edits_nothing() {
        let service = Arc::new(AStandInService::new());
        service.then_answers(r#"{"ok":true,"result":{"message_id":40}}"#);
        let (mut messages, mut store) = a_jobs_messages(&service);
        let keeper = TheJobsOwnMessages::default();
        keeper.the_start(&mut messages, &kitchen_rendering(0, false), true, false).unwrap();
        let record = store.the_jobs_message_of("job-1").unwrap();
        let server = AServerLabel::with_colour("RunPod 4090", Some("red"));
        keeper.the_cut(&mut messages, "job-1", &server, "Kitchen", crate::job_messages::A_FINISHED_ENDING);
        store.record_a_jobs_message(record).unwrap();
        let after = service.heard().len();
        keeper.a_tick(&mut messages, &[kitchen_rendering(240, true)]);
        while messages.queue.send_the_next().is_some() {}
        assert_eq!(service.heard().len(), after, "the cut stays the last word");
    }

    /// **Sol's fresh look, round 1: I3's line that no edit has shown yet is
    /// handed back at the cut**, so it is said as a message of its own rather
    /// than lost with the job's message; once a tick has shown it, it is not.
    #[test]
    fn twenty_eighth_18_a_line_never_shown_is_handed_back_at_the_cut() {
        let service = Arc::new(AStandInService::new());
        service.then_answers(r#"{"ok":true,"result":{"message_id":40}}"#);
        let (mut messages, _store) = a_jobs_messages(&service);
        let keeper = TheJobsOwnMessages::default();
        keeper.the_start(&mut messages, &kitchen_rendering(0, false), true, false).unwrap();
        keeper.a_last_line("job-1", "Preview not sent: over Telegram's limits.");
        assert_eq!(keeper.the_lines_never_shown("job-1"), ["Preview not sent: over Telegram's limits."]);
        keeper.a_tick(&mut messages, &[kitchen_rendering(31, false)]);
        while messages.queue.send_the_next().is_some() {}
        keeper.a_tick(&mut messages, &[kitchen_rendering(31, false)]);
        assert!(keeper.the_lines_never_shown("job-1").is_empty(), "the tick showed it");
    }

    /// **Sol's fresh look, round 2: the question asked in place twice — a
    /// retry, or two quick presses — shows two buttons, not four.** It failed
    /// before the fix: the tick rebuilt the question from every pair written.
    #[test]
    fn twenty_eighth_18_a_question_asked_in_place_twice_shows_two_buttons() {
        let service = Arc::new(AStandInService::new());
        service.then_answers(r#"{"ok":true,"result":{"message_id":40}}"#);
        let (mut messages, _store) = a_jobs_messages(&service);
        let keeper = TheJobsOwnMessages::default();
        keeper.the_start(&mut messages, &kitchen_rendering(0, false), true, false).unwrap();
        keeper.a_tick(&mut messages, &[kitchen_rendering(31, false)]);
        let question = crate::job_messages::the_stop_question_for("RunPod 4090", "Kitchen", &crate::job_messages::AStopQuestion::ARender);
        let words = crate::job_messages::the_stop_questions_words(&crate::job_messages::AStopQuestion::ARender);
        let subject = kitchen_rendering(31, false).subject;
        keeper.the_question_in_place(&mut messages, 40, &question, &words, &subject);
        keeper.the_question_in_place(&mut messages, 40, &question, &words, &subject);
        let restarted = TheJobsOwnMessages::default();
        let before = service.heard().len();
        restarted.a_tick(&mut messages, &[kitchen_rendering(32, false)]);
        while messages.queue.send_the_next().is_some() {}
        let shown = the_fields(&service.heard()[before]);
        assert_eq!(the_labels(&shown["reply_markup"]), [["Keep rendering"], ["Stop this job"]]);
    }

    /// **Sol's fresh look, round 2: a line said at the cut is said once**,
    /// however often the cut is tried again. It failed before the fix (a
    /// compile failure: the method is new).
    #[test]
    fn twenty_eighth_18_a_line_said_at_the_cut_is_not_said_again() {
        let keeper = TheJobsOwnMessages::default();
        keeper.a_last_line("job-1", "Preview not sent: over Telegram's limits.");
        assert_eq!(keeper.the_lines_never_shown("job-1").len(), 1);
        keeper.the_lines_are_said("job-1");
        assert!(keeper.the_lines_never_shown("job-1").is_empty());
    }

    /// **`Telegram` C7a — the user deleted the job's message**: the tick's edit
    /// is refused as impossible, and the next tick sends the job's message
    /// again, without a sound, with the current words and buttons, recorded in
    /// its place. Sol's ask: the user deleting the job's message.
    #[test]
    fn twenty_eighth_18_a_deleted_jobs_message_is_sent_again_silently() {
        let service = Arc::new(AStandInService::new());
        service.then_answers(r#"{"ok":true,"result":{"message_id":40}}"#);
        let (mut messages, store) = a_jobs_messages(&service);
        let keeper = TheJobsOwnMessages::default();
        keeper.the_start(&mut messages, &kitchen_rendering(0, false), true, false).unwrap();
        service.then_answers(r#"{"ok":false,"error_code":400,"description":"Bad Request: message to edit not found"}"#);
        keeper.a_tick(&mut messages, &[kitchen_rendering(31, false)]);
        while messages.queue.send_the_next().is_some() {}
        service.then_answers(r#"{"ok":true,"result":{"message_id":41}}"#);
        let before = service.heard().len();
        keeper.a_tick(&mut messages, &[kitchen_rendering(32, false)]);
        let heard = service.heard();
        let sent = the_fields(&heard[before]);
        assert_eq!(heard[before].method, "sendMessage");
        assert_eq!(sent["disable_notification"], json!(true));
        assert!(sent["text"].as_str().unwrap().ends_with("Rendering: frame 32 of 240 \u{b7} 13%"));
        assert_eq!(store.the_jobs_message_of("job-1").unwrap().message, 41);
    }

    /// **`Telegram` D0 asked in place**: the job's message gains the question's
    /// words and its two buttons, *Keep rendering*'s record carrying the words,
    /// so a tick — even after a restart — keeps the question; answered, C1d's
    /// words and buttons come back at once.
    #[test]
    fn twenty_eighth_18_the_stop_question_in_place_stays_until_answered() {
        let service = Arc::new(AStandInService::new());
        service.then_answers(r#"{"ok":true,"result":{"message_id":40}}"#);
        let (mut messages, store) = a_jobs_messages(&service);
        let keeper = TheJobsOwnMessages::default();
        keeper.the_start(&mut messages, &kitchen_rendering(0, false), true, false).unwrap();
        keeper.a_tick(&mut messages, &[kitchen_rendering(31, false)]);
        let question = crate::job_messages::the_stop_question_for("RunPod 4090", "Kitchen", &crate::job_messages::AStopQuestion::ARender);
        let words = crate::job_messages::the_stop_questions_words(&crate::job_messages::AStopQuestion::ARender);
        let subject = kitchen_rendering(31, false).subject;
        assert_eq!(keeper.the_question_in_place(&mut messages, 40, &question, &words, &subject), WhatTheEditCameTo::Done);
        let heard = service.heard();
        let asked = the_fields(heard.last().unwrap());
        assert_eq!(asked["text"], json!("🟥 RunPod 4090\nKitchen\nRendering: frame 31 of 240 \u{b7} 12%\nStop this job?\nThe current frame may not be saved."));
        assert_eq!(the_labels(&asked["reply_markup"]), [["Keep rendering"], ["Stop this job"]]);
        // A restart: a new keeper knows nothing, and the tick keeps the question.
        let after_a_restart = TheJobsOwnMessages::default();
        let before = service.heard().len();
        after_a_restart.a_tick(&mut messages, &[kitchen_rendering(32, false)]);
        while messages.queue.send_the_next().is_some() {}
        let kept = the_fields(&service.heard()[before]);
        assert!(kept["text"].as_str().unwrap().ends_with("frame 32 of 240 \u{b7} 13%\nStop this job?\nThe current frame may not be saved."));
        assert_eq!(the_labels(&kept["reply_markup"]), [["Keep rendering"], ["Stop this job"]]);
        // *Keep rendering*: the question goes and C1d's buttons come back.
        assert!(after_a_restart.the_question_is_answered(&mut messages, 40));
        let back = the_fields(service.heard().last().unwrap());
        assert_eq!(back["text"], json!("🟥 RunPod 4090\nKitchen\nRendering: frame 32 of 240 \u{b7} 13%"));
        assert_eq!(the_labels(&back["reply_markup"]), [["Stop this job"]]);
        assert!(store
            .the_action_contexts_bound_to(40)
            .unwrap()
            .iter()
            .filter(|r| r.action == crate::job_messages::KEEP_RENDERING)
            .all(|r| r.payload["stop_removed"] == json!(true)));
    }

    /// **Item 39's one switch** is one constant, and it is on: the tick edits.
    #[test]
    fn twenty_eighth_18_the_one_switch_is_on() {
        assert!(THE_TICK_EDITS_A_JOBS_MESSAGE);
    }

    // ------------------------------------------------------------ piece T, the twenty-ninth notes

    /// **The Creator's moment for these tests**, 10 Nov 2026, 14:22:07 UTC.
    const PIECE_T_MOMENT: u64 = 1_794_320_527;

    fn kitchen_rendering_at(frame: u64, seconds: u64) -> WhatAJobsMessageSays {
        WhatAJobsMessageSays { at: Some(at(seconds)), ..kitchen_rendering(frame, false) }
    }

    /// The `date_time` entities a call carried.
    fn its_moments(fields: &serde_json::Map<String, Value>) -> Vec<Value> {
        fields["entities"].as_array().unwrap().iter().filter(|e| e["type"] == "date_time").cloned().collect()
    }

    /// Telegram's own date, `behind` seconds behind this computer's clock now.
    fn telegrams_date_behind(behind: u64) -> u64 {
        SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() - behind
    }

    /// **Telegram when things aren't ideal, problem 5 (`Telegram` C1d as
    /// changed 2026-11-13): a server whose clock runs four minutes ahead
    /// still writes Telegram's moment.** The edit Telegram accepted carried an
    /// `edit_date` 240 seconds behind this computer's clock, so the next
    /// *Updated* moment is the tick's less 240 seconds, and the status
    /// message's the same. Before, both carried the server's own moment, and
    /// the phone read *Updated 4 minutes ago* right after the change.
    #[test]
    fn not_ideal_5_the_updated_moment_is_on_telegrams_clock() {
        let service = Arc::new(AStandInService::new());
        service.then_answers(r#"{"ok":true,"result":{"message_id":40}}"#);
        let (mut messages, _store) = a_jobs_messages(&service);
        let keeper = TheJobsOwnMessages::default();
        keeper.the_start(&mut messages, &kitchen_rendering(0, false), true, false).unwrap();
        service.then_answers(&format!(r#"{{"ok":true,"result":{{"message_id":40,"date":1,"edit_date":{}}}}}"#, telegrams_date_behind(240)));
        keeper.a_tick(&mut messages, &[kitchen_rendering_at(31, PIECE_T_MOMENT)]);
        while messages.queue.send_the_next().is_some() {}
        let before = service.heard().len();
        keeper.a_tick(&mut messages, &[kitchen_rendering_at(32, PIECE_T_MOMENT + 5)]);
        while messages.queue.send_the_next().is_some() {}
        let moment = its_moments(&the_fields(&service.heard()[before]))[0]["unix_time"].as_u64().unwrap();
        let expected = PIECE_T_MOMENT + 5 - 240;
        assert!(moment <= expected && moment + 2 >= expected, "the job's message: {moment}, not about {expected}");

        // The status message, over the same queue's answers.
        let queue = &messages.queue;
        let mut store = InMemory::with_routing_key("00112233445566778899aabbccddeeff");
        store.record_the_status_message(88).unwrap();
        let mut clock = TheClock::for_the_conversation(CHAT);
        let before = service.heard().len();
        clock.a_tick(&mut store, queue, &rendering(), None, at(PIECE_T_MOMENT + 10)).unwrap();
        queue.send_the_next();
        let moment = its_moments(&the_fields(&service.heard()[before]))[0]["unix_time"].as_u64().unwrap();
        let expected = PIECE_T_MOMENT + 10 - 240;
        assert!(moment <= expected && moment + 2 >= expected, "the status message: {moment}, not about {expected}");
    }

    /// **`Telegram` C1d's *Updated* line (the twenty-ninth notes, note 9)**:
    /// while the job renders, its message ends with *Updated* and Telegram's
    /// `date_time` entity, format `r`, holding the tick's moment; its own text
    /// is the moment in C2a's form. **A tick with the same progress at a new
    /// moment edits it**, since the moment is part of what it says. It failed
    /// before the change: no line, no entity, and the second tick sent
    /// nothing.
    #[test]
    fn piece_t_a_rendering_jobs_message_says_when_it_was_updated() {
        let service = Arc::new(AStandInService::new());
        service.then_answers(r#"{"ok":true,"result":{"message_id":40}}"#);
        let (mut messages, _store) = a_jobs_messages(&service);
        let keeper = TheJobsOwnMessages::default();
        keeper.the_start(&mut messages, &kitchen_rendering(0, false), true, false).unwrap();
        let before = service.heard().len();
        keeper.a_tick(&mut messages, &[kitchen_rendering_at(31, PIECE_T_MOMENT)]);
        while messages.queue.send_the_next().is_some() {}
        let edit = the_fields(&service.heard()[before]);
        let text = edit["text"].as_str().unwrap().to_string();
        assert_eq!(
            text,
            "🟥 RunPod 4090\nKitchen\nRendering: frame 31 of 240 \u{b7} 12%\nUpdated 10 Nov 2026, 14:22:07 UTC"
        );
        let moments = its_moments(&edit);
        assert_eq!(moments.len(), 1, "{:#?}", edit["entities"]);
        let moment = &moments[0];
        assert_eq!(moment["unix_time"], json!(PIECE_T_MOMENT));
        assert_eq!(moment["date_time_format"], json!("r"));
        let utf16: Vec<u16> = text.encode_utf16().collect();
        let (offset, length) = (moment["offset"].as_u64().unwrap() as usize, moment["length"].as_u64().unwrap() as usize);
        assert_eq!(offset + length, utf16.len(), "the entity is the last line's moment");
        assert_eq!(String::from_utf16(&utf16[offset..]).unwrap(), "10 Nov 2026, 14:22:07 UTC");
        // The same progress five seconds on: edited, with the new moment.
        keeper.a_tick(&mut messages, &[kitchen_rendering_at(31, PIECE_T_MOMENT + 5)]);
        while messages.queue.send_the_next().is_some() {}
        assert_eq!(service.heard().len(), before + 2, "the moment moved, so the message is edited");
        let again = the_fields(&service.heard()[before + 1]);
        assert!(again["text"].as_str().unwrap().ends_with("\nUpdated 10 Nov 2026, 14:22:12 UTC"));
        assert_eq!(its_moments(&again)[0]["unix_time"], json!(PIECE_T_MOMENT + 5));
    }

    /// **The line is only on a rendering job's message** (`Telegram` C1d as
    /// changed 2026-11-10): not while it says *Stopping.* or waits its turn,
    /// not while D0's question is asked in place, and not where no tick read
    /// a moment (the start). It failed before the change only by the first
    /// assertion's partner test above; these hold the edges.
    #[test]
    fn piece_t_the_line_is_only_on_a_rendering_jobs_message() {
        let server = AServerLabel::with_colour("RunPod 4090", Some("red"));
        let keeper = TheJobsOwnMessages::default();
        let queue = a_queue(&Arc::new(AStandInService::new()));
        let stopping = WhatAJobsMessageSays { at: Some(at(PIECE_T_MOMENT)), ..kitchen(AJobsState::Stopping) };
        assert!(!keeper.the_words(&stopping, true, &queue).text.contains("Updated"));
        let queued = WhatAJobsMessageSays { at: Some(at(PIECE_T_MOMENT)), ..kitchen(AJobsState::Queued { the_frames: None }) };
        assert!(!keeper.the_words(&queued, true, &queue).text.contains("Updated"));
        assert!(!keeper.the_words(&kitchen_rendering_at(31, PIECE_T_MOMENT), false, &queue).text.contains("Updated"), "a question open");
        assert!(!keeper.the_words(&kitchen_rendering(31, false), true, &queue).text.contains("Updated"), "no moment read");
        assert!(keeper.the_words(&kitchen_rendering_at(0, PIECE_T_MOMENT), true, &queue).text.ends_with("Started.\nUpdated 10 Nov 2026, 14:22:07 UTC"));
        assert!(!crate::job_messages::the_jobs_message_cut(&server, "Kitchen", crate::job_messages::A_FINISHED_ENDING).text.contains("Updated"));
    }

    /// **`Telegram` C1e as changed 2026-11-10 — the ending's line at once**
    /// (the twenty-ninth notes, note 11): it waits out a tick's edit offered
    /// before it, shows the one line with no buttons, marks the buttons'
    /// records removed, **and keeps the job's message recorded**, so the last
    /// frame's pictures still reply to it. No tick edits it after. **The cut
    /// that follows sends nothing**, Telegram having already accepted that
    /// line, and ends the record. It failed before the change: nothing was
    /// edited (`NoJobsMessage`).
    #[test]
    fn piece_t_the_ending_is_told_at_once_and_the_message_stays_recorded() {
        let service = Arc::new(AStandInService::new());
        service.then_answers(r#"{"ok":true,"result":{"message_id":40}}"#);
        let (mut messages, store) = a_jobs_messages(&service);
        let keeper = TheJobsOwnMessages::default();
        keeper.the_start(&mut messages, &kitchen_rendering(0, false), true, false).unwrap();
        keeper.a_tick(&mut messages, &[kitchen_rendering_at(240, PIECE_T_MOMENT)]);
        assert_eq!(messages.queue.how_many_are_waiting(), 1, "the tick's edit waits in the queue");
        let server = AServerLabel::with_colour("RunPod 4090", Some("red"));
        assert_eq!(
            keeper.the_ending_told(&mut messages, "job-1", &server, "Kitchen", crate::job_messages::A_FINISHED_ENDING),
            WhatTheEditCameTo::Done
        );
        let heard = service.heard();
        let last = the_fields(heard.last().unwrap());
        assert_eq!(last["text"], json!("🟥 RunPod 4090 \u{b7} Kitchen \u{b7} Finished."));
        assert_eq!(last["reply_markup"], json!({ "inline_keyboard": [] }));
        assert!(the_fields(&heard[heard.len() - 2])["text"].as_str().unwrap().contains("frame 240"), "the tick's went first");
        assert_eq!(store.the_jobs_message_of("job-1").map(|r| r.message), Some(40), "still recorded, for the pictures");
        assert_eq!(keeper.the_message_of(&**store.lock().unwrap(), "job-1"), Some(40));
        let records = store.the_action_contexts_bound_to(40).unwrap();
        assert!(!records.is_empty() && records.iter().all(|r| r.payload["stop_removed"] == json!(true)));
        let after = service.heard().len();
        keeper.a_tick(&mut messages, &[kitchen_rendering_at(240, PIECE_T_MOMENT + 5)]);
        while messages.queue.send_the_next().is_some() {}
        assert_eq!(service.heard().len(), after, "no tick edits it after its ending");
        assert_eq!(
            keeper.the_cut(&mut messages, "job-1", &server, "Kitchen", crate::job_messages::A_FINISHED_ENDING),
            WhatTheEditCameTo::Done
        );
        assert_eq!(service.heard().len(), after, "the cut sends the same line no second time");
        assert_eq!(store.the_jobs_message_of("job-1"), None, "the cut ends the record");
    }

    /// **A restart between the ending told and the cut** forgets what
    /// Telegram accepted: the cut then sends the line again, and Telegram's
    /// *message is not modified* counts as done, so the record ends.
    #[test]
    fn piece_t_a_cut_after_a_restart_takes_not_modified_as_done() {
        let service = Arc::new(AStandInService::new());
        service.then_answers(r#"{"ok":true,"result":{"message_id":40}}"#);
        let (mut messages, store) = a_jobs_messages(&service);
        let keeper = TheJobsOwnMessages::default();
        keeper.the_start(&mut messages, &kitchen_rendering(0, false), true, false).unwrap();
        let server = AServerLabel::with_colour("RunPod 4090", Some("red"));
        keeper.the_ending_told(&mut messages, "job-1", &server, "Kitchen", crate::job_messages::A_FINISHED_ENDING);
        let restarted = TheJobsOwnMessages::default();
        restarted.a_tick(&mut messages, &[]);
        service.then_answers(r#"{"ok":false,"error_code":400,"description":"Bad Request: message is not modified: specified new message content and reply markup are exactly the same as a current content and reply markup of the message"}"#);
        assert_eq!(
            restarted.the_cut(&mut messages, "job-1", &server, "Kitchen", crate::job_messages::A_FINISHED_ENDING),
            WhatTheEditCameTo::Done
        );
        assert_eq!(store.the_jobs_message_of("job-1"), None);
    }
}

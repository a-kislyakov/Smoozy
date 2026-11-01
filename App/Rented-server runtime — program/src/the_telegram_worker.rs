//! **The Telegram role worker — §5.3's one role worker — and the words it and
//! the supervisor speak to each other.** The words were added 2026-10-16, the
//! first piece of the Telegram worker's map; **the worker itself the same day,
//! its second piece**, below the words.
//!
//! **What the worker asks for, and why each is the supervisor's to answer.**
//! `Telegram` F2: the worker writes no durable store itself, so every durable
//! fact it holds is written by the supervisor at its request, through the
//! machine-local endpoint like every other worker's. Eight requests:
//!
//! 1. **its start** — this run's identifier, the machine kind, the
//!    installation identity, the routing key and the whole state store;
//! 2. **one fact written** to its state store, published whole before the
//!    answer goes back;
//! 3. **its own journal read forward**, from its own cursor;
//! 4. **its own cursor moved**, which writes the cursor in its store and the
//!    journal's registration for the Telegram worker — never the desktop's
//!    (`State, identity and recovery` E2);
//! 5. **an instruction's recorded outcome, read**, and
//! 6. **an instruction's outcome, recorded** — `Telegram` B9, in the shape of
//!    `Server protocol and transfers` C4: the first recorded wins and a repeat
//!    is answered with it; and, from the map's piece 5,
//! 7. **a job submitted again** — `Telegram` Y1, the one submission the
//!    machine makes for itself, its new job identity issued by the supervisor
//!    (`State, identity and recovery` Y7a); and, from the map's piece 6,
//! 8. **a frame's picture made again** — `Telegram` I4, a cleared derivative
//!    re-produced through the preview route, the supervisor judging from its
//!    records whether the frame is one a picture can be made of.
//!
//! **Every one of them is answered only from the role worker this run
//! started** — §5.3's paragraph after W7. Any other caller of them is refused
//! with [`THE_REFUSAL_FIELD`] saying [`END_YOURSELF`], and a worker that reads
//! that ends itself.
//!
//! **Neither end of these words is the desktop**, so they are this program's
//! own and are not in `smoozy_framing::requests` — the same line the setup
//! worker's and the render worker's words are drawn on.
//!
//! **The worker performs and asks; the supervisor writes.** Nothing here opens
//! a store. The answers are in `main.rs`, beside every other answer, because
//! they reach the supervisor's own start.
//!
//! # The worker itself, added 2026-10-16
//!
//! **`smoozy-server work telegram` is the bot.** It asks its start from its own
//! process — the match the supervisor makes is on the process that connects,
//! so no child may ask for it — reads the token and the role configuration
//! once (`Telegram` K2, T1), puts the `smoozy-telegram` crate together in
//! [`the_worker_assembled`], and runs it until it is stopped. The pieces of
//! that crate a later piece of the map fills are handed stand-ins that do
//! nothing and say so, each naming its piece — since the map's fifth piece,
//! none on [`TheCommandPathOverTheSupervisor`]. **From the
//! map's third piece it reads the journal on every turn and tells the phone
//! about jobs** ([`crate::the_telegram_journal`]), and a render's captured
//! streams are read whole for the words button. **From its fourth, what the
//! phone asks for reaches the machine**: a stop and an answer to a waiting job
//! go to the supervisor as commands, and what this machine holds is read off
//! its line ([`TheLineOnThisMachine`]). **From its fifth, *try it again*
//! makes one new job on this machine** (`Telegram` Y1), with nobody connected.
//! **From its sixth, the status message keeps its clock** ([`TheStatusOnItsClock`]),
//! **a picture goes per frame, the contact sheet at a job's end, and *send a
//! frame* is answered here**, a cleared picture made again through the preview
//! route.
//!
//! **When it cannot work it stays** (`Telegram` F1, its paragraph of
//! 2026-10-12, narrowed 2026-10-16): no token file, a role configuration it
//! cannot read, or Telegram refusing the token. (A processor below the floor
//! of the client's cryptography was a fourth until 2026-10-24, when the
//! cryptography became `ring`, which has no such floor.) It says so once on its complaints stream
//! and waits for the polite stop, reading nothing and making one call — its
//! start, asked once a minute. **And when the supervisor refuses one of its
//! requests it ends**, because it is a worker of an earlier run.
//!
//! **The polite stop** is `Telegram` section 19's: the interrupt, delivered to
//! this process, sets the stop flag; the turn that sees it ends the run; and a
//! worker holding the slot then makes one best-effort board edit marking this
//! machine silent ([`TheAssembledWorker::a_last_board_edit_marking_this_machine_silent`]).
//!
//! **What it says on its complaints stream never carries the token or a
//! message's content**: the crate's notes are written to that rule, and every
//! sentence composed here names a condition and never quotes a file.

use smoozy_telegram::job_messages::AServerLabel;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};

use smoozy_framing::framing::{Answered, Fields, Request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS};
use smoozy_runtime::acting::endpoint::TheEndpointAsACaller;
use smoozy_runtime::acting::stores::TheTelegramWorkersStateAsHeld;
use smoozy_runtime::machine_kind::MachineKind;
use smoozy_state::identities::{Identity, IdentityKind};
use smoozy_framing::requests as asked_of_the_machine;
use smoozy_queue_history_machine::line::{JobRecord, QueueState};
use smoozy_telegram::actions::{
    wait_for, AJournalledOutcome, ARecordedOutcome, AReleasesOutcome, TheJobMessages,
    TheListenersAcknowledgement, TheOwnersActs, TheSubject, TheSupervisorsCommandPath,
};
use smoozy_telegram::job_messages::{
    the_job_reports, AKindOfReport, AStopsOutcome, ASubmissionsOutcome, TheAnswer,
    TheReportingChoice, WhatTheHoldWaitsFor,
};
use smoozy_telegram::pictures::{
    a_frame_is_being_made_again, answer_send_a_frame, there_is_no_picture_of_the_frame,
    ADerivative, AnAnswerToSendAFrame, TheDerivativeSet, WhatTheSetHolds,
};
use smoozy_telegram::status::{
    the_interval_for, AReading, CardMemory, TheClock, TheMachineKind, TheStatus, TheUptime,
    WhatTheMachineIsDoing,
};
use smoozy_telegram::listener::{
    run_until_stopped, Notes, OnThisComputer, ReadsItsOwnClock, TheListener, TheSettings,
    WhatTheListenerIsHanded, WhoThisMachineIs,
};
use smoozy_telegram::material::{
    TheRoleConfiguration, THE_ROLE_CONFIGURATIONS_NAME, THE_TOKEN_FILES_NAME,
};
use smoozy_telegram::pacing::{AClassOfSend, HowASendEnded, OnThisComputersClock, TheSendQueue, Waits};
use smoozy_telegram::routing::context::AnActionContextRecord;
use smoozy_telegram::routing::gather::{AJobAsHeld, AJobWaits, ReadsWhatThisMachineHolds, TheProgress};
use smoozy_telegram::routing::{TheRouter, WhatTheRouterIsHanded};
use smoozy_telegram::transport::{TheClient, WhyTheCallFailed};
use smoozy_telegram::worker_state::{ASentMessage, ThePhonesChoice, TheTelegramWorkersState, WhyTheStoreFailed};

use crate::the_telegram_journal::{TheJournalReader, TheRecordsOnThisMachine, WhatTheReaderIsHanded};

// ---------------------------------------------------------------------------
// The seven requests
// ---------------------------------------------------------------------------

/// **The worker's start**: what it needs before it reads anything — this
/// run's identifier, the machine kind, the installation identity, the routing
/// key and the whole state store. **A pure reading, with one exception**: a
/// machine whose role was switched on before the routing key existed holds
/// none, and the key is issued here then, once.
pub const THE_TELEGRAM_WORKERS_START_REQUEST: &str = "the telegram worker's start";

/// **One fact written to the worker's state store** (`Telegram` F2), named in
/// [`THE_FACT_FIELD`].
pub const THE_TELEGRAM_WORKERS_FACT_REQUEST: &str = "a telegram worker's fact is written";

/// **The worker's own journal, read forward** from [`smoozy_framing::requests::THE_CURSOR_FIELD`]
/// where the request carries one, and from the cursor in its store where it
/// does not. **A pure reading**: nothing is registered or moved.
pub const THE_TELEGRAM_WORKERS_READ_FORWARD_REQUEST: &str =
    "the telegram worker reads its journal forward";

/// **The worker's own cursor moves forward**, to
/// [`smoozy_framing::requests::THE_CURSOR_FIELD`] — after the work every event
/// below it asked for is done (`State, identity and recovery` E3).
pub const THE_TELEGRAM_WORKERS_CURSOR_MOVES_REQUEST: &str =
    "the telegram worker's cursor moves forward";

/// **An instruction's recorded outcome, read** (`Telegram` B9), by
/// [`THE_INSTRUCTION_FIELD`].
pub const THE_INSTRUCTIONS_OUTCOME_READING_REQUEST: &str = "an instruction's recorded outcome is read";

/// **An instruction's outcome, recorded beside its identity** (`Telegram` B9),
/// by [`THE_INSTRUCTION_FIELD`] and [`THE_OUTCOME_FIELD`].
pub const THE_INSTRUCTIONS_OUTCOME_IS_RECORDED_REQUEST: &str = "an instruction's outcome is recorded";

/// **`Telegram` Y1: the job a *try it again* names, submitted again by this
/// machine for itself**, with nobody connected — the seventh request, added
/// 2026-10-16 by the map's piece 5. By [`smoozy_framing::requests::THE_JOB_FIELD`],
/// the failed job.
///
/// **The request's own identity is the submission's**, the one the button's
/// record carries (`Telegram` R4) — never one issued for the asking — so a
/// second press repeats it and finds what the first one made (`Server
/// protocol and transfers` C4). **The new job's identity is the supervisor's
/// to issue**, before anything is composed (`State, identity and recovery`
/// Y7a), and comes back in [`THE_NEW_JOB_FIELD`].
pub const THE_JOB_IS_SUBMITTED_AGAIN_REQUEST: &str = "a job is submitted again by this machine";

/// **What became of the submission**: [`IT_WAS_SUBMITTED_NOW`],
/// [`IT_WAS_SUBMITTED_BEFORE`], [`IT_WAS_NOT_SUBMITTED`] with the machine's own
/// words beside it, or [`IT_IS_STILL_BEING_MADE`].
pub const WHAT_BECAME_OF_THE_SUBMISSION_FIELD: &str = "what became of the submission";
/// The job was made by this request.
pub const IT_WAS_SUBMITTED_NOW: &str = "submitted now";
/// **A repeat**: an earlier request under the same identity made the job, and
/// nothing was made this time.
pub const IT_WAS_SUBMITTED_BEFORE: &str = "submitted before";
/// Nothing was made, and the machine says why.
pub const IT_WAS_NOT_SUBMITTED: &str = "not submitted";
/// **An earlier request under the same identity was recorded and never
/// finished** — the supervisor ended part-way — so whether the job stands is
/// said beside it (`Server protocol and transfers` C8's *still in hand*).
pub const IT_IS_STILL_BEING_MADE: &str = "still being made";
/// **The new job's identity**, as text.
pub const THE_NEW_JOB_FIELD: &str = "new job";
/// **The moment the new job was made**, on the machine's clock, as whole
/// seconds since 1970, a point and nine figures of nanoseconds — the release
/// moment's form.
pub const THE_SUBMISSIONS_MOMENT_FIELD: &str = "submitted at";

/// **`Telegram` I4 — a frame's picture made again through the preview
/// route**, the eighth request, added 2026-10-16 by the map's piece 6: no
/// request let the worker ask for a preview, and `Telegram` I4 has a cleared derivative
/// re-produced *"through the same route"* under its own declarations
/// (`Job files and outputs` B24). **The supervisor judges, from its own
/// records, whether the frame is one a picture can be made of** — the worker
/// judges no frame finished (`Telegram` I1) — and answers in
/// [`WHAT_BECAME_OF_THE_FRAME_FIELD`]. Carries the job in
/// `smoozy_framing::requests::THE_JOB_FIELD` and the frame in
/// [`THE_FRAME_ASKED_FIELD`].
pub const THE_FRAMES_PICTURE_IS_MADE_AGAIN_REQUEST: &str = "a frame's picture is made again";
/// The frame asked for, as an integer in text.
pub const THE_FRAME_ASKED_FIELD: &str = "frame asked";
/// **What became of the frame asked for**: one of the five below.
pub const WHAT_BECAME_OF_THE_FRAME_FIELD: &str = "what became of the frame";
/// Its picture is published in the job's record directory: read it there.
pub const ITS_PICTURE_IS_PUBLISHED: &str = "its picture is published";
/// **Its picture is being made** — queued now, because its records show it
/// was published once and has been cleared (`Queue & History` P7), or
/// already queued or being made. The picture follows on the journal.
pub const ITS_PICTURE_IS_BEING_MADE: &str = "its picture is being made";
/// The job is rendering and the frame has not finished, by the records.
pub const IT_HAS_NOT_FINISHED: &str = "it has not finished";
/// **The records hold no picture of it, made or owed**: a frame the job does
/// not cover, one the candidate rules left nothing to make of, or one of a
/// job that has ended without it.
pub const THERE_IS_NO_PICTURE_OF_IT: &str = "there is no picture of it";
/// The job is not on this machine, or its record directory is gone.
pub const THE_JOB_IS_NOT_HERE: &str = "the job is not on this machine";

/// **Every request only the role worker makes**, which is what §5.3's
/// paragraph after W7 calls *the role worker's own requests*. Read by the
/// tests that hold every one of them to the refusal.
#[cfg_attr(not(test), allow(dead_code))]
pub const THE_TELEGRAM_WORKERS_OWN_REQUESTS: [&str; 8] = [
    THE_TELEGRAM_WORKERS_START_REQUEST,
    THE_TELEGRAM_WORKERS_FACT_REQUEST,
    THE_TELEGRAM_WORKERS_READ_FORWARD_REQUEST,
    THE_TELEGRAM_WORKERS_CURSOR_MOVES_REQUEST,
    THE_INSTRUCTIONS_OUTCOME_READING_REQUEST,
    THE_INSTRUCTIONS_OUTCOME_IS_RECORDED_REQUEST,
    THE_JOB_IS_SUBMITTED_AGAIN_REQUEST,
    THE_FRAMES_PICTURE_IS_MADE_AGAIN_REQUEST,
];

// ---------------------------------------------------------------------------
// The refusal
// ---------------------------------------------------------------------------

/// **Carried by every refusal of one of the worker's own requests to a caller
/// that is not the role worker this run started**, and by nothing else.
pub const THE_REFUSAL_FIELD: &str = "the telegram worker is refused";
/// **The refusal's one value**: the worker that reads it ends itself (§5.3's
/// paragraph after W7).
pub const END_YOURSELF: &str = "end yourself";
/// Why, in the supervisor's own words, for the worker's complaints stream.
pub const WHY_IT_WAS_REFUSED_FIELD: &str = "why the telegram worker is refused";

/// The refusal, as the supervisor answers it.
pub fn the_refusal(why: &str) -> Fields {
    Fields::none()
        .and(THE_REFUSAL_FIELD, END_YOURSELF)
        .and(WHY_IT_WAS_REFUSED_FIELD, why)
}

/// **Whether an answer tells the worker to end itself.** The worker's half.
pub fn it_says_end_yourself(answered: &Answered) -> bool {
    answered.fields.text(THE_REFUSAL_FIELD) == Some(END_YOURSELF)
}

// ---------------------------------------------------------------------------
// The start
// ---------------------------------------------------------------------------

/// This run's identifier, as text.
pub const THE_RUN_FIELD: &str = "run";
/// The machine kind, in [`the_word_for_the_machine_kind`]'s words — for the
/// uptime line, which a systemd machine alone carries.
pub const THE_MACHINE_KIND_FIELD: &str = "machine kind";
/// The installation identity, as text. **Absent where the machine holds none**
/// (G31a's never-held answer), never made up.
pub const THE_INSTALLATION_FIELD: &str = "installation";
/// The 16-byte machine-routing key, as hexadecimal text.
pub const THE_ROUTING_KEY_FIELD: &str = "routing key";
/// **The whole state store**, in its written form.
pub const THE_STATE_STORE_FIELD: &str = "state store";

/// The machine kind as the start carries it.
pub fn the_word_for_the_machine_kind(kind: MachineKind) -> &'static str {
    match kind {
        MachineKind::Systemd => "systemd",
        MachineKind::Container => "container",
    }
}

/// The kind that word names, and nothing else is read as a kind.
pub fn the_machine_kind_named(word: &str) -> Option<MachineKind> {
    [MachineKind::Systemd, MachineKind::Container]
        .into_iter()
        .find(|kind| the_word_for_the_machine_kind(*kind) == word)
}

/// **What the worker is handed at its start.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheWorkersStart {
    pub run: String,
    pub machine_kind: MachineKind,
    pub installation: Option<String>,
    pub routing_key: String,
    pub state: TheTelegramWorkersStateAsHeld,
}

/// The start, as the supervisor answers it.
pub fn the_start_as_fields(start: &TheWorkersStart) -> Result<Fields, String> {
    let store = serde_json::to_string(&start.state).map_err(|why| why.to_string())?;
    let mut fields = Fields::none()
        .and(THE_RUN_FIELD, start.run.as_str())
        .and(
            THE_MACHINE_KIND_FIELD,
            the_word_for_the_machine_kind(start.machine_kind),
        )
        .and(THE_ROUTING_KEY_FIELD, start.routing_key.as_str())
        .and(THE_STATE_STORE_FIELD, store);
    if let Some(installation) = &start.installation {
        fields = fields.and(THE_INSTALLATION_FIELD, installation.as_str());
    }
    Ok(fields)
}

/// **The start, read back by the worker.** The worker's half. **A start
/// missing any part but the installation identity is not a start**, and is
/// refused rather than filled in.
pub fn the_start_in(fields: &Fields) -> Result<TheWorkersStart, String> {
    let named = |name: &str| {
        fields
            .text(name)
            .map(str::to_string)
            .ok_or_else(|| format!("the start carries no {name}"))
    };
    let machine_kind = the_machine_kind_named(&named(THE_MACHINE_KIND_FIELD)?)
        .ok_or_else(|| "the start names a machine kind this program does not know".to_string())?;
    let state = serde_json::from_str(&named(THE_STATE_STORE_FIELD)?)
        .map_err(|why| format!("the state store does not read back: {why}"))?;
    Ok(TheWorkersStart {
        run: named(THE_RUN_FIELD)?,
        machine_kind,
        installation: fields.text(THE_INSTALLATION_FIELD).map(str::to_string),
        routing_key: named(THE_ROUTING_KEY_FIELD)?,
        state,
    })
}

// ---------------------------------------------------------------------------
// One fact
// ---------------------------------------------------------------------------

/// **The fact a write carries**, in [`AFact`]'s written form.
pub const THE_FACT_FIELD: &str = "fact";
/// **What became of a write**: [`IT_WAS_WRITTEN`] once it is durable, or
/// [`IT_WAS_NOT_WRITTEN`] with the store's own words beside it in
/// `smoozy_framing::requests::THE_MACHINES_OWN_WORDS_FIELD`.
pub const WHAT_BECAME_OF_THE_WRITE_FIELD: &str = "what became of the write";
pub const IT_WAS_WRITTEN: &str = "written";
pub const IT_WAS_NOT_WRITTEN: &str = "not written";
/// **An instruction's outcome was already recorded**, so nothing was written
/// and the outcome that stands is the one answered.
pub const IT_WAS_ALREADY_RECORDED: &str = "already recorded";

/// **One durable fact the worker holds**, one to a write (`Telegram` F2).
///
/// **The journal cursor is not one of them**: its move also moves the
/// journal's registration, so it has a request of its own. **Nor is the
/// routing key**: the supervisor issues it and the worker never writes it.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum AFact {
    /// The next Telegram update identity to ask from (`Telegram` F3).
    TheUpdatePosition(i64),
    /// The board this machine created (`Telegram` B3).
    TheBoard(i64),
    /// This machine's machinery message (`Telegram` B10).
    TheMachineryMessage(i64),
    /// This machine's status message (`Telegram` C7).
    TheStatusMessage(i64),
    /// One action-context record, replacing any under the same identity
    /// (`Telegram` R1). The record is the worker's own written form.
    AnActionContext { identity: String, record: String },
    /// These action-context records are gone — superseded, or their subject
    /// generation ended (`Telegram` R1).
    TheseActionContextsAreGone(Vec<String>),
    /// **`Telegram` R1b — one line of the register**, the worker's own
    /// written form, under its message. Added 2026-10-31.
    AMessageIsRegistered { message: i64, line: String },
    /// **`Telegram` R1b — these lines are gone**: their job's record folder
    /// is.
    TheseMessagesAreForgotten(Vec<i64>),
    /// **`Telegram` V1d — the phone's choice**, the worker's own written
    /// form. The worker has already decided it is later than the one held.
    ThePhonesChoice(String),
}

impl AFact {
    /// The store, with this fact written into it.
    pub fn written_into(self, held: &mut TheTelegramWorkersStateAsHeld) {
        match self {
            AFact::TheUpdatePosition(next) => held.update_position = Some(next),
            AFact::TheBoard(message) => held.board = Some(message),
            AFact::TheMachineryMessage(message) => held.machinery_message = Some(message),
            AFact::TheStatusMessage(message) => held.status_message = Some(message),
            AFact::AnActionContext { identity, record } => {
                held.action_contexts.insert(identity, record);
            }
            AFact::TheseActionContextsAreGone(gone) => {
                for identity in gone {
                    held.action_contexts.remove(&identity);
                }
            }
            AFact::AMessageIsRegistered { message, line } => {
                held.sent_messages.insert(message, line);
                smoozy_telegram::worker_state::the_register_kept_within_its_bound(&mut held.sent_messages);
            }
            AFact::TheseMessagesAreForgotten(gone) => {
                for message in gone {
                    held.sent_messages.remove(&message);
                }
            }
            AFact::ThePhonesChoice(written) => held.phones_choice = Some(written),
        }
    }
}

/// A fact, as the worker asks for it. The worker's half.
pub fn the_fact_as_fields(fact: &AFact) -> Result<Fields, String> {
    let written = serde_json::to_string(fact).map_err(|why| why.to_string())?;
    Ok(Fields::none().and(THE_FACT_FIELD, written))
}

/// A fact, read back by the supervisor, or nothing where the request carries
/// none it can read.
pub fn the_fact_in(fields: &Fields) -> Option<AFact> {
    serde_json::from_str(fields.text(THE_FACT_FIELD)?).ok()
}

// ---------------------------------------------------------------------------
// An instruction's outcome
// ---------------------------------------------------------------------------

/// **The instruction's identity**, exactly as the board carries it.
pub const THE_INSTRUCTION_FIELD: &str = "instruction";
/// **The outcome**, in [`AnInstructionsOutcome::as_recorded`]'s words.
pub const THE_OUTCOME_FIELD: &str = "outcome";
/// **What a reading found**: [`AN_OUTCOME_IS_RECORDED`] with the outcome
/// beside it, or [`NOTHING_IS_RECORDED`] — a stated absence, never a claim.
pub const WHAT_IS_RECORDED_FIELD: &str = "what is recorded";
pub const AN_OUTCOME_IS_RECORDED: &str = "an outcome";
pub const NOTHING_IS_RECORDED: &str = "nothing";

/// **`Telegram` B9's outcome: success or failure**, and a failure carries what
/// the user was told.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnInstructionsOutcome {
    Done,
    NotDone(String),
}

/// What a recorded failure's words follow.
const NOT_DONE: &str = "not done: ";

impl AnInstructionsOutcome {
    /// The outcome as the store holds it and the wire carries it.
    pub fn as_recorded(&self) -> String {
        match self {
            AnInstructionsOutcome::Done => "done".to_string(),
            AnInstructionsOutcome::NotDone(why) => format!("{NOT_DONE}{why}"),
        }
    }

    /// The outcome those words name, or nothing.
    pub fn read_back(words: &str) -> Option<AnInstructionsOutcome> {
        if words == "done" {
            return Some(AnInstructionsOutcome::Done);
        }
        words
            .strip_prefix(NOT_DONE)
            .map(|why| AnInstructionsOutcome::NotDone(why.to_string()))
    }
}

// ---------------------------------------------------------------------------
// The worker's half: asking the supervisor
// ---------------------------------------------------------------------------

/// **Whatever answers the worker's requests.** On a machine it is the
/// machine-local endpoint at Z2's recorded path, asked from the worker's own
/// process; a test hands in the supervisor's real answers over a channel.
pub trait AsksTheSupervisor: Send + Sync {
    /// **One request, and the answer correlated with it** — or `None` where
    /// nothing answered, and `Err` where the system would not let the request
    /// be made at all.
    fn ask(&self, request: &Request) -> Result<Option<Answered>, String>;

    /// The same, waiting up to `until` for a supervisor to be there at all.
    fn ask_waiting(&self, request: &Request, _until: Duration) -> Result<Option<Answered>, String> {
        self.ask(request)
    }
}

impl AsksTheSupervisor for TheEndpointAsACaller {
    fn ask(&self, request: &Request) -> Result<Option<Answered>, String> {
        TheEndpointAsACaller::ask(self, request)
    }
    fn ask_waiting(&self, request: &Request, until: Duration) -> Result<Option<Answered>, String> {
        self.ask_waiting_for_a_supervisor(request, until)
    }
}

/// **Why a request of the worker's came back with no answer to act on.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhyItWasNotAnswered {
    /// **The supervisor told this worker to end itself** (§5.3's paragraph
    /// after W7), with the supervisor's words.
    ItIsToEndItself(String),
    /// Nothing answered, or the request could not be made.
    NothingAnswered(String),
}

impl WhyItWasNotAnswered {
    pub(crate) fn in_words(&self) -> String {
        match self {
            WhyItWasNotAnswered::ItIsToEndItself(why) => {
                format!("the supervisor told this worker to end itself: {why}")
            }
            WhyItWasNotAnswered::NothingAnswered(why) => why.clone(),
        }
    }
}

/// **One of the worker's own requests, asked and read for the refusal.**
pub(crate) fn asked(
    supervisor: &dyn AsksTheSupervisor,
    name: &str,
    fields: Fields,
) -> Result<Answered, WhyItWasNotAnswered> {
    let identity = Identity::issue(IdentityKind::Operation).map_err(|_| {
        WhyItWasNotAnswered::NothingAnswered("the system would not supply random bytes".into())
    })?;
    asked_under(supervisor, identity, name, fields)
}

/// **The same, under an identity the caller holds** — the one request whose
/// identity is not issued for the asking: `Telegram` Y1's submission, which
/// goes under the submission identity its button's record carries, so a
/// second press repeats it (`Server protocol and transfers` C4).
pub(crate) fn asked_under(
    supervisor: &dyn AsksTheSupervisor,
    identity: Identity,
    name: &str,
    fields: Fields,
) -> Result<Answered, WhyItWasNotAnswered> {
    let request =
        Request::speaking(THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, identity, name).carrying(fields);
    match supervisor.ask(&request) {
        Ok(Some(answered)) if it_says_end_yourself(&answered) => {
            Err(WhyItWasNotAnswered::ItIsToEndItself(
                answered
                    .fields
                    .text(WHY_IT_WAS_REFUSED_FIELD)
                    .unwrap_or("no reason was given")
                    .to_string(),
            ))
        }
        Ok(Some(answered)) => Ok(answered),
        Ok(None) => Err(WhyItWasNotAnswered::NothingAnswered(format!(
            "no supervisor answered \"{name}\""
        ))),
        Err(why) => Err(WhyItWasNotAnswered::NothingAnswered(why)),
    }
}

/// **What asking for the start came to, where it gave no start.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TheStartWasNotGiven {
    /// The supervisor told this worker to end itself.
    ItIsToEndItself(String),
    /// No supervisor answered within the bound.
    NoSupervisorAnswered(String),
    /// The supervisor answered with its own words and no start — a store it
    /// could not read, or a key it could not issue.
    OnlyWords(String),
}

/// **The worker's first act: its start, asked from its own process.**
pub fn the_start_asked(
    supervisor: &dyn AsksTheSupervisor,
    looking_for_it: Duration,
) -> Result<TheWorkersStart, TheStartWasNotGiven> {
    let identity = Identity::issue(IdentityKind::Operation).map_err(|_| {
        TheStartWasNotGiven::NoSupervisorAnswered("the system would not supply random bytes".into())
    })?;
    let request = Request::speaking(
        THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
        identity,
        THE_TELEGRAM_WORKERS_START_REQUEST,
    );
    let answered = match supervisor.ask_waiting(&request, looking_for_it) {
        Ok(Some(answered)) => answered,
        Ok(None) => {
            return Err(TheStartWasNotGiven::NoSupervisorAnswered(
                "nothing answered at the endpoint's recorded path".into(),
            ))
        }
        Err(why) => return Err(TheStartWasNotGiven::NoSupervisorAnswered(why)),
    };
    if it_says_end_yourself(&answered) {
        return Err(TheStartWasNotGiven::ItIsToEndItself(
            answered
                .fields
                .text(WHY_IT_WAS_REFUSED_FIELD)
                .unwrap_or("no reason was given")
                .to_string(),
        ));
    }
    the_start_in(&answered.fields).map_err(|why| {
        TheStartWasNotGiven::OnlyWords(
            answered
                .fields
                .text(smoozy_framing::requests::THE_MACHINES_OWN_WORDS_FIELD)
                .map(str::to_string)
                .unwrap_or(why),
        )
    })
}

// ---------------------------------------------------------------------------
// The worker's half: its state store, over the supervisor
// ---------------------------------------------------------------------------

/// **`Telegram` F2's store as the worker holds it: read from the start, and
/// written one fact at a time by the supervisor.**
///
/// **Reads come from the copy the start handed over**, kept current by every
/// write that came back *written*. That is sound because the worker is the only
/// writer of every fact in it: the supervisor writes only the routing key,
/// which the start already carried, and the instruction outcomes, which are
/// never read from here.
///
/// **A write is durable when the answer says so, and only then does the copy
/// change** (`Telegram` F3: nothing that depends on it is confirmed before).
/// **A refusal sets the worker's refused flag**, and the turn that sees it ends
/// the run.
///
/// **Action-context records are kept as the crate's own written form**, keyed
/// by identity; the supervisor keeps them and never reads them. Which ones a
/// removal takes is decided here and sent as the identities that are gone.
pub struct TheStoreOverTheSupervisor {
    supervisor: Arc<dyn AsksTheSupervisor>,
    held: TheTelegramWorkersStateAsHeld,
    refused: Arc<AtomicBool>,
}

impl TheStoreOverTheSupervisor {
    pub fn over(
        supervisor: Arc<dyn AsksTheSupervisor>,
        start: &TheWorkersStart,
        refused: Arc<AtomicBool>,
    ) -> TheStoreOverTheSupervisor {
        let mut held = start.state.clone();
        held.routing_key = Some(start.routing_key.clone());
        TheStoreOverTheSupervisor { supervisor, held, refused }
    }

    fn asked(&self, name: &str, fields: Fields) -> Result<Answered, WhyTheStoreFailed> {
        asked(self.supervisor.as_ref(), name, fields).map_err(|why| {
            if let WhyItWasNotAnswered::ItIsToEndItself(_) = why {
                self.refused.store(true, Ordering::SeqCst);
            }
            WhyTheStoreFailed(why.in_words())
        })
    }

    fn the_fact_written(&mut self, fact: AFact) -> Result<(), WhyTheStoreFailed> {
        let fields = the_fact_as_fields(&fact).map_err(WhyTheStoreFailed)?;
        let answered = self.asked(THE_TELEGRAM_WORKERS_FACT_REQUEST, fields)?;
        match answered.fields.text(WHAT_BECAME_OF_THE_WRITE_FIELD) {
            Some(IT_WAS_WRITTEN) => {
                fact.written_into(&mut self.held);
                Ok(())
            }
            _ => Err(WhyTheStoreFailed(the_words_in(&answered, "the fact was not written"))),
        }
    }

    fn the_records(&self) -> Result<Vec<AnActionContextRecord>, WhyTheStoreFailed> {
        self.held
            .action_contexts
            .values()
            .map(|written| {
                serde_json::from_str(written).map_err(|why| {
                    WhyTheStoreFailed(format!("an action-context record does not read back: {why}"))
                })
            })
            .collect()
    }
}

/// The supervisor's own words in an answer, or `otherwise`.
fn the_words_in(answered: &Answered, otherwise: &str) -> String {
    answered
        .fields
        .text(smoozy_framing::requests::THE_MACHINES_OWN_WORDS_FIELD)
        .unwrap_or(otherwise)
        .to_string()
}

impl TheTelegramWorkersState for TheStoreOverTheSupervisor {
    fn the_update_position(&self) -> Result<Option<i64>, WhyTheStoreFailed> {
        Ok(self.held.update_position)
    }
    fn record_the_update_position(&mut self, next: i64) -> Result<(), WhyTheStoreFailed> {
        self.the_fact_written(AFact::TheUpdatePosition(next))
    }
    fn the_board(&self) -> Result<Option<i64>, WhyTheStoreFailed> {
        Ok(self.held.board)
    }
    fn record_the_board(&mut self, message: i64) -> Result<(), WhyTheStoreFailed> {
        self.the_fact_written(AFact::TheBoard(message))
    }
    fn the_machinery_message(&self) -> Result<Option<i64>, WhyTheStoreFailed> {
        Ok(self.held.machinery_message)
    }
    fn record_the_machinery_message(&mut self, message: i64) -> Result<(), WhyTheStoreFailed> {
        self.the_fact_written(AFact::TheMachineryMessage(message))
    }
    fn the_routing_key(&self) -> Result<String, WhyTheStoreFailed> {
        self.held
            .routing_key
            .clone()
            .ok_or_else(|| WhyTheStoreFailed("the start carried no routing key".into()))
    }
    fn holds_a_context_record_bound_to(&self, source_message: i64) -> Result<bool, WhyTheStoreFailed> {
        Ok(self.the_records()?.iter().any(|record| record.source_message == Some(source_message)))
    }
    fn record_an_action_context(&mut self, record: AnActionContextRecord) -> Result<(), WhyTheStoreFailed> {
        let written = serde_json::to_string(&record).map_err(|why| WhyTheStoreFailed(why.to_string()))?;
        self.the_fact_written(AFact::AnActionContext { identity: record.identity, record: written })
    }
    fn the_action_context(&self, identity: &str) -> Result<Option<AnActionContextRecord>, WhyTheStoreFailed> {
        self.held
            .action_contexts
            .get(identity)
            .map(|written| {
                serde_json::from_str(written).map_err(|why| {
                    WhyTheStoreFailed(format!("an action-context record does not read back: {why}"))
                })
            })
            .transpose()
    }
    fn the_action_contexts(&self) -> Result<Vec<AnActionContextRecord>, WhyTheStoreFailed> {
        self.the_records()
    }
    fn the_action_contexts_bound_to(&self, source_message: i64) -> Result<Vec<AnActionContextRecord>, WhyTheStoreFailed> {
        Ok(self
            .the_records()?
            .into_iter()
            .filter(|record| record.source_message == Some(source_message))
            .collect())
    }
    fn remove_action_contexts_where(
        &mut self,
        which: &dyn Fn(&AnActionContextRecord) -> bool,
    ) -> Result<usize, WhyTheStoreFailed> {
        let gone: Vec<String> = self
            .the_records()?
            .into_iter()
            .filter(|record| which(record))
            .map(|record| record.identity)
            .collect();
        if gone.is_empty() {
            return Ok(0);
        }
        let how_many = gone.len();
        self.the_fact_written(AFact::TheseActionContextsAreGone(gone))?;
        Ok(how_many)
    }
    fn the_journal_cursor(&self) -> Result<Option<u64>, WhyTheStoreFailed> {
        Ok(self.held.journal_cursor)
    }
    /// **Its own request**, which moves the journal's registration for the
    /// Telegram worker as well (`State, identity and recovery` E2). Nothing in
    /// this piece moves it; the journal reader is the map's piece 3.
    fn record_the_journal_cursor(&mut self, next: u64) -> Result<(), WhyTheStoreFailed> {
        let answered = self.asked(
            THE_TELEGRAM_WORKERS_CURSOR_MOVES_REQUEST,
            Fields::none().and(smoozy_framing::requests::THE_CURSOR_FIELD, next.to_string()),
        )?;
        match answered.fields.text(WHAT_BECAME_OF_THE_WRITE_FIELD) {
            Some(IT_WAS_WRITTEN) | Some(IT_WAS_ALREADY_RECORDED) => {
                self.held.journal_cursor = answered
                    .fields
                    .text(smoozy_framing::requests::THE_CURSOR_FIELD)
                    .and_then(|at| at.parse().ok())
                    .or(Some(next));
                Ok(())
            }
            _ => Err(WhyTheStoreFailed(the_words_in(&answered, "the cursor did not move"))),
        }
    }
    fn the_status_message(&self) -> Result<Option<i64>, WhyTheStoreFailed> {
        Ok(self.held.status_message)
    }
    fn record_the_status_message(&mut self, message: i64) -> Result<(), WhyTheStoreFailed> {
        self.the_fact_written(AFact::TheStatusMessage(message))
    }
    fn register_a_sent_message(&mut self, line: ASentMessage) -> Result<(), WhyTheStoreFailed> {
        let written = serde_json::to_string(&line).map_err(|why| WhyTheStoreFailed(why.to_string()))?;
        self.the_fact_written(AFact::AMessageIsRegistered { message: line.message, line: written })
    }
    fn the_sent_message(&self, message: i64) -> Result<Option<ASentMessage>, WhyTheStoreFailed> {
        self.held
            .sent_messages
            .get(&message)
            .map(|written| serde_json::from_str(written).map_err(|why| WhyTheStoreFailed(format!("a register line does not read back: {why}"))))
            .transpose()
    }
    fn the_sent_messages(&self) -> Result<Vec<ASentMessage>, WhyTheStoreFailed> {
        self.held
            .sent_messages
            .values()
            .map(|written| serde_json::from_str(written).map_err(|why| WhyTheStoreFailed(format!("a register line does not read back: {why}"))))
            .collect()
    }
    fn forget_sent_messages_where(&mut self, which: &dyn Fn(&ASentMessage) -> bool) -> Result<usize, WhyTheStoreFailed> {
        let gone: Vec<i64> = self.the_sent_messages()?.into_iter().filter(|line| which(line)).map(|line| line.message).collect();
        if gone.is_empty() {
            return Ok(0);
        }
        let how_many = gone.len();
        self.the_fact_written(AFact::TheseMessagesAreForgotten(gone))?;
        Ok(how_many)
    }
    fn the_phones_choice(&self) -> Result<Option<ThePhonesChoice>, WhyTheStoreFailed> {
        self.held
            .phones_choice
            .as_deref()
            .map(|written| serde_json::from_str(written).map_err(|why| WhyTheStoreFailed(format!("the phone's choice does not read back: {why}"))))
            .transpose()
    }
    fn record_the_phones_choice(&mut self, choice: ThePhonesChoice) -> Result<bool, WhyTheStoreFailed> {
        if self.the_phones_choice()?.is_some_and(|held| !choice.is_later_than(&held)) {
            return Ok(false);
        }
        let written = serde_json::to_string(&choice).map_err(|why| WhyTheStoreFailed(why.to_string()))?;
        self.the_fact_written(AFact::ThePhonesChoice(written))?;
        Ok(true)
    }
}

// ---------------------------------------------------------------------------
// The worker's half: the owner's acts, and the stand-ins later pieces replace
// ---------------------------------------------------------------------------

/// **`Telegram` B9 on the supervisor's path.** The recorded outcome beside an
/// instruction is read and written through the two outcome requests, which is
/// what the map's piece 1 built them for.
///
/// **The stop and the answer to a waiting job are the map's piece 4's**, added
/// 2026-10-16. Each is an ordinary command on the one command path the SSH
/// adapter uses (`Telegram` F1; `Rented-server runtime` U10), **under an
/// operation identity issued for it here**: a board instruction's identity is
/// not a machine operation identity (piece 1's seam), so the instruction's own
/// outcome is recorded in the worker's store beside the instruction, and the
/// command's beside the command on the machine (`Server protocol and
/// transfers` C4). **The command returns once the machine has it** (`Telegram`
/// F3), and its recorded outcome reaches the phone off the journal (piece 3).
///
/// **What the command's own answer tells the phone, and only that**: a stop
/// finding the render already stopped or already ended (`Telegram` D4, R4), and
/// an answer finding another already applied (`Rented-server runtime` U14) —
/// the winner having been told off the journal, never twice.
///
/// ***Try it again* is the map's piece 5's**, added the same day: the failed
/// job submitted again by this machine for itself (`Telegram` Y1), under the
/// button's own submission identity, the new job's identity issued by the
/// supervisor (`State, identity and recovery` Y7a).
/// Optional constraints on a confirmed phone command. The supervisor checks
/// these in the same turn that admits the ordinary stop/cancel command.
pub const THE_CONFIRMED_JOB_FIELD: &str = "the confirmed Telegram job";
pub const THE_CONFIRMED_GENERATION_FIELD: &str = "the confirmed Telegram generation";

pub struct TheCommandPathOverTheSupervisor {
    supervisor: Arc<dyn AsksTheSupervisor>,
    refused: Arc<AtomicBool>,
    records: TheRecordsOnThisMachine,
    /// **Who tells the phone what a command's own answer says**, where this
    /// path is the owner's; none where it is the reporter's own.
    teller: Option<TheTeller>,
    context_store: Option<OneStore>,
    frame_answers: std::collections::HashMap<String, smoozy_telegram::pictures::ThePictures>,
}

/// **What the owner's command path tells the phone with** —
/// `TheOwnersActs::report`'s inputs, so the words are the crate's.
struct TheTeller {
    conversation: i64,
    server: AServerLabel,
    machine: String,
    queue: Arc<TheSendQueue>,
    holds: OneReaderOfWhatIsHeld,
    notes: SaysOnItsComplaintsStream,
    /// **The frames asked for whose picture is being made again** — the
    /// map's piece 6, shared with the journal reader, which sends each once
    /// its publication is read.
    frames: TheFramesAsked,
}

/// **A release moment as the machine writes it**: whole seconds since 1970, a
/// point, and nine figures of nanoseconds
/// (`smoozy_runtime::acting::stores::the_release_moment_as_text`). Nothing
/// where it does not read.
fn the_release_moment_in(text: &str) -> Option<SystemTime> {
    let (seconds, nanoseconds) = text.split_once('.')?;
    let seconds: u64 = seconds.parse().ok()?;
    let nanoseconds: u32 = nanoseconds.parse().ok()?;
    SystemTime::UNIX_EPOCH.checked_add(Duration::new(seconds, nanoseconds))
}

impl TheCommandPathOverTheSupervisor {
    /// **A command path with nobody to tell** — the reporter's own, and a
    /// test's.
    pub(crate) fn over(
        supervisor: Arc<dyn AsksTheSupervisor>,
        refused: Arc<AtomicBool>,
        records: TheRecordsOnThisMachine,
    ) -> TheCommandPathOverTheSupervisor {
        TheCommandPathOverTheSupervisor {
            supervisor,
            refused,
            records,
            teller: None,
            context_store: None,
            frame_answers: std::collections::HashMap::new(),
        }
    }

    fn asked(&self, name: &str, fields: Fields) -> Result<Answered, String> {
        asked(self.supervisor.as_ref(), name, fields).map_err(|why| {
            if let WhyItWasNotAnswered::ItIsToEndItself(_) = why {
                self.refused.store(true, Ordering::SeqCst);
            }
            why.in_words()
        })
    }

    /// **Recorded as not done, in words that say why** — `Telegram` B9's
    /// recorded failure, which the owner's acts tell the user in plain words.
    fn not_done(&mut self, operation: &str, why: &str) -> Result<ARecordedOutcome, String> {
        let outcome = ARecordedOutcome::Failed(why.to_string());
        self.record_the_outcome(operation, &outcome)?;
        Ok(outcome)
    }

    /// **Done, recorded beside the instruction** — once the machine has the
    /// command.
    fn done(&mut self, operation: &str) -> Result<ARecordedOutcome, String> {
        self.record_the_outcome(operation, &ARecordedOutcome::Done)?;
        Ok(ARecordedOutcome::Done)
    }

    /// **`TheOwnersActs::report`, from the owner's path** — through acts put
    /// together the way the assembly puts the owner's together, with a command
    /// path that tells nobody, so nothing here can report twice.
    fn report(&self, job: &str, journalled: AJournalledOutcome) {
        let Some(teller) = &self.teller else {
            return;
        };
        let notes = teller.notes.clone();
        let mut acts = TheOwnersActs {
            conversation: teller.conversation,
            supervisor: Box::new(TheCommandPathOverTheSupervisor::over(
                self.supervisor.clone(),
                self.refused.clone(),
                self.records.clone(),
            )),
            words: Box::new(self.records.clone()),
            holds: Box::new(teller.holds.clone()),
            queue: teller.queue.clone(),
            notes: Box::new(move |what: &str| notes(&format!("the owner's acts: {what}"))),
        };
        let mut path = TheCommandPathOverTheSupervisor::over(
            self.supervisor.clone(),
            self.refused.clone(),
            self.records.clone(),
        );
        path.context_store = self.context_store.clone();
        acts.supervisor = Box::new(path);
        let generation = Identity::read_back(IdentityKind::Job, job)
            .and_then(|id| self.records.the_job_record(&id))
            .map(|record| self.records.action_generation(&record))
            .unwrap_or_else(|| job.into());
        // An older released hold can outlive every record naming its job.
        // Keep that immutable hold identity; never label it as a job identity.
        let ended_hold = Identity::read_back(IdentityKind::Operation, job).map(|hold| hold.as_text());
        let subject = TheSubject {
            owning_machine: teller.machine.clone(),
            job: ended_hold.is_none().then(|| job.into()),
            hold: ended_hold,
            generation,
        };
        let label = match &journalled {
            AJournalledOutcome::AStop { label, .. }
            | AJournalledOutcome::ARelease { label, .. }
            | AJournalledOutcome::ASubmission { label, .. } => label.clone(),
        };
        acts.report_about(journalled, &subject, &teller.server, &label);
    }

    fn confirmation_fields(
        &self,
        fields: Fields,
        job: &Identity,
        generation: Option<&str>,
    ) -> Fields {
        match generation {
            Some(generation) => fields
                .and(THE_CONFIRMED_JOB_FIELD, job.as_text())
                .and(THE_CONFIRMED_GENERATION_FIELD, generation),
            None => fields,
        }
    }

    fn stop_with_generation(
        &mut self,
        operation: &str,
        job: &str,
        generation: Option<&str>,
    ) -> Result<ARecordedOutcome, String> {
        let Some(job) = Identity::read_back(IdentityKind::Job, job) else {
            return self.not_done(operation, "the button names no job");
        };
        let target = match self.records.the_job_record(&job).map(|record| record.state) {
            Some(QueueState::Rendering { operation })
            | Some(QueueState::Ended { outcome: operation }) => operation,
            Some(QueueState::Held { .. }) | Some(QueueState::Waiting) => {
                let answered = self.a_command(
                    asked_of_the_machine::THE_CANCEL_REQUEST,
                    self.confirmation_fields(
                        Fields::none().and(asked_of_the_machine::THE_JOB_FIELD, job.as_text()),
                        &job,
                        generation,
                    ),
                )?;
                if answered
                    .fields
                    .text(asked_of_the_machine::THE_CANCELS_OUTCOME_FIELD)
                    == Some(asked_of_the_machine::IT_WAS_REMOVED)
                {
                    return self.done(operation);
                }
                return self.not_done(
                    operation,
                    &the_words_in(&answered, "the waiting job was not cancelled"),
                );
            }
            None => return self.not_done(operation, "this server no longer holds the job"),
        };
        let answered = self.a_command(
            asked_of_the_machine::THE_STOP_REQUEST,
            self.confirmation_fields(
                Fields::none().and(
                    asked_of_the_machine::THE_OPERATION_TO_STOP_FIELD,
                    target.as_text(),
                ),
                &job,
                generation,
            ),
        )?;
        let words = the_words_in(&answered, "the machine said nothing more");
        match answered
            .fields
            .text(asked_of_the_machine::WHAT_BECAME_OF_IT_FIELD)
        {
            Some(asked_of_the_machine::IT_IS_STILL_IN_HAND) => self.done(operation),
            Some(asked_of_the_machine::IT_HAS_A_RECORDED_OUTCOME) => {
                let already = match answered
                    .fields
                    .text(asked_of_the_machine::THE_STOPS_OUTCOME_FIELD)
                {
                    Some(asked_of_the_machine::IT_WAS_LEFT_STOPPED) => {
                        let at = self
                            .records
                            .the_note(&job)
                            .and_then(|note| note.ended)
                            .map_or_else(SystemTime::now, |ended| ended.taken_at());
                        AStopsOutcome::AlreadyStopped { at }
                    }
                    Some(asked_of_the_machine::ITS_OWN_ENDING) => {
                        AStopsOutcome::TheJobHadAlreadyEnded
                    }
                    Some(other) => {
                        return self.not_done(
                            operation,
                            &format!("the stop came to \u{201c}{other}\u{201d}: {words}"),
                        )
                    }
                    None => return self.not_done(operation, &words),
                };
                let outcome = self.done(operation)?;
                let label = self.records.what_the_job_is_called(&job);
                self.report(
                    &job.as_text(),
                    AJournalledOutcome::AStop {
                        server: self.the_server(),
                        label,
                        outcome: already,
                    },
                );
                Ok(outcome)
            }
            // **`Server protocol and transfers` C8's third: refused before it was recorded**, and the work
            // did not happen. Said, rather than tried again for ever.
            Some(asked_of_the_machine::THERE_IS_NO_RECORD_OF_IT) => {
                self.not_done(operation, &words)
            }
            _ => Err(format!(
                "the stop's answer said nothing of what became of it: {words}"
            )),
        }
    }

    fn job_of_hold(&self, hold: &str) -> Option<String> {
        if let Some(record) = Identity::read_back(IdentityKind::Operation, hold)
            .and_then(|id| self.records.the_hold(&id))
        {
            if let smoozy_runtime::holds::Subject::Job(job) = record.subject {
                return Some(job.as_text());
            }
        }
        let found = Mutex::new(None);
        let mut store = self.context_store.clone()?;
        store
            .remove_action_contexts_where(&|record| {
                if record.hold.as_deref() == Some(hold) {
                    if let Some(job) = record.payload["job"].as_str() {
                        *lock(&found) = Some(job.into());
                    }
                }
                false
            })
            .ok()?;
        found.into_inner().ok().flatten()
    }

    fn the_server(&self) -> AServerLabel {
        self.teller.as_ref().map(|teller| teller.server.clone()).unwrap_or_default()
    }

    /// **One command, under an operation identity issued for it** — the
    /// answer, or `Err` where nothing answered, so the instruction stays
    /// unacknowledged and is tried again (`Telegram` B9).
    fn a_command(&self, name: &str, fields: Fields) -> Result<Answered, String> {
        let answered = self.asked(name, fields)?;
        if answered.the_request_name_it_says_it_does_not_answer().is_some() {
            return Err(format!("this machine's supervisor does not answer \"{name}\""));
        }
        Ok(answered)
    }
}

impl TheSupervisorsCommandPath for TheCommandPathOverTheSupervisor {
    fn the_action_context_store(&self) -> Option<Box<dyn TheTelegramWorkersState>> {
        self.context_store
            .as_ref()
            .map(|store| Box::new(store.clone()) as Box<dyn TheTelegramWorkersState>)
    }

    fn submit_a_confirmed_stop(
        &mut self,
        operation: &str,
        job: &str,
        generation: &str,
    ) -> Result<ARecordedOutcome, String> {
        let Some(record) = Identity::read_back(IdentityKind::Job, job)
            .and_then(|job| self.records.the_job_record(&job))
        else {
            return self.not_done(
                operation,
                smoozy_telegram::actions::THE_JOB_OR_ATTEMPT_ENDED,
            );
        };
        if self.records.action_generation(&record) != generation
            || matches!(record.state, QueueState::Ended { .. })
        {
            return self.not_done(
                operation,
                smoozy_telegram::actions::THE_JOB_OR_ATTEMPT_ENDED,
            );
        }
        self.stop_with_generation(operation, job, Some(generation))
    }

    fn the_recorded_outcome(
        &mut self,
        operation: &str,
    ) -> Result<Option<ARecordedOutcome>, String> {
        let answered = self.asked(
            THE_INSTRUCTIONS_OUTCOME_READING_REQUEST,
            Fields::none().and(THE_INSTRUCTION_FIELD, operation),
        )?;
        match answered.fields.text(WHAT_IS_RECORDED_FIELD) {
            Some(NOTHING_IS_RECORDED) => Ok(None),
            Some(AN_OUTCOME_IS_RECORDED) => answered
                .fields
                .text(THE_OUTCOME_FIELD)
                .and_then(AnInstructionsOutcome::read_back)
                .map(|outcome| {
                    Some(match outcome {
                        AnInstructionsOutcome::Done => ARecordedOutcome::Done,
                        AnInstructionsOutcome::NotDone(why) => ARecordedOutcome::Failed(why),
                    })
                })
                .ok_or_else(|| "the recorded outcome does not read back".to_string()),
            _ => Err(the_words_in(&answered, "the recorded outcome was not read")),
        }
    }

    fn record_the_outcome(&mut self, operation: &str, outcome: &ARecordedOutcome) -> Result<(), String> {
        let recorded = match outcome {
            ARecordedOutcome::Done => AnInstructionsOutcome::Done,
            ARecordedOutcome::Failed(why) => AnInstructionsOutcome::NotDone(why.clone()),
        };
        let answered = self.asked(
            THE_INSTRUCTIONS_OUTCOME_IS_RECORDED_REQUEST,
            Fields::none()
                .and(THE_INSTRUCTION_FIELD, operation)
                .and(THE_OUTCOME_FIELD, recorded.as_recorded()),
        )?;
        match answered.fields.text(WHAT_BECAME_OF_THE_WRITE_FIELD) {
            Some(IT_WAS_WRITTEN) | Some(IT_WAS_ALREADY_RECORDED) => Ok(()),
            _ => Err(the_words_in(&answered, "the outcome was not recorded")),
        }
    }

    /// **`Telegram` D1: the stop, on the render the job's record names** —
    /// `Server protocol and transfers` C21's command, its own operation
    /// identity naming the render operation's. The render the job's queue
    /// state carries is the one the button was sent about (`Queue & History`
    /// Q2), which also names the render once it has ended.
    ///
    /// *Still in hand* is the machine having it: the interrupt went where it
    /// could, and the stop's recorded outcome is told off the journal
    /// (`Telegram` D2). **A stop that finds the render already ended is told
    /// here**, since the journal says nothing new for it: stopped before
    /// (`Telegram` D4's second press) or ended on its own.
    fn submit_a_stop(&mut self, operation: &str, job: &str) -> Result<ARecordedOutcome, String> {
        self.stop_with_generation(operation, job, None)
    }

    /// **`Telegram` A2: one answer, by the phone's route** (`Rented-server
    /// runtime` U9, U10), on the hold the button names.
    ///
    /// **`Rented-server runtime` U13 and U14: the first answer recorded wins,
    /// and the one that lost is told which was applied and when** — through
    /// `TheOwnersActs::report`, as *another answer was applied*. This answer
    /// lost where the hold was already answered before it was sent, or where
    /// the release the machine answers with is not this answer by this route.
    /// The winner is told off the journal (`Telegram` A3), so it is told
    /// nothing here.
    fn release_a_hold(&mut self, operation: &str, hold: &str, answer: TheAnswer) -> Result<ARecordedOutcome, String> {
        let Some(hold) = Identity::read_back(IdentityKind::Operation, hold) else {
            return self.not_done(operation, "the button names no waiting job");
        };
        let standing = self.records.the_hold(&hold);
        let it_was_answered_before = standing.as_ref().is_some_and(|record| record.release.is_some());
        let (label, frames_held) = standing.as_ref().map(|record| self.records.the_label_and_count_of(record)).unwrap_or_default();
        let waits_for = match &standing {
            Some(record) if record.reason == crate::the_telegram_journal::THE_DRIVERS_REASON => {
                WhatTheHoldWaitsFor::TheDriverUpdate
            }
            _ => WhatTheHoldWaitsFor::TheUsersAnswer,
        };
        let answered = self.a_command(
            asked_of_the_machine::THE_ANSWER_TO_A_HOLD_REQUEST,
            Fields::none()
                .and(asked_of_the_machine::THE_HOLD_FIELD, hold.as_text())
                .and(
                    asked_of_the_machine::THE_ANSWER_FIELD,
                    match answer {
                        TheAnswer::GoOn => asked_of_the_machine::THE_ANSWER_IS_YES,
                        TheAnswer::Dont => asked_of_the_machine::THE_ANSWER_IS_NO,
                    },
                )
                .and(asked_of_the_machine::THE_ROUTE_FIELD, asked_of_the_machine::THE_ROUTE_IS_THE_PHONE),
        )?;
        match answered.fields.text(crate::unix::WHAT_BECAME_OF_THE_ANSWER_FIELD) {
            Some(crate::unix::THE_ANSWER_HAS_A_RECORDED_OUTCOME) => {
                let applied = match answered.fields.text(asked_of_the_machine::THE_RELEASE_OUTCOME_FIELD) {
                    Some("released") => TheAnswer::GoOn,
                    Some("stopped") => TheAnswer::Dont,
                    _ => {
                        return self.not_done(operation, "the waiting job was dropped before the answer arrived")
                    }
                };
                let by_this_route = answered
                    .fields
                    .text(asked_of_the_machine::THE_PRODUCED_BY_FIELD)
                    .is_some_and(|by| by.ends_with(&format!("{:?}", smoozy_runtime::holds::AnswerRoute::ThePhone)));
                let at = answered
                    .fields
                    .text(asked_of_the_machine::THE_RELEASE_MOMENT_FIELD)
                    .and_then(the_release_moment_in)
                    .unwrap_or_else(SystemTime::now);
                let outcome = self.done(operation)?;
                if it_was_answered_before || applied != answer || !by_this_route {
                    self.report(
                        &self
                            .job_of_hold(&hold.as_text())
                            .unwrap_or_else(|| hold.as_text()),
                        AJournalledOutcome::ARelease {
                            server: self.the_server(),
                            label,
                            answer,
                            frames_held,
                            waits_for,
                            outcome: AReleasesOutcome::AnotherAnswerWasApplied {
                                answer: applied,
                                at,
                            },
                        },
                    );
                }
                Ok(outcome)
            }
            // **The machine has the answer and its act did not land yet** — a
            // release the store refused, held and re-attempted until it lands
            // (`Rented-server runtime` S14). **Where the hold no longer stands**
            // it was answered and removed, or dropped, meanwhile, and this
            // answer changed nothing.
            Some(crate::unix::THE_ANSWER_IS_STILL_IN_HAND) => {
                if self.records.the_hold(&hold).is_none() {
                    return self.not_done(operation, "this waiting job was already answered, or is no longer on the server");
                }
                self.done(operation)
            }
            _ => Err(format!(
                "the answer was not recorded by the machine: {}",
                the_words_in(&answered, "it said nothing more")
            )),
        }
    }

    /// **`Telegram` Y1: the failed job submitted again, by this machine for
    /// itself** — the map's piece 5, shaped like the stop. The submission
    /// goes under **the button's own submission identity** (`Telegram` R4),
    /// so a second press repeats the first and finds what it made (`Server
    /// protocol and transfers` C4); the supervisor issues the new job's
    /// identity (`State, identity and recovery` Y7a), copies the manifest
    /// and writes the record.
    ///
    /// **What the command's own answer tells the phone, and only that**: a
    /// second press, *… was already resubmitted at …*. A job made now is told
    /// off the journal, where its event carries the mark that says the
    /// machine made it (`crate::the_telegram_journal::the_job_a_resubmission_was_made_from`).
    /// A refusal is recorded *not done* with the machine's words, and the
    /// owner's acts tell it (`Telegram` B9).
    fn submit_again(&mut self, operation: &str, submission: &str, failed_job: &str) -> Result<ARecordedOutcome, String> {
        let Some(failed) = Identity::read_back(IdentityKind::Job, failed_job) else {
            return self.not_done(operation, "the button names no job");
        };
        let Some(submission) = Identity::read_back(IdentityKind::Operation, submission) else {
            return self.not_done(operation, "the button carries no submission identity this machine can read");
        };
        let answered = asked_under(
            self.supervisor.as_ref(),
            submission,
            THE_JOB_IS_SUBMITTED_AGAIN_REQUEST,
            Fields::none().and(asked_of_the_machine::THE_JOB_FIELD, failed.as_text()),
        )
        .map_err(|why| {
            if let WhyItWasNotAnswered::ItIsToEndItself(_) = why {
                self.refused.store(true, Ordering::SeqCst);
            }
            why.in_words()
        })?;
        if answered.the_request_name_it_says_it_does_not_answer().is_some() {
            return Err(format!("this machine's supervisor does not answer \"{THE_JOB_IS_SUBMITTED_AGAIN_REQUEST}\""));
        }
        let words = the_words_in(&answered, "the machine said nothing more");
        match answered.fields.text(WHAT_BECAME_OF_THE_SUBMISSION_FIELD) {
            Some(IT_WAS_SUBMITTED_NOW) => self.done(operation),
            Some(IT_WAS_SUBMITTED_BEFORE) => {
                let at = answered
                    .fields
                    .text(THE_SUBMISSIONS_MOMENT_FIELD)
                    .and_then(the_release_moment_in)
                    .unwrap_or_else(SystemTime::now);
                let outcome = self.done(operation)?;
                let label = self.records.what_the_job_is_called(&failed);
                self.report(
                    failed_job,
                    AJournalledOutcome::ASubmission {
                        server: self.the_server(),
                        label,
                        outcome: ASubmissionsOutcome::AlreadySubmitted { at },
                    },
                );
                Ok(outcome)
            }
            // **The supervisor ended part-way through the first press**: the
            // job stands, and is told off the journal where its event went
            // out, or it does not, and nothing will make it now.
            Some(IT_IS_STILL_BEING_MADE) => match answered.fields.text(asked_of_the_machine::ON_THIS_MACHINE_FIELD) {
                Some(asked_of_the_machine::IT_IS_PRESENT) => self.done(operation),
                _ => self.not_done(operation, "the machine stopped part-way through making the job, and it was not made"),
            },
            Some(IT_WAS_NOT_SUBMITTED) => self.not_done(operation, &words),
            _ => Err(format!("the submission's answer said nothing of what became of it: {words}")),
        }
    }

    /// **`Telegram` R14, `Telegram` I4 and `Telegram` I5 — a routed frame request, answered on this
    /// machine**, the map's piece 6, in place of the answer that said a frame
    /// could not be sent yet. Only the owner's path answers it; the
    /// reporter's, which tells nobody, has no actor.
    ///
    /// 1. **Every recorded copy published**: the frame is sent as the answer,
    ///    as one album, each copy named
    ///    (`Telegram` I4 as written 2026-10-18, the Creator's fourteenth notes,
    ///    note 7; [`TheDerivativeSetOnThisMachine::every_copy`],
    ///    `ThePictures::send_a_frame_with_its_stop`).
    /// 2. **Any recorded copy cleared**: the supervisor is asked to make it again
    ///    through the preview route ([`THE_FRAMES_PICTURE_IS_MADE_AGAIN_REQUEST`]),
    ///    and judges from its own records. *Being made*: the user is told so,
    ///    and the picture is sent once its publication is read off the
    ///    journal, whatever the job's reporting choice ([`TheFramesAsked`]).
    ///    Otherwise the negative case is said in the crate's words (`Telegram` I5).
    /// 3. **A job this machine no longer holds** is said so.
    ///
    /// **Done is recorded once the answer went**; a line that reached nothing
    /// leaves the request to be tried again (`Telegram` B9), and asking again
    /// for a frame already being made changes nothing.
    /// **`Telegram` R13a and `Telegram` C1c** — the newest frame the job's derivative set
    /// holds, where this path answers frame requests at all.
    fn the_newest_frame_held(&mut self, job: &str) -> Option<Option<u64>> {
        self.teller.as_ref()?;
        Some(
            Identity::read_back(IdentityKind::Job, job)
                .and_then(|job| self.records.the_newest_frame_held(&job))
                .and_then(|frame| u64::try_from(frame).ok()),
        )
    }

    /// **`Telegram` V1d on this machine**: `Telegram` V1c's own request for
    /// every job waiting, held or rendering — the supervisor writes both of a
    /// first-frame pair in its one act — then the phone's choice written in
    /// the Telegram worker's state store where it is later. **Each change is
    /// `Telegram` V1c's command under its own identity, and `Telegram` V1c writes a value already in
    /// force again harmlessly**, so an instruction tried again after a part of
    /// it landed changes nothing twice. A job that ended between the reading
    /// and its change is left alone.
    fn change_every_live_jobs_reporting(
        &mut self,
        operation: &str,
        choice: &smoozy_telegram::worker_state::ThePhonesChoice,
    ) -> Option<Result<usize, String>> {
        let mut store = self.context_store.clone()?;
        Some((|| {
            let records = self.records.every_job_record().ok_or("the queue could not be read")?;
            let mut changed = 0;
            let mut pairs_done = HashSet::new();
            for record in records {
                if matches!(record.state, QueueState::Ended { .. }) || pairs_done.contains(&record.job.as_text()) {
                    continue;
                }
                let answered = self.a_command(
                    asked_of_the_machine::THE_TELEGRAM_CHOICE_IS_CHANGED_REQUEST,
                    Fields::none()
                        .and(asked_of_the_machine::THE_JOB_FIELD, record.job.as_text())
                        .and(asked_of_the_machine::THE_REPORTING_CHOICE_FIELD, choice.value.to_string()),
                )?;
                match answered.fields.text(asked_of_the_machine::THE_CHANGES_OUTCOME_FIELD) {
                    Some(asked_of_the_machine::IT_WAS_CHANGED) => changed += 1,
                    Some(asked_of_the_machine::IT_WAS_NOT_CHANGED) => {}
                    _ => return Err(the_words_in(&answered, "a job's reporting was not changed")),
                }
                pairs_done.insert(record.job.as_text());
            }
            store
                .record_the_phones_choice(choice.clone())
                .map_err(|why| format!("the phone's choice was not written: {}", why.0))?;
            let _ = operation;
            Ok(changed)
        })())
    }

    /// **`Telegram` R1a, widened — `Telegram` V1c's change for one job, from a reply.**
    fn change_one_jobs_reporting(
        &mut self,
        operation: &str,
        job: &str,
        choice: smoozy_telegram::job_messages::TheReportingChoice,
    ) -> Option<Result<ARecordedOutcome, String>> {
        self.teller.as_ref()?;
        let _ = operation;
        Some((|| {
            let answered = self.a_command(
                asked_of_the_machine::THE_TELEGRAM_CHOICE_IS_CHANGED_REQUEST,
                Fields::none()
                    .and(asked_of_the_machine::THE_JOB_FIELD, job)
                    .and(asked_of_the_machine::THE_REPORTING_CHOICE_FIELD, choice.in_the_manifests_form().to_string()),
            )?;
            match answered.fields.text(asked_of_the_machine::THE_CHANGES_OUTCOME_FIELD) {
                Some(asked_of_the_machine::IT_WAS_CHANGED) => Ok(ARecordedOutcome::Done),
                Some(asked_of_the_machine::IT_WAS_NOT_CHANGED) => {
                    Ok(ARecordedOutcome::Failed(smoozy_telegram::actions::THE_JOB_OR_ATTEMPT_ENDED.into()))
                }
                // The supervisor holds no record of it, or refused the value:
                // nothing was changed, and the user is told so.
                _ => Ok(ARecordedOutcome::Failed(the_words_in(&answered, "the job's reporting was not changed"))),
            }
        })())
    }

    fn answer_a_frame_request(
        &mut self,
        operation: &str,
        job: &str,
        frame: u64,
    ) -> Option<Result<ARecordedOutcome, String>> {
        use smoozy_telegram::pictures::{APictureSent, ThePictures};
        let teller = self.teller.as_ref()?;
        let (conversation, server, queue, frames, store, machine) = (
            teller.conversation,
            teller.server.clone(),
            teller.queue.clone(),
            teller.frames.clone(),
            self.context_store.clone(),
            teller.machine.clone(),
        );
        let label = Identity::read_back(IdentityKind::Job, job)
            .map(|id| self.records.what_the_job_is_called(&id))
            .unwrap_or_default();
        let generation = Identity::read_back(IdentityKind::Job, job)
            .and_then(|id| self.records.the_job_record(&id))
            .map(|record| self.records.action_generation(&record))
            .unwrap_or_else(|| job.into());
        let subject = TheSubject {
            owning_machine: machine,
            job: Some(job.into()),
            hold: None,
            generation,
        };
        // **`Telegram` C1b: requested output carries Stop only while its job
        // is live** — waiting, rendering or held as the records read now.
        let live = Identity::read_back(IdentityKind::Job, job)
            .and_then(|id| self.records.the_job_record(&id))
            .is_some_and(|record| !matches!(record.state, QueueState::Ended { .. }));
        let messages = || {
            store.as_ref().map(|store| TheJobMessages {
                conversation,
                store: Box::new(store.clone()),
                queue: queue.clone(),
                notes: Box::new(|_| {}),
            })
        };
        let say = |text: smoozy_telegram::job_messages::AMarkedText| match messages() {
            Some(mut messages) => the_answers_outcome(smoozy_telegram::actions::message_result(
                messages.send_about_while(
                    live,
                    &smoozy_telegram::job_messages::AComposedMessage {
                        text,
                        buttons: vec![],
                    },
                    &subject,
                    &server,
                    &label,
                    AClassOfSend::RoutedAnswersAndAcknowledgements,
                ),
            )),
            None => the_answers_outcome(an_answer_said(&queue, conversation, text)),
        };
        let mut pictures = self
            .frame_answers
            .remove(operation)
            .unwrap_or_else(|| ThePictures::for_a_request(conversation, &server));
        let set = TheDerivativeSetOnThisMachine {
            records: self.records.clone(),
        };
        let mut send = |derivative: ADerivative| {
            let copies = set.every_copy(derivative).ok_or_else(|| {
                "the frame's copies were cleared before its answer was read".to_string()
            })?;
            let Some(mut messages) = messages() else {
                return Err("a frame answer has no action-context store".into());
            };
            match pictures.send_a_frame_with_its_stop(
                &mut messages,
                copies,
                AClassOfSend::RoutedAnswersAndAcknowledgements,
                live.then_some(&subject),
            ) {
                APictureSent::NotAccepted(HowASendEnded::HandedBack(
                    WhyTheCallFailed::NotReached(why),
                )) => Err(why),
                _ => Ok(ARecordedOutcome::Done),
            }
        };
        let result = match answer_send_a_frame(&set, &server, job, &label, frame) {
            AnAnswerToSendAFrame::ThePicture(derivative) => send(derivative),
            AnAnswerToSendAFrame::Say(text) => say(text),
            AnAnswerToSendAFrame::ProduceItAgain { .. } => match self.a_command(
                THE_FRAMES_PICTURE_IS_MADE_AGAIN_REQUEST,
                Fields::none()
                    .and(asked_of_the_machine::THE_JOB_FIELD, job)
                    .and(THE_FRAME_ASKED_FIELD, frame.to_string()),
            ) {
                Err(why) => Err(why),
                Ok(answered) => match answered.fields.text(WHAT_BECAME_OF_THE_FRAME_FIELD) {
                    Some(ITS_PICTURE_IS_PUBLISHED) => match set.what_it_holds(job, frame) {
                        WhatTheSetHolds::Held(derivative) => send(derivative),
                        _ => {
                            Err("the frame's copies were cleared before its answer was read".into())
                        }
                    },
                    Some(ITS_PICTURE_IS_BEING_MADE) => {
                        frames.ask(job, i64::try_from(frame).unwrap_or(i64::MAX));
                        say(a_frame_is_being_made_again(&server, &label, frame))
                    }
                    Some(IT_HAS_NOT_FINISHED) => say(
                        match answer_send_a_frame(
                            &ASetThatSays(WhatTheSetHolds::NotFinished),
                            &server,
                            job,
                            &label,
                            frame,
                        ) {
                            AnAnswerToSendAFrame::Say(text) => text,
                            _ => unreachable!(),
                        },
                    ),
                    Some(THERE_IS_NO_PICTURE_OF_IT) => {
                        say(there_is_no_picture_of_the_frame(&server, &label, frame))
                    }
                    Some(THE_JOB_IS_NOT_HERE) => say(
                        match answer_send_a_frame(
                            &ASetThatSays(WhatTheSetHolds::TheRecordIsGone),
                            &server,
                            job,
                            &label,
                            frame,
                        ) {
                            AnAnswerToSendAFrame::Say(text) => text,
                            _ => unreachable!(),
                        },
                    ),
                    _ => Err(format!(
                        "the machine did not say what became of the frame: {}",
                        the_words_in(&answered, "it said nothing more")
                    )),
                },
            },
        };
        if result.is_err() {
            self.frame_answers.insert(operation.into(), pictures);
        }
        Some(result)
    }
}

/// **How many ended jobs one reply to a gather carries** — `Telegram` R13
/// names every job whose record directory still exists, and the machinery
/// message that carries the reply is bounded (`Telegram` B2), so the newest
/// endings are carried and the rest are not. **A Build figure, the map's
/// piece 6**: five ended jobs and their bounded labels sit well inside the
/// bound beside a line of work.
pub const THE_MOST_ENDED_JOBS_A_REPLY_CARRIES: usize = 5;

/// **What this machine holds, read on this machine** — the map's piece 4,
/// added 2026-10-16, in place of the stand-in that read nothing (and made
/// *how is it going* say the machine had nothing to do).
///
/// **The jobs a gather is answered with** (`Telegram` R10): every job still
/// work — rendering, held, or waiting in the line — read straight off the
/// queue in the package area, named as every message names it, **with the
/// live figures the supervisor holds in memory** (`Queue & History` R5), read
/// through the queue reading (`Queue & History` A1) because nothing else holds
/// them. A rendering job whose worker has reported nothing yet says it is
/// rendering and nothing more (`Queue & History` R4: absent, never a nought).
/// ~~**Ended jobs are not in it**: they are `Telegram` R13's candidates for a
/// frame, which the map's piece 6 answers, and a machine's whole history
/// would put its reply over the machinery message's bound.~~ **Since the map's
/// piece 6, the newest ended jobs are in it too** — at most
/// [`THE_MOST_ENDED_JOBS_A_REPLY_CARRIES`], newest ending first, while their
/// record directories stand — so *send a frame* finds a job that has
/// finished (`Telegram` R13), and the reply stays inside the machinery
/// message's bound. They carry neither progress nor a wait, so *how is it
/// going* reads them as nothing to do.
///
/// **A subject's live generation** (`Telegram` R1) is piece 3's rule: a job's
/// is the identity its queue state carries — the job while it waits, the hold
/// while held, the render while it runs and once it has ended. **A hold's is
/// its own identity for as long as its hold record stands**, answered or not,
/// so an answer that lost a race still reaches the machine and is told which
/// answer won and when (`Rented-server runtime` U14); once the record has
/// gone with its subject, its buttons have expired.
pub struct TheLineOnThisMachine {
    supervisor: Arc<dyn AsksTheSupervisor>,
    records: TheRecordsOnThisMachine,
}

impl TheLineOnThisMachine {
    pub fn over(supervisor: Arc<dyn AsksTheSupervisor>, package_area: &Path) -> TheLineOnThisMachine {
        TheLineOnThisMachine { supervisor, records: TheRecordsOnThisMachine::in_the_package_area(package_area) }
    }

    /// **Every rendering job's newest figures**, by job, **the jobs whose
    /// part carries the *stopping* mark** (`Queue & History` R5), **and the
    /// jobs the reading gives as ended**, off one queue reading. Nothing where
    /// the reading did not come back.
    ///
    /// **The mark is read on its own**, never from the figures: after a
    /// restart of the supervisor a stopped render carries the mark and no
    /// figures, and it is still stopping (the live pass's faults, piece E).
    ///
    /// **A job the reading gives as ended while its record still reads
    /// *rendering* is one whose ending is owed to a full store** —
    /// `Rented-server runtime` S14, *Readings stay truthful*: the supervisor
    /// holds the ending it could not write, and its reading carries it as the
    /// job's state. The record catches up at the landing. Step 12's SC4
    /// follow-up.
    fn the_figures(&self) -> WhatTheQueueReadingSays {
        let mut figures = std::collections::HashMap::new();
        let mut stopping = Vec::new();
        let mut ended = std::collections::HashSet::new();
        let Ok(read) = asked(self.supervisor.as_ref(), asked_of_the_machine::THE_QUEUE_READING_REQUEST, Fields::none()) else {
            return WhatTheQueueReadingSays { figures, stopping, ended };
        };
        let how_many: usize = read
            .fields
            .text(asked_of_the_machine::HOW_MANY_JOBS_FIELD)
            .and_then(|text| text.parse().ok())
            .unwrap_or(0);
        for at in 0..how_many {
            let item = |name: &str| {
                read.fields
                    .text(&asked_of_the_machine::the_field_of_item(asked_of_the_machine::THE_JOB_FIELD, at, name))
            };
            let Some(job) = item(asked_of_the_machine::THE_JOB_FIELD).map(str::to_string) else {
                continue;
            };
            if item(asked_of_the_machine::THE_QUEUE_STATE_FIELD) == Some(asked_of_the_machine::IT_HAS_ENDED) {
                ended.insert(job.clone());
            }
            if item(asked_of_the_machine::A_STOPS_INTERRUPT_WENT_OUT_FIELD) == Some(asked_of_the_machine::THE_ANSWER_IS_YES) {
                stopping.push(job.clone());
            }
            if let Some(report) = smoozy_queue_history_machine::readings::AWorkersReport::read_back(item) {
                figures.insert(job, report);
            }
        }
        WhatTheQueueReadingSays { figures, stopping, ended }
    }

    /// **`Telegram` R10's progress for a rendering job**: the frame in hand by
    /// its place in the job's frames, of how many, and how far along — the
    /// plan's units where the worker reports them (`Queue & History` B6), the
    /// frames counted otherwise. **Frame nought where no figure stands**, which
    /// the gather says as *rendering* and nothing more.
    fn the_progress(&self, record: &JobRecord, report: Option<&smoozy_queue_history_machine::readings::AWorkersReport>) -> TheProgress {
        let of = self
            .records
            .the_manifest(record)
            .map_or(0, |manifest| self.records.the_frames_of(record, &manifest).len() as u64);
        let Some(report) = report else {
            return TheProgress { frame: 0, of, percent: 0 };
        };
        let counted = report.readings.value().frames_completed;
        let percent = match (report.the_units_finished, report.the_plan_total) {
            (Some(finished), Some(total)) if total > 0 => finished.saturating_mul(100) / total,
            _ if of > 0 => counted.saturating_mul(100) / of,
            _ => 0,
        };
        TheProgress { frame: (counted + 1).min(of), of, percent: percent.min(100) as u8 }
    }
}

/// **What [`TheLineOnThisMachine`] takes off one queue reading.**
#[derive(Default)]
struct WhatTheQueueReadingSays {
    /// Every rendering job's newest figures, by job.
    figures: std::collections::HashMap<String, smoozy_queue_history_machine::readings::AWorkersReport>,
    /// The jobs whose part carries the *stopping* mark.
    stopping: Vec<String>,
    /// **The jobs the reading gives as ended**, by job — among them any whose
    /// ending is owed to a full store while the record still reads
    /// *rendering* (`Rented-server runtime` S14).
    ended: std::collections::HashSet<String>,
}

impl ReadsWhatThisMachineHolds for TheLineOnThisMachine {
    fn the_jobs(&self) -> Vec<AJobAsHeld> {
        self.the_jobs_and_those_being_stopped().0
    }

    fn the_jobs_and_those_being_stopped(&self) -> (Vec<AJobAsHeld>, Vec<String>) {
        let Some(every) = self.records.every_job_record() else {
            return (Vec::new(), Vec::new());
        };
        let WhatTheQueueReadingSays { figures, stopping, ended: the_reading_has_ended } =
            if every.iter().any(|record| matches!(record.state, QueueState::Rendering { .. })) {
                self.the_figures()
            } else {
                WhatTheQueueReadingSays::default()
            };
        let mut work = Vec::new();
        let mut ended = Vec::new();
        // **Endings owed to a full store, newest of all**: their note has no
        // ending yet, so they go before every ending a note dates.
        let mut owed = Vec::new();
        for record in every {
            let (rendering, waiting) = match &record.state {
                // **`Rented-server runtime` S14: the reading moved it off
                // *rendering***, so the phone does too — never *Rendering* over
                // a render that has ended. Step 12's SC4 follow-up.
                QueueState::Rendering { .. } if the_reading_has_ended.contains(&record.job.as_text()) => {
                    if Path::new(&record.record_directory).is_dir() {
                        owed.push(record);
                    }
                    continue;
                }
                QueueState::Rendering { .. } => {
                    (Some(self.the_progress(&record, figures.get(&record.job.as_text()))), None)
                }
                QueueState::Held { hold } => (
                    None,
                    Some(match self.records.the_hold(hold) {
                        Some(standing) if standing.reason == crate::the_telegram_journal::THE_DRIVERS_REASON => {
                            AJobWaits::ForTheDriverUpdate
                        }
                        _ => AJobWaits::OnYourAnswer,
                    }),
                ),
                QueueState::Waiting => (None, Some(AJobWaits::Queued)),
                QueueState::Ended { .. } => {
                    // **The map's piece 6: an ended job is a frame's candidate**
                    // (`Telegram` R13) while its record directory stands —
                    // the newest few only, below.
                    if Path::new(&record.record_directory).is_dir() {
                        let ended_at = self
                            .records
                            .the_note(&record.job)
                            .and_then(|note| note.ended)
                            .map(|at| at.taken_at());
                        ended.push((ended_at, record));
                    }
                    continue;
                }
            };
            work.push(AJobAsHeld {
                job: record.job.as_text(),
                label: self.records.what_the_job_is_called(&record.job),
                generation: self.records.action_generation(&record),
                record_directory: Path::new(&record.record_directory).is_dir(),
                rendering,
                waiting,
            });
        }
        // **Newest ending first, and no more than the bound**, each label
        // bounded as a drop's is, so a machine's whole history never puts its
        // reply over the machinery message's bound (`Telegram` B2). An ended
        // job whose note says no ending sorts last.
        ended.sort_by(|a, b| b.0.cmp(&a.0));
        work.extend(
            owed.into_iter()
                .map(|record| (None, record))
                .chain(ended)
                .take(THE_MOST_ENDED_JOBS_A_REPLY_CARRIES)
                .map(|(_, record)| {
                    let mut label = self.records.what_the_job_is_called(&record.job);
                    let mut end = label
                        .len()
                        .min(crate::the_telegram_journal::THE_BOUND_ON_A_DROPPED_LABEL);
                    while !label.is_char_boundary(end) {
                        end -= 1;
                    }
                    label.truncate(end);
                    AJobAsHeld {
                        job: record.job.as_text(),
                        label,
                        generation: self.records.action_generation(&record),
                        record_directory: true,
                        rendering: None,
                        waiting: None,
                    }
                }),
        );
        (work, stopping)
    }

    fn the_stop_state(&self, job: &str) -> smoozy_telegram::routing::gather::AStopState {
        use smoozy_telegram::routing::gather::AStopState;
        match Identity::read_back(IdentityKind::Job, job)
            .and_then(|job| self.records.the_job_record(&job))
            .map(|record| record.state)
        {
            Some(QueueState::Rendering { .. }) => AStopState::Rendering,
            Some(QueueState::Waiting) | Some(QueueState::Held { .. }) => AStopState::Waiting,
            _ => AStopState::Ended,
        }
    }

    /// **A subject's text does not say whether it names a job or a hold**
    /// (an identity's written form carries no kind), so the job's record is
    /// looked for first and the hold record after.
    fn the_live_generation(&self, subject: &str) -> Option<String> {
        let job = Identity::read_back(IdentityKind::Job, subject)?;
        if let Some(record) = self.records.the_job_record(&job) {
            return Some(self.records.action_generation(&record));
        }
        let hold = Identity::read_back(IdentityKind::Operation, subject)?;
        self.records.the_hold(&hold).map(|_| hold.as_text())
    }
}

/// **One reader of what this machine holds, handed to the routing and to the
/// owner's acts alike**, the way one store is.
#[derive(Clone)]
struct OneReaderOfWhatIsHeld(Arc<Mutex<Box<dyn ReadsWhatThisMachineHolds>>>);

impl ReadsWhatThisMachineHolds for OneReaderOfWhatIsHeld {
    fn the_jobs(&self) -> Vec<AJobAsHeld> {
        lock(&self.0).the_jobs()
    }
    fn the_live_generation(&self, subject: &str) -> Option<String> {
        lock(&self.0).the_live_generation(subject)
    }
    fn the_stop_state(&self, job: &str) -> smoozy_telegram::routing::gather::AStopState {
        lock(&self.0).the_stop_state(job)
    }
    fn the_jobs_and_those_being_stopped(&self) -> (Vec<AJobAsHeld>, Vec<String>) {
        lock(&self.0).the_jobs_and_those_being_stopped()
    }
}

fn lock<T: ?Sized>(held: &Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    held.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}

// ---------------------------------------------------------------------------
// The status message on its clock — the map's piece 6
// ---------------------------------------------------------------------------

/// **The readings the worker takes itself** — `Rented-server runtime` V2's one
/// named exception: taken by this worker, on its own machine, about that
/// machine, on its own clock, for the status message and nothing else; **never
/// sent to the desktop and never written** (V5). Each reading comes back with
/// the moment it was taken, or as unavailable, and is never carried forward
/// (`Telegram` C1's fifth bullet). The mode hands in
/// [`TheReadingsOnThisMachine`]; a test hands in readings of its own.
pub trait TakesTheStatusReadings: Send {
    /// **The cards' memory**, by the driver's own answer (`Rented-server
    /// runtime` V6's second consequence).
    fn the_cards_memory(&self) -> AReading<CardMemory>;
    /// **Free space on the state area's mount** (`Rented-server runtime` V7),
    /// in gigabytes.
    fn the_free_disk_in_gb(&self) -> AReading<f64>;
    /// **How long the machine has been up.** Asked only on a systemd
    /// machine: on a container no uptime reading is taken (`Telegram` C1's
    /// second bullet, §21 item 21).
    fn how_long_the_machine_has_been_up(&self) -> AReading<Duration>;
}

/// **How long the driver's own tool is given for the status reading.** The
/// reading is taken inside the worker's turn, so a wedged driver must not hold
/// the turn — and with it the holder's heartbeat — for long. **A Build figure,
/// the map's piece 6**: the tool ordinarily answers well inside it, and a slow
/// one is a reading that could not be taken. What the repeated query costs
/// beside a render is `Telegram` §21 item 20's, unmeasured.
pub const HOW_LONG_THE_CARD_TOOL_IS_GIVEN_FOR_THE_STATUS: Duration = Duration::from_secs(2);

/// **The status readings, taken on this machine** — `smoozy_runtime`'s own
/// readings for the cards and the uptime, chosen by the machine kind the
/// supervisor's start names (`Rented-server runtime` K1, V6), and the free
/// space of the package area's mount (V7: free space is a choice of mount).
pub struct TheReadingsOnThisMachine {
    readings: smoozy_runtime::acting::readings::TheMachinesOwnReadings,
    package_area: PathBuf,
}

impl TheReadingsOnThisMachine {
    pub fn on(kind: MachineKind, package_area: &Path) -> TheReadingsOnThisMachine {
        TheReadingsOnThisMachine {
            readings: smoozy_runtime::acting::readings::TheMachinesOwnReadings::on(
                kind,
                HOW_LONG_THE_CARD_TOOL_IS_GIVEN_FOR_THE_STATUS,
            ),
            package_area: package_area.to_path_buf(),
        }
    }
}

/// **A mebibyte's bytes, over a gigabyte's**: the driver prints mebibytes,
/// and a card's memory is said in the gigabytes it is sold by (`24 GB` for
/// 24,564 MiB, the design's own figure for that card).
const MEBIBYTES_IN_A_CARDS_GIGABYTE: f64 = 1024.0;

impl TakesTheStatusReadings for TheReadingsOnThisMachine {
    /// **Every card's line, summed as lines** — V6's count is a count of
    /// lines, and so is this sum; a machine with no line the driver answered
    /// is a reading that could not be taken, never nought.
    fn the_cards_memory(&self) -> AReading<CardMemory> {
        match self.readings.each_cards_memory() {
            Ok(cards) if !cards.is_empty() => AReading::Taken {
                value: CardMemory {
                    used_gb: cards.iter().map(|card| card.used_mib).sum::<f64>() / MEBIBYTES_IN_A_CARDS_GIGABYTE,
                    total_gb: cards.iter().map(|card| card.total_mib).sum::<f64>() / MEBIBYTES_IN_A_CARDS_GIGABYTE,
                },
                at: SystemTime::now(),
            },
            _ => AReading::Unavailable,
        }
    }

    fn the_free_disk_in_gb(&self) -> AReading<f64> {
        match the_free_space_at(&self.package_area) {
            Ok(bytes) => AReading::Taken { value: bytes as f64 / 1_000_000_000.0, at: SystemTime::now() },
            Err(_) => AReading::Unavailable,
        }
    }

    fn how_long_the_machine_has_been_up(&self) -> AReading<Duration> {
        match self.readings.the_machines_own_running_time() {
            Ok(Some(up)) => AReading::Taken { value: up, at: SystemTime::now() },
            _ => AReading::Unavailable,
        }
    }
}

/// **The space an ordinary write may still take on the mount `path` sits
/// on**, in bytes — the supervisor's own reading of the same figure
/// (`the_available_figure_at`), taken here for the status message alone.
fn the_free_space_at(path: &Path) -> Result<u64, String> {
    let named = std::ffi::CString::new(path.as_os_str().as_encoded_bytes())
        .map_err(|why| format!("{}: {why}", path.display()))?;
    // SAFETY: `named` is a valid NUL-terminated string for the whole call, and
    // `read` is a zeroed value of the structure the call fills.
    let mut read: libc::statvfs = unsafe { std::mem::zeroed() };
    let answered = unsafe { libc::statvfs(named.as_ptr(), &mut read) };
    if answered != 0 {
        return Err(format!("{}: {}", path.display(), std::io::Error::last_os_error()));
    }
    #[allow(clippy::unnecessary_cast)]
    Ok((read.f_bavail as u64).saturating_mul(read.f_frsize as u64))
}

/// **`Telegram` C1, `Telegram` C2 and `Telegram` C7 — this machine's status message on its clock**,
/// the map's piece 6.
///
/// **One message, created once and then edited, never pinned, re-used after a
/// restart** — the crate's [`TheClock`] keeps all three, its identity in the
/// worker's store (`Telegram` C7). **Its interval is 5 seconds times the
/// enabled machines** (`Telegram` C2): the machines on the board's roster as this machine last
/// read it, since a machine whose bot is switched off is taken off it
/// (`Telegram` B14), and one where no board has been read yet counts itself.
///
/// **What it says** is the line's facts — which job and how far along, read
/// as the gather reads them, with the live figures off the queue reading — and
/// the readings this worker takes itself ([`TakesTheStatusReadings`]). **A job
/// whose reporting choice leaves out the running commentary** (`Telegram` V1;
/// `smoozy_telegram::job_messages::the_job_reports`) is named and said to be
/// rendering, and nothing of its progress. *Stop this job* rides the message
/// while a job renders (`Telegram` D1). **Once a stop's interrupt has gone out
/// on the render, the message says *Stopping.* in place of the frame line**,
/// with the stop still under it, until the render ends (`Telegram` C1, read off
/// the queue reading's *stopping* mark, `Queue & History` R5; the live pass's
/// faults, piece E).
pub(crate) struct TheStatusOnItsClock {
    clock: TheClock,
    store: OneStore,
    queue: Arc<TheSendQueue>,
    holds: OneReaderOfWhatIsHeld,
    records: TheRecordsOnThisMachine,
    readings: Box<dyn TakesTheStatusReadings>,
    kind: TheMachineKind,
    server: AServerLabel,
    machine: String,
    the_listeners_clock: Arc<dyn ReadsItsOwnClock>,
    the_machines_now: Arc<dyn Fn() -> SystemTime + Send + Sync>,
    refused: Arc<AtomicBool>,
    /// The listener's clock at the last tick, if one was made in this run.
    last_tick: Option<Duration>,
    notes: SaysOnItsComplaintsStream,
}

impl TheStatusOnItsClock {
    /// **One turn**: how the last edit ended is taken in, and a tick is made
    /// where the interval has passed.
    pub(crate) fn a_turn(&mut self, held: &TheListener) {
        if self.refused.load(Ordering::SeqCst) {
            return;
        }
        let mut store = self.store.clone();
        if let Err(why) = self.clock.take_in_what_ended(&mut store) {
            (self.notes)(&format!("the status message: the store did not take in the last edit: {}", why.0));
        }
        let enabled = held.the_board_as_last_read().map_or(1, |board| board.roster.len());
        let now = self.the_listeners_clock.now();
        if self.last_tick.is_some_and(|last| now.saturating_sub(last) < the_interval_for(enabled)) {
            return;
        }
        self.last_tick = Some(now);
        let (status, stop) = self.what_it_says_now();
        // **`Telegram` C1c: *Send a frame* from the job's first finished
        // frame**, read off its derivative set at each tick.
        let finished = stop
            .as_ref()
            .and_then(|subject| subject.job.as_deref())
            .and_then(|job| Identity::read_back(IdentityKind::Job, job))
            .is_some_and(|job| self.records.the_newest_frame_held(&job).is_some());
        self.clock.a_frame_can_be_sent(finished);
        if let Err(why) = self.clock.a_tick(&mut store, &self.queue, &status, stop.as_ref(), (self.the_machines_now)()) {
            (self.notes)(&format!("the status message: the tick was not made: {}", why.0));
        }
    }

    /// **Whether a job's reporting choice lets its progress be said** —
    /// `Telegram` V1's running commentary. A choice that cannot be read is the
    /// narrowest, as everywhere else.
    fn its_progress_is_said(&self, job: &str) -> bool {
        // **`Telegram` V1c: the choice in force at this edit**, so a change to
        // `Nothing` drops the job from the message and a change away brings it
        // back.
        let choice = Identity::read_back(IdentityKind::Job, job)
            .and_then(|job| self.records.the_job_record(&job))
            .map_or(TheReportingChoice::OnlyWhatNeedsAnAnswer, |record| {
                self.records.the_choice_in_force(&record)
            });
        the_job_reports(choice, AKindOfReport::TheStatusMessage)
    }

    /// **What one tick says**, and the job a stop on it would stop.
    fn what_it_says_now(&self) -> (TheStatus, Option<TheSubject>) {
        let (jobs, stopping) = self.holds.the_jobs_and_those_being_stopped();
        let waiting = |waits: AJobWaits| jobs.iter().find(|job| job.waiting == Some(waits));
        let (mut doing, mut stop) = if let Some((job, progress)) =
            jobs.iter().find_map(|job| job.rendering.map(|progress| (job, progress)))
        {
            let progress = if self.its_progress_is_said(&job.job) {
                progress
            } else {
                TheProgress { frame: 0, of: 0, percent: 0 }
            };
            (
                // **`Telegram` C1: once a stop has been sent, *Stopping.* in
                // place of the frame line until the render ends**, with the
                // stop still on the message — Blender goes on through the
                // range after the first interrupt and saves nothing, so its
                // count is not the job's progress. Whatever the job's
                // reporting choice: it is the job's state, not running
                // commentary. The live pass's faults, piece E.
                if stopping.contains(&job.job) {
                    WhatTheMachineIsDoing::Stopping { job_label: job.label.clone() }
                } else {
                    WhatTheMachineIsDoing::Rendering {
                        job_label: job.label.clone(),
                        detail: None,
                        frame: progress.frame,
                        of: progress.of,
                        percent: progress.percent,
                        this_frame: None,
                    }
                },
                Some(TheSubject {
                    owning_machine: self.machine.clone(),
                    job: Some(job.job.clone()),
                    hold: None,
                    generation: job.generation.clone(),
                }),
            )
        } else if let Some(job) = waiting(AJobWaits::OnYourAnswer) {
            (
                WhatTheMachineIsDoing::WaitingOnYou {
                    job_label: job.label.clone(),
                },
                Some(TheSubject {
                    owning_machine: self.machine.clone(),
                    job: Some(job.job.clone()),
                    hold: None,
                    generation: job.generation.clone(),
                }),
            )
        } else if let Some(job) = waiting(AJobWaits::ForTheDriverUpdate) {
            (
                WhatTheMachineIsDoing::WaitingForTheDriverUpdate {
                    job_label: job.label.clone(),
                },
                Some(TheSubject {
                    owning_machine: self.machine.clone(),
                    job: Some(job.job.clone()),
                    hold: None,
                    generation: job.generation.clone(),
                }),
            )
        } else if let Some(job) = waiting(AJobWaits::Queued) {
            (
                WhatTheMachineIsDoing::Queued {
                    job_label: job.label.clone(),
                },
                Some(TheSubject {
                    owning_machine: self.machine.clone(),
                    job: Some(job.job.clone()),
                    hold: None,
                    generation: job.generation.clone(),
                }),
            )
        } else {
            // **`Queue & History` W1 to `Queue & History` W3's quiet stretch**, on this
            // machine's own clock, from the latest ending any note holds;
            // left out where none does.
            //
            // **Not said while an ending is owed to a full store**
            // (`Rented-server runtime` S14): that ending is the latest event
            // that left the line, and no note dates it yet, so a stretch
            // counted from an older note's ending would be longer than the
            // truth. Its record still reads *rendering* where the reading has
            // it ended. Step 12's SC4 follow-up.
            let an_ending_is_owed = jobs.iter().any(|job| {
                job.rendering.is_none()
                    && job.waiting.is_none()
                    && Identity::read_back(IdentityKind::Job, &job.job)
                        .and_then(|job| self.records.the_job_record(&job))
                        .is_some_and(|record| matches!(record.state, QueueState::Rendering { .. }))
            });
            let now = (self.the_machines_now)();
            let quiet_for = self.records.the_latest_ending_on_any_note().filter(|_| !an_ending_is_owed).and_then(|ended| {
                now.duration_since(SystemTime::UNIX_EPOCH)
                    .ok()
                    .and_then(|now| now.as_secs().checked_sub(ended))
                    .map(Duration::from_secs)
            });
            (WhatTheMachineIsDoing::NothingRendering { quiet_for }, None)
        };
        if stop.as_ref().and_then(|subject| subject.job.as_deref())
            .and_then(|job| Identity::read_back(IdentityKind::Job, job))
            .and_then(|job| self.records.the_job_record(&job))
            .is_some_and(|record| self.records.the_job_reports_nothing(&record)) {
            doing = WhatTheMachineIsDoing::WorkNotReported;
            stop = None;
        }
        let status = TheStatus {
            server_label: self.server.clone(),
            doing,
            card_memory: self.readings.the_cards_memory(),
            disk_free_gb: self.readings.the_free_disk_in_gb(),
            uptime: TheUptime::on(self.kind, || self.readings.how_long_the_machine_has_been_up()),
        };
        (status, stop)
    }
}

/// **`Rented-server runtime` K1's two kinds, as the crate names them.**
fn the_kind_the_status_names(kind: MachineKind) -> TheMachineKind {
    match kind {
        MachineKind::Systemd => TheMachineKind::ASystemdMachine,
        MachineKind::Container => TheMachineKind::AContainer,
    }
}

// ---------------------------------------------------------------------------
// Frame pictures and *send a frame* — the map's piece 6
// ---------------------------------------------------------------------------

/// **The frames the user asked for whose picture is being made again**
/// (`Telegram` I4), by job and frame. **Memory only, shared by the owner's
/// path, which asks, and the journal reader, which sends the picture once its
/// publication is read** — as the answer it is, whatever the job's reporting
/// choice. A restart forgets it; the frame is then sent only where the job's
/// choice sends pictures, the shape the crash window of `Telegram` G3 already accepts.
///
/// **Each frame keeps the moment it was asked for**, on the worker's own
/// clock, because **a picture not back in [`A_PICTURE_IS_WAITED_FOR`] is said
/// to have failed** (owed piece 11, the build runner's ruling of 2026-10-16):
/// the supervisor journals nothing for a production that made nothing, so
/// without a limit the user told *being made* would hear nothing more.
#[derive(Clone)]
pub(crate) struct TheFramesAsked {
    asked: Arc<Mutex<std::collections::BTreeMap<(String, i64), Duration>>>,
    clock: Arc<dyn ReadsItsOwnClock>,
}

/// **How long a picture being made again is waited for** before the user is
/// told it could not be made — ten minutes, the build runner's ruling of
/// 2026-10-16 (owed piece 11).
pub(crate) const A_PICTURE_IS_WAITED_FOR: Duration = Duration::from_secs(10 * 60);

impl TheFramesAsked {
    /// Nothing asked yet, the moments read off `clock`.
    pub(crate) fn on(clock: Arc<dyn ReadsItsOwnClock>) -> TheFramesAsked {
        TheFramesAsked { asked: Arc::default(), clock }
    }

    /// **Asked now** — or, for a frame already asked for, left at the moment
    /// it was first asked, so asking again does not put the limit off.
    fn ask(&self, job: &str, frame: i64) {
        let now = self.clock.now();
        lock(&self.asked).entry((job.to_string(), frame)).or_insert(now);
    }

    /// True once for a frame asked for, which is then no longer asked.
    pub(crate) fn answered(&self, job: &str, frame: i64) -> bool {
        lock(&self.asked).remove(&(job.to_string(), frame)).is_some()
    }

    /// Whether a frame is asked for, leaving it asked.
    pub(crate) fn is_asked(&self, job: &str, frame: i64) -> bool {
        lock(&self.asked).contains_key(&(job.to_string(), frame))
    }

    /// **The frames asked for [`A_PICTURE_IS_WAITED_FOR`] or longer ago**,
    /// leaving them asked: each is taken by [`Self::answered`] once the user
    /// has been told.
    pub(crate) fn waited_for_too_long(&self) -> Vec<(String, i64)> {
        let now = self.clock.now();
        lock(&self.asked)
            .iter()
            .filter(|(_, at)| now.saturating_sub(**at) >= A_PICTURE_IS_WAITED_FOR)
            .map(|(frame, _)| frame.clone())
            .collect()
    }
}

/// **`Telegram` I2 and `Telegram` I4 — the derivative set, read on this machine**: the
/// main copy of a frame, at its published name in the job's record directory
/// (`Job files and outputs` B22, `Job files and outputs` B30) — ~~the other
/// View Layers' copies are never read here (`Queue & History` P10)~~ **and,
/// since 2026-10-18, every other copy of it** ([`Self::every_copy`];
/// `Telegram` I4 and `Queue & History` P10 as written that day, the Creator's
/// fourteenth notes, note 7). A frame is held only while every copy its
/// preview records name can be read (`Telegram` I4).
///
/// **What it says where any copy is missing is *cleared*, and that is only
/// the start of the answer**: the worker judges no frame finished
/// (`Telegram` I1), so whether the frame is one a picture can be made of — cleared, owed,
/// not finished, or nothing to make — is the supervisor's to say from its own
/// records, asked with [`THE_FRAMES_PICTURE_IS_MADE_AGAIN_REQUEST`].
pub struct TheDerivativeSetOnThisMachine {
    records: TheRecordsOnThisMachine,
}

impl TheDerivativeSet for TheDerivativeSetOnThisMachine {
    fn what_it_holds(&self, job: &str, frame: u64) -> WhatTheSetHolds {
        let Some(identity) = Identity::read_back(IdentityKind::Job, job) else {
            return WhatTheSetHolds::TheRecordIsGone;
        };
        let Some(record) = self.records.the_job_record(&identity) else {
            return WhatTheSetHolds::TheRecordIsGone;
        };
        if !Path::new(&record.record_directory).is_dir() {
            return WhatTheSetHolds::TheRecordIsGone;
        }
        let label = self.records.what_the_job_is_called(&identity);
        let frame = i64::try_from(frame).unwrap_or(i64::MAX);
        let named = self.records.which_picture_the_preview_record_names(&identity, frame);
        match self.records.the_frames_copies(&record, &identity, &label, frame, named, None)
            .and_then(|copies| copies.into_iter().next())
        {
            // **Its caption names the View Layer, or the source, the preview
            // record it was published from carries** (`Telegram` I2, piece
            // T2c of step 10c), as one sent off its event does.
            Some(derivative) => WhatTheSetHolds::Held(derivative),
            None => WhatTheSetHolds::Cleared,
        }
    }
}

impl TheDerivativeSetOnThisMachine {
    /// **Every copy of the frame `main` is the main copy of**, in its place in
    /// the frame's set — `main` first, then each other View Layer's copy, named
    /// as the preview records name it — read from the set as *send a frame*
    /// reads it, with no production named
    /// ([`TheRecordsOnThisMachine::the_frames_copies`]). **The contact sheet
    /// never reads this** (`Telegram` M2: the main copies alone). A missing
    /// copy or record gives nothing; no partial album can be sent.
    pub fn every_copy(&self, main: ADerivative) -> Option<Vec<ADerivative>> {
        let frame = i64::try_from(main.frame).unwrap_or(i64::MAX);
        let read = Identity::read_back(IdentityKind::Job, &main.job).and_then(|identity| {
            let record = self.records.the_job_record(&identity)?;
            self.records.the_frames_copies(&record, &identity, &main.job_label, frame, main.which_picture.clone(), None)
        });
        match read {
            Some(copies) if copies.first().is_some_and(|first| first.bytes == main.bytes) => Some(copies),
            _ => None,
        }
    }
}

/// **A set that holds one answer**, so the crate's own words are used for a
/// negative case the supervisor named.
struct ASetThatSays(WhatTheSetHolds);

impl TheDerivativeSet for ASetThatSays {
    fn what_it_holds(&self, _: &str, _: u64) -> WhatTheSetHolds {
        self.0.clone()
    }
}

/// **What an answer's send came to, as the outcome to record**: accepted, or
/// refused by Telegram, is done; a line that reached nothing is tried again
/// (`Telegram` B9), so the instruction stays unacknowledged.
fn the_answers_outcome(ended: HowASendEnded) -> Result<ARecordedOutcome, String> {
    match ended {
        HowASendEnded::HandedBack(WhyTheCallFailed::NotReached(why)) => {
            Err(format!("the answer to a frame request reached nothing: {why}"))
        }
        _ => Ok(ARecordedOutcome::Done),
    }
}

/// **A text in answer to a request**, in `Telegram` C6's first class.
pub(crate) fn an_answer_said(queue: &TheSendQueue, conversation: i64, text: smoozy_telegram::job_messages::AMarkedText) -> HowASendEnded {
    wait_for(
        queue,
        queue.offer(
            AClassOfSend::RoutedAnswersAndAcknowledgements,
            Box::new(move |c: &TheClient| c.send_marked_message(conversation, &text, None)),
        ),
    )
}

// ---------------------------------------------------------------------------
// The worker's half: what it can work with
// ---------------------------------------------------------------------------

/// **The conditions the worker cannot fix itself** (`Telegram` F1, its
/// paragraph of 2026-10-12). Each one is said once and the worker stays.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhyItCannotWork {
    NoTokenFile,
    TheTokenFileIsEmpty,
    TheTokenFileCouldNotBeRead(String),
    NoRoleConfiguration,
    TheRoleConfigurationCouldNotBeRead(String),
    TheRoleConfigurationDoesNotRead(&'static str),
    TelegramRefusesTheToken,
    TheStartWasNotGiven(String),
}

impl WhyItCannotWork {
    /// The condition in plain words. **Never the token, never the file's
    /// content.**
    pub fn in_words(&self) -> String {
        match self {
            WhyItCannotWork::NoTokenFile => {
                format!("there is no token file ({THE_TOKEN_FILES_NAME}) in the package area")
            }
            WhyItCannotWork::TheTokenFileIsEmpty => "the token file is empty".to_string(),
            WhyItCannotWork::TheTokenFileCouldNotBeRead(why) => {
                format!("the token file could not be read: {why}")
            }
            WhyItCannotWork::NoRoleConfiguration => format!(
                "there is no role configuration ({THE_ROLE_CONFIGURATIONS_NAME}) in the package area"
            ),
            WhyItCannotWork::TheRoleConfigurationCouldNotBeRead(why) => {
                format!("the role configuration could not be read: {why}")
            }
            WhyItCannotWork::TheRoleConfigurationDoesNotRead(why) => {
                format!("the role configuration does not read: {why}")
            }
            WhyItCannotWork::TelegramRefusesTheToken => {
                "Telegram does not accept the bot token, which is how it answers a token that was \
                 revoked or never existed"
                    .to_string()
            }
            WhyItCannotWork::TheStartWasNotGiven(why) => {
                format!("the supervisor did not give it its start: {why}")
            }
        }
    }
}

/// **The token and the role configuration, read once, and the client built
/// over the token** — or the condition that stops the worker.
///
/// **`reach` builds the client** (`TheClient::reaching_telegram_with`); a test
/// hands in one over a stand-in.
/// **Then one `getMe`**: Telegram answers a revoked token exactly as one that
/// never existed, with a refusal whose code is 401 (`Telegram` Fact 67; the
/// crate's walkthrough reads it the same way), and only that refusal stops the
/// worker. A call that reached nothing is not a refused token: the worker runs,
/// and its listener meets the line as it is.
pub fn what_it_can_work_with(
    package_area: &Path,
    reach: &dyn Fn(&str) -> TheClient,
) -> Result<(TheClient, TheRoleConfiguration), WhyItCannotWork> {
    let token = match std::fs::read(package_area.join(THE_TOKEN_FILES_NAME)) {
        Ok(bytes) => bytes,
        Err(why) if why.kind() == std::io::ErrorKind::NotFound => return Err(WhyItCannotWork::NoTokenFile),
        Err(why) => return Err(WhyItCannotWork::TheTokenFileCouldNotBeRead(format!("{:?}", why.kind()))),
    };
    let token = String::from_utf8(token)
        .map_err(|_| WhyItCannotWork::TheTokenFileCouldNotBeRead("it is not text".into()))?;
    let token = token.trim();
    if token.is_empty() {
        return Err(WhyItCannotWork::TheTokenFileIsEmpty);
    }
    let configuration = match std::fs::read(package_area.join(THE_ROLE_CONFIGURATIONS_NAME)) {
        Ok(bytes) => TheRoleConfiguration::read_back(&bytes)
            .map_err(|why| WhyItCannotWork::TheRoleConfigurationDoesNotRead(why.in_words()))?,
        Err(why) if why.kind() == std::io::ErrorKind::NotFound => {
            return Err(WhyItCannotWork::NoRoleConfiguration)
        }
        Err(why) => {
            return Err(WhyItCannotWork::TheRoleConfigurationCouldNotBeRead(format!("{:?}", why.kind())))
        }
    };
    let client = reach(token);
    match client.get_me() {
        Err(WhyTheCallFailed::Refused(refusal)) if refusal.error_code == Some(401) => {
            Err(WhyItCannotWork::TelegramRefusesTheToken)
        }
        _ => Ok((client, configuration)),
    }
}

// ---------------------------------------------------------------------------
// The worker's half: the crate put together
// ---------------------------------------------------------------------------

/// What the worker says on its complaints stream, in words that never carry
/// the token or a message's content.
pub type SaysOnItsComplaintsStream = Arc<dyn Fn(&str) + Send + Sync>;

/// **Everything the assembly is handed.** The mode hands in the real client,
/// this machine's clock and the queue's real waits; a test hands in a client
/// over a simulated conversation, a clock it moves itself, and waits that wait
/// for nothing.
pub struct WhatTheWorkerIsHanded {
    pub client: TheClient,
    pub clock: Arc<dyn ReadsItsOwnClock>,
    pub waits: Arc<dyn Waits>,
    pub supervisor: Arc<dyn AsksTheSupervisor>,
    pub start: TheWorkersStart,
    pub configuration: TheRoleConfiguration,
    /// **What this machine holds** — [`TheLineOnThisMachine`] in the mode,
    /// since the map's piece 4.
    pub holds: Box<dyn ReadsWhatThisMachineHolds>,
    /// **The package area**, whose records the journal reader and the
    /// captured streams are read from, read-only (the map's piece 3).
    pub package_area: PathBuf,
    /// **The machine's own now** — this machine's clock in the mode, a clock
    /// a test moves on in the tests — for `Queue & History` W3's half hour and
    /// the waiting message's wait for its picture.
    pub the_machines_now: Arc<dyn Fn() -> SystemTime + Send + Sync>,
    /// **The status readings the worker takes itself** — the map's piece 6:
    /// [`TheReadingsOnThisMachine`] in the mode, readings a test sets in the
    /// tests.
    pub readings: Box<dyn TakesTheStatusReadings>,
    pub notes: SaysOnItsComplaintsStream,
}

/// The one store, reached from the listener, the routing, a job's messages
/// and the journal reader.
pub(crate) type OneStore = Arc<Mutex<Box<dyn TheTelegramWorkersState>>>;

/// **The crate, put together once, as the desk's live pass put it together by
/// hand** (`App/Telegram/tests/the_live_pass.rs`, `start_a_process`).
pub struct TheAssembledWorker {
    listener: Arc<Mutex<TheListener>>,
    queue: Arc<TheSendQueue>,
    router: TheRouter,
    acknowledgement: TheListenersAcknowledgement,
    /// **The map's piece 3: the journal read forward, and a job's messages
    /// sent from it**, on every turn after the routing and the acknowledgement.
    journal: TheJournalReader,
    /// **The map's piece 6: the status message on its clock**, on every turn
    /// after the journal.
    status: TheStatusOnItsClock,
    store: OneStore,
    conversation: i64,
    me: WhoThisMachineIs,
    refused: Arc<AtomicBool>,
    notes: SaysOnItsComplaintsStream,
}

/// The display name a machine with no label is announced by.
const A_MACHINE_WITH_NO_LABEL: &str = "A server";

/// **The assembly, in one function** — the map's section 4, piece 2.
///
/// 1. **The client** is handed in.
/// 2. **The send queue** over it (`Telegram` C6).
/// 3. **The store** over the supervisor, from the start ([`TheStoreOverTheSupervisor`]).
/// 4. **The routing**, handed the same store, the reader of what this machine
///    holds, the queue and the clock.
/// 5. **The owner's acts**: the command path over the supervisor, the captured
///    streams read whole off the package area (the map's piece 3), and no lock.
/// 6. **The listener**, its hands being the routing's.
/// 7. **The acknowledgement** — `Telegram` D2's first half.
/// 8. **The journal reader** and the job's messages it sends — the map's piece
///    3 ([`crate::the_telegram_journal`]) — with, from the map's piece 6, a
///    picture per frame, the contact sheet, and the frames asked for.
/// 9. **The status message on its clock** — the map's piece 6
///    ([`TheStatusOnItsClock`]).
pub fn the_worker_assembled(handed: WhatTheWorkerIsHanded) -> TheAssembledWorker {
    let refused = Arc::new(AtomicBool::new(false));
    let conversation = handed.configuration.conversation.0;
    let me = WhoThisMachineIs {
        machine: handed.configuration.machine.clone(),
        display: AServerLabel::with_colour(
            handed.configuration.label.as_deref().unwrap_or(A_MACHINE_WITH_NO_LABEL),
            handed.configuration.colour.as_deref(),
        ),
    };
    let notes = handed.notes;
    let noting = |whose: &'static str| -> Notes {
        let notes = notes.clone();
        Box::new(move |what: &str| notes(&format!("{whose}: {what}")))
    };
    let store: OneStore = Arc::new(Mutex::new(Box::new(TheStoreOverTheSupervisor::over(
        handed.supervisor.clone(),
        &handed.start,
        refused.clone(),
    ))));
    let queue = Arc::new(TheSendQueue::new(handed.client.clone(), handed.waits));
    queue.note_failed_sends_with(notes.clone());
    let holds = OneReaderOfWhatIsHeld(Arc::new(Mutex::new(handed.holds)));
    let records = TheRecordsOnThisMachine::in_the_package_area(&handed.package_area);
    let frames = TheFramesAsked::on(handed.clock.clone());
    let router = TheRouter::new(WhatTheRouterIsHanded {
        conversation,
        me: me.clone(),
        store: Box::new(store.clone()),
        holds: Box::new(holds.clone()),
        queue: queue.clone(),
        clock: handed.clock.clone(),
        notes: noting("the routing"),
    });
    // **The owner's command path tells the phone what a command's own answer
    // says** (the map's piece 4); the reporter's tells nobody, being the one
    // `report` itself is called on.
    let the_owners_acts = |whose: &'static str, telling: bool| TheOwnersActs {
        conversation,
        supervisor: Box::new(TheCommandPathOverTheSupervisor {
            supervisor: handed.supervisor.clone(),
            refused: refused.clone(),
            records: records.clone(),
            context_store: Some(store.clone()),
            frame_answers: std::collections::HashMap::new(),
            teller: telling.then(|| TheTeller {
                conversation,
                server: me.display.clone(),
                machine: me.machine.clone(),
                queue: queue.clone(),
                holds: holds.clone(),
                notes: notes.clone(),
                frames: frames.clone(),
            }),
        }),
        words: Box::new(records.clone()),
        holds: Box::new(holds.clone()),
        queue: queue.clone(),
        notes: noting(whose),
    };
    let owner = the_owners_acts("the owner's acts", true);
    let journal = TheJournalReader::new(WhatTheReaderIsHanded {
        supervisor: handed.supervisor.clone(),
        refused: refused.clone(),
        store: store.clone(),
        messages: TheJobMessages {
            conversation,
            store: Box::new(store.clone()),
            queue: queue.clone(),
            notes: noting("a job's messages"),
        },
        // **`TheOwnersActs::report`'s home**: the same acts, kept for the
        // outcomes the journal carries (`Telegram` D2, A3, Y1).
        reporter: the_owners_acts("the outcomes reported", false),
        records: records.clone(),
        machine: me.machine.clone(),
        server: me.display.clone(),
        reporting: handed.configuration.reporting.clone(),
        the_machines_now: handed.the_machines_now.clone(),
        frames,
        // **`Telegram` I2a counts its minutes on the listener's own clock**,
        // monotonic, never the wall clock (`State, identity and recovery` D5).
        clock: handed.clock.clone(),
        notes: notes.clone(),
    });
    let status = TheStatusOnItsClock {
        clock: TheClock::for_the_conversation(conversation),
        store: store.clone(),
        queue: queue.clone(),
        holds: holds.clone(),
        records: records.clone(),
        readings: handed.readings,
        kind: the_kind_the_status_names(handed.start.machine_kind.clone()),
        server: me.display.clone(),
        machine: me.machine.clone(),
        the_listeners_clock: handed.clock.clone(),
        the_machines_now: handed.the_machines_now.clone(),
        refused: refused.clone(),
        last_tick: None,
        notes: notes.clone(),
    };
    let listener = TheListener::new(WhatTheListenerIsHanded {
        client: handed.client,
        queue: queue.clone(),
        clock: handed.clock,
        me: me.clone(),
        settings: TheSettings::for_the_conversation(conversation),
        store: Box::new(store.clone()),
        actor: Box::new(owner),
        hands_on: Box::new(router.hands()),
        notes: noting("the listener"),
    });
    let listener = Arc::new(Mutex::new(listener));
    let weak_listener = Arc::downgrade(&listener);
    queue.before_a_flood_retry(Arc::new(move || {
        let Some(listener) = weak_listener.upgrade() else { return false };
        let Ok(mut held) = listener.try_lock() else { return false };
        held.renew_before_a_flood_retry()
    }));
    let acknowledgement = TheListenersAcknowledgement {
        conversation,
        queue: queue.clone(),
        said: HashSet::new(),
    };
    TheAssembledWorker {
        listener,
        queue,
        router,
        acknowledgement,
        journal,
        status,
        store,
        conversation,
        me,
        refused,
        notes,
    }
}

/// **Why the run ended.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhyTheRunEnded {
    /// `Telegram` section 19's polite stop, and what the last board edit came to.
    ThePoliteStop(WhatTheLastEditCameTo),
    /// The supervisor refused one of its requests: a worker of an earlier run.
    TheSupervisorToldItToEndItself,
}

/// **What `Telegram` section 19's one final board edit came to.** Best effort,
/// unguaranteed, and nothing rests on it (`Telegram` C5 covers the machine
/// that could not say goodbye).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheLastEditCameTo {
    /// This machine was not holding the slot, so the board was not its own.
    NotHolding,
    /// The board already did not list this machine.
    NothingToMark,
    /// The edit was accepted.
    Edited,
    /// The edit was not made, and why.
    NotMade(String),
}

impl TheAssembledWorker {
    /// **Whether the supervisor has refused one of its requests.** The run
    /// reads the flag itself; a test reads it here.
    #[cfg(test)]
    pub fn is_refused(&self) -> bool {
        self.refused.load(Ordering::SeqCst)
    }

    #[cfg(test)]
    pub fn is_holding(&self) -> bool {
        lock(&self.listener).is_holding()
    }

    /// **A handle for sending a job's messages through this worker's store and
    /// queue** — what a test sends through; the journal reader holds its own.
    #[cfg(test)]
    pub fn the_job_messages(&self) -> TheJobMessages {
        let notes = self.notes.clone();
        TheJobMessages {
            conversation: self.conversation,
            store: Box::new(self.store.clone()),
            queue: self.queue.clone(),
            notes: Box::new(move |what: &str| notes(&format!("a job's messages: {what}"))),
        }
    }

    /// **The journal reader**, for a test to pose a kill between a send and
    /// the cursor's move.
    #[cfg(test)]
    pub fn the_journal(&mut self) -> &mut TheJournalReader {
        &mut self.journal
    }

    /// **The instructions its listener accepted lately** — what a test
    /// delivers again, to pose `Telegram` B9's repeated delivery.
    #[cfg(test)]
    pub fn the_instructions_accepted_lately(&self) -> Vec<smoozy_telegram::board::instructions::AnEntry> {
        lock(&self.listener).the_instructions_accepted_lately()
    }

    /// **One turn by hand, then the ask** — what a test drives the worker by,
    /// on a clock it moves itself. The same order the run takes: the routing,
    /// the acknowledgement, the journal and the status message, the turn, then
    /// the holder's ask — **and then what the run's sender and uploader would
    /// send meanwhile**, sent here, since a test runs neither thread.
    #[cfg(test)]
    pub fn a_turn_by_hand(&mut self) {
        let TheAssembledWorker {
            listener,
            queue,
            router,
            acknowledgement,
            journal,
            status,
            ..
        } = self;
        {
            let mut held = lock(listener);
            router.before_each_turn(&mut held);
            acknowledgement.before_each_turn(&held);
        }
        journal.a_turn();
        let mut held = lock(listener);
        status.a_turn(&held);
        held.a_turn();
        if let Some(ask) = held.the_ask_to_make() {
            let got = held.client().get_updates(ask.offset, ask.wait_seconds);
            held.what_the_ask_brought(ask, got);
        }
        while queue.send_the_next().is_some() {}
        while queue.send_the_next_upload() {}
    }

    /// **The run**: the crate's `run_until_stopped`, with the routing and the
    /// acknowledgement before every turn, until `the_polite_stop_arrived` says
    /// so or the supervisor refuses a request. **Then, on the polite stop, the
    /// one last board edit.**
    pub fn run_until_stopped(
        self,
        the_polite_stop_arrived: impl Fn() -> bool + 'static,
    ) -> WhyTheRunEnded {
        let stop = Arc::new(AtomicBool::new(false));
        let TheAssembledWorker {
            listener,
            queue,
            router,
            mut acknowledgement,
            mut journal,
            mut status,
            store,
            conversation,
            me,
            refused,
            notes,
        } = self;
        // `Telegram` F3a / `Telegram` C6a: report sends never hold the listener's turn lock.
        let journal_thread = {
            let (stop, refused, notes) = (stop.clone(), refused.clone(), notes.clone());
            std::thread::spawn(move || {
                let ended = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    while !stop.load(Ordering::SeqCst) && !refused.load(Ordering::SeqCst) {
                        journal.a_turn();
                        std::thread::sleep(smoozy_telegram::board::staleness::THE_READ_CADENCE);
                    }
                }));
                if ended.is_err() {
                    notes("the journal's reporting thread panicked; the worker ends so the supervisor can restart it");
                    stop.store(true, Ordering::SeqCst);
                }
                notes("the journal's reporting thread ended");
            })
        };
        {
            let (stop, refused) = (stop.clone(), refused.clone());
            run_until_stopped(listener.clone(), queue.clone(), stop.clone(), move |held| {
                if the_polite_stop_arrived() || refused.load(Ordering::SeqCst) {
                    stop.store(true, Ordering::SeqCst);
                    return;
                }
                router.before_each_turn(held);
                acknowledgement.before_each_turn(held);
                // **The map's piece 6**: the status message, on its clock.
                status.a_turn(held);
            });
        }
        if journal_thread.join().is_err() {
            notes("the journal's reporting thread panicked; the worker ends so the supervisor can restart it");
        }
        drop(notes);
        if refused.load(Ordering::SeqCst) {
            return WhyTheRunEnded::TheSupervisorToldItToEndItself;
        }
        WhyTheRunEnded::ThePoliteStop(the_last_board_edit(&listener, &store, conversation, &me))
    }

    /// **`Telegram` section 19's one final board edit, marking this machine
    /// silent** — made where it holds the slot when the polite stop arrives.
    ///
    /// **What *marking silent* is, in the board's own fields**: this machine
    /// taken off the roster (`Telegram` B14). The roster is who a gather asks,
    /// and a machine whose bot has been switched off will answer no gather; a
    /// successor copies the roster as it finds it (`Telegram` B3), and the
    /// machine announces itself again when its bot is next started (`Telegram`
    /// B14's re-announcement). **The heartbeat is not changed**, so a machine standing
    /// down still counts the holder's silence from its last renewal (`Telegram` B5) and
    /// takes over no later for this edit. Nothing else on the board says a
    /// machine is silent, and nothing new is added to its serialization.
    ///
    /// **One edit, never read back**: best effort, and nothing rests on it. The
    /// run makes it through the same function at its end; a test calls it here.
    #[cfg(test)]
    pub fn a_last_board_edit_marking_this_machine_silent(&self) -> WhatTheLastEditCameTo {
        the_last_board_edit(&self.listener, &self.store, self.conversation, &self.me)
    }
}

/// The edit itself, for the run's end and for a test alike.
fn the_last_board_edit(
    listener: &Mutex<TheListener>,
    store: &OneStore,
    conversation: i64,
    me: &WhoThisMachineIs,
) -> WhatTheLastEditCameTo {
    let held = lock(listener);
    if !held.is_holding() {
        return WhatTheLastEditCameTo::NotHolding;
    }
    let (Some(board), Ok(Some(id))) = (held.the_board_as_last_read(), store.the_board()) else {
        return WhatTheLastEditCameTo::NotHolding;
    };
    let mut silent = board.clone();
    silent.roster.retain(|entry| entry.machine != me.machine);
    if silent == board {
        return WhatTheLastEditCameTo::NothingToMark;
    }
    let Ok(text) = silent.compose() else {
        return WhatTheLastEditCameTo::NotMade("the board would be over its bound".into());
    };
    match held.client().edit_a_machinery_message(conversation, id, &text) {
        Ok(_) => WhatTheLastEditCameTo::Edited,
        Err(WhyTheCallFailed::Refused(refusal)) => WhatTheLastEditCameTo::NotMade(format!(
            "Telegram refused it ({})",
            refusal.description.unwrap_or_else(|| "no description".into())
        )),
        Err(WhyTheCallFailed::NotReached(why)) => {
            WhatTheLastEditCameTo::NotMade(format!("it reached nothing: {why}"))
        }
        Err(WhyTheCallFailed::TheAnswerCouldNotBeRead(why)) => {
            WhatTheLastEditCameTo::NotMade(format!("the answer could not be read: {why}"))
        }
    }
}

// ---------------------------------------------------------------------------
// The mode: `smoozy-server work telegram`
// ---------------------------------------------------------------------------

/// **Set by the polite stop**, and read by the turn and by a worker that stays.
static THE_POLITE_STOP_ARRIVED: AtomicBool = AtomicBool::new(false);

extern "C" fn the_polite_stop(_: libc::c_int) {
    THE_POLITE_STOP_ARRIVED.store(true, Ordering::SeqCst);
}

/// **`Telegram` section 19's termination is the interrupt, delivered to this
/// process** (`THE_ROLE_WORKERS_TERMINATION_CONTRACT` in `main.rs`). It is
/// caught here rather than left to end the process, so the turn sees it and the
/// last board edit can be made.
fn catch_the_polite_stop() -> Result<(), String> {
    // SAFETY: the action is zeroed and then filled, and the handler does one
    // thing — a store into an atomic — which is safe inside a signal handler.
    let caught = unsafe {
        let mut action: libc::sigaction = std::mem::zeroed();
        action.sa_sigaction = the_polite_stop as extern "C" fn(libc::c_int) as libc::sighandler_t;
        action.sa_flags = libc::SA_RESTART;
        libc::sigemptyset(&mut action.sa_mask);
        libc::sigaction(libc::SIGINT, &action, std::ptr::null_mut())
    };
    if caught == 0 {
        Ok(())
    } else {
        Err(std::io::Error::last_os_error().to_string())
    }
}

/// **How often a worker that stays looks at the stop flag.** A Build decision
/// and nothing is sized on it: it only keeps the wait from being a spin, and
/// it reads nothing but the flag.
const HOW_OFTEN_A_WORKER_THAT_STAYS_LOOKS: Duration = Duration::from_millis(250);

/// **`Telegram` F1 as narrowed 2026-10-16: a worker that stays asks the
/// supervisor for its start once a minute**, its one call, so a worker of an
/// earlier run is refused and ends rather than idling until a reboot. **The
/// minute is the plan's own figure**, and the real program's test waits it
/// out rather than shortening it through a seam.
pub const HOW_OFTEN_A_WORKER_THAT_STAYS_ASKS_FOR_ITS_START: Duration = Duration::from_secs(60);

/// **What one of a staying worker's asks came to.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheAskCameTo {
    /// The supervisor refused it: a worker of an earlier run, which ends.
    ItIsToEndItself(String),
    /// Its start, the supervisor's words, or no supervisor at all: it stays.
    ItStays,
}

/// **One ask of a staying worker's** — the start request and nothing else,
/// read only for the refusal. **The start it may be given is not taken up**:
/// the condition that keeps the worker staying cannot fix itself, and new
/// material reaches it by the restart the desktop asks for (`Telegram` L4).
pub fn a_staying_workers_ask(supervisor: &dyn AsksTheSupervisor) -> WhatTheAskCameTo {
    match asked(supervisor, THE_TELEGRAM_WORKERS_START_REQUEST, Fields::none()) {
        Err(WhyItWasNotAnswered::ItIsToEndItself(why)) => WhatTheAskCameTo::ItIsToEndItself(why),
        _ => WhatTheAskCameTo::ItStays,
    }
}

/// **`Telegram` F1's paragraph of 2026-10-12, performed, as narrowed
/// 2026-10-16**: said once, then nothing read, and one call — its start,
/// asked once a minute — until the polite stop or a refusal. **It never ends
/// on the condition**: ending would only have the supervisor start it again
/// (`Rented-server runtime` W10) to meet the same condition. **A refusal ends
/// it**, because it is then a worker of an earlier run (§5.3's paragraph
/// after W7).
fn it_stays_until_it_is_stopped(
    why: &WhyItCannotWork,
    supervisor: &dyn AsksTheSupervisor,
) -> Result<(), String> {
    eprintln!(
        "smoozy-server: the Telegram worker cannot work: {}. It stays, reading nothing and asking \
         the supervisor only for its start once a minute, until it is stopped.",
        why.in_words()
    );
    let mut asked_at = std::time::Instant::now();
    while !THE_POLITE_STOP_ARRIVED.load(Ordering::SeqCst) {
        std::thread::sleep(HOW_OFTEN_A_WORKER_THAT_STAYS_LOOKS);
        if asked_at.elapsed() < HOW_OFTEN_A_WORKER_THAT_STAYS_ASKS_FOR_ITS_START {
            continue;
        }
        asked_at = std::time::Instant::now();
        if let WhatTheAskCameTo::ItIsToEndItself(why) = a_staying_workers_ask(supervisor) {
            eprintln!(
                "smoozy-server: the Telegram worker was told to end itself, as a role worker this \
                 supervisor run did not start, and it ends: {why}"
            );
            return Ok(());
        }
    }
    eprintln!("smoozy-server: the Telegram worker was stopped.");
    Ok(())
}

/// **The Telegram role worker's own run** — `smoozy-server work telegram`.
///
/// 1. **The polite stop is caught.**
/// 2. **Its start is asked, from this process.** Told to end itself, it ends;
///    with no supervisor answering, it ends saying so, as every worker of this
///    program does; given only the supervisor's words, it stays.
/// 3. **The token and the role configuration are read once, the client is
///    built and Telegram is asked whether it takes the token**
///    ([`what_it_can_work_with`]). Any condition it cannot fix: it stays.
/// 4. **The crate is assembled and run** until the polite stop or a refusal.
pub fn the_telegram_worker(package_area: &str, recorded_endpoint_path: &str) -> Result<(), String> {
    if let Err(why) = catch_the_polite_stop() {
        eprintln!(
            "smoozy-server: the Telegram worker could not catch the polite stop, so a stop will end \
             it with no last board edit: {why}"
        );
    }
    let supervisor: Arc<dyn AsksTheSupervisor> = Arc::new(TheEndpointAsACaller::at(
        recorded_endpoint_path,
        crate::the_render_worker::HOW_LONG_ONE_EXCHANGE_MAY_TAKE,
    ));
    let start = match the_start_asked(
        supervisor.as_ref(),
        crate::the_render_worker::HOW_LONG_THE_WORKER_LOOKS_FOR_ITS_SUPERVISOR,
    ) {
        Ok(start) => start,
        Err(TheStartWasNotGiven::ItIsToEndItself(why)) => {
            eprintln!(
                "smoozy-server: the Telegram worker was told to end itself, as a role worker this \
                 supervisor run did not start, and it ends: {why}"
            );
            return Ok(());
        }
        Err(TheStartWasNotGiven::NoSupervisorAnswered(why)) => {
            return Err(format!(
                "smoozy-server: the Telegram worker (`Telegram`'s role worker) asked for its start \
                 and no supervisor answered, so it ends: {why}"
            ))
        }
        Err(TheStartWasNotGiven::OnlyWords(why)) => {
            return it_stays_until_it_is_stopped(
                &WhyItCannotWork::TheStartWasNotGiven(why),
                supervisor.as_ref(),
            )
        }
    };
    let (client, configuration) =
        match what_it_can_work_with(Path::new(package_area), &|token| TheClient::reaching_telegram_with(token)) {
            Ok(found) => found,
            Err(why) => return it_stays_until_it_is_stopped(&why, supervisor.as_ref()),
        };
    let holds = Box::new(TheLineOnThisMachine::over(supervisor.clone(), Path::new(package_area)));
    let readings = Box::new(TheReadingsOnThisMachine::on(start.machine_kind.clone(), Path::new(package_area)));
    let worker = the_worker_assembled(WhatTheWorkerIsHanded {
        client,
        clock: Arc::new(OnThisComputer::new()),
        waits: Arc::new(OnThisComputersClock),
        supervisor,
        start,
        configuration,
        holds,
        package_area: PathBuf::from(package_area),
        the_machines_now: Arc::new(SystemTime::now),
        readings,
        notes: Arc::new(|what: &str| eprintln!("smoozy-server: the Telegram worker, {what}")),
    });
    eprintln!("smoozy-server: the Telegram worker is running");
    match worker.run_until_stopped(|| THE_POLITE_STOP_ARRIVED.load(Ordering::SeqCst)) {
        WhyTheRunEnded::TheSupervisorToldItToEndItself => eprintln!(
            "smoozy-server: the Telegram worker was told to end itself, as a role worker this \
             supervisor run did not start, and it ends"
        ),
        WhyTheRunEnded::ThePoliteStop(last) => eprintln!(
            "smoozy-server: the Telegram worker was stopped; its last board edit: {}",
            match last {
                WhatTheLastEditCameTo::NotHolding => "none, as it was not holding the slot".to_string(),
                WhatTheLastEditCameTo::NothingToMark => "none, as the board did not list it".to_string(),
                WhatTheLastEditCameTo::Edited => "made".to_string(),
                WhatTheLastEditCameTo::NotMade(why) => format!("not made: {why}"),
            }
        ),
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// A simulated conversation, for the tests that assemble the worker
// ---------------------------------------------------------------------------

/// **A conversation simulated in memory, for the tests that put the worker
/// together** — here and in `main.rs`, where the supervisor's real answers are.
///
/// **Why not the crate's own stand-in service**: that one answers a call from a
/// list it is handed and holds no conversation, so no slot can be held over
/// it. This keeps the observed ground the crate's own fleet tests keep —
/// the newest ask wins and the older is refused, a pin is a stack and produces
/// an update carrying the pinned message, and a read reports the most recent
/// pinned message by sending date (`Telegram` B13) — over the crate's own
/// `ReachesTelegram`. **It reaches no address and holds no token.**
#[cfg(test)]
pub mod a_simulated_conversation {
    use std::collections::{HashMap, HashSet};
    use std::sync::{Arc, Mutex, MutexGuard};
    use std::time::Duration;

    use serde_json::{json, Value};
    use smoozy_telegram::board::text::{what_is_pinned, TheBoard, WhatIsPinned, THE_HEAD};
    use smoozy_telegram::listener::ReadsItsOwnClock;
    use smoozy_telegram::pacing::Waits;
    use smoozy_telegram::transport::{ReachesTelegram, TheClient, WhatACallCarries};

    /// The one conversation every simulated machine is in.
    pub const THE_CONVERSATION: i64 = 4242;

    fn lock<T>(held: &Mutex<T>) -> MutexGuard<'_, T> {
        held.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    #[derive(Default)]
    pub struct TheConversation {
        next_message: i64,
        next_update: i64,
        next_callback: i64,
        pub texts: HashMap<i64, String>,
        pub pins: Vec<i64>,
        updates: Vec<Value>,
        confirmed_below: i64,
        open_ask: Option<String>,
        refused_next: HashSet<String>,
        /// Every call, as (caller, method, fields).
        pub heard: Vec<(String, String, Value)>,
        /// Callers whose token Telegram does not accept.
        pub refused_tokens: HashSet<String>,
        /// **The messages sent as pictures**, by identity — their captions are
        /// in [`Self::texts`] (the map's piece 3).
        pub photos: Vec<i64>,
        /// **How many of the next pictures sent reach nothing**, the line
        /// being down for them (the map's piece 3).
        pub the_next_pictures_reach_nothing: usize,
        /// **How many of the next albums Telegram refuses as too large**
        /// (*413 Request Entity Too Large*, as the live Telegram pass saw a
        /// contact sheet refused).
        pub the_next_albums_are_refused: usize,
        pub fail_message_once_with_prefix: Option<String>,
        /// Persistent, caller-specific broken sends in the two-server desk test.
        pub broken_pictures_from: HashSet<String>,
        pub broken_texts: Vec<(String, String)>,
        pub unreachable_callers: HashSet<String>,
        /// **The albums sent, each by the messages it made**, in order of
        /// sending — a frame of several View Layers (`Telegram` I2, 2026-10-18)
        /// and the contact sheet (M2).
        pub albums: std::collections::BTreeMap<usize, Vec<i64>>,
        next_album: usize,
        /// **`Telegram` B19 and `Telegram` B18 — every pin's line still standing**, by
        /// the line's own identity, with the message the pin was of.
        pub lines: HashMap<i64, i64>,
        /// The lines a `deleteMessage` took away, in order.
        pub deleted: Vec<i64>,
        /// **Anything asked to be deleted that is not a pin's line** — a
        /// pinned message or any message — which `Telegram` B18 forbids.
        pub asked_to_delete_what_is_not_a_line: Vec<i64>,
        /// **Lines a test has made 48 hours old or older**, which Telegram
        /// refuses to delete (`Sources` entry 87).
        pub past_two_days: HashSet<i64>,
    }

    impl TheConversation {
        fn the_pinned_message(&self, id: i64) -> Value {
            json!({ "message_id": id, "from": { "is_bot": true }, "text": self.texts[&id] })
        }
    }

    /// **The conversation**, shared by every machine's client.
    #[derive(Default)]
    pub struct Sim(Mutex<TheConversation>);

    impl Sim {
        pub fn state(&self) -> MutexGuard<'_, TheConversation> {
            lock(&self.0)
        }

        /// A client for one machine, by name.
        pub fn a_client_for(self: &Arc<Self>, caller: &str) -> TheClient {
            TheClient::over(Arc::new(ACaller { name: caller.to_string(), sim: self.clone() }))
        }

        /// **The user types `text` in the conversation** — `Telegram` R15's
        /// typed requests (the map's piece 4).
        pub fn the_user_says(&self, text: &str) -> i64 {
            let mut s = self.state();
            s.next_message += 1;
            s.next_update += 1;
            let (message, update) = (s.next_message, s.next_update);
            s.updates.push(json!({ "update_id": update, "message": { "message_id": message,
                "chat": { "id": THE_CONVERSATION }, "from": { "is_bot": false }, "text": text } }));
            update
        }

        /// **The user replies to `to` with `text`** (`Telegram` R1a), as a
        /// command picked from the list; Telegram's `date` is the update's
        /// number, so later replies are later.
        pub fn the_user_replies(&self, to: i64, text: &str) -> i64 {
            let mut s = self.state();
            s.next_message += 1;
            s.next_update += 1;
            let (message, update) = (s.next_message, s.next_update);
            s.updates.push(json!({ "update_id": update, "message": { "message_id": message, "date": update,
                "chat": { "id": THE_CONVERSATION }, "from": { "is_bot": false }, "text": text,
                "reply_to_message": { "message_id": to, "from": { "is_bot": true } } } }));
            update
        }

        /// A message from the bot, sent by nobody in particular.
        pub fn a_message_from_the_bot(&self, text: &str) -> i64 {
            let mut s = self.state();
            s.next_message += 1;
            let id = s.next_message;
            s.texts.insert(id, text.to_string());
            id
        }

        /// The user taps a button on `message` carrying `data`.
        pub fn the_user_taps(&self, message: i64, data: &str) -> i64 {
            let mut s = self.state();
            s.next_callback += 1;
            s.next_update += 1;
            let (callback, update) = (format!("cb{}", s.next_callback), s.next_update);
            s.updates.push(json!({ "update_id": update, "callback_query": { "id": callback, "data": data,
                "message": { "message_id": message, "chat": { "id": THE_CONVERSATION } } } }));
            update
        }

        /// The board on top of the pin stack, as a read would report it.
        pub fn the_board_on_top(&self) -> Option<(i64, TheBoard)> {
            let s = self.state();
            let top = *s.pins.iter().max()?;
            match what_is_pinned(&json!({ "pinned_message": s.the_pinned_message(top) })) {
                WhatIsPinned::ABoard { board, .. } => Some((top, board)),
                _ => None,
            }
        }

        /// Every text that is not a board, a machinery message **or a status
        /// message** (the map's piece 6, which [`Self::the_status_messages`]
        /// reads), in order.
        pub fn said_to_the_user(&self) -> Vec<String> {
            let s = self.state();
            let mut said: Vec<(&i64, &String)> = s
                .texts
                .iter()
                .filter(|(_, t)| !t.starts_with(THE_HEAD) && !it_is_a_status_message(t))
                .collect();
            said.sort();
            said.into_iter().map(|(_, t)| t.clone()).collect()
        }

        /// **The status messages in the conversation**, by identity, with
        /// their text as last edited (the map's piece 6).
        pub fn the_status_messages(&self) -> Vec<(i64, String)> {
            let s = self.state();
            let mut found: Vec<(i64, String)> =
                s.texts.iter().filter(|(_, t)| it_is_a_status_message(t)).map(|(id, t)| (*id, t.clone())).collect();
            found.sort();
            found
        }

        /// **Every call one caller made that sent or edited a status
        /// message**, as (method, fields), in order.
        pub fn status_calls(&self, caller: &str) -> Vec<(String, Value)> {
            self.state()
                .heard
                .iter()
                .filter(|(c, m, fields)| {
                    c == caller
                        && (m == "sendMessage" || m == "editMessageText")
                        && fields["text"].as_str().is_some_and(it_is_a_status_message)
                })
                .map(|(_, m, fields)| (m.clone(), fields.clone()))
                .collect()
        }

        /// The calls one caller made with this method.
        pub fn calls(&self, caller: &str, method: &str) -> Vec<Value> {
            self.state()
                .heard
                .iter()
                .filter(|(c, m, _)| c == caller && m == method)
                .map(|(_, _, fields)| fields.clone())
                .collect()
        }
    }

    /// **A status message's text** (`Telegram` C1, `Telegram` C7): its *Using* and
    /// *Updated* lines, or the words an old one is marked superseded with.
    pub fn it_is_a_status_message(text: &str) -> bool {
        (text.contains("\nGPU memory: ") && text.contains("\nUpdated ")) || text.contains("No longer updated. See the newer status message.")
    }

    struct ACaller {
        name: String,
        sim: Arc<Sim>,
    }

    fn refused(code: i64, description: &str) -> Result<String, String> {
        Ok(json!({ "ok": false, "error_code": code, "description": description }).to_string())
    }
    fn accepted(result: Value) -> Result<String, String> {
        Ok(json!({ "ok": true, "result": result }).to_string())
    }

    impl ReachesTelegram for ACaller {
        fn call(&self, method: &str, carries: &WhatACallCarries, _: Duration) -> Result<String, String> {
            let fields = match carries {
                WhatACallCarries::Fields(f) => Value::Object(f.clone()),
                WhatACallCarries::FieldsAndFiles { fields, .. } => Value::Object(fields.clone()),
                _ => json!({}),
            };
            let mut s = self.sim.state();
            s.heard.push((self.name.clone(), method.to_string(), fields.clone()));
            if s.unreachable_callers.contains(&self.name) { return Err("the line is down".into()); }
            if method == "sendMessage" && s.broken_texts.iter().any(|(caller, words)| {
                caller == &self.name && fields["text"].as_str().is_some_and(|text| text.contains(words))
            }) { return Err("this text keeps reaching nothing".into()); }
            if s.refused_tokens.contains(&self.name) {
                return refused(401, "Unauthorized");
            }
            if (method == "sendPhoto" || method == "sendMediaGroup") && s.broken_pictures_from.contains(&self.name) {
                return Err("this picture keeps reaching nothing".into());
            }
            if (method == "sendPhoto" || method == "sendMediaGroup") && s.the_next_pictures_reach_nothing > 0 {
                s.the_next_pictures_reach_nothing -= 1;
                return Err("the line is down".to_string());
            }
            let id = fields.get("message_id").and_then(Value::as_i64).unwrap_or(0);
            match method {
                "getMe" => {
                    accepted(json!({ "id": 1, "is_bot": true, "username": "a_simulated_bot" }))
                }
                "sendMessage" => {
                    if s.fail_message_once_with_prefix
                        .as_ref()
                        .is_some_and(|prefix| {
                            fields["text"]
                                .as_str()
                                .is_some_and(|text| text.starts_with(prefix))
                        })
                    {
                        s.fail_message_once_with_prefix = None;
                        return Err("the question's line reached nothing".into());
                    }
                    s.next_message += 1;
                    let id = s.next_message;
                    s.texts.insert(id, fields["text"].as_str().unwrap_or_default().to_string());
                    accepted(json!({ "message_id": id }))
                }
                "sendPhoto" => {
                    s.next_message += 1;
                    let id = s.next_message;
                    s.texts.insert(id, fields["caption"].as_str().unwrap_or_default().to_string());
                    s.photos.push(id);
                    accepted(json!({ "message_id": id }))
                }
                // **An album, as Telegram sends one** (`Sources` entry 88): one
                // message per picture, each with its own caption, every one
                // given back in order. Learnt 2026-10-18 for a frame of several
                // View Layers (`Telegram` I2); until then an album here answered
                // `true` and kept nothing. **It takes two to ten and no
                // buttons**, and anything else fails the test.
                "sendMediaGroup" => {
                    if s.the_next_albums_are_refused > 0 {
                        s.the_next_albums_are_refused -= 1;
                        return refused(413, "Request Entity Too Large");
                    }
                    let media = fields["media"].as_array().cloned().unwrap_or_default();
                    assert!((2..=10).contains(&media.len()), "an album of {} pictures", media.len());
                    assert!(fields.get("reply_markup").is_none(), "an album was sent with buttons");
                    let mut sent = Vec::new();
                    let album = s.next_album;
                    for one in media {
                        s.next_message += 1;
                        let id = s.next_message;
                        s.texts.insert(id, one["caption"].as_str().unwrap_or_default().to_string());
                        s.photos.push(id);
                        s.albums.entry(album).or_default().push(id);
                        sent.push(json!({ "message_id": id }));
                    }
                    s.next_album += 1;
                    accepted(Value::Array(sent))
                }
                "editMessageText" => {
                    if !s.texts.contains_key(&id) {
                        return refused(400, "Bad Request: message to edit not found");
                    }
                    s.texts.insert(id, fields["text"].as_str().unwrap_or_default().to_string());
                    accepted(json!({ "message_id": id }))
                }
                "pinChatMessage" => {
                    if !s.texts.contains_key(&id) {
                        return refused(400, "Bad Request: message to pin not found");
                    }
                    if !s.pins.contains(&id) {
                        let pinned = s.the_pinned_message(id);
                        s.pins.push(id);
                        s.next_update += 1;
                        let update = s.next_update;
                        // `Telegram` B19: the pin writes a line, a service message with
                        // its own identity (`Sources` entries 43 and 49).
                        s.lines.insert(50_000 + update, id);
                        s.updates.push(json!({ "update_id": update, "message": { "message_id": 50_000 + update,
                            "chat": { "id": THE_CONVERSATION }, "from": { "is_bot": true }, "pinned_message": pinned } }));
                    }
                    accepted(json!(true))
                }
                // **`Telegram` B18**: only a pin's line is deleted, and one 48
                // hours old or older is refused and stays (`Sources` entry 87).
                "deleteMessage" => {
                    if s.pins.contains(&id) || s.texts.contains_key(&id) {
                        s.asked_to_delete_what_is_not_a_line.push(id);
                        return refused(400, "Bad Request: message can't be deleted");
                    }
                    if !s.lines.contains_key(&id) {
                        return refused(400, "Bad Request: message to delete not found");
                    }
                    if s.past_two_days.contains(&id) {
                        return refused(400, "Bad Request: message can't be deleted");
                    }
                    s.lines.remove(&id);
                    s.deleted.push(id);
                    accepted(json!(true))
                }
                "unpinChatMessage" => {
                    s.pins.retain(|p| *p != id);
                    accepted(json!(true))
                }
                "getChat" => match s.pins.iter().max() {
                    Some(top) => accepted(json!({ "id": THE_CONVERSATION, "pinned_message": s.the_pinned_message(*top) })),
                    None => accepted(json!({ "id": THE_CONVERSATION })),
                },
                "getUpdates" => {
                    if s.refused_next.remove(&self.name) {
                        return refused(409, "Conflict: terminated by other getUpdates request");
                    }
                    if let Some(open) = s.open_ask.clone().filter(|o| *o != self.name) {
                        s.refused_next.insert(open);
                    }
                    s.open_ask = Some(self.name.clone());
                    if let Some(offset) = fields.get("offset").and_then(Value::as_i64) {
                        s.confirmed_below = s.confirmed_below.max(offset);
                    }
                    let below = s.confirmed_below;
                    let waiting: Vec<Value> =
                        s.updates.iter().filter(|u| u["update_id"].as_i64().unwrap_or(0) >= below).cloned().collect();
                    accepted(Value::Array(waiting))
                }
                "getWebhookInfo" => accepted(json!({ "url": "" })),
                _ => accepted(json!(true)),
            }
        }
    }

    /// **The machines' clock, moved by the test** — a read cadence a round.
    #[derive(Default)]
    pub struct TheTestsClock(pub Mutex<Duration>);

    impl TheTestsClock {
        pub fn moved_on_by(&self, by: Duration) {
            *lock(&self.0) += by;
        }
    }

    impl ReadsItsOwnClock for TheTestsClock {
        fn now(&self) -> Duration {
            *lock(&self.0)
        }
        fn the_moment(&self) -> String {
            format!("t+{}", lock(&self.0).as_secs())
        }
        /// **The machines' clock moves on by the pause** (`Telegram` B3 and
        /// `Telegram` B11's spaced reads, 2026-10-18), as it would.
        fn pause(&self, how_long: Duration) {
            *lock(&self.0) += how_long;
        }
    }

    /// Waits that wait for nothing, so a flood refusal costs a test no time.
    pub struct NoWaits;

    impl Waits for NoWaits {
        fn wait(&self, _: Duration) {}
    }

    /// **The status readings a test sets** (the map's piece 6): a card's
    /// memory, the free disk and the uptime, each absent until set — and how
    /// many times the uptime was asked, since a container must never ask it.
    #[derive(Debug, Default)]
    pub struct WhatATestSets {
        pub card: Option<(f64, f64)>,
        pub disk_gb: Option<f64>,
        pub up: Option<Duration>,
        pub uptime_asked: usize,
    }

    #[derive(Debug, Clone, Default)]
    pub struct TheReadingsATestSets(pub Arc<Mutex<WhatATestSets>>);

    impl crate::the_telegram_worker::TakesTheStatusReadings for TheReadingsATestSets {
        fn the_cards_memory(&self) -> smoozy_telegram::status::AReading<smoozy_telegram::status::CardMemory> {
            match lock(&self.0).card {
                Some((used_gb, total_gb)) => smoozy_telegram::status::AReading::Taken {
                    value: smoozy_telegram::status::CardMemory { used_gb, total_gb },
                    at: std::time::SystemTime::now(),
                },
                None => smoozy_telegram::status::AReading::Unavailable,
            }
        }
        fn the_free_disk_in_gb(&self) -> smoozy_telegram::status::AReading<f64> {
            match lock(&self.0).disk_gb {
                Some(value) => smoozy_telegram::status::AReading::Taken { value, at: std::time::SystemTime::now() },
                None => smoozy_telegram::status::AReading::Unavailable,
            }
        }
        fn how_long_the_machine_has_been_up(&self) -> smoozy_telegram::status::AReading<Duration> {
            let mut set = lock(&self.0);
            set.uptime_asked += 1;
            match set.up {
                Some(value) => smoozy_telegram::status::AReading::Taken { value, at: std::time::SystemTime::now() },
                None => smoozy_telegram::status::AReading::Unavailable,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **The start the supervisor composes is the start the worker reads.**
    #[test]
    fn the_start_reads_back_as_it_was_composed() {
        let mut state = TheTelegramWorkersStateAsHeld::default();
        state.update_position = Some(9);
        state.action_contexts.insert("a".into(), "{}".into());
        let start = TheWorkersStart {
            run: "r".into(),
            machine_kind: MachineKind::Container,
            installation: None,
            routing_key: "k".into(),
            state,
        };
        let fields = the_start_as_fields(&start).unwrap();
        assert_eq!(the_start_in(&fields).unwrap(), start);
        assert!(the_start_in(&Fields::none()).is_err(), "a start missing its parts is refused");
    }

    /// **Each fact reads back as itself, and writes only its own field.**
    #[test]
    fn a_fact_reads_back_and_writes_only_its_own_field() {
        for fact in [
            AFact::TheUpdatePosition(3),
            AFact::TheBoard(4),
            AFact::TheMachineryMessage(5),
            AFact::TheStatusMessage(6),
            AFact::AnActionContext { identity: "x".into(), record: "{\"a\":1}".into() },
            AFact::TheseActionContextsAreGone(vec!["x".into()]),
        ] {
            assert_eq!(the_fact_in(&the_fact_as_fields(&fact).unwrap()), Some(fact));
        }
        let mut held = TheTelegramWorkersStateAsHeld::default();
        AFact::AnActionContext { identity: "x".into(), record: "r".into() }.written_into(&mut held);
        AFact::AnActionContext { identity: "y".into(), record: "s".into() }.written_into(&mut held);
        AFact::TheseActionContextsAreGone(vec!["x".into()]).written_into(&mut held);
        assert_eq!(held.action_contexts.keys().collect::<Vec<_>>(), vec!["y"]);
        assert_eq!(held.update_position, None);
    }

    #[test]
    fn an_outcome_reads_back_as_itself() {
        for outcome in [
            AnInstructionsOutcome::Done,
            AnInstructionsOutcome::NotDone("the subject has ended".into()),
        ] {
            assert_eq!(AnInstructionsOutcome::read_back(&outcome.as_recorded()), Some(outcome));
        }
        assert_eq!(AnInstructionsOutcome::read_back("maybe"), None);
    }

    /// **The real program's tests spell these words again**, since a
    /// program's own words cannot be reached from there; this holds the two
    /// spellings together.
    #[test]
    fn the_program_runs_spells_the_workers_words_as_the_program_does() {
        let text = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/the_program_runs.rs"),
        )
        .expect("the real program's tests are readable");
        for name in THE_TELEGRAM_WORKERS_OWN_REQUESTS
            .into_iter()
            .chain([THE_REFUSAL_FIELD, END_YOURSELF, THE_ROUTING_KEY_FIELD])
        {
            assert!(text.contains(&format!("\"{name}\"")), "{name} is not spelt there");
        }
    }

    /// **`Telegram` F1's paragraph of 2026-10-12: the conditions the worker
    /// cannot fix itself are each found** — no token file, an empty one, no role
    /// configuration, one that does not read, and Telegram refusing the token — **and each is said in words that carry
    /// neither the token nor the file.** A token Telegram takes, over a
    /// configuration that reads, is worked with.
    #[test]
    fn what_the_worker_cannot_fix_is_found_and_said_without_the_token() {
        use smoozy_telegram::transport::a_stand_in::AStandInService;
        let folder = std::env::temp_dir().join(format!(
            "smoozy-telegram-worker-{}",
            Identity::issue(IdentityKind::Operation).expect("randomness").as_text()
        ));
        std::fs::create_dir_all(&folder).expect("made");
        let a_working_client = |_: &str| -> TheClient { TheClient::over(Arc::new(AStandInService::new())) };
        let found = |reach: &dyn Fn(&str) -> TheClient| {
            let found = what_it_can_work_with(&folder, reach);
            if let Err(why) = &found {
                assert!(!why.in_words().contains("secret-part"), "the token is never said: {why:?}");
                assert!(!why.in_words().contains("9001"), "the file is never quoted: {why:?}");
            }
            found.map(|(_, configuration)| configuration)
        };
        assert_eq!(found(&a_working_client), Err(WhyItCannotWork::NoTokenFile));
        std::fs::write(folder.join(THE_TOKEN_FILES_NAME), b"  \n").expect("written");
        assert_eq!(found(&a_working_client), Err(WhyItCannotWork::TheTokenFileIsEmpty));
        std::fs::write(folder.join(THE_TOKEN_FILES_NAME), b"123:secret-part\n").expect("written");
        assert_eq!(found(&a_working_client), Err(WhyItCannotWork::NoRoleConfiguration));
        std::fs::write(folder.join(THE_ROLE_CONFIGURATIONS_NAME), br#"{"conversation":9001}"#).expect("written");
        assert_eq!(
            found(&a_working_client),
            Err(WhyItCannotWork::TheRoleConfigurationDoesNotRead("it names no machine"))
        );
        let configuration = TheRoleConfiguration {
            conversation: smoozy_telegram::proof::TheConversation(9001),
            machine: "machine-a".into(),
            colour: None,
            label: Some("Server A".into()),
            reporting: vec![],
        };
        std::fs::write(folder.join(THE_ROLE_CONFIGURATIONS_NAME), configuration.written_form()).expect("written");
        let the_token_is_refused = |_: &str| -> TheClient {
            let service = AStandInService::new();
            service.then_answers(r#"{"ok":false,"error_code":401,"description":"Unauthorized"}"#);
            TheClient::over(Arc::new(service))
        };
        assert_eq!(found(&the_token_is_refused), Err(WhyItCannotWork::TelegramRefusesTheToken));
        let unreachable = |_: &str| -> TheClient {
            let service = AStandInService::new();
            service.then_is_not_reached("no route to the service");
            TheClient::over(Arc::new(service))
        };
        assert_eq!(found(&unreachable), Ok(configuration.clone()), "a line that is down is not a refused token");
        assert_eq!(found(&a_working_client), Ok(configuration));
        std::fs::remove_dir_all(&folder).ok();
    }

    #[test]
    fn the_refusal_is_read_as_end_yourself() {
        let request = smoozy_framing::framing::Request::speaking(
            smoozy_framing::framing::THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
            smoozy_state::identities::Identity::issue(
                smoozy_state::identities::IdentityKind::Operation,
            )
            .unwrap(),
            THE_TELEGRAM_WORKERS_START_REQUEST,
        );
        let refused = Answered::to(
            &request,
            smoozy_framing::framing::THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
            "0",
        )
        .carrying(the_refusal("not this run's worker"));
        assert!(it_says_end_yourself(&refused));
    }
}

//! **Wave 5's one live pass over `Telegram`, after its audit round**, on the
//! Creator's own bot (`Audits/Build/Step 10/Brief — wave 5, the live pass.md`).
//!
//! **Off by default, and it says so when skipped.** Run it by hand, at the
//! desk, alone, when the turn is yours, with the conversation's identity taken
//! from piece 4's diary line, the way piece 6's second run took it:
//!
//! `SMOOZY_TELEGRAM_CONVERSATION=<identity> cargo test -p smoozy-telegram --test the_live_pass -- --ignored --exact the_live_pass --nocapture`
//!
//! **What it shows, on one owning desk process and one helper:**
//!
//! - **`Telegram` B24** — every board, machinery message and pin is sent with
//!   `disable_notification`, and every message for the user without it. The
//!   log writes the flag beside each call and how Telegram took it; the
//!   Creator reports what the phone did.
//! - **B25** — every machinery send and edit carries one spoiler entity after
//!   the head, checked on each call against its own text.
//! - **E3d over the cap** — job C's words, 50,000,017 bytes held in memory.
//! - **Item 6 of the bot-and-chat Verdict** — *Stop this job* on a frame
//!   picture (job F) and on the status message (job D), each reaching the
//!   owner's stop and answered (D1, D2).
//! - **Item 7 and item 10** — the first-frame check sent as one picture with
//!   its caption and two buttons (job E), and the answer that won told (A3).
//! - **U7(b) and (c)** — what every read of the pinned message returned,
//!   against the machinery texts Telegram had accepted.
//!
//! **The watch waits for four distinct subjects** — job C's words, hold E's
//! release, job D's stop and job F's stop — never for a count of replies, so a
//! repeated tap (R4) cannot end it early.
//!
//! **Afterwards** it stops both processes, unpins by identity everything they
//! pinned, asks once more with the filter exactly as found, and reads the
//! webhook state again to prove it. **It deletes nothing but the lines its own pins write** (`Telegram` B18, deleted once processed). The token is read
//! by this process from its file and goes nowhere else.

use std::collections::hash_map::DefaultHasher;
use std::collections::{BTreeSet, HashMap};
use std::hash::{Hash, Hasher};
use std::panic::{catch_unwind, resume_unwind, AssertUnwindSafe};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant, SystemTime};

use serde_json::{json, Map, Value};
use smoozy_state::identities::{Identity, IdentityKind};
use smoozy_telegram::actions::a_stand_in::AStandInSupervisor;
use smoozy_telegram::actions::{
    AJournalledOutcome, ReadsTheWordsBack, TheJobMessages, TheListenersAcknowledgement, TheOwnersActs, TheSubject,
};
use smoozy_telegram::board::instructions::AnEntry;
use smoozy_telegram::board::text::{what_is_pinned, WhatIsPinned};
use smoozy_telegram::job_messages::{
    the_failure_message, ACollectionOutcome, AFailedJob, AStream, AWaitingJob, TheCause, WhatTheHoldWaitsFor, THE_CAP_ON_A_FILE,
};
use smoozy_telegram::listener::{
    run_until_stopped, ActsOnAnInstruction, HandsOn, Notes, OnThisComputer, TheListener, TheSettings,
    WhatTheListenerIsHanded, WhoThisMachineIs,
};
use smoozy_telegram::pacing::{AClassOfSend, OnThisComputersClock, TheSendQueue};
use smoozy_telegram::pictures::{ADerivative, APictureSent, ThePictures};
use smoozy_telegram::routing::gather::{AJobAsHeld, ReadsWhatThisMachineHolds};
use smoozy_telegram::routing::{TheRouter, TheRoutersHands, WhatTheRouterIsHanded};
use smoozy_telegram::status::{
    the_interval_for, AReading, CardMemory, TheClock, TheMachineKind, TheStatus, TheUptime, WhatTheMachineIsDoing,
};
use smoozy_telegram::transport::{OverHttps, ReachesTelegram, TheClient, WhatACallCarries};
use smoozy_telegram::worker_state::{InMemory, TheTelegramWorkersState};

/// How long the Creator is given for the four taps.
const THE_CREATORS_TAPS: Duration = Duration::from_secs(30 * 60);

const SERVER: &str = "Desk process";
const HELPER: &str = "Desk helper";

/// Where the stand-in frame pictures were made, outside the repository.
const THE_PICTURES: &str = "SMOOZY_LIVE_PASS_PICTURES";

fn the_token() -> String {
    let home = std::env::var_os("HOME").expect("this desk has a home folder");
    let text = std::fs::read_to_string(Path::new(&home).join(".smoozy-keys").join("telegram.txt"))
        .expect("the bot token file is readable");
    let shaped: Vec<&str> = text
        .lines()
        .map(|line| line.split_once('=').map_or(line.trim(), |(_, value)| value.trim()))
        .filter(|value| match value.split_once(':') {
            Some((bot, secret)) => {
                !bot.is_empty()
                    && bot.chars().all(|c| c.is_ascii_digit())
                    && !secret.is_empty()
                    && secret.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
            }
            None => false,
        })
        .collect();
    assert!(shaped.len() == 1, "the key file does not hold exactly one line shaped like a bot token");
    shaped[0].to_string()
}

#[derive(Clone, Default)]
struct TheLog(Arc<Mutex<Vec<String>>>);

impl TheLog {
    fn notes(&self, started: Instant, who: &'static str) -> Notes {
        let log = self.clone();
        Box::new(move |what| log.push(started, &format!("[{who}] {what}")))
    }
    fn push(&self, started: Instant, what: &str) {
        let line = format!("{:9.3} {what}", started.elapsed().as_secs_f64());
        eprintln!("  | {line}");
        self.0.lock().unwrap_or_else(|p| p.into_inner()).push(line);
    }
    fn lines(&self, containing: &str) -> Vec<String> {
        self.0.lock().unwrap_or_else(|p| p.into_inner()).iter().filter(|l| l.contains(containing)).cloned().collect()
    }
}

/// A fingerprint of a text, so the log can compare texts without holding one.
fn fingerprint(text: &str) -> String {
    let mut h = DefaultHasher::new();
    text.hash(&mut h);
    format!("{:016x}", h.finish())
}

/// **B25, checked on the call itself**: one spoiler entity, starting at or
/// after the head's blank line, with only whitespace between them, and running
/// to the end of the text but for trailing whitespace. Units are UTF-16.
fn the_spoiler_on(fields: &Map<String, Value>) -> String {
    let text = fields.get("text").and_then(Value::as_str).unwrap_or_default();
    let spoilers: Vec<&Value> = fields
        .get("entities")
        .and_then(Value::as_array)
        .map(|all| all.iter().filter(|e| e["type"] == "spoiler").collect())
        .unwrap_or_default();
    if spoilers.is_empty() {
        return "none".into();
    }
    if spoilers.len() > 1 {
        return format!("{} spoilers (wrong)", spoilers.len());
    }
    let units: Vec<u16> = text.encode_utf16().collect();
    let (offset, length) = (spoilers[0]["offset"].as_u64().unwrap_or(0) as usize, spoilers[0]["length"].as_u64().unwrap_or(0) as usize);
    let head = text.find("\n\n").map(|b| text[..b].encode_utf16().count());
    let right = match head {
        None => false,
        Some(head) => {
            offset >= head
                && offset + length <= units.len()
                && String::from_utf16_lossy(&units[head..offset]).trim().is_empty()
                && String::from_utf16_lossy(&units[offset + length..]).trim().is_empty()
                && length > 0
        }
    };
    format!(
        "offset {offset} length {length} of {} units, head {} units, {}",
        units.len(),
        head.map_or("missing".into(), |h| h.to_string()),
        if right { "right" } else { "WRONG" }
    )
}

/// **What the log needs from every machinery text Telegram accepted**, per
/// message, in the order accepted — for U7(c).
#[derive(Default)]
struct TheAcceptedTexts {
    by_message: HashMap<i64, Vec<String>>,
    last_unpin: Option<Instant>,
}

/// **Every call written down with its time, how it ended, whether it was sent
/// silently, and its spoiler** — never its content or the token.
struct WritesDownEveryCall {
    inner: OverHttps,
    log: TheLog,
    started: Instant,
    who: &'static str,
    accepted: Arc<Mutex<TheAcceptedTexts>>,
}

impl ReachesTelegram for WritesDownEveryCall {
    fn call(&self, method: &str, carries: &WhatACallCarries, bound: Duration) -> Result<String, String> {
        let answered = self.inner.call(method, carries, bound);
        let result = answered.as_deref().ok().map(smoozy_telegram::transport::what_the_answer_says);
        let outcome = match &result {
            None => "not reached".to_string(),
            Some(Ok(_)) => "ok".to_string(),
            Some(Err(smoozy_telegram::transport::WhyTheCallFailed::Refused(r))) => {
                format!("refused {:?} retry_after={:?} {:?}", r.error_code, r.retry_after, r.description)
            }
            Some(Err(_)) => "unreadable".to_string(),
        };
        let ok = matches!(result, Some(Ok(_)));
        let empty = Map::new();
        let fields = match carries {
            WhatACallCarries::Fields(f) => f,
            WhatACallCarries::FieldsAndFiles { fields, .. } => fields,
            WhatACallCarries::Nothing => &empty,
        };
        let when = self.started.elapsed().as_secs_f64();
        match method {
            "getUpdates" => {}
            "getChat" => {
                // U7(b) and (c): what every read of the pinned message returned.
                let mut accepted = self.accepted.lock().unwrap_or_else(|p| p.into_inner());
                let since_unpin = accepted.last_unpin.map(|t| format!("{:.1}", t.elapsed().as_secs_f64()));
                let line = match result.as_ref().and_then(|r| r.as_ref().ok()) {
                    None => format!("read the pin: {outcome}"),
                    Some(chat) => match chat.get("pinned_message") {
                        None => format!("read the pin: nothing pinned, {}s after the last unpin", since_unpin.unwrap_or("-".into())),
                        Some(pinned) => {
                            let id = pinned.get("message_id").and_then(Value::as_i64).unwrap_or_default();
                            let text = pinned.get("text").and_then(Value::as_str).unwrap_or_default();
                            let print = fingerprint(text);
                            let texts = accepted.by_message.entry(id).or_default();
                            let standing = match texts.iter().rposition(|t| *t == print) {
                                None if texts.is_empty() => "not a machinery text this run sent".to_string(),
                                None => "a text not yet answered as accepted".to_string(),
                                Some(i) if i + 1 == texts.len() => "the newest accepted text".to_string(),
                                Some(i) => format!("an OLDER accepted text, {} behind", texts.len() - 1 - i),
                            };
                            let kind = match what_is_pinned(chat) {
                                WhatIsPinned::Nothing => "nothing",
                                WhatIsPinned::ABoard { .. } => "a board",
                                WhatIsPinned::AMachineryMessage { .. } => "a machinery message",
                                _ => "something else",
                            };
                            let entities = pinned.get("entities").map_or("no entities".to_string(), |e| format!("entities {e}"));
                            format!("read the pin: message {id}, {kind}, text {print}, {standing}; {entities}")
                        }
                    },
                };
                drop(accepted);
                self.log.push(self.started, &format!("[{}] {line}", self.who));
            }
            _ => {
                let silent = fields.get("disable_notification") == Some(&json!(true));
                let spoiler = the_spoiler_on(fields);
                let buttons = fields.contains_key("reply_markup");
                let answered_id = result
                    .as_ref()
                    .and_then(|r| r.as_ref().ok())
                    .and_then(|r| r.get("message_id"))
                    .and_then(Value::as_i64);
                let message = fields.get("message_id").and_then(Value::as_i64).or(answered_id);
                let mut accepted = self.accepted.lock().unwrap_or_else(|p| p.into_inner());
                if ok && spoiler != "none" && (method == "sendMessage" || method == "editMessageText") {
                    if let (Some(id), Some(text)) = (message, fields.get("text").and_then(Value::as_str)) {
                        accepted.by_message.entry(id).or_default().push(fingerprint(text));
                    }
                }
                if ok && method == "unpinChatMessage" {
                    accepted.last_unpin = Some(Instant::now());
                }
                let text = fields.get("text").or_else(|| fields.get("caption")).and_then(Value::as_str);
                drop(accepted);
                self.log.push(
                    self.started,
                    &format!(
                        "[{}] call {method} {outcome}; message {}; silent = {silent}; spoiler: {spoiler}; buttons = {buttons}{}",
                        self.who,
                        message.map_or("-".into(), |m| m.to_string()),
                        text.map_or(String::new(), |t| format!("; text {}", fingerprint(t))),
                    ),
                );
                let _ = when;
            }
        }
        answered
    }
}

/// **What the owning desk process holds**, live at generation 1.
struct DeskHolds;

impl ReadsWhatThisMachineHolds for DeskHolds {
    fn the_jobs(&self) -> Vec<AJobAsHeld> {
        Vec::new()
    }
    fn the_live_generation(&self, subject: &str) -> Option<String> {
        ["job-c", "job-d", "job-f", "hold-e"].contains(&subject).then(|| "1".to_string())
    }
}

/// **What the helper holds: nothing.**
struct NothingHeld;

impl ReadsWhatThisMachineHolds for NothingHeld {
    fn the_jobs(&self) -> Vec<AJobAsHeld> {
        Vec::new()
    }
    fn the_live_generation(&self, _: &str) -> Option<String> {
        None
    }
}

/// **The helper owns nothing**, so nothing is ever addressed to it.
struct ActsOnNothing;

impl ActsOnAnInstruction for ActsOnNothing {
    fn act_once(&mut self, entry: &AnEntry) -> Result<(), String> {
        Err(format!("the helper owns nothing, and was handed {}", entry.action))
    }
}

struct WritesDownEveryRead {
    inner: AStandInSupervisor,
    log: TheLog,
    started: Instant,
}

impl ReadsTheWordsBack for WritesDownEveryRead {
    fn read_whole(&self, job: &str, attempt: u32, stream: AStream) -> Result<Vec<u8>, String> {
        let read = self.inner.read_whole(job, attempt, stream);
        let length = read.as_ref().map_or("unread".to_string(), |b| b.len().to_string());
        self.log.push(self.started, &format!("read the words of {job} {} ({length} bytes)", stream.the_name()));
        read
    }
}

struct SetAsideThenRoute {
    routing: TheRoutersHands,
    set_aside_through: i64,
    log: TheLog,
    started: Instant,
}

impl HandsOn for SetAsideThenRoute {
    fn an_update_from_the_conversation(&mut self, update: &Value) -> Result<(), String> {
        if let Some(id) = update.get("update_id").and_then(Value::as_i64).filter(|id| *id <= self.set_aside_through) {
            self.log.push(self.started, &format!("set aside update {id}, from before the watch"));
            return Ok(());
        }
        self.routing.an_update_from_the_conversation(update)
    }
    fn an_entry_addressed_here(&mut self, entry: &AnEntry) -> Option<Value> {
        self.routing.an_entry_addressed_here(entry)
    }
    fn an_entry_ended_at_its_deadline(&mut self, entry: &AnEntry) {
        self.routing.an_entry_ended_at_its_deadline(entry);
    }
    fn a_reply_to_a_gather(&mut self, from: &str, reply: &Value) {
        self.routing.a_reply_to_a_gather(from, reply);
    }
    fn an_entry_was_completed(&mut self, entry: &AnEntry) {
        self.routing.an_entry_was_completed(entry);
    }
    fn an_entry_was_not_admitted(&mut self, entry: &AnEntry) {
        self.routing.an_entry_was_not_admitted(entry);
    }
}

fn wait_for(what: &str, most: Duration, mut done: impl FnMut() -> bool) -> bool {
    let started = Instant::now();
    loop {
        if done() {
            eprintln!("{what}: after {} seconds", started.elapsed().as_secs());
            return true;
        }
        if started.elapsed() >= most {
            eprintln!("{what}: NOT seen within {} seconds", most.as_secs());
            return false;
        }
        std::thread::sleep(Duration::from_secs(2));
    }
}

fn a_render_line(n: usize) -> String {
    format!(
        "Fra:{n} Mem:812.4M | Time:00:{:02}.{:02} | Rendering {n} / 40 | C:\\Users\\Анна\\レンダー\\scene_{n}.blend 🎬 *_[]()~`>#+-=|{{}}.!\n",
        n % 60,
        n % 100
    )
}

fn the_status(label: &str, rendering: bool) -> TheStatus {
    let now = SystemTime::now();
    TheStatus {
        server_label: SERVER.into(),
        doing: if rendering {
            WhatTheMachineIsDoing::Rendering {
                job_label: label.into(),
                detail: Some("Desk test".into()),
                frame: 13,
                of: 40,
                percent: 32,
                this_frame: None,
            }
        } else {
            WhatTheMachineIsDoing::NothingRendering { quiet_for: None }
        },
        card_memory: AReading::Taken { value: CardMemory { used_gb: 18.2, total_gb: 24.0 }, at: now },
        disk_free_gb: AReading::Taken { value: 61.4, at: now },
        uptime: TheUptime::on(TheMachineKind::ASystemdMachine, || AReading::Taken { value: Duration::from_secs(3600), at: now }),
    }
}

/// One listener, running on its own threads.
struct AProcess {
    stop: Arc<AtomicBool>,
    worker: Option<std::thread::JoinHandle<()>>,
    holding: Arc<AtomicBool>,
}

impl AProcess {
    fn stop(&mut self) {
        self.stop.store(true, Ordering::SeqCst);
        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
    }
}

#[derive(Default)]
struct TheRun {
    processes: Vec<AProcess>,
    side_threads_stop: Arc<AtomicBool>,
    side_threads: Vec<std::thread::JoinHandle<()>>,
}

fn a_client(token: &str, log: &TheLog, started: Instant, who: &'static str, accepted: &Arc<Mutex<TheAcceptedTexts>>) -> TheClient {
    TheClient::over(Arc::new(WritesDownEveryCall {
        inner: OverHttps::reaching_telegram_with(token),
        log: log.clone(),
        started,
        who,
        accepted: accepted.clone(),
    }))
}

#[allow(clippy::too_many_arguments)]
fn start_a_process(
    run: &mut TheRun,
    client: TheClient,
    conversation: i64,
    me: WhoThisMachineIs,
    store: Arc<Mutex<Box<dyn TheTelegramWorkersState>>>,
    queue: Arc<TheSendQueue>,
    actor: Box<dyn ActsOnAnInstruction>,
    holds: Box<dyn ReadsWhatThisMachineHolds>,
    set_aside_through: i64,
    log: &TheLog,
    started: Instant,
    who: &'static str,
) -> Arc<AtomicBool> {
    let clock = Arc::new(OnThisComputer::new());
    let router = TheRouter::new(WhatTheRouterIsHanded {
        conversation,
        me: me.clone(),
        store: Box::new(store.clone()),
        holds,
        queue: queue.clone(),
        clock: clock.clone(),
        notes: log.notes(started, who),
    });
    let listener = Arc::new(Mutex::new(TheListener::new(WhatTheListenerIsHanded {
        client,
        queue: queue.clone(),
        clock,
        me,
        settings: TheSettings::for_the_conversation(conversation),
        store: Box::new(store),
        actor,
        hands_on: Box::new(SetAsideThenRoute { routing: router.hands(), set_aside_through, log: log.clone(), started }),
        notes: log.notes(started, who),
    })));
    let holding = Arc::new(AtomicBool::new(false));
    let stop = Arc::new(AtomicBool::new(false));
    // D2's first half, said by whichever process holds.
    let mut acknowledgement = TheListenersAcknowledgement { conversation, queue: queue.clone(), said: Default::default() };
    let worker = {
        let (listener, queue, stop, holding) = (listener, queue, stop.clone(), holding.clone());
        std::thread::spawn(move || {
            run_until_stopped(listener, queue, stop, move |held| {
                router.before_each_turn(held);
                acknowledgement.before_each_turn(held);
                holding.store(held.is_holding(), Ordering::SeqCst);
            })
        })
    };
    run.processes.push(AProcess { stop, worker: Some(worker), holding: holding.clone() });
    holding
}

fn the_watch(token: &str, conversation: i64, run: &mut TheRun, log: &TheLog, accepted: &Arc<Mutex<TheAcceptedTexts>>, started: Instant) {
    let pictures = std::env::var_os(THE_PICTURES).expect("the folder of stand-in frame pictures is named");
    let frame_e = std::fs::read(Path::new(&pictures).join("frame-e.jpg")).expect("frame E is readable");
    let frame_f = std::fs::read(Path::new(&pictures).join("frame-f.jpg")).expect("frame F is readable");

    let owner_client = a_client(token, log, started, "owner", accepted);
    let waiting = owner_client.get_updates(None, 0).expect("the ask was answered");
    let ids: Vec<i64> = waiting.as_array().into_iter().flatten().filter_map(|u| u.get("update_id")?.as_i64()).collect();
    eprintln!("updates waiting before the watch began, to be set aside: {ids:?}");
    let set_aside_through = ids.iter().copied().max().unwrap_or(0);

    // ---- job C's over-cap stream, held in memory only
    let supervisor = AStandInSupervisor::default();
    let mut over: Vec<u8> = Vec::with_capacity(THE_CAP_ON_A_FILE as usize + 200);
    let mut n = 0;
    while (over.len() as u64) <= THE_CAP_ON_A_FILE {
        n += 1;
        over.extend_from_slice(a_render_line(n % 1000).as_bytes());
    }
    over.extend_from_slice(b"Blender quit\n");
    eprintln!("seeded: job C output {} bytes (over the cap of {THE_CAP_ON_A_FILE}, held in memory only)", over.len());
    supervisor.heard().words.insert(("job-c".into(), AStream::TheOrdinaryOutput), over);

    let routing_key = Identity::issue(IdentityKind::Operation).unwrap().as_text();
    let me = WhoThisMachineIs { machine: Identity::issue(IdentityKind::Installation).unwrap().as_text(), display: SERVER.into() };
    let store: Arc<Mutex<Box<dyn TheTelegramWorkersState>>> = Arc::new(Mutex::new(Box::new(InMemory::with_routing_key(&routing_key))));
    let queue = Arc::new(TheSendQueue::new(owner_client.clone(), Arc::new(OnThisComputersClock)));
    let owner = TheOwnersActs {
        conversation,
        supervisor: Box::new(supervisor.clone()),
        words: Box::new(WritesDownEveryRead { inner: supervisor.clone(), log: log.clone(), started }),
        holds: Box::new(DeskHolds),
        queue: queue.clone(),
        notes: log.notes(started, "owner"),
    };
    let holding = start_a_process(
        run,
        owner_client.clone(),
        conversation,
        me.clone(),
        store.clone(),
        queue.clone(),
        Box::new(owner),
        Box::new(DeskHolds),
        set_aside_through,
        log,
        started,
        "owner",
    );
    assert!(wait_for("the owning desk process holds the slot", Duration::from_secs(300), || holding.load(Ordering::SeqCst)), "the slot was never held");

    // ---- the journal reader, which step 8b's machine will be (D2, A3)
    {
        let mut reporter = TheOwnersActs {
            conversation,
            supervisor: Box::new(supervisor.clone()),
            words: Box::new(supervisor.clone()),
            holds: Box::new(DeskHolds),
            queue: queue.clone(),
            notes: log.notes(started, "journal"),
        };
        let (supervisor, stop) = (supervisor.clone(), run.side_threads_stop.clone());
        let labels: HashMap<&str, &str> =
            [("job-d", "Desk test job D"), ("job-f", "Desk test job F"), ("hold-e", "Desk test job E")].into_iter().collect();
        run.side_threads.push(std::thread::spawn(move || {
            let (mut stops, mut releases) = (0, 0);
            while !stop.load(Ordering::SeqCst) {
                let (new_stops, new_releases) = {
                    let heard = supervisor.heard();
                    (heard.stops[stops..].to_vec(), heard.releases[releases..].to_vec())
                };
                for (job, outcome) in new_stops {
                    stops += 1;
                    let label = labels.get(job.as_str()).copied().unwrap_or("a job").to_string();
                    reporter.report(AJournalledOutcome::AStop { server: SERVER.into(), label, outcome });
                }
                for (hold, answer, outcome) in new_releases {
                    releases += 1;
                    let label = labels.get(hold.as_str()).copied().unwrap_or("a job").to_string();
                    reporter.report(AJournalledOutcome::ARelease {
                        server: SERVER.into(),
                        label,
                        answer,
                        frames_held: 39,
                        waits_for: WhatTheHoldWaitsFor::TheUsersAnswer,
                        outcome,
                    });
                }
                std::thread::sleep(Duration::from_millis(500));
            }
        }));
    }

    // ---- the messages the Creator taps, in the order they arrive
    let mut messages = TheJobMessages { conversation, store: Box::new(store.clone()), queue: queue.clone(), notes: log.notes(started, "owner") };
    let a_job = |job: &str| TheSubject { owning_machine: me.machine.clone(), job: Some(job.into()), hold: None, generation: "1".into() };

    // 1. Job C: a failure whose words are over the cap (E3d).
    let failed = AFailedJob {
        server: SERVER.into(),
        job: "job-c".into(),
        label: "Desk test job C".into(),
        generation: "1".into(),
        attempt: 1,
        at_frame: Some((13, 40)),
        exit_status: Some(1),
        frames_on_the_server: 12,
        ordinary_output: ACollectionOutcome::Collected,
        complaints: ACollectionOutcome::CollectedAndTheStreamWasEmpty,
        cause: TheCause::TheFilesOwn,
        disk_free_at_the_ending: None,
    };
    let c = messages.send_with_buttons(&the_failure_message(&failed), &a_job("job-c")).expect("job C's failure message and its one button");

    // 2. Job E: the first-frame check, as one picture with its caption and two buttons (item 7).
    let hold = TheSubject { owning_machine: me.machine.clone(), job: None, hold: Some("hold-e".into()), generation: "1".into() };
    let e = messages
        .send_the_waiting_message(
            &AWaitingJob {
                server: SERVER.into(),
                label: "Desk test job E".into(),
                hold: "hold-e".into(),
                generation: "1".into(),
                waits_for: WhatTheHoldWaitsFor::TheUsersAnswer,
                the_frame_shown: 1,
                frames_held: 39,
                the_frames: String::new(),
            },
            Some(&ADerivative { job: "job-e".into(), job_label: "Desk test job E".into(), frame: 1, file_name: "frame-0001.jpg".into(), bytes: frame_e, which_picture: None }),
            &hold,
        )
        .expect("job E's waiting picture and its two buttons")
        .expect("a hold awaiting the user is told");

    // 3. Job D: the status message, its stop attached by the clock (item 6).
    let stopped_d = Arc::new(AtomicBool::new(false));
    {
        let (store, queue, stop, stopped_d, subject) = (store.clone(), queue.clone(), run.side_threads_stop.clone(), stopped_d.clone(), a_job("job-d"));
        let supervisor = supervisor.clone();
        let tick_log = log.clone();
        run.side_threads.push(std::thread::spawn(move || {
            let mut clock = TheClock::for_the_conversation(conversation);
            let mut store: Arc<Mutex<Box<dyn TheTelegramWorkersState>>> = store;
            while !stop.load(Ordering::SeqCst) {
                if !stopped_d.load(Ordering::SeqCst) && supervisor.heard().stops.iter().any(|(job, _)| job == "job-d") {
                    stopped_d.store(true, Ordering::SeqCst);
                }
                let rendering = !stopped_d.load(Ordering::SeqCst);
                let status = the_status("Desk test job D", rendering);
                match clock.a_tick(&mut store, &queue, &status, rendering.then_some(&subject), SystemTime::now()) {
                    Ok(did) => tick_log.push(started, &format!("[clock] {did:?}")),
                    Err(why) => tick_log.push(started, &format!("[clock] the store failed: {why:?}")),
                }
                std::thread::sleep(the_interval_for(1));
            }
        }));
    }
    assert!(
        wait_for("the status message's first edit, which carries its stop, was offered", Duration::from_secs(60), || log
            .lines("[clock] OfferedAnEdit")
            .len()
            >= 2),
        "the status message never carried its stop"
    );

    // 4. Job F: a frame picture, its stop attached (item 6).
    let mut frames = ThePictures::new(conversation, SERVER);
    let f = frames.send_with_its_stop(
        &mut messages,
        ADerivative { job: "job-f".into(), job_label: "Desk test job F".into(), frame: 13, file_name: "frame-0013.jpg".into(), bytes: frame_f, which_picture: None },
        AClassOfSend::FramePicturesAndMedia,
        Some(&a_job("job-f")),
    );
    eprintln!("sent: job C as message {c}, job E's picture as message {e}, job F's picture: {f:?}");
    assert!(matches!(f, APictureSent::Sent { stop: Some(Ok(())), .. }), "job F's picture and its stop");
    eprintln!("\n>>> READY FOR TAPS <<<\n");

    let four = wait_for("job C's words, hold E's release, job D's stop and job F's stop", THE_CREATORS_TAPS, || {
        let heard = supervisor.heard();
        !log.lines("read the words of job-c").is_empty()
            && heard.releases.iter().any(|(h, _, _)| h == "hold-e")
            && heard.stops.iter().any(|(j, _)| j == "job-d")
            && heard.stops.iter().any(|(j, _)| j == "job-f")
    });
    std::thread::sleep(Duration::from_secs(20));
    eprintln!("the stand-in supervisor heard: {:#?}", supervisor.heard().commands);

    // ---- the silence check: the helper announces itself (B10, B14, B24)
    eprintln!("\n>>> SILENCE CHECK: the helper starts <<<\n");
    let helper_client = a_client(token, log, started, "helper", accepted);
    let helper_store: Arc<Mutex<Box<dyn TheTelegramWorkersState>>> = Arc::new(Mutex::new(Box::new(InMemory::with_routing_key(&routing_key))));
    let helper_queue = Arc::new(TheSendQueue::new(helper_client.clone(), Arc::new(OnThisComputersClock)));
    start_a_process(
        run,
        helper_client,
        conversation,
        WhoThisMachineIs { machine: Identity::issue(IdentityKind::Installation).unwrap().as_text(), display: HELPER.into() },
        helper_store,
        helper_queue,
        Box::new(ActsOnNothing),
        Box::new(NothingHeld),
        set_aside_through,
        log,
        started,
        "helper",
    );
    let heard_the_helper = wait_for("the holder took in the helper's machinery message", Duration::from_secs(300), || {
        !log.lines("[owner] took in machinery message").is_empty()
    });
    std::thread::sleep(Duration::from_secs(45));
    let closing = messages.send(smoozy_telegram::job_messages::a_user_message(SERVER, "", "", "The live pass is over. This message should make a sound."));
    eprintln!("the closing message: {closing:?}");
    std::thread::sleep(Duration::from_secs(30));

    assert!(four, "not every one of the four taps was acted on");
    assert!(heard_the_helper, "the helper's machinery message was never taken in");
}

#[test]
#[ignore = "uses the Creator's live bot token, asks for updates and sends messages; run by hand at the desk, alone, when the turn is yours, with SMOOZY_TELEGRAM_CONVERSATION and SMOOZY_LIVE_PASS_PICTURES set and -- --ignored --exact --nocapture"]
fn the_live_pass() {
    let Some(conversation) = std::env::var("SMOOZY_TELEGRAM_CONVERSATION").ok().and_then(|c| c.parse::<i64>().ok()) else {
        eprintln!("skipped: SMOOZY_TELEGRAM_CONVERSATION is not set");
        return;
    };
    let token = the_token();
    let started = Instant::now();
    let log = TheLog::default();
    let accepted = Arc::new(Mutex::new(TheAcceptedTexts::default()));
    let client = a_client(&token, &log, started, "desk", &accepted);

    let before = client.get_webhook_info().expect("getWebhookInfo was answered");
    let url_before = before.get("url").cloned();
    let filter_before = before.get("allowed_updates").cloned();
    eprintln!(
        "before: webhook address empty = {}, standing filter = {}, updates waiting = {}",
        url_before.as_ref().and_then(Value::as_str) == Some(""),
        filter_before.as_ref().map_or("not stated".into(), Value::to_string),
        before.get("pending_update_count").map_or("not stated".into(), Value::to_string),
    );
    assert_eq!(url_before.as_ref().and_then(Value::as_str), Some(""), "a webhook is registered; this watch removes nothing of the Creator's");
    let Some(filter_before) = filter_before.filter(Value::is_array) else {
        panic!("no standing filter was stated, so it could not be put back exactly; nothing was asked");
    };

    let mut run = TheRun::default();
    let outcome = catch_unwind(AssertUnwindSafe(|| the_watch(&token, conversation, &mut run, &log, &accepted, started)));

    // ---- afterwards, whatever happened: the helper first, then the owner
    for process in run.processes.iter_mut().rev() {
        process.stop();
    }
    run.side_threads_stop.store(true, Ordering::SeqCst);
    for side in run.side_threads.drain(..) {
        let _ = side.join();
    }
    let still_holding = run.processes.iter().filter(|p| p.holding.load(Ordering::SeqCst)).count();
    eprintln!("processes stopped: {}; last seen holding: {still_holding}", run.processes.len());
    std::thread::sleep(Duration::from_secs(35));
    let pinned: BTreeSet<i64> = log
        .lines("pinned board ")
        .iter()
        .chain(log.lines("pinned message ").iter())
        .filter_map(|l| l.rsplit(' ').next()?.parse().ok())
        .collect();
    for id in &pinned {
        let _ = client.unpin_chat_message(conversation, *id);
    }
    let top = client.get_chat(conversation).ok().map(|c| what_is_pinned(&c));
    let still_ours = top.as_ref().and_then(WhatIsPinned::the_identity).is_some_and(|id| pinned.contains(&id));
    eprintln!("unpinned by identity: {pinned:?}; one of ours still the pin = {still_ours}");
    eprintln!(
        "left in the conversation, nothing deleted but {} pins' lines: {} boards, {} machinery messages, {} sends accepted, {} photos accepted, {} pin calls accepted, {} tap answers taken",
        log.lines("call deleteMessage ok").len(),
        log.lines("sent board ").len(),
        log.lines("sent machinery message ").len(),
        log.lines("call sendMessage ok").len(),
        log.lines("call sendPhoto ok").len(),
        log.lines("call pinChatMessage ok").len(),
        log.lines("call answerCallbackQuery ok").len(),
    );
    eprintln!("updates confirmed: {:?}", log.lines("confirmed update "));
    let reads = log.lines("read the pin:");
    let count = |what: &str| reads.iter().filter(|l| l.contains(what)).count();
    eprintln!(
        "U7: {} reads of the pin; nothing pinned {}; the newest accepted text {}; an older accepted text {}; a text not yet answered as accepted {}; carrying entities {}",
        reads.len(),
        count("nothing pinned"),
        count("the newest accepted text"),
        count("an OLDER accepted text"),
        count("not yet answered as accepted"),
        reads.iter().filter(|l| l.contains("entities [")).count(),
    );

    let wire = OverHttps::reaching_telegram_with(&token);
    let mut fields = Map::new();
    fields.insert("timeout".into(), json!(0));
    fields.insert("allowed_updates".into(), filter_before.clone());
    let restored = wire.call("getUpdates", &WhatACallCarries::Fields(fields), Duration::from_secs(30));
    eprintln!("the filter was asked back: answered = {}", restored.as_deref().is_ok_and(|t| t.contains("\"ok\":true")));
    let after = client.get_webhook_info().expect("getWebhookInfo was answered again");
    eprintln!(
        "after: webhook address empty = {}, standing filter = {}, updates waiting = {}",
        after.get("url").and_then(Value::as_str) == Some(""),
        after.get("allowed_updates").map_or("not stated".into(), Value::to_string),
        after.get("pending_update_count").map_or("not stated".into(), Value::to_string),
    );
    assert_eq!(after.get("url").cloned(), url_before, "the webhook address moved");
    assert_eq!(after.get("allowed_updates").cloned(), Some(filter_before), "the standing filter was not put back");

    if let Err(failed) = outcome {
        resume_unwind(failed);
    }
}

//! **A failure's words read in the chat, and every tap answered — the live
//! check beside piece 7's, on the Creator's own bot.** `Telegram` E3d and R16,
//! on one desk process built on piece 2's listener and piece 3's routing.
//!
//! **Off by default, and it says so when skipped.** The hosted runners have no
//! token, and only one agent may ask for the bot's updates at a time. Run it by
//! hand, at the desk, alone, when the turn is yours, with the conversation's
//! identity taken from the proof's diary line (piece 6's second run):
//!
//! `SMOOZY_TELEGRAM_CONVERSATION=<identity> cargo test -p smoozy-telegram --test a_failures_words_in_the_chat -- --ignored --exact a_failures_words_in_the_chat --nocapture`
//!
//! **What it sends**, from seeded values, each failure on the file's own cause
//! so the only buttons are words buttons:
//!
//! 1. **Desk test job A**, whose output fits: one button, *Send Output log*. The
//!    tap brings the words whole in a fixed-width block,
//!    and no file.
//! 2. **Desk test job B**, with words on both streams: two buttons, *Send Output
//!    log* and *Send Errors log*. Its output does not fit, so that tap brings the end of it
//!    and then the whole output as a file; its errors fit and come whole.
//! 3. **Desk test job C**, whose output is over the 50 MB cap — a stand-in
//!    stream held in memory, never sent whole. The tap brings the end, and the
//!    label saying it is too large to send.
//!
//! **Every tap is answered at once with no text (R16)**, and each answer is
//! written down with how Telegram took it.
//!
//! **What it leaves, and what it puts back.** It reads the webhook state and
//! the standing filter first, and stops before asking anything where a webhook
//! is registered. Afterwards it stops the process, unpins by identity every
//! board it pinned, **asks once more with the filter exactly as found**, and
//! reads the webhook state again to prove it. **It deletes nothing but the lines its own pins write** (`Telegram` B18, deleted once processed), and prints
//! what it left.
//!
//! **The token** is read by this process from its file and goes nowhere else.

use std::collections::BTreeSet;
use std::panic::{catch_unwind, resume_unwind, AssertUnwindSafe};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use serde_json::{json, Map, Value};
use smoozy_state::identities::{Identity, IdentityKind};
use smoozy_telegram::actions::a_stand_in::AStandInSupervisor;
use smoozy_telegram::actions::{ReadsTheWordsBack, TheJobMessages, TheOwnersActs, TheSubject};
use smoozy_telegram::board::instructions::AnEntry;
use smoozy_telegram::board::text::{what_is_pinned, WhatIsPinned};
use smoozy_telegram::job_messages::{the_failure_message, ACollectionOutcome, AFailedJob, AStream, TheCause, THE_CAP_ON_A_FILE};
use smoozy_telegram::listener::{run_until_stopped, HandsOn, Notes, OnThisComputer, TheListener, TheSettings, WhatTheListenerIsHanded, WhoThisMachineIs};
use smoozy_telegram::pacing::{OnThisComputersClock, TheSendQueue};
use smoozy_telegram::routing::gather::{AJobAsHeld, ReadsWhatThisMachineHolds};
use smoozy_telegram::routing::{TheRouter, TheRoutersHands, WhatTheRouterIsHanded};
use smoozy_telegram::transport::{OverHttps, ReachesTelegram, TheClient, WhatACallCarries};
use smoozy_telegram::worker_state::{InMemory, TheTelegramWorkersState};

/// How long the Creator is given for all the taps before the watch gives up.
const THE_CREATORS_TAPS: Duration = Duration::from_secs(30 * 60);

const SERVER: &str = "Desk process";

/// **The one value in the key file shaped like a bot token**, read the way
/// `tests/the_real_service_answers.rs` reads it, and never printed.
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

/// Everything the process noted, with its time, never a token or a message's
/// content.
#[derive(Clone, Default)]
struct TheLog(Arc<Mutex<Vec<String>>>);

impl TheLog {
    fn notes(&self, started: Instant) -> Notes {
        let log = self.clone();
        Box::new(move |what| log.push(started, what))
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

/// **Every call written down with its time and how it ended**, never its
/// content — piece 3's lesson from its first run.
struct WritesDownEveryCall {
    inner: OverHttps,
    log: TheLog,
    started: Instant,
}

impl ReachesTelegram for WritesDownEveryCall {
    fn call(&self, method: &str, carries: &WhatACallCarries, bound: Duration) -> Result<String, String> {
        let answered = self.inner.call(method, carries, bound);
        let outcome = match &answered {
            Err(_) => "not reached".to_string(),
            Ok(text) => match smoozy_telegram::transport::what_the_answer_says(text) {
                Ok(_) => "ok".to_string(),
                Err(smoozy_telegram::transport::WhyTheCallFailed::Refused(r)) => {
                    format!("refused {:?} retry_after={:?} {:?}", r.error_code, r.retry_after, r.description)
                }
                Err(_) => "unreadable".to_string(),
            },
        };
        if method != "getChat" && method != "getUpdates" {
            self.log.push(self.started, &format!("call {method} {outcome}"));
        }
        answered
    }
}

/// **What this desk process owns**, live at generation 1.
struct DeskHolds;

impl ReadsWhatThisMachineHolds for DeskHolds {
    fn the_jobs(&self) -> Vec<AJobAsHeld> {
        Vec::new()
    }
    fn the_live_generation(&self, subject: &str) -> Option<String> {
        ["job-a", "job-b", "job-c"].contains(&subject).then(|| "1".to_string())
    }
}

/// **The stand-in's streams, with every read written down**: which job and
/// which stream the tap asked for, and how long it was.
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

/// **Updates waiting before the watch are set aside**: confirmed, and neither
/// answered nor routed.
struct SetAsideThenRoute {
    routing: TheRoutersHands,
    set_aside_through: i64,
    log: TheLog,
    started: Instant,
}

impl HandsOn for SetAsideThenRoute {
    fn an_update_from_the_conversation(&mut self, update: &Value) -> Result<(), String> {
        if update.get("update_id").and_then(Value::as_i64).is_some_and(|id| id <= self.set_aside_through) {
            self.log.push(self.started, "set aside an update from before the watch");
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

/// A wait with an end it always reaches: the condition, or the bound.
fn wait_for(what: &str, most: Duration, mut done: impl FnMut() -> bool) {
    let started = Instant::now();
    loop {
        if done() {
            eprintln!("{what}: after {} seconds", started.elapsed().as_secs());
            return;
        }
        assert!(started.elapsed() < most, "{what}: not seen within {} seconds", most.as_secs());
        std::thread::sleep(Duration::from_secs(2));
    }
}

fn a_failure(job: &str, label: &str, ordinary: ACollectionOutcome, complaints: ACollectionOutcome) -> AFailedJob {
    AFailedJob {
        server: SERVER.into(),
        job: job.into(),
        label: label.into(),
        generation: "1".into(),
        attempt: 1,
        at_frame: Some((13, 40)),
        exit_status: Some(1),
        frames_on_the_server: 12,
        ordinary_output: ordinary,
        complaints,
        // The file's own: no *Retry*, so the only buttons are words.
        cause: TheCause::TheFilesOwn,
        disk_free_at_the_ending: None,
    }
}

/// **Lines a render might print**, mixing Cyrillic, Japanese, emoji, a Windows
/// path and every character markup would need escaped.
fn a_render_line(n: usize) -> String {
    format!(
        "Fra:{n} Mem:812.4M | Time:00:{:02}.{:02} | Rendering {n} / 40 | C:\\Users\\Анна\\レンダー\\scene_{n}.blend 🎬 *_[]()~`>#+-=|{{}}.!\n",
        n % 60,
        n % 100
    )
}

struct TheRun {
    log: TheLog,
    supervisor: AStandInSupervisor,
    stop: Arc<AtomicBool>,
    worker: Option<std::thread::JoinHandle<()>>,
}

fn the_watch(client: &TheClient, conversation: i64, run: &mut TheRun, started: Instant) {
    let log = run.log.clone();
    let waiting = client.get_updates(None, 0).expect("the ask was answered");
    let ids: Vec<i64> = waiting.as_array().into_iter().flatten().filter_map(|u| u.get("update_id")?.as_i64()).collect();
    eprintln!("updates waiting before the watch began, to be set aside: {ids:?}");
    let set_aside_through = ids.iter().copied().max().unwrap_or(0);

    // ---- the seeded streams
    let fits: String = "Desk test: what a render printed, seeded by the words check.\n".to_string()
        + &(1..=6).map(a_render_line).collect::<String>()
        + "Blender quit\n";
    let too_long: String = (1..=400).map(a_render_line).collect::<String>() + "Blender quit\n";
    let errors = "Error: Cannot read file \"C:\\Users\\Анна\\テクスチャ\\wood_<1001>.png\": No such file or directory\n\
                  Error: Python: Traceback (most recent call last) in 'addon_[beta]*.py' 🙃\n";
    let mut over: Vec<u8> = Vec::with_capacity(THE_CAP_ON_A_FILE as usize + 200);
    let mut n = 0;
    while (over.len() as u64) <= THE_CAP_ON_A_FILE {
        n += 1;
        over.extend_from_slice(a_render_line(n % 1000).as_bytes());
    }
    over.extend_from_slice(b"Blender quit\n");
    assert!(fits.len() < 3000, "job A's output is meant to fit");
    assert!(too_long.encode_utf16().count() > 4096 && too_long.len() < 1_000_000, "job B's output is meant not to fit, and to be a small file");
    eprintln!(
        "seeded: job A output {} bytes; job B output {} bytes and errors {} bytes; job C output {} bytes (over the cap, held in memory only)",
        fits.len(),
        too_long.len(),
        errors.len(),
        over.len()
    );
    {
        let mut heard = run.supervisor.heard();
        heard.words.insert(("job-a".into(), AStream::TheOrdinaryOutput), fits.into_bytes());
        heard.words.insert(("job-b".into(), AStream::TheOrdinaryOutput), too_long.into_bytes());
        heard.words.insert(("job-b".into(), AStream::TheComplaints), errors.as_bytes().to_vec());
        heard.words.insert(("job-c".into(), AStream::TheOrdinaryOutput), over);
    }

    let me = WhoThisMachineIs { machine: Identity::issue(IdentityKind::Installation).unwrap().as_text(), display: SERVER.into() };
    let routing_key = Identity::issue(IdentityKind::Operation).unwrap().as_text();
    let store: Arc<Mutex<Box<dyn TheTelegramWorkersState>>> = Arc::new(Mutex::new(Box::new(InMemory::with_routing_key(&routing_key))));
    let queue = Arc::new(TheSendQueue::new(client.clone(), Arc::new(OnThisComputersClock)));
    let clock = Arc::new(OnThisComputer::new());
    let router = TheRouter::new(WhatTheRouterIsHanded {
        conversation,
        me: me.clone(),
        store: Box::new(store.clone()),
        holds: Box::new(DeskHolds),
        queue: queue.clone(),
        clock: clock.clone(),
        notes: log.notes(started),
    });
    let owner = TheOwnersActs {
        conversation,
        supervisor: Box::new(run.supervisor.clone()),
        words: Box::new(WritesDownEveryRead { inner: run.supervisor.clone(), log: log.clone(), started }),
        holds: Box::new(DeskHolds),
        queue: queue.clone(),
        notes: log.notes(started),
    };
    let listener = Arc::new(Mutex::new(TheListener::new(WhatTheListenerIsHanded {
        client: client.clone(),
        queue: queue.clone(),
        clock,
        me: me.clone(),
        settings: TheSettings::for_the_conversation(conversation),
        store: Box::new(store.clone()),
        actor: Box::new(owner),
        hands_on: Box::new(SetAsideThenRoute { routing: router.hands(), set_aside_through, log: log.clone(), started }),
        notes: log.notes(started),
    })));
    let holding = Arc::new(AtomicBool::new(false));
    run.worker = Some({
        let (listener, queue, stop, holding) = (listener.clone(), queue.clone(), run.stop.clone(), holding.clone());
        std::thread::spawn(move || {
            run_until_stopped(listener, queue, stop, move |held| {
                router.before_each_turn(held);
                holding.store(held.is_holding(), Ordering::SeqCst);
            })
        })
    });

    wait_for("the desk process holds the slot", Duration::from_secs(300), || holding.load(Ordering::SeqCst));

    let mut messages = TheJobMessages { conversation, store: Box::new(store.clone()), queue: queue.clone(), notes: log.notes(started) };
    let subject = |job: &str| TheSubject { owning_machine: me.machine.clone(), job: Some(job.into()), hold: None, generation: "1".into() };
    use ACollectionOutcome::*;
    let a = messages
        .send_with_buttons(&the_failure_message(&a_failure("job-a", "Desk test job A", Collected, CollectedAndTheStreamWasEmpty)), &subject("job-a"))
        .expect("job A's failure message and its one button");
    let b = messages
        .send_with_buttons(&the_failure_message(&a_failure("job-b", "Desk test job B", Collected, Collected)), &subject("job-b"))
        .expect("job B's failure message and its two buttons");
    let c = messages
        .send_with_buttons(&the_failure_message(&a_failure("job-c", "Desk test job C", Collected, CollectedAndTheStreamWasEmpty)), &subject("job-c"))
        .expect("job C's failure message and its one button");
    eprintln!("sent: job A as message {a}, job B as message {b}, job C as message {c}");
    eprintln!("\n>>> READY FOR TAPS <<<\n");

    wait_for("four words taps acted on", THE_CREATORS_TAPS, || log.lines("sent the words for").len() >= 4);
    std::thread::sleep(Duration::from_secs(30));

    let reads = log.lines("read the words of");
    let words = log.lines("sent the words for");
    // C6b: the file is offered to the upload lane and not waited for, so
    // whether it was taken is the call's own line.
    let files = log.lines("offered the whole words for");
    let answers = log.lines("call answerCallbackQuery");
    eprintln!("reads: {reads:#?}\nwords messages: {words:#?}\nfiles: {files:#?}\ntap answers: {answers:#?}");
    assert_eq!(words.len(), 4, "one words message per tap");
    assert!(words.iter().all(|l| l.contains("accepted = true")), "every words message was accepted");
    let order: Vec<String> = reads.iter().filter_map(|l| l.split("read the words of ").nth(1)?.split(" (").next().map(str::to_string)).collect();
    assert_eq!(order, ["job-a Output", "job-b Output", "job-b Errors", "job-c Output"], "the Creator's order of taps");
    let to_follow: Vec<bool> = words.iter().map(|l| l.contains("a file to follow = true")).collect();
    assert_eq!(to_follow, [false, true, false, false], "a file follows only the output that does not fit");
    assert_eq!(files.len(), 1, "one file");
    assert_eq!(log.lines("call sendDocument ok").len(), 1, "the file was accepted");
    assert!(answers.len() >= 4 && answers.iter().all(|l| l.contains(" ok")), "every tap was answered, and every answer was taken");
}

#[test]
#[ignore = "uses the Creator's live bot token, asks for updates and sends messages; run by hand at the desk, alone, when the turn is yours, with SMOOZY_TELEGRAM_CONVERSATION set and -- --ignored --exact --nocapture"]
fn a_failures_words_in_the_chat() {
    let Some(conversation) = std::env::var("SMOOZY_TELEGRAM_CONVERSATION").ok().and_then(|c| c.parse::<i64>().ok()) else {
        eprintln!("skipped: SMOOZY_TELEGRAM_CONVERSATION is not set");
        return;
    };
    let token = the_token();
    let started = Instant::now();
    let log = TheLog::default();
    let client = TheClient::over(Arc::new(WritesDownEveryCall {
        inner: OverHttps::reaching_telegram_with(&token),
        log: log.clone(),
        started,
    }));

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

    let mut run = TheRun { log: log.clone(), supervisor: AStandInSupervisor::default(), stop: Arc::new(AtomicBool::new(false)), worker: None };
    let outcome = catch_unwind(AssertUnwindSafe(|| the_watch(&client, conversation, &mut run, started)));

    // ---- afterwards, whatever happened
    run.stop.store(true, Ordering::SeqCst);
    if let Some(worker) = run.worker.take() {
        let _ = worker.join();
    }
    // The holder's own ask may still be open; it ends within its wait and a
    // margin. The filter is asked back only after it has.
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
    eprintln!("unpinned by identity: {:?}; one of ours still the pin = {still_ours}", pinned);
    eprintln!(
        "left in the conversation, nothing deleted but {} pins' lines: {} boards, {} job messages, {} words messages, {} files, {} pin calls accepted, {} tap answers taken",
        log.lines("call deleteMessage ok").len(),
        log.lines("sent board ").len(),
        log.lines("sent a job message").len(),
        log.lines("sent the words for").len(),
        log.lines("call sendDocument ok").len(),
        log.lines("call pinChatMessage ok").len(),
        log.lines("call answerCallbackQuery ok").len(),
    );
    eprintln!("updates confirmed: {:?}", log.lines("confirmed update "));

    // **The filter put back exactly as found**, by one ask outside `src/` that
    // names the kinds found and no offset, so it confirms nothing.
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

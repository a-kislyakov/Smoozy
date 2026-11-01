//! **Step 10 piece 7's live check: a job's messages and the phone actions, on
//! the Creator's own bot.** `Telegram` §13, §14, §17 and §18, on one desk
//! process built on piece 2's listener and piece 3's routing.
//!
//! **Off by default, and it says so when skipped.** The hosted runners have no
//! token, and only one agent may ask for the bot's updates at a time. Run it by
//! hand, at the desk, alone, when the turn is yours:
//!
//! `cargo test -p smoozy-telegram --test a_jobs_messages_on_the_phone a_jobs_messages_on_the_phone -- --ignored --exact --nocapture`
//!
//! **What it watches.** The Creator sends any message, which finds the
//! conversation. One desk process takes the slot and sends, from seeded values:
//!
//! 1. **A failure on the machine's own cause** — *Send Output log* and *Try
//!    it again* (E1 to E3, Y4a).
//! 2. **A failure on the file's own cause** — two words buttons and no *Try it
//!    again*, and the sentence that the job needs a change.
//! 3. **A failure whose cause is undetermined**, with nothing recorded to send —
//!    no button at all, and the sentence that the cause is unknown
//!    (Y4a, corrected 2026-09-16).
//! 4. **A group of three finishing**, composed after each member's ending and
//!    sent once; and a group whose members all failed, which sends nothing
//!    (G1, G2).
//! 5. **A waiting job** with its two buttons (A2).
//!
//! **Then the Creator taps**, in order: *Send Output log* on the first — its
//! words arrive whole in the chat as a fixed-width block, with no file, since
//! they fit (E3d); *Retry* on the first — one
//! submission to the stand-in supervisor; *Render all* and then *Cancel
//! it* on the waiting job — two release commands, the second told which
//! answer was applied (A3).
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
use std::time::{Duration, Instant, SystemTime};

use serde_json::{json, Map, Value};
use smoozy_state::identities::{Identity, IdentityKind};
use smoozy_telegram::actions::a_stand_in::AStandInSupervisor;
use smoozy_telegram::actions::{TheJobMessages, TheListenersAcknowledgement, TheOwnersActs, TheSubject};
use smoozy_telegram::board::instructions::AnEntry;
use smoozy_telegram::board::text::{what_is_pinned, WhatIsPinned};
use smoozy_telegram::job_messages::{
    the_failure_message, the_waiting_message, what_the_group_says, ACollectionOutcome, AFailedJob, AMember, AMembersEnding,
    AStream, AWaitingJob, TheCause, WhatTheHoldWaitsFor,
};
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
                    format!("refused {:?} retry_after={:?}", r.error_code, r.retry_after)
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
        ["job-a", "job-b", "job-c", "hold-d"].contains(&subject).then(|| "1".to_string())
    }
}

/// **Updates waiting before the watch, and the Creator's first message, are
/// set aside**: confirmed, and neither answered nor routed.
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

/// **The conversation, found from the Creator's first private message**, asked
/// for without an offset so nothing is confirmed.
fn the_conversation(client: &TheClient) -> (i64, i64) {
    let waiting = client.get_updates(None, 0).expect("the ask was answered");
    let ids: Vec<i64> = waiting.as_array().into_iter().flatten().filter_map(|u| u.get("update_id")?.as_i64()).collect();
    eprintln!("updates waiting before the watch began, to be set aside: {ids:?}");
    eprintln!("\n>>> LISTENING: waiting for the Creator's first message. <<<\n");
    let until = Instant::now() + THE_CREATORS_TAPS;
    loop {
        let updates = client.get_updates(None, 20).expect("the ask was answered");
        let found = updates.as_array().into_iter().flatten().filter(|u| !ids.contains(&u["update_id"].as_i64().unwrap_or(0))).find_map(|u| {
            let chat = u.pointer("/message/chat")?;
            let id = (chat.get("type")?.as_str()? == "private").then(|| chat.get("id")?.as_i64()).flatten()?;
            Some((id, u.get("update_id")?.as_i64()?))
        });
        if let Some((chat, update)) = found {
            eprintln!("the conversation was found from update {update}, left unconfirmed for the listener to set aside");
            return (chat, update);
        }
        assert!(Instant::now() < until, "no message from the Creator arrived");
    }
}

fn a_failure(job: &str, label: &str, cause: TheCause, ordinary: ACollectionOutcome, complaints: ACollectionOutcome) -> AFailedJob {
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
        cause,
        disk_free_at_the_ending: None,
    }
}

struct TheRun {
    conversation: i64,
    log: TheLog,
    supervisor: AStandInSupervisor,
    stop: Arc<AtomicBool>,
    worker: Option<std::thread::JoinHandle<()>>,
}

fn the_watch(client: &TheClient, run: &mut TheRun, started: Instant) {
    let (conversation, set_aside_through) = the_conversation(client);
    run.conversation = conversation;
    let log = run.log.clone();

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
    let words = "Desk test: the lines a render printed, seeded by step 10 piece 7's live check.\n\
                 Fra:13 | Rendering 13 / 40\nBlender quit\n";
    run.supervisor.heard().words.insert(("job-a".into(), AStream::TheOrdinaryOutput), words.as_bytes().to_vec());
    let owner = TheOwnersActs {
        conversation,
        supervisor: Box::new(run.supervisor.clone()),
        words: Box::new(run.supervisor.clone()),
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
    let mut acknowledgement = TheListenersAcknowledgement { conversation, queue: queue.clone(), said: Default::default() };
    run.worker = Some({
        let (listener, queue, stop, holding) = (listener.clone(), queue.clone(), run.stop.clone(), holding.clone());
        std::thread::spawn(move || {
            run_until_stopped(listener, queue, stop, move |held| {
                router.before_each_turn(held);
                acknowledgement.before_each_turn(held);
                holding.store(held.is_holding(), Ordering::SeqCst);
            })
        })
    });

    wait_for("the desk process holds the slot", Duration::from_secs(300), || holding.load(Ordering::SeqCst));

    let mut messages = TheJobMessages { conversation, store: Box::new(store.clone()), queue: queue.clone(), notes: log.notes(started) };
    let subject = |job: &str| TheSubject { owning_machine: me.machine.clone(), job: Some(job.into()), hold: None, generation: "1".into() };
    use ACollectionOutcome::*;
    let a = messages
        .send_with_buttons(&the_failure_message(&a_failure("job-a", "Desk test job A", TheCause::TheMachinesOwn, Collected, CollectedAndTheStreamWasEmpty)), &subject("job-a"))
        .expect("the first failure message and its two buttons");
    messages
        .send_with_buttons(&the_failure_message(&a_failure("job-b", "Desk test job B", TheCause::TheFilesOwn, Collected, Collected)), &subject("job-b"))
        .expect("the second failure message and its two buttons");
    messages
        .send_with_buttons(&the_failure_message(&a_failure("job-c", "Desk test job C", TheCause::Undetermined, NeverEstablished, CollectedAndTheStreamWasEmpty)), &subject("job-c"))
        .expect("the third failure message, with no buttons");

    // G1: composed at each member's ending; G2: said once, and only where one finished.
    let now = SystemTime::now();
    let mut members: Vec<AMember> = (0..3)
        .map(|n| AMember { job: format!("member-{n}"), ending: None, started: Some(now - Duration::from_secs(5400 - n * 600)), ended: None })
        .collect();
    let mut group_messages = 0;
    for n in 0..3 {
        members[n].ending = Some(AMembersEnding::Finished { frames: 40 });
        members[n].ended = Some(now - Duration::from_secs(1800 - n as u64 * 900));
        if let Some(text) = what_the_group_says(SERVER, "Desk test group", &members) {
            messages.send(text);
            group_messages += 1;
        }
    }
    let failed: Vec<AMember> = (0..2).map(|n| AMember { job: format!("failed-{n}"), ending: Some(AMembersEnding::Failed), started: Some(now), ended: Some(now) }).collect();
    assert!(what_the_group_says(SERVER, "Desk test group that failed", &failed).is_none());
    assert_eq!(group_messages, 1, "a group speaks once");

    let waiting = AWaitingJob {
        server: SERVER.into(),
        label: "Desk test job D".into(),
        hold: "hold-d".into(),
        generation: "1".into(),
        waits_for: WhatTheHoldWaitsFor::TheUsersAnswer,
        the_frame_shown: 1,
        frames_held: 39,
        the_frames: String::new(),
    };
    let d = messages
        .send_with_buttons(&the_waiting_message(&waiting).expect("a hold awaiting the user is told"), &TheSubject { owning_machine: me.machine.clone(), job: None, hold: Some("hold-d".into()), generation: "1".into() })
        .expect("the waiting job's message and its two buttons");
    eprintln!("sent: the first failure as message {a}, the waiting job as message {d}");
    eprintln!("\n>>> READY FOR TAPS <<<\n");

    let supervisor = run.supervisor.clone();
    wait_for("the words sent in the chat, a submission, and two release commands", THE_CREATORS_TAPS, || {
        let heard = supervisor.heard();
        !log.lines("sent the words for").is_empty()
            && heard.commands.iter().any(|c| c.starts_with("submit job-a again as"))
            && heard.commands.iter().filter(|c| c.starts_with("release hold-d")).count() >= 2
    });
    std::thread::sleep(Duration::from_secs(20));
    let heard = supervisor.heard();
    eprintln!("the stand-in supervisor heard: {:#?}", heard.commands);
    assert_eq!(log.lines("sent the words for").len(), 1, "one words message");
    assert!(log.lines("sent the words for")[0].contains("accepted = true"), "the words message was accepted");
    assert!(log.lines("sent the words for")[0].contains("a file to follow = false"), "the seeded words fit, so no file");
    assert_eq!(heard.submitted.len(), 1, "one submission");
    let releases: Vec<&String> = heard.commands.iter().filter(|c| c.starts_with("release hold-d")).collect();
    assert_eq!(releases.len(), 2, "one release command per button");
    assert!(releases[0].contains("GoOn") && releases[1].contains("Dont"), "the Creator's order: render the rest, then cancel");
    assert!(!heard.commands.iter().any(|c| c.contains("job-b") || c.contains("job-c")), "nothing for the other two failures");
}

#[test]
#[ignore = "uses the Creator's live bot token, asks for updates and sends messages; run by hand at the desk, alone, when the turn is yours, with -- --ignored --exact --nocapture"]
fn a_jobs_messages_on_the_phone() {
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

    let mut run = TheRun { conversation: 0, log: log.clone(), supervisor: AStandInSupervisor::default(), stop: Arc::new(AtomicBool::new(false)), worker: None };
    let outcome = catch_unwind(AssertUnwindSafe(|| the_watch(&client, &mut run, started)));

    // ---- afterwards, whatever happened
    run.stop.store(true, Ordering::SeqCst);
    if let Some(worker) = run.worker.take() {
        let _ = worker.join();
    }
    // The holder's own ask may still be open; it ends within its wait and a
    // margin, and nothing it brings is confirmed under a win that is over. The
    // filter is asked back only after it has.
    std::thread::sleep(Duration::from_secs(35));
    let pinned: BTreeSet<i64> = log
        .lines("pinned board ")
        .iter()
        .chain(log.lines("pinned message ").iter())
        .filter_map(|l| l.rsplit(' ').next()?.parse().ok())
        .collect();
    if run.conversation != 0 {
        for id in &pinned {
            let _ = client.unpin_chat_message(run.conversation, *id);
        }
        let top = client.get_chat(run.conversation).ok().map(|c| what_is_pinned(&c));
        let still_ours = top.as_ref().and_then(WhatIsPinned::the_identity).is_some_and(|id| pinned.contains(&id));
        eprintln!("unpinned by identity: {} messages; one of ours still the pin = {still_ours}", pinned.len());
    }
    eprintln!(
        "left in the conversation, nothing deleted but {} pins' lines: {} boards, {} job messages, {} answers to taps, {} files, {} pin calls accepted",
        log.lines("call deleteMessage ok").len(),
        log.lines("sent board ").len(),
        log.lines("sent a job message").len(),
        log.lines("answered the user").len(),
        log.lines("call sendDocument ok").len(),
        log.lines("call pinChatMessage ok").len(),
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

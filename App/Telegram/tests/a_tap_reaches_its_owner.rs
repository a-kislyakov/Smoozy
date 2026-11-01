//! **The rest of the milestone's first half, watched on the Creator's own bot:
//! the bot answers from the user's own conversation.** `Telegram` §5, R1 to
//! R15 with R1a, on desk processes built on the listener.
//!
//! **Off by default, and it says so when skipped.** The hosted runners have no
//! token, and only one agent may ask for the bot's updates at a time. Run it by
//! hand, at the desk, alone, when the turn is yours:
//!
//! `cargo test -p smoozy-telegram --test a_tap_reaches_its_owner a_tap_reaches_its_owner -- --ignored --exact --nocapture`
//!
//! **What it watches, in the order the Creator meets it.**
//!
//! 1. **The Creator types *status*.** Four desk processes start; one
//!    holds the slot; one that is not holding is killed; and the three left
//!    answer as one message naming the killed one as not answering (R10).
//! 2. **The Creator types *frame 1* and taps one of the two jobs offered**
//!    under *Which job?* (R13). The button is the holder's; the tap resolves
//!    across the board to the process standing as that job's owner, which acts
//!    on it once and acknowledges (R1, R2, R14, B9).
//! 3. **The holder that issued those buttons is killed, and the Creator taps
//!    the other job.** Nobody holds that button's record any more, so the
//!    process that takes the slot answers that nothing was done (R1, R3, B15).
//!
//! An update from any other conversation being ignored (P2) is watched against
//! the stand-in, in `src/routing/tests.rs`, since nobody else may be asked to
//! message the bot.
//!
//! **What already waits is set aside.** Any update waiting before the watch
//! begins — the first run left the Creator's *send frame 1* — is noted, handed
//! to the processes, confirmed by them and neither answered nor routed, so it
//! is never read as one of this run's steps. The conversation may be found from
//! it.
//!
//! **What it leaves, and what it puts back.** It reads the webhook state and
//! the standing filter first and stops before asking anything where a webhook
//! is registered. Afterwards it stops every process it started and checks each
//! is gone, unpins by identity everything the processes pinned, **asks once more
//! with the filter exactly as found**, and reads the webhook state again to
//! prove it. **It deletes nothing but the lines its own pins write** (`Telegram` B18, deleted once processed), and prints what it left.
//!
//! **The token** is read by each process from its file and goes nowhere else.
//! A child is told only the folder it works in.

use std::collections::BTreeSet;
use std::io::Write as _;
use std::panic::{catch_unwind, resume_unwind, AssertUnwindSafe};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use serde_json::{json, Map, Value};
use smoozy_state::identities::{Identity, IdentityKind};
use smoozy_telegram::board::instructions::AnEntry;
use smoozy_telegram::board::text::{what_is_pinned, TheBoard, WhatIsPinned};
use smoozy_telegram::listener::{
    run_until_stopped, ActsOnAnInstruction, HandsOn, OnThisComputer, TheListener, TheSettings, WhatTheListenerIsHanded,
    WhoThisMachineIs,
};
use smoozy_telegram::pacing::{OnThisComputersClock, TheSendQueue};
use smoozy_telegram::routing::context::the_owner_finds_the_subject_live;
use smoozy_telegram::routing::gather::{AJobAsHeld, ReadsWhatThisMachineHolds, TheProgress};
use smoozy_telegram::routing::{TheRouter, TheRoutersHands, WhatTheRouterIsHanded};
use smoozy_telegram::transport::{OverHttps, ReachesTelegram, TheClient, WhatACallCarries};
use smoozy_telegram::worker_state::{AFileStandIn, TheTelegramWorkersState};

/// The one thing a child is told: the folder it works in.
const THE_MEMBERS_FOLDER: &str = "SMOOZY_ROUTING_MEMBER_FOLDER";

/// How long the Creator is given for each step before the watch gives up.
const A_STEP_OF_THE_CREATORS: Duration = Duration::from_secs(20 * 60);

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

fn append(path: &Path, line: &str) {
    let mut file = std::fs::OpenOptions::new().create(true).append(true).open(path).expect("a member's file opens");
    writeln!(file, "{line}").expect("a member's file is written");
    file.sync_all().expect("a member's file is synced");
}

/// **Every call a desk process makes, written down with its time and how it
/// ended**, never its content. Added after the first run of 2026-09-13, whose
/// processes' logs went with their folder and whose cause could not be read.
struct WritesDownEveryCall {
    inner: OverHttps,
    log: PathBuf,
    started: Instant,
}

impl ReachesTelegram for WritesDownEveryCall {
    fn call(&self, method: &str, carries: &WhatACallCarries, bound: Duration) -> Result<String, String> {
        let at = self.started.elapsed().as_secs_f64();
        let answered = self.inner.call(method, carries, bound);
        let took = self.started.elapsed().as_secs_f64() - at;
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
        append(&self.log, &format!("{at:9.3} {method} {outcome} took={took:.3}"));
        answered
    }
}

fn timed(path: &Path, started: Instant, line: &str) {
    append(path, &format!("{:9.3} {line}", started.elapsed().as_secs_f64()));
}

fn lines_of(path: &Path) -> Vec<String> {
    std::fs::read_to_string(path).map(|t| t.lines().map(str::to_string).collect()).unwrap_or_default()
}

// ------------------------------------------------------------ one member

/// **What a desk process holds**: the jobs the watch seeds into its folder,
/// read at every call, as a machine would read its note and line.
struct JobsInItsFolder {
    folder: PathBuf,
}

impl ReadsWhatThisMachineHolds for JobsInItsFolder {
    fn the_jobs(&self) -> Vec<AJobAsHeld> {
        std::fs::read_to_string(self.folder.join("jobs.json"))
            .ok()
            .and_then(|t| serde_json::from_str(&t).ok())
            .unwrap_or_default()
    }
}

/// **`Telegram` B9 at a desk**: the identity recorded before acting, and the
/// act — the owner's generation check, then a line saying what was asked.
struct ActsInItsFolder {
    folder: PathBuf,
}

impl ActsOnAnInstruction for ActsInItsFolder {
    /// B9: a repeat finds the outcome recorded beside the identity and acts on
    /// nothing; the act is written, then its outcome.
    fn act_once(&mut self, entry: &AnEntry) -> Result<(), String> {
        let recorded = self.folder.join("recorded.txt");
        if lines_of(&recorded).contains(&entry.operation) {
            return Ok(());
        }
        let holds = JobsInItsFolder { folder: self.folder.clone() };
        let live = the_owner_finds_the_subject_live(entry, &|s| holds.the_live_generation(s));
        append(
            &self.folder.join("acted.txt"),
            &format!("{} {} frame={} live={live}", entry.operation, entry.action, entry.payload["frame"]),
        );
        append(&recorded, &entry.operation);
        Ok(())
    }
}

/// **The watch's own gate**: nothing the Creator sends is handled until the
/// watch has killed its first process and seeded the jobs. An update not
/// handled stays unconfirmed and comes back (F3).
struct HandsOnOnceReady {
    folder: PathBuf,
    routing: TheRoutersHands,
    /// **Updates already waiting when the watch began are set aside**: noted,
    /// confirmed, and neither answered nor routed, so a message left over from
    /// an earlier run is never read as one of this run's steps.
    set_aside_through: Option<i64>,
}

impl HandsOn for HandsOnOnceReady {
    fn an_update_from_the_conversation(&mut self, update: &Value) -> Result<(), String> {
        let id = update.get("update_id").and_then(Value::as_i64);
        if let (Some(id), Some(through)) = (id, self.set_aside_through) {
            if id <= through {
                append(&self.folder.join("events.log"), &format!("set aside update {id}: it was waiting before the watch began"));
                return Ok(());
            }
        }
        if !self.folder.join("ready").exists() {
            return Err("the desk watch is not ready".into());
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

/// **One desk process**, started only by [`a_tap_reaches_its_owner`].
#[test]
#[ignore = "one process of the live routing watch; started by a_tap_reaches_its_owner and never on its own"]
fn a_routing_member() {
    let Some(folder) = std::env::var_os(THE_MEMBERS_FOLDER).map(PathBuf::from) else {
        eprintln!("skipped: a routing member runs only when a_tap_reaches_its_owner starts it");
        return;
    };
    let settings: Value = serde_json::from_str(&std::fs::read_to_string(folder.join("settings.json")).unwrap()).unwrap();
    let events = folder.join("events.log");
    let started = Instant::now();
    let client = TheClient::over(Arc::new(WritesDownEveryCall {
        inner: OverHttps::reaching_telegram_with(&the_token()),
        log: folder.join("calls.log"),
        started,
    }));
    let timeline = folder.join("timeline.log");
    let queue = Arc::new(TheSendQueue::new(client.clone(), Arc::new(OnThisComputersClock)));
    let conversation = settings["conversation"].as_i64().unwrap();
    let me = WhoThisMachineIs {
        machine: settings["machine"].as_str().unwrap().to_string(),
        display: settings["display"].as_str().unwrap().into(),
    };
    let file = AFileStandIn::at(folder.join("state.json"), settings["routing_key"].as_str().unwrap()).unwrap();
    let store: Arc<Mutex<Box<dyn TheTelegramWorkersState>>> = Arc::new(Mutex::new(Box::new(file)));
    let clock = Arc::new(OnThisComputer::new());
    let router_notes = events.clone();
    let router_timeline = timeline.clone();
    let router = TheRouter::new(WhatTheRouterIsHanded {
        conversation,
        me: me.clone(),
        store: Box::new(store.clone()),
        holds: Box::new(JobsInItsFolder { folder: folder.clone() }),
        queue: queue.clone(),
        clock: clock.clone(),
        notes: Box::new(move |what| {
            append(&router_notes, what);
            timed(&router_timeline, started, what);
        }),
    });
    let listener_notes = events.clone();
    let listener_timeline = timeline.clone();
    let listener = Arc::new(Mutex::new(TheListener::new(WhatTheListenerIsHanded {
        client,
        queue: queue.clone(),
        clock,
        me,
        settings: TheSettings::for_the_conversation(conversation),
        store: Box::new(store),
        actor: Box::new(ActsInItsFolder { folder: folder.clone() }),
        hands_on: Box::new(HandsOnOnceReady {
            folder: folder.clone(),
            routing: router.hands(),
            set_aside_through: settings["set_aside_through"].as_i64(),
        }),
        notes: Box::new(move |what| {
            append(&listener_notes, what);
            timed(&listener_timeline, started, what);
        }),
    })));
    append(&events, "started");
    let stop = Arc::new(AtomicBool::new(false));
    let stopping = stop.clone();
    let at_the_end = events.clone();
    run_until_stopped(listener, queue, stop, move |held| {
        if folder.join("stop").exists() {
            stopping.store(true, Ordering::SeqCst);
        }
        router.before_each_turn(held);
        let standing = if held.is_holding() { "holding" } else { "not holding" };
        let _ = std::fs::write(folder.join("standing.txt"), standing);
    });
    append(&at_the_end, "stopped");
}

// ------------------------------------------------------------ the watch

struct AMemberProcess {
    name: String,
    machine: String,
    folder: PathBuf,
    child: Option<Child>,
}

struct TheFleet {
    root: PathBuf,
    conversation: i64,
    /// The newest update waiting before the watch began, set aside.
    set_aside: Option<i64>,
    members: Vec<AMemberProcess>,
}

impl TheFleet {
    fn prepare(&mut self, name: &str) {
        let folder = self.root.join(name);
        std::fs::create_dir_all(&folder).unwrap();
        let machine = Identity::issue(IdentityKind::Installation).unwrap().as_text();
        // The routing key's issuer is the supervisor (`State, identity and
        // recovery` §2.2); at this desk it is drawn the same way, 16 random bytes.
        let routing_key = Identity::issue(IdentityKind::Operation).unwrap().as_text();
        let settings = json!({
            "conversation": self.conversation,
            "machine": machine,
            "display": format!("Desk process {}", name.to_uppercase()),
            "routing_key": routing_key,
            "set_aside_through": self.set_aside,
        });
        std::fs::write(folder.join("settings.json"), settings.to_string()).unwrap();
        std::fs::write(folder.join("jobs.json"), "[]").unwrap();
        self.members.push(AMemberProcess { name: name.into(), machine, folder, child: None });
    }

    fn member(&mut self, name: &str) -> &mut AMemberProcess {
        self.members.iter_mut().find(|m| m.name == name).unwrap()
    }

    fn start(&mut self, name: &str) {
        let member = self.member(name);
        let out = std::fs::File::create(member.folder.join("out.txt")).unwrap();
        let child = Command::new(std::env::current_exe().unwrap())
            .args(["a_routing_member", "--exact", "--ignored", "--nocapture", "--test-threads=1"])
            .env(THE_MEMBERS_FOLDER, &member.folder)
            .stdout(Stdio::from(out.try_clone().unwrap()))
            .stderr(Stdio::from(out))
            .spawn()
            .expect("a member process starts");
        member.child = Some(child);
        eprintln!("started {name}");
    }

    /// **Killed, not asked**: the process gets no chance to say goodbye.
    fn kill(&mut self, name: &str) {
        let member = self.member(name);
        let mut child = member.child.take().expect("the member was running");
        let _ = child.kill();
        let status = child.wait().expect("the killed member was reaped");
        eprintln!("killed {name} ({status})");
    }

    fn the_name_of(&self, machine: &str) -> String {
        self.members.iter().find(|m| m.machine == machine).map(|m| m.name.clone()).unwrap_or_default()
    }

    fn stop_every_member(&mut self) {
        for member in &self.members {
            let _ = std::fs::write(member.folder.join("stop"), "");
        }
        let until = Instant::now() + Duration::from_secs(40);
        for member in &mut self.members {
            let Some(mut child) = member.child.take() else { continue };
            loop {
                match child.try_wait() {
                    Ok(Some(_)) => break,
                    _ if Instant::now() > until => {
                        let _ = child.kill();
                        let _ = child.wait();
                        break;
                    }
                    _ => std::thread::sleep(Duration::from_millis(200)),
                }
            }
            assert!(matches!(child.try_wait(), Ok(Some(_))), "{} is gone", member.name);
            eprintln!("{} stopped and gone", member.name);
        }
    }

    fn lines(&self, starting: &str) -> Vec<String> {
        self.members
            .iter()
            .flat_map(|m| lines_of(&m.folder.join("events.log")))
            .filter(|l| l.starts_with(starting))
            .collect()
    }

    fn acts(&self) -> Vec<(String, String)> {
        self.members
            .iter()
            .flat_map(|m| lines_of(&m.folder.join("acted.txt")).into_iter().map(|l| (m.name.clone(), l)))
            .collect()
    }

    fn every_pinned_identity(&self) -> BTreeSet<i64> {
        self.lines("pinned message ").iter().filter_map(|l| l.rsplit(' ').next()?.parse().ok()).collect()
    }
}

fn the_board_read(client: &TheClient, conversation: i64) -> Option<TheBoard> {
    match what_is_pinned(&client.get_chat(conversation).ok()?) {
        WhatIsPinned::ABoard { board, .. } => Some(board),
        _ => None,
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

/// **The conversation, found without writing it down**: the first private
/// message the Creator sends, asked for without an offset so nothing is
/// confirmed. That message stays waiting for the processes.
fn the_conversation(client: &TheClient) -> (i64, Option<i64>) {
    // **What already waits is set aside, never read as a step**: noted here and
    // by each process, confirmed, and neither answered nor routed. The first run
    // of 2026-09-13 left the Creator's *send frame 1* waiting.
    let waiting = client.get_updates(None, 0).expect("the ask was answered");
    let ids: Vec<i64> = waiting.as_array().into_iter().flatten().filter_map(|u| u.get("update_id")?.as_i64()).collect();
    let set_aside = ids.iter().copied().max();
    eprintln!("updates waiting before the watch began, to be set aside: {ids:?}");
    eprintln!("\n>>> LISTENING: the watch is asking for the Creator's first message now. <<<\n");
    let until = Instant::now() + A_STEP_OF_THE_CREATORS;
    loop {
        let updates = client.get_updates(None, 20).expect("the ask was answered");
        let found = updates.as_array().into_iter().flatten().find_map(|u| {
            let chat = u.pointer("/message/chat")?;
            let id = (chat.get("type")?.as_str()? == "private").then(|| chat.get("id")?.as_i64()).flatten()?;
            Some((id, u.get("update_id")?.as_i64()?))
        });
        if let Some((chat, update)) = found {
            eprintln!("the conversation was found from update {update}, left unconfirmed");
            return (chat, set_aside);
        }
        assert!(Instant::now() < until, "no message from the Creator arrived");
    }
}

fn the_watch(client: &TheClient, fleet: &mut TheFleet) {
    (fleet.conversation, fleet.set_aside) = the_conversation(client);
    let chat = fleet.conversation;
    for name in ["a", "b", "c", "d"] {
        fleet.prepare(name);
        fleet.start(name);
    }

    // ---- 1. status, with one process killed first
    let four: BTreeSet<String> = fleet.members.iter().map(|m| m.machine.clone()).collect();
    let mut holder = String::new();
    wait_for("one holds and all four are on the roster", Duration::from_secs(300), || {
        let Some(board) = the_board_read(client, chat) else { return false };
        let listed: BTreeSet<String> = board.roster.iter().map(|r| r.machine.clone()).collect();
        let holding: Vec<&AMemberProcess> = fleet
            .members
            .iter()
            .filter(|m| std::fs::read_to_string(m.folder.join("standing.txt")).is_ok_and(|s| s == "holding"))
            .collect();
        holder = fleet.the_name_of(&board.holder);
        four.is_subset(&listed) && holding.len() == 1 && holding[0].machine == board.holder
    });
    let killed = ["d", "c", "b", "a"].into_iter().find(|n| *n != holder).unwrap().to_string();
    let killed_machine = fleet.member(&killed).machine.clone();
    fleet.kill(&killed);
    let owners: Vec<String> =
        ["a", "b", "c", "d"].into_iter().filter(|n| *n != holder && *n != killed).map(str::to_string).collect();
    for (at, name) in owners.iter().enumerate() {
        let job = AJobAsHeld {
            job: Identity::issue(IdentityKind::Job).unwrap().as_text(),
            label: format!("Desk test job {}", name.to_uppercase()),
            generation: "1".into(),
            record_directory: true,
            rendering: Some(TheProgress { frame: 12 + at as u64, of: 40, percent: 30 + at as u8 }),
            waiting: None,
        };
        std::fs::write(fleet.member(name).folder.join("jobs.json"), json!([job]).to_string()).unwrap();
    }
    for member in &fleet.members {
        std::fs::write(member.folder.join("ready"), "").unwrap();
    }
    eprintln!("{holder} holds; {killed} was killed; the jobs are on {owners:?}; ready");
    wait_for("the Creator's status answered as one message", A_STEP_OF_THE_CREATORS, || {
        !fleet.lines("sent how it is going").is_empty()
    });
    let sent = fleet.lines("sent how it is going").remove(0);
    eprintln!("{sent}");
    assert!(sent.contains(": 3 answered, 1 did not answer") && sent.contains(&killed_machine), "the killed process named as not answering");

    // ---- 2. frame, and a tap resolved to its owner
    wait_for("the choice of job offered with two buttons", A_STEP_OF_THE_CREATORS, || {
        fleet.lines("ask-").iter().any(|l| l.contains("sent which job") && l.contains("with 2 buttons, added = true"))
    });
    wait_for("a tap acted on by its owner and trimmed from the board", A_STEP_OF_THE_CREATORS, || {
        !fleet.acts().is_empty() && the_board_read(client, chat).is_some_and(|b| b.instructions.is_empty())
    });
    let acts = fleet.acts();
    eprintln!("acted: {acts:?}");
    assert_eq!(acts.len(), 1, "one act");
    assert!(owners.contains(&acts[0].0) && acts[0].1.contains("send a frame") && acts[0].1.contains("live=true"));

    // ---- 3. the issuer gone, and a tap on its other button
    fleet.kill(&holder);
    wait_for("the other tap answered: nothing was done", A_STEP_OF_THE_CREATORS, || {
        fleet.lines("told the user tap-").len() == 1
    });
    std::thread::sleep(Duration::from_secs(20));
    assert_eq!(fleet.lines("told the user tap-").len(), 1, "answered once");
    assert_eq!(fleet.acts().len(), 1, "nothing was done for the second tap");
    eprintln!("the second tap was answered that nothing was done, and nothing was acted on");
}

#[test]
#[ignore = "uses the Creator's live bot token, asks for updates and starts real processes; run by hand at the desk, alone, when the turn is yours, with -- --ignored --exact --nocapture"]
fn a_tap_reaches_its_owner() {
    let token = the_token();
    let client = TheClient::reaching_telegram_with(&token);

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

    let root = std::env::temp_dir().join(format!("smoozy-routing-{}", Identity::issue(IdentityKind::Operation).unwrap().as_text()));
    std::fs::create_dir_all(&root).unwrap();
    let mut fleet = TheFleet { root: root.clone(), conversation: 0, set_aside: None, members: Vec::new() };

    let outcome = catch_unwind(AssertUnwindSafe(|| the_watch(&client, &mut fleet)));

    // ---- afterwards, whatever happened
    fleet.stop_every_member();
    let pinned = fleet.every_pinned_identity();
    if fleet.conversation != 0 {
        for id in &pinned {
            let _ = client.unpin_chat_message(fleet.conversation, *id);
        }
        let top = client.get_chat(fleet.conversation).ok().map(|c| what_is_pinned(&c));
        let still_ours = top.as_ref().and_then(WhatIsPinned::the_identity).is_some_and(|id| pinned.contains(&id));
        eprintln!("unpinned by identity: {} messages; one of ours still the pin = {still_ours}", pinned.len());
        assert!(!still_ours, "a message the processes pinned is still pinned");
    }
    eprintln!(
        "left in the conversation, nothing deleted but {} pins' lines: {} boards, {} machinery messages, {} messages to the user from the listener, {} gather messages, {} which-job messages, {} expiry answers, {} other answers, {} pin calls accepted",
        fleet.lines("deleted pin line ").len(),
        fleet.lines("sent board ").len(),
        fleet.lines("sent machinery message ").len(),
        fleet.lines("sent a message to the user").len(),
        fleet.lines("sent how it is going").len(),
        fleet.lines("ask-").iter().filter(|l| l.contains("sent which job")).count(),
        fleet.lines("told the user tap-").len(),
        fleet.lines("answered update ").len(),
        fleet.lines("pinned message ").len(),
    );
    eprintln!("updates confirmed: {:?}", fleet.lines("confirmed update "));
    eprintln!("updates dismissed: {:?}", fleet.lines("dismissed update "));
    eprintln!("updates set aside: {:?}", fleet.lines("set aside update "));
    eprintln!("machinery messages taken in from their pin updates: {}", fleet.lines("took in machinery message ").len());
    eprintln!("answers: {:?}", fleet.lines("answered update "));

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
    // **The processes' logs are kept**, outside the repository, before the
    // working folder goes: what each process noted and every call it made, with
    // times. Never the settings, which name the conversation.
    let kept = std::env::temp_dir().join(format!("{}-logs", root.file_name().unwrap().to_string_lossy()));
    for member in &fleet.members {
        let into = kept.join(&member.name);
        let _ = std::fs::create_dir_all(&into);
        for file in ["events.log", "timeline.log", "calls.log", "acted.txt", "out.txt"] {
            let _ = std::fs::copy(member.folder.join(file), into.join(file));
        }
    }
    eprintln!("the processes' logs are kept at {}", kept.display());
    let _ = std::fs::remove_dir_all(&root);
    assert_eq!(after.get("url").cloned(), url_before, "the webhook address moved");
    assert_eq!(after.get("allowed_updates").cloned(), Some(filter_before), "the standing filter was not put back");

    if let Err(failed) = outcome {
        resume_unwind(failed);
    }
}

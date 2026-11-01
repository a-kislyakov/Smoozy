//! **The milestone's second half, watched on the Creator's own bot: a fleet of
//! processes shares one pinned slot without losing an instruction.**
//! `Telegram` §3, B1 to B23, and §19's first three rows.
//!
//! **Off by default, and it says so when skipped.** The hosted runners have no
//! token, and only one agent may use the bot at a time: Telegram hands a bot's
//! updates to one caller, and the newest caller wins. Run it by hand, at the
//! desk, alone, and only when the turn is yours:
//!
//! `cargo test -p smoozy-telegram --test a_fleet_shares_one_slot a_fleet_shares_one_slot -- --ignored --exact --nocapture`
//!
//! **What it watches.** Three desk processes on the one token: one holds and the
//! rest stand down; an instruction is proposed by one that is not holding and
//! accepted onto the board; its sender and the holder are killed; the slot
//! moves to the third within one renewal interval and a read, carrying the
//! instruction; a fourth process addressed by it starts, acts on it once and
//! acknowledges it; the pins are popped back to the board; and `getChat` reads
//! the board as the pinned message.
//!
//! **What it leaves, and what it puts back.** It reads the webhook state and the
//! standing filter first, and stops before asking anything where a webhook is
//! registered. Afterwards it stops every process it started and checks each is
//! gone, unpins by identity everything the fleet pinned, **asks once more with
//! the filter exactly as found**, and reads the webhook state again to prove
//! it. **It deletes nothing but the lines its own pins write** (`Telegram` B18, deleted once processed): the boards, the machinery messages and any
//! message sent to the user stay in the conversation, and it prints how many.
//!
//! **What the Creator is asked to do**: send the bot any message, once, if no
//! message from their conversation is already waiting. That is how the
//! conversation is found without writing it anywhere; the message is consumed
//! by the fleet and its update identity is printed.
//!
//! **The token** is read by each process from its file and goes nowhere else:
//! not printed, not in a file, not an argument or an environment value of a
//! child. A child is told only the folder it works in.

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
    an_action_for, run_until_stopped, ActsOnAnInstruction, HandsOn, OnThisComputer, TheListener, TheSettings,
    WhatTheListenerIsHanded, WhoThisMachineIs,
};
use smoozy_telegram::pacing::{OnThisComputersClock, TheSendQueue};
use smoozy_telegram::transport::{OverHttps, ReachesTelegram, TheClient, WhatACallCarries};
use smoozy_telegram::worker_state::AFileStandIn;

/// The one thing a child is told: the folder it works in.
const THE_MEMBERS_FOLDER: &str = "SMOOZY_FLEET_MEMBER_FOLDER";

/// **The one value in the key file shaped like a bot token** — read the way
/// `tests/the_real_service_answers.rs` reads it, and never printed.
fn the_token() -> String {
    let home = std::env::var_os("HOME").expect("this desk has a home folder");
    let path = Path::new(&home).join(".smoozy-keys").join("telegram.txt");
    let text = std::fs::read_to_string(path).expect("the bot token file is readable");
    let shaped: Vec<&str> = text
        .lines()
        .map(|line| match line.split_once('=') {
            Some((_, value)) => value.trim(),
            None => line.trim(),
        })
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

// ------------------------------------------------------------ one member

fn append(path: &Path, line: &str) {
    let mut file = std::fs::OpenOptions::new().create(true).append(true).open(path).expect("a member's file opens");
    writeln!(file, "{line}").expect("a member's file is written");
    file.sync_all().expect("a member's file is synced");
}

fn lines_of(path: &Path) -> Vec<String> {
    std::fs::read_to_string(path).map(|t| t.lines().map(str::to_string).collect()).unwrap_or_default()
}

/// **`Telegram` B9 at a desk**: the identity recorded in a file before acting,
/// and the act written to another.
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
        append(&self.folder.join("acted.txt"), &entry.operation);
        append(&recorded, &entry.operation);
        Ok(())
    }
}

/// Piece 3 routes what the user sends; at this desk the update's identity is
/// written down, never its content.
struct WritesDownWhatItWasHanded {
    events: PathBuf,
}

impl HandsOn for WritesDownWhatItWasHanded {
    fn an_update_from_the_conversation(&mut self, update: &Value) -> Result<(), String> {
        append(&self.events, &format!("handed on update {}", update["update_id"]));
        Ok(())
    }
}

/// **One member of the fleet**, started only by [`a_fleet_shares_one_slot`].
#[test]
#[ignore = "one process of the live fleet; started by a_fleet_shares_one_slot and never on its own"]
fn a_fleet_member() {
    let Some(folder) = std::env::var_os(THE_MEMBERS_FOLDER).map(PathBuf::from) else {
        eprintln!("skipped: a fleet member runs only when a_fleet_shares_one_slot starts it");
        return;
    };
    let settings: Value =
        serde_json::from_str(&std::fs::read_to_string(folder.join("settings.json")).unwrap()).unwrap();
    let events = folder.join("events.log");
    let client = TheClient::reaching_telegram_with(&the_token());
    let queue = Arc::new(TheSendQueue::new(client.clone(), Arc::new(OnThisComputersClock)));
    let store = AFileStandIn::at(folder.join("state.json"), settings["routing_key"].as_str().unwrap()).unwrap();
    let noted = events.clone();
    let listener = Arc::new(Mutex::new(TheListener::new(WhatTheListenerIsHanded {
        client,
        queue: queue.clone(),
        clock: Arc::new(OnThisComputer::new()),
        me: WhoThisMachineIs {
            machine: settings["machine"].as_str().unwrap().to_string(),
            display: settings["display"].as_str().unwrap().into(),
        },
        settings: TheSettings::for_the_conversation(settings["conversation"].as_i64().unwrap()),
        store: Box::new(store),
        actor: Box::new(ActsInItsFolder { folder: folder.clone() }),
        hands_on: Box::new(WritesDownWhatItWasHanded { events: events.clone() }),
        notes: Box::new(move |what| append(&noted, what)),
    })));
    append(&events, "started");
    let stop = Arc::new(AtomicBool::new(false));
    let stopping = stop.clone();
    let at_the_end = events.clone();
    run_until_stopped(listener, queue, stop, move |held| {
        if folder.join("stop").exists() {
            stopping.store(true, Ordering::SeqCst);
        }
        let proposal = folder.join("propose.json");
        if let Ok(text) = std::fs::read_to_string(&proposal) {
            let _ = std::fs::remove_file(&proposal);
            let entry: AnEntry = serde_json::from_str(&text).unwrap();
            append(&events, &format!("proposed instruction {}", entry.operation));
            held.propose(entry);
        }
        let standing = if held.is_holding() { "holding" } else { "not holding" };
        let _ = std::fs::write(folder.join("standing.txt"), standing);
    });
    append(&at_the_end, "stopped");
}

// ------------------------------------------------------------ the fleet

struct AMemberProcess {
    name: String,
    machine: String,
    folder: PathBuf,
    child: Option<Child>,
}

struct TheFleet {
    root: PathBuf,
    conversation: i64,
    members: Vec<AMemberProcess>,
}

impl TheFleet {
    fn prepare(&mut self, name: &str) {
        let folder = self.root.join(name);
        std::fs::create_dir_all(&folder).unwrap();
        let machine = Identity::issue(IdentityKind::Installation).unwrap().as_text();
        let routing_key = Identity::issue(IdentityKind::Operation).unwrap().as_text();
        let settings = json!({
            "conversation": self.conversation,
            "machine": machine,
            "display": format!("Desk process {}", name.to_uppercase()),
            "routing_key": routing_key,
        });
        std::fs::write(folder.join("settings.json"), settings.to_string()).unwrap();
        self.members.push(AMemberProcess { name: name.into(), machine, folder, child: None });
    }

    fn member(&mut self, name: &str) -> &mut AMemberProcess {
        self.members.iter_mut().find(|m| m.name == name).unwrap()
    }

    fn start(&mut self, name: &str) {
        let member = self.member(name);
        let out = std::fs::File::create(member.folder.join("out.txt")).unwrap();
        let child = Command::new(std::env::current_exe().unwrap())
            .args(["a_fleet_member", "--exact", "--ignored", "--nocapture", "--test-threads=1"])
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

    fn the_machine_of(&self, name: &str) -> String {
        self.members.iter().find(|m| m.name == name).unwrap().machine.clone()
    }

    fn the_name_of(&self, machine: &str) -> String {
        self.members.iter().find(|m| m.machine == machine).map(|m| m.name.clone()).unwrap_or_default()
    }

    /// **Every process started is stopped and checked gone.**
    fn stop_every_member(&mut self) {
        for member in &self.members {
            let _ = std::fs::write(member.folder.join("stop"), "");
        }
        let until = Instant::now() + Duration::from_secs(15);
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

    fn every_line_of_every_member(&self, starting: &str) -> Vec<String> {
        self.members
            .iter()
            .flat_map(|m| lines_of(&m.folder.join("events.log")))
            .filter(|l| l.starts_with(starting))
            .collect()
    }

    fn every_pinned_identity(&self) -> BTreeSet<i64> {
        self.every_line_of_every_member("pinned message ")
            .iter()
            .filter_map(|l| l.rsplit(' ').next()?.parse().ok())
            .collect()
    }
}

fn the_board_read(client: &TheClient, conversation: i64) -> Option<(i64, TheBoard)> {
    match what_is_pinned(&client.get_chat(conversation).ok()?) {
        WhatIsPinned::ABoard { id, board } => Some((id, board)),
        _ => None,
    }
}

/// A wait with an end it always reaches: the condition, or the bound.
fn wait_for(what: &str, most: Duration, mut done: impl FnMut() -> bool) -> Duration {
    let started = Instant::now();
    loop {
        if done() {
            eprintln!("{what}: after {} seconds", started.elapsed().as_secs());
            return started.elapsed();
        }
        assert!(started.elapsed() < most, "{what}: not seen within {} seconds", most.as_secs());
        std::thread::sleep(Duration::from_secs(2));
    }
}

/// **The conversation, found without writing it down**: the newest private
/// message among the updates waiting, asked for without an offset so nothing is
/// confirmed. Where none waits, the Creator is asked to send one.
fn the_conversation(client: &TheClient) -> i64 {
    let until = Instant::now() + Duration::from_secs(300);
    let mut asked = false;
    loop {
        let updates = client.get_updates(None, if asked { 20 } else { 0 }).expect("the ask was answered");
        let found = updates.as_array().into_iter().flatten().rev().find_map(|u| {
            let chat = u.pointer("/message/chat")?;
            (chat.get("type")?.as_str()? == "private").then(|| chat.get("id")?.as_i64()).flatten()
        });
        if let Some(chat) = found {
            eprintln!("the conversation was found from a waiting update");
            return chat;
        }
        if !asked {
            eprintln!("\n>>> FOR THE CREATOR: send the Smoozy test bot any message in Telegram now. <<<\n");
            asked = true;
        }
        assert!(Instant::now() < until, "no message from the conversation arrived within five minutes");
    }
}

fn the_watch(client: &TheClient, fleet: &mut TheFleet) {
    fleet.conversation = the_conversation(client);
    let chat = fleet.conversation;
    for name in ["a", "b", "c", "d"] {
        fleet.prepare(name);
    }
    for name in ["a", "b", "c"] {
        fleet.start(name);
    }
    let mut holder = String::new();
    let three: BTreeSet<String> = ["a", "b", "c"].iter().map(|n| fleet.the_machine_of(n)).collect();
    wait_for("one holds and the others stand down, all three on the roster", Duration::from_secs(240), || {
        let Some((_, board)) = the_board_read(client, chat) else { return false };
        let listed: BTreeSet<String> = board.roster.iter().map(|r| r.machine.clone()).collect();
        let holding: Vec<&AMemberProcess> = fleet
            .members
            .iter()
            .filter(|m| std::fs::read_to_string(m.folder.join("standing.txt")).is_ok_and(|s| s == "holding"))
            .collect();
        holder = fleet.the_name_of(&board.holder);
        three.is_subset(&listed) && holding.len() == 1 && holding[0].machine == board.holder
    });
    eprintln!("{holder} holds the slot");

    let sender = ["a", "b", "c"].into_iter().find(|n| *n != holder).unwrap().to_string();
    let survivor = ["a", "b", "c"].into_iter().find(|n| *n != holder && *n != sender).unwrap().to_string();
    let entry = an_action_for(&fleet.the_machine_of("d"), "desk fleet test instruction", json!({})).unwrap();
    let operation = entry.operation.clone();
    std::fs::write(fleet.member(&sender).folder.join("propose.json"), serde_json::to_string(&entry).unwrap()).unwrap();
    wait_for(&format!("{sender}'s instruction accepted onto the board"), Duration::from_secs(90), || {
        the_board_read(client, chat).is_some_and(|(_, b)| b.instructions.iter().any(|e| e.operation == operation))
    });

    fleet.kill(&sender);
    fleet.kill(&holder);
    let survivor_machine = fleet.the_machine_of(&survivor);
    let moved = wait_for(&format!("the slot moved to {survivor}, still carrying the instruction"), Duration::from_secs(180), || {
        the_board_read(client, chat)
            .is_some_and(|(_, b)| b.holder == survivor_machine && b.instructions.iter().any(|e| e.operation == operation))
    });
    assert!(moved <= Duration::from_secs(60 + 30), "the slot took {} seconds to move", moved.as_secs());

    fleet.start("d");
    let acted = fleet.member("d").folder.join("acted.txt");
    wait_for("d acted on the instruction and the holder trimmed it", Duration::from_secs(120), || {
        !lines_of(&acted).is_empty()
            && the_board_read(client, chat).is_some_and(|(_, b)| b.instructions.iter().all(|e| e.operation != operation))
    });
    wait_for("the pins popped back to the board, and getChat reads the board as the pin", Duration::from_secs(60), || {
        the_board_read(client, chat).is_some_and(|(_, b)| b.holder == survivor_machine)
    });
    std::thread::sleep(Duration::from_secs(10));
    assert_eq!(lines_of(&acted), vec![operation.clone()], "d acted on it exactly once");
    let (_, board) = the_board_read(client, chat).expect("the board is still the pinned message");
    assert_eq!(board.holder, survivor_machine);
    eprintln!("the instruction survived its sender and its holder and was acknowledged");
}

#[test]
#[ignore = "uses the Creator's live bot token and starts real processes; run by hand at the desk, alone, when the turn is yours, with -- --ignored --exact --nocapture"]
fn a_fleet_shares_one_slot() {
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

    let root = std::env::temp_dir().join(format!("smoozy-fleet-{}", Identity::issue(IdentityKind::Operation).unwrap().as_text()));
    std::fs::create_dir_all(&root).unwrap();
    let mut fleet = TheFleet { root: root.clone(), conversation: 0, members: Vec::new() };

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
        assert!(!still_ours, "a message the fleet pinned is still pinned");
    }
    eprintln!(
        "left in the conversation, nothing deleted but {} pins' lines: {} boards, {} machinery messages, {} messages to the user, {} pin calls accepted (a pinned-message line is written only where a message went from unpinned to pinned, so the lines are at most this many)",
        fleet.every_line_of_every_member("deleted pin line ").len(),
        fleet.every_line_of_every_member("sent board ").len(),
        fleet.every_line_of_every_member("sent machinery message ").len(),
        fleet.every_line_of_every_member("sent a message to the user").len(),
        fleet.every_line_of_every_member("pinned message ").len(),
    );
    eprintln!("updates confirmed by the fleet: {:?}", fleet.every_line_of_every_member("confirmed update "));
    eprintln!("updates handed on: {:?}", fleet.every_line_of_every_member("handed on update "));
    eprintln!("updates dismissed: {:?}", fleet.every_line_of_every_member("dismissed update "));

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
        "after: webhook address empty = {}, standing filter = {}",
        after.get("url").and_then(Value::as_str) == Some(""),
        after.get("allowed_updates").map_or("not stated".into(), Value::to_string),
    );
    let _ = std::fs::remove_dir_all(&root);
    assert_eq!(after.get("url").cloned(), url_before, "the webhook address moved");
    assert_eq!(after.get("allowed_updates").cloned(), Some(filter_before), "the standing filter was not put back");

    if let Err(failed) = outcome {
        resume_unwind(failed);
    }
}

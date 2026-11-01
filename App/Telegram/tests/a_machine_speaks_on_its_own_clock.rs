//! **Step 10 piece 6's watch: a desk process speaking on a machine's own
//! clock** — `Telegram` C1, C2, C2a, C7 and B16 for the status message, I3 for
//! pictures, and M2 for the contact sheet.
//!
//! **Two runs of one sequence.** The first is against the stand-in and runs
//! everywhere. The second is against Telegram's real service on the Creator's
//! bot, **off by default**, and run by hand at the desk by the one agent holding
//! the turn:
//!
//! `SMOOZY_TELEGRAM_CONVERSATION=<the conversation's identity> SMOOZY_PIECE6_PICTURES=<a folder> cargo test -p smoozy-telegram --test a_machine_speaks_on_its_own_clock -- --ignored --nocapture`
//!
//! The folder holds ten pictures named `frame01.jpg` to `frame10.jpg` and two
//! named `too-large-1.jpg` and `too-large-2.jpg`, all placed by hand.
//!
//! **What the live run does**: reads the webhook state; creates one status
//! message and edits it five times on C2's clock for one machine; sends one
//! picture; is handed two pictures outside Telegram's limits for one job and
//! says so once; sends an album of ten; reads the webhook state again, which
//! must be as found. **It asks for no updates, pins nothing, unpins nothing
//! and deletes nothing** — a wire around the real one fails the run if any of
//! those is called. **Every message it leaves is printed by identity.**
//!
//! **The token is read by this process from its file and goes nowhere else.**

use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};

use serde_json::Value;
use smoozy_telegram::media::{offer_the_contact_sheet, the_contact_sheet};
use smoozy_telegram::pacing::{AClassOfSend, HowASendEnded, OnThisComputersClock, TheSendQueue};
use smoozy_telegram::pictures::{ADerivative, ThePictures, WhatBecameOfAPicture};
use smoozy_telegram::status::{
    the_interval_for, AReading, CardMemory, TheClock, TheMachineKind, TheStatus, TheUptime, WhatATickDid,
    WhatTheMachineIsDoing,
};
use smoozy_telegram::transport::a_stand_in::AStandInService;
use smoozy_telegram::transport::{ReachesTelegram, TheClient, WhatACallCarries};
use smoozy_telegram::worker_state::{InMemory, TheTelegramWorkersState};

/// **The calls this watch may never make**: no ask for updates (the turn
/// rule), no pin or unpin (B16), no deletion (B18).
const NEVER: [&str; 4] = ["getUpdates", "pinChatMessage", "unpinChatMessage", "deleteMessage"];

/// A wire that writes down every method called, around whichever wire is real.
struct WritesDown {
    inner: Arc<dyn ReachesTelegram>,
    methods: Mutex<Vec<String>>,
}

impl ReachesTelegram for WritesDown {
    fn call(&self, method: &str, carries: &WhatACallCarries, bound: Duration) -> Result<String, String> {
        self.methods.lock().unwrap().push(method.to_string());
        assert!(!NEVER.contains(&method), "the watch called {method}, which it may never call");
        self.inner.call(method, carries, bound)
    }
}

struct WhatTheWatchSaw {
    ticks: Vec<WhatATickDid>,
    status_message: Option<i64>,
    edits_accepted: usize,
    left_in_the_conversation: Vec<(String, i64)>,
    too_large: Vec<&'static str>,
}

fn ended(told: Receiver<HowASendEnded>) -> HowASendEnded {
    told.recv_timeout(Duration::from_secs(360)).expect("the send ended")
}

fn the_message_ids(result: &Value) -> Vec<i64> {
    match result {
        Value::Array(many) => many.iter().filter_map(|m| m["message_id"].as_i64()).collect(),
        one => one["message_id"].as_i64().into_iter().collect(),
    }
}

fn a_status(frame: u64, now: SystemTime) -> TheStatus {
    TheStatus {
        server_label: "Desk test server".into(),
        doing: WhatTheMachineIsDoing::Rendering {
            job_label: "Desk test job \u{2014} piece 6".into(),
            detail: Some("Scene".into()),
            frame,
            of: 240,
            percent: (frame * 100 / 240) as u8,
            this_frame: None,
        },
        card_memory: AReading::Taken { value: CardMemory { used_gb: 9.4, total_gb: 16.0 }, at: now },
        disk_free_gb: AReading::Taken { value: 212.0, at: now },
        uptime: TheUptime::on(TheMachineKind::AContainer, || unreachable!("never read on a container")),
    }
}

/// **The one sequence both runs make.**
fn the_watch(
    client: TheClient,
    chat: i64,
    tick_every: Duration,
    ten: Vec<(String, Vec<u8>)>,
    too_large: Vec<(String, Vec<u8>)>,
) -> WhatTheWatchSaw {
    let queue = Arc::new(TheSendQueue::new(client, Arc::new(OnThisComputersClock)));
    let stop = Arc::new(AtomicBool::new(false));
    let sender = {
        let (queue, stop) = (queue.clone(), stop.clone());
        std::thread::spawn(move || {
            while !stop.load(Ordering::SeqCst) {
                if queue.send_the_next().is_none() {
                    std::thread::sleep(Duration::from_millis(20));
                }
            }
        })
    };
    // **`Telegram` C6b: the album goes in the upload lane, sent beside the
    // serial one.**
    let uploader = {
        let (queue, stop) = (queue.clone(), stop.clone());
        std::thread::spawn(move || {
            while !stop.load(Ordering::SeqCst) {
                if !queue.send_the_next_upload() {
                    std::thread::sleep(Duration::from_millis(20));
                }
            }
        })
    };
    let mut saw = WhatTheWatchSaw {
        ticks: Vec::new(),
        status_message: None,
        edits_accepted: 0,
        left_in_the_conversation: Vec::new(),
        too_large: Vec::new(),
    };

    // **C2, C2a, C7 — six ticks: one creation and five edits.**
    let mut store = InMemory::with_routing_key("00112233445566778899aabbccddeeff");
    let mut clock = TheClock::for_the_conversation(chat);
    for tick in 0..6u64 {
        let now = SystemTime::now();
        let did = clock.a_tick(&mut store, &queue, &a_status(148 + tick, now), None, now).unwrap();
        eprintln!("tick {tick}: {did:?}");
        saw.ticks.push(did);
        // Wait for this tick's send to end, so a slow round trip is not
        // mistaken for a dropped tick, then keep to the clock.
        let started = std::time::Instant::now();
        while clock.take_in_what_ended(&mut store).unwrap() {
            std::thread::sleep(Duration::from_millis(50));
        }
        if matches!(saw.ticks.last(), Some(WhatATickDid::OfferedAnEdit(_))) {
            saw.edits_accepted += 1;
        }
        std::thread::sleep(tick_every.saturating_sub(started.elapsed()));
    }
    saw.status_message = store.the_status_message().unwrap();
    if let Some(id) = saw.status_message {
        saw.left_in_the_conversation.push(("the status message".into(), id));
    }

    // **I3 — one picture sent, and two outside the limits said once.**
    let mut pictures = ThePictures::new(chat, "Desk test server");
    let a_derivative = |frame: u64, (file_name, bytes): (String, Vec<u8>)| ADerivative {
        job: "desk-test-job-piece-6".into(),
        job_label: "Desk test job \u{2014} piece 6".into(),
        frame,
        file_name,
        bytes,
        which_picture: None,
    };
    match pictures.offer(&queue, a_derivative(148, ten[0].clone()), AClassOfSend::FramePicturesAndMedia) {
        WhatBecameOfAPicture::Offered(told) => match ended(told) {
            HowASendEnded::Accepted(result) => {
                for id in the_message_ids(&result) {
                    saw.left_in_the_conversation.push(("one picture".into(), id));
                }
            }
            other => panic!("the picture was not accepted: {other:?}"),
        },
        other => panic!("the picture was not offered: {other:?}"),
    }
    for (at, picture) in too_large.into_iter().enumerate() {
        match pictures.offer(&queue, a_derivative(200 + at as u64, picture), AClassOfSend::FramePicturesAndMedia) {
            WhatBecameOfAPicture::NotSentAndSaid(told) => {
                saw.too_large.push("not sent, and said");
                if let HowASendEnded::Accepted(result) = ended(told) {
                    for id in the_message_ids(&result) {
                        saw.left_in_the_conversation.push(("the once-per-job telling".into(), id));
                    }
                }
            }
            WhatBecameOfAPicture::NotSentAlreadySaid => saw.too_large.push("not sent, already said"),
            WhatBecameOfAPicture::Offered(_) => saw.too_large.push("offered, which is wrong"),
        }
    }

    // **M2 — the contact sheet: ten of 240 frames, evenly spread.**
    let frames: Vec<u64> = (1..=240).collect();
    let chosen: Vec<ADerivative> =
        the_contact_sheet(&frames).into_iter().zip(ten).map(|(frame, picture)| a_derivative(frame, picture)).collect();
    let told = offer_the_contact_sheet(&queue, chat, "Desk test server", "Desk test job \u{2014} piece 6", None, 240, chosen)
        .expect("ten pictures make a sheet");
    match ended(told) {
        HowASendEnded::Accepted(result) => {
            for id in the_message_ids(&result) {
                saw.left_in_the_conversation.push(("the album".into(), id));
            }
        }
        other => panic!("the album was not accepted: {other:?}"),
    }

    stop.store(true, Ordering::SeqCst);
    sender.join().unwrap();
    uploader.join().unwrap();
    saw
}

fn what_must_hold(saw: &WhatTheWatchSaw) {
    assert_eq!(saw.ticks[0], WhatATickDid::OfferedACreation { replacing: None });
    let id = saw.status_message.expect("the status message was recorded");
    assert!(saw.ticks[1..].iter().all(|t| *t == WhatATickDid::OfferedAnEdit(id)), "{:?}", saw.ticks);
    assert_eq!(saw.edits_accepted, 5);
    assert_eq!(saw.too_large, ["not sent, and said", "not sent, already said"]);
    assert_eq!(saw.left_in_the_conversation.iter().filter(|(what, _)| what == "the album").count(), 10);
}

fn a_png_header(width: u32, height: u32) -> Vec<u8> {
    let mut bytes = vec![0x89, b'P', b'N', b'G', 0x0d, 0x0a, 0x1a, 0x0a, 0, 0, 0, 13];
    bytes.extend_from_slice(b"IHDR");
    bytes.extend_from_slice(&width.to_be_bytes());
    bytes.extend_from_slice(&height.to_be_bytes());
    bytes.extend_from_slice(&[8, 2, 0, 0, 0, 0, 0, 0, 0]);
    bytes
}

/// **The sequence against the stand-in**, with every call checked.
#[test]
fn a_machine_speaks_on_its_own_clock_against_the_stand_in() {
    let service = Arc::new(AStandInService::new());
    service.then_answers(r#"{"ok":true,"result":{"message_id":500}}"#);
    for _ in 0..5 {
        service.then_answers(r#"{"ok":true,"result":{"message_id":500}}"#);
    }
    service.then_answers(r#"{"ok":true,"result":{"message_id":501}}"#);
    service.then_answers(r#"{"ok":true,"result":{"message_id":502}}"#);
    let album: Vec<Value> = (503..513).map(|id| serde_json::json!({ "message_id": id })).collect();
    service.then_answers(&serde_json::json!({ "ok": true, "result": album }).to_string());

    let wire = Arc::new(WritesDown { inner: service.clone(), methods: Mutex::new(Vec::new()) });
    let ten = (1..=10).map(|n| (format!("frame{n:02}.png"), a_png_header(1280, 720))).collect();
    let too_large = (1..=2).map(|n| (format!("too-large-{n}.png"), a_png_header(6000, 6000))).collect();
    let saw = the_watch(TheClient::over(wire.clone()), 4242, Duration::ZERO, ten, too_large);

    what_must_hold(&saw);
    assert_eq!(
        *wire.methods.lock().unwrap(),
        [
            "sendMessage",
            "editMessageText",
            "editMessageText",
            "editMessageText",
            "editMessageText",
            "editMessageText",
            "sendPhoto",
            "sendMessage",
            "sendMediaGroup"
        ]
    );
    let heard = service.heard();
    let WhatACallCarries::Fields(last_edit) = &heard[5].carries else { panic!() };
    let text = last_edit["text"].as_str().unwrap();
    assert!(text.starts_with("Desk test server\nDesk test job \u{2014} piece 6 \u{b7} Scene\nRendering: frame 153 of 240"), "{text}");
    assert!(text.contains("\nUptime: unavailable\nUpdated ") && text.ends_with(" UTC"), "{text}");
}

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

/// **The sequence against Telegram's real service, on the Creator's bot.**
#[test]
#[ignore = "sends to the Creator's live bot; run by hand at the desk on the runner's turn, with -- --ignored --nocapture"]
fn a_machine_speaks_on_its_own_clock_on_the_real_service() {
    let (Some(chat), Some(folder)) = (
        std::env::var("SMOOZY_TELEGRAM_CONVERSATION").ok().and_then(|c| c.parse::<i64>().ok()),
        std::env::var_os("SMOOZY_PIECE6_PICTURES"),
    ) else {
        eprintln!("skipped: SMOOZY_TELEGRAM_CONVERSATION and SMOOZY_PIECE6_PICTURES are not both set");
        return;
    };
    let read = |name: &str| (name.to_string(), std::fs::read(Path::new(&folder).join(name)).expect("a hand-placed picture"));
    let ten: Vec<_> = (1..=10).map(|n| read(&format!("frame{n:02}.jpg"))).collect();
    let too_large = vec![read("too-large-1.jpg"), read("too-large-2.jpg")];

    let real = TheClient::reaching_telegram_with(&the_token());
    let before = real.get_webhook_info().expect("getWebhookInfo was answered");
    eprintln!(
        "before: webhook address empty = {}, standing filter = {}, updates waiting = {}",
        before["url"] == "",
        before["allowed_updates"],
        before["pending_update_count"]
    );

    let wire = Arc::new(WritesDown {
        inner: Arc::new(smoozy_telegram::transport::OverHttps::reaching_telegram_with(&the_token())),
        methods: Mutex::new(Vec::new()),
    });
    let saw = the_watch(TheClient::over(wire.clone()), chat, the_interval_for(1), ten, too_large);

    let after = real.get_webhook_info().expect("getWebhookInfo was answered again");
    eprintln!(
        "after: webhook address empty = {}, standing filter = {}, updates waiting = {}",
        after["url"] == "",
        after["allowed_updates"],
        after["pending_update_count"]
    );
    eprintln!("calls made: {:?}", wire.methods.lock().unwrap());
    for (what, id) in &saw.left_in_the_conversation {
        eprintln!("left in the conversation: {what}, message {id}");
    }
    assert_eq!(after["url"], before["url"], "the webhook address moved");
    assert_eq!(after["allowed_updates"], before["allowed_updates"], "the standing filter moved");
    what_must_hold(&saw);
}

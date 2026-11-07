//! **The one way to Telegram.** Every call Smoozy makes to Telegram's service —
//! from the desktop, and from a desk process standing in for a machine — goes
//! through [`TheClient`]. Nothing else in this crate, and nothing outside it,
//! speaks to the service. The send queue that paces what a machine sends on its
//! own is beside this file, at [`crate::pacing`].
//!
//! **`Telegram` P6: the second list is the fleet's hearing.** The token carries
//! a standing filter of update kinds. One caller setting it changes what every
//! machine hears, it shows in no ordinary answer, and only `getWebhookInfo`
//! reveals it. A kind left out drops the user's own messages, fleet-wide, and a
//! message dropped that way is gone. **So the kinds are written once, in
//! [`THE_UPDATE_KINDS_EVERY_ASK_NAMES`]; the one ask this client makes names
//! them on every call; and a test in this file fails if any code in the crate
//! builds a list of its own or asks for updates by another route.**
//!
//! **`Telegram` J1, J2 and J4 are deliberately not built.** The webhook route
//! is optional and nothing in the plan needs it (J1); J4 says no build takes it
//! until the content type of Telegram's own request is known, and that is not
//! known. **So this client has no registration call at all**, and J2's conduct
//! for a registration holder has nothing to act on. `deleteWebhook` is here
//! because `Telegram` J3 uses it on the asking route, and it never passes
//! `drop_pending_updates` (J2, J3): queued updates are the user's messages.
//!
//! **The token is held by [`OverHttps`] and nowhere else, and it is never
//! shown.** That type's debug form leaves it out, and every error the client
//! raises has it taken out before the text leaves this file, because the
//! client's error text can carry the address and the address carries the token.
//!
//! **What this file relies on about the service**, from
//! `https://core.telegram.org/bots/api`, read 2026-09-13 — every request is
//! made to `https://api.telegram.org/bot<token>/METHOD_NAME` over HTTPS;
//! parameters may be sent as JSON *"except for uploading files"*, and files go
//! as `multipart/form-data`, attached by `attach://<name>` inside an album; an
//! answer always carries `ok`, an accepted one carries `result`, and a refused
//! one carries `error_code`, `description` and sometimes
//! `parameters.retry_after`; `getUpdates` confirms nothing unless it is given
//! an `offset`; and `allowed_updates`, *"If not specified, the previous setting
//! will be used."*
//!
//! **A development build can be told to reach nothing at all**, so that a desk
//! test starting the real machine program with made-up Telegram material never
//! asks Telegram anything. The switch is an environment value read once where
//! the real client is made; with it on, every call fails at once as a line that
//! is down, before any connection is opened and before the address that carries
//! the token is even written. **It is not an address redirect**: the client is
//! never pointed anywhere else, so the token has nowhere to go. **A release
//! build has no switch** — no name, no reading, no check —
//! [`whether_the_switch_says_reach_nothing`] answers *no* there whatever it is
//! handed, and a test says so. See
//! [`THE_DEVELOPMENT_SWITCH_THAT_REACHES_NOTHING`].
//!
//! **Markers in this file are all `Telegram`'s**: P6, P6a, J1, J2, J3, J4,
//! B18, B24, B25, R16, and C6 where a call's bound is shortened.

use std::collections::VecDeque;
use std::fmt;
use std::io::Write as _;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use serde_json::{json, Map, Value};

/// **`Telegram` P6 and P6a — the update kinds every ask names, and the only
/// place they are written.**
///
/// - **`message`** carries the user's typed requests and replies, the
///   walkthrough's Start, and every pin's service message — the channel the
///   machines use to talk to each other (`Telegram` B10, B11). **That third use
///   is the one a shorter list would silently break.**
/// - **`callback_query`** carries every tap on a button.
///
/// **Nothing else is named, and that is a decision.** P6a: a later clause that
/// needs another kind adds it here, in this one place, and every ask carries it
/// by construction.
pub const THE_UPDATE_KINDS_EVERY_ASK_NAMES: [&str; 2] = ["message", "callback_query"];

/// Where Telegram's service answers.
pub const THE_SERVICES_ADDRESS: &str = "https://api.telegram.org";

/// **How long an ordinary call may take, sending and answer together.** A call
/// that has not ended by then is reported as not reached, so a stalled line
/// cannot hold a send queue's one slot for ever.
pub const THE_BOUND_ON_AN_ORDINARY_CALL: Duration = Duration::from_secs(30);

/// **The least time a call that uploads a file is given**, however small the
/// file. Telegram accepts up to 50 MB for a file other than a photo, and
/// `Server protocol and transfers` R17 has this project's own reading of a real
/// line at well under a megabyte a second.
pub const THE_BOUND_ON_SENDING_A_FILE: Duration = Duration::from_secs(300);

/// **The slowest line an upload's time limit allows for: 50 KB a second.**
/// A choice, not a fact; the round-1 code audit of the bot and the chat, fix-here
/// item 15.
pub const THE_SLOWEST_LINE_AN_UPLOAD_ALLOWS_FOR: u64 = 50_000;

/// **`Telegram` C6b — an upload *"runs to its end or to its own failure"*, so
/// its time limit grows with the file**: the larger of
/// [`THE_BOUND_ON_SENDING_A_FILE`] and the file's size at
/// [`THE_SLOWEST_LINE_AN_UPLOAD_ALLOWS_FOR`] — about 17 minutes for 50 MB. A flat
/// 300 seconds could never finish a 50 MB file on a line under about 170 KB a
/// second. The upload lane holds nothing else back, so a long limit costs
/// nothing.
pub fn the_bound_on_sending(bytes: usize) -> Duration {
    let at_the_slowest = (bytes as u64).div_ceil(THE_SLOWEST_LINE_AN_UPLOAD_ALLOWS_FOR);
    THE_BOUND_ON_SENDING_A_FILE.max(Duration::from_secs(at_the_slowest))
}

/// **How much longer than its own wait an ask for updates may take.** The ask
/// tells the service how long to hold the request open; the call's bound is
/// that wait plus this, so a slow answer at the end of a long wait is not cut.
pub const THE_MARGIN_BEYOND_AN_ASKS_OWN_WAIT: Duration = Duration::from_secs(30);

/// **The most bytes one answer may hold.** A hundred updates at the service's
/// own limit is far below this; it exists so a wrong answer cannot fill memory.
pub const THE_MOST_AN_ANSWER_MAY_HOLD: u64 = 16 * 1024 * 1024;

/// What the token is replaced with wherever text could otherwise carry it.
const WHERE_THE_TOKEN_WAS: &str = "[the bot token, not shown]";

// ------------------------------------------------------------ the switch

/// **The development switch: set to `1`, the real client reaches nothing at
/// all.** Read once, where [`OverHttps::reaching_telegram_with`] makes the
/// client — which, on a machine, is the Telegram worker's start. Any other
/// value, or none, leaves the client as it is.
///
/// **Why an environment value and not a launch argument.** The worker is
/// started by the supervisor, not by the test, and the supervisor composes the
/// worker's arguments itself; an argument would have to be carried through
/// both, in the program's own code. An environment value is inherited by every
/// process the supervisor starts, so the test sets it once on the supervisor it
/// starts and nothing in between is touched. It carries no secret (`Security
/// and licence` §13's rule 1 is about secrets) and it moves nothing: it can
/// only take the line away.
///
/// **Present in a development build only** — `cfg(debug_assertions)`, the gate
/// the throwaway launch setting uses (`Audits/Build/Step 10b/Piece 1 — what
/// landed.md`). Every release route builds the release profile, where debug
/// assertions are off, so this name is not in a release build at all.
#[cfg(debug_assertions)]
pub const THE_DEVELOPMENT_SWITCH_THAT_REACHES_NOTHING: &str =
    "SMOOZY_DEVELOPMENT_TELEGRAM_REACHES_NOTHING";

/// What every call gives back with the switch on: *not reached*, as a down
/// line gives. It names the switch, so nobody reading a note mistakes it for
/// Telegram being away.
#[cfg(debug_assertions)]
fn what_a_call_says_with_the_switch_on() -> String {
    format!(
        "not reached: this development run was started with \
         {THE_DEVELOPMENT_SWITCH_THAT_REACHES_NOTHING}=1, so no call leaves this process"
    )
}

/// **Whether the switch, holding `value`, says reach nothing.** It exists in
/// both builds so the release gate is asked a real question: in a development
/// build `1` says yes; **in a release build the answer is no, whatever the
/// value**.
pub fn whether_the_switch_says_reach_nothing(value: Option<&std::ffi::OsStr>) -> bool {
    #[cfg(debug_assertions)]
    {
        value.is_some_and(|value| value == "1")
    }
    #[cfg(not(debug_assertions))]
    {
        let _ = value;
        false
    }
}

/// **This process's reading of the switch.** In a release build nothing is
/// read.
fn this_run_reaches_nothing() -> bool {
    #[cfg(debug_assertions)]
    {
        whether_the_switch_says_reach_nothing(
            std::env::var_os(THE_DEVELOPMENT_SWITCH_THAT_REACHES_NOTHING).as_deref(),
        )
    }
    #[cfg(not(debug_assertions))]
    {
        false
    }
}

// ------------------------------------------------------------ the call

/// **What one call carries to the service.**
#[derive(Debug, Clone, PartialEq)]
pub enum WhatACallCarries {
    /// No parameters at all.
    Nothing,
    /// Parameters, sent as JSON.
    Fields(Map<String, Value>),
    /// Parameters and files, sent as a multipart form. A parameter that is not
    /// text is written as JSON, which is how the service reads one there.
    FieldsAndFiles {
        fields: Map<String, Value>,
        files: Vec<AFileToSend>,
    },
}

/// **One file a call uploads**, under the form part name the call refers to it
/// by.
#[derive(Clone, PartialEq, Eq)]
pub struct AFileToSend {
    pub part_name: String,
    pub file_name: String,
    pub bytes: Vec<u8>,
}

impl fmt::Debug for AFileToSend {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AFileToSend")
            .field("part_name", &self.part_name)
            .field("file_name", &self.file_name)
            .field("bytes", &self.bytes.len())
            .finish()
    }
}

/// **A refusal, as the service stated it.** Every field is optional because the
/// service's own page says `error_code` is *"subject to change in the future"*
/// and `parameters` appears only on some refusals.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ARefusal {
    pub error_code: Option<i64>,
    pub description: Option<String>,
    /// Seconds to wait before the same request may be repeated.
    pub retry_after: Option<u64>,
}

/// **Why a call did not come back accepted.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhyTheCallFailed {
    /// The service answered and refused. What to do about a refusal is the
    /// caller's; the send queue's rule is `Telegram` C6a, at [`crate::pacing`].
    Refused(ARefusal),
    /// No answer came back: no connection, a broken one, or the bound passed.
    /// **The text has had the token taken out.**
    NotReached(String),
    /// Something came back that is not an answer the service's page describes.
    /// **The answer's own text is not kept**, because an answer can hold the
    /// user's messages.
    TheAnswerCouldNotBeRead(&'static str),
}

/// **The one thing that actually reaches the service.** The real one is
/// [`OverHttps`]; tests of this crate and of every later piece use
/// [`a_stand_in::AStandInService`].
///
/// It is handed the method name, what the call carries and how long it may
/// take, and gives back the answer's text or why there was none. Reading the
/// answer is [`what_the_answer_says`]'s, so both the real one and a stand-in
/// are read by the same rule.
pub trait ReachesTelegram: Send + Sync {
    fn call(
        &self,
        method: &str,
        carries: &WhatACallCarries,
        bound: Duration,
    ) -> Result<String, String>;
}

/// **Read an answer the way the service's page describes it.** `ok` true gives
/// `result`; `ok` false gives the refusal; anything else is unreadable, and
/// unreadable is never taken as accepted.
pub fn what_the_answer_says(text: &str) -> Result<Value, WhyTheCallFailed> {
    let whole: Value = serde_json::from_str(text)
        .map_err(|_| WhyTheCallFailed::TheAnswerCouldNotBeRead("the answer was not JSON"))?;
    match whole.get("ok").and_then(Value::as_bool) {
        Some(true) => {
            whole
                .get("result")
                .cloned()
                .ok_or(WhyTheCallFailed::TheAnswerCouldNotBeRead(
                    "an accepted answer carried no result",
                ))
        }
        Some(false) => Err(WhyTheCallFailed::Refused(ARefusal {
            error_code: whole.get("error_code").and_then(Value::as_i64),
            description: whole
                .get("description")
                .and_then(Value::as_str)
                .map(str::to_string),
            retry_after: whole
                .pointer("/parameters/retry_after")
                .and_then(Value::as_u64),
        })),
        None => Err(WhyTheCallFailed::TheAnswerCouldNotBeRead(
            "the answer carried no ok field",
        )),
    }
}

// ------------------------------------------------------------ the real one

/// **The real service, over the one HTTPS client.**
pub struct OverHttps {
    agent: ureq::Agent,
    address: String,
    token: String,
    /// **The development switch, as read when this was made.** Not in a
    /// release build.
    #[cfg(debug_assertions)]
    reaches_nothing: bool,
}

impl fmt::Debug for OverHttps {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut said = f.debug_struct("OverHttps");
        said.field("address", &self.address)
            .field("token", &WHERE_THE_TOKEN_WAS);
        #[cfg(debug_assertions)]
        said.field("reaches_nothing", &self.reaches_nothing);
        said.finish()
    }
}

impl OverHttps {
    /// **Reach Telegram's service with this token.** **In a development build
    /// the switch is read here** ([`THE_DEVELOPMENT_SWITCH_THAT_REACHES_NOTHING`]).
    ///
    /// **No processor is refused.** The cryptography is `ring`'s, which runs on
    /// any 64-bit Intel or AMD processor and any Apple silicon one; until
    /// 2026-10-24 it was a provider that needed six instruction sets and this
    /// asked for them first (the Creator's eighteenth notes, note 3).
    pub fn reaching_telegram_with(token: &str) -> OverHttps {
        OverHttps::reaching(THE_SERVICES_ADDRESS, token, this_run_reaches_nothing())
    }

    fn reaching(address: &str, token: &str, reaches_nothing: bool) -> OverHttps {
        #[cfg(not(debug_assertions))]
        let _ = reaches_nothing;
        #[cfg(debug_assertions)]
        if reaches_nothing {
            // Said once, where the process's complaints go, so a development
            // run that hears nothing from Telegram says why.
            eprintln!("smoozy: Telegram client: {}", what_a_call_says_with_the_switch_on());
        }

        // The provider is handed to this client rather than installed for the
        // whole program, the same choice the release fetcher and the Blender
        // listing make, so no part of Smoozy's cryptography depends on which
        // part ran first.
        let config = ureq::Agent::config_builder()
            .tls_config(
                ureq::tls::TlsConfig::builder()
                    .provider(ureq::tls::TlsProvider::Rustls)
                    .unversioned_rustls_crypto_provider(Arc::new(
                        rustls::crypto::ring::default_provider(),
                    ))
                    .build(),
            )
            .https_only(true)
            // The service is one address and hands nothing on.
            .max_redirects(0)
            // A refusal comes back with a status and a body, and the body is
            // what says why, so the status is read rather than raised.
            .http_status_as_error(false)
            .build();

        OverHttps {
            agent: ureq::Agent::new_with_config(config),
            address: address.to_string(),
            token: token.to_string(),
            #[cfg(debug_assertions)]
            reaches_nothing,
        }
    }

    fn without_the_token(&self, text: String) -> String {
        if self.token.is_empty() {
            text
        } else {
            text.replace(&self.token, WHERE_THE_TOKEN_WAS)
        }
    }
}

impl ReachesTelegram for OverHttps {
    fn call(
        &self,
        method: &str,
        carries: &WhatACallCarries,
        bound: Duration,
    ) -> Result<String, String> {
        // **The development switch comes first**: nothing below runs — no
        // address written, no connection, no body — so neither the call nor
        // the token leaves this process. Not in a release build.
        #[cfg(debug_assertions)]
        if self.reaches_nothing {
            return Err(what_a_call_says_with_the_switch_on());
        }
        let address = format!("{}/bot{}/{}", self.address, self.token, method);
        let request = self
            .agent
            .post(&address)
            .config()
            .timeout_global(Some(bound))
            .build();

        let sent = match carries {
            WhatACallCarries::Nothing => request.send_empty(),
            WhatACallCarries::Fields(fields) => {
                let body = serde_json::to_vec(fields)
                    .map_err(|_| "the call's parameters could not be written".to_string())?;
                request.content_type("application/json").send(body)
            }
            WhatACallCarries::FieldsAndFiles { fields, files } => {
                let boundary = a_fresh_boundary()?;
                let body = a_form(fields, files, &boundary);
                request
                    .content_type(format!("multipart/form-data; boundary={boundary}"))
                    .send(body)
            }
        };

        let mut answer = sent.map_err(|said| self.without_the_token(said.to_string()))?;
        answer
            .body_mut()
            .with_config()
            .limit(THE_MOST_AN_ANSWER_MAY_HOLD)
            .read_to_string()
            .map_err(|said| self.without_the_token(said.to_string()))
    }
}

/// A form boundary nobody's file will contain by accident: 128 random bits.
fn a_fresh_boundary() -> Result<String, String> {
    let mut bytes = [0u8; 16];
    getrandom::fill(&mut bytes).map_err(|_| "no randomness for a form boundary".to_string())?;
    Ok(format!(
        "smoozy-{}",
        bytes.iter().map(|b| format!("{b:02x}")).collect::<String>()
    ))
}

/// **A multipart form, written by hand.** The pinned client's own form support
/// is a feature the workspace pin leaves off, and switching it on would move a
/// pin; a form is a few lines.
fn a_form(fields: &Map<String, Value>, files: &[AFileToSend], boundary: &str) -> Vec<u8> {
    let mut body = Vec::new();
    for (name, value) in fields {
        let text = match value {
            Value::String(text) => text.clone(),
            other => other.to_string(),
        };
        let _ = write!(
            body,
            "--{boundary}\r\nContent-Disposition: form-data; name=\"{}\"\r\n\r\n{text}\r\n",
            safe_in_a_header(name)
        );
    }
    for file in files {
        let _ = write!(
            body,
            "--{boundary}\r\nContent-Disposition: form-data; name=\"{}\"; filename=\"{}\"\r\nContent-Type: application/octet-stream\r\n\r\n",
            safe_in_a_header(&file.part_name),
            safe_in_a_header(&file.file_name)
        );
        body.extend_from_slice(&file.bytes);
        body.extend_from_slice(b"\r\n");
    }
    let _ = write!(body, "--{boundary}--\r\n");
    body
}

/// A name with nothing in it that could end its quotation or its line.
fn safe_in_a_header(name: &str) -> String {
    name.chars()
        .filter(|c| !matches!(c, '"' | '\r' | '\n' | '\\'))
        .collect()
}

// ------------------------------------------------------------ the client

/// **The client every other piece calls.** Cheap to clone; clones share the
/// one connection pool.
#[derive(Clone)]
pub struct TheClient {
    wire: Arc<dyn ReachesTelegram>,
    /// **The longest any call through this handle may take**, where the send
    /// queue has shortened it so the holder's heartbeat is not held back
    /// (`Telegram` C6; [`TheClient::no_call_longer_than`]).
    at_most: Option<Duration>,
    reached: Arc<AtomicU64>,
}

impl fmt::Debug for TheClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("TheClient")
    }
}

/// What every call gives back: the service's `result`, or why not.
pub type WhatTheServiceGaveBack = Result<Value, WhyTheCallFailed>;

impl TheClient {
    /// **The real service, with this token.**
    pub fn reaching_telegram_with(token: &str) -> TheClient {
        TheClient::over(Arc::new(OverHttps::reaching_telegram_with(token)))
    }

    /// **Any wire at all** — a stand-in, in a test.
    pub fn over(wire: Arc<dyn ReachesTelegram>) -> TheClient {
        TheClient { wire, at_most: None, reached: Arc::new(AtomicU64::new(0)) }
    }

    /// **The same client, with no call allowed longer than `window`** — each
    /// call keeps its own bound where that is shorter. Added 2026-09-14 for the
    /// send queue: a file upload may take up to [`THE_BOUND_ON_SENDING_A_FILE`],
    /// and `Telegram` C6 says no report send defers the holder's heartbeat past
    /// the renewal interval. A call cut this way is *not reached*, like any
    /// other that ran out of time.
    pub fn no_call_longer_than(&self, window: Duration) -> TheClient {
        TheClient {
            wire: self.wire.clone(),
            at_most: Some(window),
            reached: self.reached.clone(),
        }
    }

    fn ask(
        &self,
        method: &str,
        carries: WhatACallCarries,
        bound: Duration,
    ) -> WhatTheServiceGaveBack {
        let bound = self.at_most.map_or(bound, |window| bound.min(window));
        let text = self
            .wire
            .call(method, &carries, bound)
            .map_err(WhyTheCallFailed::NotReached)?;
        let answer = what_the_answer_says(&text);
        if answer.is_ok() {
            self.reached.fetch_add(1, Ordering::SeqCst);
        }
        answer
    }

    /// `Telegram` F3a: shared by this worker's handles, never by another server.
    pub fn how_many_calls_reached_telegram(&self) -> u64 {
        self.reached.load(Ordering::SeqCst)
    }

    /// **`Telegram` P6 and P6a — ask for updates, naming the kinds, on every
    /// call.** This is the only ask for updates in the crate.
    ///
    /// `offset` is left out where the caller has confirmed nothing yet, and
    /// then nothing is confirmed by this call. `wait_seconds` is how long the
    /// service may hold the request open; `0` answers at once.
    pub fn get_updates(&self, offset: Option<i64>, wait_seconds: u32) -> WhatTheServiceGaveBack {
        let mut fields = Map::new();
        if let Some(offset) = offset {
            fields.insert("offset".into(), Value::from(offset));
        }
        fields.insert("timeout".into(), Value::from(wait_seconds));
        fields.insert(
            "allowed_updates".into(),
            Value::from(THE_UPDATE_KINDS_EVERY_ASK_NAMES.to_vec()),
        );
        self.ask(
            "getUpdates",
            WhatACallCarries::Fields(fields),
            Duration::from_secs(u64::from(wait_seconds)) + THE_MARGIN_BEYOND_AN_ASKS_OWN_WAIT,
        )
    }

    /// Which bot this token is.
    pub fn get_me(&self) -> WhatTheServiceGaveBack {
        self.ask(
            "getMe",
            WhatACallCarries::Nothing,
            THE_BOUND_ON_AN_ORDINARY_CALL,
        )
    }

    /// One conversation, including its pinned message.
    pub fn get_chat(&self, chat_id: i64) -> WhatTheServiceGaveBack {
        self.ask(
            "getChat",
            fields(json!({ "chat_id": chat_id })),
            THE_BOUND_ON_AN_ORDINARY_CALL,
        )
    }

    /// A text message, with buttons where `buttons` is given — the service's
    /// `reply_markup`.
    pub fn send_message(
        &self,
        chat_id: i64,
        text: &str,
        buttons: Option<Value>,
    ) -> WhatTheServiceGaveBack {
        let mut carried = json!({ "chat_id": chat_id, "text": text });
        with_buttons(&mut carried, buttons);
        self.ask(
            "sendMessage",
            fields(carried),
            THE_BOUND_ON_AN_ORDINARY_CALL,
        )
    }

    /// Replace one message's text, and its buttons where `buttons` is given.
    pub fn edit_message_text(
        &self,
        chat_id: i64,
        message_id: i64,
        text: &str,
        buttons: Option<Value>,
    ) -> WhatTheServiceGaveBack {
        let mut carried = json!({ "chat_id": chat_id, "message_id": message_id, "text": text });
        with_buttons(&mut carried, buttons);
        self.ask(
            "editMessageText",
            fields(carried),
            THE_BOUND_ON_AN_ORDINARY_CALL,
        )
    }

    /// **`Telegram` R1 and U4's call — attach buttons to a message already
    /// sent, changing nothing else about it.** Telegram's page: *"Use this
    /// method to edit only the reply markup of messages."* (`Sources` entry
    /// 45e). A button's record names its message, whose identity exists only
    /// after the send, so a picture or a video is sent without buttons, its
    /// records are written, and only then are its buttons attached here. **The
    /// page does not say which kinds of message it accepts**; a picture's and a
    /// video's are owed to the live pass.
    pub fn edit_message_reply_markup(&self, chat_id: i64, message_id: i64, buttons: Value) -> WhatTheServiceGaveBack {
        self.ask(
            "editMessageReplyMarkup",
            fields(json!({ "chat_id": chat_id, "message_id": message_id, "reply_markup": buttons })),
            THE_BOUND_ON_AN_ORDINARY_CALL,
        )
    }

    /// **`Telegram` E3d — a text message formatted by an entity list and never
    /// by markup**, so nothing in the text needs escaping. `entities` is the
    /// service's list, its positions in UTF-16 code units. Added for a
    /// failure's words read in the chat.
    pub fn send_message_with_entities(&self, chat_id: i64, text: &str, entities: Value) -> WhatTheServiceGaveBack {
        self.ask(
            "sendMessage",
            fields(json!({ "chat_id": chat_id, "text": text, "entities": entities })),
            THE_BOUND_ON_AN_ORDINARY_CALL,
        )
    }

    /// `Telegram` R7a: user text and its ranges, with optional buttons.
    pub fn send_marked_message(&self, chat: i64, text: &crate::job_messages::AMarkedText, buttons: Option<Value>) -> WhatTheServiceGaveBack {
        let mut carried = json!({"chat_id": chat, "text": text.text, "entities": text.entities()});
        with_buttons(&mut carried, buttons);
        self.ask("sendMessage", fields(carried), THE_BOUND_ON_AN_ORDINARY_CALL)
    }
    pub fn edit_marked_message(&self, chat: i64, id: i64, text: &crate::job_messages::AMarkedText, buttons: Option<Value>) -> WhatTheServiceGaveBack {
        let mut carried = json!({"chat_id": chat, "message_id": id, "text": text.text, "entities": text.entities()});
        with_buttons(&mut carried, buttons);
        self.ask("editMessageText", fields(carried), THE_BOUND_ON_AN_ORDINARY_CALL)
    }

    /// **`Telegram` B24 — a machinery message, sent silently**: a board (B2)
    /// or one machine's machinery message (B10, with B15's proposals), each
    /// *"sent with `disable_notification`"*. Anything composed for the user
    /// goes through [`TheClient::send_message`] and keeps its notification
    /// (B24). **This client passes no such setting on an edit**: B24's body
    /// names sends and pins, and no source this project records says an edit
    /// takes one. The build's reasons are in
    /// `Audits/Build/Telegram/Decisions record.md`.
    ///
    /// **`Telegram` B25 — everything after the head is a spoiler**, sent as an
    /// entity list and never as markup; where it goes is
    /// [`crate::board::text::the_spoiler_after_the_head`].
    pub fn send_a_machinery_message(&self, chat_id: i64, text: &str) -> WhatTheServiceGaveBack {
        let mut carried = json!({ "chat_id": chat_id, "text": text, "disable_notification": true });
        with_the_spoiler(&mut carried, text);
        self.ask(
            "sendMessage",
            fields(carried),
            THE_BOUND_ON_AN_ORDINARY_CALL,
        )
    }

    /// **`Telegram` B25 — a board or machinery message edited, with its
    /// spoiler sent again**: *"The marking is sent again with every edit"*,
    /// because Telegram's pages do not say an edit keeps it. Carries no
    /// silence setting, as [`TheClient::send_a_machinery_message`] says. An edit
    /// of anything composed for the user goes through
    /// [`TheClient::edit_message_text`] and carries no spoiler.
    pub fn edit_a_machinery_message(&self, chat_id: i64, message_id: i64, text: &str) -> WhatTheServiceGaveBack {
        let mut carried = json!({ "chat_id": chat_id, "message_id": message_id, "text": text });
        with_the_spoiler(&mut carried, text);
        self.ask(
            "editMessageText",
            fields(carried),
            THE_BOUND_ON_AN_ORDINARY_CALL,
        )
    }

    /// **Pin one message, always silently — `Telegram` B24: *"every pin is made
    /// with it"*.** Only the board and machinery messages are ever pinned
    /// (B16). B19 and §21 item 17 record that `disable_notification` silences
    /// the notification and nothing else: the line in the conversation is
    /// written either way. **B3 and B10 still say the notification is off in a
    /// private chat whatever the call passes** (`Sources` entry 41, Telegram's
    /// own page); the Creator reported the buzzing, and B24 was written.
    pub fn pin_chat_message(&self, chat_id: i64, message_id: i64) -> WhatTheServiceGaveBack {
        self.ask(
            "pinChatMessage",
            fields(json!({ "chat_id": chat_id, "message_id": message_id, "disable_notification": true })),
            THE_BOUND_ON_AN_ORDINARY_CALL,
        )
    }

    /// **Unpin one message, by its identity.** There is no unpin without one in
    /// this client: `Telegram` B3 never uses it, and a bare unpin removes
    /// whichever pin is newest, which may be another machine's board.
    pub fn unpin_chat_message(&self, chat_id: i64, message_id: i64) -> WhatTheServiceGaveBack {
        self.ask(
            "unpinChatMessage",
            fields(json!({ "chat_id": chat_id, "message_id": message_id })),
            THE_BOUND_ON_AN_ORDINARY_CALL,
        )
    }

    /// **`Telegram` B18 — delete the line one pin wrote, by that line's own
    /// identity.** The service message a pin of Smoozy's own puts in the
    /// conversation, once the holder has processed that pin. **Never a pinned
    /// message**: the listener is the only caller, and it hands in the identity
    /// of a pin update's own service message, never the identity of what was
    /// pinned. A refusal is the caller's to read.
    pub fn delete_a_pins_line(&self, chat_id: i64, line_id: i64) -> WhatTheServiceGaveBack {
        self.delete_one_message(chat_id, line_id)
    }

    /// **`Telegram` C1b's line of 2026-11-02 and B18's second deletion: an
    /// album's companion, once its job has ended**, by its own identity. The
    /// bot's own message, never pinned, and with nothing left to say once its
    /// one button has gone. [`crate::actions::TheJobMessages`] is the only
    /// caller. A refusal is the caller's to read.
    pub fn delete_an_albums_companion(&self, chat_id: i64, message_id: i64) -> WhatTheServiceGaveBack {
        self.delete_one_message(chat_id, message_id)
    }

    /// **`deleteMessage`, written once.** It takes service messages, lets a bot
    /// delete its own in a private chat, and refuses one sent 48 hours or more
    /// before (`Sources` entry 87).
    fn delete_one_message(&self, chat_id: i64, message_id: i64) -> WhatTheServiceGaveBack {
        self.ask(
            "deleteMessage",
            fields(json!({ "chat_id": chat_id, "message_id": message_id })),
            THE_BOUND_ON_AN_ORDINARY_CALL,
        )
    }

    /// A picture, uploaded.
    pub fn send_photo(
        &self,
        chat_id: i64,
        file_name: &str,
        bytes: Vec<u8>,
        caption: Option<&str>,
        buttons: Option<Value>,
    ) -> WhatTheServiceGaveBack {
        self.send_one_file(
            "sendPhoto",
            "photo",
            chat_id,
            file_name,
            bytes,
            caption,
            buttons,
        )
    }

    /// A file, uploaded as a document.
    pub fn send_document(
        &self,
        chat_id: i64,
        file_name: &str,
        bytes: Vec<u8>,
        caption: Option<&str>,
        buttons: Option<Value>,
    ) -> WhatTheServiceGaveBack {
        self.send_one_file(
            "sendDocument",
            "document",
            chat_id,
            file_name,
            bytes,
            caption,
            buttons,
        )
    }

    /// A moving picture, uploaded.
    pub fn send_video(
        &self,
        chat_id: i64,
        file_name: &str,
        bytes: Vec<u8>,
        caption: Option<&str>,
        buttons: Option<Value>,
    ) -> WhatTheServiceGaveBack {
        self.send_one_file(
            "sendVideo",
            "video",
            chat_id,
            file_name,
            bytes,
            caption,
            buttons,
        )
    }

    pub fn send_photo_marked(&self, chat: i64, name: &str, bytes: Vec<u8>, caption: Option<&crate::job_messages::AMarkedText>, buttons: Option<Value>) -> WhatTheServiceGaveBack {
        self.send_one_marked_file("sendPhoto", "photo", chat, name, bytes, caption, buttons)
    }
    pub fn send_video_marked(&self, chat: i64, name: &str, bytes: Vec<u8>, caption: Option<&crate::job_messages::AMarkedText>, buttons: Option<Value>) -> WhatTheServiceGaveBack {
        self.send_one_marked_file("sendVideo", "video", chat, name, bytes, caption, buttons)
    }
    pub fn send_document_marked(&self, chat: i64, name: &str, bytes: Vec<u8>, caption: Option<&crate::job_messages::AMarkedText>, buttons: Option<Value>) -> WhatTheServiceGaveBack {
        self.send_one_marked_file("sendDocument", "document", chat, name, bytes, caption, buttons)
    }

    fn send_one_marked_file(&self, method: &'static str, part: &str, chat: i64, name: &str, bytes: Vec<u8>, caption: Option<&crate::job_messages::AMarkedText>, buttons: Option<Value>) -> WhatTheServiceGaveBack {
        let bound = the_bound_on_sending(bytes.len());
        let mut carried = json!({"chat_id": chat, part: format!("attach://{part}")});
        if let Some(caption) = caption { carried["caption"] = json!(caption.text); carried["caption_entities"] = caption.entities(); }
        with_buttons(&mut carried, buttons);
        let WhatACallCarries::Fields(fields) = fields(carried) else { unreachable!() };
        self.ask(method, WhatACallCarries::FieldsAndFiles { fields, files: vec![AFileToSend { part_name: part.into(), file_name: name.into(), bytes }] }, bound)
    }

    /// **An album of pictures**, each uploaded and referred to by its part
    /// name. The service takes two to ten; how many to send is the caller's.
    pub fn send_media_group(
        &self,
        chat_id: i64,
        pictures: Vec<(String, Vec<u8>)>,
    ) -> WhatTheServiceGaveBack {
        self.send_an_album(chat_id, pictures.into_iter().map(|(name, bytes)| (name, bytes, None)).collect())
    }

    /// **An album, each picture with its own caption where one is given** —
    /// added at step 10 piece 6, because `Telegram` R7 has every message name
    /// its server and job, and an album's only words are the captions on its
    /// pictures (the service's `InputMediaPhoto` has `caption`). The contact
    /// sheet (`Telegram` M2) captions its first picture alone. **Since
    /// 2026-10-18 a frame of several View Layers is an album too, and each of
    /// its pictures carries its own caption naming its layer** (`Telegram` I2,
    /// the Creator's fourteenth notes, note 7; `Sources` entry 88:
    /// `InputMediaPhoto.caption`, *"0-1024 characters after entities
    /// parsing"*). **An album takes no buttons** (`sendMediaGroup` lists no
    /// `reply_markup`), and keeps its notification: it is written for the user
    /// (B24).
    pub fn send_an_album(
        &self,
        chat_id: i64,
        pictures: Vec<(String, Vec<u8>, Option<String>)>,
    ) -> WhatTheServiceGaveBack {
        // C6b: the time limit grows with the whole album.
        let all_the_bytes: usize = pictures.iter().map(|(_, bytes, _)| bytes.len()).sum();
        let mut media = Vec::new();
        let mut files = Vec::new();
        for (at, (file_name, bytes, caption)) in pictures.into_iter().enumerate() {
            let part_name = format!("picture{at}");
            let mut one = json!({ "type": "photo", "media": format!("attach://{part_name}") });
            if let Some(caption) = caption {
                one["caption"] = Value::from(caption);
            }
            media.push(one);
            files.push(AFileToSend {
                part_name,
                file_name,
                bytes,
            });
        }
        let carried = fields(json!({ "chat_id": chat_id, "media": media }));
        let WhatACallCarries::Fields(fields) = carried else {
            unreachable!("fields() always gives fields")
        };
        self.ask(
            "sendMediaGroup",
            WhatACallCarries::FieldsAndFiles { fields, files },
            the_bound_on_sending(all_the_bytes),
        )
    }

    pub fn send_a_marked_album(
        &self,
        chat_id: i64,
        pictures: Vec<(String, Vec<u8>, Option<crate::job_messages::AMarkedText>)>,
    ) -> WhatTheServiceGaveBack {
        // C6b: the time limit grows with the whole album.
        let all_the_bytes: usize = pictures.iter().map(|(_, bytes, _)| bytes.len()).sum();
        let mut media = Vec::new();
        let mut files = Vec::new();
        for (at, (file_name, bytes, caption)) in pictures.into_iter().enumerate() {
            let part_name = format!("picture{at}");
            let mut one = json!({ "type": "photo", "media": format!("attach://{part_name}") });
            if let Some(caption) = caption {
                one["caption"] = json!(caption.text);
                one["caption_entities"] = caption.entities();
            }
            media.push(one);
            files.push(AFileToSend {
                part_name,
                file_name,
                bytes,
            });
        }
        let carried = fields(json!({ "chat_id": chat_id, "media": media }));
        let WhatACallCarries::Fields(fields) = carried else {
            unreachable!("fields() always gives fields")
        };
        self.ask(
            "sendMediaGroup",
            WhatACallCarries::FieldsAndFiles { fields, files },
            the_bound_on_sending(all_the_bytes),
        )
    }

    /// **`Telegram` R16 — answer one tap on a button, with nothing but its
    /// identifier.** Telegram's `answerCallbackQuery` shows the user nothing
    /// when no text is given, and clears the waiting mark on the button.
    /// **This call carries no text, no alert, no link and no cache time, and
    /// has no parameter for any of them**, because R16 answers with no text
    /// and everything the user is told stays in the messages the plan already
    /// sends (`Audits/Build/Step 10/Answering a button tap — what Telegram's
    /// pages say.md`, section 3).
    pub fn answer_callback_query(&self, callback_query_id: &str) -> WhatTheServiceGaveBack {
        self.ask(
            "answerCallbackQuery",
            fields(json!({ "callback_query_id": callback_query_id })),
            THE_BOUND_ON_AN_ORDINARY_CALL,
        )
    }

    /// **The webhook state and the standing filter of update kinds** — the only
    /// place the filter P6 is written against can be seen.
    pub fn get_webhook_info(&self) -> WhatTheServiceGaveBack {
        self.ask(
            "getWebhookInfo",
            WhatACallCarries::Nothing,
            THE_BOUND_ON_AN_ORDINARY_CALL,
        )
    }

    /// **`Telegram` R17 — the bot's command list**, for the default scope and
    /// no language code (Sources entry 89: `getMyCommands`).
    pub fn get_my_commands(&self) -> WhatTheServiceGaveBack {
        self.ask("getMyCommands", WhatACallCarries::Nothing, THE_BOUND_ON_AN_ORDINARY_CALL)
    }

    /// **`Telegram` R17 — the bot's command list set**, for the default scope
    /// and no language code (Sources entry 89: `setMyCommands`, its
    /// `commands` an array of `BotCommand`).
    pub fn set_my_commands(&self, commands: Value) -> WhatTheServiceGaveBack {
        self.ask("setMyCommands", fields(json!({ "commands": commands })), THE_BOUND_ON_AN_ORDINARY_CALL)
    }

    /// **Remove a webhook registration — `Telegram` J3's removal.** It never
    /// passes `drop_pending_updates` (J2, J3), and nothing may be read from the
    /// answer's description (J3's added paragraph): the readings on either side
    /// of the call decide.
    pub fn delete_webhook(&self) -> WhatTheServiceGaveBack {
        self.ask(
            "deleteWebhook",
            WhatACallCarries::Nothing,
            THE_BOUND_ON_AN_ORDINARY_CALL,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn send_one_file(
        &self,
        method: &str,
        part_name: &str,
        chat_id: i64,
        file_name: &str,
        bytes: Vec<u8>,
        caption: Option<&str>,
        buttons: Option<Value>,
    ) -> WhatTheServiceGaveBack {
        // C6b: the time limit grows with the file.
        let the_bound = the_bound_on_sending(bytes.len());
        let mut carried = json!({ "chat_id": chat_id, part_name: format!("attach://{part_name}") });
        if let Some(caption) = caption {
            carried["caption"] = Value::from(caption);
        }
        with_buttons(&mut carried, buttons);
        let WhatACallCarries::Fields(fields) = fields(carried) else {
            unreachable!("fields() always gives fields")
        };
        self.ask(
            method,
            WhatACallCarries::FieldsAndFiles {
                fields,
                files: vec![AFileToSend {
                    part_name: part_name.to_string(),
                    file_name: file_name.to_string(),
                    bytes,
                }],
            },
            the_bound,
        )
    }
}

fn fields(object: Value) -> WhatACallCarries {
    match object {
        Value::Object(map) => WhatACallCarries::Fields(map),
        _ => WhatACallCarries::Fields(Map::new()),
    }
}

/// **`Telegram` B25 — the spoiler as the service's `entities` field**: one
/// `spoiler` entity, its start and length in UTF-16 code units. A text with
/// nothing after its head carries none.
fn with_the_spoiler(carried: &mut Value, text: &str) {
    if let Some((offset, length)) = crate::board::text::the_spoiler_after_the_head(text) {
        carried["entities"] = json!([{ "type": "spoiler", "offset": offset, "length": length }]);
    }
}

fn with_buttons(carried: &mut Value, buttons: Option<Value>) {
    if let Some(buttons) = buttons {
        carried["reply_markup"] = buttons;
    }
}

// ------------------------------------------------------------ the stand-in

/// **A stand-in for Telegram's service**, for this crate's tests and every later
/// piece's. It hears every call and answers from a list it is given, and
/// accepts with `true` once the list runs out. It holds no token.
pub mod a_stand_in {
    use super::*;

    /// One call the stand-in heard.
    #[derive(Debug, Clone, PartialEq)]
    pub struct ACallItHeard {
        pub method: String,
        pub carries: WhatACallCarries,
        pub bound: Duration,
    }

    #[derive(Default)]
    pub struct AStandInService {
        answers: Mutex<VecDeque<Result<String, String>>>,
        heard: Mutex<Vec<ACallItHeard>>,
    }

    impl AStandInService {
        pub fn new() -> AStandInService {
            AStandInService::default()
        }

        /// The next call is answered with this text.
        pub fn then_answers(&self, text: &str) -> &AStandInService {
            lock(&self.answers).push_back(Ok(text.to_string()));
            self
        }

        /// The next call is not reached, for this reason.
        pub fn then_is_not_reached(&self, why: &str) -> &AStandInService {
            lock(&self.answers).push_back(Err(why.to_string()));
            self
        }

        /// Every call heard, in order.
        pub fn heard(&self) -> Vec<ACallItHeard> {
            lock(&self.heard).clone()
        }
    }

    impl ReachesTelegram for AStandInService {
        fn call(
            &self,
            method: &str,
            carries: &WhatACallCarries,
            bound: Duration,
        ) -> Result<String, String> {
            lock(&self.heard).push(ACallItHeard {
                method: method.to_string(),
                carries: carries.clone(),
                bound,
            });
            lock(&self.answers)
                .pop_front()
                .unwrap_or_else(|| Ok(r#"{"ok":true,"result":true}"#.to_string()))
        }
    }

    fn lock<T>(held: &Mutex<T>) -> std::sync::MutexGuard<'_, T> {
        held.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
    }
}

#[cfg(test)]
mod tests {
    use super::a_stand_in::AStandInService;
    use super::*;
    use std::path::{Path, PathBuf};

    #[test]
    fn twenty_first_marked_sends_and_edits_keep_ranges_on_text_and_every_media_kind() {
        let (service, client) = a_client();
        let name = crate::job_messages::AServerLabel::with_colour("Сервер 🎬", Some("gold"));
        let text = crate::job_messages::a_user_message(&name, "台所", " · Frame 147", "Started.");
        client.send_marked_message(42, &text, None).unwrap();
        client.edit_marked_message(42, 7, &text, Some(json!({"inline_keyboard":[]}))).unwrap();
        client.send_photo_marked(42, "p.png", vec![1], Some(&text), None).unwrap();
        client.send_video_marked(42, "v.mp4", vec![2], Some(&text), None).unwrap();
        client.send_document_marked(42, "w.txt", vec![3], Some(&text), None).unwrap();
        client.send_a_marked_album(42, vec![("a.png".into(), vec![4], Some(text.clone())), ("b.png".into(), vec![5], None)]).unwrap();
        let machinery = "Smoozy — server coordination · Сервер\n\nmachinery";
        client.send_a_machinery_message(42, machinery).unwrap();
        let heard = service.heard();
        for (i, call) in heard[..6].iter().enumerate() {
            let carried = match &call.carries { WhatACallCarries::Fields(f) | WhatACallCarries::FieldsAndFiles { fields: f, .. } => f, WhatACallCarries::Nothing => panic!("a user send has fields") };
            assert!(carried.get("parse_mode").is_none());
            if i < 2 { assert_eq!(carried["entities"], text.entities()); assert_eq!(carried["text"], text.text); }
            else if i < 5 { assert_eq!(carried["caption_entities"], text.entities()); assert_eq!(carried["caption"], text.text); let kind = ["photo", "video", "document"][i - 2]; assert_eq!(carried[kind], format!("attach://{kind}")); }
            else { assert_eq!(carried["media"][0]["caption_entities"], text.entities()); assert!(carried["media"][1].get("caption_entities").is_none()); }
        }
        let WhatACallCarries::Fields(fields) = &heard[6].carries else { panic!() };
        assert_eq!(fields["text"], machinery);
        assert_eq!(fields["entities"].as_array().unwrap().len(), 1);
        assert_eq!(fields["entities"][0]["type"], "spoiler");
        assert_eq!(fields["disable_notification"], true);
    }

    fn a_client() -> (Arc<AStandInService>, TheClient) {
        let service = Arc::new(AStandInService::new());
        (service.clone(), TheClient::over(service))
    }

    fn the_fields_of(carries: &WhatACallCarries) -> &Map<String, Value> {
        match carries {
            WhatACallCarries::Fields(fields) => fields,
            WhatACallCarries::FieldsAndFiles { fields, .. } => fields,
            WhatACallCarries::Nothing => panic!("this call carried nothing"),
        }
    }

    #[test]
    fn twenty_first_reaches_are_shared_by_clones_and_never_by_another_worker() {
        let service = Arc::new(a_stand_in::AStandInService::new());
        let client = TheClient::over(service.clone());
        let clone = client.no_call_longer_than(Duration::from_secs(2));
        clone.get_me().unwrap();
        assert_eq!(client.how_many_calls_reached_telegram(), 1);
        service.then_is_not_reached("down");
        assert!(clone.get_me().is_err());
        service.then_answers(r#"{"ok":false,"error_code":429,"parameters":{"retry_after":3}}"#);
        assert!(clone.get_me().is_err());
        assert_eq!(client.how_many_calls_reached_telegram(), 1, "neither a failed call nor its own flood answer spends a try");
        let another_worker = TheClient::over(service);
        another_worker.get_me().unwrap();
        assert_eq!(client.how_many_calls_reached_telegram(), 1);
        assert_eq!(another_worker.how_many_calls_reached_telegram(), 1);
    }

    /// **`Telegram` P6a: the list is exactly these two, in this one place.**
    #[test]
    fn the_kinds_are_exactly_message_and_callback_query() {
        assert_eq!(
            THE_UPDATE_KINDS_EVERY_ASK_NAMES,
            ["message", "callback_query"]
        );
    }

    /// **`Telegram` P6: every ask names the kinds**, whether or not it confirms
    /// anything and however long it waits.
    #[test]
    fn every_ask_for_updates_names_the_kinds() {
        let (service, client) = a_client();
        client.get_updates(None, 0).unwrap();
        client.get_updates(Some(812_345_678), 25).unwrap();

        let heard = service.heard();
        assert_eq!(heard.len(), 2);
        for call in &heard {
            assert_eq!(call.method, "getUpdates");
            assert_eq!(
                the_fields_of(&call.carries)["allowed_updates"],
                json!(["message", "callback_query"])
            );
        }
        assert!(!the_fields_of(&heard[0].carries).contains_key("offset"));
        assert_eq!(
            the_fields_of(&heard[1].carries)["offset"],
            json!(812_345_678)
        );
        assert_eq!(heard[1].bound, Duration::from_secs(55));
    }

    /// **The test the brief asks for: it fails if any code in this crate builds
    /// its own list of update kinds, or asks for updates any other way.**
    ///
    /// It reads the working half of every Rust file under `src/` — the text
    /// before the file's own test module — so a later piece's file is read the
    /// moment it exists. Three things must hold across all of them:
    ///
    /// - **the service's field name for the kinds is written at one site**, and
    ///   that site reads [`THE_UPDATE_KINDS_EVERY_ASK_NAMES`];
    /// - **the ask's method name is written at one site**;
    /// - **no line names both kinds**, except the constant's own.
    ///
    /// And the registration call is named nowhere, because `Telegram` J1, J2
    /// and J4 are not built and a registration also carries the kinds.
    #[test]
    fn no_call_builds_its_own_list_of_update_kinds() {
        let mut the_field = Vec::new();
        let mut the_ask = Vec::new();
        let mut both_kinds = Vec::new();
        for (path, working) in the_working_half_of_every_file_under_src() {
            let lines: Vec<&str> = working.lines().collect();
            for (at, line) in lines.iter().enumerate() {
                let site = format!("{}:{}", path.display(), at + 1);
                if line.contains("\"allowed_updates\"") {
                    // **The whole statement, not the one line**: formatting
                    // may put the list it reads on the line after the name.
                    let mut statement = String::new();
                    for next in &lines[at..] {
                        statement.push_str(next);
                        if next.contains(';') {
                            break;
                        }
                    }
                    the_field.push((site.clone(), statement));
                }
                if line.contains("\"getUpdates\"") {
                    the_ask.push(site.clone());
                }
                if line.contains("\"message\"") && line.contains("\"callback_query\"") {
                    both_kinds.push(site.clone());
                }
                assert!(
                    !line.contains("\"setWebhook\""),
                    "{site} names the registration call, which is not built and would carry a list of kinds"
                );
            }
        }
        assert_eq!(
            the_field.len(),
            1,
            "the kinds' field is written at {the_field:?}; it must be one site"
        );
        assert!(
            the_field[0].1.contains("THE_UPDATE_KINDS_EVERY_ASK_NAMES"),
            "the one site naming the kinds' field does not read the one list: {}",
            the_field[0].0
        );
        assert_eq!(
            the_ask.len(),
            1,
            "updates are asked for at {the_ask:?}; it must be one site"
        );
        assert_eq!(
            both_kinds.len(),
            1,
            "both kinds are named together at {both_kinds:?}; only the constant may"
        );
    }

    /// **`Telegram` B18 — the one deletion is a pin's line, asked for by
    /// identity, and only the listener asks.** The call names the line and the
    /// conversation and nothing else; Telegram's method is written at one site
    /// in the crate, and the listener is the one file that calls it.
    #[test]
    fn the_two_deletions_are_each_by_their_identity_and_each_has_one_caller() {
        let (service, client) = a_client();
        client.delete_a_pins_line(42, 50_007).unwrap();
        let heard = service.heard();
        assert_eq!(heard[0].method, "deleteMessage");
        assert_eq!(Value::Object(the_fields_of(&heard[0].carries).clone()), json!({ "chat_id": 42, "message_id": 50_007 }));

        let mut the_method = Vec::new();
        let mut the_callers = Vec::new();
        let mut the_companions_callers = Vec::new();
        for (path, working) in the_working_half_of_every_file_under_src() {
            let name = path.file_name().and_then(|n| n.to_str()).unwrap_or_default().to_string();
            for (at, line) in working.lines().enumerate() {
                if line.contains("\"deleteMessage\"") {
                    the_method.push(format!("{name}:{}", at + 1));
                }
                if line.contains(".delete_a_pins_line(") {
                    the_callers.push(name.clone());
                }
                if line.contains(".delete_an_albums_companion(") {
                    the_companions_callers.push(name.clone());
                }
            }
        }
        assert!(
            the_method.len() == 1 && the_method[0].starts_with("transport.rs:"),
            "deleteMessage is written at {the_method:?}; it must be one site, here"
        );
        the_callers.dedup();
        assert_eq!(the_callers, ["listener.rs"], "a pin's line is deleted from {the_callers:?}");
        assert_eq!(the_companions_callers, ["actions.rs"], "a companion is deleted from {the_companions_callers:?}");
    }

    fn the_working_half_of_every_file_under_src() -> Vec<(PathBuf, String)> {
        let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let mut out = Vec::new();
        let mut to_read = vec![src];
        while let Some(folder) = to_read.pop() {
            for entry in std::fs::read_dir(&folder).expect("src is readable") {
                let path = entry.expect("an entry is readable").path();
                if path.is_dir() {
                    to_read.push(path);
                } else if path.extension().and_then(|e| e.to_str()) == Some("rs") {
                    let whole = std::fs::read_to_string(&path).expect("a source file is readable");
                    let working = match whole.split_once("\n#[cfg(test)]") {
                        Some((working, _)) => working.to_string(),
                        None => whole,
                    };
                    out.push((path, working));
                }
            }
        }
        assert!(out.len() >= 3, "this crate's own files were not found");
        out
    }

    /// **`Telegram` J2 and J3: the removal never drops the user's queued
    /// messages.**
    #[test]
    fn the_webhook_removal_never_drops_pending_updates() {
        let (service, client) = a_client();
        client.delete_webhook().unwrap();
        let heard = service.heard();
        assert_eq!(heard[0].method, "deleteWebhook");
        assert_eq!(heard[0].carries, WhatACallCarries::Nothing);
    }

    #[test]
    fn an_unpin_always_names_the_message() {
        let (service, client) = a_client();
        client.unpin_chat_message(42, 7).unwrap();
        let heard = service.heard();
        assert_eq!(heard[0].method, "unpinChatMessage");
        assert_eq!(the_fields_of(&heard[0].carries)["message_id"], json!(7));
        assert_eq!(the_fields_of(&heard[0].carries)["chat_id"], json!(42));
    }

    /// **`Telegram` B24: every pin is silent.**
    #[test]
    fn a_pin_is_always_silent() {
        let (service, client) = a_client();
        client.pin_chat_message(42, 7).unwrap();
        let heard = service.heard();
        assert_eq!(heard[0].method, "pinChatMessage");
        assert_eq!(the_fields_of(&heard[0].carries)["message_id"], json!(7));
        assert_eq!(the_fields_of(&heard[0].carries)["disable_notification"], json!(true));
    }

    /// **`Telegram` B24: a machinery message is sent silently; a message
    /// composed for the user keeps its notification.**
    #[test]
    fn a_machinery_message_is_silent_and_a_message_for_the_user_is_not() {
        let (service, client) = a_client();
        client.send_a_machinery_message(42, "a board").unwrap();
        client.send_message(42, "for the user", None).unwrap();
        let heard = service.heard();
        assert_eq!(heard[0].method, "sendMessage");
        assert_eq!(the_fields_of(&heard[0].carries)["disable_notification"], json!(true));
        assert!(!the_fields_of(&heard[1].carries).contains_key("disable_notification"));
    }

    /// **`Telegram` B25: a board or machinery message is sent and edited with
    /// its spoiler after the head; a message for the user is sent and edited
    /// with none.** Each call is put through the stand-ins' own check.
    #[test]
    fn a_machinery_message_carries_its_spoiler_on_every_send_and_edit_and_a_message_for_the_user_does_not() {
        use crate::board::text::tests::{a_board, the_spoiler_is_right};
        let (service, client) = a_client();
        let board = a_board("m1").compose().unwrap();
        client.send_a_machinery_message(42, &board).unwrap();
        client.edit_a_machinery_message(42, 7, &board).unwrap();
        client.send_message(42, "Which job?", None).unwrap();
        client.edit_message_text(42, 8, "Which job?", Some(json!({ "inline_keyboard": [] }))).unwrap();
        let heard = service.heard();
        assert_eq!(
            heard.iter().map(|c| c.method.as_str()).collect::<Vec<_>>(),
            ["sendMessage", "editMessageText", "sendMessage", "editMessageText"]
        );
        for call in &heard {
            the_spoiler_is_right(&call.method, &Value::Object(the_fields_of(&call.carries).clone()));
        }
        assert_eq!(the_fields_of(&heard[0].carries)["entities"][0]["type"], json!("spoiler"));
        assert_eq!(the_fields_of(&heard[0].carries)["entities"], the_fields_of(&heard[1].carries)["entities"]);
        assert!(!the_fields_of(&heard[1].carries).contains_key("disable_notification"));
        assert!(!the_fields_of(&heard[2].carries).contains_key("entities"));
        assert!(!the_fields_of(&heard[3].carries).contains_key("entities"));
    }

    #[test]
    fn a_message_carries_its_buttons_only_when_given() {
        let (service, client) = a_client();
        client.send_message(42, "one", None).unwrap();
        client
            .send_message(42, "two", Some(json!({ "inline_keyboard": [] })))
            .unwrap();
        let heard = service.heard();
        assert!(!the_fields_of(&heard[0].carries).contains_key("reply_markup"));
        assert_eq!(
            the_fields_of(&heard[1].carries)["reply_markup"],
            json!({ "inline_keyboard": [] })
        );
    }

    #[test]
    fn an_album_refers_to_each_upload_by_its_part_name() {
        let (service, client) = a_client();
        client
            .send_media_group(
                42,
                vec![("a.jpg".into(), vec![1]), ("b.jpg".into(), vec![2, 2])],
            )
            .unwrap();
        let heard = service.heard();
        let WhatACallCarries::FieldsAndFiles { fields, files } = &heard[0].carries else {
            panic!("an album uploads files");
        };
        assert_eq!(
            fields["media"],
            json!([
                { "type": "photo", "media": "attach://picture0" },
                { "type": "photo", "media": "attach://picture1" }
            ])
        );
        assert_eq!(files[0].part_name, "picture0");
        assert_eq!(files[1].bytes, vec![2, 2]);
        assert_eq!(heard[0].bound, THE_BOUND_ON_SENDING_A_FILE);
    }

    /// **`Telegram` I2 as written 2026-10-18 (the fourteenth notes, note 7):
    /// each picture of an album carries its own caption**, where one is given;
    /// no buttons and no silence are passed (`Sources` entry 88; B24).
    #[test]
    fn each_picture_of_an_album_carries_its_own_caption() {
        let (service, client) = a_client();
        client
            .send_an_album(
                42,
                vec![
                    ("1.png".into(), vec![1], Some("frame 1 \u{b7} Interior".into())),
                    ("1-layer-1.png".into(), vec![2], Some("frame 1 \u{b7} Reflections".into())),
                    ("1-layer-2.png".into(), vec![3], None),
                ],
            )
            .unwrap();
        let heard = service.heard();
        let WhatACallCarries::FieldsAndFiles { fields, .. } = &heard[0].carries else {
            panic!("an album uploads files");
        };
        assert_eq!(heard[0].method, "sendMediaGroup");
        assert_eq!(
            fields["media"],
            json!([
                { "type": "photo", "media": "attach://picture0", "caption": "frame 1 \u{b7} Interior" },
                { "type": "photo", "media": "attach://picture1", "caption": "frame 1 \u{b7} Reflections" },
                { "type": "photo", "media": "attach://picture2" }
            ])
        );
        assert!(!fields.contains_key("reply_markup") && !fields.contains_key("disable_notification"));
    }

    #[test]
    fn a_single_upload_is_attached_under_its_own_kind() {
        let (service, client) = a_client();
        client
            .send_photo(42, "frame.jpg", vec![9], Some("frame 12"), None)
            .unwrap();
        let heard = service.heard();
        let WhatACallCarries::FieldsAndFiles { fields, files } = &heard[0].carries else {
            panic!("a photo uploads a file");
        };
        assert_eq!(heard[0].method, "sendPhoto");
        assert_eq!(fields["photo"], json!("attach://photo"));
        assert_eq!(fields["caption"], json!("frame 12"));
        assert_eq!(files[0].part_name, "photo");
    }

    /// The refusal this project recorded on registration, word for word, as
    /// `Telegram` §10's Constraint quotes it.
    #[test]
    fn a_flood_refusal_is_read_with_its_wait() {
        assert_eq!(
            what_the_answer_says(
                r#"{"ok":false,"error_code":429,"description":"Too Many Requests: retry after 1","parameters":{"retry_after":1}}"#
            ),
            Err(WhyTheCallFailed::Refused(ARefusal {
                error_code: Some(429),
                description: Some("Too Many Requests: retry after 1".into()),
                retry_after: Some(1),
            }))
        );
    }

    #[test]
    fn answers_are_read_by_the_published_shape_and_nothing_else() {
        assert_eq!(
            what_the_answer_says(r#"{"ok":true,"result":[]}"#),
            Ok(json!([]))
        );
        assert_eq!(
            what_the_answer_says(r#"{"ok":false,"error_code":400,"description":"Bad Request"}"#),
            Err(WhyTheCallFailed::Refused(ARefusal {
                error_code: Some(400),
                description: Some("Bad Request".into()),
                retry_after: None,
            }))
        );
        assert!(matches!(
            what_the_answer_says(r#"{"ok":true}"#),
            Err(WhyTheCallFailed::TheAnswerCouldNotBeRead(_))
        ));
        assert!(matches!(
            what_the_answer_says(r#"{"result":true}"#),
            Err(WhyTheCallFailed::TheAnswerCouldNotBeRead(_))
        ));
        assert!(matches!(
            what_the_answer_says("<html>502</html>"),
            Err(WhyTheCallFailed::TheAnswerCouldNotBeRead(_))
        ));
    }

    /// **The token never leaves in an error or a debug form.** The address is
    /// made unusable on purpose, so the client refuses it before any
    /// connection, and its complaint carries the address, token and all.
    #[test]
    fn the_token_is_taken_out_of_every_error_and_never_shown() {
        let token = "123456:a-token-that-is-not-real";
        let wire = OverHttps::reaching("https://not a host", token, false);
        assert!(!format!("{wire:?}").contains(token));

        let why = wire
            .call("getMe", &WhatACallCarries::Nothing, Duration::from_secs(1))
            .expect_err("an address with a space in it is refused");
        assert!(
            !why.contains("a-token-that-is-not-real"),
            "the token reached an error: {why}"
        );
    }

    #[test]
    fn a_form_is_written_the_way_a_browser_uploads_a_file() {
        let mut fields = Map::new();
        fields.insert("chat_id".into(), json!(42));
        fields.insert("caption".into(), json!("a \"quoted\" word"));
        let files = vec![AFileToSend {
            part_name: "photo".into(),
            file_name: "fr\"ame.jpg".into(),
            bytes: vec![0xff, 0xd8],
        }];
        let body = a_form(&fields, &files, "B");
        let mut expected = Vec::new();
        expected.extend_from_slice(
            b"--B\r\nContent-Disposition: form-data; name=\"caption\"\r\n\r\na \"quoted\" word\r\n\
              --B\r\nContent-Disposition: form-data; name=\"chat_id\"\r\n\r\n42\r\n\
              --B\r\nContent-Disposition: form-data; name=\"photo\"; filename=\"frame.jpg\"\r\nContent-Type: application/octet-stream\r\n\r\n",
        );
        expected.extend_from_slice(&[0xff, 0xd8]);
        expected.extend_from_slice(b"\r\n--B--\r\n");
        assert_eq!(body, expected);
    }
}

#[cfg(test)]
mod buttons_attached_and_upload_bounds {
    //! The round-1 code audit of the bot and the chat, fix-here items 6 and 15.
    use super::a_stand_in::AStandInService;
    use super::*;

    /// **`Telegram` R1 — the buttons-only call carries the message, the
    /// buttons, and nothing else.**
    #[test]
    fn buttons_are_attached_by_a_call_that_changes_only_them() {
        let service = Arc::new(AStandInService::new());
        let client = TheClient::over(service.clone());
        let markup = json!({ "inline_keyboard": [[{ "text": "Stop this job", "callback_data": "t" }]] });
        client.edit_message_reply_markup(7, 90, markup.clone()).unwrap();
        let heard = service.heard();
        assert_eq!(heard[0].method, "editMessageReplyMarkup");
        let WhatACallCarries::Fields(fields) = &heard[0].carries else { panic!("fields") };
        assert_eq!(Value::Object(fields.clone()), json!({ "chat_id": 7, "message_id": 90, "reply_markup": markup }));
    }

    /// **`Telegram` C6b — an upload's time limit grows with the file**: 300
    /// seconds for a small one, and a 50 MB one gets its size at 50 KB a second.
    /// The flat 300 seconds would have failed the second half.
    #[test]
    fn an_uploads_time_limit_grows_with_its_size() {
        assert_eq!(the_bound_on_sending(64), Duration::from_secs(300));
        assert_eq!(the_bound_on_sending(50_000_000), Duration::from_secs(1000));
        let service = Arc::new(AStandInService::new());
        let client = TheClient::over(service.clone());
        client.send_document(7, "words.txt", vec![0; 50_000_000], None, None).unwrap();
        client.send_media_group(7, vec![("a.png".into(), vec![0; 20_000_000]), ("b.png".into(), vec![0; 20_000_000])]).unwrap();
        client.send_photo(7, "small.png", vec![0; 10], None, None).unwrap();
        let bounds: Vec<Duration> = service.heard().iter().map(|c| c.bound).collect();
        assert_eq!(bounds, [Duration::from_secs(1000), Duration::from_secs(800), Duration::from_secs(300)]);
    }
}

#[cfg(test)]
mod the_development_switch {
    //! **No desk test reaches Telegram** (`Audits/Build/Step 10c/Brief — no
    //! desk test reaches Telegram.md`): the switch, proved by connections
    //! counted at a listener the client is pointed at, and its absence from a
    //! release build.
    use super::*;
    use std::ffi::OsStr;
    #[cfg(debug_assertions)]
    use std::net::TcpListener;
    #[cfg(debug_assertions)]
    use std::time::Instant;

    #[cfg(debug_assertions)]
    const A_MADE_UP_TOKEN: &str = "123456:a-token-that-is-not-real";

    /// **A listener on this computer that answers nothing**, and the address
    /// that points a client at it. A connection made to it waits in its queue
    /// whether or not anybody takes it, so counting what waits counts every
    /// connection the client opened.
    #[cfg(debug_assertions)]
    fn a_silent_listener() -> (TcpListener, String) {
        let listener = TcpListener::bind("127.0.0.1:0").expect("a port on this computer");
        listener.set_nonblocking(true).expect("the listener is counted without waiting");
        let address = format!("https://{}", listener.local_addr().expect("its address"));
        (listener, address)
    }

    #[cfg(debug_assertions)]
    fn connections_made_to(listener: &TcpListener) -> usize {
        let mut made = 0;
        while listener.accept().is_ok() {
            made += 1;
        }
        made
    }

    /// **With the switch on, every kind of call the client makes comes back
    /// at once as a line that is down, and no connection is opened** — counted
    /// at a listener the client is pointed at, so silence there means the
    /// client really tried nothing. **And the probe is shown to see**: the same
    /// client with the switch off, pointed at the same listener, is counted.
    ///
    /// Every call here is cut to three seconds, so a call that did connect to
    /// the silent listener would take that long; one second is the bar for
    /// *at once*.
    #[cfg(debug_assertions)]
    #[test]
    fn with_the_switch_on_every_call_reaches_nothing_and_opens_no_connection() {
        let (listener, address) = a_silent_listener();
        let wire = OverHttps::reaching(&address, A_MADE_UP_TOKEN, true);
        let client = TheClient::over(Arc::new(wire)).no_call_longer_than(Duration::from_secs(3));

        type ACall = Box<dyn Fn(&TheClient) -> WhatTheServiceGaveBack>;
        let calls: Vec<(&str, ACall)> = vec![
            ("getMe", Box::new(|c| c.get_me())),
            ("getUpdates", Box::new(|c| c.get_updates(Some(7), 25))),
            ("getChat", Box::new(|c| c.get_chat(42))),
            ("sendMessage", Box::new(|c| c.send_message(42, "hello", None))),
            ("editMessageText", Box::new(|c| c.edit_message_text(42, 7, "hello", None))),
            ("a machinery message", Box::new(|c| c.send_a_machinery_message(42, "a board"))),
            ("pinChatMessage", Box::new(|c| c.pin_chat_message(42, 7))),
            ("deleteMessage", Box::new(|c| c.delete_a_pins_line(42, 7))),
            ("sendPhoto", Box::new(|c| c.send_photo(42, "frame.jpg", vec![1, 2, 3], None, None))),
            ("sendMediaGroup", Box::new(|c| c.send_media_group(42, vec![("a.jpg".into(), vec![1]), ("b.jpg".into(), vec![2])]))),
            ("answerCallbackQuery", Box::new(|c| c.answer_callback_query("a tap"))),
            ("getWebhookInfo", Box::new(|c| c.get_webhook_info())),
            ("deleteWebhook", Box::new(|c| c.delete_webhook())),
        ];
        for (which, call) in &calls {
            let asked_at = Instant::now();
            let got = call(&client);
            let took = asked_at.elapsed();
            match got {
                Err(WhyTheCallFailed::NotReached(why)) => {
                    assert_eq!(why, what_a_call_says_with_the_switch_on(), "{which}");
                    assert!(!why.contains(A_MADE_UP_TOKEN), "{which}: the token reached the outcome");
                }
                other => panic!("{which}: with the switch on the call came back {other:?}"),
            }
            assert!(took < Duration::from_secs(1), "{which} took {took:?}; with the switch on it is at once");
        }
        assert_eq!(
            connections_made_to(&listener),
            0,
            "with the switch on the client opened a connection"
        );

        // **The probe sees**: switch off, the same listener, one call.
        let unswitched = OverHttps::reaching(&address, A_MADE_UP_TOKEN, false);
        let got = unswitched.call("getMe", &WhatACallCarries::Nothing, Duration::from_secs(2));
        assert!(got.is_err(), "the silent listener answers nothing");
        assert_eq!(
            connections_made_to(&listener),
            1,
            "with the switch off the client is counted, so a count of nothing above is silence"
        );
    }

    /// **The switch is absent from a release build**, asked as a real
    /// question: the one function that exists in both builds is handed the
    /// switch's *on* value, and must answer yes only where debug assertions are
    /// on. Run it both ways:
    /// `cargo test -p smoozy-telegram --lib -- the_development_switch` and the same with
    /// `--release`.
    #[test]
    fn the_switch_says_reach_nothing_in_a_development_build_only() {
        assert_eq!(
            whether_the_switch_says_reach_nothing(Some(OsStr::new("1"))),
            cfg!(debug_assertions),
            "a release build must not honour the switch; a development build must"
        );
        for not_on in [None, Some(OsStr::new("")), Some(OsStr::new("0")), Some(OsStr::new("yes"))] {
            assert!(!whether_the_switch_says_reach_nothing(not_on), "{not_on:?} is not on");
        }
    }
}

/// **The walkthrough's calls over a line that goes silent** — the dead-line
/// desk coverage, 2026-10-19.
#[cfg(test)]
mod a_line_that_goes_silent {
    use super::*;

    /// **A loopback listener that takes every connection and says nothing**,
    /// for as long as `held_for`: a line gone silent before any answer came.
    fn a_service_that_goes_silent(held_for: Duration) -> (String, std::thread::JoinHandle<()>) {
        let listener = std::net::TcpListener::bind(("127.0.0.1", 0)).expect("a loopback listener");
        let address = format!("https://127.0.0.1:{}", listener.local_addr().expect("an address").port());
        listener.set_nonblocking(true).expect("a listener that can be polled");
        let held = std::thread::spawn(move || {
            let began = std::time::Instant::now();
            let mut taken = Vec::new();
            while began.elapsed() < held_for {
                if let Ok((stream, _)) = listener.accept() {
                    taken.push(stream);
                }
                std::thread::sleep(Duration::from_millis(20));
            }
            drop(taken);
        });
        (address, held)
    }

    /// **The walkthrough's token check and its long poll, over a line that
    /// goes silent, each end at their own bounds — thirty seconds for the
    /// ordinary call, the poll's own wait and thirty for the poll — say
    /// Telegram could not be reached, keep the paste, and the check asked
    /// again of a service that answers passes.** The dead-line desk coverage,
    /// 2026-10-19. The far end is a loopback listener and the token a dummy:
    /// nothing here reaches Telegram, and the bot is never used. **The body's
    /// silence is not proved here and cannot be at this desk**: the client
    /// goes nowhere but HTTPS and trusts only the bundled roots, so no local
    /// listener answers as far as a body; the whole call is under the one
    /// bound either way.
    #[test]
    fn the_walkthroughs_calls_over_a_silent_line_end_at_their_bounds_and_the_paste_is_kept() {
        use crate::walkthrough::{check_the_bot_answers_through, ThePaste, WhatTheCheckFound, WhyTheCheckDidNotFinish};
        let (address, held) = a_service_that_goes_silent(Duration::from_secs(80));
        let token = "123456:a-dummy-token-for-the-desk";
        let paste = ThePaste::what_the_user_pasted(format!("Use this token to access the HTTP API: {token}"));
        let silent = |token: &str| {
            TheClient::over(Arc::new(OverHttps::reaching(&address, token, false)))
        };
        let polled = {
            let client = silent(token);
            std::thread::spawn(move || {
                let began = std::time::Instant::now();
                let polled = client.get_updates(None, 1);
                (polled, began.elapsed())
            })
        };
        let began = std::time::Instant::now();
        let checked = check_the_bot_answers_through(&paste, &silent);
        let took = began.elapsed();
        let (polled, poll_took) = polled.join().expect("the poll ran");
        println!("the check came to {checked:?} in {took:?}; the poll to {polled:?} in {poll_took:?}");
        assert!(
            matches!(checked, WhatTheCheckFound::DidNotFinish(WhyTheCheckDidNotFinish::TelegramCouldNotBeReached)),
            "{checked:?}"
        );
        let near = |took: Duration, bound: Duration| took + Duration::from_secs(1) >= bound && took <= bound + Duration::from_secs(5);
        assert!(near(took, THE_BOUND_ON_AN_ORDINARY_CALL), "the check ended after {took:?}");
        assert!(matches!(polled, Err(WhyTheCallFailed::NotReached(_))), "{polled:?}");
        assert!(
            near(poll_took, Duration::from_secs(1) + THE_MARGIN_BEYOND_AN_ASKS_OWN_WAIT),
            "the long poll ended after {poll_took:?}"
        );
        assert!(paste.holds_a_token(), "the paste was not kept for the retry");
        // **The line back: the same paste, asked again of a service that
        // answers, passes** — the existing stand-in service.
        let service = Arc::new(a_stand_in::AStandInService::new());
        service.then_answers(
            r#"{"ok":true,"result":{"id":42,"is_bot":true,"first_name":"Smoozy","username":"smoozy_desk_bot"}}"#,
        );
        let answering = |_: &str| TheClient::over(Arc::clone(&service) as Arc<dyn ReachesTelegram>);
        assert!(
            matches!(check_the_bot_answers_through(&paste, &answering), WhatTheCheckFound::Answered(_)),
            "the check asked again did not pass"
        );
        drop(held);
    }
}

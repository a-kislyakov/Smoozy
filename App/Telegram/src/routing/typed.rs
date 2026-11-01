//! **What the user sends, read — `Telegram` R1a, R3, R13, R13a and R15,
//! with the words the user reads.**
//!
//! **R15 as widened 2026-10-31: four typed requests, each also a command in
//! R17's list** — `status` (R10; *how is it going* still read), `frame` with
//! or without a number (R13, R13a), `reporting` with or without a number
//! (V1d) and `stop` (R13a). **Each is read with or without its leading `/`,
//! with Telegram's `@` and the bot's username after the command word
//! ignored**, and with case and a closing `?`, `.` or `!` ignored as before.
//! `/start` is §7's and never one of these. **R3: an update the listener
//! cannot resolve is answered, never guessed at.**
//!
//! **The words.** `Telegram` has no Design document; the drawing is the design
//! (`Preparation/Design/Mockup/Smoozy.html`, the `tg-ask`, `tg-slash`,
//! `tg-reporting` and `tg-reportingset` states). **Answers not drawn are
//! written here in standard words**, which Design may replace (R3: the
//! wording is Design's).

use serde_json::Value;

/// **The drawing's question for R13's disambiguation.**
pub const WHICH_JOB: &str = "Which job?";

/// **`Telegram` R1 and R3 — the one answer to a button whose resolution
/// expired or was rejected.** The holder that removes the entry cannot tell a
/// machine that is gone from one that rejected the press, so it says both, and
/// says plainly that nothing was done.
pub const A_BUTTON_THAT_EXPIRED: &str = "Button expired or its server did not respond. Nothing was done.";

/// **`Telegram` R3 and R15 — a message that is none of the four requests.**
/// Changed 2026-10-31 with R15's four: it named *status* and *frame* alone.
pub const NOT_A_REQUEST: &str = "Unknown request. Send /status, /frame, /reporting or /stop.";

/// **`Telegram` V1d — `reporting N` with N outside 1 to 60.**
pub const WHOLE_MINUTES: &str = "Whole minutes, 1 to 60.";

/// **`Telegram` R13a — `stop` with no job waiting, held or rendering.**
pub const NOTHING_TO_STOP: &str = "Nothing is rendering or waiting.";

/// **`Telegram` R1a, widened 2026-10-31 — a reply to a message no machine
/// claimed**, for `stop` and `reporting`, which fail closed.
pub const COULD_NOT_TELL_WHICH_JOB: &str = "Could not tell which job that message is about. Nothing changed.";

/// **`Telegram` R13 — a frame request with no job on any server to answer it.**
pub const NO_JOB_TO_SEND_A_FRAME_FROM: &str = "No jobs on your servers.";

/// **`Telegram` R17 — the bot's command list, in order, each description
/// exactly as the user reads it.** `BotCommand.command` is 1–32 lowercase
/// letters, digits and underscores, `description` 1–256 characters (Sources
/// entry 89).
pub const THE_COMMANDS: [(&str, &str); 4] = [
    ("status", "Status of every server"),
    ("frame", "Send the newest frame, or /frame 120"),
    ("reporting", "Change what every job reports"),
    ("stop", "Stop a job"),
];

/// **R17 — the list as `setMyCommands` takes it.**
pub fn the_command_list() -> Value {
    Value::Array(THE_COMMANDS.iter().map(|(command, description)| serde_json::json!({ "command": command, "description": description })).collect())
}

/// **R17 — whether the list Telegram holds differs from this program's**:
/// the same commands and descriptions in the same order, or it differs.
pub fn the_command_list_differs(standing: &Value) -> bool {
    let read = |list: &Value| -> Option<Vec<(String, String)>> {
        list.as_array()?
            .iter()
            .map(|one| Some((one.get("command")?.as_str()?.to_string(), one.get("description")?.as_str()?.to_string())))
            .collect()
    };
    read(standing) != read(&the_command_list())
}

/// **`Telegram` R15 — the four typed requests.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ATypedRequest {
    /// R10's gather.
    HowIsItGoing,
    /// R13: a frame number and no job.
    SendAFrame(u64),
    /// **R13a: `frame` with no number** — the newest frame the chosen job's
    /// derivative set holds.
    SendTheNewestFrame,
    /// **V1d: `reporting`**, with its minutes where a number was typed. The
    /// number's range (1 to 60) is checked where it is answered, so a number
    /// outside it is told [`WHOLE_MINUTES`] rather than *unknown*.
    Reporting(Option<u64>),
    /// **R13a: `stop`.**
    Stop,
}

/// **Read a message's text as one of the four requests, or as none.** The
/// first word may carry a leading `/` and Telegram's `@` and the bot's
/// username (R15); case, the space between words and a closing question mark,
/// full stop or exclamation mark do not matter. *status* and *frame 120* are
/// the words the help named before the commands; *how is it going*, *send
/// frame 120* and *send a frame 120* are still read, so a request typed the
/// old way is never told it is unknown.
pub fn what_was_typed(text: &str) -> Option<ATypedRequest> {
    let lowered = text.trim().to_lowercase();
    let mut words: Vec<&str> = lowered
        .trim_end_matches(['?', '.', '!'])
        .split_whitespace()
        .collect();
    if let Some(first) = words.first_mut() {
        if let Some(command) = first.strip_prefix('/') {
            *first = command.split('@').next().unwrap_or_default();
        }
    }
    match words.as_slice() {
        ["status"] | ["how", "is", "it", "going"] => Some(ATypedRequest::HowIsItGoing),
        ["frame"] => Some(ATypedRequest::SendTheNewestFrame),
        ["frame", n] | ["send", "frame", n] | ["send", "a", "frame", n] => n.parse().ok().map(ATypedRequest::SendAFrame),
        ["reporting"] => Some(ATypedRequest::Reporting(None)),
        ["reporting", n] => n.parse().ok().map(|n| ATypedRequest::Reporting(Some(n))),
        ["stop"] => Some(ATypedRequest::Stop),
        _ => None,
    }
}

/// **What one update from the proved conversation asks.** The conversation is
/// already proved by the listener (`Telegram` P2, P3).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatAnUpdateAsks {
    /// **R1** — a press on a button.
    /// `label` is the pressed button's own label, read from the message the tap
    /// carries, where it can be found: what the user is told names it (R7,
    /// Rule 9). Nothing routes by it.
    ATap { update: i64, callback: String, data: Option<String>, label: Option<String>, conversation: i64, source_message: i64 },
    /// **R10, R13, R13a, V1d, and R1a where `replying_to` is present.**
    /// `date` is Telegram's own moment for the message (V1d); `to_a_person`
    /// is whether the reply shows the replied-to message was a person's own
    /// (R1a's at-once check; §21 item 38: read only where present).
    ARequest { update: i64, request: ATypedRequest, conversation: i64, replying_to: Option<i64>, date: i64, to_a_person: bool },
    /// **R3** — text that is neither request, or a tap that carries no message
    /// to resolve against.
    NotUnderstood { update: i64 },
    /// Nothing to answer: a message with no text, such as a photo the user
    /// sent. Nothing is routed from it.
    Nothing { update: i64 },
}

/// **Read an update.** The two kinds every ask names are the two read.
pub fn what_an_update_asks(update: &Value) -> Option<WhatAnUpdateAsks> {
    let id = update.get("update_id").and_then(Value::as_i64)?;
    if let Some(tap) = update.get("callback_query") {
        let callback = tap.get("id").and_then(Value::as_str);
        let conversation = tap.pointer("/message/chat/id").and_then(Value::as_i64);
        let source_message = tap.pointer("/message/message_id").and_then(Value::as_i64);
        return Some(match (callback, conversation, source_message) {
            (Some(callback), Some(conversation), Some(source_message)) => WhatAnUpdateAsks::ATap {
                update: id,
                callback: callback.to_string(),
                data: tap.get("data").and_then(Value::as_str).map(str::to_string),
                label: tap.get("data").and_then(Value::as_str).and_then(|data| {
                    tap.pointer("/message/reply_markup/inline_keyboard")?
                        .as_array()?
                        .iter()
                        .filter_map(Value::as_array)
                        .flatten()
                        .find(|button| button.get("callback_data").and_then(Value::as_str) == Some(data))?
                        .get("text")?
                        .as_str()
                        .map(str::to_string)
                }),
                conversation,
                source_message,
            },
            _ => WhatAnUpdateAsks::NotUnderstood { update: id },
        });
    }
    let said = update.get("message")?;
    let conversation = said.pointer("/chat/id").and_then(Value::as_i64)?;
    // P3: bot text and pin service updates are not requests from a person.
    // The listener reads pins separately before handing ordinary requests on.
    if said.pointer("/from/is_bot").and_then(Value::as_bool) == Some(true)
        || said.get("pinned_message").is_some()
    {
        return Some(WhatAnUpdateAsks::Nothing { update: id });
    }
    let Some(text) = said.get("text").and_then(Value::as_str) else {
        return Some(WhatAnUpdateAsks::Nothing { update: id });
    };
    let command = text.split_whitespace().next().unwrap_or_default();
    if command.split('@').next() == Some("/start") {
        return Some(WhatAnUpdateAsks::Nothing { update: id });
    }
    Some(match what_was_typed(text) {
        Some(request) => WhatAnUpdateAsks::ARequest {
            update: id,
            request,
            conversation,
            replying_to: said.pointer("/reply_to_message/message_id").and_then(Value::as_i64),
            date: said.get("date").and_then(Value::as_i64).unwrap_or_default(),
            to_a_person: said.pointer("/reply_to_message/from/is_bot").and_then(Value::as_bool) == Some(false),
        },
        None => WhatAnUpdateAsks::NotUnderstood { update: id },
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    /// **`Telegram` R15 as widened 2026-10-31: four requests, each with or
    /// without its `/` and the bot's `@` name, the old words still read.**
    #[test]
    fn the_four_requests_are_read_in_every_form() {
        for status in ["status", "/status", "/status@SmoozyBot", "/Status@smoozy_bot ", "Status!"] {
            assert_eq!(what_was_typed(status), Some(ATypedRequest::HowIsItGoing), "{status}");
        }
        for newest in ["frame", "/frame", "/frame@SmoozyBot", " Frame. "] {
            assert_eq!(what_was_typed(newest), Some(ATypedRequest::SendTheNewestFrame), "{newest}");
        }
        assert_eq!(what_was_typed("/frame 120"), Some(ATypedRequest::SendAFrame(120)));
        assert_eq!(what_was_typed("/frame@SmoozyBot 7"), Some(ATypedRequest::SendAFrame(7)));
        assert_eq!(what_was_typed("/reporting"), Some(ATypedRequest::Reporting(None)));
        assert_eq!(what_was_typed("reporting"), Some(ATypedRequest::Reporting(None)));
        assert_eq!(what_was_typed("/reporting@SmoozyBot 10"), Some(ATypedRequest::Reporting(Some(10))));
        assert_eq!(what_was_typed("/reporting 90"), Some(ATypedRequest::Reporting(Some(90))), "the range is checked where it is answered");
        for stop in ["stop", "/stop", "/stop@SmoozyBot", "Stop."] {
            assert_eq!(what_was_typed(stop), Some(ATypedRequest::Stop), "{stop}");
        }
        for none in ["/start", "/reporting ten", "/frame one", "/stop now", "/help", "reporting 1 2", "/", "@SmoozyBot"] {
            assert_eq!(what_was_typed(none), None, "{none}");
        }
    }

    #[test]
    fn two_requests_are_recognised_and_nothing_else() {
        assert_eq!(what_was_typed("status"), Some(ATypedRequest::HowIsItGoing));
        assert_eq!(what_was_typed("  Status? "), Some(ATypedRequest::HowIsItGoing));
        assert_eq!(what_was_typed("frame 120"), Some(ATypedRequest::SendAFrame(120)));
        assert_eq!(what_was_typed("Frame 7."), Some(ATypedRequest::SendAFrame(7)));
        // **The words before step 13's final words sweep are still read.**
        assert_eq!(what_was_typed("  How is it  going? "), Some(ATypedRequest::HowIsItGoing));
        assert_eq!(what_was_typed("send frame 120"), Some(ATypedRequest::SendAFrame(120)));
        assert_eq!(what_was_typed("Send a frame 7."), Some(ATypedRequest::SendAFrame(7)));
        for neither in ["send a frame", "send frame one", "frame one", "status now", "how is it", "/start", "", "how is it going now"] {
            assert_eq!(what_was_typed(neither), None, "{neither}");
        }
    }

    #[test]
    fn an_update_is_read_as_a_tap_a_request_a_reply_or_neither() {
        let tap = json!({ "update_id": 1, "callback_query": { "id": "cb", "data": "t",
            "message": { "message_id": 5, "chat": { "id": 9 } } } });
        assert_eq!(
            what_an_update_asks(&tap),
            Some(WhatAnUpdateAsks::ATap { update: 1, callback: "cb".into(), data: Some("t".into()), label: None, conversation: 9, source_message: 5 })
        );
        let labelled = json!({ "update_id": 1, "callback_query": { "id": "cb", "data": "t",
            "message": { "message_id": 5, "chat": { "id": 9 },
                "reply_markup": { "inline_keyboard": [[{ "text": "Cancel", "callback_data": "u" }], [{ "text": "Stop this job", "callback_data": "t" }]] } } } });
        assert!(matches!(what_an_update_asks(&labelled), Some(WhatAnUpdateAsks::ATap { label: Some(l), .. }) if l == "Stop this job"));
        let typed = json!({ "update_id": 2, "message": { "message_id": 6, "chat": { "id": 9 }, "text": "status" } });
        assert_eq!(
            what_an_update_asks(&typed),
            Some(WhatAnUpdateAsks::ARequest { update: 2, request: ATypedRequest::HowIsItGoing, conversation: 9, replying_to: None, date: 0, to_a_person: false })
        );
        let reply = json!({ "update_id": 3, "message": { "message_id": 7, "chat": { "id": 9 }, "text": "frame 4", "date": 1730000000,
            "reply_to_message": { "message_id": 5 } } });
        assert_eq!(
            what_an_update_asks(&reply),
            Some(WhatAnUpdateAsks::ARequest { update: 3, request: ATypedRequest::SendAFrame(4), conversation: 9, replying_to: Some(5), date: 1730000000, to_a_person: false })
        );
        // §21 item 38: a reply that shows the replied-to message was a person's.
        let to_a_person = json!({ "update_id": 8, "message": { "message_id": 9, "chat": { "id": 9 }, "text": "/stop",
            "reply_to_message": { "message_id": 6, "from": { "is_bot": false } } } });
        assert!(matches!(what_an_update_asks(&to_a_person), Some(WhatAnUpdateAsks::ARequest { to_a_person: true, request: ATypedRequest::Stop, .. })));
        // `/start` is still §7's, and nothing here.
        let start = json!({ "update_id": 10, "message": { "chat": { "id": 9 }, "text": "/start@SmoozyBot" } });
        assert_eq!(what_an_update_asks(&start), Some(WhatAnUpdateAsks::Nothing { update: 10 }));
        let neither = json!({ "update_id": 4, "message": { "chat": { "id": 9 }, "text": "hello" } });
        assert_eq!(what_an_update_asks(&neither), Some(WhatAnUpdateAsks::NotUnderstood { update: 4 }));
        let photo = json!({ "update_id": 5, "message": { "chat": { "id": 9 }, "photo": [] } });
        assert_eq!(what_an_update_asks(&photo), Some(WhatAnUpdateAsks::Nothing { update: 5 }));
        let bare_tap = json!({ "update_id": 6, "callback_query": { "id": "cb" } });
        assert_eq!(what_an_update_asks(&bare_tap), Some(WhatAnUpdateAsks::NotUnderstood { update: 6 }));
    }

    /// **Rule 9 and `Telegram` R3: the answers say plainly what happened.**
    #[test]
    fn the_answers_say_nothing_was_done() {
        assert!(A_BUTTON_THAT_EXPIRED.contains("Nothing was done."));
        assert_eq!(WHICH_JOB, "Which job?");
    }
}

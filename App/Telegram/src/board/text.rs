//! **The board's text and the machinery message's text — `Telegram` B2, B10,
//! B21, B22 and B23.** One canonical, versioned form for each, composed whole,
//! refused over Telegram's bound, and read back into the same values.
//!
//! **`Telegram` B23: every message Smoozy pins begins with a short, plainly
//! readable head.** B21 observed that the conversation's line quotes a pinned
//! message from its start and that a phone's banner shows its first line, so
//! the head is all either surface has room for (B22: that is the one lever, and
//! it changes no routing or delivery rule). **The words are the design's**:
//! `Preparation/Design/Mockup/Smoozy.html`, the pinned bar in `#tgview` and the
//! note *"The pinned message names itself"* — *Smoozy — server coordination*
//! for the board (*Smoozy — working note* until step 13's words sweep), and
//! the same head with the server's name for a machinery message.

use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::board::instructions::{AKindOfEntry, AnEntry, TheAddressee};
use crate::board::roster::{the_display_of, ARosterEntry};
use crate::listener::in_the_users_words;

/// **`Telegram` B2 — Telegram's published bound on `sendMessage` and
/// `editMessageText`: 4,096 characters after entity parsing.** B25's spoiler
/// is sent as an entity list beside the text, never as markup inside it, so it
/// adds nothing to what is counted here (B25: *"The marking counts for nothing
/// in B2's 4,096 bound, which is on the text"*). **Length is counted in UTF-16
/// units**, which is never fewer than characters however the service counts,
/// so a candidate this accepts is inside the bound either way.
pub const THE_BOUND_ON_A_MESSAGE: usize = 4096;

/// **`Telegram` B23 — the board's head**, the design's own words.
pub const THE_HEAD: &str = "Smoozy \u{2014} server coordination";

/// **The head every pin of Smoozy's opened with until step 13's words sweep
/// (2026-10-28).** Nothing reads a board or a machinery message by its head
/// (the marks below do that), but the listener tells Smoozy's own pins apart
/// by it ([`crate::listener::the_line_a_pin_of_smoozys_wrote`]), so a pin an
/// older server made is still told as Smoozy's.
pub const THE_HEAD_BEFORE_2026_10_28: &str = "Smoozy \u{2014} working note";

/// The design's second line for the board, from the same drawing.
pub const THE_BOARDS_SECOND_LINE: &str = "Used by your servers. No action needed.";

/// **`Telegram` B2 — the versioned mark before the canonical form**: the
/// mark, then the version of the serialization after it.
const A_BOARD_MARK: &str = "smoozy-board ";
const A_MACHINERY_MARK: &str = "smoozy-machinery ";

/// **`Telegram` B2 and B9a — the board serialization this build writes**, and
/// the last one it reads. **It writes only this one.**
pub const THE_BOARD_VERSION: u32 = 1;

/// **`Telegram` B10 and B9a — the machinery message's serialization this
/// build writes**, and the last one it reads.
pub const THE_MACHINERY_VERSION: u32 = 1;

/// **`Telegram` B2 — the route mode.** Only asking is built; J1, J2 and J4 are
/// recorded as not built at the head of `src/transport.rs`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TheRouteMode {
    Asking,
    RegistrationHeld,
}

/// **`Telegram` B1 and B2 — the board.** The holder's machine identity; the
/// opaque heartbeat with the moment on the holder's own clock beside it; the
/// stated renewal interval; the route mode; the roster (B14); and the
/// authoritative list of unacknowledged instructions (B8).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TheBoard {
    pub holder: String,
    pub heartbeat: String,
    pub heartbeat_moment: String,
    pub renewal_seconds: u64,
    pub route: TheRouteMode,
    pub roster: Vec<ARosterEntry>,
    pub instructions: Vec<AnEntry>,
}

/// A candidate over B2's bound. **Never sent.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OverTheBound;

/// **How long Telegram's bound counts a text**, in UTF-16 units.
pub fn how_long_it_counts(text: &str) -> usize {
    text.encode_utf16().count()
}

fn within_the_bound(text: String) -> Result<String, OverTheBound> {
    if how_long_it_counts(&text) <= THE_BOUND_ON_A_MESSAGE {
        Ok(text)
    } else {
        Err(OverTheBound)
    }
}

impl TheBoard {
    /// **`Telegram` B2 and B23 — the complete candidate**: the head, the
    /// design's rows for a person, then the versioned canonical form. **The
    /// rows are composed from the same values and never read back**; only the
    /// canonical form is.
    pub fn compose(&self) -> Result<String, OverTheBound> {
        let display = |machine: &str| the_display_of(self, machine).unwrap_or("a server").to_string();
        let mut rows = vec![
            THE_HEAD.to_string(),
            THE_BOARDS_SECOND_LINE.to_string(),
            String::new(),
            format!("Receiving: {} \u{00b7} {}", display(&self.holder), self.heartbeat_moment),
            format!(
                "Button presses: {}",
                match self.route {
                    TheRouteMode::Asking => "polled".to_string(),
                    TheRouteMode::RegistrationHeld => format!("sent to {}", display(&self.holder)),
                }
            ),
        ];
        if !self.roster.is_empty() {
            let names: Vec<&str> = self.roster.iter().map(|r| r.display.as_str()).collect();
            rows.push(format!("Servers: {}", names.join(" \u{00b7} ")));
        }
        for entry in self.instructions.iter().filter(|e| e.kind == AKindOfEntry::Action) {
            let to = match &entry.addressee {
                TheAddressee::Machine(machine) => display(machine),
                _ => "a server".to_string(),
            };
            // **The button or request and the job, never the internal
            // action name** (R7, Rule 9) — the words the listener's own
            // messages use for the same entry, as the drawing's row does.
            rows.push(format!("Pending: {}, sent to {} at {}", in_the_users_words(entry), to, entry.accepted_at));
        }
        let canonical = serde_json::to_string(self).map_err(|_| OverTheBound)?;
        rows.push(String::new());
        rows.push(format!("{A_BOARD_MARK}{THE_BOARD_VERSION}"));
        rows.push(canonical);
        within_the_bound(rows.join("\n"))
    }

    /// Whether the complete candidate is inside B2's bound.
    pub fn fits(&self) -> bool {
        self.compose().is_ok()
    }
}

/// **`Telegram` B10 and B15 — everything one machine must say to the holder,
/// composed whole into its one machinery message**: its announcement (B14),
/// its acknowledgements (B9), its proposals (B15), its replacements of a
/// context-resolution entry (B8), and its replies to a gather (R11, piece 3's).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TheUtterance {
    pub machine: String,
    pub display: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub colour: Option<String>,
    pub announce: bool,
    pub acknowledged: Vec<String>,
    pub proposed: Vec<AnEntry>,
    pub replacing: Vec<AnEntry>,
    pub replies: Vec<Value>,
}

impl TheUtterance {
    pub fn has_nothing_to_say(&self) -> bool {
        !self.announce
            && self.acknowledged.is_empty()
            && self.proposed.is_empty()
            && self.replacing.is_empty()
            && self.replies.is_empty()
    }

    /// **`Telegram` B23 — the head with the server's name first**, then the
    /// versioned canonical form.
    pub fn compose(&self) -> Result<String, OverTheBound> {
        let canonical = serde_json::to_string(self).map_err(|_| OverTheBound)?;
        within_the_bound(format!(
            "{THE_HEAD} \u{00b7} {}\n\n{A_MACHINERY_MARK}{THE_MACHINERY_VERSION}\n{canonical}",
            self.display
        ))
    }
}

/// **`Telegram` B25 — where the spoiler goes on a board or a machinery
/// message: everything after B23's head.** Given as a start and a length in
/// **UTF-16 code units**, the unit Telegram's `MessageEntity` names for both
/// (*"Offset in UTF-16 code units to the start of the entity"*, read
/// 2026-09-14, `Audits/Build/Step 10/The spoiler — what Telegram's pages
/// say.md`).
///
/// **The head ends at the first blank line.** Both compositions above put one
/// there and nowhere earlier: the board after its head and the design's second
/// sentence — which the drawing keeps plain beside the head, with only the
/// rows under it blurred (`Preparation/Design/Mockup/Smoozy.html`, the pinned
/// bar in `#tgview`) — and a machinery message after its one head line.
///
/// **The blank lines themselves are left out of the start, and trailing
/// whitespace out of the length**, as Telegram's entities page asks (*"the
/// length of an entity must not include the length of trailing newlines or
/// whitespaces"*). A text with nothing after its head has no spoiler.
pub fn the_spoiler_after_the_head(text: &str) -> Option<(usize, usize)> {
    let head_ends = text.find("\n\n")?;
    let after_the_head = &text[head_ends..];
    let hidden_starts = head_ends + (after_the_head.len() - after_the_head.trim_start().len());
    let hidden = text[hidden_starts..].trim_end();
    if hidden.is_empty() {
        return None;
    }
    Some((how_long_it_counts(&text[..hidden_starts]), how_long_it_counts(hidden)))
}

/// **What the reported pin is**, read from a `getChat` result's
/// `pinned_message` (B13).
#[derive(Debug, Clone, PartialEq)]
pub enum WhatIsPinned {
    Nothing,
    ABoard { id: i64, board: TheBoard },
    AMachineryMessage { id: i64, utterance: TheUtterance },
    /// **`Telegram` B3**: shaped as a board of this bot's and not readable as
    /// one — not silently replaced unless it is stale. `text` is kept, compared
    /// for equality only, for B3's stale-board test.
    AnUnreadableBoard { id: i64, text: String },
    /// Anything else pinned in the conversation.
    SomethingElse { id: i64 },
}

impl WhatIsPinned {
    pub fn the_identity(&self) -> Option<i64> {
        match self {
            WhatIsPinned::Nothing => None,
            WhatIsPinned::ABoard { id, .. }
            | WhatIsPinned::AMachineryMessage { id, .. }
            | WhatIsPinned::AnUnreadableBoard { id, .. }
            | WhatIsPinned::SomethingElse { id } => Some(*id),
        }
    }
}

/// **Read the reported pin out of a `getChat` result.** Only a message this
/// bot sent can be a board or a machinery message; a person typing the same
/// words makes nothing of Smoozy's.
///
/// **`Telegram` B9a — every board and machinery message of this build's
/// version and of every earlier one is read**, each through the reader for its
/// own version, so a newer holder never stands down over an older machine's
/// board or message. **A later version is not guessed at**: a board of one is
/// unreadable (B2's stated cost, and B3's rule), and a machinery message of one
/// is something else, which the holder unpins without reading (B2, B11).
pub fn what_is_pinned(chat: &Value) -> WhatIsPinned {
    what_is_pinned_by(chat, WhatAReaderSpeaks { board: THE_BOARD_VERSION, machinery: THE_MACHINERY_VERSION })
}

/// **The last version of each serialization a reader speaks.** This build's
/// is its own; a test names a later one to read as the next release would.
#[derive(Debug, Clone, Copy)]
struct WhatAReaderSpeaks {
    board: u32,
    machinery: u32,
}

fn what_is_pinned_by(chat: &Value, speaks: WhatAReaderSpeaks) -> WhatIsPinned {
    let Some(pinned) = chat.get("pinned_message") else {
        return WhatIsPinned::Nothing;
    };
    let Some(id) = pinned.get("message_id").and_then(Value::as_i64) else {
        return WhatIsPinned::Nothing;
    };
    let from_a_bot = pinned.pointer("/from/is_bot").and_then(Value::as_bool) == Some(true);
    let text = pinned.get("text").and_then(Value::as_str).unwrap_or("");
    if !from_a_bot {
        return WhatIsPinned::SomethingElse { id };
    }
    let lines: Vec<&str> = text.lines().collect();
    if let Some((at, version)) = the_mark(&lines, A_BOARD_MARK) {
        let canonical = lines[at + 1..].join("\n");
        return match version.filter(|v| *v <= speaks.board).and_then(|v| a_board_of_version(v, &canonical)) {
            Some(board) => WhatIsPinned::ABoard { id, board },
            None => WhatIsPinned::AnUnreadableBoard { id, text: text.to_string() },
        };
    }
    if let Some((at, Some(version))) = the_mark(&lines, A_MACHINERY_MARK) {
        if version <= speaks.machinery {
            if let Some(utterance) = an_utterance_of_version(version, &lines[at + 1..].join("\n")) {
                return WhatIsPinned::AMachineryMessage { id, utterance };
            }
        }
    }
    WhatIsPinned::SomethingElse { id }
}

/// **The first line carrying `mark`, and the version written after it**, where
/// that is a number.
fn the_mark(lines: &[&str], mark: &str) -> Option<(usize, Option<u32>)> {
    let at = lines.iter().position(|l| l.trim_start().starts_with(mark))?;
    Some((at, lines[at].trim()[mark.len()..].trim().parse().ok()))
}

/// **`Telegram` B9a — the reader of each board serialization, from the first to
/// this build's own.** A later build that changes the serialization adds a row
/// here and keeps every row above it; the first version's own bytes are pinned
/// in this file's tests.
fn a_board_of_version(version: u32, canonical: &str) -> Option<TheBoard> {
    match version {
        1 => serde_json::from_str(canonical).ok(),
        _ => None,
    }
}

/// **`Telegram` B9a — the reader of each machinery message serialization, from
/// the first to this build's own**, kept the same way.
fn an_utterance_of_version(version: u32, canonical: &str) -> Option<TheUtterance> {
    match version {
        1 => serde_json::from_str(canonical).ok(),
        _ => None,
    }
}

/// **A moment on this computer's own clock, as the design writes one**: a
/// complete UTC date and 24-hour time with seconds, UTC said (the design's
/// status message, *"Updated 12 Aug 2026, 14:22:07 UTC"*). Never subtracted
/// from anything.
pub fn the_moment_in_utc(at: SystemTime) -> String {
    let seconds = at.duration_since(UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
    let (days, of_day) = ((seconds / 86_400) as i64, seconds % 86_400);
    // The civil calendar from a day count, the well-known proleptic Gregorian
    // arithmetic.
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = yoe + era * 400 + i64::from(month <= 2);
    const MONTHS: [&str; 12] = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
    format!(
        "{day} {} {year}, {:02}:{:02}:{:02} UTC",
        MONTHS[(month - 1) as usize],
        of_day / 3600,
        of_day % 3600 / 60,
        of_day % 60
    )
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use serde_json::json;
    use std::time::Duration;

    pub(crate) fn a_board(holder: &str) -> TheBoard {
        TheBoard {
            holder: holder.into(),
            heartbeat: "0123abcd".into(),
            heartbeat_moment: "13 Sep 2026, 21:05:07 UTC".into(),
            renewal_seconds: 60,
            route: TheRouteMode::Asking,
            roster: vec![ARosterEntry { colour: None, machine: holder.into(), display: "RunPod 4090".into() }],
            instructions: vec![],
        }
    }

    fn pinned(id: i64, is_bot: bool, text: &str) -> Value {
        json!({ "id": 42, "pinned_message": { "message_id": id, "from": { "is_bot": is_bot }, "text": text } })
    }

    /// **`Telegram` B23: the head comes first**, so the line and the banner
    /// read as Smoozy's own note.
    #[test]
    fn every_pinned_text_opens_with_the_head() {
        let board = a_board("m1").compose().unwrap();
        assert!(board.starts_with("Smoozy \u{2014} server coordination\n"));
        let utterance = TheUtterance { machine: "m2".into(), display: "Vast 5060".into(), announce: true, ..Default::default() };
        assert!(utterance.compose().unwrap().starts_with("Smoozy \u{2014} server coordination \u{00b7} Vast 5060\n"));
    }

    #[test]
    fn a_board_and_an_utterance_read_back_as_the_same_values() {
        let board = a_board("m1");
        assert_eq!(
            what_is_pinned(&pinned(9, true, &board.compose().unwrap())),
            WhatIsPinned::ABoard { id: 9, board }
        );
        let utterance = TheUtterance { machine: "m2".into(), display: "Vast".into(), acknowledged: vec!["op".into()], ..Default::default() };
        assert_eq!(
            what_is_pinned(&pinned(10, true, &utterance.compose().unwrap())),
            WhatIsPinned::AMachineryMessage { id: 10, utterance }
        );
    }

    #[test]
    fn what_is_pinned_is_told_apart() {
        assert_eq!(what_is_pinned(&json!({ "id": 42 })), WhatIsPinned::Nothing);
        let text = a_board("m1").compose().unwrap();
        // A person cannot make a board by typing one.
        assert_eq!(what_is_pinned(&pinned(3, false, &text)), WhatIsPinned::SomethingElse { id: 3 });
        let broken = format!("{THE_HEAD}\n\nsmoozy-board 1\n{{not json");
        assert!(matches!(what_is_pinned(&pinned(4, true, &broken)), WhatIsPinned::AnUnreadableBoard { id: 4, text } if text == broken));
        let later = format!("{THE_HEAD}\n\nsmoozy-board 2\n{{}}");
        assert!(matches!(what_is_pinned(&pinned(5, true, &later)), WhatIsPinned::AnUnreadableBoard { id: 5, text } if text == later));
        assert_eq!(what_is_pinned(&pinned(6, true, "hello")), WhatIsPinned::SomethingElse { id: 6 });
    }

    /// **`Telegram` B2: the bound is counted, and a character outside the
    /// basic plane counts twice.**
    #[test]
    fn the_bound_is_counted_conservatively() {
        assert_eq!(how_long_it_counts("ab"), 2);
        assert_eq!(how_long_it_counts("\u{1F4CC}"), 2);
        let mut board = a_board("m1");
        board.roster[0].display = "x".repeat(4096);
        assert_eq!(board.compose(), Err(OverTheBound));
        assert!(!board.fits());
    }

    /// **`Telegram` B25, enforced by both simulated conversations: a test
    /// fails where a board or machinery message is sent or edited without its
    /// spoiler, where the spoiler covers any of the head or leaves anything
    /// after it plain, where markup is used instead of an entity list, or where
    /// a message for the user carries a spoiler at all.**
    ///
    /// It works the positions out on its own, in UTF-16 code units, rather
    /// than calling [`the_spoiler_after_the_head`], so a wrong rule there is
    /// caught here rather than copied. A board or machinery message is told
    /// apart by opening with B23's head, as the silence check does.
    pub(crate) fn the_spoiler_is_right(method: &str, fields: &Value) {
        if method != "sendMessage" && method != "editMessageText" {
            return;
        }
        assert!(fields.get("parse_mode").is_none(), "B25: markup was used, not an entity list: {fields}");
        let text = fields["text"].as_str().unwrap_or("");
        let spoilers: Vec<&Value> = fields
            .get("entities")
            .and_then(Value::as_array)
            .map(|all| all.iter().filter(|e| e["type"] == "spoiler").collect())
            .unwrap_or_default();
        if !text.starts_with(THE_HEAD) {
            assert!(spoilers.is_empty(), "B25: a message for the user carries a spoiler: {fields}");
            return;
        }
        assert_eq!(spoilers.len(), 1, "B25: a machinery {method} lacks its spoiler: {fields}");
        let offset = spoilers[0]["offset"].as_u64().expect("a spoiler has an offset") as usize;
        let length = spoilers[0]["length"].as_u64().expect("a spoiler has a length") as usize;
        let units: Vec<u16> = text.encode_utf16().collect();
        assert!(length > 0 && offset + length <= units.len(), "B25: the spoiler is outside the text: {fields}");
        let head = text.split("\n\n").next().unwrap_or("").encode_utf16().count();
        assert!(offset >= head, "B25: the spoiler covers the head: {fields}");
        let left_plain = String::from_utf16(&units[head..offset]).expect("the spoiler starts on a character");
        assert!(left_plain.trim().is_empty(), "B25: something after the head is left plain: {fields}");
        assert_eq!(
            offset + length,
            text.trim_end().encode_utf16().count(),
            "B25: the spoiler does not run to the end: {fields}"
        );
    }

    fn a_send(text: &str, entities: Option<Value>) -> Value {
        let mut fields = json!({ "chat_id": 42, "text": text });
        if let Some(entities) = entities {
            fields["entities"] = entities;
        }
        fields
    }

    /// **`Telegram` B25: the spoiler starts after the head's blank line and
    /// runs to the end, counted in UTF-16 code units** — a name outside the
    /// basic plane in the head moves the start by two, not one.
    #[test]
    fn the_spoiler_covers_everything_after_the_head_and_nothing_of_it() {
        let board = a_board("m1").compose().unwrap();
        let (offset, length) = the_spoiler_after_the_head(&board).unwrap();
        let plain: String = String::from_utf16(&board.encode_utf16().take(offset).collect::<Vec<_>>()).unwrap();
        assert_eq!(plain, format!("{THE_HEAD}\n{THE_BOARDS_SECOND_LINE}\n\n"));
        assert_eq!(offset + length, how_long_it_counts(&board));
        the_spoiler_is_right("sendMessage", &a_send(&board, Some(json!([{ "type": "spoiler", "offset": offset, "length": length }]))));

        let utterance = TheUtterance { machine: "m2".into(), display: "Vast \u{1F3A5}".into(), announce: true, ..Default::default() };
        let text = utterance.compose().unwrap();
        let (offset, length) = the_spoiler_after_the_head(&text).unwrap();
        assert_eq!(offset, how_long_it_counts(&format!("{THE_HEAD} \u{00b7} Vast \u{1F3A5}\n\n")));
        assert_ne!(offset, text.find("\n\n").unwrap() + 2, "counted in UTF-16 units, not in bytes");
        assert_ne!(offset, text[..text.find("\n\n").unwrap() + 2].chars().count(), "counted in UTF-16 units, not in characters");
        assert_eq!(offset + length, how_long_it_counts(&text));
        the_spoiler_is_right("editMessageText", &a_send(&text, Some(json!([{ "type": "spoiler", "offset": offset, "length": length }]))));

        assert_eq!(the_spoiler_after_the_head("Smoozy \u{2014} server coordination"), None);
        assert_eq!(the_spoiler_after_the_head("Smoozy \u{2014} server coordination\n\n  \n"), None);
        assert_eq!(the_spoiler_after_the_head("a\n\nb  \n"), Some((3, 1)));
    }

    #[test]
    #[should_panic(expected = "lacks its spoiler")]
    fn the_stand_in_check_fails_a_machinery_send_without_its_spoiler() {
        the_spoiler_is_right("sendMessage", &a_send(&a_board("m1").compose().unwrap(), None));
    }

    #[test]
    #[should_panic(expected = "lacks its spoiler")]
    fn the_stand_in_check_fails_a_machinery_edit_without_its_spoiler() {
        let text = TheUtterance { machine: "m2".into(), display: "Vast".into(), announce: true, ..Default::default() }.compose().unwrap();
        the_spoiler_is_right("editMessageText", &a_send(&text, None));
    }

    #[test]
    #[should_panic(expected = "covers the head")]
    fn the_stand_in_check_fails_a_spoiler_over_the_head() {
        let text = a_board("m1").compose().unwrap();
        let (offset, length) = the_spoiler_after_the_head(&text).unwrap();
        the_spoiler_is_right("sendMessage", &a_send(&text, Some(json!([{ "type": "spoiler", "offset": offset - 3, "length": length + 3 }]))));
    }

    #[test]
    #[should_panic(expected = "left plain")]
    fn the_stand_in_check_fails_a_spoiler_that_starts_late() {
        let text = a_board("m1").compose().unwrap();
        let (offset, length) = the_spoiler_after_the_head(&text).unwrap();
        the_spoiler_is_right("sendMessage", &a_send(&text, Some(json!([{ "type": "spoiler", "offset": offset + 3, "length": length - 3 }]))));
    }

    #[test]
    #[should_panic(expected = "for the user carries a spoiler")]
    fn the_stand_in_check_fails_a_message_for_the_user_with_a_spoiler() {
        the_spoiler_is_right("sendMessage", &a_send("Which job?", Some(json!([{ "type": "spoiler", "offset": 0, "length": 5 }]))));
    }

    #[test]
    #[should_panic(expected = "markup was used")]
    fn the_stand_in_check_fails_markup() {
        let mut fields = a_send("Smoozy \u{2014} server coordination\n\n||hidden||", None);
        fields["parse_mode"] = json!("MarkdownV2");
        the_spoiler_is_right("sendMessage", &fields);
    }

    /// **`Telegram` B25: no reader relies on the marking** — a pinned message
    /// read with its spoiler, with an empty list, or with none reads the same.
    #[test]
    fn what_is_pinned_reads_the_same_with_or_without_the_marking() {
        let board = a_board("m1");
        let text = board.compose().unwrap();
        let (offset, length) = the_spoiler_after_the_head(&text).unwrap();
        let mut with = pinned(9, true, &text);
        with["pinned_message"]["entities"] = json!([{ "type": "spoiler", "offset": offset, "length": length }]);
        let mut empty = pinned(9, true, &text);
        empty["pinned_message"]["entities"] = json!([]);
        let expected = WhatIsPinned::ABoard { id: 9, board };
        assert_eq!(what_is_pinned(&with), expected);
        assert_eq!(what_is_pinned(&empty), expected);
        assert_eq!(what_is_pinned(&pinned(9, true, &text)), expected);
    }

    /// **`Telegram` B9a — the first version's own bytes, pinned.** A board and
    /// a machinery message exactly as this build composes them, with a roster,
    /// an action entry and a context-resolution entry. **No released program
    /// has ever pinned one** — nothing on a machine runs the board — and the
    /// serialization has not changed since 2026-09-14, so these are the only
    /// shapes there are. A later build that changes either keeps these bytes
    /// and adds its own beside them.
    ///
    /// **These are the bytes as written until step 13's words sweep
    /// (2026-10-28)**, which changed only the rows for a person — the head,
    /// the second line, *Button presses* and *Pending* — and not the
    /// serialization; what this build writes is [`A_FIRST_VERSION_BOARD_NOW`]
    /// and [`A_FIRST_VERSION_MACHINERY_MESSAGE_NOW`], and both are read.
    const A_FIRST_VERSION_BOARD: &str = concat!(
        "Smoozy \u{2014} working note\n",
        "Your servers use this message to coordinate. Nothing here is for you to read or answer.\n",
        "\n",
        "Receiving: RunPod 4090 \u{00b7} 13 Sep 2026, 21:05:07 UTC\n",
        "Button presses by: checked on a timer\n",
        "Servers: RunPod 4090 \u{00b7} Vast 5060\n",
        "Waiting: stop \u{2014} sent to Vast 5060 at 13 Sep 2026, 21:05:09 UTC\n",
        "\n",
        "smoozy-board 1\n",
        r#"{"holder":"machine-a","heartbeat":"0123abcd","heartbeat_moment":"13 Sep 2026, 21:05:07 UTC","renewal_seconds":60,"route":"asking","roster":[{"machine":"machine-a","display":"RunPod 4090"},{"machine":"machine-b","display":"Vast 5060"}],"instructions":[{"version":1,"operation":"tap-cb1","addressee":{"machine":"machine-b"},"kind":"action","subjects":["job-1"],"action":"stop","payload":{"generation":"1","labels":{"button":"Stop this job","label":"Kitchen","server":"Vast 5060"}},"source_update":7,"accepted_at":"13 Sep 2026, 21:05:09 UTC","deadline_seconds":30},{"version":1,"operation":"tap-cb2","addressee":{"routing_key":"00112233445566778899aabbccddeeff"},"kind":"context_resolution","subjects":["job-1"],"action":"stop","payload":{},"source_update":7,"accepted_at":"13 Sep 2026, 21:05:10 UTC","deadline_seconds":10}]}"#,
    );
    const A_FIRST_VERSION_MACHINERY_MESSAGE: &str = concat!(
        "Smoozy \u{2014} working note \u{00b7} Vast 5060\n",
        "\n",
        "smoozy-machinery 1\n",
        r#"{"machine":"machine-b","display":"Vast 5060","announce":true,"acknowledged":["tap-cb0"],"proposed":[{"version":1,"operation":"tap-cb1","addressee":{"machine":"machine-b"},"kind":"action","subjects":["job-1"],"action":"stop","payload":{"generation":"1","labels":{"button":"Stop this job","label":"Kitchen","server":"Vast 5060"}},"source_update":7,"accepted_at":"13 Sep 2026, 21:05:09 UTC","deadline_seconds":30}],"replacing":[],"replies":[{"jobs":[],"operation":"gather-7"}]}"#,
    );

    /// **The first version as this build writes it**: the same serialization
    /// as [`A_FIRST_VERSION_BOARD`], under the rows' step 13 words.
    const A_FIRST_VERSION_BOARD_NOW: &str = concat!(
        "Smoozy \u{2014} server coordination\n",
        "Used by your servers. No action needed.\n",
        "\n",
        "Receiving: RunPod 4090 \u{00b7} 13 Sep 2026, 21:05:07 UTC\n",
        "Button presses: polled\n",
        "Servers: RunPod 4090 \u{00b7} Vast 5060\n",
        "Pending: \u{201c}Stop this job\u{201d} for Kitchen, sent to Vast 5060 at 13 Sep 2026, 21:05:09 UTC\n",
        "\n",
        "smoozy-board 1\n",
        r#"{"holder":"machine-a","heartbeat":"0123abcd","heartbeat_moment":"13 Sep 2026, 21:05:07 UTC","renewal_seconds":60,"route":"asking","roster":[{"machine":"machine-a","display":"RunPod 4090"},{"machine":"machine-b","display":"Vast 5060"}],"instructions":[{"version":1,"operation":"tap-cb1","addressee":{"machine":"machine-b"},"kind":"action","subjects":["job-1"],"action":"stop","payload":{"generation":"1","labels":{"button":"Stop this job","label":"Kitchen","server":"Vast 5060"}},"source_update":7,"accepted_at":"13 Sep 2026, 21:05:09 UTC","deadline_seconds":30},{"version":1,"operation":"tap-cb2","addressee":{"routing_key":"00112233445566778899aabbccddeeff"},"kind":"context_resolution","subjects":["job-1"],"action":"stop","payload":{},"source_update":7,"accepted_at":"13 Sep 2026, 21:05:10 UTC","deadline_seconds":10}]}"#,
    );
    const A_FIRST_VERSION_MACHINERY_MESSAGE_NOW: &str = concat!(
        "Smoozy \u{2014} server coordination \u{00b7} Vast 5060\n",
        "\n",
        "smoozy-machinery 1\n",
        r#"{"machine":"machine-b","display":"Vast 5060","announce":true,"acknowledged":["tap-cb0"],"proposed":[{"version":1,"operation":"tap-cb1","addressee":{"machine":"machine-b"},"kind":"action","subjects":["job-1"],"action":"stop","payload":{"generation":"1","labels":{"button":"Stop this job","label":"Kitchen","server":"Vast 5060"}},"source_update":7,"accepted_at":"13 Sep 2026, 21:05:09 UTC","deadline_seconds":30}],"replacing":[],"replies":[{"jobs":[],"operation":"gather-7"}]}"#,
    );

    fn the_first_versions_values() -> (TheBoard, TheUtterance) {
        let stop = AnEntry {
            version: 1,
            operation: "tap-cb1".into(),
            addressee: TheAddressee::Machine("machine-b".into()),
            kind: AKindOfEntry::Action,
            subjects: vec!["job-1".into()],
            action: "stop".into(),
            payload: json!({ "generation": "1", "labels": { "server": "Vast 5060", "label": "Kitchen", "button": "Stop this job" } }),
            source_update: Some(7),
            accepted_at: "13 Sep 2026, 21:05:09 UTC".into(),
            deadline_seconds: 30,
        };
        let tap = AnEntry {
            operation: "tap-cb2".into(),
            addressee: TheAddressee::RoutingKey("00112233445566778899aabbccddeeff".into()),
            kind: AKindOfEntry::ContextResolution,
            payload: json!({}),
            accepted_at: "13 Sep 2026, 21:05:10 UTC".into(),
            deadline_seconds: 10,
            ..stop.clone()
        };
        let mut board = a_board("machine-a");
        board.roster.push(ARosterEntry { colour: None, machine: "machine-b".into(), display: "Vast 5060".into() });
        board.instructions = vec![stop.clone(), tap];
        let utterance = TheUtterance {
            colour: None,
            machine: "machine-b".into(),
            display: "Vast 5060".into(),
            announce: true,
            acknowledged: vec!["tap-cb0".into()],
            proposed: vec![stop],
            replacing: vec![],
            replies: vec![json!({ "operation": "gather-7", "jobs": [] })],
        };
        (board, utterance)
    }

    /// **`Telegram` B9a — an older board and an older machine's message are
    /// read by a newer machine**: the first version's pinned bytes, read by a
    /// reader one version ahead of this build — the next release — and by this
    /// build, come back as the same values. Before piece 11 a reader knew only
    /// its own version's mark, so the next release would have read this board
    /// as unreadable and stood down over it (B3), and unpinned this message
    /// without reading its acknowledgement (B11).
    #[test]
    fn an_older_board_and_message_are_read_by_a_newer_machine() {
        let (board, utterance) = the_first_versions_values();
        let a_newer_machine = WhatAReaderSpeaks { board: THE_BOARD_VERSION + 1, machinery: THE_MACHINERY_VERSION + 1 };
        let this_build = WhatAReaderSpeaks { board: THE_BOARD_VERSION, machinery: THE_MACHINERY_VERSION };
        for speaks in [a_newer_machine, this_build] {
            let read = |chat: &Value| what_is_pinned_by(chat, speaks);
            for (board_text, machinery_text) in
                [(A_FIRST_VERSION_BOARD, A_FIRST_VERSION_MACHINERY_MESSAGE), (A_FIRST_VERSION_BOARD_NOW, A_FIRST_VERSION_MACHINERY_MESSAGE_NOW)]
            {
                assert_eq!(read(&pinned(9, true, board_text)), WhatIsPinned::ABoard { id: 9, board: board.clone() });
                assert_eq!(
                    read(&pinned(10, true, machinery_text)),
                    WhatIsPinned::AMachineryMessage { id: 10, utterance: utterance.clone() }
                );
            }
        }
        // And this build writes exactly the first version, under the rows'
        // step 13 words.
        assert_eq!(board.compose().unwrap(), A_FIRST_VERSION_BOARD_NOW);
        assert_eq!(utterance.compose().unwrap(), A_FIRST_VERSION_MACHINERY_MESSAGE_NOW);
    }

    /// **`Telegram` B9a — a later version is never guessed at, and this build
    /// writes only its own.** A board of a later version is unreadable (B2's
    /// cost, B3's rule); a machinery message of a later version is something
    /// else, unpinned unread (B2, B11); a version before the first, or a mark
    /// with no number, is read as nothing of this build's either.
    #[test]
    fn a_later_version_is_not_guessed_at_and_only_this_builds_is_written() {
        let later_board = A_FIRST_VERSION_BOARD.replace("smoozy-board 1", &format!("smoozy-board {}", THE_BOARD_VERSION + 1));
        assert!(matches!(what_is_pinned(&pinned(4, true, &later_board)), WhatIsPinned::AnUnreadableBoard { id: 4, .. }));
        let later_message = A_FIRST_VERSION_MACHINERY_MESSAGE
            .replace("smoozy-machinery 1", &format!("smoozy-machinery {}", THE_MACHINERY_VERSION + 1));
        assert_eq!(what_is_pinned(&pinned(5, true, &later_message)), WhatIsPinned::SomethingElse { id: 5 });
        for mark in ["smoozy-board 0", "smoozy-board", "smoozy-board one"] {
            let text = A_FIRST_VERSION_BOARD.replace("smoozy-board 1", mark);
            let read = what_is_pinned(&pinned(6, true, &text));
            if mark == "smoozy-board" {
                // No space after it, so no mark at all: not a board of Smoozy's.
                assert_eq!(read, WhatIsPinned::SomethingElse { id: 6 });
            } else {
                assert!(matches!(read, WhatIsPinned::AnUnreadableBoard { id: 6, .. }), "{mark}");
            }
        }
        let written = a_board("m1").compose().unwrap();
        assert!(written.lines().any(|l| l == format!("smoozy-board {THE_BOARD_VERSION}")));
        let said = TheUtterance { machine: "m2".into(), display: "Vast".into(), announce: true, ..Default::default() }.compose().unwrap();
        assert!(said.lines().any(|l| l == format!("smoozy-machinery {THE_MACHINERY_VERSION}")));
        // What this build writes, read by the release before it: not guessed at.
        let an_older_machine = WhatAReaderSpeaks { board: THE_BOARD_VERSION - 1, machinery: THE_MACHINERY_VERSION - 1 };
        assert!(matches!(what_is_pinned_by(&pinned(7, true, &written), an_older_machine), WhatIsPinned::AnUnreadableBoard { id: 7, .. }));
        assert_eq!(what_is_pinned_by(&pinned(8, true, &said), an_older_machine), WhatIsPinned::SomethingElse { id: 8 });
    }

    #[test]
    fn a_moment_is_written_the_way_the_design_writes_one() {
        let at = UNIX_EPOCH + Duration::from_secs(1_786_543_507);
        assert_eq!(the_moment_in_utc(at), "12 Aug 2026, 14:05:07 UTC");
        assert_eq!(the_moment_in_utc(UNIX_EPOCH), "1 Jan 1970, 00:00:00 UTC");
    }
}

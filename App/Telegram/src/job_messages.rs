//! **A job's messages, composed — `Telegram` §13 E1 to E4, §14 G1 to G4,
//! §15 V1 to V3 and the words of §16 D2, §17 A2 to A4 and §18 Y4a.**
//!
//! **Rules over values.** Everything a message is composed from — the note's
//! label, times and ending, the two collection outcomes, `Rendering` K14a's
//! cause, the manifest's reporting field, the hold record — is handed in as a
//! value, because `smoozy-queue-history`, `smoozy-job-files` and
//! `smoozy-rendering` reach the SSH client and this crate may reach none of
//! them (`tests/the_crates_shape_holds.rs`). **Reading those records on a
//! rented machine is step 8b's machine half** (`Audits/Build/Step 10/Telegram —
//! what each piece owns and in what order.md` §8 item 4). Nothing here reads a
//! record, sends anything or holds a store; `crate::actions` does the sending.
//!
//! **The words are the drawing's** (`Preparation/Design/Mockup/Smoozy.html`,
//! the `tg-fail`, `tg-check`, `tg-batch` and `tg` states; Rule 9). Where the
//! drawing has no words for a message, standard words were chosen and each is
//! recorded at `Audits/Build/Telegram/Decisions record.md`, piece 7.
//!
//! **E1: nothing here is composed from what the machine printed.** E3 and E3a:
//! no stream, no captured output and no add-on's printing is ever part of a
//! message unasked; the words travel only on a tap, read in the chat and
//! followed by a file where they do not fit (E3d, [`the_words_in_the_chat`];
//! `crate::actions` sends them). The exit status is a number and goes as E1
//! says, verbatim.
//!
//! **Markers in this file are all `Telegram`'s** unless another document is
//! named.

use std::time::{Duration, SystemTime};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::board::text::{how_long_it_counts, the_moment_in_utc, THE_BOUND_ON_A_MESSAGE};

// ------------------------------------------------------------ the values handed in

/// **`Rendering` K14a's three values**, handed in from the note (`Telegram`
/// Y4b: *"the machine's, the file's own, or undetermined"*).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TheCause {
    TheMachinesOwn,
    TheFilesOwn,
    Undetermined,
}

/// **`Rented-server runtime` M13's closed set of four, one per stream per
/// attempt** (`Telegram` E2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ACollectionOutcome {
    Collected,
    CollectedAndTheStreamWasEmpty,
    CollectionIncomplete,
    NeverEstablished,
}

/// **The two captured streams** (`Rented-server runtime` §11).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AStream {
    /// What the render printed.
    TheOrdinaryOutput,
    /// The errors the render reported.
    TheComplaints,
}

impl AStream {
    /// The words for this stream inside a sentence.
    pub fn in_words(self) -> &'static str {
        match self {
            AStream::TheOrdinaryOutput => "what the render printed",
            AStream::TheComplaints => "the errors the render reported",
        }
    }

    /// **E3d — the stream's name**: the desk's own tab names, the words the
    /// label on the words message begins with (`Preparation/Design/Mockup/
    /// Smoozy.html`, Design notes under *The phone*: *"the stream is Output or
    /// Errors, the desk's own tab names"*).
    pub fn the_name(self) -> &'static str {
        match self {
            AStream::TheOrdinaryOutput => "Output",
            AStream::TheComplaints => "Errors",
        }
    }

    /// **E3 and E3d — the label of the one button offering this stream's
    /// words, naming its stream**: *Send Output log* or *Send Errors log* —
    /// the desk's own tab name and the standard word *log* (step 13's words
    /// sweep, Rule 9; until then *Send what the render printed — Output*), so
    /// a failure with words on both streams shows two buttons told apart
    /// (ruling at the decisions record, E3d).
    pub fn the_buttons_label(self) -> String {
        format!("Send {} log", self.the_name())
    }
}

/// **`Telegram` V1 and V1a — the manifest's reporting field: one value from
/// the closed set of ~~three~~ four, as an identifier.** The set is `Render
/// Settings` `ReportingChoice`'s, written into the manifest by `Rendering` S5a
/// as its variant name; it is read here from that value and never from a
/// wording.
///
/// **The fourth, from 2026-10-19, is `APictureEveryFewMinutes`, and it alone
/// carries a value** — `{"APictureEveryFewMinutes":{"minutes":5}}`, the
/// minutes a whole number from 1 to 60 (V1a). It sends what `Everything` sends
/// ([`the_job_reports`]), the frame pictures paced by the worker (I2a).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheReportingChoice {
    Everything,
    ThePicturesWithoutTheRunningCommentary,
    APictureEveryFewMinutes { minutes: u8 },
    OnlyWhatNeedsAnAnswer,
    /// V1b: per-job silence, with explicit requests still answered.
    Nothing,
}

/// **V1a's range for the fourth member's minutes**, both ends taken.
pub const THE_FEWEST_MINUTES_BETWEEN_PICTURES: u64 = 1;
pub const THE_MOST_MINUTES_BETWEEN_PICTURES: u64 = 60;

impl TheReportingChoice {
    /// **V1a: the field is present on every job's manifest**, so the worker
    /// always reads a value. A value this build does not know is not guessed
    /// at: it is read as the narrowest choice, whose floor still sends every
    /// ending and every question (ruling at the decisions record, piece 7).
    /// **So is the fourth member with its `minutes` missing, not a whole
    /// number, or outside 1 to 60** (V1a, 2026-10-19), and any other shape
    /// beside it.
    pub fn from_the_manifests_field(field: &Value) -> TheReportingChoice {
        match field.as_str() {
            Some("Nothing") => return TheReportingChoice::Nothing,
            Some("Everything") => return TheReportingChoice::Everything,
            Some("ThePicturesWithoutTheRunningCommentary") => {
                return TheReportingChoice::ThePicturesWithoutTheRunningCommentary
            }
            _ => {}
        }
        let minutes = field
            .as_object()
            .filter(|members| members.len() == 1)
            .and_then(|members| members.get("APictureEveryFewMinutes"))
            .and_then(Value::as_object)
            .filter(|fields| fields.len() == 1)
            .and_then(|fields| fields.get("minutes"))
            .and_then(Value::as_u64)
            .filter(|minutes| {
                (THE_FEWEST_MINUTES_BETWEEN_PICTURES..=THE_MOST_MINUTES_BETWEEN_PICTURES).contains(minutes)
            });
        match minutes {
            Some(minutes) => TheReportingChoice::APictureEveryFewMinutes { minutes: minutes as u8 },
            None => TheReportingChoice::OnlyWhatNeedsAnAnswer,
        }
    }

    /// **`Telegram` V1c: the choice in force** — the job record's current
    /// choice where the desk changed it while the job was live, and the
    /// manifest's field where it never did. The record's value is JSON text in
    /// the manifest's own form, read by the same rule as the manifest; text
    /// that is not JSON is read as a value this build does not know. Added
    /// 2026-10-30, the twenty-first notes, note 7.
    pub fn in_force(written_on_the_record: Option<&str>, the_manifests_field: &Value) -> TheReportingChoice {
        match written_on_the_record {
            Some(text) => TheReportingChoice::from_the_manifests_field(
                &serde_json::from_str::<Value>(text).unwrap_or(Value::Null),
            ),
            None => TheReportingChoice::from_the_manifests_field(the_manifests_field),
        }
    }

    /// **`Telegram` V1c: a value the desk may set, read strictly** — exactly
    /// one of V1a's five shapes, the minutes inside 1 to 60. Anything else is
    /// nothing, so the supervisor refuses it rather than writing a value the
    /// worker would read as the narrowest choice.
    pub fn exactly(field: &Value) -> Option<TheReportingChoice> {
        if field.as_str() == Some("OnlyWhatNeedsAnAnswer") {
            return Some(TheReportingChoice::OnlyWhatNeedsAnAnswer);
        }
        match TheReportingChoice::from_the_manifests_field(field) {
            TheReportingChoice::OnlyWhatNeedsAnAnswer => None,
            read => Some(read),
        }
    }

    /// **`Telegram` V1d — the choice in the words the phone shows**, the five
    /// buttons' own (*Everything*, *Pictures only*, *A picture every 5
    /// minutes*, *Alerts only*, *Off*) and the reply's. Added 2026-10-31, the
    /// Telegram discussion, the server half.
    pub fn as_the_phone_says(self) -> String {
        match self {
            TheReportingChoice::Everything => "Everything".into(),
            TheReportingChoice::ThePicturesWithoutTheRunningCommentary => "Pictures only".into(),
            TheReportingChoice::APictureEveryFewMinutes { minutes: 1 } => "A picture every minute".into(),
            TheReportingChoice::APictureEveryFewMinutes { minutes } => format!("A picture every {minutes} minutes"),
            TheReportingChoice::OnlyWhatNeedsAnAnswer => "Alerts only".into(),
            TheReportingChoice::Nothing => "Off".into(),
        }
    }

    /// **The value in the manifest's own form**, which `Render Settings`
    /// `ReportingChoice` writes and [`Self::from_the_manifests_field`] reads.
    pub fn in_the_manifests_form(self) -> Value {
        match self {
            TheReportingChoice::Everything => json!("Everything"),
            TheReportingChoice::ThePicturesWithoutTheRunningCommentary => {
                json!("ThePicturesWithoutTheRunningCommentary")
            }
            TheReportingChoice::APictureEveryFewMinutes { minutes } => {
                json!({ "APictureEveryFewMinutes": { "minutes": minutes } })
            }
            TheReportingChoice::OnlyWhatNeedsAnAnswer => json!("OnlyWhatNeedsAnAnswer"),
            TheReportingChoice::Nothing => json!("Nothing"),
        }
    }

    /// **`Telegram` I2a — how long a job's frame pictures are held apart**,
    /// where its choice asks for it; nothing for every other choice, whose
    /// pictures go as they arrive.
    pub fn the_interval_between_pictures(self) -> Option<std::time::Duration> {
        match self {
            TheReportingChoice::APictureEveryFewMinutes { minutes } => {
                Some(std::time::Duration::from_secs(u64::from(minutes) * 60))
            }
            _ => None,
        }
    }
}

/// **What a machine may send about one job**, for V1's rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AKindOfReport {
    /// C1's status message about this job — the running commentary.
    TheStatusMessage,
    /// I2's pictures of frames as they arrive.
    AFramePicture,
    /// M4's moving picture.
    AMovingPicture,
    /// M2's contact sheet.
    AContactSheet,
    /// **M10's finished video**, at a finished ending, sent where the contact
    /// sheet would be. Added 2026-10-27, the nineteenth notes, note 4.
    AFinishedVideo,
    /// E1's finishing message, and G1's for a group.
    Finished,
    /// E1's failing message.
    Failed,
    /// A3's waiting-on-you message, and A4's drop. **The waiting message
    /// follows all available first-frame copies** (A3a); the copies and the
    /// separate question go under every reporting choice.
    WaitingOnYou,
    /// D2's stop outcome and every answer to a tap the user made.
    AnAnswerToATap,
    /// **C1d — the job's own message**, sent at its start and edited as the
    /// job moves, under every choice but `Nothing` (V1b). Added 2026-11-09,
    /// the twenty-eighth notes, note 18.
    TheJobsMessage,
}

/// **`Telegram` V1 — whether a job's reporting choice lets this be sent.**
/// The design names the three as *"everything, the pictures without the
/// running commentary, or only what needs an answer"*, with one floor under
/// all three: *"finished, failed and waiting on you are always sent, whichever
/// is chosen"* (`Preparation/Design/Settings.md`, *What a job reports*). **An
/// answer to a tap is always sent**: the user asked for it. **The one home of
/// this rule** — pieces 6 and 7 both read it.
///
/// **The fourth member sends what `Everything` sends** (V1a, 2026-10-19):
/// *when* its frame pictures go is I2a's, the worker's, and not this rule's.
pub fn the_job_reports(choice: TheReportingChoice, kind: AKindOfReport) -> bool {
    use AKindOfReport::*;
    if choice == TheReportingChoice::Nothing {
        return kind == AnAnswerToATap;
    }
    let choice = match choice {
        TheReportingChoice::APictureEveryFewMinutes { .. } => TheReportingChoice::Everything,
        other => other,
    };
    match kind {
        Finished | Failed | WaitingOnYou | AnAnswerToATap => true,
        AFramePicture | AMovingPicture | AContactSheet | AFinishedVideo => {
            choice != TheReportingChoice::OnlyWhatNeedsAnAnswer
        }
        TheStatusMessage => choice == TheReportingChoice::Everything,
        TheJobsMessage => true,
    }
}

/// **`Telegram` V1e's rows — the sends about a job whose sound the job's
/// choice decides.** Everything else composed for the user is B24's and keeps
/// its sound; an edit takes no setting at all (Sources entry 90). Added
/// 2026-11-09, the twenty-eighth notes, note 18.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AJobsSend {
    /// The job's message at its start (C1a, C1d).
    TheStart,
    /// A frame's picture or album (I2, I2a).
    AFramesPicture,
    /// An ending (E1), a group's message (G1), A3a's pictures and question.
    AnEndingOrAQuestion,
    /// A contact sheet (M2), a finished video and its too-large message (M10),
    /// a moving picture (M4), and I3's and M2's refusals after the ending.
    AfterTheEnding,
    /// The job's message sent again after the user deleted it (C7a), or after
    /// V1c's change away from `Nothing`.
    TheJobsMessageSentAgain,
}

/// **`Telegram` V1e — whether a send about a job makes a sound**, by the
/// job's choice. **One home, beside [`the_job_reports`]**, which decides
/// whether it is sent at all; no send decides its own sound. *"A sound for
/// what the choice is named for, and for endings and questions"*: the start
/// sounds under `Everything` alone, a picture wherever pictures go, an ending
/// or a question always, and what follows an ending or repeats the job's
/// message never. Creator-stated, the twenty-eighth notes, note 18.
pub fn it_makes_a_sound(choice: TheReportingChoice, send: AJobsSend) -> bool {
    match send {
        AJobsSend::TheStart => choice == TheReportingChoice::Everything,
        AJobsSend::AFramesPicture | AJobsSend::AnEndingOrAQuestion => true,
        AJobsSend::AfterTheEnding | AJobsSend::TheJobsMessageSentAgain => false,
    }
}

/// **`Telegram` C1d — the job's message**: R7a's head, then the job's line in
/// C1's words (`crate::status::the_state_line`), *Started.* until the first
/// progress, and any last line a refusal left (I3), each on its own line. None
/// of C1's machine readings and no uptime. ~~No moment.~~ **While the job
/// renders, its message's keeper adds C1d's *Updated* line after these**
/// ([`AMarkedText::with_when_it_was_updated`], the twenty-ninth notes, note 9).
pub fn the_jobs_message(
    server: &(impl TheServersName + ?Sized),
    label: &str,
    state_line: Option<&str>,
    last_lines: &[&str],
) -> AMarkedText {
    let mut body = state_line.unwrap_or("Started.").to_string();
    for line in last_lines {
        body.push('\n');
        body.push_str(line);
    }
    a_user_message(server, label, "", &body)
}

/// **`Telegram` C1d's *Updated* line**: its word, and the `date_time`
/// entity's format, `r`, *"Displays the time relative to the current time"*
/// (Sources entry 91).
pub const THE_UPDATED_LINES_WORD: &str = "Updated";
pub const THE_UPDATED_LINES_FORMAT: &str = "r";

/// **`Telegram` C1e's state words, the job's message cut to one line ending in
/// one of them** — as R7a words each ending, a failure keeping its 🔴.
pub const A_FINISHED_ENDING: &str = "Finished.";
pub const A_FAILED_ENDING: &str = "🔴 Failed.";
pub const A_STOPPED_ENDING: &str = "Stopped.";
/// *Interrupted.* since 2026-11-11 (the four questions, scout 3.24): it read
/// *Abandoned.*, as if the user gave up.
pub const AN_ABANDONED_ENDING: &str = "Interrupted.";
/// **V1c's change to `Nothing`** (C1e).
pub const REPORTING_OFF: &str = "Reporting off.";
/// **A job removed from the queue with no ending of its own** — a cancel, a
/// clear, or a drop for a removed Blender. The build's word: C1e names none
/// for it, and a message left saying *Queued.* would go stale (Specs
/// `Telegram`, *One message per job*: the cut is so it no longer says anything
/// that has stopped being true).
pub const A_REMOVED_JOB: &str = "Removed.";

/// **`Telegram` C1e — the job's message cut to one line**: R7a's mark, the
/// server label in bold, ` · `, the job label in italic, ` · `, then the state
/// word — R7a's one exception to its two-line head.
pub fn the_jobs_message_cut(server: &(impl TheServersName + ?Sized), label: &str, ended: &str) -> AMarkedText {
    let mut out = AMarkedText::default();
    if let Some(mark) = the_colour_square(server.colour()) {
        out.append(&format!("{mark} "), None);
    }
    out.append(server.label(), Some("bold"));
    out.append(" \u{b7} ", None);
    out.append(label, Some("italic"));
    out.append(&format!(" \u{b7} {ended}"), None);
    out
}

/// `Telegram` R7a: marked runs travel beside their text, never as markup.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AMarkedText {
    pub text: String,
    pub entities: Vec<Value>,
}

impl AMarkedText {
    pub fn plain(text: impl Into<String>) -> Self { Self { text: text.into(), entities: Vec::new() } }
    pub fn append(&mut self, text: &str, kind: Option<&str>) {
        let offset = self.text.encode_utf16().count();
        self.text.push_str(text);
        let length = text.encode_utf16().count();
        if let Some(kind) = kind.filter(|_| length > 0) {
            self.entities.push(json!({"type": kind, "offset": offset, "length": length}));
        }
    }
    pub fn joined(&mut self, other: &Self) {
        let offset = self.text.encode_utf16().count();
        self.text.push_str(&other.text);
        self.entities.extend(other.entities.iter().cloned().map(|mut entity| {
            entity["offset"] = json!(entity["offset"].as_u64().unwrap() + offset as u64);
            entity
        }));
    }
    pub fn push_str(&mut self, text: &str) { self.append(text, None); }
    /// **`Telegram` C1d's *Updated* line, as of 2026-11-10 (the twenty-ninth
    /// notes, note 9)**: a new last line, *Updated* and then Telegram's
    /// `date_time` entity, format `r`, holding `at` (Sources entry 91). The
    /// entity's own text is `at` in C2a's form, so an app that cannot draw
    /// the age still reads one named UTC instant. Smoozy computes no age.
    pub fn with_when_it_was_updated(&mut self, at: SystemTime) {
        self.append(&format!("\n{THE_UPDATED_LINES_WORD} "), None);
        let moment = the_moment_in_utc(at);
        let offset = self.text.encode_utf16().count();
        self.text.push_str(&moment);
        let unix_time = at.duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
        self.entities.push(json!({
            "type": "date_time",
            "offset": offset,
            "length": moment.encode_utf16().count(),
            "unix_time": unix_time,
            "date_time_format": THE_UPDATED_LINES_FORMAT,
        }));
    }
    pub fn as_str(&self) -> &str { &self.text }
    pub fn entities(&self) -> Value { json!(self.entities) }
}
impl std::ops::Deref for AMarkedText { type Target = str; fn deref(&self) -> &str { &self.text } }
impl std::fmt::Display for AMarkedText { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { self.text.fmt(f) } }
impl From<String> for AMarkedText { fn from(text: String) -> Self { Self::plain(text) } }
impl From<&str> for AMarkedText { fn from(text: &str) -> Self { Self::plain(text) } }
impl PartialEq<&str> for AMarkedText { fn eq(&self, text: &&str) -> bool { self.text == *text } }
impl PartialEq<String> for AMarkedText { fn eq(&self, text: &String) -> bool { self.text == *text } }

/// `Telegram` L1/R7a: a display label carries its colour explicitly.
/// Old records use a plain string; coloured records add display material only.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum AServerLabel {
    Plain(String),
    Coloured { label: String, colour: String },
}
impl AServerLabel {
    pub fn with_colour(label: &str, colour: Option<&str>) -> Self {
        match colour { Some(colour) => Self::Coloured { label: label.into(), colour: colour.into() }, None => Self::Plain(label.into()) }
    }
    pub fn colour(&self) -> Option<&str> { match self { Self::Coloured { colour, .. } => Some(colour), _ => None } }
}
impl Default for AServerLabel { fn default() -> Self { Self::Plain(String::new()) } }
impl From<String> for AServerLabel { fn from(label: String) -> Self { Self::Plain(label) } }
impl From<&str> for AServerLabel { fn from(label: &str) -> Self { Self::Plain(label.into()) } }
impl std::ops::Deref for AServerLabel { type Target = str; fn deref(&self) -> &str { match self { Self::Plain(label) | Self::Coloured { label, .. } => label } } }
impl std::fmt::Display for AServerLabel { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { std::fmt::Display::fmt(&**self, f) } }
impl PartialEq<&str> for AServerLabel { fn eq(&self, text: &&str) -> bool { &**self == *text } }
impl PartialEq<String> for AServerLabel { fn eq(&self, text: &String) -> bool { &**self == text } }

pub trait TheServersName: serde::Serialize {
    fn label(&self) -> &str;
    fn colour(&self) -> Option<&str> { None }
    fn named(&self) -> AServerLabel { AServerLabel::with_colour(self.label(), self.colour()) }
}
impl TheServersName for str { fn label(&self) -> &str { self } }
impl TheServersName for String { fn label(&self) -> &str { self } }
impl TheServersName for AServerLabel {
    fn label(&self) -> &str { self }
    fn colour(&self) -> Option<&str> { self.colour() }
}

pub fn the_colour_square(colour: Option<&str>) -> Option<&'static str> {
    match colour {
        Some("red") => Some("🟥"), Some("gold") => Some("🟧"), Some("green" | "jade") => Some("🟩"),
        Some("cyan" | "blue") => Some("🟦"), Some("violet" | "magenta") => Some("🟪"), _ => None,
    }
}

/// `Telegram` R7a: the display line may begin with L1's square. Only the
/// label is bold; the job alone is italic, and detail and body remain plain.
pub fn a_user_message(server: &(impl TheServersName + ?Sized), job: &str, detail: &str, body: &str) -> AMarkedText {
    let mut out = AMarkedText::default();
    if let Some(mark) = the_colour_square(server.colour()) { out.append(&format!("{mark} "), None); }
    out.append(server.label(), Some("bold"));
    if !job.is_empty() { out.append("\n", None); out.append(job, Some("italic")); }
    out.append(detail, None);
    if !body.is_empty() { out.append("\n", None); out.append(body, None); }
    out
}

// ------------------------------------------------------------ a composed message

/// **A button a message offers**, before its action-context record exists.
/// `crate::actions` writes the record (R1) and only then adds the button.
#[derive(Debug, Clone, PartialEq)]
pub struct AButtonToOffer {
    pub label: String,
    pub action: &'static str,
    /// The action-context record's payload (R1). Carries the labels the
    /// answers name (R7), never anything that routes.
    pub payload: Value,
}

/// **One message for the user**: its words, and its buttons in order.
#[derive(Debug, Clone, PartialEq)]
pub struct AComposedMessage {
    pub text: AMarkedText,
    pub buttons: Vec<AButtonToOffer>,
}

/// C1b/D0: the first tap only asks; the affirmative confirmation is marked
/// in its durable record, never inferred from the button's visible words.
pub fn a_stop_to_offer(server: &(impl TheServersName + ?Sized), label: &str, confirmed: bool) -> AButtonToOffer {
    AButtonToOffer {
        label: "Stop this job".into(),
        action: if confirmed {
            STOP_THIS_JOB
        } else {
            ASK_TO_STOP_THIS_JOB
        },
        payload: json!({"labels": {"server": server, "label": label, "button": "Stop this job"}, "confirmed": confirmed}),
    }
}

pub fn the_start_message(server: &(impl TheServersName + ?Sized), label: &str) -> AMarkedText {
    a_user_message(server, label, "", "Started.")
}

/// **`Telegram` D0's question.** A waiting job's is the window's own cancel
/// question, *Cancel this job?* with *Keep the job* and *Cancel the job* (was
/// *Cancel this waiting job?*, *Keep waiting* and *Stop this job* — the
/// Creator's twenty-eighth notes, note 16); a rendering job's is *Stop this
/// job? The current frame may not be saved.* with *Keep rendering* and *Stop
/// this job*. The confirmation is the same act either way: a waiting job is
/// cancelled, a rendering one stopped.
pub fn the_stop_question(server: &(impl TheServersName + ?Sized), label: &str, waiting: bool) -> AComposedMessage {
    the_stop_question_for(server, label, if waiting { &AStopQuestion::AWaitingJob } else { &AStopQuestion::ARender })
}

/// **Which of `Telegram` D0's questions a Stop asks.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AStopQuestion {
    /// A rendering job.
    ARender,
    /// A first-frame check with its range held on it (`Queue & History`
    /// L19a), its one frame and the range in words.
    ACheck { first_frame: String, the_frames: String },
    /// A job waiting its turn.
    AWaitingJob,
    /// **A range waiting its turn after *Render all*** — C1d's *Cancel this
    /// job* on its job's message, asking with the pair's sentence. The
    /// twenty-eighth notes, note 16, its design item 9.
    AWaitingRange { the_frames: String },
}

impl AStopQuestion {
    fn is_a_cancel(&self) -> bool {
        matches!(self, AStopQuestion::AWaitingJob | AStopQuestion::AWaitingRange { .. })
    }
}

/// **`Telegram` D0 — the question's own words, below the head**: what a
/// question sent as a new message says, and what the job's message gains when
/// the question is asked in place (C1d).
pub fn the_stop_questions_words(question: &AStopQuestion) -> String {
    match question {
        AStopQuestion::ARender => "Stop this job?\nThe current frame may not be saved.".into(),
        AStopQuestion::ACheck { first_frame, the_frames } => {
            format!("Stop this job?\nFrame {first_frame} may not be saved. Frames {the_frames} will not render.")
        }
        AStopQuestion::AWaitingJob => "Cancel this job?".into(),
        AStopQuestion::AWaitingRange { the_frames } => format!(
            "Cancel this job?\nFrames {the_frames} have not started. Their settings are deleted too. The first-frame check stays in History."
        ),
    }
}

/// **`Telegram` D0's question, whole** — its words and its two buttons: *Keep
/// rendering* and *Stop this job*, or for a waiting job *Keep the job* and
/// *Cancel the job*.
pub fn the_stop_question_for(server: &(impl TheServersName + ?Sized), label: &str, question: &AStopQuestion) -> AComposedMessage {
    let waiting = question.is_a_cancel();
    let mut keep = a_stop_to_offer(server, label, false);
    keep.action = KEEP_RENDERING;
    keep.label = if waiting {
        "Keep the job"
    } else {
        "Keep rendering"
    }
    .into();
    keep.payload["labels"]["button"] = json!(keep.label);
    let mut go = a_stop_to_offer(server, label, true);
    if waiting {
        go.label = "Cancel the job".into();
        go.payload["labels"]["button"] = json!(go.label);
    }
    AComposedMessage {
        text: a_user_message(server, label, "", &the_stop_questions_words(question)),
        buttons: vec![keep, go],
    }
}

/// **`Telegram` D0 for a first-frame check whose range waits on it**: *Stop
/// this job? Frame 1 may not be saved. Frames 1–240 will not render.* with
/// *Keep rendering* and *Stop this job* — the confirmation ends the whole job
/// (`Queue & History` L19a). The Creator's twenty-eighth notes, note 16.
pub fn the_checks_stop_question(
    server: &(impl TheServersName + ?Sized),
    label: &str,
    first_frame: &str,
    the_frames: &str,
) -> AComposedMessage {
    the_stop_question_for(
        server,
        label,
        &AStopQuestion::ACheck { first_frame: first_frame.into(), the_frames: the_frames.into() },
    )
}

pub fn the_job_already_ended(server: &(impl TheServersName + ?Sized), label: &str) -> AMarkedText {
    a_user_message(server, label, "", "This job has already ended.")
}

/// **The actions a button can carry** — the four phone actions (`Telegram`
/// §16, §17, §18) and E3's words. Written on the record and on the board entry
/// (R1); nothing routes by them.
pub const STOP_AN_ENDED_JOB: &str = "stop an ended job";
pub const ASK_TO_STOP_THIS_JOB: &str = "ask to stop this job";
pub const KEEP_RENDERING: &str = "keep rendering";
pub const STOP_THIS_JOB: &str = "stop this job";
pub const ANSWER_A_WAITING_JOB: &str = "answer a waiting job";
/// **`Telegram` A2 as changed 2026-11-09: the first-frame question's *Cancel
/// job* asks in place** before the *no* is sent; *Keep the job* puts the two
/// answers back. The Creator's twenty-eighth notes, note 16.
pub const ASK_TO_CANCEL_THE_JOB: &str = "ask to cancel the job";
pub const KEEP_THE_JOB: &str = "keep the job";
pub const TRY_IT_AGAIN: &str = "try it again";
/// **`Telegram` Y4a as changed 2026-11-11 (the four questions, scout 1.5)**:
/// *Resume*, Y1's submission from the frame the payload carries.
pub const RESUME_IT: &str = "resume it";
pub const SEND_THE_WORDS: &str = "send the words";

// ------------------------------------------------------------ E1 to E3, and Y4a

/// **One failed job, as the note and the journal hold it** (E1; `Queue &
/// History` N1, N3).
#[derive(Debug, Clone, PartialEq)]
pub struct AFailedJob {
    /// The server label the machine was last told (R7, R8).
    pub server: AServerLabel,
    pub job: String,
    /// **What the job is called** (`Telegram` R7, `Telegram` R7a): the note's
    /// label, whole (`Queue & History` N1), or the `.blend`'s name where the
    /// label is empty. **Every job's name in this file is that value**, handed
    /// in by the machine from `TheNote::what_the_job_is_called` in
    /// `smoozy-queue-history-machine`, never the note's label alone.
    pub label: String,
    /// The subject generation the buttons are valid in (R1).
    pub generation: String,
    pub attempt: u32,
    /// The frame the render was on, and the job's frame count, where known.
    /// **The frame is signed** — `Specs/Render Settings`: a range may start
    /// below zero (the four questions, batch 4's S2 leftover).
    pub at_frame: Option<(i64, u64)>,
    /// The exit status verbatim; nothing where none was reaped (`Rented-server
    /// runtime` M21).
    pub exit_status: Option<i32>,
    pub frames_on_the_server: u64,
    pub ordinary_output: ACollectionOutcome,
    pub complaints: ACollectionOutcome,
    /// `Rendering` K14a.
    pub cause: TheCause,
    /// **The free space in bytes against the job's own output folder, read
    /// at the ending** (`Rented-server runtime` V12, carried on the note's
    /// ending), or nothing where none was read. `Whole app` carves out this
    /// one cause Smoozy can read off the machine (`Telegram` E1 carries its
    /// sentence).
    pub disk_free_at_the_ending: Option<u64>,
    /// **Whether the attempt ended before any render began** (the note's ending,
    /// `Rendering` K14). `Design/Queue & History`'s *Failed before rendering.*, in
    /// this message's short form.
    pub no_render_began: bool,
    /// **The reason the machine recorded** (the note's ending), said where no
    /// render began and where Blender exited at 0 and the job still failed: it
    /// is Smoozy's own sentence and nothing is read from a stream (`Telegram` E1).
    pub the_reason_the_machine_recorded: Option<String>,
    /// **How many frames the job asked for**, for *Submit again — 240 frames*;
    /// none read, and the button names no count (`Telegram` Y4a as changed
    /// 2026-11-11, the four questions, scout 1.5).
    pub frames_asked: u64,
    /// **Where *Resume* would start, and how many frames it holds** — the
    /// note's `Queue & History` N6 figure and the asked frames from it on, as
    /// the app's History reckons them (`Rendering` A4, A5). None where the app
    /// would not offer it.
    pub resume: Option<AResume>,
}

/// **The phone's *Resume*** (`Telegram` Y4a as changed 2026-11-11): the frame
/// the new job starts at, and how many frames it renders.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AResume {
    pub from: i64,
    pub frames: u64,
}

/// **The app's own label for one of the two ways** (`Design/Queue & History`,
/// *Submit again — 240 frames*, *Resume — 181 frames*): the count where one is
/// known.
fn one_of_the_two_ways(name: &str, frames: u64) -> String {
    if frames == 0 {
        return name.to_string();
    }
    format!("{name} \u{2014} {}", frames_in_words(frames))
}

/// **`Whole app`'s sentence for the one failure whose cause Smoozy can read**:
/// where the free space read at the ending is none, the failure says so and to
/// free space, **beside** the machine's own words (`Telegram` E1).
///
/// **One sentence for the one diagnosis, and it is the desktop's.** The
/// reading is per folder, so the sentence names the folder and never the
/// server's disk (`Desktop app and delivery` M17, `Server protocol and
/// transfers` F16) — the words of `the_disk_full_carve_out.rs`'s
/// `THE_DIAGNOSIS` in `smoozy-desktop`, whose own test holds the two the same,
/// on the runner's ruling of 2026-10-25 (step 12's SC4 findings). *(It read
/// "The server's disk is full. Free some space on it before sending the job
/// again." until then, which no reading here ever fired.)*
///
/// *(Shortened 2026-10-28 with the desktop's, the final words sweep: it read
/// *The folder this job wrote to had no space left when the render ended, so
/// free space on the server and render it again.*; and again 2026-11-01,
/// the second words sweep, with the desktop's: *…; free space on the server
/// and render again.*)*
pub const THE_DISK_IS_FULL: &str =
    "The output folder on the server was full; free space, then Submit again.";

/// **Y4a's sentence where the cause is *the file's own*, and only there.** The
/// drawing has no words for it; the spec's are *"the message says the job
/// needs a change before it can go again"* (`Preparation/Specs/Telegram.md`),
/// said only where the file itself failed.
pub const IT_NEEDS_A_CHANGE: &str = "Change the job in Smoozy, then submit again.";

/// **Y4a's sentence where the cause is *undetermined*** (corrected
/// 2026-09-16). Smoozy cannot name the cause, so the sentence says so, says
/// where the job can be sent again, and says nothing about the file. The
/// drawing does not draw this case; its desk wording for the same fact is
/// *"Smoozy cannot tell why this one did"*, and *Submit again* is the desk's
/// own button (`Preparation/Design/Mockup/Smoozy.html`).
///
/// *(Shortened 2026-11-01, the second words sweep: it read *Cause unknown. To
/// resubmit, use Submit again in Smoozy on your computer.*)* *(And 2026-11-12,
/// `Telegram` Y4a as changed that day: it read *Cause unknown. Use Submit again
/// in Smoozy.*, and the message now carries *Submit again* itself.)*
pub const SMOOZY_CANNOT_TELL_WHY: &str = "Cause unknown.";

/// **E2 — the sentence for one stream's outcome**, or nothing where *collected*
/// says it with its button alone. **Collected-and-empty and never-established
/// are two sentences and never one** (E2, `Desktop app and delivery` M10).
pub fn what_the_outcome_says(stream: AStream, outcome: ACollectionOutcome) -> Option<String> {
    use ACollectionOutcome::*;
    use AStream::*;
    Some(match (stream, outcome) {
        (_, Collected) => return None,
        // The drawing's own wording for the complaints stream. **Reworded
        // 2026-10-11, the Creator's eighth notes, note 6**: both said the render
        // reported no errors or printed nothing *and there is nothing further
        // to read*, which spoke for the other stream; Blender 5 writes its
        // errors to its ordinary output. Each now claims only its own stream.
        (TheComplaints, CollectedAndTheStreamWasEmpty) => "Errors log is empty.".to_string(),
        (TheOrdinaryOutput, CollectedAndTheStreamWasEmpty) => "Output log is empty.".to_string(),
        // **Step 13's words sweep (Rule 9)**: the desk's tab names with the
        // standard word *log*. *No output was recorded.* is the desk panel's
        // own words for *never established*.
        (TheOrdinaryOutput, CollectionIncomplete) => "Output log incomplete.".to_string(),
        (TheComplaints, CollectionIncomplete) => "Errors log incomplete.".to_string(),
        (TheOrdinaryOutput, NeverEstablished) => "Output log not recorded.".to_string(),
        (TheComplaints, NeverEstablished) => "Errors log not recorded.".to_string(),
    })
}

/// **E2 as narrowed 2026-10-31 — an empty Errors stream is said only where
/// the Output log holds no words either.** Where the ordinary stream's words
/// are offered (*collected*, or *collection incomplete*), its *Send Output log*
/// is the message's whole account of what Blender printed, and nothing is said
/// of Errors: Blender 5 writes its errors to its ordinary output, so *Blender
/// wrote nothing under Errors.* beside a log holding `ERROR Cannot render, no
/// camera` read as *no errors* (the live Telegram pass of 2026-10-31, fault 7;
/// Rule 8). The drawing's `tg-fail` already says nothing of Errors there.
fn an_empty_errors_stream_goes_unsaid(stream: AStream, outcome: ACollectionOutcome, ordinary_output: ACollectionOutcome) -> bool {
    stream == AStream::TheComplaints
        && outcome == ACollectionOutcome::CollectedAndTheStreamWasEmpty
        && the_words_are_offered(ordinary_output)
}

/// **E3 — whether a stream's words are offered behind a button**: where the
/// outcome is *collected* and the stream is not empty, and where it is
/// *collection incomplete*, offered marked as incomplete (E2, `Rented-server
/// runtime` M26). One button per such stream.
pub fn the_words_are_offered(outcome: ACollectionOutcome) -> bool {
    matches!(outcome, ACollectionOutcome::Collected | ACollectionOutcome::CollectionIncomplete)
}

/// **Y4a — the *try it again* button is attached at composition, from the
/// recorded cause, and never at the tap**: ~~only on the machine's own~~ on
/// every cause but the file's own, which keeps its *needs a change* sentence
/// ([`what_is_said_instead_of_try_it_again`]). *Changed 2026-11-12, `Telegram`
/// Y4a as changed that day, Creator-stated: the phone matches the app, whose
/// History offers both ways on an undetermined cause.*
pub fn try_it_again_is_offered(cause: TheCause) -> bool {
    cause != TheCause::TheFilesOwn
}

/// **The app's two ways as buttons** (`Telegram` Y4a): *Submit again* with the
/// frames asked, and *Resume* beside it where there is a frame to start from.
fn the_two_ways(labels: &Value, frames_asked: u64, resume: Option<AResume>) -> Vec<AButtonToOffer> {
    let mut buttons = vec![AButtonToOffer {
        label: one_of_the_two_ways("Submit again", frames_asked),
        action: TRY_IT_AGAIN,
        payload: json!({ "labels": labels }),
    }];
    if let Some(resume) = resume {
        buttons.push(AButtonToOffer {
            label: one_of_the_two_ways("Resume", resume.frames),
            action: RESUME_IT,
            payload: json!({ "labels": labels, "from": resume.from }),
        });
    }
    buttons
}

/// **Y4a, corrected 2026-09-16 — the sentence a failure carries where no *try
/// it again* is attached, one per cause.** *The file's own* says the job needs
/// a change; *undetermined* says Smoozy cannot tell why and where the job can
/// be sent again, and never that the file is at fault. **The one place the
/// cause becomes words**; every ending `Rendering` K14a records as
/// *undetermined* reaches the user through it.
pub fn what_is_said_instead_of_try_it_again(cause: TheCause) -> Option<&'static str> {
    match cause {
        TheCause::TheMachinesOwn => None,
        TheCause::TheFilesOwn => Some(IT_NEEDS_A_CHANGE),
        TheCause::Undetermined => Some(SMOOZY_CANNOT_TELL_WHY),
    }
}

/// **`Telegram` E1, E2, E3 and Y4a — the failing message.** The drawing's
/// `tg-fail`: *"Kitchen — final failed at frame 148 of 240. Blender exited
/// with code 1."*, *"147 frames are on the server and can be downloaded."*,
/// then the buttons.
pub fn the_failure_message(failed: &AFailedJob) -> AComposedMessage {
    let mut lines = Vec::new();
    if failed.no_render_began {
        // **`Telegram` E1, 2026-11-04: `Design/Queue & History`'s words.** *Blender
        // ended without an exit status* was false: no render had started, and on the
        // film's first job no render Blender had even been launched.
        lines.push("🔴 Failed before rendering.".to_string());
        if let Some(reason) = failed.the_reason_the_machine_recorded.as_deref().filter(|reason| !reason.is_empty()) {
            let mut letters = reason.chars();
            lines.extend(letters.next().map(|first| first.to_uppercase().chain(letters).collect::<String>()));
        }
    } else {
        let at = match failed.at_frame {
            Some((frame, of)) => format!(" at frame {frame} of {of}"),
            None => String::new(),
        };
        let status = match failed.exit_status {
            // **An exit at 0 is a failure only by the machine's own reason**
            // (`Rendering` K14b, or a render that never reported starting), so
            // the phone says that reason, as History does. *Changed 2026-11-06,
            // the last words read before the release: it read *Blender ended
            // with status 0.* beside *Failed*.*
            Some(0) => failed
                .the_reason_the_machine_recorded
                .as_deref()
                .filter(|reason| !reason.is_empty())
                .map(|reason| {
                    let mut letters = reason.chars();
                    let first: String = letters.next().map(|first| first.to_uppercase().collect()).unwrap_or_default();
                    format!("{first}{}.", letters.as_str())
                })
                .unwrap_or_else(|| "Blender exited normally but wrote none of the job's files.".to_string()),
            // **`Telegram` E1 as changed 2026-11-11** (the four questions, scout
            // 3.25): the code stays in the log, since a bare number is not
            // something an artist acts on.
            Some(_) => "Blender exited with an error.".to_string(),
            None => "Blender quit unexpectedly.".to_string(),
        };
        lines.push(format!("🔴 Failed{at}. {status}"));
    }
    // `Whole app` and `Telegram` E1: the disk is full, beside the exit status.
    // **It says *when the render ended***, so not where none began.
    if failed.disk_free_at_the_ending == Some(0) && !failed.no_render_began {
        lines.push(THE_DISK_IS_FULL.to_string());
    }
    lines.push(match failed.frames_on_the_server {
        0 => "No frames on the server.".to_string(),
        1 => "1 frame on the server.".to_string(),
        n => format!("{} frames on the server.", with_thousands(n)),
    });
    let streams = [(AStream::TheOrdinaryOutput, failed.ordinary_output), (AStream::TheComplaints, failed.complaints)];
    lines.extend(
        streams
            .iter()
            .filter(|(stream, outcome)| !an_empty_errors_stream_goes_unsaid(*stream, *outcome, failed.ordinary_output))
            // **Where no render began and no stream was opened, there is nothing to
            // say of either**: the headline has said it.
            .filter(|(_, outcome)| !(failed.no_render_began && *outcome == ACollectionOutcome::NeverEstablished))
            .filter_map(|(stream, outcome)| what_the_outcome_says(*stream, *outcome)),
    );
    if let Some(sentence) = what_is_said_instead_of_try_it_again(failed.cause) {
        lines.push(sentence.to_string());
    }

    let labels = json!({ "server": failed.server, "label": failed.label });
    let mut buttons: Vec<AButtonToOffer> = streams
        .iter()
        .filter(|(_, outcome)| the_words_are_offered(*outcome))
        .map(|(stream, outcome)| AButtonToOffer {
            label: stream.the_buttons_label(),
            action: SEND_THE_WORDS,
            payload: json!({
                "labels": labels,
                "attempt": failed.attempt,
                "stream": stream,
                "incomplete": *outcome == ACollectionOutcome::CollectionIncomplete,
            }),
        })
        .collect();
    // **`Telegram` Y4a as changed 2026-11-11, Creator-stated (the four
    // questions, scout 1.5): the app's two ways, never *Retry*.**
    if try_it_again_is_offered(failed.cause) {
        buttons.extend(the_two_ways(&labels, failed.frames_asked, failed.resume));
    }
    AComposedMessage { text: a_user_message(&failed.server, &failed.label, "", &lines.join("\n")), buttons }
}

/// **One finished job** (E1; the drawing's `tg` state, *"Kitchen — final
/// finished. 240 frames in 6 hours 2 minutes."*).
#[derive(Debug, Clone, PartialEq)]
pub struct AFinishedJob {
    pub server: AServerLabel,
    pub label: String,
    pub frames: u64,
    /// The note's two times, **both on the machine's own clock** (`State,
    /// identity and recovery` D5), so the difference is one clock's.
    pub started: SystemTime,
    pub ended: SystemTime,
    /// Where the job belongs to a group, G1 speaks for it instead.
    pub group: Option<String>,
    /// **E1a: C27's files written and files owed**, where the ending carried a
    /// statement.
    pub files: Option<(u64, u64)>,
}

/// **E1a: *3 of 4 files written*, where the count came up short.**
fn a_short_count_in_words(files: Option<(u64, u64)>) -> Option<String> {
    let (written, owed) = files.filter(|(written, owed)| written < owed)?;
    Some(format!(
        "{} of {} {} written",
        with_thousands(written),
        with_thousands(owed),
        if owed == 1 { "file" } else { "files" }
    ))
}

/// **E1 and G1 — the finishing message for one job, or nothing where the job
/// belongs to a group**, whose one message is G1's (the drawing's `tg-batch`:
/// *"One message for the group"*).
pub fn the_finishing_message(finished: &AFinishedJob) -> Option<AMarkedText> {
    if finished.group.is_some() {
        return None;
    }
    let took = finished.ended.duration_since(finished.started).ok();
    Some(a_user_message(&finished.server, &finished.label, "", &format!(
        "Finished. {}{}.",
        a_short_count_in_words(finished.files).unwrap_or_else(|| frames_in_words(finished.frames)),
        took.map(|d| format!(" in {}", how_long_in_words(d))).unwrap_or_default()
    )))
}

/// **E1a's sentence of 2026-11-11: one line under a finishing message for
/// each path the remap left unchanged — *Missing: <file name>*** (`Rendering`
/// K14c). The names are the user's own file names, carried as written.
/// Nothing is added where there is none.
///
/// **The lines stop where the message would pass Telegram's bound**, and a
/// last line says how many more: *and 84 more.* (the UI at every size, T-03;
/// `Telegram` E1a as changed that day). Telegram refuses a message past
/// B2's 4,096 whole (`Preparation/Architecture/Sources/Telegram.sources.md`,
/// entries 39 and 90: `sendMessage` text is *"1-4096 characters after entity
/// parsing"*, read 2026-08-20 and 2026-11-09), and a refused finish is the
/// phone never told the job finished.
pub fn with_the_files_missing(mut message: AMarkedText, missing: &[String]) -> AMarkedText {
    let units = |text: &str| text.encode_utf16().count();
    for (at, name) in missing.iter().enumerate() {
        let line = format!("\nMissing: {name}");
        let left = missing.len() - at - 1;
        // Room for this line, and for the closing count should any follow it.
        let after = if left > 0 { units(&the_files_not_named(left)) } else { 0 };
        if units(&message.text) + units(&line) + after > THE_BOUND_ON_A_MESSAGE {
            message.append(&the_files_not_named(missing.len() - at), None);
            return message;
        }
        message.append(&line, None);
    }
    message
}

/// The closing line of a long *Missing:* list: how many names it left out.
fn the_files_not_named(how_many: usize) -> String {
    format!("\nand {how_many} more.")
}

/// **One job given up after a second interruption** (`Queue & History` N1;
/// the spec: *"said the same way"* as a failure).
#[derive(Debug, Clone, PartialEq)]
pub struct AGivenUpJob {
    pub server: AServerLabel,
    pub job: String,
    pub label: String,
    pub generation: String,
    pub attempt: u32,
    pub frames_on_the_server: u64,
    pub ordinary_output: ACollectionOutcome,
    pub complaints: ACollectionOutcome,
    /// **How many frames the job asked for, and where *Resume* would start**,
    /// as on [`AFailedJob`]: `Telegram` Y4a as changed 2026-11-12, the phone
    /// matching the app's History on a job given up.
    pub frames_asked: u64,
    pub resume: Option<AResume>,
}

/// **`Telegram` E1 and E3 — the given-up message.** The server, the label,
/// *was interrupted twice and abandoned* (step 13's final words sweep; until
/// then *was given up after being interrupted twice*), the frames on the server, and
/// the words button per stream where there are words (E3), ~~**No *try it
/// again***: no failure cause was classified, and Y4a attaches that button
/// only on a recorded cause of the machine's own.~~ and since 2026-11-12 the
/// app's two ways after them (`Telegram` Y4a as changed that day). Standard words that Design
/// may replace; added by the round-1 code audit of the bot and the chat,
/// fix-here item 11, since *failed* with an exit status is not true here.
pub fn the_given_up_message(given_up: &AGivenUpJob) -> AComposedMessage {
    let mut lines = vec!["Interrupted twice. No more retries.".to_string()];
    lines.push(match given_up.frames_on_the_server {
        0 => "No frames on the server.".to_string(),
        1 => "1 frame on the server.".to_string(),
        n => format!("{} frames on the server.", with_thousands(n)),
    });
    let labels = json!({ "server": given_up.server, "label": given_up.label });
    let streams = [(AStream::TheOrdinaryOutput, given_up.ordinary_output), (AStream::TheComplaints, given_up.complaints)];
    let mut buttons: Vec<AButtonToOffer> = streams
        .iter()
        .filter(|(_, outcome)| the_words_are_offered(*outcome))
        .map(|(stream, outcome)| AButtonToOffer {
            label: stream.the_buttons_label(),
            action: SEND_THE_WORDS,
            payload: json!({
                "labels": labels,
                "attempt": given_up.attempt,
                "stream": stream,
                "incomplete": *outcome == ACollectionOutcome::CollectionIncomplete,
            }),
        })
        .collect();
    // **`Telegram` Y4a as changed 2026-11-12, Creator-stated: the app's History
    // offers both ways on a job given up, and so does the phone.**
    buttons.extend(the_two_ways(&labels, given_up.frames_asked, given_up.resume));
    AComposedMessage { text: a_user_message(&given_up.server, &given_up.label, "", &lines.join("\n")), buttons }
}

// ------------------------------------------------------------ G1 to G4

/// **A group member's recorded ending** (`Queue & History` N1). A member with
/// none has not ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AMembersEnding {
    Finished {
        frames: u64,
        /// **E1a: C27's files written and files owed**, where the ending
        /// carried a statement.
        files: Option<(u64, u64)>,
    },
    Failed,
    GivenUp,
    /// G2: *"a member left stopped still counts toward the last member"*.
    LeftStopped,
}

/// **One member of a group, as this machine's own records hold it** (G1: the
/// group identity on each member's job record; `Queue & History` L18).
#[derive(Debug, Clone, PartialEq)]
pub struct AMember {
    pub job: String,
    pub ending: Option<AMembersEnding>,
    /// The note's times, on this machine's clock.
    pub started: Option<SystemTime>,
    pub ended: Option<SystemTime>,
}

/// **`Telegram` G1 and G2 — the group's one finishing message, or nothing.**
/// Composed on each member's journalled ending from every member record this
/// machine holds with that group identity: **nothing until the last member
/// has a recorded ending, and nothing where no member finished** — a failed
/// member's own failing message speaks for it. `Desktop app and delivery`
/// T11 and T27 mirrored, so the two speak alike.
///
/// **G3: no spoken mark is kept.** The send happens, then the cursor advances;
/// a crash between the two sends it twice, never not at all. **G4: this
/// machine speaks for its own members only**, and a group that spans machines
/// is `Rendering`'s submission shape to keep whole.
pub fn what_the_group_says(server: &(impl TheServersName + ?Sized), the_groups_label: &str, members: &[AMember]) -> Option<AMarkedText> {
    if members.is_empty() || members.iter().any(|m| m.ending.is_none()) {
        return None;
    }
    let finished: Vec<u64> = members
        .iter()
        .filter_map(|m| match m.ending {
            Some(AMembersEnding::Finished { frames, .. }) => Some(frames),
            _ => None,
        })
        .collect();
    // **E1a: where any finished member came up short**, the two counts over
    // every finished member that has them.
    let the_files = members.iter().filter_map(|m| match m.ending {
        Some(AMembersEnding::Finished { files, .. }) => files,
        _ => None,
    });
    let any_came_up_short = the_files.clone().any(|(written, owed)| written < owed);
    let the_files_in_words = any_came_up_short
        .then(|| the_files.fold((0, 0), |(written, owed), one| (written + one.0, owed + one.1)))
        .and_then(|sum| a_short_count_in_words(Some(sum)))
        .map(|words| format!(", {words}"))
        .unwrap_or_default();
    if finished.is_empty() {
        return None;
    }
    let scenes = match (finished.len(), members.len()) {
        (f, n) if f == n => format!("{n} {}", if n == 1 { "scene" } else { "scenes" }),
        (f, n) => format!("{f} of {n} scenes"),
    };
    let first = members.iter().filter_map(|m| m.started).min();
    let last = members.iter().filter_map(|m| m.ended).max();
    let took = match (first, last) {
        (Some(first), Some(last)) => last.duration_since(first).ok().map(|d| format!(" in {}", how_long_in_words(d))),
        _ => None,
    };
    Some(a_user_message(server, the_groups_label, "", &format!(
        "Finished. {scenes}, {}{the_files_in_words}{}.",
        frames_in_words(finished.iter().sum()),
        took.unwrap_or_default()
    )))
}

// ------------------------------------------------------------ V3

/// **`Telegram` V3 — the idle message, spoken once per idle stretch.** The
/// quiet moment and the half hour are `Queue & History` W1 to W3's, computed
/// on the machine's clock from its own records and handed in; the switch is
/// V2's, delivered by the told-record. **No spoken mark is written** — G3's
/// standing: a restart may say it again, never not at all. The stretch is
/// named by its quiet moment, so a new stretch speaks again.
#[derive(Debug, Default)]
pub struct TheIdleMessage {
    spoken_for: Option<String>,
}

/// **The drawing's words** (`tg`, *"Idle for 30 minutes."* — step 13's words
/// sweep; until then *Nothing rendering and nothing waiting for the last 30
/// minutes.*). The constant keeps its old name.
pub const NOTHING_TO_DO: &str = "Idle for 30\u{a0}minutes.";

impl TheIdleMessage {
    /// What to send now, or nothing. `past_the_half_hour` is `Queue & History`
    /// W3's reading; `the_quiet_moment` names the stretch (`Queue & History` W2).
    pub fn what_to_say(
        &mut self,
        server: &(impl TheServersName + ?Sized),
        the_switch_is_on: bool,
        idle: bool,
        the_quiet_moment: Option<&str>,
        past_the_half_hour: bool,
    ) -> Option<AMarkedText> {
        let stretch = the_quiet_moment.filter(|_| idle)?;
        if !the_switch_is_on || !past_the_half_hour || self.spoken_for.as_deref() == Some(stretch) {
            return None;
        }
        self.spoken_for = Some(stretch.to_string());
        Some(a_user_message(server, "", "", NOTHING_TO_DO))
    }
}

// ------------------------------------------------------------ A2 to A4

/// **What a hold waits for**, as the worker reads it off the hold record's
/// reason (`Rented-server runtime` U2) and hands it in — this crate reads no
/// hold record and names no reason (`tests/the_crates_shape_holds.rs`).
/// Added 2026-10-12, held-job piece B.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WhatTheHoldWaitsFor {
    /// **A person's answer** — the first-frame check's second job, and a job
    /// held at the start of its render.
    TheUsersAnswer,
    /// **The desktop's release after the driver update** — a hold whose reason
    /// is *the driver* (`Rendering` S18a; `Server Setup` R1c). Nothing waits
    /// on the user.
    TheDriverUpdate,
}

impl WhatTheHoldWaitsFor {
    /// **`Telegram` A3 — whether the phone is told of this hold at all**: its
    /// *waiting on you* message, its two buttons (A2), and the message when
    /// its release is recorded. **A hold for the driver sends none of it**:
    /// such a job is told as any job waiting its turn is, which is not at all,
    /// and its ending as any job's. **The one home of this rule.**
    pub fn the_phone_is_told(self) -> bool {
        match self {
            WhatTheHoldWaitsFor::TheUsersAnswer => true,
            WhatTheHoldWaitsFor::TheDriverUpdate => false,
        }
    }
}

/// **One hold awaiting the user** (`Rented-server runtime` §8, U26): the
/// first-frame check's second job.
#[derive(Debug, Clone, PartialEq)]
pub struct AWaitingJob {
    pub server: AServerLabel,
    pub label: String,
    pub hold: String,
    pub generation: String,
    /// **What the hold waits for** — where it is the driver update, nothing is
    /// sent ([`WhatTheHoldWaitsFor::the_phone_is_told`]).
    pub waits_for: WhatTheHoldWaitsFor,
    /// The frame that came back first, **by its own number, below zero too**
    /// (Sol's S2, the four questions, batch 4: a frame below zero was said as
    /// frame 1).
    pub the_frame_shown: i64,
    /// **The held job's own frame count** — the whole range, its first frame
    /// included (`Rendering` S13 as amended 2026-10-02, decision C), so *Render
    /// all — 240 frames* for a range of 240.
    pub frames_held: u64,
    /// **The held range in words**, *1–240* — `Telegram` A3a: the question
    /// names the frame and the held range. Empty where the frames were not
    /// read, and the count stands in for it.
    pub the_frames: String,
}

/// **A job's frames as the question names them**: runs of consecutive frames
/// as *first–last*, single frames alone, joined by commas — *1–240*,
/// *1,3,5–9*.
pub fn the_frames_in_words(frames: impl IntoIterator<Item = i64>) -> String {
    let mut runs: Vec<(i64, i64)> = Vec::new();
    for frame in frames {
        match runs.last_mut() {
            Some((_, last)) if frame == *last + 1 => *last = frame,
            _ => runs.push((frame, frame)),
        }
    }
    runs.iter()
        .map(|(first, last)| if first == last { first.to_string() } else { format!("{first}\u{2013}{last}") })
        .collect::<Vec<_>>()
        .join(",")
}

/// **The two answers**, as the release command carries them (`Rented-server runtime` U9).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TheAnswer {
    GoOn,
    Dont,
}

impl TheAnswer {
    fn in_words(self, frames_held: u64) -> String {
        match self {
            TheAnswer::GoOn => format!("Render all \u{2014} {} {}", with_thousands(frames_held), frames_word(frames_held)),
            TheAnswer::Dont => "Cancel job".to_string(),
        }
    }
}

/// **`Telegram` A2 and A3 — the waiting job's message, with its two
/// buttons.** The drawing's `tg-check`: *"Kitchen — final · frame 1"*,
/// *"Needs your answer. 240 frames on hold."* (step 13's final words sweep;
/// until then *"Waiting on you. All 240 frames are held until you answer."*),
/// *Render all — 240 frames* and *Cancel* (*Cancel it* until the second words
/// sweep, 2026-11-01) — corrected 2026-10-02 (decision C) from *The other
/// 239…* and *Render the rest — 239 frames*, when the held job became the
/// whole range. The question reads *Needs your answer. Render frames 1–240?*
/// (until 2026-11-01 *Needs your answer. Review frame 1 and choose whether to
/// render frames 1–240.*).
///
/// **Nothing for a hold that waits for the driver update** (A3): no message
/// and no buttons.
pub fn the_waiting_message(waiting: &AWaitingJob) -> Option<AComposedMessage> {
    if !waiting.waits_for.the_phone_is_told() {
        return None;
    }
    let held = waiting.frames_held;
    // **`Telegram` A3a: the question names the frame and the held range** —
    // the drawing's `tg-check`. Until 2026-10-28 it named a count only
    // (*240 frames on hold.*).
    let the_range = if waiting.the_frames.is_empty() {
        frames_in_words(held)
    } else {
        format!("frames {}", waiting.the_frames)
    };
    // **The header names the held range too**, as the drawing's `tg-check`
    // does; where the frames were not read it names the frame shown.
    let the_header = if waiting.the_frames.is_empty() {
        format!("frame {}", waiting.the_frame_shown)
    } else {
        format!("{} {}", if held == 1 { "frame" } else { "frames" }, waiting.the_frames)
    };
    let text = a_user_message(&waiting.server, &waiting.label, &format!(" · {the_header}"), &format!("Needs your answer. Render {the_range}?"));
    let buttons = the_questions_two_answers(&waiting.server, &waiting.label, held, &waiting.the_frames);
    Some(AComposedMessage { text, buttons })
}

/// **`Telegram` A2: the question's two answers** — *Render all — 240 frames*,
/// the *yes*, sent at once; and *Cancel job*, which asks in place first (the
/// Creator's twenty-eighth notes, note 16, and their answer that it asks).
/// Each carries what the question in place needs to say and to put back.
pub fn the_questions_two_answers(
    server: &(impl TheServersName + ?Sized),
    label: &str,
    frames_held: u64,
    the_frames: &str,
) -> Vec<AButtonToOffer> {
    [TheAnswer::GoOn, TheAnswer::Dont]
        .into_iter()
        .map(|answer| AButtonToOffer {
            label: answer.in_words(frames_held),
            action: match answer {
                TheAnswer::GoOn => ANSWER_A_WAITING_JOB,
                TheAnswer::Dont => ASK_TO_CANCEL_THE_JOB,
            },
            payload: json!({
                "labels": { "server": server, "label": label },
                "answer": answer,
                "frames_held": frames_held,
                "the_frames": the_frames,
            }),
        })
        .collect()
}

/// **`Telegram` A2: *Cancel job* asked in place, on the question itself** —
/// its words gain *Cancel this job? Frames 1–240 will not render. Their
/// settings are deleted too. The first-frame check stays in History.*, and its
/// buttons become *Keep the job* and *Cancel the job*; only the second sends
/// the *no*. The Creator's twenty-eighth notes, note 16.
pub fn the_question_asks_to_cancel(
    question: &AMarkedText,
    server: &(impl TheServersName + ?Sized),
    label: &str,
    frames_held: u64,
    the_frames: &str,
) -> AComposedMessage {
    let what_goes = if the_frames.is_empty() {
        format!("{} will not render.", frames_in_words(frames_held))
    } else {
        format!("Frames {the_frames} will not render.")
    };
    let mut text = question.clone();
    text.push_str(&format!(
        "\n\nCancel this job? {what_goes} Their settings are deleted too. The first-frame check stays in History."
    ));
    let carried = json!({
        "labels": { "server": server, "label": label },
        "frames_held": frames_held,
        "the_frames": the_frames,
    });
    let mut keep = carried.clone();
    keep["labels"]["button"] = json!("Keep the job");
    let mut go = carried;
    go["answer"] = json!(TheAnswer::Dont);
    go["labels"]["button"] = json!("Cancel the job");
    AComposedMessage {
        text,
        buttons: vec![
            AButtonToOffer { label: "Keep the job".into(), action: KEEP_THE_JOB, payload: keep },
            AButtonToOffer { label: "Cancel the job".into(), action: ANSWER_A_WAITING_JOB, payload: go },
        ],
    }
}

/// **`Telegram` A3 and `Rented-server runtime` U14 — the answer that lost is
/// told which answer was applied, and when**, as a recorded outcome and never
/// as a failure. The moment is the machine's, written as C2a writes one.
pub fn the_answer_already_applied(server: &(impl TheServersName + ?Sized), label: &str, applied: TheAnswer, frames_held: u64, at: SystemTime) -> AMarkedText {
    a_user_message(server, label, "", &format!("Already answered \u{201c}{}\u{201d} at {}.", applied.in_words(frames_held), the_moment_in_utc(at)))
}

/// `Telegram` A3/R7a: the recorded answer under the job's head.
pub fn the_answer_was_applied(server: &(impl TheServersName + ?Sized), label: &str, applied: TheAnswer, frames_held: u64) -> AMarkedText {
    a_user_message(server, label, "", &format!("Answered \u{201c}{}\u{201d}.", applied.in_words(frames_held)))
}

/// **What the dependency's recorded ending was** (`Rented-server runtime`
/// U17: *"where the first-frame job fails or is stopped"*).
///
/// **Two more since 2026-10-16, the Telegram worker's piece 3**, because the
/// machine drops a hold on four endings of its dependency and not two
/// (`Queue & History` H9 and `smoozy_queue_history_machine::waiting`): a
/// first-frame job given up, and one cancelled before it ever ran. Told as
/// *failed* or *was stopped*, either would have been untrue.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheDependencyEnded {
    Failed,
    Stopped,
    GivenUp,
    Cancelled,
}

/// **`Telegram` A4 — the drop, told**: the machine dropped the hold with
/// nobody connected (`Rented-server runtime` U17 to U20), and the phone carries the news and the why.
pub fn the_drop_message(server: &(impl TheServersName + ?Sized), label: &str, frames_held: u64, the_frame_shown: i64, why: TheDependencyEnded) -> AMarkedText {
    let how = match why {
        TheDependencyEnded::Failed => "failed",
        TheDependencyEnded::Stopped => "was stopped",
        TheDependencyEnded::GivenUp => "was interrupted twice",
        TheDependencyEnded::Cancelled => "was cancelled",
    };
    // **The count first and the reason after, without *held*** —
    // `Architecture/Telegram` A4's build detail of 2026-11-12 (the four
    // questions, scout 3.46).
    a_user_message(server, label, "", &format!(
        "{} {} not rendered: frame {the_frame_shown} {how}.",
        with_thousands(frames_held),
        frames_word(frames_held)
    ))
}

/// **`Queue & History` L22 — a waiting job dropped at its turn because the
/// Blender it was sent with was removed from the machine**, told off the
/// journal event alone, since every record of that job went in the same act
/// (*"the phone reads the same event"*). The label is the event's bounded
/// copy and the version the one L21's mark held. Standard words that Design
/// may replace; added 2026-10-16, the Telegram worker's piece 3.
pub fn the_drop_for_a_removed_blender(server: &(impl TheServersName + ?Sized), label: &str, version: &str) -> AMarkedText {
    a_user_message(server, label, "", &format!("Not rendered: Blender {version} was removed from this server."))
}

/// **`State, identity and recovery` E5 — the journal dropped events before
/// this server's bot read them, said rather than skipped**, so what follows
/// is never presented as the whole account. Standard words that Design may
/// replace; added 2026-10-16, the Telegram worker's piece 3.
pub fn some_events_were_missed(server: &(impl TheServersName + ?Sized)) -> AMarkedText {
    a_user_message(server, "", "", "Some messages about this server's jobs were not sent.")
}

// ------------------------------------------------------------ D2 and D4

/// **`Telegram` D2's first acknowledgement, from the listener**: the stop was
/// sent to the named machine for the named job.
pub fn the_stop_was_sent(server: &(impl TheServersName + ?Sized), label: &str) -> AMarkedText {
    a_user_message(server, label, "", "Stop sent.")
}

/// **The stop's recorded outcome** (`Server protocol and transfers` C8), as
/// the supervisor's command path answers it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AStopsOutcome {
    /// Stopped now, with the counts.
    Stopped { frames_on_the_server: u64, frames_not_rendered: u64 },
    /// D4 and R4: a second press finds the recorded outcome.
    AlreadyStopped { at: SystemTime },
    /// The job had ended before the stop landed.
    TheJobHadAlreadyEnded,
}

/// **`Telegram` D2's second acknowledgement, from the owning machine**: the
/// recorded outcome, with the counts and what remains; and D4's second press.
pub fn the_stops_outcome(server: &(impl TheServersName + ?Sized), label: &str, outcome: &AStopsOutcome) -> AMarkedText {
    let body = match outcome {
        AStopsOutcome::Stopped { frames_on_the_server, frames_not_rendered } => format!(
            "Stopped. {} {} on the server; {} {} not rendered.",
            with_thousands(*frames_on_the_server),
            if *frames_on_the_server == 1 { "frame" } else { "frames" },
            with_thousands(*frames_not_rendered),
            if *frames_not_rendered == 1 { "frame" } else { "frames" },
        ),
        AStopsOutcome::AlreadyStopped { at } => {
            format!("Already stopped at {}.", the_moment_in_utc(*at))
        }
        AStopsOutcome::TheJobHadAlreadyEnded => format!("This job has already ended."),
    };
    a_user_message(server, label, "", &body)
}

// ------------------------------------------------------------ Y1 and R4

/// **What a *try it again* submission answered** on the supervisor's command
/// path (`Server protocol and transfers` C4, C8).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ASubmissionsOutcome {
    Submitted,
    /// *Resume*'s job stands, starting at `from` (`Telegram` Y4a as changed
    /// 2026-11-11).
    Resumed { from: i64 },
    /// R4: a second press lands on the machine's own idempotency.
    AlreadySubmitted { at: SystemTime },
}

/// **The owning machine's truthful answer to *try it again*.** Not drawn;
/// standard words, recorded at the decisions record.
pub fn the_submissions_outcome(server: &(impl TheServersName + ?Sized), label: &str, outcome: &ASubmissionsOutcome) -> AMarkedText {
    let body = match outcome {
        ASubmissionsOutcome::Submitted => format!("Submitted again."),
        // The desk's own line under *Resume* (`Design/Queue & History`).
        ASubmissionsOutcome::Resumed { from } => {
            format!("Resumed from frame {from}. The frames will be in two folders.")
        }
        ASubmissionsOutcome::AlreadySubmitted { at } => {
            format!("Already submitted again at {}.", the_moment_in_utc(*at))
        }
    };
    a_user_message(server, label, "", &body)
}

// ------------------------------------------------------------ B9 and R14

/// **`Telegram` B9 — a failure recorded beside an instruction's identity, told
/// in plain words**: what was not done, naming the server and the job. Standard
/// words that Design may replace.
pub fn the_act_was_not_done(server: &(impl TheServersName + ?Sized), label: &str, asked: &str) -> AMarkedText {
    // **The word sweep of 2026-11-13**: a button's own name is quoted; a
    // stand-in where the record carried none (*the answer*, *the action*) is
    // not, so it never reads as the name of a button the user tapped.
    let what = match asked.chars().next() {
        Some(first) if first.is_lowercase() => format!("{}{}", first.to_uppercase(), &asked[first.len_utf8()..]),
        _ => format!("\u{201c}{asked}\u{201d}"),
    };
    a_user_message(server, label, "", &format!("{what} was refused by this server."))
}

/// **`Telegram` R14, until step 8b composes the actor that answers a routed
/// frame request**: the truth, that the frame cannot be sent yet. Standard
/// words.
pub fn a_frame_cannot_be_sent_yet(server: &(impl TheServersName + ?Sized), label: &str, frame: i64) -> AMarkedText {
    a_user_message(server, label, &format!(" · Frame {frame}"), "Not available on this server yet.")
}

// ------------------------------------------------------------ E3's tap

/// **E3 and M8 — the published cap on a file a bot sends, 50 MB, and the one
/// place it is kept**: the words file and the moving picture's trim both read
/// it (`crate::media`). Counted as 50,000,000 bytes, the smaller reading, so
/// nothing near the cap is sent to be refused. **Whether Telegram means the
/// smaller is not established** (`Telegram` §21 item 32), and this stays until
/// that is answered.
pub const THE_CAP_ON_A_FILE: u64 = 50_000_000;

/// **`Telegram` M10 — whether a finished video goes to the chat**: at most
/// [`THE_CAP_ON_A_FILE`], the one cap the build keeps. A file over it is not
/// sent and not trimmed, and the job's messages say how big it is. Added
/// 2026-10-27, the nineteenth notes, note 4.
pub fn a_finished_video_fits(bytes: u64) -> bool {
    bytes <= THE_CAP_ON_A_FILE
}

/// **`Telegram` M10 — a file's size in words**, as the desktop's Transfers
/// writes one: in thousands, *B*, *KB*, *MB*, *GB*, *TB*, one decimal from
/// *KB* up — *38.2 MB*, *1.4 GB*. Thousands rather than 1,024s, so the size
/// reads against Telegram's *50 MB* the way [`THE_CAP_ON_A_FILE`] counts it.
pub fn a_size_in_words(bytes: u64) -> String {
    const STEPS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];
    let mut left = bytes as f64;
    let mut at = 0;
    while left >= 1000.0 && at < STEPS.len() - 1 {
        left /= 1000.0;
        at += 1;
    }
    if at == 0 {
        format!("{bytes} B")
    } else {
        format!("{left:.1} {}", STEPS[at])
    }
}

/// **E3d — how much of the stream the block shows.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HowMuchIsShown {
    Whole,
    TheLastLines(usize),
    /// The last line alone is longer than the room, so the block holds the
    /// end of that one line (ruling at the decisions record, E3d).
    TheEndOfTheLastLine,
}

/// **E3d/R7a — the plain log text under marked names, and its file, if any.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheWordsInTheChat {
    /// The server's line, the job label, then the plain log text.
    pub text: AMarkedText,
    /// The file's name, where the whole stream follows as a file.
    pub file: Option<String>,
    /// **E3e — what the button under the message copies**: the block's last
    /// line with anything but white space, inside Telegram's 256. Nothing
    /// where the block holds white space alone.
    pub copies: Option<String>,
}

/// **E3d — the label**, the drawing's three lines word for word for the
/// output stream (`Preparation/Design/Mockup/Smoozy.html`, Design notes under
/// *The phone*), and the same shape for the errors stream and for an
/// incomplete record (ruling at the decisions record, E3d).
fn the_words_label(_label: &str, stream: AStream, shown: HowMuchIsShown, over_the_cap: bool, incomplete: bool) -> String {
    let name = format!("{} log", stream.the_name());
    let how = match shown {
        HowMuchIsShown::Whole if incomplete => return format!("{name}, incomplete."),
        HowMuchIsShown::Whole => return format!("{name}, in full."),
        HowMuchIsShown::TheLastLines(1) => "last line".to_string(),
        HowMuchIsShown::TheLastLines(n) => format!("last {} lines", with_thousands(n as u64)),
        HowMuchIsShown::TheEndOfTheLastLine => "the end of the last line".to_string(),
    };
    let the_whole = match (stream, over_the_cap) {
        (AStream::TheOrdinaryOutput, false) => "Full log in the file below.",
        (AStream::TheComplaints, false) => "Full log in the file below.",
        (AStream::TheOrdinaryOutput, true) => {
            "Full log over 50 MB, too large to send. Open it in Smoozy on your computer."
        }
        (AStream::TheComplaints, true) => {
            "Full log over 50 MB, too large to send. Open it in Smoozy on your computer."
        }
    };
    let partly = if incomplete { " Log incomplete." } else { "" };
    format!("{name}, {how}. {the_whole}{partly}")
}

/// **E3d — the file's name**, the drawing's *Kitchen - final - Output.txt*:
/// the job's label with its dashes and any character a file name cannot hold
/// written as a hyphen, then the stream's name.
pub fn the_words_file_name(label: &str, stream: AStream) -> String {
    let safe: String = label
        .chars()
        .map(|c| match c {
            '\u{2014}' | '\u{2013}' | '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '-',
            c if c.is_control() => '-',
            c => c,
        })
        .collect();
    format!("{safe} - {}.txt", stream.the_name())
}

/// **`Telegram` E3d — the words read in the chat.** One ordinary message:
/// the server's line (R7a), the label, then the stream in plain text.
///
/// - **Whole where it fits** Telegram's 4,096 with the label, and no file.
/// - **Otherwise the end**, cut at a line boundary so it fits, the label
///   saying so, and the whole stream as a file in a second message.
/// - **Over the 50 MB cap**, the end, and the label saying the whole record is
///   too large to send and is read from the desk; no file.
///
/// **The 4,096 is counted in UTF-16 code units**, the one unit Telegram's
/// pages name for a message's text, and the larger of the two readings of
/// *characters*, so a text that fits here fits either way (`Audits/Build/Step
/// 10/Long machine output in the chat — what Telegram's pages say.md` §1 and
/// §3). **The cut is decided by length alone** (E3d; `Rented-server runtime`
/// M24): as many whole lines from the end as fit. **A last line longer than
/// all the room** is shown as its own end, cut between whole characters.
/// **The file is never cut** (`Rented-server runtime` M8); the words are read as UTF-8 for the block,
/// with any bytes that are not replaced by the replacement mark, and the file
/// carries the bytes as they are.
pub fn the_words_in_the_chat(server: &(impl TheServersName + ?Sized), label: &str, stream: AStream, words: &[u8], incomplete: bool) -> TheWordsInTheChat {
    let over_the_cap = words.len() as u64 > THE_CAP_ON_A_FILE;
    let text = String::from_utf8_lossy(words);
    let head = |shown| { let mut head = a_user_message(server, label, "", &the_words_label(label, stream, shown, over_the_cap, incomplete)); head.append("\n", None); head };
    // **E3e (2026-11-12, Creator-stated): the stream's text is one folded
    // quotation**, the label outside it, and the button under the message
    // copies the block's last line.
    let composed = |mut head: AMarkedText, shown: &str, file: Option<String>| {
        head.append(shown, Some(THE_FOLDED_QUOTATION));
        TheWordsInTheChat { text: head, file, copies: the_line_the_button_copies(shown) }
    };

    if !over_the_cap {
        let whole = head(HowMuchIsShown::Whole);
        if how_long_it_counts(&whole) + how_long_it_counts(&text) <= THE_BOUND_ON_A_MESSAGE {
            return composed(whole, &text, None);
        }
    }
    let file = (!over_the_cap).then(|| the_words_file_name(label, stream));

    let lines: Vec<&str> = text.split_inclusive('\n').collect();
    let (mut taken, mut length) = (0, 0);
    for line in lines.iter().rev() {
        let longer = length + how_long_it_counts(line);
        if how_long_it_counts(&head(HowMuchIsShown::TheLastLines(taken + 1))) + longer > THE_BOUND_ON_A_MESSAGE {
            break;
        }
        taken += 1;
        length = longer;
    }
    if taken > 0 {
        return composed(head(HowMuchIsShown::TheLastLines(taken)), &lines[lines.len() - taken..].concat(), file);
    }

    let last = lines.last().copied().unwrap_or("");
    let the_head = head(HowMuchIsShown::TheEndOfTheLastLine);
    let room = THE_BOUND_ON_A_MESSAGE.saturating_sub(how_long_it_counts(&the_head));
    let (mut starts, mut used) = (last.len(), 0);
    for (at, c) in last.char_indices().rev() {
        if used + c.len_utf16() > room {
            break;
        }
        used += c.len_utf16();
        starts = at;
    }
    composed(the_head, &last[starts..], file)
}

/// **E3e — the entity the log's text is sent as.** Telegram's page: *"“expandable_blockquote”
/// (collapsed-by-default block quotation)"* (Sources entry 95).
pub const THE_FOLDED_QUOTATION: &str = "expandable_blockquote";

/// **E3e — the copy button's words** (Rule 9).
pub const COPY_LAST_LINE: &str = "Copy last line";

/// **E3e — Telegram's bound on what a copy button holds**: *"The text to be
/// copied to the clipboard; 1-256 characters"*. The page names no unit, so
/// these are UTF-16 code units, the larger reading, as E3d counts its bound.
pub const THE_BOUND_ON_A_COPIED_TEXT: usize = 256;

/// **E3e — what the button copies: the block's last line with anything but
/// white space**, without the white space around it, and its first 256 units
/// where it is longer, cut between whole characters. **Chosen by where the
/// line is, never by what it says** (E3d: nothing is mined for a cause).
pub fn the_line_the_button_copies(block: &str) -> Option<String> {
    let line = block.lines().rev().map(str::trim).find(|line| !line.is_empty())?;
    let (mut ends, mut used) = (0, 0);
    for (at, c) in line.char_indices() {
        if used + c.len_utf16() > THE_BOUND_ON_A_COPIED_TEXT {
            break;
        }
        used += c.len_utf16();
        ends = at + c.len_utf8();
    }
    Some(line[..ends].to_string())
}

/// **E3e — the keyboard under a log**: one inline button with `copy_text`
/// (Telegram's `CopyTextButton`). Pressed on the phone; nothing comes back
/// to a machine, so it has no token and no record.
pub fn the_copy_button_under(told: &TheWordsInTheChat) -> Option<Value> {
    let line = told.copies.as_deref()?;
    Some(json!({ "inline_keyboard": [[{ "text": COPY_LAST_LINE, "copy_text": { "text": line } }]] }))
}

/// **E3 — the words could not be read back whole** (`Rented-server runtime`
/// M11: whole or not at all). Nothing is sent.
pub fn the_words_could_not_be_read(server: &(impl TheServersName + ?Sized), label: &str, stream: AStream) -> AMarkedText {
    a_user_message(server, label, "", &format!("Could not read the {} log. Nothing was sent.", stream.the_name()))
}

// ------------------------------------------------------------ words

fn frames_word(n: u64) -> &'static str {
    if n == 1 {
        "frame"
    } else {
        "frames"
    }
}

fn frames_in_words(n: u64) -> String {
    format!("{} {}", with_thousands(n), frames_word(n))
}

/// A count as the drawing writes one: *1,440*.
pub fn with_thousands(n: u64) -> String {
    let digits = n.to_string();
    let mut out = String::new();
    for (at, digit) in digits.chars().enumerate() {
        if at > 0 && (digits.len() - at).is_multiple_of(3) {
            out.push(',');
        }
        out.push(digit);
    }
    out
}

/// **A length of time as the drawing writes one**: *6 hours 2 minutes*,
/// *14 hours 6 minutes*. Under a minute, in seconds.
pub fn how_long_in_words(d: Duration) -> String {
    let seconds = d.as_secs();
    let (hours, minutes) = (seconds / 3600, (seconds % 3600) / 60);
    let unit = |n: u64, one: &str, many: &str| format!("{n} {}", if n == 1 { one } else { many });
    match (hours, minutes) {
        (0, 0) => unit(seconds, "second", "seconds"),
        (0, m) => unit(m, "minute", "minutes"),
        (h, 0) => unit(h, "hour", "hours"),
        (h, m) => format!("{} {}", unit(h, "hour", "hours"), unit(m, "minute", "minutes")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn names_in(text: &AMarkedText) -> Vec<(String, String)> {
        let units: Vec<u16> = text.text.encode_utf16().collect();
        text.entities.iter().map(|e| {
            let at = e["offset"].as_u64().unwrap() as usize;
            let len = e["length"].as_u64().unwrap() as usize;
            (e["type"].as_str().unwrap().into(), String::from_utf16(&units[at..at + len]).unwrap())
        }).collect()
    }

    /// **`Telegram` V1e — which sends make a sound, by the job's choice**, in
    /// the one home beside [`the_job_reports`]: the start under `Everything`
    /// alone, pictures where they are sent, endings and questions always, the
    /// contact sheet, the video and a message sent again never. The
    /// twenty-eighth notes, note 18.
    #[test]
    fn twenty_eighth_18_the_sound_follows_the_choice() {
        use AJobsSend::*;
        use TheReportingChoice::*;
        let paced = APictureEveryFewMinutes { minutes: 5 };
        for (send, everything, pictures, a_picture_every, alerts) in [
            (TheStart, true, false, false, false),
            (AFramesPicture, true, true, true, true),
            (AnEndingOrAQuestion, true, true, true, true),
            (AfterTheEnding, false, false, false, false),
            (TheJobsMessageSentAgain, false, false, false, false),
        ] {
            assert_eq!(it_makes_a_sound(Everything, send), everything, "{send:?}");
            assert_eq!(it_makes_a_sound(ThePicturesWithoutTheRunningCommentary, send), pictures, "{send:?}");
            assert_eq!(it_makes_a_sound(paced, send), a_picture_every, "{send:?}");
            assert_eq!(it_makes_a_sound(OnlyWhatNeedsAnAnswer, send), alerts, "{send:?}");
        }
        assert!(the_job_reports(OnlyWhatNeedsAnAnswer, AKindOfReport::TheJobsMessage));
        assert!(the_job_reports(ThePicturesWithoutTheRunningCommentary, AKindOfReport::TheJobsMessage));
        assert!(!the_job_reports(Nothing, AKindOfReport::TheJobsMessage));
    }

    /// **`Telegram` C1d and C1e — the job's message and its one-line cut.**
    /// The head, then C1's own line for the job; *Started.* until the first
    /// progress; a refusal's line last. The cut: the mark, the server in bold,
    /// the job in italic and the state word, on one line.
    #[test]
    fn twenty_eighth_18_the_jobs_message_and_its_cut() {
        let server = AServerLabel::with_colour("RunPod 4090", Some("red"));
        let started = the_jobs_message(&server, "Kitchen \u{2014} final", None, &[]);
        assert_eq!(started.text, "🟥 RunPod 4090\nKitchen \u{2014} final\nStarted.");
        let rendering = the_jobs_message(
            &server,
            "Kitchen \u{2014} final",
            Some("Rendering: frame 31 of 240 \u{b7} 13%"),
            &["Preview not sent: over Telegram's limits."],
        );
        assert_eq!(
            rendering.text,
            "🟥 RunPod 4090\nKitchen \u{2014} final\nRendering: frame 31 of 240 \u{b7} 13%\nPreview not sent: over Telegram's limits."
        );
        let cut = the_jobs_message_cut(&server, "Kitchen \u{2014} final", A_FAILED_ENDING);
        assert_eq!(cut.text, "🟥 RunPod 4090 \u{b7} Kitchen \u{2014} final \u{b7} 🔴 Failed.");
        assert_eq!(names_in(&cut), [("bold".into(), "RunPod 4090".into()), ("italic".into(), "Kitchen \u{2014} final".into())]);
        assert!(!the_jobs_message_cut("Vast 5060", "Spins", A_FINISHED_ENDING).text.contains('\n'));
    }

    /// **`Telegram` D0 asked in place, and the waiting range's question with
    /// the pair's sentence** — the words the job's message gains. The
    /// twenty-eighth notes, notes 16 and 18.
    #[test]
    fn twenty_eighth_18_the_stop_questions_words_in_place() {
        assert_eq!(the_stop_questions_words(&AStopQuestion::ARender), "Stop this job?\nThe current frame may not be saved.");
        assert_eq!(
            the_stop_questions_words(&AStopQuestion::ACheck { first_frame: "1".into(), the_frames: "1\u{2013}240".into() }),
            "Stop this job?\nFrame 1 may not be saved. Frames 1\u{2013}240 will not render."
        );
        assert_eq!(the_stop_questions_words(&AStopQuestion::AWaitingJob), "Cancel this job?");
        let range = AStopQuestion::AWaitingRange { the_frames: "1\u{2013}240".into() };
        assert_eq!(
            the_stop_questions_words(&range),
            "Cancel this job?\nFrames 1\u{2013}240 have not started. Their settings are deleted too. The first-frame check stays in History."
        );
        let asked = the_stop_question_for("RunPod 4090", "Kitchen", &range);
        let labels: Vec<_> = asked.buttons.iter().map(|b| b.label.as_str()).collect();
        assert_eq!(labels, ["Keep the job", "Cancel the job"]);
        assert!(asked.text.text.ends_with("The first-frame check stays in History."));
    }

    #[test]
    fn twenty_first_names_are_explicit_utf16_runs_even_with_squares_and_markup_in_the_name() {
        let server = AServerLabel::with_colour("🟥 サーバー <&> 🎬", Some("blue"));
        let text = a_user_message(&server, "Кухня 🎬 *[1]*", " · Frame 147", "Finished. 240 frames.");
        assert_eq!(text.text, "🟦 🟥 サーバー <&> 🎬\nКухня 🎬 *[1]* · Frame 147\nFinished. 240 frames.");
        assert_eq!(text.entities[0]["offset"], 3);
        assert_eq!(text.entities[1]["offset"], 3 + server.encode_utf16().count() + 1);
        assert_eq!(names_in(&text), [("bold".into(), server.to_string()), ("italic".into(), "Кухня 🎬 *[1]*".into())]);
        let old = a_user_message("🟥 サーバー", "Кухня", "", "Started.");
        assert!(old.starts_with("🟥 サーバー\n"));
        assert_eq!(old.entities[0]["offset"], 0, "a square in a user's name is not a told colour");
        assert_eq!(names_in(&old)[0], ("bold".into(), "🟥 サーバー".into()));
        for (colour, square) in [("red", "🟥"), ("gold", "🟧"), ("green", "🟩"), ("jade", "🟩"), ("cyan", "🟦"), ("blue", "🟦"), ("violet", "🟪"), ("magenta", "🟪")] {
            assert_eq!(the_colour_square(Some(colour)), Some(square));
        }
    }

    #[test]
    fn twenty_first_every_job_composer_keeps_the_same_names_and_plain_words() {
        let server = AServerLabel::with_colour("Сервер 🎬", Some("jade"));
        let job = "台所 <&>";
        let mut failed = kitchen(TheCause::TheMachinesOwn, ACollectionOutcome::Collected, ACollectionOutcome::Collected);
        failed.server = server.clone(); failed.label = job.into();
        let given = AGivenUpJob { server: server.clone(), job: "id".into(), label: job.into(), generation: "1".into(), attempt: 2, frames_on_the_server: 3, ordinary_output: ACollectionOutcome::Collected, complaints: ACollectionOutcome::Collected, frames_asked: 0, resume: None };
        let waiting = AWaitingJob { server: server.clone(), label: job.into(), hold: "h".into(), generation: "1".into(), waits_for: WhatTheHoldWaitsFor::TheUsersAnswer, the_frame_shown: 1, frames_held: 3, the_frames: "1–3".into() };
        let mut texts = vec![
            the_start_message(&server, job), the_stop_question(&server, job, false).text,
            the_stop_question(&server, job, true).text, the_job_already_ended(&server, job),
            the_failure_message(&failed).text, the_given_up_message(&given).text,
            the_finishing_message(&AFinishedJob { server: server.clone(), label: job.into(), frames: 3, started: at(0), ended: at(60), group: None, files: None }).unwrap(),
            what_the_group_says(&server, job, &[member(Some(AMembersEnding::Finished { frames: 3, files: None }), 0, 60)]).unwrap(),
            the_waiting_message(&waiting).unwrap().text,
            the_answer_was_applied(&server, job, TheAnswer::GoOn, 3),
            the_answer_already_applied(&server, job, TheAnswer::Dont, 3, at(0)),
            the_drop_message(&server, job, 3, 1, TheDependencyEnded::Failed),
            the_drop_for_a_removed_blender(&server, job, "4.5"), the_stop_was_sent(&server, job),
            the_stops_outcome(&server, job, &AStopsOutcome::Stopped { frames_on_the_server: 1, frames_not_rendered: 2 }),
            the_stops_outcome(&server, job, &AStopsOutcome::AlreadyStopped { at: at(0) }),
            the_stops_outcome(&server, job, &AStopsOutcome::TheJobHadAlreadyEnded),
            the_submissions_outcome(&server, job, &ASubmissionsOutcome::Submitted),
            the_submissions_outcome(&server, job, &ASubmissionsOutcome::AlreadySubmitted { at: at(0) }),
            the_act_was_not_done(&server, job, "Stop this job"), a_frame_cannot_be_sent_yet(&server, job, 1),
            the_words_in_the_chat(&server, job, AStream::TheOrdinaryOutput, b"one\ntwo", false).text,
            the_words_could_not_be_read(&server, job, AStream::TheComplaints),
        ];
        for text in &texts {
            assert!(text.starts_with("🟩 Сервер 🎬\n台所 <&>"), "{text}");
            // **The names, and nothing else marked** — but E3e's fold over
            // a log's own text (2026-11-12), which is no name.
            let names: Vec<(String, String)> =
                names_in(text).into_iter().filter(|(kind, _)| kind != THE_FOLDED_QUOTATION).collect();
            assert_eq!(names, [("bold".into(), server.to_string()), ("italic".into(), job.into())]);
            assert_eq!(text.matches(job).count(), 1, "{text}");
        }
        assert!(texts.remove(4).contains("\n🔴 Failed at frame 148 of 240."));
        assert!(texts.iter().all(|text| !text.contains("🔴")));
    }

    fn kitchen(cause: TheCause, ordinary: ACollectionOutcome, complaints: ACollectionOutcome) -> AFailedJob {
        AFailedJob {
            server: "RunPod 4090".into(),
            job: "job-1".into(),
            label: "Kitchen \u{2014} final".into(),
            generation: "1".into(),
            attempt: 1,
            at_frame: Some((148, 240)),
            exit_status: Some(1),
            frames_on_the_server: 147,
            ordinary_output: ordinary,
            complaints,
            cause,
            disk_free_at_the_ending: Some(61_000_000_000),
            no_render_began: false,
            the_reason_the_machine_recorded: None,
            frames_asked: 240,
            resume: None,
        }
    }

    /// **A job interrupted twice is said as that, never *abandoned***, which
    /// read as if the user gave up — the four questions, scout 3.24 as Sol
    /// narrowed it, 2026-11-11: no cause is named that the ending cannot know.
    #[test]
    fn a_job_interrupted_twice_is_said_as_that() {
        assert_eq!(AN_ABANDONED_ENDING, "Interrupted.");
        let given_up = AGivenUpJob {
            server: "RunPod 4090".into(),
            job: "job-1".into(),
            label: "Kitchen \u{2014} final".into(),
            generation: "1".into(),
            attempt: 2,
            frames_on_the_server: 0,
            ordinary_output: ACollectionOutcome::NeverEstablished,
            complaints: ACollectionOutcome::NeverEstablished,
            frames_asked: 0,
            resume: None,
        };
        assert!(the_given_up_message(&given_up).text.contains("\nInterrupted twice. No more retries.\n"));
        assert!(the_drop_message("A", "K", 2, 1, TheDependencyEnded::GivenUp)
            .text
            .ends_with("2 frames not rendered: frame 1 was interrupted twice."));
    }

    /// **`Specs/Render Settings`: nothing assumes the first frame is frame 1**
    /// — a failure at a frame below zero says *at frame* with that frame, as
    /// any other does. The four questions, batch 4's S2 leftover.
    #[test]
    fn four_questions_s2_a_failure_below_zero_says_its_frame() {
        use ACollectionOutcome::*;
        let mut failed = kitchen(TheCause::TheMachinesOwn, Collected, CollectedAndTheStreamWasEmpty);
        failed.at_frame = Some((-3, 10));
        let lines: Vec<String> = the_failure_message(&failed).text.lines().map(str::to_string).collect();
        assert_eq!(lines[2], "🔴 Failed at frame -3 of 10. Blender exited with an error.");
    }

    /// **The phone says how Blender ended in words, never a bare exit code**,
    /// and a full disk names the server and the app's own button — the four
    /// questions, scout 3.25 and 3.23, 2026-11-11. The code stays in the log.
    #[test]
    fn the_phone_says_how_blender_ended_in_words() {
        use ACollectionOutcome::*;
        let mut failed = kitchen(TheCause::TheMachinesOwn, Collected, CollectedAndTheStreamWasEmpty);
        let lines: Vec<String> = the_failure_message(&failed).text.lines().map(str::to_string).collect();
        assert_eq!(lines[2], "🔴 Failed at frame 148 of 240. Blender exited with an error.");
        failed.exit_status = None;
        let lines: Vec<String> = the_failure_message(&failed).text.lines().map(str::to_string).collect();
        assert_eq!(lines[2], "🔴 Failed at frame 148 of 240. Blender quit unexpectedly.");
        assert!(!the_failure_message(&failed).text.contains("code"));
        assert_eq!(
            THE_DISK_IS_FULL,
            "The output folder on the server was full; free space, then Submit again."
        );
    }

    /// **E1, `Design/Queue & History`: a job that failed before any render says so,
    /// with the reason**, and nothing about Blender ending, about a render or about a
    /// stream never opened. The buttons follow the cause as ever.
    #[test]
    fn a_failure_before_any_render_says_so_in_its_short_form() {
        use ACollectionOutcome::NeverEstablished;
        let mut failed = kitchen(TheCause::Undetermined, NeverEstablished, NeverEstablished);
        failed.no_render_began = true;
        failed.the_reason_the_machine_recorded = Some("could not copy the bake folder //caches/shot/".to_string());
        failed.exit_status = None;
        failed.at_frame = None;
        failed.frames_on_the_server = 0;
        failed.disk_free_at_the_ending = Some(0);
        let text = the_failure_message(&failed).text;
        assert!(text.contains("🔴 Failed before rendering.\nCould not copy the bake folder //caches/shot/\nNo frames on the server."), "{text}");
        for false_here in ["exit code", "exit status", "No output was recorded", "Errors log", "was full"] {
            assert!(!text.contains(false_here), "{false_here}: {text}");
        }
        // A job Blender was launched for keeps what its streams hold.
        failed.ordinary_output = ACollectionOutcome::CollectionIncomplete;
        assert!(the_failure_message(&failed).text.contains("Output log incomplete."));
    }

    /// **`Whole app` and E1 — the disk is full, said beside the machine's own
    /// words**, only where the reading at the ending is none, and never where it
    /// could not be read.
    #[test]
    fn a_full_disk_is_said_beside_the_exit_status() {
        use ACollectionOutcome::*;
        let mut failed = kitchen(TheCause::TheMachinesOwn, Collected, CollectedAndTheStreamWasEmpty);
        failed.disk_free_at_the_ending = Some(0);
        let lines: Vec<String> = the_failure_message(&failed).text.lines().map(str::to_string).collect();
        assert_eq!(lines[2], "🔴 Failed at frame 148 of 240. Blender exited with an error.");
        assert_eq!(lines[3], THE_DISK_IS_FULL);
        for reading in [None, Some(1)] {
            failed.disk_free_at_the_ending = reading;
            assert!(!the_failure_message(&failed).text.contains(THE_DISK_IS_FULL), "{reading:?}");
        }
    }

    /// **E1 and E3 — a job given up says so and offers the words**, ~~and never
    /// *try it again*~~ and since 2026-11-12 *Submit again* after them (Y4a as
    /// changed that day).
    #[test]
    fn a_given_up_job_says_it_was_given_up_and_offers_the_words_then_submit_again() {
        let given_up = AGivenUpJob {
            server: "RunPod 4090".into(),
            job: "job-1".into(),
            label: "Kitchen \u{2014} final".into(),
            generation: "1".into(),
            attempt: 2,
            frames_on_the_server: 147,
            ordinary_output: ACollectionOutcome::Collected,
            complaints: ACollectionOutcome::CollectedAndTheStreamWasEmpty,
            frames_asked: 0,
            resume: None,
        };
        let message = the_given_up_message(&given_up);
        assert_eq!(
            message.text,
            "RunPod 4090\nKitchen — final\nInterrupted twice. No more retries.\n147 frames on the server."
        );
        assert!(!message.text.contains("failed"));
        assert_eq!(labels(&message), ["Send Output log", "Submit again"]);
    }

    /// **A3 — the winning answer is told**, in the shape of the losing one's.
    #[test]
    fn the_winning_answer_is_told_which_was_applied() {
        assert_eq!(
            the_answer_was_applied("RunPod 4090", "Kitchen \u{2014} final", TheAnswer::GoOn, 240),
            "RunPod 4090\nKitchen — final\nAnswered “Render all — 240 frames”."
        );
        assert_eq!(the_answer_was_applied("RunPod 4090", "Kitchen", TheAnswer::Dont, 3), "RunPod 4090\nKitchen\nAnswered “Cancel job”.");
    }

    /// **The twenty-eighth notes, note 16 — the phone's words.** `/stop` on a
    /// waiting job asks the window's own question; a first-frame check's stop
    /// says the whole job ends; the question's *Cancel job* asked in place
    /// keeps the question and adds the window's cancel sentence, with *Keep
    /// the job* and *Cancel the job*, only the second the *no*. **It failed
    /// before the change**: *Cancel this waiting job?* with *Keep waiting* and
    /// *Stop this job*, and no composer for the other two.
    #[test]
    fn twenty_eighth_16_the_questions_speak_of_the_job() {
        let waiting = the_stop_question("RunPod 4090", "Kitchen", true);
        assert_eq!(waiting.text, "RunPod 4090\nKitchen\nCancel this job?");
        assert_eq!(labels(&waiting), ["Keep the job", "Cancel the job"]);
        assert_eq!(waiting.buttons[1].action, STOP_THIS_JOB);
        let rendering = the_stop_question("RunPod 4090", "Kitchen", false);
        assert_eq!(labels(&rendering), ["Keep rendering", "Stop this job"]);
        let check = the_checks_stop_question("RunPod 4090", "Kitchen", "1", "1\u{2013}240");
        assert_eq!(
            check.text,
            "RunPod 4090\nKitchen\nStop this job?\nFrame 1 may not be saved. Frames 1\u{2013}240 will not render."
        );
        assert_eq!(labels(&check), ["Keep rendering", "Stop this job"]);
        assert_eq!(check.buttons[1].action, STOP_THIS_JOB);

        let question = AMarkedText::plain("RunPod 4090\nKitchen\nNeeds your answer. Render frames 1\u{2013}240?");
        let asked = the_question_asks_to_cancel(&question, "RunPod 4090", "Kitchen", 240, "1\u{2013}240");
        assert_eq!(
            asked.text,
            "RunPod 4090\nKitchen\nNeeds your answer. Render frames 1\u{2013}240?\n\nCancel this job? Frames 1\u{2013}240 will not render. Their settings are deleted too. The first-frame check stays in History."
        );
        assert_eq!(labels(&asked), ["Keep the job", "Cancel the job"]);
        assert_eq!(asked.buttons[0].action, KEEP_THE_JOB);
        assert_eq!(asked.buttons[1].action, ANSWER_A_WAITING_JOB);
        assert_eq!(asked.buttons[1].payload["answer"], json!(TheAnswer::Dont));
        let unread = the_question_asks_to_cancel(&question, "RunPod 4090", "Kitchen", 240, "");
        assert!(unread.text.contains("Cancel this job? 240 frames will not render."));
    }

    fn labels(message: &AComposedMessage) -> Vec<String> {
        message.buttons.iter().map(|b| b.label.clone()).collect()
    }

    /// **The drawing's `tg-fail`, word for word**, on the machine's own cause:
    /// with the Output log offered, nothing is said of an empty Errors stream
    /// (E2 as narrowed 2026-10-31). *(It ended* Blender wrote nothing under
    /// Errors. *until the live Telegram pass's fault 7, which the drawing
    /// never drew.)*
    #[test]
    fn the_failure_message_is_the_drawings() {
        use ACollectionOutcome::*;
        let message = the_failure_message(&kitchen(TheCause::TheMachinesOwn, Collected, CollectedAndTheStreamWasEmpty));
        assert_eq!(
            message.text,
            "RunPod 4090\nKitchen — final\n🔴 Failed at frame 148 of 240. Blender exited with an error.\n147 frames on the server."
        );
        assert_eq!(labels(&message), ["Send Output log", "Submit again \u{2014} 240 frames"]);
    }

    /// **The live Telegram pass, fault 7 (E2 as narrowed 2026-10-31): a
    /// failure whose Output log holds words says nothing of an empty Errors
    /// stream**, and offers *Send Output log* — the pass's *no camera* ending,
    /// status 1, undetermined. Where the Output log holds no words either, the
    /// empty Errors stream is still said, beside Output's own sentence.
    #[test]
    fn live_pass_7_an_empty_errors_stream_is_unsaid_beside_an_output_log_with_words() {
        use ACollectionOutcome::*;
        let mut no_camera = kitchen(TheCause::Undetermined, Collected, CollectedAndTheStreamWasEmpty);
        no_camera.at_frame = None;
        no_camera.frames_on_the_server = 0;
        let message = the_failure_message(&no_camera);
        assert_eq!(
            message.text,
            "RunPod 4090\nKitchen — final\n🔴 Failed. Blender exited with an error.\nNo frames on the server.\nCause unknown."
        );
        assert_eq!(labels(&message), ["Send Output log", "Submit again \u{2014} 240 frames"]);
        let partly = the_failure_message(&kitchen(TheCause::Undetermined, CollectionIncomplete, CollectedAndTheStreamWasEmpty));
        assert!(partly.text.contains("Output log incomplete.") && !partly.text.contains("Errors"));
        for ordinary in [CollectedAndTheStreamWasEmpty, NeverEstablished] {
            let said = the_failure_message(&kitchen(TheCause::Undetermined, ordinary, CollectedAndTheStreamWasEmpty));
            assert!(said.text.contains("Errors log is empty."), "{ordinary:?}");
            assert!(said.text.contains(&what_the_outcome_says(AStream::TheOrdinaryOutput, ordinary).unwrap()), "{ordinary:?}");
        }
        let output_empty = the_failure_message(&kitchen(TheCause::Undetermined, CollectedAndTheStreamWasEmpty, Collected));
        assert!(output_empty.text.contains("Output log is empty."));
        assert_eq!(labels(&output_empty), ["Send Errors log", "Submit again \u{2014} 240 frames"]);
    }

    /// **The Creator's eighth notes, note 6 (2026-10-11): an empty stream's
    /// sentence claims only what is true of that stream.** Blender 5 writes its
    /// errors to its ordinary output, so neither sentence says the render
    /// reported no errors or that there is nothing further to read.
    #[test]
    fn an_empty_streams_sentence_claims_nothing_about_the_other() {
        use ACollectionOutcome::*;
        use AStream::*;
        let errors = what_the_outcome_says(TheComplaints, CollectedAndTheStreamWasEmpty).unwrap();
        let output = what_the_outcome_says(TheOrdinaryOutput, CollectedAndTheStreamWasEmpty).unwrap();
        assert_eq!(errors, "Errors log is empty.");
        assert_eq!(output, "Output log is empty.");
        for said in [&errors, &output] {
            let said = said.to_lowercase();
            assert!(!said.contains("no errors"), "{said}");
            assert!(!said.contains("nothing further"), "{said}");
        }
        assert!(!errors.contains("Output") && !output.contains("Errors"));
    }

    /// **`Telegram` Y4a as changed 2026-11-12, Creator-stated: the phone
    /// matches the app.** The live pass of that date, new fault 8: on a killed
    /// Blender (an undetermined cause) and on a job interrupted twice, the
    /// app's History offered *Submit again — 40 frames* and *Resume — 34/38
    /// frames*, and the phone offered neither. The file's own keeps none.
    #[test]
    fn live_pass_8_the_phone_offers_the_apps_two_ways_where_the_app_does() {
        use ACollectionOutcome::*;
        let mut unknown = kitchen(TheCause::Undetermined, NeverEstablished, NeverEstablished);
        unknown.resume = Some(AResume { from: 35, frames: 6 });
        let message = the_failure_message(&unknown);
        assert_eq!(labels(&message), ["Submit again \u{2014} 240 frames", "Resume \u{2014} 6 frames"]);
        assert!(message.text.ends_with("\nCause unknown."), "{}", message.text);
        assert_eq!(message.buttons[1].payload["from"], json!(35));

        let the_files = the_failure_message(&kitchen(TheCause::TheFilesOwn, NeverEstablished, NeverEstablished));
        assert!(the_files.buttons.is_empty());
        assert!(the_files.text.ends_with(IT_NEEDS_A_CHANGE));

        let given_up = AGivenUpJob {
            server: "RunPod 4090".into(),
            job: "job-1".into(),
            label: "Kitchen \u{2014} final".into(),
            generation: "1".into(),
            attempt: 2,
            frames_on_the_server: 7,
            ordinary_output: Collected,
            complaints: NeverEstablished,
            frames_asked: 40,
            resume: Some(AResume { from: 8, frames: 33 }),
        };
        let message = the_given_up_message(&given_up);
        assert_eq!(labels(&message), ["Send Output log", "Submit again \u{2014} 40 frames", "Resume \u{2014} 33 frames"]);
        assert!(message.text.contains("\nInterrupted twice. No more retries.\n"));
    }

    /// **Y4a: *try it again* on every cause but the file's own** (as changed
    /// 2026-11-12; it was the machine's own alone); the file's own says the
    /// job needs a change, and undetermined says the cause is unknown. **E3's
    /// words button is independent of it.**
    #[test]
    fn try_it_again_is_attached_on_every_cause_but_the_files_own() {
        use ACollectionOutcome::*;
        let the_files = the_failure_message(&kitchen(TheCause::TheFilesOwn, Collected, Collected));
        assert!(!the_files.buttons.iter().any(|b| b.action == TRY_IT_AGAIN));
        assert!(the_files.text.ends_with(IT_NEEDS_A_CHANGE));
        assert_eq!(labels(&the_files), ["Send Output log", "Send Errors log"]);
        let unknown = the_failure_message(&kitchen(TheCause::Undetermined, Collected, Collected));
        assert!(unknown.text.ends_with(SMOOZY_CANNOT_TELL_WHY));
        assert_eq!(labels(&unknown), ["Send Output log", "Send Errors log", "Submit again \u{2014} 240 frames"]);
        let machines = the_failure_message(&kitchen(TheCause::TheMachinesOwn, NeverEstablished, NeverEstablished));
        assert_eq!(labels(&machines), ["Submit again \u{2014} 240 frames"]);
        assert!(!machines.text.contains(IT_NEEDS_A_CHANGE));
        let neither = the_failure_message(&kitchen(TheCause::Undetermined, NeverEstablished, CollectedAndTheStreamWasEmpty));
        assert_eq!(labels(&neither), ["Submit again \u{2014} 240 frames"]);
    }

    /// **Y4a, corrected 2026-09-16: a failure nobody can explain is not told
    /// as the file's fault.** *Undetermined* carries [`SMOOZY_CANNOT_TELL_WHY`]
    /// and never [`IT_NEEDS_A_CHANGE`]; *the file's own* still carries
    /// [`IT_NEEDS_A_CHANGE`] and never the other; the file's own never offers
    /// *try it again*, and since 2026-11-12 undetermined always does (Y4a as
    /// changed that day), whatever the two streams recorded.
    #[test]
    fn an_undetermined_failure_says_smoozy_cannot_tell_and_never_blames_the_file() {
        use ACollectionOutcome::*;
        let outcomes = [Collected, CollectedAndTheStreamWasEmpty, CollectionIncomplete, NeverEstablished];
        for ordinary in outcomes {
            for complaints in outcomes {
                let neither = the_failure_message(&kitchen(TheCause::Undetermined, ordinary, complaints));
                assert!(neither.text.ends_with(SMOOZY_CANNOT_TELL_WHY), "{ordinary:?} {complaints:?}");
                assert!(!neither.text.contains(IT_NEEDS_A_CHANGE), "{ordinary:?} {complaints:?}");
                assert!(!neither.text.contains("needs a change"), "{ordinary:?} {complaints:?}");
                assert!(neither.buttons.iter().any(|b| b.action == TRY_IT_AGAIN), "{ordinary:?} {complaints:?}");

                let the_files = the_failure_message(&kitchen(TheCause::TheFilesOwn, ordinary, complaints));
                assert!(the_files.text.ends_with(IT_NEEDS_A_CHANGE), "{ordinary:?} {complaints:?}");
                assert!(!the_files.text.contains(SMOOZY_CANNOT_TELL_WHY), "{ordinary:?} {complaints:?}");
                assert!(the_files.buttons.iter().all(|b| b.action != TRY_IT_AGAIN), "{ordinary:?} {complaints:?}");

                let the_machines = the_failure_message(&kitchen(TheCause::TheMachinesOwn, ordinary, complaints));
                assert!(!the_machines.text.contains(IT_NEEDS_A_CHANGE) && !the_machines.text.contains(SMOOZY_CANNOT_TELL_WHY));
                assert!(the_machines.buttons.iter().any(|b| b.action == TRY_IT_AGAIN));
            }
        }
    }

    /// **E2: one outcome per stream, and the four are four sentences** —
    /// collected-and-empty and never-established are never collapsed. **E3: a
    /// button where there are words, incomplete marked.**
    #[test]
    fn every_outcome_says_its_own_sentence_per_stream() {
        use ACollectionOutcome::*;
        let mut said = std::collections::HashSet::new();
        for stream in [AStream::TheOrdinaryOutput, AStream::TheComplaints] {
            assert_eq!(what_the_outcome_says(stream, Collected), None);
            for outcome in [CollectedAndTheStreamWasEmpty, CollectionIncomplete, NeverEstablished] {
                assert!(said.insert(what_the_outcome_says(stream, outcome).unwrap()), "{stream:?} {outcome:?}");
            }
        }
        assert_eq!(said.len(), 6);
        let incomplete = the_failure_message(&kitchen(TheCause::TheFilesOwn, CollectionIncomplete, NeverEstablished));
        assert_eq!(incomplete.buttons.len(), 1);
        assert_eq!(incomplete.buttons[0].payload["incomplete"], json!(true));
        assert!(incomplete.text.contains("Output log incomplete.") && incomplete.text.contains("Errors log not recorded."));
    }

    /// **E3a: nothing the machine printed is ever in a message.** Only the
    /// exit status, a number, goes automatically.
    #[test]
    fn a_message_carries_no_words_of_the_machines() {
        let mut failed = kitchen(TheCause::TheMachinesOwn, ACollectionOutcome::Collected, ACollectionOutcome::Collected);
        failed.exit_status = None;
        failed.at_frame = None;
        failed.frames_on_the_server = 0;
        let message = the_failure_message(&failed);
        assert!(message.text.contains("🔴 Failed. Blender quit unexpectedly."));
        assert!(message.text.contains("No frames on the server."));
        for button in &message.buttons {
            assert!(!button.payload.to_string().contains("printed:"));
        }
    }

    /// **`Rendering` K14b on the phone: an exit at 0 that failed says the
    /// machine's reason, as History does**, and never *code 0* beside *Failed*.
    /// Added 2026-11-06, the last words read before the release.
    #[test]
    fn a_failure_at_nought_says_the_machines_reason() {
        let mut failed = kitchen(TheCause::Undetermined, ACollectionOutcome::Collected, ACollectionOutcome::Collected);
        failed.exit_status = Some(0);
        failed.at_frame = None;
        failed.frames_on_the_server = 0;
        failed.the_reason_the_machine_recorded = Some("Blender exited normally but wrote none of the job's files".to_string());
        let text = the_failure_message(&failed).text;
        assert!(text.contains("🔴 Failed. Blender exited normally but wrote none of the job's files.\n"), "{text}");
        failed.the_reason_the_machine_recorded = None;
        let text = the_failure_message(&failed).text;
        assert!(text.contains("🔴 Failed. Blender exited normally but wrote none of the job's files.\n"), "{text}");
        assert!(!text.contains("code 0"), "{text}");
        // **The other exit-at-0 reason, a render that never reported
        // starting**, reads as the same pair. Added 2026-11-06, the check
        // after the last fix.
        failed.the_reason_the_machine_recorded =
            Some("Blender exited normally but never reported starting to render".to_string());
        let text = the_failure_message(&failed).text;
        assert!(text.contains("🔴 Failed. Blender exited normally but never reported starting to render.\n"), "{text}");
        assert!(!text.contains("code 0"), "{text}");
    }

    fn at(seconds: u64) -> SystemTime {
        SystemTime::UNIX_EPOCH + Duration::from_secs(seconds)
    }

    fn member(ending: Option<AMembersEnding>, start: u64, end: u64) -> AMember {
        AMember { job: format!("job-{start}"), ending, started: Some(at(start)), ended: ending.map(|_| at(end)) }
    }

    /// **G1 and G2: nothing until the last member ends; then one message, and
    /// only where a member finished.** The drawing's `tg-batch`.
    #[test]
    fn a_group_speaks_once_and_only_when_a_member_finished() {
        let finished = Some(AMembersEnding::Finished { frames: 120, files: None });
        let mut members: Vec<AMember> = (0..12).map(|n| member(finished, n * 3600, n * 3600 + 4000)).collect();
        members[11].ending = None;
        assert_eq!(what_the_group_says("Vast 5060", "Product spins", &members), None);
        members[11] = member(finished, 11 * 3600, 50_760);
        assert_eq!(
            what_the_group_says("Vast 5060", "Product spins", &members).unwrap(),
            "Vast 5060\nProduct spins\nFinished. 12 scenes, 1,440 frames in 14 hours 6 minutes."
        );

        let failed: Vec<AMember> = (0..5).map(|n| member(Some(AMembersEnding::Failed), n, n + 1)).collect();
        assert_eq!(what_the_group_says("Vast 5060", "Product spins", &failed), None, "all failed: no finishing message");

        // A member left stopped counts toward the last member.
        let mixed = vec![member(finished, 0, 600), member(Some(AMembersEnding::LeftStopped), 0, 60), member(Some(AMembersEnding::GivenUp), 0, 60)];
        assert_eq!(
            what_the_group_says("Vast 5060", "Product spins", &mixed).unwrap(),
            "Vast 5060\nProduct spins\nFinished. 1 of 3 scenes, 120 frames in 10 minutes."
        );
    }

    /// **E1 and G1: a finished job speaks for itself unless it is in a group.**
    #[test]
    fn a_finished_job_speaks_unless_its_group_does() {
        let mut job = AFinishedJob {
            server: "RunPod 4090".into(),
            label: "Kitchen \u{2014} final".into(),
            frames: 240,
            started: at(0),
            ended: at(6 * 3600 + 120),
            group: None,
            files: None,
        };
        assert_eq!(
            the_finishing_message(&job).unwrap(),
            "RunPod 4090\nKitchen — final\nFinished. 240 frames in 6 hours 2 minutes."
        );
        job.group = Some("group-1".into());
        assert_eq!(the_finishing_message(&job), None);
    }

    /// **E1a's sentence of 2026-11-11: one *Missing:* line for each file, under
    /// the finishing sentence, and nothing where there is none.**
    #[test]
    fn e1a_a_finish_says_missing_for_each_file_the_remap_left_unchanged() {
        let server: AServerLabel = "RunPod 4090".into();
        let said = a_user_message(&server, "Kitchen", "", "Finished. 240 frames.");
        assert_eq!(with_the_files_missing(said.clone(), &[]), said);
        assert_eq!(
            with_the_files_missing(said, &["wood.png".to_string(), "props.blend".to_string()]),
            "RunPod 4090\nKitchen\nFinished. 240 frames.\nMissing: wood.png\nMissing: props.blend"
        );
    }

    /// **T-03 of the UI at every size: a list of missing files too long for
    /// one message stops inside Telegram's bound and says how many more**, so
    /// the finish is still sent. Two hundred 28-letter names were about 7,800
    /// characters, which Telegram refuses whole.
    #[test]
    fn e1a_a_long_missing_list_stays_inside_the_bound_and_counts_the_rest() {
        let server: AServerLabel = "RunPod 4090".into();
        let said = a_user_message(&server, "Kitchen", "", "Finished. 240 frames.");
        let names: Vec<String> = (0..200).map(|at| format!("texture_wood_planks_{at:04}.png")).collect();
        let told = with_the_files_missing(said, &names);
        assert!(told.text.encode_utf16().count() <= THE_BOUND_ON_A_MESSAGE);
        let named = told.text.lines().filter(|line| line.starts_with("Missing: ")).count();
        assert!(named > 50 && named < 200, "{named} named");
        assert_eq!(told.text.lines().last(), Some(format!("and {} more.", 200 - named).as_str()));
        assert!(told.text.contains("Missing: texture_wood_planks_0000.png"), "the first names are the ones kept");
    }

    /// **E1a: a count that came up short takes the frames' place, and the state
    /// word stays.** A whole count, and none at all, read as they did.
    #[test]
    fn e1a_a_finish_whose_count_came_up_short_says_how_many_of_how_many() {
        let job = |files| AFinishedJob {
            server: "RunPod 4090".into(),
            label: "Kitchen".into(),
            frames: 0,
            started: at(0),
            ended: at(120),
            group: None,
            files,
        };
        assert_eq!(
            the_finishing_message(&job(Some((1, 2)))).unwrap(),
            "RunPod 4090\nKitchen\nFinished. 1 of 2 files written in 2 minutes."
        );
        assert_eq!(
            the_finishing_message(&job(Some((1200, 2400)))).unwrap(),
            "RunPod 4090\nKitchen\nFinished. 1,200 of 2,400 files written in 2 minutes."
        );
        assert_eq!(the_finishing_message(&job(Some((2, 2)))).unwrap(), "RunPod 4090\nKitchen\nFinished. 0 frames in 2 minutes.");
        assert_eq!(the_finishing_message(&job(None)).unwrap(), "RunPod 4090\nKitchen\nFinished. 0 frames in 2 minutes.");

        // **G1: over the finished members, where any came up short.**
        let finished = |frames, files| member(Some(AMembersEnding::Finished { frames, files }), 0, 600);
        assert_eq!(
            what_the_group_says("Vast 5060", "Spins", &[finished(120, Some((120, 120))), finished(60, Some((60, 120))), finished(40, None)]).unwrap(),
            "Vast 5060\nSpins\nFinished. 3 scenes, 220 frames, 180 of 240 files written in 10 minutes."
        );
        assert_eq!(
            what_the_group_says("Vast 5060", "Spins", &[finished(120, Some((120, 120))), finished(60, None)]).unwrap(),
            "Vast 5060\nSpins\nFinished. 2 scenes, 180 frames in 10 minutes."
        );
    }

    /// **V1: the three choices, with the floor under all three.**
    #[test]
    fn what_each_reporting_choice_sends() {
        use AKindOfReport::*;
        use TheReportingChoice::*;
        for choice in [
            Everything,
            ThePicturesWithoutTheRunningCommentary,
            APictureEveryFewMinutes { minutes: 5 },
            OnlyWhatNeedsAnAnswer,
        ] {
            for floor in [Finished, Failed, WaitingOnYou, AnAnswerToATap] {
                assert!(the_job_reports(choice, floor), "{choice:?} {floor:?}");
            }
        }
        // **V1a's fourth member sends what `Everything` sends** (2026-10-19);
        // its pictures' pacing is the worker's (I2a).
        for kind in [TheStatusMessage, AFramePicture, AMovingPicture, AContactSheet] {
            assert!(the_job_reports(APictureEveryFewMinutes { minutes: 1 }, kind), "{kind:?}");
        }
        assert_eq!(APictureEveryFewMinutes { minutes: 7 }.the_interval_between_pictures(), Some(std::time::Duration::from_secs(420)));
        assert_eq!(Everything.the_interval_between_pictures(), None);
        assert!(the_job_reports(Everything, TheStatusMessage));
        assert!(!the_job_reports(ThePicturesWithoutTheRunningCommentary, TheStatusMessage));
        assert!(the_job_reports(ThePicturesWithoutTheRunningCommentary, AFramePicture));
        assert!(the_job_reports(ThePicturesWithoutTheRunningCommentary, AContactSheet));
        for kind in [TheStatusMessage, AFramePicture, AMovingPicture, AContactSheet] {
            assert!(!the_job_reports(OnlyWhatNeedsAnAnswer, kind), "{kind:?}");
        }
    }

    #[test]
    fn nothing_sends_no_automatic_report_but_answers_a_tap() {
        use AKindOfReport::*;
        let choice = TheReportingChoice::from_the_manifests_field(&json!("Nothing"));
        for kind in [TheStatusMessage, AFramePicture, AMovingPicture, AContactSheet,
            AFinishedVideo, Finished, Failed, WaitingOnYou] {
            assert!(!the_job_reports(choice, kind), "{kind:?}");
        }
        assert!(the_job_reports(choice, AnAnswerToATap));
    }

    /// **V1a: read from the manifest's field as `Rendering` S5a writes it.**
    #[test]
    fn the_manifests_field_is_read_as_an_identifier() {
        assert_eq!(TheReportingChoice::from_the_manifests_field(&json!("Everything")), TheReportingChoice::Everything);
        assert_eq!(
            TheReportingChoice::from_the_manifests_field(&json!("ThePicturesWithoutTheRunningCommentary")),
            TheReportingChoice::ThePicturesWithoutTheRunningCommentary
        );
        assert_eq!(
            TheReportingChoice::from_the_manifests_field(&json!("OnlyWhatNeedsAnAnswer")),
            TheReportingChoice::OnlyWhatNeedsAnAnswer
        );
        assert_eq!(TheReportingChoice::from_the_manifests_field(&json!("Everything, please")), TheReportingChoice::OnlyWhatNeedsAnAnswer);
    }

    /// **V1a's fourth member, 2026-10-19: read with its minutes, 1 to 60**;
    /// minutes missing, not whole or out of range, and any other shape, read
    /// as the narrowest, as every value not known is.
    /// **`Telegram` V1c: the record's value wins over the manifest's, and only
    /// one of the five shapes may be set.**
    #[test]
    fn twenty_first_7_the_record_s_choice_is_in_force_and_only_five_shapes_are_set() {
        assert_eq!(
            TheReportingChoice::in_force(Some("\"Nothing\""), &json!("Everything")),
            TheReportingChoice::Nothing
        );
        assert_eq!(
            TheReportingChoice::in_force(None, &json!("Everything")),
            TheReportingChoice::Everything
        );
        assert_eq!(
            TheReportingChoice::in_force(Some("not json"), &json!("Everything")),
            TheReportingChoice::OnlyWhatNeedsAnAnswer
        );
        for choice in [
            TheReportingChoice::Everything,
            TheReportingChoice::ThePicturesWithoutTheRunningCommentary,
            TheReportingChoice::APictureEveryFewMinutes { minutes: 1 },
            TheReportingChoice::APictureEveryFewMinutes { minutes: 60 },
            TheReportingChoice::OnlyWhatNeedsAnAnswer,
            TheReportingChoice::Nothing,
        ] {
            assert_eq!(TheReportingChoice::exactly(&choice.in_the_manifests_form()), Some(choice));
        }
        for refused in [
            json!("Everything, please"),
            json!({ "APictureEveryFewMinutes": { "minutes": 0 } }),
            json!({ "APictureEveryFewMinutes": { "minutes": 61 } }),
            json!(null),
        ] {
            assert_eq!(TheReportingChoice::exactly(&refused), None, "{refused}");
        }
    }

    #[test]
    fn the_fourth_member_is_read_with_its_minutes_and_a_malformed_one_as_the_narrowest() {
        let read = |field: Value| TheReportingChoice::from_the_manifests_field(&field);
        assert_eq!(
            read(json!({"APictureEveryFewMinutes": {"minutes": 5}})),
            TheReportingChoice::APictureEveryFewMinutes { minutes: 5 }
        );
        assert_eq!(read(json!({"APictureEveryFewMinutes": {"minutes": 1}})), TheReportingChoice::APictureEveryFewMinutes { minutes: 1 });
        assert_eq!(read(json!({"APictureEveryFewMinutes": {"minutes": 60}})), TheReportingChoice::APictureEveryFewMinutes { minutes: 60 });
        for malformed in [
            json!({"APictureEveryFewMinutes": {"minutes": 0}}),
            json!({"APictureEveryFewMinutes": {"minutes": 61}}),
            json!({"APictureEveryFewMinutes": {"minutes": 2.5}}),
            json!({"APictureEveryFewMinutes": {"minutes": "5"}}),
            json!({"APictureEveryFewMinutes": {}}),
            json!({"APictureEveryFewMinutes": {"minutes": 5, "seconds": 3}}),
            json!({"APictureEveryFewMinutes": 5}),
            json!("APictureEveryFewMinutes"),
            json!({"APictureEveryFewMinutes": {"minutes": 5}, "Everything": null}),
        ] {
            assert_eq!(read(malformed.clone()), TheReportingChoice::OnlyWhatNeedsAnAnswer, "{malformed}");
        }
    }

    /// **V3: once per idle stretch, only past the half hour and with the
    /// switch on**; a new stretch speaks again.
    #[test]
    fn the_idle_message_is_spoken_once_per_stretch() {
        let mut idle = TheIdleMessage::default();
        assert_eq!(idle.what_to_say("Overnight box", true, true, Some("q1"), false), None);
        assert_eq!(idle.what_to_say("Overnight box", false, true, Some("q1"), true), None);
        assert_eq!(
            idle.what_to_say("Overnight box", true, true, Some("q1"), true).unwrap(),
            format!("Overnight box\n{NOTHING_TO_DO}")
        );
        assert_eq!(idle.what_to_say("Overnight box", true, true, Some("q1"), true), None);
        assert_eq!(idle.what_to_say("Overnight box", true, false, Some("q1"), true), None);
        assert!(idle.what_to_say("Overnight box", true, true, Some("q2"), true).is_some());
    }

    /// **A2 and A3: the drawing's `tg-check`, and the loser told which answer
    /// won and when.**
    #[test]
    fn the_waiting_job_and_its_two_answers() {
        let waiting = AWaitingJob {
            server: "RunPod 4090".into(),
            label: "Kitchen \u{2014} final".into(),
            hold: "hold-1".into(),
            generation: "1".into(),
            waits_for: WhatTheHoldWaitsFor::TheUsersAnswer,
            the_frame_shown: 1,
            frames_held: 240,
            the_frames: the_frames_in_words(1..=240),
        };
        let message = the_waiting_message(&waiting).expect("a hold awaiting the user is told");
        assert_eq!(
            message.text,
            "RunPod 4090\nKitchen \u{2014} final \u{b7} frames 1\u{2013}240\nNeeds your answer. Render frames 1\u{2013}240?"
        );
        assert_eq!(the_frames_in_words([1, 3, 5, 6, 7]), "1,3,5\u{2013}7");
        let unread = AWaitingJob { the_frames: String::new(), ..waiting.clone() };
        assert!(the_waiting_message(&unread).unwrap().text.ends_with("Render 240 frames?"));
        // **The twenty-eighth notes, note 16**: *Cancel job*, which asks in
        // place first; *Render all* answers at once.
        assert_eq!(labels(&message), ["Render all \u{2014} 240 frames", "Cancel job"]);
        assert_eq!(
            message.buttons.iter().map(|b| b.action).collect::<Vec<_>>(),
            [ANSWER_A_WAITING_JOB, ASK_TO_CANCEL_THE_JOB]
        );
        assert_eq!(
            the_answer_already_applied("RunPod 4090", "Kitchen \u{2014} final", TheAnswer::GoOn, 240, at(0)),
            "RunPod 4090\nKitchen — final\nAlready answered “Render all — 240 frames” at 1 Jan 1970, 00:00:00 UTC."
        );
        // **The drop names the held count without *the other***: the held job
        // holds frame 1 too since decision C (2026-10-02).
        assert_eq!(
            the_drop_message("RunPod 4090", "Kitchen \u{2014} final", 240, 1, TheDependencyEnded::Failed),
            "RunPod 4090\nKitchen \u{2014} final\n240 frames not rendered: frame 1 failed."
        );
        // **The two endings added for the Telegram worker's piece 3** are told
        // as what they were, never as a failure or a stop.
        assert!(the_drop_message("A", "K", 2, 1, TheDependencyEnded::GivenUp).ends_with("2 frames not rendered: frame 1 was interrupted twice."));
        assert!(the_drop_message("A", "K", 2, 1, TheDependencyEnded::Cancelled).ends_with("2 frames not rendered: frame 1 was cancelled."));
    }

    /// **The four questions, scout 3.46 (Sol's narrowed form): the drop says
    /// the count first and the reason after, without *held*** —
    /// `Architecture/Telegram` A4's build detail of 2026-11-12.
    #[test]
    fn fifth_part_3_46_the_drop_names_its_count_without_held() {
        assert_eq!(
            the_drop_message("RunPod 4090", "Kitchen \u{2014} final", 240, 1, TheDependencyEnded::Failed),
            "RunPod 4090\nKitchen \u{2014} final\n240 frames not rendered: frame 1 failed."
        );
        assert!(the_drop_message("A", "K", 1, -3, TheDependencyEnded::Stopped).text.ends_with("1 frame not rendered: frame -3 was stopped."));
    }

    /// **`Queue & History` L22's drop and `State, identity and recovery` E5's
    /// gap, in the words the Telegram worker's piece 3 sends.**
    #[test]
    fn a_drop_for_a_removed_blender_and_a_gap_are_said_plainly() {
        assert_eq!(
            the_drop_for_a_removed_blender("RunPod 4090", "Kitchen", "4.2.3"),
            "RunPod 4090\nKitchen\nNot rendered: Blender 4.2.3 was removed from this server."
        );
        let gap = some_events_were_missed("RunPod 4090");
        assert!(gap.starts_with("RunPod 4090\n"));
        assert!(gap.contains("were not sent"), "{gap}");
    }

    /// **A3, held-job piece B: a hold for the driver update sends no waiting
    /// message and no buttons**, and a hold awaiting the user still sends both.
    /// The rule's one home says the same of the message on its release.
    #[test]
    fn a_hold_for_the_driver_update_sends_no_waiting_message_and_no_buttons() {
        let mut waiting = AWaitingJob {
            server: "RunPod 4090".into(),
            label: "Kitchen \u{2014} night".into(),
            hold: "hold-7".into(),
            generation: "1".into(),
            waits_for: WhatTheHoldWaitsFor::TheDriverUpdate,
            the_frame_shown: 1,
            frames_held: 240,
            the_frames: String::new(),
        };
        assert_eq!(the_waiting_message(&waiting), None, "nothing waits on the user");
        assert!(!WhatTheHoldWaitsFor::TheDriverUpdate.the_phone_is_told());
        waiting.waits_for = WhatTheHoldWaitsFor::TheUsersAnswer;
        assert_eq!(the_waiting_message(&waiting).map(|message| message.buttons.len()), Some(2));
        assert!(WhatTheHoldWaitsFor::TheUsersAnswer.the_phone_is_told());
    }

    /// **D2 and D4: the stop's two acknowledgements and a second press.**
    #[test]
    fn the_stops_words() {
        assert_eq!(the_stop_was_sent("RunPod 4090", "Kitchen"), "RunPod 4090\nKitchen\nStop sent.");
        let stopped = AStopsOutcome::Stopped { frames_on_the_server: 147, frames_not_rendered: 93 };
        assert_eq!(
            the_stops_outcome("RunPod 4090", "Kitchen", &stopped),
            "RunPod 4090\nKitchen\nStopped. 147 frames on the server; 93 frames not rendered."
        );
        assert!(the_stops_outcome("RunPod 4090", "Kitchen", &AStopsOutcome::AlreadyStopped { at: at(0) }).contains("Already stopped at"));
    }

    #[test]
    fn counts_and_lengths_read_as_the_drawing_writes_them() {
        assert_eq!(with_thousands(7), "7");
        assert_eq!(with_thousands(1440), "1,440");
        assert_eq!(with_thousands(1_234_567), "1,234,567");
        assert_eq!(how_long_in_words(Duration::from_secs(40)), "40 seconds");
        assert_eq!(how_long_in_words(Duration::from_secs(3600)), "1 hour");
        assert_eq!(how_long_in_words(Duration::from_secs(3660)), "1 hour 1 minute");
    }

    /// **`Telegram` M10: a finished video fits at 50,000,000 bytes and not one
    /// byte over** — §21 item 32's smaller reading, the one cap the build
    /// keeps; its size is said as the desktop's Transfers says one; and it is
    /// sent where the contact sheet is, and not under *only what needs an
    /// answer*. The nineteenth notes, note 4.
    #[test]
    fn video_wants_the_cap_the_words_for_a_size_and_which_choices_send_the_video() {
        assert!(a_finished_video_fits(0));
        assert!(a_finished_video_fits(50_000_000));
        assert!(!a_finished_video_fits(50_000_001));
        for (bytes, words) in [
            (0, "0 B"),
            (999, "999 B"),
            (2_000, "2.0 KB"),
            (38_200_000, "38.2 MB"),
            (50_000_000, "50.0 MB"),
            (1_400_000_000, "1.4 GB"),
            (2_500_000_000_000, "2.5 TB"),
        ] {
            assert_eq!(a_size_in_words(bytes), words, "{bytes}");
        }
        for choice in [
            TheReportingChoice::Everything,
            TheReportingChoice::ThePicturesWithoutTheRunningCommentary,
            TheReportingChoice::APictureEveryFewMinutes { minutes: 5 },
            TheReportingChoice::OnlyWhatNeedsAnAnswer,
        ] {
            assert_eq!(
                the_job_reports(choice, AKindOfReport::AFinishedVideo),
                the_job_reports(choice, AKindOfReport::AContactSheet),
                "{choice:?}"
            );
        }
        assert!(!the_job_reports(TheReportingChoice::OnlyWhatNeedsAnAnswer, AKindOfReport::AFinishedVideo));
    }

    /// **Sol's S2 (the four questions, batch 4, 2026-11-11): the first-frame
    /// question and its drop name the checked frame by its own number**, below
    /// zero too. Before, a frame below zero was said as frame 1.
    #[test]
    fn four_questions_s2_the_checked_frame_below_zero_is_named_as_it_is() {
        let dropped = the_drop_message("S", "J", 3, -2, TheDependencyEnded::Failed);
        assert!(dropped.text.contains("not rendered: frame -2 failed"), "{}", dropped.text);
        let waiting = AWaitingJob {
            server: "S".into(),
            label: "J".into(),
            hold: "h".into(),
            generation: "1".into(),
            waits_for: WhatTheHoldWaitsFor::TheUsersAnswer,
            the_frame_shown: -2,
            frames_held: 3,
            the_frames: String::new(),
        };
        let asked = the_waiting_message(&waiting).unwrap().text;
        assert!(asked.text.contains(" · frame -2\n"), "{}", asked.text);
    }

    /// **The four questions, batch 4 (scout 1.5), Creator-stated 2026-11-11:
    /// the phone offers the app's two ways, in the app's words, and never
    /// *Retry*** (`Specs/Rendering`, `Telegram` Y4a as changed that day):
    /// *Submit again — 240 frames* for the whole job, and *Resume — 181
    /// frames* where the job's entry kept the frame it ended at, carrying that
    /// frame. The answer to *Resume* says where it started and the two folders.
    /// Before, the one button read *Retry* and sent the whole range again.
    #[test]
    fn four_questions_1_5_submit_again_and_resume_never_retry() {
        use ACollectionOutcome::*;
        let mut job = kitchen(TheCause::TheMachinesOwn, NeverEstablished, NeverEstablished);
        job.frames_asked = 240;
        job.resume = Some(AResume { from: 60, frames: 181 });
        let message = the_failure_message(&job);
        assert_eq!(labels(&message), ["Submit again \u{2014} 240 frames", "Resume \u{2014} 181 frames"]);
        let resume = message.buttons.iter().find(|b| b.action == RESUME_IT).expect("Resume");
        assert_eq!(resume.payload["from"], serde_json::json!(60));
        job.resume = None;
        job.frames_asked = 1;
        assert_eq!(labels(&the_failure_message(&job)), ["Submit again \u{2014} 1 frame"]);
        job.cause = TheCause::TheFilesOwn;
        job.resume = Some(AResume { from: 60, frames: 181 });
        assert!(the_failure_message(&job).buttons.is_empty(), "the file's own: neither");
        assert_eq!(
            the_submissions_outcome("S", "J", &ASubmissionsOutcome::Resumed { from: -4 }),
            a_user_message("S", "J", "", "Resumed from frame -4. The frames will be in two folders.")
        );
    }
}

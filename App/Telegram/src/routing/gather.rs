//! **The gather — `Telegram` R7 to R14.** *How is it going* as one message
//! covering every machine, and *send a frame*'s candidates.
//!
//! **R10: only one machine hears the request**, and no machine reaches another
//! except through the conversation, so the listener boards a state-request
//! addressed to every roster machine (B14); each answers with the note's and
//! the line's facts and its own latest reading, through its machinery message;
//! and the listener composes **one** message from the replies that arrive
//! within its own deadline, naming which machines answered and which did not.
//! **A machine that did not answer is listed as not answering, never
//! invented.** Nothing durable is written from a reading.
//!
//! **R11: a gather is on demand only.** Nothing here runs on a schedule; a
//! gather exists only because a typed request boarded one.
//!
//! **R12: the deadline is the listener's own**, `Telegram` B12's ten seconds,
//! strictly greater than B13's two-second read.
//!
//! **R7 to R9: every line names the machine and the job by their labels**, and
//! nothing is routed by a label. R8: a machine names itself by the display
//! material it was last told, delivered with its role configuration.
//!
//! **The words are the drawing's** (`Preparation/Design/Mockup/Smoozy.html`,
//! `tg-ask`): one line per server — *Kitchen — final · frame 148 of 240 · 62%*,
//! or *idle* (*nothing to do* until step 13's words sweep). **Not drawn: the
//! line for a server that did not answer**, written here as *did not answer*.

use crate::job_messages::{AMarkedText, AServerLabel, a_user_message};
use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::board::instructions::{
    AKindOfEntry, AnEntry, TheAddressee, THE_DEFAULT_RESOLUTION_DEADLINE, THE_PROTOCOL_VERSION,
};

/// **`Telegram` R10 — how far along a rendering job is**, as the line and the
/// machine's latest reading hold it. Composed into words, never computed here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TheProgress {
    pub frame: u64,
    pub of: u64,
    pub percent: u8,
}

/// **One job as its machine holds it**: what the job is called — the note's
/// label (`Queue & History` N1), or the `.blend`'s name where there is none
/// (`Telegram` R7a) — the subject generation, whether its record directory still exists
/// (R13), and the line's progress where it is rendering.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AJobAsHeld {
    pub job: String,
    pub label: String,
    pub generation: String,
    pub record_directory: bool,
    pub rendering: Option<TheProgress>,
    /// **A job not rendering that is still work**: held for the user's answer,
    /// or queued. Added by the round-1 code audit of the bot and the chat,
    /// fix-here item 5, so the gather never says *idle* over either.
    #[serde(default)]
    pub waiting: Option<AJobWaits>,
}

/// **Why a job that is not rendering is still work.**
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AJobWaits {
    /// Held for the user's answer — money spent while nothing renders.
    OnYourAnswer,
    /// **Held until the server's NVIDIA driver is updated** (`Rendering`
    /// S18a). Nothing waits on the user, so its line reads as its queue entry
    /// does and never *waiting on you* (`Telegram` A3). Held-job piece B,
    /// 2026-10-12.
    ForTheDriverUpdate,
    /// In the line.
    Queued,
}

/// D0: the exact job's state, read again before confirmation is submitted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AStopState {
    Rendering,
    Waiting,
    Ended,
}

/// **What a machine holds, read on that machine.** On a rented machine these
/// are the note, the line and the reading, which step 8b's machine half hands
/// in; a desk process hands in its own.
pub trait ReadsWhatThisMachineHolds: Send {
    fn the_jobs(&self) -> Vec<AJobAsHeld>;

    /// **The live generation of one job**, or nothing where the job is gone.
    fn the_live_generation(&self, subject: &str) -> Option<String> {
        self.the_jobs()
            .into_iter()
            .find(|j| j.job == subject)
            .map(|j| j.generation)
    }

    fn the_stop_state(&self, job: &str) -> AStopState {
        match self.the_jobs().iter().find(|one| one.job == job) {
            Some(one) if one.waiting.is_some() => AStopState::Waiting,
            Some(one) if one.rendering.is_some() => AStopState::Rendering,
            Some(_) => AStopState::Ended,
            None if self.the_live_generation(job).is_some() => AStopState::Rendering,
            None => AStopState::Ended,
        }
    }

    /// **The jobs, and which of the rendering ones a stop's interrupt has gone
    /// out on**, off one reading — `Telegram` C1's *Stopping.*, read from the
    /// queue reading's *stopping* mark (`Queue & History` R5). A reader that
    /// holds no such mark names none. Added 2026-10-19, the live pass's faults,
    /// piece E.
    fn the_jobs_and_those_being_stopped(&self) -> (Vec<AJobAsHeld>, Vec<String>) {
        (self.the_jobs(), Vec::new())
    }
}

/// **`Telegram` R10 and B11 — one machine's reply to a gather**, carried in its
/// machinery message until the state-request leaves the board.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AReply {
    /// The state-request entry this answers; the listener keeps the reply in
    /// its machinery message while that entry is on the board.
    pub operation: String,
    pub gather: String,
    pub machine: String,
    pub display: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub colour: Option<String>,
    pub jobs: Vec<AJobAsHeld>,
    /// The moment of the reading on the machine's own clock (C1).
    pub moment: String,
    /// **`Telegram` V1d — whether this machine's program acts on the phone's
    /// choice.** Every reply from the release that brings V1d says so; a
    /// reply from an older program carries no such field and reads as
    /// `false`, which is how the listener knows that machine needs a Smoozy
    /// update for it: an older program never acknowledges an action it does
    /// not know (it is tried again for ever, B12), so asking first is the one
    /// way to tell. Added 2026-10-31, the Telegram discussion, the server half.
    #[serde(default)]
    pub speaks_the_phones_choice: bool,
}

impl AReply {
    pub fn as_value(&self) -> Value {
        json!(self)
    }
}

/// **What a gather is for**: the plain request, R13's disambiguation, and,
/// from 2026-10-31, R13a's two and V1d's round. A gather's purpose is read
/// back from its entries by whichever machine composes it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatAGatherIsFor {
    HowIsItGoing,
    SendAFrame(u64),
    /// R13a: `frame` alone.
    SendTheNewestFrame,
    /// R13a: `stop`.
    Stop,
    /// V1d: which machines can take the phone's choice, before it is told.
    ThePhonesChoice,
}

/// **`Telegram` R10 and B12 — one state-request entry for one roster machine.**
/// B8's addressee is one machine, so a gather is one entry per machine, all
/// under one gather identity seeded by the typed request's update.
pub fn a_state_request(
    gather: &str,
    machine: &str,
    purpose: WhatAGatherIsFor,
    update: i64,
) -> AnEntry {
    // **A purpose an older composer does not know reads to it as the plain
    // request**, so a hand-over mid-gather costs a status message, never a
    // wrong act.
    let (action, payload) = match purpose {
        WhatAGatherIsFor::HowIsItGoing => ("how is it going", json!({ "gather": gather })),
        WhatAGatherIsFor::SendAFrame(frame) => {
            ("send frame", json!({ "gather": gather, "frame": frame }))
        }
        WhatAGatherIsFor::SendTheNewestFrame => ("send the newest frame", json!({ "gather": gather, "purpose": "newest frame" })),
        WhatAGatherIsFor::Stop => ("stop a job", json!({ "gather": gather, "purpose": "stop" })),
        WhatAGatherIsFor::ThePhonesChoice => ("the phone's choice", json!({ "gather": gather, "purpose": "the phone's choice" })),
    };
    AnEntry {
        version: THE_PROTOCOL_VERSION,
        operation: format!("{gather}-{machine}"),
        addressee: TheAddressee::Machine(machine.to_string()),
        kind: AKindOfEntry::StateRequest,
        subjects: Vec::new(),
        action: action.to_string(),
        payload,
        source_update: Some(update),
        accepted_at: String::new(),
        deadline_seconds: THE_DEFAULT_RESOLUTION_DEADLINE.as_secs(),
    }
}

/// The gather identity for a typed request.
pub fn the_gather_for(update: i64) -> String {
    format!("ask-{update}")
}

/// **Which gather a state-request belongs to, and what it is for**, or nothing
/// where the entry is not a state-request.
pub fn the_gather_of(entry: &AnEntry) -> Option<(String, WhatAGatherIsFor)> {
    if entry.kind != AKindOfEntry::StateRequest {
        return None;
    }
    let gather = entry.payload.get("gather")?.as_str()?.to_string();
    let purpose = match (entry.payload.get("frame").and_then(Value::as_u64), entry.payload.get("purpose").and_then(Value::as_str)) {
        (Some(frame), _) => WhatAGatherIsFor::SendAFrame(frame),
        (None, Some("newest frame")) => WhatAGatherIsFor::SendTheNewestFrame,
        (None, Some("stop")) => WhatAGatherIsFor::Stop,
        (None, Some("the phone's choice")) => WhatAGatherIsFor::ThePhonesChoice,
        _ => WhatAGatherIsFor::HowIsItGoing,
    };
    Some((gather, purpose))
}

/// The job the report actually names; controls use the same selection.
pub fn the_job_named(reply: &AReply) -> Option<&AJobAsHeld> {
    reply
        .jobs
        .iter()
        .find(|job| job.rendering.is_some())
        .or_else(|| {
            [
                AJobWaits::OnYourAnswer,
                AJobWaits::Queued,
                AJobWaits::ForTheDriverUpdate,
            ]
            .into_iter()
            .find_map(|waits| reply.jobs.iter().find(|job| job.waiting == Some(waits)))
        })
}

fn a_line(display: &AServerLabel, reply: Option<&AReply>) -> AMarkedText {
    let Some(reply) = reply else { return a_user_message(display, "", "", "Did not answer."); };
    let Some(job) = the_job_named(reply) else { return a_user_message(display, "", "", "Idle."); };
    let state = if let Some(progress) = job.rendering {
        if progress.frame == 0 || progress.of == 0 { "Rendering.".to_string() }
        else { return a_user_message(display, &job.label, &format!(" · frame {} of {} · {}%", progress.frame, progress.of, progress.percent), ""); }
    } else {
        match job.waiting.unwrap() {
            AJobWaits::OnYourAnswer => "Needs your answer.",
            AJobWaits::Queued => "Queued.",
            AJobWaits::ForTheDriverUpdate => "Waiting for driver update.",
        }.to_string()
    };
    a_user_message(display, &job.label, "", &state)
}

/// `Telegram` R10/R7a: one reply, with a marked head for each server asked.
pub fn how_it_is_going(asked: &[(String, AServerLabel)], replies: &BTreeMap<String, AReply>) -> AMarkedText {
    let mut out = AMarkedText::default();
    for (machine, display) in asked {
        let reply = replies.get(machine);
        if !out.text.is_empty() { out.append("\n", None); }
        let name = reply.map(|r| AServerLabel::with_colour(&r.display, r.colour.as_deref())).unwrap_or_else(|| display.clone());
        out.joined(&a_line(&name, reply));
    }
    out
}

/// One job a frame can be asked of (R13).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ACandidate {
    pub owner: String,
    pub job: String,
    pub label: String,
    pub generation: String,
    /// **Waiting, rendering or held** in the machine's reply: only such a job
    /// gets a Stop beside it (`Telegram` C1b).
    pub live: bool,
}

/// **`Telegram` R13 — every job on the roster's machines whose record directory
/// still exists**, not only what is rendering, in the order the machines were
/// asked. A machine that did not answer offers none.
///
/// **One candidate per label on each machine** (the Creator's twenty-second
/// notes, note 2 of 2026-10-29: *"a list with duplicate jobs"*). A button names
/// a job by its label alone, so two jobs of one scene — a resubmission, or a
/// first-frame check beside its full range — were two buttons nobody could tell
/// apart. A reply lists live work first and ended jobs newest first, so the one
/// kept is the live one where there is one, else the newest.
pub fn the_candidates(
    asked: &[(String, AServerLabel)],
    replies: &BTreeMap<String, AReply>,
) -> Vec<ACandidate> {
    let mut seen = std::collections::HashSet::new();
    asked
        .iter()
        .filter_map(|(machine, _)| replies.get(machine))
        .flat_map(|reply| {
            reply
                .jobs
                .iter()
                .filter(|j| j.record_directory)
                .map(|j| ACandidate {
                    owner: reply.machine.clone(),
                    job: j.job.clone(),
                    label: j.label.clone(),
                    generation: j.generation.clone(),
                    live: j.rendering.is_some() || j.waiting.is_some(),
                })
        })
        .filter(|candidate| seen.insert((candidate.owner.clone(), candidate.label.clone())))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::staleness::THE_READ_CADENCE;

    #[test]
    fn twenty_first_status_names_each_servers_explicit_colour_and_jobs_with_shifted_ranges() {
        let asked = vec![("m1".into(), AServerLabel::with_colour("🟥 自分", Some("blue"))), ("m2".into(), "旧サーバー".into())];
        let mut reply = a_reply("m1", "🟥 自分", vec![a_job("j1", "Кухня 🎬", Some(TheProgress { frame: 1, of: 3, percent: 33 }))]);
        reply.colour = Some("blue".into());
        let text = how_it_is_going(&asked, &BTreeMap::from([("m1".into(), reply)]));
        assert_eq!(text.text, "🟦 🟥 自分\nКухня 🎬 · frame 1 of 3 · 33%\n旧サーバー\nDid not answer.");
        assert_eq!(text.entities[0]["offset"], 3);
        assert_eq!(text.entities[2]["offset"], "🟦 🟥 自分\nКухня 🎬 · frame 1 of 3 · 33%\n".encode_utf16().count());
        assert_eq!(text.entities.iter().map(|e| e["type"].as_str().unwrap()).collect::<Vec<_>>(), ["bold", "italic", "bold"]);
    }

    fn a_reply(machine: &str, display: &str, jobs: Vec<AJobAsHeld>) -> AReply {
        AReply {
            operation: format!("g-{machine}"),
            gather: "g".into(),
            machine: machine.into(),
            display: display.into(),
            colour: None,
            jobs,
            moment: "t".into(),
            speaks_the_phones_choice: true,
        }
    }

    fn a_job(job: &str, label: &str, rendering: Option<TheProgress>) -> AJobAsHeld {
        AJobAsHeld {
            job: job.into(),
            label: label.into(),
            generation: "1".into(),
            record_directory: true,
            rendering,
            waiting: None,
        }
    }

    /// **R10 and fix-here item 5 — a job held for the answer, or queued, is
    /// never *idle*.**
    #[test]
    fn a_waiting_or_queued_job_is_not_nothing_to_do() {
        let asked = vec![
            ("m1".to_string(), "A".into()),
            ("m2".to_string(), "B".into()),
        ];
        let mut held = a_job("j1", "Kitchen", None);
        held.waiting = Some(AJobWaits::OnYourAnswer);
        let mut queued = a_job("j2", "Turntable", None);
        queued.waiting = Some(AJobWaits::Queued);
        let mut replies = BTreeMap::new();
        replies.insert("m1".into(), a_reply("m1", "A", vec![queued.clone(), held]));
        replies.insert("m2".into(), a_reply("m2", "B", vec![queued]));
        assert_eq!(
            how_it_is_going(&asked, &replies),
            "A\nKitchen\nNeeds your answer.\nB\nTurntable\nQueued."
        );
    }

    /// **A3 and R10, held-job piece B: a job held for the driver update reads
    /// *waiting for driver update*, never *waiting on you*** — and never
    /// *idle*.
    #[test]
    fn a_job_held_for_the_driver_update_is_never_waiting_on_you() {
        let asked = vec![("m1".to_string(), "A".into())];
        let mut held = a_job("j1", "Kitchen \u{2014} night", None);
        held.waiting = Some(AJobWaits::ForTheDriverUpdate);
        let mut replies = BTreeMap::new();
        replies.insert("m1".into(), a_reply("m1", "A", vec![held]));
        assert_eq!(
            how_it_is_going(&asked, &replies),
            "A\nKitchen — night\nWaiting for driver update."
        );
    }

    /// **`Telegram` R10, in the drawing's words: one message, every machine
    /// asked, and the one that did not answer named as not answering.**
    #[test]
    fn one_message_names_who_answered_and_who_did_not() {
        let asked = vec![
            ("m1".to_string(), "RunPod 4090".into()),
            ("m2".to_string(), "Vast 5060".into()),
            ("m3".to_string(), "Overnight box".into()),
            ("m4".to_string(), "Spare box".into()),
        ];
        let mut replies = BTreeMap::new();
        replies.insert(
            "m1".into(),
            a_reply(
                "m1",
                "RunPod 4090",
                vec![a_job(
                    "j1",
                    "Kitchen \u{2014} final",
                    Some(TheProgress {
                        frame: 148,
                        of: 240,
                        percent: 62,
                    }),
                )],
            ),
        );
        replies.insert(
            "m2".into(),
            a_reply(
                "m2",
                "Vast 5060",
                vec![a_job(
                    "j2",
                    "Product spins \u{2014} Scene 02",
                    Some(TheProgress {
                        frame: 31,
                        of: 120,
                        percent: 26,
                    }),
                )],
            ),
        );
        replies.insert("m3".into(), a_reply("m3", "Overnight box", vec![]));
        assert_eq!(
            how_it_is_going(&asked, &replies),
            "RunPod 4090\nKitchen — final · frame 148 of 240 · 62%\nVast 5060\nProduct spins — Scene 02 · frame 31 of 120 · 26%\nOvernight box\nIdle.\nSpare box\nDid not answer."
        );
    }

    /// **`Queue & History` R4 in R10's line: a render with no figure yet is
    /// said to be rendering, never frame nought at nought per cent** — and a
    /// render whose frames are not known the same.
    #[test]
    fn a_render_with_no_figure_yet_is_said_to_be_rendering() {
        let asked = vec![
            ("m1".to_string(), "A".into()),
            ("m2".to_string(), "B".into()),
        ];
        let mut replies = BTreeMap::new();
        replies.insert(
            "m1".into(),
            a_reply(
                "m1",
                "A",
                vec![a_job(
                    "j1",
                    "Kitchen",
                    Some(TheProgress {
                        frame: 0,
                        of: 240,
                        percent: 0,
                    }),
                )],
            ),
        );
        replies.insert(
            "m2".into(),
            a_reply(
                "m2",
                "B",
                vec![a_job(
                    "j2",
                    "Hall",
                    Some(TheProgress {
                        frame: 3,
                        of: 0,
                        percent: 0,
                    }),
                )],
            ),
        );
        assert_eq!(
            how_it_is_going(&asked, &replies),
            "A\nKitchen\nRendering.\nB\nHall\nRendering."
        );
    }

    /// **`Telegram` R13: every job whose record directory exists, finished ones
    /// included, and none from a machine that did not answer.**
    #[test]
    fn the_candidates_include_finished_jobs_and_exclude_the_silent() {
        let asked = vec![
            ("m1".to_string(), "A".into()),
            ("m2".to_string(), "B".into()),
        ];
        let mut finished = a_job("j2", "Turntable", None);
        let mut gone = a_job("j3", "Gone", None);
        gone.record_directory = false;
        finished.generation = "7".into();
        let mut replies = BTreeMap::new();
        replies.insert(
            "m1".into(),
            a_reply(
                "m1",
                "A",
                vec![
                    a_job(
                        "j1",
                        "Kitchen",
                        Some(TheProgress {
                            frame: 1,
                            of: 2,
                            percent: 50,
                        }),
                    ),
                    finished,
                    gone,
                ],
            ),
        );
        let candidates = the_candidates(&asked, &replies);
        assert_eq!(
            candidates
                .iter()
                .map(|c| c.label.as_str())
                .collect::<Vec<_>>(),
            ["Kitchen", "Turntable"]
        );
        assert_eq!(candidates[1].generation, "7");
    }

    /// **The twenty-second notes: one button per label on a machine**, the
    /// live job kept over an ended one of the same name; the same label on
    /// another machine is still its own.
    #[test]
    fn one_label_on_one_machine_is_one_candidate_and_the_live_one_is_kept() {
        let asked = vec![
            ("m1".to_string(), "A".into()),
            ("m2".to_string(), "B".into()),
        ];
        let mut replies = BTreeMap::new();
        replies.insert(
            "m1".into(),
            a_reply(
                "m1",
                "A",
                vec![
                    a_job(
                        "j1",
                        "Kitchen",
                        Some(TheProgress {
                            frame: 1,
                            of: 2,
                            percent: 50,
                        }),
                    ),
                    a_job("j2", "Kitchen", None),
                    a_job("j3", "Hall", None),
                    a_job("j4", "Hall", None),
                ],
            ),
        );
        replies.insert(
            "m2".into(),
            a_reply("m2", "B", vec![a_job("j5", "Kitchen", None)]),
        );
        let candidates = the_candidates(&asked, &replies);
        assert_eq!(
            candidates
                .iter()
                .map(|c| c.job.as_str())
                .collect::<Vec<_>>(),
            ["j1", "j3", "j5"]
        );
        assert!(candidates[0].live);
    }

    /// **`Telegram` R12: the gather's deadline is strictly greater than the
    /// read cadence**, so a healthy machine is never reported as not answering.
    #[test]
    fn the_deadline_is_longer_than_the_read() {
        let entry = a_state_request("ask-5", "m1", WhatAGatherIsFor::SendAFrame(120), 5);
        assert!(entry.deadline() > THE_READ_CADENCE);
        assert_eq!(entry.addressee, TheAddressee::Machine("m1".into()));
        assert_eq!(
            the_gather_of(&entry),
            Some(("ask-5".to_string(), WhatAGatherIsFor::SendAFrame(120)))
        );
        let plain = a_state_request("ask-6", "m1", WhatAGatherIsFor::HowIsItGoing, 6);
        assert_eq!(
            the_gather_of(&plain),
            Some(("ask-6".to_string(), WhatAGatherIsFor::HowIsItGoing))
        );
    }
}

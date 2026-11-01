//! **The holder is the stack's janitor — `Telegram` B4, B11, B17, B18 and
//! B20.** What the holder does about whatever is on top, and what one machine's
//! utterance asks of the board.
//!
//! **B17: nothing ever reads beneath the top pin**, and here nothing can: the
//! only reading is the reported pin. **B18: nothing here deletes anything**;
//! the one deletion, a processed pin's line, is the listener's. **B20: popping
//! every machinery pin is what keeps the cost at one line per utterance,
//! deleted once processed**, since nothing of Smoozy's stays pinned but the
//! board.

use serde_json::Value;

use crate::board::instructions::{admit, AnEntry, replace_a_resolution, without_the_acknowledged, AReplacement, AnAdmission};
use crate::board::roster::{list, AListing, ARosterEntry};
use crate::board::text::{TheBoard, TheUtterance, WhatIsPinned};

/// What the holder does about the reported pin.
#[derive(Debug, Clone, PartialEq)]
pub enum WhatTheJanitorDoes {
    /// Its own board is the reported pin: the steady state (B1).
    Nothing,
    /// **B11**: take in what the machinery message says, then unpin it by its
    /// identity.
    TakeInAndUnpin { id: i64, utterance: TheUtterance },
    /// **B11**: unpin the newest, one top at a time.
    Unpin { id: i64 },
    /// **B11**: the board is absent — re-pin it.
    PinTheBoardAgain,
    /// **B4**: the reported pin is a board that is not this holder's own —
    /// stand down and touch nothing.
    StandDown,
}

/// **`Telegram` B4 and B11, decided.** A board whose identity is not this
/// holder's, or whose content is not what this holder last confirmed, or a
/// board-shaped pin that cannot be read, is displacement.
pub fn what_the_janitor_does(top: &WhatIsPinned, my_board_id: i64, my_board: &TheBoard) -> WhatTheJanitorDoes {
    match top {
        WhatIsPinned::Nothing => WhatTheJanitorDoes::PinTheBoardAgain,
        WhatIsPinned::ABoard { id, board } if *id == my_board_id && board == my_board => WhatTheJanitorDoes::Nothing,
        WhatIsPinned::ABoard { .. } | WhatIsPinned::AnUnreadableBoard { .. } => WhatTheJanitorDoes::StandDown,
        WhatIsPinned::AMachineryMessage { id, utterance } => {
            WhatTheJanitorDoes::TakeInAndUnpin { id: *id, utterance: utterance.clone() }
        }
        WhatIsPinned::SomethingElse { id } => WhatTheJanitorDoes::Unpin { id: *id },
    }
}

/// Something asked of the board that did not fit, and so is owed a truthful
/// message to the user (B2, B14).
#[derive(Debug, Clone, PartialEq)]
pub enum NotAdmitted {
    /// The whole entry, so what the user is told names the button or request
    /// and the job, never an internal action name (R7, Rule 9).
    AnInstruction(Box<AnEntry>),
    AMachine { machine: String, display: crate::job_messages::AServerLabel },
}

/// What one utterance asks of the holder.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct WhatAnUtteranceAsks {
    /// The candidate board, where anything changed. **Nothing is accepted
    /// until this is read back (`Rented-server runtime` T12).**
    pub candidate: Option<TheBoard>,
    pub not_admitted: Vec<NotAdmitted>,
    /// A gather's replies, handed on to piece 3's gather (R11) with who sent
    /// them.
    pub replies: Vec<(String, Value)>,
}

/// **`Telegram` B11 — an acknowledgement trims the matching instruction, an
/// announcement joins the roster, a proposal is appended (B15), a replacement
/// is exchanged by identity or refused (B8, B15), a reply feeds the gather.**
pub fn what_an_utterance_asks(board: &TheBoard, utterance: &TheUtterance, moment: &str) -> WhatAnUtteranceAsks {
    let mut asks = WhatAnUtteranceAsks::default();
    let mut current = board.clone();

    if let Some(trimmed) = without_the_acknowledged(&current, &utterance.acknowledged) {
        current = trimmed;
    }
    if utterance.announce {
        let entry = ARosterEntry { colour: utterance.colour.clone(), machine: utterance.machine.clone(), display: utterance.display.clone() };
        match list(&current, &entry) {
            AListing::Listed(with) => current = with,
            AListing::AlreadyListed => {}
            AListing::DoesNotFit => asks.not_admitted.push(NotAdmitted::AMachine {
                machine: entry.machine,
                display: crate::job_messages::AServerLabel::with_colour(&entry.display, entry.colour.as_deref()),
            }),
        }
    }
    for proposal in &utterance.proposed {
        match admit(&current, proposal, moment) {
            AnAdmission::Accepted(with) => current = with,
            AnAdmission::AlreadyOnTheBoard => {}
            AnAdmission::DoesNotFit => asks.not_admitted.push(NotAdmitted::AnInstruction(Box::new(proposal.clone()))),
        }
    }
    for replacement in &utterance.replacing {
        match replace_a_resolution(&current, replacement, moment) {
            AReplacement::Exchanged(with) => current = with,
            AReplacement::AlreadyReplaced | AReplacement::Refused => {}
            AReplacement::DoesNotFit => asks.not_admitted.push(NotAdmitted::AnInstruction(Box::new(replacement.clone()))),
        }
    }
    asks.replies = utterance
        .replies
        .iter()
        .map(|reply| (utterance.machine.clone(), reply.clone()))
        .collect();
    if &current != board {
        asks.candidate = Some(current);
    }
    asks
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::instructions::tests::an_entry;
    use crate::board::instructions::{AKindOfEntry, TheAddressee};
    use crate::board::text::tests::a_board;
    use serde_json::json;

    #[test]
    fn the_janitor_pops_to_the_board_and_stands_down_under_another() {
        let mine = a_board("m1");
        let top = |w| what_the_janitor_does(&w, 9, &mine);
        assert_eq!(top(WhatIsPinned::ABoard { id: 9, board: mine.clone() }), WhatTheJanitorDoes::Nothing);
        assert_eq!(top(WhatIsPinned::Nothing), WhatTheJanitorDoes::PinTheBoardAgain);
        assert_eq!(top(WhatIsPinned::SomethingElse { id: 12 }), WhatTheJanitorDoes::Unpin { id: 12 });
        assert_eq!(top(WhatIsPinned::ABoard { id: 10, board: a_board("m2") }), WhatTheJanitorDoes::StandDown);
        let mut edited = mine.clone();
        edited.heartbeat = "someone else's".into();
        assert_eq!(top(WhatIsPinned::ABoard { id: 9, board: edited }), WhatTheJanitorDoes::StandDown);
        let utterance = TheUtterance { machine: "m2".into(), announce: true, ..Default::default() };
        assert_eq!(
            top(WhatIsPinned::AMachineryMessage { id: 11, utterance: utterance.clone() }),
            WhatTheJanitorDoes::TakeInAndUnpin { id: 11, utterance }
        );
    }

    #[test]
    fn an_utterance_is_taken_in_whole() {
        let mut board = a_board("m1");
        board.instructions.push(an_entry("done", TheAddressee::Machine("m2".into()), AKindOfEntry::Action));
        let utterance = TheUtterance {
            colour: None,
            machine: "m2".into(),
            display: "Vast".into(),
            announce: true,
            acknowledged: vec!["done".into()],
            proposed: vec![an_entry("new", TheAddressee::Machine("m3".into()), AKindOfEntry::Action)],
            replacing: vec![an_entry("late", TheAddressee::Machine("m2".into()), AKindOfEntry::Action)],
            replies: vec![json!({"job": "x"})],
        };
        let asks = what_an_utterance_asks(&board, &utterance, "t");
        let candidate = asks.candidate.unwrap();
        let operations: Vec<&str> = candidate.instructions.iter().map(|e| e.operation.as_str()).collect();
        assert_eq!(operations, ["new"], "trimmed, appended, and the late replacement refused");
        assert!(candidate.roster.iter().any(|r| r.machine == "m2"));
        assert_eq!(asks.replies, vec![("m2".to_string(), json!({"job": "x"}))]);
        assert!(asks.not_admitted.is_empty());
    }

    #[test]
    fn nothing_changes_where_nothing_was_asked() {
        let board = a_board("m1");
        let utterance = TheUtterance { machine: "m1".into(), display: "RunPod 4090".into(), announce: true, ..Default::default() };
        assert_eq!(what_an_utterance_asks(&board, &utterance, "t").candidate, None);
    }

    #[test]
    fn what_does_not_fit_is_named() {
        let board = a_board("m1");
        let mut big = an_entry("big", TheAddressee::Machine("m3".into()), AKindOfEntry::Action);
        big.payload = json!("x".repeat(5000));
        let utterance = TheUtterance {
            colour: None,
            machine: "m2".into(),
            display: "y".repeat(5000),
            announce: true,
            proposed: vec![big],
            ..Default::default()
        };
        let asks = what_an_utterance_asks(&board, &utterance, "t");
        assert_eq!(asks.candidate, None);
        assert_eq!(asks.not_admitted.len(), 2);
    }
}

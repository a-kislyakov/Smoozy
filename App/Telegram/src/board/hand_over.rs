//! **Taking the slot — `Telegram` B3, B6 and B7.** The claimant's decisions,
//! and the successor board it composes.
//!
//! **B3: accepted instructions are preserved before the pin moves.** The
//! claimant reads down to the current board, copies its whole instruction list
//! by identity, pins its own board, reads it back, and only then unpins the old
//! board **by that board's own identity — never by a bare unpin**, which the
//! client does not even offer. A death before the new pin leaves the old board
//! authoritative; a death after it leaves the new one.
//!
//! **B7: which machine holds is emergent**; nothing here prefers one.

use std::time::Duration;

use crate::board::janitor::{what_an_utterance_asks, NotAdmitted};
use crate::board::roster::{list, AListing, ARosterEntry};
use crate::board::text::{TheBoard, TheRouteMode, TheUtterance, WhatIsPinned};

/// What a machine whose ask was accepted does about the reported pin.
#[derive(Debug, Clone, PartialEq)]
pub enum WhatTheClaimantDoes {
    /// **B3**: process a transient machinery message and unpin it by identity.
    TakeInAndUnpin { id: i64, utterance: TheUtterance },
    /// A pin that is neither a board nor a machinery message: unpinned by
    /// identity, as B11's janitor would, until a board can be read.
    Unpin { id: i64 },
    /// The current board, to copy.
    CopyIt { id: i64, board: TheBoard },
    /// No pin at all: there is no board to preserve.
    StartAFreshBoard,
    /// **B3's stale-board rule**: an unreadable board whose text has not
    /// changed across a full renewal interval is stale. A fresh board is started
    /// with nothing copied, the stale one is unpinned by this identity once the
    /// fresh board is confirmed, and the user is told the instructions on it
    /// could not be carried over.
    StartAFreshBoardOverAStaleOne { id: i64 },
    /// **B3**: a malformed old board is not silently replaced — stand down and
    /// report the board unreadable.
    StandDownAndSayTheBoardIsUnreadable,
}

/// **`Telegram` B3 — what a claimant does about the reported pin.**
/// `the_unreadable_board_is_stale` is [`TheUnreadableBoardWatch::is_stale`]'s
/// answer for that pin.
pub fn what_the_claimant_does(top: &WhatIsPinned, the_unreadable_board_is_stale: bool) -> WhatTheClaimantDoes {
    match top {
        WhatIsPinned::Nothing => WhatTheClaimantDoes::StartAFreshBoard,
        WhatIsPinned::ABoard { id, board } => WhatTheClaimantDoes::CopyIt { id: *id, board: board.clone() },
        WhatIsPinned::AMachineryMessage { id, utterance } => {
            WhatTheClaimantDoes::TakeInAndUnpin { id: *id, utterance: utterance.clone() }
        }
        WhatIsPinned::SomethingElse { id } => WhatTheClaimantDoes::Unpin { id: *id },
        WhatIsPinned::AnUnreadableBoard { id, .. } if the_unreadable_board_is_stale => {
            WhatTheClaimantDoes::StartAFreshBoardOverAStaleOne { id: *id }
        }
        WhatIsPinned::AnUnreadableBoard { .. } => WhatTheClaimantDoes::StandDownAndSayTheBoardIsUnreadable,
    }
}

/// **`Telegram` B3's stale-board test, on B5's own terms** — what completed
/// reads have shown of an unreadable board: its identity and text, compared for
/// equality only, and since when, on this machine's own clock. Nothing is
/// subtracted across clocks (`State, identity and recovery` D5). Added by the
/// round-1 code audit of the bot and the chat, U3 and fix-here item 9: before
/// it, a board from a later version whose holder was gone left every older
/// machine standing down for good.
#[derive(Debug, Clone, Default)]
pub struct TheUnreadableBoardWatch {
    seen: Option<(i64, String)>,
    unchanged_since: Option<Duration>,
}

impl TheUnreadableBoardWatch {
    /// **A read completed at `at`.** An unreadable board with different text
    /// starts the count again; a readable board, or nothing pinned, ends it; a
    /// machinery message or anything else on top is not a reading of the board
    /// and moves nothing. A read that did not complete is never handed in (B5).
    pub fn a_read_completed(&mut self, at: Duration, top: &WhatIsPinned) {
        match top {
            WhatIsPinned::AnUnreadableBoard { id, text } => {
                if self.seen.as_ref().is_none_or(|(seen_id, seen_text)| seen_id != id || seen_text != text) {
                    self.seen = Some((*id, text.clone()));
                    self.unchanged_since = Some(at);
                }
            }
            WhatIsPinned::Nothing | WhatIsPinned::ABoard { .. } => {
                self.seen = None;
                self.unchanged_since = None;
            }
            WhatIsPinned::AMachineryMessage { .. } | WhatIsPinned::SomethingElse { .. } => {}
        }
    }

    /// **Whether `top` is an unreadable board whose text completed reads have
    /// shown unchanged for at least `renewal`** — the claimant's own stated
    /// interval, since the unreadable board's cannot be read. A live holder of
    /// any version changes its text at every renewal (B4), so it never is.
    pub fn is_stale(&self, top: &WhatIsPinned, now: Duration, renewal: Duration) -> bool {
        let WhatIsPinned::AnUnreadableBoard { id, text } = top else {
            return false;
        };
        self.seen.as_ref().is_some_and(|(seen_id, seen_text)| seen_id == id && seen_text == text)
            && self.unchanged_since.is_some_and(|since| now.saturating_sub(since) >= renewal)
    }
}

/// **B3 — the copied list and roster do not fit under this holder**, so the
/// claimant cannot prove it preserved them and stands down.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TheListCannotBePreserved;

/// Who is taking the slot, and the heartbeat it starts with.
#[derive(Debug, Clone)]
pub struct TheClaimant<'a> {
    pub machine: &'a str,
    pub display: &'a str,
    pub colour: Option<&'a str>,
    pub heartbeat: &'a str,
    pub moment: &'a str,
    pub renewal_seconds: u64,
}

/// **`Telegram` B3 and B14 — the successor board.** The previous board's full
/// instruction list and admitted roster, copied by identity; this machine as
/// holder, listed where it fits; then whatever the popped machinery messages
/// said, taken in exactly as the janitor takes it in.
pub fn the_successor(
    previous: Option<&TheBoard>,
    claimant: &TheClaimant<'_>,
    taken_in: &[TheUtterance],
) -> Result<(TheBoard, Vec<NotAdmitted>), TheListCannotBePreserved> {
    let mut board = TheBoard {
        holder: claimant.machine.to_string(),
        heartbeat: claimant.heartbeat.to_string(),
        heartbeat_moment: claimant.moment.to_string(),
        renewal_seconds: claimant.renewal_seconds,
        route: TheRouteMode::Asking,
        roster: previous.map(|b| b.roster.clone()).unwrap_or_default(),
        instructions: previous.map(|b| b.instructions.clone()).unwrap_or_default(),
    };
    if !board.fits() {
        return Err(TheListCannotBePreserved);
    }
    let mut not_admitted = Vec::new();
    let me = ARosterEntry { machine: claimant.machine.to_string(), display: claimant.display.to_string(), colour: claimant.colour.map(str::to_string) };
    match list(&board, &me) {
        AListing::Listed(with) => board = with,
        AListing::AlreadyListed => {}
        AListing::DoesNotFit => not_admitted.push(NotAdmitted::AMachine { machine: me.machine, display: crate::job_messages::AServerLabel::with_colour(&me.display, me.colour.as_deref()) }),
    }
    for utterance in taken_in {
        let asks = what_an_utterance_asks(&board, utterance, claimant.moment);
        if let Some(candidate) = asks.candidate {
            board = candidate;
        }
        not_admitted.extend(asks.not_admitted);
    }
    Ok((board, not_admitted))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::instructions::tests::an_entry;
    use crate::board::instructions::{AKindOfEntry, TheAddressee};
    use crate::board::text::tests::a_board;
    use serde_json::json;

    fn claimant() -> TheClaimant<'static> {
        TheClaimant { colour: None, machine: "m2", display: "Vast 5060", heartbeat: "fresh", moment: "t", renewal_seconds: 60 }
    }

    /// **B3: the whole list is copied by identity**, entries of every kind.
    #[test]
    fn the_successor_carries_every_accepted_instruction() {
        let mut old = a_board("m1");
        old.instructions.push(an_entry("a", TheAddressee::Machine("m3".into()), AKindOfEntry::Action));
        old.instructions.push(an_entry("r", TheAddressee::RoutingKey("k".into()), AKindOfEntry::ContextResolution));
        let (new, not_admitted) = the_successor(Some(&old), &claimant(), &[]).unwrap();
        assert_eq!(new.instructions, old.instructions);
        assert_eq!(new.holder, "m2");
        assert_eq!(new.heartbeat, "fresh");
        assert!(new.roster.iter().any(|r| r.machine == "m1"));
        assert!(new.roster.iter().any(|r| r.machine == "m2"));
        assert!(not_admitted.is_empty());
    }

    #[test]
    fn what_the_popped_messages_said_is_taken_in() {
        let mut old = a_board("m1");
        old.instructions.push(an_entry("a", TheAddressee::Machine("m3".into()), AKindOfEntry::Action));
        let ack = TheUtterance { machine: "m3".into(), acknowledged: vec!["a".into()], ..Default::default() };
        let (new, _) = the_successor(Some(&old), &claimant(), &[ack]).unwrap();
        assert!(new.instructions.is_empty());
    }

    #[test]
    fn a_list_that_cannot_fit_under_this_holder_is_not_replaced() {
        let mut old = a_board("m1");
        let mut big = an_entry("a", TheAddressee::Machine("m3".into()), AKindOfEntry::Action);
        big.payload = json!("x".repeat(3700));
        old.instructions.push(big);
        let long_name = TheClaimant { machine: &"m".repeat(400), ..claimant() };
        assert_eq!(the_successor(Some(&old), &long_name, &[]), Err(TheListCannotBePreserved));
    }

    #[test]
    fn the_claimant_reads_down_to_a_board_and_never_replaces_an_unreadable_one_that_is_not_stale() {
        let unreadable = WhatIsPinned::AnUnreadableBoard { id: 4, text: "smoozy-board 2".into() };
        assert_eq!(what_the_claimant_does(&WhatIsPinned::Nothing, false), WhatTheClaimantDoes::StartAFreshBoard);
        assert_eq!(what_the_claimant_does(&unreadable, false), WhatTheClaimantDoes::StandDownAndSayTheBoardIsUnreadable);
        assert_eq!(what_the_claimant_does(&unreadable, true), WhatTheClaimantDoes::StartAFreshBoardOverAStaleOne { id: 4 });
        assert_eq!(what_the_claimant_does(&WhatIsPinned::SomethingElse { id: 5 }, false), WhatTheClaimantDoes::Unpin { id: 5 });
    }

    /// **B3's stale-board rule on B5's test**: unchanged text for a whole
    /// interval is stale; changed text starts the count again; a machinery
    /// message on top moves nothing.
    #[test]
    fn an_unreadable_board_is_stale_only_after_a_whole_interval_unchanged() {
        let s = Duration::from_secs;
        let renewal = s(60);
        let v2 = |text: &str| WhatIsPinned::AnUnreadableBoard { id: 4, text: text.into() };
        let mut watch = TheUnreadableBoardWatch::default();
        watch.a_read_completed(s(0), &v2("beat 1"));
        watch.a_read_completed(s(30), &WhatIsPinned::AMachineryMessage { id: 9, utterance: TheUtterance::default() });
        assert!(!watch.is_stale(&v2("beat 1"), s(58), renewal));
        watch.a_read_completed(s(58), &v2("beat 2"));
        assert!(!watch.is_stale(&v2("beat 2"), s(100), renewal), "a live holder's text changed");
        watch.a_read_completed(s(118), &v2("beat 2"));
        assert!(watch.is_stale(&v2("beat 2"), s(118), renewal));
        assert!(!watch.is_stale(&v2("beat 3"), s(118), renewal), "only the text the reads showed");
        watch.a_read_completed(s(120), &WhatIsPinned::Nothing);
        assert!(!watch.is_stale(&v2("beat 2"), s(200), renewal));
    }
}

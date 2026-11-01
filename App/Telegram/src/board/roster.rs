//! **The roster — `Telegram` B14.** A bounded board field kept by the holder
//! from announcements. **A hint, not authority**: nothing is routed by it, and
//! it decides only whom a gather asks.

use serde::{Deserialize, Serialize};

use crate::board::text::TheBoard;

/// **`Telegram` B14 — what an announcement carries: the machine identity and
/// display material only.** No routing key is ever in an announcement or on
/// the roster.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ARosterEntry {
    pub machine: String,
    pub display: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub colour: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AListing {
    Listed(TheBoard),
    AlreadyListed,
    /// **`Telegram` B14: left off, and the holder sends one truthful capacity
    /// message naming it.**
    DoesNotFit,
}

/// **`Telegram` B14 — list a machine only where B2's complete candidate fits.**
/// A listed machine whose display material changed is listed again with it.
pub fn list(board: &TheBoard, entry: &ARosterEntry) -> AListing {
    if board.roster.iter().any(|r| r == entry) {
        return AListing::AlreadyListed;
    }
    let mut candidate = board.clone();
    candidate.roster.retain(|r| r.machine != entry.machine);
    candidate.roster.push(entry.clone());
    if candidate.fits() {
        AListing::Listed(candidate)
    } else {
        AListing::DoesNotFit
    }
}

/// **`Telegram` B14 — every admitted role worker re-announces whenever the
/// board it reads names a holder other than the one it last announced to, or
/// does not list it.**
pub fn must_announce(board: &TheBoard, machine: &str, last_announced_to: Option<&str>) -> bool {
    last_announced_to != Some(board.holder.as_str()) || !board.roster.iter().any(|r| r.machine == machine)
}

/// **`Telegram` B14, 2026-10-31 — a roster machine that does not answer a
/// gather by its deadline is taken off**, at the holder's next board edit; a
/// machine still alive puts itself back by re-announcing ([`must_announce`]).
/// The twenty-sixth notes, note 2, Creator-stated: *"/status … talks about
/// servers that do not exist."* Before, nothing took a destroyed server off.
pub fn without_the_silent(board: &TheBoard, machine: &str) -> TheBoard {
    let mut candidate = board.clone();
    candidate.roster.retain(|r| r.machine != machine);
    candidate
}

/// The display material a machine announced, where the roster lists it.
pub fn the_display_of<'a>(board: &'a TheBoard, machine: &str) -> Option<&'a str> {
    board
        .roster
        .iter()
        .find(|r| r.machine == machine)
        .map(|r| r.display.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::text::tests::a_board;

    fn entry(machine: &str, display: &str) -> ARosterEntry {
        ARosterEntry { machine: machine.into(), display: display.into(), colour: None }
    }

    #[test]
    fn a_machine_is_listed_once_and_only_where_it_fits() {
        let board = a_board("m1");
        let AListing::Listed(with) = list(&board, &entry("m2", "Vast 5060")) else { panic!() };
        assert_eq!(list(&with, &entry("m2", "Vast 5060")), AListing::AlreadyListed);
        assert_eq!(the_display_of(&with, "m2"), Some("Vast 5060"));
        assert_eq!(list(&with, &entry("m3", &"x".repeat(5000))), AListing::DoesNotFit);
    }

    #[test]
    fn a_machine_announces_to_a_new_holder_and_when_it_is_not_listed() {
        let mut board = a_board("m1");
        assert!(must_announce(&board, "m2", Some("m1")));
        board.roster.push(entry("m2", "Vast"));
        assert!(!must_announce(&board, "m2", Some("m1")));
        assert!(must_announce(&board, "m2", Some("m0")));
        assert!(must_announce(&board, "m2", None));
        let without = without_the_silent(&board, "m2");
        assert!(must_announce(&without, "m2", Some("m1")), "a machine taken off announces itself back");
        assert_eq!(without_the_silent(&without, "m2"), without);
    }
}

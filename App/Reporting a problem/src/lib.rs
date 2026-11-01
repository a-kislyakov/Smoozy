//! Reporting a problem.
//!
//! Its plan is `Preparation/Architecture/Reporting a problem.md`.
//!
//! **This module is built in two halves, and step 2 builds the first.** The
//! Plan moved it here rather than leaving the whole document at step 11,
//! because `Desktop app and delivery` §5.3 says outright that *"the diary's
//! own rules — its shape, cap, durability, eviction and what may enter it —
//! are `Reporting a problem` §1, §3 and §4's, implemented here and re-decided
//! nowhere"*, so the rules must exist before the service that implements them.
//!
//! **What is here:** §1 the diary, §2's run identity and run-mark — whose two
//! durable halves are `State, identity and recovery`'s
//! ([`smoozy_state::identities`] and [`smoozy_state::run_mark`]) and are not
//! rebuilt here — §3 what goes into the diary, and §4 the write boundary.
//!
//! **What step 11 adds:** §5's report cut, as pure rules over lines already
//! read back — the three slices, the ceiling and the middle drop in
//! [`report`], and the file's head, its parts' statements, its name and the
//! *report written* act line in [`report_file`]. **The surfaces are not
//! here**: the box, the machine take, the write, the reveal and the crash
//! offer are the desktop's, and the read-back of the diary sits beside the
//! diary service's own writing.
//!
//! **And the writer is not here either.** R4: the diary has one writer, the
//! diary service, and it is a named service of `Desktop app and delivery` §21,
//! landed in that document's §5.3 as `Desktop app and delivery` L14 through
//! `Desktop app and delivery` L16. A module never opens the diary,
//! never writes a line itself and never holds a diary file handle.

// The marker guard, test-only: every clause marker in this crate resolves to
// the clause it means. `markers.rs` states what it holds and what it cannot
// see.
#[cfg(test)]
mod markers;

pub mod diary;
pub mod lines;
pub mod report;
pub mod report_file;
pub mod write_boundary;

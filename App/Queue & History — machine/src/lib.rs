//! Queue & History — the half the rented machine's own program may take.
//!
//! **This crate and `App/Queue & History/` are one module in two folders, held
//! to one plan**, `Preparation/Architecture/Queue & History.md`. This half is
//! what runs on the rented machine, or is a value that half reads: the line and
//! the job record, how an ending is decided and what the entry says about the
//! devices, the readings the render worker's copy loop produces and the reader
//! that produces them, the waiting predicate, the previews' prefix arithmetic,
//! the acts on the machine-side record, and — under [`acting`] — the machine's
//! own stores and acts: the machine-local queue, the per-job note, the command
//! and act records, the held jobs, reconciliation, the machine readings, the
//! words and the previews' queueing.
//!
//! **Why it is a crate of its own.** The program a rented machine runs is the
//! far end of an SSH connection and never opens one, so it must never carry the
//! desktop's SSH client or that client's C cryptography — and
//! `smoozy-queue-history` reaches both, through the transfer contract its
//! desktop acts ride. **Nothing here opens a connection or reaches
//! `smoozy-server-protocol`.** `smoozy-queue-history` depends on this crate and
//! re-exports every module here at the path it has always been reached at, so
//! nothing that names one had to change. Split at step 10c piece 3; the account
//! is at `Audits/Build/Queue & History/Decisions record.md`.
//!
//! **How a clause marker is written here is the rule `smoozy-queue-history`
//! states in its own front matter**, and the guard below holds it: a marker
//! with no document beside it is a clause of this plan, and every other
//! document's marker is written with that document.

// The marker guard, test-only: every clause marker in this crate resolves to
// the clause it means. It is the same guard `smoozy-queue-history` carries,
// because the two crates are held to one plan.
#[cfg(test)]
mod markers;

pub mod acting;
pub mod acts;
pub mod ending;
pub mod line;
pub mod naming;
pub mod plan;
pub mod previews;
pub mod readings;
pub mod waiting;

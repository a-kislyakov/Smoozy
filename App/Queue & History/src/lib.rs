//! Queue & History.
//!
//! The machine-local queue and the line in it; how a job's identity, folder
//! and label are composed and shown; held jobs and the predicate that drops a
//! waiter; every act on the line — a cancel, a reorder, a group moved as one
//! block, a stop, a clear-then-stop whose order is the promise; the two bars
//! and the plan arithmetic behind them; how an ending is decided and what the
//! entry says about the devices; the local job record and what each of its
//! fields survives; one job's two records on two lifetimes; the idle half
//! hour; and deletion, which reaches both copies and never comes back. Its
//! plan is `Preparation/Architecture/Queue & History.md`.
//!
//! **What binds this before anything is decided** (the plan's own section of
//! that name, cited and not restated). `State, identity and recovery` §1.1 and
//! §1.2 are the store inventories and this crate opens no store outside them;
//! its §3 D1 and D2 govern every act on the line — one durable desired state,
//! one writer, and **no desired state ever inferred from an observed
//! reading**; its D4 keeps the desktop from holding a durable copy of an
//! observed machine state, with exactly three exceptions; its D5 keeps every
//! time on the clock it was taken on. **And its §1.3 rule 5 binds every
//! section: nothing reads a file's presence, its name or its size as state.**
//! Every queue state, every ending and every figure here comes from a record
//! Smoozy or the supervisor wrote.
//!
//! **Step 7 of `Preparation/Build/Plan.md` builds this module, and this is the
//! half that decides.** Every reading a judgement here stands on is behind one
//! trait — [`readings::Readings`] — and every observation the render worker
//! makes at its copy loop arrives as a value this crate defines and judges,
//! so the whole of the judgement is testable at the desk with no process, no
//! file opened and no machine reached. The half that acts — the durable queue
//! store, the reader that tees the worker's copy loop, the machine readings,
//! reconciliation, adoption and the previews' production and fetch — lands
//! beside it, and [`arriving_later`] names each piece with the clause it
//! belongs to.
//!
//! **Three lines the plan draws itself, and this crate keeps all three.** R1:
//! there is one reader, it rides the render worker's own copy loop, and it
//! produces readings, never state — so [`readings`] holds shapes and no
//! function here turns a reading into a queue state. C10: the two lists are
//! two presentations of two records with one identity, not two stores of one
//! fact — [`two_records`] composes and stores nothing. And E12a, held by the
//! type: **the device field is filled from the recorded device line and from
//! nothing else, has a third value that is a state rather than a default, and
//! nothing here infers a device from what was asked for** ([`ending`]).
//!
//! **How a clause marker is written in this crate, and it is a rule rather
//! than a habit.** A marker with no document beside it means a clause of this
//! crate's own plan. Every other document's marker is written with that
//! document — at the site, or in the file's own header list of markers that
//! belong elsewhere. **Where a marker is a clause of this crate's own plan
//! *and* of the document meant, the header list is not enough and the
//! document is written out at the site**, because a bare one there does not
//! fail to resolve: it resolves to a real and wrong clause, which nobody
//! reading it can tell from a right one. This plan's own series are `Q`, `Y`,
//! `H`, `L`, `R`, `B`, `G`, `E`, `D`, `N`, `C`, `W`, `A`, `X` and `P`, which
//! collide with most of the workspace, so nearly every other document's
//! marker here is written out at the site.

// The marker guard, test-only: every clause marker in this crate resolves to
// the clause it means. `markers.rs` states what it holds and what it cannot
// see.
#[cfg(test)]
mod markers;

pub mod acting;
pub mod arriving_later;
pub mod bars;
pub mod catch_up;
pub mod deletion;
pub mod diary;
pub mod identity;
pub mod idle;
pub mod record;
pub mod stop_and_clear;
pub mod two_records;
pub mod units;

/// **Each module named here lives whole in `smoozy-queue-history-machine`**,
/// the half of this plan the rented machine's own program may take: the line
/// and the job record, the ending and what the entry says, the readings the
/// copy loop's reader produces, the waiting predicate, the previews' prefix
/// arithmetic and the acts on the machine-side record. It was split out on
/// 2026-09-19 so that program could reach them without this crate's transfer
/// contract and the SSH client beneath it. **Nothing about any of them
/// changed**, and each is re-exported here at the path it has always had.
/// **`naming` was written there rather than moved there** (2026-10-01,
/// Vast-pass fix piece 3): what a job is called, which the desktop and the
/// phone's words both read, so it lives where both programs can reach it.
pub use smoozy_queue_history_machine::{acts, ending, line, naming, previews, readings, waiting};

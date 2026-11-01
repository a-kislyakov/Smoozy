//! The half that acts: the reader that rides the render worker's copy loop,
//! the machine-local queue and the per-job note as durable stores, every
//! command applied to the line, the reconciliation that runs when a machine
//! comes back, the readings taken of a machine, the catch-up pass, adoption,
//! the erase, the previews and the words.
//!
//! # What this is
//!
//! The rest of this crate is judgement — a rule, a shape, arithmetic over a
//! reading somebody else took. **Nothing there opens a file, writes a store,
//! reads a stream or reaches a machine.** This module is where that happens.
//! [`crate::arriving_later`] is the list it answers, act by act with the
//! clause each belongs to.
//!
//! # The seam with `Rendering`, and which side owns what
//!
//! **R1: there is one reader, it rides the render worker's own copy loop,
//! and it produces readings, never state.** The worker already holds both of
//! the render's streams and copies them to the sinks itself; the reader is a
//! tee on that loop, observing bytes as they pass.
//!
//! **So the split is: `Rendering` owns the loop and this crate owns the
//! parse.** [`reader`] is a total function from one line and one version to
//! what that line was; `Rendering`'s copy loop calls it for every line it
//! copies and folds what comes back with [`crate::readings::fold_a_line`].
//! **Nothing here opens a stream, and nothing there knows a grammar** — which
//! is the one seam step 7 turns on, and it runs one way: that crate depends
//! on this one, this one never on that.
//!
//! # What is not here, and each is a clause rather than an omission
//!
//! **The supervisor's own writes are composed here and performed by nothing.**
//! *(Corrected 2026-09-07. This said there was no program for the rented
//! machine anywhere in the workspace and no supervisor. Both arrived at step
//! 8: the program is `App/Rented-server runtime — program/`, which is
//! `smoozy-server`, and the supervisor is inside it.)* **What is still true is
//! the part that matters here**: nothing that program runs calls these stores
//! yet. *(Corrected 2026-09-19: this said the program depended on three crates
//! — `smoozy-runtime`, `smoozy-framing` and `smoozy-state` — and so could not
//! reach these stores. At step 10c piece 3 the stores moved to
//! `smoozy-queue-history-machine` and the program took that crate, so it can
//! reach them; this crate is still not among its dependencies.)* It answers six framed
//! requests and refuses every other by name, and its own
//! `WHAT_A_SUPERVISOR_STILL_OWES` says so on the complaints stream at every
//! start. **Nothing here invents a program**, exactly as
//! `Rented-server runtime`'s own stores already do not.
//!
//! # Which side of the machine each module is on, because two never meet
//!
//! **On the rented machine, under the supervisor's single writer**:
//! [`queue_store`], [`note`], [`commands`], [`held`], [`reconciliation`],
//! [`machine_readings`], [`words`]'s capture half and [`previews`]' queueing
//! half. **On the user's own computer**: [`catching_up`], [`idle_recheck`],
//! [`adoption`], [`erase`], [`submission`], [`previews`]' fetch half and
//! [`words`]' fetch half. [`reader`] is neither — it is a total function over
//! one line, called from `Rendering`'s loop.
//!
//! **A trait exists only where a machine is genuinely on the far side of it**,
//! or where the desktop's own local record is: the local record's answers
//! arrive as values, in the split `App/Keys/` drew and every crate since has
//! kept.
//!
//! # Why four of the stores here are gated to a unix, store by store
//!
//! **Four stores are published through `smoozy_runtime::acting::durable`, and
//! that module is absent on Windows.** `Rented-server runtime` gates its whole
//! `acting` module `#[cfg(unix)]` — every act in it is a unix act and the
//! machine Smoozy rents is never Windows — so on Windows this crate's four
//! stores had nothing to call and the crate did not compile at all. The four
//! are `TheCommandRecordsOnTheMachine` and `TheActRecordsOnTheMachine` in
//! [`commands`], `ThePerJobNotesOnTheMachine` in [`note`], and
//! `TheObservationsOnTheMachine` in [`words`], with the acts that take one of
//! them and the tests that exercise one.
//!
//! **Two clauses of `State, identity and recovery` govern it.** Its §1.3 rule
//! 6 says a rented machine is a Linux machine in as many words — *"On Linux —
//! every rented machine — `fsync` reaches the device"* — and its §1.2, clause
//! `State, identity and recovery` J11, is the inventory of the stores on each
//! rented server, which is where all four sit. So a Windows build has no use
//! for them, and they are absent there rather than answered wrongly.
//!
//! **The gate is on the store rather than on the file it sits in**, because
//! those files also hold shapes the desktop half reads: the note itself is
//! [`adoption`]'s, the command record and the command name travel, and the
//! words' fetch runs on the user's own computer. A module-wide gate would take
//! those away on Windows too. `Rendering` gates one function of its `endings`
//! the same way and for the same reason — the gate goes where the unix-only
//! thing it needs sits.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Rendering` K13.

pub mod adoption;
pub mod catching_up;
pub mod erase;
pub mod idle_recheck;
pub mod previews;
pub mod submission;

/// **The machine's side of this list, and the reader, live whole in
/// `smoozy-queue-history-machine`** — the half of this plan the rented
/// machine's own program may take, split out on 2026-09-19 so that program
/// can reach them without this crate's transfer contract and the SSH client
/// beneath it. Each is re-exported here at the path it has always had, so
/// nothing that names one had to change. [`previews`] is split: its queueing
/// half went with these and is re-exported from inside it.
pub use smoozy_queue_history_machine::acting::{
    commands, held, machine_readings, note, queue_store, reader, reconciliation, words,
};

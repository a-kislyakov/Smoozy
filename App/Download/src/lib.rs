//! Download.
//!
//! How files come back from a rented server to the user's own computer: the
//! manual request and the auto-download arrangement; the record of what an
//! arrangement has brought and what that record survives; the watch on the
//! job's output folder and the two mechanisms that tell a finished file from
//! one still being written; format narrowing against a list that arrives late;
//! the count, the whole, and what ends an arrangement whose count may never be
//! reached; the destination's four causes of trouble and the cadence that
//! never gives up; the download-direction ceiling and the two download
//! classes; and where downloads land on three operating systems. Its plan is
//! `Preparation/Architecture/Download.md`.
//!
//! **Step 8 of `Preparation/Build/Plan.md` builds this module, and this is the
//! half that decides.** Every clause of the plan that is a shape, a record, a
//! judgement or a derivation has its code in the files beside this one, and
//! every one of them is driven in a test by values written by hand. **Nothing
//! here opens a file, runs a process, reaches a machine or moves a byte.** The
//! half that acts — the walk performed over the file-transfer subsystem, the
//! fetch carried out as a transfer, the durable store the arrangement and the
//! request records live in, the desktop-initiated reads of the job's record
//! directory, the probe's own local write, the enumeration of unowned staging,
//! and the folder-reveal call — lands beside it, and [`arriving_later`] names
//! each piece with the clause it belongs to.
//!
//! # What binds this before anything is decided
//!
//! The plan's own section of that name, cited and not restated. Six documents
//! bind this one before it decides anything, and **not one of their answers is
//! taken a second time here**:
//!
//! - **The transfer is `Server protocol and transfers` §7's**, entire. Staging
//!   inside the destination folder on the same filesystem as the final name; a
//!   staging path synthesised from the transfer identity and never from a
//!   file's name; the desktop-side record durable before the first byte; the
//!   verified checkpointed resume with four values compared first; publication
//!   as one rename; the end-to-end integrity split; and the three endings with
//!   their opposite cleanup rules. **The partial survives a drop and never
//!   sits at the final name**, which is the single answer Rows 208 and 209
//!   need.
//! - **The local collision judgement is `Desktop app and delivery` §7's**, and
//!   this document never takes it a second time: *can this name be written at
//!   all*, before the transfer is arranged; *is this name already taken*, at
//!   publication, as an exclusive create of the final name. The adapter hands
//!   on an outcome and never a prompt, a rename or a decision.
//! - **The count and the inventory are `Job files and outputs` §7's** — one
//!   entry per expected file with the predicted path and the frame, the count
//!   the number of those entries with nothing deriving a count a second way,
//!   the one folder to watch, a video destination's single entry marked *may
//!   grow*, and the added write places carried as folders in no count.
//! - **The pool and the order are `File Browser` §6's** — four transfer
//!   channels per server, one pool serving both directions, five declared
//!   classes in one order, and the share rule with its 2N−1 bound.
//! - **The job events are `Queue & History`'s** — a job's ending is one of its
//!   four and it is carried here rather than decided here.
//! - **The connection is `Servers`'** — a repaired connection is a
//!   replacement, a connect after a disconnect is period-opening, and that
//!   document's standing repair is the whole cadence for server-side trouble.
//!   Nothing here adds one.
//!
//! # The three lines the plan draws itself, and this crate keeps all three
//!
//! **R5: the reckoning answers *what have I taken*, and only the walk answers
//! *what is there*.** No doneness judgement is ever written durably, so
//! [`vouching`] holds functions over readings and [`arrangement`] holds a
//! record with no judgement in it.
//!
//! **R8: the reckoning never asks the already-exists question.** The
//! destination is consulted exactly once per file, by the exclusive create at
//! publication, and no function here compares a reckoning against a
//! destination listing.
//!
//! **§4's size closure, with exactly one exception: a file of zero bytes is
//! never a finished frame.** No other reading of a size decides anything in
//! this crate — no plausibility floor, no minimum-bytes test, no
//! has-it-stopped-growing check — and a test reads the crate's own sources for
//! one.
//!
//! # How a clause marker is written in this crate, and it is a rule rather
//! than a habit
//!
//! A marker with no document beside it means a clause of this crate's own
//! plan. Every other document's marker is written with that document — at the
//! site, or in the file's own header list of markers that belong elsewhere.
//! **Where a marker is a clause of this crate's own plan *and* of the document
//! meant, the header list is not enough and the document is written out at the
//! site**, because a bare one there does not fail to resolve: it resolves to a
//! real and wrong clause, which nobody reading it can tell from a right one.
//! This plan's own series are `R`, `W`, `J`, `P`, `N`, `E`, `M`, `D`, `Q` and
//! `H`, which collide with most of the workspace, so nearly every other
//! document's marker here is written out at the site.

// The marker guard, test-only: every clause marker in this crate resolves to
// the clause it means. `markers.rs` states what it holds and what it cannot
// see.
#[cfg(test)]
mod markers;

pub mod acting;
pub mod arrangement;
pub mod arriving_later;
pub mod classes;
pub mod declarations;
pub mod destination;
pub mod diary;
pub mod ending;
pub mod fetching;
pub mod landing;
pub mod narrowing;
pub mod request;
pub mod vouching;
pub mod watch;

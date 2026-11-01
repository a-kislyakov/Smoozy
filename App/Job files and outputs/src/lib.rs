//! Job files and outputs.
//!
//! Where Smoozy's own directories sit on a rented machine and what every path
//! under them is called; what a job's immutable submission manifest carries and
//! where it is published; and how everything Smoozy itself places on a machine
//! for a job is published as one object. Its plan is
//! `Preparation/Architecture/Job files and outputs.md`.
//!
//! **What binds this before anything is decided**, taken from the plan's own
//! section of that name and not restated as a second set of rules:
//!
//! - **`State, identity and recovery` §1.3 rule 5.** Nothing reads a file's
//!   presence, its name or its size as state. The one carve-out is the re-run's
//!   already-there test, which is §10's and is built at [`reckoning`] —
//!   [`reckoning::the_frame_is_already_there`] and the completeness statement
//!   beside it, both inside the carve-out's own limit: a rule about what to
//!   render, never a reading of state. *(Corrected 2026-09-07. This said the
//!   test was **not built here**, which was true when this file was written at
//!   step 4 and stopped being true the next day, when §10 landed.
//!   [`arriving_later`] on this crate has said the list is empty since; this
//!   line had not caught up.)*
//! - **Publication is not this document's to design.** `Server protocol and
//!   transfers` §7 owns the staging-and-publication contract in both directions
//!   for the machine's whole life, and it is taken entire. What is settled here
//!   is where a job's material is published *to*, and under what names.
//! - **No directory's presence is ever read as evidence that Smoozy's
//!   installation is there.** A container rental's recycle destroys everything
//!   Smoozy installed and returns on the same address and port. The
//!   installation identity of `State, identity and recovery` §2.3 is the only
//!   thing that tells a rental from an installation.
//! - **The job identity's uniqueness scope is given rather than chosen** (§2.1
//!   rule 7), and **no path is ever synthesised from an identity by anything
//!   but the one document that owns that path**. This module is that owner for
//!   every path below.
//!
//! **Step 4 of `Preparation/Build/Plan.md` builds part of this document and
//! says which part**: *"`Job files and outputs`' recorded reading, its
//! expected-output inventory, its previews and its reckoning all need a Blender
//! on a machine and land with steps 6 to 8; what comes here is the directory
//! layout, the manifest and the publication contract."* [`arriving_later`]
//! names each unbuilt part with the step it lands on, so nothing is silently
//! absent.
//!
//! **The split is the one steps 2, 3 and 4 have all used.** The half that
//! decides — the root's choice, the layout, the names and the byte budget, the
//! manifest's shape, the abandoned-material derivation — takes every reading as
//! a value and is testable at the desk. The half that acts sits behind
//! [`acting::TheMachinesDirectories`], whose real end over the file-transfer
//! subsystem is in [`wiring`].
//!
//! **How a clause marker is written in this crate, and it is a rule rather than
//! a habit.** A marker with no document beside it means a clause of this
//! crate's own plan. Every other document's marker is written with that
//! document — at the site, or in the file's own header list of markers that
//! belong elsewhere. **Where a marker is a clause of this crate's own plan
//! *and* of the document meant, the header list is not enough and the document
//! is written out at the site**, because a bare one there does not fail to
//! resolve: it resolves to a real and wrong clause, which nobody reading it
//! can tell from a right one.
//!
//! **A section number is a marker for the whole of that rule.** `§1.3` and
//! `§9.2` are defined in as many documents as `M1` is, and this crate's plan has
//! a real `§1.3` and a real `§9.2` of its own — so a bare section number means a
//! section of this crate's plan, every other document's section is written
//! with that document, and where the number exists in both the document is
//! written out at the site.

// The marker guard, test-only: every clause marker in this crate resolves to
// the clause it means. `markers.rs` states what it holds and what it cannot
// see.
#[cfg(test)]
mod markers;

pub mod acting;

/// **The values, at the paths they have always been reached at.** Every module
/// below lives whole in `smoozy-job-files-machine`, the half of this plan the
/// rented machine's own program may take, and nothing about any of them
/// changed with the move. Split at step 10c piece 3; [`publication`] keeps
/// the acts beside the one type that moved out of it.
///
/// **`composing` and `reading_script` joined the list at step 10c's
/// ninth-pass fix piece 1**, because the desktop reads back the recorded
/// reading the render worker writes at render start, and it reads it as the
/// machine's own shape rather than a second one (`Download` N2).
pub use smoozy_job_files_machine::{
    attempts, composing, holds, inventory, layout, main_output, manifest, mapping, names, reading,
    reading_script, reckoning, redirect, root_choice, units, where_the_folder_sits,
};
pub mod arriving_later;
pub mod cleanup;
pub mod diary;
pub mod placement;
pub mod publication;
pub mod wiring;

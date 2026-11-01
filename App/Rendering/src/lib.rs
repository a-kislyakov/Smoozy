//! Rendering.
//!
//! The Blender launch contract — the route that reports, the one
//! command-line window and the order of the line, the per-version
//! composition, the launch material's hand-over, the version confirmation
//! and the detachment; the render kind's declarations, its termination
//! contract and its ending classification; atomic job submission and the
//! reading taken after a dropped one; the first-frame hold's two halves; the
//! missing-assets warning's three cases; submitting again and carrying on;
//! and what *which frames were rendered* is composed from. Its plan is
//! `Preparation/Architecture/Rendering.md`.
//!
//! **What binds this before anything is decided** (the plan's own section of
//! that name, cited and not restated). `Job files and outputs` §5 hands this
//! document the launch contract by name and composes everything that goes
//! into it — the material is that document's, how it is handed to Blender
//! is this contract's (`Job files and outputs` D6); `Job files and outputs`
//! D11 states the window and `Job files and outputs` D17 and `Job files and
//! outputs` D18 forbid resting anything on a launch script failing loudly. `Rented-
//! server runtime` W1 names this document the owner of the render kind's
//! declarations and its termination contract, and `Rented-server runtime`
//! X13 says **nothing
//! matches an ending on its number**. `Queue & History` §17 is the longest
//! hand-over in the stage and this crate's half of it is a citation: that
//! crate owns the `Ending` type and the mapping, and this crate supplies the
//! classification value into it. `Dependency Upload & Remap` §11 hands over
//! the remap material's application rules and the must-not-render
//! constraint. `State, identity and recovery` binds every store and every
//! reading, and its §9 is what [`units`] declares against.
//!
//! **Step 7 of `Preparation/Build/Plan.md` builds this module, and this is
//! the half that decides.** Every judgement, data shape and composition of
//! the plan is here, and every one of them is testable at the desk with no
//! process started, no file opened and no machine reached. The half that
//! acts — the render worker's pipeline, the launch itself, the copy loop,
//! the stop's seven steps performed, the readings of a machine — is
//! [`acting`], and [`arriving_later`] names each piece with the clause it
//! belongs to. **[`acting`] is also where this document's seam is designed
//! as well as filled**, this plan declaring none of its own; that module's
//! own front matter says what shape it took and why.
//!
//! **This is the plan the Build plan calls the place where a wrong answer is
//! silent, and four things here are held by construction rather than by
//! care.** The launch line's order is the type's: an argument has exactly one
//! position, the line is rendered by walking the positions in the contract's
//! order, and a wrongly ordered line cannot be composed at all ([`line`](mod@line)).
//! The device argument's spelling is declared at one point, per version, and
//! is never composed from a manifest value, a payload value or anything a
//! user can reach ([`spellings`]); nothing anywhere infers a device from what
//! was asked for ([`technology`]). The skip-existing-frames switch is imposed
//! off on every attempt by the material's own composition and the payload
//! cannot carry it ([`material`]). And the frame list is composed from the
//! reckoning's own integers, each once, in the asked order ([`frames`]).
//!
//! **How a clause marker is written in this crate, and it is a rule rather
//! than a habit.** A marker with no document beside it means a clause of this
//! crate's own plan. Every other document's marker is written with that
//! document — at the site, or in the file's own header list of markers that
//! belong elsewhere. **Where a marker is a clause of this crate's own plan
//! *and* of the document meant, the header list is not enough and the
//! document is written out at the site**, because a bare one there does not
//! fail to resolve: it resolves to a real and wrong clause, which nobody
//! reading it can tell from a right one. This plan's own series are `L`,
//! `P`, `K`, `S`, `W` and `A`, which collide with most of the workspace, so
//! nearly every other document's marker here is written out at the site.

// The marker guard, test-only: every clause marker in this crate resolves to
// the clause it means. `markers.rs` states what it holds and what it cannot
// see.
#[cfg(test)]
mod markers;

pub mod acting;
pub mod after_a_drop;
pub mod again;
pub mod arriving_later;
pub mod diary;
pub mod kind;
pub mod submission;
pub mod units;
pub mod warning;
pub mod which_frames;

/// **Each module named here lives whole in `smoozy-rendering-machine`**, the
/// half of this plan the rented machine's own program may take: the launch
/// line and the device argument's spelling, the four versions, the frame list,
/// the launch material, the nine-step pipeline and the attempt's composition,
/// the card technology, the stop's figures, and the ending classification. It
/// was split out on 2026-09-19 so that program could reach the render worker
/// without this crate's transfer contract and the SSH client beneath it.
/// **Nothing about any of them changed**, and each is re-exported here at the
/// path it has always had.
pub use smoozy_rendering_machine::{
    composition, endings, frames, line, material, pipeline, spellings, stop, technology, versions,
};

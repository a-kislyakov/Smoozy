//! Render Settings.
//!
//! The settings half of the one recorded reading of a `.blend` and the endings
//! a read can have; the override store that keeps the user's answers apart
//! from the file's values; frame-list tidying; the view-layer pin; the
//! samples-imposition set; the engine identifier's provenance; the device
//! choice's two sides; the version pick and the mismatch warning's material;
//! and the submission payload as a statement of differences. Its plan is
//! `Preparation/Architecture/Render Settings.md`.
//!
//! **What binds this before anything is decided** (the plan's own section of
//! that name, cited and not restated). `Job files and outputs` §4 owns the
//! recorded reading and this document adds its settings half — one invocation,
//! one sentinel-framed document, never a second read (Q1). `Rendering` §8
//! states the payload's required shape and §9 here is written against it
//! clause by clause. `State, identity and recovery` §1.1's volatile class is
//! where everything this document holds on the desktop lives (J2), and its D2
//! is J4's whole discipline: **no desired state is ever inferred from an
//! observed reading.** `Servers` §15 is Row 158's whole supply; `Server Setup`
//! §12 is Row 155's.
//!
//! **Step 7 of `Preparation/Build/Plan.md` builds this module, and it is built
//! in two halves.** The half that decides is every module but [`acting`]:
//! every reading a judgement stands on is behind one trait —
//! [`readings::Readings`] — and every reading of the `.blend` itself arrives
//! as the raw frame [`endings`] judges, so the whole of the judgement is
//! testable at the desk with no process, no file opened and no machine
//! reached. **The half that acts is [`acting`], and it is the only place in
//! this crate that reaches a machine**: the pre-load driven through the
//! supervisor's command path, the noticing's four-value comparison read, the
//! version-list ask, and the held hardware reading. **What is still not this
//! crate's is named clause by clause at [`arriving_later`]** — the invocation
//! itself, which is `Job files and outputs` R1's; the pin pairs, the device
//! argument and the version confirmation, which are `Rendering`'s; and the
//! drop at a publication, which is the publishing module's.
//!
//! **How a clause marker is written in this crate, and it is a rule rather
//! than a habit.** A marker with no document beside it means a clause of this
//! crate's own plan. Every other document's marker is written with that
//! document — at the site, or in the file's own header list of markers that
//! belong elsewhere. **Where a marker is a clause of this crate's own plan
//! *and* of the document meant, the header list is not enough and the
//! document is written out at the site**, because a bare one there does not
//! fail to resolve: it resolves to a real and wrong clause, which nobody
//! reading it can tell from a right one. This plan's own series are `Q`, `J`,
//! `F`, `V`, `C`, `E`, `B`, `D`, `Y` and `W`, so `Rented-server runtime` W1,
//! `Server Setup` B1, `Queue & History` B1, `State, identity and recovery` D5
//! and `Reporting a problem` J2 are the ones most often written out.

// The marker guard, test-only: every clause marker in this crate resolves to
// the clause it means. `markers.rs` states what it holds and what it cannot
// see.
#[cfg(test)]
mod markers;

pub mod acting;

/// **The values, at the paths they have always been reached at.** Every module
/// below lives whole in `smoozy-render-settings-machine`, the half of this plan
/// the rented machine's own program may take, and nothing about any of them
/// changed with the move. Split at step 10c piece 3; [`readings`] keeps the
/// trait beside the three values that moved out of it.
pub use smoozy_render_settings_machine::{
    device, engine, file_half, frames, names, payload, reading, refusals, samples, spellings,
    store, view_layers,
};
pub mod arriving_later;
pub mod declarations;
pub mod diary;
pub mod endings;
pub mod readings;
pub mod reporting;
pub mod units;
pub mod version;

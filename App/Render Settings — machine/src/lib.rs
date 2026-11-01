//! Render Settings — the half the rented machine's own program may take.
//!
//! **This crate and `smoozy-render-settings` are one module in two folders**,
//! and one plan governs both: `Preparation/Architecture/Render Settings.md`.
//! What is here moved on 2026-09-19, at step 10c piece 3: the settings half of
//! the recorded reading ([`reading`]) and the names in it ([`names`]), the
//! samples imposition set ([`samples`]) and the spellings it assigns against
//! ([`spellings`]), the view-layer pin ([`view_layers`]), the engine
//! identifier's provenance ([`engine`]), the device choice and its warning
//! ([`device`]), frame lists ([`frames`]), the override store ([`store`]), the
//! refusals ([`refusals`]), the file's half ([`file_half`]) with the values two
//! readings arrive in ([`readings`]), and the submission payload
//! ([`payload`]). **What is next door is every act and every desktop
//! judgement over a machine's answer**: the pre-load, the four-value read, the
//! version-list ask and the held hardware reading's source, the readings'
//! trait, the version pick, the endings a read can have, and the diary.
//!
//! **Why the line is here.** The program a rented machine runs must not reach
//! `smoozy-server-protocol`, which carries the SSH client and its C
//! cryptography, and `smoozy-render-settings` reaches it. The render worker
//! composes its launch material from the payload, the reading, the pin and the
//! samples set, and the queue reads the device choice; all of it is values and
//! judgements over values.
//!
//! **Nothing that reads these had to move with them.** `smoozy-render-settings`
//! re-exports every module here from the path it has always been reached at;
//! its `readings` keeps the trait beside the three values that moved out of it.
//!
//! **How a clause marker is written in this crate** is the rule next door,
//! unchanged: a marker with no document beside it means a clause of this
//! crate's own plan, and every other document's marker is written with that
//! document, at the site or in the file's own header list. **Where a marker is
//! a clause of this crate's own plan *and* of the document meant, the document
//! is written out at the site.**

// The marker guard, test-only: every clause marker in this crate resolves to
// the clause it means. It is the same guard `smoozy-render-settings` carries,
// because the two crates are held to one plan.
#[cfg(test)]
mod markers;

pub mod device;
pub mod engine;
pub mod file_half;
pub mod frames;
pub mod names;
pub mod payload;
pub mod reading;
pub mod readings;
pub mod refusals;
pub mod samples;
pub mod spellings;
pub mod store;
pub mod view_layers;

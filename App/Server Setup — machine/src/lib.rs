//! Server Setup — the half the rented machine's own program may take.
//!
//! **This crate and `smoozy-server-setup` are one module in two folders**, and
//! one plan governs both: `Preparation/Architecture/Server Setup.md`. What is
//! here moved on 2026-09-19, at step 10c piece 3: the Blender lifecycle's
//! judgements and the version a build answers ([`blender`]), the add-on
//! read-backs ([`addons`]), the setup run and its record ([`run`]), recovery
//! and re-running ([`recovery`]), who performs each step and what a stop
//! leaves ([`driving`]), and the ready action's steps with the libraries
//! step's mapping ([`ready_action`]). **What is next door is every act and
//! every desktop reading**: the machine's kind and family, the search, the
//! elevated steps, the hand-over, the placement, the listing and its fetch.
//!
//! **Why the line is here.** The program a rented machine runs must not reach
//! `smoozy-server-protocol`, which carries the SSH client and its C
//! cryptography, and `smoozy-server-setup` reaches it. The setup worker
//! performs these steps and writes this record on the machine, and the render
//! worker reads a Blender's version the way [`blender`] says; all of it is
//! values and judgements over values.
//!
//! **The setup worker's own steps are written here from step 10c piece 8**, at
//! [`acting`]: `Server Setup` Y7's machine-side list — the fetch on the
//! machine's own line, the unpack into staging, the two `ldd` reads, the
//! publication by rename, and the final verification. **Two things the machine
//! has are seams whose one real end is the program**: a request on the
//! machine's own line, and a process run. **The disk is not a seam** — staging
//! and renaming behave the same on both systems, and the tests use a real
//! folder.
//!
//! **Nothing that reads these had to move with them.** `smoozy-server-setup`
//! re-exports every module here from the path it has always been reached at;
//! `blender` and `ready_action` there keep the one or two items that could not
//! come — the Blender area's path, and the composition of a run from the
//! machine's kind — beside the re-export.
//!
//! **How a clause marker is written in this crate** is the rule next door,
//! unchanged: a marker with no document beside it means a clause of this
//! crate's own plan, and every other document's marker is written with that
//! document, at the site or in the file's own header list.

// The marker guard, test-only: every clause marker in this crate resolves to
// the clause it means. It is the same guard the framing crate carries, because
// this crate and `smoozy-server-setup` are held to one plan.
#[cfg(test)]
mod markers;

pub mod acting;
pub mod addons;
pub mod blender;
pub mod driving;
pub mod ready_action;
pub mod recovery;
pub mod run;
pub mod the_optix_library;

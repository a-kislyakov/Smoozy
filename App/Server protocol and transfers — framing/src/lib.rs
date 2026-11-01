//! Server protocol and transfers — the half both ends speak.
//!
//! **This crate and `smoozy-server-protocol` are one module in two folders**,
//! and one plan governs both: `Preparation/Architecture/Server protocol and transfers.md`.
//! What is here is §2's request framing and versioning ([`framing`]); the
//! shapes a result carries — the free-space reading ([`free_space`]) and what
//! an operation on the file-transfer subsystem answered
//! ([`subsystem_answers`]); **§4.3's acknowledgement and the three closed
//! answers about an operation identity ([`acknowledgement`])**; and **§10's
//! reporting floor ([`reporting`])**. The last two landed on 2026-09-06 with
//! the machine's own durable operation record, and both are here for the same
//! reason everything else is: **the machine speaks them and the desktop speaks
//! them, and neither opens a connection to do it.** **Two values joined them on
//! 2026-09-19, at step 10c piece 3**: the four values a source file is read by
//! ([`source_reading`]) and the one command the command channel carries
//! ([`command_string`]). The machine's own crates name both, and naming them
//! must not take those crates into the connecting half. **What is next door is every act of connecting**:
//! the SSH transport and the command channel, the file-transfer subsystem, the
//! planted-link carriage, the free-space reading's own two routes, and the
//! whole staging, integrity, resume and publication contract.
//!
//! # Where the line is, and why it is there
//!
//! **The plan draws it in its own words.** §1.1 says the pre-package surface
//! *"is made of exactly two mechanisms"* — the command channel and the
//! file-transfer subsystem — and both are things Smoozy **opens** against a
//! machine. §2 is a different thing: *"One adapter invocation frames exactly
//! one request and returns exactly one correlated result"* (P1), over the
//! machine-local endpoint, on the machine itself. **The first is a connection.
//! The second is a vocabulary**, and P1's own 2026-09-04 addition says so from
//! the other side: the framing unit is one connection to the endpoint carrying
//! one request, *"and an adapter invocation is one caller of it rather than the
//! only one"* — the other two callers named there are the health exchange and a
//! worker's report, both of which run **on the machine**, where nothing opens
//! an SSH connection at all.
//!
//! **What the line buys, and it is not tidiness.** `smoozy-server` is the
//! program the rented machine runs. It is the far end of an SSH connection and
//! never opens one. While the module was one crate, that program's dependency
//! tree carried the desktop's SSH client and that client's C cryptography, and
//! it cost two real things: weight in the artifact the desktop uploads to every
//! rented machine — whose byte length and digest the signed release manifest
//! carries, `Server protocol and transfers` B8 and B10 — and the ability to
//! build for the machine's own target at all, the C library's build script
//! wanting a cross toolchain the desk has not got.
//!
//! **The rule this crate is held to.** Nothing here opens a connection, reads a
//! disk, starts a process or waits on time. A dependency that would bring any
//! of those belongs next door.
//!
//! **Nothing that reads these types had to move with them.**
//! `smoozy-server-protocol` re-exports every one of them from the path it has
//! always been reached at, so `framing`, `free_space`, `subsystem::StatusCode`,
//! `subsystem::SubsystemFailure`, `transfers::SourceReading` and
//! `transport::CommandString` all resolve exactly as before.
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

// The marker guard, test-only: every clause marker in this crate resolves to
// the clause it means. `markers.rs` states what it holds and what it cannot
// see. It is the same guard `smoozy-server-protocol` carries, because the two
// crates are held to one plan and a marker that moved between them must go on
// resolving.
#[cfg(test)]
mod markers;

pub mod acknowledgement;
pub mod command_string;
pub mod framing;
pub mod free_space;
pub mod reporting;
pub mod requests;
pub mod source_reading;
pub mod subsystem_answers;

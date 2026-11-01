//! Security and licence.
//!
//! The rules for what Smoozy holds, how each is kept, reached and erased, and
//! what may never be written down. Its plan is
//! `Preparation/Architecture/Security and licence.md`.
//!
//! **What is here and what is not.** This module owns the rules; the adapters
//! that implement them on each of the three systems are `Desktop app and
//! delivery` §9's, which §1.4 B30 says in its own words — *"`Desktop app and
//! delivery` owns the adapter, the helper and the module API boundaries; this
//! document owns the rule and does not choose their implementation."*
//!
//! Step 2 of `Preparation/Build/Plan.md` builds the parts with nothing remote
//! in them: the six held things, the store contract on each system, the route
//! a secret takes to what consumes it, erasure, and the redaction rules. What
//! is deliberately not built here, and where it lands. **Corrected 2026-09-07:
//! this read *not built here yet* and gave each a step to come. Steps 3, 4, 6
//! and 9 have all closed, and every one of these but the last is now built
//! where it was sent** — each of those crates names this document's section by
//! marker in its own code.
//!
//! - **§6, the private half of an SSH key** — `Keys`, step 3. **Built there.**
//! - **§7, host trust** — `Servers`, step 4. **Built there.**
//! - **§8, the supervisor's machine-local endpoint** — `Rented-server
//!   runtime`, step 6. **Built there.**
//! - **§10, what Smoozy leaves on a rented machine** — `Server Setup` and
//!   `Telegram`, steps 6 and 10. **`Server Setup`'s half is built there.
//!   `Telegram`'s desktop half is built at step 10 piece 5** (2026-09-13): the
//!   token file composed at `App/Telegram/src/material.rs` and handed to
//!   `Server Setup`'s placement from `App/Desktop app and delivery/src/record/
//!   told_record.rs`, which also gives [`erasure`]'s bot-token event its first
//!   caller. **What reads the file on the machine, and never hands it to a
//!   child, is step 8b's**, and nothing yet opens the connection that carries
//!   the placement.
//!
//! §1's crossing lists are statements about what may cross rather than code;
//! each is kept by the document that owns the message, and the two that carry
//! anything at step 2 are §1.4's three hops ([`plaintext`]) and §13's bar on
//! what may be written at all ([`redaction`]).

// The marker guard, test-only: every clause marker in this crate resolves to
// the clause it means. `markers.rs` states what it holds and what it cannot
// see.
#[cfg(test)]
mod markers;

pub mod erasure;
pub mod held;
pub mod plaintext;
pub mod redaction;
pub mod secret_store;
pub mod standing;

//! Telegram. Its plan is `Preparation/Architecture/Telegram.md`.
//!
//! **The module-list rule. Read it before adding anything to this file.**
//! Written at step 10 piece 1, 2026-09-13, because eight pieces of work write
//! into this one crate, several of them side by side.
//!
//! - **This file takes one line per module, under its own piece's heading, and
//!   nothing else.** No code, no re-exports, no attributes, no shared helpers.
//!   Anything a module needs lives in that module.
//! - **A piece adds its own lines and touches no other piece's.**
//! - **A new module is its own file or folder**, named in section 5 of
//!   `Audits/Build/Step 10/Telegram — what each piece owns and in what order.md`.
//! - **`Cargo.toml` is not edited and `tests/the_crates_shape_holds.rs` is not
//!   weakened.** A piece that needs another dependency reports it.
//! - **Every call to Telegram's service goes through [`transport::TheClient`]**,
//!   and every send a machine makes on its own goes through
//!   [`pacing::TheSendQueue`]. A second client is the fault the one-client
//!   ruling exists to prevent.

// Piece 1 — the one way to Telegram, and the send queue.
pub mod pacing;
pub mod transport;

// Piece 2 — the slot and the board.
pub mod board;
pub mod listener;
pub mod worker_state;

// Piece 3 — routing, naming and the typed requests.
pub mod routing;

// Piece 4 — the walkthrough, the proof and the token.
pub mod proof;
pub mod walkthrough;

// Piece 5 — the told-record, and the replacement token.
pub mod material;
pub mod told;

// Piece 7 — a job's messages, and the four phone actions.
pub mod actions;
pub mod job_messages;

// Piece 6 — what a machine says on its own clock.
pub mod media;
pub mod pictures;
pub mod status;

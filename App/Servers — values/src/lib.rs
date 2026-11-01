//! Servers — the values the rented machine's own program may take.
//!
//! **This crate and `smoozy-servers` are one module in two folders**, and one
//! plan governs both: `Preparation/Architecture/Servers.md`. What is here is
//! the hardware reading's shape and the judgements over it ([`hardware`]) and
//! the keepalive figures ([`keepalive`]), moved whole on 2026-09-19 at step
//! 10c piece 3. **What is next door is every act**: the period, the repair,
//! the readings taken over a connection, the store, and the wiring.
//!
//! **Why the line is here.** The program a rented machine runs must not reach
//! `smoozy-server-protocol`, which carries the SSH client and its C
//! cryptography, and `smoozy-servers` reaches it. The render worker needs a
//! machine's cards and the setup worker needs the keepalive interval, and both
//! are values that read nothing and connect to nothing.
//!
//! **Nothing that reads these had to move with them.** `smoozy-servers`
//! re-exports both modules from the paths they have always been reached at, so
//! `smoozy_servers::hardware` and `smoozy_servers::keepalive` resolve exactly
//! as before.
//!
//! **How a clause marker is written in this crate** is the rule next door,
//! unchanged: a marker with no document beside it means a clause of this
//! crate's own plan, and every other document's marker is written with that
//! document, at the site or in the file's own header list.

// The marker guard, test-only: every clause marker in this crate resolves to
// the clause it means. It is the same guard `smoozy-servers` carries, because
// the two crates are held to one plan.
#[cfg(test)]
mod markers;

pub mod hardware;
pub mod keepalive;

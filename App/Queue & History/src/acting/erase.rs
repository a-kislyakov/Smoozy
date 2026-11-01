//! **§14 X2 and X3 — the owed machine-side erase.**
//!
//! **The whole of it lives in `smoozy-queue-history-machine`** since the 38's
//! piece 16 (2026-10-08): the machine's own program runs the units and the
//! desktop writes what they came to, so both ends read the one enumeration.
//! It is re-exported here at the path it always had, so nothing that names it
//! had to change. The account is at `Audits/Build/Step 10c/The 38, piece 16 —
//! what landed.md`.

pub use smoozy_queue_history_machine::acting::erase::*;

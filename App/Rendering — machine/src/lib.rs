//! Rendering — the half the rented machine's own program may take.
//!
//! **This crate and `App/Rendering/` are one module in two folders, held to one
//! plan**, `Preparation/Architecture/Rendering.md`. This half is the render
//! worker and every value it composes or judges: the launch line and the device
//! argument's spelling, the four versions, the frame list, the launch material,
//! the nine-step pipeline and the attempt's composition, the card technology,
//! the stop's figures and the ending classification — and, under [`acting`],
//! the worker's own acts on the machine: the render started and held, its two
//! streams copied and observed, the material placed and read back, and the stop
//! performed.
//!
//! **Why it is a crate of its own.** The program a rented machine runs is the
//! far end of an SSH connection and never opens one, so it must never carry the
//! desktop's SSH client or that client's C cryptography — and
//! `smoozy-rendering` reaches both, through the desk side's readings and the
//! submission's placement. **Nothing here opens a connection or reaches
//! `smoozy-server-protocol`.** `smoozy-rendering` depends on this crate and
//! re-exports every module here at the path it has always been reached at, so
//! nothing that names one had to change. Split at step 10c piece 3; the account
//! is at `Audits/Build/Rendering/Decisions record.md`.
//!
//! **How a clause marker is written here is the rule `smoozy-rendering` states
//! in its own front matter**, and the guard below holds it: a marker with no
//! document beside it is a clause of this plan, and every other document's
//! marker is written with that document.

// The marker guard, test-only: every clause marker in this crate resolves to
// the clause it means. It is the same guard `smoozy-rendering` carries,
// because the two crates are held to one plan.
#[cfg(test)]
mod markers;

pub mod acting;
pub mod composition;
pub mod endings;
pub mod frames;
pub mod line;
pub mod load_time;
pub mod material;
pub mod pipeline;
pub mod spellings;
pub mod stop;
pub mod technology;
pub mod versions;

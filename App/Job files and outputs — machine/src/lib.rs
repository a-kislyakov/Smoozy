//! Job files and outputs — the half the rented machine's own program may take.
//!
//! **This crate and `smoozy-job-files` are one module in two folders**, and one
//! plan governs both: `Preparation/Architecture/Job files and outputs.md`. What
//! is here moved on 2026-09-19, at step 10c piece 3: the layout and the names
//! under it, the submission manifest and the attempt record, the recorded
//! reading, the map, the inventory, the main output's walk, the redirect, the
//! reckoning, the root choice, the holds and the units, and the three paths a
//! job's publication uses. **What is next door is every act on the
//! file-transfer subsystem**: the placement, the attempt record's write, the
//! sweep of abandoned material, and the wiring over the connection.
//!
//! **Why the line is here.** The program a rented machine runs must not reach
//! `smoozy-server-protocol`, which carries the SSH client and its C
//! cryptography, and `smoozy-job-files` reaches it. The render worker, the
//! queue and the reckoning read the manifest, the layout and the attempt
//! record on the machine, and all of it is values.
//!
//! **Why the acts could not come too.** The seam over the file-transfer
//! subsystem is implemented next door for that crate's own connection type, and
//! the language allows an implementation only in a crate that defines the seam
//! or the type. Moving the seam here would leave that implementation defining
//! neither, so the seam, its implementation and every act over it stay
//! together.
//!
//! **Nothing that reads these had to move with them.** `smoozy-job-files`
//! re-exports every module here from the path it has always been reached at;
//! its `publication` keeps the acts beside the re-exported paths.
//!
//! **How a clause marker is written in this crate** is the rule next door,
//! unchanged: a marker with no document beside it means a clause of this
//! crate's own plan, a section number likewise, and every other document's
//! marker is written with that document, at the site or in the file's own
//! header list.

// The marker guard, test-only: every clause marker in this crate resolves to
// the clause it means. It is the same guard `smoozy-job-files` carries,
// because the two crates are held to one plan.
#[cfg(test)]
mod markers;

pub mod attempts;
pub mod composing;
pub mod derivatives;
pub mod holds;
pub mod inventory;
pub mod layout;
pub mod main_output;
pub mod manifest;
pub mod mapping;
pub mod names;
pub mod publication;
pub mod reading;
pub mod reading_script;
pub mod reckoning;
pub mod redirect;
pub mod root_choice;
pub mod units;
pub mod where_the_folder_sits;

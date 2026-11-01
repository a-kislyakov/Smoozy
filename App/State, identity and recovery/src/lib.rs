//! State, identity and recovery.
//!
//! Every durable store, what issues each identity and what it may never be
//! made of, desired state against observed state, and what survives what. Its
//! plan is `Preparation/Architecture/State, identity and recovery.md`.
//!
//! **Step 2 builds the half with nothing remote in it**, which is the whole of
//! the desktop side. What is not here, and where it lands:
//!
//! - **§1.2, the stores on each rented server** — `Rented-server runtime`,
//!   step 6. **Two of them are this document's own content and their form
//!   landed here on 2026-09-06**: the operation records and the command
//!   records, at [`machine_operations`]. The store that writes them is still
//!   the supervisor's, that crate's, and §1.3 rule 1 is why.
//! - **§4, the event journal and acknowledgement** — **its rules and its form
//!   landed here on 2026-09-06**, at [`journal`]: the event, the per-machine
//!   sequence, the cursor and acknowledgement rules, the gap a reader detects by
//!   itself, and E6a's cap and per-entry budget. **The store that appends and
//!   reads it is the supervisor's, `Rented-server runtime`'s**, and §1.3 rule 1
//!   is why. The desktop's own event cursor is a row of the local record and is
//!   subject-held on the server row.
//! - **§5, the operation lifecycle and the four outcomes** — **§5.1's form is
//!   [`machine_operations`]**; §5.2's pairing and §5.3's stop sequence are
//!   performed by `Rented-server runtime`, and §6's reconciliation over these
//!   records is that crate's too and is not built.
//! - **§6, reconciliation on the machine** — step 6. **§7, reconciliation on
//!   the desktop** — step 4, when there is a connection to reconcile over.
//! - **§8, failure records** — the plumbing that presents them is `Desktop app
//!   and delivery` §16's, and the conditions that raise them belong to the
//!   modules that raise them.
//! - **§9, the atomic-or-partial boundary** — declared per action by the module
//!   that owns the action (`Desktop app and delivery` A7), so it lands with
//!   each of them.
//!
//! W1 binds all of it before anything is decided: **Fact 11 is negative on all
//! three operating systems**, so no identity here is derived from a reading of
//! the computer; and **no lifetime here is measured in elapsed time on the
//! user's computer, with the single exception the Creator ruled on 2026-08-22**
//! — a job's fetched derivatives, and nothing else.

// The marker guard, test-only: every clause marker in this crate resolves to
// the clause it means. `markers.rs` states what it holds and what it cannot
// see.
#[cfg(test)]
mod markers;

pub mod identities;
pub mod journal;
pub mod lifetimes;
pub mod machine_operations;
pub mod observed;
pub mod outcomes;
pub mod run_mark;
pub mod store_version;
pub mod stores;
pub mod tombstones;

//! Keys.
//!
//! How SSH keys are discovered, generated, validated, fingerprinted, copied
//! and deleted on the user's computer, and how their passphrases are kept,
//! asked for and genuinely forgotten. Its plan is
//! `Preparation/Architecture/Keys.md`.
//!
//! **What binds this before anything is decided** (§1). The private half is
//! `Security and licence` §6's: Smoozy holds **a path and a fingerprint and
//! never the contents** — it never copies the file, never moves it, never
//! sends it anywhere, never writes it to any store or to any other file, and
//! never retains its bytes. The one thing Smoozy reads itself is the container
//! header, in place, transiently, to answer one yes-or-no
//! ([`container`]). **A connection is not this document's act**: `Servers` K1's
//! transport does that, and nothing here opens one.
//!
//! **The store, the route and the erasure are `Security and licence` §3, §4
//! and §5's**, and the key passphrase is one of that document’s three user-secret
//! kinds; the separate probe kind is reserved for the adapter. **The adapters are `Desktop app and
//! delivery`'s**, and no module receives a plaintext: Keys holds a handle
//! naming a kind and a subject, or is handed the outcome of a use carried out
//! on its behalf.
//!
//! **Every key act is available without a paid-access gate.**
//!
//! **Step 3 of `Preparation/Build/Plan.md` builds this module.** What is here
//! is the half that decides: the sets, the rules, the readings' shape and the
//! judgements over them. The half that touches an operating system — the
//! `ssh-keygen` invocations behind [`usable::Readings`], the askpass helper
//! that carries a passphrase to one endpoint, the key-row table and the acts
//! that write it — lands beside it.

// The marker guard, test-only: every clause marker in this crate resolves to
// the clause it means. `markers.rs` states what it holds and what it cannot
// see.
#[cfg(test)]
mod markers;

pub mod container;
pub mod diary;
pub mod fingerprint;
pub mod generation;
pub mod listing;
pub mod locations;
pub mod naming;
pub mod ownership;
pub mod passphrase;
pub mod pointing;
pub mod removal;
pub mod rows;
pub mod types;
pub mod units;
pub mod usable;

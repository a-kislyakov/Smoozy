//! The machine's half that acts: the machine-local queue and the per-job note
//! as durable stores, every command applied to the line, the held jobs'
//! release, the reconciliation that runs when a machine comes back, the
//! readings taken of a machine, the words' capture, the previews' queueing, and
//! the reader that rides the render worker's copy loop.
//!
//! **Each runs on the rented machine, under the supervisor's single writer** —
//! or, for [`reader`], inside the render worker's own copy loop, which is
//! `Rendering`'s: that crate owns the loop and this one owns the parse. The
//! desktop's acts — the catch-up, the idle recheck, adoption, the
//! submission, the previews' fetch and the words' fetch — are in
//! `smoozy-queue-history`'s own `acting`, which re-exports every module here at
//! the path it has always had. `smoozy-queue-history`'s `acting` front matter
//! says which side of the machine each module is on and why four of the stores
//! here are gated to a unix, store by store; both still hold unchanged.
//!
//! [`previews`] is half a file: the queueing half is here and the fetch half
//! stayed, split on 2026-09-19. [`erase`] came whole on 2026-10-08, the 38's
//! piece 16: the machine's program runs its units and the desktop writes what
//! they came to, so both ends read its one enumeration.

pub mod commands;
pub mod erase;
pub mod held;
pub mod machine_readings;
pub mod note;
pub mod previews;
pub mod queue_store;
pub mod reader;
pub mod reconciliation;
pub mod words;

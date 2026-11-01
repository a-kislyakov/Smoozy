//! The half that acts: the walk performed over the file-transfer subsystem,
//! the two desktop-initiated reads of the job's record directory, one file's
//! fetch carried out under the transfer contract, the manual request's
//! enumeration, the ending sequence, the destination probe's cadence, the
//! unowned-staging reading and its removal, and the folder-reveal.
//!
//! # What this is
//!
//! The rest of this crate is judgement — a shape, a record, a rule, arithmetic
//! over a reading somebody else took. **Nothing there opens a channel, reads a
//! machine, writes a store or moves a byte.** This module is where that
//! happens. [`crate::arriving_later`] is the list it answers, act by act with
//! the clause each belongs to.
//!
//! # Which side of the machine each module is on
//!
//! **Read over a connection to the rented machine**: [`walking`],
//! [`record_directory`], [`enumerating`], and the machine half of
//! [`fetching`]. **On the user's own computer**: [`probing`], [`orphans`],
//! [`revealing`], and the local half of [`fetching`]. [`running`] is the
//! sequencer over both and reaches neither directly.
//!
//! **[`wiring`] is the only file in this crate that knows a protocol exists.**
//! Everything above it takes the machine's answers as values, which is what
//! keeps the whole rules half testable at the desk.
//!
//! # A trait exists only where the far side is genuinely somebody else's
//!
//! This is the split `App/Keys/` drew and every crate since has kept. Two
//! kinds of far side arise here and they are not the same kind.
//!
//! **A machine over a connection.** [`walking::ReadsOneDirectoryOnTheMachine`]
//! and [`record_directory::ReadsOneFileOnTheMachine`] are those, and
//! [`wiring`] fills both over `Server protocol and transfers`' own subsystem
//! client. **They have real ends** — this crate drives a working transport
//! rather than composing another.
//!
//! **The platform file adapter, whose crate cannot be reached from here.**
//! `Desktop app and delivery`'s crate depends on the feature modules and
//! supplies their adapters, so a feature module cannot depend on it back —
//! the wall `Dependency Upload & Remap` F13 already met and the rules half
//! already answered. [`crate::fetching::TheReceivingSystemsRules`],
//! [`probing::WritesTheProbe`], [`orphans::TheReservedStagingDirectory`] and
//! [`revealing::OpensAFolder`] are those, and **nothing here fills one**:
//! the acts they name are that document's C3, C10, C13, F1 to F9, F10, F10a
//! and K20 to K24. **F1 to F5, F7, F9 and F10 are built in that crate**, at
//! its `platform/files.rs`; **C3, C10, C13, F6, F8, F10a's read of a stored
//! name, and K20 to K24 are not** — and **no adapter there implements any of
//! the four seams above**, which is what leaves them empty here. *(Corrected
//! 2026-09-07: this said every one of those clauses was unbuilt in that
//! crate, which was never true of the adapter's own write path, and it made
//! the seams look further off than they are.)* **What this module builds
//! around them is the sequence** — the
//! order the calls are made in, what each outcome means, and the cadence — and
//! a test double is the only thing that ever fills one here.
//!
//! **No class is produced from a byte, a name, an error number or a symbol
//! anywhere in this module**, which is the same bound the rules half kept.
//!
//! # What is not here, and each is a clause rather than an omission
//!
//! **The durable store.** [`crate::arrangement::KeepsTheArrangement`] and
//! [`crate::request::KeepsTheRequest`] stay seams. `State, identity and
//! recovery` §1.1 puts every durable store in the one local record, that
//! record is `Desktop app and delivery` §5's, and its keepers live in that
//! crate beside `File Browser`'s — `smoozy_desktop::record::upload_requests`
//! is the template and the twin. **It is not this builder's to write in**, and
//! [`crate::arriving_later`] carries it with what removes it.
//!
//! **The connected period.** Nobody in this workspace opens, holds or reopens
//! a reading channel for a connected period, and admission is the caller's by
//! `Server protocol and transfers` T5's own words. So this module takes a
//! channel it is handed and a connection condition it is told, and holds
//! neither.
//!
//! **`Server protocol and transfers` S13's four-value reading is not composed
//! here.** [`fetching::TheFourValues`] is the seam, its producer is the
//! desktop's (since the 38's piece 9), and [`fetching`]'s own header says what
//! stands where the reading cannot be taken.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Desktop app and delivery` C3, C10, C13, F1, F5,
//! F6, F7, F8, F9, F10, F10a, K20, K24, §5. `Server protocol and transfers`
//! S13, T5.
//! `State, identity and recovery` §1.1. `Dependency Upload & Remap` F13.

pub mod enumerating;
pub mod fetching;
pub mod orphans;
pub mod probing;
pub mod record_directory;
pub mod revealing;
pub mod running;
pub mod walking;
pub mod wiring;

//! **The board — `Telegram` §3, the rules over values.** One shared pinned
//! note, written only by the machine holding the slot, carrying the heartbeat,
//! the roster and the list of instructions that have not been acknowledged.
//!
//! Everything in this folder is a rule over values and calls nothing. **The
//! acts — reading the pin, asking, pinning, editing, unpinning — are
//! [`crate::listener`]'s**, so every rule here can be tested without a service.
//!
//! `Telegram` §1 is read as a section and binds this folder whole: T4 to T14
//! of `Rented-server runtime` §10 are taken as given, and what is this
//! document's is the board's contents and capacity, how instructions are
//! accepted and retained, which message is pinned, J3's removal, and the
//! context-resolution entry's expiry.
//!
//! **`Telegram` B9a — machines on different releases share one board.** A
//! machine reads every board, machinery message and instruction entry of its
//! own version and of every earlier one ([`text::what_is_pinned`],
//! [`instructions::AnEntry::is_spoken_here`]), writes only its own, and never
//! acts on an entry whose version it does not speak. **Nothing on a machine
//! runs this folder yet**: the machine's program does not take this crate, so
//! the rule is kept here, with its tests, for the day one does.

pub mod hand_over;
pub mod instructions;
pub mod janitor;
pub mod roster;
pub mod staleness;
pub mod text;

//! Servers.
//!
//! What a connected period is and how it differs from the connection under it;
//! the closed list of the moments a connection is opened and replaced; how a
//! break is learned of without using the connection first; how many repair
//! attempts there are and how long the pauses; what a saved server holds,
//! where, and what an edit and a removal each reach; when the machine's
//! hardware is read and the three states that reading has; how a server's
//! identity is trusted, compared and replaced; and what Servers presents of
//! work the machine is doing that it did not start. Its plan is
//! `Preparation/Architecture/Servers.md`.
//!
//! **What binds this before anything is decided** (§1 of the plan, the
//! *What binds this* section). `State, identity and recovery` binds hardest and
//! its rules are not restated as a second set here: §8.5 requires the
//! connection's observed value to come from a mechanism that reports a break
//! **without the caller having to use the connection first** ([`keepalive`]);
//! §2.3's installation-identity check runs before any machine-derived value
//! this module holds is used ([`hardware`]); §1.1's local record is the only
//! durable store on the user's side ([`store`]); and §3's D1, D2, D4 and D5
//! bind every reading and every state below.
//!
//! **`Server protocol and transfers` owns everything that rides on a connection
//! this module makes**, and its four hand-overs are taken rather than
//! re-decided: transfer channels and the command channel share this
//! connection, a machine permitting too few sessions is this module's decision
//! ([`period::AdditionalConnection`]); a replaced connection replaces channels
//! and nothing else ([`transfers`]); the offered host key is obtainable and
//! comparable before any connection presenting Smoozy's credential
//! ([`first_connection`]); and no health check there pings ([`keepalive`]).
//!
//! **`Security and licence` §7 owns host trust and §6 owns the private half of
//! a key.** The throwaway known-hosts file, its placement and its sweep are
//! that document's; the rule that Smoozy holds a path and a fingerprint and
//! never key material is that document's; §3 owns the secret store. None of the
//! three is retaken here.
//!
//! **Three observations run through everything below and none is softened.**
//! The danger is never the machine going away; it is the caller's own
//! connection failing — a machine closing politely and a machine powered off
//! were both announced in the same second, while the caller's own line pulled
//! took 56 seconds to detect with keepalives on and was **not detected at all
//! for 21 minutes 45 seconds** with them off. The machine sends no
//! application-level keepalive of its own, which is OpenSSH's shipped default.
//! And a container rental's recycle destroys everything Smoozy installed and
//! comes back on the same address and port with **nothing in the connection to
//! reveal it** — so Smoozy checks for its own state, never for reachability.
//!
//! **Step 4 of `Preparation/Build/Plan.md` builds this module.** What is here
//! is the half that decides: the period and its closed lists, the repair's
//! shape and its figures, the store's enumeration, the readings' shape and the
//! judgements over them, the warnings' composition. **Nothing in it opens a
//! connection, runs a process, reads a file or reaches a machine** — every
//! reading a judgement stands on sits behind one trait, and the acting half is
//! what implements it. That is the split step 2 and step 3 both used, and the
//! split the plan itself draws.
//!
//! **How a clause marker is written in this crate, and it is a rule rather than
//! a habit.** A marker with no document beside it means a clause of this
//! crate's own plan. Every other document's marker is written with that
//! document — at the site, or in the file's own header list of markers that
//! belong elsewhere. **Where a marker is a clause of this crate's own plan
//! *and* of the document meant, the header list is not enough and the document
//! is written out at the site**, because a bare one there does not fail to
//! resolve: it resolves to a real and wrong clause, which nobody reading it
//! can tell from a right one.

// The marker guard, test-only: every clause marker in this crate resolves to
// the clause it means. `markers.rs` states what it holds and what it cannot
// see.
#[cfg(test)]
mod markers;

pub mod acting;
pub mod capability;
pub mod condition;
pub mod diary;
pub mod driving;
pub mod edits;
pub mod first_connection;
/// **The hardware reading's values, at the path they have always been reached
/// at.** They live in `smoozy-servers-values`, the half of this module the
/// rented machine's own program may take. Nothing about them changed with the
/// move.
pub use smoozy_servers_values::hardware;
pub mod identity;
/// **The keepalive figures, at the path they have always been reached at**,
/// for the same reason.
pub use smoozy_servers_values::keepalive;
pub mod key_moments;
/// **The provider's connection line, pasted into the Add dialog** — §13.3.
/// Read as text and never run.
pub mod pasted_line;
pub mod period;
pub mod readings;
pub mod rental;
pub mod repair;
pub mod restart;
pub mod sequence;
pub mod setup_bit;
pub mod store;
pub mod transfers;
#[cfg(test)]
pub(crate) mod testing;
pub mod units;
pub mod wiring;
pub mod work;

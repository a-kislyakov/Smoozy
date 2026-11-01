//! Server protocol and transfers.
//!
//! The two mechanisms the pre-package surface is made of, request framing,
//! correlation so that sending a thing twice does not do it twice, and the one
//! staging-resume-integrity-publication contract that holds in both
//! directions for the machine's whole life. Its plan is
//! `Preparation/Architecture/Server protocol and transfers.md`.
//!
//! **This crate is half of the module and `App/Server protocol and transfers —
//! framing/` is the other half.** One plan governs both. **This half is every
//! act of connecting**; that half is what both ends speak, and it depends on
//! nothing that opens a connection. **The reason is the program the rented
//! machine runs**: it is the far end of an SSH connection and never opens one,
//! so it takes the framing crate and none of this one, and the SSH client and
//! its C cryptography are out of its tree altogether. That crate's own
//! front matter carries where the line falls and why.
//!
//! **What is here.** The transport and the command channel
//! ([`transport`]); the file-transfer subsystem, which is §1.1's second
//! mechanism ([`subsystem`]); §1.1's planted-link carriage, the one route a
//! machine-origin path has ([`carriage`]); §9's free-space reading, both of F2's
//! routes ([`free_space`]); and §7's whole staging, integrity, resume and
//! publication contract ([`transfers`]); and **§5's B8, B9 and B10** — the
//! signed release manifest a server package is selected from, the offline
//! release root trust is anchored in, and verification before execution at both
//! ends ([`release`]), **with the fetch that speaks HTTPS to the exact source a
//! verified manifest names** ([`release::fetching`]). **Two things that module
//! needs are the Creator's, one of
//! them is closed, and neither is stubbed** — somewhere to fetch an artifact
//! from, which is B8a's address and is still one value nobody has written down;
//! and a signing key, **of which the offline release root is closed**, its
//! public half carried in the build at
//! [`release::trust::THE_RELEASE_ROOT_THIS_BUILD_CARRIES`] and its secret half
//! held outside this repository, leaving the online manifest-signing key B9 has
//! the root vouch for. That module says all of this at itself.
//! *(Corrected 2026-09-07: this line said a signing key was absent outright,
//! which stopped being true on 2026-09-06 when the root was generated —
//! [`release`]'s own header had said so since and this one had not.)*
//!
//! **What is next door, and reachable from here unchanged.** **§2's request
//! framing over the machine-local endpoint** ([`framing`]) — the frame, the two
//! versions, what a version disagreement does, and the two rules that bind a
//! request's fields; the free-space reading's own shape, re-exported by
//! [`free_space`]; and what a subsystem operation answered, re-exported by
//! [`subsystem`] at `StatusCode` and `SubsystemFailure`. **Every one of those
//! paths resolves exactly as it did before the split**, which is why no crate
//! outside this module changed with it.
//! **§4.4 and §4.5's own two** ([`reconnection`]) — C15's read of the
//! installation identity on the pre-package surface, and C12's difference
//! between the two steps of a replaced connection that need the supervisor and
//! the two that do not. **§4.3's acknowledgement and the three closed answers
//! about an operation identity are next door**, in the framing half, because
//! both ends speak them and the machine's own program has to compose one.
//!
//! **What is not here is listed clause by clause in [`arriving_later`]**, with
//! the reason and the step each part lands at. That list is the one place an
//! absence in this document is recorded, and it covers §4 above as well as
//! everything else.
//!
//! **The connection is Smoozy's own, and no program is run to make it**
//! (`Servers` K1a). Smoozy opens, authenticates, holds up and tears down this
//! connection inside its own process, on the SSH transport it drives — R5's
//! byte loop, S15's rename extension and T5's session count are all properties
//! of the same one. `IdentityFile`, `BatchMode`, `SSH_ASKPASS_REQUIRE` and
//! `ServerAliveInterval` are not mechanisms of this architecture: each
//! configures a program it does not run.
//!
//! **A1: every request on every route is authenticated by the SSH connection
//! and by nothing of its own.** There is no second credential, no per-request
//! token, no bearer value and no signature.
//!
//! **Markers used below that belong to another document, each with the
//! document it belongs to.** `Servers` K1a.
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

pub mod arriving_later;
// **One general way to ask the machine's supervisor something and get the
// matching answer back** — step 10c piece 2, beside the per-server point that
// lends the two mechanisms it asks for.
pub mod asking_the_supervisor;
pub mod bootstrap;
pub mod carriage;

// **§2's framing, at the path it has always been reached at.** The module
// itself sits in the other half of this module; this line is what keeps
// `smoozy_server_protocol::framing::…` resolving for every crate that reads it.
pub use smoozy_framing::framing;
// **Step 10c piece 1's request names, where the desktop can reach them.** The
// desktop depends on this crate and not on the framing crate, so without this
// line its one sender (step 10c piece 6) could reach no name and would have
// had to spell one again — the fault piece 1 exists to end.
pub use smoozy_framing::requests;

pub mod free_space;
pub mod placement;
pub mod reconnection;
pub mod release;
pub mod subsystem;
pub mod transfers;
pub mod transport;

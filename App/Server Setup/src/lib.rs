//! Server Setup.
//!
//! How a rented machine is made able to render, and how that act is detected,
//! ordered, recorded, interrupted, resumed and safely run again; how Blender
//! builds are listed, fetched, sent up, unpacked, found, matched and removed;
//! how far the add-on offer's machinery reaches and what is read back after an
//! install; how the Telegram role's material is placed by either of its two
//! entry points; how the machine is restarted as part of a setup step and what
//! stands while it is down; how elevation is obtained, how a permission refusal
//! is told apart from every other failure, and what the user-facing fallback
//! re-run does on a machine whose supervisor is dead. Its plan is
//! `Preparation/Architecture/Server Setup.md`.
//!
//! # What is here, and what is not
//!
//! **`Preparation/Build/Plan.md` step 6 builds this module in two halves and
//! both are here** — every decision the module makes, expressed over traits, and
//! the acts that carry them out. ~~What is here is the rules half, with nothing
//! driven and nothing wired into the running application.~~ *(Corrected
//! 2026-09-07: that was true when it was written and stopped being true when the
//! acting half landed on 2026-09-04 and the listing fetch on 2026-09-07;
//! [`arriving_later`] said so and this paragraph had not caught up.)* That is the
//! split `Servers` and `File Browser` were built in, and the split the plan
//! itself draws. **Eleven of the plan's sections bear clause markers and two more
//! carry work without any, and all thirteen have code:**
//!
//! - **§1 the machine kind and the distribution family** — [`kind`].
//! - **§2 the ready action** — [`ready_action`] (§2.1, §2.2, §2.4, §2.5) and
//!   [`placement`] (§2.3).
//! - **§3 the setup run and its boundaries** — [`run`] (§3.1, §3.2) and
//!   [`driving`] (§3.3).
//! - **§4 interruption, resumption and the safe re-run** — [`recovery`].
//! - **§5 the Blender lifecycle** — [`blender`] (§5.1 to §5.3), [`listing`]
//!   (B2's own fetch and reading) and [`search`] (§5.4, §5.5).
//! - **§6 add-ons** — [`addons`].
//! - **§7 the Telegram role's material** — [`telegram`].
//! - **§8 the restart** — [`restart`].
//! - **§9 elevation, the refusal and the password** — [`elevation`].
//! - **§9b the user-facing fallback re-run** — [`fallback`].
//! - **§10 what this module says about a machine** — [`statements`].
//! - **§11 declarations** — [`declarations`].
//! - **§12 what this document supplies**, for its diary-act declaration —
//!   [`diary`].
//!
//! **§11 and §12 are the two that bear no clause marker.** They are sections
//! read rather than counted, and each carries work, so each has code — which is
//! why the list above is thirteen entries and the clause count below is spread
//! over the other eleven.
//!
//! **Nothing in this crate reaches a rented machine on its own account.** Every
//! reading a judgement stands on sits behind a trait, and the acts this crate
//! performs on a machine ride a connection somebody else opened and holds —
//! [`acts`], [`elevated`] and [`wiring`] — while [`arriving_later`] names each
//! act that is left to another program, with the reason.
//!
//! **One thing in this crate reaches outside on its own, and it is the only
//! one**: [`listing`] makes B2's request to the shop-front over the one HTTPS
//! client this workspace has. **It touches no machine, no connection of
//! `Servers`', no file and no process.**
//!
//! *(Corrected 2026-09-07. This said nothing here opens a connection, runs a
//! process or reads a file, which stopped being true when the acting half landed
//! and stopped being true a second way when the listing fetch did.)*
//!
//! **One seam of another document's is answered here**, because that document
//! assigns it to this one: `File Browser` P5's two elevation readings — the
//! machine kind and the non-interactive probe — which `File Browser` §12 names
//! as `Server Setup`'s. The end is [`elevation::WhatElevationWouldDo`], and it
//! maps this document's own kind table onto the three rows P5 judges over.
//! **Nothing is stood up for it**: the two answers come from
//! [`elevation::ReadsTheMachineAtARefusal`], which the acting half implements
//! over the connection.
//!
//! # What binds this before anything is decided
//!
//! **Every rule here is written per machine kind**, off a reading taken at every
//! placement and never carried forward (`Rented-server runtime` K2). **On a
//! container nothing about the driver is open at all** — all four routes closed,
//! a restart is not a recovery action there, and a card that has stopped
//! answering is terminal for that server (Fact 48) — so [`ready_action`] R3 is a
//! detection step and two branches, and one branch's honest answer is *it
//! already is ready, or it cannot be made so*.
//!
//! **One login runs everything, and elevation is therefore never the worker's.**
//! The supervisor and every worker it starts, the render included, run as one
//! login (`Job files and outputs` L13). **An elevated act is performed over the
//! SSH connection by the machine's own elevating command, with the password on
//! that command's standard input and never as an argument** (`Server protocol
//! and transfers` §1.3; `Security and licence` §4) — the route is fixed there
//! and is not re-chosen here.
//!
//! **Nothing here is keyed on a host key, on reachability, on a process name or
//! pattern, or on a file's presence, name or size** (`Rented-server runtime`
//! §15's closing rule; `State, identity and recovery` §1.3 rule 5). **A recycled
//! container answers on the same address with everything Smoozy installed
//! destroyed**, and the installation identity of `State, identity and recovery`
//! §2.3 is the only thing that tells a rental from an installation. **Nothing
//! about what a machine holds is remembered across connections**: readings here
//! are period-held, and **no desired state is ever inferred from an observed
//! reading**; readings taken on different clocks are never subtracted.
//!
//! **Do not read past the input documents.** Every machine observation this
//! plan quotes is a frozen snapshot of **one to nine boxes on one to four days,
//! across three sellers**: *"Read it as this is what that box did, never as this
//! is what a rented machine does."*
//!
//! **Rule 8 was held to throughout.** Nothing here checks the user's work,
//! warns about it, or rescues them from it beyond what a clause of the plan
//! states. **And no Blender fact is settled or looked up in this crate**: every
//! one traces to the clause of the plan that carries it, with the plan's own
//! label.
//!
//! # How a clause marker is written in this crate, and it is a rule
//!
//! **A marker with no document beside it means a clause of this crate's own
//! plan** — `R1`, `S7b`, `B12a`, `W4`. **Every other document's marker is
//! written with that document**, at the site, or in that file's own header list
//! of the markers in it that belong elsewhere. `M1` is defined in eight
//! different documents, so a bare marker does not fail to resolve: **it resolves
//! to a real and wrong clause, which nobody reading it can tell from a right
//! one.**
//!
//! **Where a marker is a clause of this crate's own plan *and* of the document
//! meant, the header list is not enough and the document is written out at the
//! site.** This plan's `M1` to `M4`, `A1` to `A9`, `S1` to `S7`, `B1` to `B15`,
//! `R1` to `R9`, `T1` to `T7`, `W1` to `W4`, `E1` to `E7`, `G1` to `G4`,
//! `F1` to `F3` and `Y1` to `Y9` all collide with somebody's.
//!
//! **[`arriving_later`] carries the test that holds this**, and it holds the
//! clause accounting beside it.
//!
//! **The header list is checked, not trusted.** Settled 2026-09-04, out of the
//! step 6 gate: every marker in a header list is checked against the clauses of
//! the document its own segment names, because the list was a whitelist that
//! forgave anything written in it and two markers filed under a plan with no
//! such prefix survived a gate run that way. **The list ends at the first empty
//! comment line**, so a file may follow it with a note naming the markers it
//! deliberately does not list.
//!
//! **This is the settled shape and the crates still to be written copy it.** The
//! five legs, the traps it cannot see, and why the file walk recurses and asserts
//! an exact count are all written at the test itself.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Rented-server runtime` K2, §15. `File Browser`
//! P5, §12. `Job files and outputs` L13. `Server protocol and transfers` §1.3.
//! `Security and licence` §4. `State, identity and recovery` §1.3, §2.3.

pub mod acts;
/// **The add-on read-backs, at the path they have always been reached at.**
/// This module and `driving`, `recovery` and `run` live whole in
/// `smoozy-server-setup-machine`, the half of this plan the rented machine's
/// own program may take. Split at step 10c piece 3.
pub use smoozy_server_setup_machine::addons;
pub mod arriving_later;
pub mod blender;
pub mod declarations;
pub mod diary;
pub use smoozy_server_setup_machine::driving;
pub mod elevated;
pub mod elevation;
pub mod fallback;
pub mod kind;
pub mod listing;
pub mod placement;
pub mod ready_action;
pub mod readings;
pub use smoozy_server_setup_machine::recovery;
pub mod restart;
pub use smoozy_server_setup_machine::run;
pub mod search;
pub mod statements;
pub mod telegram;
pub mod wiring;

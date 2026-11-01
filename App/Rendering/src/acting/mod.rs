//! The half that acts: the render started and held on the machine, the two
//! streams copied and observed, the stop performed, the submission's ordered
//! unit placed, and the readings a page takes of a machine.
//!
//! # What this is
//!
//! The rest of this crate is judgement — a rule, a table, a total function
//! over a reading somebody else took. **Nothing there starts a process, opens
//! a file, sends a signal, runs a command or writes a store.** This module is
//! where all of that happens, and it is the only place in the crate that
//! does. [`crate::arriving_later`] is the list it answers, act by act with
//! the clause each belongs to.
//!
//! # The seam, and why it has this shape
//!
//! **This plan declares no seam of its own — nothing above is behind a trait
//! — so the seam is designed here as well as filled.** Four things decided
//! the shape, and each of them is a constraint rather than a preference.
//!
//! **One. The seam is drawn where the machine is, not where the module is.**
//! This document acts on two sides that never meet in one process: the render
//! worker on the rented machine ([`launch`], [`material`], [`copy_loop`],
//! [`pipeline`], [`stopping`]) and the user's own computer ([`submitting`],
//! [`resolving`], [`page_readings`]). **Nothing on the machine side reaches a
//! connection and nothing on the desk side reaches a process**, which is P2
//! held by the module boundary rather than by care.
//!
//! **Two. A trait exists only where a machine is genuinely on the far side of
//! it.** Three do, on the machine: what starts the render ([`launch`]), what
//! writes and reads a file under the render's login ([`material`]), and what
//! asks a Blender to name itself ([`launch::AsksABlenderToNameItsVersion`]).
//! On the desk there are none of this crate's own: the connection's own seams
//! already exist and are taken whole — `Servers`' command channel, `Server
//! protocol and transfers`' file-transfer subsystem and its planted-link
//! carriage, and `Job files and outputs`' `TheMachinesDirectories`. **A
//! second seam over any of those would be a second thing to keep true.**
//!
//! **Three. The copy loop is the seam step 7 turns on, and it is one loop
//! with observers rather than two readers of one stream.** `Queue & History`
//! R1 makes its reader **a tee on this loop** — not a second read of
//! anything — and K13 puts five observations on the same loop: the live
//! readings, the ending-step completeness reading, the frame's save
//! announcement, the device line and the colour-substitution line. So
//! [`copy_loop`] holds the bytes, copies them to the supervisor's sinks
//! through `Rented-server runtime`'s own writer, splits what it copied into
//! lines, and offers each line to a list of observers. **The parse is not
//! here**: the grammars are `Queue & History` R2's and its acting half owns
//! them, and [`copy_loop::TheQueuesReaderAsATee`] is the one adapter that
//! carries a line into that crate and its readings back out. **That is the
//! whole of the seam between this step's two heavy plans**, and it runs one
//! way: this crate depends on that one, that one never on this.
//!
//! **Four. Nothing here re-implements what the workspace already has.** The
//! sinks and the write into them are `Rented-server runtime` M1 and
//! `Rented-server runtime` M3, taken at
//! [`smoozy_runtime::acting::capture`]; the ceiling's arithmetic is that
//! crate's [`smoozy_runtime::capture::CaptureCeiling`]; the interrupt is
//! [`smoozy_runtime::acting::execution_group::send_the_interrupt_to`]; the
//! kernel's own reading of a pair is that crate's
//! `ThePairAsTheKernelReportsIt`; the hold's release predicate is fed to
//! [`smoozy_runtime::holds::evaluate`]; the worker's report rides the
//! machine-local endpoint at
//! [`smoozy_runtime::acting::endpoint::TheEndpointAsACaller`] in `Server
//! protocol and transfers`' own frame. **No transport is written here and no
//! command string is composed here.**
//!
//! # Why the machine side is gated to a unix
//!
//! Every act on that side is a unix act — a process whose kernel-set start
//! time makes its recorded pair, a signal, a pipe, a file written under one
//! login. **The machine Smoozy rents is never Windows**, so those modules are
//! absent there rather than answered wrongly. **macOS is a unix and is the
//! desk**, which is what lets the launch, the copy loop, the material's
//! placement and read-back, and the interrupt be run for real at the desk
//! rather than only reasoned about.
//!
//! # What is not here, and each is a clause rather than an omission
//!
//! **Nothing starts the render worker.** *(Corrected 2026-09-07. This said —
//! as four agents had recorded — that there was no program for the rented
//! machine anywhere in this workspace. That stopped being true at step 8: the
//! program is `smoozy-server`, at `App/Rented-server runtime — program/`, and
//! it starts a supervisor and an endpoint.)* **What is still true is that it
//! starts no worker**: it refuses every one of the seven kinds by name, naming
//! the document that owns each. *(Corrected 2026-09-19: this also said the
//! program depended on `smoozy-runtime`, `smoozy-framing` and `smoozy-state`
//! alone and so could not reach this crate at all. At step 10c piece 3 the
//! render worker moved to `smoozy-rendering-machine` and the program took that
//! crate, so it can reach the worker; it still starts none, and this crate is
//! not among its dependencies.)* So the render worker below is **a library the
//! supervisor would start**, exactly as `Rented-server runtime`'s own worker start already
//! is, and
//! [`launch::HowTheRenderWorkerIsInvoked`] names what that document's
//! `SaysHowThisWorkerIsInvoked` would be handed. **Nothing here invents a
//! program**, and nothing here has a `main`.
//!
//! **The supervisor's own durable writes are not performed here.**
//! `Rented-server runtime` S1 makes the supervisor the only writer of every
//! durable store on the machine; this crate composes what is written and
//! hands it over. [`pipeline`] says at each step which act is the
//! supervisor's.
//!
//! **The desktop's own local record is reached through one trait of this
//! crate's, [`page_readings::Readings`]**, in the split `App/Keys/` drew and
//! every crate since has kept — so the judgements above stay testable with
//! the record's answers arriving as values.
//!
//! **Markers used in this file that belong to another document, each with
//! the document it belongs to.** `Rented-server runtime` M1, M3.
//! `Queue & History` R1, R2.
//!
//! **`Rented-server runtime` S1 is deliberately not on that list and is
//! written out at its site instead**, because this plan defines an S1 of its
//! own.

pub mod page_readings;
pub mod resolving;
pub mod submitting;

/// **The render worker's side of this list lives whole in
/// `smoozy-rendering-machine`** — the half of this plan the rented machine's
/// own program may take, split out on 2026-09-19 so that program can start the
/// render worker without this crate's transfer contract and the SSH client
/// beneath it. Each is re-exported here at the path it has always had, and is
/// gated to a unix there for the reason above.
#[cfg(unix)]
pub use smoozy_rendering_machine::acting::{copy_loop, launch, material, pipeline, stopping};

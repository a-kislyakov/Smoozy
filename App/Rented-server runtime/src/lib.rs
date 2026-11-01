//! Rented-server runtime.
//!
//! How the server package is placed, committed, rolled back, repaired and
//! replaced on a machine whose boot return is two arrangements rather than one;
//! what the supervisor is, what restarts it, and what its machine-local endpoint
//! is made of; how every worker is started, identified, arbitrated and stopped;
//! how work waiting for a person is held on the machine, outside its line,
//! indefinitely, and released by an answer arriving over a path Smoozy is not on;
//! what holds reconciliation back while a setup is unfinished; how the Telegram
//! role is hosted and which machine may ask for what the user sends; and how a
//! render's two streams and its exit are captured, retained with explicit loss
//! reporting, and presented so that *the render printed nothing* is a recorded
//! fact rather than an absence. Its plan is
//! `Preparation/Architecture/Rented-server runtime.md`.
//!
//! # This code runs on the rented machine, not on the user's computer
//!
//! **That is the thing that makes this module unlike every one before it.**
//! `Preparation/Architecture/The shape.md` settles it: the supervisor, its
//! machine-local endpoint, every worker and the package transaction's machine half
//! all live on a rented Linux box that Smoozy did not build and cannot trust to be
//! like the last one. **Two consequences run through every file here.** Nothing
//! reads the user's computer, and no path, figure or capability of the desktop
//! appears anywhere. And **every machine observation the plan carries is a frozen
//! snapshot of one to six boxes on one to three days** — *"Read it as *this is what
//! that box did*, never as *this is what a rented machine does*"* — so the readings
//! that could not be relied on are named as traps rather than used.
//!
//! # What is here, and what is not
//!
//! **This module is built in two halves and both are here.** The **rules half**
//! is every decision the module makes, expressed over traits; the **acting half**
//! is [`acting`], which is the real end of every one of those readings and acts
//! and is the only part of this crate that opens a socket, starts a process,
//! sends a signal, renames a file or runs a command.
//!
//! All fourteen of the plan's clause-carrying sections have code:
//!
//! - **§1 the machine kind and the boot return** — [`machine_kind`],
//!   [`registration`].
//! - **§2 the package transaction** — [`package::release`], [`package::phase`],
//!   [`package::commit`], [`package::retry`].
//! - **§3 the supervisor** — [`supervisor`].
//! - **§4 the machine-local endpoint** — [`endpoint`].
//! - **§5 worker classes and their lifecycles** — [`workers`].
//! - **§6 the recorded execution group, and the stop** — [`execution_group`].
//! - **§7 the common arbitration mechanism** — [`arbitration`].
//! - **§8 machine-held work awaiting a person** — [`holds`].
//! - **§9 the setup-in-progress state** — [`setup_state`].
//! - **§10 the Telegram role worker and the bounded board** — [`telegram_role`].
//! - **§11 captured streams, the exit and the collection outcome** — [`capture`].
//! - **§12 health readings** — [`health_readings`].
//! - **§13 retention on the machine** — [`retention`].
//! - **§14 atomic or partial** — [`declarations`].
//!
//! **§15's diary declaration is [`diary`], and it is a declaration of none** —
//! written out with its reasons rather than left implied, because `Desktop app and
//! delivery` A7 makes a module with no declaration write nothing silently, **and a
//! hosting fault and a declaration of none are indistinguishable from the
//! outside.**
//!
//! **Every reading a judgement stands on is behind a trait, and eighteen of the
//! twenty-one traits reach a machine through [`acting`].** **Three do not**, and
//! every one of the three waits on the same thing — request framing over the
//! machine-local endpoint, which is `Server protocol and transfers` §2's and is
//! unbuilt in that crate. **Which traits those are, where each real end is, and
//! what the three are still missing, is [`arriving_later::THE_ACTING_HALF`]**,
//! and a test there holds that record to what the whole application actually
//! implements.
//!
//! **The split between the halves is a rule of this crate rather than a
//! tidiness.** Every file outside [`acting`] is judgement over a reading somebody
//! else took, and none of them reaches a machine; every act that does is in that
//! one module, with the publication behind it quoted at the point of use.
//!
//! **What is not here at all is named clause by clause in [`arriving_later`]**,
//! with the reason and where it lands.
//!
//! # What binds this before anything is decided
//!
//! **`State, identity and recovery` §1.3 rule 5 binds every section with no
//! carve-out of this document's own: nothing reads a file's presence, its name or
//! its size as state.** The single carve-out is `Job files and outputs`', for the
//! re-run's already-there test. **That rule is what forces [`capture`]'s byte count
//! to be Smoozy's own count rather than a reading of a sink, and
//! [`registration::confirm`] to be a comparison of content rather than of
//! existence.**
//!
//! **A container rental's recycle destroys everything Smoozy installed and replaces
//! the boot hook with an 82-byte provider default, on the same address and port,
//! with nothing in the connection to reveal it.** *"A Smoozy that reconnects and
//! finds its address answering has no signal that its whole installation is gone.
//! It must check for its own state, not for reachability."* **Nothing in this crate
//! is keyed on a host key, on reachability, on a process name or pattern, on a bare
//! process identifier, or on a file's or directory's presence, name or size** —
//! §15's own closing rule, and it is the one sentence to hold every file here to.
//!
//! **The arrangement that makes work survive a reboot also undoes a deliberate
//! stop.** Only the supervisor is boot-restarted; **every worker outcome comes from
//! durable state** ([`supervisor::Restarter`], [`workers`]).
//!
//! **Rule 8 was held to throughout.** Nothing here checks the user's work, warns
//! about it, or rescues them from it beyond what a clause of the plan states — and
//! the two places where a rescue would have been easiest are refused in as many
//! words: [`health_readings::NO_AUTOMATIC_DESTRUCTION_IS_BUILT`], and
//! [`arriving_later::THE_STORED_FIELD_HOLE`], which is the known hole this step's
//! own milestone carries a qualifier for.
//!
//! # How a clause marker is written in this crate, and it is a rule
//!
//! **A marker with no document beside it means a clause of this crate's own plan**
//! — `K15`, `G1b`, `A5b`, `M4a`. **Every other document's marker is written with
//! that document**, at the site, and every file carries a header list of the
//! markers in it that belong elsewhere. **`M1` is defined in eight different
//! documents**, so a bare marker does not fail to resolve: **it resolves to a real
//! and wrong clause, which nobody reading it can tell from a right one.**
//!
//! **A test enforces both halves** —
//! [`the_markers_are_written_with_their_documents`](arriving_later) — because about
//! 150 bare markers were swept out of nine crates on 2026-09-03 having survived
//! five gate runs, and a rule nobody can check is a rule that decays.//!
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
//! document it belongs to.** `Desktop app and delivery` — its hosting rule for a
//! module with no declaration. `State, identity and recovery` §1.3 rule 5. `Job
//! files and outputs` — the already-there test.
//!
//! **A marker this plan also defines is never listed here and is written out
//! with its document at the site**, because a header list cannot tell the two
//! apart and a bare one there would resolve to a real and wrong clause.

pub mod arbitration;
pub mod arriving_later;
#[cfg(unix)]
pub mod acting;
pub mod capture;
pub mod declarations;
pub mod diary;
pub mod endpoint;
pub mod execution_group;
pub mod health_readings;
pub mod holds;
pub mod machine_kind;
pub mod package;
pub mod registration;
pub mod registration_content;
pub mod retention;
pub mod setup_state;
pub mod supervisor;
pub mod telegram_role;
pub mod the_program;
pub mod workers;
pub mod written_form;

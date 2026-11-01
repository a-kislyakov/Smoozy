//! File Browser.
//!
//! How the browser lists a machine and what each listed fact is worth; how a
//! permission refusal is told apart and retried with elevated access; what an
//! upload request is and what survives a drop, an app close and a fresh sitting;
//! how the already-exists question is walked, judged and kept honest against a
//! filesystem that tells capitals apart and a partial that looks whole; where
//! each uploaded file's origin is recorded; the per-server pool and the
//! transfer-priority order; managing files on the machine; how a file changed
//! under its own upload is caught; the system picker; and this module's
//! declarations. Its plan is `Preparation/Architecture/File Browser.md`.
//!
//! # What is here, and what is not
//!
//! **The whole plan has code.** Step 5 of `Preparation/Build/Plan.md` built ten
//! of the eleven sections and held two pieces back to step 7 — §8's warning
//! layer and X7a's two outward writes — **and step 7 built both, on 2026-09-04**,
//! so nothing of this document is now absent. All eleven sections:
//!
//! - **§1 the listing** — [`listing`], [`blend_version`], [`free_space`],
//!   [`cache`].
//! - **§2 permission** — [`permission`].
//! - **§3 the upload request** — [`request`].
//! - **§4 already exists, the collision rule and the partial** —
//!   [`already_exists`], [`keep_both`], [`partial`], and X7a's two outward
//!   writes at [`publication`].
//! - **§5 uploading: the binding, the names and the record** — [`uploading`].
//! - **§6 the pool and the order** — [`pool`].
//! - **§7 managing files** — [`managing`].
//! - **§8 the three warnings, and the telling** — [`warnings`].
//! - **§9 continuation, and the unchanged file** — [`continuation`].
//! - **§10 the picker** — [`picker`].
//! - **§1.8 a picture shown in Files** (L29, added 2026-10-31) — [`picture`],
//!   its reading's real end at [`wiring`].
//! - **§11 declarations** — [`declarations`] and [`diary`].
//!
//! **Every reading a judgement stands on is behind a trait, and the real ends
//! that exist are gathered in one file — [`wiring`].** That is the only module
//! that knows a protocol or an outside library exists, so everything above it is
//! testable at the desk on values written by hand.
//!
//! **Which real ends are there, and which are not.** [`wiring`] implements seven
//! — the landing folders, the machine's own file operations, the Zstandard
//! decode, and the four readings of [`wiring::TheReadingChannel`]: the listing
//! (L1), the already-exists walk's machine side (X1), the version tag's bounded
//! prefix (L7) and the permission walk's component reading (P3). **The four are
//! one channel value rather than four seams**, which is what makes L7's *ride
//! the reading channel and are serialised behind the listing* structural.
//!
//! **Two of this crate's seams are answered next door**, in `Desktop app and
//! delivery`, because that is the direction this workspace's dependencies run:
//! [`request::KeepsTheRequest`] and [`continuation::ReadsAPathIdentity`], along
//! with the local reads §12 hands to that document's platform file adapter.
//!
//! **One more seam is answered in `Server Setup`**, and it is P5's two elevation
//! readings, the machine kind and the non-interactive probe: P5 says both belong
//! to that document, §12 assigns them there, and that crate implements
//! [`permission::ReadsWhatElevationWouldDo`] over its own kind table. **So every
//! reading this crate declares reaches a machine**, and [`arriving_later`] no
//! longer records one that does not.
//!
//! **The two things that run inside
//! another document's own loop are both §9's**, and each is that document's own
//! declared seam rather than a shape invented here:
//! [`continuation::TheSourceUnderWatch`] is `Server protocol and transfers`'
//! `TheUsersOwnComputer` carrying K7's identity and K8's watch, and
//! [`continuation::TheSourcesPublicationGate`] is that document's S15b gate
//! carrying K9's read-through and comparison.
//!
//! **Three of this crate's seams are answered in `Desktop app and delivery`
//! rather than here, and §8's two are the reason.** `Dependency Upload & Remap`
//! is built on `Render Settings`, which is built on `Server Setup`, which is
//! built on this crate for P5's elevation readings — so this crate cannot name
//! the two crates whose records §8 reads and X7a writes into without a
//! dependency cycle the build refuses. **The judgements stay here and the
//! records are reached next door**, at the document that owns the local record:
//! [`warnings::ReadsWhatTheLocalRecordKnows`] and
//! [`publication::TheNeighbouringRecords`], beside [`request::KeepsTheRequest`],
//! which was already answered there. **No record shape is copied**: the adapter
//! reads `Dependency Upload & Remap`'s own record and `Queue & History`'s own
//! job records and writes through their own operations.
//!
//! **[`arriving_later`] carries no list of absences**, because there is
//! nothing left to list.
//!
//! # What binds this before anything is decided
//!
//! **The transfer contract is `Server protocol and transfers` §7 and it is
//! taken entire.** No second transfer, staging or resume shape is invented
//! here: [`smoozy_server_protocol::transfers`] is read where this document
//! needs one, and the publication intent this document supplies is that
//! document's own declared parameter ([`smoozy_server_protocol::transfers::publication::Intent`],
//! its S15a) rather than a local requirement.
//!
//! **`Servers` §15 hands four supplies whole**, and each is read from that
//! crate rather than re-derived: the connection's observed value
//! ([`smoozy_servers::condition::ObservedValue`]), the closed list of period
//! endings ([`smoozy_servers::period::PeriodEnding`]) and the two labels of its
//! 12.2 ([`smoozy_servers::transfers::ConnectionLabel`]). **No ping, no probe
//! request, and the connection condition is never derived here**
//! (`Server protocol and transfers` C18 — *"no health check in this document
//! pings"* — which `Servers` K4 restates for the keepalive).
//!
//! **Until 2026-09-04 this sentence also attributed C18 to `Servers`, and that
//! document defines no such clause** — its C-series stops well short of it. The
//! marker pointed at nothing while reading exactly like a citation, which is
//! the sharpest form of the class the guard in `markers.rs` holds. **`Servers`
//! K4 is a different clause from this plan's own K4**, which is the
//! checkpointed, verified resume, so it is written with its document here and
//! everywhere.
//!
//! **`State, identity and recovery` §1.1: the local record is the only durable
//! store on this side.** Nothing in this crate holds a durable store of its
//! own. The listing tree of [`cache`] is process memory and durable nowhere,
//! and its lifetime is stated per period ending at L16 rather than by a class
//! name; the partial's record is `Server protocol and transfers` S9's, already
//! in the local record; the upload request's record and the source-path record
//! are one §9.4 outcome record ([`request::UploadRequest`]), whose keeper is a
//! seam here ([`request::KeepsTheRequest`]) and whose store is `Desktop app and
//! delivery` §5's; and the pasteboard is period-held and durable nowhere
//! ([`managing::Pasteboard`]).
//!
//! **Rule 8 was held to throughout.** Nothing here checks the user's work,
//! warns about it, or rescues them from it beyond what a clause of the plan
//! states.
//!
//! # How a clause marker is written in this crate, and it is a rule
//!
//! **A marker with no document beside it means a clause of this crate's own
//! plan** — `L8`, `P6a`, `X7`, `H3a`. **Every other document's marker is
//! written with that document**, at the site, and every file carries a header
//! list of the markers in it that belong elsewhere. `M1` is defined in eight
//! different documents, so a bare marker does not fail to resolve: it resolves
//! to a real and wrong clause, which nobody reading it can tell from a right
//! one.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Server protocol and transfers` S15a, S15b, S9,
//! C18. `State, identity and recovery` §1.1.

// The marker guard, test-only: every clause marker in this crate resolves to
// the clause it means. `markers.rs` states what it holds and what it cannot
// see.
#[cfg(test)]
mod markers;

pub mod already_exists;
pub mod arriving_later;
pub mod blend_version;
pub mod cache;
pub mod continuation;
pub mod declarations;
pub mod diary;
pub mod free_space;
pub mod keep_both;
pub mod listing;
pub mod managing;
pub mod partial;
pub mod permission;
pub mod picker;
pub mod picture;
pub mod pool;
pub mod publication;
pub mod request;
pub mod uploading;
pub mod warnings;
pub mod wiring;

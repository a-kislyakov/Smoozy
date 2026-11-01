//! Dependency Upload & Remap.
//!
//! What this module's check reads and what a row is; the one record its whole
//! promise rests on and the two ways it is keyed; how two files are told apart
//! by contents; how the search over the user's own folders matches exactly on
//! a disk that does not; where an asset lands on the machine and what keeps it
//! there; what the remap material says and how a pair is matched; and what
//! makes `Rendering`'s warning stop and come back. Its plan is
//! `Preparation/Architecture/Dependency Upload & Remap.md`.
//!
//! **What binds this before anything is decided** (the plan's own section of
//! that name, cited and not restated). `State, identity and recovery` §1.3
//! rule 5 binds every section: **nothing reads a file's presence, its name or
//! its size as state** — a killed transfer leaves bytes at the final name
//! that are indistinguishable from a whole file (Fact 18), so *already on the
//! machine* rests on Smoozy's own record and on reading contents through, and
//! on nothing else. The transfer contract is `Server protocol and transfers`
//! §7, taken entire. Every machine path sits under `Job files and outputs`'
//! root, in the asset area its L6 assigns, outside every job folder (L7, L8),
//! checked in bytes against N5's two budgets before a byte is sent. The
//! remap's route is that document's D9 and is not re-opened. The record of §1
//! lives inside the local record and nowhere else (`State, identity and
//! recovery` §1.1). No secret is touched anywhere.
//!
//! **Step 7 of `Preparation/Build/Plan.md` builds this module, and this is
//! the half that decides.** Every reading a judgement here stands on is
//! behind one trait — [`readings::Readings`] — and every record a judgement
//! reads is behind another — [`record::TheRecord`] — so the whole of the
//! judgement is testable at the desk with no process, no file opened and no
//! machine reached. The half that acts — the check's readings driven through
//! the supervisor, the walk's real enumeration, the transfers, the fixing
//! worker, the material's application inside a running Blender — lands
//! beside it, and [`arriving_later`] names each piece with the clause it
//! belongs to.
//!
//! **Where the plan draws its own line, twice, and this crate keeps both.**
//! V1: *what Smoozy placed and fixed on this machine, and what points at
//! what* is answered from the local record alone, with no read of the machine
//! ([`knowledge`]). G14: the check is a desktop-driven loop of single-file
//! readings, not one long machine-side operation — so each reading's
//! judgement is here and the loop's driving is not ([`check`]). And V15
//! behind all of it: **presence, size and the machine's clock decide nothing**
//! — no function in this crate takes a file's presence, a size alone or a
//! machine moment as evidence of anything.
//!
//! **How a clause marker is written in this crate, and it is a rule rather
//! than a habit.** A marker with no document beside it means a clause of this
//! crate's own plan. Every other document's marker is written with that
//! document — at the site, or in the file's own header list of markers that
//! belong elsewhere. **Where a marker is a clause of this crate's own plan
//! *and* of the document meant, the header list is not enough and the
//! document is written out at the site**, because a bare one there does not
//! fail to resolve: it resolves to a real and wrong clause, which nobody
//! reading it can tell from a right one. This plan's own series are `K`,
//! `Q`, `G`, `F`, `A`, `M`, `V`, `U` and `O`, so `Rented-server runtime` A1,
//! `Job files and outputs` M1, `Desktop app and delivery` F10 and G1–G3, and
//! `File Browser` U15 are the ones most often written out.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Job files and outputs` L6, L7, L8, N5, D9.
//! `State, identity and recovery` §1.1, §1.3.

// The marker guard, test-only: every clause marker in this crate resolves to
// the clause it means. `markers.rs` states what it holds and what it cannot
// see.
#[cfg(test)]
mod markers;

pub mod acting;
pub mod adapter;
pub mod arriving_later;
/// **The byte objects a location is, at the path they have always been
/// reached at.** They live whole in `smoozy-dependencies-machine`, the half of
/// this plan the rented machine's own program may take. Split at step 10c
/// piece 3, with the items split out of `check`, `cleanup`, `landing`, `remap`
/// and `acting` re-exported in each.
pub use smoozy_dependencies_machine::bytes;
pub mod check;
pub mod cleanup;
pub mod contents;
pub mod declarations;
pub mod diary;
pub mod fixing;
pub mod knowledge;
pub mod landing;
pub mod readings;
pub mod record;
pub mod remap;
pub mod rows;
pub mod search;
pub mod selection;
pub mod shift;
pub mod standing;
pub mod units;

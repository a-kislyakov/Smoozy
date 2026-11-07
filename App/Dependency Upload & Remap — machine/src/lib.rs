//! Dependency Upload & Remap — the half the rented machine's own program may
//! take.
//!
//! **This crate and `smoozy-dependencies` are one module in two folders**, and
//! one plan governs both: `Preparation/Architecture/Dependency Upload &
//! Remap.md`. What is here moved on 2026-09-19, at step 10c piece 3: the byte
//! objects a location is ([`bytes`]), the kinds a reference can be
//! ([`check`]), the asset area and its three fixed segments ([`landing`]), the
//! remap material, its pairs and the judgements over one pair ([`remap`]), the
//! fragment that applies the pairs inside a running Blender and the reading of
//! what it reported ([`material`]), the walk over the nested marks the fragment
//! shares with the check's reading ([`reading_script`]), and the one location
//! that goes with a job's machine-side record ([`cleanup`]). **The check's
//! reading's own script and launch line followed on 2026-09-29**
//! ([`check_script`]), at step 10c's dependency wave piece 2, when the machine's
//! own program began to take that reading itself. **The instruction for an
//! uploaded bake's copy into the job folder, and the record of what was placed,
//! followed the same day** ([`placed_copies`]), at the wave's piece 8, when the
//! render worker began to make that copy. **M14's fixing script and its launch
//! line followed on 2026-10-04** ([`fixing_script`]), at the 38's piece 7, when
//! the fixing became a file-inspection operation the machine's own program
//! runs. **What is next
//! door is every act, the record, the readings' trait, the check's judgements,
//! the search, the landing rule and the composition of the pairs.**
//!
//! **Why the line is here.** The program a rented machine runs must not reach
//! `smoozy-server-protocol`, which carries the SSH client and its C
//! cryptography, and `smoozy-dependencies` reaches it. The render worker puts
//! the remap material's fragment into its launch material and reads the
//! per-pair lines back, and the queue removes the material with a job; all of
//! it is values and judgements over values.
//!
//! **Nothing that reads these had to move with them.** `smoozy-dependencies`
//! re-exports every module here, and every item split out of a module that
//! stayed, from the path it has always been reached at.
//!
//! **How a clause marker is written in this crate** is the rule next door,
//! unchanged: a marker with no document beside it means a clause of this
//! crate's own plan, and every other document's marker is written with that
//! document, at the site or in the file's own header list.

// The marker guard, test-only: every clause marker in this crate resolves to
// the clause it means. It is the same guard `smoozy-dependencies` carries,
// because the two crates are held to one plan.
#[cfg(test)]
mod markers;

pub mod bytes;
pub mod check;
pub mod check_script;
pub mod cleanup;
pub mod fixing_script;
pub mod landing;
pub mod material;
pub mod opening;
pub mod placed_copies;
pub mod reading_script;
pub mod remap;

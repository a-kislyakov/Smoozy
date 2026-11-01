//! The render worker's acts on the rented machine: the render started and held
//! ([`launch`]), the launch material placed and read back under the render's
//! login ([`material`]), the render-start reading taken ([`reading`]), the two
//! streams copied and observed ([`copy_loop`]), the nine steps performed in
//! order ([`pipeline`]), the stop performed ([`stopping`]) and L24b's lookup
//! for the OptiX library ([`the_optix_library`]).
//!
//! **The desk side's acts** — submitting, resolving a dropped submission and
//! the page's readings — are in `smoozy-rendering`'s own `acting`, which
//! re-exports every module here at the path it has always had. That module's
//! front matter says why the seam is drawn where the machine is, why the copy
//! loop is one loop with observers, and what is not here; all of it still
//! holds unchanged, and since 2026-09-19 the line it draws between the two
//! sides is also the line between the two folders.
//!
//! # Why every module here is gated to a unix
//!
//! Every act here is a unix act — a process whose kernel-set start time makes
//! its recorded pair, a signal, a pipe, a file written under one login. **The
//! machine Smoozy rents is never Windows**, so these modules are absent there
//! rather than answered wrongly. **macOS is a unix and is the desk**, which is
//! what lets the launch, the copy loop, the material's placement and read-back,
//! and the interrupt be run for real at the desk rather than only reasoned
//! about.

#[cfg(unix)]
pub mod copy_loop;
#[cfg(unix)]
pub mod launch;
#[cfg(unix)]
pub mod load_time;
#[cfg(unix)]
pub mod material;
#[cfg(unix)]
pub mod pipeline;
#[cfg(unix)]
pub mod reading;
#[cfg(unix)]
pub mod stopping;
// Owed piece 8 of step 10c: a desk check run by hand with real Blenders,
// ignored by default.
#[cfg(all(test, unix))]
mod simplify_at_the_desk;
#[cfg(unix)]
pub mod the_optix_library;

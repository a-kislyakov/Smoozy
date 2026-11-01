//! **The scenarios, played as tests, and the desk machine they play against.**
//!
//! Each file under `tests/` plays one scenario of `Preparation/Scenarios/`,
//! its situation and its steps unchanged, through the app's own acts. This
//! library is what every one of them stands on: [`the_desk_machine`], one far
//! end joining OpenSSH's own server, a line that can go silent, reset or
//! restart, the machine's own program in a throwaway package area, a stand-in
//! Blender and a local record — and the two acts only a machine does, being
//! rebooted and being destroyed.
//!
//! **Nothing in Smoozy depends on this crate**, and it ships in nothing.
//!
//! **How a clause marker is written here, and it is a rule rather than a
//! habit.** This crate has no plan of its own, so **every marker is written
//! with the document it belongs to**, at its own site, and a bare one is a
//! fault wherever any plan defines it. [`markers`] holds the test.

#[cfg(unix)]
pub mod the_desk_machine;

mod markers;

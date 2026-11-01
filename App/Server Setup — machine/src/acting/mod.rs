//! **The setup worker's own acts** — the half of a setup run that runs inside
//! a program on the rented machine, written for the first time at step 10c
//! piece 8.
//!
//! **What is here is `Server Setup` Y7's machine-side list and nothing wider.**
//! *"Machine-side steps (fetch, unpack, `ldd` reads, publication, read-backs)
//! are performed by the setup worker, commanded one step at a time; each runs
//! to its boundary and the worker then waits for the next command."* The
//! elevated half of the same run — the package installs, the driver work, the
//! hand-over and the restart — is the desktop's and is next door in
//! `smoozy-server-setup`.
//!
//! **Everything above this module is a judgement and stays one.** [`fetching`]
//! and [`steps`] perform; [`crate::run`], [`crate::recovery`],
//! [`crate::driving`] and [`crate::ready_action`] decide, and are called rather
//! than re-derived. [`worker`] is the seam between the two: what the worker may
//! do next, and what one commanded step comes to.
//!
//! **Two things the machine has that this crate must not hold itself, and each
//! is a seam with its one real end in the program.**
//!
//! - [`fetching::MakesTheRequestOnTheMachinesOwnLine`] — the HTTPS client is
//!   the machine program's, on the runner's R1 ruling of 2026-09-19, and it is
//!   the workspace's one client.
//! - [`steps::RunsOneCommandOnTheMachine`] — a process run on a Linux machine.
//!   **It is a seam so that the order, the records and the resumption can be
//!   run at the desk against stand-ins**, this desk being a Mac that has
//!   neither `ldd` nor a Linux Blender.
//!
//! **What is not a seam is the disk.** Staging, renaming and reading bytes back
//! are plain and behave the same on both systems, so they are performed here
//! and the tests use a real folder.
//!
//! **Markers used in this module that belong to another document, each with the
//! document it belongs to.** `Server protocol and transfers` §7.1, S22, S24.
//! **Every other document's marker is written out at its site**, because the
//! documents this module cites share a letter series with this plan and a
//! header list could not tell the two apart.

pub mod fetching;
pub mod steps;
pub mod worker;

/// **Y7b and Y7c: how far the fetch or the unpack has got, told as it goes.**
///
/// **The count is the worker's own** — the bytes written to the staged
/// archive (Y7b), or the bytes of it fed to the unpacker (Y7c) — **beside the
/// whole, where the whole is known**: the size the download's answer
/// announced, or the staged archive's own size. **It is never the server's
/// output** (Y7a), it is not a boundary, nothing decides anything on it, and
/// it is not durable. Added 2026-10-18, the Creator's fourteenth notes,
/// piece U.
///
/// **Telling is free to be called on every piece**; how often it reaches
/// anybody is [`NoMoreOftenThan`]'s.
pub trait TellsHowFarItHasGot {
    fn so_far(&self, bytes: u64, whole: Option<u64>);
}

/// **Nobody is told**: every caller that has no run's state to carry a count
/// on.
pub struct NobodyIsTold;

impl TellsHowFarItHasGot for NobodyIsTold {
    fn so_far(&self, _bytes: u64, _whole: Option<u64>) {}
}

/// **Y7b's *"no more often than the refresh cadence"*, held at the telling.**
///
/// **The first count goes at once**, so a step's figure is there from its
/// first piece; **every later one only once `every` has passed since the last
/// that went**. Counts in between are dropped rather than held: the next one
/// that goes is newer than any of them.
pub struct NoMoreOftenThan<'a> {
    every: std::time::Duration,
    last: std::cell::Cell<Option<std::time::Instant>>,
    to: &'a dyn TellsHowFarItHasGot,
}

impl<'a> NoMoreOftenThan<'a> {
    pub fn new(every: std::time::Duration, to: &'a dyn TellsHowFarItHasGot) -> NoMoreOftenThan<'a> {
        NoMoreOftenThan {
            every,
            last: std::cell::Cell::new(None),
            to,
        }
    }
}

impl TellsHowFarItHasGot for NoMoreOftenThan<'_> {
    fn so_far(&self, bytes: u64, whole: Option<u64>) {
        let now = std::time::Instant::now();
        if self.last.get().is_some_and(|last| now.duration_since(last) < self.every) {
            return;
        }
        self.last.set(Some(now));
        self.to.so_far(bytes, whole);
    }
}

/// **A folder one test owns alone, removed whole when it ends.**
///
/// **The two faults this exists to stop were both in this module on
/// 2026-09-19, and the runner found them by running the workspace four times
/// rather than once.** A test wrote its working files into a folder every other
/// test shares, **and told them apart by a clock reading**: `SystemTime::now()`
/// on the Creator's desk advances in 1,000-nanosecond steps and hands the same
/// value to several callers in a row, so two tests starting in the same
/// microsecond composed one path — one wrote, the other wrote, the first read
/// and removed, and the second's read found nothing there. **Measured outside
/// the repository: six threads asking at once composed the same name in 17
/// rounds of 200.**
///
/// **A fresh identity is the workspace's own answer** — sixteen random bytes,
/// the same mechanism `Rented-server runtime — program`'s own tests use — and
/// **the folder is the unit rather than the file**, so nothing a test makes can
/// land where another test is looking.
#[cfg(test)]
pub struct ItsOwnFolder {
    at: std::path::PathBuf,
}

#[cfg(test)]
impl ItsOwnFolder {
    pub fn at(&self) -> &std::path::Path {
        &self.at
    }
}

#[cfg(test)]
impl Drop for ItsOwnFolder {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.at).ok();
    }
}

/// **One such folder, made.**
#[cfg(test)]
pub fn its_own_folder(named: &str) -> ItsOwnFolder {
    let at = std::env::temp_dir().join(format!(
        "smoozy-setup-worker-{named}-{}",
        smoozy_state::identities::Identity::issue(
            smoozy_state::identities::IdentityKind::Operation
        )
        .expect("randomness")
        .as_text()
    ));
    std::fs::create_dir_all(&at).expect("the folder is made");
    ItsOwnFolder { at }
}

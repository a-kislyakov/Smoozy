//! §9 — taking the free-space reading, and the moment it is taken.
//!
//! **The shape is not here and the taking is.** `FreeSpaceReading`,
//! [`WhichRoute`] and [`WhyThereIsNoReading`] are composed by both ends — the
//! machine's own supervisor carries one in its health answer — so they sit in
//! the other half of this module and are re-exported below at the path they
//! have always been reached at. What is here drives the file-transfer
//! subsystem, which is this half's business.
//!
//! **F1: every free-space reading in this document is taken against an exact
//! path, and never against a machine.** Fact 28 is unambiguous: *"the machine's
//! free disk space is not a reading, it is a choice of which mount to read.
//! Reading by total size, or by 'the biggest', or by summing, all pick up
//! something that is not the machine's."* On one container a plain reading
//! returned **51 filesystems**, 48 of them memory-backed *"including one per
//! CPU thermal-throttle folder"*, and of the three that were not, one was **the
//! host's own 32 GiB disk bind-mounted in to deliver the driver files, showing
//! 20 GiB free the rental may not use**.
//!
//! **F1a: the per-path reading describes the layer the bytes actually land
//! on**, and it was honest at the boundary too. On a container with a 60 GB
//! allocation on a host holding far more, the subsystem's own reading returned
//! `f_frsize` 4096, `f_blocks` 15,728,640, `f_bavail` 15,728,451 — **exactly
//! the 60 GiB rented**, and not the host's 224 GB disk nor its 32 GB one. **And
//! on the machine deliberately filled, the reading read `0` free while the
//! machine's own tool read 100%** — so the figure does not lag or round at the
//! boundary, which is where a free-space reading is worth having at all.
//!
//! **F1b: on a container some paths are not on the rented layer at all, and
//! the reading names the path for that reason as well.** On the same box
//! `/etc/hosts`, `/etc/hostname` and `/etc/resolv.conf` were individually
//! bind-mounted from the host's disk, and the driver libraries read-only from
//! another of the host's partitions. **A write under one of those paths is not
//! on the rented layer**, and a reading taken against one describes the host.
//! **Nothing in this document writes outside the areas the placement made**, so
//! no route here reaches such a path; it is recorded so that nobody takes an
//! arbitrary path on a container to be the rental's.
//!
//! **F7a: a full disk is also logged by the machine and damage in the pipe is
//! not — the two are not equally silent, and nothing here is built on their
//! being so.** At the moment a filesystem filled, the machine's own log carried
//! `error: process_write: … No space left on device`; the corruption case
//! produced nothing at all. **Nothing here reads the machine's log** — it is
//! not on the surface §1.1 admits — and the asymmetry is recorded so that the
//! silence of the corruption case is never generalised.
//!
//! **F2 gives two routes in declared order, and both are per-path.** Route 1 is
//! the subsystem's own extension and is what is built here. **Route 2 is
//! §1.1's planted-link carriage, and it is not composed** — see
//! [`WhyThereIsNoReading::OnlyTheCarriageIsLeftAndItsProgramIsUnestablished`],
//! which carries the reason and what breaks.
//!
//! **Markers used below that belong to another document, each with the
//! document it belongs to.** `Servers` H10.

use crate::subsystem::client::Subsystem;
use crate::subsystem::extension;
use tokio::io::{AsyncRead, AsyncWrite};

/// **The reading's own shape, at the path it has always been reached at.**
///
/// It is composed on both sides of the connection, so it lives in the half of
/// this module that opens none. Nothing about it changed with the move.
pub use smoozy_framing::free_space::{
    FreeSpaceReading, WhichRoute, WhyThereIsNoReading,
    A_PER_PATH_ANSWER_IS_NEVER_WIDENED_INTO_A_PER_MACHINE_ONE,
};

/// **F2's reading, taken against an exact path.**
///
/// **Nothing derives a figure from a filesystem list** — picking the biggest,
/// summing, or taking the machine's — which is Fact 28's 51-row trap, and
/// nothing widens a per-path answer into a per-machine one (F16).
///
/// **F4: free space is never computed by subtracting listed file sizes.** A 500
/// GB sparse file *"sat in the measured folder and moved the free-space reading
/// by nothing."*
///
/// **F5: a writability test is not a reading and is not a signal.** On a full
/// filesystem *"the writability test returns writable, and creating an empty
/// file succeeds"* — an empty inode needs no data blocks — and *"Only writing
/// at least one byte fails"*.
pub async fn read_it<S: AsyncRead + AsyncWrite + Unpin + Send>(
    subsystem: &mut Subsystem<S>,
    path: &str,
) -> Result<FreeSpaceReading, WhyThereIsNoReading> {
    if !subsystem.announcement().announces(extension::STATVFS) {
        return Err(WhyThereIsNoReading::OnlyTheCarriageIsLeftAndItsProgramIsUnestablished);
    }
    match subsystem.filesystem_statistics(path.as_bytes()).await {
        Ok(statistics) => Ok(FreeSpaceReading {
            available_bytes: statistics.available_bytes(),
            the_path_it_was_taken_against: path.to_string(),
            route: WhichRoute::TheStatvfsExtension,
        }),
        Err(failure) => Err(WhyThereIsNoReading::TheServerAnswered(failure)),
    }
}

/// **Where the exact path does not exist yet, the reading is taken against its
/// deepest existing ancestor and the record names the path it was actually
/// taken against** (`Servers` H10).
///
/// **This walks upward and never sideways**, so no reading is ever taken
/// against a path the caller did not name a descendant of.
pub async fn read_it_at_the_deepest_ancestor_that_exists<S: AsyncRead + AsyncWrite + Unpin + Send>(
    subsystem: &mut Subsystem<S>,
    path: &str,
) -> Result<FreeSpaceReading, WhyThereIsNoReading> {
    let mut candidate = path.to_string();
    loop {
        match read_it(subsystem, &candidate).await {
            Ok(reading) => return Ok(reading),
            Err(WhyThereIsNoReading::TheServerAnswered(failure))
                if failure.is_the_path_not_being_there() =>
            {
                match ancestor_of(&candidate) {
                    Some(shorter) => candidate = shorter,
                    None => {
                        return Err(WhyThereIsNoReading::TheServerAnswered(failure));
                    }
                }
            }
            Err(other) => return Err(other),
        }
    }
}

/// One step up a path, or nothing at the root.
fn ancestor_of(path: &str) -> Option<String> {
    let trimmed = path.trim_end_matches('/');
    let cut = trimmed.rfind('/')?;
    if cut == 0 {
        if trimmed == "/" {
            None
        } else {
            Some("/".to_string())
        }
    } else {
        Some(trimmed[..cut].to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_walk_upward_stops_at_the_root() {
        assert_eq!(
            ancestor_of("/var/lib/smoozy/jobs"),
            Some("/var/lib/smoozy".to_string())
        );
        assert_eq!(ancestor_of("/var"), Some("/".to_string()));
        assert_eq!(ancestor_of("/"), None);
        assert_eq!(ancestor_of("relative/thing"), Some("relative".to_string()));
        assert_eq!(ancestor_of("relative"), None);
    }

    #[test]
    fn the_walk_never_goes_sideways() {
        // Every candidate is a prefix of the one before it, so no reading is
        // taken against a path the caller did not name a descendant of.
        let mut candidate = "/a/b/c/d".to_string();
        let mut seen = vec![candidate.clone()];
        while let Some(shorter) = ancestor_of(&candidate) {
            assert!(candidate.starts_with(&shorter));
            seen.push(shorter.clone());
            candidate = shorter;
        }
        assert_eq!(seen.last().map(String::as_str), Some("/"));
    }

    #[test]
    fn a_reading_names_the_path_it_was_taken_against() {
        // F1's requirement carried as a field rather than as a habit, so F16's
        // prohibition has something to hold on to. The shape is the other
        // half's and this reads it through the re-export, which is what says
        // the re-export is the same type.
        let reading = FreeSpaceReading {
            available_bytes: 64_423_735_296,
            the_path_it_was_taken_against: "/var/lib/smoozy/jobs".to_string(),
            route: WhichRoute::TheStatvfsExtension,
        };
        assert_eq!(reading.the_path_it_was_taken_against, "/var/lib/smoozy/jobs");
        assert!(A_PER_PATH_ANSWER_IS_NEVER_WIDENED_INTO_A_PER_MACHINE_ONE);
    }
}

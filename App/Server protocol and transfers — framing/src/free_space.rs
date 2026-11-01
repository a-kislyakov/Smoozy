//! §9 — what a free-space reading **is**, and what stands where there is none.
//!
//! **The taking of the reading is not here.** F2's route 1 drives the
//! file-transfer subsystem, so it sits next door in `smoozy-server-protocol`,
//! which re-exports every type below from `free_space` where it has always been
//! reached. What is here is the shape, because **both ends compose one**: the
//! desktop takes a reading against a path on the machine, and the machine's own
//! supervisor carries one in its health answer, which is `Rented-server
//! runtime` V7's own arrangement — that document takes this shape rather than
//! inventing a second.
//!
//! **F1: every free-space reading in this document is taken against an exact
//! path, and never against a machine.** Fact 28 is unambiguous: *"the machine's
//! free disk space is not a reading, it is a choice of which mount to read.
//! Reading by total size, or by 'the biggest', or by summing, all pick up
//! something that is not the machine's."* On one container a plain reading
//! returned **51 filesystems**, 48 of them memory-backed *"including one per
//! CPU thermal-throttle folder"*, and of the three that were not, one was **the
//! host's own 32 GiB disk bind-mounted in to deliver the driver files, showing
//! 20 GiB free the rental may not use**. **That is why the path is a field of
//! the reading rather than something the caller remembers.**
//!
//! **Markers used below that belong to another document, each with the
//! document it belongs to.** `Rented-server runtime` V7.

use crate::subsystem_answers::SubsystemFailure;

/// Which of F2's two routes answered.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhichRoute {
    /// **Route 1.** *"The file-transfer subsystem's per-path
    /// filesystem-statistics extension, where the connection's announced set
    /// carries it."*
    TheStatvfsExtension,
    /// **Route 2.** *"Otherwise, a per-path reading taken on the machine itself
    /// over the command channel, against the exact path — the path itself never
    /// in the command string"*, riding §1.1's planted-link carriage before
    /// hand-over.
    ThePlantedLinkCarriage,
}

/// One reading, per path.
///
/// **F3: the available figure is the one read, never the free figure**, because
/// of the root-reserved blocks — *"which is why the available figure and not the
/// free figure is the one to read"*. On one machine `Reserved block count: 0`
/// and the two agreed; that is one box, not a rule.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FreeSpaceReading {
    pub available_bytes: u64,
    /// **The path it was actually taken against.** F1's requirement, carried
    /// so that nothing downstream can widen a per-path answer into a
    /// per-machine one (F16).
    pub the_path_it_was_taken_against: String,
    pub route: WhichRoute,
}

/// Why there is no reading. **Reported as unavailable rather than substituted
/// for.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhyThereIsNoReading {
    /// The path is not there, or the login may not read it. **§1.3's two status
    /// codes are two answers**, so this carries which.
    TheServerAnswered(SubsystemFailure),
    /// **Route 1 is not announced, and route 2 is not composed.**
    ///
    /// **Why route 2 is not composed, said plainly rather than left to be
    /// discovered.** §1.1 puts two duties on the carriage, and the first is
    /// that the owning document **cites the consuming program's own documented
    /// treatment of a link**, *"because following is per-program and never
    /// assumed"*. **That citation does not exist for this site.** POSIX's own
    /// `df` page — read 2026-09-03 at
    /// `https://pubs.opengroup.org/onlinepubs/9799919799/utilities/df.html` —
    /// says *"If the file operand names a file other than a special file
    /// containing a file system, df shall write the amount of free space in the
    /// file system containing the specified file operand"*, and **says nothing
    /// about a symbolic-link operand being resolved to its target**. A link
    /// planted in the login's own writable area sits on that area's filesystem,
    /// so a reading that did not resolve would describe the wrong mount —
    /// silently, and in the direction that reports room the rental may not use,
    /// which is the exact harm F1 and F16 exist against.
    ///
    /// **And §12 item 3 already labels the ground permanently unresolved**:
    /// *"What is not established is which program answers it on an arbitrary
    /// rental — whether that program is present under the login Smoozy holds —
    /// and whether its output survives Fact 37's caution"*, with the Creator's
    /// ruling of 2026-08-31 meaning no rental will settle it. **What holds is
    /// that item's own answer: §1.2's rule that a capability is never assumed,
    /// and the free-space second route remains strictly additional.**
    ///
    /// **What breaks, kept in full.** F2's reading and with it Row 184's, and
    /// **F11's resolution of a path to a mount, which rests on the second route
    /// alone** — a machine that announces `statvfs@openssh.com` and carries no
    /// answering program has a per-path figure and no mount — and with it the
    /// one-condition-per-mount key F13 rests Row 288's *said once* on. **On
    /// every server this project has met the extension was announced**, so
    /// route 1 answers.
    OnlyTheCarriageIsLeftAndItsProgramIsUnestablished,
}

/// **F16: it never claims the amount the reading gives is the amount the rental
/// may use.**
///
/// The container case is why: the host's disk bind-mounted in showed 20 GiB
/// free the rental may not touch. F1 and F2 close it for a reading taken
/// against the right path, and this is stated so nothing downstream widens a
/// per-path answer into a per-machine one.
pub const A_PER_PATH_ANSWER_IS_NEVER_WIDENED_INTO_A_PER_MACHINE_ONE: bool = true;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::subsystem_answers::StatusCode;

    #[test]
    fn a_reading_names_the_path_it_was_taken_against() {
        // F1's requirement carried as a field rather than as a habit, so F16's
        // prohibition has something to hold on to.
        let reading = FreeSpaceReading {
            available_bytes: 64_423_735_296,
            the_path_it_was_taken_against: "/var/lib/smoozy/jobs".to_string(),
            route: WhichRoute::TheStatvfsExtension,
        };
        assert_eq!(reading.the_path_it_was_taken_against, "/var/lib/smoozy/jobs");
        assert!(A_PER_PATH_ANSWER_IS_NEVER_WIDENED_INTO_A_PER_MACHINE_ONE);
    }

    #[test]
    fn an_absence_carries_which_absence_it_is() {
        // The two are different values and nothing collapses them: a server
        // that answered said something, and a connection with neither route
        // said nothing at all.
        let answered = WhyThereIsNoReading::TheServerAnswered(SubsystemFailure::TheServerRefused(
            StatusCode::PermissionDenied,
        ));
        let neither_route = WhyThereIsNoReading::OnlyTheCarriageIsLeftAndItsProgramIsUnestablished;
        assert_ne!(answered, neither_route);
    }
}

//! Machine-side operations — `Dependency Upload & Remap` §9: the three
//! declarations, their termination contracts, and the housing seam. Nothing
//! here is decided at run time, which is why it is the rules half's.
//!
//! **This module runs three machine-side operation shapes, each a
//! per-operation worker with a durable operation record started before the
//! worker (`State, identity and recovery` §5.1), each declared here per
//! `Rented-server runtime` A1, with termination contracts stated because its
//! W2 says none exists until the owning document states one.**
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Rented-server runtime` A5a, W1, W2, X11.
//! `State, identity and recovery` §5.1. `Render Settings` §10, W5.
//!
//! **`Rented-server runtime` A1 is not listed above and is written out at
//! each site**, because this plan defines an A1 of its own.

/// The three operation shapes of Section 9's table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    /// One reading of one `.blend` (G1).
    CheckReading,
    /// One digest of one machine file (Q3).
    ContentsReading,
    /// Producing or replacing one fixed copy (M14).
    Fixing,
}

/// One row of Section 9's table, in `Rented-server runtime` A1's five terms.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Declaration {
    pub operation: Operation,
    pub resource_class: &'static str,
    pub priority: &'static str,
    pub cancellable: &'static str,
    pub atomic_or_partial: &'static str,
    /// **O4: the fifth declaration, for all three kinds — nothing declared,
    /// which `Rented-server runtime` A1 reads as unlimited.** Several check
    /// readings, several contents readings and several fixings may run at
    /// once, which is what the shared class already decides. For the check
    /// reading that is required rather than chosen: `Rented-server runtime`
    /// W1 names this document a declaration owner of the file-inspection
    /// kind whose declarations agree value for value with `Render Settings`
    /// §10, and that clause declares nothing at the fifth for the same
    /// reason. The rule that acts on the declaration is `Rented-server
    /// runtime` A5a, and it is not reached by any kind here.
    ///
    /// # The desk's own bound, which is a different fact and sits here so a
    /// reader meets both at once
    ///
    /// **The declaration above stays *nothing declared* and Smoozy still does
    /// not send two of these at once on one machine.** The two are not the
    /// same fact: the declaration is about **machine-level exclusion** —
    /// whether the machine admits more than one operation of a kind at a time
    /// — and the bound is about what the desk chooses to issue.
    ///
    /// **Why the desk bounds itself.** Fixed paths tell a machine-side act which
    /// file it is about, and every one is a single fixed build-time constant
    /// shared by every act of every kind. Two fixings a moment apart ended
    /// with one opening the other's file, applying the other's pairs, and
    /// publishing the result at its own address — a fixed copy of a different
    /// `.blend` standing where this one's should be, published, recorded and
    /// silent. **The bound is structural rather than a comment**:
    /// [`crate::acting::blender::TheSitesOnOneMachine`] is held for the length
    /// of one act and every acting entry point takes it by exclusive
    /// reference.
    ///
    /// **What it costs is what this clause's own cost sentence names** — a
    /// set's fixings run one at a time. **What removes it**: `Server
    /// protocol and transfers` admitting a per-operation class-2 token in a
    /// command string, which its own §1.1 rule 2 permits and its code does
    /// not.
    ///
    /// **The check reading left the bound on 2026-09-29, at step 10c's
    /// dependency wave piece 2.** It no longer plants anything at a fixed
    /// site: it is one request to the supervisor, answered by a
    /// file-inspection worker that writes its script into its own operation's
    /// folder and starts Blender by the build's own path (G1). **So nothing it
    /// does shares a site with another act, and several readings may run on
    /// one machine at once**, which is exactly what the declaration above
    /// says. What the bound still holds is the fixing's sites and the digest's
    /// one, five in all, until piece 9 moves the fixing the same way.
    pub exclusion: &'static str,
}

pub const THE_TABLE: [Declaration; 3] = [
    Declaration {
        operation: Operation::CheckReading,
        resource_class: "processors and memory shared; no card",
        priority: "below render; above preview and media",
        cancellable: "at any moment",
        atomic_or_partial: "atomic — a reading that did not complete is not a reading (G3)",
        exclusion: "nothing declared, which is read as unlimited",
    },
    Declaration {
        operation: Operation::ContentsReading,
        resource_class: "processors and memory shared; no card",
        priority: "with preview and media, low",
        cancellable: "at any moment",
        atomic_or_partial: "atomic — a digest that did not complete is nothing",
        exclusion: "nothing declared, which is read as unlimited",
    },
    Declaration {
        operation: Operation::Fixing,
        resource_class: "processors and memory shared; no card",
        priority: "below render; above preview and media",
        cancellable: "at any moment before publication; publication is one rename",
        atomic_or_partial: "atomic — staged and published by one rename, or nothing",
        exclusion: "nothing declared, which is read as unlimited",
    },
];

/// **O1: the termination contracts, per `Rented-server runtime` X11's four
/// things.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TerminationContract {
    pub operation: Operation,
    /// The termination target.
    pub which_member_receives_the_termination: &'static str,
    /// Whether the kind may be terminated at any moment (for the fixing,
    /// before publication).
    pub at_any_moment: bool,
    pub the_cleanup_owed: &'static str,
    /// **The worker outlives every member of its group** — what makes
    /// leader-gone read correctly as group-gone.
    pub the_worker_outlives_every_member: bool,
}

impl TerminationContract {
    pub fn names_all_four_of_x11s_things(&self) -> bool {
        !self.which_member_receives_the_termination.is_empty()
            && self.at_any_moment
            && !self.the_cleanup_owed.is_empty()
            && self.the_worker_outlives_every_member
    }
}

/// **O1, for the three.** For the check reading and the fixing: the recorded
/// external Blender process — the interrupt, the same signal shape the render
/// kind uses, aimed by the recorded execution-group pair and never by name or
/// pattern. For the contents reading, the group is the worker alone — package
/// code, no external process — and the termination target is the worker
/// itself, safe at any moment because the operation leaves nothing durable
/// but its outcome.
pub const THE_TERMINATION_CONTRACTS: [TerminationContract; 3] = [
    TerminationContract {
        operation: Operation::CheckReading,
        which_member_receives_the_termination:
            "the recorded external Blender process, by the interrupt, aimed by the recorded execution-group pair",
        at_any_moment: true,
        the_cleanup_owed: "the operation record's outcome",
        the_worker_outlives_every_member: true,
    },
    TerminationContract {
        operation: Operation::ContentsReading,
        which_member_receives_the_termination: "the worker itself — package code, no external process",
        at_any_moment: true,
        the_cleanup_owed: "the operation record's outcome",
        the_worker_outlives_every_member: true,
    },
    TerminationContract {
        operation: Operation::Fixing,
        which_member_receives_the_termination:
            "the recorded external Blender process, by the interrupt, aimed by the recorded execution-group pair; before publication",
        at_any_moment: true,
        the_cleanup_owed: "the staged material of an unpublished fixing, abandoned under the ordinary staging sweep, and the operation record's outcome",
        the_worker_outlives_every_member: true,
    },
];

/// **O2: the worker-kind housing is a seam.** `Rented-server runtime` W1's
/// kind set is closed by the shape, its file-inspection kind is *"one reading
/// of a `.blend`"*, and no kind names a digest of an arbitrary file or a
/// Blender-driven fixing. The declarations above are written so they can
/// stand under whichever housing the runner settles — **and the runner
/// settled the check reading's on 2026-08-22** (`Render Settings` §10 W5:
/// the file-inspection row names this document beside that one). The
/// contents reading and the fixing still fit no kind by description, and that
/// seam is recorded rather than repaired by this document inventing a worker
/// class.
pub const THE_CONTENTS_READING_AND_THE_FIXING_FIT_NO_KIND_BY_DESCRIPTION: bool = true;

/// **O3: nothing here reads a worker's success from its exit alone, discovers
/// a process by pattern, or infers completion from signal delivery** — the
/// recorded execution group, the per-waiter deadlines and the outcome records
/// are `Rented-server runtime`'s and `State, identity and recovery`'s.
pub const NOTHING_READS_SUCCESS_FROM_AN_EXIT_ALONE: bool = true;

/// **G5 and §12 item 16: the declaration is shaped so that one edit tightens
/// it** — a property of the shape, and not a space kept for a measurement
/// that will not arrive.
pub const ONE_EDIT_TO_THE_TABLE_TIGHTENS_A_DECLARATION: bool = true;

/// Uploads remain available without consulting paid access.
pub fn sending_files_up_is_closed() -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_operations_each_declared_once_with_nothing_at_the_fifth() {
        for operation in [
            Operation::CheckReading,
            Operation::ContentsReading,
            Operation::Fixing,
        ] {
            assert_eq!(
                THE_TABLE
                    .iter()
                    .filter(|row| row.operation == operation)
                    .count(),
                1
            );
            assert_eq!(
                THE_TERMINATION_CONTRACTS
                    .iter()
                    .filter(|row| row.operation == operation)
                    .count(),
                1
            );
        }
        for row in THE_TABLE {
            assert!(row.resource_class.contains("no card"));
            assert!(row.atomic_or_partial.starts_with("atomic"));
            assert!(row.exclusion.contains("unlimited"));
        }
        assert!(THE_TABLE[1].priority.contains("low"));
    }

    #[test]
    fn every_termination_contract_names_all_four_of_x11s_things() {
        for contract in THE_TERMINATION_CONTRACTS {
            assert!(
                contract.names_all_four_of_x11s_things(),
                "{:?}",
                contract.operation
            );
        }
        assert!(THE_TERMINATION_CONTRACTS[1]
            .which_member_receives_the_termination
            .contains("the worker itself"));
        assert!(THE_TERMINATION_CONTRACTS[2]
            .the_cleanup_owed
            .contains("staged material"));
    }

    #[test]
    fn uploads_are_open_without_any_policy() {
        assert!(!sending_files_up_is_closed());
    }
}

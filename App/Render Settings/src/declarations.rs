//! The file-inspection worker kind — `Render Settings` §10 W1–W5: its
//! declarations and its termination contract.
//!
//! **Constraint:** `Rented-server runtime` W1 — the kind is *one reading of a
//! `.blend`*, declarations owned by `Render Settings` and `Job files and
//! outputs`; and `Rented-server runtime` W2 — no termination contract exists until an owning
//! document states one. **This document is the last named owner, and this
//! file is the declaration.** Nothing here is decided at run time, which is
//! why it is the rules half's.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Rented-server runtime` A1, A3, A4, A6, A12, A13,
//! X1, X6, X9, X11, X12. `Dependency Upload & Remap` G1, G3, O2. `Rendering`
//! P1. `Job files and outputs` R17.
//!
//! **`Rented-server runtime` W1 and W2 are not listed above and are written
//! out at each site**, because this plan defines a W1 and a W2 of its own and
//! a bare one could mean either.

/// One row of W1's table, in `Rented-server runtime` A1's own terms.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Declared {
    pub declaration: &'static str,
    pub value: &'static str,
}

/// **W1: the declarations, against `Rented-server runtime` A1, none decided at
/// run time.** A1 took four things until 2026-08-22 and now takes five; the
/// fifth is machine-level exclusion of the kind's own kind, and **this kind
/// declares nothing there, which A1 reads as unlimited** — several readings
/// of different files may run at once.
pub const THE_DECLARATIONS: [Declared; 5] = [
    Declared {
        declaration: "resource class",
        value: "the processors and the machine's memory, shared; no card. A reading drives a headless Blender that renders nothing",
    },
    Declared {
        declaration: "priority",
        value: "below render; above preview and media. A reading gates something a person is waiting on, where a derivative is a byproduct; and the render stays first because it is the paid work",
    },
    Declared {
        declaration: "cancellable",
        value: "yes, at any moment. Nothing durable is half-written at any point: the reading is consumed only whole (Q7), so ending it anywhere loses a reading and corrupts nothing",
    },
    Declared {
        declaration: "atomic or partial",
        value: "atomic — a reading that did not complete is not a reading, the sentinel-framed document being the unit (Q7; the same rule as `Dependency Upload & Remap` G3)",
    },
    Declared {
        declaration: "exclusion",
        value: "nothing declared, which `Rented-server runtime` A1 reads as unlimited — several readings of different files may run at once, which is what the shared class already decides",
    },
];

/// **W2: the termination contract, `Rented-server runtime` X11's four named
/// things.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TheTerminationContract {
    /// **Which member receives the termination: the recorded external
    /// Blender process — the interrupt, aimed by the recorded execution-group
    /// pair, never anything found by name, pattern or process search**
    /// (`Rented-server runtime` X6, X12's shape). The same signal the render
    /// kind uses; what an interrupt leaves half-done inside a read does not
    /// matter, because an ended read is no reading.
    pub which_member_receives_the_termination: &'static str,
    /// **Terminable at any moment** — no safe boundary exists or is needed.
    pub at_any_moment: bool,
    /// **Cleanup owed: the operation record's outcome and nothing else.** A
    /// reading stages nothing, publishes nothing and writes no durable
    /// material until its document has been returned whole and consumed by
    /// its caller; the captured streams are retained under the ordinary
    /// rules.
    pub the_cleanup_owed: &'static str,
    /// **The worker outlives every member of its group.** It starts the one
    /// Blender, holds both streams to their end, reaps it, and reports; it
    /// never exits before the process it started. **Declared outright, because
    /// it is the one thing `Rented-server runtime` X1's leader-only recording
    /// and X9's leader-gone-means-group-gone are unsound without.**
    pub the_worker_outlives_every_member: bool,
}

impl TheTerminationContract {
    /// **`Rented-server runtime` X11 names four things and W2 answers every
    /// one**: a member that receives the termination, terminable at any
    /// moment, a cleanup owed, and the worker outliving every member — the
    /// last being the one X1 and X9 are unsound without.
    pub fn names_all_four_of_x11s_things(&self) -> bool {
        !self.which_member_receives_the_termination.is_empty()
            && self.at_any_moment
            && !self.the_cleanup_owed.is_empty()
            && self.the_worker_outlives_every_member
    }
}

pub const THE_TERMINATION_CONTRACT: TheTerminationContract = TheTerminationContract {
    which_member_receives_the_termination:
        "the recorded external Blender process, by the interrupt, aimed by the recorded execution-group pair",
    at_any_moment: true,
    the_cleanup_owed: "the operation record's outcome, and nothing else",
    the_worker_outlives_every_member: true,
};

/// **W3: the group is the worker and the one Blender it starts**, recorded as
/// `Rented-server runtime` X1's pair at the start. **The render-start reading
/// is not a separate group** — it runs inside the admitted render operation
/// as a step of the render worker (`Rendering` P1 step 4), contending with
/// nothing — and this kind's declarations govern the reading only where it
/// is its own operation: a pre-load, and whatever else is housed under the
/// kind. Q4's desktop-driven pre-load is admitted at the machine's one
/// admission point exactly as `Dependency Upload & Remap` G1 admits its check
/// reading (`Rented-server runtime` A4's argument).
pub const THE_GROUP_IS_THE_WORKER_AND_THE_ONE_BLENDER_IT_STARTS: bool = true;

/// **W4: the declaration is made against what stands, and what is untested
/// is named rather than resolved by assumption.** Every read taken beside a
/// running render completed correctly, every time; **the magnitude of the
/// disturbance is untested** (`Job files and outputs` R17, `Rented-server
/// runtime` A12) and **how many Blenders one machine will carry at once is
/// settled nowhere**. So the kind is declared shared-and-below-render today,
/// no concurrent-reading cap is asserted anywhere, and `Rented-server runtime`
/// A13's tightening path holds: **one edit to [`THE_DECLARATIONS`]** — to
/// *never running beside a render*, or to a stated concurrency bound — moves
/// nothing in the mechanism. §13 items 1 and 2.
pub const NO_CONCURRENT_READING_CAP_IS_ASSERTED: bool = true;

/// **W5: ruled 2026-08-22 at the Architecture seam close — the runner widened
/// `Rented-server runtime` W1's file-inspection row to name this section and
/// `Dependency Upload & Remap` §9 together, and its termination column names
/// W2 above.** Two agreeing declarations merged into one kind. **Nothing in
/// this section moved for it**, and `Dependency Upload & Remap` O2's own
/// wording of the housing is that document's and is not arbitrated here.
pub const THE_HOUSING_IS_RULED_CLOSED: bool = true;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_five_declarations_are_the_plans_and_the_kind_takes_no_card() {
        let names: Vec<&str> = THE_DECLARATIONS.iter().map(|row| row.declaration).collect();
        assert_eq!(
            names,
            vec![
                "resource class",
                "priority",
                "cancellable",
                "atomic or partial",
                "exclusion"
            ]
        );
        assert!(THE_DECLARATIONS[0].value.contains("no card"));
        assert!(THE_DECLARATIONS[1].value.starts_with("below render"));
        assert!(THE_DECLARATIONS[2].value.starts_with("yes, at any moment"));
        assert!(THE_DECLARATIONS[3].value.starts_with("atomic"));
        assert!(THE_DECLARATIONS[4].value.contains("unlimited"));
    }

    #[test]
    fn the_termination_contract_names_all_four_of_x11s_things() {
        assert!(THE_TERMINATION_CONTRACT.names_all_four_of_x11s_things());
        assert!(THE_TERMINATION_CONTRACT
            .which_member_receives_the_termination
            .contains("recorded external Blender process"));
        assert!(THE_TERMINATION_CONTRACT
            .the_cleanup_owed
            .contains("operation record"));
    }

    /// **W5's ruling, read off the runtime crate rather than asserted here**:
    /// its own owner row for the file-inspection kind names this document
    /// beside `Dependency Upload & Remap`, which is the merge of two agreeing
    /// declarations into one kind.
    #[test]
    fn the_runtime_names_this_document_as_an_owner_of_the_kinds_declarations() {
        use smoozy_runtime::workers::WorkerKind;
        let owners = WorkerKind::FileInspection.declarations_owned_by();
        assert!(owners.contains(&"Render Settings"), "{owners:?}");
        assert!(owners.contains(&"Dependency Upload & Remap"), "{owners:?}");
    }
}

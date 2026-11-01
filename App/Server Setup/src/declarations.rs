//! Declarations — `Server Setup` §11, and the capability read at this module's
//! boundary.
//!
//! **The constraint** is `Rented-server runtime` §7.1's declarations and X11's
//! four named things, **owed by this document for the setup kind**.
//! `Rented-server runtime` A1 took **five** declarations on 2026-08-22, the fifth being
//! machine-level exclusion of an operation's own kind — **added at this
//! document's own asking** (`Server Setup` §13 item 15) — and the table below
//! carries it as its last row.
//!
//! **Markers used below that belong to another document, each with the document
//! it belongs to.** `Rented-server runtime` §7.1, §9.2, A11, G20, P2,
//! X1, X9, X11, X14. `State, identity and recovery` §9.2.
//!
//! **`Rented-server runtime` A1 and A5 are not listed above and are written out
//! at each site**, because this plan defines an A1 and an A5 of its own and a
//! bare one could mean either.

/// One row of §11's table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Declared {
    /// The declaration, in `Rented-server runtime` §7.1's own terms.
    pub declaration: &'static str,
    /// The value, in this document's own words.
    pub value: &'static str,
}

/// **§11's table, entire. It is a declaration and not a computation**: nothing
/// here is decided at run time.
pub const THE_DECLARATIONS: [Declared; 5] = [
    Declared {
        declaration: "resource class",
        value: "processors and the machine's memory **shared**; no card. The machine-changing run's real exclusion is the setup-in-progress state, which holds every admission at machine level and needs no card class to do it",
    },
    Declared {
        declaration: "priority",
        value: "below the render kind; above nothing else",
    },
    Declared {
        declaration: "cancellable",
        value: "at a recorded safe boundary only; never at an arbitrary moment",
    },
    Declared {
        declaration: "atomic or partial",
        value: "partial across steps, atomic within a step — the spec promises per-step outcomes and no whole-run outcome, and each step's own material moves by staging and one publication",
    },
    Declared {
        declaration: "exclusion",
        value: "one setup run per machine, enforced at the admission point; per machine, never per app",
    },
];

/// **§11: the resource class takes no card, and the contended resources stay
/// closed at three.**
///
/// **Making either shared resource exclusive would have excluded renders too**,
/// which is not what [`crate::driving`]'s one-run-at-a-time rule says — and that
/// is exactly what stopped the exclusion being expressible before
/// `Rented-server runtime` A1 took a fifth declaration.
pub const THE_SETUP_KIND_TAKES_NO_CARD: bool = true;

/// **The termination contract's four named things** (`Rented-server runtime`
/// X11), owed by this document for the setup kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TheTerminationContract {
    /// **Which member receives the termination — none.** The setup kind takes
    /// no signal; **termination is the durable *should be stopped* honoured at
    /// the next boundary**, the same no-signal shape as transfer-support's
    /// (`Rented-server runtime` X14).
    pub which_member_receives_the_termination: Option<&'static str>,
    /// **At a boundary only**, never at any moment.
    pub at_any_moment: bool,
    /// **What cleanup is owed** — the run's staged material at the locations
    /// its operation record names, **swept on the next setup run's start under
    /// `Rented-server runtime` G20's bounded shape**; the fetch staging is kept
    /// while its record says a resume is possible.
    pub the_cleanup_owed: &'static str,
    /// **Whether the worker outlives every member of its group — yes,
    /// declared.** The setup worker remains until every external process it
    /// started has exited, or is ended only after them, **which is what makes
    /// `Rented-server runtime` X1's leader-only recording and X9's
    /// leader-gone-means-group-gone sound for this kind.**
    pub the_worker_outlives_every_member: bool,
}

/// **X11's four, as one value.**
pub const THE_TERMINATION_CONTRACT: TheTerminationContract = TheTerminationContract {
    which_member_receives_the_termination: None,
    at_any_moment: false,
    the_cleanup_owed:
        "the run's staged material at the locations its operation record names, swept on the next setup run's start; the fetch staging is kept while its record says a resume is possible",
    the_worker_outlives_every_member: true,
};

/// The setup action being composed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhichAct {
    APreparationOrUserFacingAct,
    ThePackageRepair,
}

#[derive(Debug)]
pub struct SettingAMachineUpWasOpen(());

/// Both setup and package repair remain available without paid access.
pub fn the_act_may_be_composed(
    _which: WhichAct,
) -> Result<SettingAMachineUpWasOpen, std::convert::Infallible> {
    Ok(SettingAMachineUpWasOpen(()))
}

/// **`State, identity and recovery` §9.2's partition unit for this module's own
/// acts**, which §11's *atomic or partial* row already states: **partial across
/// steps, atomic within a step.**
///
/// **§9.2's test 1 decided it**: the spec promises per-step outcomes — a step is
/// reported, resumed and re-run singly — **and no whole-run outcome**; each
/// step's own material moves by staging and one publication.
pub const THE_PARTITION_UNIT: &str = "one step of a setup run";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_five_declarations_are_the_table_and_the_fifth_is_the_exclusion() {
        assert_eq!(THE_DECLARATIONS.len(), 5);
        assert_eq!(THE_DECLARATIONS[4].declaration, "exclusion");
        assert!(THE_DECLARATIONS[4].value.contains("per machine, never per app"));
        assert!(THE_DECLARATIONS[0].value.contains("no card"));
        const { assert!(THE_SETUP_KIND_TAKES_NO_CARD) };
    }

    #[test]
    fn the_setup_kind_takes_no_signal_and_is_cancellable_at_a_boundary_only() {
        assert_eq!(
            THE_TERMINATION_CONTRACT.which_member_receives_the_termination,
            None
        );
        const { assert!(!THE_TERMINATION_CONTRACT.at_any_moment) };
        const { assert!(THE_TERMINATION_CONTRACT.the_worker_outlives_every_member) };
        assert!(THE_TERMINATION_CONTRACT
            .the_cleanup_owed
            .contains("next setup run's start"));
    }

    #[test]
    fn setup_is_open_without_any_policy() {
        assert!(the_act_may_be_composed(WhichAct::APreparationOrUserFacingAct).is_ok());
    }

    #[test]
    fn the_fallback_re_run_and_the_package_repair_stay_open_by_p3s_own_words() {
        assert!(the_act_may_be_composed(WhichAct::ThePackageRepair).is_ok());
    }

    #[test]
    fn the_partition_unit_is_one_step_of_a_run() {
        assert_eq!(THE_PARTITION_UNIT, "one step of a setup run");
        assert!(THE_DECLARATIONS[3].value.contains("partial across steps"));
    }
}

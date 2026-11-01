//! What of this document is not built here, named rather than silently absent.
//!
//! **This list is empty, and it was six entries long until step 7 — six
//! entries over seven sections, §5 and §6 sharing one.**
//! `Preparation/Build/Plan.md` step 4 built the directory layout, the manifest
//! and the publication contract, and held the rest back because *"`Job files
//! and outputs`' recorded reading, its expected-output inventory, its previews
//! and its reckoning all need a Blender on a machine"*. **All of it landed at
//! step 7**: §4 the recorded reading, §5 the redirect and its launch material,
//! §6 the rule that brings a destination under the job's folder, §7 the
//! expected-output inventory, §8 the main-output walk and the derivatives, §9
//! the three holds, and §10 the reckoning. *(Clarified 2026-09-07: this said
//! six and then listed seven, with nothing saying why the two numbers differ.
//! Both are right and the pairing is what was missing.)*
//!
//! **The two fields that carried an honest statement of absence now carry the
//! thing itself.** [`crate::manifest::TheRecordedReading`] carries the reading
//! and everything derived from it, and [`crate::attempts::TheMapInForce`]
//! carries the map. Both keep their *there is none* state, because R7's second
//! bullet is an ordinary case — a job may be submitted without the file having
//! been read — and never a stub.
//!
//! **What is outside this crate is outside it by the plan's own division, and is
//! listed at [`WHAT_ANOTHER_DOCUMENT_CARRIES_OUT`] so that a reader meets it
//! rather than discovering it.** None of it is a part of this document left
//! unbuilt.

/// One part of the document that is not built here.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NotBuiltHere {
    /// The plan's own section.
    pub section: &'static str,
    /// What it is.
    pub what: &'static str,
    /// Why it cannot be built at step 4.
    pub why: &'static str,
    /// The step of `Preparation/Build/Plan.md` it lands with.
    pub it_lands_at_step: u8,
}

/// **Every part of this document is built. The list is empty and that is the
/// point of keeping it.**
pub const EVERYTHING_ARRIVING_LATER: [NotBuiltHere; 0] = [];

/// One thing this document decides and another document carries out.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CarriedOutElsewhere {
    /// The clause of this plan that decides it.
    pub decided_at: &'static str,
    /// What is decided here.
    pub what: &'static str,
    /// The document whose job it is to carry it out, and why it is theirs.
    pub carried_out_by: &'static str,
}

/// **What this document decides and another document carries out.**
///
/// **This is a division the plan makes, not a gap.** §13 lists what this
/// document supplies and to which document, and every row below is one of those
/// supplies seen from this side.
pub const WHAT_ANOTHER_DOCUMENT_CARRIES_OUT: [CarriedOutElsewhere; 5] = [
    CarriedOutElsewhere {
        decided_at: "R1",
        what: "taking the one reading — one invocation of the Blender the manifest names, \
               headless, over the command channel, returning one document between sentinels",
        // The reading runs as a worker of the file-inspection kind, whose
        // declarations `Rented-server runtime` W1 gives to `Render Settings`
        // §10 and `Dependency Upload & Remap` §9. What is built here is what
        // the invocation names and what the document that comes back holds.
        carried_out_by: "`Rented-server runtime`'s worker start, on the file-inspection kind",
    },
    CarriedOutElsewhere {
        decided_at: "D6",
        what: "handing the launch material to Blender, and the launch line it sits on",
        carried_out_by: "`Rendering`'s launch contract, which D6 names outright",
    },
    CarriedOutElsewhere {
        decided_at: "H3",
        what: "holding a job on one of §9.1's three conditions, and releasing it",
        carried_out_by: "`Rendering` for the act, and `Rented-server runtime`'s machine-held \
                         answer-release path for the release",
    },
    CarriedOutElsewhere {
        decided_at: "B23",
        what: "producing a derivative, by the worker driving the Blender the manifest names",
        carried_out_by: "`Rented-server runtime`'s preview worker, whose arbitration \
                         declaration B24 gives it and whose `Rented-server runtime` A2 already carries",
    },
    CarriedOutElsewhere {
        decided_at: "B29",
        what: "publishing a derivative by a rename from its staging path",
        carried_out_by: "`Server protocol and transfers` §7.5, taken entire",
    },
];

/// **Nothing here is stubbed, and this is the difference stated plainly.**
///
/// A stub answers. **There is no always-yes answer, no empty map standing in for
/// a map, and no default anywhere in this crate.** Where a value is not
/// established the field carries a statement a reader can tell from a value:
/// [`crate::inventory::WhatTheInventoryIs::TheFrameSetIsNotPredictable`],
/// [`crate::main_output::NodeKind::ANodeTheWalkCannotPass`],
/// [`crate::main_output::WhatTheWalkFound::Blocked`] and
/// [`crate::mapping::HowItMapped::NotMappable`] are four of them, and each names
/// its reason.
pub const NOTHING_HERE_IS_A_STUB: bool = true;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_unbuilt_part_names_its_step_and_its_reason() {
        // The list is empty. **The loop is kept rather than deleted** so that a
        // part added back later cannot be added without its step and its
        // reason.
        for part in EVERYTHING_ARRIVING_LATER {
            assert!(!part.what.is_empty());
            assert!(!part.why.is_empty());
            assert!((6..=8).contains(&part.it_lands_at_step));
        }
        assert!(
            EVERYTHING_ARRIVING_LATER.is_empty(),
            "a part of this document is unbuilt and the crate's own header says otherwise"
        );
    }

    #[test]
    fn everything_carried_out_elsewhere_names_the_clause_and_the_document() {
        for row in WHAT_ANOTHER_DOCUMENT_CARRIES_OUT {
            assert!(!row.decided_at.is_empty());
            assert!(!row.what.is_empty());
            assert!(
                row.carried_out_by.contains('`'),
                "a row names no document: {row:?}"
            );
        }
    }
}

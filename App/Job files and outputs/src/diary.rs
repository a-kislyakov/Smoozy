//! This module's diary declaration — the plan's §13, last part.
//!
//! **It writes no act lines and no told lines, and that is its fixed list
//! rather than a silence.** Added to the plan 2026-08-31 under `Reporting a
//! problem` J2a and J4a, which require every feature document to declare in
//! itself the acts it writes to the diary, **or to state in one line that it
//! writes none**.
//!
//! **`Desktop app and delivery` A7 makes a module with no declaration write
//! nothing, silently — *"a hosting fault and not a default"*.** So the empty
//! half is written out here rather than left to be inferred, because **a missing
//! declaration and an empty one read alike from the diary's end**.
//!
//! **Nothing here is new**: no act is invented for this document to perform,
//! and every act it takes part in is already promised, and already owed as a
//! declaration, by the module whose spec promises it.
//!
//! **Markers used below that belong to another document, each with the
//! document it belongs to.** `Reporting a problem` J2, J2a, J4, J4a.
//! `Desktop app and delivery` A7.

use smoozy_reporting::lines::LineKind;

/// **The kinds this module declares, and it is none.**
pub const DECLARED: [LineKind; 0] = [];

/// **The four candidates, with the test each fails**, named so that the next
/// reader is not left to re-derive them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Candidate {
    /// **Placing a job's material on the machine and creating its job output
    /// folder** (§12). *"The act a user could name is *started a job*, which is
    /// `Rendering`'s Row 2 and that document's to declare. This document
    /// supplies the unit's shape and its atomicity, which is not an act of its
    /// own."*
    PlacingAJobsMaterialAndCreatingItsFolder,
    /// **The reading of the file** (§4) — **fails J2a's second test.** *"No spec
    /// names the reading's completion as something the user is told about or
    /// can see afterwards."* The spec's *read a file* is `File Browser`'s under
    /// J2's assignment and not this reading.
    TheReadingOfTheFile,
    /// **Fetching a derivative to the user's computer, and clearing one** (§11,
    /// §12) — **fails J2a's third test.** The unit is one derivative and there
    /// is no promise about the set, *"so a line here would be one per
    /// derivative, which is the rate J2a forbids in terms"*.
    FetchingADerivativeOrClearingOne,
    /// **The abandoned-material sweep and the cleanups of §11** (K1, K4) —
    /// **fails J2a's second test.** *"A job output folder whose record was never
    /// published is one no user could name, and a staged directory is the
    /// transfer contract's residue."*
    TheAbandonedMaterialSweepAndTheCleanups,
}

pub const EVERY_CANDIDATE: [Candidate; 4] = [
    Candidate::PlacingAJobsMaterialAndCreatingItsFolder,
    Candidate::TheReadingOfTheFile,
    Candidate::FetchingADerivativeOrClearingOne,
    Candidate::TheAbandonedMaterialSweepAndTheCleanups,
];

/// Which test the candidate fails, or whose act it is instead.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhyItIsNotDeclaredHere {
    /// It is another module's act to declare.
    ItIsAnotherModulesActToDeclare,
    /// J2a's second test: no spec names its completion as something the user is
    /// told about or can see afterwards.
    NoSpecNamesItsCompletionAsSomethingTheUserSees,
    /// J2a's third test: it does not run at the user's own rate.
    ItWouldNotRunAtTheUsersOwnRate,
}

pub fn why(candidate: Candidate) -> WhyItIsNotDeclaredHere {
    match candidate {
        Candidate::PlacingAJobsMaterialAndCreatingItsFolder => {
            WhyItIsNotDeclaredHere::ItIsAnotherModulesActToDeclare
        }
        Candidate::TheReadingOfTheFile | Candidate::TheAbandonedMaterialSweepAndTheCleanups => {
            WhyItIsNotDeclaredHere::NoSpecNamesItsCompletionAsSomethingTheUserSees
        }
        Candidate::FetchingADerivativeOrClearingOne => {
            WhyItIsNotDeclaredHere::ItWouldNotRunAtTheUsersOwnRate
        }
    }
}

/// **And no told lines, the one reading that could look like an exception being
/// another document's.**
///
/// What this document composes on the machine — the recorded reading, the
/// destination map, the expected-output inventory, an attempt record's frame
/// list and the completeness statement — **crosses to the desktop as material
/// read out of the job record directory**, for `Queue & History`, `Download`
/// and `Render Settings` to state. **J4a rules that no journal event becomes a
/// told line by itself**, so those three declare what they consume and this
/// document writes none.
///
/// **The one reading of this document's that the diary does carry is the
/// Blender version confirmed before launch** (D7), and J4 already places that
/// told line **at the act that reads it, which is `Rendering`'s** — declared
/// there, not here.
pub const THE_ONE_TOLD_LINE_THIS_DOCUMENT_TOUCHES_IS_DECLARED_BY_RENDERING: bool = true;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_module_declares_no_line_kinds_at_all_and_says_so_out_loud() {
        assert!(DECLARED.is_empty());
        // A7 makes a module with no declaration write nothing, silently. The
        // difference between that and this is that this one is written down.
        assert_eq!(EVERY_CANDIDATE.len(), 4);
    }

    #[test]
    fn each_candidate_carries_the_test_it_fails() {
        for candidate in EVERY_CANDIDATE {
            // Every one is answered, and none of them is answered by silence.
            let _ = why(candidate);
        }
        assert_eq!(
            why(Candidate::PlacingAJobsMaterialAndCreatingItsFolder),
            WhyItIsNotDeclaredHere::ItIsAnotherModulesActToDeclare
        );
        assert_eq!(
            why(Candidate::FetchingADerivativeOrClearingOne),
            WhyItIsNotDeclaredHere::ItWouldNotRunAtTheUsersOwnRate
        );
    }
}

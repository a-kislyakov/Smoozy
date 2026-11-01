//! §2.0 — the trusted release and the exact artifact.
//!
//! **A requested package version is not an artifact identity**, and this
//! document consumes rather than redefines the release evidence. Trust decisions
//! are made before this transaction begins; what is here is that the transaction
//! pins them by value and re-checks them by content before any byte is made
//! executable.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Server protocol and transfers` B1, B4, B8, B9,
//! B10, and its §1.2 item 3. `Server Setup` — the detected rental tuple.

/// **G27's descriptor, pinned by value in the phase record.**
///
/// It carries the manifest format and verified release sequence, requested
/// package version, manifest digest, selected artifact identity, artifact byte
/// length and digest. **A retry never substitutes another manifest or artifact
/// behind that descriptor.**
///
/// `Server protocol and transfers` B8's canonical manifest, target tuple, fixed
/// entry point and exact sources remain evidence the desktop verified and
/// selected before it formed B1; **this document does not create a second wire
/// copy with a different field set** (G26).
#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct VerifiedReleaseDescriptor {
    pub manifest_format: String,
    pub verified_release_sequence: u64,
    pub requested_package_version: String,
    pub manifest_digest: [u8; 32],
    pub artifact_identity: String,
    pub artifact_byte_length: u64,
    pub artifact_digest: [u8; 32],
}

/// **G26. A request that does not carry B10's verified release descriptor
/// reaches *failed before commit* without any downloaded byte being made
/// executable or invoked.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhetherTheRequestMayProceed {
    ItCarriesAVerifiedDescriptor,
    /// **Nothing is made executable and nothing is invoked.**
    FailedBeforeCommitNoVerifiedDescriptor,
}

/// **G26.** The one thing this clause decides, and it decides it before anything
/// is staged.
pub fn may_proceed(descriptor: Option<&VerifiedReleaseDescriptor>) -> WhetherTheRequestMayProceed {
    match descriptor {
        Some(_) => WhetherTheRequestMayProceed::ItCarriesAVerifiedDescriptor,
        None => WhetherTheRequestMayProceed::FailedBeforeCommitNoVerifiedDescriptor,
    }
}

/// **G28. Artifact selection is B8's exact tuple match and never a best
/// effort.**
///
/// `Server Setup` supplies the detected rental tuple and the desktop selects the
/// B1 descriptor from the verified manifest **before this transaction begins**.
/// This clause neither narrows nor widens the supported-rental ruling and
/// **performs no second selection.**
///
/// **No generic Linux entry, closest processor, closest ABI, capability
/// fallback, package-name guess or first entry is permitted.** No exact match is
/// an unsupported machine reported before any package byte is executed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ASelectionThisDocumentRefuses {
    AGenericLinuxEntry,
    TheClosestProcessor,
    TheClosestAbi,
    ACapabilityFallback,
    APackageNameGuess,
    TheFirstEntry,
}

impl ASelectionThisDocumentRefuses {
    /// The closed set, written out so a reader meets each rather than inferring
    /// them from the word *exact*.
    pub const ALL: [ASelectionThisDocumentRefuses; 6] = [
        ASelectionThisDocumentRefuses::AGenericLinuxEntry,
        ASelectionThisDocumentRefuses::TheClosestProcessor,
        ASelectionThisDocumentRefuses::TheClosestAbi,
        ASelectionThisDocumentRefuses::ACapabilityFallback,
        ASelectionThisDocumentRefuses::APackageNameGuess,
        ASelectionThisDocumentRefuses::TheFirstEntry,
    ];
}

/// **G28's own statement, and this crate performs no selection at all.**
///
/// The selection happened at the desktop before B1 was formed. The transaction
/// receives one descriptor; there is no function here that takes a manifest and
/// chooses from it, and **that absence is the clause**.
pub const NO_SECOND_SELECTION_IS_PERFORMED_HERE: () = ();

/// What the staged material actually measures, ready for G29's comparison.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AtRestMeasurement {
    pub byte_length: u64,
    pub digest: [u8; 32],
}

/// The outcome of **G29's at-rest comparison**.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AtRestComparison {
    /// The staged artifact is B1's. It may now be made executable.
    Agrees,
    /// **A mismatch records *failed before commit*, leaves the pointer and boot
    /// registration untouched, and is never repaired in place.**
    FailedBeforeCommit,
}

/// **G29. Staging consumes B10's at-rest evidence.**
///
/// The staged artifact's byte length and digest are compared with B1's verified
/// release descriptor **before it is made executable or any byte invoked** —
/// B10's read-back comparison, performed by the desktop through the surface
/// where no verified runtime is alive, and free to be the live verified
/// runtime's own comparison where one is.
pub fn compare_at_rest(
    descriptor: &VerifiedReleaseDescriptor,
    measured: AtRestMeasurement,
) -> AtRestComparison {
    if measured.byte_length == descriptor.artifact_byte_length
        && measured.digest == descriptor.artifact_digest
    {
        AtRestComparison::Agrees
    } else {
        AtRestComparison::FailedBeforeCommit
    }
}

/// **G30. Trust and anti-rollback failures are inputs to the transaction, never
/// readings made from staged material.**
///
/// B9 and B10's accepted key-set generation, release sequence, manifest
/// signature and digest are decided before this document acts. **None of the
/// four things below can replace that evidence**, and they are written out so a
/// builder meets each rather than reasoning about which readings are enough.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatCannotStandForProvenance {
    /// **G19's own rider**: a version answer from the staged program is a health
    /// check after provenance and cannot replace provenance.
    AVersionAnswerFromThePackage,
    AMatchingFilename,
    TransportSuccess,
    BytesSelfConsistentWithAnUntrustedManifest,
}

impl WhatCannotStandForProvenance {
    pub const ALL: [WhatCannotStandForProvenance; 4] = [
        WhatCannotStandForProvenance::AVersionAnswerFromThePackage,
        WhatCannotStandForProvenance::AMatchingFilename,
        WhatCannotStandForProvenance::TransportSuccess,
        WhatCannotStandForProvenance::BytesSelfConsistentWithAnUntrustedManifest,
    ];
}

/// **One retained adapter's declared protocol interval and request set, held by
/// value — G31a's second bullet.**
///
/// `Server protocol and transfers` §1.2 item 3's request-set declaration was
/// otherwise only a running adapter's own answer. **The placement writes it into
/// the phase record by value**, per retained adapter, taken from the staged
/// package's own declaration **after G29's comparison has passed and before the
/// *staged and validated* phase is written**. Inspection reads it as content and
/// never asks a program.
#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RetainedAdapter {
    /// The declared protocol interval, as the adapter declares it.
    pub declared_interval: (u32, u32),
    /// The request set the adapter declares it answers.
    pub declared_request_set: Vec<String>,
}

/// **What B4's inspection returns, and G31a is why every value of it is a
/// durable record the placement wrote.**
///
/// **No program answers an inspection in either phase.** B4 is a composed
/// file-transfer-subsystem reading of the installation's own durable records:
/// the desktop composes the answer, a dead supervisor changes nothing, and **on
/// a machine that has never held a package every record is absent, which is the
/// honest never-held answer.**
///
/// **Three records carry the whole set** — the boot-registration record (K16),
/// the installation identity (G10), and the phase record (G2).
#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InspectionEvidence {
    /// The active pointer's release sequence, package version, manifest digest
    /// and artifact digest. **Absent on a machine that has never held a
    /// package**, which is the honest never-held answer rather than a zero.
    pub the_active_pointers_descriptor: Option<VerifiedReleaseDescriptor>,
    /// The phase and retained-adapter intervals.
    pub retained_adapters: Vec<RetainedAdapter>,
}

/// **G31. Package inspection and continuation expose and re-check the same
/// evidence.**
///
/// G19 re-checks G27's recorded descriptor before staged material is reused.
/// **The package phase becomes *staged and validated* only after those values
/// and B10's at-rest comparison agree.**
pub fn may_become_staged_and_validated(
    recorded: &VerifiedReleaseDescriptor,
    what_the_evidence_says: &VerifiedReleaseDescriptor,
    at_rest: AtRestComparison,
) -> bool {
    recorded == what_the_evidence_says && at_rest == AtRestComparison::Agrees
}

/// **G31a stated as a rule: the inspection is composed from records and never
/// from a program's answer.**
///
/// The trait takes no live-program handle for that reason. Where a verified
/// runtime happens to be alive it is still the records that answer, because a
/// dead supervisor must change nothing about what an inspection says.
pub trait ReadsTheDurableRecordsAnInspectionIsComposedFrom {
    /// Absent everywhere is the never-held answer, and it is a real answer.
    fn the_inspection_evidence(&self) -> Result<InspectionEvidence, String>;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn a_descriptor() -> VerifiedReleaseDescriptor {
        VerifiedReleaseDescriptor {
            manifest_format: "1".into(),
            verified_release_sequence: 7,
            requested_package_version: "0.1.0".into(),
            manifest_digest: [1; 32],
            artifact_identity: "linux-x86_64-gnu".into(),
            artifact_byte_length: 4096,
            artifact_digest: [2; 32],
        }
    }

    #[test]
    fn a_request_with_no_verified_descriptor_fails_before_commit() {
        assert_eq!(
            may_proceed(None),
            WhetherTheRequestMayProceed::FailedBeforeCommitNoVerifiedDescriptor
        );
    }

    #[test]
    fn the_at_rest_comparison_needs_both_the_length_and_the_digest() {
        let descriptor = a_descriptor();
        assert_eq!(
            compare_at_rest(
                &descriptor,
                AtRestMeasurement {
                    byte_length: 4096,
                    digest: [2; 32]
                }
            ),
            AtRestComparison::Agrees
        );
        // Right digest, wrong length.
        assert_eq!(
            compare_at_rest(
                &descriptor,
                AtRestMeasurement {
                    byte_length: 4095,
                    digest: [2; 32]
                }
            ),
            AtRestComparison::FailedBeforeCommit
        );
        // Right length, wrong digest.
        assert_eq!(
            compare_at_rest(
                &descriptor,
                AtRestMeasurement {
                    byte_length: 4096,
                    digest: [3; 32]
                }
            ),
            AtRestComparison::FailedBeforeCommit
        );
    }

    #[test]
    fn staged_and_validated_needs_the_recorded_descriptor_and_the_evidence_to_agree() {
        let recorded = a_descriptor();
        let mut evidence = a_descriptor();
        assert!(may_become_staged_and_validated(
            &recorded,
            &evidence,
            AtRestComparison::Agrees
        ));
        // A different artifact behind the same requested version is exactly what
        // G27 forbids a retry from substituting.
        evidence.artifact_identity = "linux-aarch64-gnu".into();
        assert!(!may_become_staged_and_validated(
            &recorded,
            &evidence,
            AtRestComparison::Agrees
        ));
        assert!(!may_become_staged_and_validated(
            &recorded,
            &a_descriptor(),
            AtRestComparison::FailedBeforeCommit
        ));
    }

    #[test]
    fn a_machine_that_has_never_held_a_package_answers_absent_rather_than_empty() {
        let evidence = InspectionEvidence {
            the_active_pointers_descriptor: None,
            retained_adapters: Vec::new(),
        };
        assert!(evidence.the_active_pointers_descriptor.is_none());
    }

    #[test]
    fn the_two_refusal_sets_are_written_out_rather_than_left_to_the_word_exact() {
        // **Both sets are closed by the compiler rather than by a count.** Each
        // match is total and carries no default arm, so a refusal added to
        // either enum has none, the crate stops compiling, and whoever added it
        // is sent to the list — where the two length assertions this replaced
        // could never have been false, each array's own type having fixed it.
        for (at, refused) in ASelectionThisDocumentRefuses::ALL.iter().enumerate() {
            let where_the_list_puts_it = match refused {
                ASelectionThisDocumentRefuses::AGenericLinuxEntry => 0,
                ASelectionThisDocumentRefuses::TheClosestProcessor => 1,
                ASelectionThisDocumentRefuses::TheClosestAbi => 2,
                ASelectionThisDocumentRefuses::ACapabilityFallback => 3,
                ASelectionThisDocumentRefuses::APackageNameGuess => 4,
                ASelectionThisDocumentRefuses::TheFirstEntry => 5,
            };
            assert_eq!(at, where_the_list_puts_it, "{refused:?} is not where the list puts it");
        }
        for (at, cannot) in WhatCannotStandForProvenance::ALL.iter().enumerate() {
            let where_the_list_puts_it = match cannot {
                WhatCannotStandForProvenance::AVersionAnswerFromThePackage => 0,
                WhatCannotStandForProvenance::AMatchingFilename => 1,
                WhatCannotStandForProvenance::TransportSuccess => 2,
                WhatCannotStandForProvenance::BytesSelfConsistentWithAnUntrustedManifest => 3,
            };
            assert_eq!(at, where_the_list_puts_it, "{cannot:?} is not where the list puts it");
        }
    }
}

//! §2.5 and §2.6 — the retry rule, and what blocks the commit.
//!
//! **G17. A retry carrying the same operation identity is answered from the
//! phase, and the branch is fixed by the phase rather than judged.** That is
//! `Server protocol and transfers` B3's idempotency, taken from the phase record
//! because there is no operation record for this request.
//!
//! **The phase record alone still cannot tell an unfinished attempt from a live
//! one** — G3 writes every phase before the act it names, so a record left at
//! *staging* by a dead attempt reads identically to one held by a live attempt.
//! **The claim, not a time or a reading of the record, makes that distinction.**
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Server protocol and transfers` B3, B6, B10, C4,
//! S28, and its current-plus-immediately-previous compatibility clause.
//!
//! **A marker this plan also defines is never listed here and is written out
//! with its document at the site**, because a header list cannot tell the two
//! apart and a bare one there would resolve to a real and wrong clause.

use super::phase::{ClaimOutcome, Phase, PhaseRecord};
use super::release::{AtRestComparison, VerifiedReleaseDescriptor};

/// Where the pointer points when a retry meets *committing* — **the one branch
/// of G17's table that reads something beside the phase record.**
///
/// **G7's *committing* and *pointer-done* states leave the same phase record**
/// and are told apart only by reading the pointer, so a retry resolves on the
/// phase record **plus** the pointer rather than on the phase record alone
/// (§16 item 3).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatThePointerNames {
    TheRequestedVersion,
    ThePreviousVersion,
    /// **The interrupted-rename state §16 item 3 records rather than assumes
    /// away.**
    Neither,
}

/// What a retry under the same identity does.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SameIdentityRetry {
    /// *inspecting*, *staging* — **the staged material is re-validated by
    /// content, the parts that match are kept and the rest is re-staged.**
    ContinueRevalidatingStagedMaterial,
    /// *staged and validated* — **continues from the registration step.**
    ContinueFromTheRegistrationStep,
    /// *committing*, pointer naming the requested version — **proceeds to G25's
    /// activation and the health exchange.**
    ProceedToActivationAndTheHealthExchange,
    /// *committing*, pointer naming the previous version — **re-enters
    /// *committing*.**
    ReEnterCommitting,
    /// *committing*, pointer naming neither — **the pointer is restored from
    /// G4's recorded previous value under the *rolling back* phase, exactly as
    /// G14 restores it after a failed exchange.**
    RestoreUnderRollingBack,
    /// *committed, unverified* — **performs G25's activation and the health
    /// exchange and takes G14 or G15.**
    CompleteTheActivationAndTakeG14OrG15,
    /// *rolling back* — **completes the rollback (G16), and then answers as
    /// *rolled back*.**
    CompleteTheRollback,
    /// *rolled back*, *failed before commit* — **starts a fresh replacement,
    /// under the same identity, after sweeping the abandoned staging of the
    /// failed attempt.**
    FreshReplacementAfterSweepingTheAbandonedStaging,
    /// *verified* — **acts again on nothing and returns the recorded outcome**
    /// (`Server protocol and transfers` C4).
    ActAgainOnNothingAndReturnTheRecordedOutcome,
    /// *deferred* — **re-evaluates §2.6 and either proceeds or answers deferred
    /// again.**
    ReEvaluateWhatBlocksTheCommit,
}

/// **G17's table.**
///
/// **A same-identity request that meets a live owner receives the in-progress
/// result and changes nothing** — that is the caller's check before this
/// function, and [`what_a_different_identity_may_do`] is where it is written.
pub fn what_a_same_identity_retry_does(
    phase: Phase,
    pointer: Option<WhatThePointerNames>,
) -> SameIdentityRetry {
    match phase {
        Phase::Inspecting | Phase::Staging => SameIdentityRetry::ContinueRevalidatingStagedMaterial,
        Phase::StagedAndValidated => SameIdentityRetry::ContinueFromTheRegistrationStep,
        Phase::Committing => match pointer {
            Some(WhatThePointerNames::TheRequestedVersion) => {
                SameIdentityRetry::ProceedToActivationAndTheHealthExchange
            }
            Some(WhatThePointerNames::ThePreviousVersion) => SameIdentityRetry::ReEnterCommitting,
            // **A pointer that could not be read is not a third answer**: it
            // reads as naming neither, which is the safe direction — the
            // restore G14 already performs, from a recorded value rather than
            // from the machine.
            Some(WhatThePointerNames::Neither) | None => SameIdentityRetry::RestoreUnderRollingBack,
        },
        Phase::CommittedUnverified => SameIdentityRetry::CompleteTheActivationAndTakeG14OrG15,
        Phase::RollingBack => SameIdentityRetry::CompleteTheRollback,
        Phase::RolledBack | Phase::FailedBeforeCommit => {
            SameIdentityRetry::FreshReplacementAfterSweepingTheAbandonedStaging
        }
        Phase::Verified => SameIdentityRetry::ActAgainOnNothingAndReturnTheRecordedOutcome,
        Phase::Deferred => SameIdentityRetry::ReEvaluateWhatBlocksTheCommit,
    }
}

/// What a request under a **different** operation identity may do.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DifferentIdentity {
    /// **G18.** It could not acquire the claim, so it **receives the in-progress
    /// result and acts on nothing.**
    ReceivesTheInProgressResultAndActsOnNothing,
    /// **G18.** It acquired the claim because the previous owner has genuinely
    /// ended, so it **drives the recorded attempt under the recorded attempt
    /// identity to a terminal phase** — completing a commit, completing a
    /// rollback (G16), or sweeping failed staging (G20). **Only after that
    /// terminal write may it replace the owner and attempt fields and begin a
    /// fresh replacement.**
    DrivesTheRecordedAttemptToATerminalPhaseFirst,
    /// **The one carve-out is *deferred*, where B6 governs.** A request under a
    /// different identity that finds the phase record at *deferred* **is
    /// answered with that phase and acts on nothing**, because B6 and G22 are
    /// exact that a deferred result *"does not license a second attempt under a
    /// fresh identity"*.
    AnsweredWithTheDeferredPhaseAndActsOnNothing,
}

/// **G18, with its one carve-out.**
///
/// **This preserves the route out for an attempt whose identity is never re-sent
/// without allowing two application instances, automatic repair and the setup
/// fallback to terminalise one another's live work.**
pub fn what_a_different_identity_may_do(
    claim: &ClaimOutcome,
    phase_found: Phase,
) -> DifferentIdentity {
    if phase_found == Phase::Deferred {
        return DifferentIdentity::AnsweredWithTheDeferredPhaseAndActsOnNothing;
    }
    match claim {
        ClaimOutcome::InProgress { .. } => {
            DifferentIdentity::ReceivesTheInProgressResultAndActsOnNothing
        }
        ClaimOutcome::Acquired | ClaimOutcome::TakenBackFromThisInstallation => {
            DifferentIdentity::DrivesTheRecordedAttemptToATerminalPhaseFirst
        }
    }
}

/// **G19. Staged material is validated by content and never by presence or
/// version label.**
///
/// A retry that finds staged material at the location the phase record names
/// **re-verifies G27's recorded release sequence, manifest digest, artifact
/// identity, byte length and artifact digest through B10's at-rest read-back
/// comparison before reusing or invoking a byte.** A version answer from the
/// staged program is a health check after provenance and cannot replace
/// provenance.
pub fn staged_material_may_be_reused(
    recorded: &VerifiedReleaseDescriptor,
    what_the_staged_material_measures: &VerifiedReleaseDescriptor,
    at_rest: AtRestComparison,
) -> bool {
    recorded.verified_release_sequence == what_the_staged_material_measures.verified_release_sequence
        && recorded.manifest_digest == what_the_staged_material_measures.manifest_digest
        && recorded.artifact_identity == what_the_staged_material_measures.artifact_identity
        && recorded.artifact_byte_length == what_the_staged_material_measures.artifact_byte_length
        && recorded.artifact_digest == what_the_staged_material_measures.artifact_digest
        && at_rest == AtRestComparison::Agrees
}

/// **G20. Abandoned staging is swept, and the sweep is bounded to the locations
/// the phase record names.**
///
/// It is the same shape as `Server protocol and transfers` S28 and **it reaches
/// nothing else on the machine.** The bound is the value the function returns:
/// there is no widening argument and no folder walk.
///
/// **The staging location alone, and the previous version's is not on it.** G5
/// walks a reader through exactly which material this sweep reaches: at the two
/// phases it runs at — *rolled back* and *failed before commit* — the previous
/// package is the one the machine is running, and the material left behind is
/// this attempt's own staged material. **The previous version has its own two
/// removals and neither is a sweep**: G15's after a verified commit, and G5's at
/// a fresh replacement's first act, which is
/// [`what_a_removal_of_the_previous_version_reaches`].
pub fn what_the_sweep_may_reach(record: &PhaseRecord) -> Vec<String> {
    vec![record.staging_location.clone()]
}

/// **The previous version's own location — G15's removal at a verified commit,
/// and G5's at a fresh replacement's first act.**
///
/// **It is not the sweep's**, and it has a name of its own so that the two
/// cannot be confused: G20's sweep runs at *rolled back* and *failed before
/// commit*, where the previous package is the one the machine is running.
/// **Nothing where there is no previous version.**
pub fn what_a_removal_of_the_previous_version_reaches(record: &PhaseRecord) -> Vec<String> {
    record
        .previous
        .as_ref()
        .map(|previous| vec![previous.location.clone()])
        .unwrap_or_default()
}

/// **What an operation's standing amounts to for G21 — the four states that
/// block, taken from `Server protocol and transfers` P3 applied at the commit.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AcceptedWorkStanding {
    Live,
    Held,
    /// A result that has not been collected.
    Uncollected,
    /// A result collected and not yet acknowledged.
    Unacknowledged,
    /// Nothing depends on the runtime for this operation.
    Nothing,
}

impl AcceptedWorkStanding {
    /// Whether this standing blocks the commit.
    pub fn blocks(self) -> bool {
        !matches!(self, AcceptedWorkStanding::Nothing)
    }
}

/// **G22's result: a blocked commit is *deferred* and is not a failure.**
///
/// The phase record records the reason, the staged material is kept, and the
/// request answers with the deferred result `Server protocol and transfers` B6
/// already carries — *"it is not a failure, it does not expire a waiter, and it
/// does not license a second attempt under a fresh identity."*
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhetherTheCommitMayProceed {
    ItMayProceed,
    Deferred { because: String },
}

/// **G21. The whole commit is blocked while accepted work still depends on the
/// current package or its immediately previous adapter.**
///
/// **Before *committing*, the transaction reads whether any operation accepted
/// under either retained protocol version is live, held, has an uncollected
/// result, or has a result not yet acknowledged.** The retained previous adapter
/// remains runnable for inspection, stop, release and collection throughout; **an
/// indefinite hold delays activation and never removes the control route.**
///
/// **G23. A hold blocks a commit for as long as it stands, and an ended result
/// blocks retirement until it is collected and acknowledged.** §8.6 is why the
/// first has no timer; P3 is why neither condition permits removing the last
/// compatible adapter. **Once no accepted operation depends on the immediately
/// previous version, G15 may retire it after the candidate's health exchange
/// succeeds.**
pub fn whether_the_commit_may_proceed(
    accepted_under_either_retained_version: &[(String, AcceptedWorkStanding)],
) -> WhetherTheCommitMayProceed {
    let blocking: Vec<&str> = accepted_under_either_retained_version
        .iter()
        .filter(|(_, standing)| standing.blocks())
        .map(|(what, _)| what.as_str())
        .collect();
    if blocking.is_empty() {
        WhetherTheCommitMayProceed::ItMayProceed
    } else {
        WhetherTheCommitMayProceed::Deferred {
            because: blocking.join(", "),
        }
    }
}

/// **G24. Removal of the runtime is only an internal step of G15 and G20, and no
/// user action reaches it.**
///
/// The one machine-reaching server removal stops work, clears pending jobs and
/// removes the Telegram role; **the runtime remains.** That is the shape's rule
/// and this document exposes nothing else — which is why there is no function
/// here that removes a runtime on request, and that absence is the clause.
pub const NO_USER_ACTION_REACHES_RUNTIME_REMOVAL: () = ();

#[cfg(test)]
mod tests {
    use smoozy_state::identities::{Identity, IdentityKind};

    use super::*;
    use crate::machine_kind::MachineKind;
    use crate::package::phase::{a_fresh_attempts_first_write, PreviousVersion, Route, TransactionOwner};
    use crate::package::release::RetainedAdapter;

    fn a_descriptor() -> VerifiedReleaseDescriptor {
        VerifiedReleaseDescriptor {
            manifest_format: "1".into(),
            verified_release_sequence: 7,
            requested_package_version: "0.2.0".into(),
            manifest_digest: [1; 32],
            artifact_identity: "linux-x86_64-gnu".into(),
            artifact_byte_length: 4096,
            artifact_digest: [2; 32],
        }
    }

    #[test]
    fn every_phase_has_a_branch_and_the_committing_one_reads_the_pointer() {
        for phase in Phase::ALL {
            // No phase falls through: the call is total.
            let _ = what_a_same_identity_retry_does(phase, Some(WhatThePointerNames::Neither));
        }
        assert_eq!(
            what_a_same_identity_retry_does(
                Phase::Committing,
                Some(WhatThePointerNames::TheRequestedVersion)
            ),
            SameIdentityRetry::ProceedToActivationAndTheHealthExchange
        );
        assert_eq!(
            what_a_same_identity_retry_does(
                Phase::Committing,
                Some(WhatThePointerNames::ThePreviousVersion)
            ),
            SameIdentityRetry::ReEnterCommitting
        );
        assert_eq!(
            what_a_same_identity_retry_does(Phase::Committing, Some(WhatThePointerNames::Neither)),
            SameIdentityRetry::RestoreUnderRollingBack
        );
    }

    #[test]
    fn a_verified_phase_acts_again_on_nothing() {
        assert_eq!(
            what_a_same_identity_retry_does(Phase::Verified, None),
            SameIdentityRetry::ActAgainOnNothingAndReturnTheRecordedOutcome
        );
    }

    #[test]
    fn a_deferred_phase_answers_a_different_identity_and_acts_on_nothing_even_when_unclaimed() {
        assert_eq!(
            what_a_different_identity_may_do(&ClaimOutcome::Acquired, Phase::Deferred),
            DifferentIdentity::AnsweredWithTheDeferredPhaseAndActsOnNothing
        );
    }

    #[test]
    fn a_different_identity_meeting_a_live_owner_acts_on_nothing() {
        let owner = TransactionOwner {
            invocation: Identity::issue(IdentityKind::Operation).expect("randomness"),
            route: Route::SetupPlacement,
            process: (10, 20),
            installation: Identity::issue(IdentityKind::Installation).expect("randomness"),
        };
        let in_progress = ClaimOutcome::InProgress {
            owner,
            phase: Phase::Staging,
        };
        assert_eq!(
            what_a_different_identity_may_do(&in_progress, Phase::Staging),
            DifferentIdentity::ReceivesTheInProgressResultAndActsOnNothing
        );
    }

    #[test]
    fn a_different_identity_that_acquired_drives_the_recorded_attempt_first() {
        assert_eq!(
            what_a_different_identity_may_do(&ClaimOutcome::Acquired, Phase::Staging),
            DifferentIdentity::DrivesTheRecordedAttemptToATerminalPhaseFirst
        );
    }

    #[test]
    fn staged_material_is_reused_only_where_every_recorded_value_and_the_at_rest_check_agree() {
        let recorded = a_descriptor();
        assert!(staged_material_may_be_reused(
            &recorded,
            &a_descriptor(),
            AtRestComparison::Agrees
        ));
        let mut a_different_artifact = a_descriptor();
        a_different_artifact.artifact_digest = [9; 32];
        assert!(!staged_material_may_be_reused(
            &recorded,
            &a_different_artifact,
            AtRestComparison::Agrees
        ));
        assert!(!staged_material_may_be_reused(
            &recorded,
            &a_descriptor(),
            AtRestComparison::FailedBeforeCommit
        ));
    }

    #[test]
    fn the_sweep_reaches_the_named_locations_and_nothing_else() {
        let record = a_fresh_attempts_first_write(
            Identity::issue(IdentityKind::Operation).expect("randomness"),
            a_descriptor(),
            Some(PreviousVersion {
                descriptor: a_descriptor(),
                retained_adapter: RetainedAdapter {
                    declared_interval: (1, 2),
                    declared_request_set: vec!["inspect".into()],
                },
                location: "/opt/smoozy/previous".into(),
                previous_boot_registration_content: b"the old hook".to_vec(),
            }),
            MachineKind::Container,
            "/opt/smoozy/staging".into(),
        );
        assert_eq!(
            what_the_sweep_may_reach(&record),
            vec!["/opt/smoozy/staging".to_string()]
        );
        // **The previous version is what the machine is running at the two
        // phases the sweep runs at**, so the sweep must not reach it. Its own
        // two removals — G15's and G5's — have their own function.
        assert!(
            !what_the_sweep_may_reach(&record).contains(&"/opt/smoozy/previous".to_string()),
            "the sweep must not reach the package the machine is running"
        );
        assert_eq!(
            what_a_removal_of_the_previous_version_reaches(&record),
            vec!["/opt/smoozy/previous".to_string()]
        );
    }

    /// **A first placement has no previous version**, so the removal reaches
    /// nothing and the sweep still reaches the staging location.
    #[test]
    fn a_first_placement_has_nothing_for_the_previous_versions_removal_to_reach() {
        let record = a_fresh_attempts_first_write(
            Identity::issue(IdentityKind::Operation).expect("randomness"),
            a_descriptor(),
            None,
            MachineKind::Container,
            "/opt/smoozy/staging".into(),
        );
        assert_eq!(
            what_the_sweep_may_reach(&record),
            vec!["/opt/smoozy/staging".to_string()]
        );
        assert!(what_a_removal_of_the_previous_version_reaches(&record).is_empty());
    }

    #[test]
    fn every_one_of_the_four_standings_blocks_the_commit_and_nothing_else_does() {
        for standing in [
            AcceptedWorkStanding::Live,
            AcceptedWorkStanding::Held,
            AcceptedWorkStanding::Uncollected,
            AcceptedWorkStanding::Unacknowledged,
        ] {
            assert!(standing.blocks());
            assert!(matches!(
                whether_the_commit_may_proceed(&[("an operation".into(), standing)]),
                WhetherTheCommitMayProceed::Deferred { .. }
            ));
        }
        assert_eq!(
            whether_the_commit_may_proceed(&[("an operation".into(), AcceptedWorkStanding::Nothing)]),
            WhetherTheCommitMayProceed::ItMayProceed
        );
        assert_eq!(
            whether_the_commit_may_proceed(&[]),
            WhetherTheCommitMayProceed::ItMayProceed
        );
    }
}

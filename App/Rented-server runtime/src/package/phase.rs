//! §2.1 and §2.2 — the durable package-phase record, the one owner of the
//! transaction, and the previous version captured by value.
//!
//! **`Server protocol and transfers` B3 is exact about why there is a phase
//! record rather than an operation record**: the supervisor may not exist when
//! the request arrives, so *"A builder must not look for an operation record for
//! this request."*
//!
//! **Who actually holds the transaction, and before hand-over the answer is
//! nobody.** G1a's kernel-held exclusive lock belongs to a machine-side holder
//! and to no other. Every bare machine, and every machine whose supervisor is
//! dead — which is the whole case the bootstrap-and-repair request exists for —
//! is G1b's: the desktop performs the transaction through the surface, and **the
//! surface has no route to a kernel lock.**
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Server protocol and transfers` B1, B2, B3, B6,
//! and its §1.1. `State, identity and recovery` D1, and its §1.3 rule 6.
//! `Desktop app and delivery` — its one-instance rule. `Server Setup` — the
//! fallback re-run.
//!
//! **A marker this plan also defines is never listed here and is written out
//! with its document at the site**, because a header list cannot tell the two
//! apart and a bare one there would resolve to a real and wrong clause.

use smoozy_state::identities::Identity;

use super::release::{RetainedAdapter, VerifiedReleaseDescriptor};
use crate::machine_kind::MachineKind;

/// **G3's closed phase set, and each phase is written durably before the act it
/// names.**
///
/// That ordering is `State, identity and recovery` D1's *intent before acting*
/// applied to a transaction that has no supervisor to hold it. **What *durably*
/// means is defined once, in that document's §1.3 rule 6, and every use of the
/// word here takes it from there.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum Phase {
    /// Written before anything is staged. A later reading: **the machine is
    /// unchanged.**
    Inspecting,
    /// Written before the first byte of material is placed. **Staged material
    /// may be incomplete; the active package is untouched.**
    Staging,
    /// Written before the registration content is written. **The material is
    /// complete and validated by content; the active package is untouched.**
    StagedAndValidated,
    /// Written before the deciding write. **The pointer may or may not have
    /// moved; nothing else about the machine has.**
    Committing,
    /// Written before the running supervisor is ended (G25) and the health
    /// exchange is attempted. **The pointer has moved; the previous package is
    /// still present; the supervisor answering may be either version's.**
    CommittedUnverified,
    /// Written before the previous package is removed. **The transaction
    /// succeeded.**
    Verified,
    /// Written before the pointer is restored. **The machine is between two
    /// known states, both of which are recorded.**
    RollingBack,
    /// **The machine is on the previous package.**
    RolledBack,
    /// **The machine is on the previous package and staged material is owed a
    /// sweep.**
    FailedBeforeCommit,
    /// **The machine is on the previous package, the staged material is kept,
    /// and live or held work is blocking the commit** (§2.6).
    Deferred,
}

impl Phase {
    /// The whole set, written out. **G3 says it is closed**, and a phase not on
    /// this list is not a phase this transaction can be in.
    pub const ALL: [Phase; 10] = [
        Phase::Inspecting,
        Phase::Staging,
        Phase::StagedAndValidated,
        Phase::Committing,
        Phase::CommittedUnverified,
        Phase::Verified,
        Phase::RollingBack,
        Phase::RolledBack,
        Phase::FailedBeforeCommit,
        Phase::Deferred,
    ];

    /// What a later reading of this phase means about the machine — G3's third
    /// column, carried as a value so nothing downstream has to remember it.
    pub fn what_a_later_reading_means(self) -> &'static str {
        match self {
            Phase::Inspecting => "the machine is unchanged",
            Phase::Staging => {
                "staged material may be incomplete; the active package is untouched"
            }
            Phase::StagedAndValidated => {
                "the material is complete and validated by content; the active package is untouched"
            }
            Phase::Committing => {
                "the pointer may or may not have moved; nothing else about the machine has"
            }
            Phase::CommittedUnverified => {
                "the pointer has moved; the previous package is still present; the supervisor answering may be either version's"
            }
            Phase::Verified => "the transaction succeeded",
            Phase::RollingBack => {
                "the machine is between two known states, both of which are recorded"
            }
            Phase::RolledBack => "the machine is on the previous package",
            Phase::FailedBeforeCommit => {
                "the machine is on the previous package and staged material is owed a sweep"
            }
            Phase::Deferred => {
                "the machine is on the previous package, the staged material is kept, and live or held work is blocking the commit"
            }
        }
    }
}

/// **Which route is performing the transaction — G2's owner field.**
///
/// **The automatic repair route, every `Server Setup` package placement, and
/// that document's fallback re-run all acquire the same claim.** Two desktop
/// application instances do not have two owners: one acquires it and the other
/// receives the in-progress result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum Route {
    AutomaticRepair,
    SetupPlacement,
    UserFallback,
}

/// **G2's transaction-owner fields, written only while the claim is held** and
/// distinct from the attempt's operation identity.
#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct TransactionOwner {
    /// The invocation identity.
    #[serde(with = "crate::written_form::an_identity")]
    pub invocation: Identity,
    pub route: Route,
    /// The process identifier and its kernel-set start time — **the pair, never
    /// the identifier alone** (X1's reason, applied to the owner).
    pub process: (u32, u64),
    /// **The claiming installation identity**, which is what G1b's take-over
    /// rule reads.
    #[serde(with = "crate::written_form::an_identity")]
    pub installation: Identity,
}

/// **G1. One phase record per machine, in the package area, written by whatever
/// is carrying out the transaction, and it is the record `Server protocol and
/// transfers` B3 answers idempotency from.** That document is exact about this and
/// about why: **the supervisor may not exist when the request arrives**, so *"A
/// builder must not look for an operation record for this request."*
///
/// **G2. It holds this, and it holds nothing derived from what is on the machine
/// at the moment it is read.**
#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PhaseRecord {
    /// **The operation identity of the attempt in hand**, which is the request's
    /// idempotency key (`Server protocol and transfers` B1).
    #[serde(with = "crate::written_form::an_identity")]
    pub attempt: Identity,
    /// **G1a's own rule**: this is written only while the claim is held.
    pub owner: Option<TransactionOwner>,
    pub phase: Phase,
    /// B1's verified release descriptor.
    pub requested: VerifiedReleaseDescriptor,
    /// **The previously active package's complete descriptor by value**, with
    /// its retained-adapter record and location (G4, G31a).
    pub previous: Option<PreviousVersion>,
    /// **Each retained adapter's declared protocol interval and request set, by
    /// value** — the staged package's, written before *staged and validated*.
    pub staged_packages_retained_adapters: Vec<RetainedAdapter>,
    /// The staging location of this attempt's material.
    pub staging_location: String,
    /// **The machine kind detected for this attempt** (K2).
    pub machine_kind: MachineKind,
    /// The outcome of the health exchange, once there is one.
    pub health_exchange: Option<super::commit::HealthExchange>,
    /// The reason a commit was deferred, where one was.
    pub deferred_because: Option<String>,
}

/// **G4's carried-forward half: the previous active package, complete, by
/// value.**
///
/// **A rollback restores from that record and never from a second reading of the
/// machine**, because a rollback runs in exactly the case where what is on the
/// machine is no longer trustworthy.
#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PreviousVersion {
    pub descriptor: VerifiedReleaseDescriptor,
    pub retained_adapter: RetainedAdapter,
    /// Where the previous package's material sits.
    pub location: String,
    /// **The previous boot-registration content by value** (§2.2), which is what
    /// G14 restores in G9's case and in no other.
    pub previous_boot_registration_content: Vec<u8>,
}

/// **G4. One write, carrying the fresh attempt's identity, the carried-forward
/// values and that attempt's first phase together** — never an attempt recorded
/// first and a phase after it, there being no program that answers for them
/// (G31a).
///
/// The function composes that one write rather than performing it; performing it
/// is [`WritesThePhaseRecord`]'s.
pub fn a_fresh_attempts_first_write(
    attempt: Identity,
    requested: VerifiedReleaseDescriptor,
    carried_forward: Option<PreviousVersion>,
    machine_kind: MachineKind,
    staging_location: String,
) -> PhaseRecord {
    PhaseRecord {
        attempt,
        owner: None,
        phase: Phase::Inspecting,
        requested,
        previous: carried_forward,
        staged_packages_retained_adapters: Vec::new(),
        staging_location,
        machine_kind,
        health_exchange: None,
        deferred_because: None,
    }
}

/// **G5. There is at most one previous version kept, and it is the one the phase
/// record names.**
///
/// A machine holds the active package and, between *committing* and *verified*,
/// its immediate predecessor. **Nothing accumulates, and it is two removals
/// rather than one.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhichRemovalReachesIt {
    /// **In the forward direction**: the removal of the predecessor is the last
    /// step of a verified commit (G15).
    TheLastStepOfAVerifiedCommit,
    /// **A fresh replacement that begins with a predecessor still present
    /// removes it as its first act**, recording that removal in the phase
    /// record.
    TheFirstActOfAFreshReplacement,
    /// **In the other direction**, the material a rolled-back attempt leaves
    /// behind is not a predecessor and neither removal reaches it: after G14 it
    /// is this attempt's own staged material, and **G20's bounded sweep removes
    /// it on §13 R1's terms.** Nothing else removes it and no separate step is
    /// written for it.
    G20sBoundedSweep,
}

/// **G5's rule made a function rather than a paragraph.** Given where the
/// material sits and what the transaction reached, which removal reaches it.
pub fn which_removal_reaches(phase: Phase, is_the_predecessor: bool) -> WhichRemovalReachesIt {
    if !is_the_predecessor {
        return WhichRemovalReachesIt::G20sBoundedSweep;
    }
    match phase {
        Phase::Verified => WhichRemovalReachesIt::TheLastStepOfAVerifiedCommit,
        _ => WhichRemovalReachesIt::TheFirstActOfAFreshReplacement,
    }
}

/// **G31a's own rule, which is G17's *committing* rule already.**
///
/// Which of the phase record's two recorded descriptors is the active pointer's
/// is decided by the pointer's own value against the recorded staging and
/// previous locations. **A pointer naming neither is the interrupted-rename
/// state the plan's own Section 16 item 3 records rather than assumes away**,
/// and no descriptor is claimed for it.
///
/// **It sits in the rules half rather than in [`crate::acting`], moved there on
/// 2026-09-06, and that is this crate's own rule rather than a preference**: the
/// acting half is *"the only part of this crate that opens a socket, starts a
/// process, sends a signal, renames a file or runs a command"*, and this
/// function does none of those — it is judgement over a reading somebody else
/// took. **What made the misplacement cost something is
/// `Server protocol and transfers` B4**: that clause has the **desktop** compose
/// the same answer over the file-transfer subsystem, on all three of the user's
/// systems, and [`crate::acting`] is gated to a unix. The old path still
/// resolves — that module re-exports this.
pub fn which_descriptor_the_pointer_names(
    record: &PhaseRecord,
    pointing_at: Option<&str>,
) -> Option<VerifiedReleaseDescriptor> {
    let pointing_at = pointing_at?;
    if pointing_at == record.staging_location {
        return Some(record.requested.clone());
    }
    match &record.previous {
        Some(previous) if pointing_at == previous.location => Some(previous.descriptor.clone()),
        _ => None,
    }
}

/// **G6. A first placement has no previous version, and its phase record says
/// so.**
///
/// **A rollback with no previous version to restore leaves no active pointer**,
/// which is the *absent* branch of the installation-identity check and is safe:
/// nothing starts, nothing claims to be Smoozy's, and `Server Setup` owns what
/// the user is offered.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatARollbackRestores {
    ThePreviousPointer,
    /// G6. **Safe rather than a fault**: the absent branch.
    NoActivePointerAtAll,
}

/// **G6.** What a rollback of this record restores.
pub fn what_a_rollback_restores(record: &PhaseRecord) -> WhatARollbackRestores {
    match record.previous {
        Some(_) => WhatARollbackRestores::ThePreviousPointer,
        None => WhatARollbackRestores::NoActivePointerAtAll,
    }
}

/// **Who holds the transaction — G1a where a machine-side program performs it,
/// G1b where the desktop does.**
///
/// **G1a.** One kernel-held exclusive lock, at one fixed path in the package area
/// — the same path G1b claims — acquired before the phase record is created,
/// replaced or changed and before any phase act begins. Its holder opens it
/// itself, holds it for the whole transaction, marks the descriptor
/// close-on-exec, and passes it to no child. **A process exit therefore makes a
/// genuinely abandoned transaction claimable without a timer, while a live
/// holder remains the owner.**
///
/// **G1b.** Where the desktop performs the transaction through the surface there
/// is no Smoozy program on the machine at all, the command channel's invocations
/// are per-invocation processes whose exit releases any lock they took, and the
/// file-transfer subsystem's observed request set carries no lock operation of
/// any kind. **What claims it instead is the exclusive create of the claim file
/// itself**, observed refused over an existing path on three machines across
/// three sellers, two operating-system families, two machine kinds and three
/// versions of the server software, with a plain create over the same path
/// succeeding as the control.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhoClaims {
    /// A verified runtime is alive and performs its own half of the transaction
    /// (`Server protocol and transfers` B2). **It takes the claim file first and
    /// then G1a's exclusive lock on that same file.**
    ALiveRuntimeTakingTheClaimFileAndThenTheLock,
    /// **Every bare machine, and every machine whose supervisor is dead.** The
    /// desktop takes the claim file and **cannot take the lock**.
    TheDesktopTakingTheClaimFileAlone,
}

/// **What the claim file holds** — the claiming installation identity and G2's
/// transaction-owner fields. **It is removed at the terminal write, and by
/// nothing else.**
#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Claim {
    pub owner: TransactionOwner,
    /// The phase the claim was written against, which the in-progress result
    /// reports.
    pub phase: Phase,
}

/// What acquiring the claim gives.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClaimOutcome {
    /// The claimant owns the transaction and may act.
    Acquired,
    /// **A claimant that meets an existing claim changes no phase, pointer,
    /// staging path or health result**; it returns *package transaction in
    /// progress* with the recorded owner and phase.
    InProgress { owner: TransactionOwner, phase: Phase },
    /// **One take-over is permitted and it needs no timer: a claim naming the
    /// claimant's own installation identity.** `Desktop app and delivery` P2
    /// admits one ordinary running instance per user, so a claim written by this
    /// installation cannot also be held by a live process of it.
    TakenBackFromThisInstallation,
}

/// **G1a and G1b's one rule about meeting an existing claim.**
///
/// **A claim naming another installation is never taken over, whatever its age**
/// — that is §16 item 23's residual, and **it is the price of refusing a
/// deadline.** Nothing here reads a wall-clock age, and there is no age argument
/// to pass.
pub fn claim(standing: Option<&Claim>, claiming_installation: &Identity) -> ClaimOutcome {
    match standing {
        None => ClaimOutcome::Acquired,
        Some(held) if &held.owner.installation == claiming_installation => {
            ClaimOutcome::TakenBackFromThisInstallation
        }
        Some(held) => ClaimOutcome::InProgress {
            owner: held.owner.clone(),
            phase: held.phase,
        },
    }
}

/// **Where both performers could meet, the claim file is the single point and
/// the lock is the runtime's addition to it.**
///
/// So neither can begin while the other holds the file, and G1a's release-on-exit
/// remains what tells a later runtime that no live program is inside the
/// transaction.
pub const THE_CLAIM_FILE_IS_THE_SINGLE_POINT_AND_THE_LOCK_IS_AN_ADDITION: bool = true;

/// **A waiter's deadline may stop that waiter; it does not release another
/// process's lock and does not alter the transaction.**
pub const A_WAITERS_DEADLINE_RELEASES_NOTHING: () = ();

/// Taking and releasing the claim, behind a trait.
///
/// **The exclusive create and the kernel lock are the acting half's**; the rule
/// above is what they are for. The trait carries no timeout and no age, because
/// [`claim`] refuses both.
pub trait ClaimsTheTransaction {
    /// Attempts the claim. **[`WhoClaims`] says which instrument this
    /// implementation is**, and the two differ in what an unacquirable claim
    /// means: G1a's release-on-exit makes *live* exact, while under G1b *live*
    /// is read one degree weaker and §16 item 23 carries what that costs.
    fn take(&self, mine: &TransactionOwner) -> Result<ClaimOutcome, String>;

    /// **Removed at the terminal write, and by nothing else.**
    fn release_at_the_terminal_write(&self) -> Result<(), String>;
}

/// Writing the phase record, behind a trait — **G3's ordering is the caller's
/// and this is only the write.**
pub trait WritesThePhaseRecord {
    /// **Durable in `State, identity and recovery` §1.3 rule 6's sense**, which
    /// is where the word is defined and where this takes it from.
    fn write(&self, record: &PhaseRecord) -> Result<(), String>;
    fn read(&self) -> Result<Option<PhaseRecord>, String>;
}

#[cfg(test)]
mod tests {
    use smoozy_state::identities::IdentityKind;

    use super::*;
    use crate::package::release::VerifiedReleaseDescriptor;

    fn an_identity(kind: IdentityKind) -> Identity {
        Identity::issue(kind).expect("randomness")
    }

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

    fn an_owner(installation: Identity) -> TransactionOwner {
        TransactionOwner {
            invocation: an_identity(IdentityKind::Operation),
            route: Route::AutomaticRepair,
            process: (1493, 19186),
            installation,
        }
    }

    #[test]
    fn the_phase_set_is_closed_at_ten_and_each_says_what_a_later_reading_means() {
        // **The set is closed by the compiler rather than by a count.** The
        // match is total and carries no default arm, so a phase added to the
        // enum has none, the crate stops compiling, and whoever added it is sent
        // to this list — where the length assertion this replaced could never
        // have been false, the array's own type having fixed it.
        for (at, phase) in Phase::ALL.iter().enumerate() {
            let where_the_list_puts_it = match phase {
                Phase::Inspecting => 0,
                Phase::Staging => 1,
                Phase::StagedAndValidated => 2,
                Phase::Committing => 3,
                Phase::CommittedUnverified => 4,
                Phase::Verified => 5,
                Phase::RollingBack => 6,
                Phase::RolledBack => 7,
                Phase::FailedBeforeCommit => 8,
                Phase::Deferred => 9,
            };
            assert_eq!(at, where_the_list_puts_it, "{phase:?} is not where the list puts it");
        }
        for phase in Phase::ALL {
            assert!(!phase.what_a_later_reading_means().is_empty());
        }
    }

    #[test]
    fn a_fresh_attempts_first_write_carries_the_identity_the_values_and_the_phase_together() {
        let previous = PreviousVersion {
            descriptor: a_descriptor(),
            retained_adapter: RetainedAdapter {
                declared_interval: (1, 2),
                declared_request_set: vec!["inspect".into()],
            },
            location: "/opt/smoozy/previous".into(),
            previous_boot_registration_content: b"the old hook".to_vec(),
        };
        let attempt = an_identity(IdentityKind::Operation);
        let record = a_fresh_attempts_first_write(
            attempt.clone(),
            a_descriptor(),
            Some(previous.clone()),
            MachineKind::Systemd,
            "/opt/smoozy/staging".into(),
        );
        assert_eq!(record.attempt, attempt);
        assert_eq!(record.phase, Phase::Inspecting);
        assert_eq!(record.previous, Some(previous));
        // The owner is written only while the claim is held, so a fresh
        // record's owner field is empty rather than filled speculatively.
        assert!(record.owner.is_none());
    }

    #[test]
    fn a_first_placement_rolls_back_to_no_active_pointer_at_all() {
        let record = a_fresh_attempts_first_write(
            an_identity(IdentityKind::Operation),
            a_descriptor(),
            None,
            MachineKind::Container,
            "/opt/smoozy/staging".into(),
        );
        assert_eq!(
            what_a_rollback_restores(&record),
            WhatARollbackRestores::NoActivePointerAtAll
        );
    }

    #[test]
    fn an_unclaimed_transaction_is_acquired() {
        let installation = an_identity(IdentityKind::Installation);
        assert_eq!(claim(None, &installation), ClaimOutcome::Acquired);
    }

    #[test]
    fn a_claim_naming_another_installation_is_never_taken_over_whatever_its_age() {
        let mine = an_identity(IdentityKind::Installation);
        let theirs = an_identity(IdentityKind::Installation);
        let standing = Claim {
            owner: an_owner(theirs),
            phase: Phase::Staging,
        };
        assert!(matches!(
            claim(Some(&standing), &mine),
            ClaimOutcome::InProgress { .. }
        ));
    }

    #[test]
    fn a_claim_naming_this_installation_is_taken_back_with_no_timer() {
        let mine = an_identity(IdentityKind::Installation);
        let standing = Claim {
            owner: an_owner(mine.clone()),
            phase: Phase::Committing,
        };
        assert_eq!(
            claim(Some(&standing), &mine),
            ClaimOutcome::TakenBackFromThisInstallation
        );
    }

    #[test]
    fn the_material_a_rolled_back_attempt_leaves_is_reached_by_the_bounded_sweep_alone() {
        assert_eq!(
            which_removal_reaches(Phase::RolledBack, false),
            WhichRemovalReachesIt::G20sBoundedSweep
        );
        assert_eq!(
            which_removal_reaches(Phase::Verified, true),
            WhichRemovalReachesIt::TheLastStepOfAVerifiedCommit
        );
        assert_eq!(
            which_removal_reaches(Phase::Staging, true),
            WhichRemovalReachesIt::TheFirstActOfAFreshReplacement
        );
    }
}

//! §2.3 and §2.4 — the one deciding write, the activation step, the health
//! exchange, and what a failure restores.
//!
//! **G7. The pair problem Fact 78 names is removed rather than solved.** No
//! single operation updates two independent files atomically, so an arrangement
//! whose commit is two writes has to recover deterministically to one matching
//! pair. **This arrangement has one write: the boot registration is written to
//! start whatever the active-package pointer names, rather than to name a
//! package version.** So the registration's content does not change when the
//! version does, and the commit is the single rename of the pointer.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Security and licence` §8 requirement 3. `Server
//! protocol and transfers` C15, B2. `State, identity and recovery` §13 item 2,
//! and its §7 step 1, and its D1.

use smoozy_state::identities::Identity;

use super::phase::{Phase, PhaseRecord, PreviousVersion};
use crate::registration::BootRegistrationRecord;

/// **G8. Being startable through the fixed path the registration names is a
/// requirement on every package version and not only on the commit.**
///
/// A package version that cannot be started that way **is a version this
/// arrangement refuses at the *staged and validated* phase.** The refusal is a
/// failure before the commit and the machine is untouched.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhetherTheStagedVersionMayBeCommitted {
    ItStartsThroughTheFixedPath,
    /// **Refused at *staged and validated*.** The machine is untouched.
    RefusedFailedBeforeCommit,
}

/// **G8.**
pub fn may_be_committed(starts_through_the_fixed_path: bool) -> WhetherTheStagedVersionMayBeCommitted
{
    if starts_through_the_fixed_path {
        WhetherTheStagedVersionMayBeCommitted::ItStartsThroughTheFixedPath
    } else {
        WhetherTheStagedVersionMayBeCommitted::RefusedFailedBeforeCommit
    }
}

/// **G9. The one case where the registration's own content must change is a
/// placement whose registration shape differs from the one on the machine**, and
/// it is ordered rather than made atomic.
///
/// The registration is written and confirmed by K15's read-back **before** the
/// *committing* phase is entered, **in a form that starts whatever the pointer
/// names**, so it is correct for the active package as well as for the staged
/// one. **A crash between the two therefore leaves a registration that starts the
/// package that is still active, which is the state the machine was already
/// in.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhetherTheRegistrationChanges {
    /// G7's ordinary case: version-independent, so nothing about it changes.
    ItIsVersionIndependentAndDoesNotChange,
    /// G9's case, and **the only one in which G14 restores the registration**.
    ThisPlacementChangesItsShape,
}

/// **G9.** Whether this placement's registration content differs from the one
/// recorded on the machine.
pub fn whether_the_registration_changes(
    recorded_content: &[u8],
    this_placements_content: &[u8],
) -> WhetherTheRegistrationChanges {
    if recorded_content == this_placements_content {
        WhetherTheRegistrationChanges::ItIsVersionIndependentAndDoesNotChange
    } else {
        WhetherTheRegistrationChanges::ThisPlacementChangesItsShape
    }
}

/// **G10. The installation identity is written and made durable before the
/// pointer switch, not after and not beside it.**
///
/// `State, identity and recovery` §13 item 2 records the window this closes from
/// one side: an interrupted first commit could leave a committed package with no
/// installation identity, which that document's §7 step 1 would read as
/// **absent** and act on by discarding what the local record holds. **With this
/// ordering a machine that has a pointer always has an identity**, and the
/// remaining window is the reverse one — an identity with no pointer — which
/// reads as *present, and the supervisor did not answer*, **whose own branch
/// discards nothing**. The window is narrowed and not closed (§16 item 3).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheOrderedPair {
    /// The identity is durable; the pointer may be switched.
    TheIdentityIsDurableSoThePointerMayMove,
    /// **§14's declaration: where the identity cannot be written the pointer is
    /// *not attempted*, naming the identity as the unit that stopped it.**
    ThePointerIsNotAttempted,
}

/// **G10.**
pub fn the_ordered_pair(installation_identity_is_durable: bool) -> TheOrderedPair {
    if installation_identity_is_durable {
        TheOrderedPair::TheIdentityIsDurableSoThePointerMayMove
    } else {
        TheOrderedPair::ThePointerIsNotAttempted
    }
}

/// **G11. Nothing about the commit is inferred from a directory or a file being
/// there.**
///
/// The active-package pointer's target is **read as a value**; the registration
/// is **read as content** (K15); the installation identity is read from Smoozy's
/// own state area and **never from a path a provider also writes** (`Server
/// protocol and transfers` C15). The three traits below take those three shapes
/// and no presence answer exists among them.
pub trait ReadsWhatTheCommitRestsOn {
    /// The pointer's target, **as a value**. `None` is *no active pointer*,
    /// which is G6's safe branch and not an error.
    fn the_active_pointers_value(&self) -> Result<Option<String>, String>;
    /// The installation identity from Smoozy's own state area.
    fn the_installation_identity(&self) -> Result<Option<Identity>, String>;
}

/// **G7's deciding write, and G25's activation after it.**
///
/// **The rename does not put the new supervisor in place, so the transaction
/// does**, and it is an ordered, phase-recorded step of its own between the
/// deciding write and the health exchange. **A rename starts no process**, and
/// both restarters of §3.2 act **after an exit**. Without this step §2.4's
/// exchange would reach the previous package's supervisor on a replacement, and
/// nothing at all on a first placement, and G13 would read both as a failed
/// exchange — **so every runtime upgrade would roll back.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommitStep {
    /// **G10 first**: the installation identity, durable.
    TheInstallationIdentityIsMadeDurable,
    /// **G7**: the single rename of the pointer, which is the commit.
    TheDecidingWriteTheSingleRenameOfThePointer,
    /// The *committed, unverified* phase is written.
    TheCommittedUnverifiedPhaseIsWritten,
    /// **G25**: the running supervisor is ended where there is one. **A first
    /// placement has no supervisor to end and the step is the start alone.**
    TheRunningSupervisorIsEnded,
    /// **G25**: the restarter of §3.2 starts a supervisor — the user unit on a
    /// systemd machine, the keeper on a container — which under G7 is whatever
    /// the pointer now names.
    TheRestarterStartsTheSupervisorThePointerNames,
    /// **G12**: and only then is the exchange attempted.
    TheHealthExchangeIsAttempted,
}

impl CommitStep {
    /// **The order, and it is the whole of G25's own sentence.**
    pub const IN_ORDER: [CommitStep; 6] = [
        CommitStep::TheInstallationIdentityIsMadeDurable,
        CommitStep::TheDecidingWriteTheSingleRenameOfThePointer,
        CommitStep::TheCommittedUnverifiedPhaseIsWritten,
        CommitStep::TheRunningSupervisorIsEnded,
        CommitStep::TheRestarterStartsTheSupervisorThePointerNames,
        CommitStep::TheHealthExchangeIsAttempted,
    ];
}

/// **G25's own safety, named rather than assumed: what makes the ending safe is
/// G21**, which has already established that no live or held work depends on the
/// runtime before the *committing* phase was entered.
pub const WHAT_MAKES_THE_ENDING_SAFE_IS_G21: bool = true;

/// **What a supervisor's start does with the machine's stores**, which is
/// G15a's whole branch.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatThisStartDoesWithTheStores {
    /// The stores are at this program's own version: it reconciles and serves
    /// as it always has.
    ReconcilesAndServes,
    /// **Older stores, and the phase record is anything but *verified***: it
    /// reads and writes none of them, answers G12's four values, and refuses
    /// every other request as *not yet ready*. Reconciliation does not run
    /// (`State, identity and recovery` §6 step 0a).
    AnswersOnlyTheHealthExchangeUntilVerified,
    /// **Older stores, and *verified* is written**: it upgrades them step by
    /// step, then reconciles, then serves.
    UpgradesThenReconcilesThenServes,
    /// **Stores newer than this program**: it opens none of them and says so,
    /// answering only the health exchange (`State, identity and recovery` §1.3
    /// rule 7). Nothing Smoozy does reaches this; it is written for the case
    /// nothing plans.
    OpensNoneAndAnswersOnlyTheHealthExchange,
}

/// **G15a. The machine's stores are upgraded to the new program's shapes only
/// after *verified* is written, and never inside the transaction.**
///
/// **The order is what keeps a rollback honest**: G14 puts the previous program
/// back in front of the stores, and a previous program must never meet a store
/// the newer one rewrote. **After *verified* there is no rollback to meet the
/// upgraded stores**, G15 having removed the previous package, so no copy is
/// kept. **G25's activation starts a supervisor that does exactly this**: it
/// finds the phase at *committed, unverified* and touches no store until the
/// desktop has written *verified*.
///
/// **The case this was first written for** is a supervisor started on stores
/// its own program wrote, which is every start there has been. **The case it
/// now also meets** is a supervisor of a newer release started on stores an
/// older one wrote, inside the transaction that placed it.
pub fn what_this_start_does_with_the_stores(
    stores: smoozy_state::store_version::TheStoresAgainstThisProgram,
    phase: Phase,
) -> WhatThisStartDoesWithTheStores {
    use smoozy_state::store_version::TheStoresAgainstThisProgram as Stores;
    match stores {
        Stores::AtItsOwnVersion => WhatThisStartDoesWithTheStores::ReconcilesAndServes,
        Stores::Newer { .. } => {
            WhatThisStartDoesWithTheStores::OpensNoneAndAnswersOnlyTheHealthExchange
        }
        Stores::Older { .. } if phase == Phase::Verified => {
            WhatThisStartDoesWithTheStores::UpgradesThenReconcilesThenServes
        }
        Stores::Older { .. } => {
            WhatThisStartDoesWithTheStores::AnswersOnlyTheHealthExchangeUntilVerified
        }
    }
}

/// **A program, as the kernel identifies the file it runs**: the device the
/// file sits on and its inode number, which together name one file however it
/// was reached. **Never a path and never a name**, because the active-package
/// pointer is a link and a process started through it reports whatever the
/// kernel reports, and two paths can name one file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TheFileAProgramIs {
    pub device: u64,
    pub inode: u64,
}

/// **Who holds the machine-local endpoint, read from the kernel.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TheEndpointsHolder {
    /// Nothing is listening at the endpoint's recorded path.
    NothingHoldsIt,
    /// A process holds it, with the program it runs where that could be read.
    AProcess {
        process: u32,
        runs: Result<TheFileAProgramIs, String>,
    },
}

/// **What G25's ending does, decided from the kernel's two readings.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheEndingDoes {
    /// **A first placement's case**: there is no supervisor to end, and the
    /// step is the start alone.
    ThereIsNothingToEnd,
    /// **The holder runs the program the pointer names**, so it is the newly
    /// placed supervisor and a repeat of the ending ends nothing.
    ItAlreadyRunsThePointersProgram,
    /// **The holder runs another program, and it is ended.**
    EndIt { process: u32 },
    /// **One of the two programs could not be identified, so nothing is
    /// ended.** The plan's safety is *only where that process is not running
    /// the program the pointer names*, and a reading that cannot tell is not a
    /// reading that it is not. G13's exchange then decides.
    ItCannotBeTold(String),
}

/// **G25, struck and replaced on 2026-09-28: on a container the ending is
/// performed by the program the pointer now names, and it asks the old
/// program nothing.**
///
/// After the *committed, unverified* phase is written, the constant
/// entry-point command already reaches the **new** program, because the pointer
/// has moved. That one short-lived invocation finds the supervisor holding the
/// machine-local endpoint from the machine itself and ends it — **only where
/// that process is not running the program the pointer names**, so a repeat
/// after the new supervisor has started finds nothing to end and the newly
/// placed program cannot be taken down. **Both readings come from the kernel**
/// and neither from any record or answer of the old program's; the acts that
/// take them are `crate::acting::package`'s, with their publications quoted
/// there.
///
/// **The case this was first written for** was a supervisor that understood
/// the ending request and ended itself. **The case it now also meets** is a
/// supervisor of a program too old to know that request, or too old to be
/// spoken to at all (`Server protocol and transfers` P3b).
///
/// **It is still the one deliberate end S6 names**, performed by the
/// transaction, and **what makes it safe to end at all is G21, unchanged.**
pub fn what_the_ending_does(
    holder: &TheEndpointsHolder,
    the_pointers_program: &Result<TheFileAProgramIs, String>,
) -> WhatTheEndingDoes {
    let TheEndpointsHolder::AProcess { process, runs } = holder else {
        return WhatTheEndingDoes::ThereIsNothingToEnd;
    };
    match (runs, the_pointers_program) {
        (Err(why), _) => WhatTheEndingDoes::ItCannotBeTold(format!(
            "the program the endpoint's holder runs could not be read: {why}"
        )),
        (_, Err(why)) => WhatTheEndingDoes::ItCannotBeTold(format!(
            "the program the pointer names could not be read: {why}"
        )),
        (Ok(runs), Ok(named)) if runs == named => {
            WhatTheEndingDoes::ItAlreadyRunsThePointersProgram
        }
        (Ok(_), Ok(_)) => WhatTheEndingDoes::EndIt { process: *process },
    }
}

/// **G12. The health exchange asks for four things and takes nothing else as
/// evidence.**
///
/// It is one request over the machine-local endpoint to the supervisor the commit
/// has just activated, **and it is framed like any other request**: `Security and
/// licence` §8 requirement 3 binds it — *"The endpoint is not a trusted channel
/// that skips validation; it is a channel whose peer is known."*
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HealthAnswer {
    /// The package version the supervisor was built from.
    pub package_version: String,
    /// Its run identifier.
    pub run: Identity,
    /// The installation identity it read from the state area.
    pub installation: Identity,
    /// The phase its own reading of the phase record gives.
    pub phase: Phase,
}

/// **G13's outcome, and the caller does not distinguish the two failures.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum HealthExchange {
    /// **All four agree with the phase record.**
    Succeeded,
    /// **A supervisor that answers with a different package version, a different
    /// installation identity or a phase the record does not carry is a failed
    /// exchange, not a healthy machine** — and **a supervisor that does not
    /// answer at all is also a failed exchange.** Both restore.
    Failed,
}

/// **G13. The exchange succeeds only where all four agree with the phase
/// record.**
///
/// The run identifier is the fourth thing asked for and is not compared against
/// the record — the record holds no run identifier, and S2 writes a fresh one at
/// every start — so what it does is name **which** run answered; a caller that
/// compared it against anything would be comparing against a value nothing here
/// holds.
pub fn the_health_exchange(
    record: &PhaseRecord,
    answered: Option<&HealthAnswer>,
    the_installation_this_machine_holds: &Identity,
) -> HealthExchange {
    let Some(answer) = answered else {
        return HealthExchange::Failed;
    };
    let version_agrees =
        answer.package_version == record.requested.requested_package_version;
    let installation_agrees = &answer.installation == the_installation_this_machine_holds;
    let phase_agrees = answer.phase == record.phase;
    if version_agrees && installation_agrees && phase_agrees {
        HealthExchange::Succeeded
    } else {
        HealthExchange::Failed
    }
}

/// What a failed exchange does, and what a successful one does.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheExchangeProduces {
    /// **G14. A failed exchange restores the previous pointer from G4's recorded
    /// value, and the phase record moves to *rolling back* before it.** The
    /// previous package is present because G5 keeps it until *verified*, so the
    /// restore is **the same single rename in the other direction**.
    RestoreFromTheRecordedPreviousValue {
        // Boxed only so that the successful branch, which is the ordinary one,
        // does not carry the previous version's whole descriptor around with
        // it. Nothing about the clause turns on it.
        previous: Box<PreviousVersion>,
        /// **The boot registration is restored only in G9's case**, where this
        /// placement changed its content. In every other case G7 has left it
        /// version-independent and there is nothing about it to restore, **which
        /// is the whole reason the pointer is the one deciding write.**
        ///
        /// **The record that describes the registration rides on this same flag
        /// rather than beside it, because the two are one act.** Where this is
        /// true both go back; where it is false there is nothing to restore on
        /// either half. A second flag would be the same fact in two places, and
        /// two flags that could disagree is exactly the state the clause
        /// completing G14 exists to stop — a machine on the previous
        /// registration with the record still holding this placement's content,
        /// which K15 then reads as *content differs* for ever. **The marker for
        /// that clause sits at [`crate::arriving_later`]** and is written out
        /// there rather than here, for the reason that entry gives: no performer
        /// in this crate carries the restore out.
        the_registration_is_restored_too: bool,
    },
    /// G6's case: there is no previous version, so the rollback leaves no active
    /// pointer.
    RestoreLeavesNoActivePointer,
    /// **G15. The previous package is removed only after the exchange
    /// succeeds**, which is the shape's own rule and the reason G5 keeps exactly
    /// one.
    RemoveThePreviousPackage,
}

/// **G14 and G15.**
pub fn what_the_exchange_produces(
    record: &PhaseRecord,
    exchange: HealthExchange,
    registration_changed: WhetherTheRegistrationChanges,
) -> WhatTheExchangeProduces {
    match exchange {
        HealthExchange::Succeeded => WhatTheExchangeProduces::RemoveThePreviousPackage,
        HealthExchange::Failed => match &record.previous {
            None => WhatTheExchangeProduces::RestoreLeavesNoActivePointer,
            Some(previous) => WhatTheExchangeProduces::RestoreFromTheRecordedPreviousValue {
                previous: Box::new(previous.clone()),
                the_registration_is_restored_too: registration_changed
                    == WhetherTheRegistrationChanges::ThisPlacementChangesItsShape,
            },
        },
    }
}

/// **The boot-registration record restored with the registration, from the same
/// recorded value and in the same case.**
///
/// **G14a**, and this is the decision that clause asks for. **The marker is
/// written here from 2026-09-07**, the day the restore was built; until then it
/// sat on [`crate::arriving_later`]'s list of clauses no working file of this
/// crate names, because nothing anywhere carried the restore out. **What carries
/// it out is the desktop's own placement performer**, in `Server protocol and
/// transfers`, which names the clause at the site that performs it. **No
/// performer in this crate does** — [`PerformsTheCommit`] has no restore of
/// anything — so what is here is the judgement and not the act.
///
/// K16's record holds the registration's content by value and K15 compares what
/// is on the machine against it, **so a restore that puts the previous content
/// back on the machine and leaves the record holding this placement's content
/// makes every later read-back answer *content differs*** — silently at the
/// moment it happens, permanently afterwards, on a machine the plan calls
/// correct. **The previous content is already in the phase record by value**
/// (G4), which is what the registration itself is restored from, so the record
/// is restored from that same value in the same act.
///
/// **What comes from where, and neither half is read off the machine.** The
/// content is the previous version's, carried forward by value; the kind and the
/// path are the standing record's, because a rollback does not move a
/// registration from one path to another — G9's case is a placement whose
/// registration *content* differs, and the path a kind's registration sits at is
/// that kind's. **Nothing here reads the machine**, which is G14's own reason: a
/// rollback runs in exactly the case where what is on the machine is no longer
/// trustworthy.
pub fn the_record_a_rollback_restores(
    standing: &BootRegistrationRecord,
    previous: &PreviousVersion,
) -> BootRegistrationRecord {
    BootRegistrationRecord {
        kind: standing.kind,
        at_path: standing.at_path.clone(),
        content: previous.previous_boot_registration_content.clone(),
    }
}

/// **The order the restore takes, and it is K16b's in this direction as in the
/// other.**
///
/// **G14a is the clause that adds the record to the restore**, and this order is
/// the half of it that says which of the pair goes first. *(The marker moved
/// here from [`crate::arriving_later`] on 2026-09-07, when the restore was
/// built.)*
///
/// K16b: *"The record is written durably before the registration it describes,
/// and the same order holds when a rollback restores the pair."* It is
/// `State, identity and recovery` D1's *intent before acting*, and **the two
/// orders are not equally safe.** Interrupted after the record and before the
/// registration, the record holds content the machine does not, which is exactly
/// what K15 compares and it fails the placement before the commit. Interrupted
/// the other way round, the previous boot bytes are live on the machine with a
/// record naming this placement's — a true reading of the records and a false
/// statement about the machine.
///
/// **An interrupted restore is not a new state**, because G16 already holds the
/// transaction at *rolling back* until the next request completes it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheRestoresTwoSteps {
    /// **First**, and durably in `State, identity and recovery` §1.3 rule 6's
    /// sense.
    TheRecordHoldingThePreviousContent,
    /// **Then** the registration's own bytes on the machine.
    TheRegistrationItself,
}

impl TheRestoresTwoSteps {
    /// **K16b's order, in the rollback direction.**
    pub const IN_ORDER: [TheRestoresTwoSteps; 2] = [
        TheRestoresTwoSteps::TheRecordHoldingThePreviousContent,
        TheRestoresTwoSteps::TheRegistrationItself,
    ];
}

/// **G16. A rollback that cannot complete is not a silent one.**
///
/// The phase record stays at *rolling back*, the request answers with that
/// phase, and **the next request under any identity reads the phase and completes
/// the rollback before considering anything else.** Nothing starts a fresh
/// transaction over an incomplete rollback.
pub fn nothing_starts_a_fresh_transaction_over(phase: Phase) -> bool {
    phase == Phase::RollingBack
}

/// The health exchange itself, behind a trait.
///
/// **One request over the machine-local endpoint**, framed like any other. **Not
/// answering is an answer** — G13 makes it the same failure as a wrong answer —
/// so the trait returns `Ok(None)` for it rather than an error the caller might
/// treat differently.
pub trait AsksForTheHealthAnswer {
    fn ask(&self) -> Result<Option<HealthAnswer>, String>;
}

/// The one deciding write and the activation, behind a trait.
///
/// **There is one rename here and no second write beside it**, which is G7's
/// whole answer to Fact 78.
pub trait PerformsTheCommit {
    /// **G7.** The single rename of the pointer.
    fn rename_the_pointer_to(&self, target: &str) -> Result<(), String>;
    /// **G25.** End the running supervisor where there is one; a first placement
    /// has none.
    fn end_the_running_supervisor(&self) -> Result<(), String>;
    /// **G25.** Ask the restarter of §3.2 to start the supervisor the pointer
    /// now names. **Where it produces no answering supervisor inside the
    /// caller's own deadline, that is G13's *did not answer* branch and nothing
    /// here softens it.**
    fn start_the_supervisor_the_pointer_names(&self) -> Result<(), String>;
}

#[cfg(test)]
mod tests {
    use smoozy_state::identities::IdentityKind;

    use super::*;
    use crate::machine_kind::MachineKind;
    use crate::package::phase::a_fresh_attempts_first_write;
    use crate::package::release::{RetainedAdapter, VerifiedReleaseDescriptor};

    fn an_identity(kind: IdentityKind) -> Identity {
        Identity::issue(kind).expect("randomness")
    }

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

    fn a_previous() -> PreviousVersion {
        PreviousVersion {
            descriptor: a_descriptor(),
            retained_adapter: RetainedAdapter {
                declared_interval: (1, 2),
                declared_request_set: vec!["inspect".into()],
            },
            location: "/opt/smoozy/previous".into(),
            previous_boot_registration_content: b"the old hook".to_vec(),
        }
    }

    fn a_record(previous: Option<PreviousVersion>) -> PhaseRecord {
        let mut record = a_fresh_attempts_first_write(
            an_identity(IdentityKind::Operation),
            a_descriptor(),
            previous,
            MachineKind::Systemd,
            "/opt/smoozy/staging".into(),
        );
        record.phase = Phase::CommittedUnverified;
        record
    }

    #[test]
    fn the_commit_is_six_ordered_steps_and_the_activation_sits_between_the_rename_and_the_exchange()
    {
        let order = CommitStep::IN_ORDER;
        let rename = order
            .iter()
            .position(|step| *step == CommitStep::TheDecidingWriteTheSingleRenameOfThePointer)
            .expect("the rename is a step");
        let start = order
            .iter()
            .position(|step| {
                *step == CommitStep::TheRestarterStartsTheSupervisorThePointerNames
            })
            .expect("the start is a step");
        let exchange = order
            .iter()
            .position(|step| *step == CommitStep::TheHealthExchangeIsAttempted)
            .expect("the exchange is a step");
        assert!(rename < start && start < exchange);
        // G10: the identity is durable before the pointer moves.
        assert_eq!(order[0], CommitStep::TheInstallationIdentityIsMadeDurable);
    }

    #[test]
    fn a_supervisor_that_does_not_answer_is_the_same_failure_as_one_that_answers_wrongly() {
        let installation = an_identity(IdentityKind::Installation);
        let record = a_record(Some(a_previous()));
        assert_eq!(
            the_health_exchange(&record, None, &installation),
            HealthExchange::Failed
        );
        let wrong_version = HealthAnswer {
            package_version: "0.1.0".into(),
            run: an_identity(IdentityKind::Run),
            installation: installation.clone(),
            phase: Phase::CommittedUnverified,
        };
        assert_eq!(
            the_health_exchange(&record, Some(&wrong_version), &installation),
            HealthExchange::Failed
        );
    }

    #[test]
    fn a_different_installation_identity_is_a_failed_exchange() {
        let installation = an_identity(IdentityKind::Installation);
        let record = a_record(Some(a_previous()));
        let answer = HealthAnswer {
            package_version: "0.2.0".into(),
            run: an_identity(IdentityKind::Run),
            installation: an_identity(IdentityKind::Installation),
            phase: Phase::CommittedUnverified,
        };
        assert_eq!(
            the_health_exchange(&record, Some(&answer), &installation),
            HealthExchange::Failed
        );
    }

    #[test]
    fn all_four_agreeing_succeeds() {
        let installation = an_identity(IdentityKind::Installation);
        let record = a_record(Some(a_previous()));
        let answer = HealthAnswer {
            package_version: "0.2.0".into(),
            run: an_identity(IdentityKind::Run),
            installation: installation.clone(),
            phase: Phase::CommittedUnverified,
        };
        assert_eq!(
            the_health_exchange(&record, Some(&answer), &installation),
            HealthExchange::Succeeded
        );
    }

    #[test]
    fn a_failed_exchange_restores_the_registration_only_where_this_placement_changed_it() {
        let record = a_record(Some(a_previous()));
        let ordinary = what_the_exchange_produces(
            &record,
            HealthExchange::Failed,
            WhetherTheRegistrationChanges::ItIsVersionIndependentAndDoesNotChange,
        );
        match ordinary {
            WhatTheExchangeProduces::RestoreFromTheRecordedPreviousValue {
                the_registration_is_restored_too,
                ..
            } => assert!(!the_registration_is_restored_too),
            other => panic!("expected a restore, got {other:?}"),
        }
        let g9 = what_the_exchange_produces(
            &record,
            HealthExchange::Failed,
            WhetherTheRegistrationChanges::ThisPlacementChangesItsShape,
        );
        match g9 {
            WhatTheExchangeProduces::RestoreFromTheRecordedPreviousValue {
                the_registration_is_restored_too,
                ..
            } => assert!(the_registration_is_restored_too),
            other => panic!("expected a restore, got {other:?}"),
        }
    }

    #[test]
    fn the_previous_package_is_removed_only_after_the_exchange_succeeds() {
        let record = a_record(Some(a_previous()));
        assert_eq!(
            what_the_exchange_produces(
                &record,
                HealthExchange::Succeeded,
                WhetherTheRegistrationChanges::ItIsVersionIndependentAndDoesNotChange
            ),
            WhatTheExchangeProduces::RemoveThePreviousPackage
        );
    }

    #[test]
    fn nothing_starts_a_fresh_transaction_over_an_incomplete_rollback() {
        assert!(nothing_starts_a_fresh_transaction_over(Phase::RollingBack));
        assert!(!nothing_starts_a_fresh_transaction_over(Phase::RolledBack));
    }

    #[test]
    fn a_version_that_cannot_start_through_the_fixed_path_is_refused_before_the_commit() {
        assert_eq!(
            may_be_committed(false),
            WhetherTheStagedVersionMayBeCommitted::RefusedFailedBeforeCommit
        );
    }

    #[test]
    fn without_the_identity_the_pointer_is_not_attempted() {
        assert_eq!(the_ordered_pair(false), TheOrderedPair::ThePointerIsNotAttempted);
    }

    /// **The record a rollback restores holds the previous content and the
    /// standing record's kind and path**, and neither half is read off the
    /// machine — which is G14's own reason: a rollback runs in exactly the case
    /// where what is on the machine is no longer trustworthy.
    #[test]
    fn the_record_a_rollback_restores_carries_the_previous_content_at_the_standing_path() {
        let previous = a_previous();
        let standing = BootRegistrationRecord {
            kind: MachineKind::Container,
            at_path: "/root/onstart.sh".to_string(),
            content: b"this placement's hook".to_vec(),
        };
        let restored = the_record_a_rollback_restores(&standing, &previous);
        assert_eq!(restored.kind, standing.kind);
        assert_eq!(restored.at_path, standing.at_path);
        assert_eq!(
            restored.content,
            previous.previous_boot_registration_content
        );
        // **Without it, K15's read-back answers *content differs* for ever**:
        // the machine would hold the previous content and the record this
        // placement's.
        assert_ne!(standing.content, previous.previous_boot_registration_content);
    }

    /// **The record goes back before the registration it describes**, which is
    /// K16b's order in this direction as in the other.
    #[test]
    fn the_restore_writes_the_record_before_the_registration() {
        assert_eq!(
            TheRestoresTwoSteps::IN_ORDER,
            [
                TheRestoresTwoSteps::TheRecordHoldingThePreviousContent,
                TheRestoresTwoSteps::TheRegistrationItself,
            ]
        );
    }

    #[test]
    fn identical_registration_content_is_the_version_independent_case() {
        assert_eq!(
            whether_the_registration_changes(b"hook", b"hook"),
            WhetherTheRegistrationChanges::ItIsVersionIndependentAndDoesNotChange
        );
        assert_eq!(
            whether_the_registration_changes(b"hook", b"a different hook"),
            WhetherTheRegistrationChanges::ThisPlacementChangesItsShape
        );
    }

    /// **G15a over every phase**: older stores are touched only once
    /// *verified* is written, and newer ones never.
    #[test]
    fn older_stores_wait_for_verified_and_newer_ones_are_never_opened() {
        use smoozy_state::store_version::{StoreVersion, TheStoresAgainstThisProgram as S};
        let older = S::Older {
            at: StoreVersion(1),
            own: StoreVersion(2),
        };
        let newer = S::Newer {
            at: StoreVersion(3),
            own: StoreVersion(2),
        };
        for phase in [
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
        ] {
            assert_eq!(
                what_this_start_does_with_the_stores(S::AtItsOwnVersion, phase),
                WhatThisStartDoesWithTheStores::ReconcilesAndServes
            );
            assert_eq!(
                what_this_start_does_with_the_stores(newer, phase),
                WhatThisStartDoesWithTheStores::OpensNoneAndAnswersOnlyTheHealthExchange
            );
            assert_eq!(
                what_this_start_does_with_the_stores(older, phase),
                if phase == Phase::Verified {
                    WhatThisStartDoesWithTheStores::UpgradesThenReconcilesThenServes
                } else {
                    WhatThisStartDoesWithTheStores::AnswersOnlyTheHealthExchangeUntilVerified
                },
                "{phase:?}"
            );
        }
    }

    /// **G25's ending ends only a holder running another program than the
    /// pointer's, and never on a reading it could not take.**
    #[test]
    fn the_ending_ends_only_a_holder_of_another_program() {
        let new = TheFileAProgramIs { device: 1, inode: 10 };
        let old = TheFileAProgramIs { device: 1, inode: 9 };
        assert_eq!(
            what_the_ending_does(&TheEndpointsHolder::NothingHoldsIt, &Ok(new)),
            WhatTheEndingDoes::ThereIsNothingToEnd
        );
        assert_eq!(
            what_the_ending_does(
                &TheEndpointsHolder::AProcess { process: 7, runs: Ok(old) },
                &Ok(new)
            ),
            WhatTheEndingDoes::EndIt { process: 7 }
        );
        assert_eq!(
            what_the_ending_does(
                &TheEndpointsHolder::AProcess { process: 7, runs: Ok(new) },
                &Ok(new)
            ),
            WhatTheEndingDoes::ItAlreadyRunsThePointersProgram
        );
        // **The same inode on another device is another file.**
        assert_eq!(
            what_the_ending_does(
                &TheEndpointsHolder::AProcess {
                    process: 7,
                    runs: Ok(TheFileAProgramIs { device: 2, inode: 10 })
                },
                &Ok(new)
            ),
            WhatTheEndingDoes::EndIt { process: 7 }
        );
        for (runs, named) in [
            (Err("gone".to_string()), Ok(new)),
            (Ok(old), Err("no pointer".to_string())),
        ] {
            assert!(matches!(
                what_the_ending_does(&TheEndpointsHolder::AProcess { process: 7, runs }, &named),
                WhatTheEndingDoes::ItCannotBeTold(_)
            ));
        }
    }
}

//! Interruption, resumption and the safe re-run — `Server Setup` §4.
//!
//! **The constraint** is Row 114 — *"Running a step again is always safe.
//! Whether the last attempt finished, stopped part-way, or was never started,
//! running it again puts the machine right rather than making it worse"* — with
//! Fact 50 answered **NO against a stub with no guard**. **It is kept.** The
//! negative was a run that was admitted while the state stood, started from the
//! beginning, and cleared a state it did not own; **each of those three is
//! closed by construction below**, and the per-step rules close what remains.
//!
//! **Markers used below that belong to another document, each with the document
//! it belongs to.** `Rented-server runtime` A5b, A11, A11a, G19, G20, M7, M8,
//! M24, P1, P2, P5, P7, P8, X9, X9a, §9, §11, §16 item 4.
//! `Security and licence` §13 rule 5. `Server protocol and transfers` §7.1,
//! S22, S24. `State, identity and recovery` §1.1, §2.3, D2, D4.
//!
//! **`Rented-server runtime` R1 and R2, both in that document's §13, are not
//! listed above**, because this plan defines an R1 and an R2 of its own and a
//! bare one could mean either;
//! where this file cites one, the document is written out at the site.

use smoozy_framing::command_string::CommandString;

use crate::ready_action::Step;
use crate::run::Boundary;

/// **S1: how far a part-finished setup got, read fresh on the connection the
/// user opens and never from anything Smoozy remembered.**
///
/// **The reading distinguishes *a step never started* from *a step that stopped
/// part-way***: the boundary names the last step completed; the operation
/// record and the journal name the step that was in flight and how the run
/// ended, where it ended at all.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhereAStepGot {
    /// Every step up to and including the marker — done and verified.
    Done,
    /// The step after the marker, where the run ended in it.
    StoppedPartWay,
    /// Every step after that.
    NeverStarted,
}

/// **S1, performed over a run's composed list and its last-reached marker.**
///
/// **The three kinds are Row 360's**, and they come from the boundary and the
/// marker rather than from anything second.
pub fn how_far_it_got(
    composed: &[Boundary],
    last_reached: Option<Boundary>,
    the_step_in_flight: Option<Step>,
) -> Vec<(Boundary, WhereAStepGot)> {
    let reached_at = last_reached.and_then(|reached| composed.iter().position(|it| *it == reached));
    composed
        .iter()
        .enumerate()
        .map(|(at, boundary)| {
            let where_it_got = match reached_at {
                Some(reached) if at <= reached => WhereAStepGot::Done,
                _ if the_step_in_flight == Some(boundary.its_step()) => {
                    WhereAStepGot::StoppedPartWay
                }
                _ => WhereAStepGot::NeverStarted,
            };
            (*boundary, where_it_got)
        })
        .collect()
}

/// **S2: *what it managed* is the machine's own record and has no second
/// home.**
///
/// The desktop holds its pending-command records and nothing else durable about
/// a sitting — `State, identity and recovery` §1.1 is the bound, **and this
/// document adds no store to it.** What the user is shown is composed from the
/// fresh reading: the boundary list, the last one reached with its moment, and
/// the run's ending where one was recorded.
///
/// **A machine that was recycled underneath reads as *absent* at the
/// installation-identity gate** and every remembered value is disqualified
/// before any of this is read (`State, identity and recovery` §2.3) — the re-run
/// this document defines is what the user meets there, on a machine that is
/// genuinely bare again.
pub const THIS_DOCUMENT_ADDS_NO_DESKTOP_STORE: bool = true;

/// **S2a: the machine's own words for a step that failed are part of the run's
/// recorded ending, on the machine, and are read back with the boundary list.**
///
/// **Where they live: the setup operation record's per-kind field, beside the
/// boundary list, written in the same durable act as the ending that names the
/// failed step.** Not the event journal — its entries are structured at a
/// 512-byte budget (`Rented-server runtime` R2, in its §13) and raw tool output is
/// neither — and not §11's capture sinks, which are per render attempt and are
/// not borrowed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TheFailedStepsWords {
    /// **A machine-side step's words are the worker's own capture** —
    /// [`crate::blender`] B1 already captures both streams of every Blender run
    /// this module makes — reported to the supervisor, which writes.
    TheWorkersOwnCapture { output: Vec<u8>, complaint: Vec<u8> },
    /// **An elevated step's process is the connection's child**
    /// ([`crate::driving`] Y7), so its streams land in the desktop's own loop
    /// and nowhere on the machine; **the desktop writes the ending with the
    /// words through the ordinary command path** — the same route that writes
    /// the stop's durable state.
    TheDesktopsOwnLoop { output: Vec<u8>, complaint: Vec<u8> },
    /// **Where the failure is Smoozy's own verification rather than a tool's
    /// refusal** — a digest that did not match, an `ldd` re-read that did not
    /// come back clean, a version answer that was not the expected one — **the
    /// recorded fact of that read is the ending's content, and no tool words
    /// are claimed for it.**
    SmoozysOwnVerification { what_was_read: String },
    /// **A step cut mid-flight has no words anywhere, and none are claimed.**
    /// The elevated process died with its channel and its printed text with the
    /// desktop's loop.
    None,
}

impl TheFailedStepsWords {
    /// **S2a: verbatim, whole or not at all, streams kept apart** —
    /// `Security and licence` §13 rule 5 and `Rented-server runtime` M7 and
    /// M8's discipline, cited rather than restated: **never mined for a cause,
    /// never merged, never trimmed.**
    ///
    /// **A write the state area cannot take whole retains no fragment**; the
    /// ending is recorded either way, saying the words were not kept — **the
    /// ending never waits on the words.**
    pub fn kept_whole_or_not_at_all(self, the_area_can_take_them_whole: bool) -> TheFailedStepsWords {
        if the_area_can_take_them_whole {
            self
        } else {
            TheFailedStepsWords::None
        }
    }
}

/// **S2a: the two cases are told apart by the ending's own kind and never by
/// the words' absence.**
///
/// *"A step that failed is an ending that names it, with the words beside it
/// where any were printed; a run that was cut is that other ending."*
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HowTheRunEnded {
    /// The final verification passed and the state (where set) was cleared.
    Completed,
    /// **A step failed**, named, with the words beside it where any were
    /// printed.
    AStepFailed {
        step: Step,
        words: TheFailedStepsWords,
    },
    /// **The kind's own ending for a run that was cut**, written when
    /// [`crate::driving`]'s window elapses or at [`take_over`]'s hand — and it
    /// **carries no words**.
    EndedAtBoundaryNotCompleted { at: Option<Boundary> },
    /// **Left stopped**, from the user's own durable stop.
    LeftStopped { at: Option<Boundary> },
    /// **[`crate::blender`] B7a: declined at the version answer** — a recorded
    /// ending, not a failure and not one of `statements` in `smoozy-server-setup` W1's five.
    DeclinedAtTheVersionAnswer,
    /// **S6's second clear: a recovery the user asked to abandon the
    /// machine-changing work.** Added 2026-09-22.
    ///
    /// *"Which still runs the final verification first and reports what it
    /// found"* — so this ending carries the finding: the verification's own
    /// words where it did not pass, and none where it passed. **It is not a
    /// failure** and it never reaches *done*, so Y6a's card reads the server as
    /// not set up.
    Abandoned { what_the_check_found: TheFailedStepsWords },
}

impl HowTheRunEnded {
    /// **S2a: whether this ending may carry the failed step's words at all.**
    pub fn may_carry_words(&self) -> bool {
        // **An abandon's finding rides its ending the same way** (S6's *"reports
        // what it found"*), added 2026-09-22.
        matches!(self, HowTheRunEnded::AStepFailed { .. } | HowTheRunEnded::Abandoned { .. })
    }
}

/// **S3: *carries on where it left off* is a resumption from the recorded
/// boundary, driven over the re-made connection.**
///
/// **Nothing on the machine resumes by itself**: reconciliation is held by the
/// state, and the machine's only autonomous act is holding everything back.
pub fn anything_on_the_machine_resumes_by_itself() -> bool {
    false
}

/// **S4 item 1: admission is the guard Fact 50's stub lacked.**
///
/// While the machine-level state is set, a fresh setup submission is **refused
/// at the admission point and not queued** (`Rented-server runtime` A5b) — the
/// hold-back being that document's P2 and A11, with [`crate::run`] Y1's
/// only-the-named-run exception (A11a) — and the refusal names **the operation
/// the standing state holds**, on the identity `Rented-server runtime` P1
/// carries. **No error, no silent second worker, no two runs in flight.**
///
/// **The boundary list and its last-reached marker are [`crate::run`] Y6's
/// reading and the health answer's, not the refusal's.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WhatTheRefusalCarries {
    /// The operation identity the standing state holds.
    pub the_operation_the_state_holds: bool,
    /// **Never**: the boundary and its moment come from Y6's two routes.
    pub the_boundary_and_its_moment: bool,
}

/// **S4 item 1, performed.**
pub const WHAT_A_REFUSED_SUBMITTER_LEARNS: WhatTheRefusalCarries = WhatTheRefusalCarries {
    the_operation_the_state_holds: true,
    the_boundary_and_its_moment: false,
};

/// **S4 item 2: the route forward is the recovery run, and it takes the
/// standing state over rather than clearing it.**
///
/// A recovery command — an ordinary command with its own operation identity,
/// naming the setup operation the state names — has the supervisor: validate the
/// recorded group; write the named operation's recorded outcome; **re-point the
/// machine-level state at the recovery run's own operation identity, and in that
/// same durable act write the composed boundary list and the last-reached marker
/// into the recovery run's *own* operation record's per-kind field**; and start
/// the recovery run's worker.
///
/// **Clearing first and starting fresh would run reconciliation
/// (`Rented-server runtime` P5) and start the interrupted job while the machine
/// is still being changed underneath it, which is Row 110's forbidden outcome;
/// the take-over is required, not stylistic.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheTakeOver {
    /// The outcome written for the operation the state named — *left stopped*
    /// from its durable stop, or the kind's own *ended at boundary N*.
    pub the_taken_over_runs_recorded_outcome: HowTheRunEnded,
    /// **Written into the recovery's own record**, so a second interruption
    /// before it writes a boundary of its own still resumes from the inherited
    /// one rather than from the beginning.
    pub the_inherited_boundary_list: Vec<Boundary>,
    pub the_inherited_marker: Option<Boundary>,
    /// **The take-over gives the recovery the taken-over run's kind**
    /// ([`crate::run`] Y6b).
    pub the_kind_it_inherits: crate::run::SetupKind,
    /// **The state is never cleared in between, so no admission window ever
    /// opens.**
    pub the_state_is_cleared_in_between: bool,
}

/// **S4 item 2, performed.**
pub fn take_over(
    taken_over: &crate::run::TheSetupRecord,
    the_user_had_stopped_it: bool,
) -> TheTakeOver {
    let marker = taken_over.the_last_boundary_reached().map(|it| it.boundary);
    TheTakeOver {
        the_taken_over_runs_recorded_outcome: if the_user_had_stopped_it {
            HowTheRunEnded::LeftStopped { at: marker }
        } else {
            HowTheRunEnded::EndedAtBoundaryNotCompleted { at: marker }
        },
        // **Y4's composed list, which is what the run consisted of** — never
        // the boundaries it actually reached, whose end is the marker below.
        the_inherited_boundary_list: taken_over.the_composed_list.clone(),
        the_inherited_marker: marker,
        the_kind_it_inherits: taken_over.kind,
        the_state_is_cleared_in_between: false,
    }
}

/// **S4 item 3: the recovery run resumes at the recorded boundary and never
/// before it.**
///
/// *"Steps at or before the marker are not run again; the step after it runs
/// afresh under its own re-run rule."*
pub fn where_the_recovery_begins(
    inherited: &[Boundary],
    marker: Option<Boundary>,
) -> Vec<Boundary> {
    let Some(marker) = marker else {
        return inherited.to_vec();
    };
    match inherited.iter().position(|it| *it == marker) {
        Some(at) => inherited[at + 1..].to_vec(),
        // A marker the composed list does not hold is not a marker this run
        // may resume from, and the run begins where it was composed to begin.
        None => inherited.to_vec(),
    }
}

/// **S5: a recorded group that still validates refuses the recovery, and that
/// is the safe direction of the one reading known to be wrong.**
///
/// **The defunct-entry hole** means a dead setup group can read as live until
/// the next supervisor start; **it can never read as dead while live**
/// (`Rented-server runtime` §16 item 4 — X9's other direction holds). So a
/// recovery refused because the pair validates **costs a wait, never a second
/// run beside a live one.**
pub fn the_recovery_is_refused(the_recorded_group_still_validates: bool) -> bool {
    the_recorded_group_still_validates
}

/// **S5: what unblocks it, said exactly, because two of the three things once
/// named there do not.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatUnblocksARefusedRecovery {
    /// **The unblock that holds in the ordinary case: the group genuinely
    /// ending** — the worker ends its run at a boundary under
    /// [`crate::driving`] Y7.
    TheGroupGenuinelyEnding,
    /// **A machine restart on the systemd kind also reaps**, and
    /// `restart` in `smoozy-server-setup` T5 is what follows one — **but this module never
    /// issues a restart to unblock this**, because `restart` in `smoozy-server-setup` T1 issues
    /// one only as part of a setup step that needs it and the setup run is
    /// precisely what is being refused; and on a container no restart is issued
    /// at all.
    AMachineRestartWhichThisModuleNeverIssuesForIt,
    /// **The reap `Rented-server runtime` X9a now requires**, which is the
    /// live-supervisor case's route out. *"Nothing in any document this one may
    /// read establishes that a supervisor restart reaps the workers it
    /// started"* — what was recorded is the opposite — **so a supervisor restart
    /// is not carried as an unblock at all.**
    TheReapThatDocumentNowRequires,
}

impl WhatUnblocksARefusedRecovery {
    pub const ALL: [WhatUnblocksARefusedRecovery; 3] = [
        WhatUnblocksARefusedRecovery::TheGroupGenuinelyEnding,
        WhatUnblocksARefusedRecovery::AMachineRestartWhichThisModuleNeverIssuesForIt,
        WhatUnblocksARefusedRecovery::TheReapThatDocumentNowRequires,
    ];
}

/// **S5: `fallback` in `smoozy-server-setup` F1 step 3's plain start does not reach it
/// either** — that step fires only where the supervisor does **not** answer, and
/// here it answers.
///
/// **Plain false, in the shape every other prohibition in this crate uses.** It
/// used to read its argument and throw the answer away with `&& false`, which
/// always answered false — right, but written to look as though the argument
/// decided something.
pub fn the_plain_start_unblocks_it(_the_supervisor_answers: bool) -> bool {
    false
}

/// **S6: only two things clear the state, and the set is closed.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatClearsTheState {
    /// **A run reaching *done*.**
    ARunReachingDone,
    /// **A recovery run the user asked to abandon the machine-changing work**
    /// — which still runs the final verification first and reports what it
    /// found.
    ARecoveryTheUserAskedToAbandonTheMachineChangingWork,
}

impl WhatClearsTheState {
    /// **The two, and nothing else clears it, on the strength of nothing**
    /// (`Rented-server runtime` P7).
    pub const BOTH: [WhatClearsTheState; 2] = [
        WhatClearsTheState::ARunReachingDone,
        WhatClearsTheState::ARecoveryTheUserAskedToAbandonTheMachineChangingWork,
    ];

    /// **Both clears run reconciliation** (`Rented-server runtime` P5).
    pub fn runs_reconciliation(self) -> bool {
        true
    }
}

/// **S7: one rule spans every step below that installs a package** — the
/// tooling step, the libraries step and the driver route on either family
/// alike.
///
/// *"An install interrupted mid-transaction can leave the machine's package
/// system needing its own repair first, and that step's re-run begins with the
/// repair operation"* **where the package system reports itself interrupted.**
pub const THE_REPAIR_OPERATION: [CommandString; 2] = [
    CommandString::constant("dpkg --configure -a"),
    CommandString::constant("apt-get -o DPkg::Lock::Timeout=120 --fix-broken install"),
];

/// **S7: `dpkg --remove --force-all` is forbidden as the first act of any
/// re-run.**
///
/// It was recorded once as an observed recovery (Fact 48's container case) and
/// written here as the rule; **dpkg's own manual is why it cannot be**: *"These
/// options are mostly intended to be used by experts only. Using them without
/// fully understanding their effects may break your whole system."*, and `all`
/// turns on `remove-essential` — *"Removing them might cause the whole system to
/// stop working"* — and `remove-protected` — *"Removing them might cause the
/// whole system to be unable to boot"*. **A forced removal opening every re-run
/// sits directly against Row 114's promise.**
///
/// **`Server Setup` §13 item 5's guard: the documented sequence must never be
/// re-labelled *observed*, and the forced form must never come back** on the
/// reasoning that the documented one is untested.
pub fn the_forced_removal_may_open_a_re_run(_it_would_work: bool) -> bool {
    false
}

/// **S7b: a package step distinguishes *busy* from *broken* before it repairs
/// anything, and this is the ordinary case rather than the rare one.**
///
/// Ubuntu installs `unattended-upgrades` by default and runs it at startup —
/// Ubuntu Server documentation, read 2026-08-20: *"unattended-upgrades may often
/// run on system startup and thereby cause immediate activity and prevent other
/// package operations from taking place at that time"*, naming the
/// counter-productive case as *"VM images that are only started for some quick
/// action"*, **which is a rented render box.**
///
/// **A package operation blocked by the frontend lock fails rather than waits**:
/// apt's `DPkg::Lock::Timeout` defaults to `0` and its lock acquisition returns
/// immediately at that value, the refusal reading *"Unable to acquire the dpkg
/// frontend lock (%s), is another process using it?"*. **So every package step
/// of a setup run sets a bounded wait rather than accepting the default**, and a
/// step that still cannot take the lock is reported failed as *the machine is
/// busy with its own updates* — **never repaired, because nothing is broken.**
pub const THE_LOCK_TIMEOUT_OPTION: &str = "-o DPkg::Lock::Timeout=120";

/// **S7b and S7c: the bounded wait's own length.**
///
/// **`Server Setup` §13 item 19 says the value is not established** — the
/// setting is documented in apt's source and in neither `apt.conf(5)` nor
/// `apt-get(8)`, so its behaviour at a non-zero value is documented nowhere this
/// project may read, and closing it wants a machine to watch. **This is the
/// value the one run that exercised it used**, rather than a fresh number: S7c
/// records `apt-get -o DPkg::Lock::Timeout=120 update` returning **0 in 31
/// seconds** on a Debian virtual machine at 108 seconds of uptime, with the
/// frontend lock reported **not held** at that moment.
///
/// **What that run did not exercise is the collision it exists for**: the
/// automatic-update timers were enabled on that image and the nearer one was
/// **17 minutes away**, so the run did not collide with a job actually holding
/// the lock. **What was observed is that the setting is accepted and the
/// ordinary boot does not collide.**
///
/// **Neither direction damages a machine** (§13 item 19): too short and a
/// machine still finishing its startup updates is reported busy when waiting a
/// little longer would have served; too long and a genuinely stuck lock delays
/// the run's honest failure.
pub const THE_LOCK_TIMEOUT_SECONDS: u64 = 120;

/// **S7b's outcome for a step the lock refused.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatAPackageStepMet {
    /// The lock was taken and the step ran.
    ItTookTheLock,
    /// **Busy**: the bounded wait elapsed and the lock was still held. Reported
    /// failed as *the machine is busy with its own updates*, **and nothing is
    /// repaired**.
    TheMachineIsBusyWithItsOwnUpdates,
    /// **Broken**: the package system reports itself interrupted, which is what
    /// the repair operation is for.
    ThePackageSystemReportsItselfInterrupted,
}

/// **S7b, performed: busy before broken.**
///
/// **Without this, a step that failed only because the machine was busy re-runs
/// into the repair operation against a package system that was never damaged.**
pub fn what_a_package_step_met(
    the_lock_was_still_held_when_the_wait_elapsed: bool,
    the_package_system_reports_itself_interrupted: bool,
) -> WhatAPackageStepMet {
    if the_lock_was_still_held_when_the_wait_elapsed {
        return WhatAPackageStepMet::TheMachineIsBusyWithItsOwnUpdates;
    }
    if the_package_system_reports_itself_interrupted {
        return WhatAPackageStepMet::ThePackageSystemReportsItselfInterrupted;
    }
    WhatAPackageStepMet::ItTookTheLock
}

/// **S7b's own vindication, both ways** — `unattended-upgrades` turns out to be
/// a property of the **image** rather than of either family, observed on both
/// sides on 2026-08-22: Hyperstack's `Debian 12 Plain` carries it installed with
/// both timers enabled, and Vast's `ubuntu:24.04` container carries none of it,
/// no configuration file and no timers. **So no family may be read as carrying
/// or not carrying it**, and the rule probes the lock rather than assuming it is
/// held.
pub fn a_family_may_be_read_as_carrying_the_automatic_updates(_family_is_ubuntu: bool) -> bool {
    false
}

/// **S7: why each step is safe to run again, per step, with the mechanism
/// named.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhyTheStepIsSafeToRunAgain {
    /// **fetch** — not a `Server protocol and transfers` §7 transfer, and the
    /// mechanism is not borrowed. What binds is §7.1's four contract clauses:
    /// the archive is staged and never at a final name; the whole is verified
    /// against the published checksum before anything is unpacked; **a resume is
    /// by byte range into the same staging path**, with the published checksum
    /// as the only verification, a whole-file checksum being unable to verify a
    /// prefix; and **on a mismatch the staged material is abandoned and the
    /// fetch starts over into fresh staging, never repaired in place** (S24).
    /// **Either way nothing half-fetched is ever installed.**
    TheStagedArchiveIsVerifiedWholeBeforeAnythingIsUnpacked,
    /// **tooling / unpack** — an install cut off part-way was run again over
    /// the same folder with nothing deleted first and produced a working
    /// Blender the same as a clean one — **observed, one archive, one machine,
    /// one time**. The unpack is into staging, so even that case is narrower
    /// here than the observation needed. **That observation covers the unpack
    /// half alone**; the step's other half is an elevated package install and
    /// is covered by the repair precondition.
    TheUnpackIsIntoStagingAndTheRepairPreconditionCoversTheInstall,
    /// **libraries** — the package installs are re-issued as the same named
    /// list; **a package already installed is not work repeated.** The
    /// package-system repair precondition applies in full.
    TheSameNamedListReIssued,
    /// **driver** — the re-run is the same check-first order as the first run:
    /// **a driver that answers is never touched**, so the
    /// reinstall-becomes-upgrade harm is unreachable from a re-run **by
    /// construction**; a driver that does not answer gets the restart before any
    /// second install.
    TheSameCheckFirstOrder,
    /// **restart** — idempotent in effect; a second restart costs its return
    /// time and nothing else (Fact 51). **A recovery resuming at the restart
    /// step re-runs the check first and issues the restart only where it fails
    /// again**, because whether a restart is owed is a reading of the machine
    /// now and never an outcome recorded in a previous sitting (Row 104;
    /// `State, identity and recovery` D2).
    TheCheckIsReadAfreshAndNeverTakenFromTheRecord,
    /// **blender publication** — publication is one rename, and a re-run
    /// re-validates staged material by content before reusing a byte
    /// (`Rented-server runtime` G19's rule applied to this document's
    /// material).
    StagedMaterialIsRevalidatedByContent,
    /// **telegram material** — the placement is file placement plus a
    /// desired-state write, both idempotent; replacement deliveries reuse the
    /// same vehicle.
    BothHalvesAreIdempotent,
    /// **blender removal** — `Server Setup` B15's plain deletion of the
    /// install's folder. **Deleting a folder that is already gone does
    /// nothing**, so a re-run finishes what a cut deletion left, or finds
    /// nothing to do. Added 2026-10-10, the 38's piece 14c.
    AFolderAlreadyGoneIsNothingToDo,
}

/// **S7, performed: the mechanism that makes each step safe.**
pub fn why_safe_to_run_again(step: Step) -> Option<WhyTheStepIsSafeToRunAgain> {
    match step {
        Step::Fetch => Some(WhyTheStepIsSafeToRunAgain::TheStagedArchiveIsVerifiedWholeBeforeAnythingIsUnpacked),
        Step::Tooling => Some(
            WhyTheStepIsSafeToRunAgain::TheUnpackIsIntoStagingAndTheRepairPreconditionCoversTheInstall,
        ),
        Step::Libraries => Some(WhyTheStepIsSafeToRunAgain::TheSameNamedListReIssued),
        Step::Driver => Some(WhyTheStepIsSafeToRunAgain::TheSameCheckFirstOrder),
        Step::Restart => {
            Some(WhyTheStepIsSafeToRunAgain::TheCheckIsReadAfreshAndNeverTakenFromTheRecord)
        }
        Step::BlenderPublication => {
            Some(WhyTheStepIsSafeToRunAgain::StagedMaterialIsRevalidatedByContent)
        }
        Step::TelegramMaterial => Some(WhyTheStepIsSafeToRunAgain::BothHalvesAreIdempotent),
        Step::BlenderRemoval => Some(WhyTheStepIsSafeToRunAgain::AFolderAlreadyGoneIsNothingToDo),
        // The readings and the final verification perform nothing and have
        // nothing to be safe about.
        Step::Readings | Step::Done => None,
    }
}

/// **S7: the package-system repair precondition, and which steps carry it.**
///
/// **The driver step carries it too, from 2026-09-04.** S7's enumeration read
/// *the tooling step and the libraries step alike* until then, and it was
/// written before M1b made the Debian driver route a package install of its own
/// — `apt-get install nvidia-driver`, with a list update before it — while the
/// Ubuntu route drives the same package system. **So the driver step had no
/// repair precondition**: an install cut off mid-transaction left the package
/// system needing its own repair, and the step's re-run walked straight into it.
/// **S7b already said the wider thing** of the bounded wait — *every package
/// step of a setup run* — so two clauses of one section instructed differently
/// about one set of steps.
pub fn the_repair_precondition_applies_to(step: Step) -> bool {
    matches!(step, Step::Tooling | Step::Libraries | Step::Driver)
}

/// **S7: what the mechanism costs** — the boundary records and their
/// verification reads (a read per step), the admission refusal and its report,
/// and the recovery command. **No user-facing cost the spec does not already
/// grant, and no second store anywhere.**
pub const THE_MECHANISM_ADDS_NO_SECOND_STORE: bool = true;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::run::{RecordedBoundary, SetupKind, TheSetupRecord, WhatTheStepDid};
    use smoozy_state::observed::{Clock, MachineClock};

    /// A record whose composed list is Y4's full one and whose reached list is
    /// what the run actually got through — **two fields, never one**.
    fn a_record(reached: &[Boundary]) -> TheSetupRecord {
        TheSetupRecord {
            kind: SetupKind::TheReadyAction,
            the_composed_list: Boundary::IN_ORDER.to_vec(),
            the_boundaries_reached: reached
                .iter()
                .map(|boundary| RecordedBoundary {
                    boundary: *boundary,
                    what_the_step_did: WhatTheStepDid::Performed,
                    at: std::time::SystemTime::UNIX_EPOCH,
                    clock: Clock::AMachine(MachineClock("box-1".to_string())),
                })
                .collect(),
            ending: None,
        }
    }

    #[test]
    fn a_never_started_step_is_told_apart_from_one_that_stopped_part_way() {
        // S1, and Row 360's three kinds.
        let composed = [
            Boundary::Started,
            Boundary::Fetched,
            Boundary::Tooling,
            Boundary::Libraries,
            Boundary::Done,
        ];
        let read = how_far_it_got(&composed, Some(Boundary::Fetched), Some(Step::Tooling));
        assert_eq!(read[0].1, WhereAStepGot::Done);
        assert_eq!(read[1].1, WhereAStepGot::Done);
        assert_eq!(read[2].1, WhereAStepGot::StoppedPartWay);
        assert_eq!(read[3].1, WhereAStepGot::NeverStarted);
        assert_eq!(read[4].1, WhereAStepGot::NeverStarted);
    }

    #[test]
    fn a_run_that_reached_no_boundary_has_nothing_done() {
        let composed = [Boundary::Started, Boundary::Fetched];
        let read = how_far_it_got(&composed, None, None);
        assert!(read.iter().all(|(_, got)| *got == WhereAStepGot::NeverStarted));
    }

    #[test]
    fn the_take_over_never_clears_the_state_in_between() {
        // S4 item 2: clearing first would run reconciliation and start the
        // interrupted job while the machine is still being changed underneath
        // it, which is Row 110's forbidden outcome.
        let taken = take_over(&a_record(&[Boundary::Started, Boundary::Fetched]), false);
        assert!(!taken.the_state_is_cleared_in_between);
        assert_eq!(taken.the_inherited_marker, Some(Boundary::Fetched));
        // **The inherited list is Y4's composed list and never the boundaries
        // reached**, or the recovery would resume with no steps at all.
        assert_eq!(taken.the_inherited_boundary_list, Boundary::IN_ORDER.to_vec());
        assert!(!where_the_recovery_begins(
            &taken.the_inherited_boundary_list,
            taken.the_inherited_marker
        )
        .is_empty());
        assert_eq!(taken.the_kind_it_inherits, SetupKind::TheReadyAction);
        assert!(matches!(
            taken.the_taken_over_runs_recorded_outcome,
            HowTheRunEnded::EndedAtBoundaryNotCompleted { .. }
        ));
    }

    #[test]
    fn a_run_the_user_stopped_is_written_left_stopped() {
        let taken = take_over(&a_record(&[Boundary::Started]), true);
        assert!(matches!(
            taken.the_taken_over_runs_recorded_outcome,
            HowTheRunEnded::LeftStopped { .. }
        ));
    }

    #[test]
    fn the_recovery_resumes_after_the_marker_and_never_before_it() {
        let inherited = [
            Boundary::Started,
            Boundary::Fetched,
            Boundary::Tooling,
            Boundary::Done,
        ];
        assert_eq!(
            where_the_recovery_begins(&inherited, Some(Boundary::Fetched)),
            vec![Boundary::Tooling, Boundary::Done]
        );
        // A second interruption before the recovery writes a boundary of its
        // own still resumes from the inherited one rather than from the
        // beginning.
        assert_eq!(
            where_the_recovery_begins(&inherited, None),
            inherited.to_vec()
        );
    }

    #[test]
    fn a_group_that_still_validates_refuses_the_recovery_and_that_costs_a_wait() {
        // S5's safe direction: the reading can report a dead group live and
        // never a live one dead.
        assert!(the_recovery_is_refused(true));
        assert!(!the_recovery_is_refused(false));
        assert_eq!(WhatUnblocksARefusedRecovery::ALL.len(), 3);
        assert!(!the_plain_start_unblocks_it(true));
        assert!(!the_plain_start_unblocks_it(false));
    }

    #[test]
    fn exactly_two_things_clear_the_state_and_both_run_reconciliation() {
        assert_eq!(WhatClearsTheState::BOTH.len(), 2);
        for clear in WhatClearsTheState::BOTH {
            assert!(clear.runs_reconciliation());
        }
    }

    #[test]
    fn the_repair_operation_is_the_documented_pair_and_never_the_forced_form() {
        // S7, and §13 item 5's guard.
        assert_eq!(THE_REPAIR_OPERATION[0].as_str(), "dpkg --configure -a");
        assert!(THE_REPAIR_OPERATION[1].as_str().contains("--fix-broken"));
        for command in THE_REPAIR_OPERATION {
            assert!(!command.as_str().contains("--force-all"));
        }
        assert!(!the_forced_removal_may_open_a_re_run(true));
    }

    #[test]
    fn busy_is_decided_before_broken_and_a_busy_machine_is_never_repaired() {
        // S7b. Without this a step that failed only because the machine was
        // busy re-runs into the repair against a package system that was never
        // damaged.
        assert_eq!(
            what_a_package_step_met(true, true),
            WhatAPackageStepMet::TheMachineIsBusyWithItsOwnUpdates
        );
        assert_eq!(
            what_a_package_step_met(false, true),
            WhatAPackageStepMet::ThePackageSystemReportsItselfInterrupted
        );
        assert_eq!(
            what_a_package_step_met(false, false),
            WhatAPackageStepMet::ItTookTheLock
        );
    }

    #[test]
    fn no_family_is_read_as_carrying_the_automatic_updates() {
        // S7b's own vindication, both ways: it is a property of the image.
        assert!(!a_family_may_be_read_as_carrying_the_automatic_updates(true));
        assert!(!a_family_may_be_read_as_carrying_the_automatic_updates(false));
    }

    #[test]
    fn every_step_that_performs_something_has_a_named_re_run_mechanism() {
        for step in Step::IN_ORDER {
            match step {
                Step::Readings | Step::Done => assert!(why_safe_to_run_again(step).is_none()),
                _ => assert!(why_safe_to_run_again(step).is_some()),
            }
        }
        // S7, widened 2026-09-04: every step that installs a package, the
        // driver route on either family included.
        assert!(the_repair_precondition_applies_to(Step::Tooling));
        assert!(the_repair_precondition_applies_to(Step::Libraries));
        assert!(the_repair_precondition_applies_to(Step::Driver));
        // And no step that installs nothing carries it.
        for step in [
            Step::Readings,
            Step::Fetch,
            Step::Restart,
            Step::BlenderPublication,
            Step::BlenderRemoval,
            Step::TelegramMaterial,
            Step::Done,
        ] {
            assert!(!the_repair_precondition_applies_to(step));
        }
    }

    #[test]
    fn a_step_cut_mid_flight_carries_no_words_and_the_ending_says_which_it_was() {
        // S2a: the two cases are told apart by the ending's own kind and never
        // by the words' absence.
        let cut = HowTheRunEnded::EndedAtBoundaryNotCompleted {
            at: Some(Boundary::Tooling),
        };
        assert!(!cut.may_carry_words());
        let failed = HowTheRunEnded::AStepFailed {
            step: Step::Libraries,
            words: TheFailedStepsWords::None,
        };
        assert!(failed.may_carry_words());
    }

    #[test]
    fn words_the_area_cannot_take_whole_retain_no_fragment() {
        let words = TheFailedStepsWords::TheWorkersOwnCapture {
            output: b"a".to_vec(),
            complaint: b"b".to_vec(),
        };
        assert_eq!(
            words.clone().kept_whole_or_not_at_all(false),
            TheFailedStepsWords::None
        );
        assert_ne!(
            words.kept_whole_or_not_at_all(true),
            TheFailedStepsWords::None
        );
    }

    #[test]
    fn a_refused_submitter_learns_the_operation_and_not_the_boundary() {
        // S4 item 1, corrected 2026-08-29: the boundary list and its marker are
        // Y6's reading and the health answer's.
        const { assert!(WHAT_A_REFUSED_SUBMITTER_LEARNS.the_operation_the_state_holds) };
        const { assert!(!WHAT_A_REFUSED_SUBMITTER_LEARNS.the_boundary_and_its_moment) };
        assert!(!anything_on_the_machine_resumes_by_itself());
        const { assert!(THIS_DOCUMENT_ADDS_NO_DESKTOP_STORE) };
        const { assert!(THE_MECHANISM_ADDS_NO_SECOND_STORE) };
    }
}

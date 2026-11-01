//! §3 — the supervisor: one process, one writer, one run identifier; what
//! restarts it; and reconciliation, which runs at more moments than a start.
//!
//! **S1's serialization is what makes every rule in this crate a rule rather
//! than a hope**, and it is what answers Row 53's race at the point the shape
//! puts it.
//!
//! **The arrangement that makes work survive a reboot also undoes a deliberate
//! stop.** Observed: two seconds after a clean stop, an unconditional restart
//! directive started a fresh process and the work carried on. **Only the
//! supervisor is boot-restarted; every worker outcome comes from durable
//! state.**
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `State, identity and recovery` §1.2, §1.3 rules 1,
//! 4 and 5, §4 E1, §5.2, §5.3 step 6, §6, D2, D5. `Job files and outputs` C1,
//! C7, C15, C27, C29. `Rendering` — its ending step. `Queue & History` H8,
//! N5, P1b, Q4, Q8. `Server protocol and transfers` F6a. `Desktop app and
//! delivery` §16.3.
//!
//! **A marker this plan also defines is never listed here and is written out
//! with its document at the site**, because a header list cannot tell the two
//! apart and a bare one there would resolve to a real and wrong clause.

use smoozy_state::identities::Identity;
use smoozy_state::machine_operations::{
    DesiredState, Disposition, OperationRecord, RecordedOutcome,
};

use crate::execution_group::{RecordedPair, ValidatesTheRecordedPair};
use crate::machine_kind::MachineKind;

/// **S1. One supervisor per installation, and it is the only writer of every
/// durable store in `State, identity and recovery` §1.2 except the staging areas
/// and the captured streams, both of which are written under its control.**
///
/// **The captured streams are material rather than state and §11.1 states why
/// the writer is the worker**; §16 item 5 records that §1.3 rule 1 names the
/// staging areas and not them, and that seam closed at the Architecture seam
/// close.
pub const THE_SUPERVISOR_IS_THE_ONLY_WRITER: bool = true;

/// **S2. The supervisor writes a fresh run identifier at every start, whatever
/// caused the start** (`State, identity and recovery` §5.2).
///
/// **A recorded execution group carrying a previous run identifier is treated as
/// gone unless it validates** (X1's pair, and §6).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SupervisorRun {
    pub run: Identity,
    /// The kind detected for this installation's placement (K2), which chooses
    /// which restarter is behind this run (S4 or S5).
    pub machine_kind: MachineKind,
}

/// **S3. A second supervisor never runs.**
///
/// Where a start's bind of the machine-local endpoint is refused because another
/// process holds it, **that start ends without touching a store and without
/// signalling anything.**
///
/// **The bind is not the interlock — Z12 is, and it always was.** A bind refusal
/// carries no information about whether anything holds the endpoint: the same
/// address-in-use result is given for a live holder and for a stale filesystem
/// object left by a crashed run. **All of the separating is done by Z12's connect
/// probe.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhetherThisStartContinues {
    ItContinues,
    /// **Without touching a store and without signalling anything.**
    ItEndsBecauseAnotherSupervisorHoldsTheEndpoint,
}

/// **What restarts the supervisor, and it differs by machine kind.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Restarter {
    /// **S4. On a systemd machine the user unit carries an unconditional restart
    /// directive, and that is correct for the supervisor and forbidden for a
    /// worker.** Batch A's constraint is exact about the distinction: the restart
    /// policy that recovered work also restarted it two seconds after a
    /// deliberate stop. **That restart is the commonest reason K18 exists** —
    /// without K18, every restart this directive performs takes the running
    /// render down with the supervisor it is restarting.
    TheUserUnitsUnconditionalRestartDirective,
    /// **S5. On a container the boot hook starts a keeper, and the keeper's only
    /// job is to start the supervisor and start it again after an unexpected
    /// exit.** It is needed because the container's first process is a shell
    /// waiting on a marker file rather than an init system. **The keeper holds no
    /// policy of its own**: it never starts, stops or restarts a worker, never
    /// reads a store, and never decides an outcome.
    TheKeeperTheBootHookStarts,
}

impl Restarter {
    /// **There is exactly one restarter per machine kind**, which is the
    /// arrangement §3.2 exists to have exactly one of.
    pub fn for_kind(kind: MachineKind) -> Restarter {
        match kind {
            MachineKind::Systemd => Restarter::TheUserUnitsUnconditionalRestartDirective,
            MachineKind::Container => Restarter::TheKeeperTheBootHookStarts,
        }
    }
}

/// **S5a. That the keeper can outlive the connection that started it is observed
/// on the container kind, at the promise level.**
///
/// A detached process took its own session, the connection was closed, and on a
/// new connection it was still running, reparented to the first process, still
/// appending to its log, and a polite stop was accepted. **The negative would
/// have been the program dying with the session on the container kind, which is
/// where it was most likely**, and it did not.
///
/// **What this does not establish is survival of a restart**: on the same batch
/// the plain detached process was gone after a reboot while the service-manager
/// registration came back, **which is why the boot hook is the registration
/// rather than the detachment** (K4, K18).
pub const DETACHMENT_IS_NOT_A_REGISTRATION: bool = true;

/// **S6. Neither restarter distinguishes a crash from a deliberate end, and that
/// is what the package transaction uses.**
///
/// **The supervisor has exactly one deliberate end and it is G25's**: the
/// transaction ends the running supervisor after the pointer has moved, and the
/// restarter starts the one the pointer now names. **No user action reaches it**
/// (G24), and nothing else on the machine ends it deliberately.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhyTheSupervisorEnded {
    /// The only deliberate one.
    G25sPackageCommit,
    /// **S7. Crash return is answered on both machine kinds** — the supervisor
    /// returned after a real kernel panic on the virtual machine and after the
    /// provider-equivalent forcible restart on the container. **What remains is a
    /// genuine power cut**, which is §16 item 13 and which no rental can pose.
    ACrashOrAForcibleRestart,
}

/// **S6.** What a restarter does with the ending, and it is the same either way.
///
/// **What a start does with the work it finds is §3.3's and not the
/// restarter's.**
pub fn what_the_restarter_does(_why: WhyTheSupervisorEnded) -> &'static str {
    "starts the supervisor again"
}

/// **The moments reconciliation runs, and there are exactly two.**
///
/// **S8. Reconciliation is `State, identity and recovery` §6 entire, taken and
/// not restated**, including Step 0, its four dispositions, its two
/// prohibitions, and its requirement that a disposition is journalled every time
/// it is written.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReconciliationMoment {
    /// **S9's first**: every supervisor start.
    EverySupervisorStart,
    /// **S9's second, and it is not optional**: the moment the machine-level
    /// *setup in progress* state is cleared (§9 P5).
    TheSetupInProgressStateIsCleared,
}

impl ReconciliationMoment {
    /// **S10. Nothing else triggers it.** No timer, no free-space reading, no
    /// connection, and no reading of a job's output folder — the last is §6's own
    /// prohibition.
    pub const THE_ONLY_TWO: [ReconciliationMoment; 2] = [
        ReconciliationMoment::EverySupervisorStart,
        ReconciliationMoment::TheSetupInProgressStateIsCleared,
    ];
}

/// **The two dispositions that end a job with no worker alive to have taken the
/// ending step** — S11's *only those two*.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DispositionThatEndsAJob {
    /// Written from a durable *should be stopped*.
    LeftStopped,
    /// At the second interruption.
    GivenUp,
}

/// **S11. A disposition that ends a job carries the completeness statement, taken
/// after the disposition is decided and written in the same act.**
///
/// For those two dispositions, and only those two, **the supervisor runs the
/// reckoning of `Job files and outputs` C1 and C7 — one walk of that job's output
/// folder against the expected-output inventory for the map in force — and writes
/// its statement (C27's four sets) with the recorded outcome**, exactly as a
/// worker's ending step carries one (`Rendering` K13).
///
/// **A render's recorded outcome carries a second thing in that same act**, on
/// these two dispositions and on the ordinary ending alike: V12's free-space
/// reading against the job's own output folder.
///
/// **The statement itself is `Job files and outputs`' and is carried here rather
/// than composed** — see [`crate::arriving_later::WHAT_IS_ROUTED_TO_ANOTHER_CRATE`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletenessStatement {
    /// C27's four sets, carried opaquely. **Nothing here reads inside it**, which
    /// is what keeps §1.3 rule 5 standing (S12).
    pub the_four_sets: Vec<(String, Vec<String>)>,
}

/// **S12. §1.3 rule 5 stands and is not weakened by S11, because nothing about
/// state is decided from the walk.**
///
/// **The disposition is chosen from records alone — the desired state, the
/// recorded execution group, the attempt count — precisely as §6's table has it,
/// before the walk is taken.** The walk supplies a measurement that rides with an
/// outcome already decided; **it can move no branch of that table, and reversing
/// the order would be the thing rule 5 forbids.**
///
/// **The measurement's own limits ride with it and are not softened here** —
/// `Job files and outputs` C15, a file short but not empty still counts, and C29,
/// a write that ran out of room counts as present.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnEndingOutcome {
    pub disposition: DispositionThatEndsAJob,
    /// **S13. Where the walk cannot be taken, the outcome is written without a
    /// statement and says so.** The job output folder gone, the mount absent, the
    /// walk raising — **none of it holds up the disposition**, which is decided
    /// and written whatever the walk does.
    ///
    /// **An outcome carrying no statement is not an outcome carrying an empty
    /// one**, and `Queue & History` H8 already turns on that difference: no drop
    /// is written against a missing reading. **The absence is recorded explicitly
    /// rather than left to be inferred from four empty sets.**
    pub completeness: Option<CompletenessStatement>,
    /// V12's reading, where one could be taken. Same rule: absent is not zero.
    pub free_space: Option<smoozy_framing::free_space::FreeSpaceReading>,
}

impl AnEndingOutcome {
    /// **S13.** Whether this outcome carries a statement at all — which is the
    /// distinction `Queue & History` H8 reads, and which four empty sets would
    /// destroy.
    pub fn carries_a_completeness_statement(&self) -> bool {
        self.completeness.is_some()
    }
}

/// **S12's ordering, made a type rather than a sentence.**
///
/// The disposition is decided from records; only then is the walk taken. A
/// caller cannot build [`AnEndingOutcome`] the other way round, because the walk
/// takes the decided disposition as its input.
pub fn decide_then_walk<W>(
    disposition: DispositionThatEndsAJob,
    walk: W,
) -> AnEndingOutcome
where
    W: FnOnce(DispositionThatEndsAJob) -> (Option<CompletenessStatement>, Option<smoozy_framing::free_space::FreeSpaceReading>),
{
    let (completeness, free_space) = walk(disposition);
    AnEndingOutcome {
        disposition,
        completeness,
        free_space,
    }
}

/// **S14. A durable write refused for want of room does not end the supervisor's
/// run, and what could not be written is held and re-attempted until it lands.**
///
/// **Every disk-full clause in this stage was written about a supervisor that
/// cannot start**; a supervisor already running when the state mount fills is
/// past the directory entry that bars a start, and this is what it does.
///
/// **Observed on the container kind**: the arrangement ran through a fill to
/// genuine zero without exiting, refused loudly, moved nothing, and accepted the
/// pending write the instant space returned. **The systemd kind is unexercised
/// entirely, and three of S14's bullets with it** (§16 item 25).
#[derive(Debug, Default)]
pub struct PendingWrites {
    /// **In the order the writes were owed.** When they land, they land in that
    /// order.
    owed: Vec<OwedWrite>,
    /// **The supervisor records the first reason once in its run's memory and
    /// goes on serving** — the same conduct M5 already requires of the worker
    /// for a refused sink write.
    first_reason: Option<String>,
}

/// **What is held pending, exactly: the durable writes owed about work that has
/// already happened.**
///
/// A recorded outcome the wait returned (`State, identity and recovery` §5.3
/// step 6) with everything that rides it — the completeness statement (S11),
/// V12's free-space reading, the collection outcomes (M13), the note's ending and
/// the journal events for all of it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwedWrite {
    /// What the write is, for the record and for the reading S14a carries.
    pub what: String,
    /// **Which durable write it is, so a re-attempt performs it rather than
    /// recognising it by its words.** Added 2026-09-12, when the first caller
    /// arrived: a re-attempt is handed this value and nothing else, so a
    /// re-attempt driven off `what` alone would be a durable act decided by
    /// reading a sentence.
    pub which: TheDurableWriteThatIsOwed,
}

/// **Which of S14's durable writes one owed entry is.**
///
/// S14 names five things held pending — the recorded outcome with the
/// completeness statement, V12's free-space reading, the collection outcomes,
/// the note's ending and the journal events for all of it — and **two things
/// reach this mechanism**: U12's release outcome for a hold, and, from step 10c
/// piece 5's second sitting, **a journal event the store refused**. The rest
/// are held by nothing yet and the last member says so rather than leaving a
/// reader to infer it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TheDurableWriteThatIsOwed {
    /// **U12's single write, the one that releases the subject** — held here
    /// because a person who pressed *yes* on a full mount would otherwise be
    /// left with a job that waits for ever, this run having recorded the answer
    /// and `Server protocol and transfers` C4 forbidding a repeat of the same
    /// identity from acting again.
    AHoldsReleaseOutcome {
        hold: Identity,
        release: crate::holds::Release,
    },
    /// **A journal event the store refused, held until it lands** —
    /// S14's *the journal events for all of it*.
    ///
    /// **The event takes its sequence number at the landing**, which is
    /// `State, identity and recovery` E1's rule untouched and a reader learning
    /// late, and **its moment is the one it happened at**, carried here by
    /// value so that a landing an hour later does not say the thing happened
    /// an hour later. **Held only where the store itself refused**: an entry
    /// refused for being longer than `State, identity and recovery` E6a's budget would be refused again at
    /// every re-attempt and hold everything owed behind it for good, so the
    /// caller that composed it says so instead and the event is gone.
    ///
    /// **The clock is not carried**: the supervisor that re-attempts it names
    /// the machine's own clock the same way the first attempt did, and a
    /// machine that names none journals nothing in the first place.
    AJournalEvent {
        kind: String,
        about: Option<Identity>,
        says: String,
        happened_at: std::time::SystemTime,
    },
    /// **Not one this crate holds pending yet.** The five S14 names are the
    /// supervisor's own once the stop's ending write and the collection outcomes
    /// have a caller here.
    NotOneThisCrateHoldsYet,
}

impl OwedWrite {
    /// **A journal event the store refused, held pending** — S14's *the journal
    /// events for all of it*.
    pub fn a_journal_event(
        kind: &str,
        about: Option<&Identity>,
        says: &str,
        happened_at: std::time::SystemTime,
    ) -> Self {
        OwedWrite {
            what: format!("the journal event {kind}"),
            which: TheDurableWriteThatIsOwed::AJournalEvent {
                kind: kind.to_string(),
                about: about.cloned(),
                says: says.to_string(),
                happened_at,
            },
        }
    }

    /// **U12's release outcome for one hold, held pending.**
    pub fn a_holds_release(hold: &Identity, release: &crate::holds::Release) -> Self {
        OwedWrite {
            what: format!("the release outcome for the hold {}", hold.as_text()),
            which: TheDurableWriteThatIsOwed::AHoldsReleaseOutcome {
                hold: hold.clone(),
                release: release.clone(),
            },
        }
    }
}

/// **The occasions a pending write is re-attempted on, and they are occasions of
/// their own that something else already caused.**
///
/// **Never a timer, a watcher, a schedule or a free-space reading**, so S10
/// stands unqualified.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReAttemptOccasion {
    EveryRequestTheSupervisorServes,
    EveryWorkerReport,
    EveryReapedEnding,
    /// **On a machine where space returns with no request arriving and no worker
    /// ending, the pending writes land at the next request or the next supervisor
    /// start** — a bounded delay, never a loss.
    TheNextSupervisorStart,
}

impl PendingWrites {
    pub fn new() -> Self {
        PendingWrites::default()
    }

    /// A write was refused for want of room. **The run does not end**, and the
    /// first reason is recorded once.
    pub fn refused(&mut self, write: OwedWrite, reason: &str) -> WhetherThisRunEnds {
        if self.first_reason.is_none() {
            self.first_reason = Some(reason.to_string());
        }
        self.owed.push(write);
        WhetherThisRunEnds::ItGoesOnServing
    }

    /// The first reason, recorded once and not overwritten by later ones.
    pub fn first_reason(&self) -> Option<&str> {
        self.first_reason.as_deref()
    }

    /// What is owed, in the order it was owed.
    pub fn owed(&self) -> &[OwedWrite] {
        &self.owed
    }

    /// **Each re-attempt is a real write and its own test** (`State, identity and
    /// recovery` §1.3 rule 4). Where it lands, everything lands in its owed
    /// order and the queue empties.
    ///
    /// **Each write leaves the list as it lands, so a refusal partway leaves
    /// only what is genuinely still owed.** Clearing the list only when every
    /// write succeeded would leave the ones that already landed on it, and the
    /// next occasion would offer them again — a recorded outcome written twice
    /// over a record that is written once per stream per attempt and never
    /// rewritten, and a second journal event for one ending.
    pub fn re_attempt<W>(&mut self, _on: ReAttemptOccasion, mut write: W) -> bool
    where
        W: FnMut(&OwedWrite) -> bool,
    {
        while !self.owed.is_empty() {
            if !write(&self.owed[0]) {
                return false;
            }
            self.owed.remove(0);
        }
        true
    }

    /// **A run that ends before the landing loses the pending writes and nothing
    /// silent follows.** The operation record still carries no outcome, the group
    /// is gone, and the next start's reconciliation takes its ordinary branch —
    /// *re-run once* bounding the cost at one repeated attempt.
    pub fn what_a_run_that_ends_loses(&self) -> usize {
        self.owed.len()
    }
}

/// **S14's first bullet: the run does not end for a refused write.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhetherThisRunEnds {
    ItGoesOnServing,
}

/// **S14's fourth bullet: no act is taken over a pending write, and no new
/// refusal kind exists.**
///
/// A command whose conduct requires a durable record first is **refused loudly,
/// naming want of room** — `Queue & History` Q8's own conduct, from a supervisor
/// that answers rather than from one that is down; **the admission point starts
/// nothing**, because that document's Q4 trigger — the previous outcome
/// *written* — has not occurred and a fresh operation record could not be written
/// anyway; **and no hold is placed**, U6's rule already covering a hold that
/// cannot be journalled.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OverAPendingWrite {
    RefusedLoudlyNamingWantOfRoom,
    /// Where the command needs no durable record first, it is served as ever.
    ServedAsEver,
}

/// **S14's fourth bullet.**
pub fn over_a_pending_write(
    the_conduct_requires_a_durable_record_first: bool,
) -> OverAPendingWrite {
    if the_conduct_requires_a_durable_record_first {
        OverAPendingWrite::RefusedLoudlyNamingWantOfRoom
    } else {
        OverAPendingWrite::ServedAsEver
    }
}

/// **S14's fifth bullet and S14a: what a reading carries while the writes are
/// pending.**
///
/// **Readings stay truthful and carry what the supervisor knows.** The queue
/// reading's answer for a job whose group has ended carries the pending recorded
/// outcome as that job's state rather than *rendering* — the ending is classified
/// from the operation record, the group's validation and the reaped status (M20),
/// all of which the supervisor holds, only the durable write being owed.
///
/// **S14a. The rest of what is held pending rides the reading beside the state.**
/// S14 named five things held pending and its readings bullet carried forward
/// exactly one of them. **The others are carried too**, on the same ground:
/// withholding them would be withholding what the supervisor knows from a reader
/// whose whole purpose is to ask it what it knows.
///
/// **What it fixes.** Between the fill and the landing the user otherwise has a
/// failed job and a correct diagnosis with nothing behind either — `Rendering`
/// K18's *which frames were rendered* states nothing, and `Desktop app and
/// delivery` M10's four collection outcomes have nothing to draw over a stream
/// that is genuinely truncated.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AReadingWhileWritesArePending<T> {
    /// The value, **as of the moment it was taken**, which is V5's standing rule
    /// and no new one.
    pub value: T,
    /// **Marked in the reading as owed to the durable store rather than written
    /// to it.**
    pub owed_to_the_durable_store: bool,
}

/// **S14a's boundary, and it is what keeps the reading honest: a reading is never
/// evidence that a record landed.**
///
/// **Anything whose conduct turns on the write having landed reads the durable
/// record and not the reading**; §16 item 26 already records the journal running
/// ahead of the store, and this is the same window seen from the reading's side
/// rather than a second one.
pub const A_READING_IS_NEVER_EVIDENCE_THAT_A_RECORD_LANDED: bool = true;

/// **S14a's last bullet, and it is another document's sentence rather than
/// this one's.**
///
/// **A capture record whose collection outcome is still pending must never
/// present as *collected*.** That is `Desktop app and delivery` §16.3's to
/// write — this document supplies the outcome and does not present it — **and it
/// is routed rather than written here**
/// ([`crate::arriving_later::WHAT_IS_ROUTED_TO_ANOTHER_CRATE`]).
pub const THE_PRESENTATION_SENTENCE_IS_ROUTED_AND_NOT_WRITTEN_HERE: bool = true;

/// **The alternative S14a considered and refused**, recorded because refusing it
/// is the decision.
///
/// Withholding all five until the landing would be defensible and is the honest
/// silence; **it was refused because the whole cost of the fill falls in exactly
/// that window**, and a reading that is truthful about its own moment is what
/// this document already promises everywhere else.
pub const WITHHOLDING_ALL_FIVE_WAS_REFUSED: bool = true;

/// **`State, identity and recovery` §6 step 2's table, and this file is its one
/// home in the workspace.**
///
/// **S8 takes that section entire and does not restate it**, so what is written
/// here is that table read rather than a second copy of it: the three durable
/// facts go in — the desired state, whether the recorded execution group
/// validates, the attempt count — and one of its six rows comes out.
///
/// # Why the table lives here rather than beside a caller
///
/// **It was in `Queue & History` and nothing outside that crate could reach
/// it.** That crate carries the desktop's SSH client in its tree, so the program
/// a rented machine runs may never depend on it, and the supervisor — the one
/// thing §6 gives the loop to — sat on the other side of that wall from the
/// table it had to read. **The table was moved rather than written a second
/// time**, and that document's own render-shaped reading of it now delegates
/// here, so the six rows exist once. A fact kept in two places is the fault this
/// stage keeps meeting.
///
/// **What the move does not do is give this file the render's words.**
/// `Queue & History` D4 and `Queue & History` D12 are that document's reading of
/// these rows for a job, with its own *the render is still alive* in place of
/// *resumed*, and it keeps them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatStep2Decides {
    /// One of `The shape`'s four names, to be written durably before the act it
    /// names.
    ItTakesThisBranch(Disposition),
    /// **The *ended* row: unreachable by construction.**
    /// `State, identity and recovery` G3 writes that desired state only in the
    /// same act as the recorded outcome, so a record carrying it is closed and
    /// the loop never selects it. **It is answered rather than left off**, so a
    /// record that somehow carries it is met rather than given a branch.
    ItCarriesEndedAndIsClosed,
}

/// **`State, identity and recovery` §6 step 2, one record, exactly one branch.**
///
/// | desired state | recorded execution group | attempt count | what is written |
/// |---|---|---|---|
/// | *should be stopped* | gone | — | **left stopped**, with the recorded outcome |
/// | *should be stopped* | validates | — | **resumed**, before the re-attach; the record stays open and the desired state stays |
/// | *ended* | — | — | unreachable by construction |
/// | *should run* | validates | — | **resumed**, before the re-attach |
/// | *should run* | gone | 1 | **re-run once**, with the count of 2 |
/// | *should run* | gone | 2 | **given up**, with the recorded outcome |
///
/// **The *should be stopped* row is split by the group, and it was one row
/// until 2026-10-01.** It read *left stopped* whatever the group, and that is
/// what was built. **Written for one case and now meeting two**: it was written
/// for a stop whose render died with the machine — a reboot, a container
/// restart — where the group is gone and nobody is left to report an ending.
/// **Step 10c's pass before Vast on Sesterce met the other**: the supervisor
/// alone was restarted with a stop outstanding, and `Rented-server runtime` K18
/// keeps a render alive across exactly that. **The record and the world parted
/// company at the restart**: the row closed the job *left stopped* while
/// Blender walked on through the range, the line started the next job beside it
/// on the same card, and the render's own report was refused as stale because
/// *resumed* was never written. **Now a stop whose group still validates is
/// *resumed*** — the record open, the desired state kept, and the ending
/// arriving by the worker's report (`Rented-server runtime` W12, S8). **What is
/// sent at the re-attach is the caller's and not this table's**: nothing where
/// the record says the termination was already issued (`Rendering` K10), the
/// termination where it says not.
///
/// **A count above two takes the last row too.** The count is bounded at two by
/// `State, identity and recovery` G2's own write — the branch that sets it is the
/// only one that moves it — so a record carrying more is a record that was
/// written wrongly, and *given up* is the direction that starts nothing. **A
/// third attempt has no row to arrive at**, which is the whole of the re-run-once
/// rule.
pub fn what_step_2_decides(
    desired: DesiredState,
    the_recorded_execution_group_is_gone: bool,
    attempt_count: u32,
) -> WhatStep2Decides {
    match desired {
        DesiredState::ShouldBeStopped if !the_recorded_execution_group_is_gone => {
            WhatStep2Decides::ItTakesThisBranch(Disposition::Resumed)
        }
        DesiredState::ShouldBeStopped => {
            WhatStep2Decides::ItTakesThisBranch(Disposition::LeftStopped)
        }
        DesiredState::Ended => WhatStep2Decides::ItCarriesEndedAndIsClosed,
        DesiredState::ShouldRun if !the_recorded_execution_group_is_gone => {
            WhatStep2Decides::ItTakesThisBranch(Disposition::Resumed)
        }
        DesiredState::ShouldRun if attempt_count <= 1 => {
            WhatStep2Decides::ItTakesThisBranch(Disposition::ReRunOnce)
        }
        DesiredState::ShouldRun => WhatStep2Decides::ItTakesThisBranch(Disposition::GivenUp),
    }
}

/// **Whether the record's recorded execution group is gone, and the three ways a
/// reading can end.**
///
/// **`State, identity and recovery` G6 is the rule the first two keep**: a
/// pair that does not validate means the group is gone, never that the check
/// should be relaxed. **X6 is the direction the third keeps**: a check that did
/// not answer relaxes nothing, so a reading the supervisor could not take leaves
/// the record's branch untaken rather than reading as *gone* — which would start
/// a second attempt beside a group that may still be running.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhetherTheGroupIsGone {
    ItValidates,
    ItIsGone,
    /// The reading could not be taken, in the machine's own words.
    TheReadingCouldNotBeTaken(String),
}

/// **`State, identity and recovery` §5.2, performed over one record.**
///
/// **A record carrying no recorded execution group is gone**, which is that
/// table's own reading: `State, identity and recovery` G1 holds the group of the
/// current attempt, and a record with none has no attempt this supervisor could
/// re-attach to. **`State, identity and recovery` G9 gets no separate arm** — a
/// group recorded under a previous run identifier is treated as gone unless it
/// validates, and validating is exactly what is asked here.
///
/// **That is a decision and not a certainty, and the difference is written here
/// rather than glossed.** G9 said the collapse to *gone* after a whole-machine
/// restart holds *by construction* until 2026-09-12, and it does not: the pair
/// is a process identifier and a start time in ticks since boot, **and both
/// start again after a boot**, so an unrelated process on the machine that came
/// back can present the recorded pair and validate against it. **The rule to
/// carry is X3's** — a reboot, a container restart and a recycle are settled by
/// the run identifier the supervisor writes at every start and **never** by the
/// pair. **No arm is added for the residual**, because telling a reboot apart
/// from a supervisor crash needs a reading nothing on this machine has, and this
/// stage does not invent one.
///
/// **A field that will not read back as a pair is not half a pair.** X2 is the
/// finding against validating on the identifier alone, so an unreadable field is
/// a reading that could not be taken.
pub fn whether_the_group_is_gone(
    record: &OperationRecord,
    pairs: &dyn ValidatesTheRecordedPair,
) -> WhetherTheGroupIsGone {
    let Some(group) = &record.execution_group else {
        return WhetherTheGroupIsGone::ItIsGone;
    };
    let Some(recorded) = RecordedPair::recorded_in(&group.as_that_document_recorded_it) else {
        return WhetherTheGroupIsGone::TheReadingCouldNotBeTaken(format!(
            "the recorded execution group does not read back as a pair: {}",
            group.as_that_document_recorded_it
        ));
    };
    match pairs.what_is_at(recorded.identifier) {
        Err(why) => WhetherTheGroupIsGone::TheReadingCouldNotBeTaken(why),
        Ok(what_is_there) => {
            if recorded.validates_against(what_is_there).the_group_is_gone() {
                WhetherTheGroupIsGone::ItIsGone
            } else {
                WhetherTheGroupIsGone::ItValidates
            }
        }
    }
}
/// **What one open record is due out of §6 step 2, decided from records
/// alone.**
///
/// **This is the judgement and it takes no act.** The supervisor's program
/// performs the writes; what is settled here is which of the six rows the
/// record falls in, and it is settled before any walk is taken (S12).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatOneRecordIsDue {
    /// One of `The shape`'s four names, to be written durably before the act it
    /// names.
    ThisBranch(Disposition),
    /// **No branch, and the record stands open for the next run**, in the
    /// machine's own words. **It is never silence**: an open operation nothing
    /// takes a branch for is a job neither resumed nor re-run nor given up on a
    /// machine that is being paid for.
    NoBranchCouldBeTaken(String),
    /// The *ended* row.
    ItCarriesEndedAndIsClosed,
}

/// **§6 step 2 over one record: the pair validated, then the table read.**
///
/// **The two readings are the only ones taken** — the record's own fields and
/// `State, identity and recovery` §5.2's pair — and neither is a file's
/// presence, a name or a size.
pub fn what_one_record_is_due(
    record: &OperationRecord,
    pairs: &dyn ValidatesTheRecordedPair,
) -> WhatOneRecordIsDue {
    let gone = match whether_the_group_is_gone(record, pairs) {
        WhetherTheGroupIsGone::ItValidates => false,
        WhetherTheGroupIsGone::ItIsGone => true,
        WhetherTheGroupIsGone::TheReadingCouldNotBeTaken(why) => {
            return WhatOneRecordIsDue::NoBranchCouldBeTaken(why)
        }
    };
    match what_step_2_decides(record.desired_state(), gone, record.attempt_count) {
        WhatStep2Decides::ItTakesThisBranch(disposition) => {
            WhatOneRecordIsDue::ThisBranch(disposition)
        }
        WhatStep2Decides::ItCarriesEndedAndIsClosed => WhatOneRecordIsDue::ItCarriesEndedAndIsClosed,
    }
}

/// **Which of the two terminal dispositions an outcome belongs to**, for S11's
/// *only those two*.
///
/// **The two enumerations are different things and this is the only place they
/// meet.** `State, identity and recovery` G2's four names are what a run did
/// with a record; S11's pair is the two of them that end a job with no worker
/// alive to have taken the ending step.
pub fn which_ending_a_disposition_writes(
    disposition: Disposition,
) -> Option<DispositionThatEndsAJob> {
    match disposition.the_outcome_it_writes_with_itself() {
        Some(RecordedOutcome::LeftStopped) => Some(DispositionThatEndsAJob::LeftStopped),
        Some(RecordedOutcome::GivenUp) => Some(DispositionThatEndsAJob::GivenUp),
        // **A kind's own ending is never a disposition's**, and *resumed* and
        // *re-run once* write no outcome at all.
        Some(RecordedOutcome::TheKindsOwn(_)) | None => None,
    }
}

/// **What one open record got out of §6 step 2**, once the writes have been
/// made.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatARecordGot {
    /// *Resumed* or *re-run once* — written durably, closing nothing.
    ABranchThatKeepsItOpen(Disposition),
    /// *Left stopped* or *given up* — the disposition, the recorded outcome and
    /// the measurements in one act.
    ItEndedTheJob(AnEndingOutcome),
    /// **No branch was taken and the record stands open for the next run**, in
    /// the machine's or the store's own words.
    NoBranchCouldBeTaken(String),
    /// The *ended* row.
    ItCarriesEndedAndIsClosed,
}

/// One record, and what this run did with it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatOneRecordGot {
    pub operation: Identity,
    pub what: WhatARecordGot,
}

/// **What one pass of reconciliation did**, for the run's own stream and for a
/// test to read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatReconciliationDid {
    pub at: ReconciliationMoment,
    /// **§6 Step 0**: the setup run the standing state names, where one stands.
    /// **Nothing below it ran.**
    pub held_back_by_the_setup_run: Option<Identity>,
    /// **§6 Step 0, the other way it holds**: a state that could not be read.
    /// **The same answer as a state that stands**, because a machine whose setup
    /// state cannot be read may be a machine mid-setup and P4 is the whole reason
    /// that state is durable.
    pub the_setup_state_could_not_be_read: Option<String>,
    pub records: Vec<WhatOneRecordGot>,
    /// **§6 Step 3.**
    pub the_role_worker: String,
    /// Everything that could not be done, in whoever refused's own words.
    pub what_could_not_be_done: Vec<String>,
}

impl WhatReconciliationDid {
    /// Nothing has been done yet, at this moment.
    pub fn at(at: ReconciliationMoment) -> Self {
        WhatReconciliationDid {
            at,
            held_back_by_the_setup_run: None,
            the_setup_state_could_not_be_read: None,
            records: Vec::new(),
            the_role_worker: "not looked after: reconciliation did not run".to_string(),
            what_could_not_be_done: Vec::new(),
        }
    }

    /// Whether step 2 ran at all.
    pub fn it_ran(&self) -> bool {
        self.held_back_by_the_setup_run.is_none() && self.the_setup_state_could_not_be_read.is_none()
    }

    /// **What this pass did, for the machine's own complaints stream.**
    ///
    /// **A supervisor that reconciled quietly would be indistinguishable from
    /// one that did not reconcile at all**, which is the shape this program's
    /// standing declaration exists to prevent — so every pass says what it did,
    /// record by record.
    pub fn as_the_machine_says_it(&self) -> String {
        if let Some(run) = &self.held_back_by_the_setup_run {
            return format!(
                "reconciliation did not run: a setup is in progress under {}, and while it \
                 stands nothing is resumed, re-run, left stopped or given up",
                run.as_text()
            );
        }
        if let Some(why) = &self.the_setup_state_could_not_be_read {
            return format!(
                "reconciliation did not run: the setup-in-progress state could not be read, so \
                 this machine may be mid-setup — {why}"
            );
        }
        let mut said = format!(
            "reconciliation ran over {} open operation records; the Telegram role worker — {}",
            self.records.len(),
            self.the_role_worker
        );
        for record in &self.records {
            said.push_str(&format!(
                "\nsmoozy-server: {} — {}",
                record.operation.as_text(),
                match &record.what {
                    WhatARecordGot::ABranchThatKeepsItOpen(disposition) =>
                        format!("{disposition:?}, and the record stays open"),
                    WhatARecordGot::ItEndedTheJob(outcome) => format!(
                        "{:?}, with the recorded outcome; {}",
                        outcome.disposition,
                        if outcome.carries_a_completeness_statement() {
                            "a completeness statement rides with it"
                        } else {
                            THE_COMPLETENESS_STATEMENT_IS_NOT_SUPPLIED_HERE
                        }
                    ),
                    WhatARecordGot::NoBranchCouldBeTaken(why) =>
                        format!("no branch was taken and it stands open: {why}"),
                    WhatARecordGot::ItCarriesEndedAndIsClosed =>
                        "it carries the ended desired state and no recorded outcome, which \
                         `State, identity and recovery` G3 makes unreachable; no branch was \
                         taken"
                            .to_string(),
                }
            ));
        }
        for why in &self.what_could_not_be_done {
            said.push_str(&format!("\nsmoozy-server: {why}"));
        }
        said
    }
}

/// **What an outcome written on this machine carries in place of S11's
/// statement, and why — said in one sentence rather than left to be inferred.**
///
/// **It is an absence and never four empty sets** (S13), which is the
/// distinction `Queue & History` H8 turns on.
///
/// **Narrowed 2026-09-19, at step 10c piece 8.** This said the reckoning sits
/// in a crate the machine's program may not reach, which stopped being true at
/// step 10c piece 3: the reckoning moved into `Job files and outputs`' machine
/// half and **a render's ending written by that program carries the statement**
/// from piece 5's second sitting. **What this sentence is for now is the other
/// five per-operation kinds**, whose own endings are written by documents
/// nothing on this machine drives; an outcome written for one of them carries
/// no statement because no reckoning was ever asked for.
pub const THE_COMPLETENESS_STATEMENT_IS_NOT_SUPPLIED_HERE: &str =
    "no completeness statement rides with it: this kind's own ending is written by a document \
     nothing on this machine drives, so no reckoning was asked for — an absence recorded as one, \
     never four empty sets";

/// **What is not started when *re-run once* is written, said rather than left to
/// be discovered.**
///
/// **§6 step 2 writes the disposition and the count of 2 before the new attempt
/// starts.** The attempt itself is the kind's own document's — a render's is
/// `Queue & History` D4's, a setup run's is `Server Setup`'s.
///
/// **Narrowed 2026-09-19, at step 10c piece 8.** This said every one of the six
/// per-operation kinds is named by a document whose crate the machine's program
/// cannot reach, which stopped being true at step 10c piece 3, and **the render
/// kind's second attempt is started by the line** from piece 5's second
/// sitting. **What this sentence is for now is the other five**: the branch is
/// taken, written and journalled, and nothing starts their second attempt.
/// **The next run gives such a record up**, which is the re-run-once rule's own
/// bound rather than a new one.
pub const NO_SECOND_ATTEMPT_IS_STARTED_HERE: &str =
    "the disposition was written and no second attempt was started: this kind's own re-run is \
     named by a document nothing on this machine drives";

/// **What a journalled disposition says, and it is deliberately short.**
///
/// **`State, identity and recovery` E6a bounds one entry** — *"a kind whose
/// fields would exceed the budget names the record that holds the detail rather
/// than copying it in"* — and an entry that does not fit is refused whole rather
/// than trimmed, so a disposition that copied the two sentences above into every
/// event would be a disposition no reader ever learns of. **The sentences go on
/// the machine's own complaints stream** ([`WhatReconciliationDid::as_the_machine_says_it`]),
/// **and the event carries the disposition and the record it is about.**
///
/// **The first entry was 523 bytes against a budget of 512 and was refused**,
/// which is how this was found rather than reasoned.
pub fn what_a_journalled_disposition_says(
    disposition: Disposition,
    it_carries_a_completeness_statement: bool,
) -> String {
    match which_ending_a_disposition_writes(disposition) {
        // **The two branches that keep a record open are not the same news, and
        // one sentence for both made the good one read as a complaint.**
        // *Resumed* means the group still validates and the supervisor
        // re-attached — nothing was expected to start, so saying that nothing
        // did is a grievance about a working job. *Re-run once* is where the
        // second attempt genuinely is owed and genuinely does not happen.
        None if disposition == Disposition::Resumed => {
            format!("{disposition:?}; the group still validates and the supervisor re-attached")
        }
        None => format!("{disposition:?}; no second attempt is started here"),
        Some(_) if it_carries_a_completeness_statement => {
            format!("{disposition:?}, with the recorded outcome and a completeness statement")
        }
        Some(_) => format!("{disposition:?}, with the recorded outcome and no completeness statement"),
    }
}

/// **S9's second moment, and the only route to it.**
///
/// **P5 makes the clear two acts and this is what keeps them together**:
/// [`crate::setup_state::SetsAndClearsTheState::clear`] is the only clear there
/// is, it hands back the value this function takes, and this function is the only
/// thing that takes one. A caller that cleared the state and did not reconcile
/// would have a value it could do nothing else with.
///
/// **The state is gone durably by the time this is reached**, which is the
/// ordering P5 states: a reconciliation running over a state whose removal had
/// not landed would be the one act P2 holds back arriving early.
pub fn the_moment_a_clear_reconciles_at(
    _the_state_is_gone: crate::setup_state::TheStateIsGoneAndReconciliationIsOwed,
) -> ReconciliationMoment {
    ReconciliationMoment::TheSetupInProgressStateIsCleared
}
#[cfg(test)]
mod tests {
    use smoozy_state::identities::IdentityKind;

    use super::*;

    #[test]
    fn there_is_one_restarter_per_machine_kind() {
        assert_eq!(
            Restarter::for_kind(MachineKind::Systemd),
            Restarter::TheUserUnitsUnconditionalRestartDirective
        );
        assert_eq!(
            Restarter::for_kind(MachineKind::Container),
            Restarter::TheKeeperTheBootHookStarts
        );
    }

    #[test]
    fn neither_restarter_distinguishes_a_crash_from_a_deliberate_end() {
        assert_eq!(
            what_the_restarter_does(WhyTheSupervisorEnded::G25sPackageCommit),
            what_the_restarter_does(WhyTheSupervisorEnded::ACrashOrAForcibleRestart)
        );
    }

    #[test]
    fn reconciliation_has_exactly_two_moments() {
        // **S10's set is closed by the compiler rather than by a count.** The
        // match is total and carries no default arm, so a moment added to the
        // enum has none, the crate stops compiling, and whoever added it is sent
        // to this list — where the length assertion this replaced could never
        // have been false.
        for (at, moment) in ReconciliationMoment::THE_ONLY_TWO.iter().enumerate() {
            let where_the_list_puts_it = match moment {
                ReconciliationMoment::EverySupervisorStart => 0,
                ReconciliationMoment::TheSetupInProgressStateIsCleared => 1,
            };
            assert_eq!(at, where_the_list_puts_it, "{moment:?} is not where S10 puts it");
        }
    }

    #[test]
    fn a_supervisor_run_carries_a_fresh_identifier() {
        let one = SupervisorRun {
            run: Identity::issue(IdentityKind::Run).expect("randomness"),
            machine_kind: MachineKind::Container,
        };
        let other = SupervisorRun {
            run: Identity::issue(IdentityKind::Run).expect("randomness"),
            machine_kind: MachineKind::Container,
        };
        assert_ne!(one.run, other.run);
    }

    #[test]
    fn an_outcome_with_no_statement_is_not_one_with_an_empty_statement() {
        let none = AnEndingOutcome {
            disposition: DispositionThatEndsAJob::GivenUp,
            completeness: None,
            free_space: None,
        };
        let empty = AnEndingOutcome {
            disposition: DispositionThatEndsAJob::GivenUp,
            completeness: Some(CompletenessStatement {
                the_four_sets: Vec::new(),
            }),
            free_space: None,
        };
        assert!(!none.carries_a_completeness_statement());
        assert!(empty.carries_a_completeness_statement());
        assert_ne!(none, empty);
    }

    #[test]
    fn the_disposition_is_decided_before_the_walk_and_the_walk_is_handed_it() {
        let mut what_the_walk_was_told = None;
        let outcome = decide_then_walk(DispositionThatEndsAJob::LeftStopped, |disposition| {
            what_the_walk_was_told = Some(disposition);
            (
                Some(CompletenessStatement {
                    the_four_sets: vec![("rendered".into(), vec!["0001".into()])],
                }),
                None,
            )
        });
        assert_eq!(
            what_the_walk_was_told,
            Some(DispositionThatEndsAJob::LeftStopped)
        );
        assert_eq!(outcome.disposition, DispositionThatEndsAJob::LeftStopped);
    }

    /// **A *resumed* job is not told it had no second attempt.**
    ///
    /// One sentence covered both branches that keep a record open until
    /// 2026-09-12, and for a job the supervisor successfully re-attached to it
    /// read as a complaint about something that was never expected. **Only
    /// *re-run once* is owed a second attempt**, and only there is the absence
    /// of one worth saying.
    #[test]
    fn resumed_and_re_run_once_are_not_told_the_same_thing() {
        let resumed = what_a_journalled_disposition_says(Disposition::Resumed, false);
        let re_run = what_a_journalled_disposition_says(Disposition::ReRunOnce, false);
        assert!(
            !resumed.contains("no second attempt"),
            "a resumed job was never owed one: {resumed}"
        );
        assert!(
            resumed.contains("re-attached"),
            "it says what actually happened: {resumed}"
        );
        assert!(
            re_run.contains("no second attempt is started here"),
            "{re_run}"
        );
    }

    /// A pending write that is not one this crate holds a typed act for, for
    /// the tests below that are about the ordering rather than about the act.
    fn an_owed_write(what: &str) -> OwedWrite {
        OwedWrite {
            what: what.into(),
            which: TheDurableWriteThatIsOwed::NotOneThisCrateHoldsYet,
        }
    }

    #[test]
    fn a_refused_write_is_held_the_run_goes_on_and_the_first_reason_is_kept_once() {
        let mut pending = PendingWrites::new();
        assert_eq!(
            pending.refused(
                an_owed_write("the render's recorded outcome"),
                "No space left on device"
            ),
            WhetherThisRunEnds::ItGoesOnServing
        );
        pending.refused(
            an_owed_write("the journal event"),
            "a later and different reason",
        );
        assert_eq!(pending.first_reason(), Some("No space left on device"));
        assert_eq!(pending.owed().len(), 2);
    }

    #[test]
    fn the_pending_writes_land_in_the_order_they_were_owed() {
        let mut pending = PendingWrites::new();
        pending.refused(an_owed_write("first"), "no room");
        pending.refused(an_owed_write("second"), "no room");
        let mut landed = Vec::new();
        let all = pending.re_attempt(ReAttemptOccasion::EveryRequestTheSupervisorServes, |owed| {
            landed.push(owed.what.clone());
            true
        });
        assert!(all);
        assert_eq!(landed, vec!["first".to_string(), "second".to_string()]);
        assert!(pending.owed().is_empty());
    }

    #[test]
    fn a_re_attempt_that_is_refused_again_leaves_everything_owed() {
        let mut pending = PendingWrites::new();
        pending.refused(an_owed_write("first"), "no room");
        pending.refused(an_owed_write("second"), "no room");
        let all = pending.re_attempt(ReAttemptOccasion::EveryWorkerReport, |_| false);
        assert!(!all);
        assert_eq!(pending.owed().len(), 2);
        assert_eq!(pending.what_a_run_that_ends_loses(), 2);
    }

    /// **A refusal partway leaves only what is genuinely still owed.** The
    /// first write lands, the second does not, and a later attempt offers the
    /// second alone — never the one that already landed.
    #[test]
    fn a_write_that_landed_is_not_offered_again_after_a_refusal_partway() {
        let mut pending = PendingWrites::new();
        pending.refused(an_owed_write("the recorded outcome"), "no room");
        pending.refused(an_owed_write("the journal event"), "no room");

        let mut offered = Vec::new();
        let all = pending.re_attempt(ReAttemptOccasion::EveryReapedEnding, |owed| {
            offered.push(owed.what.clone());
            owed.what == "the recorded outcome"
        });
        assert!(!all);
        assert_eq!(
            offered,
            vec!["the recorded outcome".to_string(), "the journal event".to_string()]
        );
        assert_eq!(pending.owed().len(), 1, "only what is still owed stands");

        let mut offered_later = Vec::new();
        let all = pending.re_attempt(ReAttemptOccasion::TheNextSupervisorStart, |owed| {
            offered_later.push(owed.what.clone());
            true
        });
        assert!(all);
        assert_eq!(
            offered_later,
            vec!["the journal event".to_string()],
            "a write that already landed is never offered again"
        );
        assert!(pending.owed().is_empty());
    }

    #[test]
    fn a_command_needing_a_durable_record_first_is_refused_loudly_over_a_pending_write() {
        assert_eq!(
            over_a_pending_write(true),
            OverAPendingWrite::RefusedLoudlyNamingWantOfRoom
        );
        assert_eq!(over_a_pending_write(false), OverAPendingWrite::ServedAsEver);
    }

    #[test]
    fn a_reading_taken_while_writes_are_pending_says_it_is_owed_rather_than_written() {
        let reading = AReadingWhileWritesArePending {
            value: "ended",
            owed_to_the_durable_store: true,
        };
        assert!(reading.owed_to_the_durable_store);
        assert_eq!(reading.value, "ended");
    }


    // ---- `State, identity and recovery` §6, and the six rows it turns on ----

    /// The kernel's answers, in a list. **What is at an identifier and nothing
    /// else**, which is the one reading §5.2 admits.
    struct WhatIsAliveInAList(Vec<RecordedPair>);

    impl ValidatesTheRecordedPair for WhatIsAliveInAList {
        fn what_is_at(&self, identifier: u32) -> Result<Option<RecordedPair>, String> {
            Ok(self
                .0
                .iter()
                .find(|pair| pair.identifier == identifier)
                .copied())
        }
    }

    /// A validator that cannot answer at all.
    struct TheKernelWouldNotAnswer;

    impl ValidatesTheRecordedPair for TheKernelWouldNotAnswer {
        fn what_is_at(&self, _identifier: u32) -> Result<Option<RecordedPair>, String> {
            Err("the kernel's record does not carry the start time".to_string())
        }
    }

    fn an_identity(kind: IdentityKind) -> Identity {
        Identity::issue(kind).expect("the desk has randomness")
    }

    /// An open record with a recorded execution group at `identifier`.
    fn a_record_with_a_group(identifier: u32, start_time: u64) -> OperationRecord {
        let mut record = OperationRecord::opened(
            an_identity(IdentityKind::Operation),
            an_identity(IdentityKind::Job),
            "render",
        );
        record.execution_group =
            Some(smoozy_state::machine_operations::TheRecordedExecutionGroup {
                as_that_document_recorded_it: RecordedPair {
                    identifier,
                    start_time,
                }
                .as_this_document_records_it(),
                started_under_run: an_identity(IdentityKind::Run),
            });
        record
    }

    /// **§6 step 2's six rows, read at their one home.**
    #[test]
    fn step_2_takes_exactly_one_of_the_six_rows() {
        // *should be stopped*, the group gone, whatever the count.
        for count in 1..=3 {
            assert_eq!(
                what_step_2_decides(DesiredState::ShouldBeStopped, true, count),
                WhatStep2Decides::ItTakesThisBranch(Disposition::LeftStopped)
            );
        }
        // *should be stopped*, the group still validating, whatever the count:
        // **resumed**, and nothing closed (step 10c's pass before Vast on
        // Sesterce).
        for count in 1..=3 {
            assert_eq!(
                what_step_2_decides(DesiredState::ShouldBeStopped, false, count),
                WhatStep2Decides::ItTakesThisBranch(Disposition::Resumed)
            );
        }
        // *ended* is unreachable by construction and is answered rather than
        // given a branch.
        assert_eq!(
            what_step_2_decides(DesiredState::Ended, true, 1),
            WhatStep2Decides::ItCarriesEndedAndIsClosed
        );
        // *should run*, the group validates.
        assert_eq!(
            what_step_2_decides(DesiredState::ShouldRun, false, 1),
            WhatStep2Decides::ItTakesThisBranch(Disposition::Resumed)
        );
        // *should run*, gone, first attempt.
        assert_eq!(
            what_step_2_decides(DesiredState::ShouldRun, true, 1),
            WhatStep2Decides::ItTakesThisBranch(Disposition::ReRunOnce)
        );
        // *should run*, gone, second attempt — and every count above it, which
        // is the re-run-once rule's own bound.
        for count in 2..=10 {
            assert_eq!(
                what_step_2_decides(DesiredState::ShouldRun, true, count),
                WhatStep2Decides::ItTakesThisBranch(Disposition::GivenUp)
            );
        }
    }

    /// **A group that still validates is *resumed***, which is the branch a
    /// worker's report from an earlier run turns on.
    #[test]
    fn a_group_that_still_validates_is_resumed() {
        let record = a_record_with_a_group(4242, 99);
        assert_eq!(
            what_one_record_is_due(
                &record,
                &WhatIsAliveInAList(vec![RecordedPair {
                    identifier: 4242,
                    start_time: 99,
                }])
            ),
            WhatOneRecordIsDue::ThisBranch(Disposition::Resumed)
        );
    }

    /// **A re-handed-out identifier is not the recorded group**, which is the
    /// whole of X2: the identifier is there, the pair does not match, and the
    /// record takes the *gone* side of the table.
    #[test]
    fn an_identifier_handed_out_again_reads_as_gone_rather_than_as_the_recorded_group() {
        let record = a_record_with_a_group(1493, 19186);
        assert_eq!(
            whether_the_group_is_gone(
                &record,
                // Same identifier, a later and unrelated process.
                &WhatIsAliveInAList(vec![RecordedPair {
                    identifier: 1493,
                    start_time: 20000,
                }])
            ),
            WhetherTheGroupIsGone::ItIsGone
        );
        assert_eq!(
            what_one_record_is_due(&record, &WhatIsAliveInAList(Vec::new())),
            WhatOneRecordIsDue::ThisBranch(Disposition::ReRunOnce)
        );
    }

    /// **A record carrying no recorded execution group at all is gone**, which
    /// is what a worker started whose group could not be recorded leaves behind,
    /// and the re-run-once rule is what bounds it.
    #[test]
    fn a_record_with_no_recorded_group_reads_as_gone() {
        let mut record = a_record_with_a_group(1, 1);
        record.execution_group = None;
        assert_eq!(
            whether_the_group_is_gone(&record, &WhatIsAliveInAList(Vec::new())),
            WhetherTheGroupIsGone::ItIsGone
        );
    }

    /// **X6's direction: a reading that did not answer relaxes nothing.** A
    /// field that will not read back as a pair, and a kernel that would not
    /// answer, both leave the record's branch untaken — rather than reading as
    /// *gone* and starting a second attempt beside a group that may still be
    /// running.
    #[test]
    fn a_reading_that_could_not_be_taken_leaves_the_branch_untaken() {
        let mut damaged = a_record_with_a_group(1, 1);
        damaged
            .execution_group
            .as_mut()
            .expect("it has one")
            .as_that_document_recorded_it = "not a pair".to_string();
        assert!(matches!(
            what_one_record_is_due(&damaged, &WhatIsAliveInAList(Vec::new())),
            WhatOneRecordIsDue::NoBranchCouldBeTaken(_)
        ));
        assert!(matches!(
            what_one_record_is_due(&a_record_with_a_group(1, 1), &TheKernelWouldNotAnswer),
            WhatOneRecordIsDue::NoBranchCouldBeTaken(_)
        ));
    }

    /// **A stop asked for before the machine went down is *left stopped***
    /// where its group is gone, and it is one of S11's two. **Where the group
    /// still validates it is *resumed***: the supervisor alone was restarted
    /// and the render lives on (`Rented-server runtime` K18).
    #[test]
    fn a_durable_should_be_stopped_is_left_stopped_and_is_one_of_s11s_two() {
        let mut record = a_record_with_a_group(7, 7);
        record.the_stop_was_asked_for().expect("it is open");
        let due = what_one_record_is_due(&record, &WhatIsAliveInAList(Vec::new()));
        assert_eq!(due, WhatOneRecordIsDue::ThisBranch(Disposition::LeftStopped));
        let due = what_one_record_is_due(
            &record,
            &WhatIsAliveInAList(vec![RecordedPair {
                identifier: 7,
                start_time: 7,
            }]),
        );
        assert_eq!(due, WhatOneRecordIsDue::ThisBranch(Disposition::Resumed));
        assert_eq!(
            which_ending_a_disposition_writes(Disposition::LeftStopped),
            Some(DispositionThatEndsAJob::LeftStopped)
        );
    }

    /// **S11's *only those two*, at the one place the two enumerations meet.**
    #[test]
    fn exactly_two_of_the_four_dispositions_end_a_job() {
        assert_eq!(
            which_ending_a_disposition_writes(Disposition::GivenUp),
            Some(DispositionThatEndsAJob::GivenUp)
        );
        assert_eq!(which_ending_a_disposition_writes(Disposition::Resumed), None);
        assert_eq!(
            which_ending_a_disposition_writes(Disposition::ReRunOnce),
            None
        );
    }

    /// **P5's two acts cannot be separated**: the only route to the clear's own
    /// moment is a value the clear alone produces.
    #[test]
    fn the_clear_moment_is_reached_only_through_the_clear() {
        let moment = the_moment_a_clear_reconciles_at(
            crate::setup_state::TheStateIsGoneAndReconciliationIsOwed::the_state_is_gone(),
        );
        assert_eq!(moment, ReconciliationMoment::TheSetupInProgressStateIsCleared);
    }

    /// **A pass that was held back says so, and a pass that ran says what it did
    /// record by record** — because a supervisor that reconciled quietly is
    /// indistinguishable from one that did not reconcile at all.
    #[test]
    fn what_a_pass_did_is_said_rather_than_left_to_be_inferred() {
        let mut held = WhatReconciliationDid::at(ReconciliationMoment::EverySupervisorStart);
        let setup_run = an_identity(IdentityKind::Operation);
        held.held_back_by_the_setup_run = Some(setup_run.clone());
        assert!(!held.it_ran());
        assert!(held.as_the_machine_says_it().contains(&setup_run.as_text()));

        let mut ran =
            WhatReconciliationDid::at(ReconciliationMoment::TheSetupInProgressStateIsCleared);
        let operation = an_identity(IdentityKind::Operation);
        ran.records.push(WhatOneRecordGot {
            operation: operation.clone(),
            what: WhatARecordGot::ItEndedTheJob(AnEndingOutcome {
                disposition: DispositionThatEndsAJob::GivenUp,
                completeness: None,
                free_space: None,
            }),
        });
        ran.the_role_worker = "the role is disabled and nothing was started".to_string();
        assert!(ran.it_ran());
        let said = ran.as_the_machine_says_it();
        assert!(said.contains(&operation.as_text()));
        assert!(said.contains("GivenUp"));
        // **S13's distinction, on the stream a person actually reads.**
        assert!(said.contains("never four empty sets"));
    }
}

//! Continuation, and the unchanged file — `File Browser` §9, K1 to K13, Rows
//! 199, 200 and 201.
//!
//! **Constraint:** the carry-on across a repair, the carry-on inside a single
//! file, and the file that changed while it was going up.
//!
//! **`Server protocol and transfers` S13 assigns the unchanged-source answer for
//! a file on the user's own computer here by name, and this module is that
//! answer.** Nothing here invents a second resume shape: S12 is taken entire and
//! [`crate::request`]'s transfers are that document's.
//!
//! **Markers used below that belong to another document, each with the document
//! it belongs to.** `Server protocol and transfers` S4, S5, S9, S12, S13, S15,
//! S15b, S22, S24, S26, S26a, S27, C11, C13, R6. `Servers` 12.2, T2.
//! `Dependency Upload & Remap` A10. `State, identity and recovery` D5.

use std::time::{Duration, SystemTime};

use smoozy_server_protocol::transfers::integrity::{
    compare, Digest, Running, WhatTheComparisonSaid,
};
use smoozy_server_protocol::transfers::moving::{
    LocalFailure, TheCallersPublicationGate, TheUsersOwnComputer, WhatTheGateSaid,
};
use smoozy_server_protocol::transfers::publication::{Intent, WhatTheFinalObjectReadsAs};
use smoozy_server_protocol::transfers::{
    Ending, StableFileIdentity, StoppedReason, TransferRecord,
};
use smoozy_servers::transfers::ConnectionLabel;

/// **K1: the two labels decide, and they are read rather than re-derived.**
///
/// `Servers` 12.2: a **replacement** connection runs `Server protocol and
/// transfers` C11's four steps, whose step 4 re-establishes channels for every
/// transfer whose record says *interrupted within the current application run*
/// and resumes each under S12; **a period-opening connection resumes nothing.**
///
/// **A break under a standing period records every in-flight transfer
/// *interrupted, never stopped*** (`Servers` T2), which is what makes the
/// carry-on the default rather than an act.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatTheLabelDoesToTransfers {
    /// C11 step 4 fires: every *interrupted* record of this run is
    /// re-established and resumed under S12.
    ResumeThisRunsInterruptedTransfers,
    /// **Nothing resumes.**
    ResumeNothing,
}

/// K1, read off `Servers`' own published type.
pub fn what_the_label_does_to_transfers(label: ConnectionLabel) -> WhatTheLabelDoesToTransfers {
    if label.c11_step_four_fires() {
        WhatTheLabelDoesToTransfers::ResumeThisRunsInterruptedTransfers
    } else {
        WhatTheLabelDoesToTransfers::ResumeNothing
    }
}

/// **K1's own filter, applied to one record.** C11 step 4 resumes a record that
/// says *interrupted* **and** was written by the current application run.
pub fn c11_step_four_resumes(
    record: &TransferRecord,
    this_run: &smoozy_state::identities::Identity,
) -> bool {
    record
        .ending
        .as_ref()
        .is_some_and(Ending::may_the_work_be_carried_on)
        && &record.written_by_run == this_run
}

/// **K2: while the line is down the request reads as waiting (U5), and a waiting
/// upload is stoppable (U6).**
///
/// *"When several requests across several servers all take one repair at once,
/// admission runs under §6's declared order — that is the moment Row 189 was
/// declared for."*
pub const ADMISSION_AT_A_REPAIR_RUNS_UNDER_THE_DECLARED_ORDER: bool = true;

/// **K3: the line not crossed.**
///
/// *"Nothing resumes on a period-opening connection, nothing resumes from an
/// earlier run (`Server protocol and transfers` C11 step 4), and no upload starts
/// itself in a fresh sitting (U8) — C13's rule, the spec's own sentence, kept by
/// construction."*
pub const NOTHING_RESUMES_ITSELF_ACROSS_A_RUN_OR_A_PERIOD_OPENING: bool = true;

/// **K4: the mechanism is `Server protocol and transfers` S12's checkpointed,
/// verified resume, entire.**
///
/// *"Read the staged size, take the highest checkpoint at or below it, verify
/// the staged prefix's digest against the record, carry on from the checkpoint
/// or start over into a fresh staging path. **The waste is bounded at one
/// checkpoint interval**; the interval's tuning is Build's inside S9's record
/// shape, and no promise rests on its value."*
///
/// **The interval is that document's constant and is read rather than spelled
/// again here**, which is what keeps one figure in one place.
pub fn the_waste_a_resume_can_throw_away() -> u64 {
    smoozy_server_protocol::transfers::moving::THE_CHECKPOINT_INTERVAL
}

/// **K5: the escalation this row carried is spent, and it is recorded as spent
/// rather than raised.**
///
/// The spec's sources said a negative on resumability would be a hole in the
/// spec; **Fact 17 answered yes, in both directions, exactly, with matching
/// checksums.** The two conditions that rode with the answer are about `rsync` —
/// Apple's protocol-29 build resuming nothing on one box's sending direction, and
/// a tool's own partial directory unreliable after a hard kill — **and neither
/// reaches this route**: S4 and S5 put the staging name inside Smoozy, S12
/// verifies rather than trusts, and **no tool's resume flag exists anywhere in
/// the mechanism.**
pub const ROW_200S_ESCALATION_IS_SPENT: bool = true;

/// **K6: Fact 30's findings, posed on the Creator's own Mac on 2026-08-20** with
/// a real Blender save and a real Blender texture export, in a throwaway that
/// touched nothing of the Creator's.
///
/// Label: **observed** (macOS 26.5.2, APFS, Blender 4.5.12 and 5.0.1).
///
/// **One application, two save shapes, each defeating the check the other
/// suggests. What follows is built so that no clause rests on either shape.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HowASaveLands {
    /// **A real `.blend` save is a swap-in.** Both versions wrote a temporary
    /// `name.blend@` beside the file and renamed it over the final name: the
    /// path's inode, size and modification time all changed in **one atomic
    /// transition**, caught by a path reading within 100 ms. **The handle held
    /// open across the save went on delivering the pre-save file, whole and
    /// coherent** — so any arrangement that watches or trusts the open handle is
    /// defeated by it.
    ASwapIn,
    /// **A real texture export writes in place.** The same application's image
    /// save truncated the existing PNG to zero and rewrote it under the **same
    /// inode**, and the rewritten file came out **byte-count-identical** to the
    /// old one — only the modification time moved. **So the inode catches
    /// nothing there, the size catches nothing there, and a held handle delivers
    /// a mixture of old prefix and new tail.**
    InPlace,
}

/// K6's two shapes, and what each defeats.
pub const THE_TWO_OBSERVED_SAVE_SHAPES: [HowASaveLands; 2] =
    [HowASaveLands::ASwapIn, HowASaveLands::InPlace];

/// **K6's control: a run with no save produced zero identity changes and equal
/// digests** — the watch can fail, so it is a test.
pub const THE_CONTROL_RAN_AND_THE_WATCH_CAN_FAIL: bool = true;

/// **K7: the arrangement identity.**
///
/// *"When a transfer is arranged — and again when a quiet period ends — the
/// source's path identity is read and recorded: size, modification time at the
/// finest the system reports, and the file identifier where the system has
/// one. **Read by path, never by handle.**"*
///
/// **It is a reading and carries its moment** (`State, identity and recovery`
/// D5); comparisons of it are always desk-clock-to-desk-clock readings of one
/// path, **so no clocks are mixed.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathIdentity {
    pub size: u64,
    /// **At the finest the system reports.** The unit is nanoseconds because
    /// that is the finest any of the three systems documents, and a system that
    /// reports less fills the low digits with zeroes rather than the value being
    /// rounded here.
    pub modification_time_nanoseconds: i128,
    /// **Where the system has one.** `None` is a system that does not answer,
    /// recorded as a fact rather than left as a blank — the same shape `Server
    /// protocol and transfers` S15's identity-unavailable branch takes.
    pub file_identifier: Option<String>,
    /// The moment the reading was taken, on the desktop's own clock (D5).
    pub read_at: SystemTime,
}

impl PathIdentity {
    /// **Any difference is a difference.** The moment is not compared — it is
    /// the reading's own moment and not part of what was read.
    pub fn is_the_same_file_as(&self, other: &PathIdentity) -> bool {
        self.size == other.size
            && self.modification_time_nanoseconds == other.modification_time_nanoseconds
            && self.file_identifier == other.file_identifier
    }
}

/// **K7's reading, and it is read by path.** There is no method here that takes
/// a handle, which is how *never by handle* is kept: Fact 30 showed a held
/// handle going on delivering the pre-save file after a swap-in.
pub trait ReadsAPathIdentity {
    fn read_by_path(&mut self, path: &str) -> Option<PathIdentity>;
}

/// **K8: the watch — the mid-upload clause's early half.**
///
/// *"At each accounting interval of the byte loop — bounded in time by `Server
/// protocol and transfers` R6's fixed short period — the path identity is
/// re-read and compared. **Any difference stops that transfer**: the record is
/// written **stopped**, the staged material is never published (S24's shape) and
/// is removed at once or owed as a cleanup under S27, and the user is told the
/// file changed while it was being sent, with the offer to send the new
/// version."*
///
/// **Both observed save shapes move the modification time, so both are caught
/// within one interval on the observed system.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatTheWatchSaw {
    /// Nothing moved.
    Unchanged,
    /// **Any difference.** The transfer stops.
    ItChanged,
    /// The reading could not be taken at all. **Nothing is claimed about the
    /// user's file**, and the transfer goes on: K9's gate is contents arithmetic
    /// at publication and lands nothing that changed, so a watch that cannot see
    /// costs earliness and never correctness.
    ItCouldNotBeRead,
}

/// K8's comparison.
pub fn what_the_watch_saw(
    arrangement: &PathIdentity,
    now: Option<&PathIdentity>,
) -> WhatTheWatchSaw {
    match now {
        None => WhatTheWatchSaw::ItCouldNotBeRead,
        Some(now) if arrangement.is_the_same_file_as(now) => WhatTheWatchSaw::Unchanged,
        Some(_) => WhatTheWatchSaw::ItChanged,
    }
}

/// **The ending an arrest of Smoozy's own carries** (K8, K9, X7).
///
/// *"`Server protocol and transfers` S26 closes the set at three … **A transfer
/// Smoozy stopped must be neither of those**, or a failed file would show amber
/// as waiting for the connection on a live and healthy line and would then be
/// resumed at the next repair. **So the record says *stopped***."*
///
/// **That S26 words the row *Stopped, deliberately* while this is Smoozy
/// stopping rather than the user is settled in that document**: its S26a says
/// *deliberately* means *on a deliberate instruction* and need not be the
/// user's.
pub fn the_ending_a_changed_source_writes() -> Ending {
    Ending::Stopped(StoppedReason::TheSourceChangedUnderIt)
}

/// **K9: the publication gate — the clause the no-corruption promise rests on,
/// and it rests on arithmetic.**
///
/// *"Before any upload's publication, **the source is read through once more and
/// digested, and publication happens only where that digest equals the
/// transfer's own pass-through digest** — the digest of what was actually sent,
/// computed as the bytes passed (`Server protocol and transfers` S22)."*
///
/// **What the gate delivers, stated as what it delivers:** two full readings of
/// one path being equal while their contents differ is impossible for a
/// quiescent file and is *not* impossible for one being written concurrently, so
/// **no impossibility is claimed.** What the gate does is reduce a miss to a
/// coincidence that is **astronomically unlikely rather than impossible**, and
/// where the two agree, what was sent is one coherent version.
///
/// **So a torn read, a swap the watch missed, a writer that restores
/// modification times, and any save on any operating system whose stat moves
/// nothing are all caught at the last moment that matters, by contents alone.**
///
/// The cost is one local read-through per file — about 1.3 to 2.2 seconds for a
/// 670,000,000-byte file on the observed Mac (Fact 15), against an upload of the
/// same file measured in minutes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhetherPublicationMayHappen {
    /// The two digests agree. **What was sent is one coherent version.**
    Yes,
    /// **The transfer is stopped with the same changed-while-sending answer** —
    /// the record written *stopped*, as at K8 — **and nothing is published.**
    NoTheSourceChangedWhileItWasBeingSent,
}

/// K9's arithmetic, and nothing else.
pub fn whether_publication_may_happen(
    the_source_read_through: &Digest,
    the_pass_through_digest: &Digest,
) -> WhetherPublicationMayHappen {
    match compare(the_source_read_through, the_pass_through_digest) {
        WhatTheComparisonSaid::TheyAgree => WhetherPublicationMayHappen::Yes,
        WhatTheComparisonSaid::TheyDisagree => {
            WhetherPublicationMayHappen::NoTheSourceChangedWhileItWasBeingSent
        }
    }
}

/// **K9's own claim, stated so nobody strengthens it.** No impossibility is
/// claimed: a concurrent writer leaving the second full reading with the same
/// digest as the bytes that were actually sent is astronomically unlikely rather
/// than impossible.
pub const THE_GATE_REDUCES_A_MISS_TO_A_COINCIDENCE_AND_CLAIMS_NO_IMPOSSIBILITY: bool = true;

/// **K10: the resume rule — S13's answer, and `Dependency Upload & Remap` A10
/// adopted.**
///
/// A resume of an upload proceeds only where both hold:
///
/// 1. **the path identity equals the arrangement identity** (K7). Where it
///    differs, **the upload does not resume**: Smoozy says the file changed while
///    it was being sent and offers to send the new version — the spec's own
///    sentence for a save that landed while the line was down. **Nothing is
///    claimed beyond what the identity showed.**
/// 2. **the staged prefix verifies by contents against the source as it now
///    is** — A10's test, **adopted**: the source is read up to the highest
///    checkpoint and digested, and the resume proceeds only where that digest
///    equals the checkpoint's recorded running digest, **so carrying on writes
///    exactly the current source's bytes.**
///
/// Where the identity is unchanged and the prefix still does not verify, **the
/// transfer starts over from zero into a fresh staging path** (S12 step 5) with
/// no claim made about the user's file — *"staging damage and a stat-invisible
/// change are indistinguishable there, and starting over is safe under both."*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatTheResumeDoes {
    /// Both clauses hold. **A resumed transfer's running digest continues from
    /// the checkpoint, so K9's gate still compares the whole file at
    /// publication.**
    ItMayProceed,
    /// Clause 1 failed. **Smoozy says the file changed and offers the new
    /// version.**
    StopAndOfferTheNewVersion,
    /// Clause 1 held and clause 2 did not.
    StartOverFromZeroIntoAFreshStagingPath,
}

/// K10, over the two clauses.
pub fn what_the_resume_does(
    arrangement: &PathIdentity,
    now: Option<&PathIdentity>,
    the_prefix_digest: Option<&Digest>,
    the_checkpoints_recorded_digest: Option<&Digest>,
) -> WhatTheResumeDoes {
    match what_the_watch_saw(arrangement, now) {
        WhatTheWatchSaw::ItChanged => return WhatTheResumeDoes::StopAndOfferTheNewVersion,
        // **A reading that could not be taken is not a claim that the file
        // changed.** Clause 1 cannot be met, so the resume does not proceed, and
        // starting over is the branch that is safe under both readings — the
        // same ending clause 2's failure takes.
        WhatTheWatchSaw::ItCouldNotBeRead => {
            return WhatTheResumeDoes::StartOverFromZeroIntoAFreshStagingPath
        }
        WhatTheWatchSaw::Unchanged => {}
    }
    match (the_prefix_digest, the_checkpoints_recorded_digest) {
        (Some(read_now), Some(recorded))
            if compare(read_now, recorded) == WhatTheComparisonSaid::TheyAgree =>
        {
            WhatTheResumeDoes::ItMayProceed
        }
        _ => WhatTheResumeDoes::StartOverFromZeroIntoAFreshStagingPath,
    }
}

/// **K11: the quiet period is two seconds, and it is a test on the path identity
/// — so what it can see and what it cannot are both stated.**
///
/// *"On accepting the offer — and before any send of a file that changed — K7's
/// path identity is polled at the accounting period, and the send starts when it
/// has held unchanged for **two seconds**."*
///
/// **The number's justification is about the transition, not about the
/// writing.** What the observation establishes is that each shape's identity
/// *transition* completed inside one 100 ms poll — which is why two seconds is a
/// comfortable margin for a change the poll can see, and why it says nothing
/// about how long a save spends preparing one. **Two seconds is imperceptible
/// against a transfer measured in minutes. It is a declared constant, not a
/// tuning left open.**
pub const THE_QUIET_PERIOD: Duration = Duration::from_secs(2);

/// **K11's honest halves.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatTheQuietPeriodSees {
    /// **It detects an in-place writer.** Such a writer moves the path's own
    /// values while it works: the observed texture export truncated the file to
    /// zero and then rewrote it, and both moments were caught by 100 ms polling,
    /// **so the quiet period sees the writing and waits it out.**
    AnInPlaceWriterIsSeenAndWaitedOut,
    /// **It cannot detect a swap-in writer still preparing its replacement.**
    /// That shape writes a temporary beside the file and moves nothing at the
    /// path until the final rename, and the observed `.blend` save is that
    /// shape — **so the two-second test passes immediately while the save is
    /// still running. On the save shape Row 201's first clause is written about,
    /// the quiet period is vacuous.**
    ASwapInIsInvisibleSoTheTestIsVacuous,
}

/// K11's two halves, per observed shape.
pub fn what_the_quiet_period_sees(shape: HowASaveLands) -> WhatTheQuietPeriodSees {
    match shape {
        HowASaveLands::InPlace => WhatTheQuietPeriodSees::AnInPlaceWriterIsSeenAndWaitedOut,
        HowASaveLands::ASwapIn => WhatTheQuietPeriodSees::ASwapInIsInvisibleSoTheTestIsVacuous,
    }
}

/// **K11's own note, added because the silence should not be read as *nothing
/// else exists*.**
///
/// A standard instrument for *has anything landed in this folder* exists and is
/// documented on all three systems — Apple's File System Events, Linux's
/// `inotify(7)` and Microsoft's `ReadDirectoryChangesW`. **None of them asks the
/// caller for a filename or a naming convention**, so none of them is what Rule
/// 2 forbids. **They are not adopted here and adopting one is not this
/// document's to decide.**
///
/// **One caution travels with them if they are ever taken up**, from Microsoft's
/// own page: on buffer overflow the entire contents of the buffer are discarded
/// and the caller must then enumerate the directory itself, **so a notification
/// route needs the polling route underneath it rather than instead of it.**
pub const NO_NOTIFICATION_ROUTE_IS_ADOPTED_HERE: bool = true;

/// K11's poll, as a state machine so the two seconds are one constant in one
/// place.
///
/// **The readings are desk-clock readings of one path**, so nothing here
/// subtracts two clocks (D5).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TheQuietPeriod {
    held: Option<(PathIdentity, SystemTime)>,
}

/// What the quiet period answers at one poll.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhetherTheSendMayStart {
    /// **It has held unchanged for two seconds.**
    Yes,
    /// Still waiting it out.
    NotYet,
}

impl TheQuietPeriod {
    pub fn new() -> TheQuietPeriod {
        TheQuietPeriod { held: None }
    }

    /// One poll at the accounting period.
    pub fn a_reading(
        &mut self,
        identity: Option<PathIdentity>,
        at: SystemTime,
    ) -> WhetherTheSendMayStart {
        let Some(identity) = identity else {
            // A reading that could not be taken restarts the wait rather than
            // being read as unchanged.
            self.held = None;
            return WhetherTheSendMayStart::NotYet;
        };
        match &self.held {
            Some((held, since)) if held.is_the_same_file_as(&identity) => {
                let held_for = at.duration_since(*since).unwrap_or_default();
                if held_for >= THE_QUIET_PERIOD {
                    WhetherTheSendMayStart::Yes
                } else {
                    WhetherTheSendMayStart::NotYet
                }
            }
            _ => {
                self.held = Some((identity, at));
                WhetherTheSendMayStart::NotYet
            }
        }
    }
}

/// **K12: this section is the one unchanged-source answer, for every upload of a
/// local source.**
///
/// `Server protocol and transfers` S13 reads it as: *a resume of an upload
/// resumes only under K10, and otherwise starts over or enters the offer flow as
/// K10 says.* **`Dependency Upload & Remap` follows this row by its own words**:
/// its A10 stands as adopted inside K10, its uploads inherit the watch, the gate
/// and the quiet period unchanged, and the torn-mid-read exposure its A10
/// deliberately left open is closed by K9 for its transfers exactly as for this
/// document's.
pub const THIS_IS_THE_ONE_UNCHANGED_SOURCE_ANSWER_FOR_EVERY_LOCAL_SOURCE: bool = true;

/// **K13: what is answered where, and what breaks where it is not.**
///
/// The mid-upload clause, the resume clause and the quiet period are **answered
/// on macOS by observation.** On Windows and Linux the mechanism is identical
/// and two things are unestablished: **whether every real application save moves
/// the path identity** (K8's earliness) — and on Windows that is worse than
/// untested, since Microsoft documents the modification time as not fully
/// updated while a writing handle is open — **and which filesystem a given Linux
/// machine actually carries.**
///
/// **PERMANENTLY UNRESOLVED** (§13 item 6): the Creator ruled on 2026-08-31 that
/// there will be no further Blender research and no further rental to answer a
/// question, and **the reading route was taken first and is exhausted rather
/// than untried.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum System {
    Mac,
    Windows,
    Linux,
}

/// **The two silent shapes K13 names, and both are a promise not firing rather
/// than a wording.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ASilentShape {
    /// **A silent start-over** — K10 clause 1 passes on an unmoved identity,
    /// clause 2 fails on contents, and `Server protocol and transfers` S12 step
    /// 5 restarts the transfer from zero **with nothing said and nothing
    /// offered.**
    ASilentStartOver,
    /// **A silent carry-on** — a writer that changed only the region beyond the
    /// highest checkpoint, in place, and restored the modification time: both
    /// clauses pass, and the resume carries on, **again with nothing said and
    /// nothing offered.**
    ASilentCarryOn,
}

/// K13, per system. **What is at risk there is a promise, not a wording.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatIsAnsweredOn {
    pub system: System,
    /// Whether K8's earliness is answered by observation.
    pub the_watchs_earliness_is_observed: bool,
    /// **The no-corruption promise itself holds on all three systems**, because
    /// K9's gate is contents arithmetic and lands nothing that changed.
    pub the_no_corruption_promise_holds: bool,
    /// The shapes that can go silent where the path identity does not move.
    pub the_silent_shapes: &'static [ASilentShape],
}

/// K13's table.
pub const WHAT_IS_ANSWERED_WHERE: [WhatIsAnsweredOn; 3] = [
    WhatIsAnsweredOn {
        system: System::Mac,
        the_watchs_earliness_is_observed: true,
        the_no_corruption_promise_holds: true,
        // **K11's quiet period is vacuous for the swap-in shape on every system,
        // the observed Mac included** — what follows there is the spec's own
        // stop-and-offer loop rather than a broken promise.
        the_silent_shapes: &[],
    },
    WhatIsAnsweredOn {
        system: System::Windows,
        the_watchs_earliness_is_observed: false,
        the_no_corruption_promise_holds: true,
        the_silent_shapes: &[ASilentShape::ASilentStartOver, ASilentShape::ASilentCarryOn],
    },
    WhatIsAnsweredOn {
        system: System::Linux,
        the_watchs_earliness_is_observed: false,
        the_no_corruption_promise_holds: true,
        the_silent_shapes: &[ASilentShape::ASilentStartOver, ASilentShape::ASilentCarryOn],
    },
];

/// **K13's *Holds*: what each silent shape falls back to is honest and
/// granted.**
///
/// *"The start-over is the spec's own alternative in the same paragraph, sends
/// that file again from the beginning where it cannot tell the file is unchanged;
/// the carry-on completes the current source's bytes, which is what K10 clause 2
/// tested for. **What neither delivers is Row 201's saying and offering**, and
/// calling that *wording* would invite a reader to treat it as cosmetic."*
pub const WHAT_NEITHER_SILENT_SHAPE_DELIVERS_IS_THE_SAYING_AND_THE_OFFERING: bool = true;

/// **Why this transfer stopped, where the watch or the gate caught it.**
///
/// Read off [`TheSourceUnderWatch`] and off [`TheSourcesPublicationGate`] after
/// a run, so the caller reports which of K8 and K9 caught it. **Both end the
/// transfer the same way** — `Server protocol and transfers` S26's second row —
/// and both say the same thing to the user, which K8 and K10 clause 1 fix; what
/// this separates is the record, not the sentence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhyItStopped {
    /// K8's watch saw the path identity move.
    TheWatchSawTheSourceChange,
    /// K9's gate found the read-through and the pass-through digest disagreeing.
    ThePublicationGateRefused,
}

/// **The source end of one upload: K7's identity recorded and K8's watch
/// running.** K9's gate is [`TheSourcesPublicationGate`] beside it.
///
/// **This is one of the two places §9 acts rather than decides**, and both are
/// the source end because that is where §9's clauses sit: the arrangement
/// identity, the re-read and the read-through are all readings of the user's own
/// file, and `Server protocol and transfers` S18's contract is the seam every one
/// of them already crosses.
///
/// **How K8's stop reaches the byte loop, stated plainly because it is a seam
/// and not an obvious one.** That document's loop reads an upload's bytes
/// through [`TheUsersOwnComputer::read_at`] and treats a read that returns
/// nothing as *the source changed under it*, ending the transfer
/// `Stopped(TheSourceChangedUnderIt)` — which is the ending K8 requires. **So
/// K8's refusal is delivered as a read with no bytes in it.** That document
/// names no mid-transfer gate a caller can ask, so this is the route there is;
/// **K9's refusal no longer travels it** and goes through S15b instead.
///
/// **What this type no longer holds, and why the removal is the point.** It used
/// to build S22's pass-through digest itself and run K9 on the read that
/// completed the file. S15b hands the gate that digest out of the loop's own
/// running value, so a second copy of it here would be one value with two homes,
/// and the loop's copy is the one that covers a resume's verified prefix without
/// this end being told about it.
pub struct TheSourceUnderWatch<'a, L, I> {
    inner: &'a mut L,
    identities: &'a mut I,
    source_path: String,
    arrangement: PathIdentity,
    why_it_stopped: Option<WhyItStopped>,
}

impl<'a, L: TheUsersOwnComputer, I: ReadsAPathIdentity + Send> TheSourceUnderWatch<'a, L, I> {
    /// **K7's record, taken when the transfer is arranged.**
    ///
    /// `None` where the identity cannot be read at all: **nothing is guessed**,
    /// and a transfer with no arrangement identity is one K8 cannot watch, which
    /// K9's gate still covers.
    pub fn arranged(
        inner: &'a mut L,
        identities: &'a mut I,
        source_path: String,
    ) -> Option<TheSourceUnderWatch<'a, L, I>> {
        let arrangement = identities.read_by_path(&source_path)?;
        Some(TheSourceUnderWatch {
            inner,
            identities,
            source_path,
            arrangement,
            why_it_stopped: None,
        })
    }

    /// Whether K8 caught it.
    pub fn why_it_stopped(&self) -> Option<WhyItStopped> {
        self.why_it_stopped
    }

    /// The arrangement identity this watch was opened on.
    pub fn arrangement(&self) -> &PathIdentity {
        &self.arrangement
    }
}

/// **K9's gate, declared to the byte loop by name — `Server protocol and
/// transfers` S15b.**
///
/// *"The caller supplies a gate with the publication. The loop asks it once, at
/// the moment every byte of the transfer has passed and §7.6's end-to-end
/// comparison has been made, and before S15's durable pre-state is written. It
/// is handed the transfer's own pass-through digest (S22)."*
///
/// **That is exactly K9's own moment** — *before any upload's publication*, and
/// after everything that was sent has passed — so the read-through is taken here
/// rather than on the last bite of the loop, and it covers a change that landed
/// during the last write as well as one that landed earlier.
///
/// **It reads the user's own file on its own handle to the adapter**, beside the
/// watch's. Two `&mut` borrows of one adapter cannot exist at once, and the loop
/// holds both the local end and the gate for the whole of the transfer, so one
/// object cannot be both; `Desktop app and delivery`'s platform file adapter
/// carries no state, and a second handle to it is a second reader of the same
/// file rather than a second contract.
pub struct TheSourcesPublicationGate<'a, L> {
    inner: &'a mut L,
    source_path: String,
    total_size_expected: u64,
    why_it_stopped: Option<WhyItStopped>,
}

impl<'a, L: TheUsersOwnComputer> TheSourcesPublicationGate<'a, L> {
    /// The gate for one upload, over its own reading handle.
    pub fn over(
        inner: &'a mut L,
        source_path: String,
        total_size_expected: u64,
    ) -> TheSourcesPublicationGate<'a, L> {
        TheSourcesPublicationGate {
            inner,
            source_path,
            total_size_expected,
            why_it_stopped: None,
        }
    }

    /// Whether K9 caught it.
    pub fn why_it_stopped(&self) -> Option<WhyItStopped> {
        self.why_it_stopped
    }

    /// **K9's read-through, to the source's real end and not to the size the
    /// transfer expected**, over the adapter, in bites.
    ///
    /// K9's guarantee is stated as *"two full readings of one path being equal
    /// while their contents differ is impossible for a quiescent file"*, and the
    /// clause is *"the source is read through once more and digested"* — so a
    /// reading that stops at the expected size is **not a full reading of the
    /// file**, and the argument does not hold for it. A source that has become
    /// **longer** still digests to what was sent over its first bytes, so the
    /// digests would agree and the gate would say publish — while Row 201's
    /// first clause is that a file changed while it was going up is stopped, not
    /// landed. **Reading to the real end removes that asymmetry rather than
    /// patching one side of it**: anything past the expected size is answered as
    /// a change, which is the same answer the shorter case already gets.
    fn read_the_source_through(&mut self) -> Option<Digest> {
        let mut running = Running::new();
        let mut at = 0u64;
        while at < self.total_size_expected {
            let want = (self.total_size_expected - at).min(A_BITE as u64) as usize;
            let bytes = self.inner.read_at(&self.source_path, at, want).ok()?;
            if bytes.is_empty() {
                // The file is shorter than it was. That is a change, and the
                // gate answers it as one rather than digesting a short file.
                return None;
            }
            running.take(&bytes);
            at += bytes.len() as u64;
        }
        // **And the file's real end is where the transfer expected it.** One
        // more read past the expected size: a quiescent source of exactly that
        // size reads empty, and anything at all there is a source that grew.
        if !self
            .inner
            .read_at(&self.source_path, at, A_BITE)
            .ok()?
            .is_empty()
        {
            return None;
        }
        Some(running.so_far())
    }
}

impl<L: TheUsersOwnComputer> TheCallersPublicationGate for TheSourcesPublicationGate<'_, L> {
    /// **K9's comparison, at S15b's moment.** The digest handed in is the
    /// loop's own pass-through digest — the digest of what was actually sent —
    /// and nothing here composes a second one.
    ///
    /// **A read-through that cannot be completed is a refusal and never a
    /// pass**: a source that has become shorter than the transfer expected has
    /// changed, and K9 answers it as a change rather than digesting a short
    /// file. **A source that has become longer is the same answer**, on the same
    /// ground — see [`Self::read_the_source_through`].
    fn may_this_be_published(&mut self, what_was_sent: &Digest) -> WhatTheGateSaid {
        let read_through = self.read_the_source_through();
        let may = match read_through {
            Some(read_through) => whether_publication_may_happen(&read_through, what_was_sent),
            None => WhetherPublicationMayHappen::NoTheSourceChangedWhileItWasBeingSent,
        };
        match may {
            WhetherPublicationMayHappen::Yes => WhatTheGateSaid::ItMayBePublished,
            WhetherPublicationMayHappen::NoTheSourceChangedWhileItWasBeingSent => {
                self.why_it_stopped = Some(WhyItStopped::ThePublicationGateRefused);
                WhatTheGateSaid::TheSourceChangedUnderIt
            }
        }
    }
}

/// How big a bite K9's read-through takes. **The figure is Build's and no
/// promise rests on it**: the cost the clause quotes is 1.3 to 2.2 seconds for a
/// 670,000,000-byte file, which is a rate rather than a bite size.
const A_BITE: usize = 1024 * 1024;

impl<L: TheUsersOwnComputer, I: ReadsAPathIdentity + Send> TheUsersOwnComputer
    for TheSourceUnderWatch<'_, L, I>
{
    /// **K8's watch, at the one call the byte loop makes against the user's own
    /// file.**
    ///
    /// The watch runs before every bite, which is at least as often as the
    /// loop's own accounting interval whenever the loop is moving at all; a
    /// channel holding an allowance it cannot spend reads nothing and is watched
    /// again at its next bite, which K9's gate covers either way.
    fn read_at(&mut self, path: &str, offset: u64, length: usize) -> Result<Vec<u8>, LocalFailure> {
        if path != self.source_path {
            // A read of something other than this transfer's source — the
            // staged prefix on a download, for one — is passed straight
            // through: the watch is about this file and no other.
            return self.inner.read_at(path, offset, length);
        }

        // ---- K8.
        let now = self.identities.read_by_path(path);
        if what_the_watch_saw(&self.arrangement, now.as_ref()) == WhatTheWatchSaw::ItChanged {
            self.why_it_stopped = Some(WhyItStopped::TheWatchSawTheSourceChange);
            return Ok(Vec::new());
        }

        self.inner.read_at(path, offset, length)
    }

    fn staged_size(&mut self, path: &str) -> Option<u64> {
        self.inner.staged_size(path)
    }

    fn create_staging_exclusively(&mut self, path: &str) -> Result<(), LocalFailure> {
        self.inner.create_staging_exclusively(path)
    }

    fn write_at(&mut self, path: &str, offset: u64, bytes: &[u8]) -> Result<(), LocalFailure> {
        self.inner.write_at(path, offset, bytes)
    }

    fn make_durable(&mut self, path: &str) -> Result<(), LocalFailure> {
        self.inner.make_durable(path)
    }

    fn stable_identity_of(&mut self, path: &str) -> StableFileIdentity {
        self.inner.stable_identity_of(path)
    }

    fn publish(
        &mut self,
        staging: &str,
        final_path: &str,
        intent: Intent,
    ) -> Result<(), LocalFailure> {
        self.inner.publish(staging, final_path, intent)
    }

    fn remove(&mut self, path: &str) -> Result<(), LocalFailure> {
        self.inner.remove(path)
    }

    fn read_the_final_object(&mut self, path: &str) -> WhatTheFinalObjectReadsAs {
        self.inner.read_the_final_object(path)
    }

    /// **`Server protocol and transfers` S15c: the time the server copy
    /// takes is K7's**, the arrangement identity this watch was opened on, in
    /// the whole seconds that call carries — not a fresh reading at the end.
    fn the_sources_time(&mut self, source_path: &str) -> Option<i64> {
        if source_path != self.source_path {
            return self.inner.the_sources_time(source_path);
        }
        i64::try_from(
            self.arrangement
                .modification_time_nanoseconds
                .div_euclid(1_000_000_000),
        )
        .ok()
    }

    fn keep_the_time(&mut self, staging: &str, seconds: i64) -> Result<(), LocalFailure> {
        self.inner.keep_the_time(staging, seconds)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::{Arc, Mutex};

    use smoozy_server_protocol::subsystem::client::Subsystem;
    use smoozy_server_protocol::subsystem::wire::{kind, Composed, Fields};
    use smoozy_server_protocol::transfers::moving::{
        carry_out_under_a_publication_gate, KeepsTheRecord, NoCapIsInForce, NobodyIsWatching,
        Transfer,
    };
    use smoozy_server_protocol::transfers::Direction;
    use smoozy_state::identities::{Identity, IdentityKind};
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    fn an_identity(kind: IdentityKind) -> Identity {
        Identity::issue(kind).expect("this machine supplies random bytes")
    }

    fn an_identity_of(size: u64, time: i128, file: Option<&str>) -> PathIdentity {
        PathIdentity {
            size,
            modification_time_nanoseconds: time,
            file_identifier: file.map(str::to_string),
            read_at: SystemTime::UNIX_EPOCH,
        }
    }

    #[test]
    fn the_two_labels_decide_and_are_read_rather_than_re_derived() {
        assert_eq!(
            what_the_label_does_to_transfers(ConnectionLabel::Replacement),
            WhatTheLabelDoesToTransfers::ResumeThisRunsInterruptedTransfers
        );
        assert_eq!(
            what_the_label_does_to_transfers(ConnectionLabel::PeriodOpening),
            WhatTheLabelDoesToTransfers::ResumeNothing
        );
        const { assert!(NOTHING_RESUMES_ITSELF_ACROSS_A_RUN_OR_A_PERIOD_OPENING) };
        const { assert!(ADMISSION_AT_A_REPAIR_RUNS_UNDER_THE_DECLARED_ORDER) };
    }

    #[test]
    fn c11_step_four_takes_this_runs_interrupted_records_and_no_others() {
        let this_run = an_identity(IdentityKind::Run);
        let mut record = TransferRecord::arranged(
            an_identity(IdentityKind::Transfer),
            Direction::Up,
            an_identity(IdentityKind::LocalServer),
            None,
            "/home/a.png".to_string(),
            "/jobs/a.png".to_string(),
            "/jobs/.smoozy-staging",
            10,
            this_run.clone(),
        );
        record.ending = Some(Ending::Interrupted);
        assert!(c11_step_four_resumes(&record, &this_run));
        // An earlier run's record is not resumed.
        assert!(!c11_step_four_resumes(
            &record,
            &an_identity(IdentityKind::Run)
        ));
        // A transfer Smoozy stopped itself is not resumed either.
        record.ending = Some(the_ending_a_changed_source_writes());
        assert!(!c11_step_four_resumes(&record, &this_run));
    }

    #[test]
    fn an_arrest_of_smoozys_own_writes_stopped_and_never_interrupted() {
        let ending = the_ending_a_changed_source_writes();
        assert_eq!(
            ending,
            Ending::Stopped(StoppedReason::TheSourceChangedUnderIt)
        );
        assert!(
            !ending.may_the_work_be_carried_on(),
            "it must never compose as waiting, nor be picked up at the next repair"
        );
    }

    #[test]
    fn the_watch_stops_on_any_difference_and_claims_nothing_where_it_cannot_read() {
        let arrangement = an_identity_of(100, 5_000, Some("i1"));
        assert_eq!(
            what_the_watch_saw(&arrangement, Some(&an_identity_of(100, 5_000, Some("i1")))),
            WhatTheWatchSaw::Unchanged
        );
        // The swap-in shape: inode, size and modification time all move.
        assert_eq!(
            what_the_watch_saw(&arrangement, Some(&an_identity_of(120, 9_000, Some("i2")))),
            WhatTheWatchSaw::ItChanged
        );
        // The in-place shape: only the modification time moves.
        assert_eq!(
            what_the_watch_saw(&arrangement, Some(&an_identity_of(100, 9_000, Some("i1")))),
            WhatTheWatchSaw::ItChanged
        );
        assert_eq!(
            what_the_watch_saw(&arrangement, None),
            WhatTheWatchSaw::ItCouldNotBeRead
        );
    }

    #[test]
    fn the_moment_a_reading_carries_is_not_part_of_what_was_read() {
        let one = an_identity_of(100, 5_000, Some("i1"));
        let mut other = an_identity_of(100, 5_000, Some("i1"));
        other.read_at = SystemTime::UNIX_EPOCH + Duration::from_secs(9_999);
        assert!(one.is_the_same_file_as(&other));
    }

    #[test]
    fn the_gate_publishes_only_where_the_two_digests_agree() {
        let sent = Digest::of(b"one coherent version");
        assert_eq!(
            whether_publication_may_happen(&Digest::of(b"one coherent version"), &sent),
            WhetherPublicationMayHappen::Yes
        );
        assert_eq!(
            whether_publication_may_happen(&Digest::of(b"something else"), &sent),
            WhetherPublicationMayHappen::NoTheSourceChangedWhileItWasBeingSent
        );
        const { assert!(THE_GATE_REDUCES_A_MISS_TO_A_COINCIDENCE_AND_CLAIMS_NO_IMPOSSIBILITY) };
    }

    #[test]
    fn the_resume_stops_on_a_moved_identity_and_starts_over_on_an_unverified_prefix() {
        let arrangement = an_identity_of(100, 5_000, Some("i1"));
        let same = an_identity_of(100, 5_000, Some("i1"));
        let moved = an_identity_of(100, 9_000, Some("i1"));
        let recorded = Digest::of(b"the prefix as it was");

        assert_eq!(
            what_the_resume_does(&arrangement, Some(&moved), Some(&recorded), Some(&recorded)),
            WhatTheResumeDoes::StopAndOfferTheNewVersion
        );
        assert_eq!(
            what_the_resume_does(&arrangement, Some(&same), Some(&recorded), Some(&recorded)),
            WhatTheResumeDoes::ItMayProceed
        );
        assert_eq!(
            what_the_resume_does(
                &arrangement,
                Some(&same),
                Some(&Digest::of(b"a different prefix")),
                Some(&recorded)
            ),
            WhatTheResumeDoes::StartOverFromZeroIntoAFreshStagingPath
        );
        assert_eq!(
            what_the_resume_does(&arrangement, None, Some(&recorded), Some(&recorded)),
            WhatTheResumeDoes::StartOverFromZeroIntoAFreshStagingPath,
            "an unreadable identity claims nothing about the user's file"
        );
    }

    #[test]
    fn the_quiet_period_is_two_seconds_and_a_change_restarts_the_wait() {
        assert_eq!(THE_QUIET_PERIOD, Duration::from_secs(2));
        let base = SystemTime::UNIX_EPOCH;
        let mut quiet = TheQuietPeriod::new();
        let steady = an_identity_of(100, 5_000, Some("i1"));
        assert_eq!(
            quiet.a_reading(Some(steady.clone()), base),
            WhetherTheSendMayStart::NotYet
        );
        assert_eq!(
            quiet.a_reading(Some(steady.clone()), base + Duration::from_millis(1_900)),
            WhetherTheSendMayStart::NotYet
        );
        assert_eq!(
            quiet.a_reading(Some(steady.clone()), base + Duration::from_millis(2_000)),
            WhetherTheSendMayStart::Yes
        );
        // A writer moving the path restarts the wait.
        let moved = an_identity_of(0, 9_000, Some("i1"));
        assert_eq!(
            quiet.a_reading(Some(moved), base + Duration::from_millis(2_100)),
            WhetherTheSendMayStart::NotYet
        );
        // And an unreadable poll restarts it rather than reading as unchanged.
        assert_eq!(
            quiet.a_reading(None, base + Duration::from_millis(5_000)),
            WhetherTheSendMayStart::NotYet
        );
    }

    #[test]
    fn the_quiet_period_is_vacuous_for_the_swap_in_shape_and_not_for_the_other() {
        assert_eq!(
            what_the_quiet_period_sees(HowASaveLands::ASwapIn),
            WhatTheQuietPeriodSees::ASwapInIsInvisibleSoTheTestIsVacuous
        );
        assert_eq!(
            what_the_quiet_period_sees(HowASaveLands::InPlace),
            WhatTheQuietPeriodSees::AnInPlaceWriterIsSeenAndWaitedOut
        );
        assert_eq!(THE_TWO_OBSERVED_SAVE_SHAPES.len(), 2);
        const { assert!(THE_CONTROL_RAN_AND_THE_WATCH_CAN_FAIL) };
        const { assert!(NO_NOTIFICATION_ROUTE_IS_ADOPTED_HERE) };
    }

    #[test]
    fn the_no_corruption_promise_holds_on_all_three_systems() {
        assert_eq!(WHAT_IS_ANSWERED_WHERE.len(), 3);
        for row in &WHAT_IS_ANSWERED_WHERE {
            assert!(
                row.the_no_corruption_promise_holds,
                "K9's gate is contents arithmetic on every system"
            );
        }
        let mac = &WHAT_IS_ANSWERED_WHERE[0];
        assert!(mac.the_watchs_earliness_is_observed);
        assert!(mac.the_silent_shapes.is_empty());
        for row in &WHAT_IS_ANSWERED_WHERE[1..] {
            assert!(!row.the_watchs_earliness_is_observed);
            assert_eq!(row.the_silent_shapes.len(), 2);
        }
        const { assert!(WHAT_NEITHER_SILENT_SHAPE_DELIVERS_IS_THE_SAYING_AND_THE_OFFERING) };
        const { assert!(THIS_IS_THE_ONE_UNCHANGED_SOURCE_ANSWER_FOR_EVERY_LOCAL_SOURCE) };
        const { assert!(ROW_200S_ESCALATION_IS_SPENT) };
    }

    #[test]
    fn the_waste_a_resume_throws_away_is_the_other_documents_own_constant() {
        assert_eq!(
            the_waste_a_resume_can_throw_away(),
            smoozy_server_protocol::transfers::moving::THE_CHECKPOINT_INTERVAL
        );
    }

    // ---- The source end, run against a fake adapter at the desk.

    #[derive(Default)]
    struct AFakeSource {
        bytes: Vec<u8>,
        identity: Option<PathIdentity>,
        /// Set to swap the file's contents the moment the loop asks for the
        /// last bite — the torn read K9 exists for.
        change_at_the_last_read_to: Option<Vec<u8>>,
        reads: usize,
    }

    impl ReadsAPathIdentity for AFakeSource {
        fn read_by_path(&mut self, _path: &str) -> Option<PathIdentity> {
            self.identity.clone()
        }
    }

    struct AFakeAdapter<'a>(&'a mut AFakeSource);

    impl TheUsersOwnComputer for AFakeAdapter<'_> {
        fn staged_size(&mut self, _path: &str) -> Option<u64> {
            None
        }
        fn read_at(
            &mut self,
            _path: &str,
            offset: u64,
            length: usize,
        ) -> Result<Vec<u8>, LocalFailure> {
            self.0.reads += 1;
            let from = offset as usize;
            if from >= self.0.bytes.len() {
                return Ok(Vec::new());
            }
            let to = (from + length).min(self.0.bytes.len());
            let out = self.0.bytes[from..to].to_vec();
            // The swap lands as the loop finishes reading, which is the shape
            // K9 is written about: every earlier bite crossed, and the file the
            // gate reads through is no longer the file that was sent.
            if to >= self.0.bytes.len() {
                if let Some(replacement) = self.0.change_at_the_last_read_to.take() {
                    self.0.bytes = replacement;
                }
            }
            Ok(out)
        }
        fn create_staging_exclusively(&mut self, _path: &str) -> Result<(), LocalFailure> {
            Ok(())
        }
        fn write_at(&mut self, _path: &str, _o: u64, _b: &[u8]) -> Result<(), LocalFailure> {
            Ok(())
        }
        fn make_durable(&mut self, _path: &str) -> Result<(), LocalFailure> {
            Ok(())
        }
        fn stable_identity_of(&mut self, _path: &str) -> StableFileIdentity {
            StableFileIdentity::TheSupplierCouldNotAnswer
        }
        fn publish(&mut self, _s: &str, _f: &str, _i: Intent) -> Result<(), LocalFailure> {
            Ok(())
        }
        fn remove(&mut self, _path: &str) -> Result<(), LocalFailure> {
            Ok(())
        }
        fn read_the_final_object(&mut self, _path: &str) -> WhatTheFinalObjectReadsAs {
            WhatTheFinalObjectReadsAs::default()
        }
        fn the_sources_time(&mut self, _source_path: &str) -> Option<i64> {
            None
        }
        fn keep_the_time(&mut self, _staging: &str, _seconds: i64) -> Result<(), LocalFailure> {
            Ok(())
        }
    }

    #[test]
    fn the_server_copy_takes_the_time_k7_read_at_the_arrangement() {
        // `Server protocol and transfers` S15c: the time handed to the loop
        // is the arrangement identity's, in whole seconds, even where the file
        // reads differently by the end.
        let at_the_arrangement = PathIdentity {
            size: 4,
            modification_time_nanoseconds: 1_767_225_600_250_000_000,
            file_identifier: Some("1:2".to_string()),
            read_at: SystemTime::now(),
        };
        let mut inner = AFakeSource {
            bytes: b"mesh".to_vec(),
            identity: None,
            change_at_the_last_read_to: None,
            reads: 0,
        };
        let mut identities = AFakeSource {
            bytes: Vec::new(),
            identity: Some(at_the_arrangement),
            change_at_the_last_read_to: None,
            reads: 0,
        };
        let mut adapter = AFakeAdapter(&mut inner);
        let mut watch = TheSourceUnderWatch::arranged(
            &mut adapter,
            &mut identities,
            "/home/mesh.abc".to_string(),
        )
        .expect("the arrangement identity reads");
        assert_eq!(watch.the_sources_time("/home/mesh.abc"), Some(1_767_225_600));
        assert_eq!(watch.the_sources_time("/home/another.abc"), None);
    }

    /// Drive the watch the way the byte loop drives it: bites from the front,
    /// front to back, until the file is done or a bite comes back empty.
    fn drive(source: &mut AFakeSource, bite: usize) -> (Vec<u8>, Option<WhyItStopped>) {
        let total = source.bytes.len() as u64;
        let identity = source.identity.clone();
        let mut inner_source = AFakeSource {
            bytes: source.bytes.clone(),
            identity: identity.clone(),
            change_at_the_last_read_to: source.change_at_the_last_read_to.take(),
            reads: 0,
        };
        let mut identities = AFakeSource {
            bytes: Vec::new(),
            identity,
            change_at_the_last_read_to: None,
            reads: 0,
        };
        let mut got = Vec::new();
        let why;
        {
            let mut adapter = AFakeAdapter(&mut inner_source);
            let mut watch = TheSourceUnderWatch::arranged(
                &mut adapter,
                &mut identities,
                "/home/scene.blend".to_string(),
            )
            .expect("the arrangement identity reads");
            while (got.len() as u64) < total {
                let want = ((total - got.len() as u64) as usize).min(bite);
                let bytes = watch
                    .read_at("/home/scene.blend", got.len() as u64, want)
                    .expect("the fake adapter never refuses");
                if bytes.is_empty() {
                    break;
                }
                got.extend_from_slice(&bytes);
            }
            why = watch.why_it_stopped();
        }
        source.bytes = inner_source.bytes;
        (got, why)
    }

    /// Ask K9's gate over one source, the way `Server protocol and transfers`
    /// S15b's loop asks it.
    fn ask_the_gate(
        source: &mut AFakeSource,
        total_size_expected: u64,
        what_was_sent: &Digest,
    ) -> (WhatTheGateSaid, Option<WhyItStopped>) {
        let mut adapter = AFakeAdapter(source);
        let mut gate = TheSourcesPublicationGate::over(
            &mut adapter,
            "/home/scene.blend".to_string(),
            total_size_expected,
        );
        let said = gate.may_this_be_published(what_was_sent);
        (said, gate.why_it_stopped())
    }

    fn digest_of(bytes: &[u8]) -> Digest {
        let mut running = Running::new();
        running.take(bytes);
        running.so_far()
    }

    #[test]
    fn a_quiescent_file_passes_the_watch_and_the_gate_and_every_byte_is_handed_over() {
        let mut source = AFakeSource {
            bytes: b"a quiescent file, sent whole".to_vec(),
            identity: Some(an_identity_of(28, 5_000, Some("i1"))),
            ..Default::default()
        };
        let expected = source.bytes.clone();
        let (got, why) = drive(&mut source, 7);
        assert_eq!(got, expected);
        assert_eq!(why, None);

        let (said, refused) =
            ask_the_gate(&mut source, expected.len() as u64, &digest_of(&expected));
        assert_eq!(said, WhatTheGateSaid::ItMayBePublished);
        assert_eq!(refused, None);
    }

    #[test]
    fn the_watch_stops_the_transfer_the_moment_the_path_identity_moves() {
        // The identity the watch re-reads differs from the arrangement's, which
        // is what a save landing mid-upload looks like at the path.
        let mut inner = AFakeSource {
            bytes: b"a file being sent while it is saved".to_vec(),
            identity: Some(an_identity_of(35, 5_000, Some("i1"))),
            ..Default::default()
        };
        let mut identities = AFakeSource {
            bytes: Vec::new(),
            identity: Some(an_identity_of(35, 9_999, Some("i2"))),
            ..Default::default()
        };
        let mut adapter = AFakeAdapter(&mut inner);
        // The arrangement identity is taken from the same reader, so it is
        // moved once before the first bite to make the two disagree.
        let arrangement = an_identity_of(35, 5_000, Some("i1"));
        let mut watch = TheSourceUnderWatch {
            inner: &mut adapter,
            identities: &mut identities,
            source_path: "/home/scene.blend".to_string(),
            arrangement,
            why_it_stopped: None,
        };
        let bytes = watch
            .read_at("/home/scene.blend", 0, 8)
            .expect("no local failure");
        assert!(
            bytes.is_empty(),
            "the watch hands nothing over once it has seen the change"
        );
        assert_eq!(
            watch.why_it_stopped(),
            Some(WhyItStopped::TheWatchSawTheSourceChange)
        );
    }

    #[test]
    fn the_gate_refuses_where_the_source_changed_under_a_stat_that_did_not_move() {
        // Fact 72's shape and Windows' documented one: the contents move and
        // the path identity does not. **K9 is the only thing that catches it.**
        //
        // **And every byte still crossed the watch.** That is the half this
        // test exists for since the gate became S15b's: the refusal is the
        // gate's answer and not a read that came back empty.
        let mut source = AFakeSource {
            bytes: b"the version that was sent".to_vec(),
            identity: Some(an_identity_of(25, 5_000, Some("i1"))),
            change_at_the_last_read_to: Some(b"a different version now.".to_vec()),
            ..Default::default()
        };
        let what_was_sent = source.bytes.clone();
        let (got, why) = drive(&mut source, 25);
        assert_eq!(got, what_was_sent, "the whole file crossed");
        assert_eq!(why, None, "the watch saw nothing: the stat did not move");

        let (said, refused) = ask_the_gate(
            &mut source,
            what_was_sent.len() as u64,
            &digest_of(&what_was_sent),
        );
        assert_eq!(said, WhatTheGateSaid::TheSourceChangedUnderIt);
        assert_eq!(refused, Some(WhyItStopped::ThePublicationGateRefused));
    }

    #[test]
    fn a_source_that_became_shorter_is_a_change_and_never_a_short_digest() {
        // The read-through runs out before the transfer's own expected size.
        // **That is a change, answered as one**, rather than a digest taken over
        // whatever is left.
        let mut source = AFakeSource {
            bytes: b"shorter now".to_vec(),
            identity: Some(an_identity_of(11, 5_000, Some("i1"))),
            ..Default::default()
        };
        let (said, refused) =
            ask_the_gate(&mut source, 40, &digest_of(b"the whole of what was sent"));
        assert_eq!(said, WhatTheGateSaid::TheSourceChangedUnderIt);
        assert_eq!(refused, Some(WhyItStopped::ThePublicationGateRefused));
    }

    /// **The other half of the same asymmetry: a source that grew.**
    ///
    /// Its first bytes still digest to what was sent, so the two digests agree
    /// and a gate reading only to the expected size says publish. K9's argument
    /// does not cover that case — the clause is *the source is read through once
    /// more*, and a reading stopped at the expected size is not a full reading
    /// of the file — and Row 201's first clause is that a file changed while it
    /// was going up is stopped, not landed.
    #[test]
    fn a_source_that_grew_is_a_change_even_though_its_first_bytes_still_agree() {
        let what_was_sent = b"the whole of what was sent".to_vec();
        let mut source = AFakeSource {
            bytes: [what_was_sent.clone(), b" and then some more".to_vec()].concat(),
            identity: Some(an_identity_of(45, 5_000, Some("i1"))),
            ..Default::default()
        };
        let (said, refused) = ask_the_gate(
            &mut source,
            what_was_sent.len() as u64,
            &digest_of(&what_was_sent),
        );
        assert_eq!(said, WhatTheGateSaid::TheSourceChangedUnderIt);
        assert_eq!(refused, Some(WhyItStopped::ThePublicationGateRefused));
    }

    /// And a quiescent source of exactly the expected size still publishes —
    /// the read past the end finds nothing there.
    #[test]
    fn a_quiescent_source_of_exactly_the_expected_size_still_publishes() {
        let sent = b"exactly what was sent, to the byte".to_vec();
        let mut source = AFakeSource {
            bytes: sent.clone(),
            identity: Some(an_identity_of(sent.len() as u64, 5_000, Some("i1"))),
            ..Default::default()
        };
        let (said, refused) = ask_the_gate(&mut source, sent.len() as u64, &digest_of(&sent));
        assert_eq!(said, WhatTheGateSaid::ItMayBePublished);
        assert_eq!(refused, None);
    }

    #[test]
    fn a_read_of_another_path_passes_straight_through_and_is_not_watched() {
        let mut inner = AFakeSource {
            bytes: b"the source".to_vec(),
            identity: Some(an_identity_of(10, 5_000, Some("i1"))),
            ..Default::default()
        };
        let mut identities = AFakeSource {
            bytes: Vec::new(),
            identity: Some(an_identity_of(10, 5_000, Some("i1"))),
            ..Default::default()
        };
        let mut adapter = AFakeAdapter(&mut inner);
        let mut watch = TheSourceUnderWatch::arranged(
            &mut adapter,
            &mut identities,
            "/home/scene.blend".to_string(),
        )
        .expect("arranged");
        let bytes = watch
            .read_at("/jobs/.smoozy-staging/abc-1.smoozy-part", 0, 4)
            .expect("no local failure");
        assert_eq!(bytes, b"the ".to_vec());
        assert_eq!(watch.why_it_stopped(), None);
    }

    // ---- The declared route, driven by the real byte loop.

    /// **A far end that takes one upload, over a pipe, at the desk.**
    ///
    /// It answers exactly the packets an upload's byte loop sends — the opening
    /// `INIT`, a `STAT` of the staging path, `MKDIR`, `REMOVE`, an `OPEN` to
    /// create, byte-offset `WRITE`s and `CLOSE` — and refuses everything else
    /// rather than inventing an answer. **It announces no extension**, so the
    /// negotiated limits are the draft's own recommended pair and no capability
    /// is guessed at. **What is faked is the wire; the loop under test is the
    /// real one.**
    fn a_far_end_taking_an_upload(landed: Arc<Mutex<Vec<u8>>>) -> tokio::io::DuplexStream {
        let (ours, mut theirs) = tokio::io::duplex(128 * 1024);
        tokio::spawn(async move {
            let handle: &[u8] = b"h1";
            loop {
                let mut length = [0u8; 4];
                if theirs.read_exact(&mut length).await.is_err() {
                    return;
                }
                let mut whole = vec![0u8; u32::from_be_bytes(length) as usize];
                if theirs.read_exact(&mut whole).await.is_err() {
                    return;
                }
                let (packet_kind, payload) = (whole[0], &whole[1..]);
                let mut fields = Fields::over(payload);
                let reply = match packet_kind {
                    kind::INIT => {
                        let mut composed = Composed::new();
                        composed.u32(3);
                        composed.framed(kind::VERSION)
                    }
                    // Nothing is staged there yet — S12 step 1's own answer.
                    kind::STAT | kind::LSTAT => a_status(fields.u32().unwrap_or(0), 2),
                    kind::MKDIR | kind::REMOVE | kind::CLOSE => {
                        a_status(fields.u32().unwrap_or(0), 0)
                    }
                    kind::OPEN => {
                        let id = fields.u32().unwrap_or(0);
                        let mut composed = Composed::new();
                        composed.u32(id).string(handle);
                        composed.framed(kind::HANDLE)
                    }
                    kind::WRITE => {
                        let id = fields.u32().unwrap_or(0);
                        let _handle = fields.string();
                        let offset = fields.u64().unwrap_or(0) as usize;
                        let data = fields.string().unwrap_or_default();
                        let mut held = landed.lock().expect("the far end's own store");
                        if held.len() < offset + data.len() {
                            held.resize(offset + data.len(), 0);
                        }
                        held[offset..offset + data.len()].copy_from_slice(data);
                        drop(held);
                        a_status(id, 0)
                    }
                    _ => a_status(fields.u32().unwrap_or(0), 4),
                };
                if theirs.write_all(&reply).await.is_err() {
                    return;
                }
                if theirs.flush().await.is_err() {
                    return;
                }
            }
        });
        ours
    }

    fn a_status(id: u32, code: u32) -> Vec<u8> {
        let mut composed = Composed::new();
        composed.u32(id).u32(code).string(b"").string(b"");
        composed.framed(kind::STATUS)
    }

    struct WritesItDown;
    impl KeepsTheRecord for WritesItDown {
        fn write_it_down(&mut self, _record: &TransferRecord) -> Result<(), LocalFailure> {
            Ok(())
        }
    }

    /// **The test that would have caught the fault, and it is the whole of item
    /// 1.**
    ///
    /// K9's refusal used to travel on a read that came back empty — a reading
    /// the loop treats as *the source ended short*. It produced the right record
    /// and the right ending, so nothing was broken and nothing could be asserted
    /// about the route. **This asserts the route:** every byte crosses, the
    /// watch sees nothing, `Server protocol and transfers` S15b's gate is what
    /// refuses, and the transfer stops with S26's second row and no pre-state.
    #[tokio::test]
    async fn the_refusal_travels_the_declared_gate_and_not_a_read_that_came_back_empty() {
        let what_was_sent = vec![7u8; 40_000];
        let what_the_file_became = vec![8u8; 40_000];
        let identity = an_identity_of(40_000, 5_000, Some("i1"));

        // The watch's own handle to the user's computer, serving what was sent.
        let mut passing = AFakeSource {
            bytes: what_was_sent.clone(),
            identity: Some(identity.clone()),
            ..Default::default()
        };
        // **The gate's own handle, which is the same disk after the save.** The
        // path identity is unmoved, so K8 has nothing to see and K9 is the only
        // thing that can catch it — Fact 72's shape.
        let mut read_through = AFakeSource {
            bytes: what_the_file_became,
            identity: Some(identity.clone()),
            ..Default::default()
        };
        let mut identities = AFakeSource {
            bytes: Vec::new(),
            identity: Some(identity),
            ..Default::default()
        };

        let landed = Arc::new(Mutex::new(Vec::new()));
        let mut subsystem = Subsystem::negotiate(a_far_end_taking_an_upload(landed.clone()))
            .await
            .expect("the far end announces revision 3");

        let issue = |kind| Identity::issue(kind).expect("random bytes");
        let mut record = TransferRecord::arranged(
            issue(IdentityKind::Transfer),
            Direction::Up,
            issue(IdentityKind::LocalServer),
            Some(issue(IdentityKind::Installation)),
            "/home/scene.blend".to_string(),
            "/jobs/scene.blend".to_string(),
            "/jobs/.smoozy-staging",
            what_was_sent.len() as u64,
            issue(IdentityKind::Run),
        );
        record.expected_digest = Some(digest_of(&what_was_sent));

        let mut budget = NoCapIsInForce;
        let mut keeper = WritesItDown;
        let mut watcher = NobodyIsWatching;
        let mut passing_adapter = AFakeAdapter(&mut passing);
        let mut read_through_adapter = AFakeAdapter(&mut read_through);
        let mut gate = TheSourcesPublicationGate::over(
            &mut read_through_adapter,
            "/home/scene.blend".to_string(),
            what_was_sent.len() as u64,
        );
        let mut watch = TheSourceUnderWatch::arranged(
            &mut passing_adapter,
            &mut identities,
            "/home/scene.blend".to_string(),
        )
        .expect("the arrangement identity reads");

        let happened = carry_out_under_a_publication_gate(
            Transfer {
                subsystem: &mut subsystem,
                local: &mut watch,
                budget: &mut budget,
                keeper: &mut keeper,
                watcher: &mut watcher,
                inactivity_deadline: Duration::from_secs(5),
                intent: Intent::CreateIntended,
            },
            &mut record,
            &mut gate,
        )
        .await;

        // **S26's second row**, which is the ending K8 and K9 both require.
        assert_eq!(happened.ending, the_ending_a_changed_source_writes());
        // **The gate refused and the watch did not**, which is the whole claim.
        assert_eq!(
            gate.why_it_stopped(),
            Some(WhyItStopped::ThePublicationGateRefused)
        );
        assert_eq!(watch.why_it_stopped(), None);
        // **Every byte crossed**, so no read came back empty and the old route
        // was not the one taken.
        assert_eq!(
            landed.lock().expect("the far end's own store").len(),
            what_was_sent.len()
        );
        // **The pre-state was never written**, which is what says the gate's
        // moment is before it: a durable *publication pending* record past a
        // refusal would be retried past it.
        assert_eq!(record.publication_pending, None);
    }
}

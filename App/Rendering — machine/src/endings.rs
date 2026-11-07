//! The ending classification — §3.6 K14, K14a, K15, K15a, K15b, K16 — and
//! the value it supplies into `Queue & History`'s `Ending`.
//!
//! **K14: the render kind's ending classification, bounded by `Queue &
//! History` E2's two fixed rules — an ending with no status is failed, and a
//! successful exit status is never read as *everything was written*.**
//! *Finished* is exit status 0 **and the marker observed passing the
//! worker's own loop** — a statement about the render's ending and never
//! about the outputs. *Failed* is a non-zero status; or the group gone and
//! no status reaped; or a launch made and the marker never observed where
//! the ordinary stream's collection outcome is *never established* — the
//! run never reached the launch material, so what it rendered was not the
//! job (D17's observed case: an unreadable script, the factory scene at
//! exit 0); or the attempt failed before the render instruction. *Held for
//! an answer* ends the attempt and not the job. *Left stopped* and *given
//! up* are not the kind's own endings.
//!
//! **K14b: a render that ends at 0 having written none of what the job owed
//! is *failed*, in every version.** The count decides, never the printed
//! words ([`the_outcome_against_the_count`]).
//!
//! **K14a: every *failed* ending carries one cause-side value beside the
//! reason — *the machine's own*, *the file's own*, *undetermined*** — three
//! values, `Telegram` Y4b's own three, written at the ending on the machine
//! from what the machine already holds, **never composed from the exit
//! number and never mined from a stream** (K6). What separates the first
//! two is repeatability: the file's own is what an identical re-send
//! repeats by construction; the machine's own only where a record the
//! machine already holds names an event outside the render; **undetermined
//! everywhere else, and it is the default** — a two-value field would put
//! the phone's tap on a failure whose cause is unknown, which is the second
//! identical failure and the second bill the ruling refuses.
//!
//! **This crate supplies the value; `Queue & History` owns the `Ending`
//! type and the mapping** ([`as_the_render_kinds_own_ending`]). Nothing in
//! that crate names this one, and the dependency runs one way.
//!
//! **Markers used in this file that belong to another document, each with
//! the document it belongs to.** `Queue & History` E1, E2, E3, E5, N3,
//! D4, Q4. `Rented-server runtime` M13, M14, M20, M21, M22, M23, M28, U6,
//! U7, U8, X13. `Job files and outputs` C9, C27, D17. `Render Settings`
//! Y13, Y14. `Telegram` E1, Y4a, Y4b. `State, identity and recovery` §6.
//! `Security and licence` §13.

use smoozy_job_files_machine::reckoning::TheCompletenessStatement;
use smoozy_queue_history_machine::ending::{CollectionOutcome, TheRenderKindsOwnEnding};

/// **The occasions of K14's failed limb that end the attempt before the
/// render instruction**, each with the reason recorded verbatim by its
/// clause. **A hold is not one of them**: it is its own outcome.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BeforeTheRenderInstruction {
    /// L30: the remap material the manifest names could not be read.
    TheRemapMaterialCouldNotBeRead { material: String },
    /// L14: the version confirmation refused.
    TheVersionConfirmationRefused { reason: String },
    /// L10, `Job files and outputs` D10: the read-back did not match the
    /// map's pair.
    TheReadBackMismatched { read: Vec<u8>, expected: Vec<u8> },
    /// `Job files and outputs` C9: a reading that did not end successfully.
    ///
    /// **C9's reading is of the job's output folder**, before the reckoning
    /// (P1 step 6), **and this occasion is that one alone.** The step-4 reading
    /// of the job's `.blend` has its own, [`Self::TheJobsFileReadingDidNotEndSuccessfully`],
    /// since 2026-09-29.
    AReadingThatDidNotEndSuccessfully,
    /// **P1 step 4: the reading of the job's file did not end successfully** —
    /// K14's failed limb, *"the step-4 reading of the job's file did not end
    /// successfully"*. Added 2026-09-29, step 10c's transfer wave piece 12.
    ///
    /// **Until then this reading's failure borrowed C9's occasion**, whose
    /// words name the output folder, so a job whose `.blend` could not be read
    /// was told to its user as a folder that could not be read — step 10c's
    /// eighth pass on Hyperstack, fault 5. **K14 had no entry for it and the
    /// crate did not widen a closed set of the plan's on its own**; the plan
    /// moved first, on 2026-09-28, and this is its occasion.
    ///
    /// **It carries nothing**, and on purpose: telling a truncated file from
    /// any other failure would mean mining Blender's words, which K14a and
    /// `Security and licence` §13 rule 5 forbid. The words say *which* reading
    /// failed, and nothing about why.
    TheJobsFileReadingDidNotEndSuccessfully,
    /// L11, L8a: a pair whose assignment raised.
    APairWhoseAssignmentRaised { pair: String },
    /// L10a: a settings pair whose own name the object it was assigned on
    /// does not carry.
    ///
    /// **Nothing constructs this variant, and that is recorded rather than
    /// left to look otherwise.** L10a's failure is a line the material
    /// prints into the marker stream — `unapplied`, at
    /// `crate::acting::material` — and **nothing in this crate reads the
    /// marker stream for it**, so the variant has no producer. It is a
    /// finding of Round 1's quotation check, named there rather than made a
    /// must-do, and it is written here so a later reader does not take the
    /// variant for a reachable one.
    ///
    /// **Its row in K14a's table is *the file's own* and is correct as it
    /// stands.** Round 1 was asked to move it and refused: nothing
    /// constructs the variant, the marker line it would be built from is
    /// never printed because the material's walk raises one step before the
    /// read that prints it, and the value that failure actually reaches is
    /// [`FailedReason::ANonZeroStatus`] at 63, whose cause is *undetermined*.
    /// **Do not repair it.**
    ASettingsPairWhoseNameTheObjectDoesNotCarry { property: String },
    /// P1a, `Render Settings` Y13–Y14: a name-shaped payload entry the
    /// reading in force does not carry.
    ANameTheReadingInForceDoesNotCarry { name: String },
    /// L23: a card make with no technology to name.
    ACardMakeWithNoTechnologyToName { make: String },
    /// **L24b: the job names `OPTIX` and the system's loader does not find
    /// the OptiX library on this machine.** Added 2026-10-03, with the
    /// Creator's ruling at L23 that NVIDIA cards render with OptiX.
    ///
    /// **It carries nothing**: the library's name is the one NVIDIA's own
    /// components list gives (L23), and what is missing is the same on every
    /// machine it is missing from. **Its K14a value is *undetermined***: an
    /// identical job sent to the same machine meets it again, and it says
    /// nothing about the artist's file.
    TheOptixLibraryWasNotFound,

    // -----------------------------------------------------------------
    // **Four occasions K14's failed limb gained on 2026-09-05**, out of
    // Round 1 of this crate's code audit. The driver reached all four and
    // the plan's closed set named none of them; the crate was right to
    // refuse to widen a closed set of the plan's on its own authority, and
    // the plan moved first. **All four are *undetermined* in K14a.**
    // -----------------------------------------------------------------
    /// **L12: the launch material's placement was not confirmed by the
    /// read-back under the render's login** — written and unreadable, or
    /// read back as something else. **Nothing is written again**, because a
    /// second write is a second thing that could have gone wrong.
    TheMaterialsPlacementWasNotConfirmed { what_happened: String },
    /// **The attempt could not be composed at all** — a spelling that has
    /// not been read (`Render Settings` §13 item 29's five landed on
    /// 2026-09-20 and nothing is owed today), a reckoning list with no frame
    /// to name, or a sequence L8's own order refuses.
    TheAttemptCouldNotBeComposed { what_happened: String },
    /// **The launch itself could not be made** (L6, L7) — the build's path
    /// could not be run, or its two streams could not be held.
    TheLaunchCouldNotBeMade { why: String },
    /// **The attempt record could not be written durably before the render
    /// started** (`Job files and outputs` M8; P1 step 7). No render is
    /// started without it: a render whose frame list no later attempt can
    /// read falls to `Job files and outputs` C20's whole range and is
    /// re-rendered and re-paid for entire.
    TheAttemptRecordCouldNotBeWritten { why: String },
    /// **`Dependency Upload & Remap` M22c: an uploaded bake could not be copied
    /// into the job folder before the render** — which copy, and why. Added
    /// 2026-09-29, step 10c's dependency wave piece 8, with K14's row moved
    /// first: **without it a bake the check sent is never read by the render,
    /// and nothing says so**, so the attempt ends before the render and names
    /// the copy. **Undetermined** in K14a, like the four above it: it says
    /// nothing about the artist's file, and a full disk repeats.
    AnUploadedBakeCouldNotBeCopied { copy: String, why: String },
}

impl BeforeTheRenderInstruction {
    /// **What ended the attempt, in standard software words** (Rule 9) — for
    /// the day a screen reads the attempt record's reason.
    ///
    /// **Added 2026-09-23, out of step 10c's fifth pass on Vast, fault 5.** The
    /// attempt record's reason was written with this type's debugging
    /// rendering, and on a real rented machine it read
    /// `BeforeTheRenderInstruction(ACardMakeWithNoTechnologyToName { make:
    /// "NoReadingStoodAtSubmission" })`. **Nothing on a desktop reads it
    /// today**, so nothing was on a screen — and that is exactly why the words
    /// go beside the type now rather than the day something does, which is the
    /// day five renderings land at once.
    ///
    /// **The values each occasion carries stay in the words** where they are
    /// the machine's own answer or a name the file uses, because that is what a
    /// person needs to act on; **nothing is parsed back out of them** and a
    /// caller that must act on one reads its own field.
    pub fn in_words(&self) -> String {
        match self {
            BeforeTheRenderInstruction::TheRemapMaterialCouldNotBeRead { material } => format!(
                "could not read the job's dependency remap ({material})"
            ),
            BeforeTheRenderInstruction::TheVersionConfirmationRefused { reason } => {
                format!("Blender did not confirm its version: {reason}")
            }
            BeforeTheRenderInstruction::TheReadBackMismatched { read, expected } => format!(
                "a setting read back wrong (read {}, expected {})",
                String::from_utf8_lossy(read),
                String::from_utf8_lossy(expected)
            ),
            BeforeTheRenderInstruction::AReadingThatDidNotEndSuccessfully => {
                "reading the job's output folder did not finish".to_string()
            }
            BeforeTheRenderInstruction::TheJobsFileReadingDidNotEndSuccessfully => {
                "reading the job's .blend file did not finish".to_string()
            }
            BeforeTheRenderInstruction::APairWhoseAssignmentRaised { pair } => {
                format!("Blender rejected a setting ({pair})")
            }
            BeforeTheRenderInstruction::ASettingsPairWhoseNameTheObjectDoesNotCarry {
                property,
            } => format!(
                "setting not found on its object ({property})"
            ),
            BeforeTheRenderInstruction::ANameTheReadingInForceDoesNotCarry { name } => {
                format!("the file has nothing named {name}")
            }
            BeforeTheRenderInstruction::ACardMakeWithNoTechnologyToName { make } => format!(
                "could not choose a GPU backend ({make})"
            ),
            // **L24b's own words**, opened with a small letter and left
            // without a full stop like every other occasion here: the
            // desktop opens and closes another party's words itself.
            BeforeTheRenderInstruction::TheOptixLibraryWasNotFound => format!(
                "the NVIDIA driver has no OptiX library ({})",
                smoozy_server_setup_machine::the_optix_library::THE_LIBRARY
            ),
            BeforeTheRenderInstruction::TheMaterialsPlacementWasNotConfirmed {
                what_happened,
            } => format!(
                "the launch script could not be placed: {what_happened}"
            ),
            BeforeTheRenderInstruction::TheAttemptCouldNotBeComposed { what_happened } => {
                format!("could not prepare the render: {what_happened}")
            }
            BeforeTheRenderInstruction::TheLaunchCouldNotBeMade { why } => {
                format!("could not start Blender: {why}")
            }
            BeforeTheRenderInstruction::TheAttemptRecordCouldNotBeWritten { why } => {
                format!("could not save the attempt record: {why}")
            }
            // **Shortened 2026-11-04** (`Design/Queue & History`): it read *the files
            // uploaded for the geometry nodes bake folder (/ephemeral/…) could not be
            // copied into the job's folder: this job's folder has no place for
            // //caches/…*, and a user now reads it at once. **Where the copy found no
            // place there is nothing further a user can use**, so nothing follows.
            BeforeTheRenderInstruction::AnUploadedBakeCouldNotBeCopied { copy, why } if why.is_empty() => {
                format!("could not copy {copy}")
            }
            BeforeTheRenderInstruction::AnUploadedBakeCouldNotBeCopied { copy, why } => {
                format!("could not copy {copy}: {why}")
            }
        }
    }
}

/// What the worker reaped and observed at the ending step — the inputs to
/// the classification, and nothing Blender printed is among them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatTheWorkerReaped {
    /// The exit status, verbatim; `None` where the group is gone and no
    /// status was reaped (M21).
    pub exit_status: Option<i32>,
    /// **The marker observed passing the worker's own loop** (M28) — the
    /// worker's own recorded act, and reading it is not reading a stream.
    pub the_marker_was_observed: bool,
    /// The ordinary stream's collection outcome (M13's closed set of four).
    pub the_ordinary_streams_collection: CollectionOutcome,
}

/// **K14a's three values.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cause {
    TheMachinesOwn,
    TheFilesOwn,
    Undetermined,
}

/// The reason a failed ending records — verbatim, and no number names a
/// cause.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FailedReason {
    BeforeTheRenderInstruction(BeforeTheRenderInstruction),
    /// The worker reaped a non-zero status; the number is recorded and
    /// names nothing (K6).
    ANonZeroStatus {
        exit_status: i32,
    },
    /// The group is gone and no status was reaped — an ending with no
    /// status (M21).
    NoStatusWasReaped,
    /// A launch was made and the marker was never observed, the ordinary
    /// stream's collection outcome being *never established* (M13).
    TheMarkerWasNeverObserved {
        exit_status: i32,
    },
    /// **L5a: the render was started and its recorded pair could not be
    /// written.** **The one ending on K14's failed limb that is after the
    /// launch**, and the only reason it is not on
    /// [`BeforeTheRenderInstruction`]'s list.
    ///
    /// **The render is ended at that moment, by the kill**, and L5a rules
    /// that rather than the code inventing it: no stop can ever reach such a
    /// render, because K4's termination is aimed at the recorded pair and
    /// there is no recorded pair. Left running it is a Blender on a machine
    /// the user pays for by the second that nothing can end. **What that
    /// costs is stated at L5a and is not bounded here**: a kill leaves a
    /// short file at the frame's own final name, and a short-but-not-empty
    /// file still counts as already there.
    TheRecordedPairCouldNotBeWritten {
        why: String,
    },
    /// **K14b: the render ended at 0 and the completeness statement found none
    /// of what the job owed written.** *Reports 1*: 5.3 ends a scene with no
    /// camera this way; *Reports 12* has others. **It carries nothing**: the
    /// status is 0 by construction, and telling one such ending from another
    /// would mean mining Blender's words (K14a).
    NothingTheJobOwedWasWritten,
}

impl FailedReason {
    /// **Why the attempt failed, in standard software words** (Rule 9) —
    /// beside the type, for the day a screen reads this.
    ///
    /// **The exit number is carried and names nothing** (K6): it is the
    /// machine's own figure and no cause is read off it here or anywhere else.
    pub fn in_words(&self) -> String {
        match self {
            FailedReason::BeforeTheRenderInstruction(occasion) => occasion.in_words(),
            FailedReason::ANonZeroStatus { exit_status } => {
                format!("Blender exited with code {exit_status}")
            }
            FailedReason::NoStatusWasReaped => {
                "Blender exited without an exit code".to_string()
            }
            // **Made only at an exit of 0, so it says *normally*, as K14b's
            // reason does, and never *code 0* beside *Failed*.** *Changed
            // 2026-11-06, the check after the last fix: it read *Blender
            // exited with code 0 and never reported starting to render*.* A
            // non-zero figure, which only a test makes, still says its code.
            FailedReason::TheMarkerWasNeverObserved { exit_status: 0 } => {
                "Blender exited normally but never reported starting to render".to_string()
            }
            FailedReason::TheMarkerWasNeverObserved { exit_status } => format!(
                "Blender exited with code {exit_status} and never reported starting to render"
            ),
            FailedReason::TheRecordedPairCouldNotBeWritten { why } => format!(
                "the render started but could not be recorded: {why}"
            ),
            FailedReason::NothingTheJobOwedWasWritten => {
                "Blender exited normally but wrote none of the job's files".to_string()
            }
        }
    }
}

/// **K14's recorded outcomes, the kind's own.** *Left stopped* and *given
/// up* are written by the stop path and by reconciliation and are not here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecordedOutcome {
    /// Exit 0 and the marker observed, and not K14b's nothing written. **Never
    /// a statement about the outputs**: the completeness statement stands
    /// beside it.
    Finished,
    Failed {
        reason: FailedReason,
        cause: Cause,
    },
    /// The attempt evaluated the hold conditions and placed a hold. It
    /// ends the attempt and not the job, carries no completeness reading,
    /// and the release starts a fresh attempt like any other.
    HeldForAnAnswer,
}

impl RecordedOutcome {
    /// **Whether the attempt ended before any render began**: K14's *before the
    /// render instruction*, whether Blender was never launched or was launched and
    /// stopped in the launch material. **The truth `Design/Queue & History`'s *Failed
    /// before rendering.* rests on**, and it is the machine's own act rather than
    /// anything read off a stream. *The marker was never observed* is not here, because
    /// a marker not seen does not say none was printed.
    pub fn no_render_began(&self) -> bool {
        matches!(
            self,
            RecordedOutcome::Failed { reason: FailedReason::BeforeTheRenderInstruction(_), .. }
        )
    }
}

/// **K14 and K14a, performed.** The two inputs are what the attempt
/// established before the render instruction and what the worker reaped;
/// where the first is present the second is not consulted, because no
/// launch was made.
pub fn classify(
    before: Option<BeforeTheRenderInstruction>,
    a_hold_was_placed: bool,
    reaped: Option<&WhatTheWorkerReaped>,
) -> RecordedOutcome {
    if a_hold_was_placed {
        return RecordedOutcome::HeldForAnAnswer;
    }
    if let Some(occasion) = before {
        let cause = k14as_value_for(&occasion);
        return RecordedOutcome::Failed {
            reason: FailedReason::BeforeTheRenderInstruction(occasion),
            cause,
        };
    }
    let Some(reaped) = reaped else {
        // No launch was made and nothing was established before the render
        // instruction either: an ending with no status is failed (E2).
        return RecordedOutcome::Failed {
            reason: FailedReason::NoStatusWasReaped,
            cause: Cause::TheMachinesOwn,
        };
    };
    match reaped.exit_status {
        None => RecordedOutcome::Failed {
            reason: FailedReason::NoStatusWasReaped,
            cause: Cause::TheMachinesOwn,
        },
        Some(0) => {
            if reaped.the_marker_was_observed {
                return RecordedOutcome::Finished;
            }
            match reaped.the_ordinary_streams_collection {
                // A capture that stopped cannot support the inference: the
                // ending is classified on the exit status alone and the
                // collection outcome stands beside it.
                CollectionOutcome::CollectionIncomplete => RecordedOutcome::Finished,
                // A launch was made and the marker was never observed. M13
                // names an ordinary stream with no marker at all *never
                // established*; a collected stream with no marker is that
                // same fact and takes the same limb.
                CollectionOutcome::NeverEstablished
                | CollectionOutcome::Collected
                | CollectionOutcome::CollectedAndTheStreamWasEmpty => RecordedOutcome::Failed {
                    reason: FailedReason::TheMarkerWasNeverObserved { exit_status: 0 },
                    cause: Cause::Undetermined,
                },
            }
        }
        Some(exit_status) => RecordedOutcome::Failed {
            reason: FailedReason::ANonZeroStatus { exit_status },
            cause: Cause::Undetermined,
        },
    }
}

/// **K14b, performed: the ending against the count.** [`classify`] reads what
/// the worker reaped; this reads the completeness statement the same ending
/// took (`Job files and outputs` C27; P1 step 9), so the walk comes before the
/// outcome is final.
///
/// **It moves *finished* and nothing else**, and only where the statement stands
/// and says the job wrote none of what it owed. **No statement is no count**
/// (`Rented-server runtime` S13), so the exit status has classified alone. A
/// partly written job stays *finished*, its count short beside it (`Queue &
/// History` E3), and a job that owed nothing is not a failure.
///
/// **K14a's value is *undetermined***: the count cannot tell a scene with no
/// camera from a single still on a full disk, so no cause is established and
/// `Telegram` Y4a's button is withheld. **Nothing here looks for a camera**: that
/// would be a safety net past Rule 8's line.
pub fn the_outcome_against_the_count(
    outcome: RecordedOutcome,
    statement: Option<&TheCompletenessStatement>,
) -> RecordedOutcome {
    match (&outcome, statement) {
        (RecordedOutcome::Finished, Some(statement)) if statement.none_of_what_was_owed_was_written() => {
            RecordedOutcome::Failed {
                reason: FailedReason::NothingTheJobOwedWasWritten,
                cause: Cause::Undetermined,
            }
        }
        _ => outcome,
    }
}

/// **K14a's table, over K14's failed limb and nothing wider**: the
/// pre-render-instruction occasions are *the file's own* — except C9's
/// reading that did not end, which establishes that nothing was learned
/// and is *undetermined*, except the four the table gained on
/// 2026-09-05, which are *undetermined* too, and except P1 step 4's reading
/// of the job's file, which K14a's row of 2026-09-28 makes *undetermined*:
/// telling a truncated file from anything else would mean mining Blender's
/// words (`Security and licence` §13 rule 5) — and except the copy of an
/// uploaded bake that could not be made (`Dependency Upload & Remap` M22c),
/// which K14a's row of 2026-09-29 makes *undetermined* for the four's own
/// reason below.
///
/// **Why those four are *undetermined*, which K14a states and this only
/// performs.** The three values exist so the user is not offered *try it
/// again* on a failure that will repeat, and **every one of the four
/// repeats by construction**: a launch that could not be made, a record
/// that could not be written, a placement that could not be confirmed, a
/// composition L8's order refused. **None is *the file's own***, because
/// none says anything about the artist's file — the manifest is immutable
/// and the material composes the same way at every attempt. **And none is
/// *the machine's own***, which is the value `Telegram` Y4a attaches the
/// retry button to: a tap on any of them buys the user a second bill for
/// the identical refusal.
fn k14as_value_for(occasion: &BeforeTheRenderInstruction) -> Cause {
    match occasion {
        BeforeTheRenderInstruction::AReadingThatDidNotEndSuccessfully
        | BeforeTheRenderInstruction::TheJobsFileReadingDidNotEndSuccessfully
        | BeforeTheRenderInstruction::TheMaterialsPlacementWasNotConfirmed { .. }
        | BeforeTheRenderInstruction::TheAttemptCouldNotBeComposed { .. }
        | BeforeTheRenderInstruction::TheLaunchCouldNotBeMade { .. }
        | BeforeTheRenderInstruction::TheAttemptRecordCouldNotBeWritten { .. }
        | BeforeTheRenderInstruction::AnUploadedBakeCouldNotBeCopied { .. }
        // **L24b, K14a's row of 2026-10-03**: the same job on the same
        // machine repeats it exactly, and the file has nothing to do with it.
        | BeforeTheRenderInstruction::TheOptixLibraryWasNotFound => {
            Cause::Undetermined
        }
        BeforeTheRenderInstruction::TheRemapMaterialCouldNotBeRead { .. }
        | BeforeTheRenderInstruction::TheVersionConfirmationRefused { .. }
        | BeforeTheRenderInstruction::TheReadBackMismatched { .. }
        | BeforeTheRenderInstruction::APairWhoseAssignmentRaised { .. }
        | BeforeTheRenderInstruction::ASettingsPairWhoseNameTheObjectDoesNotCarry { .. }
        | BeforeTheRenderInstruction::ANameTheReadingInForceDoesNotCarry { .. }
        | BeforeTheRenderInstruction::ACardMakeWithNoTechnologyToName { .. } => Cause::TheFilesOwn,
    }
}

/// **K14 and K14a over the driver's own endings: the one route from where
/// an attempt ended into the classification.**
///
/// **Nothing converted a [`crate::acting::pipeline::WhereTheAttemptEnded`]
/// into a [`RecordedOutcome`] until 2026-09-05**, and [`classify`] was
/// called from nowhere but its own tests. What the shapes allowed a builder
/// to reach for was `classify(None, false, None)`, which is *failed* with
/// the cause **the machine's own** — and `Telegram` Y4a attaches *try it
/// again* to that value alone. **So a job whose launch material could not be
/// placed, or whose composition refused because a `Render Settings` spelling
/// has not landed, would have been recorded as the machine's own**: the user
/// taps, the identical job goes again, the material cannot be placed again,
/// and they have paid twice for the same refusal.
///
/// **The six endings have one home here rather than six inventions at the
/// call sites.** Five of them are K14's own since 2026-09-05; the sixth —
/// the walk that did not complete — is `Job files and outputs` C9's
/// occasion, which K14's list already carried and which the repaired clause
/// says in as many words is not a sixth.
///
/// `None` is not an omission and is two cases, each stated:
///
/// - **the render was launched**, so the attempt is not over — steps 8 and 9
///   reap it and observe the marker, and [`classify`] takes their answer;
/// - **no hold was placed and reconciliation takes it** (P3), whose ending is
///   *given up*, written by reconciliation under `State, identity and
///   recovery` §6 and **not one of the kind's own** (K14).
#[cfg(unix)]
pub fn the_outcome_of(
    ended: &crate::acting::pipeline::WhereTheAttemptEnded,
) -> Option<RecordedOutcome> {
    use crate::acting::pipeline::WhereTheAttemptEnded as Ended;
    let occasion = match ended {
        Ended::TheRenderWasLaunched(_) | Ended::NoHoldWasPlacedAndReconciliationTakesIt => {
            return None
        }
        Ended::AHoldWasPlaced { .. } => return Some(classify(None, true, None)),
        // **L5a, and it is the one ending here that is after the launch**, so
        // it is not an occasion of the before-the-render-instruction limb and
        // is built with its own reason. **Its value is K14a's *undetermined*
        // for the same reason the other four take it**: it repeats by
        // construction and says nothing about the artist's file.
        Ended::TheRecordedPairCouldNotBeWritten { why, .. } => {
            return Some(RecordedOutcome::Failed {
                reason: FailedReason::TheRecordedPairCouldNotBeWritten { why: why.clone() },
                cause: Cause::Undetermined,
            })
        }
        Ended::FailedBeforeTheRenderInstruction { reason, .. } => reason.clone(),
        // **`Job files and outputs` C9, and the repaired K14 says so at the
        // site**: a walk of the job output folder that did not complete is
        // that document's *reading that did not end successfully* and is not
        // an ending of its own.
        Ended::TheWalkDidNotComplete { .. } => {
            BeforeTheRenderInstruction::AReadingThatDidNotEndSuccessfully
        }
        Ended::TheMaterialsPlacementWasNotConfirmed { what_happened, .. } => {
            BeforeTheRenderInstruction::TheMaterialsPlacementWasNotConfirmed {
                // **A sentence and never this type's name** — step 10c's fifth
                // pass on Vast, fault 5.
                what_happened: what_happened.in_words(),
            }
        }
        Ended::TheAttemptCouldNotBeComposed { what_happened, .. } => {
            BeforeTheRenderInstruction::TheAttemptCouldNotBeComposed {
                what_happened: what_happened.clone(),
            }
        }
        Ended::TheLaunchCouldNotBeMade { why, .. } => {
            BeforeTheRenderInstruction::TheLaunchCouldNotBeMade { why: why.clone() }
        }
        Ended::TheAttemptRecordCouldNotBeWritten { why, .. } => {
            BeforeTheRenderInstruction::TheAttemptRecordCouldNotBeWritten { why: why.clone() }
        }
    };
    Some(classify(Some(occasion), false, None))
}

/// **The value this crate supplies into `Queue & History`'s mapping.** A
/// hold ends the attempt and not the job, so it maps to no ending at all;
/// an ending with no status is that crate's own `NoStatusWasReaped`, so its
/// mapping keeps M21's rule rather than trusting this one to.
pub fn as_the_render_kinds_own_ending(
    outcome: &RecordedOutcome,
) -> Option<TheRenderKindsOwnEnding> {
    match outcome {
        RecordedOutcome::Finished => Some(TheRenderKindsOwnEnding::Finished),
        RecordedOutcome::Failed {
            reason: FailedReason::NoStatusWasReaped,
            ..
        } => Some(TheRenderKindsOwnEnding::NoStatusWasReaped),
        RecordedOutcome::Failed { .. } => Some(TheRenderKindsOwnEnding::Failed),
        RecordedOutcome::HeldForAnAnswer => None,
    }
}

/// **K14a: the value exists on the *failed* outcome alone.** *Finished* and
/// *held for an answer* carry none, and an outcome carrying no value is not
/// *the machine's own*.
pub fn the_cause_on(outcome: &RecordedOutcome) -> Option<Cause> {
    match outcome {
        RecordedOutcome::Failed { cause, .. } => Some(*cause),
        RecordedOutcome::Finished | RecordedOutcome::HeldForAnAnswer => None,
    }
}

/// **K15: an ending that never arrives is not one of the classification's
/// cases, and deliberately.** A run that sits for ever is a live process,
/// reads as live work, the entry stays *rendering* while the group
/// validates, and the stop is the user's route. **No timer of Smoozy's ends
/// a render or manufactures an ending for one.** K15a: the ordinary
/// occasion is an engine named on the launch line whose add-on is absent —
/// 4.5 does not exit at all. K15b: the machine's own re-run reproduces it
/// with nobody watching. K16: what bounds the cost is the classification
/// refusing to hide it; the want — saying something when readings stop
/// advancing — is parked with three arguments and not granted here.
///
/// **L20 is the failure this classification is written for, and it is named
/// here because nothing else in this crate carries it.** An engine name no
/// engine bears ends the run at a failure status from 5.0, while 4.5 prints
/// its message and then does not end at all — one run sat for nine minutes
/// with no status to record. L19's provenance rule is what makes the case
/// unreachable by composition; where it arises anyway — an adopted job, a
/// version substituted on the machine — the run reads as live work and the
/// stop is the route, which is exactly what this function answering nothing
/// keeps.
pub fn an_ending_that_never_arrives(
    _readings_have_stopped_for: std::time::Duration,
) -> Option<RecordedOutcome> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reaped(
        exit: Option<i32>,
        marker: bool,
        collection: CollectionOutcome,
    ) -> WhatTheWorkerReaped {
        WhatTheWorkerReaped {
            exit_status: exit,
            the_marker_was_observed: marker,
            the_ordinary_streams_collection: collection,
        }
    }

    fn a_statement(expected: usize, present: usize, at_zero_bytes: usize) -> TheCompletenessStatement {
        TheCompletenessStatement {
            expected,
            present: vec![b"a file".to_vec(); present],
            absent: vec![b"a file".to_vec(); expected - present - at_zero_bytes],
            present_at_zero_bytes: vec![b"a file".to_vec(); at_zero_bytes],
            what_the_added_write_places_produced: Vec::new(),
        }
    }

    /// **K14b: nothing written of what was owed is *failed*, undetermined, and
    /// maps to the render kind's own *failed*.**
    #[test]
    fn finished_with_none_of_what_was_owed_written_is_failed_and_undetermined() {
        let the_failure = RecordedOutcome::Failed {
            reason: FailedReason::NothingTheJobOwedWasWritten,
            cause: Cause::Undetermined,
        };
        for statement in [a_statement(2, 0, 0), a_statement(2, 0, 2), a_statement(1, 0, 1)] {
            assert_eq!(
                the_outcome_against_the_count(RecordedOutcome::Finished, Some(&statement)),
                the_failure,
                "{statement:?}"
            );
        }
        assert_eq!(
            as_the_render_kinds_own_ending(&the_failure),
            Some(TheRenderKindsOwnEnding::Failed)
        );
        assert_eq!(the_cause_on(&the_failure), Some(Cause::Undetermined));
    }

    /// **K14b's three places it does not fire, and every other outcome.**
    #[test]
    fn the_count_moves_only_a_finish_over_nothing_owed_and_written() {
        for (statement, why) in [
            (Some(a_statement(2, 1, 0)), "partly written"),
            (Some(a_statement(2, 1, 1)), "partly written, the rest empty"),
            (Some(a_statement(2, 2, 0)), "whole"),
            (Some(a_statement(0, 0, 0)), "owed nothing"),
            (None, "no statement, so no count"),
        ] {
            assert_eq!(
                the_outcome_against_the_count(RecordedOutcome::Finished, statement.as_ref()),
                RecordedOutcome::Finished,
                "{why}"
            );
        }
        let nothing_written = a_statement(2, 0, 0);
        for outcome in [
            RecordedOutcome::HeldForAnAnswer,
            RecordedOutcome::Failed {
                reason: FailedReason::ANonZeroStatus { exit_status: 1 },
                cause: Cause::Undetermined,
            },
            RecordedOutcome::Failed {
                reason: FailedReason::NoStatusWasReaped,
                cause: Cause::TheMachinesOwn,
            },
        ] {
            assert_eq!(
                the_outcome_against_the_count(outcome.clone(), Some(&nothing_written)),
                outcome
            );
        }
    }

    #[test]
    fn finished_is_exit_zero_and_the_marker_observed_and_says_nothing_about_the_outputs() {
        let outcome = classify(
            None,
            false,
            Some(&reaped(Some(0), true, CollectionOutcome::Collected)),
        );
        assert_eq!(outcome, RecordedOutcome::Finished);
        assert_eq!(
            as_the_render_kinds_own_ending(&outcome),
            Some(TheRenderKindsOwnEnding::Finished)
        );
        assert_eq!(the_cause_on(&outcome), None);
    }

    /// **D17's observed case**: exit 0, the marker never observed, the
    /// ordinary stream never established — the run never reached the
    /// launch material and what it rendered was not the job.
    #[test]
    fn exit_zero_with_no_marker_on_a_collected_stream_is_failed_and_undetermined() {
        for collection in [
            CollectionOutcome::NeverEstablished,
            CollectionOutcome::Collected,
            CollectionOutcome::CollectedAndTheStreamWasEmpty,
        ] {
            let outcome = classify(None, false, Some(&reaped(Some(0), false, collection)));
            assert_eq!(
                outcome,
                RecordedOutcome::Failed {
                    reason: FailedReason::TheMarkerWasNeverObserved { exit_status: 0 },
                    cause: Cause::Undetermined
                }
            );
        }
        // A capture that stopped cannot support the inference: exit status
        // alone, the collection outcome beside it.
        let incomplete = classify(
            None,
            false,
            Some(&reaped(
                Some(0),
                false,
                CollectionOutcome::CollectionIncomplete,
            )),
        );
        assert_eq!(incomplete, RecordedOutcome::Finished);
    }

    /// **E2's first fixed rule, and K14a's one non-default row.**
    #[test]
    fn an_ending_with_no_status_is_failed_and_the_machines_own() {
        let outcome = classify(
            None,
            false,
            Some(&reaped(None, true, CollectionOutcome::Collected)),
        );
        assert_eq!(
            outcome,
            RecordedOutcome::Failed {
                reason: FailedReason::NoStatusWasReaped,
                cause: Cause::TheMachinesOwn
            }
        );
        assert_eq!(
            as_the_render_kinds_own_ending(&outcome),
            Some(TheRenderKindsOwnEnding::NoStatusWasReaped)
        );
        assert_eq!(classify(None, false, None), outcome);
    }

    /// **K6: a non-zero number is recorded and names nothing; the cause is
    /// undetermined whatever the number.**
    #[test]
    fn a_non_zero_status_is_failed_and_undetermined_whatever_the_number() {
        for exit in [1, 2, 63, 134, 137, 139, 143] {
            let outcome = classify(
                None,
                false,
                Some(&reaped(Some(exit), true, CollectionOutcome::Collected)),
            );
            assert_eq!(
                outcome,
                RecordedOutcome::Failed {
                    reason: FailedReason::ANonZeroStatus { exit_status: exit },
                    cause: Cause::Undetermined
                }
            );
            assert_eq!(
                as_the_render_kinds_own_ending(&outcome),
                Some(TheRenderKindsOwnEnding::Failed)
            );
        }
    }

    /// **K14a's table over the pre-render-instruction occasions.**
    #[test]
    fn the_occasions_before_the_render_instruction_are_the_files_own_except_a_reading_that_did_not_end(
    ) {
        let files_own = [
            BeforeTheRenderInstruction::TheRemapMaterialCouldNotBeRead {
                material: "remaps/j1".to_string(),
            },
            BeforeTheRenderInstruction::TheVersionConfirmationRefused {
                reason: "4.5.9 answered".to_string(),
            },
            BeforeTheRenderInstruction::TheReadBackMismatched {
                read: b"a".to_vec(),
                expected: b"b".to_vec(),
            },
            BeforeTheRenderInstruction::APairWhoseAssignmentRaised {
                pair: "scene.render.resolution_percentage".to_string(),
            },
            BeforeTheRenderInstruction::ASettingsPairWhoseNameTheObjectDoesNotCarry {
                property: "scene.cycles.samplez".to_string(),
            },
            BeforeTheRenderInstruction::ANameTheReadingInForceDoesNotCarry {
                name: "Attic".to_string(),
            },
            BeforeTheRenderInstruction::ACardMakeWithNoTechnologyToName {
                make: "Matrox".to_string(),
            },
        ];
        for occasion in files_own {
            let outcome = classify(Some(occasion.clone()), false, None);
            assert_eq!(
                the_cause_on(&outcome),
                Some(Cause::TheFilesOwn),
                "{occasion:?}"
            );
            assert!(
                matches!(outcome, RecordedOutcome::Failed { reason: FailedReason::BeforeTheRenderInstruction(ref o), .. } if *o == occasion)
            );
        }
        let reading = classify(
            Some(BeforeTheRenderInstruction::AReadingThatDidNotEndSuccessfully),
            false,
            None,
        );
        assert_eq!(the_cause_on(&reading), Some(Cause::Undetermined));
        // **K14a's row of 2026-09-28: P1 step 4's reading of the job's file
        // is *undetermined* too**, never *the file's own* — a failed read is
        // not proof the file is at fault, and nothing is mined from Blender's
        // words to say it is.
        let the_files_reading = classify(
            Some(BeforeTheRenderInstruction::TheJobsFileReadingDidNotEndSuccessfully),
            false,
            None,
        );
        assert_eq!(the_cause_on(&the_files_reading), Some(Cause::Undetermined));
        // Where the attempt failed before the render instruction, what the
        // worker might have reaped is not consulted: no launch was made.
        let with_reaped = classify(
            Some(BeforeTheRenderInstruction::AReadingThatDidNotEndSuccessfully),
            false,
            Some(&reaped(Some(0), true, CollectionOutcome::Collected)),
        );
        assert_eq!(with_reaped, reading);
    }

    /// **L24b: the OptiX library not found is *failed*, cause
    /// *undetermined*** (K14a's row of 2026-10-03) — so the phone offers no
    /// *try it again* for a refusal the same machine would give again — **and
    /// its words are the plan's own**, which name the library.
    #[test]
    fn the_optix_library_not_found_is_failed_undetermined_and_says_what_is_missing() {
        let outcome = classify(
            Some(BeforeTheRenderInstruction::TheOptixLibraryWasNotFound),
            false,
            None,
        );
        assert_eq!(the_cause_on(&outcome), Some(Cause::Undetermined));
        let RecordedOutcome::Failed { reason, .. } = &outcome else {
            panic!("{outcome:?}");
        };
        assert_eq!(
            reason.in_words(),
            "the NVIDIA driver has no OptiX library (libnvoptix.so.1)"
        );
    }

    /// **An uploaded bake that could not be copied into the job folder is
    /// *failed*, cause *undetermined*** (K14a's row of 2026-09-29), and its
    /// words name the copy and why — `Dependency Upload & Remap` M22c.
    #[test]
    fn an_uploaded_bake_that_could_not_be_copied_is_failed_and_undetermined() {
        let outcome = classify(
            Some(BeforeTheRenderInstruction::AnUploadedBakeCouldNotBeCopied {
                copy: "the bake output folder //bake/".to_string(),
                why: "No space left on device".to_string(),
            }),
            false,
            None,
        );
        assert_eq!(the_cause_on(&outcome), Some(Cause::Undetermined));
        let RecordedOutcome::Failed { reason, .. } = &outcome else {
            panic!("{outcome:?}");
        };
        assert_eq!(reason.in_words(), "could not copy the bake output folder //bake/: No space left on device");
        assert_eq!(
            FailedReason::BeforeTheRenderInstruction(BeforeTheRenderInstruction::AnUploadedBakeCouldNotBeCopied {
                copy: "the bake folder //caches/".to_string(),
                why: String::new(),
            })
            .in_words(),
            "could not copy the bake folder //caches/",
            "the film's first job: no place, and nothing further a user can use"
        );
        assert!(outcome.no_render_began());
    }

    /// **No refusal reaches a person as a type's name.** Fault 5 of step
    /// 10c's fifth pass on Vast, 2026-09-23: the attempt record's reason was
    /// this type's debugging rendering, on a real rented machine.
    ///
    /// **Every occasion and every failed reason is checked**, by the shape a
    /// debugging rendering has and never by a list somebody has to remember to
    /// add to: a variant's own name in the words, or a brace, is the fault.
    #[test]
    fn no_ending_says_itself_in_a_type_name() {
        let occasions = [
            BeforeTheRenderInstruction::TheRemapMaterialCouldNotBeRead {
                material: "remaps/j1".to_string(),
            },
            BeforeTheRenderInstruction::TheVersionConfirmationRefused {
                reason: "4.5.9 answered".to_string(),
            },
            BeforeTheRenderInstruction::TheReadBackMismatched {
                read: b"a".to_vec(),
                expected: b"b".to_vec(),
            },
            BeforeTheRenderInstruction::AReadingThatDidNotEndSuccessfully,
            BeforeTheRenderInstruction::TheJobsFileReadingDidNotEndSuccessfully,
            BeforeTheRenderInstruction::APairWhoseAssignmentRaised {
                pair: "scene.render.resolution_percentage".to_string(),
            },
            BeforeTheRenderInstruction::ASettingsPairWhoseNameTheObjectDoesNotCarry {
                property: "scene.cycles.samplez".to_string(),
            },
            BeforeTheRenderInstruction::ANameTheReadingInForceDoesNotCarry {
                name: "Attic".to_string(),
            },
            BeforeTheRenderInstruction::ACardMakeWithNoTechnologyToName {
                make: "NoReadingStoodAtSubmission".to_string(),
            },
            BeforeTheRenderInstruction::TheOptixLibraryWasNotFound,
            BeforeTheRenderInstruction::TheMaterialsPlacementWasNotConfirmed {
                what_happened: "the file was not there afterwards".to_string(),
            },
            BeforeTheRenderInstruction::TheAttemptCouldNotBeComposed {
                what_happened: "the frame list named no frame".to_string(),
            },
            BeforeTheRenderInstruction::TheLaunchCouldNotBeMade {
                why: "the build could not be run".to_string(),
            },
            BeforeTheRenderInstruction::TheAttemptRecordCouldNotBeWritten {
                why: "the disk is full".to_string(),
            },
            BeforeTheRenderInstruction::AnUploadedBakeCouldNotBeCopied {
                copy: "the bake output folder //bake/".to_string(),
                why: "No space left on device".to_string(),
            },
        ];
        let reasons: Vec<FailedReason> = occasions
            .iter()
            .cloned()
            .map(FailedReason::BeforeTheRenderInstruction)
            .chain([
                FailedReason::ANonZeroStatus { exit_status: 137 },
                FailedReason::NoStatusWasReaped,
                FailedReason::TheMarkerWasNeverObserved { exit_status: 0 },
                FailedReason::NothingTheJobOwedWasWritten,
                FailedReason::TheRecordedPairCouldNotBeWritten {
                    why: "the record would not take it".to_string(),
                },
            ])
            .collect();
        for reason in &reasons {
            let said = reason.in_words();
            assert!(!said.is_empty(), "{reason:?} says nothing");
            for mark in ['{', '}', '(', ')'] {
                // **A parenthesis is allowed where it holds a value** — a
                // material's name, a make — and a brace never is.
                if mark == '{' || mark == '}' {
                    assert!(!said.contains(mark), "{reason:?} says {said}");
                }
            }
            // **The variant's own name is what a debugging rendering leads
            // with**, so no reading of it may carry one.
            let rendered = format!("{reason:?}");
            let the_name = rendered
                .split(['(', ' ', '{'])
                .next()
                .expect("a rendering")
                .to_string();
            assert!(!said.contains(&the_name), "{reason:?} says its own name: {said}");
        }
        // **An exit at 0 never says *code 0* beside *Failed***: both of its
        // reasons say *normally*. Added 2026-11-06, the check after the last
        // fix.
        for reason in [
            FailedReason::TheMarkerWasNeverObserved { exit_status: 0 },
            FailedReason::NothingTheJobOwedWasWritten,
        ] {
            let said = reason.in_words();
            assert!(said.starts_with("Blender exited normally but "), "{reason:?} says {said}");
            assert!(!said.contains("code 0"), "{reason:?} says {said}");
        }
        // **And each occasion's own words reach the reason unchanged**, so
        // there is one wording and not two.
        for occasion in &occasions {
            assert_eq!(
                FailedReason::BeforeTheRenderInstruction(occasion.clone()).in_words(),
                occasion.in_words()
            );
        }
    }

    #[test]
    fn a_hold_ends_the_attempt_and_not_the_job_and_maps_to_no_ending() {
        let held = classify(
            None,
            true,
            Some(&reaped(Some(0), true, CollectionOutcome::Collected)),
        );
        assert_eq!(held, RecordedOutcome::HeldForAnAnswer);
        assert_eq!(as_the_render_kinds_own_ending(&held), None);
        assert_eq!(the_cause_on(&held), None);
    }

    /// **The six endings the driver reaches, and not one of them is *the
    /// machine's own*.**
    ///
    /// **The user's moment this stands against.** A builder wires the worker
    /// up and reaches for the one function that exists. A job whose launch
    /// material could not be placed, or whose composition refused because a
    /// `Render Settings` spelling has not landed, is recorded as the
    /// machine's own; `Telegram` Y4a attaches *try it again* to that value
    /// alone; the user taps it, the identical job goes again, the material
    /// cannot be placed again, and they have paid twice for the same
    /// refusal.
    #[cfg(unix)]
    #[test]
    fn every_ending_the_driver_reaches_has_a_home_and_none_of_the_six_is_the_machines_own() {
        use crate::acting::pipeline::WhereTheAttemptEnded as Ended;
        use crate::pipeline::THE_NINE_STEPS;
        let step = THE_NINE_STEPS[6];

        let six = [
            Ended::TheWalkDidNotComplete { at: step },
            Ended::TheMaterialsPlacementWasNotConfirmed {
                at: step,
                what_happened:
                    crate::acting::material::WhetherThePlacementIsConfirmed::ItReadBackAsSomethingElse,
            },
            Ended::TheAttemptCouldNotBeComposed {
                at: step,
                what_happened: "a spelling that has not landed".to_string(),
            },
            Ended::TheLaunchCouldNotBeMade {
                at: step,
                why: "the streams could not be held".to_string(),
            },
            Ended::TheAttemptRecordCouldNotBeWritten {
                at: step,
                why: "no room on the mount".to_string(),
            },
            Ended::TheRecordedPairCouldNotBeWritten {
                at: step,
                why: "no room on the mount".to_string(),
            },
        ];
        for ending in six {
            let outcome = the_outcome_of(&ending).expect("the six all end the attempt");
            assert_eq!(
                the_cause_on(&outcome),
                Some(Cause::Undetermined),
                "{ending:?} took a cause K14a does not give it"
            );
            assert_eq!(
                as_the_render_kinds_own_ending(&outcome),
                Some(TheRenderKindsOwnEnding::Failed),
                "{ending:?}"
            );
        }

        // **A hold ends the attempt and not the job**, and the driver's own
        // arm for it reaches K14's *held for an answer* rather than a
        // failure.
        let held = the_outcome_of(&Ended::AHoldWasPlaced {
            hold: smoozy_state::identities::Identity::issue(
                smoozy_state::identities::IdentityKind::Condition,
            )
            .unwrap(),
        });
        assert_eq!(held, Some(RecordedOutcome::HeldForAnAnswer));

        // **The two that carry no outcome yet**, each for its own reason: a
        // render that was launched is steps 8 and 9's to reap, and P3's
        // interrupted attempt is reconciliation's *given up*, which is not
        // one of the kind's own endings.
        assert_eq!(
            the_outcome_of(&Ended::NoHoldWasPlacedAndReconciliationTakesIt),
            None
        );

        // **A failure before the render instruction keeps the row K14a
        // already gives it**, and this mapping changes none of them.
        let files_own = the_outcome_of(&Ended::FailedBeforeTheRenderInstruction {
            at: step,
            reason: BeforeTheRenderInstruction::TheRemapMaterialCouldNotBeRead {
                material: "remaps/j1".to_string(),
            },
        })
        .unwrap();
        assert_eq!(the_cause_on(&files_own), Some(Cause::TheFilesOwn));
    }

    /// **K15: no timer manufactures an ending**, however long the readings
    /// have stopped.
    #[test]
    fn no_ending_is_manufactured_for_a_run_that_never_ends() {
        for hours in [1, 12, 240] {
            assert_eq!(
                an_ending_that_never_arrives(std::time::Duration::from_secs(hours * 3600)),
                None
            );
        }
    }

    /// **The signature is the guard for K6 and K14a together**: the sources
    /// are read for a cause composed from a number.
    #[test]
    fn no_line_of_this_crate_composes_a_cause_from_an_exit_number() {
        let machines_own = ["Cause::", "TheMachinesOwn"].concat();
        for (path, text) in crate::markers::tests::every_source_of_this_crate() {
            for line in text.lines() {
                let trimmed = line.trim_start();
                if trimmed.starts_with("//") {
                    continue;
                }
                let matches_a_number = trimmed.starts_with("Some(") && trimmed.contains("=>");
                assert!(
                    !(matches_a_number && trimmed.contains(&machines_own)),
                    "{path}: {line}"
                );
            }
        }
    }
}

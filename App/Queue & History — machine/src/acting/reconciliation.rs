//! **§9 performed — the machine going down and coming back.** The disposition
//! decided from durable records alone, the attempt count advanced durably
//! before the second attempt starts, the re-run's explicit frame list taken
//! from `Job files and outputs`' own reckoning, and the video switch thrown
//! before the render from the recorded reading.
//!
//! # Where this runs, and what it never waits for
//!
//! **D4: the re-run is the machine's own act, from durable state, and nothing
//! waits on anybody.** Every function here is the supervisor's, on the rented
//! machine, with no connection and no Smoozy in it. **Nothing in this module
//! opens a connection, composes a command, or reads anything the desktop
//! wrote.**
//!
//! # The order this runs in, and it may not be reversed
//!
//! **The disposition is chosen from records alone — the desired state, the
//! recorded execution group, the attempt count — before any walk is taken.**
//! That is `Rented-server runtime` S12's rule and the reason
//! [`smoozy_runtime::supervisor::decide_then_walk`] is used here rather than a
//! second arrangement of the same two steps: the walk supplies a measurement
//! that rides an outcome already decided, and reversing the order is what
//! `State, identity and recovery` §1.3 rule 5 forbids.
//!
//! # D14: an outstanding act on the line is finished before anything starts
//!
//! **The clear, the reorder and the block move are partial across records and
//! made whole by a durable record of the act** (L23, L24), and **the record
//! changes nothing across a crash unless somebody finishes it.** That is what
//! [`finish_the_outstanding_acts`] does, at the supervisor's start, before
//! [`what_reconciliation_may_start`] may return anything — the clears first
//! and the rewrites after. **The order is kept by the type**:
//! [`TheLineIsSettled`] has a field this module owns, so nothing else can
//! compose one.
//!
//! **The case it is for is the one that matters most**: a supervisor killed
//! between two removals comes back, finds the line half-cleared, and starts a
//! job the user asked to have cleared.
//!
//! # What is taken whole rather than re-derived
//!
//! - **The two dispositions that end a job** are
//!   [`smoozy_runtime::supervisor::DispositionThatEndsAJob`], and the outcome
//!   they are written in is that crate's [`AnEndingOutcome`].
//! - **D7's list contents** are `Job files and outputs` §10's entire —
//!   [`smoozy_job_files_machine::reckoning::the_reckoning`] — with C12 to C13's
//!   already-there test, C17's re-rendered last frame, C18a's walk back to the
//!   latest list and C20's whole range. **Not one of them is restated here.**
//! - **D5's ground** — no version resumes inside a frame — is the joined
//!   answer's own sentence and is why the reckoning exists at all.
//!
//! # What this module refuses to do
//!
//! **D6: Blender's own skipping is never used**, and nothing here composes a
//! frame set as text — `Job files and outputs` C3, at
//! [`smoozy_job_files_machine::reckoning::THE_FRAME_LIST_IS_INTEGERS_AND_NEVER_TEXT`],
//! is the rule and every frame below is an integer.
//!
//! **D10: nothing reads a video file's size, ever.** There is no size reading
//! anywhere in this module and a test scans its own source for one.
//!
//! **D8 is permanently unresolved and no rescue is built for it.** What a true
//! power cut leaves is not established, and what holds is the spec's own
//! fallback, already written into the promise: render the frames again rather
//! than trust a file Smoozy cannot vouch for.
//! [`WHAT_HOLDS_WITHOUT_FACT_44`] carries it beside the code.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Rented-server runtime` S11, S12, S13, S14.
//! `Job files and outputs` C12, C13, C17, C18a, C20, M2. `State, identity and
//! recovery` §1.3, §6. `Dependency Upload & Remap` U2.
//!
//! **Seven of that document's markers are deliberately not on that list and
//! are written out at every site instead** — `Job files and outputs` C1,
//! `Job files and outputs` C3, `Job files and outputs` C6,
//! `Job files and outputs` C7, `Job files and outputs` C9,
//! `Job files and outputs` I19 and `Job files and outputs` I23 — because this
//! plan defines a clause of each of those names too, and a bare one could
//! then mean either.

use smoozy_job_files_machine::attempts::AttemptRecord;
use smoozy_job_files_machine::inventory::WhatTheInventoryIs;
use smoozy_job_files_machine::reckoning::{the_reckoning, WhatTheReckoningSays, WhatTheWalkOfTheFolderIs};
use smoozy_runtime::supervisor::{
    decide_then_walk, what_step_2_decides, AnEndingOutcome, CompletenessStatement,
    DispositionThatEndsAJob, WhatStep2Decides,
};
// **`State, identity and recovery` §6's own four names**, imported under a name
// of their own because this plan has a `Disposition` of its own and the two are
// different things: §6's is what one supervisor run did with a record, and this
// plan's is what reconciliation decided about a render.
use smoozy_state::machine_operations::{DesiredState, Disposition as TheFourNames};

#[cfg(unix)]
use crate::acting::commands::{finish_a_recorded_rewrite, TheActsRecord};
// The act records are a store on a rented server — `State, identity and
// recovery` J11 — and a rented machine is a Linux machine (that document's
// §1.3 rule 6), so the store and what takes it are on a unix only.
// `crate::acting` states the arrangement once.
#[cfg(unix)]
use crate::acting::commands::{remove_each_covered_record, TheActRecordsOnTheMachine};
#[cfg(unix)]
use crate::acting::queue_store::{TheQueueOnTheMachine, WhatTheStoreSaid};
use crate::line::JobRecord;

/// **The durable desired state D4 and D12 read**, and it is `State, identity
/// and recovery` §3 D1's one durable desired state for a render — never an
/// observed reading of anything.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheDesiredState {
    ShouldRun,
    /// Written durably before the signal, `State, identity and recovery` §5.3.
    ShouldBeStopped,
}

/// **The three durable facts §6's step 2 decides from, and there is no
/// fourth.**
///
/// **None of them is a reading of a file, a folder or a stream.** The
/// execution group is the recorded one, validated by the kernel's own answer
/// about a recorded pair (`Rented-server runtime` X1); the attempt count lives
/// in the operation record.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WhatTheRecordsSay {
    pub desired: TheDesiredState,
    /// The recorded execution group no longer validates.
    pub the_recorded_execution_group_is_gone: bool,
    /// The count in the operation record, before this reconciliation.
    pub attempt_count: u32,
}

/// What reconciliation decides for one job.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Disposition {
    /// **E1's other half, met here: a group that still validates is a live
    /// run**, and reconciliation starts nothing and ends nothing.
    TheRenderIsStillAlive,
    /// **D4: *should run*, group gone, attempt count 1.**
    ReRunOnce,
    /// **D12 and L10's two**, both of which end the job with no worker alive
    /// to have taken the ending step (`Rented-server runtime` S11).
    ItEndsTheJob(DispositionThatEndsAJob),
}

/// **D4 and D12's table, and nothing else is in it.**
///
/// **`State, identity and recovery` §6 owns the table and this is that
/// document's step 2 read for a render**, cited rather than re-derived: the
/// two branches this plan states in its own words are *should run*, group
/// gone, attempt count 1 → **re-run once**, and the same with attempt count 2
/// → **given up**. A durable *should be stopped* is **left stopped** whatever
/// the count, which is L10's outcome written by reconciliation with no worker
/// alive (H7).
///
/// **D3 needs no rule of its own**: the interrupted job's record still holds
/// the lowest live key, so the re-run precedes the rest of the line by Q4's
/// ordinary admission rule.
///
/// # The rows are read from their one home rather than written here
///
/// **Changed 2026-09-07, and it is a move rather than a rewrite.** The five
/// rows used to be written out in this function, and the supervisor — the one
/// thing `State, identity and recovery` §6 gives the loop to — could not reach
/// them: this crate carries the desktop's SSH client in its tree, so the
/// program a rented machine runs may never depend on it. **The rows now live at
/// [`smoozy_runtime::supervisor::what_step_2_decides`]**, which that program can
/// reach, **and this function reads them there.** A fact kept in two places is
/// the fault this stage keeps meeting; the branches below are the naming of the
/// rows in this plan's own words and decide nothing.
///
/// **What this plan keeps that §6 has no word for is E1's live run.** *Should
/// run* with a group that still validates is §6's *resumed*; for a render, this
/// document's own answer is that reconciliation starts nothing and ends nothing,
/// which is what [`Disposition::TheRenderIsStillAlive`] says.
pub fn the_disposition(say: &WhatTheRecordsSay) -> Disposition {
    let desired = match say.desired {
        TheDesiredState::ShouldRun => DesiredState::ShouldRun,
        TheDesiredState::ShouldBeStopped => DesiredState::ShouldBeStopped,
    };
    match what_step_2_decides(
        desired,
        say.the_recorded_execution_group_is_gone,
        say.attempt_count,
    ) {
        WhatStep2Decides::ItTakesThisBranch(TheFourNames::Resumed) => {
            Disposition::TheRenderIsStillAlive
        }
        WhatStep2Decides::ItTakesThisBranch(TheFourNames::ReRunOnce) => Disposition::ReRunOnce,
        // **D12: re-run once, not forever.** Two attempts and the machine
        // moves on.
        WhatStep2Decides::ItTakesThisBranch(TheFourNames::GivenUp) => {
            Disposition::ItEndsTheJob(DispositionThatEndsAJob::GivenUp)
        }
        WhatStep2Decides::ItTakesThisBranch(TheFourNames::LeftStopped) => {
            Disposition::ItEndsTheJob(DispositionThatEndsAJob::LeftStopped)
        }
        // **The *ended* row cannot be reached from here**, because the mapping
        // three lines above turns this plan's two durable states into two of
        // §6's three and never into the third. **It is said rather than given an
        // answer**: every answer this function could return would be a branch
        // this plan does not state, and a wrong one would end a job.
        WhatStep2Decides::ItCarriesEndedAndIsClosed => unreachable!(
            "`TheDesiredState` has no member that maps to the ended desired state"
        ),
    }
}

/// **D12, performed: the disposition and the recorded outcome are written
/// together**, with the completeness statement taken after the disposition is
/// decided and in the same act (`Rented-server runtime` S11 and S12).
///
/// **The walk is handed in and never taken here**: it is `Job files and
/// outputs` C1 and `Job files and outputs` C7's one walk, and where it cannot
/// be taken the outcome is
/// written without a statement and says so (S13) — which is exactly the
/// difference H8 turns on, and which four empty sets would destroy.
pub fn the_ending_outcome<W>(disposition: DispositionThatEndsAJob, walk: W) -> AnEndingOutcome
where
    W: FnOnce(DispositionThatEndsAJob) -> Option<CompletenessStatement>,
{
    decide_then_walk(disposition, |disposition| (walk(disposition), None))
}

/// **D9's switch, thrown before the render and read off the recorded
/// reading.**
///
/// *"Where any destination is video-shaped: the reckoning does not run and the
/// attempt covers the whole range."* **The shape is a field of the reading
/// (`Job files and outputs` I19 to `Job files and outputs` I23), known at
/// Pre-load Settings or at job start**, so this is a read
/// of a record Smoozy wrote and never a look at a file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhetherTheReckoningRuns {
    /// **D9 and D11**: the whole range again. **D11's closed-off split file is
    /// not read as progress** — the artist's own splitting setting changes
    /// nothing here.
    ItDoesNotRunAndTheAttemptCoversTheWholeRange,
    ItRuns,
}

/// **D9's switch, read and never inferred.**
pub fn whether_the_reckoning_runs(a_destination_is_video_shaped: bool) -> WhetherTheReckoningRuns {
    if a_destination_is_video_shaped {
        WhetherTheReckoningRuns::ItDoesNotRunAndTheAttemptCoversTheWholeRange
    } else {
        WhetherTheReckoningRuns::ItRuns
    }
}

/// **D10, kept as a value: a video job's entry carries no on-disk progress
/// figure of any kind and no last-frame figure.**
///
/// *"Its size does not move while the render runs on one record and grew
/// stepwise on another — the two topics disagree, neither is chosen."*
/// `State, identity and recovery` §1.3 rule 5 forbids the reading anyway.
pub const NOTHING_HERE_READS_A_VIDEO_FILES_SIZE: bool = true;

/// **D8's *what holds*, recorded beside the code rather than rescued.**
///
/// Fact 44 is not posable on anything this project runs and the register's own
/// route is with the Creator. **What holds is the spec's own fallback, already
/// written into the promise**: render the frames again rather than trust a
/// file Smoozy cannot vouch for. **No size floor, no trailing-byte check and
/// no re-render of anything that looks small is built here** — each would be a
/// rescue for a known hole, and `Job files and outputs` C13's zero-byte rule
/// is the whole of what this stage refuses to count.
pub const WHAT_HOLDS_WITHOUT_FACT_44: &str =
    "render the frames again rather than trust a file Smoozy cannot vouch for";

/// **D4's own constraint on a re-run, and it is a queue fact rather than a
/// preference**: *"a re-run that cannot read the remap material its manifest
/// names must not render, ... such an attempt fails rather than rendering the
/// artist's own recorded locations."*
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheReRunDoes {
    /// **D13 and Q8.** The count could not be advanced, so nothing starts:
    /// *"where the state mount is full the count cannot advance — and nothing
    /// runs at all"*. The machine's own words are carried.
    TheCountCouldNotBeAdvancedSoNothingStarts { why: String },
    /// **D4's remap constraint.** The material the manifest names could not be
    /// read, so the attempt fails rather than rendering the artist's own
    /// recorded locations.
    ItMustNotRenderBecauseTheRemapMaterialCouldNotBeRead { at: String },
    /// **`Job files and outputs` C9**: the walk did not complete, so there was
    /// no reading to reckon against.
    TheWalkDidNotCompleteSoThisAttemptFails,
    /// **D6: the explicit list of frames, composed from integers.**
    RenderTheseFrames { attempt: u32, frames: Vec<i64> },
}

/// **What the supervisor hands the re-run, all of it read from durable
/// records.**
///
/// **M2: a re-run reads, never recomposes.** The whole range, the inventory
/// and the previous attempts all come out of the immutable manifest and the
/// job record directory; nothing here composes a manifest or a reading.
pub struct TheReRunsGround<'a> {
    /// The manifest's frame range, expanded to integers by the document that
    /// owns it.
    pub the_whole_range: &'a [i64],
    pub inventory: &'a WhatTheInventoryIs,
    /// Oldest first, which is the order their numbers put them in.
    pub previous_attempts: &'a [AttemptRecord],
    pub walk: &'a WhatTheWalkOfTheFolderIs,
    pub job_output_folder: &'a str,
    /// **D9's switch**, read off the recorded reading before any of this.
    pub a_destination_is_video_shaped: bool,
}

/// **D4 and D6 performed, in the one order that survives the crash it
/// counts.**
///
/// **The count of 2 is written durably before the new attempt starts** —
/// *"which is why the very crash it counts cannot lose it"* — so
/// `advance_the_count` is called first and a refusal ends the act with nothing
/// started (D13). Only then is the remap material read, and only then is the
/// reckoning run.
///
/// **The remap material is located by `Dependency Upload & Remap` U2's own
/// rule** through [`crate::acts::the_remap_material_a_cancel_removes`], never
/// by a path composed here.
pub fn the_re_run<A, R>(
    ground: &TheReRunsGround<'_>,
    advance_the_count: A,
    read_the_remap_material: R,
) -> WhatTheReRunDoes
where
    A: FnOnce() -> Result<u32, String>,
    R: FnOnce() -> Result<(), String>,
{
    let attempt = match advance_the_count() {
        Ok(attempt) => attempt,
        Err(why) => return WhatTheReRunDoes::TheCountCouldNotBeAdvancedSoNothingStarts { why },
    };
    if let Err(at) = read_the_remap_material() {
        return WhatTheReRunDoes::ItMustNotRenderBecauseTheRemapMaterialCouldNotBeRead { at };
    }
    // **D9: the switch is thrown before the render**, and where it is thrown
    // the reckoning does not run at all.
    if whether_the_reckoning_runs(ground.a_destination_is_video_shaped)
        == WhetherTheReckoningRuns::ItDoesNotRunAndTheAttemptCoversTheWholeRange
    {
        return WhatTheReRunDoes::RenderTheseFrames {
            attempt,
            frames: ground.the_whole_range.to_vec(),
        };
    }
    match the_reckoning(
        ground.the_whole_range,
        ground.inventory,
        ground.previous_attempts,
        ground.walk,
        ground.job_output_folder,
    ) {
        WhatTheReckoningSays::TheWalkDidNotCompleteSoThisIsAFailedAttempt => {
            WhatTheReRunDoes::TheWalkDidNotCompleteSoThisAttemptFails
        }
        WhatTheReckoningSays::RenderTheseFrames(frames) => {
            WhatTheReRunDoes::RenderTheseFrames { attempt, frames }
        }
    }
}

/// **D14's answer, and it cannot be composed any other way.**
///
/// The one value [`what_reconciliation_may_start`] takes beside the records,
/// and [`finish_the_outstanding_acts`] is the only thing that produces one —
/// its field is this module's, so **the order D14 states is kept by the type
/// rather than by anybody remembering it.**
#[derive(Debug)]
pub struct TheLineIsSettled(());

/// **D14, performed: an outstanding clear, reorder or block move is finished
/// from its own record before the admission point may start anything.**
///
/// **The clears go first and the rewrites after**, which is the order D14
/// states: a rewrite over records a clear is about to remove would write keys
/// nothing will read.
///
/// **Without this the records L23 and L24 write change nothing across a
/// crash**, which is the case that matters most — a supervisor killed between
/// two removals comes back, finds the line half-cleared, and starts a job the
/// user asked to have cleared.
///
/// **A completion that is itself refused leaves the record standing, the act
/// outstanding and nothing started**, which is Q8's disk-full answer applied
/// here and not a second one: the refusal is handed back and no value of
/// [`TheLineIsSettled`] is produced.
///
/// **On a unix only, because it reads and writes the act records** — the same
/// reason those carry.
///
/// **Its first caller is the program a rented machine runs, from 2026-09-29,
/// step 10c's first stop-and-clear piece**: at every supervisor start, before reconciliation and
/// before the line, and again at the line before anything is admitted.
/// **`after_each_removal` is L23's last bullet** — each record this pass
/// removes is handed back to be journalled and cleaned up after, exactly as
/// the clear that began it hands them back. **What comes back beside the
/// settled line is every clear this pass finished**, so the caller can close
/// that clear's command and carry out the stop paired with it.
#[cfg(unix)]
pub fn finish_the_outstanding_acts(
    store: &mut TheQueueOnTheMachine,
    acts: &mut TheActRecordsOnTheMachine,
    after_each_removal: &mut dyn FnMut(&smoozy_state::identities::Identity),
) -> Result<(TheLineIsSettled, Vec<smoozy_state::identities::Identity>), WhatTheStoreSaid> {
    let outstanding = acts.every_outstanding()?;
    let mut finished_clears = Vec::new();
    for (act, record) in &outstanding {
        if let TheActsRecord::AClear { covered } = record {
            let mut standing = covered.clone();
            remove_each_covered_record(store, acts, act, &mut standing, after_each_removal)?;
            let finished = TheActsRecord::AClear { covered: standing };
            if finished.every_identity_is_done() {
                acts.cleared(act)?;
                finished_clears.push(act.clone());
            }
        }
    }
    for (act, record) in &outstanding {
        if let TheActsRecord::ARewrite { keys } = record {
            let (records, _) = store.every_record()?;
            finish_a_recorded_rewrite(store, &records, keys)?;
            acts.cleared(act)?;
        }
    }
    Ok((TheLineIsSettled(()), finished_clears))
}

/// **D1, read off the records the store hands back.**
///
/// *"The queue survives in order because the order is data."* The line is
/// [`crate::line::the_line`]'s over the durable records, and this function is
/// the whole of what reconciliation adds: **it starts nothing whose durable
/// state forbids it.**
///
/// A record whose state is *held* is not in the line at all (Q5); a record
/// whose state is *rendering* is the interrupted job and is the disposition
/// table's subject, not the admission point's; a record marked by L21 is
/// dropped at the admission point rather than started (L22).
///
/// **D14: it may return nothing until every outstanding act on the line has
/// been finished from its own record**, which is what the second argument
/// says and what only [`finish_the_outstanding_acts`] can produce.
pub fn what_reconciliation_may_start<'a>(
    records: &'a [JobRecord],
    _the_line_is_settled: &TheLineIsSettled,
) -> Vec<&'a JobRecord> {
    crate::line::the_line(records)
        .into_iter()
        .filter(|record| record.removed_build.is_none())
        .collect()
}

/// **D2's two named states, both told apart honestly and neither silent.**
///
/// *"A job expected to resume that neither starts nor gains an ending is one
/// of exactly two named states."* **The promise's own escalation clause was
/// written for an unanswered Fact 1; Fact 1 is answered, and what remains is
/// these two.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhyAJobExpectedToResumeDidNeither {
    /// **A recycled or reimaged machine.** The installation identity check
    /// reads absent or different, nothing on it is Smoozy's, and every
    /// machine-derived value against the row is disqualified.
    TheMachineIsGone,
    /// **The state mount filled.** Where the supervisor is down, the machine
    /// reads as the supervisor-unavailable condition and the repair is `Server
    /// Setup`'s and `Desktop app and delivery`'s trigger (Q8); where it lived
    /// through the fill, it answers throughout and the ending is held pending
    /// under `Rented-server runtime` S14 until its write lands.
    TheStateMountFilled,
}

/// **D2, read from the two facts that separate the pair**, and from nothing
/// else. **Neither answer is silence** — that is the whole of what the clause
/// promises.
pub fn why_a_job_expected_to_resume_did_neither(
    the_installation_identity_reads_absent_or_different: bool,
) -> WhyAJobExpectedToResumeDidNeither {
    if the_installation_identity_reads_absent_or_different {
        WhyAJobExpectedToResumeDidNeither::TheMachineIsGone
    } else {
        WhyAJobExpectedToResumeDidNeither::TheStateMountFilled
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use smoozy_job_files_machine::inventory::{AnEntry, TheInventory};
    use smoozy_job_files_machine::reckoning::{AWalkEntry, WhatKindOfEntry};

    fn records(desired: TheDesiredState, gone: bool, attempt_count: u32) -> WhatTheRecordsSay {
        WhatTheRecordsSay {
            desired,
            the_recorded_execution_group_is_gone: gone,
            attempt_count,
        }
    }

    /// **D4 and D12's table, both branches and the live case.**
    #[test]
    fn the_table_re_runs_once_then_gives_up_and_never_touches_a_live_run() {
        assert_eq!(
            the_disposition(&records(TheDesiredState::ShouldRun, true, 1)),
            Disposition::ReRunOnce
        );
        assert_eq!(
            the_disposition(&records(TheDesiredState::ShouldRun, true, 2)),
            Disposition::ItEndsTheJob(DispositionThatEndsAJob::GivenUp)
        );
        assert_eq!(
            the_disposition(&records(TheDesiredState::ShouldBeStopped, true, 1)),
            Disposition::ItEndsTheJob(DispositionThatEndsAJob::LeftStopped)
        );
        // **A group that still validates is a live run** — E1's rule, so
        // reconciliation neither starts nor ends it, whatever the count.
        for count in 1..=3 {
            assert_eq!(
                the_disposition(&records(TheDesiredState::ShouldRun, false, count)),
                Disposition::TheRenderIsStillAlive
            );
        }
    }

    /// **D12: re-run once, not forever.** A third attempt has no branch to
    /// arrive at, so a machine that takes itself down every time costs two
    /// attempts and then a *given up* entry.
    #[test]
    fn a_scene_that_takes_the_machine_down_costs_two_attempts_and_no_more() {
        for count in 2..=10 {
            assert_eq!(
                the_disposition(&records(TheDesiredState::ShouldRun, true, count)),
                Disposition::ItEndsTheJob(DispositionThatEndsAJob::GivenUp)
            );
        }
    }

    fn an_inventory(frames: &[i64]) -> WhatTheInventoryIs {
        WhatTheInventoryIs::It(TheInventory {
            entries: frames
                .iter()
                .map(|frame| AnEntry {
                    predicted_path: format!("f_{frame:04}.png").into_bytes(),
                    frame: Some(*frame),
                    the_route_that_produced_the_name:
                        smoozy_job_files_machine::inventory::WhichRouteProducedTheName::ItWasAskedOfBlender,
                    the_destinations_aiming_at_it: vec![0],
                    may_grow: false,
                })
                .collect(),
            added_write_place_folders: Vec::new(),
            collision_record: Vec::new(),
            a_destination_is_video_shaped: false,
            why_it_owes_nothing: None,
        })
    }

    fn a_walk(present: &[i64]) -> WhatTheWalkOfTheFolderIs {
        WhatTheWalkOfTheFolderIs::ItCompleted(
            present
                .iter()
                .map(|frame| AWalkEntry {
                    path_under_the_job_output_folder: format!("f_{frame:04}.png").into_bytes(),
                    kind: WhatKindOfEntry::AFile,
                    size: 10,
                })
                .collect(),
        )
    }

    fn an_attempt(number: u32, list: &[i64]) -> AttemptRecord {
        AttemptRecord {
            number,
            frame_list_in_the_order_it_was_written: list.to_vec(),
            map_in_force: smoozy_job_files_machine::attempts::TheMapInForce::NoneWasComposedForThisAttempt,
            file_identity_at_this_attempt: smoozy_framing::source_reading::SourceReading {
                size: 1,
                inode: "1".to_string(),
                modification_time: 1,
                change_time: 1,
            },
            the_file_changed_before_this_attempt: false,
            a_frame_an_earlier_attempt_wrote_was_kept: false,
        }
    }

    fn ground<'a>(
        range: &'a [i64],
        inventory: &'a WhatTheInventoryIs,
        attempts: &'a [AttemptRecord],
        walk: &'a WhatTheWalkOfTheFolderIs,
        video: bool,
    ) -> TheReRunsGround<'a> {
        TheReRunsGround {
            the_whole_range: range,
            inventory,
            previous_attempts: attempts,
            walk,
            job_output_folder: "/ephemeral/smoozy/outputs/job",
            a_destination_is_video_shaped: video,
        }
    }

    /// **D6 and D7: the re-run's frame set is an explicit integer list from
    /// the reckoning**, with C17's last frame asked for re-rendered anyway.
    #[test]
    fn the_re_run_hands_over_the_reckonings_own_integers_and_re_renders_the_boundary_frame() {
        let range = [1, 2, 3, 4, 5];
        let inventory = an_inventory(&range);
        // Frames 1 to 3 are on disk; the previous attempt asked for all five.
        let walk = a_walk(&[1, 2, 3]);
        let attempts = [an_attempt(1, &range)];
        let ground = ground(&range, &inventory, &attempts, &walk, false);
        let what = the_re_run(&ground, || Ok(2), || Ok(()));
        let WhatTheReRunDoes::RenderTheseFrames { attempt, frames } = what else {
            panic!("{what:?}");
        };
        assert_eq!(attempt, 2);
        // **`Job files and outputs` C17: frame 3 is the last one asked for
        // that is already there and is re-rendered anyway.**
        assert_eq!(frames, vec![3, 4, 5]);
    }

    /// **D9: a video-shaped destination gets none of the skipping**, and the
    /// reckoning does not run at all — the whole range again.
    #[test]
    fn a_video_job_covers_the_whole_range_and_the_reckoning_never_runs() {
        let range = [1, 2, 3, 4, 5];
        let inventory = an_inventory(&range);
        let walk = a_walk(&[1, 2, 3, 4]);
        let attempts = [an_attempt(1, &range)];
        let ground = ground(&range, &inventory, &attempts, &walk, true);
        let what = the_re_run(&ground, || Ok(2), || Ok(()));
        assert_eq!(
            what,
            WhatTheReRunDoes::RenderTheseFrames {
                attempt: 2,
                frames: range.to_vec()
            }
        );
        assert_eq!(
            whether_the_reckoning_runs(true),
            WhetherTheReckoningRuns::ItDoesNotRunAndTheAttemptCoversTheWholeRange
        );
    }

    /// **D4 and D13: the count is advanced before anything else, and a refused
    /// advance starts nothing.** The remap material is not even reached.
    #[test]
    fn a_count_that_cannot_be_advanced_starts_nothing_and_reaches_nothing_after_it() {
        let range = [1, 2];
        let inventory = an_inventory(&range);
        let walk = a_walk(&[]);
        let attempts = [an_attempt(1, &range)];
        let ground = ground(&range, &inventory, &attempts, &walk, false);
        let mut the_material_was_read = false;
        let what = the_re_run(
            &ground,
            || Err("No space left on device (os error 28)".to_string()),
            || {
                the_material_was_read = true;
                Ok(())
            },
        );
        assert!(matches!(
            what,
            WhatTheReRunDoes::TheCountCouldNotBeAdvancedSoNothingStarts { .. }
        ));
        assert!(
            !the_material_was_read,
            "nothing runs at all where the count cannot advance"
        );
    }

    /// **D4: a re-run that cannot read the remap material must not render.**
    #[test]
    fn a_re_run_that_cannot_read_its_remap_material_fails_rather_than_rendering() {
        let range = [1, 2];
        let inventory = an_inventory(&range);
        let walk = a_walk(&[]);
        let attempts = [an_attempt(1, &range)];
        let ground = ground(&range, &inventory, &attempts, &walk, false);
        let what = the_re_run(&ground, || Ok(2), || Err("remaps/abc".to_string()));
        assert_eq!(
            what,
            WhatTheReRunDoes::ItMustNotRenderBecauseTheRemapMaterialCouldNotBeRead {
                at: "remaps/abc".to_string()
            }
        );
    }

    /// **`Job files and outputs` C9, carried rather than smoothed**: a walk
    /// that did not complete is a failed attempt and never a whole range
    /// quietly rendered again.
    #[test]
    fn a_walk_that_did_not_complete_fails_the_attempt() {
        let range = [1, 2];
        let inventory = an_inventory(&range);
        let walk = WhatTheWalkOfTheFolderIs::ItDidNotComplete;
        let attempts = [an_attempt(1, &range)];
        let ground = ground(&range, &inventory, &attempts, &walk, false);
        assert_eq!(
            the_re_run(&ground, || Ok(2), || Ok(())),
            WhatTheReRunDoes::TheWalkDidNotCompleteSoThisAttemptFails
        );
    }

    /// **S12's order held by the type**: the statement is taken after the
    /// disposition is decided, and an outcome that carries none says so rather
    /// than carrying four empty sets (S13, H8).
    #[test]
    fn the_ending_outcome_decides_first_and_an_absent_statement_is_absent() {
        let mut decided_first = None;
        let outcome = the_ending_outcome(DispositionThatEndsAJob::GivenUp, |disposition| {
            decided_first = Some(disposition);
            Some(CompletenessStatement {
                the_four_sets: vec![("absent".to_string(), vec!["f_0002.png".to_string()])],
            })
        });
        assert_eq!(decided_first, Some(DispositionThatEndsAJob::GivenUp));
        assert!(outcome.carries_a_completeness_statement());
        let none = the_ending_outcome(DispositionThatEndsAJob::LeftStopped, |_| None);
        assert!(
            !none.carries_a_completeness_statement(),
            "absent is not an empty statement, and H8 turns on the difference"
        );
    }

    /// **D1: reconciliation starts nothing whose durable state forbids it.**
    #[test]
    fn reconciliation_starts_nothing_held_nothing_rendering_and_nothing_marked() {
        use crate::line::{QueueState, RemovedBuildMark, SequenceKey};
        use smoozy_state::identities::{Identity, IdentityKind};
        let job = |key: u64| {
            let mut record = JobRecord::submitted(
                Identity::issue(IdentityKind::Job).expect("randomness"),
                None,
                "/ephemeral/smoozy/job-records/x".to_string(),
                &[],
            );
            record.sequence_key = Some(SequenceKey::read_back(key));
            record
        };
        let mut held = job(1);
        held.state = QueueState::Held {
            hold: Identity::issue(IdentityKind::Operation).expect("randomness"),
        };
        held.sequence_key = None;
        let mut rendering = job(2);
        rendering.state = QueueState::Rendering {
            operation: Identity::issue(IdentityKind::Operation).expect("randomness"),
        };
        let mut marked = job(3);
        marked.removed_build = Some(RemovedBuildMark {
            removal: Identity::issue(IdentityKind::Operation).expect("randomness"),
            the_answered_version_the_job_was_submitted_with: "4.5.0".to_string(),
        });
        let startable = job(4);
        let all = [held, rendering, marked, startable.clone()];
        let may = what_reconciliation_may_start(&all, &TheLineIsSettled(()));
        assert_eq!(may.len(), 1);
        assert_eq!(may[0].job, startable.job);
    }

    #[cfg(unix)]
    fn a_root(named: &str) -> std::path::PathBuf {
        let root = std::env::temp_dir().join(format!(
            "smoozy-queue-reconciliation-{}-{named}",
            std::process::id()
        ));
        std::fs::remove_dir_all(&root).ok();
        std::fs::create_dir_all(&root).expect("the place is made");
        root
    }

    #[cfg(unix)]
    fn a_waiting_record(key: u64) -> JobRecord {
        use crate::line::SequenceKey;
        use smoozy_state::identities::{Identity, IdentityKind};
        let job = Identity::issue(IdentityKind::Job).expect("randomness");
        let mut record = JobRecord::submitted(
            job.clone(),
            None,
            format!("/ephemeral/smoozy/job-records/{}", job.as_text()),
            &[],
        );
        record.sequence_key = Some(SequenceKey::read_back(key));
        record
    }

    /// **D14, and it is the whole point of L23's record: a supervisor killed
    /// part-way through a clear comes back, finishes it from the record, and
    /// only then may the admission point see anything.**
    ///
    /// **The fault is introduced where a crash puts it** — the record stands
    /// naming two identities, one of them already removed, and the line still
    /// holds the other. Without the completion the admission point would start
    /// a job the user asked to have cleared.
    #[cfg(unix)]
    #[test]
    fn an_outstanding_clear_is_finished_before_reconciliation_may_start_anything() {
        use crate::acting::commands::HowAnIdentityEnded;
        use smoozy_state::identities::{Identity, IdentityKind};
        let root = a_root("outstanding-clear");
        let mut store = TheQueueOnTheMachine::at(&root);
        let mut acts = TheActRecordsOnTheMachine::at(&root);
        let gone = a_waiting_record(10);
        let still_there = a_waiting_record(20);
        // The first removal landed and the machine died; the second record is
        // still in the line.
        store.write(&still_there).expect("written");
        let clear = Identity::issue(IdentityKind::Operation).expect("randomness");
        acts.record(
            &clear,
            &TheActsRecord::AClear {
                covered: vec![
                    (gone.job.as_text(), Some(HowAnIdentityEnded::Done)),
                    (still_there.job.as_text(), None),
                ],
            },
        )
        .expect("recorded");

        let (before, _) = store.every_record().expect("read");
        assert_eq!(before.len(), 1, "the half-cleared line still holds one");
        let mut told = Vec::new();
        let (settled, finished) =
            finish_the_outstanding_acts(&mut store, &mut acts, &mut |job| told.push(job.clone()))
                .expect("finished");
        assert_eq!(told, vec![still_there.job.clone()], "only what this pass removed");
        assert_eq!(finished, vec![clear.clone()]);
        let (after, _) = store.every_record().expect("read");
        assert!(
            what_reconciliation_may_start(&after, &settled).is_empty(),
            "the job the user asked to have cleared is not started"
        );
        assert_eq!(
            acts.read(&clear).expect("read"),
            None,
            "the record is cleared"
        );
        std::fs::remove_dir_all(&root).ok();
    }

    /// **D14: an outstanding reorder is finished from L24's record**, which is
    /// the only thing that still holds the order the user asked for — the
    /// rewrite writes back only the keys that moved, so a refusal part-way
    /// leaves an order nothing else could recompute.
    #[cfg(unix)]
    #[test]
    fn an_outstanding_reorder_is_finished_from_the_record_it_wrote_before_the_first_key_moved() {
        use smoozy_state::identities::{Identity, IdentityKind};
        let root = a_root("outstanding-reorder");
        let mut store = TheQueueOnTheMachine::at(&root);
        let mut acts = TheActRecordsOnTheMachine::at(&root);
        let first = a_waiting_record(10);
        let second = a_waiting_record(20);
        store.write(&first).expect("written");
        store.write(&second).expect("written");
        let act = Identity::issue(IdentityKind::Operation).expect("randomness");
        // The intended assignment puts the second job in front, and no key
        // has moved yet.
        acts.record(
            &act,
            &TheActsRecord::ARewrite {
                keys: vec![(second.job.as_text(), 1), (first.job.as_text(), 2)],
            },
        )
        .expect("recorded");

        let (settled, _) =
            finish_the_outstanding_acts(&mut store, &mut acts, &mut |_| {}).expect("finished");
        let (records, _) = store.every_record().expect("read");
        let line: Vec<smoozy_state::identities::Identity> =
            what_reconciliation_may_start(&records, &settled)
                .into_iter()
                .map(|record| record.job.clone())
                .collect();
        assert_eq!(line, vec![second.job.clone(), first.job.clone()]);
        assert_eq!(acts.read(&act).expect("read"), None);
        std::fs::remove_dir_all(&root).ok();
    }

    /// **D2: both states are named and neither is silence.**
    #[test]
    fn the_two_states_are_told_apart_and_neither_is_a_guess() {
        assert_eq!(
            why_a_job_expected_to_resume_did_neither(true),
            WhyAJobExpectedToResumeDidNeither::TheMachineIsGone
        );
        assert_eq!(
            why_a_job_expected_to_resume_did_neither(false),
            WhyAJobExpectedToResumeDidNeither::TheStateMountFilled
        );
    }

    /// **D10: nothing in this module reads a size.** The guard is a scan of
    /// this file's own working half, because the fault it catches is a line
    /// added later rather than one written today.
    ///
    /// **The scan stops at the test module.** A guard that read its own list
    /// of forbidden spellings would fail on itself and could never pass,
    /// which is a guard that tests nothing.
    #[test]
    fn nothing_in_this_module_reads_a_files_size() {
        let source = include_str!("reconciliation.rs");
        let working_half = source
            .split("#[cfg(test)]")
            .next()
            .expect("the file has a working half");
        let body: String = working_half
            .lines()
            .filter(|line| !line.trim_start().starts_with("//"))
            .collect::<Vec<_>>()
            .join(" ");
        for forbidden in ["metadata", "size", "len()"] {
            assert!(
                !body.contains(forbidden),
                "{forbidden} would be a size reading, which D10 forbids"
            );
        }
        const _: () = assert!(NOTHING_HERE_READS_A_VIDEO_FILES_SIZE);
        assert!(!WHAT_HOLDS_WITHOUT_FACT_44.is_empty());
    }
}

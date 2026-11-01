//! §10 — completeness: already there, the last frame, the exit status, and the
//! stop.
//!
//! **Everything in this file sits inside `State, identity and recovery` §1.3
//! rule 5's one carve-out, and inside its limit: it is a rule about what to
//! render, never a reading of state.** **C5: the result of the reckoning is never
//! written as state.** What is written is the attempt record's frame list, which
//! is a record Smoozy wrote and is therefore a state fact rather than an
//! exception to that rule.
//!
//! **C1: the reckoning is a step of the render worker, before the render is
//! launched, and it runs on the machine.** The spec settles that *Smoozy* does
//! the reckoning and hands Blender the frames it wants; on the machine the render
//! worker is Smoozy. **It needs no connection**, which is what makes the
//! automatic re-run after a machine comes back with nobody connected and Smoozy
//! closed possible at all.
//!
//! **C10: nothing here reads a file's presence or size for any purpose but this
//! rule.** The completeness statement is derived from the same one reading and is
//! not a second one.
//!
//! **C11: nothing compares a file's size against an expectation of what a whole
//! frame should weigh.** Smoozy has no idea what a whole frame of a given scene
//! should weigh, and *"opening a frame proves nothing"* — a part-written file
//! **opens** and can report the full picture size.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Rendering` A4.

use serde::{Deserialize, Serialize};

use crate::attempts::AttemptRecord;
use crate::inventory::{AnEntry, TheInventory, WhatTheInventoryIs};
use crate::mapping::SEPARATOR;
use crate::reading::AnAddedWritePlace;

/// What one entry of the walk is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WhatKindOfEntry {
    AFile,
    ADirectory,
    /// Anything else the walk returned. **Nothing is concluded from it** — it is
    /// not a file the inventory predicted and it is not a folder.
    SomethingElse,
}

/// One entry the walk returned.
///
/// **C7: one walk of the job output folder and everything under it, taken locally
/// by the render worker, returning each entry's path relative to the job output
/// folder, its kind and its size, compared against the expected-output
/// inventory.** *"It is a walk and not a listing because §6 deepens paths by
/// construction, so the inventory's predicted paths sit in subfolders for every
/// job with more than one destination, and a single directory's entries would
/// miss them."*
///
/// **C8: size comes with the walk and costs nothing extra, which is what makes
/// the rule buildable.** Observed at protocol level: a 15-entry listing cost
/// **zero per-file stats**, **5010 entries cost 52 round trips rather than
/// 5010**, and the command-channel walk returned kind, size, time and name for
/// 5010 entries in **about a quarter of a second**.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AWalkEntry {
    /// **Relative to the job output folder**, as bytes — a name a disk holds
    /// need not be text (N15, R4a).
    pub path_under_the_job_output_folder: Vec<u8>,
    pub kind: WhatKindOfEntry,
    pub size: u64,
}

/// **C9: a reading that did not complete is not a reading.**
///
/// Observed: **partial output is delivered before a connection dies**, and a
/// truncated listing and a genuinely short one were both four lines, *"separable
/// only by exit status"*. **So the reckoning proceeds only on a reading that
/// ended successfully, and a reading that did not is a failed attempt rather than
/// a folder that was found empty.**
///
/// **Held by the type.** There is no way to hand a partial walk to the reckoning
/// below: [`WhatTheWalkOfTheFolderIs::ItDidNotComplete`] carries no entries at
/// all, so nothing downstream can read one as an empty folder.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WhatTheWalkOfTheFolderIs {
    ItCompleted(Vec<AWalkEntry>),
    /// **A failed attempt**, and never a folder that was found empty.
    ItDidNotComplete,
}

/// **C2: Blender's own way of skipping frames is not used and cannot be.**
///
/// *"It looks at one path only — the one the regular output path would write — it
/// judges by bare existence so an empty file counts as done, and **it does not
/// reach file output nodes at all**. Handing over a list of frames instead works
/// in every version, and the list may mix single frames and stretches."*
pub const BLENDERS_OWN_SKIP_EXISTING_FRAMES_IS_NOT_USED: bool = true;

/// **C3: the list handed over is composed from the reckoning's own integers and
/// never from text.**
///
/// *"A malformed frame list **renders nothing at all and still reports
/// success**, so the list is never assembled from anything a user typed or a
/// document parsed."* **Held by the type**: every frame below is an `i64` and no
/// function in this file takes or produces a frame as text.
pub const THE_FRAME_LIST_IS_INTEGERS_AND_NEVER_TEXT: bool = true;

/// **C4: the desktop may perform the same reckoning while it is connected, and
/// what it produces is a reading it shows and never acts on.**
///
/// *"It reads the same inventory by the same rule. **The machine's reckoning is
/// the one the render uses.** `Rendering`'s carry-on figure is that module's and
/// is shown against this reading."*
pub const THE_MACHINES_RECKONING_IS_THE_ONE_THE_RENDER_USES: bool = true;

/// What the reckoning says.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WhatTheReckoningSays {
    /// The frames to hand over, **in the order the reckoning composed them**.
    RenderTheseFrames(Vec<i64>),
    /// **C9.** The walk did not complete, so there was no reading to reckon
    /// against and the attempt fails.
    TheWalkDidNotCompleteSoThisIsAFailedAttempt,
}

/// **C12: a frame counts as already there only when every file it should have
/// produced is present and none of them is empty.**
///
/// *Should have produced* is **the inventory's entries for that frame, one per
/// expected file**, which is I6's per-frame reckoning kept — and it is why the
/// count is worked out frame by frame rather than multiplied.
///
/// **C14: a frame that should produce nothing is already there**, since there is
/// nothing owed and nothing to wait for.
///
/// **C16: a folder existing is not enough, and the reckoning never reads one.**
/// It compares the inventory's predicted paths against the walk's paths. **A
/// folder with a file missing from inside it is a frame that is not done.**
///
/// **C15: the limit is stated rather than hidden — a file that is short but not
/// empty still counts as already there.** *"So a machine that went down can leave
/// a job reporting complete with a hole in it, where the file left behind happens
/// to hold some bytes. Where that bites, the two signals are the count coming up
/// short and what the machine printed."*
///
/// **Two unresolved facts decide how often this limit is met and neither will
/// ever be answered.** **Fact 28** — whether a partway fill of the machine's
/// mount leaves a truncated file at the final name, unposed on Linux and
/// permanently unresolved (§14 item 23). **Fact 44** — what reaches the disk when
/// a machine powers down abruptly, unposable on anything this project runs; **a
/// provider's forced hardware maintenance is the ordinary occasion for it rather
/// than a corner case**, which is what makes this limit a routine exposure rather
/// than a rare one.
///
/// **No rescue is built for it, deliberately.** A size floor, a trailing-byte
/// check or a re-render of anything that looks small would each be a safety net
/// nobody asked for over a fact that is permanently unresolved, and C13's rule is
/// the whole of what this document refuses to count.
pub fn the_frame_is_already_there(
    inventory: &TheInventory,
    frame: i64,
    walk: &[AWalkEntry],
    job_output_folder: &str,
) -> bool {
    let owed: Vec<&AnEntry> = inventory
        .entries
        .iter()
        .filter(|entry| entry.frame == Some(frame))
        .collect();
    // **C14.** Nothing owed and nothing to wait for.
    if owed.is_empty() {
        return true;
    }
    owed.iter().all(|entry| {
        let wanted = beneath(&entry.predicted_path, job_output_folder);
        walk.iter().any(|found| {
            found.kind == WhatKindOfEntry::AFile
                && found.path_under_the_job_output_folder == wanted
                // **C13: a file of zero bytes is never a finished frame,
                // wherever in the range it sits.**
                && found.size > 0
        })
    })
}

/// **The frames the completeness statement counts already there, in the order
/// handed in** — the set `Queue & History` P1b queues preview work over at a
/// job's ending. Added 2026-10-03, the 38's piece 3.
///
/// **It is C12's own test, read off the statement the ending step composed**
/// rather than off a second walk: the statement's present and nought-length
/// files are what that one walk found at every predicted path, so they are
/// handed to [`the_frame_is_already_there`] as the walk, and the test is not
/// written a second time. **A frame that owes nothing is left out**: C14
/// counts it as done, and there is no file at it to make a picture of.
pub fn the_frames_the_statement_counts_already_there(
    inventory: &TheInventory,
    statement: &TheCompletenessStatement,
    order: &[i64],
    job_output_folder: &str,
) -> Vec<i64> {
    let as_found = |path: &Vec<u8>, size: u64| AWalkEntry {
        path_under_the_job_output_folder: beneath(path, job_output_folder),
        kind: WhatKindOfEntry::AFile,
        size,
    };
    let walk: Vec<AWalkEntry> = statement
        .present
        .iter()
        .map(|path| as_found(path, 1))
        .chain(statement.present_at_zero_bytes.iter().map(|path| as_found(path, 0)))
        .collect();
    let mut out: Vec<i64> = Vec::new();
    for frame in order {
        let owes_a_file = inventory.entries.iter().any(|entry| entry.frame == Some(*frame));
        if owes_a_file
            && !out.contains(frame)
            && the_frame_is_already_there(inventory, *frame, &walk, job_output_folder)
        {
            out.push(*frame);
        }
    }
    out
}

/// **C13, stated where it is kept.**
///
/// *"This is not Smoozy judging whether a picture is whole; it is Smoozy refusing
/// to count nothing as something."* **Two observed routes put a nought-length
/// file at a frame's own name, and the first of them is not a lost write at
/// all.** A machine restarted mid-render left eleven whole frames and *"six
/// further files sat beside them at **zero bytes**"* — `f_0012.png` through
/// `f_0017.png`, *"the placeholders Blender writes ahead of itself"* — so *"the
/// output folder said seventeen and the truth was eleven"*. **Those six are frames
/// never written rather than writes that failed, and without this rule all six
/// count as already there.**
///
/// **Not every lost write leaves one, and this rule does not claim otherwise** —
/// on a full disk the file left behind is short and not empty, which is C15's
/// stated limit and C29's, and **no zero-byte test reaches it**.
pub const ZERO_BYTES_IS_NEVER_A_FINISHED_FRAME: u64 = 0;

/// **C18 and C18a: the previous attempt's frame list, found by when a list was
/// written and never by how an attempt ended.**
///
/// **C18a: where the immediately previous attempt record carries no frame list,
/// the reckoning walks back to the latest record that carries one.** *"Two
/// attempts write no list: one held at `Rendering` P1 step 5 writes a record with
/// no list, and one failing before P1 step 7 writes no record at all."*
///
/// **Without this a builder takes the immediately previous record literally,
/// finds no list, and falls to C20 — re-rendering and re-paying for everything
/// the last attempt already wrote.**
///
/// The records are given oldest first, which is the order their numbers put them
/// in.
pub fn the_latest_frame_list(records: &[AttemptRecord]) -> Option<&[i64]> {
    records
        .iter()
        .rev()
        .map(|record| record.frame_list_in_the_order_it_was_written.as_slice())
        .find(|list| !list.is_empty())
}

/// **C17: the last frame Smoozy asked for that counts as already there is
/// re-rendered anyway.**
///
/// *"It is the last one **in the order Smoozy asked for**, which Smoozy knows
/// because it wrote the list."*
///
/// **So the carried-on range opens *at* that frame and never after it.** That
/// frame is the one the render was part-way through writing when it stopped, so
/// **it is the one frame in the range that can hold a file which is present, is
/// not empty, and is not whole** — the shape C15 states as its limit and §14 item
/// 23 states as permanently unresolved. **A frame that may be truncated is never
/// counted as already there.** *"`Rendering` A4 composes the figure the user sees
/// for *carry on from where this stopped* and follows this clause; where the two
/// ever read differently this one governs, because the other ending is a broken
/// picture kept under a finished frame's name for good."*
///
/// **C19: it is not the highest frame number on disk, and the difference is
/// real.** *"A plain range is walked forward ascending, a stepped range moves by
/// the step and stops at or below the end, and a typed list of frames is rendered
/// in the order it was written, not sorted — so the highest-numbered frame present
/// need not be the last one finished."*
///
/// **C18: the reckoning walks that list backwards and the first frame in it that
/// counts as already there is the one re-rendered anyway.**
pub fn the_last_frame_asked_for_that_is_already_there(
    list: &[i64],
    is_already_there: impl Fn(i64) -> bool,
) -> Option<i64> {
    list.iter().rev().copied().find(|frame| is_already_there(*frame))
}

/// **C1, C6, C9, C12, C17, C18, C18a and C20 together: what to render.**
///
/// **C6: where any destination is video-shaped the reckoning does not run at all**
/// and the attempt covers the whole range (I22). *"A re-run renders the whole
/// range again, because there is no frame-shaped thing on disk to skip and a
/// partly written video is not something a second run can carry on from."*
///
/// **C20: where no previous attempt record can be read, the attempt covers the
/// whole range.** *"That is the safe reading and it is stated rather than left to
/// be discovered: with no record of what was asked for, there is no last frame to
/// name, and what an outright power loss leaves is not established."* The ground
/// is Fact 44, whose blocker is what is installed rather than what exists, and
/// **that route is with the Creator** — `Queue & History` D8 was brought to the
/// same ground.
///
/// **C21: this rule and the zero-byte rule are separate and both are needed.**
/// *"That one reads the machine; this one reads a record Smoozy made before the
/// render. A zero-byte file at exactly the name the finished frame would carry is
/// present, and presence is what the already-there test reads — so without C13 an
/// abruptly stopped job would treat a frame that was never written as finished,
/// and without C17 the frame in flight would pass as done."*
pub fn the_reckoning(
    the_whole_range: &[i64],
    inventory: &WhatTheInventoryIs,
    previous_attempts: &[AttemptRecord],
    walk: &WhatTheWalkOfTheFolderIs,
    job_output_folder: &str,
) -> WhatTheReckoningSays {
    let WhatTheWalkOfTheFolderIs::ItCompleted(entries) = walk else {
        // **C9.**
        return WhatTheReckoningSays::TheWalkDidNotCompleteSoThisIsAFailedAttempt;
    };
    let WhatTheInventoryIs::It(inventory) = inventory else {
        // **I9a: nothing reads absence as evidence** where the names could not
        // be predicted, so no frame is called already there and the whole range
        // is rendered.
        return WhatTheReckoningSays::RenderTheseFrames(the_whole_range.to_vec());
    };
    // **C6.**
    if inventory.a_destination_is_video_shaped {
        return WhatTheReckoningSays::RenderTheseFrames(the_whole_range.to_vec());
    }
    let already_there =
        |frame: i64| the_frame_is_already_there(inventory, frame, entries, job_output_folder);
    // **C20 and C18a.**
    let Some(list) = the_latest_frame_list(previous_attempts) else {
        return WhatTheReckoningSays::RenderTheseFrames(the_whole_range.to_vec());
    };
    // **C17.**
    let re_rendered_anyway = the_last_frame_asked_for_that_is_already_there(list, already_there);
    let frames = the_whole_range
        .iter()
        .copied()
        .filter(|frame| Some(*frame) == re_rendered_anyway || !already_there(*frame))
        .collect();
    WhatTheReckoningSays::RenderTheseFrames(frames)
}

/// **C12a: beside its frame list the reckoning says one further thing —
/// whether any frame it did not hand over was one an earlier attempt had
/// written.**
///
/// It is **a frame in the job's range that the reckoning did not hand over and
/// that owed at least one file** (I14). Such a frame counts as already there
/// only because its files were found present and not empty (C12), and nothing
/// but an earlier attempt writes into the job output folder — **so it is a frame
/// an earlier attempt wrote and this one keeps.**
///
/// **The *owed at least one file* half is what leaves C14's produce-nothing
/// frames out**: nothing was ever rendered for one of those, so keeping it is
/// not reuse.
///
/// **This reads nothing new.** It is derived from the list the reckoning
/// composed and the inventory it composed the list against; it takes no second
/// walk (C10 is untouched), no comparison of the two maps R13 keeps, and no
/// reading of any file's presence or size beyond the one C7 already took.
///
/// **A change can move part of an arrangement and leave the rest, and this
/// answers that without a rule of its own**: it asks whether *any* frame was
/// kept, so a job landing partly in each case reads as one where frames were
/// reused — true of the frames that were, and the half the entry's sentence is
/// about.
///
/// **It carries no count**, for the reason `Queue & History` N9a's last bullet
/// gives — a number of frames reused under the earlier arrangement would want a
/// walk of that arrangement, which no clause here takes.
///
/// **What it is for.** R14 routes the telling to `Queue & History` N9a, and this
/// is the fact that separates its two cases. The already-there test reads
/// presence and size and **nothing about which attempt or which file wrote a
/// frame** (C12), so a file changed in a way that moves no destination yields
/// the same map, the same predicted paths, and the earlier attempt's frames
/// found and kept.
pub fn any_frame_an_earlier_attempt_wrote_was_kept(
    the_whole_range: &[i64],
    inventory: &WhatTheInventoryIs,
    what_the_reckoning_said: &WhatTheReckoningSays,
) -> bool {
    let WhatTheReckoningSays::RenderTheseFrames(handed_over) = what_the_reckoning_said else {
        // **C9**: there was no reading to reckon against, so nothing was kept.
        return false;
    };
    let WhatTheInventoryIs::It(inventory) = inventory else {
        // **I9a**: the names could not be predicted, so no frame was called
        // already there and nothing was kept.
        return false;
    };
    let handed_over: std::collections::HashSet<i64> = handed_over.iter().copied().collect();
    the_whole_range.iter().any(|frame| {
        !handed_over.contains(frame)
            && inventory
                .entries
                .iter()
                .any(|entry| entry.frame == Some(*frame))
    })
}

/// The part of a predicted path that sits beneath the job output folder, which is
/// what the walk returns its entries as (C7).
fn beneath(predicted_path: &[u8], job_output_folder: &str) -> Vec<u8> {
    let folder = job_output_folder.as_bytes();
    let rest = predicted_path.strip_prefix(folder).unwrap_or(predicted_path);
    rest.strip_prefix(&[SEPARATOR]).unwrap_or(rest).to_vec()
}

// =============================================================================
// §10.5 — nothing here reads an exit status as *everything was written*.
// =============================================================================

/// **C22: the exit status is used for exactly one thing in this document:
/// separating *the job never started* from *the job ran*.**
///
/// *"A run that could not read the `.blend` at all ends at a failing status.
/// Nothing else about completeness is read off it."*
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WhatTheExitStatusSeparates {
    TheJobNeverStarted,
    TheJobRan,
}

/// **C22, performed, and it answers the one question a status can answer.**
///
/// **C24: exit 1 is not proof that nothing was written either.** *"A stray
/// argument taken as a file name ended a run at 1 with the frame already on
/// disk. So a failing exit status never removes a file, never marks a job's
/// outputs absent, and never skips the reckoning: the inventory-against-listing
/// comparison runs whatever the status was."*
pub fn what_the_exit_status_separates(
    the_blend_could_not_be_read_at_all: bool,
) -> WhatTheExitStatusSeparates {
    if the_blend_could_not_be_read_at_all {
        WhatTheExitStatusSeparates::TheJobNeverStarted
    } else {
        WhatTheExitStatusSeparates::TheJobRan
    }
}

/// **C24, held where it can be checked: a failing exit status never skips the
/// reckoning.**
///
/// [`the_reckoning`] takes no exit status at all, so there is nothing there to
/// branch on.
pub const THE_RECKONING_RUNS_WHATEVER_THE_STATUS_WAS: bool = true;

/// **C23: twenty endings report success, six of them ending at 0 with an empty
/// output folder.**
///
/// *"The six matter more than the count, and for anything deciding completeness
/// they are the load-bearing fact: a gate reading only the status would call all
/// six finished jobs with their frames written."* **Nothing in this clause's own
/// decision moves — it never rested on the number.**
///
/// **This document's own share is exact**: a node's write failing for any cause
/// tried — a name taken by a folder, a folder refusing writing, a broken path
/// template, a full disk — leaves the render running to its last frame at status
/// 0 with every other file for that frame written; **a node's folder that cannot
/// be made behaves the same way**, where the regular output path in the same
/// position stops at the first frame and exits 1 with nothing written; and a
/// failure raised inside a script that catches it leaves a successful exit status
/// behind, **which is why D18 forbids the material catching anything**.
pub const HOW_MANY_ENDINGS_REPORT_SUCCESS: u32 = 20;
/// **The six that matter more than the count.**
pub const HOW_MANY_OF_THEM_END_AT_ZERO_WITH_AN_EMPTY_OUTPUT_FOLDER: u32 = 6;

/// **C25: the rule behind the successful ending is exact and is not what it looks
/// like.**
///
/// *"A part-written frame never sets the exit status; what sets it is the **next**
/// write, the one that cannot be opened at all. So a one-frame job that runs out
/// of room prints its ordinary saved line and ends successfully over a picture
/// that is not whole."* **Nothing in this document treats a job's last frame
/// differently for that reason**; C17 already re-renders it and C13 already
/// refuses an empty one.
pub const NOTHING_TREATS_THE_LAST_FRAME_DIFFERENTLY_FOR_THE_EXIT_STATUS: bool = true;

/// **C26: the words Blender prints for a failed write differ by version and by
/// operating system — four wordings for one failure — so nothing here is decided
/// by reading them.**
///
/// *"That is also `Security and licence` §13 rule 5 from the other side: a tool's
/// output is captured whole or not at all and is never mined for a cause."*
/// **Held by there being no text of the machine's anywhere in this file.**
pub const NOTHING_HERE_READS_A_WORD_THE_MACHINE_PRINTED: bool = true;

// =============================================================================
// §10.6 — the completeness reading.
// =============================================================================

/// **C27a: for each of D20's four added write places, whether the walk returned
/// anything under that place's mapped folder.**
///
/// **D21f states when the reading is taken and D21g states what it does and does
/// not claim**, and neither is restated here. **Such a folder holds no inventory
/// entry**, so it belongs to none of C27's four sets, changes no count, and
/// leaves *the count coming up short* exactly the signal C27 says it is.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WhatAnAddedWritePlaceProduced {
    pub place: AnAddedWritePlace,
    /// **Presence under the folder, never a read-back of the setting (D21b-ii)
    /// and never a size or a name.**
    ///
    /// **Absent means *not readable for this place***, which is D21f's own
    /// withholding: the reading is sound only where the place's mapped folder is
    /// its own, and a place that went through the ordinary shared-run rule can
    /// land at the job output folder itself (P7) or in a folder it shares with a
    /// render destination (P17). In the first shape a *yes* is fixed whatever
    /// the cache wrote, in the second a *no* is. Added 2026-09-05, following the
    /// plan's repair out of a code round; **the field used to state an answer
    /// nobody had read.**
    ///
    /// **Nothing downstream branches on it** — D21g says a place that produced
    /// nothing is not by itself a fault, holds no job and moves no count — so
    /// this moves no file and changes no path.
    pub the_walk_returned_something_under_its_folder: Option<bool>,
}

/// **C27: the completeness statement for a job, and it names four sets.**
///
/// *"The inventory compared against one walk of the job output folder (C7): the
/// files expected — the inventory's entries, one per expected file — the files
/// present, the files absent, and the files present at zero bytes. **The count
/// coming up short is that statement, and it is the only completeness signal this
/// document produces.**"*
///
/// **C28: it names the absent and empty paths.** *"That is what makes the spec's
/// *only signal* usable at all: the only signal a lost write leaves anywhere is a
/// line naming the path in what the machine printed, and the completeness
/// statement is what says which paths to look for."* **Whether anything searches
/// the retained streams for them is `Queue & History`'s and `Desktop app and
/// delivery`'s**; the streams themselves and their retention are `Rented-server
/// runtime`'s under Row 286.
///
/// **C29: counting the files that arrive does not catch a write that ran out of
/// room, and this document does not claim it does.** *"Such a write leaves a file
/// at the right name, Blender prints its ordinary saved line for it, and Smoozy
/// does not tell it from a finished one. The completeness statement counts it as
/// present"*, which is the same standing C15 already gives it.
///
/// **C30: whether Blender says anything about the disk beside that saved line is
/// not firm**, and nothing here rests on it (§14 item 5).
///
/// **C31: nothing on the node records a failed write and Blender fires no hook
/// for a failed frame write.** *"So a program driving the render cannot be told,
/// only read"*, and this statement is the whole of what this document reads.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TheCompletenessStatement {
    /// **The files expected** — the inventory's entries, one per expected file.
    pub expected: usize,
    /// **The files present.**
    pub present: Vec<Vec<u8>>,
    /// **The files absent**, named.
    pub absent: Vec<Vec<u8>>,
    /// **The files present at zero bytes**, named.
    pub present_at_zero_bytes: Vec<Vec<u8>>,
    /// **C27a's further reading, and it is not about files owed.**
    pub what_the_added_write_places_produced: Vec<WhatAnAddedWritePlaceProduced>,
}

impl TheCompletenessStatement {
    /// **The count coming up short**, which C27 says is the only completeness
    /// signal this document produces.
    pub fn the_count_came_up_short(&self) -> bool {
        self.present.len() < self.expected
    }
}

/// **C27, C27a and C28, composed against the same one walk (C10).**
pub fn the_completeness_statement(
    inventory: &TheInventory,
    walk: &[AWalkEntry],
    job_output_folder: &str,
) -> TheCompletenessStatement {
    let mut present = Vec::new();
    let mut absent = Vec::new();
    let mut present_at_zero_bytes = Vec::new();
    for entry in &inventory.entries {
        let wanted = beneath(&entry.predicted_path, job_output_folder);
        match walk.iter().find(|found| {
            found.kind == WhatKindOfEntry::AFile && found.path_under_the_job_output_folder == wanted
        }) {
            None => absent.push(entry.predicted_path.clone()),
            Some(found) if found.size == 0 => {
                present_at_zero_bytes.push(entry.predicted_path.clone());
            }
            Some(_) => present.push(entry.predicted_path.clone()),
        }
    }
    let what_the_added_write_places_produced = inventory
        .added_write_place_folders
        .iter()
        .map(|folder| {
            let under = beneath(&folder.mapped_folder, job_output_folder);
            WhatAnAddedWritePlaceProduced {
                place: folder.place,
                // **Withheld where the folder is not the place's own** (D21f),
                // which the inventory worked out once off the map.
                the_walk_returned_something_under_its_folder: folder.the_folder_is_its_own.then(
                    || {
                        walk.iter().any(|found| {
                            found.path_under_the_job_output_folder != under
                                && found
                                    .path_under_the_job_output_folder
                                    .starts_with(&[under.as_slice(), &[SEPARATOR]].concat())
                        })
                    },
                ),
            }
        })
        .collect();
    TheCompletenessStatement {
        expected: inventory.entries.len(),
        present,
        absent,
        present_at_zero_bytes,
        what_the_added_write_places_produced,
    }
}

// =============================================================================
// §10.7 — why the stop must be the interrupt.
// =============================================================================

/// The three endings a render can be sent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HowARenderIsEnded {
    /// **C32: the one Smoozy sends.**
    TheInterrupt,
    ThePoliteRequestToClose,
    TheOutrightKill,
}

/// **C32: a render Smoozy ends is ended with the interrupt and never with the
/// outright kill, and never with the polite request to close.**
///
/// *"`Rendering` owns sending it under its Row 17; what this document owns is
/// **why**, and the why is what each ending leaves rather than how quickly it
/// arrives."*
pub const HOW_A_RENDER_SMOOZY_ENDS_IS_ENDED: HowARenderIsEnded = HowARenderIsEnded::TheInterrupt;

/// **C33: for a set of images.**
///
/// *"An **interrupt** has never left a short image — not in one run, by anybody,
/// in any version, on either machine, across hundreds of runs. A **polite request
/// to close** and an **outright kill** each left the last image short in all
/// eighty of their runs when the stop was aimed at the moment the file
/// appeared."*
///
/// **The zero-byte half is bounded to the batch that produced it.** That figure
/// is **one batch's** — the largest of three sets of runs — and *Frames 12* says
/// outright that **whether a terminate or a kill leaves a short image, and how
/// often, is not fixed**: three sets answered it three ways and none was chosen.
/// *Frames 13*'s own 160-run block over four image formats found **a spread of
/// sizes on both machines**. **So the zero-byte shape is not a general property of
/// a Linux short file and nothing here uses it as one.** **No further Blender
/// research will settle which batch governs** (§14 item 26).
///
/// **What the bound costs, exactly.** C13 catches a short image only where that
/// image is empty, and on the wider evidence a terminate or a kill can leave one
/// with bytes in it. **Such a file falls to C15's limit** — short but not empty
/// counts as already there — **and what stands between it and being skipped is
/// C17**, because a stop's short image is by construction the last frame asked for
/// that is present. **The decision in this clause does not move.**
pub const NOTHING_HERE_TREATS_A_SHORT_FILE_ON_LINUX_AS_CERTAINLY_EMPTY: bool = true;

/// **C34: for a video, the difference is total, and this is the half
/// `Rendering`'s row does not carry.**
///
/// *"An **interrupt** leaves a file holding the frames that were finished. A
/// **polite request to close** and an **outright kill** each leave the file at its
/// token size **with nothing in it**. So the gentler-sounding of the two endings
/// is the one that loses the whole video, and for a video-shaped destination the
/// choice of ending is the difference between the render and nothing."*
///
/// **There is no already-there machinery to recover it** (I22), which is why this
/// is stated as a constraint on the stop rather than as something the re-run
/// absorbs.
pub fn what_a_stop_leaves_of_a_video(how: HowARenderIsEnded) -> bool {
    matches!(how, HowARenderIsEnded::TheInterrupt)
}

/// **C35: a file output node writes before the frame's own file, every frame**,
/// so a stop landing between the two leaves the node's file short with nothing at
/// all at the frame's own path.
///
/// **The already-there test answers that without a rule of its own**: the frame
/// owes a file that is not there, and where the node's file is empty C13 refuses
/// it too.
///
/// **The machine bound, stated as I9a and D21b-ii already state theirs.** The
/// ordering is *Frames 13*'s and it carries a condition this clause did not: it
/// was **run on both machines for Blender 4.5, and on the Creator's Mac alone for
/// 5.0, 5.1 and 5.2**, coming back the same shape in all three. **Smoozy renders
/// on the machine that is not the Mac**, so for three of the four versions the
/// ordering is observed only on a machine Smoozy never uses. **It will not be
/// widened** (§14 item 26).
///
/// **What breaks if the order is the other way round on rented Linux in 5.0, 5.1
/// or 5.2: nothing decided here.** A stop landing between the two files leaves one
/// of the pair absent whichever is written first, the already-there test reads the
/// frame as owing a file that is not there in both directions, and C17 re-renders
/// that frame anyway. **This clause is a reason rather than a mechanism**, and
/// nothing in this file branches on the order.
pub const NOTHING_HERE_BRANCHES_ON_WHICH_OF_A_FRAMES_TWO_FILES_IS_WRITTEN_FIRST: bool = true;

/// **C36: what must be signalled is the render's own process and not a shell
/// around it.**
///
/// *"A render had to be started detached to survive the connection at all, so the
/// thing that must be reached is by construction not the thing the connection
/// started."* **The recorded execution group and its validation are `Rented-server
/// runtime`'s and `State, identity and recovery` §5.2's**, cited here because this
/// document's reason for the interrupt is worthless if it reaches the wrong
/// process. **Nothing here composes a second way to find that process.**
pub const THE_EXECUTION_GROUP_THAT_IS_SIGNALLED_IS_ANOTHER_DOCUMENTS: bool = true;

/// **C37: nothing here matches an ending on its number.**
///
/// *"The number a polite terminate ends with is recorded as `-15` by one topic and
/// `143` by three others, and the joined answer says anything matching an ending
/// on its number must accept both."* **Held by there being no number of an ending
/// anywhere in this file.**
pub const NOTHING_HERE_MATCHES_AN_ENDING_ON_ITS_NUMBER: bool = true;

/// **C38: where a machine goes down of its own accord nothing is sent at all**,
/// and that case is the one C17 was always written for.
pub const WHERE_A_MACHINE_GOES_DOWN_NOTHING_IS_SENT: bool = true;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inventory::compose_the_inventory;
    use crate::inventory::tests::{a_frame, a_node_destination, a_reading, mapped, JOB_FOLDER};
    use crate::reading::TheVersionInHand;
    use smoozy_framing::source_reading::SourceReading;

    const VERSION: TheVersionInHand = TheVersionInHand { major: 4, minor: 5 };

    fn a_file(path: &str, size: u64) -> AWalkEntry {
        AWalkEntry {
            path_under_the_job_output_folder: path.as_bytes().to_vec(),
            kind: WhatKindOfEntry::AFile,
            size,
        }
    }

    fn a_record(number: u32, frames: Vec<i64>) -> AttemptRecord {
        AttemptRecord {
            number,
            frame_list_in_the_order_it_was_written: frames,
            map_in_force: crate::attempts::TheMapInForce::NoneWasComposedForThisAttempt,
            file_identity_at_this_attempt: SourceReading {
                size: 4096,
                inode: "912".to_string(),
                modification_time: 1_756_800_000,
                change_time: 1_756_800_001,
            },
            the_file_changed_before_this_attempt: false,
            a_frame_an_earlier_attempt_wrote_was_kept: false,
        }
    }

    /// A job with one node destination writing one file a frame, over frames 10
    /// to 17 — the shape the rented machine left when it was restarted
    /// mid-render.
    fn an_inventory(frames: &[i64]) -> (WhatTheInventoryIs, Vec<i64>) {
        let reading = a_reading(
            vec![a_node_destination("Beauty", "f_", "/home/a/out", None)],
            frames.iter().map(|frame| a_frame(*frame, &["VL"], None)).collect(),
            VERSION,
        );
        let map = mapped(&reading, VERSION);
        (
            compose_the_inventory(&reading, &map),
            frames.to_vec(),
        )
    }

    fn it(inventory: &WhatTheInventoryIs) -> &TheInventory {
        match inventory {
            WhatTheInventoryIs::It(it) => it,
            other => panic!("withheld: {other:?}"),
        }
    }

    #[test]
    fn the_output_folder_said_seventeen_and_the_truth_was_eleven() {
        // **C13, against the observation it was written for.** A machine
        // restarted mid-render left eleven whole frames and six further files at
        // zero bytes — the placeholders Blender writes ahead of itself. Without
        // this rule all six count as already there.
        let (inventory, range) = an_inventory(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17]);
        let mut walk: Vec<AWalkEntry> = (1..=11)
            .map(|frame| a_file(&format!("f_{frame:04}.png"), 4096))
            .collect();
        walk.extend((12..=17).map(|frame| a_file(&format!("f_{frame:04}.png"), 0)));
        for frame in 1..=11 {
            assert!(the_frame_is_already_there(it(&inventory), frame, &walk, JOB_FOLDER));
        }
        for frame in 12..=17 {
            assert!(
                !the_frame_is_already_there(it(&inventory), frame, &walk, JOB_FOLDER),
                "frame {frame} counted as already there at zero bytes"
            );
        }
        // And the completeness statement names the six.
        let statement = the_completeness_statement(it(&inventory), &walk, JOB_FOLDER);
        assert_eq!(statement.expected, 17);
        assert_eq!(statement.present.len(), 11);
        assert_eq!(statement.present_at_zero_bytes.len(), 6);
        assert!(statement.the_count_came_up_short());
        let _ = range;
    }

    /// **`Queue & History` P1b's set, read off the statement**: the eleven
    /// whole frames and not the six at nought bytes, in the order handed in,
    /// each once — the same answer C12's test gives against the walk itself.
    #[test]
    fn the_statement_counts_the_same_frames_already_there_as_the_walk_did() {
        let (inventory, _) = an_inventory(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17]);
        let mut walk: Vec<AWalkEntry> = (1..=11)
            .map(|frame| a_file(&format!("f_{frame:04}.png"), 4096))
            .collect();
        walk.extend((12..=17).map(|frame| a_file(&format!("f_{frame:04}.png"), 0)));
        let statement = the_completeness_statement(it(&inventory), &walk, JOB_FOLDER);
        let order: Vec<i64> = vec![17, 3, 3, 1, 2, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 99];
        let counted =
            the_frames_the_statement_counts_already_there(it(&inventory), &statement, &order, JOB_FOLDER);
        assert_eq!(counted, vec![3, 1, 2, 4, 5, 6, 7, 8, 9, 10, 11]);
        for frame in &counted {
            assert!(the_frame_is_already_there(it(&inventory), *frame, &walk, JOB_FOLDER));
        }
        // **Frame 99 owes nothing**, so C14 would count it done; it is left
        // out, there being no file at it to make a picture of.
        assert!(the_frame_is_already_there(it(&inventory), 99, &walk, JOB_FOLDER));
    }

    #[test]
    fn a_frame_owing_two_files_with_one_present_is_not_already_there() {
        // **C12: every file it should have produced is present.** A frame is
        // one file in the ordinary case, so a rule that asked whether *any* of
        // the frame's files was present would pass every test written on such a
        // job — and would skip a frame whose node file was written and whose own
        // frame file was not, which C35 says is exactly what a stop between the
        // two leaves.
        let reading = a_reading(
            vec![
                a_node_destination("Beauty", "f_", "/home/a/out", None),
                a_node_destination("Passes", "p_", "/home/a/passes", None),
            ],
            vec![a_frame(1, &["VL"], None)],
            VERSION,
        );
        let map = mapped(&reading, VERSION);
        let inventory = compose_the_inventory(&reading, &map);
        assert_eq!(it(&inventory).the_count(), 2);
        let only_one = vec![a_file("out/f_0001.png", 4096)];
        assert!(!the_frame_is_already_there(
            it(&inventory),
            1,
            &only_one,
            JOB_FOLDER
        ));
        let both = vec![
            a_file("out/f_0001.png", 4096),
            a_file("passes/p_0001.png", 4096),
        ];
        assert!(the_frame_is_already_there(it(&inventory), 1, &both, JOB_FOLDER));
    }

    #[test]
    fn c15s_limit_is_built_as_written_and_no_rescue_is_built_for_it() {
        // **A file short but not empty still counts as already there.** Two
        // permanently unresolved facts decide how often that bites and neither
        // will ever be answered, so the limit is stated rather than patched.
        let (inventory, _) = an_inventory(&[1]);
        let walk = vec![a_file("f_0001.png", 1)];
        assert!(the_frame_is_already_there(it(&inventory), 1, &walk, JOB_FOLDER));
    }

    #[test]
    fn the_last_frame_asked_for_is_re_rendered_anyway_and_it_is_not_the_highest_number() {
        // C17 and C19: a typed list of frames is rendered in the order it was
        // written, not sorted, so the highest-numbered frame present need not be
        // the last one finished.
        let asked_for = [10, 4, 7, 1];
        let already_there = |frame: i64| matches!(frame, 10 | 4 | 7);
        assert_eq!(
            the_last_frame_asked_for_that_is_already_there(&asked_for, already_there),
            Some(7)
        );
        // Read as the highest number on disk it would have been 10, and frame 7
        // — the one in flight — would have passed as done.
        assert_ne!(
            the_last_frame_asked_for_that_is_already_there(&asked_for, already_there),
            Some(10)
        );
    }

    #[test]
    fn the_carried_on_range_opens_at_that_frame_and_never_after_it() {
        // C17, through the whole reckoning.
        let (inventory, range) = an_inventory(&[1, 2, 3, 4, 5]);
        let walk: Vec<AWalkEntry> = (1..=3)
            .map(|frame| a_file(&format!("f_{frame:04}.png"), 4096))
            .collect();
        let records = vec![a_record(1, vec![1, 2, 3, 4, 5])];
        assert_eq!(
            the_reckoning(
                &range,
                &inventory,
                &records,
                &WhatTheWalkOfTheFolderIs::ItCompleted(walk),
                JOB_FOLDER
            ),
            WhatTheReckoningSays::RenderTheseFrames(vec![3, 4, 5])
        );
    }

    #[test]
    fn c18a_walks_back_to_the_latest_record_that_carries_a_list() {
        // **Without this a builder takes the immediately previous record
        // literally, finds no list, and falls to C20 — re-rendering and
        // re-paying for everything the last attempt already wrote.**
        let records = vec![
            a_record(1, vec![1, 2, 3, 4, 5]),
            // One held at `Rendering` P1 step 5 writes a record with no list.
            a_record(2, Vec::new()),
        ];
        assert_eq!(
            the_latest_frame_list(&records),
            Some([1, 2, 3, 4, 5].as_slice())
        );

        let (inventory, range) = an_inventory(&[1, 2, 3, 4, 5]);
        let walk: Vec<AWalkEntry> = (1..=3)
            .map(|frame| a_file(&format!("f_{frame:04}.png"), 4096))
            .collect();
        assert_eq!(
            the_reckoning(
                &range,
                &inventory,
                &records,
                &WhatTheWalkOfTheFolderIs::ItCompleted(walk),
                JOB_FOLDER
            ),
            // Not the whole range, which is what falling to C20 would have given.
            WhatTheReckoningSays::RenderTheseFrames(vec![3, 4, 5])
        );
    }

    #[test]
    fn where_no_record_carries_a_list_at_all_the_attempt_covers_the_whole_range() {
        // C20, and it is the safe reading rather than a discovered one.
        let (inventory, range) = an_inventory(&[1, 2, 3]);
        let walk = vec![a_file("f_0001.png", 4096), a_file("f_0002.png", 4096)];
        for records in [Vec::new(), vec![a_record(1, Vec::new())]] {
            assert_eq!(
                the_reckoning(
                    &range,
                    &inventory,
                    &records,
                    &WhatTheWalkOfTheFolderIs::ItCompleted(walk.clone()),
                    JOB_FOLDER
                ),
                WhatTheReckoningSays::RenderTheseFrames(vec![1, 2, 3])
            );
        }
    }

    #[test]
    fn a_walk_that_did_not_complete_is_a_failed_attempt_and_never_an_empty_folder() {
        // **C9.** Partial output is delivered before a connection dies, and a
        // truncated listing and a genuinely short one were both four lines,
        // separable only by exit status. Read as an empty folder this would
        // re-render and re-pay for the whole job.
        let (inventory, range) = an_inventory(&[1, 2, 3]);
        assert_eq!(
            the_reckoning(
                &range,
                &inventory,
                &[a_record(1, vec![1, 2, 3])],
                &WhatTheWalkOfTheFolderIs::ItDidNotComplete,
                JOB_FOLDER
            ),
            WhatTheReckoningSays::TheWalkDidNotCompleteSoThisIsAFailedAttempt
        );
    }

    #[test]
    fn a_frame_that_should_produce_nothing_is_already_there() {
        // C14, and its second half: where every frame of a job produces nothing
        // the job is complete and the re-run has nothing to do.
        let (inventory, range) = an_inventory(&[]);
        assert!(the_frame_is_already_there(it(&inventory), 1, &[], JOB_FOLDER));
        assert_eq!(it(&inventory).the_count(), 0);
        let _ = range;
    }

    #[test]
    fn a_folder_existing_is_not_enough_and_a_directory_entry_is_never_the_file() {
        // C16: a folder with a file missing from inside it is a frame that is
        // not done.
        let (inventory, _) = an_inventory(&[1]);
        let only_a_folder = vec![AWalkEntry {
            path_under_the_job_output_folder: b"f_0001.png".to_vec(),
            kind: WhatKindOfEntry::ADirectory,
            size: 4096,
        }];
        assert!(!the_frame_is_already_there(
            it(&inventory),
            1,
            &only_a_folder,
            JOB_FOLDER
        ));
    }

    #[test]
    fn a_video_shaped_job_renders_the_whole_range_again() {
        // C6 and I22: there is no frame-shaped thing on disk to skip and a partly
        // written video is not something a second run can carry on from.
        let mut regular = crate::reading::tests::a_destination(
            crate::reading::DestinationKind::TheRegularOutputPath,
            "/home/a/out/whole",
            crate::reading::WhereItComesFrom::TheScenesOutputPath,
        );
        regular.format = Some(crate::reading::Format {
            as_the_file_records_it: crate::inventory::tests::text("FFMPEG"),
            the_ending_it_writes: Some(crate::inventory::tests::text(".mp4")),
            is_video_shaped: true,
        });
        let reading = a_reading(
            vec![regular],
            vec![a_frame(1, &["VL"], Some("/home/a/out/whole.mp4"))],
            VERSION,
        );
        let map = mapped(&reading, VERSION);
        let inventory = compose_the_inventory(&reading, &map);
        assert_eq!(
            the_reckoning(
                &[1, 2, 3],
                &inventory,
                &[a_record(1, vec![1, 2, 3])],
                &WhatTheWalkOfTheFolderIs::ItCompleted(vec![a_file("whole.mp4", 4096)]),
                JOB_FOLDER
            ),
            WhatTheReckoningSays::RenderTheseFrames(vec![1, 2, 3])
        );
    }

    #[test]
    fn c27a_says_whether_an_added_write_place_produced_anything_and_moves_no_count() {
        // D21f and D21g: presence under the folder, never a read-back of the
        // setting, and it is not a fault and not in any count.
        let node = a_node_destination("Beauty", "f_", "/home/a/out", None);
        let cache = crate::reading::Destination {
            kind: crate::reading::DestinationKind::AnAddedWritePlace(
                AnAddedWritePlace::ThePointCache,
            ),
            recorded_location: None,
            format: None,
            origin: crate::reading::WhereItComesFrom::AnAddedWritePlaceNamed(
                AnAddedWritePlace::ThePointCache,
            ),
            scene: crate::reading::WhichScene::TheJobsOwnScene,
            the_view_layers_that_feed_it: Vec::new(),
            the_location_with_its_folder_replaced: None,
        };
        let reading = a_reading(vec![node, cache], vec![a_frame(1, &["VL"], None)], VERSION);
        let map = mapped(&reading, VERSION);
        let inventory = compose_the_inventory(&reading, &map);

        let nothing_under_it = vec![a_file("f_0001.png", 4096)];
        let statement = the_completeness_statement(it(&inventory), &nothing_under_it, JOB_FOLDER);
        assert_eq!(statement.expected, 1);
        assert!(!statement.the_count_came_up_short());
        assert_eq!(statement.what_the_added_write_places_produced.len(), 1);
        assert_eq!(
            statement.what_the_added_write_places_produced[0]
                .the_walk_returned_something_under_its_folder,
            Some(false)
        );

        let with_a_cache = vec![
            a_file("f_0001.png", 4096),
            a_file("pointcache/sim_000001_00.bphys", 128),
        ];
        let second = the_completeness_statement(it(&inventory), &with_a_cache, JOB_FOLDER);
        assert_eq!(
            second.what_the_added_write_places_produced[0]
                .the_walk_returned_something_under_its_folder,
            Some(true)
        );
        // It moves no count either way.
        assert_eq!(second.expected, statement.expected);
    }

    #[test]
    fn c27a_withholds_the_reading_where_the_places_folder_is_not_its_own() {
        // D21f, as repaired 2026-09-05. The walk returns every entry under the
        // folder, so where a place's folder is shared the reading is a fixed
        // answer whatever the cache wrote. **From 2026-10-04 every place takes
        // its own named folder (P7a widened)**, so the one shape left is a
        // render destination whose folder, after the shared run, is that same
        // name: here the artist's own `…/out/pointcache` lands at
        // `<job>/pointcache`, which is the point cache's folder too.
        let node = a_node_destination("Beauty", "f_", "/home/a/out/pointcache", None);
        let other = a_node_destination("Mist", "m_", "/home/a/out/mist", None);
        let cache = crate::reading::Destination {
            kind: crate::reading::DestinationKind::AnAddedWritePlace(
                AnAddedWritePlace::ThePointCache,
            ),
            recorded_location: Some(crate::reading::tests::a_location("/home/a/sim")),
            format: None,
            origin: crate::reading::WhereItComesFrom::AnAddedWritePlaceNamed(
                AnAddedWritePlace::ThePointCache,
            ),
            scene: crate::reading::WhichScene::TheJobsOwnScene,
            the_view_layers_that_feed_it: Vec::new(),
            the_location_with_its_folder_replaced: None,
        };
        let reading = a_reading(vec![node, other, cache], vec![a_frame(1, &["VL"], None)], VERSION);
        let map = mapped(&reading, VERSION);
        let inventory = compose_the_inventory(&reading, &map);
        assert!(!it(&inventory).added_write_place_folders[0].the_folder_is_its_own);

        // The render wrote its own frames and the cache wrote nothing. The old
        // reading would have said the place produced something, off a file that
        // is not its.
        let the_renders_own_file = vec![
            a_file("pointcache/f_0001.png", 4096),
            a_file("mist/m_0001.png", 4096),
        ];
        let statement =
            the_completeness_statement(it(&inventory), &the_renders_own_file, JOB_FOLDER);
        assert_eq!(
            statement.what_the_added_write_places_produced[0]
                .the_walk_returned_something_under_its_folder,
            None,
            "the reading is withheld, not answered"
        );
        // And it moves no count, by D21g.
        assert_eq!(statement.expected, 2);
        assert!(!statement.the_count_came_up_short());
    }

    #[test]
    fn the_exit_status_separates_one_thing_and_never_skips_the_reckoning() {
        // C22 and C24.
        assert_eq!(
            what_the_exit_status_separates(true),
            WhatTheExitStatusSeparates::TheJobNeverStarted
        );
        assert_eq!(
            what_the_exit_status_separates(false),
            WhatTheExitStatusSeparates::TheJobRan
        );
        assert_eq!(HOW_MANY_ENDINGS_REPORT_SUCCESS, 20);
        assert_eq!(HOW_MANY_OF_THEM_END_AT_ZERO_WITH_AN_EMPTY_OUTPUT_FOLDER, 6);
    }

    #[test]
    fn the_stop_is_the_interrupt_and_the_gentler_ending_loses_the_whole_video() {
        // C32 and C34.
        assert_eq!(HOW_A_RENDER_SMOOZY_ENDS_IS_ENDED, HowARenderIsEnded::TheInterrupt);
        assert!(what_a_stop_leaves_of_a_video(HowARenderIsEnded::TheInterrupt));
        assert!(!what_a_stop_leaves_of_a_video(
            HowARenderIsEnded::ThePoliteRequestToClose
        ));
        assert!(!what_a_stop_leaves_of_a_video(HowARenderIsEnded::TheOutrightKill));
    }

    /// **C12a: whether any frame the reckoning did not hand over was one an
    /// earlier attempt had written.**
    ///
    /// This is the fact that separates `Queue & History` N9a's two cases, and
    /// it exists because C12 reads presence and size and **nothing about which
    /// attempt or which file wrote a frame** — so a file changed in a way that
    /// moves no destination yields the same map, the same predicted paths, and
    /// the earlier attempt's frames found and kept.
    #[test]
    fn c12a_says_a_frame_an_earlier_attempt_wrote_was_kept_and_c14s_frames_are_not_reuse() {
        let (inventory, range) = an_inventory(&[1, 2, 3, 4, 5]);
        let records = vec![a_record(1, vec![1, 2, 3, 4, 5])];

        // **The arrangement did not move**: frames 1 and 2 are found at exactly
        // the paths this attempt predicts. C17 re-renders frame 2, the last one
        // asked for that is already there, and frame 1 is kept.
        let found = vec![a_file("f_0001.png", 4096), a_file("f_0002.png", 4096)];
        let said = the_reckoning(
            &range,
            &inventory,
            &records,
            &WhatTheWalkOfTheFolderIs::ItCompleted(found),
            JOB_FOLDER,
        );
        assert_eq!(said, WhatTheReckoningSays::RenderTheseFrames(vec![2, 3, 4, 5]));
        assert!(any_frame_an_earlier_attempt_wrote_was_kept(
            &range, &inventory, &said
        ));

        // **The arrangement moved**: nothing is found where this attempt looks,
        // so the whole range is rendered again and nothing was kept.
        let said = the_reckoning(
            &range,
            &inventory,
            &records,
            &WhatTheWalkOfTheFolderIs::ItCompleted(Vec::new()),
            JOB_FOLDER,
        );
        assert_eq!(
            said,
            WhatTheReckoningSays::RenderTheseFrames(vec![1, 2, 3, 4, 5])
        );
        assert!(!any_frame_an_earlier_attempt_wrote_was_kept(
            &range, &inventory, &said
        ));

        // **C9 and I9a hand over nothing that was kept either.**
        assert!(!any_frame_an_earlier_attempt_wrote_was_kept(
            &range,
            &inventory,
            &WhatTheReckoningSays::TheWalkDidNotCompleteSoThisIsAFailedAttempt
        ));
        assert!(!any_frame_an_earlier_attempt_wrote_was_kept(
            &range,
            &WhatTheInventoryIs::TheFrameSetIsNotPredictable(
                crate::inventory::WhyTheFrameSetIsNotPredictable::TheSceneWritesMoreThanOneViewSeparately
            ),
            &said
        ));
    }

    /// **C14's produce-nothing frames are not reuse**, and the *owed at least
    /// one file* half of C12a is the whole of what keeps them out.
    #[test]
    fn c12a_does_not_read_a_frame_that_owed_nothing_as_a_frame_an_earlier_attempt_wrote() {
        // Frame 2 renders no view layer, so it owes nothing and C14 counts it
        // already there — but nothing was ever written for it.
        let reading = a_reading(
            vec![a_node_destination("Beauty", "f_", "/home/a/out", None)],
            vec![
                a_frame(1, &["VL"], None),
                a_frame(2, &[], None),
                a_frame(3, &["VL"], None),
            ],
            VERSION,
        );
        let map = mapped(&reading, VERSION);
        let inventory = compose_the_inventory(&reading, &map);
        assert!(!it(&inventory).entries.iter().any(|e| e.frame == Some(2)));

        let range = vec![1, 2, 3];
        // **C19: a typed list is rendered in the order it was written**, so
        // C17's last-frame-asked-for is frame 3 and frame 2 is the only frame
        // the reckoning does not hand over.
        let records = vec![a_record(1, vec![2, 1, 3])];
        let said = the_reckoning(
            &range,
            &inventory,
            &records,
            &WhatTheWalkOfTheFolderIs::ItCompleted(vec![a_file("f_0003.png", 4096)]),
            JOB_FOLDER,
        );
        assert_eq!(said, WhatTheReckoningSays::RenderTheseFrames(vec![1, 3]));
        // Without the *owed at least one file* half, frame 2 would read as a
        // frame an earlier attempt wrote, and the entry would tell the user
        // work was reused that was never rendered.
        assert!(!any_frame_an_earlier_attempt_wrote_was_kept(
            &range, &inventory, &said
        ));
    }
}

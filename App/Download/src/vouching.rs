//! Telling a finished file from one still being written — `Download` §4, J1 to
//! J10, Rows 211 and 203.
//!
//! **Constraint, and it is a narrowing rather than a promise:** *"Auto-download
//! is not a check run on the user's behalf and is not written as one — it
//! fetches what looks finished, and says what it could not vouch for"*
//! (Creator-stated, 2026-08-12). **Nothing here claims the arrangement can
//! always tell.**
//!
//! # What is closed off, before anything is built
//!
//! - **Opening the file** — rejected by the Creator, 2026-08-12, and a stopped
//!   TIFF opens and reports the full picture size anyway.
//! - **Changing anything on the machine** — the spec's own rule.
//! - **Keying on a size, with exactly one exception**: a file of zero bytes is
//!   never a finished frame. **No other reading of a size decides anything in
//!   this document**, so no plausibility floor, no minimum-bytes-for-this-format
//!   test and no has-it-stopped-growing check follows from the zero-byte rule.
//! - **Watching the writing process** — *"held by nobody"* across a user
//!   boundary from 18 MB to 1.6 GB, the same answer it gives for a finished
//!   file, with exit 0 and no rows.
//! - **Size stability** — a frozen transfer sat at 17,233,920 bytes for 11.1
//!   seconds while unfinished.
//!
//! **Markers used below that belong to another document, each with the
//! document it belongs to.** `Job files and outputs` I14, I20, I21.
//! `Rendering` K18, K19, K21, K22. `Server protocol and transfers` S13, S25.
//! `Queue & History` C1.
//!
//! **A marker this plan also defines is deliberately not listed above,
//! whichever document is meant, and is written out with its document at every
//! site instead** — the header list cannot tell the two apart, and a bare one
//! would resolve to a real and wrong clause. `Job files and outputs` M8,
//! `Queue & History` E2, `File Browser` W5 and `Rented-server runtime` M22 are
//! the four that arise here.

use smoozy_queue_history::ending::Ending;
use smoozy_rendering::which_frames::WhatTheOutcomeCarries;

/// **J1's third supplied fact, and the one exception to §4's size closure: a
/// file of zero bytes is never a finished frame and is never brought down.**
///
/// Creator-stated, 2026-08-12, and the spec's own rule. **It is not widened
/// anywhere**: a size above zero decides nothing at all.
pub fn a_zero_byte_file_is_never_a_finished_frame(size: u64) -> bool {
    size == 0
}

/// **J1's first supplied fact: the inventory names every file the job is
/// expected to leave, with its frame** (`Job files and outputs` I14) — so
/// every walk entry that matches an inventory entry has a frame.
pub const EVERY_WALK_ENTRY_THAT_MATCHES_AN_INVENTORY_ENTRY_HAS_A_FRAME: bool = true;

/// **J1's second supplied fact: the render works through its frames in a known
/// order, and writes each frame's files as that frame finishes.**
///
/// A plain range ascending, a stepped range by the step, a typed list in the
/// order written; a node writes its files as each frame finishes, never in one
/// lot at the end; and with placeholders on, *"the empty file for the next
/// frame appears at the moment the previous frame's picture is complete"*.
pub const EACH_FRAMES_FILES_ARE_WRITTEN_AS_THAT_FRAME_FINISHES: bool = true;

/// **What both mechanisms rest on: the first write event belonging to a later
/// frame comes after every write of the earlier frame is complete, so a later
/// frame's activity is evidence about an earlier frame.**
///
/// *Writing 14* asks this exact question and returns a verdict: *"Every write
/// belonging to one frame is finished before any write belonging to the next
/// frame begins."* — settled by test in all four versions, on the Creator's
/// Mac and on a rented Linux machine, with no window and again with a window.
///
/// **Two instruments produced it and that question insists on the split:**
/// *"Two instruments were used and they are two separate claims. Do not let
/// one carry the other."* The folder was watched at intervals on both
/// machines, with sixty-three separate readings falling inside one write on
/// the Mac and thirty-one on Linux — **and that half is what settles macOS**.
/// And every file call was traced thread by thread on the rented Linux machine
/// alone, which *"settles Linux and says nothing whatever about macOS"*.
///
/// **The residue is macOS alone and it is permanently unresolved**, not
/// pending: what would close it is a thread-by-thread trace, which *"has no
/// equivalent on the Creator's Mac that does not need a system setting
/// changed"*, and under the Creator's ruling of 2026-08-31 there is no further
/// Blender research. **No space is left in this crate or in its record for a
/// trace that is not coming** (§15 item 17).
///
/// **Where it fails, both mechanisms judge in the fetching-*wrong* direction**
/// — a frame judged finished with a write to it still pending is fetched torn,
/// recorded *fetched* and never fetched again. That is the one place in this
/// document where a wrong judgement fetches wrong rather than late.
pub const A_LATER_FRAMES_ACTIVITY_IS_EVIDENCE_ABOUT_AN_EARLIER_FRAME: bool = true;

/// **The neighbouring result the trace settles outright, which this document
/// leans on elsewhere**: *"there is no rename call anywhere in it"*, and every
/// file is written under its final name from the first byte — **so no
/// temporary name is ever mistakable for a finished frame.**
pub const EVERY_FILE_IS_WRITTEN_UNDER_ITS_FINAL_NAME_FROM_THE_FIRST_BYTE: bool = true;

/// **J2: the attempt's own ordered frame list, read rather than guessed.**
///
/// `Job files and outputs` M8 writes one attempt record before each render
/// starts, holding the frame list handed to the render in the order it was
/// written — attempt 1 included — **so attempt 1 is not a special case and
/// needs no rule of its own**. The list is read from the attempt record for
/// every attempt.
///
/// **The composition is the machine's and never the desktop's.** The
/// reckoning that composes a frame list is a step of the render worker and
/// runs on the machine; what the desktop composes and sends is the manifest's
/// frame **range**, which is not an order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheAttemptsOrder(Vec<i64>);

impl TheAttemptsOrder {
    /// **As the attempt record carries it, in the order it was written.**
    ///
    /// **Nothing re-sorts, re-derives or infers the order from a range.** A
    /// typed list is rendered in the order it was written and not sorted, and
    /// `Render Settings` holds a list as an alternative to the first, last and
    /// step rather than as a form of them — so a list written `50, 10, 20` is
    /// that order, and reading a three-number range in its place would put the
    /// frontier at 50 while Blender wrote 50 first, judging frames 10 and 20
    /// finished before they were rendered.
    pub fn as_the_attempt_record_carries_it(frames: Vec<i64>) -> TheAttemptsOrder {
        TheAttemptsOrder(frames)
    }

    pub fn frames(&self) -> &[i64] {
        &self.0
    }

    /// Where a frame sits in this order, if it is in it at all.
    pub fn position_of(&self, frame: i64) -> Option<usize> {
        self.0.iter().position(|already| *already == frame)
    }

    /// Every frame **strictly earlier than** the given one **in this order**,
    /// which is what both frontiers judge finished.
    pub fn strictly_before(&self, frame: i64) -> Vec<i64> {
        match self.position_of(frame) {
            None => Vec::new(),
            Some(at) => self.0[..at].to_vec(),
        }
    }
}

/// **J2's fallback: where the ordered list cannot be read — for any attempt —
/// mechanism one does not run for that attempt and mechanism two alone
/// decides.**
///
/// **Nothing is vouched for on a guessed order.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TheOrder {
    The(TheAttemptsOrder),
    /// The attempt record could not be read. **Not an empty order** — an empty
    /// order would judge nothing finished and read as a fact about the render.
    ItCouldNotBeRead,
}

impl TheOrder {
    /// **J2: mechanism one runs only where the order was read.**
    pub fn mechanism_one_runs(&self) -> bool {
        matches!(self, TheOrder::The(_))
    }
}

/// **J3: the live frontier — the highest frame in the attempt's order with
/// activity in the current walk.**
///
/// **It reads the current walk alone and it rises with the render**, and it is
/// recomputed at every walk like every other derivation here.
pub fn the_live_frontier(order: &TheAttemptsOrder, active_frames: &[i64]) -> Option<i64> {
    order
        .frames()
        .iter()
        .rev()
        .find(|frame| active_frames.contains(frame))
        .copied()
}

/// **J3: a frame is judged finished when a write event belonging to a strictly
/// later frame in the attempt's order is observed.**
///
/// Every frame strictly earlier than the live frontier in that order is judged
/// finished. **A frame that is the frontier at one walk is judged finished at
/// the next walk showing a later frame active.**
///
/// **This is not mechanism two's baseline frontier** ([`what_the_catch_up_vouches_for`]),
/// which is a different rule with a different shape. Mixing them freezes the
/// boundary at the first frame that showed activity after a connection's
/// baseline and leaves every later frame to the ending walk — a six-hour
/// render delivering a handful of frames live and the rest in one lot, with
/// nothing erroring and no surface showing it.
pub fn what_the_live_walk_vouches_for(order: &TheOrder, active_frames: &[i64]) -> Vec<i64> {
    let TheOrder::The(order) = order else {
        return Vec::new();
    };
    match the_live_frontier(order, active_frames) {
        None => Vec::new(),
        Some(frontier) => order.strictly_before(frontier),
    }
}

/// **J5: the last frame of an attempt has no later frame, so its signal is the
/// job's ending** — one of `Queue & History` E2's four, live or at catch-up.
///
/// **This is also the whole of what mechanism one claims**: it never claims
/// the file itself proved anything, and a file a full disk truncated at its
/// final name passes both mechanisms and is fetched — which is the case the
/// Creator's narrowing exists for, and the saying is the count and the
/// machine's printed line.
pub const THE_LAST_FRAMES_SIGNAL_IS_THE_JOBS_ENDING: bool = true;

/// **J4: what mechanism one deliberately tolerates, and the sub-second path is
/// the exception rather than the ordinary case.**
///
/// With placeholders on, the next frame's placeholder is itself the completion
/// signal for the frame before it, at sub-second latency; with placeholders
/// off, the signal is the next frame's first real write, so a frame is
/// confirmed about one frame-render later.
///
/// **The reason it is stated is so nobody plans on it.** The Placeholders
/// setting is off out of the box in all four versions, and it is the scene's
/// own output alone: it does not reach a file output node, and a two-frame
/// animation with placeholders on *"produced the node's files and the scene's
/// files, and no placeholder for the node"*. **So a job whose frames are
/// written by file output nodes alone has no placeholder path at all** and its
/// signal is always the next frame's first real write. **Both are inside the
/// promise** — files come home as they are written, at worst one frame behind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HowTheSignalArrives {
    /// The next frame's placeholder, at sub-second latency. **The exception.**
    TheNextFramesPlaceholder,
    /// The next frame's first real write, about one frame-render later. **The
    /// ordinary case, and the only one for a job written by nodes alone.**
    TheNextFramesFirstRealWrite,
}

/// **J6: on every (re)connection, and when an arrangement is switched on
/// mid-render, the first complete walk is the baseline and judges nothing by
/// itself.**
///
/// A file already present gives no event and carries no evidence — presence is
/// not proof, stability is not proof. **What decides is one of two later
/// signals.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TheCatchUpsSignal {
    /// **The job's ending is on record or arrives** — by the live event, or by
    /// the catch-up handed over once per connection (`Queue & History` C1).
    /// From that moment nothing more is being written, and every expected file
    /// that is present and not zero bytes is fetchable.
    TheJobsEnding(Ending),
    /// **The job is still rendering, and activity is observed since this
    /// connection's baseline.**
    ActivitySinceTheBaseline(Vec<i64>),
    /// **Until a signal arrives, nothing is vouched for and nothing is
    /// fetched** — waiting is the honest state and it is bounded by the frame
    /// in hand.
    NothingYet,
}

/// **J6: the baseline frontier — the *earliest* frame in the attempt's order
/// with activity observed since this connection's baseline.**
///
/// **It is cumulative and one-time by design**, which is what a catch-up needs
/// and what mechanism one's live frontier must never be confused with: the
/// since-baseline set only grows and its earliest member never rises, so this
/// rule is right exactly once and wrong as a standing frontier.
pub fn the_baseline_frontier(
    order: &TheAttemptsOrder,
    active_since_the_baseline: &[i64],
) -> Option<i64> {
    order
        .frames()
        .iter()
        .find(|frame| active_since_the_baseline.contains(frame))
        .copied()
}

/// **J6, as one judgement over the two signals.**
///
/// So switching on forty frames in vouches for frames 1–40 the moment frame
/// 41's first write is seen — and for frame 41 as well where what is seen is
/// frame 42's placeholder, which J1 puts at the instant frame 41's picture
/// completes. **After that one judgement, mechanism one's live frontier
/// carries the rest of the render.**
pub fn what_the_catch_up_vouches_for(
    order: &TheOrder,
    signal: &TheCatchUpsSignal,
) -> WhatTheCatchUpVouchesFor {
    match signal {
        // **The ending signal needs no order at all**, and this arm asks for
        // none: from that moment nothing more is being written, so every frame
        // the attempt expected is past the point of being written to. **J2's
        // fallback says mechanism two alone decides where the order could not
        // be read**, so an arm that answered *nothing* on an unreadable order
        // would leave that fallback with nothing to decide with.
        TheCatchUpsSignal::TheJobsEnding(_) => {
            WhatTheCatchUpVouchesFor::EverythingTheAttemptExpected
        }
        TheCatchUpsSignal::ActivitySinceTheBaseline(active) => {
            let TheOrder::The(order) = order else {
                return WhatTheCatchUpVouchesFor::TheseFrames(Vec::new());
            };
            WhatTheCatchUpVouchesFor::TheseFrames(match the_baseline_frontier(order, active) {
                None => Vec::new(),
                Some(frontier) => order.strictly_before(frontier),
            })
        }
        TheCatchUpsSignal::NothingYet => WhatTheCatchUpVouchesFor::TheseFrames(Vec::new()),
    }
}

/// **What mechanism two came to.**
///
/// **The two answers are different in kind and not two lengths of one list.**
/// The since-baseline signal names frames, drawn from the attempt's order. The
/// ending signal names **everything the attempt expected** and needs no order
/// to say so — so the caller enumerates it from the entries it already holds,
/// which is the one place that set is known whether or not the attempt record
/// could be read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheCatchUpVouchesFor {
    /// **Everything the attempt expected**, the job having ended.
    EverythingTheAttemptExpected,
    /// These frames, in the attempt's order.
    TheseFrames(Vec<i64>),
}

impl WhatTheCatchUpVouchesFor {
    /// The frames this names, **enumerated against the entries the caller
    /// holds** where the answer is *everything the attempt expected*.
    ///
    /// **That enumeration is the caller's because the entries are the
    /// caller's**, and it is what makes the ending arm independent of an order
    /// that may not have been readable.
    pub fn the_frames<'a>(
        &self,
        every_frame_the_attempt_expected: impl Iterator<Item = &'a i64>,
    ) -> Vec<i64> {
        match self {
            WhatTheCatchUpVouchesFor::EverythingTheAttemptExpected => {
                let mut frames: Vec<i64> = Vec::new();
                for frame in every_frame_the_attempt_expected {
                    if !frames.contains(frame) {
                        frames.push(*frame);
                    }
                }
                frames
            }
            WhatTheCatchUpVouchesFor::TheseFrames(frames) => frames.clone(),
        }
    }
}

/// **J7: the two mechanisms together keep the catch-up promise as the spec
/// narrowed it.**
///
/// Mechanism one keeps *outputs come home as they are written*; mechanism two
/// keeps *switching it on mid-render brings back what has already been
/// written* and *picks up everything that landed while they were away*.
/// **Neither vouches for a file it has no signal for**, and what neither can
/// vouch for is said on the job's entry through the count and the printed
/// lines.
pub const NEITHER_MECHANISM_VOUCHES_FOR_A_FILE_IT_HAS_NO_SIGNAL_FOR: bool = true;

/// One expected file of a frame, as a walk shows it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnExpectedFile {
    pub predicted_path: Vec<u8>,
    pub present: bool,
    /// The size the walk read. **Only [`a_zero_byte_file_is_never_a_finished_frame`]
    /// ever reads it**, and nothing else in this crate reads a size at all.
    pub size: u64,
}

/// **J3's fetchability test: a frame judged finished is fetchable where every
/// one of its expected files is present and none is zero bytes.**
///
/// **A zero-byte file in a judged-finished frame is an emptied or refused
/// write** and goes to *never coming* at the ending rather than being fetched.
pub fn a_judged_finished_frame_is_fetchable(files: &[AnExpectedFile]) -> bool {
    !files.is_empty()
        && files
            .iter()
            .all(|file| file.present && !a_zero_byte_file_is_never_a_finished_frame(file.size))
}

/// **J8: the files with no frame, judged by the ending alone.**
///
/// Files under an added write place's mapped folder, files matching no
/// current-attempt inventory entry, and any inventory entry carrying no frame
/// — a video destination's entry among them — **have no frame and no order, so
/// mechanism one cannot reach them; they are fetched at the ending walk, never
/// live.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhyItHasNoFrame {
    ItIsUnderAnAddedWritePlacesMappedFolder,
    ItMatchesNoCurrentAttemptInventoryEntry,
    /// **A video destination's entry** (`Job files and outputs` I20).
    ItsInventoryEntryCarriesNoFrame,
}

/// **J8: a video destination's entry is never fetchable before the ending
/// walk, whatever its size reads.**
///
/// It matches an inventory entry, so the unmatched-file rule does not reach
/// it, and it carries no frame, so the live frontier cannot judge it — and the
/// only fetchability test outside mechanism one is *present and not zero
/// bytes*, **which it passes from the first moments**: `Job files and outputs`
/// I21 has the file appearing within moments at a token size and holding that
/// size until the file is closed. Fetched on that reading it would be
/// published as the whole animation, recorded *fetched*, made un-refetchable,
/// and the arrangement would read complete over a one-second stub. **So it is
/// said outright rather than left to the one test available.**
pub const A_VIDEO_ENTRY_IS_NEVER_FETCHABLE_BEFORE_THE_ENDING_WALK: bool = true;

/// **J9: a manual request's vouching mark.**
///
/// A manual request has no arrangement, no reckoning and usually no inventory
/// in hand, **so its vouching reading is deliberately thinner — and it is
/// stated as what it is.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AVouchingMark {
    /// **Marked.** Smoozy says it cannot vouch for that file and brings it
    /// anyway if the user goes ahead.
    CannotBeVouchedFor,
    /// **Unmarked, and that is not a claim that the file is whole.** Outside a
    /// job's output folder, or for a job whose recorded ending is *finished*,
    /// **Smoozy has no standing to judge and makes no claim** — which is
    /// exactly the spec's *"this reaches exactly as far as what Smoozy can
    /// actually judge."*
    NoClaimIsMade,
}

/// What the local job record says about a file's job, as J9 reads it.
///
/// **Read off the same local job records the live-set reading `File Browser`
/// W5 already takes, with no read of the machine.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatTheLocalRecordSays {
    /// Whether the file lies inside that job's own output folder at all.
    pub the_file_is_inside_the_jobs_output_folder: bool,
    /// **`Queue & History` E2's four endings, or no recorded ending.**
    pub the_recorded_ending: Option<Ending>,
    /// **J9b: `Rendering` K18's per-frame reckoning where the recorded ending
    /// carried it.** *"An outcome carrying no statement is never read as an
    /// empty one"*, which is `Rendering` K22's own rule, so the absence of a
    /// statement is a value here and never an empty set.
    pub the_completeness_statement: Option<WhatTheOutcomeCarries>,
}

/// **J9 with J9a's correction and J9b's narrowing, as one judgement.**
///
/// **J9a: the key is the ending's kind and not an ending's absence, and the
/// mark is the whole folder and not the ending walk's frontier.** *Finished*
/// stays unmarked, matching the ending walk, which marks on the same three
/// endings and not on that one: a job can end *finished* with a frame that ran
/// out of room (`Rented-server runtime` M22 — a successful exit status is
/// never read as *everything was written*), and marking every file of every
/// finished job would be a mark that says nothing.
///
/// **J9b: where the recorded ending carried `Rendering` K18's reckoning, the
/// mark is read off that reckoning instead of laid over the whole folder.**
/// The reckoning covers the job's whole asked range, is composed once on the
/// machine at the ending (`Rendering` K19) and is carried to the local job
/// record with the recorded outcome, **so J9's own first sentence is not true
/// of this case**. J9a's objection is met rather than overruled: it refuses a
/// frontier *computed over what the user picked*, and nothing here computes
/// one.
///
/// **The zero-byte rule stays unconditional**: a zero-byte file inside a job
/// output folder is marked whatever the ending is and whatever the reckoning
/// says.
///
/// **What the narrowing costs, stated rather than hidden.** `Rendering` K21
/// rides with the set — a file short but not empty counts as present — **so a
/// frame the reckoning puts in *rendered* can still hold a torn picture, and
/// it is unmarked where the wide rule marked it.** That is the same standing
/// the auto route already accepts for the identical ending, and the same
/// standing every file of a *finished* job already has.
pub fn the_mark_for(
    says: &WhatTheLocalRecordSays,
    the_files_frame: Option<i64>,
    the_size_the_walk_read: u64,
) -> AVouchingMark {
    if !says.the_file_is_inside_the_jobs_output_folder {
        return AVouchingMark::NoClaimIsMade;
    }
    // J1's exception, and it is unconditional inside a job output folder.
    if a_zero_byte_file_is_never_a_finished_frame(the_size_the_walk_read) {
        return AVouchingMark::CannotBeVouchedFor;
    }
    let Some(ending) = says.the_recorded_ending else {
        // J9's own key, untouched by J9b: no recorded ending marks the whole
        // folder.
        return AVouchingMark::CannotBeVouchedFor;
    };
    if matches!(ending, Ending::Finished) {
        return AVouchingMark::NoClaimIsMade;
    }
    match (&says.the_completeness_statement, the_files_frame) {
        // J9b, where the statement arrived and the file has a frame for the
        // reckoning to speak about.
        (Some(WhatTheOutcomeCarries::TheSet(set)), Some(frame)) => {
            if set.rendered.contains(&frame) {
                AVouchingMark::NoClaimIsMade
            } else {
                AVouchingMark::CannotBeVouchedFor
            }
        }
        // J9b's second and third bullets: a file with no frame keeps the
        // whole-folder mark, and where no statement arrived nothing moves.
        _ => AVouchingMark::CannotBeVouchedFor,
    }
}

/// **J9a's own reason, held as a value: the mark is wide on the by-hand route
/// and narrow at the ending walk, and neither is a tuning of the other.**
///
/// The ending walk names the entries of the highest frame in the attempt's
/// order that shows any file, and it can: the arrangement holds the attempt's
/// frame order, the inventory and a complete walk of the whole job output
/// folder. **A manual request holds none of the three**, and its enumeration
/// descends from what the user picked, which need not be the whole folder. A
/// frontier computed over part of a folder names the highest frame *shown*,
/// which is an earlier frame than the one that was in flight, **so it would
/// mark a whole file and clear the torn one** — a false claim in the vouching
/// direction rather than a silence, which is the one direction this section
/// refuses everywhere.
pub const NO_FRONTIER_IS_COMPUTED_OVER_WHAT_THE_USER_PICKED: bool = true;

/// **J10: the saying happens first.**
///
/// The marked files are named in the request's pre-flight, beside the
/// collision question and before any transfer starts; the user's go-ahead
/// covers the request; **the files come down when they insist — the machine is
/// theirs.** Wording and presentation are Design's.
///
/// **A file re-written under an uninterrupted fetch is not detected, and the
/// limit is stated rather than covered by a safety net that is not there.**
/// `Server protocol and transfers` S13's four values are compared when a
/// download is arranged and before a resume, so a single uninterrupted pass
/// has no second comparison at all; and the pass-through digest proves only
/// that the bytes read are the bytes written — S25's own words: *"It does not
/// prove the file on the sending disk was whole before Smoozy read it."* A
/// frame is built up in place at its own final name, so a fetch beginning
/// mid-write reads a prefix of one state and a suffix of another, and the
/// digest matches. **Outside the live-set reading of J9 there is no further
/// claim.**
pub const THE_MARKED_FILES_ARE_NAMED_BEFORE_ANY_TRANSFER_STARTS: bool = true;

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use smoozy_rendering::which_frames::TheSet;

    pub(crate) fn an_order(frames: &[i64]) -> TheOrder {
        TheOrder::The(TheAttemptsOrder::as_the_attempt_record_carries_it(
            frames.to_vec(),
        ))
    }

    fn file(path: &[u8], present: bool, size: u64) -> AnExpectedFile {
        AnExpectedFile {
            predicted_path: path.to_vec(),
            present,
            size,
        }
    }

    #[test]
    fn the_live_frontier_is_the_highest_frame_in_the_attempts_order_and_not_the_highest_number() {
        // A typed list is rendered in the order it was written and not sorted.
        let order = an_order(&[50, 10, 20]);
        assert_eq!(what_the_live_walk_vouches_for(&order, &[10]), vec![50]);
        assert_eq!(what_the_live_walk_vouches_for(&order, &[20]), vec![50, 10]);
        // Reading the numbers as a range would have put the frontier at 50 and
        // judged 10 and 20 finished before they were rendered.
        assert_eq!(
            what_the_live_walk_vouches_for(&order, &[50]),
            Vec::<i64>::new()
        );
        // **Two frames active in one walk, the higher number earlier in the
        // order.** This is the case a frontier taking the largest number
        // passes every other assertion above on: it would vouch for nothing
        // where the order says frame 50 is finished.
        assert_eq!(what_the_live_walk_vouches_for(&order, &[50, 10]), vec![50]);
    }

    #[test]
    fn the_live_frontier_reads_the_current_walk_alone_and_rises_with_the_render() {
        let order = an_order(&[1, 2, 3, 4, 5]);
        assert_eq!(what_the_live_walk_vouches_for(&order, &[3]), vec![1, 2]);
        assert_eq!(what_the_live_walk_vouches_for(&order, &[4]), vec![1, 2, 3]);
        // Two frames active in one walk: the highest is the frontier.
        assert_eq!(
            what_the_live_walk_vouches_for(&order, &[2, 4]),
            vec![1, 2, 3]
        );
    }

    #[test]
    fn mechanism_one_does_not_run_at_all_on_an_order_that_could_not_be_read() {
        assert!(!TheOrder::ItCouldNotBeRead.mechanism_one_runs());
        assert_eq!(
            what_the_live_walk_vouches_for(&TheOrder::ItCouldNotBeRead, &[1, 2, 3]),
            Vec::<i64>::new()
        );
    }

    #[test]
    fn the_baseline_frontier_is_the_earliest_since_the_baseline_and_never_the_highest() {
        let order = an_order(&[1, 2, 3, 4, 5]);
        // Switching on at frame 3 with 3 and 4 both seen since the baseline:
        // the catch-up vouches for 1 and 2 and stops there.
        assert_eq!(
            what_the_catch_up_vouches_for(
                &order,
                &TheCatchUpsSignal::ActivitySinceTheBaseline(vec![3, 4])
            ),
            WhatTheCatchUpVouchesFor::TheseFrames(vec![1, 2])
        );
        // The live frontier over the same set would have gone to 4 and vouched
        // for 3 as well, which is the confusion J6 forbids.
        assert_eq!(
            what_the_live_walk_vouches_for(&order, &[3, 4]),
            vec![1, 2, 3]
        );
    }

    #[test]
    fn the_catch_up_vouches_for_nothing_until_a_signal_arrives() {
        let order = an_order(&[1, 2, 3]);
        assert_eq!(
            what_the_catch_up_vouches_for(&order, &TheCatchUpsSignal::NothingYet),
            WhatTheCatchUpVouchesFor::TheseFrames(Vec::new())
        );
        assert_eq!(
            what_the_catch_up_vouches_for(
                &order,
                &TheCatchUpsSignal::ActivitySinceTheBaseline(Vec::new())
            ),
            WhatTheCatchUpVouchesFor::TheseFrames(Vec::new())
        );
    }

    #[test]
    fn the_ending_signal_vouches_for_every_frame_whatever_the_ending_was() {
        let order = an_order(&[1, 2, 3]);
        for ending in Ending::ALL {
            let vouched =
                what_the_catch_up_vouches_for(&order, &TheCatchUpsSignal::TheJobsEnding(ending));
            assert_eq!(
                vouched,
                WhatTheCatchUpVouchesFor::EverythingTheAttemptExpected
            );
            assert_eq!(vouched.the_frames([1, 2, 3].iter()), vec![1, 2, 3]);
        }
    }

    /// **J6's first bullet needs no order, and neither does this arm.**
    ///
    /// *"From that moment nothing more is being written, and every expected
    /// file that is present and not zero bytes is fetchable."* **J2's fallback
    /// puts the whole decision on mechanism two where the attempt record could
    /// not be read**, so an ending arm that answered *nothing* on an
    /// unreadable order would leave that fallback deciding nothing at all.
    #[test]
    fn the_ending_signal_vouches_for_everything_even_where_the_order_could_not_be_read() {
        let vouched = what_the_catch_up_vouches_for(
            &TheOrder::ItCouldNotBeRead,
            &TheCatchUpsSignal::TheJobsEnding(Ending::Finished),
        );
        assert_eq!(
            vouched,
            WhatTheCatchUpVouchesFor::EverythingTheAttemptExpected
        );
        // The caller enumerates it from the entries it holds, which is the one
        // place that set is known whatever became of the attempt record.
        assert_eq!(vouched.the_frames([7, 7, 9].iter()), vec![7, 9]);
    }

    #[test]
    fn a_judged_finished_frame_is_fetchable_only_where_every_file_is_present_and_none_is_empty() {
        assert!(a_judged_finished_frame_is_fetchable(&[
            file(b"beauty/f_0001.exr", true, 1024),
            file(b"mist/f_0001.exr", true, 2048),
        ]));
        assert!(!a_judged_finished_frame_is_fetchable(&[
            file(b"beauty/f_0001.exr", true, 1024),
            file(b"mist/f_0001.exr", false, 0),
        ]));
        assert!(!a_judged_finished_frame_is_fetchable(&[
            file(b"beauty/f_0001.exr", true, 1024),
            file(b"mist/f_0001.exr", true, 0),
        ]));
        // A frame with no expected file at all vouches for nothing.
        assert!(!a_judged_finished_frame_is_fetchable(&[]));
    }

    #[test]
    fn no_size_but_zero_decides_anything() {
        assert!(a_zero_byte_file_is_never_a_finished_frame(0));
        for size in [1u64, 17_233_920, 4_000_000_000] {
            assert!(!a_zero_byte_file_is_never_a_finished_frame(size));
        }
    }

    fn a_record(
        inside: bool,
        ending: Option<Ending>,
        statement: Option<WhatTheOutcomeCarries>,
    ) -> WhatTheLocalRecordSays {
        WhatTheLocalRecordSays {
            the_file_is_inside_the_jobs_output_folder: inside,
            the_recorded_ending: ending,
            the_completeness_statement: statement,
        }
    }

    #[test]
    fn outside_a_job_output_folder_smoozy_makes_no_claim_at_all() {
        assert_eq!(
            the_mark_for(&a_record(false, None, None), Some(1), 1024),
            AVouchingMark::NoClaimIsMade
        );
        // Not even for a zero-byte file: the zero-byte rule is J1's exception
        // *inside a job output folder*, which is where Smoozy has standing.
        assert_eq!(
            the_mark_for(&a_record(false, None, None), Some(1), 0),
            AVouchingMark::NoClaimIsMade
        );
    }

    #[test]
    fn the_key_is_the_endings_kind_and_finished_stays_unmarked() {
        assert_eq!(
            the_mark_for(&a_record(true, Some(Ending::Finished), None), Some(1), 1024),
            AVouchingMark::NoClaimIsMade
        );
        for ended_short in [Ending::Stopped, Ending::Failed, Ending::GivenUp] {
            assert_eq!(
                the_mark_for(&a_record(true, Some(ended_short), None), Some(1), 1024),
                AVouchingMark::CannotBeVouchedFor
            );
        }
        // J9's own key, which J9b leaves untouched.
        assert_eq!(
            the_mark_for(&a_record(true, None, None), Some(1), 1024),
            AVouchingMark::CannotBeVouchedFor
        );
    }

    #[test]
    fn a_zero_byte_file_inside_a_job_output_folder_is_marked_whatever_the_ending_says() {
        let rendered = WhatTheOutcomeCarries::TheSet(TheSet {
            rendered: vec![1, 2],
            not_rendered: Vec::new(),
        });
        assert_eq!(
            the_mark_for(&a_record(true, Some(Ending::Finished), None), Some(1), 0),
            AVouchingMark::CannotBeVouchedFor
        );
        assert_eq!(
            the_mark_for(
                &a_record(true, Some(Ending::Stopped), Some(rendered)),
                Some(1),
                0
            ),
            AVouchingMark::CannotBeVouchedFor
        );
    }

    #[test]
    fn the_reckoning_narrows_the_mark_where_it_arrived_and_nothing_moves_where_it_did_not() {
        let statement = Some(WhatTheOutcomeCarries::TheSet(TheSet {
            rendered: vec![1, 2],
            not_rendered: vec![3],
        }));
        let says = a_record(true, Some(Ending::Stopped), statement);
        assert_eq!(
            the_mark_for(&says, Some(1), 1024),
            AVouchingMark::NoClaimIsMade
        );
        assert_eq!(
            the_mark_for(&says, Some(3), 1024),
            AVouchingMark::CannotBeVouchedFor
        );
        // J9b's second bullet: a file with no frame keeps the whole-folder
        // mark, because the reckoning says nothing about it.
        assert_eq!(
            the_mark_for(&says, None, 1024),
            AVouchingMark::CannotBeVouchedFor
        );
        // J9b's third bullet: no statement is never read as an empty one.
        let no_statement = a_record(
            true,
            Some(Ending::Stopped),
            Some(WhatTheOutcomeCarries::NoStatement),
        );
        assert_eq!(
            the_mark_for(&no_statement, Some(1), 1024),
            AVouchingMark::CannotBeVouchedFor
        );
    }
}

#[cfg(test)]
mod the_size_closure {
    use crate::markers::tests::every_source_of_this_crate;

    /// The one function in this crate that reads a size to decide anything.
    const THE_ONE_READING_OF_A_SIZE: &str = "a_zero_byte_file_is_never_a_finished_frame";

    /// Whether a line reads a `.size` field at all — `.size_as_enumerated` and
    /// the like are different fields and are not this.
    fn it_reads_a_size_field(line: &str) -> bool {
        let mut rest = line;
        while let Some(at) = rest.find(".size") {
            let after = rest[at + ".size".len()..].chars().next().unwrap_or(' ');
            if !(after.is_ascii_alphanumeric() || after == '_') {
                return true;
            }
            rest = &rest[at + ".size".len()..];
        }
        false
    }

    /// **§4's size closure, held rather than remembered.**
    ///
    /// *"Keying on a size — with exactly one exception, stated at J1 and
    /// nowhere widened: a file of zero bytes is never a finished frame. **No
    /// other reading of a size decides anything in this document**"* — so no
    /// plausibility floor, no minimum-bytes-for-this-format test and no
    /// has-it-stopped-growing check follows from the zero-byte rule.
    ///
    /// **Every line of this crate's working code that reads a size field
    /// either hands it to that one function, or is W5's change detection**,
    /// which compares two sizes against each other and decides nothing about a
    /// file being finished.
    ///
    /// **What it cannot see, said rather than glossed.** It is line-based and
    /// compiles nothing: a size carried through a variable of another name, or
    /// read in a file this walk does not reach, is invisible to it. It fires
    /// on the shape the class actually took every time it was written — a
    /// size read at a site that judges — and not on a size laundered through
    /// three assignments first.
    #[test]
    fn no_reading_of_a_size_but_the_zero_byte_rule_decides_anything() {
        let mut faults = Vec::new();
        for (path, text) in every_source_of_this_crate() {
            let mut in_a_test_module = false;
            for line in text.lines() {
                let trimmed = line.trim();
                if trimmed.starts_with("#[cfg(test)]") {
                    in_a_test_module = true;
                }
                if in_a_test_module || trimmed.starts_with("//") {
                    continue;
                }
                if !it_reads_a_size_field(trimmed) {
                    continue;
                }
                let hands_it_to_the_one_reading = trimmed.contains(THE_ONE_READING_OF_A_SIZE);
                // W5's change detection: one size against another, and never
                // against a number.
                let compares_two_sizes =
                    trimmed.matches(".size").count() >= 2 && !trimmed.contains('0');
                // **Carrying a size is not reading one.** Added 2026-09-05
                // with the acting half, which is the first thing in this crate
                // to carry a size at all: the walk takes the route's own size
                // field into [`crate::watch::AWalkEntry`]'s, and the
                // enumeration hands one on to the vouching mark, which hands
                // it to the one reading three calls further down where this
                // line-based test cannot follow it.
                //
                // **The allowance is narrow and it is about the shape of the
                // line rather than about where the value goes**: a struct
                // field initialisation of a `size` field, or a bare argument
                // in a call, **with no comparison in it at all.** A line that
                // decided anything about a size would hold one, so
                // `if size > n`, `size == 0` and `size != before` are all
                // still faults, wherever they are written.
                let it_is_a_field_or_an_argument =
                    trimmed.starts_with("size:") || trimmed.ends_with(',');
                let it_compares = trimmed.contains('<')
                    || trimmed.contains('>')
                    || trimmed.contains("==")
                    || trimmed.contains("!=")
                    || trimmed.contains("if ");
                let carries_it_without_deciding = it_is_a_field_or_an_argument && !it_compares;
                if !hands_it_to_the_one_reading
                    && !compares_two_sizes
                    && !carries_it_without_deciding
                {
                    faults.push(format!("{path}: {trimmed}"));
                }
            }
        }
        assert!(faults.is_empty(), "{faults:#?}");
    }

    /// **R8 held by the same reading: the reckoning never asks the
    /// already-exists question.**
    ///
    /// The fetch decision is the reckoning's — *what have I taken* — never a
    /// comparison against the destination, and **the destination is consulted
    /// exactly once per file, by the exclusive create at publication**. So no
    /// working line of the arrangement's own file may name a destination
    /// listing at all; the already-exists comparison lives in the manual half,
    /// which is where the question belongs.
    #[test]
    fn the_arrangements_own_file_never_names_a_destination_listing() {
        let mut faults = Vec::new();
        for (path, text) in every_source_of_this_crate() {
            if !path.ends_with("arrangement.rs") {
                continue;
            }
            let mut in_a_test_module = false;
            for line in text.lines() {
                let trimmed = line.trim();
                if trimmed.starts_with("#[cfg(test)]") {
                    in_a_test_module = true;
                }
                if in_a_test_module || trimmed.starts_with("//") {
                    continue;
                }
                if trimmed.contains("listing") {
                    faults.push(format!("{path}: {trimmed}"));
                }
            }
        }
        assert!(faults.is_empty(), "{faults:#?}");
    }
}

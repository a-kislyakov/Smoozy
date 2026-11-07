//! **The History screen** — Smoozy's own record of every job that ever started
//! rendering, beside the entry of the one that is chosen. The page half is
//! `web/the-history.js`.
//!
//! **It is a second screen and not a tab of the queue, and Design gives the
//! reason**: *"They empty differently … the queue is blank on a fresh launch
//! until a server is connected, because Smoozy keeps no lasting copy of it,
//! while history is Smoozy's own record and is there straight away."*
//!
//! ## What this piece draws, cited the way the runner ruled on 2026-09-12
//!
//! **The Design headings this file draws**: *History*, *The two ways to render
//! it again*, *Deleting an entry*, *Clearing history*, and — shared with the
//! queue and composed in [`crate::surface::the_queue`] — *A rendering entry*,
//! *The marks*, *The rest of the entry* and *The entry's controls*.
//!
//! **The mockup state names this file draws**: `h`, `h-unk`, `h-nofr`,
//! `h-disk`, `h-ok`, `h-stop`, `h-gave`, `h-clear`, `h-clearall`, `h-delete`,
//! `h-gone`, `h-checkjob`, `h-large`, `h-pvmissing`, `h-pvbad`. **Fifteen, and the queue's states are [`crate::surface::the_queue`]'s.**
//! *(Ten until 2026-10-18, when `h-gone` was drawn for the Creator's
//! fourteenth notes, note 8: a refused act's answer under the acts row, which
//! `web/the-queue.js` places for both screens.)*
//!
//! ## The failure shape, and `WhoseSentenceItIs`
//!
//! **Three of the six endings carry the whole-app failure shape and this screen
//! composes its plain sentence**, because `Desktop app and delivery` §24 hands
//! the words to Design and Design's words for these three are in
//! `Preparation/Design/Queue & History.md`. **So this screen passes
//! [`WhoseSentenceItIs::TheComposersOwn`]** — M32, repaired 2026-09-12: *"the
//! capture belongs to whatever composes the sentence the user reads, and a path
//! that composes a stand-in sentence captures nothing."*
//!
//! **Getting that wrong produces no error, no failing test and no visible
//! fault.** The shared half's own two presentation paths pass the stand-in
//! because they have no words; this screen has them, and a stand-in passed from
//! here would leave the diary holding *Something is not working* for the life of
//! the record while the user read something else.
//!
//! **Where the record has already been presented once the diary is handed
//! nothing**, which is M32's *once* and is the ordinary answer rather than a
//! failure.
//!
//! ## Where the entries come from, and they come from a store at last
//!
//! **Until record version 23 there was no local job record and no history
//! record in the local record at all**, and this screen answered an empty
//! history on every run: `src/record/schema.rs` held `history_tombstone` and no
//! entry for it to be a tombstone of. **Step 10b piece 6 landed the table**, in
//! `src/record/job_records.rs`, and [`what_is_kept`] reads it.
//!
//! **`Queue & History` C9 decides which records are drawn here and the standing
//! does not**: a job is in history from the moment Smoozy learns it started
//! rendering. A provisional record is never shown as one while its server can
//! still be asked, and a confirmed record whose render start has not been
//! learned is in no list at all.
//!
//! **C9's second way in, item C1 of step 10b round 1 of *jobs, history and
//! downloads*** — *Creator-stated, 2026-09-18*: a record whose start was never
//! learned, whose server is gone, is kept and drawn, marked that Smoozy never
//! heard whether it started. Its row reads [`START_NOT_KNOWN`] and its entry
//! says [`SMOOZY_NEVER_HEARD_WHETHER_IT_STARTED`].
//!
//! **The machine's per-job note carries its half onto the same record.** Ruled
//! by the build runner on 2026-09-17: `Queue & History` N1's times, ending,
//! attempt marks and last-frame figure are fields of `LocalJobRecord` rather
//! than a record kind of their own, and `State, identity and recovery` V1 step
//! 4's fill-in, at that document's §7, is what writes them.
//!
//! **The fill-in runs on every connection since step 10c piece 7.**
//! `crate::servers::the_machines_work` performs `State, identity and recovery`
//! V1 step 4 at `Servers` Q1 step 7, and again whenever a P10d pass applies an
//! event about the line — so an entry's start, ending, times and figure are
//! what the machine's note said when a connection last read it. **The same
//! piece is the first caller of item C1's mark** at `State, identity and
//! recovery` V1 step 1 (`src/servers/a_period.rs`): a server whose installation
//! is found gone marks its records *start never heard*, and they are drawn
//! here from then on. **The frame pair the drawing shows as *60/240*** is the
//! frames the server held when the job ended — the note's figure and every
//! frame before it in the asked order `Queue & History` N9's fill-in lands (the
//! 38's piece 10; counted up to and including the figure since the live pass
//! on Vast, 2026-10-19). *(Until that piece this said the pair had neither half
//! on this side, which was true: nothing read the asked order.)*
//!
//! ## The three acts on this screen that are acts, and they are the only ones
//! on either screen
//!
//! **Step 10b piece 8: the delete box's *Delete*, *Clear this server's history*
//! and *Clear all history* change the record now**, through
//! [`the_act_is_performed`]. Each is `Queue & History` X1's two writes as one
//! unit — the entry's record removed and its tombstone written — and a clearing
//! is that act over every entry its scope covers, partial across entries and
//! whole within each.
//!
//! **What a deletion still owes is the machine's own copy**, which `Queue &
//! History` X2 runs at the next connection and which rides the tombstone as a
//! debt. Nothing here attempts it, so every deletion Smoozy makes today is X4's
//! deletion made while disconnected.
//!
//! **The two ways to render it again act, since the 38's piece 2.** *Submit
//! again* and the carry-on are `Rendering`'s, and their route from an entry to
//! the Render Settings page is `crate::a_new_job_from_an_old_one`;
//! [`crate::surface::the_queue::where_an_act_on_a_job_goes`] is where which act
//! goes where is written down, and the test beside it is what keeps this
//! paragraph honest. The group is drawn from the asked order the fill-in lands
//! (`Queue & History` N9, the 38's piece 10) and only where the record kept the
//! setup a new job is made from.
//!
//! ## This file carries no header list of other documents' markers either
//!
//! The reason is [`crate::surface::the_queue`]'s and is written out there: this
//! crate's own plan defines a clause of the same short name for very nearly
//! every marker these two screens cite. **So every one is written out with its
//! document at the site**, and a bare marker below means a clause of this
//! crate's own plan.

use std::sync::Mutex;

use tauri::State;

use crate::events::{Event, Said, Subject as About, TheOneQueue};
use crate::presentation::the_failure_shape::{AControl, WhatItIs};
use crate::presentation::the_two_hooks::{
    the_diary_is_handed_the_sentence, TheDiaryWasHanded, WhoseSentenceItIs,
};
use crate::record::failure_records::{DurableSubject, FailureRecords, Standing};
use crate::record::job_records::JobRecords;
use crate::record::LocalRecord;
use crate::start::Started;
use crate::surface::the_queue::{
    a_name, a_part_is_said_under, by_count, the_box_is_said, the_entry_is_said, AFactOnTheEntry,
    AMarkOnTheEntry, APart, TheActOnAJob, TheBoxThatAsks, TheJobEntry, ThePair, TheRangeInHistory,
    TheSharedWords, ENTRIES_WORD, ENTRY_WORD, FRAMES_WORD, FRAME_WORD,
};
use crate::typography::{Prose, Verbatim};
use smoozy_queue_history::ending::Ending;
use smoozy_queue_history::record::{LocalJobRecord, TheJobOutputFolderTheRecordHolds};
use smoozy_state::identities::Identity;

/// **The one kind this screen's page listens for**, declared here beside the
/// service that posts it and copied into `web/the-history.js`.
pub const THE_HISTORY_SAYS: &str = "history:the-history";

// ───────────────────────────────────────────────────────────────────────────
// The words
// ───────────────────────────────────────────────────────────────────────────

const THE_HEADING: &str = "History";
const KEPT_UNTIL_YOU_CLEAR_THEM: &str = "kept until you clear them";
const THE_JOBS_WORD: &str = "jobs";
const THE_JOB_WORD: &str = "job";

/// **Design, *History*: five endings, and the colour is on the word itself.**
/// The fifth is the absence of one and carries the difference in the shape of
/// the phrase rather than in a fifth colour — Label: **Creator-stated**
/// (2026-08-13).
const FINISHED: &str = "Finished";
const FAILED: &str = "Failed";
const STOPPED: &str = "Stopped";
const GIVEN_UP: &str = "Abandoned";
const ENDING_NOT_KNOWN: &str = "Ending unknown";
/// **Design, *History*: a job Smoozy never heard had started** — item C1,
/// *Creator-stated, 2026-09-18*. The same grey italic as *Ending unknown*,
/// because it is the same kind of absence one step earlier.
const START_NOT_KNOWN: &str = "Unknown if started";
/// **Design, *History*: a job that has started and whose ending Smoozy has not
/// learned, on a server that is still there** — E1-1, added 2026-10-01 from
/// step 10c's twelfth pass, where a job rendering in Queue read *Ending not
/// known* here. *Ending unknown* is kept for a removed server.
///
/// **No History row draws it since 2026-10-04**: such a job is not in the
/// list at all (the Creator's second notes, piece 7, note 11 — `Queue &
/// History` C9 as narrowed that day). [`AnEnding::InProgress`] stays because
/// the queue builds its entry with [`the_entry_for`] and reads its class.
const IN_PROGRESS: &str = "In progress";

/// The header's own controls — Design, *History*.
const CLEAR_THIS_SERVERS_HISTORY: &str = "Clear this server's history";
const CLEAR_ALL_HISTORY: &str = "Clear all history";
const DELETE: &str = "Delete";

/// **Design, *History*: jobs from a server no longer in Smoozy sit together in
/// one group at the foot of History, and their badge is grey** — the
/// eighteenth notes, note 12, Creator-stated. The group's heading (its count
/// takes the boxes' own *entry* and *entries*), and the line under an entry's
/// name — *Lambda A100, removed*, the copied name being the value before it.
const REMOVED_SERVERS: &str = "Removed servers";
const REMOVED_TRAIL: &str = ", removed";
/// **The grey squircle in place of the server's colour**, as the drawing's own
/// class (`.badge.brm`). A value, never shown.
const THE_REMOVED_BADGE: &str = "brm";
/// **Design, *Clearing history*: the fourth act**, on its own line under the
/// group's heading — the eighteenth notes, note 13, Creator-stated
/// (`Queue & History` X6b).
const CLEAR_REMOVED_SERVERS_HISTORY: &str = "Clear removed servers' history";

/// The status line, one sentence per ending, each in parts because a frame
/// figure and a duration travel beside the words (X7).
const FAILED_AT_FRAME: &str = "at frame";
const AFTER_WORD: &str = "after";
const FINISHED_LEAD: &str = "—";
const FRAMES_IN: &str = "frames in";
const SUBMITTED_LEAD: &str = "— submitted";

/// **Design, *The rest of the entry*: the plain sentence of the whole-app
/// failure shape, one per ending, and none of them names a cause.**
const THE_RENDER_DID_NOT_FINISH: &str = "The render did not finish.";
const SMOOZY_DOES_NOT_KNOW_WHY: &str = "Cause unknown.";
const NO_FRAME_WAS_COMPLETED: &str =
    "The render did not finish. No frames completed.";
/// **Design, *Where no render began*: nothing started, so nothing did not finish.**
const FAILED_BEFORE_RENDERING: &str = "Failed before rendering.";
const YOU_STOPPED_THIS_RENDER: &str = "You stopped this render.";
/// **Design, *History*: the stop box's conditional, said after the act, and
/// never that the frame was not saved.** `Servers` W19a: a stop lands either
/// mid-render, when the frame never arrives, or during that frame's own write,
/// when the frame is whole, and nothing at the ending says which. **Not
/// composed from the box's sentence** (`src/surface/the_queue.rs`'s
/// `THE_FRAME_IT_IS_ON_MAY_NOT_BE_SAVED`), because the tense differs; a test
/// holds the two to the same conditional. *(S3's loose ends, item 1,
/// 2026-09-29: this said *was not saved*, the flat claim R1 took out of the
/// stop box on 2026-09-17.)*
const THE_FRAME_IT_WAS_ON_MAY_NOT_BE_SAVED: &str = "The frame it was on may not be saved.";
const INTERRUPTED_TWICE_AND_GIVEN_UP: &str =
    "The render was interrupted twice and abandoned.";

/// **Design, *The rest of the entry*: where a failed render's error stream was
/// empty, this is the plain sentence** — the one the user reads first and the
/// one the diary keeps (must-do 9 of step 10a round 1 of the queue, history,
/// settings and downloads). It points at the output and claims nothing about
/// either stream. **Reworded 2026-10-11, the Creator's eighth notes, note 6:**
/// it read *This render reported no errors, and nothing in Blender's own output
/// says why it ended.*, and Blender 5 writes its errors to its ordinary output,
/// which Smoozy never reads for a cause.
const THE_RENDER_FAILED_AND_THE_OUTPUT_MAY_SAY_WHY: &str =
    "The render failed. Blender's output may say why.";

// **The one cause Smoozy names on a failed job** — the disk-full carve-out,
// which is read off the machine rather than worked out, **in its one
// sentence**: [`crate::presentation::the_disk_full_carve_out::THE_DIAGNOSIS`],
// which names the folder and never the server's disk (`Desktop app and
// delivery` M17, `Server protocol and transfers` F16). The server's name
// travels beside it as a value, under its own label (X7). *(Until step 12's
// SC4 findings this file kept a second sentence of its own, "The disk on this
// server is full.", which that clause refuses; it went on the runner's
// ruling.)*
use crate::presentation::the_disk_full_carve_out::{what_to_do_about_it, TheCarriedReading, THE_DIAGNOSIS};

/// **Design, *The two ways to render it again*.**
const RENDER_IT_AGAIN: &str = "Render it again";
const SUBMIT_AGAIN: &str = "Submit again —";
const CARRY_ON: &str = "Resume —";
const CARRY_ON_MAKES_A_NEW_JOB: &str =
    "The frames end up split across two folders.";
const CARRYING_ON_FROM_FRAME: &str = "Resuming from frame";
/// **Where the carry-on's figure came from, said only as it is** — must-do
/// 2(c), and `Rendering` A9: *"a remembered figure is never presented as a
/// checked one."*
const READ_FROM_THE_SERVER_JUST_NOW: &str = "— read from the server.";
/// **The recorded figure, in the words the render settings page already uses
/// for the same case**, and **taken from that page rather than written again**
/// — step 10a, the screens round's loose ends, L6, which is must-do 2(c)'s
/// *"Keep that sentence in one place and use it from both files."*
use crate::surface::render_settings::the_recorded_figure;
const THE_FRAME_THIS_ENTRY_RECORDED: &str = concat!("— ", the_recorded_figure!(), ".");
const IS_THE_ONE_SMOOZY_CANNOT_CONFIRM: &str =
    "is not confirmed finished, so it is rendered again. Lower the number to start earlier.";
const THE_FRAME_WORD: &str = "Frame";
const EVERY_FRAME_IS_RENDERED: &str =
    "All frames rendered.";
/// **`Queue & History` E3, reversed 2026-11-02: what a finish says in place of
/// that line where only part of what it owed was written.** *3 of 4 files
/// written.*, the figures travelling as values beside these words.
const FILES_WORD_OF: &str = "of";
const FILES_WRITTEN_TRAIL: &str = "files written.";
const ONE_FILE_WRITTEN_TRAIL: &str = "file written.";
pub(crate) const NO_FRAME_TO_START_FROM: &str =
    "Resume unavailable: this job's progress is unknown.";
pub(crate) const NO_FRAME_FINISHED_SO_NOTHING_TO_START_FROM: &str =
    "Resume unavailable: no frame finished.";
/// **`Queue & History` N17: where the entry's server is no longer in Smoozy,
/// resuming is not offered**, in Design's words for a removed server
/// (must-do 2(d)).
pub(crate) const THE_SERVER_IS_GONE_SO_NO_CARRY_ON: &str =
    "Resume unavailable: the server was removed.";

/// **Design, *History*: the entry keeps its times, and where it can be had,
/// which parts of the server did the work.**
const RENDERED_HEADING: &str = "Rendered";
const LAST_FRAME_FACT: &str = "Last frame";
const SUBMITTED_FACT: &str = "Submitted";
const STARTED_FACT: &str = "Started";
const ENDED_FACT: &str = "Ended";
const RENDERED_FOR_FACT: &str = "Rendered for";
const RAN_ON_FACT: &str = "Ran on";
const NOT_KNOWN: &str = "not known";
const NONE_FINISHED: &str = "none finished";
const NOT_CONFIRMED_TRAIL: &str = "not confirmed";
const THE_FRAME_LEAD: &str = "— frame";
const THE_SERVER_WAS_DESTROYED: &str =
    "The server was destroyed before the next connection. Its record of this job is lost.";
/// **Design, *History*: the entry's one line for a job Smoozy never heard had
/// started**, true whether the user removed the server or Smoozy found nothing
/// of its own left on the machine.
const SMOOZY_NEVER_HEARD_WHETHER_IT_STARTED: &str =
    "Whether this job started is unknown, and its server is no longer available.";
const LAST_COMPLETE: &str = "last complete";

/// **Design, *Deleting an entry*: three sentences and the control is one
/// word.** It says *settings* and not *setup*.
const DELETE_THIS_ENTRY_QUESTION: &str = "Delete this entry?";
/// **A first-frame pair's delete box** — `Design/Queue & History § A
/// first-frame check and its job`; the rest of the box is the full range's.
const DELETE_THIS_ENTRY_AND_ITS_CHECK_QUESTION: &str = "Delete this entry and its first-frame check?";
/// **The drawing's class for *In Queue* and *Check only***: grey, where the
/// ending goes (the nineteenth notes, note 7).
const THE_CHECKS_GREY: &str = "e-chk";
const THE_RECORD_IS_DELETED: &str =
    "This entry and its settings are deleted permanently, so it cannot be submitted again.";
/// **What the delete box says is not changed, in one sentence** — *60 frames
/// on the server and 48 downloaded frames are not changed.* — as the clear
/// boxes say it. *Changed 2026-11-06, the last words read before the release:
/// it was two sentences, *The 60 frames on the server are untouched.* and *The
/// 48 frames already downloaded to your computer are untouched too.**
const FRAMES_ON_THE_SERVER: &str = "frames on the server";
const FRAME_ON_THE_SERVER: &str = "frame on the server";
const FRAMES_ON_THE_SERVER_AND: &str = "frames on the server and";
const FRAME_ON_THE_SERVER_AND: &str = "frame on the server and";
const DOWNLOADED_FRAMES: &str = "downloaded frames";
const DOWNLOADED_FRAME: &str = "downloaded frame";
const ARE_NOT_CHANGED: &str = "are not changed.";
const IS_NOT_CHANGED: &str = "is not changed.";
/// **Design, *Deleting an entry*: each half said without a number, where the
/// entry knows none** — the upstream must-do of step 10b round 1 of *jobs,
/// history and downloads*.
const THE_FRAMES_ON_THE_SERVER_ARE_UNTOUCHED_UNCOUNTED: &str = "Frames on the server";
const FRAMES_ON_THE_SERVER_UNCOUNTED_AND: &str = "Frames on the server and";
const ANYTHING_ALREADY_DOWNLOADED: &str = "Downloaded frames are not changed.";
const DOWNLOADED_FRAMES_UNCOUNTED: &str = "downloaded frames";
const NOTHING_WAS_RENDERED: &str =
    "Nothing was rendered.";
const WHATEVER_IT_RENDERED_IS_ON_A_SERVER: &str =
    "What it rendered is on a server that is no longer available.";
const KEEP_THE_ENTRY: &str = "Keep the entry";

/// **A History read that failed is said** — must-do 3(a) of step 10b round 1 of
/// *jobs, history and downloads*, on the pattern of Transfers' *"Smoozy could
/// not read your servers."* Drawn through the one failure shape in place of the
/// list, and never as the empty History a person who never rendered sees.
const SMOOZY_COULD_NOT_READ_YOUR_HISTORY: &str = "Could not read History.";
/// **Rows that would not read back are counted and said** — must-do 3(c). The
/// figure travels as a value beside these words.
const ENTRY_COULD_NOT_BE_READ: &str = "entry could not be read.";
const ENTRIES_COULD_NOT_BE_READ: &str = "entries could not be read.";

/// **Design, *Clearing history*: both boxes follow the single delete's shape —
/// what goes, then what does not — and the two do not get the same box.**
const CLEAR_LEAD: &str = "Clear";
const HISTORY_QUESTION_TRAIL: &str = "history?";
const CLEAR_ALL_HISTORY_QUESTION: &str = "Clear all history?";
const ITS_ENTRIES_ARE_DELETED: &str =
    "entries and their settings are deleted permanently, so those jobs cannot be submitted again.";
const ITS_ENTRY_IS_DELETED: &str =
    "entry and its settings are deleted permanently, so that job cannot be submitted again.";
const ITS_WORD: &str = "Its";
const ALL_WORD: &str = "All";
/// The same word inside a control's own label, where it does not open a
/// sentence. Rule 9: a control reads as a control and not as a heading.
const ALL_LOWER: &str = "all";
const AFTERWARDS_NOTHING_SHOWS_THEM: &str =
    "entries and their settings are deleted permanently.";
const AFTERWARDS_NOTHING_SHOWS_IT: &str =
    "entry and its settings are deleted permanently.";
/// A lead for one entry, where *All* would read *All 1*.
const THE_ONE_WORD: &str = "The";
const THE_FRAMES_ON_ARE_UNTOUCHED: &str =
    "Frames on the servers and downloaded files are not changed.";
const THE_FRAMES_ON_THIS_SERVER_ARE_UNTOUCHED: &str =
    "and downloaded files are not changed.";
const THE_FRAMES_ON_LEAD: &str = "Frames on";
const KEEP_THEM: &str = "Keep them";
/// **Design, *Clearing history*: the fourth act's box** — `h-clearremoved`.
const CLEAR_THE_HISTORY_OF_REMOVED_SERVERS_QUESTION: &str = "Clear the history of removed servers?";
const THEIR_WORD: &str = "Their";
const THEIR_ENTRIES_GO_WITH_SUBMIT_AGAIN: &str =
    "entries and their settings are deleted permanently, so those jobs cannot be submitted again.";
const ITS_ENTRY_GOES_WITH_SUBMIT_AGAIN: &str =
    "entry and its settings are deleted permanently, so that job cannot be submitted again.";
const NOTHING_IS_CHANGED_ON_REMOVED_SERVERS: &str =
    "These servers and downloaded files are not changed.";
const CLEAR_WORD: &str = "Clear";
const SERVERS_THAT_ARE_GONE: &str = "Removed servers";
/// **The disk-full server's label**, which the failure shape draws beside the
/// server's name. Prose the user reads, sent as prose (must-do 20).
const THE_SERVER_LABEL: &str = "Server";

// ───────────────────────────────────────────────────────────────────────────
// The marks and the setup — Design, *The marks* and *The rest of the entry*
// ───────────────────────────────────────────────────────────────────────────
//
// **These sentences sit in two files and that is a fault rather than a
// choice, and it is named here so nobody reads it as one.**
// `src/surface/the_queue.rs` declares its own copy of fifteen of them, written
// by step 10a for a queue composer that was never built — **nothing in that
// file reads any of them**, and the queue's entry is composed nowhere. *(Step
// 13's words sweep, 2026-10-28, deleted four of those copies rather than
// reword them twice.)* This
// file is the only live home. **The two must become one home the moment the
// queue gets a composer**, and until then a change made to one of these
// sentences has to be made twice, which is exactly the fault this build stage
// keeps meeting.
//
// **Why the copy is here and not there.** The marks are Design's words for one
// entry that both screens fill, so their one home belongs beside
// [`crate::surface::the_queue::the_entry_is_said`]. That file is another
// agent's this chunk and could not be touched, and a mark drawn from no words
// at all is an entry with a blank where a sentence goes. Recorded in
// `Audits/Build/Queue & History/Decisions record.md`.

const A_FILE_THIS_JOB_NEEDS_WAS_REPLACED: &str = "A file this job needs was replaced.";
const IT_MAY_HAVE_BEEN_REPLACED: &str = "A file this job needs may have been replaced.";
const WAS_REPLACED_AFTER_SUBMISSION: &str = "was replaced after this job was submitted.";
// **`Design/Queue & History § The marks`: the verb is the act's** — replaced,
// deleted, moved or renamed. Step 10c's Files' owed acts piece 2.
const A_FILE_THIS_JOB_NEEDS_WAS_DELETED: &str = "A file this job needs was deleted.";
const IT_MAY_HAVE_BEEN_DELETED: &str = "A file this job needs may have been deleted.";
const WAS_DELETED_AFTER_SUBMISSION: &str = "was deleted after this job was submitted.";
const A_FILE_THIS_JOB_NEEDS_WAS_MOVED: &str = "A file this job needs was moved.";
const IT_MAY_HAVE_BEEN_MOVED: &str = "A file this job needs may have been moved.";
const WAS_MOVED_AFTER_SUBMISSION: &str = "was moved after this job was submitted.";
const A_FILE_THIS_JOB_NEEDS_WAS_RENAMED: &str = "A file this job needs was renamed.";
const IT_MAY_HAVE_BEEN_RENAMED: &str = "A file this job needs may have been renamed.";
const WAS_RENAMED_AFTER_SUBMISSION: &str = "was renamed after this job was submitted.";
// **`File Browser` W9: where the act's outcome was never learned, the line
// under the mark says *may have been* too**, and never claims the act. After
// 0.1.8, the checks' leftovers, item 5.
const MAY_HAVE_BEEN_REPLACED_AFTER_SUBMISSION: &str =
    "may have been replaced after this job was submitted.";
const MAY_HAVE_BEEN_DELETED_AFTER_SUBMISSION: &str =
    "may have been deleted after this job was submitted.";
const MAY_HAVE_BEEN_MOVED_AFTER_SUBMISSION: &str = "may have been moved after this job was submitted.";
const MAY_HAVE_BEEN_RENAMED_AFTER_SUBMISSION: &str =
    "may have been renamed after this job was submitted.";

/// **The mark's three sentences for the act the record keeps** —
/// `Queue & History` N21's act intended, and `Design/Queue & History § The
/// marks`: *the verb is the act's*. Step 10c's Files' owed acts piece 2.
///
/// **The acts are `File Browser`'s closed set** (`File Browser` B3, at its §11.1: *delete*,
/// *rename*, *move*, *copy*, *new folder*), with an upload's *Replace*
/// beside them. **A copy, and an upload, publish over the file, so they
/// replace it.** A record whose act is none of these — one written before any
/// act wrote a mark — reads *replaced*, the one word every mark said until
/// this piece.
fn the_marks_words(
    act_intended: &str,
) -> (&'static str, &'static str, &'static str, &'static str) {
    match act_intended {
        "delete" => (
            A_FILE_THIS_JOB_NEEDS_WAS_DELETED,
            IT_MAY_HAVE_BEEN_DELETED,
            WAS_DELETED_AFTER_SUBMISSION,
            MAY_HAVE_BEEN_DELETED_AFTER_SUBMISSION,
        ),
        "move" => (
            A_FILE_THIS_JOB_NEEDS_WAS_MOVED,
            IT_MAY_HAVE_BEEN_MOVED,
            WAS_MOVED_AFTER_SUBMISSION,
            MAY_HAVE_BEEN_MOVED_AFTER_SUBMISSION,
        ),
        "rename" => (
            A_FILE_THIS_JOB_NEEDS_WAS_RENAMED,
            IT_MAY_HAVE_BEEN_RENAMED,
            WAS_RENAMED_AFTER_SUBMISSION,
            MAY_HAVE_BEEN_RENAMED_AFTER_SUBMISSION,
        ),
        _ => (
            A_FILE_THIS_JOB_NEEDS_WAS_REPLACED,
            IT_MAY_HAVE_BEEN_REPLACED,
            WAS_REPLACED_AFTER_SUBMISSION,
            MAY_HAVE_BEEN_REPLACED_AFTER_SUBMISSION,
        ),
    }
}
// **A delete that took more than one file from a job's folder**: one mark,
// the count and the folder — the build runner's ruling (d) of 2026-10-03,
// `Design/Queue & History § The marks`.
const FILES_THIS_JOB_NEEDS_WERE_DELETED: &str = "Files this job needs were deleted.";
const FILES_THIS_JOB_NEEDS_MAY_HAVE_BEEN_DELETED: &str = "Files this job needs may have been deleted.";
const FILES_WERE_DELETED_FROM: &str = "files were deleted from";
const FILES_MAY_HAVE_BEEN_DELETED_FROM: &str = "files may have been deleted from";
const AFTER_THIS_JOB_WAS_SUBMITTED: &str = "after this job was submitted.";
const NOTHING_CHECKED_THIS_FILES_DEPENDENCIES: &str =
    "Dependencies were not checked before submission.";
const THE_CHECK_WAS_MADE_WITH_ANOTHER_BLENDER: &str =
    "This file's dependency check was made with a different Blender version.";
const THE_CHECK_CARRIED_A_REPLACED_MARK: &str =
    "This file's dependency check already included a replaced file.";
const CARRIES_ON_FROM: &str = "Resumes";
const SUBMITTED_WITH_NO_SCENE_NAMED: &str = "Submitted with no scene named.";
const BLENDER_RENDERED_LEAD: &str = "Blender rendered";
const THE_FILES_ACTIVE_SCENE: &str = ", the file's active scene.";
const IT_RENDERS_WHICHEVER_SCENE: &str = "It renders the file's active scene.";
const THE_FILE_CHANGED_BEFORE_ATTEMPT: &str = "The file changed before attempt";
const FRAMES_ALREADY_FINISHED_WERE_REUSED: &str =
    "Frames already finished were reused.";
const FRAMES_ALREADY_FINISHED_WERE_RENDERED_AGAIN: &str =
    "Frames already finished were rendered again.";
/// **Design, *The marks*, the seventh** (`Queue & History` N23): ***The
/// NVIDIA driver, 525.147.05, was too old for OptiX in Blender 5.1**, and
/// Smoozy couldn't update it.* The two versions travel as values between
/// these words. Its last words are the server card's own (`Design/Server
/// Setup § This server`). Held-job piece B, 2026-10-12.
const THE_NVIDIA_DRIVER: &str = "The NVIDIA driver,";
const WAS_TOO_OLD_FOR_OPTIX_IN_BLENDER: &str = ", was too old for OptiX in Blender";
/// **Where the driver gave no version** — it no longer answered, one of the
/// ways `Server Setup` R1c releases after a run that did not bring it up.
/// Design words only the case with a version; this is that sentence less the
/// value it has not got. Ruled at this site.
const THE_NVIDIA_DRIVER_WAS_TOO_OLD_FOR_OPTIX_IN_BLENDER: &str =
    "The NVIDIA driver was too old for OptiX in Blender";
/// **The two again from 5.3**, where the card cannot be used at all under Cycles
/// (`Rendering` L23a's bullet of 2026-11-02).
const WAS_TOO_OLD_FOR_CYCLES_GPU_IN_BLENDER: &str = ", was too old for Cycles GPU rendering in Blender";
const THE_NVIDIA_DRIVER_WAS_TOO_OLD_FOR_CYCLES_GPU_IN_BLENDER: &str =
    "The NVIDIA driver was too old for Cycles GPU rendering in Blender";
const THE_BOLD_ENDS: &str = ",";
const AND_SMOOZY_COULDNT_UPDATE_IT: &str = "and could not be updated.";

/// **Design, *The marks*, the fifth: it was interrupted and re-run, and from
/// which frame.** `Queue & History` N1 holds what it needs — the attempt count
/// and the frame the re-run proceeded from — and this stage supplies the
/// words, Design giving none of its own beyond *interrupted and re-run*.
///
/// **The drawing's second sentence is not composed and cannot be.** It reads
/// *"The server went down at 00:52 and started the job again at 01:09"*, and
/// `Queue & History` N1's mark carries neither moment: what it holds is the attempt and the
/// frame. One line, true of this job, and no moment invented for it.
const INTERRUPTED_AND_RE_RUN: &str = "Interrupted and re-run";
const FROM_FRAME: &str = "from frame";

/// The setup group's own field names — Design, *The rest of the entry*, and
/// the drawing's own six rows in the drawing's own order.
const BLENDER_FACT: &str = "Blender";
const FRAMES_FACT: &str = "Frames";
const RESOLUTION_FACT: &str = "Resolution";
const SAMPLES_FACT: &str = "Samples";
const ENGINE_FACT: &str = "Engine";
/// **`Queue & History` E12a's ask**, under the settings page's own label for
/// the same choice — one name for one setting.
const RENDERS_ON_FACT: &str = crate::surface::render_settings::words::RENDERS_ON;
const WRITING_TO_FACT: &str = "Writing to";
/// **The multiplication sign the drawing uses between the two pixel figures**,
/// and it travels as the trail of the first rather than joined to either (X7).
const BY_SIGN: &str = "×";
const A_PER_CENT: &str = "%";

// ───────────────────────────────────────────────────────────────────────────
// The reading
// ───────────────────────────────────────────────────────────────────────────

/// **The five endings, and the fifth is the absence of one** (C11).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnEnding {
    Finished,
    Failed,
    Stopped,
    GivenUp,
    /// **Not an ending at all**, and the word carries the difference rather
    /// than the colour.
    NotKnown,
    /// **Item C1: Smoozy never heard whether the job started**, and its server
    /// is gone — `Queue & History` C9's second way in. Not an ending either,
    /// and one step further back than [`AnEnding::NotKnown`].
    StartNotKnown,
    /// **E1-1: started, no ending learned, and its server's row still
    /// stands** — `Queue & History` C9 puts the job here from its render
    /// start, so a job still rendering is in History before it has an ending.
    /// Not an ending either, and not *not known*: its server can still be
    /// asked, and the ending comes in by `Queue & History` C11 when it is
    /// learned. **[`AnEnding::NotKnown`] is the same record once its server's
    /// row is gone.**
    ///
    /// **The History list leaves it out** — the Creator's second notes, piece
    /// 7 (note 11), `Queue & History` C9 as narrowed 2026-10-04: a running
    /// job is in Queue only. It is still the queue's entry state, since the
    /// queue builds its entry with [`the_entry_for`].
    InProgress,
}

impl AnEnding {
    fn said(self) -> &'static str {
        match self {
            AnEnding::Finished => FINISHED,
            AnEnding::Failed => FAILED,
            AnEnding::Stopped => STOPPED,
            AnEnding::GivenUp => GIVEN_UP,
            AnEnding::NotKnown => ENDING_NOT_KNOWN,
            AnEnding::StartNotKnown => START_NOT_KNOWN,
            AnEnding::InProgress => IN_PROGRESS,
        }
    }

    /// The drawing's own class for the word's colour.
    fn class(self) -> &'static str {
        match self {
            AnEnding::Finished => "e-ok",
            AnEnding::Failed => "e-fail",
            AnEnding::Stopped => "e-stop",
            AnEnding::GivenUp => "e-gave",
            // **Design: the same grey italic**, so the same class.
            AnEnding::NotKnown | AnEnding::StartNotKnown => "e-unk",
            // **E1-1 gives *In progress* no colour of its own and it takes
            // none**: not an ending, so none of the four endings' colours, and
            // not the grey italic Design keeps for what Smoozy cannot confirm.
            // The word is drawn in the list's own ink. Ruled by this piece's
            // builder; recorded in its account.
            AnEnding::InProgress => "",
        }
    }

    /// **The class of an ending Smoozy learned**, for the queue's count — the
    /// Creator's ninth notes, note 3: each word of the count is in its History
    /// colour, so it takes the same class History's own word takes, from this
    /// one place.
    pub(crate) fn the_class_of(ending: Ending) -> &'static str {
        match ending {
            Ending::Finished => AnEnding::Finished,
            Ending::Failed => AnEnding::Failed,
            Ending::Stopped => AnEnding::Stopped,
            Ending::GivenUp => AnEnding::GivenUp,
        }
        .class()
    }

    /// **Whether the History list draws an entry with this reading** — the
    /// Creator's second notes, piece 7 (note 11). `Specs/Queue & History`
    /// *History* and `Queue & History` C9 as narrowed 2026-10-04: a job shows
    /// in the list once it has an ending, or once its server is gone (*Ending
    /// unknown*, *Unknown if started*). **A job started and still
    /// running on a standing server is in Queue only**, and its record is kept
    /// and filled in as before. The list, its count, the boxes and the acts
    /// all read this one test, so nothing in History can reach a running job's
    /// record: every entry drawn may be deleted and cleared
    /// (`Queue & History` X1, `Queue & History` X6, `Queue & History` X6a).
    fn the_list_draws_it(self) -> bool {
        self != AnEnding::InProgress
    }

    /// **Design: an entry that failed, was given up, or was stopped carries the
    /// whole-app failure shape.** A clean finish does not, and neither does an
    /// ending Smoozy never learned — the shape is for a job that did not do what
    /// it was asked, and one of those did and the other is not an ending.
    fn carries_the_failure_shape(self) -> bool {
        matches!(
            self,
            AnEnding::Failed | AnEnding::Stopped | AnEnding::GivenUp
        )
    }
}

/// One row of the history list — Design, *History*: three columns.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AHistoryRowDrawn {
    /// **The job's own identity** — what choosing the row hands back, and what
    /// a box is composed over. **Never its place in the list**, which moves
    /// when a job elsewhere starts rendering (must-do 1). A value, never shown.
    pub job: String,
    /// **The entry's copied local server identifier** — `Queue & History` N12
    /// — which *this server* means when clearing. A value, never shown.
    pub server_row: String,
    /// The job's whole label (`Queue & History` Y8). A value.
    pub name: String,
    /// **A first-frame pair's check, under the name** — *First-frame check ·
    /// frame 1* — on the pair's one row and on a check drawn alone
    /// (`Design/Queue & History § A first-frame check and its job`). Empty on
    /// every other row. The nineteenth notes, note 7.
    pub the_check: Vec<crate::surface::the_queue::APartOfThePair>,
    /// **What the row reads where the ending goes, in grey, in place of the
    /// ending**: *In Queue* for a check whose full range is still live, and
    /// *Check only* for a finished check whose full range never rendered.
    pub in_place_of_the_ending: Option<&'static str>,
    /// **A check whose full range is still live** — not deleted alone, nor
    /// by a clearing, because the waiting question shows its picture (`Queue
    /// & History` X1a). A value, never shown.
    pub the_check_waits_on_its_range: bool,
    /// **`Queue & History` N6: the last frame Smoozy confirmed, not the range it was submitted
    /// with.** Absent where the ending was never learned.
    pub frames: Option<(String, String)>,
    pub ending: Option<AnEnding>,
    pub server: String,
    pub badge: Option<String>,
    pub scene: Option<String>,
    /// How long it rendered. A value; absent where it is not known.
    pub how_long: Option<String>,
    /// **When it ended, in seconds since the epoch, and the page writes it.** A
    /// value. **Absent where no moment is known at all** — a job whose ending
    /// Smoozy never learned and whose record predates `Queue & History` N16's
    /// submitted moment has none, and absent draws as absent.
    ///
    /// **The application does not write the date and cannot.** Choosing between
    /// *today 04:12*, *8 Aug 23:14* and a year is a comparison against now on
    /// the reader's own computer, and the words are that computer's locale
    /// data; every other screen in this application already sends the moment
    /// and lets the page write it.
    pub when: Option<String>,
    /// **The ending is a reading's, owed to a full store** ([`what_is_kept_beside`]):
    /// a clearing does not reach the entry and does not count it. A value,
    /// never shown.
    pub the_ending_is_owed: bool,
    /// **The job's server is no longer in Smoozy** — `Queue & History` N17's
    /// draw-time lookup finds no row for the copied identifier. The row sits
    /// in the *Removed servers* group at the foot, with the grey badge
    /// (Design, *History*; the eighteenth notes, note 12). A value, never shown.
    pub the_server_is_removed: bool,
}

impl AHistoryRowDrawn {
    /// **Whether a clearing takes this entry**: not one whose ending a reading
    /// carries while it is owed, and not a check whose full range is still
    /// live (`Queue & History` X1a).
    fn a_clearing_takes_it(&self) -> bool {
        !self.the_ending_is_owed && !self.the_check_waits_on_its_range
    }
}

/// **What this screen hands the one failure shape** — §16, and the shape is
/// [`crate::presentation::the_failure_shape`]'s. **Nothing here builds a shape
/// of its own.**
///
/// **The mockup draws that shape eleven times in five modules with five class
/// prefixes and this screen copies none of them.** What it composes is the
/// parts: the plain sentence, the second sentence where Smoozy actually knows,
/// the values beside it, and the controls in the shape's own order.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TheEndingsFailure {
    /// The plain sentence, **before the typography point** — which is what M32
    /// captures and what `Reporting a problem` X4 is handed.
    pub said: String,
    /// **The disk-full carve-out's second sentence, and nothing else ever fills
    /// this** (M14, M15a). *"The diagnosis is shown alongside the machine's own
    /// words and never instead of them."*
    pub what_to_do_about_it: Option<String>,
    /// Every value the sentence would otherwise have had inside it, each beside
    /// its own label — the label is prose, the value is not (X7, must-do 20).
    pub values: Vec<(&'static str, String)>,
    /// **The reason the machine recorded for this ending, whole** — `Rendering`
    /// K14, kept beside the record at version 27. **It is the raw text behind
    /// *What the server said* and never part of the sentence**: the plain
    /// sentence names no cause (Design, *The rest of the entry*), and the
    /// whole-app shape puts a server's own words one control away, the control
    /// naming whose words they are. Step 10c's sixth pass, fault 3: the machine
    /// recorded why both jobs failed and no screen said it.
    pub what_the_server_said: Option<String>,
    /// **Where no render began, that same reason, said at once beneath the plain
    /// sentence** (Design, *Where no render began*; Rule 8's truth over *Nothing else
    /// on a failed entry names a cause*, for this case alone). It is what Smoozy
    /// itself was doing, not a cause read from Blender's words. **A value, and never
    /// through the typography point** (M9), because it can hold a path as the file
    /// records it. [`Self::what_the_server_said`] is then empty: one reason, one place.
    pub the_reason_beneath: Option<String>,
    /// Whether the record opens the machine's own words (M13a). **The control
    /// and never a line of either stream** (M13b).
    pub blender_output: bool,
    /// Whether the report entry point is offered — M33, and Design draws it on
    /// every ending but the one the user caused.
    pub report_a_problem: bool,
}

impl TheEndingsFailure {
    /// **The sentence as the diary keeps it — before X7's substitutions.**
    ///
    /// M32: the captured string is the composed sentence before the typography
    /// point, because the diary is not a screen.
    pub fn as_the_diary_keeps_it(&self) -> String {
        self.said.clone()
    }

    /// **The controls, in the shape's own order**, composed here and never on
    /// the page — F-13's repair, kept.
    ///
    /// **No cross**: a recorded ending is not a live condition, so there is
    /// nothing on it to clear. **No retry either**: Design is explicit that the
    /// act that would try again is the *Render it again* group further down the
    /// entry, *"so the failure does not repeat it"*.
    pub fn the_controls(&self) -> Vec<AControl> {
        let mut controls = Vec::new();
        // **The server's own words first**: where the machine recorded a
        // reason, it is the raw text that has words in it, and the shape's
        // order puts raw text before reporting.
        if self.what_the_server_said.is_some() {
            controls.push(AControl {
                what_it_is: WhatItIs::TheRawText,
                label: Prose::new(WHAT_THE_SERVER_SAID),
                act: Verbatim::new(THE_SERVERS_WORDS_ACT),
            });
        }
        if self.blender_output {
            controls.push(AControl {
                what_it_is: WhatItIs::TheRawText,
                label: Prose::new(TheSharedWords::BLENDER_OUTPUT),
                act: Verbatim::new("blender-output"),
            });
        }
        if self.report_a_problem {
            controls.push(AControl {
                what_it_is: WhatItIs::ReportAProblem,
                label: Prose::new(REPORT_A_PROBLEM),
                act: Verbatim::new(""),
            });
        }
        controls
    }
}

/// **The control that opens the server's own words**, in the words every other
/// screen already uses for it (`crate::surface::transfers::words`).
const WHAT_THE_SERVER_SAID: &str = crate::surface::transfers::words::WHAT_THE_SERVER_SAID;
/// The act that control hands back, which `web/the-history.js` opens the panel
/// on.
const THE_SERVERS_WORDS_ACT: &str = "what-the-server-said";

/// **M33's own label**, which the shared shape supplies for a failure the
/// plumbing composes and which this screen supplies for one it composes itself.
const REPORT_A_PROBLEM: &str = "Report a problem…";

/// The two ways to render it again — Design, *The two ways to render it
/// again*.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RenderItAgain {
    /// *Submit again — 240 frames*, always offered (`Rendering` A1 and
    /// `Rendering` A1a): how many frames the job asked for.
    pub submit_again: usize,
    /// *Resume — 181 frames*, and the frame it starts at. **Absent rather
    /// than greyed** where there is nothing to start from.
    pub carry_on: Option<TheCarryOn>,
    /// Why the second control is missing, where it is.
    pub why: Option<&'static str>,
    /// **`Queue & History` E3: a finish whose count came up short says this in
    /// place of [`Self::why`]**, the outcome staying *Finished*.
    pub the_count_came_up_short: Option<smoozy_queue_history::acting::note::TheFilesCounted>,
}

impl RenderItAgain {
    /// **`Queue & History` E3 over one finish**: where the count came up short,
    /// the line is the count. No count, and a whole one, leave it as it is.
    fn over_a_count(mut self, counted: Option<smoozy_queue_history::acting::note::TheFilesCounted>) -> RenderItAgain {
        if let Some(counted) = counted.filter(|counted| counted.it_came_up_short()) {
            self.why = None;
            self.the_count_came_up_short = Some(counted);
        }
        self
    }

    /// **The line, in words.** The figures come from `Job files and outputs`
    /// C27, so nothing here reckons them.
    pub(crate) fn the_line(&self) -> Option<String> {
        match (self.the_count_came_up_short, self.why) {
            (Some(counted), _) => Some(format!(
                "{} {FILES_WORD_OF} {} {}",
                counted.written,
                counted.owed,
                if counted.owed == 1 { ONE_FILE_WRITTEN_TRAIL } else { FILES_WRITTEN_TRAIL },
            )),
            (None, Some(why)) => Some(why.to_string()),
            (None, None) => None,
        }
    }
}

/// **The carry-on, as the architecture reckons it** — must-do 2 of step 10a
/// round 1 of the queue, history, settings and downloads.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheCarryOn {
    /// How many of the job's asked frames the carried-on range holds: the
    /// boundary frame and every frame the job asked for after it, in the job's
    /// own asked order (`Queue & History` N6, `Rendering` A4).
    pub frames: usize,
    /// **The frame the range opens at** — the boundary frame itself, which is
    /// the one Smoozy cannot confirm finished. The field's starting value and
    /// the *why* line's frame; the page adds nothing to it.
    pub at: i64,
    pub came_from: WhereTheFigureCameFrom,
}

/// **Where the carry-on's figure came from** (`Rendering` A4 and `Rendering` A9).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WhereTheFigureCameFrom {
    /// Read from the machine when the action was taken.
    ReadFromTheServerJustNow,
    /// `Queue & History` N6's figure, kept when the job ended. **Nothing can
    /// read the server yet, so this is the one drawn.**
    #[default]
    RecordedWhenTheJobEnded,
}

/// What the history entry adds to the shared entry shape.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TheHistoryEntry {
    pub entry: TheJobEntry,
    /// **History's own *Blender output*, on the one ending that carries no
    /// failure shape to hold it** — a clean finish, and only where the record
    /// keeps the job's words (`Queue & History` E9). **It is History's and not
    /// the shared entry's**, so the shared entry's own control — which reads
    /// a running job's words — is never drawn here. The Creator's notes on
    /// 0.1.8, piece 6d.
    pub blender_output: bool,
    pub ending: Option<AnEnding>,
    pub failure: Option<TheEndingsFailure>,
    pub again: Option<RenderItAgain>,
    /// The *Rendered* group — the times and the devices (`Queue & History` N16,
    /// E12 and E13). **Up to two parts a line**, because the last-frame line
    /// carries two figures and neither goes inside the words between them.
    pub rendered: Vec<ARenderedFact>,
    /// One line under the times, where the record can say no more.
    pub rendered_note: Option<&'static str>,
    /// **What the delete box says is left untouched**, read off the record with
    /// the rest of the entry — `Design/Queue & History § Deleting an entry`.
    pub what_deleting_leaves: WhatDeletingLeaves,
    /// **The ending is a reading's, owed to a full store**
    /// ([`what_is_kept_beside`]), so *Delete* is not offered over it.
    pub the_ending_is_owed: bool,
    /// **A check whose full range is still live**, so *Delete* is not offered
    /// over it (`Queue & History` X1a). The nineteenth notes, note 7.
    pub the_check_waits_on_its_range: bool,
    /// **The entry is a first-frame pair's, and *Delete* deletes the pair
    /// together** — its box asks *Delete this entry and its first-frame
    /// check?* (`Queue & History` X1a).
    pub deleted_with_its_check: bool,
}

/// One line of the *Rendered* group.
///
/// **`a_moment` is the whole of why this is a type rather than a pair.** Three
/// of the six lines are moments, and **the application does not write a date**:
/// the moment goes out as seconds since the epoch and `web/the-history.js`
/// writes it with the reader's own locale data, exactly as every other screen
/// does. **The page cannot tell a moment from any other figure**, so the line
/// says which it is.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ARenderedFact {
    pub name: &'static str,
    pub parts: Vec<APart>,
    /// Whether the line's one value is a moment in seconds for the page to
    /// write as a date.
    pub a_moment: bool,
}

/// The three facts [`the_delete_box`] is composed from.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct WhatDeletingLeaves {
    pub frames_on_the_server: Option<u32>,
    pub the_server_is_gone: bool,
    pub downloaded: Option<u32>,
}

/// **What the page asked this screen to put in front of the user.** H34: a
/// closed request type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TheHistoryAsk {
    /// The chosen entry's *Delete*, from its control or from the delete key —
    /// `Design/Queue & History § Keyboard shortcuts`: *"The delete key deletes
    /// a history entry, asking as the control does."*
    Delete,
    /// *Clear this server's history* — the server of the entry that is open.
    ClearThisServer,
    /// *Clear all history*.
    ClearAll,
    /// ***Clear removed servers' history*** — `Queue & History` X6b, under the
    /// *Removed servers* heading. The eighteenth notes, note 13.
    ClearRemovedServers,
    /// **The entry's *Blender output*** — `Queue & History` E9's kept words,
    /// opened in the one panel. Not a box: the answer carries the panel's
    /// words with the history. The Creator's notes on 0.1.8, piece 6d.
    BlenderOutput,
    /// **The *Blender output* in a pair's box: the check's own kept words**
    /// (`Design/Queue & History § A first-frame check and its job`). Not a
    /// box either. The nineteenth notes, note 7.
    TheChecksBlenderOutput,
}

/// **The whole of what this screen hands the page** — H34's closed response
/// type.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TheHistoryOnScreen {
    pub jobs: u32,
    pub rows: Vec<AHistoryRowDrawn>,
    /// **The record would not open or the read failed** (must-do 3(a)). Said
    /// in place of the list, never drawn as an empty History.
    pub could_not_read: bool,
    /// **How many rows in history would not read back** (must-do 3(c)), said
    /// beside the list.
    pub could_not_be_read: u32,
    /// **The chosen entry's job identity** — never a place in the list.
    pub chosen: Option<String>,
    pub entry: Option<TheHistoryEntry>,
    pub asking: Option<TheBoxThatAsks>,
    /// **The chosen entry's kept words, where the page asked for them** —
    /// [`TheHistoryAsk::BlenderOutput`]. Absent on every other answer, so the
    /// words travel once, when they are asked for.
    pub words: Option<TheKeptWords>,
}

/// **The words the record keeps for one job, as the panel opens them** —
/// `Queue & History` E9 and E11: each attempt's own two streams, apart, in
/// attempt order, every tab one of M10's four or M10a's fifth.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheKeptWords {
    /// What the job is called and the server it ran on — the panel's own line
    /// under its title. Values.
    pub job: String,
    pub server: String,
    pub attempts: Vec<(u32, crate::presentation::the_machines_own_words::ThePanel)>,
}

/// **Each attempt's panel, from the streams the record keeps.** The words are
/// the record's own bytes and are not read for anything; a byte that is not
/// text is drawn as the replacement mark and the record keeps it as it came.
pub fn the_panels_of(
    kept: &[crate::record::kept_words::AStreamKept],
) -> Vec<(u32, crate::presentation::the_machines_own_words::ThePanel)> {
    use crate::presentation::the_machines_own_words::{the_panel, AStreamsRecord, Stream};
    let mut attempts: Vec<u32> = kept.iter().map(|one| one.attempt).collect();
    attempts.dedup();
    attempts
        .into_iter()
        .map(|attempt| {
            let records: Vec<AStreamsRecord> = Stream::BOTH
                .iter()
                .filter_map(|stream| {
                    kept.iter()
                        .find(|one| one.attempt == attempt && one.stream == *stream)
                })
                .map(|one| AStreamsRecord {
                    stream: one.stream,
                    outcome: one
                        .outcome
                        .map(smoozy_queue_history::acting::words::which_silence_it_was),
                    as_read_back: String::from_utf8_lossy(&one.the_words).into_owned(),
                    kept: one.retained,
                    dropped: one.discarded,
                    why_the_rest_went: one.why_the_rest_went.clone(),
                })
                .collect();
            (attempt, the_panel(&records))
        })
        .collect()
}

/// **The chosen entry's kept words, read from the record when the page asks
/// for them** — the one moment the bytes leave the record. **Nothing where
/// none are kept**, and the page then has no panel to open, which is never an
/// empty one.
pub fn the_kept_words(records: Option<&LocalRecord>, says: &TheHistoryOnScreen) -> Option<TheKeptWords> {
    let entry = says.entry.as_ref()?;
    the_kept_words_of(records, &entry.entry.job, &entry.entry.name, &entry.entry.server)
}

/// **One job's kept words, under the name and server the panel's line
/// gives** — the entry's own, or the check's in a pair's box, which is named
/// as the pair is (the nineteenth notes, note 7).
pub(crate) fn the_kept_words_of(
    records: Option<&LocalRecord>,
    job: &str,
    name: &str,
    server: &str,
) -> Option<TheKeptWords> {
    use smoozy_state::identities::IdentityKind;
    let job = Identity::read_back(IdentityKind::Job, job)?;
    let kept = crate::record::kept_words::the_words_kept_for(records?.connection(), &job).ok()?;
    let attempts = the_panels_of(&kept);
    (!attempts.is_empty()).then(|| TheKeptWords {
        job: name.to_string(),
        server: server.to_string(),
        attempts,
    })
}

/// **What is kept, read from the record.**
///
/// The history record is `State, identity and recovery` D4's second exception —
/// Smoozy's own note, durable, user-held — and since record version 23 the
/// local record has a table for it: `src/record/job_records.rs`.
///
/// **`Queue & History` C9 decides which records are here and it is not the
/// standing**: a job is in history from the moment Smoozy learns it started
/// rendering, so a provisional record and a confirmed record whose render start
/// has not been learned are both in no list at all.
///
/// **Every figure below is the record's own and nothing is composed from a
/// machine.** No connection is opened and no reading is taken, which is what
/// Design means by *"history is Smoozy's own record and is there straight
/// away."*
///
/// **`chosen` is the job the user chose, by its identity** — *"Return opens
/// the chosen entry"* and the drawing's own press on a row — and the entry
/// beside the list is that job's.
///
/// **A record that would not open, or a read that failed, is said** (must-do
/// 3(a)): the answer carries [`TheHistoryOnScreen::could_not_read`] and no
/// rows, and the page draws the sentence in place of the list. Until then both
/// drew the empty History a person who never rendered sees.
pub fn what_is_kept(records: Option<&LocalRecord>, chosen: Option<&str>) -> TheHistoryOnScreen {
    what_is_kept_over(records, chosen, &|_| true)
}

/// [`what_is_kept`], with **which servers are connected** — the one thing the
/// entry draws that is not the record's own. *Files on the server* is not
/// drawn on an entry whose server stands and is not connected
/// ([`crate::surface::the_queue::TheJobEntry::the_server_is_not_connected`]),
/// and comes back when it connects: History is drawn again then
/// ([`crate::surface::the_queue::the_queue_is_told_again`]). Step 10c's *Files
/// on the server* follow-up.
pub fn what_is_kept_over(
    records: Option<&LocalRecord>,
    chosen: Option<&str>,
    is_connected: &dyn Fn(&str) -> bool,
) -> TheHistoryOnScreen {
    what_is_kept_beside(records, chosen, is_connected, &[])
}

/// **An ending a period-held reading carries while it is owed to a full
/// store** — `Rented-server runtime` S14 and `Rented-server runtime` S14a — with the server whose
/// reading carries it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnEndingTheReadingCarries {
    /// The server's local identifier, as the run holds its period.
    pub server: String,
    pub job: Identity,
    pub owed: smoozy_queue_history::acting::note::TheEndingOwed,
}

/// **Every ending owed that the run's readings carry**, server by server —
/// what [`what_is_kept_beside`] draws a job's ending from while the record has
/// none. Read off the period-held readings alone (`Queue & History` C6), and
/// written nowhere.
pub fn the_endings_owed_in(
    holds: &crate::servers::the_machines_work::WhatThisRunHolds,
) -> Vec<AnEndingTheReadingCarries> {
    holds
        .every_server()
        .into_iter()
        .flat_map(|(server, period)| {
            period
                .reading
                .map(|reading| reading.value().jobs.clone())
                .unwrap_or_default()
                .into_iter()
                .filter_map(move |job| {
                    let owed = job.live.the_ending_owed()?.clone();
                    Some(AnEndingTheReadingCarries { server: server.clone(), job: job.job, owed })
                })
        })
        .collect()
}

/// [`what_is_kept_over`], **with the endings the run's readings carry while
/// they are owed to a full store** ([`the_endings_owed_in`]).
///
/// **`Queue & History` C11: when the job ends it leaves the queue and the
/// history entry takes the ending Smoozy learns — from watching, or from the
/// note.** On a full disk the machine knows the ending and cannot write it
/// (`Rented-server runtime` S14), so the queue reading gives the job as
/// *ended* with the outcome beside it, marked owed (`Rented-server runtime`
/// S14a), and the Queue lets it go. **Until the landing the record has no
/// ending, so the entry is drawn here from the reading**, with the ending the
/// reading carries and `Rented-server runtime` V12's reading behind the
/// disk-full sentence — *"carrying it forward is what lets the disk-full
/// diagnosis reach the user inside the window"* (`Rented-server runtime`
/// S14a). **The landing replaces
/// it with the record**: once the note's ending is learned, the record's
/// ending is drawn and the reading's is not read. Until step 12's SC4
/// follow-up the job was on neither list for as long as the disk stayed full.
///
/// **Nothing durable is written from the reading** (`Rented-server runtime`
/// S14a: *a reading is never evidence that a record landed*). **So an entry
/// drawn from it is not reached by *Delete* or by a clearing**: those are acts
/// on the record, whose job is still running by the record's own account, as
/// [`the_act_is_performed`] reads it. Neither is offered over it, and a
/// clearing's count leaves it out.
pub fn what_is_kept_beside(
    records: Option<&LocalRecord>,
    chosen: Option<&str>,
    is_connected: &dyn Fn(&str) -> bool,
    owed: &[AnEndingTheReadingCarries],
) -> TheHistoryOnScreen {
    let the_ending_owed_for = |record: &LocalJobRecord| {
        owed.iter()
            .find(|one| one.job == record.job && one.server == record.server.local_server.as_text())
            .map(|one| &one.owed)
    };
    let mut kept = TheHistoryOnScreen::default();
    let Some(record) = records else {
        kept.could_not_read = true;
        return kept;
    };
    let Ok(held) = JobRecords::over(record.connection()).in_history() else {
        kept.could_not_read = true;
        return kept;
    };
    kept.could_not_be_read = held.could_not_be_read;
    // **E1-1 turns on whether the entry's server row still stands** —
    // `Queue & History` N17's draw-time lookup, read once for every row.
    let standing = the_server_rows_that_stand(records);
    // **Piece 7: a running job on a standing server is not in the list**, and
    // the count below is this list's own.
    let mut held: Vec<_> = held
        .entries
        .into_iter()
        .filter(|one| {
            let stands = standing.contains(&one.record.server.local_server.as_text());
            the_ending_drawn(&one.record, one.start_never_heard, stands, the_ending_owed_for(&one.record))
                .0
                .the_list_draws_it()
        })
        .collect();
    // **Newest first**, by the moment the job ended, and a record with no
    // ending moment sorts by its submitted moment. Neither is subtracted from
    // the other: the comparison is on one moment each and `Queue & History`
    // N4's one duration is untouched.
    held.sort_by_key(|one| {
        std::cmp::Reverse(
            one.record
                .what_the_note_carried
                .ended
                .as_ref()
                .or(one.record.submitted_at.as_ref())
                .map(smoozy_state::observed::Reading::taken_at),
        )
    });
    // **Design, *History*: removed servers' jobs sit together at the foot**,
    // keeping the list's own order inside the group (the eighteenth notes,
    // note 12). The test is `Queue & History` N17's, read once above.
    held.sort_by_key(|one| !standing.contains(&one.record.server.local_server.as_text()));
    // **`Design/Queue & History § A first-frame check and its job`: a pair is
    // one row in History too**, the full range's, and counts once — so a
    // check whose full range is in the list is not a row of its own. The
    // nineteenth notes, note 7.
    let store = JobRecords::over(record.connection());
    let drawn: std::collections::HashSet<String> = held.iter().map(|one| one.record.job.as_text()).collect();
    let is_drawn = |job: &Identity| drawn.contains(&job.as_text());
    let parts: Vec<AsAPart> = held.iter().map(|one| as_a_part(&store, &one.record, &is_drawn)).collect();
    let rows: Vec<(&_, &AsAPart)> = held
        .iter()
        .zip(parts.iter())
        .filter(|(_, part)| !matches!(part, AsAPart::TheHiddenCheck(_)))
        .collect();
    kept.jobs = rows.len() as u32;
    kept.rows = rows
        .iter()
        .map(|(one, part)| {
            let stands = standing.contains(&one.record.server.local_server.as_text());
            let mut row = a_history_row(&one.record, one.start_never_heard, stands);
            // **The ending a reading carries, while it is owed** — the row's
            // other figures are the record's, which has none of the ending's.
            if let (ending, Some(_)) =
                the_ending_drawn(&one.record, one.start_never_heard, stands, the_ending_owed_for(&one.record))
            {
                row.ending = Some(ending);
                row.the_ending_is_owed = true;
            }
            part.is_drawn_on(&mut row, &one.record);
            row
        })
        .collect();
    // **A check drawn on its pair's row is chosen as that row.**
    let chosen = chosen.map(|job| {
        parts
            .iter()
            .find_map(|part| match part {
                AsAPart::TheHiddenCheck(pair) if pair.check.as_text() == job => Some(pair.range.as_text()),
                _ => None,
            })
            .unwrap_or_else(|| job.to_string())
    });
    kept = the_row_is_chosen(kept, chosen.as_deref());
    // **The entry beside the list, composed from that job's own record** —
    // step 10b piece 6a, and until it landed this field was set by nothing but
    // tests, so the entry column was blank, the header's *Delete* was not drawn
    // and the delete key opened no box.
    if let Some(chosen) = kept.chosen.clone() {
        if let Some(one) = held.iter().find(|one| one.record.job.as_text() == chosen) {
            let stands = standing.contains(&one.record.server.local_server.as_text());
            // **What is kept beside the record is read for the one entry
            // drawn**, and a read that fails draws it as a record with none.
            let beside = BesideTheRecord::read(record.connection(), &one.record.job);
            let mut entry = the_entry_beside_a_reading(
                &one.record,
                &beside,
                one.start_never_heard,
                stands,
                the_ending_owed_for(&one.record),
            );
            entry.entry.the_server_is_not_connected =
                stands && !is_connected(&one.record.server.local_server.as_text());
            // **History's *Writing to* is the queue's same link** (`Design/Queue
            // & History`, *The job's name, and its folder*, which covers both
            // lists); the event says so only where *Files on the server* is
            // offered. The Creator's seventh notes, note 5.
            crate::surface::the_queue::the_writing_to_path_opens_files(&mut entry.entry);
            // **The check on its pair's entry, and a check drawn alone**
            // (`Design/Queue & History § A first-frame check and its job`).
            let part = as_a_part(&store, &one.record, &is_drawn);
            part.is_drawn_on_the_entry(record.connection(), &mut entry);
            kept.entry = Some(entry);
        }
    }
    kept
}

/// **One History record's part in a first-frame pair** — `Design/Queue &
/// History § A first-frame check and its job`, read from the saved purpose
/// (`Queue & History` P5a) on the record and its partner's. The nineteenth
/// notes, note 7.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum AsAPart {
    /// No saved purpose, or a partner whose own names another pair: an
    /// ordinary row. **Older records stay as they were.**
    NotAPart,
    /// **A check whose full range is in the list**: drawn on the full range's
    /// row, not as a row of its own.
    TheHiddenCheck(ThePair),
    /// **The full range, with its check in the list**: the pair's one row,
    /// with the check's ending beside it.
    TheRangeWithItsCheck(ThePair, Option<Ending>),
    /// **A check drawn alone**: its full range is still live, or never
    /// rendered.
    TheCheckAlone(ThePair, TheRangeInHistory, Option<Ending>),
}

/// **`record`'s part in a pair**, `is_drawn` saying which records the list
/// draws.
pub(crate) fn as_a_part(
    store: &JobRecords<'_>,
    record: &LocalJobRecord,
    is_drawn: &dyn Fn(&Identity) -> bool,
) -> AsAPart {
    let Some(pair) = ThePair::of(store, &record.job) else {
        return AsAPart::NotAPart;
    };
    let the_check = pair.is_the_check(&record.job);
    let partner = if the_check { &pair.range } else { &pair.check };
    let partners_record = store.read(partner).ok().flatten();
    // **The partner's own saved purpose must name the same pair**, where its
    // record stands; one that names another is no partner.
    if partners_record.is_some() && ThePair::of(store, partner).is_none_or(|theirs| theirs != pair) {
        return AsAPart::NotAPart;
    }
    if the_check {
        if is_drawn(&pair.range) {
            return AsAPart::TheHiddenCheck(pair);
        }
        let range = if partners_record.is_some() { TheRangeInHistory::InQueue } else { TheRangeInHistory::NeverRendered };
        return AsAPart::TheCheckAlone(pair, range, record.the_ending_learned());
    }
    match partners_record {
        Some(check) if is_drawn(&pair.check) => {
            let ending = check.the_ending_learned();
            AsAPart::TheRangeWithItsCheck(pair, ending)
        }
        _ => AsAPart::NotAPart,
    }
}

impl AsAPart {
    /// **The row**: the check's line under the name, and *In Queue* or
    /// *Check only* where the ending goes, in grey — or, for a check that did
    /// not finish, its own ending as the row already reads it.
    fn is_drawn_on(&self, row: &mut AHistoryRowDrawn, record: &LocalJobRecord) {
        use crate::surface::the_queue::{APartOfThePair, CHECK_ONLY, IN_QUEUE};
        let the_line = |pair: &ThePair, ending: Option<Ending>| APartOfThePair {
            what: pair.the_checks_line(),
            state: APart::default(),
            dot: if ending == Some(Ending::Finished) { "done" } else { "" },
        };
        match self {
            AsAPart::NotAPart | AsAPart::TheHiddenCheck(_) => {}
            AsAPart::TheRangeWithItsCheck(pair, ending) => row.the_check = vec![the_line(pair, *ending)],
            AsAPart::TheCheckAlone(pair, range, ending) => {
                row.the_check = vec![the_line(pair, *ending)];
                match range {
                    TheRangeInHistory::InQueue => {
                        row.in_place_of_the_ending = Some(IN_QUEUE);
                        row.the_check_waits_on_its_range = true;
                    }
                    TheRangeInHistory::NeverRendered if record.the_ending_learned() == Some(Ending::Finished) => {
                        row.in_place_of_the_ending = Some(CHECK_ONLY);
                    }
                    _ => {}
                }
            }
        }
    }

    /// **The entry**: the pair's box at its top. A pair deletes together; a
    /// check whose full range is live is not deleted at all; and **a check
    /// offers no *Render it again*** — a one-frame copy of it is not what
    /// anyone submitted.
    fn is_drawn_on_the_entry(&self, connection: &rusqlite::Connection, entry: &mut TheHistoryEntry) {
        use crate::surface::the_queue::the_check_in_history;
        match self {
            AsAPart::NotAPart | AsAPart::TheHiddenCheck(_) => {}
            AsAPart::TheRangeWithItsCheck(pair, _) => {
                entry.entry.the_check = Some(the_check_in_history(connection, pair, TheRangeInHistory::Ended));
                entry.deleted_with_its_check = true;
            }
            AsAPart::TheCheckAlone(pair, range, _) => {
                entry.entry.the_check = Some(the_check_in_history(connection, pair, *range));
                entry.again = None;
                entry.the_check_waits_on_its_range = *range == TheRangeInHistory::InQueue;
            }
        }
    }
}

/// **What the local record keeps beside a job's own record**, read for the
/// one entry that is drawn.
///
/// - **The kept setup** — record version 25, the payload the job was submitted
///   with (`Queue & History` N10, `Rendering` A1).
/// - **The reason the machine recorded** — record version 27, `Rendering`
///   K14's reason for a failed ending, carried in by a connection and kept
///   whole. Step 10c's sixth pass, fault 3.
///
/// - **What each kept stream came to** — record version 34, `Queue & History`
///   E9's kept words, read without the words themselves: the bytes are read
///   only when *Blender output* is pressed. The Creator's notes on 0.1.8,
///   piece 6d.
///
/// - **The newest picture held** — record version 37, `Queue & History` P2
///   and P4's fetched derivative, its frame and P10a's name. The 38's piece 3.
///
/// **All are the record's own and none is read off a machine at the draw**
/// — *"history is Smoozy's own record and is there straight away."*
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BesideTheRecord {
    pub first_frame_purpose: Option<crate::record::job_records::FirstFramePurpose>,
    pub the_setup: Option<crate::record::job_records::TheKeptSetup>,
    /// **The frame text the user typed** — record version 51, `Queue &
    /// History` P5b — which *Frames* and the waiting first-frame question draw
    /// exactly as stored. `None` draws the numbers form. The eighteenth notes,
    /// piece 7.
    pub the_typed_frames: Option<String>,
    /// **Whether the job this one carries on from still has an entry** —
    /// `Queue & History` N20's draw-time lookup on the copied identity, so the
    /// mark offers its way only where there is somewhere to go. The 38's
    /// piece 2.
    pub the_earlier_entry_stands: bool,
    pub the_reason_the_machine_recorded: Option<String>,
    /// **The newest copy the record holds** (`Queue & History` P2), where one
    /// is held.
    pub the_picture: Option<crate::record::previews::AFrameOfTheSet>,
    /// **Every copy the record holds for the job**, frame by frame and in each
    /// frame's order (P4) — the frames and View Layers the preview hands the
    /// page. The Creator's seventh notes, note 4.
    pub the_copies_held: Vec<crate::record::previews::AFrameOfTheSet>,
    pub the_outcomes_kept: Vec<(
        u32,
        crate::presentation::the_machines_own_words::Stream,
        Option<smoozy_runtime::capture::CollectionOutcome>,
    )>,
}

impl BesideTheRecord {
    /// **Read from the record, and a read that fails reads as nothing kept.**
    pub fn read(connection: &rusqlite::Connection, job: &Identity) -> BesideTheRecord {
        let records = JobRecords::over(connection);
        let the_earlier_entry_stands = records
            .read(job)
            .ok()
            .flatten()
            .and_then(|record| record.carries_on_from)
            .is_some_and(|earlier| matches!(records.read(&earlier.earlier_job), Ok(Some(_))));
        BesideTheRecord {
            first_frame_purpose: records.first_frame_purpose(job).ok().flatten(),
            the_earlier_entry_stands,
            the_setup: records.the_kept_setup_of(job).ok().flatten(),
            the_typed_frames: records.the_typed_frames_of(job).ok().flatten(),
            the_reason_the_machine_recorded: records
                .the_reason_the_machine_recorded(job)
                .ok()
                .flatten(),
            the_outcomes_kept: crate::record::kept_words::the_outcomes_kept_for(connection, job)
                .unwrap_or_default(),
            the_picture: crate::record::previews::the_newest_held(connection, job)
                .ok()
                .flatten(),
            the_copies_held: crate::record::previews::every_copy_held_for(connection, job)
                .unwrap_or_default(),
        }
    }

    /// **Whether the record keeps any of the job's words** — and so whether
    /// History offers *Blender output* at all. **No words kept is no
    /// control**: the panel never shows an empty box (M10), and *not built* is
    /// never the answer.
    pub fn the_words_are_kept(&self) -> bool {
        !self.the_outcomes_kept.is_empty()
    }

    /// **Whether the ending attempt's complaints stream was recorded empty**
    /// — `Rented-server runtime` M13's *collected, and the stream was empty*,
    /// on the attempt the note points at (`Queue & History` N3). **The
    /// outcome is the record's and nothing reads the words for it** (M11).
    pub fn the_complaints_stream_was_empty_on(&self, attempt: Option<u32>) -> bool {
        let Some(attempt) = attempt else {
            return false;
        };
        self.the_outcomes_kept.iter().any(|(at, stream, outcome)| {
            *at == attempt
                && *stream == crate::presentation::the_machines_own_words::Stream::TheComplaints
                && *outcome
                    == Some(smoozy_runtime::capture::CollectionOutcome::CollectedAndTheStreamWasEmpty)
        })
    }
}

/// **The job's server badge, as the drawing's own class** — one of
/// [`crate::surface::the_rail::THE_EIGHT_BADGES`], `b1` to `b8`.
///
/// **The record keeps the badge's number** (`render_settings.rs`, *"the record
/// keeps its number"*), and until the Creator's notes on the test copy, piece 8
/// (note 11), this file handed the page the bare number. The page added `1` as
/// a class no style knows, so every queue and History entry drew its badge
/// empty and still gave it room, which pushed the line under the name in.
/// **A number that is not one of the eight draws no badge**, the rule the
/// rail keeps for its own rows.
pub(crate) fn the_badge_of(hue: Option<smoozy_queue_history::record::Hue>) -> Option<String> {
    let number = usize::from(hue?.0);
    crate::surface::the_rail::THE_EIGHT_BADGES
        .get(number.checked_sub(1)?)
        .map(|badge| badge.to_string())
}

/// **The history entry, composed from one `LocalJobRecord` and from nothing
/// else.**
///
/// **No connection is opened, no machine is read and no reading is taken** —
/// which is what Design means by *"history is Smoozy's own record and is there
/// straight away"*. The one lookup that is not the record's own is
/// `the_server_stands`, which is `Queue & History` N17's draw-time read of the
/// entry's copied server identifier against the server rows.
///
/// **Three things the drawing shows had no half on this side, and two still
/// draw an absence rather than a made-up value.**
///
/// - **The two ways to render it again, drawn since the 38's piece 2.** Both
///   name their own frame count — Design: *"a pair naming 240 frames and 181
///   frames"* — counted from the asked order the fill-in lands (`Queue &
///   History` N9, the 38's piece 10), and drawn only where the record kept the
///   setup a new job is made from (record version 25). *(Until then the group
///   was absent: record version 23 had no column for the setup, and a range
///   left as the file's own had no count on this record.)* An entry with no
///   asked order, or no kept setup, still draws neither.
/// - **The preview.** `Queue & History` P10 picks the picture on the machine
///   and nothing carries one to the desk, and `Desktop app and delivery` M10 is
///   flat that an empty panel is never what the user meets — so the column is
///   absent rather than a frame with no picture in it.
/// - **What deleting leaves** holds only what the record knows, which today is
///   whether the server is gone. Must-do 10's rule stands — a count that is
///   not known is never drawn as *0 frames* — and the upstream must-do of step 10b round 1 of
///   *jobs, history and downloads* has the box say what is untouched without a
///   number instead.
///
/// **`start_never_heard` is `Queue & History` C9's second way in** (item C1):
/// the entry reads [`AnEnding::StartNotKnown`] and says so in one line.
///
/// **`beside` is what the local record keeps beside the job's own record** —
/// the kept setup the *Submitted with* group draws the user's answers from,
/// and the reason the machine recorded, which the failure puts behind *What
/// the server said*. See [`BesideTheRecord`].
pub fn the_entry_for(
    record: &LocalJobRecord,
    beside: &BesideTheRecord,
    start_never_heard: bool,
    the_server_stands: bool,
) -> TheHistoryEntry {
    the_entry_beside_a_reading(record, beside, start_never_heard, the_server_stands, None)
}

/// [`the_entry_for`], **with the ending a reading carries while it is owed to
/// a full store** ([`what_is_kept_beside`]).
///
/// **Only the ending, and `Rented-server runtime` V12's reading behind it,
/// come from the reading.** Everything else is the record's, which has none of
/// the ending's facts yet: no figure, no times, no words. **And `Queue &
/// History` N6's figure is owed with the ending** (`Rented-server runtime`
/// S14a: the completeness statement rides the pending outcome), **so no claim
/// that reads the figure's absence as a fact is made** — not *no frame was
/// completed*, not *none finished*, not *nothing to start from*, and no empty
/// picture. Those are read as for an ending not yet learned: Smoozy does not
/// know how far the job got, which is true until the landing.
pub fn the_entry_beside_a_reading(
    record: &LocalJobRecord,
    beside: &BesideTheRecord,
    start_never_heard: bool,
    the_server_stands: bool,
    owed: Option<&smoozy_queue_history::acting::note::TheEndingOwed>,
) -> TheHistoryEntry {
    let (ending, owed) = the_ending_drawn(record, start_never_heard, the_server_stands, owed);
    // **What the figure's absence is read against**: the ending, or — while
    // the figure is owed with it — an ending not yet learned.
    let the_figure_reads_against = if owed.is_some() { AnEnding::InProgress } else { ending };
    let moments = record.the_four_moments();
    let rendered_for = moments.exactly_one_duration().map(for_how_long);
    // **`Queue & History` N6's figure, and `Queue & History` N1's *given up,
    // where it was* behind it** — the same figure by `Queue & History` N1's own words, kept as a
    // fallback for a note that carried the one and not the other. **Never a
    // figure reckoned by adding one to another** (must-do 2).
    let figure = record
        .what_the_note_carried
        .last_frame_figure
        .or(record.what_the_note_carried.given_up_where_it_was);
    let no_render_began = record
        .what_the_note_carried
        .ending
        .as_ref()
        .is_some_and(|ending| ending.no_render_began);
    TheHistoryEntry {
        entry: TheJobEntry {
            job: record.job.as_text(),
            // **Set by [`what_is_kept_beside`] where the entry is a first-frame
            // pair's**, which this record-only composition does not read.
            the_check: None,
            // **Called what the queue calls it**, a job sent with no name by
            // its file — step 10c's Vast pass, fault 3.
            name: crate::surface::the_queue::what_the_job_is_called(record),
            server: record.server.label.clone().unwrap_or_default(),
            // **The grey squircle where the server is no longer in Smoozy**
            // (Design, *History*), as on the row.
            badge: if the_server_stands {
                the_badge_of(record.server.hue)
            } else {
                Some(THE_REMOVED_BADGE.to_string())
            },
            blend: record.blend_path_on_the_machine.clone().unwrap_or_default(),
            scene: record.scene_name.clone(),
            reach_backs: the_server_stands,
            // **Set by [`what_is_kept_over`] from the connections this run
            // holds**, which this record-only composition does not read.
            the_server_is_not_connected: false,
            the_folder: the_folder_on(record),
            status: the_status_line(ending, figure, rendered_for.as_deref()),
            status_is: ending.class(),
            // **A history entry has no live half at all.** The two bars, the
            // machine's readings, the six statements and the two that hold the
            // job are the queue's, read on a connection; a recorded ending is
            // none of them. Design: *"A running entry never says which parts of
            // the server did the work"*, and the entry that does is this one's
            // *Rendered* group below.
            progress: None,
            readings: Vec::new(),
            marks: the_marks_on(record, beside.the_earlier_entry_stands),
            writes: Vec::new(),
            held: Vec::new(),
            facts: the_setup_on(record, beside.the_setup.as_ref(), beside.the_typed_frames.as_deref()),
            // **The shared entry's own control reads a running job's words**
            // (`Queue & History` E8), and a history entry has no running job.
            // History's own is [`TheHistoryEntry::blender_output`].
            blender_output: false,
            // **M13b stands unchanged on History**: no line of either stream
            // is lifted out of it (the Creator's notes on the test copy,
            // piece 8, narrowed it for a rendering queue entry alone).
            lines: Vec::new(),
            preview: the_preview_on(
                record,
                beside.the_picture.as_ref(),
                &beside.the_copies_held,
                the_figure_reads_against,
                figure,
            ),
            downloading: None,
            telegram: None,
            // **Design, *The entry's controls*: no job is ever both**, and a
            // history entry is neither — the job has ended.
            stop_or_cancel: None,
        },
        // **The standalone control, on the one ending that carries no failure
        // shape to hold it, and only where the words are kept** (`Queue &
        // History` E9). The three that carry one get it from
        // [`TheEndingsFailure::the_controls`], on the same condition, and a
        // second control beside that would be the same act twice.
        blender_output: ending == AnEnding::Finished && beside.the_words_are_kept(),
        ending: Some(ending),
        failure: the_failure_for(
            ending,
            // **`Queue & History` N6: an ending short of a clean finish with no
            // figure is a job that finished no frame** — and an ending owed
            // has its figure owed with it, which is not that.
            figure.is_none() && owed.is_none(),
            // **The full disk is read off the machine**: `Rented-server
            // runtime` V12's reading, carried with the ending onto this record
            // (`Desktop app and delivery` M15a), or on the reading beside an
            // ending owed (`Rented-server runtime` S14a) — and route (b) fires
            // only on a render that ended failing and a reading of nothing
            // available.
            the_disk_was_full_on(
                record,
                ending,
                match owed {
                    Some(owed) => owed.the_free_space_at_the_ending.as_ref().and_then(|free| free.the_reading()),
                    None => record.what_the_note_carried.the_free_space_at_the_ending.as_ref(),
                },
            ),
            // **Whether the complaints stream came back empty is the ending
            // attempt's own outcome**, kept with the words since record version
            // 34 (`Queue & History` E9) — must-do 9's no-errors sentence, which
            // had nothing to read until then.
            beside.the_complaints_stream_was_empty_on(
                record
                    .what_the_note_carried
                    .ending
                    .as_ref()
                    .map(|ending| ending.the_attempt_whose_capture_record_holds_the_words),
            ),
            // **Record version 60: the machine's own word that no render began**
            // (`Rendering` K14). Absent on an older record, which keeps its words.
            no_render_began,
        )
        .map(|failure| TheEndingsFailure {
            what_the_server_said: beside
                .the_reason_the_machine_recorded
                .clone()
                .filter(|_| !no_render_began),
            the_reason_beneath: beside
                .the_reason_the_machine_recorded
                .as_deref()
                .filter(|_| no_render_began)
                .map(the_reason_as_a_sentence),
            // **The raw text is offered where there is some** (`Design/Whole
            // app`, the failure shape) — never a control that opens nothing.
            blender_output: beside.the_words_are_kept(),
            ..failure
        }),
        // **Design, *The two ways to render it again*** — drawn where the record
        // kept the setup a new job is made from and the job's asked order is
        // known, which is what both controls name their frame counts from. The
        // figure the carry-on names is the entry's own until the press reads
        // the server (`Rendering` A4, `Rendering` A9). The 38's piece 2.
        again: match (&beside.the_setup, &record.the_frames.asked) {
            (Some(crate::record::job_records::TheKeptSetup::Kept(_)), Some(asked))
                if !record.the_frames.they_cannot_be_counted =>
            {
                let again = the_ways_to_render_it_again(
                    the_figure_reads_against,
                    &asked.the_order(),
                    figure,
                    WhereTheFigureCameFrom::RecordedWhenTheJobEnded,
                    the_server_stands,
                );
                // **`Queue & History` E3: *All frames rendered.* does not stand
                // over a count that came up short.** Read against a finish the
                // record learned, never one a reading says is owed.
                Some(if the_figure_reads_against == AnEnding::Finished {
                    again.over_a_count(record.what_the_note_carried.the_files_counted)
                } else {
                    again
                })
            }
            _ => None,
        },
        rendered: the_rendered_group(
            // **A clean finish confirmed every frame it asked for, so its last
            // frame is the asked order's last** (`Design/Queue & History`, the
            // frame count is the last frame Smoozy confirmed) — where the
            // order is on the record and can be counted. Only this line reads
            // it; the figure above, which the failure shape and the carry-on
            // read, is left as it is. The Creator's seventh notes, note 3.
            match ending {
                AnEnding::Finished => the_last_frame_asked(record),
                _ => figure,
            },
            // **The figure is the boundary frame — the one Smoozy cannot
            // confirm finished** (`Queue & History` N6, `Rendering` A4), and it
            // exists only for an ending short of a clean finish.
            ending != AnEnding::Finished,
            // **An absent figure means two different things and only one of
            // them is *none finished*.** `Queue & History` N6 computes the
            // figure for every ending short of a clean finish, so an ending
            // short of one with no figure is a job that finished no frame —
            // the drawing's own such entry. **A clean finish whose asked order
            // is not on the record, or cannot be counted, has no last frame to
            // name**, and **an ending Smoozy never learned has none either**;
            // the drawing words both *not known*. Saying *none finished* of a job that
            // rendered every frame is a plain falsehood, and it is what the
            // running application showed before this line existed.
            match the_figure_reads_against {
                AnEnding::Finished
                | AnEnding::NotKnown
                | AnEnding::StartNotKnown
                | AnEnding::InProgress => NOT_KNOWN,
                AnEnding::Failed | AnEnding::Stopped | AnEnding::GivenUp => NONE_FINISHED,
            },
            moments
                .the_submitted_moment()
                .and_then(in_seconds)
                .as_deref(),
            moments
                .started
                .as_ref()
                .map(smoozy_state::observed::Reading::taken_at)
                .and_then(in_seconds)
                .as_deref(),
            moments
                .ended
                .as_ref()
                .map(smoozy_state::observed::Reading::taken_at)
                .and_then(in_seconds)
                .as_deref(),
            rendered_for.as_deref(),
            // **Which parts of the server did the work is `Queue & History`
            // E12 and E13's device line**, observed in passing on the version's
            // own stream and carried in on the note's ending since step 10c's
            // fix piece 2 (the paragraph after `Queue & History` N3).
            // **Stated only on a finished ending**, by the backend word alone
            // where the line is a shape the research holds, and as printed
            // otherwise (the Creator's third notes, note 9); every other
            // ending draws *not known*. **A finished job that printed no line
            // draws no *Ran on* at all** (`Design/Queue & History`, 2026-11-03):
            // nothing was path traced, a compositor-only frame among them.
            the_devices_that_did_the_work(record).as_deref(),
            ending == AnEnding::Finished,
        ),
        // **Design, and the drawing's own line on the entry whose ending was
        // never learned.** Drawn only where both halves are true: no ending was
        // ever learned, and the server's row is gone.
        //
        // **Item C1's entry says its one line whether or not a row stands**:
        // its server is gone either way — removed, or found with nothing of
        // Smoozy's left on it — and the line is true of both.
        rendered_note: match ending {
            AnEnding::StartNotKnown => Some(SMOOZY_NEVER_HEARD_WHETHER_IT_STARTED),
            AnEnding::NotKnown if !the_server_stands => Some(THE_SERVER_WAS_DESTROYED),
            _ => None,
        },
        what_deleting_leaves: WhatDeletingLeaves {
            // **How many frames are on the server is a walk of the machine**,
            // and this record holds no count of one. Absent is never *0
            // frames*; the round's upstream must-do has the box say they are untouched without a number.
            frames_on_the_server: None,
            the_server_is_gone: !the_server_stands,
            // **How much of the job came home is `Download`'s own record**, and
            // `Queue & History` N18 reaches it through a set of landing places
            // rather than a count of frames. Absent draws the upstream must-do's sentence with
            // no number.
            downloaded: None,
        },
        the_ending_is_owed: owed.is_some(),
        // **Set by [`what_is_kept_beside`]**, as `the_check` is.
        the_check_waits_on_its_range: false,
        deleted_with_its_check: false,
    }
}

/// **`Queue & History` E12: what did the work, from the line the note's ending
/// carried, and only on a finished ending.**
///
/// **The rule is `Queue & History`'s own** —
/// `the_device_field_from_the_line_as_printed` — so this decides nothing: a
/// failed, stopped or given-up job that printed a line still says *not known*,
/// because a line from a run that did not finish is not evidence. **The ask is
/// not an argument** (`Queue & History` E12a): what the job was asked to
/// render on is the *Renders on* row, and it never fills this one.
fn the_devices_that_did_the_work(record: &LocalJobRecord) -> Option<String> {
    use smoozy_queue_history::ending::{
        the_device_field_from_the_line_as_printed, DevicesThatDidTheWork,
    };
    let ending = record.what_the_note_carried.ending.as_ref()?;
    match the_device_field_from_the_line_as_printed(
        ending.outcome.as_a_recorded_outcome(),
        ending.the_device_line_as_observed.as_deref(),
    ) {
        DevicesThatDidTheWork::Known { the_line_as_printed } => {
            Some(how_blender_reached_the_work(&the_line_as_printed))
        }
        DevicesThatDidTheWork::NotKnown => None,
    }
}

/// **`Design/Queue & History`, *Ran on* (2026-10-06, the Creator's third
/// notes, note 9): only the word Blender prints in brackets after the device —
/// *OptiX*, *CUDA*, *CPU* or *Metal*.** Read only in the four shapes the
/// research holds (`Blender Research/Findings/5.1/The machine.md` and *What
/// actually ran — second pass*), each `Path tracing on: <device> <tail>
/// [<identifier>]`:
///
/// - `… RTX 4090 (OptiX) [CUDA_…_OptiX]`
/// - `… RTX 4090 (CUDA) [CUDA_…]`
/// - `… 64-Core Processor (CPU) (61 threads) [CPU]`
/// - `… Apple M1 Pro (GPU - 14 cores) (Metal) (display) [METAL_…]`
///
/// **Any other shape is drawn whole, as printed** — a word guessed from a
/// line nobody has read would be the entry saying more than it knows.
fn how_blender_reached_the_work(the_line_as_printed: &str) -> String {
    match the_device_and_its_route(the_line_as_printed) {
        Some((_, the_route)) => the_route.to_string(),
        None => the_line_as_printed.to_string(),
    }
}

/// **The one reading of Cycles' device line: the device as Blender names it,
/// and the word in brackets after it** — *NVIDIA GeForce RTX 4090* and
/// *OptiX* — in the four shapes [`how_blender_reached_the_work`] names, and
/// nothing in any other shape.
///
/// **Two places read the line and both read it here**, so they cannot read
/// one line two ways: History's *Ran on*, which takes the word, and Server
/// Setup's *Last job rendered on* (`crate::surface::set_up`), which takes
/// both — `Design/Server Setup` line 56, changed 2026-10-17 (the Creator's
/// tenth notes, note 3). It came out of [`how_blender_reached_the_work`] that
/// day, unchanged, so the second reader would not be a second copy.
pub(crate) fn the_device_and_its_route(the_line_as_printed: &str) -> Option<(&str, &'static str)> {
    const THE_HEAD: &str = "Path tracing on: ";
    let rest = the_line_as_printed.trim().strip_prefix(THE_HEAD)?;
    let (before_the_identifier, identifier) = rest.rsplit_once(" [")?;
    if !identifier.ends_with(']') || identifier.len() < 2 {
        return None;
    }
    // The CPU's tail carries its thread count: `(CPU) (61 threads)`.
    let the_cpus_tail = |text: &str| -> Option<usize> {
        let (before, threads) = text.rsplit_once(" (")?;
        let count = threads.strip_suffix(" threads)")?;
        if count.is_empty() || !count.bytes().all(|b| b.is_ascii_digit()) {
            return None;
        }
        before.strip_suffix(" (CPU)").map(str::len)
    };
    let (the_word, the_device_ends_at) = if let Some(device) =
        before_the_identifier.strip_suffix(" (OptiX)")
    {
        ("OptiX", device.len())
    } else if let Some(device) = before_the_identifier.strip_suffix(" (CUDA)") {
        ("CUDA", device.len())
    } else if let Some(device) = before_the_identifier.strip_suffix(" (Metal) (display)") {
        ("Metal", device.len())
    } else if let Some(device) = the_cpus_tail(before_the_identifier) {
        ("CPU", device)
    } else {
        return None;
    };
    let the_device = before_the_identifier[..the_device_ends_at].trim();
    if the_device.is_empty() {
        return None;
    }
    Some((the_device, the_word))
}

/// **The ending this record carries, and the fifth is the absence of one** —
/// C11, read through `LocalJobRecord::the_ending_learned` so E2's mapping has
/// one home. **Item C1's entry is one step further back**: no start was ever
/// learned, so there is no ending to read.
///
/// **No ending learned is two things, and the server row says which** —
/// `Design/Queue & History § History`, E1-1, over `Queue & History` C9 and
/// `Queue & History` C11. **While the entry's server row stands the job is *In
/// progress***: `Queue & History` C9 put it here at its render start and its
/// ending has not come in yet. **Once the row is gone it is *Ending not
/// known***, which `Queue & History` C11 keeps for good. Until
/// step 10c's twelfth-pass fix piece 5 every job still rendering read *Ending
/// not known* here, with *Delete* — the twelfth pass on Hyperstack, fault 6.
fn the_ending_on(
    record: &LocalJobRecord,
    start_never_heard: bool,
    the_server_stands: bool,
) -> AnEnding {
    if start_never_heard && !record.the_render_start_has_been_learned {
        return AnEnding::StartNotKnown;
    }
    match record.the_ending_learned() {
        Some(Ending::Finished) => AnEnding::Finished,
        Some(Ending::Failed) => AnEnding::Failed,
        Some(Ending::Stopped) => AnEnding::Stopped,
        Some(Ending::GivenUp) => AnEnding::GivenUp,
        None if the_server_stands => AnEnding::InProgress,
        None => AnEnding::NotKnown,
    }
}

/// **The ending drawn, and the reading's ending owed where it is the one
/// drawn** — [`the_ending_on`], except that a job the record has as still
/// running, on a standing server, whose reading carries its ending owed to a
/// full store, is drawn with that ending (`Queue & History` C11, *from
/// watching*; `Rented-server runtime` S14a). **The record's ending always
/// wins**: once the landing is learned the reading is not read.
fn the_ending_drawn<'o>(
    record: &LocalJobRecord,
    start_never_heard: bool,
    the_server_stands: bool,
    owed: Option<&'o smoozy_queue_history::acting::note::TheEndingOwed>,
) -> (AnEnding, Option<&'o smoozy_queue_history::acting::note::TheEndingOwed>) {
    match (the_ending_on(record, start_never_heard, the_server_stands), owed) {
        (AnEnding::InProgress, Some(owed)) => {
            let ending = match smoozy_queue_history::ending::the_ending(owed.outcome.as_a_recorded_outcome()) {
                Ending::Finished => AnEnding::Finished,
                Ending::Failed => AnEnding::Failed,
                Ending::Stopped => AnEnding::Stopped,
                Ending::GivenUp => AnEnding::GivenUp,
            };
            (ending, Some(owed))
        }
        (ending, _) => (ending, None),
    }
}

/// **A moment as seconds since the epoch, for the page to write as a date.**
///
/// **Nothing in this application writes a date and nothing can**: choosing
/// between *today 04:12*, *8 Aug 23:14* and a year is a comparison against now
/// on the reader's own computer, and the words are that computer's locale data.
/// The local time zone is not in the standard library and the manifest is ruled
/// tight, so no date library may be added. **`theMomentReads` is
/// `web/servers.js`'s and there is one of it.**
fn in_seconds(at: std::time::SystemTime) -> Option<String> {
    at.duration_since(std::time::UNIX_EPOCH)
        .ok()
        .map(|since| since.as_secs().to_string())
}

/// **The status line, in the drawing's own shapes, with each clause dropped
/// where its figure is absent** — Design, *A rendering entry*: *"It opens with
/// a plain status line — what it is doing, and how long it has been doing
/// it."*
///
/// **The frame is `Queue & History` N6's figure exactly as recorded.** The
/// drawing reads *Failed at frame 61* against a recorded figure of 60, which is
/// the drawing predating `Rendering` A4's correction of 2026-09-02: the figure
/// **is** the frame Smoozy cannot confirm finished, so it is the frame the job
/// was on, and this entry never reckons one from the other by adding one
/// (must-do 2).
///
/// **The moment on the *ending not known* line is not drawn.** The drawing has
/// *— submitted 4 Aug 21:40*, and a moment on this line would have to be
/// written as a date by `web/the-queue.js`, which composes the status line for
/// both screens and is not this piece's file. The submitted moment is on the
/// *Rendered* group below, where this screen's own page writes it.
fn the_status_line(ending: AnEnding, at_frame: Option<i64>, rendered_for: Option<&str>) -> Vec<APart> {
    let mut said = vec![APart::value(ending.said())];
    match ending {
        // *Finished — 240 frames in 5 hours 8 minutes*, less the count, which
        // is the frames the job asked for and is the kept setup's.
        AnEnding::Finished => {
            if let Some(how_long) = rendered_for {
                said.push(APart::said(FINISHED_LEAD, how_long.to_string(), ""));
            }
        }
        // **Not an ending at all**, and nothing follows it that this file can
        // compose.
        AnEnding::NotKnown | AnEnding::StartNotKnown | AnEnding::InProgress => {}
        AnEnding::Failed | AnEnding::Stopped | AnEnding::GivenUp => {
            if let Some(frame) = at_frame {
                said.push(APart::said(FAILED_AT_FRAME, frame.to_string(), ""));
            }
            if let Some(how_long) = rendered_for {
                said.push(APart::said(AFTER_WORD, how_long.to_string(), ""));
            }
        }
    }
    said
}

/// **The marks — Design, *The marks*: each shown only when it is true, and
/// every one of them stays on the entry into history.**
///
/// **Six of Design's seven are here and one is absent for a stated reason.**
/// *Simplify was on* is an answer of the kept setup, which record version 23
/// has no column for; drawing it from nothing would say Simplify was on for
/// every job or for none. *(The seventh, `Queue & History` N23's driver mark,
/// joined at held-job piece B, 2026-10-12.)*
///
/// **The way to the earlier entry is offered where that entry still stands**
/// — `Queue & History` N20, since the 38's piece 2. The press hands back this
/// entry's own job and the earlier one is read from its mark, so
/// [`AMarkOnTheEntry`] carries the words alone. Where the earlier entry is
/// gone the mark reads whole without a way, which is what the copied label is
/// for.
fn the_marks_on(record: &LocalJobRecord, the_earlier_entry_stands: bool) -> Vec<AMarkOnTheEntry> {
    use smoozy_queue_history::record::{
        AMark, DependencyCheckAtSubmission as Checked, DisturbanceReading, NoSceneMark,
        WhatBecameOfTheFramesAlreadyFinished as Became,
    };
    let a_mark = |said: &'static str, parts: Vec<APart>| AMarkOnTheEntry {
        said,
        parts,
        way: None,
        bold: 0,
    };
    let mut marks: Vec<AMarkOnTheEntry> = record
        .the_marks()
        .into_iter()
        .map(|mark| match mark {
            // **One delete, more than one file, one mark**: the count and the
            // folder they went from (ruling (d) of 2026-10-03).
            AMark::AFileTheJobNeedsWasDisturbed(disturbance) if disturbance.how_many.is_some() => {
                let how_many = disturbance.how_many.unwrap_or_default();
                let (said, from) = match disturbance.reading {
                    DisturbanceReading::Disturbed => (FILES_THIS_JOB_NEEDS_WERE_DELETED, FILES_WERE_DELETED_FROM),
                    DisturbanceReading::MayHaveBeenDisturbed => {
                        (FILES_THIS_JOB_NEEDS_MAY_HAVE_BEEN_DELETED, FILES_MAY_HAVE_BEEN_DELETED_FROM)
                    }
                };
                a_mark(
                    said,
                    vec![
                        APart::said("", how_many.to_string(), from),
                        APart::said("", disturbance.the_member.clone(), AFTER_THIS_JOB_WAS_SUBMITTED),
                    ],
                )
            }
            AMark::AFileTheJobNeedsWasDisturbed(disturbance) => {
                let (was, may_have_been, after, may_have_been_after) =
                    the_marks_words(&disturbance.the_act_intended);
                // **`File Browser` W9: the act's outcome was never learned**,
                // and both lines say so rather than claiming it.
                let (said, after) = match disturbance.reading {
                    DisturbanceReading::Disturbed => (was, after),
                    DisturbanceReading::MayHaveBeenDisturbed => (may_have_been, may_have_been_after),
                };
                a_mark(said, vec![APart::said("", disturbance.the_member.clone(), after)])
            }
            // **`Rendering` W2's three states, and never a single bit** — the
            // record carries the state and each state has its own sentence.
            AMark::TheDependencyCheckAtSubmission(state) => a_mark(
                match state {
                    Checked::NoStandingAtAll => NOTHING_CHECKED_THIS_FILES_DEPENDENCIES,
                    Checked::AStandingCarryingTheReplacedSinceCheckOrDisturbedMark => {
                        THE_CHECK_CARRIED_A_REPLACED_MARK
                    }
                    Checked::AStandingKeyedToAnotherBlenderVersion => {
                        THE_CHECK_WAS_MADE_WITH_ANOTHER_BLENDER
                    }
                    // A complete standing draws no mark, and `the_marks` has
                    // already dropped it.
                    Checked::ACompleteStandingStood => NOTHING_CHECKED_THIS_FILES_DEPENDENCIES,
                },
                Vec::new(),
            ),
            AMark::ItCarriesOnFromAnEarlierJob(carries_on) => AMarkOnTheEntry {
                way: the_earlier_entry_stands.then_some(crate::surface::the_queue::THAT_JOB),
                ..a_mark(
                    CARRIES_ON_FROM,
                    vec![APart::value(carries_on.the_label_as_it_stood.clone())],
                )
            },
            AMark::ItWasSubmittedWithNoSceneNamed => a_mark(
                SUBMITTED_WITH_NO_SCENE_NAMED,
                // **`Queue & History` N22's two draws: the mark does not go once the scene is
                // known, and it says which scene it turned out to be.**
                match record.the_no_scene_mark() {
                    NoSceneMark::SubmittedWithNoSceneNamed {
                        the_scene_it_turned_out_to_be: Some(scene),
                    } => vec![APart::said(
                        BLENDER_RENDERED_LEAD,
                        scene.to_string(),
                        THE_FILES_ACTIVE_SCENE,
                    )],
                    _ => vec![APart::words(IT_RENDERS_WHICHEVER_SCENE)],
                },
            ),
            AMark::TheFileChangedBeforeAttempt(attempt) => a_mark(
                THE_FILE_CHANGED_BEFORE_ATTEMPT,
                vec![
                    APart::value(attempt.attempt.to_string()),
                    APart::words(
                        match attempt.what_became_of_the_frames_already_finished {
                            Became::TheyWereReused => FRAMES_ALREADY_FINISHED_WERE_REUSED,
                            Became::TheyWereRenderedAgain => {
                                FRAMES_ALREADY_FINISHED_WERE_RENDERED_AGAIN
                            }
                        },
                    ),
                ],
            ),
            // **`Queue & History` N23's mark, the seventh**: the driver's
            // version and the job's Blender, named by its line, inside the
            // bold; the clause after it outside. Held-job piece B.
            AMark::TheDriverWasBelowTheBar(below) => {
                let blender = crate::surface::the_queue::the_blenders_line(&below.the_jobs_blender);
                let the_rest = APart::words(AND_SMOOZY_COULDNT_UPDATE_IT);
                let (after_the_version, with_no_version) =
                    if crate::surface::the_queue::the_blender_cannot_use_the_card_below_its_bar(&below.the_jobs_blender) {
                        (WAS_TOO_OLD_FOR_CYCLES_GPU_IN_BLENDER, THE_NVIDIA_DRIVER_WAS_TOO_OLD_FOR_CYCLES_GPU_IN_BLENDER)
                    } else {
                        (WAS_TOO_OLD_FOR_OPTIX_IN_BLENDER, THE_NVIDIA_DRIVER_WAS_TOO_OLD_FOR_OPTIX_IN_BLENDER)
                    };
                match &below.the_drivers_version {
                    Some(version) => AMarkOnTheEntry {
                        bold: 2,
                        ..a_mark(
                            THE_NVIDIA_DRIVER,
                            vec![
                                APart::said("", version.clone(), after_the_version),
                                APart::said("", blender, THE_BOLD_ENDS),
                                the_rest,
                            ],
                        )
                    },
                    None => AMarkOnTheEntry {
                        bold: 1,
                        ..a_mark(
                            with_no_version,
                            vec![APart::said("", blender, THE_BOLD_ENDS), the_rest],
                        )
                    },
                }
            }
        })
        .collect();
    // **`Queue & History` N1's re-run marks, which are not among
    // `the_marks`'s** — they ride the note's carried half rather than the
    // submission's own fields, and Design makes them marks like the others.
    // **A mark whose attempt list was empty carries no frame**, which is not a
    // zero, so it says only that the job was interrupted and re-run.
    for mark in &record.what_the_note_carried.re_run_marks {
        marks.push(a_mark(
            INTERRUPTED_AND_RE_RUN,
            mark.the_re_run_proceeded_from
                .map(|frame| vec![APart::said(FROM_FRAME, frame.to_string(), "")])
                .unwrap_or_default(),
        ));
    }
    marks
}

/// **The setup it was submitted with — Design, *The rest of the entry***, in
/// the drawing's own rows and its italic-and-bold convention: the file's own
/// values in italic, anything the user changed upright and bold with the
/// file's value named beside it.
///
/// **The frame range, the sample count and the engine are answers of the kept
/// setup** — the payload `Rendering` A1 opens the settings page from, which
/// `Queue & History` N10 puts on this record and record version 25 keeps
/// beside it (`src/record/job_records.rs`). **Until step 10c's sixth pass this
/// read none of it**: the column had been written at every submission since
/// version 25 and nothing drew from it, so a job submitted with 1,024 samples
/// typed read *Samples — not known*, in bold, under *bold = yours*.
///
/// **Three faces, and each row takes the one that is true of it.**
/// - **The user's**, upright and bold, where the kept setup carries an entry —
///   with `Queue & History` N14's companion beside it where a reading stood at
///   submission. **The companion is never drawn as the value in force**: it
///   holds the *file's* value for a setting the user overrode.
/// - **The file's**, in italic, where the kept setup stands and carries no
///   entry for the setting — the job ran on the file's own value, which the
///   companion holds where a reading stood at submission (since 2026-10-07,
///   the Creator's second notes, piece 16), and with no reading on the record
///   that value is *not known*.
/// - **Neither**, where no kept setup reads back — a record from before version
///   25, or a column that will not read — because then nothing says whether the
///   user answered, and *not known* in either face would claim one.
///
/// **`Queue & History` E12a: what the job was asked to render on is stated
/// whatever the ending**, from the record's own ask, which is written at every
/// submission and needs no kept setup. **Design words it nowhere but the
/// drawing's *Cycles on the GPU***, which is one value in one face; the engine
/// and the device are two answers that can each be the user's or the file's,
/// so they are two rows, and the second takes the settings page's own label.
/// **It is the ask and never what did the work** — that is the *Rendered*
/// group's *Ran on*, which `Queue & History` E12 keeps to a finished job.
///
/// **`Queue & History` P5b: *Frames* reads exactly as the user typed it**,
/// where the record keeps that text (record version 51) and the kept setup
/// holds the range it was accepted with — *1,2-4* stays *1,2-4*, never *1, 2,
/// 3, 4* (the eighteenth notes, note 8). **The text is drawn and never read**:
/// the count beside it and every other number are the kept setup's. A record
/// with no text draws the numbers form, as before.
fn the_setup_on(
    record: &LocalJobRecord,
    kept: Option<&crate::record::job_records::TheKeptSetup>,
    typed_frames: Option<&str>,
) -> Vec<AFactOnTheEntry> {
    use crate::record::job_records::TheKeptSetup;
    use crate::surface::render_settings::words as the_page;
    use smoozy_queue_history::record::{AnswerOrigin, TheDeviceAsk};
    use smoozy_render_settings::device::DeviceChoice;
    use smoozy_render_settings::engine::OfferedEngine;
    use smoozy_render_settings::reading::DeviceSetting;

    let payload = match kept {
        Some(TheKeptSetup::Kept(payload)) => Some(payload),
        _ => None,
    };
    let companion = record.the_files_values_beside_the_overrides.as_ref();
    // **The row for a setting the user did not answer**, in whichever face is
    // true of it — see above — **with the file's own value where the
    // companion holds it** (`Render Settings` Y1a as corrected 2026-10-07,
    // the Creator's second notes, piece 16). Only where a kept setup reads
    // back: without one, nothing says the companion's value is not the one
    // beside an override, and it is never drawn as the value in force.
    let untouched = |name: &'static str, the_files_own: Option<String>| AFactOnTheEntry {
        name,
        value: the_files_own
            .filter(|_| payload.is_some())
            .unwrap_or_else(|| NOT_KNOWN.to_string()),
        the_files: payload.is_some(),
        neither: payload.is_none(),
        ..AFactOnTheEntry::default()
    };
    let the_users = |name: &'static str, value: String, was: Option<String>| AFactOnTheEntry {
        name,
        value,
        the_files_own: was,
        ..AFactOnTheEntry::default()
    };
    let an_engine = |engine: OfferedEngine| match engine {
        OfferedEngine::Cycles => the_page::CYCLES,
        OfferedEngine::Eevee => the_page::EEVEE,
    }
    .to_string();
    let a_device = |choice: DeviceChoice| match choice {
        DeviceChoice::TheCards => the_page::GPU,
        DeviceChoice::TheProcessor => the_page::CPU,
    }
    .to_string();

    let mut facts = vec![
        // **`Queue & History` N19: a pair, not a version**, and on an entry
        // whose machine is gone the path reads as nothing — so the row shows
        // the answered version, which is what the drawing draws. **`Queue &
        // History` N10a says which face it takes**: the user's own override is
        // theirs, and a match Smoozy resolved for them is not.
        AFactOnTheEntry {
            name: BLENDER_FACT,
            value: record.blender.pick.version.clone(),
            the_files: record.blender.origin == AnswerOrigin::ResolvedForThem,
            ..AFactOnTheEntry::default()
        },
        match payload.and_then(|payload| payload.frame_range.as_ref()) {
            Some(form) => the_users(
                FRAMES_FACT,
                typed_frames
                    .map(str::to_string)
                    .unwrap_or_else(|| crate::surface::render_settings::written_out(form)),
                companion.and_then(|companion| companion.frame_range).map(|range| {
                    crate::surface::render_settings::a_range_as_the_field_writes_it(
                        range.start,
                        range.end,
                        range.step,
                    )
                }),
            ),
            None => untouched(
                FRAMES_FACT,
                companion.and_then(|companion| companion.frame_range).map(|range| {
                    crate::surface::render_settings::a_range_as_the_field_writes_it(
                        range.start,
                        range.end,
                        range.step,
                    )
                }),
            ),
        },
        // **Design: the picture size is the percentage and the pixels
        // together**, and where the file was never read there is no full size
        // to take a percentage of — so the entry carries the percentage alone
        // where the user typed one and says the size is not known.
        match record.resolution_percentage_typed {
            Some(typed) => AFactOnTheEntry {
                beside: record
                    .picture_size_from_the_read
                    .map(|(across, down)| format!("{across} {BY_SIGN} {down}")),
                ..the_users(
                    RESOLUTION_FACT,
                    format!("{typed}{A_PER_CENT}"),
                    companion
                        .and_then(|companion| companion.resolution_percentage)
                        .map(|was| format!("{was}{A_PER_CENT}")),
                )
            },
            None => AFactOnTheEntry {
                beside: record
                    .picture_size_from_the_read
                    .map(|(across, down)| format!("{across} {BY_SIGN} {down}")),
                // **The percentage is the record's own field, written at every
                // submission**, so an untyped one is the file's whatever the
                // kept setup says — **and so is the companion's percentage
                // here**, which before 2026-10-07 stood only beside a typed one.
                value: companion
                    .and_then(|companion| companion.resolution_percentage)
                    .map(|was| format!("{was}{A_PER_CENT}"))
                    .unwrap_or_else(|| NOT_KNOWN.to_string()),
                the_files: true,
                neither: false,
                ..untouched(RESOLUTION_FACT, None)
            },
        },
        match payload.and_then(|payload| payload.sample_count) {
            Some(count) => the_users(
                SAMPLES_FACT,
                count.to_string(),
                companion
                    .and_then(|companion| companion.sample_count)
                    .map(|was| was.to_string()),
            ),
            None => untouched(
                SAMPLES_FACT,
                companion
                    .and_then(|companion| companion.sample_count)
                    .map(|was| was.to_string()),
            ),
        },
        match payload.and_then(|payload| payload.engine.as_ref()) {
            Some(engine) => the_users(
                ENGINE_FACT,
                an_engine(engine.pick),
                companion
                    .and_then(|companion| companion.engine.as_ref())
                    .map(|was| the_files_engine_by_name(was.as_text())),
            ),
            None => untouched(
                ENGINE_FACT,
                companion
                    .and_then(|companion| companion.engine.as_ref())
                    .map(|was| the_files_engine_by_name(was.as_text())),
            ),
        },
        match record.device_asked_for {
            TheDeviceAsk::TheUsersOwn(choice) => the_users(
                RENDERS_ON_FACT,
                a_device(choice),
                companion
                    .and_then(|companion| companion.device)
                    .map(|was| match was {
                        DeviceSetting::TheCards => the_page::GPU.to_string(),
                        DeviceSetting::TheProcessor => the_page::CPU.to_string(),
                    }),
            ),
            TheDeviceAsk::TheFilesOwnAsTheReadingInForceGaveIt(choice) => AFactOnTheEntry {
                name: RENDERS_ON_FACT,
                value: a_device(choice),
                the_files: true,
                ..AFactOnTheEntry::default()
            },
            // **Submitted unread and unanswered: the file's own, whatever the
            // file says when the job starts** — the file's face, not known.
            TheDeviceAsk::TheFilesOwnUnread => AFactOnTheEntry {
                name: RENDERS_ON_FACT,
                value: NOT_KNOWN.to_string(),
                the_files: true,
                ..AFactOnTheEntry::default()
            },
        },
        // **Design, *The job's name, and its folder*: the entry shows the
        // recorded path verbatim rather than re-composing it from the label.**
        // `Queue & History` N10 lands it by the read's own route, so an entry
        // no read has reached yet says it is not known rather than composing a
        // folder name out of the identity and the label.
        AFactOnTheEntry {
            name: WRITING_TO_FACT,
            value: record
                .main_output_from_the_read
                .clone()
                .unwrap_or_else(|| NOT_KNOWN.to_string()),
            // **Neither face**: it is Smoozy's own composition and not a
            // setting either side answered — the drawing's own comment on this
            // row, and until the sixth pass it was drawn bold.
            neither: true,
            under: why_the_folder_fell_back(record),
            ..AFactOnTheEntry::default()
        },
    ];
    // **`Queue & History` E12a's bullet of 2026-11-02: only a Cycles job states
    // the ask.** The file's saved Cycles device decides nothing under EEVEE, and
    // `Rendering` L21 names no technology for it. Not known: as it was.
    let the_engine_is_not_cycles = the_engine_in_force_is_cycles(
        payload.and_then(|payload| payload.engine.as_ref()).map(|engine| engine.pick),
        companion.and_then(|companion| companion.engine.as_ref()).map(|was| was.as_text()),
    ) == Some(false);
    if the_engine_is_not_cycles {
        facts.retain(|fact| fact.name != RENDERS_ON_FACT);
    }
    // **A job whose folder is chosen at render start and not learned yet has
    // no folder, so nothing is *written to***: the drawing's `q-wait` draws no
    // *Writing to* row. The Creator's second notes, piece 4's queue half.
    if record.the_job_output_folder == TheJobOutputFolderTheRecordHolds::ChosenAtRenderStart
        && record.main_output_from_the_read.is_none()
    {
        facts.retain(|fact| fact.name != WRITING_TO_FACT);
    }
    facts
}

/// **The file's engine, in the word the settings page gives it** — *Cycles*
/// or *EEVEE* (Rule 9) — **and by its own name where it is neither**, as the
/// page names a third engine. The identifiers are `Render Settings` Q2 item
/// 4's: `CYCLES` in every version, the built-in real-time engine
/// `BLENDER_EEVEE_NEXT` in 4.5 and `BLENDER_EEVEE` in 5.2. **Until 2026-10-07
/// the entry drew the identifier itself** beside a picked engine.
/// **`Queue & History` E12a's bullet of 2026-11-02: whether the engine in force
/// is Cycles**, the user's pick, else the file's own as the entry holds it.
/// **Nothing where neither stands.** Beside [`the_files_engine_by_name`], whose
/// spelling of Cycles this is.
fn the_engine_in_force_is_cycles(
    pick: Option<smoozy_render_settings::engine::OfferedEngine>,
    the_files_own: Option<&str>,
) -> Option<bool> {
    match (pick, the_files_own) {
        (Some(pick), _) => Some(pick == smoozy_render_settings::engine::OfferedEngine::Cycles),
        (None, Some(identifier)) => Some(identifier == "CYCLES"),
        (None, None) => None,
    }
}

fn the_files_engine_by_name(identifier: &str) -> String {
    use crate::surface::render_settings::words as the_page;
    match identifier {
        "CYCLES" => the_page::CYCLES.to_string(),
        "BLENDER_EEVEE" | "BLENDER_EEVEE_NEXT" => the_page::EEVEE.to_string(),
        other => other.to_string(),
    }
}

/// **Whether the entry's job has a folder yet, read off the record and never
/// composed** (`Job files and outputs` P21, `Queue & History` Y8). A job whose
/// folder is chosen at render start has none until `Queue & History` N9 brings
/// the choice; **while its render has not started the entry says so**, and
/// once it has, the entry draws nothing in its place. The Creator's second
/// notes, piece 4's queue half.
/// **The last frame of the job's asked order**, where the order is on the
/// record and can be counted — read from its last run, never by unfolding the
/// order. The Creator's seventh notes, note 3.
fn the_last_frame_asked(record: &LocalJobRecord) -> Option<i64> {
    if record.the_frames.they_cannot_be_counted {
        return None;
    }
    let run = record.the_frames.asked.as_ref()?.runs.last().filter(|run| run.how_many > 0)?;
    let steps = i64::try_from(run.how_many - 1).ok()?;
    run.step.checked_mul(steps)?.checked_add(run.first)
}

fn the_folder_on(record: &LocalJobRecord) -> crate::surface::the_queue::TheFolderOnTheEntry {
    use crate::surface::the_queue::TheFolderOnTheEntry;
    match record.the_job_output_folder {
        TheJobOutputFolderTheRecordHolds::ChosenAtRenderStart if !record.the_render_start_has_been_learned => {
            TheFolderOnTheEntry::ItIsMadeWhenTheRenderStarts
        }
        TheJobOutputFolderTheRecordHolds::ChosenAtRenderStart => TheFolderOnTheEntry::ItHasNotArrived,
        _ => TheFolderOnTheEntry::ItHasOne,
    }
}

/// **The one or two lines under *Writing to*'s path** — `Design/Queue &
/// History`, *The job's name, and its folder*: where the folder fell back,
/// `Job files and outputs` P19's case says why, and where the server's chosen
/// fallback folder was passed over (`Job files and outputs` P23) a second line says so. **Both stay
/// into History**, which draws this same fact. A fallback with no case — a job
/// with no render destination, `Job files and outputs` P18's last sentence — says nothing of a case.
/// Nothing on a folder where the file points, or one named before 2026-10-04.
fn why_the_folder_fell_back(record: &LocalJobRecord) -> Vec<&'static str> {
    use crate::surface::the_queue::{
        SMOOZY_CANT_USE_THE_FILES_FOLDER, THE_FILES_FOLDER_IS_ON_ANOTHER_COMPUTER,
        THE_SERVERS_FALLBACK_FOLDER_WAS_PASSED_OVER,
    };
    use smoozy_job_files::where_the_folder_sits::{WhereItIs, WhyTheFilesOwnPlaceWasNotUsed};
    let TheJobOutputFolderTheRecordHolds::It(sits) = &record.the_job_output_folder else {
        return Vec::new();
    };
    let WhereItIs::TheFallback { why, the_servers_fallback_folder_was_passed_over } = &sits.where_it_is else {
        return Vec::new();
    };
    let mut lines = Vec::new();
    match why {
        Some(WhyTheFilesOwnPlaceWasNotUsed::ItIsAnotherComputers) => lines.push(THE_FILES_FOLDER_IS_ON_ANOTHER_COMPUTER),
        Some(WhyTheFilesOwnPlaceWasNotUsed::ItIsAFolderSmoozyCannotUse) => lines.push(SMOOZY_CANT_USE_THE_FILES_FOLDER),
        None => {}
    }
    if *the_servers_fallback_folder_was_passed_over {
        lines.push(THE_SERVERS_FALLBACK_FOLDER_WAS_PASSED_OVER);
    }
    lines
}

/// **One row, composed from the local job record alone.**
///
/// **Three of the drawing's figures are absent and each is absent for its own
/// stated reason rather than because nothing filled it in.**
///
/// - **The frame pair is the drawing's *60/240*** — the frames the server held
///   when the job ended, `Queue & History` N6's figure and every frame before
///   it in the job's asked order, over every frame asked: the count Telegram's
///   ending message says. `LocalJobRecord::how_far_it_got` composes it from the
///   note's figure and the asked order `Queue & History` N9's fill-in landed
///   (record version 35, the 38's piece 10, item 24; up to and including the
///   figure since the live pass on Vast, 2026-10-19, fault 5). **The carry-on
///   does not read this pair** — it opens at the figure itself and renders it
///   again (`Rendering` A4). **Absent where either half is**
///   — no ending learned, no asked order read, or a scene whose frames cannot
///   be counted (`Job files and outputs` I9a) — and absent draws the
///   drawing's own dash. *(Until that piece the record held neither half and
///   every row drew the dash.)*
/// - **The scene name** is filled by `Queue & History` N9's recorded reading,
///   which arrives by the same route.
/// - **A moment is absent only where the record has none at all**, which is a
///   record written before `Queue & History` N16's rule and never learned to
///   have ended.
fn a_history_row(
    record: &LocalJobRecord,
    start_never_heard: bool,
    the_server_stands: bool,
) -> AHistoryRowDrawn {
    let moments = record.the_four_moments();
    AHistoryRowDrawn {
        job: record.job.as_text(),
        server_row: record.server.local_server.as_text(),
        name: crate::surface::the_queue::what_the_job_is_called(record),
        the_check: Vec::new(),
        in_place_of_the_ending: None,
        the_check_waits_on_its_range: false,
        // **`Queue & History` N6's figure against the job's asked order: the
        // frames the server held** — the 38's piece 10, item 24, and the live
        // pass's fault 5. Absent draws as the drawing's own dash.
        frames: record
            .how_far_it_got()
            .map(|(got, of)| (got.to_string(), of.to_string())),
        // **`Queue & History` C11: an entry whose machine never came back keeps
        // the job with no ending Smoozy ever learned, shown as such, for
        // good** — read through the one mapping [`the_ending_on`].
        ending: Some(the_ending_on(record, start_never_heard, the_server_stands)),
        // **`Queue & History` N12 and `Queue & History` N13: the copied label
        // and hue, and absent
        // is drawn as absent and never as a default colour.**
        server: record.server.label.clone().unwrap_or_default(),
        // **Design, *History*: a removed server's grey squircle in place of
        // its colour**, so it is never read as one still in the rail. The
        // grey replaces `Queue & History` N12's copied hue at the draw; nothing
        // is stored.
        badge: if the_server_stands {
            the_badge_of(record.server.hue)
        } else {
            Some(THE_REMOVED_BADGE.to_string())
        },
        scene: record.scene_name.clone(),
        how_long: moments.exactly_one_duration().map(for_how_long),
        when: moments
            .ended
            .as_ref()
            .map(smoozy_state::observed::Reading::taken_at)
            .or_else(|| moments.the_submitted_moment())
            .and_then(|at| {
                at.duration_since(std::time::UNIX_EPOCH)
                    .ok()
                    .map(|since| since.as_secs().to_string())
            }),
        // **The record's own row**; [`what_is_kept_beside`] marks one whose
        // ending a reading carries.
        the_ending_is_owed: false,
        the_server_is_removed: !the_server_stands,
    }
}

/// **How long it rendered, in the drawing's own two shapes** — *2h 32m* and
/// *47m*.
///
/// **The third shape is this stage's and is recorded rather than assumed.**
/// Design draws nothing shorter than *4m*, and a render that took less than a
/// minute would read *0m*, which `src/surface/settings.rs` already refuses for
/// a reading of its own — a figure of zero for something that plainly happened.
/// **So it reads `<1m`**: a standard shape, not a metaphor, and true. The
/// reasoning is in `Audits/Build/Queue & History/Decisions record.md`.
///
/// **It is the note's two machine-clock readings and never a clock of this
/// computer's** — `Queue & History` N4, kept by the type: the duration handed
/// in is the one `TheFourMoments::exactly_one_duration` produced.
pub(crate) fn for_how_long(rendered: std::time::Duration) -> String {
    let minutes = rendered.as_secs() / 60;
    if minutes >= 60 {
        return format!("{}h {:02}m", minutes / 60, minutes % 60);
    }
    if minutes >= 1 {
        return format!("{minutes}m");
    }
    "<1m".to_string()
}

/// **Design, *The rest of the entry*: down the side, the picture** — `Queue &
/// History` P2's newest derivative the set holds, under a heading that names
/// its frame and, where the copy carries one, the view layer it came from
/// (P10a, read through the built `the_heading_over_the_picture`). History's
/// word under it is *last complete*; the queue's is *latest*, set where the
/// queue draws its entry. The 38's piece 3.
///
/// - **Where no frame finished there is no picture, and one line sits where it
///   was** — an ending short of a clean finish with no figure, which is
///   `Queue & History` N6's *finished no frame*.
/// - **Where frames finished and no copy is held** — not fetched yet, cleared
///   by age, or one the machine could not make — **nothing is drawn**. Design
///   has no words for a picture that is not held, and none are invented here.
pub(crate) fn the_preview_on(
    record: &LocalJobRecord,
    held: Option<&crate::record::previews::AFrameOfTheSet>,
    every_copy_held: &[crate::record::previews::AFrameOfTheSet],
    ending: AnEnding,
    figure: Option<i64>,
) -> Option<crate::surface::the_queue::ThePreview> {
    use smoozy_queue_history::acting::previews::{the_heading_over_the_picture, TheHeadingOverThePicture};
    if let Some(held) = held.filter(|held| held.is_held()) {
        let name = held
            .view_layer
            .as_deref()
            .map(|name| String::from_utf8_lossy(name).into_owned());
        let (frame, layer) = match the_heading_over_the_picture(held.frame, name.as_deref()) {
            TheHeadingOverThePicture::TheFrameAndItsViewLayer { frame, view_layer } => (frame, Some(view_layer)),
            TheHeadingOverThePicture::TheFrameAlone { frame } => (frame, None),
        };
        return Some(crate::surface::the_queue::ThePreview {
            frame: frame.to_string(),
            layer,
            picture: Some(record.job.as_text()),
            which: LAST_COMPLETE,
            follows: false,
            nothing_to_show: false,
            frames_held: crate::surface::the_queue::the_frames_held(every_copy_held),
        });
    }
    let finished_no_frame =
        matches!(ending, AnEnding::Failed | AnEnding::Stopped | AnEnding::GivenUp) && figure.is_none();
    finished_no_frame.then(|| crate::surface::the_queue::ThePreview {
        nothing_to_show: true,
        ..Default::default()
    })
}

/// **The job chosen, by its identity** (must-do 1). A job still in history is
/// chosen; otherwise the first entry is, so an open entry always has a server
/// for *Clear this server's history* to mean; with no entries nothing is.
pub fn the_row_is_chosen(mut kept: TheHistoryOnScreen, chosen: Option<&str>) -> TheHistoryOnScreen {
    kept.chosen = chosen
        .filter(|job| kept.rows.iter().any(|row| row.job == *job))
        .or_else(|| kept.rows.first().map(|row| row.job.as_str()))
        .map(str::to_string);
    kept
}

/// **The box the page asked for, over the entry whose identity the page
/// named.** *Delete* asks exactly as the control does: the one delete box,
/// composed from what that entry says deleting leaves. **An identity no longer
/// in history opens no box** — a job starting elsewhere moves every row, and a
/// box must never open over a different job than the one pressed (must-do 1).
pub fn the_history_as_asked(
    mut kept: TheHistoryOnScreen,
    about: Option<&str>,
    asking: Option<TheHistoryAsk>,
) -> TheHistoryOnScreen {
    let Some(asking) = asking else {
        return kept;
    };
    let Some(about) = about.filter(|job| kept.chosen.as_deref() == Some(*job)) else {
        return kept;
    };
    let Some(row) = kept.rows.iter().find(|row| row.job == about) else {
        return kept;
    };
    kept.asking = match asking {
        // **Every entry the list draws may be deleted** (piece 7): a running
        // job is not in the list, so the key and the control ask alike —
        // **but one whose ending a reading carries while it is owed**, which
        // the record still has running ([`what_is_kept_beside`]).
        //
        // **A check whose full range is still live is not deleted at all**,
        // and a pair's box asks for both (`Queue & History` X1a; the
        // nineteenth notes, note 7).
        TheHistoryAsk::Delete => kept
            .entry
            .as_ref()
            .filter(|entry| entry.entry.job == about && !entry.the_ending_is_owed && !entry.the_check_waits_on_its_range)
            .map(|entry| {
                let leaves = &entry.what_deleting_leaves;
                let mut asking = the_delete_box(leaves.frames_on_the_server, leaves.the_server_is_gone, leaves.downloaded);
                if entry.deleted_with_its_check {
                    asking.heading = vec![APart::words(DELETE_THIS_ENTRY_AND_ITS_CHECK_QUESTION)];
                }
                asking
            }),
        // **`Queue & History` X6 and `Queue & History` X6a: both counts are
        // what goes**, which since piece 7 is every entry in the scope — a
        // running job is not in the list, and one whose ending is owed is
        // not counted — and **a scope with nothing to clear opens no box**.
        TheHistoryAsk::ClearThisServer => {
            let entries = kept
                .rows
                .iter()
                .filter(|one| one.server_row == row.server_row && one.a_clearing_takes_it())
                .count();
            (entries > 0).then(|| the_clear_box(&row.server, entries as u32))
        }
        TheHistoryAsk::ClearAll => {
            let mut per_server: Vec<(String, String, u32)> = Vec::new();
            for one in kept.rows.iter().filter(|one| one.a_clearing_takes_it()) {
                match per_server.iter_mut().find(|(server, _, _)| *server == one.server_row) {
                    Some((_, _, count)) => *count += 1,
                    None => per_server.push((one.server_row.clone(), one.server.clone(), 1)),
                }
            }
            let per_server: Vec<(String, u32)> =
                per_server.into_iter().map(|(_, name, count)| (name, count)).collect();
            let entries = kept.rows.iter().filter(|one| one.a_clearing_takes_it()).count();
            (entries > 0).then(|| the_clear_all_box(entries as u32, &per_server))
        }
        // **`Queue & History` X6b: the removed servers' box lists each one by
        // its copied name with its own count**, over exactly the group the
        // list draws at its foot, and a group with nothing to clear opens no
        // box. The eighteenth notes, note 13.
        TheHistoryAsk::ClearRemovedServers => {
            let mut per_server: Vec<(String, String, u32)> = Vec::new();
            for one in kept.rows.iter().filter(|one| one.the_server_is_removed && one.a_clearing_takes_it()) {
                match per_server.iter_mut().find(|(server, _, _)| *server == one.server_row) {
                    Some((_, _, count)) => *count += 1,
                    None => per_server.push((one.server_row.clone(), one.server.clone(), 1)),
                }
            }
            let entries: u32 = per_server.iter().map(|(_, _, count)| count).sum();
            let per_server: Vec<(String, u32)> =
                per_server.into_iter().map(|(_, name, count)| (name, count)).collect();
            (entries > 0).then(|| the_clear_removed_box(entries, &per_server))
        }
        // **Not a box**: the kept words are read from the record by
        // [`the_history`] and travel on [`TheHistoryOnScreen::words`].
        TheHistoryAsk::BlenderOutput | TheHistoryAsk::TheChecksBlenderOutput => None,
    };
    kept
}

// ───────────────────────────────────────────────────────────────────────────
// M32's hook, with this screen's own value
// ───────────────────────────────────────────────────────────────────────────

/// **The diary is handed the sentence this screen composed** — M32, and
/// [`WhoseSentenceItIs::TheComposersOwn`] because these are Design's words for
/// this ending and no plumbing supplied them.
///
/// **The hook is called after the presentation has been decided and never
/// before**, which is M32's own rule: a record found not to stand hands the
/// diary nothing.
///
/// **A screen that passed the other value would produce no error, no failing
/// test and no visible fault** — only a problem report months later saying
/// Smoozy said something it did not say. That is why this function exists at
/// all rather than the call being written inline: there is one call site and it
/// is named.
pub fn the_diary_is_handed_this_screens_sentence(
    started: &mut Started,
    job: &Identity,
    failure: &TheEndingsFailure,
) -> Option<TheDiaryWasHanded> {
    let sentence = failure.as_the_diary_keeps_it();
    let diary = started.diary.as_ref();
    let run = started.run.clone();
    let record = started.record.as_ref()?;
    let connection = record.connection();
    let mut records = FailureRecords::over(connection);
    let subject = DurableSubject::HistoryRecord(job.clone());
    let standing: Vec<Standing> = records.standing_against(&subject).ok()?;
    let first = standing.into_iter().next()?;
    Some(the_diary_is_handed_the_sentence(
        diary,
        &mut records,
        run.as_ref(),
        &first,
        // **This screen composed the sentence the user reads.** M32's repair of
        // 2026-09-12, and the one value this screen may pass.
        WhoseSentenceItIs::TheComposersOwn(&sentence),
    ))
}

// ───────────────────────────────────────────────────────────────────────────
// The commands
// ───────────────────────────────────────────────────────────────────────────

// ───────────────────────────────────────────────────────────────────────────
// The three acts this screen performs — step 10b piece 8
// ───────────────────────────────────────────────────────────────────────────

/// **What a deletion the record refused says.** The words are the shape
/// Files and the dependency check use for a write or read that would not go
/// through — *Could not read Smoozy's database.* — with the act's own verb,
/// because a delete is not a read and one wording doing two jobs is what Rule
/// 9 catches. **Ruled at this site**: Design writes the boxes and gives no
/// sentence for a refused write. *(Step 13's words sweep, 2026-10-28: it read
/// "Smoozy could not delete that from its own record.")*
pub const THE_RECORD_REFUSED_IT: &str = "Could not delete that from Smoozy's database.";

/// **The delete box's go-ahead and the three clearing boxes, performed** —
/// `Queue & History` X1, `Queue & History` X6, `Queue & History` X6a and
/// `Queue & History` X6b.
///
/// **Nothing here composes a deletion.** `smoozy_queue_history::deletion` does
/// — `ADeletion::of` and `a_clearing` — and this reads the records, hands them
/// over, and performs what comes back through
/// [`crate::record::deletions::TheDeletions`], which is the one place the
/// record and its tombstone are written as a unit.
///
/// **`None` means this act does not come here at all** and answers instead;
/// `Some(Err(()))` is an act that came here and could not be done.
///
/// **The erase on the machine is owed and is not attempted.** `Queue & History`
/// X2's units are run at the next connection to a machine holding the identity,
/// and the debt rides the tombstone this writes — which is X4's *a deletion
/// made while disconnected is remembered as a deletion, and only as one*.
/// **Every deletion Smoozy can make today is made that way**, nothing having a
/// route to a machine's supervisor.
pub fn the_act_is_performed(
    started: &Started,
    act: TheActOnAJob,
    job: Option<&str>,
) -> Option<Result<(), ()>> {
    use crate::record::deletions::TheDeletions;
    use crate::surface::the_queue::{where_an_act_on_a_job_goes, WhereAnActOnAJobGoes};
    use smoozy_queue_history::deletion::{
        a_clearing, ADeletion, AnEntryTheListDraws, ClearingScope,
    };
    use smoozy_state::identities::IdentityKind;

    let goes = where_an_act_on_a_job_goes(act);
    // **The queue's own acts are not this function's** — they raise a box or
    // reach a machine, and `an_act_on_a_job` routes them before this is asked
    // (step 10c's S3).
    if matches!(
        goes,
        WhereAnActOnAJobGoes::ItAnswers
            | WhereAnActOnAJobGoes::ItsBox
            | WhereAnActOnAJobGoes::ItReachesTheMachine(_)
    ) {
        return None;
    }
    let Some(record) = started.record.as_ref() else {
        return Some(Err(()));
    };
    let store = JobRecords::over(record.connection());
    let Ok(held) = store.in_history() else {
        return Some(Err(()));
    };
    // **Every act here reaches only the entries the list draws** — read as
    // [`what_is_kept`] reads them, through [`AnEnding::the_list_draws_it`] and
    // `Queue & History` N17's server-row lookup, so the box's count and what
    // is deleted are one reading. **A running job's record is never reached
    // from History** (piece 7): not by a clear, and not by a delete asked from
    // a page drawn before the list changed.
    let standing = the_server_rows_that_stand(Some(record));
    let held: Vec<LocalJobRecord> = held
        .entries
        .into_iter()
        .filter(|one| {
            let stands = standing.contains(&one.record.server.local_server.as_text());
            the_ending_on(&one.record, one.start_never_heard, stands).the_list_draws_it()
        })
        .map(|one| one.record)
        .collect();
    // **`Queue & History` X1a: a check whose full range is still live is
    // reached by nothing here** — not a delete, not a clearing — because the
    // waiting question shows its picture; and **a pair is deleted as one**,
    // X1 over each of its two records. The nineteenth notes, note 7.
    let drawn: std::collections::HashSet<String> = held.iter().map(|one| one.job.as_text()).collect();
    let is_drawn = |job: &Identity| drawn.contains(&job.as_text());
    // **The open entry's server**, which *this server* means — read before the
    // check that waits on its range is set aside, since its entry may be the
    // one open.
    let the_open_entrys_server = job
        .and_then(|text| Identity::read_back(IdentityKind::Job, text))
        .and_then(|job| held.iter().find(|record| record.job == job))
        .map(|record| record.server.local_server.clone());
    let parts: Vec<AsAPart> = held.iter().map(|one| as_a_part(&store, one, &is_drawn)).collect();
    let held: Vec<LocalJobRecord> = held
        .into_iter()
        .zip(parts.iter())
        .filter(|(_, part)| !matches!(part, AsAPart::TheCheckAlone(_, TheRangeInHistory::InQueue, _)))
        .map(|(one, _)| one)
        .collect();
    let the_partner_of = |record: &LocalJobRecord| {
        parts.iter().find_map(|part| match part {
            AsAPart::TheHiddenCheck(pair) | AsAPart::TheRangeWithItsCheck(pair, _)
                if pair.check == record.job || pair.range == record.job =>
            {
                let partner = if pair.check == record.job { &pair.range } else { &pair.check };
                held.iter().find(|one| one.job == *partner)
            }
            _ => None,
        })
    };
    // **`Queue & History` C9's test is the store's** — `in_history` reads the
    // records whose render start has been learned, and since item C1 the ones
    // marked *start never heard* too, and no others — so every entry handed
    // over below is one the list draws, once piece 7's filter above has run.
    //
    // **The flag is that crate's whole C9 test and it names only the first way
    // in.** `App/Queue & History/src/deletion.rs` reads it as *the list draws
    // this entry*, which is what is passed; a marked entry is deleted like any
    // other, which is the ruling's own words. Recorded for the runner: that
    // crate's `two_records::is_in_history` does not know C9's second way in.
    let entries: Vec<AnEntryTheListDraws<'_>> = held
        .iter()
        .map(|record| AnEntryTheListDraws {
            record,
            the_render_start_has_been_learned: true,
        })
        .collect();
    let now = std::time::SystemTime::now();
    let chosen = job
        .and_then(|text| Identity::read_back(IdentityKind::Job, text))
        .and_then(|job| held.iter().find(|record| record.job == job));
    let deletions = match goes {
        // **A box that named a job no longer in history deletes nothing and
        // says so**, which is must-do 1's own rule for a box composed over a
        // place: the list moved under the user, and deleting the job that has
        // taken that place would be exactly the fault that must-do repaired.
        WhereAnActOnAJobGoes::ADeletion => match chosen {
            Some(record) => std::iter::once(record)
                .chain(the_partner_of(record))
                .map(|record| ADeletion::of(record, now))
                .collect(),
            None => return Some(Err(())),
        },
        // **Design, *Clearing history*: *this server* is the server of the
        // entry that is open**, and there is nowhere else on this screen for
        // the act to take one from.
        WhereAnActOnAJobGoes::AClearingOfOneServersHistory => match the_open_entrys_server {
            Some(server) => a_clearing(&entries, &ClearingScope::OneMachines(server), now),
            None => return Some(Err(())),
        },
        WhereAnActOnAJobGoes::AClearingOfTheWholeOfHistory => {
            a_clearing(&entries, &ClearingScope::TheWholeOfHistory, now)
        }
        // **`Queue & History` X6b: every entry whose copied server identifier
        // resolves to no server row**, read now with `Queue & History` N17's
        // lookup over the local records alone, reaching no machine. **A server
        // list that will not read clears nothing**: read as empty it would
        // take every entry.
        WhereAnActOnAJobGoes::AClearingOfTheRemovedServersHistory => {
            let Ok(rows) = crate::record::server_rows::all(record.connection()) else {
                return Some(Err(()));
            };
            let the_rows_that_stand = rows
                .iter()
                .filter_map(|row| Identity::read_back(IdentityKind::LocalServer, &row.local_server_identifier))
                .collect();
            a_clearing(&entries, &ClearingScope::TheServersNoLongerInSmoozy { the_rows_that_stand }, now)
        }
        WhereAnActOnAJobGoes::ItAnswers
        | WhereAnActOnAJobGoes::TheArrangementIsSwitchedOff
        | WhereAnActOnAJobGoes::TheJobsFilesAreShown
        | WhereAnActOnAJobGoes::TheJobsFolderOpensInFiles
        | WhereAnActOnAJobGoes::TheNoticeIsDismissed
        | WhereAnActOnAJobGoes::ItsBox
        | WhereAnActOnAJobGoes::ItReachesTheMachine(_)
        | WhereAnActOnAJobGoes::ItAnswersTheHold(_)
        | WhereAnActOnAJobGoes::ANewJobFromTheEntry
        | WhereAnActOnAJobGoes::TheEarlierEntryOpens
        | WhereAnActOnAJobGoes::TheRemainingScenesAreSubmitted
        // **The untracked offer accepted is the queue's own** (owed piece 10).
        | WhereAnActOnAJobGoes::TheWorkIsTracked
        | WhereAnActOnAJobGoes::TheReportingIsChanged
        // **A move on the line is the queue's own** (the 38's piece 8).
        | WhereAnActOnAJobGoes::ItMovesOnTheLine => return None,
    };
    Some(
        TheDeletions::over(record.connection())
            .perform_all(&deletions)
            .map_err(|_| ()),
    )
}

/// **The page asks what history holds.** P6: the reading arrives as an event.
///
/// **`chosen` is a job's identity and never a place in the list** (must-do 1),
/// and a box is asked for over that job.
#[tauri::command]
pub fn the_history(
    started: State<'_, Mutex<Started>>,
    queue: State<'_, std::sync::Arc<TheOneQueue>>,
    periods: State<'_, std::sync::Arc<crate::servers::a_period::ThePeriodsThisRun>>,
    chosen: Option<String>,
    asking: Option<TheHistoryAsk>,
) {
    let connected = crate::surface::the_queue::the_servers_connected(periods.the_machines_work());
    let owed = the_endings_owed_in(periods.the_machines_work());
    let Ok(started) = started.lock() else {
        return;
    };
    if let Ok(mut held) = the_chosen_entry().lock() {
        held.clone_from(&chosen);
    }
    history_is_opened(started.record.as_ref());
    the_history_is_posted(
        &queue,
        what_the_page_asked_for(
            started.record.as_ref(),
            chosen.as_deref(),
            asking,
            &|server| connected.contains(server),
            &owed,
        ),
    );
}

/// **Opening History clears the queue's count** — `Queue & History` C2a as
/// changed 2026-10-12, Creator-stated, the Creator's ninth notes (2026-10-11),
/// note 3, and `Design/Queue & History § The catch-up notice`: *"Pressing it
/// opens History, where every one of those jobs is, and opening History clears
/// it."* Every ending comes off every standing notice record; a job dropped
/// before it rendered keeps its line under the Queue heading, since it has no
/// entry here.
///
/// **Every ask from this screen's page is History on screen** — it is drawn
/// the first time the user goes there and asks again each time it is current,
/// and its other asks come from controls on it — so [`the_history`] calls this
/// on each. History drawn again by a connection (`the_history_is_told_again`)
/// is the application's act, not the user's opening, and does not.
///
/// **A record that refuses the write leaves the count standing**, which still
/// says nothing untrue: those jobs did end, and the next opening tries again.
pub fn history_is_opened(records: Option<&LocalRecord>) {
    if let Some(record) = records {
        let _ = crate::record::catch_up_notices::history_is_opened(record.connection());
    }
}

/// **History as the page asked for it** — what is kept, the box asked for,
/// and **the chosen entry's kept words where *Blender output* was pressed**
/// (`Queue & History` E9), read from the record at that one moment and never
/// at a draw. **Only over the job the press named** (must-do 1): a list that
/// moved under the press opens nothing. The Creator's notes on 0.1.8, piece
/// 6d.
pub fn what_the_page_asked_for(
    records: Option<&LocalRecord>,
    chosen: Option<&str>,
    asking: Option<TheHistoryAsk>,
    is_connected: &dyn Fn(&str) -> bool,
    owed: &[AnEndingTheReadingCarries],
) -> TheHistoryOnScreen {
    let kept = what_is_kept_beside(records, chosen, is_connected, owed);
    let mut says = the_history_as_asked(kept, chosen, asking);
    if asking == Some(TheHistoryAsk::BlenderOutput)
        && says.chosen.is_some()
        && says.chosen.as_deref() == chosen
    {
        says.words = the_kept_words(records, &says);
    }
    // **The check's own words, from a pair's box**, where the box offers them.
    if asking == Some(TheHistoryAsk::TheChecksBlenderOutput)
        && says.chosen.is_some()
        && says.chosen.as_deref() == chosen
    {
        says.words = says.entry.as_ref().and_then(|entry| {
            let the_check = entry.entry.the_check.as_ref().filter(|the_check| the_check.blender_output)?;
            the_kept_words_of(records, &the_check.job, &entry.entry.name, &entry.entry.server)
        });
    }
    says
}

/// **The job whose entry the page last asked for**, held for the run so that
/// history drawn again by a connection keeps the user's choice.
fn the_chosen_entry() -> &'static Mutex<Option<String>> {
    static CHOSEN: std::sync::OnceLock<Mutex<Option<String>>> = std::sync::OnceLock::new();
    CHOSEN.get_or_init(|| Mutex::new(None))
}

/// **History, drawn again because a connection moved the record under it** —
/// `State, identity and recovery` V1 steps 2 and 4, and a P10d pass that
/// learned a start or an ending (step 10c piece 7). No box is raised: a box is
/// the page's to ask for.
pub fn the_history_is_told_again<R: tauri::Runtime>(app: &tauri::AppHandle<R>) {
    use tauri::Manager;
    let Some(queue) = app.try_state::<std::sync::Arc<TheOneQueue>>() else {
        return;
    };
    let Some(started) = app.try_state::<Mutex<Started>>() else {
        return;
    };
    let over = what_history_is_drawn_over(app);
    let Ok(started) = started.lock() else {
        return;
    };
    the_history_is_drawn_again(&queue, started.record.as_ref(), &over);
}

/// **What the run's periods say History is drawn over**: the servers
/// connected, and the endings their readings carry while they are owed —
/// drawn until the landing replaces them ([`what_is_kept_beside`]). Read
/// apart from the record's lock, so a caller already holding it can draw.
#[derive(Debug, Clone, Default)]
pub struct WhatHistoryIsDrawnOver {
    pub connected: std::collections::BTreeSet<String>,
    pub owed: Vec<AnEndingTheReadingCarries>,
}

pub fn what_history_is_drawn_over<R: tauri::Runtime>(app: &tauri::AppHandle<R>) -> WhatHistoryIsDrawnOver {
    use tauri::Manager;
    let periods = app.try_state::<std::sync::Arc<crate::servers::a_period::ThePeriodsThisRun>>();
    WhatHistoryIsDrawnOver {
        connected: periods
            .as_ref()
            .map(|periods| crate::surface::the_queue::the_servers_connected(periods.the_machines_work()))
            .unwrap_or_default(),
        owed: periods
            .as_ref()
            .map(|periods| the_endings_owed_in(periods.the_machines_work()))
            .unwrap_or_default(),
    }
}

/// **History drawn again from the record in hand**, keeping the user's chosen
/// entry — the one composition every redraw goes through, the removal of a
/// server included (step 11's D6b).
pub fn the_history_is_drawn_again(
    queue: &TheOneQueue,
    records: Option<&crate::record::LocalRecord>,
    over: &WhatHistoryIsDrawnOver,
) {
    let chosen = the_chosen_entry().lock().ok().and_then(|held| held.clone());
    let kept = what_is_kept_beside(records, chosen.as_deref(), &|server| over.connected.contains(server), &over.owed);
    the_history_is_posted(queue, kept);
}

/// The post, with the reading handed in — the same split every piece makes.
pub fn the_history_is_posted(queue: &TheOneQueue, says: TheHistoryOnScreen) {
    let _ = queue.read_then_post::<TheHistoryOnScreen, std::convert::Infallible>(
        || Ok(says.clone()),
        the_history_says,
    );
}

// ───────────────────────────────────────────────────────────────────────────
// The event
// ───────────────────────────────────────────────────────────────────────────

/// **What history says, in the two routes and never in one string.**
///
/// **Every list is sent whole**, its field names made by the one name-maker
/// (must-do 3): until then a history longer than twenty-four entries was cut
/// with nothing said.
pub fn the_history_says(says: &TheHistoryOnScreen) -> Event {
    let mut event = Event::about(About::TheRun, THE_HISTORY_SAYS)
        .saying("theHeading", Said::Prose(Prose::new(THE_HEADING)));

    // **Must-do 3(a): a read that failed is said, in place of the list**, and
    // nothing of the list is sent with it — no count, no *kept until you clear
    // them*, no clearing control, no entry.
    if says.could_not_read {
        return event.saying(
            "couldNotRead",
            Said::Prose(Prose::new(SMOOZY_COULD_NOT_READ_YOUR_HISTORY)),
        );
    }
    event = event
        .saying(
            "theJobsFigure",
            Said::Value(Verbatim::new(says.jobs.to_string())),
        )
        .saying(
            "theJobsWord",
            Said::Prose(Prose::new(by_count(says.jobs, THE_JOB_WORD, THE_JOBS_WORD))),
        )
        .saying(
            "keptUntilYouClearThem",
            Said::Prose(Prose::new(KEPT_UNTIL_YOU_CLEAR_THEM)),
        );
    // **The entry's *Delete* is sent with every entry the list draws** —
    // since piece 7 a running job is not in the list (E1-1's entry with no
    // *Delete* has gone with it), and the page draws the control only where
    // its word arrives.
    //
    // **Not over an entry whose ending a reading carries while it is owed**
    // ([`what_is_kept_beside`]): *Delete* is an act on the record, which has
    // the job still running.
    //
    // **Nor over a check whose full range is still live** (`Queue & History`
    // X1a): the waiting question shows its picture.
    if says.entry.as_ref().is_some_and(|entry| !entry.the_ending_is_owed && !entry.the_check_waits_on_its_range) {
        event = event.saying("delete", Said::Prose(Prose::new(DELETE)));
    }

    // **Must-do 3(c): rows that would not read back are counted and said.**
    if says.could_not_be_read > 0 {
        event = event
            .saying(
                "unreadFigure",
                Said::Value(Verbatim::new(says.could_not_be_read.to_string())),
            )
            .saying(
                "unreadWord",
                Said::Prose(Prose::new(by_count(
                    says.could_not_be_read,
                    ENTRY_COULD_NOT_BE_READ,
                    ENTRIES_COULD_NOT_BE_READ,
                ))),
            );
    }

    // **Must-do 5(e): with no entries, neither clearing control is drawn.**
    // *This server* is the server of the entry that is open, and an empty list
    // has nothing to clear.
    //
    // **Since piece 7 every entry drawn is one a clear takes**, but one whose
    // ending a reading carries while it is owed ([`what_is_kept_beside`]), so
    // a list with a row of the record's own always has something to go.
    let clearable = |row: &&AHistoryRowDrawn| row.a_clearing_takes_it();
    if says.rows.iter().any(|row| clearable(&row)) {
        event = event.saying(
            "clearAllHistory",
            Said::Prose(Prose::new(CLEAR_ALL_HISTORY)),
        );
        // *This server* is the open entry's, so the control needs one open,
        // and that server something to clear.
        let open = says
            .chosen
            .as_ref()
            .and_then(|job| says.rows.iter().find(|row| &row.job == job));
        if open.is_some_and(|open| {
            says.rows.iter().filter(clearable).any(|row| row.server_row == open.server_row)
        }) {
            event = event.saying(
                "clearThisServersHistory",
                Said::Prose(Prose::new(CLEAR_THIS_SERVERS_HISTORY)),
            );
        }
    }

    // **Design, *History*: the *Removed servers* group at the foot, its count,
    // and the fourth clearing act on its own line under the heading** — there
    // only while the group holds an entry a clear takes (the eighteenth notes,
    // notes 12 and 13). The fold is the page's, kept for the run.
    let removed = says.rows.iter().filter(|row| row.the_server_is_removed).count() as u32;
    if removed > 0 {
        event = event
            .saying("theRemovedServers", Said::Prose(Prose::new(REMOVED_SERVERS)))
            .saying("theRemovedFigure", Said::Value(Verbatim::new(removed.to_string())))
            .saying(
                "theRemovedWord",
                Said::Prose(Prose::new(by_count(removed, ENTRY_WORD, ENTRIES_WORD))),
            );
        if says.rows.iter().any(|row| row.the_server_is_removed && clearable(&row)) {
            event = event.saying(
                "clearRemovedServersHistory",
                Said::Prose(Prose::new(CLEAR_REMOVED_SERVERS_HISTORY)),
            );
        }
    }

    for (at, row) in says.rows.iter().enumerate() {
        if row.the_server_is_removed {
            event = event.saying(a_name("hrow", at, "Removed"), Said::Value(Verbatim::new("yes")));
        }
        event = event
            .saying(a_name("hrow", at, "Job"), Said::Value(Verbatim::new(row.job.clone())))
            .saying(a_name("hrow", at, "Name"), Said::Value(Verbatim::new(row.name.clone())))
            .saying(a_name("hrow", at, "Server"), Said::Value(Verbatim::new(row.server.clone())));
        event = crate::surface::the_queue::the_pair_is_said(event, "hrow", at, &row.the_check);
        if let Some(when) = &row.when {
            event = event
                .saying(a_name("hrow", at, "When"), Said::Value(Verbatim::new(when.clone())));
        }
        if let Some((done, of)) = &row.frames {
            event = event
                .saying(a_name("hrow", at, "Frames"), Said::Value(Verbatim::new(done.clone())))
                .saying(a_name("hrow", at, "Of"), Said::Value(Verbatim::new(of.clone())));
        }
        // **A check drawn alone reads *In Queue* or *Check only* where the
        // ending goes, in grey** — the drawing's `e-chk` (the nineteenth notes,
        // note 7).
        if let Some(instead) = row.in_place_of_the_ending {
            event = event
                .saying(a_name("hrow", at, "Ending"), Said::Prose(Prose::new(instead)))
                .saying(a_name("hrow", at, "EndingIs"), Said::Value(Verbatim::new(THE_CHECKS_GREY)));
        } else if let Some(ending) = row.ending {
            event = event
                .saying(a_name("hrow", at, "Ending"), Said::Prose(Prose::new(ending.said())))
                .saying(a_name("hrow", at, "EndingIs"), Said::Value(Verbatim::new(ending.class())));
        }
        if let Some(badge) = &row.badge {
            event = event.saying(a_name("hrow", at, "Badge"), Said::Value(Verbatim::new(badge.clone())));
        }
        if let Some(scene) = &row.scene {
            event = event.saying(a_name("hrow", at, "Scene"), Said::Value(Verbatim::new(scene.clone())));
        }
        if let Some(how_long) = &row.how_long {
            event = event.saying(
                a_name("hrow", at, "HowLong"),
                Said::Value(Verbatim::new(how_long.clone())),
            );
        }
    }
    event = event.saying("hrowCount", Said::Value(Verbatim::new(says.rows.len().to_string())));
    // **The chosen entry travels as its job's identity** (must-do 1).
    if let Some(chosen) = &says.chosen {
        event = event.saying("hrowChosen", Said::Value(Verbatim::new(chosen.clone())));
    }

    if let Some(entry) = &says.entry {
        event = the_entry_is_said(event, Some(&entry.entry));
        event = the_history_entry_is_said(event, entry);
    }
    // **The kept words, where the page asked for them** — the one panel's own
    // keys, composed where the panel's other event is (`Queue & History` E9).
    if let Some(words) = &says.words {
        event = crate::surface::the_machines_own_words::the_kept_words_are_said(
            event,
            &words.job,
            &words.server,
            &words.attempts,
        );
    }
    the_box_is_said(event, says.asking.as_ref())
}

/// **The subject a history entry's ending is recorded against**, in
/// `State, identity and recovery` §8.7's written form — the same words
/// `crate::record::failure_records::DurableSubject::kind_as_text` writes for a
/// history record, and the test beside the report box reads it back as one.
pub(crate) const THE_SUBJECT_AN_ENDING_IS_RECORDED_AGAINST: &str = "history record";

fn the_history_entry_is_said(mut event: Event, entry: &TheHistoryEntry) -> Event {
    // **Design, *History*: the line under a removed server's entry reads
    // *Lambda A100, removed*** — the copied name, then these words. History's
    // entry reaches back exactly where its server stands (`Queue & History`
    // N17), so that is the test. The eighteenth notes, note 12.
    if !entry.entry.reach_backs {
        event = event.saying("entryServerRemoved", Said::Prose(Prose::new(REMOVED_TRAIL)));
    }
    // **History's own *Blender output*, on a clean finish whose words are
    // kept** — the control's label, and the page asks for the words with it.
    if entry.blender_output {
        event = event.saying(
            "historyBlenderOutput",
            Said::Prose(Prose::new(TheSharedWords::BLENDER_OUTPUT)),
        );
    }
    if let Some(ending) = entry.ending {
        event = event
            .saying("entryEnding", Said::Prose(Prose::new(ending.said())))
            .saying("entryEndingIs", Said::Value(Verbatim::new(ending.class())));
    }

    if let Some(failure) = &entry.failure {
        // **The parts of the one shape, and the page hands them to
        // `theFailureShape`** — the builder `web/the-failure-shape.js` exports.
        // **Nothing here draws a shape and nothing on the page composes one.**
        event = event
            .saying("failSaid", Said::Prose(Prose::new(failure.said.clone())))
            // **M32's captured string: the sentence before the point's
            // substitutions**, which is also what `Reporting a problem` X4 is
            // handed. The two are one value and cannot disagree.
            .saying(
                "failSaidForTheReport",
                Said::Value(Verbatim::new(failure.said.clone())),
            )
            // **Which of `State, identity and recovery` §8.7's subjects the
            // ending is recorded against**, in that list's written form, so
            // M33's handover from this entry names its subject — the entry's
            // own job, as a history record. Added at step 11's piece R4, when
            // the report box began to consume the handover and this one
            // carried no subject at all.
            .saying(
                "failSubjectKind",
                Said::Value(Verbatim::new(THE_SUBJECT_AN_ENDING_IS_RECORDED_AGAINST)),
            );
        if let Some(second) = &failure.what_to_do_about_it {
            event = event.saying(
                "failWhatToDoAboutIt",
                Said::Prose(Prose::new(second.clone())),
            );
        }
        // **Design, *Where no render began*: where a second sentence goes, as a
        // value**, since it can hold a path as the file records it.
        if let Some(reason) = &failure.the_reason_beneath {
            event = event.saying("failWhatToDoAboutIt", Said::Value(Verbatim::new(reason.clone())));
        }
        // **Each value beside its label: the label is prose the user reads and
        // the value is not** (must-do 20).
        for (at, (label, value)) in failure.values.iter().enumerate() {
            event = event
                .saying(a_name("failValue", at, "Label"), Said::Prose(Prose::new(*label)))
                .saying(a_name("failValue", at, ""), Said::Value(Verbatim::new(value.clone())));
        }
        event = event.saying(
            "failValueCount",
            Said::Value(Verbatim::new(failure.values.len().to_string())),
        );
        event = crate::surface::the_failure_shape::the_controls_on_the_event(
            event,
            &failure.the_controls(),
        );
        // **The server's own words, whole, for the panel its control opens**
        // — a value, never passed through the typography point (M9) — with the
        // panel's title and its *Close* as prose. **No note under them**: the
        // panel's note says Smoozy does not work out why, and these words are
        // the machine's own statement of why, so the Set up tab's panel over
        // the same kind of words carries none either.
        if let Some(words) = &failure.what_the_server_said {
            event = event
                .saying("failTheServersWords", Said::Value(Verbatim::new(words.clone())))
                .saying("failWordsCopy", Said::Prose(Prose::new(crate::surface::the_machines_own_words::COPY)))
                .saying(
                    "failTheServersWordsTitle",
                    Said::Prose(Prose::new(WHAT_THE_SERVER_SAID)),
                )
                .saying(
                    "failWordsClose",
                    Said::Prose(Prose::new(crate::surface::the_machines_own_words::CLOSE)),
                );
        }
    }

    if let Some(again) = &entry.again {
        event = event
            .saying("renderItAgain", Said::Prose(Prose::new(RENDER_IT_AGAIN)))
            .saying("submitAgain", Said::Prose(Prose::new(SUBMIT_AGAIN)))
            .saying(
                "submitAgainFrames",
                Said::Value(Verbatim::new(again.submit_again.to_string())),
            )
            .saying(
                "submitAgainFramesWord",
                Said::Prose(Prose::new(by_count(again.submit_again, FRAME_WORD, FRAMES_WORD))),
            );
        match &again.carry_on {
            Some(carry_on) => {
                let from = match carry_on.came_from {
                    WhereTheFigureCameFrom::ReadFromTheServerJustNow => READ_FROM_THE_SERVER_JUST_NOW,
                    WhereTheFigureCameFrom::RecordedWhenTheJobEnded => THE_FRAME_THIS_ENTRY_RECORDED,
                };
                event = event
                    .saying("carryOn", Said::Prose(Prose::new(CARRY_ON)))
                    .saying(
                        "carryOnFrames",
                        Said::Value(Verbatim::new(carry_on.frames.to_string())),
                    )
                    .saying(
                        "carryOnFramesWord",
                        Said::Prose(Prose::new(by_count(carry_on.frames, FRAME_WORD, FRAMES_WORD))),
                    )
                    .saying(
                        "carryOnMakesANewJob",
                        Said::Prose(Prose::new(CARRY_ON_MAKES_A_NEW_JOB)),
                    )
                    .saying(
                        "carryingOnFromFrame",
                        Said::Prose(Prose::new(CARRYING_ON_FROM_FRAME)),
                    )
                    // **The field's starting value, and the *why* line's frame,
                    // are one figure the application sent** — the page adds
                    // nothing (must-do 2(b)).
                    .saying("carryOnFrom", Said::Value(Verbatim::new(carry_on.at.to_string())))
                    .saying("carryOnFromSays", Said::Prose(Prose::new(from)))
                    .saying("theFrameWord", Said::Prose(Prose::new(THE_FRAME_WORD)))
                    .saying(
                        "carryOnWhyFrame",
                        Said::Value(Verbatim::new(carry_on.at.to_string())),
                    )
                    .saying(
                        "isTheOneSmoozyCannotConfirm",
                        Said::Prose(Prose::new(IS_THE_ONE_SMOOZY_CANNOT_CONFIRM)),
                    );
            }
            // **Absent rather than greyed, and one line says why** — Design's
            // own rule for the missing carry-on.
            None => {
                if let Some(line) = again.the_line() {
                    event = event.saying("carryOnWhy", Said::Prose(Prose::new(line)));
                }
            }
        }
    }

    if !entry.rendered.is_empty() {
        event = event
            .saying("renderedHeading", Said::Prose(Prose::new(RENDERED_HEADING)))
            .saying(
                "renderedCount",
                Said::Value(Verbatim::new(entry.rendered.len().to_string())),
            );
    }
    for (at, line) in entry.rendered.iter().enumerate() {
        event = event.saying(
            a_name("rendered", at, "Name"),
            Said::Prose(Prose::new(line.name)),
        );
        if let Some(part) = line.parts.first() {
            event = a_part_is_said_under(event, "rendered", at, "", part);
        }
        // **The second half of the last-frame line is its own part**, because
        // it carries a figure of its own — the frame Smoozy cannot confirm —
        // and a figure never travels inside the words beside it (X7).
        if let Some(part) = line.parts.get(1) {
            event = a_part_is_said_under(event, "renderedB", at, "", part);
        }
        // **Which lines are moments, so the page writes them as dates.**
        if line.a_moment {
            event = event.saying(
                a_name("rendered", at, "Moment"),
                Said::Value(Verbatim::new("yes")),
            );
        }
    }
    if let Some(note) = entry.rendered_note {
        event = event.saying("renderedNote", Said::Prose(Prose::new(note)));
    }
    event
}

// ───────────────────────────────────────────────────────────────────────────
// The three boxes this screen raises
// ───────────────────────────────────────────────────────────────────────────

/// **Design, *Deleting an entry*: the record's sentence, then what is not
/// changed, and the control is one word.** Where nothing of the job was
/// downloaded the downloaded half is absent, because there is nothing to name.
///
/// **Must-do 10: never *0 frames*, and never *nothing* for *not known*.** A
/// count that is known is said as a count, and none downloaded draws no
/// sentence about downloads, because there is nothing untouched to name.
///
/// **The upstream must-do of step 10b round 1 of *jobs, history and downloads*: a count that is
/// not known is said without one, and never left out.** Until then an unknown
/// count drew no sentence, and since the entry never holds either count the
/// box never said the renders were safe on a server that still stood — the
/// one thing Design says the user is checking. The downloaded half reuses
/// [`ANYTHING_ALREADY_DOWNLOADED`], the box's own words for a server that is
/// gone, rather than a second copy of them.
pub fn the_delete_box(
    frames_on_the_server: Option<u32>,
    the_server_is_gone: bool,
    downloaded: Option<u32>,
) -> TheBoxThatAsks {
    let mut said = vec![vec![APart::words(THE_RECORD_IS_DELETED)]];
    // **The server's half**, where it has one to say: a count, or none known.
    let on_the_server = if the_server_is_gone {
        said.push(vec![APart::words(WHATEVER_IT_RENDERED_IS_ON_A_SERVER)]);
        None
    } else {
        match frames_on_the_server {
            Some(0) => {
                said.push(vec![APart::words(NOTHING_WAS_RENDERED)]);
                None
            }
            known => Some(known),
        }
    };
    // **The downloaded half**: nothing where none was downloaded.
    let downloaded = match downloaded {
        Some(0) => None,
        known => Some(known),
    };
    match (on_the_server, downloaded) {
        (Some(on_the_server), Some(downloaded)) => {
            let mut line = vec![match on_the_server {
                Some(how_many) => APart::figure(how_many.to_string(), by_count(how_many, FRAME_ON_THE_SERVER_AND, FRAMES_ON_THE_SERVER_AND)),
                None => APart::words(FRAMES_ON_THE_SERVER_UNCOUNTED_AND),
            }];
            line.push(match downloaded {
                Some(how_many) => APart::figure(how_many.to_string(), by_count(how_many, DOWNLOADED_FRAME, DOWNLOADED_FRAMES)),
                None => APart::words(DOWNLOADED_FRAMES_UNCOUNTED),
            });
            line.push(APart::words(ARE_NOT_CHANGED));
            said.push(line);
        }
        (Some(on_the_server), None) => said.push(match on_the_server {
            Some(how_many) => vec![
                APart::figure(how_many.to_string(), by_count(how_many, FRAME_ON_THE_SERVER, FRAMES_ON_THE_SERVER)),
                APart::words(by_count(how_many, IS_NOT_CHANGED, ARE_NOT_CHANGED)),
            ],
            None => vec![APart::words(THE_FRAMES_ON_THE_SERVER_ARE_UNTOUCHED_UNCOUNTED), APart::words(ARE_NOT_CHANGED)],
        }),
        (None, Some(Some(how_many))) => said.push(vec![
            APart::figure(how_many.to_string(), by_count(how_many, DOWNLOADED_FRAME, DOWNLOADED_FRAMES)),
            APart::words(by_count(how_many, IS_NOT_CHANGED, ARE_NOT_CHANGED)),
        ]),
        (None, Some(None)) => said.push(vec![APart::words(ANYTHING_ALREADY_DOWNLOADED)]),
        (None, None) => {}
    }
    TheBoxThatAsks {
        which: "delete",
        heading: vec![APart::words(DELETE_THIS_ENTRY_QUESTION)],
        said,
        keep: KEEP_THE_ENTRY,
        go_ahead: vec![APart::words(DELETE)],
        grave: true,
        go_ahead_act: TheActOnAJob::DeleteTheEntry,
        per_server: Vec::new(),
    }
}

/// **Design, *Clearing history*: one server's history is bounded and
/// countable, so the box names the server and says how many entries go.**
pub fn the_clear_box(server: &str, entries: u32) -> TheBoxThatAsks {
    let mut said = vec![vec![
        APart::words(ITS_WORD),
        APart::figure(
            entries.to_string(),
            by_count(entries, ITS_ENTRY_IS_DELETED, ITS_ENTRIES_ARE_DELETED),
        ),
    ]];
    said.push(vec![
        APart::words(THE_FRAMES_ON_LEAD),
        APart::said("", server.to_string(), THE_FRAMES_ON_THIS_SERVER_ARE_UNTOUCHED),
    ]);
    TheBoxThatAsks {
        which: "clear",
        heading: vec![
            APart::words(CLEAR_LEAD),
            APart::said("", server.to_string(), HISTORY_QUESTION_TRAIL),
        ],
        said,
        keep: KEEP_THEM,
        go_ahead: vec![
            APart::words(CLEAR_WORD),
            APart::figure(entries.to_string(), by_count(entries, ENTRY_WORD, ENTRIES_WORD)),
        ],
        grave: true,
        go_ahead_act: TheActOnAJob::ClearThisServersHistory,
        per_server: Vec::new(),
    }
}

/// **Design, *Clearing history*: clearing all of it cannot be undone across
/// every machine at once and a count alone does not show its reach, so that box
/// lists every server it covers with that server's own count.**
///
/// The list travels as the box's own sentences, each a server's name and its
/// figure, so no name is joined into a sentence (X7).
pub fn the_clear_all_box(entries: u32, per_server: &[(String, u32)]) -> TheBoxThatAsks {
    let mut said = vec![vec![
        APart::words(by_count(entries, THE_ONE_WORD, ALL_WORD)),
        APart::figure(
            entries.to_string(),
            by_count(entries, AFTERWARDS_NOTHING_SHOWS_IT, AFTERWARDS_NOTHING_SHOWS_THEM),
        ),
    ]];
    said.push(vec![APart::words(THE_FRAMES_ON_ARE_UNTOUCHED)]);
    TheBoxThatAsks {
        which: "clear-all",
        heading: vec![APart::words(CLEAR_ALL_HISTORY_QUESTION)],
        said,
        keep: KEEP_THEM,
        go_ahead: vec![
            APart::words(CLEAR_WORD),
            APart::words(ALL_LOWER),
            APart::figure(entries.to_string(), by_count(entries, ENTRY_WORD, ENTRIES_WORD)),
        ],
        grave: true,
        go_ahead_act: TheActOnAJob::ClearAllHistory,
        // **The list is the whole of the difference between the two boxes**:
        // one server's history is bounded and countable, and clearing all of it
        // cannot be undone across every machine at once.
        per_server: per_server.to_vec(),
    }
}

/// **Design, *Clearing history*: the fourth act's box follows *Clear all
/// history*'s shape** — each removed server by its name with its own count,
/// then what goes and what does not — **and says nothing will be done on any
/// server**, since those servers are no longer in Smoozy (`Queue & History`
/// X6b reaches no machine). The eighteenth notes, note 13; the drawing's
/// `h-clearremoved`.
pub fn the_clear_removed_box(entries: u32, per_server: &[(String, u32)]) -> TheBoxThatAsks {
    TheBoxThatAsks {
        which: "clear-removed",
        heading: vec![APart::words(CLEAR_THE_HISTORY_OF_REMOVED_SERVERS_QUESTION)],
        said: vec![
            vec![
                APart::words(THEIR_WORD),
                APart::figure(
                    entries.to_string(),
                    by_count(entries, ITS_ENTRY_GOES_WITH_SUBMIT_AGAIN, THEIR_ENTRIES_GO_WITH_SUBMIT_AGAIN),
                ),
            ],
            vec![APart::words(NOTHING_IS_CHANGED_ON_REMOVED_SERVERS)],
        ],
        keep: KEEP_THEM,
        go_ahead: vec![
            APart::words(CLEAR_WORD),
            APart::figure(entries.to_string(), by_count(entries, ENTRY_WORD, ENTRIES_WORD)),
        ],
        grave: true,
        go_ahead_act: TheActOnAJob::ClearRemovedServersHistory,
        per_server: per_server.to_vec(),
    }
}

// ───────────────────────────────────────────────────────────────────────────
// The failure sentence for each ending — Design's words, composed here
// ───────────────────────────────────────────────────────────────────────────

/// **The plain sentence for each of the three endings that carry the shape**,
/// and the disk-full carve-out's second sentence beside it.
///
/// **Nothing else on a failed entry names a cause** (M13b, and Design's own
/// *Nothing else on a failed entry names a cause*), **but one**: where no render
/// began, the sentence says so and the entry puts the machine's reason beneath it
/// (Design, *Where no render began*, the build runner's ruling of 2026-11-04: Rule 8's
/// truth outranks it for this case). A render that began keeps these words.
///
/// **Must-do 9: where a failed render's error stream was empty and no full disk
/// is named, the plain sentence *is* the no-errors one** — the headline Design
/// gives it, and so the sentence the diary keeps. It was once a second
/// paragraph under *The render did not finish*, which the diary kept instead.
pub fn the_failure_for(
    ending: AnEnding,
    no_frame_was_completed: bool,
    the_disk_is_full_on: Option<&str>,
    the_complaints_stream_was_empty: bool,
    no_render_began: bool,
) -> Option<TheEndingsFailure> {
    if !ending.carries_the_failure_shape() {
        return None;
    }
    // **Design, *Where no render began***, ahead of everything below: each of those
    // sentences is about a render, and the disk-full one says *when the render ended*.
    let the_disk_is_full_on = the_disk_is_full_on.filter(|_| !no_render_began);
    let said = match (ending, no_frame_was_completed) {
        (AnEnding::Failed, _) if no_render_began => FAILED_BEFORE_RENDERING.to_string(),
        (AnEnding::Failed, _) if the_complaints_stream_was_empty && the_disk_is_full_on.is_none() => {
            THE_RENDER_FAILED_AND_THE_OUTPUT_MAY_SAY_WHY.to_string()
        }
        (AnEnding::Failed, true) => NO_FRAME_WAS_COMPLETED.to_string(),
        // **The second half is dropped where a cause is named**, because the
        // carve-out's own sentence says what Smoozy knows and the two together
        // would say both.
        (AnEnding::Failed, false) if the_disk_is_full_on.is_some() => {
            THE_RENDER_DID_NOT_FINISH.to_string()
        }
        (AnEnding::Failed, false) => {
            format!("{THE_RENDER_DID_NOT_FINISH} {SMOOZY_DOES_NOT_KNOW_WHY}")
        }
        (AnEnding::Stopped, _) => {
            format!("{YOU_STOPPED_THIS_RENDER} {THE_FRAME_IT_WAS_ON_MAY_NOT_BE_SAVED}")
        }
        (AnEnding::GivenUp, _) => INTERRUPTED_TWICE_AND_GIVEN_UP.to_string(),
        _ => return None,
    };
    Some(TheEndingsFailure {
        said,
        // **The server's name is a value of its own and never inside the
        // words** (X7), so the second sentence names no server and the name
        // travels beside it.
        what_to_do_about_it: the_disk_is_full_on.map(|_| THE_DIAGNOSIS.to_string()),
        values: the_disk_is_full_on
            .map(|server| vec![(THE_SERVER_LABEL, server.to_string())])
            .unwrap_or_default(),
        // **The record's own, and [`the_entry_for`] puts it on** — this
        // composes the sentence from the ending alone.
        what_the_server_said: None,
        the_reason_beneath: None,
        blender_output: true,
        // **Design draws the report control on every failed ending but the one
        // the user caused**: a stop is not a fault and there is nothing to
        // report about it.
        report_a_problem: ending != AnEnding::Stopped,
    })
}

/// **The machine's reason, to stand as a line of its own**: the record keeps it as a
/// clause, begun in lower case. Only its first letter changes, and nothing else.
fn the_reason_as_a_sentence(reason: &str) -> String {
    let mut letters = reason.chars();
    match letters.next() {
        Some(first) => first.to_uppercase().chain(letters).collect(),
        None => String::new(),
    }
}

/// **Where the disk-full carve-out fires on this entry, the server's name to
/// draw beside it** — `Server protocol and transfers` F6's route (b): *"a
/// render ended failing, and a free-space reading taken against that job's own
/// output path reports nothing available"*.
///
/// **The reading is the one the ending carried and is never taken here**
/// (`Desktop app and delivery` M15a): History opens no connection. It is the
/// record's, or, beside an ending owed to a full store, the queue reading's
/// (`Rented-server runtime` S14a) — the same first attempt's reading either
/// way, never one taken on a freed mount. **No reading
/// is not a reading of nought** — the carve-out's own
/// [`what_to_do_about_it`] answers nothing for it, and nothing infers a full
/// disk from the absence.
fn the_disk_was_full_on<'r>(
    record: &'r LocalJobRecord,
    ending: AnEnding,
    the_reading_the_ending_carried: Option<&smoozy_queue_history::acting::note::TheFreeSpaceReadAtTheEnding>,
) -> Option<&'r str> {
    if ending != AnEnding::Failed {
        return None;
    }
    let read = the_reading_the_ending_carried?;
    let smoozy_state::observed::Clock::AMachine(clock) = read.taken_at.clock() else {
        return None;
    };
    let carried = TheCarriedReading::of(
        read.available_bytes,
        read.taken_at.taken_at(),
        clock.clone(),
        &read.the_folder_it_was_taken_against,
    );
    what_to_do_about_it(Some(&carried))?;
    Some(record.server.label.as_deref().unwrap_or_default())
}

/// **The two ways to render it again, for one ending** — Design, *So ended
/// short is not on its own enough to earn the pair* — **reckoned as the
/// architecture reckons them** (must-do 2 of step 10a round 1 of the queue,
/// history, settings and downloads).
///
/// - `asked` is **the frames the job asked for, in its own asked order**
///   (`Queue & History` N6). Nothing here assumes the first is frame 1 or that
///   the step is 1 (`Specs/Render Settings`).
/// - `boundary` is `Queue & History` N6's figure: **the last frame that counts
///   as already there**. `Rendering` A4 and `Rendering` A9 open the carried-on range
///   **at** it, because it is the one Smoozy cannot confirm finished. **Its absence**,
///   and never a frame number of nought, is *no frame finished* — nought can be
///   a real frame.
/// - `the_server_stands` is `Queue & History` N17's draw-time lookup of the
///   entry's copied server identifier against the standing server rows: where
///   no row stands, carrying on is not offered.
pub fn the_ways_to_render_it_again(
    ending: AnEnding,
    asked: &[i64],
    boundary: Option<i64>,
    came_from: WhereTheFigureCameFrom,
    the_server_stands: bool,
) -> RenderItAgain {
    let submit_again = asked.len();
    let without = |why: &'static str| RenderItAgain {
        submit_again,
        carry_on: None,
        why: Some(why),
        the_count_came_up_short: None,
    };
    match ending {
        // A clean finish: nothing left to carry on from.
        AnEnding::Finished => return without(EVERY_FRAME_IS_RENDERED),
        // **No ending Smoozy ever learned: there is no frame figure to start
        // from**, and the action is not offered rather than offered and unable
        // to work.
        AnEnding::NotKnown | AnEnding::StartNotKnown | AnEnding::InProgress => {
            return without(NO_FRAME_TO_START_FROM)
        }
        _ => {}
    }
    if !the_server_stands {
        return without(THE_SERVER_IS_GONE_SO_NO_CARRY_ON);
    }
    // **Ended short having finished no frame: there is no frame to start from,
    // and carrying on would be the whole range** — Label: **Creator-stated**
    // (2026-08-13).
    let Some(boundary) = boundary else {
        return without(NO_FRAME_FINISHED_SO_NOTHING_TO_START_FROM);
    };
    // **A figure that is not one of the frames the job asked for** is not a
    // figure this entry can start from, and Smoozy does not know how far the job
    // got.
    let Some(at) = asked.iter().position(|frame| *frame == boundary) else {
        return without(NO_FRAME_TO_START_FROM);
    };
    RenderItAgain {
        submit_again,
        carry_on: Some(TheCarryOn {
            frames: asked.len() - at,
            at: boundary,
            came_from,
        }),
        why: None,
        the_count_came_up_short: None,
    }
}

/// **Whether a server row stands for an entry's copied identifier** — `Queue &
/// History` N17, looked up at the draw and cached never. **A dangling
/// identifier reads as nothing** (`Queue & History` N12), and so does a read that fails: nothing
/// is offered that could not work.
pub fn a_server_row_stands(record: Option<&crate::record::LocalRecord>, server_row: &str) -> bool {
    the_server_rows_that_stand(record).contains(server_row)
}

/// **Every server row that stands, by its local identifier**, read once for a
/// whole list. **A read that fails stands none**, which is what
/// [`a_server_row_stands`] has always answered.
fn the_server_rows_that_stand(
    record: Option<&crate::record::LocalRecord>,
) -> std::collections::BTreeSet<String> {
    record
        .and_then(|record| crate::record::server_rows::all(record.connection()).ok())
        .map(|rows| rows.into_iter().map(|row| row.local_server_identifier).collect())
        .unwrap_or_default()
}

/// The *Rendered* group's own fields — Design, *History*: it keeps its times,
/// and where it can be had, which parts of the server did the work.
///
/// **`the_last_frame_is_unconfirmed` names the last frame itself as the one not
/// confirmed** — the boundary frame of `Rendering` A4, which the entry never
/// reckons from by adding one (must-do 2).
/// **Eight arguments and they are the drawing's own six rows.** A struct here
/// would be the same list under a name, and the name would be *the Rendered
/// group*, which is what this function already is.
#[allow(clippy::too_many_arguments)]
pub fn the_rendered_group(
    last_frame: Option<i64>,
    the_last_frame_is_unconfirmed: bool,
    when_there_is_no_figure: &'static str,
    submitted: Option<&str>,
    started: Option<&str>,
    ended: Option<&str>,
    rendered_for: Option<&str>,
    ran_on: Option<&str>,
    it_finished: bool,
) -> Vec<ARenderedFact> {
    let a_line = |name: &'static str, parts: Vec<APart>| ARenderedFact {
        name,
        parts,
        a_moment: false,
    };
    let a_value = |value: Option<&str>| match value {
        Some(value) => vec![APart::value(value.to_string())],
        // An absent duration is shown as unknown, never as zero.
        None => vec![APart::words(NOT_KNOWN)],
    };
    // **A moment goes out as seconds and the page writes the date**, which is
    // what every other screen in this application already does.
    let a_time = |name: &'static str, value: Option<&str>| ARenderedFact {
        name,
        parts: a_value(value),
        a_moment: value.is_some(),
    };
    let mut facts = vec![
        a_line(
            LAST_FRAME_FACT,
            match last_frame {
                Some(frame) if the_last_frame_is_unconfirmed => vec![
                    APart::value(frame.to_string()),
                    APart::said(THE_FRAME_LEAD, frame.to_string(), NOT_CONFIRMED_TRAIL),
                ],
                Some(frame) => vec![APart::value(frame.to_string())],
                // **An absent figure is two different facts and the drawing
                // words them differently**, so the caller says which: a job
                // that ended short having finished no frame, and a job whose
                // figure the record simply does not hold.
                None => vec![APart::words(when_there_is_no_figure)],
            },
        ),
        a_time(SUBMITTED_FACT, submitted),
        a_time(STARTED_FACT, started),
        a_time(ENDED_FACT, ended),
        a_line(RENDERED_FOR_FACT, a_value(rendered_for)),
    ];
    if ran_on.is_some() || !it_finished {
        facts.push(a_line(RAN_ON_FACT, a_value(ran_on)));
    }
    facts
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **The 38's piece 10, item 24: a job stopped with frames 1–60 of 1–240
    /// on the server draws *60/240* on its History row** — the note's figure is
    /// the last frame there, 60, and the count runs up to and including it.
    /// **A job whose asked order no read has landed draws the dash**, and so
    /// does one whose frames cannot be counted (`Job files and outputs` I9a).
    ///
    /// **The live pass on Vast, 2026-10-19, fault 5**: a job stopped with
    /// frames 1–52 of 150 on the server read *51/150* in History while the
    /// server and Telegram said 52. It reads *52/150* now, and *Resume*
    /// still opens at 52 and renders it again (`Rendering` A4): 99 frames.
    #[test]
    fn piece_10_a_job_stopped_with_60_of_1_to_240_on_the_server_draws_60_of_240_on_its_row() {
        use smoozy_queue_history::acting::note::{TheEndingOnTheNote, TheOutcomeAsWritten};
        use smoozy_queue_history::record::{TheFramesAsked, TheFramesTheReadFound};
        let server = Identity::issue(smoozy_state::identities::IdentityKind::LocalServer).unwrap();
        let mut record = crate::record::deletions::tests::a_record_in_history(&server, "Kitchen");
        record.what_the_note_carried.ending = Some(TheEndingOnTheNote {
            outcome: TheOutcomeAsWritten::LeftStopped,
            exit_status: None,
            the_attempt_whose_capture_record_holds_the_words: 1,
            the_reason_the_machine_recorded: None,
            no_render_began: false,
            the_device_line_as_observed: None,
        });
        record.what_the_note_carried.last_frame_figure = Some(60);
        assert_eq!(a_history_row(&record, false, true).frames, None, "no asked order: the dash");
        record.the_frames = TheFramesTheReadFound {
            asked: TheFramesAsked::from_the_order(&(1..=240).collect::<Vec<i64>>()),
            they_cannot_be_counted: false,
        };
        assert_eq!(
            a_history_row(&record, false, true).frames,
            Some(("60".to_string(), "240".to_string()))
        );
        record.the_frames.they_cannot_be_counted = true;
        assert_eq!(a_history_row(&record, false, true).frames, None);

        // **The live pass's job: frames 1–52 of 150 on the server.**
        let order: Vec<i64> = (1..=150).collect();
        record.what_the_note_carried.last_frame_figure = Some(52);
        record.the_frames = TheFramesTheReadFound {
            asked: TheFramesAsked::from_the_order(&order),
            they_cannot_be_counted: false,
        };
        assert_eq!(
            a_history_row(&record, false, true).frames,
            Some(("52".to_string(), "150".to_string())),
            "History says what the server and Telegram say"
        );
        // ***Resume* is unchanged: it opens at the figure and renders it
        // again**, whatever the row's count says.
        let again = the_ways_to_render_it_again(
            AnEnding::Stopped,
            &order,
            record.what_the_note_carried.last_frame_figure,
            WhereTheFigureCameFrom::RecordedWhenTheJobEnded,
            true,
        );
        let carry_on = again.carry_on.expect("a stopped job offers Resume");
        assert_eq!(carry_on.at, 52, "Resume opens at the boundary frame");
        assert_eq!(carry_on.frames, 99, "frames 52–150");
    }

    fn said(event: &Event) -> String {
        format!("{event:?}")
    }

    fn a_row(job: &str, server_row: &str, server: &str) -> AHistoryRowDrawn {
        AHistoryRowDrawn {
            job: job.to_string(),
            server_row: server_row.to_string(),
            name: format!("Job {job}"),
            server: server.to_string(),
            when: Some("1754688840".to_string()),
            ..AHistoryRowDrawn::default()
        }
    }

    /// **A record that could not be opened is said, and draws no history and
    /// invents none** — must-do 3(a) of step 10b round 1 of *jobs, history and
    /// downloads*, and L8a: a screen never draws an absence in place of a read
    /// that did not answer.
    ///
    /// **This failed before the fix**: no record answered exactly what a person
    /// who never rendered sees — no rows, *0 jobs*, *kept until you clear them*.
    #[test]
    fn no_record_is_said_and_draws_no_rows_and_nothing_chosen() {
        let unread = what_is_kept(None, None);
        assert!(unread.could_not_read, "a record that would not open was drawn as an empty History");
        assert!(unread.rows.is_empty());
        assert!(what_is_kept(None, Some("job-1")).chosen.is_none());
        assert_ne!(unread, TheHistoryOnScreen::default());

        // **Said in place of the list**, and nothing of the list is sent.
        let text = said(&the_history_says(&unread));
        assert!(text.contains("couldNotRead"), "{text}");
        assert!(!text.contains("theJobsFigure"), "{text}");
        assert!(!text.contains("clearAllHistory"), "{text}");
        // **And a History that simply holds nothing says nothing of the kind.**
        let empty = said(&the_history_says(&TheHistoryOnScreen::default()));
        assert!(!empty.contains("couldNotRead"), "{empty}");
        assert!(include_str!("../../web/the-history.js").contains("said.couldNotRead"));
    }

    /// **A read that fails on an open record is said too**, and so are rows
    /// that would not read back — must-do 3(a) and 3(c). The table is dropped
    /// under the record to make the read fail; a row's disturbance word is
    /// spoiled to make one row unreadable.
    #[test]
    fn a_failed_read_is_said_and_an_unreadable_row_is_counted() {
        use crate::record::deletions::tests::a_record_in_history;
        use smoozy_state::identities::IdentityKind;

        let mut path = std::env::temp_dir();
        path.push(format!("smoozy-history-unread-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).unwrap();
        let record = LocalRecord::open_at(&path.join("local-record.sqlite3")).expect("a record");
        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        let store = JobRecords::over(record.connection());
        let mut spoiled = None;
        for label in ["Kitchen", "Bedroom"] {
            let mut made = a_record_in_history(&server, label);
            made.disturbances = vec![smoozy_queue_history::record::ADisturbance {
                the_member: "/a/tex.png".to_string(),
                the_act_intended: "replace".to_string(),
                reading: smoozy_queue_history::record::DisturbanceReading::Disturbed,
                how_many: None,
            }];
            store.write_the_provisional_record(&made).expect("written");
            spoiled = Some(made.job.as_text());
        }
        record
            .connection()
            .execute(
                "UPDATE job_record_disturbance SET reading = 'smudged' WHERE job = ?1",
                [spoiled.unwrap()],
            )
            .unwrap();

        let kept = what_is_kept(Some(&record), None);
        assert!(!kept.could_not_read);
        assert_eq!(kept.jobs, 1, "a record with a mark that would not read back was drawn");
        assert_eq!(kept.could_not_be_read, 1, "the unreadable row was dropped without a word");
        let text = said(&the_history_says(&kept));
        assert!(text.contains("unreadFigure"), "{text}");
        assert!(text.contains("unreadWord"), "{text}");

        record.connection().execute_batch("DROP TABLE job_record_disturbance").unwrap();
        assert!(what_is_kept(Some(&record), None).could_not_read);

        drop(record);
        let _ = std::fs::remove_dir_all(&path);
    }

    /// **Item C1: a job whose server was removed before Smoozy heard whether
    /// it started is drawn, marked so** — end to end over a real record,
    /// through the removal the Servers screen performs.
    ///
    /// **This failed before the change**: the removal deleted the record and
    /// History drew nothing at all.
    #[test]
    fn a_job_whose_start_was_never_heard_is_drawn_after_its_server_is_removed() {
        use crate::record::server_rows;
        use smoozy_queue_history::record::Standing;

        let mut path = std::env::temp_dir();
        path.push(format!("smoozy-history-never-heard-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).unwrap();
        let record = LocalRecord::open_at(&path.join("local-record.sqlite3")).expect("a record");

        let row = smoozy_servers::store::ServerRow::saved(
            Identity::issue(smoozy_state::identities::IdentityKind::LocalServer)
                .unwrap()
                .as_text(),
            smoozy_servers::identity::Host::what_the_user_typed("5.180.183.77").unwrap(),
            smoozy_servers::identity::Port::what_the_user_typed(33510).unwrap(),
            smoozy_servers::identity::Username::what_the_user_typed("root").unwrap(),
            smoozy_servers::identity::KeyReference {
                local_key_identifier: "key-1".into(),
                fingerprint: "SHA256:aaa".into(),
            },
        );
        server_rows::add(record.connection(), &row).expect("a server row");
        let server = Identity::read_back(
            smoozy_state::identities::IdentityKind::LocalServer,
            &row.local_server_identifier,
        )
        .unwrap();
        let mut made = crate::record::deletions::tests::a_record_in_history(&server, "Kitchen");
        made.the_render_start_has_been_learned = false;
        made.standing = Standing::Provisional;
        made.what_the_note_carried = Default::default();
        JobRecords::over(record.connection())
            .write_the_provisional_record(&made)
            .expect("written");
        assert_eq!(what_is_kept(Some(&record), None).jobs, 0, "a job not yet known to have started is not in History");

        server_rows::remove_walking_the_enumeration(record.connection(), &row.local_server_identifier)
            .expect("removed");

        let kept = what_is_kept(Some(&record), None);
        assert_eq!(kept.jobs, 1);
        assert_eq!(kept.rows[0].ending, Some(AnEnding::StartNotKnown));
        let entry = kept.entry.as_ref().expect("the entry is composed");
        assert_eq!(entry.ending, Some(AnEnding::StartNotKnown));
        assert_eq!(entry.rendered_note, Some(SMOOZY_NEVER_HEARD_WHETHER_IT_STARTED));
        assert!(entry.failure.is_none(), "an unknown start is not a failure");
        let text = said(&the_history_says(&kept));
        assert!(text.contains(&format!("{:?}", Prose::new(START_NOT_KNOWN))), "{text}");
        assert!(text.contains("e-unk"), "{text}");
        // **It is deleted like any other entry.**
        let started = a_started_over(record);
        assert_eq!(
            the_act_is_performed(
                &started,
                crate::surface::the_queue::TheActOnAJob::DeleteTheEntry,
                Some(&made.job.as_text())
            ),
            Some(Ok(()))
        );
        assert_eq!(what_is_kept(started.record.as_ref(), None).jobs, 0);

        drop(started);
        let _ = std::fs::remove_dir_all(&path);
    }

    /// **The Creator's second notes, piece 7 (note 11): a rendering job is in
    /// Queue and not in History, and it is in History once it has ended** —
    /// `Specs/Queue & History` *History* and `Queue & History` C9 as narrowed
    /// 2026-10-04. Its record is kept all the while, and nothing in History
    /// can reach it: no row, no count, no box, no act. A job on a removed
    /// server still shows *Ending unknown*. End to end over a real record
    /// and the event the page draws from.
    ///
    /// **This failed before the fix**: the running job drew a row reading *In
    /// progress* and the count was 2. *(It replaces step 10c's twelfth-pass
    /// fix piece 5's test, which held that row.)*
    #[test]
    fn piece_7_a_rendering_job_is_in_queue_and_not_in_history_until_it_has_ended() {
        use crate::record::server_rows;
        use crate::surface::the_queue::TheActOnAJob;
        use smoozy_queue_history::acting::note::{TheEndingOnTheNote, TheNote, TheOutcomeAsWritten};
        use smoozy_state::observed::{Clock, MachineClock, Reading};
        use std::time::{Duration, SystemTime};
        use smoozy_queue_history::line::QueueState;
        use smoozy_state::tombstones::HistoryTombstones;

        let mut path = std::env::temp_dir();
        path.push(format!("smoozy-history-piece-7-running-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).unwrap();
        let record = LocalRecord::open_at(&path.join("local-record.sqlite3")).expect("a record");

        let a_row_for = |host: &str| {
            let row = smoozy_servers::store::ServerRow::saved(
                Identity::issue(smoozy_state::identities::IdentityKind::LocalServer)
                    .unwrap()
                    .as_text(),
                smoozy_servers::identity::Host::what_the_user_typed(host).unwrap(),
                smoozy_servers::identity::Port::what_the_user_typed(22).unwrap(),
                smoozy_servers::identity::Username::what_the_user_typed("ubuntu").unwrap(),
                smoozy_servers::identity::KeyReference {
                    local_key_identifier: "key-1".into(),
                    fingerprint: "SHA256:aaa".into(),
                },
            );
            server_rows::add(record.connection(), &row).expect("a server row");
            let server = Identity::read_back(
                smoozy_state::identities::IdentityKind::LocalServer,
                &row.local_server_identifier,
            )
            .unwrap();
            (row, server)
        };
        let (row, server) = a_row_for("5.180.183.77");
        let (_, other) = a_row_for("5.180.183.78");

        let store = JobRecords::over(record.connection());
        let started_at = SystemTime::UNIX_EPOCH + Duration::from_secs(1_754_600_000);
        let the_note = |on: &Identity, label: &str, ended: bool| {
            let machine = Clock::AMachine(MachineClock(on.as_text()));
            let mut note = TheNote::created_with_the_job_record(label);
            note.started = Some(Reading::taken((), started_at, machine.clone()));
            if ended {
                note.ended = Some(Reading::taken((), started_at + Duration::from_secs(600), machine));
                note.ending = Some(TheEndingOnTheNote {
                    outcome: TheOutcomeAsWritten::Finished,
                    exit_status: Some(0),
                    the_attempt_whose_capture_record_holds_the_words: 1,
                    the_reason_the_machine_recorded: None,
                    no_render_began: false,
                    the_device_line_as_observed: None,
                });
            }
            note
        };
        let a_job = |on: &Identity, server_label: &str, label: &str, ended: bool| {
            let mut made = crate::record::deletions::tests::a_record_in_history(on, label);
            made.server.label = Some(server_label.to_string());
            store.write_the_provisional_record(&made).expect("written");
            store.the_fill_in(&made.job, &the_note(on, label, ended)).expect("filled in");
            made
        };

        // **One job rendering on a standing server, one ended on another**,
        // and a third rendering on the server that will be removed.
        let running = a_job(&other, "Vast", "p12 long1", false);
        let ended = a_job(&other, "Vast", "p12 done", true);
        let stranded = a_job(&server, "Hyperstack", "p12 long2", false);
        let job = running.job.as_text();
        let in_progress = format!("{:?}", Prose::new("In progress"));
        let ending_not_known = format!("{:?}", Prose::new("Ending unknown"));
        let the_delete_control = format!("\"delete\": Prose({:?})", Prose::new("Delete"));

        // **In Queue**: by the queue's own rule the running job has not left.
        let held = store.read(&running.job).expect("read").expect("its record is kept");
        let rendering = QueueState::Rendering {
            operation: Identity::issue(smoozy_state::identities::IdentityKind::Operation).unwrap(),
        };
        assert!(!crate::surface::the_queue::it_has_left_the_queue(&rendering, Some(&held)));

        // **Not in History**: no row, not counted, never chosen, and the
        // event says nothing of it.
        let kept = what_is_kept(Some(&record), Some(&job));
        let rows: Vec<&str> = kept.rows.iter().map(|row| row.job.as_str()).collect();
        assert!(!rows.contains(&job.as_str()), "a rendering job is in History: {rows:?}");
        // Only the ended job: the other running job is not drawn either.
        assert_eq!(kept.jobs, 1, "the count is not the list's own");
        assert_ne!(kept.chosen.as_deref(), Some(job.as_str()));
        let text = said(&the_history_says(&kept));
        assert!(!text.contains(&in_progress), "{text}");
        assert!(!text.contains("p12 long1"), "{text}");
        // **Nothing in History reaches its record**: a delete asked over its
        // identity opens no box and deletes nothing.
        assert_eq!(the_history_as_asked(kept, Some(&job), Some(TheHistoryAsk::Delete)).asking, None);

        // **A job on a removed server still shows *Ending unknown*, with
        // *Delete*.**
        server_rows::remove_walking_the_enumeration(record.connection(), &row.local_server_identifier)
            .expect("removed");
        let gone = what_is_kept(Some(&record), Some(&stranded.job.as_text()));
        assert_eq!(gone.jobs, 2);
        let text = said(&the_history_says(&gone));
        assert!(text.contains(&ending_not_known), "{text}");
        assert!(text.contains(&the_delete_control), "{text}");
        assert_eq!(gone.entry.as_ref().and_then(|entry| entry.rendered_note), Some(THE_SERVER_WAS_DESTROYED));

        // **The ended job keeps its ending and its *Delete*.**
        let finished = what_is_kept(Some(&record), Some(&ended.job.as_text()));
        let text = said(&the_history_says(&finished));
        assert!(text.contains(&the_delete_control), "{text}");
        assert!(the_history_as_asked(finished, Some(&ended.job.as_text()), Some(TheHistoryAsk::Delete))
            .asking
            .is_some());

        let started = a_started_over(record);
        let tombstoned = |job: &Identity| {
            JobRecords::over(started.record.as_ref().unwrap().connection())
                .tombstones()
                .unwrap()
                .tombstone_for(job)
                .is_some()
        };
        assert_eq!(the_act_is_performed(&started, TheActOnAJob::DeleteTheEntry, Some(&job)), Some(Err(())));
        assert!(!tombstoned(&running.job), "a delete reached a running job's record");

        // **Once it has ended it is in History**, counted, with its ending and
        // its *Delete*, and by the queue's rule it has left the queue.
        let records = JobRecords::over(started.record.as_ref().unwrap().connection());
        records.the_fill_in(&running.job, &the_note(&other, "p12 long1", true)).expect("filled in");
        let held = records.read(&running.job).expect("read").expect("kept");
        assert!(crate::surface::the_queue::it_has_left_the_queue(&rendering, Some(&held)));
        let now = what_is_kept(started.record.as_ref(), Some(&job));
        assert_eq!(now.jobs, 3);
        assert_eq!(now.chosen.as_deref(), Some(job.as_str()));
        let text = said(&the_history_says(&now));
        assert!(text.contains(&format!("{:?}", Prose::new("Finished"))), "{text}");
        assert!(text.contains(&the_delete_control), "{text}");

        drop(started);
        let _ = std::fs::remove_dir_all(&path);
    }

    /// **Both clears take every entry the list draws and never a running
    /// job's record** — `Queue & History` X6 and `Queue & History` X6a, with the Creator's second
    /// notes, piece 7. The boxes count what goes and say nothing of what is
    /// kept (`Design/Queue & History § Clearing history`, the kept line struck
    /// 2026-10-04). End to end over a real record.
    ///
    /// **Bit by** the list filter taken out of the act: the clear then
    /// tombstones the running job.
    #[test]
    fn piece_7_clearing_history_takes_every_entry_drawn_and_never_a_running_job() {
        use crate::record::server_rows;
        use crate::surface::the_queue::TheActOnAJob;
        use smoozy_queue_history::acting::note::{TheEndingOnTheNote, TheNote, TheOutcomeAsWritten};
        use smoozy_state::observed::{Clock, MachineClock, Reading};
        use std::time::{Duration, SystemTime};
        use smoozy_state::tombstones::HistoryTombstones;

        let mut path = std::env::temp_dir();
        path.push(format!("smoozy-history-piece-7-clear-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).unwrap();
        let record = LocalRecord::open_at(&path.join("local-record.sqlite3")).expect("a record");

        let a_row_for = |host: &str| {
            let row = smoozy_servers::store::ServerRow::saved(
                Identity::issue(smoozy_state::identities::IdentityKind::LocalServer)
                    .unwrap()
                    .as_text(),
                smoozy_servers::identity::Host::what_the_user_typed(host).unwrap(),
                smoozy_servers::identity::Port::what_the_user_typed(22).unwrap(),
                smoozy_servers::identity::Username::what_the_user_typed("ubuntu").unwrap(),
                smoozy_servers::identity::KeyReference {
                    local_key_identifier: "key-1".into(),
                    fingerprint: "SHA256:aaa".into(),
                },
            );
            server_rows::add(record.connection(), &row).expect("a server row");
            let server = Identity::read_back(
                smoozy_state::identities::IdentityKind::LocalServer,
                &row.local_server_identifier,
            )
            .unwrap();
            (row, server)
        };
        let (row, server) = a_row_for("5.180.183.77");
        let (_, other) = a_row_for("5.180.183.78");

        let store = JobRecords::over(record.connection());
        let started_at = SystemTime::UNIX_EPOCH + Duration::from_secs(1_754_600_000);
        let the_note = |on: &Identity, label: &str, ended: bool| {
            let machine = Clock::AMachine(MachineClock(on.as_text()));
            let mut note = TheNote::created_with_the_job_record(label);
            note.started = Some(Reading::taken((), started_at, machine.clone()));
            if ended {
                note.ended = Some(Reading::taken((), started_at + Duration::from_secs(600), machine));
                note.ending = Some(TheEndingOnTheNote {
                    outcome: TheOutcomeAsWritten::Finished,
                    exit_status: Some(0),
                    the_attempt_whose_capture_record_holds_the_words: 1,
                    the_reason_the_machine_recorded: None,
                    no_render_began: false,
                    the_device_line_as_observed: None,
                });
            }
            note
        };
        let a_job = |on: &Identity, server_label: &str, label: &str, ended: bool| {
            let mut made = crate::record::deletions::tests::a_record_in_history(on, label);
            made.server.label = Some(server_label.to_string());
            store.write_the_provisional_record(&made).expect("written");
            store.the_fill_in(&made.job, &the_note(on, label, ended)).expect("filled in");
            made
        };

        let running = a_job(&server, "Hyperstack", "p12 long1", false);
        let done = a_job(&server, "Hyperstack", "p12 done", true);
        let elsewhere = a_job(&other, "Vast", "p12 vast", true);
        assert_eq!(what_is_kept(Some(&record), None).jobs, 2);

        let from_done = what_is_kept(Some(&record), Some(&done.job.as_text()));
        let this = the_history_as_asked(
            from_done.clone(),
            Some(&done.job.as_text()),
            Some(TheHistoryAsk::ClearThisServer),
        );
        assert_eq!(this.asking, Some(the_clear_box("Hyperstack", 1)));
        let words = every_word(this.asking.as_ref().unwrap());
        assert!(!words.contains("in progress"), "{words}");
        let all = the_history_as_asked(from_done.clone(), Some(&done.job.as_text()), Some(TheHistoryAsk::ClearAll));
        assert_eq!(
            all.asking,
            Some(the_clear_all_box(2, &[("Hyperstack".to_string(), 1), ("Vast".to_string(), 1)]))
        );

        let mut started = a_started_over(record);
        let tombstoned = |started: &Started, job: &Identity| {
            JobRecords::over(started.record.as_ref().unwrap().connection())
                .tombstones()
                .unwrap()
                .tombstone_for(job)
                .is_some()
        };
        assert_eq!(
            the_act_is_performed(&started, TheActOnAJob::ClearThisServersHistory, Some(&done.job.as_text())),
            Some(Ok(()))
        );
        assert!(tombstoned(&started, &done.job));
        assert!(!tombstoned(&started, &running.job), "a clear reached a running job's record");
        assert_eq!(
            the_act_is_performed(&started, TheActOnAJob::ClearAllHistory, Some(&elsewhere.job.as_text())),
            Some(Ok(()))
        );
        assert!(tombstoned(&started, &elsewhere.job));
        assert!(!tombstoned(&started, &running.job), "a clear reached a running job's record");
        let kept = what_is_kept(started.record.as_ref(), None);
        assert_eq!(kept.jobs, 0);
        let text = said(&the_history_says(&kept));
        assert!(!text.contains("clearAllHistory"), "{text}");

        // **Its server removed, the running job shows *Ending unknown* and
        // is cleared as any other.**
        server_rows::remove_walking_the_enumeration(
            started.record.as_ref().unwrap().connection(),
            &row.local_server_identifier,
        )
        .expect("removed");
        assert_eq!(what_is_kept(started.record.as_ref(), None).jobs, 1);
        assert_eq!(
            the_act_is_performed(&started, TheActOnAJob::ClearAllHistory, Some(&running.job.as_text())),
            Some(Ok(()))
        );
        assert_eq!(what_is_kept(started.record.as_ref(), None).jobs, 0);
        assert!(tombstoned(&started, &running.job));

        started.record = None;
        let _ = std::fs::remove_dir_all(&path);
    }

    /// **Step 12's SC4 follow-up: between the fill and the landing, the job is
    /// in History, drawn from the reading** — `Queue & History` C11 (*the
    /// history entry takes the ending Smoozy learns — from watching*),
    /// `Rented-server runtime` S14 (*the entry … moves off rendering*) and
    /// `Rented-server runtime` S14a (*carrying it forward is what lets the disk-full diagnosis reach
    /// the user inside the window*). End to end over a real record:
    ///
    /// 1. **With no reading the job is in no list**, as before: the record has
    ///    it running, and the Queue let it go on the reading's *ended*.
    /// 2. **With the reading's ending owed** the row reads *Failed*, and the
    ///    entry says *The render did not finish.* with the disk-full sentence,
    ///    the server beside it. **Nothing is said from the figure the landing
    ///    has yet to bring**: no *no frame was completed*, no *none finished*,
    ///    no empty picture.
    /// 3. **Neither *Delete* nor a clearing reaches it**, and a clearing's
    ///    count leaves it out — those are acts on the record, which has the
    ///    job still running.
    /// 4. **A reading of another server's job, or one with room, says
    ///    nothing it does not know.**
    /// 5. **The landing replaces it with the record.**
    ///
    /// **What it bites on**: the reading not read (the state before this
    /// piece — step 1's answer for the whole window), a figure's absence read
    /// as *none finished*, and *Delete* offered over a record still running.
    #[test]
    fn an_ending_owed_to_a_full_store_is_in_history_from_the_reading_until_the_landing() {
        use crate::record::server_rows;
        use smoozy_queue_history::acting::note::{
            TheEndingOnTheNote, TheEndingOwed, TheFreeSpaceAtTheEnding, TheFreeSpaceReadAtTheEnding, TheNote,
            TheOutcomeAsWritten,
        };
        use smoozy_state::observed::{Clock, MachineClock, Reading};
        use std::time::{Duration, SystemTime};

        let mut path = std::env::temp_dir();
        path.push(format!("smoozy-history-an-ending-owed-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).unwrap();
        let record = LocalRecord::open_at(&path.join("local-record.sqlite3")).expect("a record");
        let row = smoozy_servers::store::ServerRow::saved(
            Identity::issue(smoozy_state::identities::IdentityKind::LocalServer).unwrap().as_text(),
            smoozy_servers::identity::Host::what_the_user_typed("5.180.183.77").unwrap(),
            smoozy_servers::identity::Port::what_the_user_typed(22).unwrap(),
            smoozy_servers::identity::Username::what_the_user_typed("ubuntu").unwrap(),
            smoozy_servers::identity::KeyReference {
                local_key_identifier: "key-1".into(),
                fingerprint: "SHA256:aaa".into(),
            },
        );
        server_rows::add(record.connection(), &row).expect("a server row");
        let server =
            Identity::read_back(smoozy_state::identities::IdentityKind::LocalServer, &row.local_server_identifier).unwrap();

        let store = JobRecords::over(record.connection());
        let started_at = SystemTime::UNIX_EPOCH + Duration::from_secs(1_754_600_000);
        let machine = Clock::AMachine(MachineClock(server.as_text()));
        let the_note = |label: &str, ended: bool| {
            let mut note = TheNote::created_with_the_job_record(label);
            note.started = Some(Reading::taken((), started_at, machine.clone()));
            if ended {
                note.ended = Some(Reading::taken((), started_at + Duration::from_secs(600), machine.clone()));
                note.ending = Some(TheEndingOnTheNote {
                    outcome: TheOutcomeAsWritten::Failed,
                    exit_status: Some(1),
                    the_attempt_whose_capture_record_holds_the_words: 1,
                    the_reason_the_machine_recorded: None,
                    no_render_began: false,
                    the_device_line_as_observed: None,
                });
                note.last_frame_figure = Some(40);
            }
            note
        };
        let mut shot = crate::record::deletions::tests::a_record_in_history(&server, "Shot 40");
        shot.server.label = Some("RunPod 4090".to_string());
        store.write_the_provisional_record(&shot).expect("written");
        store.the_fill_in(&shot.job, &the_note("Shot 40", false)).expect("filled in");
        let mut done = crate::record::deletions::tests::a_record_in_history(&server, "Earlier");
        done.server.label = Some("RunPod 4090".to_string());
        store.write_the_provisional_record(&done).expect("written");
        store.the_fill_in(&done.job, &the_note("Earlier", true)).expect("filled in");

        let a_reading_of = |available_bytes: u64| TheEndingOwed {
            outcome: TheOutcomeAsWritten::Failed,
            the_free_space_at_the_ending: Some(TheFreeSpaceAtTheEnding::It(TheFreeSpaceReadAtTheEnding {
                available_bytes,
                the_folder_it_was_taken_against: "/root/jobs/shot-40".to_string(),
                taken_at: Reading::taken((), started_at + Duration::from_secs(300), machine.clone()),
            })),
        };
        let owed = vec![AnEndingTheReadingCarries {
            server: server.as_text(),
            job: shot.job.clone(),
            owed: a_reading_of(0),
        }];
        let chosen = shot.job.as_text();

        // **1. No reading: in no list.**
        assert!(what_is_kept(Some(&record), Some(&chosen)).rows.iter().all(|row| row.job != chosen));

        // **2. The reading's ending, drawn.**
        let kept = what_is_kept_beside(Some(&record), Some(&chosen), &|_| true, &owed);
        assert_eq!(kept.jobs, 2);
        let row = kept.rows.iter().find(|row| row.job == chosen).expect("the job is in History");
        assert_eq!(row.ending, Some(AnEnding::Failed));
        assert!(row.the_ending_is_owed);
        assert_eq!(row.frames, None, "the frames the server held come with the landing");
        assert_eq!(kept.chosen.as_deref(), Some(chosen.as_str()));
        let entry = kept.entry.as_ref().expect("its entry");
        assert_eq!(entry.ending, Some(AnEnding::Failed));
        assert!(entry.the_ending_is_owed);
        let failure = entry.failure.as_ref().expect("the failure shape");
        assert_eq!(failure.said, THE_RENDER_DID_NOT_FINISH);
        assert_eq!(failure.what_to_do_about_it.as_deref(), Some(THE_DIAGNOSIS));
        assert_eq!(failure.values, vec![(THE_SERVER_LABEL, "RunPod 4090".to_string())]);
        assert!(!failure.blender_output, "no words are kept yet, so no control opens none");
        assert!(entry.entry.preview.is_none(), "no empty picture: whether a frame finished is not known yet");
        let last_frame = entry.rendered.iter().find(|fact| fact.name == LAST_FRAME_FACT).expect("the last-frame line");
        assert!(
            last_frame.parts.iter().all(|part| part.lead != NONE_FINISHED)
                && last_frame.parts.iter().any(|part| part.lead == NOT_KNOWN),
            "the last frame is not known yet, never none finished: {last_frame:?}"
        );
        let text = said(&the_history_says(&kept));
        assert!(text.contains("\"failWhatToDoAboutIt\""), "the diagnosis reaches the page: {text}");

        // **3. Neither Delete nor a clearing reaches it.**
        assert_eq!(the_history_as_asked(kept.clone(), Some(&chosen), Some(TheHistoryAsk::Delete)).asking, None);
        assert!(!text.contains("\"delete\""), "Delete is not offered over it: {text}");
        assert_eq!(
            the_history_as_asked(kept.clone(), Some(&chosen), Some(TheHistoryAsk::ClearThisServer)).asking,
            Some(the_clear_box("RunPod 4090", 1)),
            "the count is the entry that goes"
        );
        assert_eq!(
            the_history_as_asked(kept.clone(), Some(&chosen), Some(TheHistoryAsk::ClearAll)).asking,
            Some(the_clear_all_box(1, &[("RunPod 4090".to_string(), 1)]))
        );

        // **4. Another server's reading, and one with room.**
        let elsewhere = vec![AnEndingTheReadingCarries { server: "another".to_string(), ..owed[0].clone() }];
        assert!(what_is_kept_beside(Some(&record), None, &|_| true, &elsewhere).rows.iter().all(|row| row.job != chosen));
        let room = vec![AnEndingTheReadingCarries { owed: a_reading_of(155_000_000), ..owed[0].clone() }];
        let with_room = what_is_kept_beside(Some(&record), Some(&chosen), &|_| true, &room);
        let failure = with_room.entry.and_then(|entry| entry.failure).expect("the failure shape");
        assert_eq!(failure.what_to_do_about_it, None, "a reading with room is not a full disk");
        assert_eq!(failure.said, format!("{THE_RENDER_DID_NOT_FINISH} {SMOOZY_DOES_NOT_KNOW_WHY}"));

        // **5. The landing: the record's ending, and the reading not read.**
        store.the_fill_in(&shot.job, &the_note("Shot 40", true)).expect("the landing is learned");
        let landed = what_is_kept_beside(Some(&record), Some(&chosen), &|_| true, &owed);
        let row = landed.rows.iter().find(|row| row.job == chosen).expect("still in History");
        assert!(!row.the_ending_is_owed);
        assert_eq!(row.ending, Some(AnEnding::Failed));
        let entry = landed.entry.as_ref().expect("its entry");
        assert!(!entry.the_ending_is_owed);
        assert!(
            the_history_as_asked(landed.clone(), Some(&chosen), Some(TheHistoryAsk::Delete)).asking.is_some(),
            "a record with its ending may be deleted"
        );
        assert!(said(&the_history_says(&landed)).contains("\"delete\""), "and Delete is offered over it");

        drop(record);
        let _ = std::fs::remove_dir_all(&path);
    }

    /// **A job that started rendering draws a row, and this is the first time
    /// anything in Smoozy has put one on this screen.**
    ///
    /// **`Queue & History` C9 is the test and the standing is not**: the
    /// provisional record and the confirmed one whose render start is unlearned
    /// are both absent, and the one the fill-in carried a note over for is
    /// there.
    #[test]
    fn a_job_that_started_rendering_draws_a_row_and_the_others_draw_none() {
        use crate::record::job_records::JobRecords;
        use smoozy_queue_history::acting::note::{TheEndingOnTheNote, TheNote, TheOutcomeAsWritten};
        use smoozy_queue_history::record::{
            AnswerOrigin, Hue, Standing, TheBlenderTheEntryKeeps, TheDeviceAsk, TheServerAsItStood,
            WhatTheNoteCarried,
        };
        use smoozy_state::identities::IdentityKind;
        use smoozy_state::observed::{Clock, MachineClock, Reading};
        use std::time::{Duration, SystemTime};

        let mut path = std::env::temp_dir();
        path.push(format!("smoozy-history-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).unwrap();
        let path = path.join("local-record.sqlite3");
        let record = LocalRecord::open_at(&path).expect("a fresh record opens");

        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        // **The server stands**, so its badge is its own colour: since the
        // eighteenth notes, piece 7, a server with no row draws the grey one
        // (Design, *History*, note 12).
        record
            .connection()
            .execute(
                "INSERT INTO server_row (
                     identifier, host, port, username, key_identifier, key_fingerprint,
                     remembered_setup
                 ) VALUES (?1, 'h', 22, 'root', 'k', 'f', 0)",
                [server.as_text()],
            )
            .unwrap();
        let machine = Clock::AMachine(MachineClock(server.as_text()));
        let started = SystemTime::UNIX_EPOCH + Duration::from_secs(1_754_600_000);
        let records = JobRecords::over(record.connection());

        let mut jobs = Vec::new();
        for label in ["Kitchen", "Bedroom", "Hallway"] {
            let job = Identity::issue(IdentityKind::Job).unwrap();
            records
                .write_the_provisional_record(&LocalJobRecord {
                    job: job.clone(),
                    group: None,
                    server: TheServerAsItStood {
                        local_server: server.clone(),
                        label: Some("the big box".to_string()),
                        hue: Some(Hue(3)),
                    },
                    submitted_at: Some(Reading::taken((), started, Clock::ThisComputer)),
                    label: Some(label.to_string()),
                    blend_path_on_the_machine: None,
                    scene_name: Some("Interior".to_string()),
                    submitted_with_no_scene_named: false,
                    resolution_percentage_typed: None,
                    picture_size_from_the_read: None,
                    main_output_from_the_read: None,
                    blender: TheBlenderTheEntryKeeps {
                        pick: smoozy_render_settings::payload::VersionPick {
                            build_path: "/b".to_string(),
                            version: "4.5.12".to_string(),
                        },
                        origin: AnswerOrigin::ResolvedForThem,
                    },
                    device_asked_for: TheDeviceAsk::TheFilesOwnUnread,
                    the_files_values_beside_the_overrides: None,
                    dependency_check: None,
                    carries_on_from: None,
                    disturbances: vec![],
                    attempts_where_the_file_changed: vec![],
                    the_frames: smoozy_queue_history::record::TheFramesTheReadFound::default(),
                    standing: Standing::Provisional,
                    the_render_start_has_been_learned: false,
                    what_the_note_carried: WhatTheNoteCarried::default(),
                    the_job_output_folder: Default::default(),
                    the_driver_was_below_the_bar: None,
                })
                .expect("written");
            jobs.push(job);
        }
        // The second is confirmed and has not started; the third has.
        records
            .the_resolution(
                &jobs[1],
                smoozy_queue_history::record::WhatTheMachineSaidAboutTheJob::ItHoldsTheJob,
            )
            .expect("resolved");
        let mut note = TheNote::created_with_the_job_record("Hallway");
        note.started = Some(Reading::taken((), started, machine.clone()));
        note.ended = Some(Reading::taken((), started + Duration::from_secs(9_120), machine));
        note.ending = Some(TheEndingOnTheNote {
            outcome: TheOutcomeAsWritten::Finished,
            exit_status: Some(0),
            the_attempt_whose_capture_record_holds_the_words: 1,
            the_reason_the_machine_recorded: None,
            no_render_began: false,
            the_device_line_as_observed: None,
        });
        records.the_fill_in(&jobs[2], &note).expect("filled in");

        let kept = what_is_kept(Some(&record), None);
        assert_eq!(kept.jobs, 1);
        assert_eq!(kept.rows.len(), 1);
        let row = &kept.rows[0];
        assert_eq!(row.job, jobs[2].as_text());
        assert_eq!(row.name, "Hallway");
        assert_eq!(row.ending, Some(AnEnding::Finished));
        assert_eq!(row.server, "the big box");
        // **The badge is the drawing's own class**, never the bare number the
        // record keeps (the Creator's notes on the test copy, piece 8).
        assert_eq!(row.badge.as_deref(), Some("b3"));
        assert_eq!(row.scene.as_deref(), Some("Interior"));
        // **2h 32m**, the drawing's own shape, from the note's two readings.
        assert_eq!(row.how_long.as_deref(), Some("2h 32m"));
        assert_eq!(row.when.as_deref(), Some("1754609120"));
        // And the chosen entry falls to the one row there is.
        assert_eq!(kept.chosen.as_deref(), Some(jobs[2].as_text().as_str()));

        drop(record);
        let _ = std::fs::remove_dir_all(path.parent().unwrap());
    }

    /// **The two shapes Design draws, and the third this stage picked.**
    #[test]
    fn for_how_long_reads_the_way_the_drawing_does() {
        use std::time::Duration;
        assert_eq!(for_how_long(Duration::from_secs(9_120)), "2h 32m");
        assert_eq!(for_how_long(Duration::from_secs(2_820)), "47m");
        assert_eq!(for_how_long(Duration::from_secs(240)), "4m");
        // **Never *0m* for something that plainly happened.**
        assert_eq!(for_how_long(Duration::from_secs(20)), "<1m");
    }

    /// **The five endings, and the fifth is not an ending.** Design: the word
    /// carries the difference rather than the colour, and every other ending is
    /// one word while this one is plainly not.
    #[test]
    fn the_fifth_ending_is_a_phrase_and_not_a_fifth_colour_on_a_word() {
        let words = |ending: AnEnding| ending.said().split(' ').count();
        for ending in [
            AnEnding::Finished,
            AnEnding::Failed,
            AnEnding::Stopped,
            AnEnding::GivenUp,
        ] {
            assert!(words(ending) == 1, "{}", ending.said());
        }
        // **The shape of the phrase does the work**, which is why it is longer
        // than any of the four: two greys side by side would be the weakest
        // distinction on the screen.
        assert!(words(AnEnding::NotKnown) > 1);
        let mut classes: Vec<&str> = [
            AnEnding::Finished,
            AnEnding::Failed,
            AnEnding::Stopped,
            AnEnding::GivenUp,
            AnEnding::NotKnown,
        ]
        .map(AnEnding::class)
        .to_vec();
        classes.sort_unstable();
        classes.dedup();
        assert_eq!(classes.len(), 5);
        // **Item C1's word is the same grey italic, and a phrase too.**
        assert_eq!(AnEnding::StartNotKnown.class(), AnEnding::NotKnown.class());
        assert!(words(AnEnding::StartNotKnown) > 2);
        assert!(the_failure_for(AnEnding::StartNotKnown, false, None, false, false).is_none());
    }

    /// **Only three of the five carry the failure shape.** A clean finish did
    /// what it was asked; an ending Smoozy never learned is not an ending.
    #[test]
    fn a_clean_finish_and_an_unknown_ending_carry_no_failure_shape() {
        assert!(the_failure_for(AnEnding::Finished, false, None, false, false).is_none());
        assert!(the_failure_for(AnEnding::NotKnown, false, None, false, false).is_none());
        assert!(the_failure_for(AnEnding::Failed, false, None, false, false).is_some());
        assert!(the_failure_for(AnEnding::Stopped, false, None, false, false).is_some());
        assert!(the_failure_for(AnEnding::GivenUp, false, None, false, false).is_some());
    }

    /// **The disk-full carve-out fills the second sentence and takes the
    /// *Cause unknown* half off the first** — M14, and Design's
    /// *Nothing else on a failed entry names a cause*.
    #[test]
    fn the_disk_full_sentence_replaces_the_dont_know_half_and_names_the_server() {
        let plain = the_failure_for(AnEnding::Failed, false, None, false, false).unwrap();
        assert!(plain
            .as_the_diary_keeps_it()
            .contains(SMOOZY_DOES_NOT_KNOW_WHY));
        assert!(plain.what_to_do_about_it.is_none());
        assert!(plain.values.is_empty());

        let full =
            the_failure_for(AnEnding::Failed, false, Some("RunPod 4090"), false, false).unwrap();
        assert!(!full
            .as_the_diary_keeps_it()
            .contains(SMOOZY_DOES_NOT_KNOW_WHY));
        // **The server's name is a value of its own and is never inside the
        // words** — X7's trap, and the one this whole split exists for.
        assert_eq!(full.values, vec![(THE_SERVER_LABEL, "RunPod 4090".to_string())]);
        let second = full.what_to_do_about_it.unwrap();
        assert!(!second.contains("RunPod"));
        assert!(!full.said.contains("RunPod"));
    }

    /// **Step 12's SC4 findings 2 and 3: the failure names the full disk from
    /// the reading the ending carried, in the carve-out's one sentence.**
    ///
    /// `Rented-server runtime` V12's reading rides the ending onto the record
    /// (`Desktop app and delivery` M15a); route (b) fires on a failed render
    /// whose reading says nothing was available, and on nothing else — a
    /// reading with room, no reading at all, and a stop all draw no second
    /// sentence. **Before the fix the entry passed no reading**, so the scenario's
    /// failed job said only that Smoozy did not know why. **And the sentence is
    /// the per-folder one** (`Desktop app and delivery` M17, `Server protocol
    /// and transfers` F16): History's own *"The disk on this server is full."*
    /// went, on the runner's ruling.
    #[test]
    fn the_entry_names_the_full_disk_from_the_reading_its_ending_carried() {
        use smoozy_queue_history::acting::note::{
            TheEndingOnTheNote, TheFreeSpaceReadAtTheEnding, TheOutcomeAsWritten,
        };
        let server = Identity::issue(smoozy_state::identities::IdentityKind::LocalServer).unwrap();
        let mut record = crate::record::deletions::tests::a_record_in_history(&server, "Shot 40");
        let ended = |record: &mut LocalJobRecord, outcome, available: Option<u64>| {
            record.what_the_note_carried.ending = Some(TheEndingOnTheNote {
                outcome,
                exit_status: Some(28),
                the_attempt_whose_capture_record_holds_the_words: 1,
                the_reason_the_machine_recorded: None,
                no_render_began: false,
                the_device_line_as_observed: None,
            });
            record.what_the_note_carried.last_frame_figure = Some(40);
            record.what_the_note_carried.the_free_space_at_the_ending =
                available.map(|available_bytes| TheFreeSpaceReadAtTheEnding {
                    available_bytes,
                    the_folder_it_was_taken_against: "/root/jobs/shot-40".to_string(),
                    taken_at: smoozy_state::observed::Reading::taken(
                        (),
                        std::time::SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(1_700_000_000),
                        smoozy_state::observed::Clock::AMachine(smoozy_state::observed::MachineClock(
                            "the big box".to_string(),
                        )),
                    ),
                });
        };
        let second = |record: &LocalJobRecord| {
            the_entry_for(record, &BesideTheRecord::default(), false, true)
                .failure
                .and_then(|failure| failure.what_to_do_about_it)
        };
        ended(&mut record, TheOutcomeAsWritten::Failed, Some(0));
        let failure = the_entry_for(&record, &BesideTheRecord::default(), false, true)
            .failure
            .expect("a failed ending carries the failure shape");
        assert_eq!(failure.what_to_do_about_it.as_deref(), Some(THE_DIAGNOSIS));
        assert_eq!(failure.said, THE_RENDER_DID_NOT_FINISH, "a named cause keeps its own sentence");
        assert_eq!(failure.values, vec![(THE_SERVER_LABEL, "the big box".to_string())]);
        ended(&mut record, TheOutcomeAsWritten::Failed, Some(155_000_000));
        assert_eq!(second(&record), None, "a reading with room is not a full disk");
        ended(&mut record, TheOutcomeAsWritten::Failed, None);
        assert_eq!(second(&record), None, "no reading is not a reading of nought");
        ended(&mut record, TheOutcomeAsWritten::LeftStopped, Some(0));
        assert_eq!(second(&record), None, "route (b) is a render that ended failing");
        // **One sentence for the one diagnosis**, and it is the folder's.
        let full = the_failure_for(AnEnding::Failed, false, Some("the big box"), false, false).unwrap();
        assert_eq!(full.what_to_do_about_it.as_deref(), Some(THE_DIAGNOSIS));
        let source = include_str!("the_history.rs").split("#[cfg(test)]").next().unwrap().to_string();
        assert!(!source.contains(concat!("const THE_DISK", "_ON")), "History keeps a disk sentence of its own");
    }

    /// **Design, *Where no render began*: the film's first job.** It reads what
    /// happened, with the reason at once, no control to open first and no sentence
    /// about a render. The disk-full sentence says *when the render ended*, so it does
    /// not stand here either.
    #[test]
    fn a_failure_before_any_render_says_so_and_a_render_that_began_keeps_its_words() {
        let before = the_failure_for(AnEnding::Failed, true, Some("Old 4080"), true, true).unwrap();
        assert_eq!(before.as_the_diary_keeps_it(), "Failed before rendering.");
        assert_eq!(before.what_to_do_about_it, None);
        assert!(before.values.is_empty());
        assert!(before.report_a_problem);
        for no_frame in [true, false] {
            let began = the_failure_for(AnEnding::Failed, no_frame, None, false, false).unwrap();
            assert_ne!(began.as_the_diary_keeps_it(), "Failed before rendering.");
        }
        assert_eq!(
            the_failure_for(AnEnding::Failed, true, None, false, false).unwrap().as_the_diary_keeps_it(),
            "The render did not finish. No frames completed.",
            "a render that began and ended badly keeps today's words"
        );
        let stopped = the_failure_for(AnEnding::Stopped, true, None, false, true).unwrap();
        assert_ne!(stopped.as_the_diary_keeps_it(), "Failed before rendering.");
        assert_eq!(
            the_reason_as_a_sentence("could not copy the bake folder //caches/shot/"),
            "Could not copy the bake folder //caches/shot/"
        );
    }

    /// **Must-do 20: the page does not write the word *Server* itself.** The
    /// label travels as prose and the name as a value, and the page draws both
    /// as they arrive. Before the fix the page pushed a literal.
    #[test]
    fn the_disk_full_servers_label_is_prose_the_application_sent() {
        let entry = TheHistoryEntry {
            failure: the_failure_for(AnEnding::Failed, false, Some("RunPod 4090"), false, false),
            ..TheHistoryEntry::default()
        };
        let text = said(&the_history_entry_is_said(Event::about(About::TheRun, THE_HISTORY_SAYS), &entry));
        assert!(text.contains("failValue0Label"));
        assert!(text.contains(&format!("{:?}", Prose::new(THE_SERVER_LABEL))));
        assert!(text.contains(&format!("{:?}", Verbatim::new("RunPod 4090"))));
        let page = include_str!("../../web/the-history.js");
        assert!(!page.contains("[\"Server\""), "the page writes the word Server itself");
        assert!(page.contains("said[\"failValue\" + at + \"Label\"]"));
    }

    /// **The Creator's eighth notes, note 6 (2026-10-11): a failed entry whose
    /// *Errors* stream was empty claims nothing about either stream.** Blender 5
    /// writes its errors to its ordinary output, so the sentence neither says
    /// the render reported no errors nor speaks for the output it never read.
    #[test]
    fn a_failed_entry_with_empty_errors_claims_nothing_about_the_output() {
        for no_frame in [false, true] {
            let failure = the_failure_for(AnEnding::Failed, no_frame, None, true, false).unwrap();
            assert_eq!(failure.said, "The render failed. Blender's output may say why.");
            let said = failure.said.to_lowercase();
            assert!(!said.contains("no errors"), "{said}");
            assert!(!said.contains("nothing in"), "{said}");
            assert!(!said.contains("says why it ended"), "{said}");
        }
    }

    /// **Must-do 9: the no-errors ending has one plain sentence, and it is the
    /// one the diary keeps.** Before the fix the diary kept *The render did not
    /// finish* while the page drew the no-errors line under it.
    #[test]
    fn the_no_errors_ending_is_the_plain_sentence_and_the_diarys() {
        for no_frame in [false, true] {
            let failure = the_failure_for(AnEnding::Failed, no_frame, None, true, false).unwrap();
            assert_eq!(failure.as_the_diary_keeps_it(), THE_RENDER_FAILED_AND_THE_OUTPUT_MAY_SAY_WHY);
        }
        let with_the_disk = the_failure_for(AnEnding::Failed, false, Some("Old 4080"), true, false).unwrap();
        assert_eq!(with_the_disk.said, THE_RENDER_DID_NOT_FINISH, "a named cause keeps its own sentence");
        let entry = TheHistoryEntry {
            failure: the_failure_for(AnEnding::Failed, false, None, true, false),
            ..TheHistoryEntry::default()
        };
        let text = said(&the_history_entry_is_said(Event::about(About::TheRun, THE_HISTORY_SAYS), &entry));
        assert!(!text.contains("failReportedNoErrors"), "a second paragraph is still drawn");
        let page = include_str!("../../web/the-history.js");
        assert!(!page.contains("failReportedNoErrors"));
    }

    /// **The controls are the shape's own and the application composes them** —
    /// F-13's repair. **No cross on an ending, and no retry either**: Design
    /// puts the act that would try again in the *Render it again* group *"so
    /// the failure does not repeat it"*.
    #[test]
    fn a_recorded_ending_carries_no_cross_and_does_not_repeat_the_retry() {
        for ending in [AnEnding::Failed, AnEnding::Stopped, AnEnding::GivenUp] {
            let failure = the_failure_for(ending, false, None, false, false).unwrap();
            for control in failure.the_controls() {
                assert_ne!(control.what_it_is, WhatItIs::TheCross);
                assert_ne!(control.what_it_is, WhatItIs::TryAgain);
            }
        }
    }

    /// **A stop is not a fault, so nothing on it offers to report one.** Design
    /// draws the report control on every other failed ending.
    #[test]
    fn the_one_ending_the_user_caused_offers_no_report() {
        let stopped = the_failure_for(AnEnding::Stopped, false, None, false, false).unwrap();
        assert!(!stopped.report_a_problem);
        for ending in [AnEnding::Failed, AnEnding::GivenUp] {
            assert!(the_failure_for(ending, false, None, false, false)
                .unwrap()
                .report_a_problem);
        }
    }

    /// **X7 over the whole event: no name and no figure is joined into a piece
    /// of prose before the point.**
    #[test]
    fn no_name_and_no_figure_travels_inside_a_sentence() {
        let says = TheHistoryOnScreen {
            jobs: 128,
            rows: vec![AHistoryRowDrawn {
                name: "Kitchen — final v3".to_string(),
                frames: Some(("60".to_string(), "240".to_string())),
                ending: Some(AnEnding::Failed),
                server: "RunPod 4090".to_string(),
                when: Some("1754688840".to_string()),
                ..AHistoryRowDrawn::default()
            }],
            ..TheHistoryOnScreen::default()
        };
        let text = said(&the_history_says(&says));
        assert!(!text.contains("128 jobs"));
        assert!(!text.contains("60/240"));
        assert!(text.contains("hrow0Frames"));
        assert!(text.contains("hrow0Of"));
    }

    fn a_range(first: i64, last: i64, step: usize) -> Vec<i64> {
        (first..=last).step_by(step).collect()
    }

    /// **Must-do 2: the carry-on opens at the boundary frame and counts the
    /// job's own asked frames from there** — `Rendering` A4, `Queue & History`
    /// N6. Design's worked example: the figure is 60 of 1 to 240, so the
    /// carry-on starts at 60 and renders 181 frames. Before the fix this said
    /// 180 and named frame 61.
    #[test]
    fn the_carry_on_opens_at_the_boundary_frame() {
        let asked = a_range(1, 240, 1);
        let pair = the_ways_to_render_it_again(AnEnding::Failed, &asked, Some(60), WhereTheFigureCameFrom::RecordedWhenTheJobEnded, true);
        assert_eq!(pair.submit_again, 240);
        let carry_on = pair.carry_on.expect("a frame figure earns the pair");
        assert_eq!((carry_on.frames, carry_on.at), (181, 60));
        assert!(pair.why.is_none());
    }

    /// **A range that does not start at 1, and one with a step**, which the
    /// old subtraction got wrong for both. `Specs/Render Settings`: *"Nothing in
    /// Smoozy assumes the first frame is frame 1."*
    #[test]
    fn the_carry_on_counts_asked_frames_for_a_range_not_starting_at_one() {
        let asked = a_range(1001, 1100, 1);
        let pair = the_ways_to_render_it_again(AnEnding::Stopped, &asked, Some(1060), WhereTheFigureCameFrom::RecordedWhenTheJobEnded, true);
        let carry_on = pair.carry_on.expect("offered");
        assert_eq!((carry_on.frames, carry_on.at), (41, 1060));
        assert_eq!(pair.submit_again, 100);

        let stepped = a_range(0, 98, 2);
        let pair = the_ways_to_render_it_again(AnEnding::GivenUp, &stepped, Some(0), WhereTheFigureCameFrom::RecordedWhenTheJobEnded, true);
        let carry_on = pair.carry_on.expect("frame 0 is a real frame and a real figure");
        assert_eq!((carry_on.frames, carry_on.at), (50, 0));
        let pair = the_ways_to_render_it_again(AnEnding::GivenUp, &stepped, Some(90), WhereTheFigureCameFrom::RecordedWhenTheJobEnded, true);
        assert_eq!(pair.carry_on.expect("offered").frames, 5);
    }

    /// **The pair is earned by a frame figure and a standing server, and by
    /// nothing else** — Design: *So ended short is not on its own enough to earn
    /// the pair*, and `Queue & History` N17. Every missing carry-on says why.
    #[test]
    fn only_an_entry_with_a_frame_figure_and_a_server_offers_the_carry_on() {
        let asked = a_range(1, 240, 1);
        let recorded = WhereTheFigureCameFrom::RecordedWhenTheJobEnded;
        for (again, why) in [
            (the_ways_to_render_it_again(AnEnding::Finished, &asked, Some(240), recorded, true), EVERY_FRAME_IS_RENDERED),
            (the_ways_to_render_it_again(AnEnding::NotKnown, &asked, None, recorded, true), NO_FRAME_TO_START_FROM),
            (the_ways_to_render_it_again(AnEnding::Failed, &asked, None, recorded, true), NO_FRAME_FINISHED_SO_NOTHING_TO_START_FROM),
            (the_ways_to_render_it_again(AnEnding::Failed, &asked, Some(60), recorded, false), THE_SERVER_IS_GONE_SO_NO_CARRY_ON),
            (the_ways_to_render_it_again(AnEnding::Failed, &asked, Some(999), recorded, true), NO_FRAME_TO_START_FROM),
        ] {
            assert!(again.carry_on.is_none());
            assert_eq!(again.why, Some(why));
            assert_eq!(again.submit_again, 240, "Submit again is always offered");
        }
    }

    /// **Must-do 2(b) and (c): the page draws the boundary frame the
    /// application sent and adds nothing, and the *from* line says where the
    /// figure came from.** The recorded wording is the render settings page's.
    #[test]
    fn the_carry_on_line_says_where_its_figure_came_from_and_the_page_does_no_sums() {
        let again = |came_from| TheHistoryEntry {
            again: Some(the_ways_to_render_it_again(AnEnding::Failed, &a_range(1, 240, 1), Some(60), came_from, true)),
            ..TheHistoryEntry::default()
        };
        let recorded = said(&the_history_entry_is_said(
            Event::about(About::TheRun, THE_HISTORY_SAYS),
            &again(WhereTheFigureCameFrom::RecordedWhenTheJobEnded),
        ));
        assert!(recorded.contains(&format!("{:?}", Prose::new(THE_FRAME_THIS_ENTRY_RECORDED))));
        assert!(!recorded.contains("read from the server"), "a remembered figure was presented as a checked one");
        assert!(recorded.contains("carryOnWhyFrame"));
        assert!(recorded.contains(&format!("{:?}", Verbatim::new("181"))));
        let read = said(&the_history_entry_is_said(
            Event::about(About::TheRun, THE_HISTORY_SAYS),
            &again(WhereTheFigureCameFrom::ReadFromTheServerJustNow),
        ));
        assert!(read.contains(&format!("{:?}", Prose::new(READ_FROM_THE_SERVER_JUST_NOW))));

        // **One wording for one fact, and now one phrase** (L6): the words
        // are the render settings page's own and this file holds no copy.
        assert!(
            crate::surface::render_settings::words::CARRYING_ON_UNREAD
                .contains(the_recorded_figure!()),
            "the render settings page no longer says {:?}; one fact has two wordings",
            the_recorded_figure!()
        );
        let (working, _) = include_str!("the_history.rs")
            .split_once("#[cfg(test)]")
            .expect("tests last");
        assert!(
            !working.contains("\"the frame recorded when the job ended\""),
            "this file writes the phrase out again instead of taking it from its one home"
        );
        let page = include_str!("../../web/the-history.js");
        assert!(!page.contains("+ 1)"), "the page adds to the frame");
        assert!(page.contains("said.carryOnWhyFrame"));
        assert!(page.contains("field.value = said.carryOnFrom"));
        assert!(page.contains("from: field.value"), "the typed figure does not go with the carry-on act");
    }

    /// **The *Rendered* line names the boundary frame itself as the one not
    /// confirmed** — nothing adds one to it.
    #[test]
    fn the_rendered_line_names_the_boundary_frame() {
        let group = the_rendered_group(Some(60), true, NONE_FINISHED, None, None, None, None, None, false);
        assert_eq!(group[0].parts[1].value, "60");
        let group = the_rendered_group(Some(0), false, NONE_FINISHED, None, None, None, None, None, false);
        assert_eq!(group[0].parts, vec![APart::value("0")], "frame 0 is a frame and not *none finished*");
        let group = the_rendered_group(None, false, NONE_FINISHED, None, None, None, None, None, false);
        assert_eq!(group[0].parts, vec![APart::words(NONE_FINISHED)]);
    }

    /// **`Queue & History` N17's lookup: no record, or no row, stands for
    /// nothing.**
    #[test]
    fn a_server_that_is_not_in_the_record_does_not_stand() {
        assert!(!a_server_row_stands(None, "row-1"));
    }

    /// **The delete box says *settings* and never *setup*** — Design, and Rule
    /// 9: *setup* is this app's word for a server setup on three other screens,
    /// and one word doing two jobs is the vagueness that rule exists to catch.
    #[test]
    fn the_boxes_say_settings_and_never_setup() {
        for text in [
            THE_RECORD_IS_DELETED,
            ITS_ENTRIES_ARE_DELETED,
            ITS_ENTRY_IS_DELETED,
            AFTERWARDS_NOTHING_SHOWS_THEM,
            AFTERWARDS_NOTHING_SHOWS_IT,
        ] {
            assert!(!text.to_lowercase().contains("setup"), "{text}");
        }
        assert!(THE_RECORD_IS_DELETED.contains("settings"));
    }

    fn every_word(the_box: &TheBoxThatAsks) -> String {
        the_box
            .said
            .iter()
            .flatten()
            .map(|part| format!("{} {} {}", part.lead, part.value, part.trail))
            .collect::<Vec<_>>()
            .join(" | ")
    }

    /// **Where nothing of the job was downloaded the sentence is absent**,
    /// because there is nothing untouched to name.
    #[test]
    fn a_job_that_downloaded_nothing_names_no_untouched_downloads() {
        let none = the_delete_box(Some(60), false, Some(0));
        assert_eq!(none.said.len(), 2);
        let some = the_delete_box(Some(60), false, Some(48));
        assert_eq!(some.said.len(), 2, "one sentence names both halves");
        assert_eq!(
            some.said[1],
            vec![
                APart::figure("60", FRAMES_ON_THE_SERVER_AND),
                APart::figure("48", DOWNLOADED_FRAMES),
                APart::words(ARE_NOT_CHANGED),
            ]
        );
        assert_eq!(
            none.said[1],
            vec![APart::figure("60", FRAMES_ON_THE_SERVER), APart::words(ARE_NOT_CHANGED)]
        );
    }

    /// **Must-do 10, first case: a gone server with nothing downloaded never
    /// says *The 0 frames already downloaded…*.** Before the fix the guard
    /// covered only half the pattern and it did.
    #[test]
    fn a_gone_server_with_nothing_downloaded_says_no_nought() {
        let the_box = the_delete_box(None, true, Some(0));
        let said = every_word(&the_box);
        assert!(!said.contains(" 0 "), "{said}");
        assert_eq!(the_box.said.len(), 2, "{said}");
        let unknown = the_delete_box(None, true, None);
        assert!(every_word(&unknown).contains(ANYTHING_ALREADY_DOWNLOADED));
    }

    /// **Must-do 10, second case: a count on the server that is not known is
    /// never said as *Nothing was rendered*.**
    #[test]
    fn an_unknown_count_on_the_server_is_not_said_as_nothing() {
        let said = every_word(&the_delete_box(None, false, None));
        assert!(!said.contains(NOTHING_WAS_RENDERED), "{said}");
        assert!(every_word(&the_delete_box(Some(0), false, None)).contains(NOTHING_WAS_RENDERED));
    }

    /// **Must-do 9, over the upstream must-do: on a server that still stands, with neither count
    /// known — which is every entry today — the box says both halves are
    /// untouched, without a number.**
    ///
    /// **This failed before the fix**: the box said only that the record goes,
    /// and never answered whether the renders go too.
    #[test]
    fn a_standing_server_with_no_count_says_what_is_untouched_without_a_number() {
        let the_box = the_delete_box(None, false, None);
        assert_eq!(
            the_box.said,
            vec![
                vec![APart::words(THE_RECORD_IS_DELETED)],
                vec![
                    APart::words(FRAMES_ON_THE_SERVER_UNCOUNTED_AND),
                    APart::words(DOWNLOADED_FRAMES_UNCOUNTED),
                    APart::words(ARE_NOT_CHANGED),
                ],
            ]
        );
        let words = every_word(&the_box);
        assert!(!words.chars().any(|one| one.is_ascii_digit()), "{words}");
        // **The counted case is unchanged** — the drawing's own.
        assert!(every_word(&the_delete_box(Some(60), false, Some(48))).contains(FRAMES_ON_THE_SERVER_AND));
        assert_eq!(
            the_delete_box(None, false, Some(0)).said[1],
            vec![APart::words(THE_FRAMES_ON_THE_SERVER_ARE_UNTOUCHED_UNCOUNTED), APart::words(ARE_NOT_CHANGED)]
        );
        // **One home for the downloaded sentence**: it is written once in this
        // file's working half.
        let working = include_str!("the_history.rs").split("#[cfg(test)]").next().unwrap();
        assert_eq!(working.matches("\"Downloaded frames are not changed.\"").count(), 1);
    }

    /// **Must-do 12: one of anything is singular**, in all three boxes.
    #[test]
    fn one_entry_is_an_entry_and_one_frame_is_a_frame() {
        let one_of_each = every_word(&the_delete_box(Some(1), false, Some(1)));
        assert!(one_of_each.contains(FRAME_ON_THE_SERVER_AND), "{one_of_each}");
        assert!(one_of_each.contains(DOWNLOADED_FRAME), "{one_of_each}");
        let alone = every_word(&the_delete_box(Some(1), false, Some(0)));
        assert!(alone.contains(FRAME_ON_THE_SERVER) && alone.contains(IS_NOT_CHANGED), "{alone}");
        let one = the_clear_box("RunPod 4090", 1);
        assert!(every_word(&one).contains(ITS_ENTRY_IS_DELETED));
        assert_eq!(one.go_ahead[1].trail, ENTRY_WORD);
        let all = the_clear_all_box(1, &[("RunPod 4090".to_string(), 1)]);
        assert!(every_word(&all).contains(AFTERWARDS_NOTHING_SHOWS_IT));
        assert_eq!(all.said[0][0].lead, THE_ONE_WORD);
        assert_eq!(all.go_ahead[2].trail, ENTRY_WORD);
        let text = said(&the_box_is_said(Event::about(About::TheRun, THE_HISTORY_SAYS), Some(&all)));
        assert!(text.contains("boxServer0EntriesWord"));
        let two = the_clear_box("RunPod 4090", 2);
        assert!(every_word(&two).contains(ITS_ENTRIES_ARE_DELETED));
    }

    /// **`WhoseSentenceItIs::TheComposersOwn` is the one value this screen
    /// passes**, and this is the line that fails if somebody changes it.
    ///
    /// **A screen that passed the other one produces no error, no failing test
    /// and no visible fault** — only a problem report months later — so the
    /// check is on the source of the one call site rather than on behaviour.
    #[test]
    fn this_screen_hands_the_diary_its_own_sentence_and_never_a_stand_in() {
        let source = include_str!("the_history.rs");
        let (working, _) = source
            .split_once("#[cfg(test)]")
            .expect("the tests are the last thing in this file");
        assert_eq!(
            working.matches("WhoseSentenceItIs::TheComposersOwn(").count(),
            1,
            "this screen reaches `WhoseSentenceItIs` more than once. **There is \
             one call site and it passes `TheComposersOwn`** — M32: the capture \
             belongs to whatever composes the sentence the user reads"
        );
        assert!(
            !working.contains("AStandInThePlumbingComposed"),
            "this screen composes the sentence the user reads and must pass \
             `TheComposersOwn`. **A stand-in passed from here leaves the diary \
             holding the plumbing's words for the life of the record**, with \
             nothing failing and nothing visible"
        );
    }

    /// **The sentence the diary keeps is the composed one before the typography
    /// point** — M32, and `Reporting a problem` V1's parity.
    #[test]
    fn the_diarys_sentence_is_the_composed_one_before_the_point() {
        let failure = the_failure_for(AnEnding::Failed, false, Some("Old 4080"), false, false).unwrap();
        let kept = failure.as_the_diary_keeps_it();
        assert_eq!(kept, THE_RENDER_DID_NOT_FINISH);
        assert!(!kept.contains("Old 4080"));
        assert!(
            !kept.contains('\u{00A0}'),
            "the diary's sentence carries a non-breaking space, so it went \
             through the typography point. **X7: the point is on the path to \
             the screen and on no other path**, and the diary is not a screen"
        );
    }

    fn two_kept() -> TheHistoryOnScreen {
        let leaves = WhatDeletingLeaves {
            frames_on_the_server: Some(60),
            the_server_is_gone: false,
            downloaded: Some(48),
        };
        let kept = TheHistoryOnScreen {
            rows: vec![a_row("job-a", "row-1", "RunPod 4090"), a_row("job-b", "row-2", "Old 4080"), a_row("job-c", "row-1", "RunPod 4090")],
            entry: Some(TheHistoryEntry {
                entry: TheJobEntry {
                    job: "job-b".to_string(),
                    ..TheJobEntry::default()
                },
                what_deleting_leaves: leaves,
                ..TheHistoryEntry::default()
            }),
            ..TheHistoryOnScreen::default()
        };
        the_row_is_chosen(kept, Some("job-b"))
    }

    /// **Must-do 1: history picks an entry by the job, never by its place in
    /// the list**, and R4's *"The delete key deletes a history entry, asking as
    /// the control does."* An identity no longer in history opens no box.
    /// **This fails if the event or the command carries a row position as the
    /// choice**, which is how a Delete could open over a different job.
    #[test]
    fn delete_asks_over_the_job_named_and_never_over_a_row_position() {
        let kept = two_kept();
        assert_eq!(kept.chosen.as_deref(), Some("job-b"));
        assert_eq!(the_row_is_chosen(kept.clone(), Some("job-gone")).chosen.as_deref(), Some("job-a"));

        let asked = the_history_as_asked(kept.clone(), Some("job-b"), Some(TheHistoryAsk::Delete));
        assert_eq!(asked.asking, Some(the_delete_box(Some(60), false, Some(48))));
        assert_eq!(the_history_as_asked(kept.clone(), Some("job-b"), None).asking, None);
        assert_eq!(
            the_history_as_asked(kept.clone(), Some("job-gone"), Some(TheHistoryAsk::Delete)).asking,
            None,
            "a box opened over a job that is no longer in history"
        );
        assert_eq!(
            the_history_as_asked(TheHistoryOnScreen::default(), Some("job-b"), Some(TheHistoryAsk::Delete)).asking,
            None,
            "a box was raised with no entry to delete"
        );

        let text = said(&the_history_says(&kept));
        assert!(text.contains("hrowChosen"));
        assert!(text.contains(&format!("{:?}", Verbatim::new("job-b"))));
        assert!(!text.contains(&format!("\"hrowChosen\": Value({:?})", Verbatim::new("1"))), "the chosen entry travels as a row position");
        assert!(text.contains("hrow1Job"));

        // The command takes an identity, and the page hands one back.
        let source = include_str!("the_history.rs");
        let (working, _) = source.split_once("#[cfg(test)]").expect("the tests are last");
        assert!(working.contains("    chosen: Option<String>,\n    asking: Option<TheHistoryAsk>,"));
        assert!(!working.contains("chosen: Option<u32>"));
        assert!(!working.contains("chosen: Option<usize>"));
        let page = include_str!("../../web/the-history.js");
        assert!(!page.contains("dataset.at"), "the page still chooses a row by its place");
        assert!(page.contains("row.dataset.job = job;"));
        assert!(page.contains("theHistoryIsAskedFor(row.dataset.job, DELETE_IT);"));
        assert!(page.contains("remove.addEventListener(\"click\", () => theHistoryIsAskedFor(theChosenJob, DELETE_IT));"));
        assert!(page.contains("press.key !== \"Delete\" && press.key !== \"Backspace\""));
        assert!(page.contains("row.addEventListener(\"click\", () => theHistoryIsAskedFor(job));"));
        assert!(page.contains("export function theHistoryEntryIsOpened(job)"));
    }

    /// **The two clearing boxes are composed over the chosen entry's server**,
    /// and *Clear all* lists every server with its own count.
    #[test]
    fn the_clearing_boxes_count_by_server() {
        let kept = the_row_is_chosen(two_kept(), Some("job-a"));
        let this = the_history_as_asked(kept.clone(), Some("job-a"), Some(TheHistoryAsk::ClearThisServer));
        assert_eq!(this.asking, Some(the_clear_box("RunPod 4090", 2)));
        let all = the_history_as_asked(kept, Some("job-a"), Some(TheHistoryAsk::ClearAll));
        assert_eq!(
            all.asking,
            Some(the_clear_all_box(3, &[("RunPod 4090".to_string(), 2), ("Old 4080".to_string(), 1)]))
        );
    }

    /// **Must-do 5(e): with no entries, neither clearing control is drawn**;
    /// with entries, both are. Before the fix both were drawn on the empty
    /// screen and could be pressed.
    #[test]
    fn an_empty_history_draws_no_clearing_control() {
        let empty = said(&the_history_says(&TheHistoryOnScreen::default()));
        assert!(!empty.contains("clearAllHistory"));
        assert!(!empty.contains("clearThisServersHistory"));
        let kept = said(&the_history_says(&two_kept()));
        assert!(kept.contains("clearAllHistory"));
        assert!(kept.contains("clearThisServersHistory"));
        let page = include_str!("../../web/the-history.js");
        assert!(page.contains("if (said.clearThisServersHistory)"));
        assert!(page.contains("if (said.clearAllHistory)"));
    }

    /// **Must-do 3: no list is ever cut short.** Thirty history entries and ten
    /// servers with rows, every one of them on the event — the old tables held
    /// twenty-four entries and eight servers, and cut the rest with nothing
    /// said.
    #[test]
    fn thirty_entries_and_ten_servers_are_all_on_the_event() {
        let rows: Vec<AHistoryRowDrawn> = (0..30)
            .map(|at| a_row(&format!("job-{at}"), &format!("row-{}", at % 10), &format!("Server {}", at % 10)))
            .collect();
        let says = TheHistoryOnScreen {
            jobs: 30,
            rows,
            ..TheHistoryOnScreen::default()
        };
        let text = said(&the_history_says(&says));
        for at in 0..30 {
            assert!(text.contains(&format!("\"hrow{at}Name\"")), "entry {at} is missing");
            assert!(text.contains(&format!("{:?}", Verbatim::new(format!("job-{at}")))));
        }
        assert!(text.contains(&format!("\"hrowCount\": Value({:?})", Verbatim::new("30"))));

        use crate::surface::the_queue::{the_queue_says, ARowInTheQueue, AServersLine, TheQueueOnScreen};
        let queue = TheQueueOnScreen {
            jobs: 30,
            servers: 10,
            lines: (0..10)
                .map(|line| AServersLine {
                    row: format!("row-{line}"),
                    name: format!("Server {line}"),
                    rows: (0..3)
                        .map(|at| ARowInTheQueue {
                            job: format!("job-{line}-{at}"),
                            name: format!("Job {line}-{at}"),
                            ..ARowInTheQueue::default()
                        })
                        .collect(),
                    ..AServersLine::default()
                })
                .collect(),
            ..TheQueueOnScreen::default()
        };
        let text = said(&the_queue_says(&queue));
        for line in 0..10 {
            assert!(text.contains(&format!("\"line{line}Name\"")), "server {line} is missing");
            for at in 0..3 {
                assert!(text.contains(&format!("{:?}", Verbatim::new(format!("job-{line}-{at}")))), "row {line}-{at} is missing");
            }
        }
        assert!(text.contains(&format!("\"lineCount\": Value({:?})", Verbatim::new("10"))));
        assert!(text.contains(&format!("\"rowCount\": Value({:?})", Verbatim::new("30"))));
        for source in [include_str!("the_history.rs"), include_str!("the_queue.rs")] {
            let (working, _) = source.split_once("#[cfg(test)]").expect("the tests are last");
            assert!(!working.contains("macro_rules! indexed"), "a fixed table of names is back");
            assert!(!working.contains("THE_LONGEST_LIST"));
        }
    }

    /// **The three acts this screen performs, end to end over a real record.**
    ///
    /// **Nothing in Smoozy had ever deleted a history entry before step 10b
    /// piece 8**, the table having arrived with piece 6 and no writer of the
    /// tombstone with it. **The half this catches is the tombstone**: a record
    /// gone with none is the one state `Queue & History` X1 forbids, because
    /// the fill-in would bring the entry back at the next connection.
    #[test]
    fn deleting_and_clearing_change_the_record_and_leave_the_tombstones_behind() {
        use crate::record::deletions::tests::a_record_in_history;
        use crate::surface::the_queue::TheActOnAJob;
        use smoozy_state::identities::IdentityKind;
        use smoozy_state::tombstones::HistoryTombstones;

        let mut path = std::env::temp_dir();
        path.push(format!("smoozy-history-acts-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).unwrap();
        let record = LocalRecord::open_at(&path.join("local-record.sqlite3")).expect("a record");

        let one = Identity::issue(IdentityKind::LocalServer).unwrap();
        let two = Identity::issue(IdentityKind::LocalServer).unwrap();
        let store = JobRecords::over(record.connection());
        let written: Vec<LocalJobRecord> = [(&one, "Kitchen"), (&one, "Bedroom"), (&two, "Hallway")]
            .into_iter()
            .map(|(server, label)| {
                let made = a_record_in_history(server, label);
                store.write_the_provisional_record(&made).expect("written");
                made
            })
            .collect();

        let mut started = a_started_over(record);
        assert_eq!(what_is_kept(started.record.as_ref(), None).jobs, 3);

        // **The delete box's *Delete* takes that entry and no other.**
        let gone = written[0].job.as_text();
        assert_eq!(
            the_act_is_performed(&started, TheActOnAJob::DeleteTheEntry, Some(&gone)),
            Some(Ok(()))
        );
        let kept = what_is_kept(started.record.as_ref(), Some(&gone));
        assert_eq!(kept.jobs, 2);
        // **The chosen job fell to a job that is still there** — must-do 1.
        assert_ne!(kept.chosen.as_deref(), Some(gone.as_str()));
        let store = JobRecords::over(started.record.as_ref().unwrap().connection());
        assert!(store.tombstones().unwrap().tombstone_for(&written[0].job).is_some());
        // **A box over a job that has left deletes nothing and says so.**
        assert_eq!(
            the_act_is_performed(&started, TheActOnAJob::DeleteTheEntry, Some(&gone)),
            Some(Err(()))
        );

        // **Clear this server's history takes that server's entries alone.**
        let left = written[1].job.as_text();
        assert_eq!(
            the_act_is_performed(&started, TheActOnAJob::ClearThisServersHistory, Some(&left)),
            Some(Ok(()))
        );
        let kept = what_is_kept(started.record.as_ref(), None);
        assert_eq!(kept.jobs, 1);

        // **Clear all history takes what is left, whichever job is open.**
        let last = written[2].job.as_text();
        assert_eq!(
            the_act_is_performed(&started, TheActOnAJob::ClearAllHistory, Some(&last)),
            Some(Ok(()))
        );
        assert_eq!(what_is_kept(started.record.as_ref(), None).jobs, 0);
        let store = JobRecords::over(started.record.as_ref().unwrap().connection());
        let tombstones = store.tombstones().unwrap();
        for made in &written {
            assert!(
                tombstones.tombstone_for(&made.job).is_some(),
                "a record went with no tombstone, which X1 forbids"
            );
            assert!(!tombstones.may_be_filled_in(&made.job));
        }

        // **An act that does not come here is not performed here.**
        assert_eq!(
            the_act_is_performed(&started, TheActOnAJob::SubmitAgain, Some(&last)),
            None
        );
        // **No record at all is an act that could not be done**, never a
        // silent success (L8a).
        started.record = None;
        assert_eq!(
            the_act_is_performed(&started, TheActOnAJob::ClearAllHistory, None),
            Some(Err(()))
        );

        let _ = std::fs::remove_dir_all(&path);
    }

    /// **Design, *History* and *Clearing history*: removed servers' jobs sit
    /// together at the foot with a grey badge, the entry says *…, removed*,
    /// and *Clear removed servers' history* takes that group and nothing else**
    /// — `Queue & History` N17's test, `Queue & History` X6b's act, the
    /// eighteenth notes, notes 12 and 13. The box lists each removed server by
    /// its copied name with its own count and says nothing is changed on them;
    /// the act reaches no machine and leaves every entry of a standing server.
    #[test]
    fn removed_servers_sit_together_at_the_foot_and_their_history_clears_alone() {
        use crate::record::deletions::tests::a_record_in_history;
        use crate::surface::the_queue::TheActOnAJob;
        use smoozy_state::identities::IdentityKind;
        use smoozy_state::tombstones::{HistoryTombstones, MachineSideCopy};

        let mut path = std::env::temp_dir();
        path.push(format!("smoozy-history-removed-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).unwrap();
        let record = LocalRecord::open_at(&path.join("local-record.sqlite3")).expect("a record");
        let standing = Identity::issue(IdentityKind::LocalServer).unwrap();
        let lambda = Identity::issue(IdentityKind::LocalServer).unwrap();
        let paperspace = Identity::issue(IdentityKind::LocalServer).unwrap();
        record
            .connection()
            .execute(
                "INSERT INTO server_row (
                     identifier, host, port, username, key_identifier, key_fingerprint,
                     remembered_setup
                 ) VALUES (?1, 'h', 22, 'root', 'k', 'f', 0)",
                [standing.as_text()],
            )
            .unwrap();
        let store = JobRecords::over(record.connection());
        let mut at = 0u64;
        let written: Vec<LocalJobRecord> = [
            (&lambda, "Lambda A100", "Turntable"),
            (&standing, "RunPod 4090", "Kitchen"),
            (&paperspace, "Paperspace A6000", "Spins"),
            (&lambda, "Lambda A100", "Night"),
            (&standing, "RunPod 4090", "Hallway"),
        ]
        .into_iter()
        .map(|(server, name, label)| {
            let mut made = a_record_in_history(server, label);
            made.server.label = Some(name.to_string());
            made.server.hue = Some(smoozy_queue_history::record::Hue(1));
            // Newest last, so the list's own order is the reverse of this one.
            at += 60;
            made.submitted_at = Some(smoozy_state::observed::Reading::taken(
                (),
                std::time::SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(1_754_600_000 + at),
                smoozy_state::observed::Clock::ThisComputer,
            ));
            made.what_the_note_carried.ending = Some(smoozy_queue_history::acting::note::TheEndingOnTheNote {
                outcome: smoozy_queue_history::acting::note::TheOutcomeAsWritten::Finished,
                exit_status: Some(0),
                the_attempt_whose_capture_record_holds_the_words: 1,
                the_reason_the_machine_recorded: None,
                no_render_began: false,
                the_device_line_as_observed: None,
            });
            store.write_the_provisional_record(&made).expect("written");
            made
        })
        .collect();
        let started = a_started_over(record);
        let kept = what_is_kept(started.record.as_ref(), None);
        let names: Vec<(&str, bool, Option<&str>)> = kept
            .rows
            .iter()
            .map(|row| (row.name.as_str(), row.the_server_is_removed, row.badge.as_deref()))
            .collect();
        assert_eq!(
            names,
            vec![
                ("Hallway", false, Some("b1")),
                ("Kitchen", false, Some("b1")),
                ("Night", true, Some("brm")),
                ("Spins", true, Some("brm")),
                ("Turntable", true, Some("brm")),
            ],
            "removed servers' rows sit at the foot, in the list's own order, with the grey badge"
        );
        assert_eq!(kept.rows[2].server, "Lambda A100", "the row keeps the copied name");
        let text = said(&the_history_says(&kept));
        for key in ["theRemovedServers", "theRemovedFigure", "theRemovedWord", "clearRemovedServersHistory", "hrow2Removed", "hrow4Removed"] {
            assert!(text.contains(key), "{key} is not on the event");
        }
        assert!(text.contains(&format!("{:?}", Prose::new(REMOVED_SERVERS))));
        assert!(text.contains(&format!("{:?}", Prose::new(CLEAR_REMOVED_SERVERS_HISTORY))));
        assert!(!text.contains("hrow0Removed") && !text.contains("hrow1Removed"));
        // **No removed server, no group and no control.**
        let without = TheHistoryOnScreen { rows: kept.rows[..2].to_vec(), ..kept.clone() };
        let text = said(&the_history_says(&without));
        assert!(!text.contains("theRemovedServers") && !text.contains("clearRemovedServersHistory"));

        // **The entry: *Lambda A100, removed*, beside the grey squircle.**
        let night = written[3].job.as_text();
        let chosen = what_is_kept(started.record.as_ref(), Some(&night));
        let entry = chosen.entry.as_ref().expect("the entry is drawn");
        assert_eq!(entry.entry.server, "Lambda A100");
        assert_eq!(entry.entry.badge.as_deref(), Some("brm"));
        let text = said(&the_history_says(&chosen));
        assert!(text.contains("entryServerRemoved"));
        assert!(text.contains(&format!("{:?}", Prose::new(REMOVED_TRAIL))));
        let hallway = written[4].job.as_text();
        let text = said(&the_history_says(&what_is_kept(started.record.as_ref(), Some(&hallway))));
        assert!(!text.contains("entryServerRemoved"), "a standing server's entry is not removed");

        // **The box: each removed server by name, with its own count.**
        let asked = the_history_as_asked(chosen.clone(), Some(&night), Some(TheHistoryAsk::ClearRemovedServers));
        let the_box = asked.asking.clone().expect("the box opens");
        assert_eq!(the_box.which, "clear-removed");
        assert_eq!(the_box.go_ahead_act, TheActOnAJob::ClearRemovedServersHistory);
        assert_eq!(
            the_box.per_server,
            vec![("Lambda A100".to_string(), 2), ("Paperspace A6000".to_string(), 1)]
        );
        assert_eq!(the_box.go_ahead[1].value, "3");
        assert_eq!(the_box.said[1], vec![APart::words(NOTHING_IS_CHANGED_ON_REMOVED_SERVERS)]);
        assert_eq!(the_box, the_clear_removed_box(3, &the_box.per_server));
        let text = said(&the_history_says(&asked));
        assert!(text.contains(&format!("{:?}", Prose::new(CLEAR_THE_HISTORY_OF_REMOVED_SERVERS_QUESTION))));

        // **The act takes the removed servers' entries alone**, each with its
        // tombstone and the machine-side copy still owed.
        assert_eq!(
            the_act_is_performed(&started, TheActOnAJob::ClearRemovedServersHistory, Some(&night)),
            Some(Ok(()))
        );
        let left = what_is_kept(started.record.as_ref(), None);
        let names: Vec<&str> = left.rows.iter().map(|row| row.name.as_str()).collect();
        assert_eq!(names, vec!["Hallway", "Kitchen"]);
        let store = JobRecords::over(started.record.as_ref().unwrap().connection());
        let tombstones = store.tombstones().unwrap();
        for (at, made) in written.iter().enumerate() {
            let tombstone = tombstones.tombstone_for(&made.job);
            if made.server.local_server == standing {
                assert!(tombstone.is_none(), "entry {at} of a standing server was taken");
            } else {
                assert_eq!(tombstone.map(|one| one.machine_side_copy), Some(MachineSideCopy::StillOwed), "entry {at}");
            }
        }
        // **With nothing left in the group, no box opens and no control is sent.**
        let hallway = left.rows[0].job.clone();
        assert!(the_history_as_asked(left.clone(), Some(&hallway), Some(TheHistoryAsk::ClearRemovedServers)).asking.is_none());
        assert!(!said(&the_history_says(&left)).contains("clearRemovedServersHistory"));
        drop(started);
        let _ = std::fs::remove_dir_all(&path);
    }

    /// **The page draws the *Removed servers* group as the drawing does, and
    /// its fold is the page's for the run** — `web/the-history.js` run under
    /// Node over the queue harness's stand-ins. The removed rows go into one
    /// group at the foot under its heading and count; the clearing control sits
    /// under the heading and asks for `Queue & History` X6b's box; a press folds and unfolds it,
    /// folded the heading and control stay, and the fold survives a redraw;
    /// with no removed row there is no group. The entry's line reads *…,
    /// removed*. The eighteenth notes, notes 12 and 13. **Skipped, and says so,
    /// where no Node is installed.**
    #[test]
    fn the_page_draws_the_removed_servers_group_and_folds_it_for_the_run() {
        let folder = std::env::temp_dir().join(format!("smoozy-history-removed-page-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a scratch folder");
        let web = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        for page in ["the-queue.js", "the-history.js", "the-value-drawn.js"] {
            std::fs::copy(web.join(page), folder.join(page)).expect("the page is copied");
        }
        for (name, text) in crate::surface::the_queue::tests::THE_BOXES_STAND_INS {
            let text = if name == "harness.js" {
                let (the_document, _) = text.split_once("// ── The queue ──").expect("the harness's document half");
                format!("{the_document}{THE_REMOVED_GROUP_STEPS}")
            } else {
                text.to_string()
            };
            std::fs::write(folder.join(name), text).expect("a stand-in is written");
        }
        let ran = std::process::Command::new("node").arg(folder.join("harness.js")).current_dir(&folder).output();
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so History's page was not run");
            let _ = std::fs::remove_dir_all(&folder);
            return;
        };
        let _ = std::fs::remove_dir_all(&folder);
        assert!(ran.status.success(), "the page did not run: {}", String::from_utf8_lossy(&ran.stderr));
        let said = String::from_utf8_lossy(&ran.stdout).trim().to_string();
        let expected = [
            "rows outside the group: Kitchen",
            "the group: Removed servers 2 entries, open true",
            "rows inside the group: Turntable Night",
            "the control: Clear removed servers\u{2019} history",
            "asked: clear-removed-servers",
            "after a press: open false",
            "after a redraw: open false",
            "after a second press: open true",
            "with no removed row: 0 groups",
            "the line under the name: Lambda A100, removed",
        ]
        .join("\n");
        assert_eq!(said, expected);
    }

    /// **Step 11's D6b: the two clearing boxes that list servers list them
    /// between their two sentences**, as the drawing's `hclearallmodal` and
    /// `hclearremovedmodal` do — what goes, then whose, then what is
    /// untouched. Until then the shared box drew both sentences and the list
    /// after them. The boxes are the real ones, composed and delivered as the
    /// application posts them, and drawn by `web/the-queue.js` under Node.
    #[test]
    fn the_clearing_boxes_list_their_servers_between_the_two_sentences() {
        use std::sync::{Arc, Mutex};
        struct Held(Arc<Mutex<Vec<crate::events::Delivery>>>);
        impl crate::events::Delivers for Held {
            fn deliver(&self, delivery: &crate::events::Delivery) {
                self.0.lock().expect("held").push(delivery.clone());
            }
        }
        let delivered = |asking: TheBoxThatAsks| -> serde_json::Value {
            let seen = Arc::new(Mutex::new(Vec::new()));
            let queue = TheOneQueue::new();
            queue.the_page_is_ready(Box::new(Held(seen.clone())));
            let _ = queue.read_then_post::<(), std::convert::Infallible>(
                || Ok(()),
                |_| crate::surface::the_queue::the_box_is_said(Event::about(About::TheRun, THE_HISTORY_SAYS), Some(&asking)),
            );
            let seen = seen.lock().expect("held");
            serde_json::to_value(&seen[0]).expect("a delivery serialises")["said"].clone()
        };
        let per_server = vec![("RunPod 4090".to_string(), 41), ("Lambda A100".to_string(), 9)];
        let boxes = serde_json::json!({
            "clearAll": delivered(the_clear_all_box(50, &per_server)),
            "clearRemoved": delivered(the_clear_removed_box(50, &per_server)),
        });

        let folder = std::env::temp_dir().join(format!("smoozy-history-clearing-boxes-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a scratch folder");
        let web = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        for page in ["the-queue.js", "the-history.js", "the-value-drawn.js"] {
            std::fs::copy(web.join(page), folder.join(page)).expect("the page is copied");
        }
        for (name, text) in crate::surface::the_queue::tests::THE_BOXES_STAND_INS {
            let text = if name == "harness.js" {
                let (the_document, _) = text.split_once("// ── The queue ──").expect("the harness's document half");
                format!("{the_document}const BOXES = {boxes};\n{THE_CLEARING_BOX_STEPS}")
            } else {
                text.to_string()
            };
            std::fs::write(folder.join(name), text).expect("a stand-in is written");
        }
        let ran = std::process::Command::new("node").arg(folder.join("harness.js")).current_dir(&folder).output();
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the clearing boxes were not drawn");
            let _ = std::fs::remove_dir_all(&folder);
            return;
        };
        let _ = std::fs::remove_dir_all(&folder);
        assert!(ran.status.success(), "the page did not run: {}", String::from_utf8_lossy(&ran.stderr));
        let said = String::from_utf8_lossy(&ran.stdout).trim().to_string();
        assert_eq!(
            said,
            ["clear-all: h3 p dl p div", "clear-removed: h3 p dl p div"].join("\n"),
            "the list is not between the two sentences"
        );
    }

    /// The steps [`the_clearing_boxes_list_their_servers_between_the_two_sentences`]
    /// runs after the harness's document half.
    const THE_CLEARING_BOX_STEPS: &str = r#"
const history = anElement("div");
globalThis.theRoot.appendChild(history);
globalThis.screens.get("historyview").itIsDrawn(history);
const HISTORY = "history:the-history";
const aHistory = (more) => Object.assign({
  theHeading: "History", hrowCount: "1", hrow0Job: "j1", hrow0Name: "Kitchen", hrowChosen: "j1", entryJob: "j1",
}, more || {});
for (const which of ["clearAll", "clearRemoved"]) {
  tell(HISTORY, aHistory(BOXES[which]));
  const modal = history.querySelector(".modal");
  const box = modal.querySelector(".mbox");
  console.log(modal.dataset.asks + ": " + box.children.map((child) => child.tag).join(" "));
  modal.parentNode.removeChild(modal);
}
"#;

    /// The steps [`the_page_draws_the_removed_servers_group_and_folds_it_for_the_run`]
    /// runs after the harness's document half.
    const THE_REMOVED_GROUP_STEPS: &str = r#"
// The harness's half above has loaded both pages and defined `tell`. The
// pages hold the shell's own object, so its ask is watched from here.
const asked = [];
globalThis.window.__TAURI__.core.invoke = (command, args) => {
  if (command === "the_history" && args && args.asking) asked.push(args.asking);
  if (command === "the_pane_layout") return Promise.resolve({ seams: [] });
  return Promise.resolve(null);
};
const words = (element) => element.text + element.children.map(words).join("");
const history = anElement("div");
globalThis.theRoot.appendChild(history);
globalThis.screens.get("historyview").itIsDrawn(history);
const HISTORY = "history:the-history";
const aHistory = (more) => Object.assign({
  theHeading: "History", hrowCount: "3",
  hrow0Job: "j1", hrow0Name: "Kitchen",
  hrow1Job: "j2", hrow1Name: "Turntable", hrow1Removed: "yes", hrow1Badge: "brm",
  hrow2Job: "j3", hrow2Name: "Night", hrow2Removed: "yes", hrow2Badge: "brm",
  theRemovedServers: "Removed servers", theRemovedFigure: "2", theRemovedWord: "entries",
  clearRemovedServersHistory: "Clear removed servers’ history",
  hrowChosen: "j2", entryJob: "j2", entryName: "Turntable", entryServer: "Lambda A100", entryServerRemoved: ", removed",
}, more || {});
tell(HISTORY, aHistory());
const scroll = history.querySelector(".qscroll");
const outside = scroll.children.filter((child) => child.classList && child.classList.contains("hrow"));
console.log("rows outside the group: " + outside.map((row) => words(row.querySelector(".hn"))).join(" "));
let group = scroll.querySelector("section.hgone");
const heading = group.querySelector("button.qmach");
console.log("the group: " + words(heading.querySelector("b")) + " " + words(heading.querySelector(".qm2")).replace(" ", " ") + ", open " + group.dataset.open);
console.log("rows inside the group: " + group.querySelector(".qmrows").children.map((row) => words(row.querySelector(".hn"))).join(" "));
const clear = group.querySelector(".hgonehead").querySelector("button.danger");
console.log("the control: " + words(clear));
clear.click();
console.log("asked: " + asked.join(" "));
heading.click();
console.log("after a press: open " + group.dataset.open);
tell(HISTORY, aHistory());
group = history.querySelector("section.hgone");
console.log("after a redraw: open " + group.dataset.open);
group.querySelector("button.qmach").click();
console.log("after a second press: open " + group.dataset.open);
tell(HISTORY, aHistory({ hrowCount: "1", hrowChosen: "j1", entryJob: "j1", entryServerRemoved: undefined }));
console.log("with no removed row: " + history.querySelectorAll("section.hgone").length + " groups");
tell(HISTORY, aHistory());
console.log("the line under the name: " + words(history.querySelector("p.qdsub")).split(" · ")[0]);
"#;

    /// **The Creator's notes on the test copy, piece 8 (note 11): the badge
    /// travels as the drawing's own class.** The record keeps the badge's
    /// number; a bare number is a class no style knows, and the badge drew
    /// empty and still took room. **What it bites on**: the bare number, and a
    /// number outside the eight drawn as a class.
    #[test]
    fn the_badge_is_the_drawings_own_class_and_a_number_outside_the_eight_draws_none() {
        use smoozy_queue_history::record::Hue;
        for number in 1..=8u16 {
            assert_eq!(the_badge_of(Some(Hue(number))), Some(format!("b{number}")));
        }
        for outside in [0u16, 9, 210] {
            assert_eq!(the_badge_of(Some(Hue(outside))), None, "{outside} is not one of the eight");
        }
        assert_eq!(the_badge_of(None), None);
    }

    // ───────────────────────────────────────────────────────────────────────
    // The entry — step 10b piece 6a
    // ───────────────────────────────────────────────────────────────────────

    /// **The entry beside the list is composed, and the delete box opens over
    /// it. This is the whole of the piece.**
    ///
    /// **Until this landed [`TheHistoryOnScreen::entry`] was set by nothing but
    /// tests**: `what_is_kept` filled the rows and left the entry `None`, so the
    /// entry column was blank, the header's *Delete* was never drawn, and the
    /// delete key opened no box — **while every test of the delete act passed**,
    /// because a test hands the act its job directly. It was found by looking at
    /// the running application and by nothing else.
    #[test]
    fn the_chosen_entry_is_composed_and_the_delete_box_opens_over_it() {
        use crate::record::deletions::tests::a_record_in_history;
        use smoozy_state::identities::IdentityKind;

        let mut path = std::env::temp_dir();
        path.push(format!("smoozy-history-entry-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).unwrap();
        let record = LocalRecord::open_at(&path.join("local-record.sqlite3")).expect("a record");

        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        let store = JobRecords::over(record.connection());
        let mut made = a_record_in_history(&server, "Kitchen — final v3");
        made.server.hue = Some(smoozy_queue_history::record::Hue(5));
        made.blend_path_on_the_machine = Some("/ephemeral/smoozy/assets/kitchen_v13.blend".to_string());
        made.scene_name = Some("Interior".to_string());
        store.write_the_provisional_record(&made).expect("written");
        let job = made.job.as_text();

        let kept = what_is_kept(Some(&record), Some(&job));
        let entry = kept.entry.as_ref().expect("the chosen row has an entry");
        assert_eq!(entry.entry.job, job);
        assert_eq!(entry.entry.name, "Kitchen — final v3");
        assert_eq!(entry.entry.server, "the big box");
        // **No server row stands, so the badge is the grey one** (Design,
        // *History*; the eighteenth notes, piece 7, note 12). It was *b5*,
        // the copied hue, until then.
        assert_eq!(entry.entry.badge.as_deref(), Some("brm"));
        assert_eq!(entry.entry.blend, "/ephemeral/smoozy/assets/kitchen_v13.blend");
        assert_eq!(entry.entry.scene.as_deref(), Some("Interior"));
        // **`Queue & History` N17: no server row stands, so the reach-backs
        // are not offered and the entry says the machine is gone.**
        assert!(!entry.entry.reach_backs);
        assert!(entry.what_deleting_leaves.the_server_is_gone);
        // **Design, *The entry's controls*: a history entry is neither Stop nor
        // Cancel.**
        assert!(entry.entry.stop_or_cancel.is_none());

        // **The box opens by the same route the header's control and the delete
        // key take**, which is the one the running application could not reach.
        let asked = the_history_as_asked(kept, Some(&job), Some(TheHistoryAsk::Delete));
        let box_that_asks = asked.asking.as_ref().expect("the delete box opens");
        assert_eq!(box_that_asks.which, "delete");
        assert_eq!(box_that_asks.go_ahead_act, TheActOnAJob::DeleteTheEntry);
        // **Must-do 10: a count that is not known is never *0 frames* and
        // never *nothing*.** What the box says is the record's deletion, that
        // the server is gone, and the uncounted sentence about what is already
        // on the user's computer.
        assert_eq!(box_that_asks.said.len(), 3);
        assert_eq!(box_that_asks.said[1], vec![APart::words(WHATEVER_IT_RENDERED_IS_ON_A_SERVER)]);
        assert_eq!(box_that_asks.said[2], vec![APart::words(ANYTHING_ALREADY_DOWNLOADED)]);

        // **And the header's *Delete* travels with the screen**, which is what
        // the page hangs the control on.
        let said = said(&the_history_says(&asked));
        assert!(said.contains("entryJob"), "{said}");
        assert!(said.contains("boxWhich"), "{said}");

        drop(record);
        let _ = std::fs::remove_dir_all(&path);
    }

    /// **The status line takes `Queue & History` N6's figure exactly as
    /// recorded, and nothing anywhere adds one to it** (must-do 2).
    ///
    /// The drawing reads *Failed at frame 61* against a recorded 60, which is
    /// the drawing predating `Rendering` A4's correction of 2026-09-02: the
    /// figure **is** the frame Smoozy cannot confirm finished.
    #[test]
    fn the_status_line_takes_the_recorded_figure_and_never_adds_one() {
        let failed = the_status_line(AnEnding::Failed, Some(60), Some("2h 32m"));
        assert_eq!(failed[0], APart::value(FAILED));
        assert_eq!(failed[1], APart::said(FAILED_AT_FRAME, "60", ""));
        assert_eq!(failed[2], APart::said(AFTER_WORD, "2h 32m", ""));
        // **Each clause is dropped where its figure is absent**, and never
        // drawn as a zero.
        assert_eq!(
            the_status_line(AnEnding::Stopped, None, None),
            vec![APart::value(STOPPED)]
        );
        // A clean finish carries no figure at all (`Queue & History` N6), so it says how long.
        assert_eq!(
            the_status_line(AnEnding::Finished, None, Some("5h 08m")),
            vec![APart::value(FINISHED), APart::said(FINISHED_LEAD, "5h 08m", "")]
        );
        // **Not an ending at all**, and nothing this file can compose follows
        // it — the submitted moment is on the *Rendered* group, where this
        // screen's own page writes it as a date.
        assert_eq!(
            the_status_line(AnEnding::NotKnown, None, Some("2h 32m")),
            vec![APart::value(ENDING_NOT_KNOWN)]
        );
        // **Frame 0 is a frame**, not an absence.
        assert_eq!(
            the_status_line(AnEnding::GivenUp, Some(0), None)[1],
            APart::said(FAILED_AT_FRAME, "0", "")
        );
    }

    /// **The mark's verb is the act's** — `Design/Queue & History § The
    /// marks`, added 2026-10-02: a delete, a move and a rename say what they
    /// did, and a record whose act is none of the known ones reads *replaced*,
    /// as every mark did before. Step 10c's Files' owed acts piece 2.
    ///
    /// **And both lines say *may have been* where the act's outcome was never
    /// learned** (`File Browser` W9) — after 0.1.8, the checks' leftovers,
    /// item 5: the check before 0.1.8 read *may have been replaced.* over a
    /// line saying the file *was replaced after this job was submitted.*
    #[test]
    fn the_marks_verb_is_the_acts() {
        use smoozy_queue_history::record::{ADisturbance, DisturbanceReading};
        use smoozy_state::identities::IdentityKind;
        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        for (act, reading, said, after) in [
            ("delete", DisturbanceReading::Disturbed, A_FILE_THIS_JOB_NEEDS_WAS_DELETED, WAS_DELETED_AFTER_SUBMISSION),
            ("move", DisturbanceReading::MayHaveBeenDisturbed, IT_MAY_HAVE_BEEN_MOVED, MAY_HAVE_BEEN_MOVED_AFTER_SUBMISSION),
            ("rename", DisturbanceReading::Disturbed, A_FILE_THIS_JOB_NEEDS_WAS_RENAMED, WAS_RENAMED_AFTER_SUBMISSION),
            ("replace", DisturbanceReading::Disturbed, A_FILE_THIS_JOB_NEEDS_WAS_REPLACED, WAS_REPLACED_AFTER_SUBMISSION),
            ("copy", DisturbanceReading::Disturbed, A_FILE_THIS_JOB_NEEDS_WAS_REPLACED, WAS_REPLACED_AFTER_SUBMISSION),
            ("an upload request", DisturbanceReading::MayHaveBeenDisturbed, IT_MAY_HAVE_BEEN_REPLACED, MAY_HAVE_BEEN_REPLACED_AFTER_SUBMISSION),
            ("delete", DisturbanceReading::MayHaveBeenDisturbed, IT_MAY_HAVE_BEEN_DELETED, MAY_HAVE_BEEN_DELETED_AFTER_SUBMISSION),
            ("rename", DisturbanceReading::MayHaveBeenDisturbed, IT_MAY_HAVE_BEEN_RENAMED, MAY_HAVE_BEEN_RENAMED_AFTER_SUBMISSION),
        ] {
            let mut record = crate::record::deletions::tests::a_record_in_history(&server, "Kitchen");
            record.submitted_with_no_scene_named = false;
            record.disturbances = vec![ADisturbance {
                the_member: "textures/wood_oak.png".to_string(),
                the_act_intended: act.to_string(),
                reading,
                how_many: None,
            }];
            let marks = the_marks_on(&record, false);
            assert_eq!(marks[0].said, said, "{act}");
            assert_eq!(marks[0].parts, vec![APart::said("", "textures/wood_oak.png", after)], "{act}");
        }
    }

    /// **One delete that took more than one file from a job's folder is one
    /// mark: the count and the folder** — the build runner's ruling (d) of
    /// 2026-10-03, `Design/Queue & History § The marks`. One file is named as
    /// every other mark names its member.
    #[test]
    fn a_delete_of_many_files_is_one_mark_with_the_count_and_the_folder() {
        use smoozy_queue_history::record::{ADisturbance, DisturbanceReading};
        use smoozy_state::identities::IdentityKind;
        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        let mut record = crate::record::deletions::tests::a_record_in_history(&server, "Kitchen");
        record.submitted_with_no_scene_named = false;
        record.disturbances = vec![
            ADisturbance {
                the_member: "/root/job/frames/".to_string(),
                the_act_intended: "delete".to_string(),
                reading: DisturbanceReading::Disturbed,
                how_many: Some(12),
            },
            ADisturbance {
                the_member: "/root/job/frames/0001.png".to_string(),
                the_act_intended: "delete".to_string(),
                reading: DisturbanceReading::Disturbed,
                how_many: None,
            },
            ADisturbance {
                the_member: "/root/job/frames/".to_string(),
                the_act_intended: "delete".to_string(),
                reading: DisturbanceReading::MayHaveBeenDisturbed,
                how_many: Some(3),
            },
        ];
        let marks = the_marks_on(&record, false);
        assert_eq!(marks.len(), 3);
        assert_eq!(marks[0].said, FILES_THIS_JOB_NEEDS_WERE_DELETED);
        assert_eq!(
            marks[0].parts,
            vec![
                APart::said("", "12", FILES_WERE_DELETED_FROM),
                APart::said("", "/root/job/frames/", AFTER_THIS_JOB_WAS_SUBMITTED),
            ]
        );
        assert_eq!(marks[1].said, A_FILE_THIS_JOB_NEEDS_WAS_DELETED);
        assert_eq!(marks[1].parts, vec![APart::said("", "/root/job/frames/0001.png", WAS_DELETED_AFTER_SUBMISSION)]);
        assert_eq!(marks[2].said, FILES_THIS_JOB_NEEDS_MAY_HAVE_BEEN_DELETED);
        assert_eq!(marks[2].parts[0], APart::said("", "3", FILES_MAY_HAVE_BEEN_DELETED_FROM));
    }

    /// **The marks — Design, *The marks*: each shown only when it is true.**
    #[test]
    fn the_marks_are_composed_from_the_record_and_each_is_shown_only_when_true() {
        use smoozy_queue_history::acting::note::AnAttemptMark;
        use smoozy_queue_history::record::{
            ADisturbance, AnAttemptWhereTheFileChanged, CarriesOnFrom,
            DependencyCheckAtSubmission, DisturbanceReading,
            WhatBecameOfTheFramesAlreadyFinished,
        };
        use smoozy_state::identities::IdentityKind;

        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        let plain = {
            let mut one = crate::record::deletions::tests::a_record_in_history(&server, "Kitchen");
            one.submitted_with_no_scene_named = false;
            one
        };
        // **Most jobs carry none of this and the entry shows none of it.**
        assert!(the_marks_on(&plain, false).is_empty());

        let mut record = plain.clone();
        record.disturbances.push(ADisturbance {
            the_member: "textures/wood_oak.png".to_string(),
            the_act_intended: "replace".to_string(),
            reading: DisturbanceReading::MayHaveBeenDisturbed,
            how_many: None,
        });
        record.dependency_check = Some(DependencyCheckAtSubmission::NoStandingAtAll);
        record.carries_on_from = Some(CarriesOnFrom {
            earlier_job: Identity::issue(IdentityKind::Job).unwrap(),
            the_label_as_it_stood: "Kitchen — final v2".to_string(),
        });
        record.submitted_with_no_scene_named = true;
        record.scene_name = Some("Interior".to_string());
        record.attempts_where_the_file_changed = vec![AnAttemptWhereTheFileChanged {
            attempt: 2,
            what_became_of_the_frames_already_finished:
                WhatBecameOfTheFramesAlreadyFinished::TheyWereReused,
        }];
        record.what_the_note_carried.re_run_marks = vec![AnAttemptMark::for_a_re_run(2, &[45, 46])];

        let marks = the_marks_on(&record, false);
        let said: Vec<&str> = marks.iter().map(|mark| mark.said).collect();
        assert_eq!(
            said,
            vec![
                IT_MAY_HAVE_BEEN_REPLACED,
                NOTHING_CHECKED_THIS_FILES_DEPENDENCIES,
                CARRIES_ON_FROM,
                SUBMITTED_WITH_NO_SCENE_NAMED,
                THE_FILE_CHANGED_BEFORE_ATTEMPT,
                INTERRUPTED_AND_RE_RUN,
            ]
        );
        // **`Queue & History` N22: the mark does not go once the scene is known, and it says
        // which scene it turned out to be.**
        assert_eq!(
            marks[3].parts,
            vec![APart::said(BLENDER_RENDERED_LEAD, "Interior", THE_FILES_ACTIVE_SCENE)]
        );
        // **`Queue & History` N9a: which of the two happened, and never the wrong one.**
        assert_eq!(marks[4].parts[1], APart::words(FRAMES_ALREADY_FINISHED_WERE_REUSED));
        // **`Queue & History` N1's re-run mark names the frame the re-run proceeded from.**
        assert_eq!(marks[5].parts, vec![APart::said(FROM_FRAME, "45", "")]);
        // **`Queue & History` N20: the way to the earlier entry is offered
        // where that entry stands**, on the carry-on mark alone, and not where
        // it is gone (the 38's piece 2).
        assert!(marks.iter().all(|mark| mark.way.is_none()));
        let offered = the_marks_on(&record, true);
        assert_eq!(offered[2].way, Some(crate::surface::the_queue::THAT_JOB));
        assert_eq!(offered.iter().filter(|mark| mark.way.is_some()).count(), 1);
        // **A mark whose attempt list was empty carries no frame**, which is
        // not a zero.
        let mut empty = plain.clone();
        empty.what_the_note_carried.re_run_marks = vec![AnAttemptMark::for_a_re_run(2, &[])];
        assert!(the_marks_on(&empty, false)[0].parts.is_empty());
    }

    /// **Held-job piece B: `Queue & History` N23's field draws Design's
    /// seventh mark** — ***The NVIDIA driver, 525.147.05, was too old for
    /// OptiX in Blender 5.1**, and could not be updated.* — the job's
    /// Blender named by its line, the two values inside the bold, the clause
    /// after it outside. **Where the driver gave no version** the sentence
    /// loses only the value. **No field, no mark.**
    #[test]
    fn n23s_field_draws_the_seventh_mark() {
        use smoozy_queue_history::record::TheDriverWasBelowTheBar;
        use smoozy_state::identities::IdentityKind;
        // **The sentence as the page sets it**: the parts in order, a space
        // between, none before words that cling.
        let read_whole = |mark: &AMarkOnTheEntry| {
            let mut whole = mark.said.to_string();
            for part in &mark.parts {
                for (piece, clings) in [
                    (part.lead.to_string(), false),
                    (part.value.clone(), false),
                    (part.trail.to_string(), crate::surface::the_queue::the_trail_clings(part)),
                ] {
                    if piece.is_empty() {
                        continue;
                    }
                    if !clings && !whole.is_empty() {
                        whole.push(' ');
                    }
                    whole.push_str(&piece);
                }
            }
            whole
        };
        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        let mut record = crate::record::deletions::tests::a_record_in_history(&server, "Kitchen");
        record.submitted_with_no_scene_named = false;
        record.blender.pick.version = "5.1.0".to_string();
        assert!(the_marks_on(&record, false).is_empty(), "no field, no mark");

        record.the_driver_was_below_the_bar = Some(TheDriverWasBelowTheBar {
            the_drivers_version: Some("525.147.05".to_string()),
            the_jobs_blender: "5.1.0".to_string(),
        });
        let marks = the_marks_on(&record, false);
        assert_eq!(marks.len(), 1, "{marks:?}");
        assert_eq!(
            read_whole(&marks[0]),
            "The NVIDIA driver, 525.147.05, was too old for OptiX in Blender 5.1, and could not be updated."
        );
        assert_eq!(marks[0].bold, 2, "the two values sit inside the bold");
        assert_eq!(marks[0].parts[0].value, "525.147.05");
        assert_eq!(marks[0].parts[1].value, "5.1");

        record.the_driver_was_below_the_bar = Some(TheDriverWasBelowTheBar {
            the_drivers_version: None,
            the_jobs_blender: "5.1.0".to_string(),
        });
        let marks = the_marks_on(&record, false);
        assert_eq!(
            read_whole(&marks[0]),
            "The NVIDIA driver was too old for OptiX in Blender 5.1, and could not be updated."
        );
        assert_eq!(marks[0].bold, 1);

        // **From 5.3 the mark says what stops there** (`Rendering` L23a's bullet
        // of 2026-11-02, the live pass's own figures).
        record.the_driver_was_below_the_bar = Some(TheDriverWasBelowTheBar {
            the_drivers_version: Some("575.64.03".to_string()),
            the_jobs_blender: "5.3.0".to_string(),
        });
        assert_eq!(
            read_whole(&the_marks_on(&record, false)[0]),
            "The NVIDIA driver, 575.64.03, was too old for Cycles GPU rendering in Blender 5.3, and could not be updated."
        );
        record.the_driver_was_below_the_bar = Some(TheDriverWasBelowTheBar {
            the_drivers_version: None,
            the_jobs_blender: "5.3.0".to_string(),
        });
        assert_eq!(
            read_whole(&the_marks_on(&record, false)[0]),
            "The NVIDIA driver was too old for Cycles GPU rendering in Blender 5.3, and could not be updated."
        );
        record.the_driver_was_below_the_bar = Some(TheDriverWasBelowTheBar {
            the_drivers_version: None,
            the_jobs_blender: "5.1.0".to_string(),
        });

        // **On the entry both screens draw**, and on the event.
        let beside = BesideTheRecord::default();
        let entry = the_entry_for(&record, &beside, false, true).entry;
        assert_eq!(entry.marks.len(), 1);
    }

    /// **The setup group, and the three lines the kept setup would have
    /// filled.**
    ///
    /// **`Queue & History` N14's companion is the *file's* value for a setting
    /// the user overrode**, so it can never stand as the value in force — doing
    /// so would show the value the job did **not** run with, on exactly the
    /// settings the user changed.
    #[test]
    fn the_setup_says_not_known_where_the_record_has_no_half_and_never_shows_the_files_value_instead()
    {
        use smoozy_queue_history::record::AnswerOrigin;
        use smoozy_render_settings::payload::{Companion, TheFilesFrameRange};
        use smoozy_state::identities::IdentityKind;

        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        let mut record = crate::record::deletions::tests::a_record_in_history(&server, "Kitchen");
        record.resolution_percentage_typed = Some(100);
        record.picture_size_from_the_read = Some((3840, 2160));
        record.main_output_from_the_read = Some("kitchen-final-v3-p8w2ktr6/".to_string());
        record.blender.origin = AnswerOrigin::TheUsersOwnOverride;
        record.the_files_values_beside_the_overrides = Some(Companion {
            frame_range: Some(TheFilesFrameRange { start: 1, end: 240, step: 1 }),
            sample_count: Some(512),
            resolution_percentage: Some(50),
            ..Companion::default()
        });

        // **No kept setup read back**: nothing says whether the user answered
        // the three it would have held, so they are drawn in neither face.
        let facts = the_setup_on(&record, None, None);
        let by_name = |name: &str| {
            facts
                .iter()
                .find(|fact| fact.name == name)
                .unwrap_or_else(|| panic!("{name} is drawn"))
        };
        assert_eq!(by_name(BLENDER_FACT).value, "4.5.12");
        // **`Queue & History` N10a: the user's own override takes the user's face.**
        assert!(!by_name(BLENDER_FACT).the_files);
        assert_eq!(by_name(RESOLUTION_FACT).value, "100%");
        assert_eq!(by_name(RESOLUTION_FACT).the_files_own.as_deref(), Some("50%"));
        assert_eq!(by_name(RESOLUTION_FACT).beside.as_deref(), Some("3840 × 2160"));
        assert_eq!(by_name(WRITING_TO_FACT).value, "kitchen-final-v3-p8w2ktr6/");
        // **The three the kept setup would have filled.** The companion holds
        // the file's own 512 samples and its own 1–240 range, and neither is
        // drawn as the value in force.
        for name in [FRAMES_FACT, SAMPLES_FACT, ENGINE_FACT] {
            assert_eq!(by_name(name).value, NOT_KNOWN, "{name}");
            assert!(by_name(name).the_files_own.is_none(), "{name}");
            assert!(by_name(name).neither && !by_name(name).the_files, "{name}");
        }
        assert!(by_name(WRITING_TO_FACT).neither, "Smoozy's own folder takes neither face");
        assert!(
            !facts.iter().any(|fact| fact.value.contains("512")),
            "the file's own sample count is drawn as the value the job ran with"
        );
        // **Where nothing was read there is no full size and no percentage**,
        // and the line says so rather than showing a figure from anywhere else.
        let mut unread = crate::record::deletions::tests::a_record_in_history(&server, "Kitchen");
        unread.the_files_values_beside_the_overrides = None;
        let facts = the_setup_on(&unread, None, None);
        let resolution = facts.iter().find(|fact| fact.name == RESOLUTION_FACT).unwrap();
        assert_eq!(resolution.value, NOT_KNOWN);
        assert!(resolution.beside.is_none());
    }

    /// **`Queue & History` P5b: *Frames* reads exactly as the user typed it,
    /// on History's entry and the queue's alike, and the waiting first-frame
    /// question names the frames the same way** — the eighteenth notes, piece
    /// 7, note 8: *1,2-4* stays *1,2-4*, never *1,2,3,4*. A record with no
    /// typed text draws the numbers form, as before, and the count is the
    /// kept setup's.
    #[test]
    fn the_typed_frames_are_drawn_exactly_as_typed_and_the_numbers_form_without_them() {
        use crate::record::job_records::{JobRecords, TheSubmissionsWriter};
        use smoozy_queue_history::acting::submission::{AProvisionalRecord, WritesTheProvisionalRecord};
        use smoozy_render_settings::file_half::BlenderIdentity;
        use smoozy_render_settings::frames::FrameForm;
        use smoozy_render_settings::payload::{compose, Composition};
        use smoozy_render_settings::store::SceneEntries;
        use smoozy_state::identities::IdentityKind;

        let folder = std::env::temp_dir().join(format!("smoozy-history-typed-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).unwrap();
        let local = LocalRecord::open_at(&folder.join("record")).unwrap();
        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        let payload = compose(Composition {
            version_pick: &BlenderIdentity {
                path: "/root/smoozy/blender/blender-5.2.2/blender".to_string(),
                version: smoozy_server_setup::blender::Version { major: 5, minor: 2, patch: 2 },
            },
            scene: None,
            answers: &SceneEntries {
                frame_range: Some(FrameForm::List(vec![1, 2, 3, 4])),
                ..Default::default()
            },
            standing: None,
            stale: false,
        });
        let write = |label: &str, typed: Option<&str>| {
            let record = crate::record::deletions::tests::a_record_in_history(&server, label);
            let provisional = AProvisionalRecord {
                job: record.job.clone(),
                server: server.clone(),
                group: None,
                label: record.label.clone().unwrap_or_default(),
            };
            TheSubmissionsWriter::of(local.connection(), record.clone(), payload.clone())
                .with_the_typed_frames(typed.map(str::to_string))
                .write_it(&provisional)
                .unwrap();
            record
        };
        let typed = write("typed", Some("1,2-4"));
        let untyped = write("untyped", None);
        let frames_of = |record: &LocalJobRecord| {
            let beside = BesideTheRecord::read(local.connection(), &record.job);
            let entry = the_entry_for(record, &beside, false, true);
            entry.entry.facts.into_iter().find(|fact| fact.name == FRAMES_FACT).expect("Frames is drawn")
        };
        let frames = frames_of(&typed);
        assert_eq!(frames.value, "1,2-4");
        assert!(!frames.the_files && !frames.neither, "a typed range is the user's, upright and bold");
        assert_eq!(frames_of(&untyped).value, "1,2,3,4", "no typed text draws the numbers form");

        // **The waiting first-frame question and the pair's row, on the held
        // range job.**
        let submitted = FrameForm::List(vec![1, 2, 3, 4]);
        let typed_text = JobRecords::over(local.connection()).the_typed_frames_of(&typed.job).unwrap();
        assert_eq!(crate::surface::the_queue::the_full_ranges_frames(typed_text, &submitted), "1,2-4");
        assert_eq!(crate::surface::the_queue::the_full_ranges_frames(None, &submitted), "1,2,3,4");
        drop(local);
        let _ = std::fs::remove_dir_all(folder);
    }

    #[test]
    fn only_a_cycles_job_states_what_it_was_asked_to_render_on() {
        // **`Queue & History` E12a's bullet of 2026-11-02**, the live pass on
        // real production files: an EEVEE job seen on the card read *Renders
        // on CPU*, from the file's saved Cycles device.
        use crate::record::job_records::TheKeptSetup;
        use smoozy_queue_history::record::TheDeviceAsk;
        use smoozy_render_settings::device::DeviceChoice;
        use smoozy_render_settings::engine::{EngineEntry, OfferedEngine};
        use smoozy_render_settings::file_half::BlenderIdentity;
        use smoozy_render_settings::payload::{compose, Companion, Composition};
        use smoozy_render_settings::store::SceneEntries;
        use smoozy_state::identities::IdentityKind;

        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        let mut record = crate::record::deletions::tests::a_record_in_history(&server, "a job");
        let the_payload = |engine: Option<OfferedEngine>| {
            let mut payload = compose(Composition {
                version_pick: &BlenderIdentity {
                    path: "/root/smoozy/blender/5.3/blender".to_string(),
                    version: smoozy_server_setup::blender::Version { major: 5, minor: 3, patch: 0 },
                },
                scene: None,
                answers: &SceneEntries::default(),
                standing: None,
                stale: false,
            });
            payload.engine = engine.map(|pick| EngineEntry { pick, identifier: None });
            TheKeptSetup::Kept(payload)
        };
        let a_companion = |engine: &str| Companion {
            engine: Some(smoozy_render_settings::names::ExactName::as_the_reading_produced_it(engine)),
            ..Companion::default()
        };
        let the_row = |record: &LocalJobRecord, kept: &TheKeptSetup| {
            the_setup_on(record, Some(kept), None)
                .into_iter()
                .find(|fact| fact.name == RENDERS_ON_FACT)
                .map(|fact| fact.value)
        };
        for ask in [
            TheDeviceAsk::TheFilesOwnAsTheReadingInForceGaveIt(DeviceChoice::TheProcessor),
            TheDeviceAsk::TheFilesOwnAsTheReadingInForceGaveIt(DeviceChoice::TheCards),
            TheDeviceAsk::TheUsersOwn(DeviceChoice::TheCards),
        ] {
            record.device_asked_for = ask;
            // **The file's own EEVEE, the user's EEVEE, a third engine: no row.**
            record.the_files_values_beside_the_overrides = Some(a_companion("BLENDER_EEVEE"));
            assert_eq!(the_row(&record, &the_payload(None)), None, "{ask:?}");
            record.the_files_values_beside_the_overrides = Some(a_companion("BLENDER_EEVEE_NEXT"));
            assert_eq!(the_row(&record, &the_payload(None)), None, "{ask:?}");
            record.the_files_values_beside_the_overrides = Some(a_companion("CYCLES"));
            assert_eq!(the_row(&record, &the_payload(Some(OfferedEngine::Eevee))), None, "{ask:?}");
            record.the_files_values_beside_the_overrides = Some(a_companion("BLENDER_WORKBENCH"));
            assert_eq!(the_row(&record, &the_payload(None)), None, "{ask:?}");
            // **Cycles keeps it, the user's and the file's alike.**
            record.the_files_values_beside_the_overrides = Some(a_companion("CYCLES"));
            assert!(the_row(&record, &the_payload(None)).is_some(), "{ask:?}");
            record.the_files_values_beside_the_overrides = Some(a_companion("BLENDER_EEVEE"));
            assert!(the_row(&record, &the_payload(Some(OfferedEngine::Cycles))).is_some(), "{ask:?}");
            // **The engine not known: as it was.**
            record.the_files_values_beside_the_overrides = None;
            assert!(the_row(&record, &the_payload(None)).is_some(), "{ask:?}");
        }
    }

    /// **Step 10c's sixth pass, fault 6: *Submitted with* draws what the user
    /// gave.** The pass's second job was submitted on an unread file with 1,024
    /// samples typed and *CPU* pressed; the manifest carried both, and History
    /// read *Samples — not known* in bold under *bold = yours*. **The kept
    /// setup had been written beside the record all along**, and nothing drew
    /// from it.
    #[test]
    fn the_setup_draws_the_users_own_answers_from_the_kept_setup() {
        use crate::record::job_records::TheKeptSetup;
        use smoozy_queue_history::record::TheDeviceAsk;
        use smoozy_render_settings::device::DeviceChoice;
        use smoozy_render_settings::file_half::BlenderIdentity;
        use smoozy_render_settings::payload::{compose, Companion, Composition};
        use smoozy_render_settings::store::SceneEntries;
        use smoozy_state::identities::IdentityKind;

        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        let mut record = crate::record::deletions::tests::a_record_in_history(&server, "p12v6 cube cpu");
        record.the_files_values_beside_the_overrides = None;
        record.device_asked_for = TheDeviceAsk::TheUsersOwn(DeviceChoice::TheProcessor);
        let answers = SceneEntries {
            sample_count: Some(1024),
            device: Some(DeviceChoice::TheProcessor),
            ..Default::default()
        };
        let payload = compose(Composition {
            version_pick: &BlenderIdentity {
                path: "/root/smoozy/blender/82382b72-blender-5.2.2/blender".to_string(),
                version: smoozy_server_setup::blender::Version { major: 5, minor: 2, patch: 2 },
            },
            scene: None,
            answers: &answers,
            standing: None,
            stale: false,
        });
        let kept = TheKeptSetup::Kept(payload);

        let facts = the_setup_on(&record, Some(&kept), None);
        let by_name = |name: &str| {
            facts
                .iter()
                .find(|fact| fact.name == name)
                .unwrap_or_else(|| panic!("{name} is drawn"))
        };
        // **The user's own**, upright and bold, with no file value beside it
        // because no reading stood at submission.
        let samples = by_name(SAMPLES_FACT);
        assert_eq!(samples.value, "1024");
        assert!(!samples.the_files && !samples.neither, "1024 was typed, so it is the user's");
        assert!(samples.the_files_own.is_none());
        // **`Queue & History` E12a: the ask, whatever the ending.**
        let device = by_name(RENDERS_ON_FACT);
        assert_eq!(device.value, "CPU");
        assert!(!device.the_files && !device.neither, "CPU was pressed, so it is the user's");
        // **The settings the user left alone are the file's, and not known.**
        for name in [FRAMES_FACT, ENGINE_FACT] {
            assert_eq!(by_name(name).value, NOT_KNOWN, "{name}");
            assert!(by_name(name).the_files && !by_name(name).neither, "{name} is the file's");
        }

        // **With a reading at submission, the file's value stands beside the
        // user's** — `Queue & History` N14 — and is never drawn as the value.
        record.the_files_values_beside_the_overrides = Some(Companion {
            sample_count: Some(16),
            device: Some(smoozy_render_settings::reading::DeviceSetting::TheCards),
            ..Companion::default()
        });
        let facts = the_setup_on(&record, Some(&kept), None);
        let samples = facts.iter().find(|fact| fact.name == SAMPLES_FACT).unwrap();
        assert_eq!(samples.value, "1024");
        assert_eq!(samples.the_files_own.as_deref(), Some("16"));
        let device = facts.iter().find(|fact| fact.name == RENDERS_ON_FACT).unwrap();
        assert_eq!(device.the_files_own.as_deref(), Some("GPU"));

        // **Through the entry, and so through History and the queue alike.**
        let entry = the_entry_for(
            &record,
            &BesideTheRecord {
                the_setup: Some(kept.clone()),
                ..BesideTheRecord::default()
            },
            false,
            true,
        );
        assert!(entry
            .entry
            .facts
            .iter()
            .any(|fact| fact.name == SAMPLES_FACT && fact.value == "1024"));

        // **The page draws the neither face plain**, so nothing that is not
        // the user's reads as *bold = yours*.
        let page = include_str!("../../web/the-queue.js");
        assert!(page.contains(r#"said["fact" + at + "Neither"]"#));
    }

    /// **The Creator's second notes, piece 16, and the fourth notes' note 4:
    /// *Submitted with* names the file's own frames and engine.** The way the
    /// Creator's jobs were submitted: *Read the file* on a file of several
    /// scenes, one scene ticked, the samples and the percentage typed, the
    /// frames and the engine left alone. **Until this piece the job read
    /// *Frames — not known* and *Engine — not known***, though the reading on
    /// the page named both: `Render Settings` Y1a kept the file's value only
    /// beside an override, so a setting left alone kept nothing, and nothing
    /// else on the record holds it.
    ///
    /// Composed by the real composition, written and read back through the
    /// record, and drawn — so it covers the record's half as well as the
    /// entry's.
    #[test]
    fn the_setup_names_the_files_own_frames_and_engine_where_the_user_left_them_alone() {
        use crate::record::job_records::JobRecords;
        use crate::surface::render_settings::tests::a_scene;
        use crate::surface::render_settings::words as the_page;
        use smoozy_queue_history::record::TheDeviceAsk;
        use smoozy_render_settings::file_half::{BlenderIdentity, FileHalf, FileKey, StartedUnder};
        use smoozy_render_settings::names::{ExactName, NamedAnswer, Provenance};
        use smoozy_render_settings::payload::{compose, the_companion, Composition};
        use smoozy_render_settings::reading::{OfferedEngineIdentifiers, SettingsHalf};
        use smoozy_render_settings::store::SceneEntries;
        use smoozy_state::identities::IdentityKind;

        // **A file of several scenes**, the second one ticked, each with its
        // own range and engine — so a value taken from the wrong scene shows.
        let interior = a_scene("Interior");
        let mut exterior = a_scene("Exterior");
        exterior.frame_start = 10;
        exterior.frame_end = 120;
        exterior.frame_step = 2;
        exterior.engine_as_read = ExactName::as_the_reading_produced_it("BLENDER_EEVEE");
        exterior.sample_counts.eevee = 96;
        exterior.resolution_percentage = 75;
        let picked = BlenderIdentity {
            path: "/root/smoozy/blender/04bef984-blender-5.2.2/blender".to_string(),
            version: smoozy_server_setup::blender::Version { major: 5, minor: 2, patch: 2 },
        };
        let four_values = crate::surface::render_settings::tests::four_values();
        let key = FileKey {
            server: Identity::issue(IdentityKind::LocalServer).unwrap(),
            path: "/home/ubuntu/rendering/mattress.blend".to_string(),
        };
        let mut half = FileHalf::default();
        let _ = half.a_reading_landed(
            key.clone(),
            &StartedUnder {
                version_pick: picked.clone(),
                file_identity: four_values.clone(),
            },
            SettingsHalf {
                scenes: vec![interior, exterior],
                offered_engine_identifiers: OfferedEngineIdentifiers {
                    cycles: Some(ExactName::as_the_reading_produced_it("CYCLES")),
                    eevee: Some(ExactName::as_the_reading_produced_it("BLENDER_EEVEE")),
                },
                announced_add_on_case: None,
                the_folders_at_the_machines_top: None,
            },
            Some(&picked),
            &four_values,
            std::time::SystemTime::now(),
        );
        let reading = half.standing(&key).expect("the reading stands").clone();
        let ticked = NamedAnswer {
            name: ExactName::as_the_reading_produced_it("Exterior"),
            provenance: Provenance::ARecordedReading(reading.identity()),
        };
        let payload = compose(Composition {
            version_pick: &picked,
            scene: Some(&ticked),
            answers: &SceneEntries {
                sample_count: Some(500),
                resolution_percentage: Some(300),
                ..Default::default()
            },
            standing: Some(&reading),
            stale: false,
        });

        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        let mut record = crate::record::deletions::tests::a_record_in_history(&server, "mattress");
        record.scene_name = Some("Exterior".to_string());
        record.submitted_with_no_scene_named = false;
        record.resolution_percentage_typed = Some(300);
        record.device_asked_for = TheDeviceAsk::TheFilesOwnAsTheReadingInForceGaveIt(
            smoozy_render_settings::device::DeviceChoice::TheCards,
        );
        record.the_files_values_beside_the_overrides =
            the_companion(&payload, Some(&reading), payload.scene.as_ref().map(|scene| &scene.name));

        let connection = crate::record::deletions::tests::a_record_store();
        let store = JobRecords::over(&connection);
        store
            .write_the_provisional_record_and_its_kept_setup(&record, &payload)
            .unwrap();
        let held = store.read(&record.job).unwrap().expect("the record stands");
        let kept = store.the_kept_setup_of(&record.job).unwrap();

        let facts = the_setup_on(&held, kept.as_ref(), None);
        let by_name = |name: &str| {
            facts
                .iter()
                .find(|fact| fact.name == name)
                .unwrap_or_else(|| panic!("{name} is drawn"))
        };
        // **Left alone: the file's own, in the file's face**, from the ticked
        // scene and never the first one.
        let frames = by_name(FRAMES_FACT);
        assert_eq!(frames.value, "10-120x2", "the file's own range, not *not known*");
        assert!(frames.the_files && !frames.neither && frames.the_files_own.is_none());
        let engine = by_name(ENGINE_FACT);
        assert_eq!(engine.value, the_page::EEVEE, "the file's engine, in Smoozy's word for it");
        assert!(engine.the_files && !engine.neither && engine.the_files_own.is_none());
        // **Typed: the user's, with the file's own beside it**, as before —
        // and the samples beside are the count of the engine in force.
        let samples = by_name(SAMPLES_FACT);
        assert_eq!(samples.value, "500");
        assert_eq!(samples.the_files_own.as_deref(), Some("96"));
        assert!(!samples.the_files && !samples.neither);
        let resolution = by_name(RESOLUTION_FACT);
        assert_eq!(resolution.value, "300%");
        assert_eq!(resolution.the_files_own.as_deref(), Some("75%"));
        assert!(
            !facts.iter().any(|fact| fact.value == NOT_KNOWN && fact.name != WRITING_TO_FACT),
            "nothing the reading named is drawn as not known: {facts:#?}"
        );
    }

    /// **Design, *Where no render began*, the film's first job whole**: no exit
    /// status, a recorded reason and the machine's word that no render began. The
    /// sentence says what happened, the reason stands beneath it at once as a value,
    /// and there is no control to open first. **The page draws it** where a second
    /// sentence goes. On an older record the word is absent and today's words stand.
    #[test]
    fn a_job_that_failed_before_any_render_says_so_with_the_reason_beneath() {
        use smoozy_queue_history::acting::note::{TheEndingOnTheNote, TheOutcomeAsWritten};
        use smoozy_state::identities::IdentityKind;

        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        let mut record = crate::record::deletions::tests::a_record_in_history(&server, "shot");
        let reason = "could not copy the bake folder //caches/shots/100_0040/";
        let ending = |no_render_began| TheEndingOnTheNote {
            outcome: TheOutcomeAsWritten::Failed,
            exit_status: None,
            the_attempt_whose_capture_record_holds_the_words: 1,
            the_reason_the_machine_recorded: Some(reason.to_string()),
            no_render_began,
            the_device_line_as_observed: None,
        };
        let beside = BesideTheRecord {
            the_reason_the_machine_recorded: Some(reason.to_string()),
            ..BesideTheRecord::default()
        };
        record.what_the_note_carried.ending = Some(ending(true));
        let entry = the_entry_for(&record, &beside, false, true);
        let failure = entry.failure.clone().expect("a failed ending carries the shape");
        assert_eq!(failure.said, "Failed before rendering.");
        assert_eq!(
            failure.the_reason_beneath.as_deref(),
            Some("Could not copy the bake folder //caches/shots/100_0040/")
        );
        assert_eq!(failure.what_the_server_said, None, "one reason, in one place");
        assert!(failure.the_controls().iter().all(|control| control.act.as_str() != THE_SERVERS_WORDS_ACT));
        let mut kept = TheHistoryOnScreen::default();
        kept.entry = Some(entry);
        let text = said(&the_history_says(&kept));
        assert!(text.contains("failWhatToDoAboutIt"), "{text}");
        assert!(text.contains("Could not copy the bake folder //caches/shots/100_0040/"), "{text}");
        assert!(text.contains("Failed before rendering."), "{text}");

        // **A render that began, or a record from before the word**: today's words.
        record.what_the_note_carried.ending = Some(ending(false));
        let failure = the_entry_for(&record, &beside, false, true).failure.expect("the shape");
        assert_eq!(failure.said, NO_FRAME_WAS_COMPLETED);
        assert_eq!(failure.what_the_server_said.as_deref(), Some(reason));
        assert_eq!(failure.the_reason_beneath, None);
    }

    /// **Step 10c's sixth pass, fault 3: why a job failed reaches the entry,
    /// where the machine's reason has been kept beside the record.** The plain
    /// sentence is unchanged and names no cause; the reason is the raw text
    /// behind *What the server said*, whole, first among the controls, and the
    /// page opens it in the one words panel.
    #[test]
    fn the_reason_the_machine_recorded_is_behind_what_the_server_said() {
        use smoozy_queue_history::acting::note::{TheEndingOnTheNote, TheOutcomeAsWritten};
        use smoozy_state::identities::IdentityKind;

        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        let mut record = crate::record::deletions::tests::a_record_in_history(&server, "p12v6 cube");
        record.what_the_note_carried.ending = Some(TheEndingOnTheNote {
            outcome: TheOutcomeAsWritten::Failed,
            exit_status: None,
            the_attempt_whose_capture_record_holds_the_words: 1,
            the_reason_the_machine_recorded: None,
            no_render_began: false,
            the_device_line_as_observed: None,
        });
        let reason = "Smoozy does not know how to render on a NVIDIA GeForce GTX 1650";
        let entry = the_entry_for(
            &record,
            &BesideTheRecord {
                the_reason_the_machine_recorded: Some(reason.to_string()),
                ..BesideTheRecord::default()
            },
            false,
            true,
        );
        let failure = entry.failure.clone().expect("a failed ending carries the shape");
        assert_eq!(failure.said, NO_FRAME_WAS_COMPLETED, "the sentence names no cause");
        assert!(!failure.said.contains("NVIDIA"));
        assert_eq!(failure.what_the_server_said.as_deref(), Some(reason));
        let controls = failure.the_controls();
        assert_eq!(controls[0].what_it_is, WhatItIs::TheRawText);
        assert_eq!(controls[0].label.as_str(), Prose::new(WHAT_THE_SERVER_SAID).as_str());
        assert_eq!(controls[0].act.as_str(), THE_SERVERS_WORDS_ACT);

        // **On the event, whole, as a value.**
        let mut kept = TheHistoryOnScreen::default();
        kept.entry = Some(entry);
        let text = said(&the_history_says(&kept));
        assert!(text.contains("failTheServersWords"), "{text}");
        assert!(text.contains(reason), "{text}");

        // **Where no reason was kept, nothing is drawn for one**, as before.
        let without = the_entry_for(&record, &BesideTheRecord::default(), false, true);
        let failure = without.failure.expect("the shape");
        assert!(failure.what_the_server_said.is_none());
        assert!(failure
            .the_controls()
            .iter()
            .all(|control| control.act.as_str() != THE_SERVERS_WORDS_ACT));

        // **The page opens it in the one panel and nothing else.**
        let page = include_str!("../../web/the-history.js");
        assert!(page.contains(r#"act !== "what-the-server-said""#));
        assert!(page.contains("theWordsPanel({"));
    }

    /// **`Queue & History` E12: *Ran on* is the device line the note's ending
    /// carried, stated only on a finished ending** — step 10c's fix piece 2,
    /// and the paragraph after `Queue & History` N3.
    ///
    /// **Undone, the finished case fails**: before fix piece 2 the line was
    /// fed nothing and every entry read *Ran on — not known*, on all four
    /// providers. **The failed case is the trap**: a failed job that printed a
    /// line still says *not known*, because a line from a run that did not
    /// finish is not evidence — and the ask never fills it (`Queue & History` E12a).
    #[test]
    fn ran_on_is_the_carried_line_on_a_finished_ending_and_not_known_on_every_other() {
        use smoozy_queue_history::acting::note::{TheEndingOnTheNote, TheOutcomeAsWritten};
        use smoozy_state::identities::IdentityKind;

        let line = "Path tracing on: NVIDIA GeForce RTX 4090 (CUDA)";
        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        let ended = |outcome, the_line: Option<&str>| {
            let mut record =
                crate::record::deletions::tests::a_record_in_history(&server, "Kitchen");
            record.what_the_note_carried.ending = Some(TheEndingOnTheNote {
                outcome,
                exit_status: Some(0),
                the_attempt_whose_capture_record_holds_the_words: 1,
                the_reason_the_machine_recorded: None,
                no_render_began: false,
                the_device_line_as_observed: the_line.map(str::to_string),
            });
            record
        };
        let ran_on = |record: &LocalJobRecord| {
            the_entry_for(record, &BesideTheRecord::default(), false, true)
                .rendered
                .into_iter()
                .find(|fact| fact.name == RAN_ON_FACT)
                .expect("the Rendered group always has the line")
                .parts
        };

        let finished = ran_on(&ended(TheOutcomeAsWritten::Finished, Some(line)));
        assert_eq!(finished, vec![APart::value(line.to_string())], "stated plainly, as printed");

        for not_finished in [
            TheOutcomeAsWritten::Failed,
            TheOutcomeAsWritten::NoStatusWasReaped,
            TheOutcomeAsWritten::LeftStopped,
            TheOutcomeAsWritten::GivenUp,
        ] {
            assert_eq!(
                ran_on(&ended(not_finished, Some(line))),
                vec![APart::words(NOT_KNOWN)],
                "{not_finished:?}: a line from a run that did not finish says nothing"
            );
        }
        // **`Design/Queue & History`, 2026-11-03: a finished job that printed no
        // line has no *Ran on* at all**: nothing was path traced.
        assert!(
            the_entry_for(&ended(TheOutcomeAsWritten::Finished, None), &BesideTheRecord::default(), false, true)
                .rendered
                .into_iter()
                .all(|fact| fact.name != RAN_ON_FACT),
            "a compositor-only frame"
        );
        // **An ending Smoozy never learned has no line to read.**
        let unknown = crate::record::deletions::tests::a_record_in_history(&server, "Kitchen");
        assert_eq!(ran_on(&unknown), vec![APart::words(NOT_KNOWN)]);
    }

    /// **`Design/Queue & History`, *Ran on* (2026-10-06, the third notes,
    /// note 9): the entry names only the backend**, for the research's own
    /// lines, and the whole line for a shape nobody has read.
    ///
    /// **Undone** (the line handed through as printed), every one of the four
    /// research lines reads whole and the first assertion fails.
    #[test]
    fn ran_on_names_only_the_backend_for_the_researchs_lines_and_the_whole_line_otherwise() {
        use smoozy_queue_history::acting::note::{TheEndingOnTheNote, TheOutcomeAsWritten};
        use smoozy_state::identities::IdentityKind;

        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        let ran_on = |the_line: &str| {
            let mut record =
                crate::record::deletions::tests::a_record_in_history(&server, "Kitchen");
            record.what_the_note_carried.ending = Some(TheEndingOnTheNote {
                outcome: TheOutcomeAsWritten::Finished,
                exit_status: Some(0),
                the_attempt_whose_capture_record_holds_the_words: 1,
                the_reason_the_machine_recorded: None,
                no_render_began: false,
                the_device_line_as_observed: Some(the_line.to_string()),
            });
            the_entry_for(&record, &BesideTheRecord::default(), false, true)
                .rendered
                .into_iter()
                .find(|fact| fact.name == RAN_ON_FACT)
                .expect("the Rendered group always has the line")
                .parts
        };

        // The research's lines, word for word (`The machine.md`, *What
        // actually ran — second pass*), and the Creator's own from the note.
        for (line, word) in [
            (
                "Path tracing on: NVIDIA GeForce RTX 4090 (OptiX) [CUDA_NVIDIA GeForce RTX 4090_0000:00:07_OptiX]",
                "OptiX",
            ),
            (
                "Path tracing on: NVIDIA GeForce RTX 5060 (OptiX) [CUDA_NVIDIA GeForce RTX 5060_0000:00:07_OptiX]",
                "OptiX",
            ),
            (
                "Path tracing on: NVIDIA GeForce RTX 4090 (CUDA) [CUDA_NVIDIA GeForce RTX 4090_0000:00:07]",
                "CUDA",
            ),
            (
                "Path tracing on: AMD EPYC 7B13 64-Core Processor (CPU) (61 threads) [CPU]",
                "CPU",
            ),
            (
                "Path tracing on: Apple M1 Pro (GPU - 14 cores) (Metal) (display) [METAL_Apple M1 Pro (GPU - 14 cores)]",
                "Metal",
            ),
        ] {
            assert_eq!(ran_on(line), vec![APart::value(word.to_string())], "{line}");
        }

        // **A shape the research does not hold is drawn whole.**
        for unknown in [
            "Path tracing on: AMD Radeon PRO W7900 (HIP) [HIP_AMD Radeon PRO W7900_0000:03:00]",
            "Path tracing on: NVIDIA GeForce RTX 4090 (CUDA)",
            "Path tracing on: (OptiX) [CUDA_OptiX]",
            "Path tracing on: a processor (CPU) (many threads) [CPU]",
        ] {
            assert_eq!(ran_on(unknown), vec![APart::value(unknown.to_string())], "{unknown}");
        }
    }

    /// **What the entry does not draw, each absent for its own stated reason
    /// rather than because nothing filled it in.**
    #[test]
    fn the_entry_draws_no_live_half_and_no_control_it_cannot_name_a_count_for() {
        use smoozy_state::identities::IdentityKind;

        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        let record = crate::record::deletions::tests::a_record_in_history(&server, "Kitchen");
        let entry = the_entry_for(&record, &BesideTheRecord::default(), false, true);

        // **The two ways to render it again name their own frame counts and
        // the count is the kept setup's**, which record version 23 has no
        // column for — so the group is absent rather than naming a made-up
        // number.
        assert!(entry.again.is_none());
        // **`Desktop app and delivery` M10: an empty panel is never what the
        // user meets**, and with no copy held there is no picture to draw.
        assert!(entry.entry.preview.is_none());
        // **A history entry has no live half at all.**
        assert!(entry.entry.progress.is_none());
        assert!(entry.entry.readings.is_empty());
        assert!(entry.entry.writes.is_empty());
        assert!(entry.entry.held.is_empty());
        assert!(entry.entry.downloading.is_none());
        // **A clean finish carries no failure shape and keeps the standalone
        // control; the three that carry one take it from the shape, never
        // twice.**
        let mut finished = record.clone();
        finished.what_the_note_carried.ending =
            Some(smoozy_queue_history::acting::note::TheEndingOnTheNote {
                outcome: smoozy_queue_history::acting::note::TheOutcomeAsWritten::Finished,
                exit_status: Some(0),
                the_attempt_whose_capture_record_holds_the_words: 1,
                the_reason_the_machine_recorded: None,
                no_render_began: false,
                the_device_line_as_observed: None,
            });
        // **Piece 6d: the control is offered where the words are kept**, and
        // it is History's own — never the shared entry's, which reads a
        // running job's words.
        let words_kept = BesideTheRecord {
            the_outcomes_kept: vec![(
                1,
                crate::presentation::the_machines_own_words::Stream::TheOrdinaryOutput,
                Some(smoozy_runtime::capture::CollectionOutcome::Collected),
            )],
            ..BesideTheRecord::default()
        };
        let none_kept = the_entry_for(&finished, &BesideTheRecord::default(), false, true);
        assert!(!none_kept.blender_output, "no words kept, no control");
        let finished = the_entry_for(&finished, &words_kept, false, true);
        assert!(finished.blender_output);
        assert!(!finished.entry.blender_output);
        assert!(finished.failure.is_none());
        assert_eq!(finished.ending, Some(AnEnding::Finished));
        // **An ending Smoozy never learned carries neither**, and where the
        // server is gone with it the *Rendered* group says so.
        let unknown = the_entry_for(&record, &BesideTheRecord::default(), false, false);
        assert!(!unknown.entry.blender_output);
        assert!(!unknown.blender_output);
        assert!(unknown.failure.is_none());
        assert_eq!(unknown.rendered_note, Some(THE_SERVER_WAS_DESTROYED));
        assert_eq!(the_entry_for(&record, &BesideTheRecord::default(), false, true).rendered_note, None);
    }

    /// **The page takes the job the application says is chosen.**
    ///
    /// **Without it the header's *Delete* and the delete key do nothing until a
    /// row has been clicked.** The page opens holding no job and asks with
    /// none; [`the_row_is_chosen`] falls back to the first row and answers with
    /// that job's identity; and [`the_history_as_asked`] composes a box only
    /// over the job it was named (must-do 1), so an ask carrying none composes
    /// none. **Found by pressing the control in the running application**, and
    /// every test of the box passed throughout, because a test hands it the
    /// identity.
    #[test]
    fn the_page_takes_the_chosen_job_the_application_answers_with() {
        let page = include_str!("../../web/the-history.js");
        assert!(
            page.contains("if (said.hrowChosen) theChosenJob = said.hrowChosen;"),
            "the page holds no job until a row is clicked, so the header's Delete asks with none"
        );
        // **And the application answers with one whenever history has a row**,
        // which is what the page is taking.
        assert!(the_row_is_chosen(
            TheHistoryOnScreen {
                rows: vec![a_row("job-1", "row-1", "the big box")],
                ..TheHistoryOnScreen::default()
            },
            None
        )
        .chosen
        .is_some());
    }

    /// **An absent last-frame figure is two different facts and only one of
    /// them is *none finished*.**
    ///
    /// **Found by looking at the running application**: a job that rendered
    /// every frame it asked for read *Last frame — none finished*, because
    /// `Queue & History` N6 computes the figure only for an ending short of a
    /// clean finish and the line had one word for every absence.
    #[test]
    fn a_clean_finish_never_reads_none_finished_and_an_unlearned_ending_never_does_either() {
        use smoozy_queue_history::acting::note::{TheEndingOnTheNote, TheOutcomeAsWritten};
        use smoozy_state::identities::IdentityKind;

        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        let record = crate::record::deletions::tests::a_record_in_history(&server, "Kitchen");
        let the_last_frame = |entry: &TheHistoryEntry| {
            entry
                .rendered
                .iter()
                .find(|line| line.name == LAST_FRAME_FACT)
                .expect("the last-frame line is drawn")
                .parts
                .clone()
        };

        // **No ending Smoozy ever learned**: there is no figure and none is
        // claimed.
        assert_eq!(
            the_last_frame(&the_entry_for(&record, &BesideTheRecord::default(), false, true)),
            vec![APart::words(NOT_KNOWN)]
        );

        let with = |outcome| {
            let mut one = record.clone();
            one.what_the_note_carried.ending = Some(TheEndingOnTheNote {
                outcome,
                exit_status: Some(0),
                the_attempt_whose_capture_record_holds_the_words: 1,
                the_reason_the_machine_recorded: None,
                no_render_began: false,
                the_device_line_as_observed: None,
            });
            the_entry_for(&one, &BesideTheRecord::default(), false, true)
        };
        // **A clean finish has no figure by `Queue & History` N6's own rule**, and its last frame
        // is the last frame the job asked for — the kept setup's.
        assert_eq!(
            the_last_frame(&with(TheOutcomeAsWritten::Finished)),
            vec![APart::words(NOT_KNOWN)]
        );
        // **An ending short of a clean finish with no figure is a job that
        // finished no frame**, which is the drawing's own wording.
        assert_eq!(
            the_last_frame(&with(TheOutcomeAsWritten::Failed)),
            vec![APart::words(NONE_FINISHED)]
        );
    }

    /// **The Creator's seventh notes, note 3: a clean finish reads the last
    /// frame of the asked order**, which it confirmed with every other frame
    /// (`Design/Queue & History`, the frame count is the last frame Smoozy
    /// confirmed; the drawing's finished entry reads *Last frame 240*). An
    /// order that cannot be counted still reads *not known*, and an ending
    /// short of a clean finish keeps its own figure. **Undone**, the finished
    /// entry reads *not known* and the first assertion fails.
    #[test]
    fn seventh_notes_3_a_clean_finish_reads_the_last_frame_of_the_asked_order() {
        use smoozy_queue_history::acting::note::{TheEndingOnTheNote, TheOutcomeAsWritten};
        use smoozy_queue_history::record::{ARunOfFrames, TheFramesAsked};
        use smoozy_state::identities::IdentityKind;

        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        let mut record = crate::record::deletions::tests::a_record_in_history(&server, "Kitchen");
        record.the_frames.asked = Some(TheFramesAsked {
            runs: vec![ARunOfFrames { first: 1, step: 1, how_many: 240 }],
        });
        let the_last_frame = |one: &LocalJobRecord, outcome| {
            let mut one = one.clone();
            one.what_the_note_carried.ending = Some(TheEndingOnTheNote {
                outcome,
                exit_status: Some(0),
                the_attempt_whose_capture_record_holds_the_words: 1,
                the_reason_the_machine_recorded: None,
                no_render_began: false,
                the_device_line_as_observed: None,
            });
            the_entry_for(&one, &BesideTheRecord::default(), false, true)
                .rendered
                .iter()
                .find(|line| line.name == LAST_FRAME_FACT)
                .expect("the last-frame line is drawn")
                .parts
                .clone()
        };

        assert_eq!(
            the_last_frame(&record, TheOutcomeAsWritten::Finished),
            vec![APart::value("240".to_string())],
            "a clean finish reads the asked order's last frame, plain"
        );
        // **The order's last frame, not its highest** — the frame the job
        // asked for last.
        let mut out_of_order = record.clone();
        out_of_order.the_frames.asked = TheFramesAsked::from_the_order(&[10, 20, 5]);
        assert_eq!(
            the_last_frame(&out_of_order, TheOutcomeAsWritten::Finished),
            vec![APart::value("5".to_string())]
        );
        // **An order that cannot be counted is not known.**
        let mut uncounted = record.clone();
        uncounted.the_frames.they_cannot_be_counted = true;
        assert_eq!(
            the_last_frame(&uncounted, TheOutcomeAsWritten::Finished),
            vec![APart::words(NOT_KNOWN)]
        );
        // **An ending short of a clean finish is untouched**: no figure is
        // still *none finished*, never the asked order's last frame.
        assert_eq!(
            the_last_frame(&record, TheOutcomeAsWritten::Failed),
            vec![APart::words(NONE_FINISHED)]
        );
    }

    /// **The Creator's seventh notes, note 5: History's *Writing to* is the
    /// queue's same link**, offered exactly where *Files on the server* is —
    /// composed the way the application composes History and said the way it
    /// posts it. **Undone**, the connected entry carries no link and the first
    /// assertion fails.
    #[test]
    fn seventh_notes_5_history_writing_to_is_a_link_only_where_files_on_the_server_is() {
        let (path, record, job) =
            a_finished_job_writing_to("seventh-5", Some("/k/kitchen.blend"), Some("/home/ubuntu/Render/kitchen/"));
        let server = crate::record::server_rows::all(record.connection()).unwrap()[0]
            .local_server_identifier
            .clone();
        let here = what_is_kept_over(Some(&record), Some(&job), &|one| one == server);
        let away = what_is_kept_over(Some(&record), Some(&job), &|_| false);
        let _ = std::fs::remove_dir_all(&path);
        let the_link = |kept: &TheHistoryOnScreen| {
            let entry = &kept.entry.as_ref().expect("the entry").entry;
            let at = entry
                .facts
                .iter()
                .position(|fact| fact.name == WRITING_TO_FACT)
                .expect("Writing to is drawn");
            said(&the_history_says(kept)).contains(&format!("fact{at}OpensFiles"))
        };

        assert!(the_link(&here), "History's Writing to is a link where Files on the server is offered");
        assert!(!the_link(&away), "no link where the server is not connected");
    }

    /// **A moment on the *Rendered* group goes out as seconds and the page
    /// writes the date**, which is what every other screen already does.
    ///
    /// **The application cannot write a date on any screen**: the local time
    /// zone is not in the standard library and the manifest is ruled tight, so
    /// no date library may be added. `theMomentReads` is `web/servers.js`'s and
    /// there is one of it.
    #[test]
    fn a_moment_on_the_rendered_group_goes_out_as_seconds_and_the_page_writes_it() {
        use smoozy_state::identities::IdentityKind;

        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        let record = crate::record::deletions::tests::a_record_in_history(&server, "Kitchen");
        let entry = the_entry_for(&record, &BesideTheRecord::default(), false, true);
        let submitted = entry
            .rendered
            .iter()
            .find(|line| line.name == SUBMITTED_FACT)
            .expect("the submitted line is drawn");
        assert!(submitted.a_moment);
        assert_eq!(submitted.parts[0].value, "1754600000");
        // **A line with no moment is not one**, and it says *not known* rather
        // than a zero the page would turn into 1970.
        let ended = entry.rendered.iter().find(|line| line.name == ENDED_FACT).unwrap();
        assert!(!ended.a_moment);
        assert_eq!(ended.parts, vec![APart::words(NOT_KNOWN)]);

        let said = said(&the_history_says(&TheHistoryOnScreen {
            entry: Some(entry),
            ..TheHistoryOnScreen::default()
        }));
        assert!(said.contains("rendered1Moment"), "{said}");

        let page = include_str!("../../web/the-history.js");
        assert!(
            page.contains("Moment\"] && held !== undefined"),
            "the page draws a moment without being told it is one"
        );
        assert!(page.contains("value.textContent = theMomentReads(held)"));
    }

    /// **S3's loose ends, item 1, as the user meets it: a job they stopped
    /// reads in History that the frame it was on *may not be saved*, and never
    /// that it was not.** `Servers` W19a: a stop lands in one of two places and
    /// in one of them the frame is whole, so the flat claim is false there. The
    /// stop box was corrected for this on 2026-09-17 (R1) and this sentence was
    /// missed. **The conditional is the stop box's own**, and this fails if the
    /// two drift apart.
    #[test]
    fn a_stopped_entry_never_claims_the_frame_was_not_saved() {
        use smoozy_queue_history::acting::note::{TheEndingOnTheNote, TheNote, TheOutcomeAsWritten};
        use smoozy_state::identities::{Identity, IdentityKind};

        let mut path = std::env::temp_dir();
        path.push(format!("smoozy-history-stopped-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).unwrap();
        let record = LocalRecord::open_at(&path.join("local-record.sqlite3")).unwrap();
        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        let store = crate::record::job_records::JobRecords::over(record.connection());
        let mut job = crate::record::deletions::tests::a_record_in_history(&server, "Shot 040");
        job.the_render_start_has_been_learned = true;
        store.write_the_provisional_record(&job).unwrap();
        let mut note = TheNote::created_with_the_job_record("Shot 040");
        note.ending = Some(TheEndingOnTheNote {
            outcome: TheOutcomeAsWritten::LeftStopped,
            exit_status: None,
            the_attempt_whose_capture_record_holds_the_words: 1,
            the_reason_the_machine_recorded: None,
            no_render_began: false,
            the_device_line_as_observed: None,
        });
        store.the_fill_in(&job.job, &note).unwrap();

        let text = job.job.as_text();
        let kept = what_is_kept(Some(&record), Some(&text));
        let entry = kept.entry.expect("the stopped job's entry");
        assert_eq!(entry.ending, Some(AnEnding::Stopped));
        let said = entry.failure.expect("a stopped entry carries the failure shape").said;
        assert_eq!(said, "You stopped this render. The frame it was on may not be saved.");
        for flat in ["was not saved", "is lost", "was lost"] {
            assert!(!said.contains(flat), "History says the frame {flat:?}: {said}");
        }
        let (the_stop_box, _) = include_str!("the_queue.rs")
            .split_once("#[cfg(test)]")
            .expect("the tests come last");
        assert!(
            the_stop_box.contains("\"The current frame may not be saved.\""),
            "the stop box no longer says the frame may not be saved, so History and the box \
             say one fact two ways"
        );
        let _ = std::fs::remove_dir_all(&path);
    }

    /// **A finished job sent with no name**, on a server that stands, the way
    /// the Vast pass's *pv-anim* was: no label on the record, and the machine's
    /// note carrying an empty one, which is what a submission with *Name this
    /// job* left empty writes.
    fn a_finished_job_with_no_name(tag: &str, blend: Option<&str>) -> (std::path::PathBuf, LocalRecord, String) {
        a_finished_job_writing_to(tag, blend, None)
    }

    /// The same, with the folder the read found for *Writing to*.
    fn a_finished_job_writing_to(
        tag: &str,
        blend: Option<&str>,
        folder: Option<&str>,
    ) -> (std::path::PathBuf, LocalRecord, String) {
        use crate::record::deletions::tests::a_record_in_history;
        use smoozy_state::identities::IdentityKind;

        let mut path = std::env::temp_dir();
        path.push(format!("smoozy-history-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).unwrap();
        let record = LocalRecord::open_at(&path.join("local-record.sqlite3")).unwrap();
        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        crate::record::server_rows::add(
            record.connection(),
            &smoozy_servers::store::ServerRow::saved(
                server.as_text(),
                smoozy_servers::identity::Host::what_the_user_typed("127.0.0.1").unwrap(),
                smoozy_servers::identity::Port::what_the_user_typed(22).unwrap(),
                smoozy_servers::identity::Username::what_the_user_typed("root").unwrap(),
                smoozy_servers::identity::KeyReference {
                    local_key_identifier: "key-1".into(),
                    fingerprint: "SHA256:aaa".into(),
                },
            ),
        )
        .unwrap();
        let store = JobRecords::over(record.connection());
        let mut job = a_record_in_history(&server, "");
        job.label = None;
        job.blend_path_on_the_machine = blend.map(str::to_string);
        job.main_output_from_the_read = folder.map(str::to_string);
        store.write_the_provisional_record(&job).unwrap();
        crate::surface::the_queue::tests::its_ending_is_learned(&store, &job.job, "");
        (path, record, job.job.as_text())
    }

    /// **History, posted the way the application posts it and drawn by
    /// `web/the-history.js` under Node** — the entry's heading, and whether
    /// *Download all* and *Files on the server* are on it. `None` where no Node
    /// is installed.
    fn what_the_history_page_draws(tag: &str, kept: &TheHistoryOnScreen) -> Option<String> {
        use std::sync::{Arc, Mutex};
        struct Collects(Arc<Mutex<Vec<crate::events::Delivery>>>);
        impl crate::events::Delivers for Collects {
            fn deliver(&self, delivery: &crate::events::Delivery) {
                self.0.lock().expect("the collector").push(delivery.clone());
            }
        }
        let queue = crate::events::TheOneQueue::new();
        let delivered = Arc::new(Mutex::new(Vec::new()));
        queue.the_page_is_ready(Box::new(Collects(Arc::clone(&delivered))));
        the_history_is_posted(&queue, kept.clone());
        let said = serde_json::to_string(&delivered.lock().unwrap().last().expect("History was posted").said)
            .expect("what History says is written out");

        let folder = std::env::temp_dir().join(format!("smoozy-history-page-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a scratch folder");
        let web = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        for page in ["the-queue.js", "the-history.js", "the-value-drawn.js"] {
            std::fs::copy(web.join(page), folder.join(page)).expect("the page is copied");
        }
        for (name, text) in crate::surface::the_queue::tests::THE_BOXES_STAND_INS {
            let text = if name == "harness.js" {
                // **The stand-in document and the two pages loaded**, and none
                // of that harness's own posts.
                let (the_document, _) = text.split_once("// ── The queue ──").expect("the harness's document");
                format!(
                    "{the_document}{}",
                    r#"
const { readFileSync } = await import("node:fs");
const said = JSON.parse(readFileSync("said.json", "utf8"));
const history = anElement("div");
globalThis.theRoot.appendChild(history);
globalThis.screens.get("historyview").itIsDrawn(history);
tell("history:the-history", said);
const textOf = (element) => (element.text || "") + (element.children || []).map(textOf).join("");
// The stand-in's selector reads no digit in a tag, so the heading is the
// header's first column's first line, as `theEntryIsDrawn` builds it.
const header = history.querySelector("div.qdh");
const heading = header ? header.children[0].children[0] : null;
const buttons = history.querySelectorAll("button").map(textOf);
console.log(JSON.stringify({
  heading: heading ? textOf(heading) : null,
  downloadAll: buttons.includes(said.downloadAll),
  filesOnTheServer: buttons.includes(said.filesOnTheServer),
}));
"#
                )
            } else {
                text.to_string()
            };
            std::fs::write(folder.join(name), text).expect("a stand-in is written");
        }
        std::fs::write(folder.join("said.json"), said).expect("what History says is written");
        let ran = std::process::Command::new("node").arg(folder.join("harness.js")).current_dir(&folder).output();
        let _ = std::fs::remove_dir_all(&folder);
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so History's page was not run");
            return None;
        };
        assert!(ran.status.success(), "the page did not run: {}", String::from_utf8_lossy(&ran.stderr));
        Some(String::from_utf8_lossy(&ran.stdout).trim().to_string())
    }

    /// **Step 10c's Vast pass, fault 3: a job sent with no name could not be
    /// reached.** *pv-anim* was sent with *Name this job* left empty — which
    /// the spec allows, the label being optional (`Specs/Queue & History`) —
    /// and its record holds no label. History drew its row with no name, and
    /// **selecting it drew an empty pane**: no *Download all*, no *Files on the
    /// server*, so its frames had no way home.
    ///
    /// **`Design/Queue & History § A rendering entry`, Creator-stated
    /// 2026-08-10: *"Where the job carries no label of its own, the name is the
    /// file."*** The job's folder is the identity alone (`Job files and
    /// outputs` N3), which the pass saw and which is right.
    ///
    /// This runs the record the pass left — a finished job, its server
    /// standing — through History's own composition and post, and draws it
    /// with `web/the-history.js`.
    #[test]
    fn a_job_sent_with_no_name_is_called_by_its_file_and_its_entry_draws() {
        let (path, record, job) = a_finished_job_with_no_name("no-name", Some("/pv-huge/blend/pv-anim.blend"));
        let kept = what_is_kept(Some(&record), Some(&job));
        let drawn = what_the_history_page_draws("no-name", &kept);
        let _ = std::fs::remove_dir_all(&path);

        assert_eq!(kept.rows[0].name, "pv-anim.blend", "History's row has no name");
        let entry = kept.entry.as_ref().expect("the job's entry");
        assert_eq!(entry.entry.name, "pv-anim.blend", "the entry's heading has no name");
        assert!(entry.entry.reach_backs, "the server stands");
        if let Some(drawn) = drawn {
            assert_eq!(
                drawn,
                r#"{"heading":"pv-anim.blend","downloadAll":true,"filesOnTheServer":true}"#,
                "the entry of a job with no name is not drawn whole"
            );
        }
    }

    /// **Step 10c's *Files on the server* follow-up: History offers *Files on
    /// the server* only while the entry's server is connected**, and the
    /// downloads stay. Composed and posted the way the application does, and
    /// drawn by `web/the-history.js`. **Undone** (the connection not read, or
    /// not said), it fails.
    #[test]
    fn follow_up_history_offers_files_on_the_server_only_while_its_server_is_connected() {
        let (path, record, job) = a_finished_job_with_no_name("follow-up", Some("/k/kitchen.blend"));
        let server = crate::record::server_rows::all(record.connection()).unwrap()[0]
            .local_server_identifier
            .clone();
        let away = what_is_kept_over(Some(&record), Some(&job), &|_| false);
        let here = what_is_kept_over(Some(&record), Some(&job), &|one| one == server);
        let drawn_away = what_the_history_page_draws("follow-up-away", &away);
        let drawn_here = what_the_history_page_draws("follow-up-here", &here);
        let _ = std::fs::remove_dir_all(&path);

        assert!(away.entry.as_ref().expect("the entry").entry.the_server_is_not_connected);
        assert!(!here.entry.as_ref().expect("the entry").entry.the_server_is_not_connected);
        if let (Some(away), Some(here)) = (drawn_away, drawn_here) {
            assert_eq!(away, r#"{"heading":"kitchen.blend","downloadAll":true,"filesOnTheServer":false}"#);
            assert_eq!(here, r#"{"heading":"kitchen.blend","downloadAll":true,"filesOnTheServer":true}"#);
        }
    }

    /// **The pane draws for any job**, even one with nothing to call it — a
    /// record written before the file's path was kept (`Queue & History` N11),
    /// with no label either. **Until the Vast pass's fix the page took an
    /// empty name for no entry at all**, and drew nothing.
    #[test]
    fn an_entry_with_nothing_to_call_it_still_draws_its_controls() {
        let (path, record, job) = a_finished_job_with_no_name("no-name-no-file", None);
        let kept = what_is_kept(Some(&record), Some(&job));
        let drawn = what_the_history_page_draws("no-name-no-file", &kept);
        let _ = std::fs::remove_dir_all(&path);

        assert_eq!(kept.entry.as_ref().expect("the job's entry").entry.name, "");
        if let Some(drawn) = drawn {
            assert_eq!(
                drawn,
                r#"{"heading":"","downloadAll":true,"filesOnTheServer":true}"#,
                "an entry with no name is drawn as no entry"
            );
        }
    }

    // ── Piece 6d: *Blender output* in History (`Queue & History` E9) ──────

    /// A real record in a throwaway folder, with one job of one server whose
    /// ending the fill-in has learned — so its words are owed — and the
    /// streams of `landed` kept for it.
    fn a_job_with_words(
        tag: &str,
        outcome: smoozy_queue_history::acting::note::TheOutcomeAsWritten,
        landed: &[crate::record::kept_words::AStreamKept],
    ) -> (std::path::PathBuf, LocalRecord, String) {
        use crate::record::deletions::tests::a_record_in_history;
        use smoozy_state::identities::IdentityKind;
        let path = std::env::temp_dir().join(format!("smoozy-history-words-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).unwrap();
        let record = LocalRecord::open_at(&path.join("local-record.sqlite3")).unwrap();
        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        let store = JobRecords::over(record.connection());
        let mut job = a_record_in_history(&server, "Kitchen");
        job.what_the_note_carried.ending = None;
        store.write_the_provisional_record(&job).unwrap();
        let mut note = crate::record::kept_words::tests::a_finished_note();
        if let Some(ending) = note.ending.as_mut() {
            ending.outcome = outcome;
            ending.the_attempt_whose_capture_record_holds_the_words = 2;
        }
        note.attempts = 2;
        store.the_fill_in(&job.job, &note).unwrap();
        crate::record::kept_words::the_words_land(record.connection(), &job.job, landed, true).unwrap();
        (path, record, job.job.as_text())
    }

    fn two_attempts_kept() -> Vec<crate::record::kept_words::AStreamKept> {
        use crate::presentation::the_machines_own_words::Stream;
        use crate::record::kept_words::tests::a_stream;
        use smoozy_runtime::capture::CollectionOutcome;
        let mut cut = a_stream(1, Stream::TheOrdinaryOutput, b"Fra:61 Mem:9.84G | Sample 96/2048");
        cut.outcome = Some(CollectionOutcome::CollectionIncomplete);
        cut.discarded = Some(41_800_000);
        cut.why_the_rest_went = Some("the size limit was reached".to_string());
        let mut not_yet = a_stream(1, Stream::TheComplaints, b"");
        not_yet.outcome = None;
        vec![
            cut,
            not_yet,
            a_stream(2, Stream::TheOrdinaryOutput, b"Fra:240 | Saved: '/out/0240.exr'"),
            a_stream(2, Stream::TheComplaints, b""),
        ]
    }

    /// **A failed ending offers *Blender output* where the words are kept,
    /// and only there** — never a control that opens nothing — **and an
    /// ending attempt whose complaints stream was recorded empty says so in
    /// its plain sentence** (must-do 9), which nothing could tell until the
    /// outcomes were kept.
    #[test]
    fn a_failed_endings_blender_output_is_offered_only_where_words_are_kept() {
        use smoozy_queue_history::acting::note::TheOutcomeAsWritten;
        let (path, record, job) =
            a_job_with_words("failed", TheOutcomeAsWritten::Failed, &two_attempts_kept());
        let kept = what_is_kept(Some(&record), Some(&job));
        let entry = kept.entry.as_ref().expect("the job's entry");
        let failure = entry.failure.as_ref().expect("a failure carries the shape");
        assert!(failure.blender_output);
        assert!(!entry.blender_output, "the failure holds it, never twice");
        assert!(!entry.entry.blender_output, "never the shared entry's own");
        assert_eq!(failure.said, THE_RENDER_FAILED_AND_THE_OUTPUT_MAY_SAY_WHY, "attempt 2's errors were empty");
        assert!(failure
            .the_controls()
            .iter()
            .any(|control| control.act == Verbatim::new("blender-output")));

        let (path_none, record_none, job_none) =
            a_job_with_words("failed-none", TheOutcomeAsWritten::Failed, &[]);
        let kept_none = what_is_kept(Some(&record_none), Some(&job_none));
        let failure_none = kept_none.entry.as_ref().unwrap().failure.as_ref().unwrap();
        assert!(!failure_none.blender_output, "no words kept, no control");
        assert_ne!(failure_none.said, THE_RENDER_FAILED_AND_THE_OUTPUT_MAY_SAY_WHY, "nothing says the errors were empty");
        let _ = std::fs::remove_dir_all(&path);
        let _ = std::fs::remove_dir_all(&path_none);
    }

    /// **E9 end to end on this side: the press opens every attempt's kept
    /// words in the one panel — each stream apart under its own name, each
    /// tab the record or a sentence, never an empty box, M10's four and
    /// M10a's fifth as they were recorded — and *not built* is nowhere.**
    /// The words travel only when asked for. The map's test *"Blender output
    /// answers not built nowhere it is offered"*, History's half.
    #[test]
    fn the_press_opens_every_attempts_kept_words_and_never_an_empty_box() {
        use crate::presentation::the_machines_own_words::{Stream, WhatATabSays};
        use smoozy_queue_history::acting::note::TheOutcomeAsWritten;
        let (path, record, job) =
            a_job_with_words("finished", TheOutcomeAsWritten::Finished, &two_attempts_kept());

        let unasked = what_the_page_asked_for(Some(&record), Some(&job), None, &|_| true, &[]);
        assert!(unasked.words.is_none(), "the words travel only when asked for");
        assert!(unasked.entry.as_ref().unwrap().blender_output);
        let unasked_text = said(&the_history_says(&unasked));
        assert!(unasked_text.contains("historyBlenderOutput"));
        assert!(!unasked_text.contains("wordsAttempts"));
        assert!(!unasked_text.contains("\"blenderOutput\""), "never the shared entry's control");

        let asked = what_the_page_asked_for(Some(&record), Some(&job), Some(TheHistoryAsk::BlenderOutput), &|_| true, &[]);
        assert!(asked.asking.is_none(), "not a box");
        let words = asked.words.as_ref().expect("the kept words");
        assert_eq!(words.attempts.iter().map(|(at, _)| *at).collect::<Vec<_>>(), vec![1, 2]);
        for (_, panel) in &words.attempts {
            assert_eq!(
                panel.tabs.iter().map(|tab| tab.the_stream_it_came_from).collect::<Vec<_>>(),
                vec![Stream::TheOrdinaryOutput, Stream::TheComplaints],
                "two streams, apart, never merged"
            );
            for tab in &panel.tabs {
                assert!(
                    tab.the_record().is_some() || !tab.the_sentences().is_empty(),
                    "an empty box"
                );
            }
        }
        let first = &words.attempts[0].1.tabs;
        assert!(matches!(first[0].says, WhatATabSays::OnlyPartWasRecorded { .. }));
        assert_eq!(first[1].says, WhatATabSays::TheOutcomeHasNotBeenRecordedYet, "M10a");
        let second = &words.attempts[1].1.tabs;
        assert_eq!(
            second[0].the_record().map(|record| record.as_str().to_string()),
            Some("Fra:240 | Saved: '/out/0240.exr'".to_string())
        );
        assert_eq!(second[1].says, WhatATabSays::TheStreamWasEmpty);

        let text = said(&the_history_says(&asked));
        for key in ["wordsAttempts", "words0theOutputRecord", "words1theErrorsSays", "wordsAttemptWord"] {
            assert!(text.contains(key), "{key} is not on the event");
        }
        assert!(text.contains("Fra:240 | Saved:"));
        assert!(!text.contains(crate::surface::files::A_CONNECTION_IS_NOT_BUILT));

        // **A press over a job the list no longer holds opens nothing.**
        let moved = what_the_page_asked_for(Some(&record), Some("job-gone"), Some(TheHistoryAsk::BlenderOutput), &|_| true, &[]);
        assert!(moved.words.is_none());
        let _ = std::fs::remove_dir_all(&path);
    }

    /// **The failure's *Blender output* asks History for the kept words**,
    /// the same ask as a clean finish's control, and **is never handed to the
    /// queue's acts**, where it would answer *not built*. Read off the page,
    /// because the failure shape that hands the press over is
    /// `web/the-failure-shape.js`'s and the Node harness stands it in.
    #[test]
    fn the_failures_blender_output_asks_history_for_the_words() {
        let page = include_str!("../../web/the-history.js");
        let (_, on_act) = page.split_once("onAct(act) {").expect("the failure's own press");
        let (asks, _) = on_act
            .split_once("what-the-server-said")
            .expect("the server's words are the other raw text");
        assert!(asks.contains("if (act === BLENDER_OUTPUT) {"), "{asks}");
        assert!(asks.contains("theHistoryIsAskedFor(said.entryJob, BLENDER_OUTPUT);"), "{asks}");
        assert!(page.contains("const BLENDER_OUTPUT = \"blender-output\";"));
        assert!(!page.contains("anActIsAskedFor(open"), "History's control is never the queue's act");
    }

    /// **History's own *Blender output*, drawn by `web/the-history.js` under
    /// Node: pressing it asks for the words, and the answer opens them in the
    /// one panel** — `web/the-machines-own-words.js`'s own `theWordsPanel`,
    /// the real one — **on the screen, open, with a tab per stream of each
    /// attempt named by both, and the record in it.** Skipped where no Node
    /// is installed.
    #[test]
    fn the_page_asks_for_the_kept_words_and_opens_them_in_the_one_panel() {
        use smoozy_queue_history::acting::note::TheOutcomeAsWritten;
        let (path, record, job) =
            a_job_with_words("page", TheOutcomeAsWritten::Finished, &two_attempts_kept());
        let unasked = what_the_page_asked_for(Some(&record), Some(&job), None, &|_| true, &[]);
        let asked = what_the_page_asked_for(Some(&record), Some(&job), Some(TheHistoryAsk::BlenderOutput), &|_| true, &[]);
        let _ = std::fs::remove_dir_all(&path);
        let as_the_page_reads_it = |says: &TheHistoryOnScreen| {
            use std::sync::{Arc, Mutex};
            struct Collects(Arc<Mutex<Vec<crate::events::Delivery>>>);
            impl crate::events::Delivers for Collects {
                fn deliver(&self, delivery: &crate::events::Delivery) {
                    self.0.lock().expect("the collector").push(delivery.clone());
                }
            }
            let queue = crate::events::TheOneQueue::new();
            let delivered = Arc::new(Mutex::new(Vec::new()));
            queue.the_page_is_ready(Box::new(Collects(Arc::clone(&delivered))));
            the_history_is_posted(&queue, says.clone());
            let written =
                serde_json::to_string(&delivered.lock().unwrap().last().expect("posted").said).unwrap();
            written
        };

        let folder = std::env::temp_dir().join(format!("smoozy-history-words-page-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).unwrap();
        let web = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        for page in ["the-queue.js", "the-history.js", "the-value-drawn.js", "the-machines-own-words.js"] {
            std::fs::copy(web.join(page), folder.join(page)).unwrap();
        }
        for (name, text) in crate::surface::the_queue::tests::THE_BOXES_STAND_INS {
            if name == "the-machines-own-words.js" {
                continue;
            }
            let text = if name == "harness.js" {
                let (the_document, _) = text.split_once("// ── The queue ──").expect("the harness's document");
                let the_document = the_document.replace(
                    "if (command === \"the_pane_layout\")",
                    "globalThis.invoked = (globalThis.invoked || []).concat([[command, arguments[1]]]);\n        if (command === \"the_pane_layout\")",
                );
                format!(
                    "{the_document}{}",
                    r#"
const { readFileSync } = await import("node:fs");
const unasked = JSON.parse(readFileSync("unasked.json", "utf8"));
const asked = JSON.parse(readFileSync("asked.json", "utf8"));
const history = anElement("div");
globalThis.theRoot.appendChild(history);
globalThis.screens.get("historyview").itIsDrawn(history);
tell("history:the-history", unasked);
const textOf = (element) => (element.text || "") + (element.children || []).map(textOf).join("");
const control = history.querySelectorAll("button").find((one) => textOf(one) === unasked.historyBlenderOutput);
const everyOne = (element) => [element].concat((element.children || []).flatMap(everyOne));
const theGroup = control && control.parentNode ? control.parentNode.parentNode : null;
const theHeadings = everyOne(history)
  .filter((one) => String(one.tag || "").toLowerCase() === "h3")
  .map(textOf);
const theGroupHolds = theGroup ? theGroup.children.map((one) => String(one.tag || "").toLowerCase()) : [];
globalThis.invoked = [];
if (control) control.click();
const theAsk = globalThis.invoked.find(([command]) => command === "the_history");
const before = history.querySelectorAll("div.words-panel").length;
tell("history:the-history", asked);
const panels = history.querySelectorAll("div.words-panel");
const panel = panels[0];
console.log(JSON.stringify({
  control: !!control,
  headings: theHeadings.filter((one) => one === unasked.historyBlenderOutput).length,
  theGroupHolds,
  asking: theAsk ? theAsk[1].asking : null,
  chosen: theAsk ? theAsk[1].chosen : null,
  before,
  panels: panels.length,
  open: panel ? !panel.hidden : false,
  onTheScreen: panel ? panel.parentNode === history : false,
  tabs: panel ? panel.querySelectorAll("button.ltab").map(textOf) : [],
  text: panel ? textOf(panel) : "",
}));
"#
                )
            } else {
                text.to_string()
            };
            std::fs::write(folder.join(name), text).unwrap();
        }
        std::fs::write(folder.join("unasked.json"), as_the_page_reads_it(&unasked)).unwrap();
        std::fs::write(folder.join("asked.json"), as_the_page_reads_it(&asked)).unwrap();
        let ran = std::process::Command::new("node").arg(folder.join("harness.js")).current_dir(&folder).output();
        let _ = std::fs::remove_dir_all(&folder);
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so History's page was not run");
            return;
        };
        assert!(ran.status.success(), "the page did not run: {}", String::from_utf8_lossy(&ran.stderr));
        let drawn: serde_json::Value =
            serde_json::from_str(String::from_utf8_lossy(&ran.stdout).trim()).expect("one line of JSON");
        assert_eq!(drawn["control"], true, "History drew no Blender output: {drawn}");
        // **No *Blender output* heading over the control** — the Creator's
        // third notes, note 15. Undone, the group holds an `h3` before the
        // control and the heading reads the control's own words.
        assert_eq!(drawn["headings"], 0, "History drew a Blender output heading: {drawn}");
        assert_eq!(drawn["theGroupHolds"], serde_json::json!(["div"]), "the control stands alone: {drawn}");
        assert_eq!(drawn["asking"], "blender-output", "{drawn}");
        assert_eq!(drawn["chosen"], job.as_str(), "{drawn}");
        assert_eq!(drawn["before"], 0, "the panel opened before the words arrived: {drawn}");
        assert_eq!(drawn["panels"], 1, "{drawn}");
        assert_eq!(drawn["open"], true, "{drawn}");
        assert_eq!(drawn["onTheScreen"], true, "the panel must outlive a redraw of the entry: {drawn}");
        assert_eq!(
            drawn["tabs"],
            serde_json::json!([
                "Output · Attempt 1",
                "Errors · Attempt 1",
                "Output · Attempt 2",
                "Errors · Attempt 2"
            ]),
            "{drawn}"
        );
        // **The prose as the typography point hands it over** — its last
        // space bound (X2) and its apostrophe the point's — so the space is
        // read as a space and the words around the apostrophe are looked for.
        let text = drawn["text"].as_str().unwrap_or_default().replace('\u{a0}', " ");
        assert!(text.contains("Fra:240 | Saved: '/out/0240.exr'"), "{drawn}");
        assert!(text.contains("Not recorded yet."), "{drawn}");
        assert!(text.contains("Blender wrote nothing here."), "{drawn}");
    }

    /// A spine over a throwaway record, as `src/surface/servers.rs`'s tests
    /// make one.
    /// **The nineteenth notes, note 7** — `Design/Queue & History § A
    /// first-frame check and its job`, in History: a pair is one row and one
    /// entry, the full range's, with the check's box at its top; a check
    /// whose full range is still live reads *In Queue* and is not deleted;
    /// a check whose full range never rendered reads *Check only* and offers
    /// no *Render it again*; *Delete* takes a pair together; and older
    /// records stay as they were.
    mod nineteenth_note_7 {
        use super::*;
        use crate::surface::the_queue::tests::nineteenth_note_7::a_pair;
        use crate::surface::the_queue::{CHECK_ONLY, IN_QUEUE};
        use smoozy_queue_history::acting::note::TheOutcomeAsWritten;

        /// **`h-checkjob`: both ended, one row**, the full range's figures
        /// and ending, the check under the name; the entry is the full
        /// range's with the check's picture and *You chose Render all*; and
        /// *Delete* asks for both and takes both.
        #[test]
        fn a_pair_is_one_row_and_delete_takes_both() {
            let pair = a_pair("history-pair", Some(TheOutcomeAsWritten::Finished), Some(TheOutcomeAsWritten::Finished), true, true);
            let (check, range) = (pair.check.job.as_text(), pair.range.job.as_text());
            let started = a_started_over(pair.record);
            for chosen in [Some(check.as_str()), Some(range.as_str()), None] {
                let kept = what_is_kept(started.record.as_ref(), chosen);
                assert_eq!(kept.jobs, 1, "a pair counts once");
                assert_eq!(kept.rows.len(), 1);
                let row = &kept.rows[0];
                assert_eq!(row.job, range, "the row is the full range's");
                assert_eq!(row.ending, Some(AnEnding::Finished), "the full range's ending");
                assert_eq!(row.the_check.len(), 1);
                assert_eq!(row.the_check[0].what.value, "1");
                assert_eq!(row.the_check[0].dot, "done");
                assert_eq!(row.in_place_of_the_ending, None);
                assert_eq!(kept.chosen.as_deref(), Some(range.as_str()));
                let entry = kept.entry.unwrap();
                assert_eq!(entry.entry.job, range);
                assert!(entry.deleted_with_its_check && !entry.the_check_waits_on_its_range);
                let the_check = entry.entry.the_check.unwrap();
                assert!(the_check.you_chose_render_all);
                assert_eq!(the_check.rendered_for.as_deref(), Some("2m"));
                assert!(the_check.when.is_some(), "History says when the check ended");
                assert_eq!(the_check.picture.unwrap().picture, Some(check.clone()));
            }
            let kept = what_is_kept(started.record.as_ref(), Some(&range));
            let asked = the_history_as_asked(kept, Some(&range), Some(TheHistoryAsk::Delete)).asking.unwrap();
            assert_eq!(asked.heading, vec![APart::words(DELETE_THIS_ENTRY_AND_ITS_CHECK_QUESTION)]);
            assert!(said(&the_history_says(&what_is_kept(started.record.as_ref(), Some(&range)))).contains("\"delete\""));
            assert_eq!(the_act_is_performed(&started, TheActOnAJob::DeleteTheEntry, Some(&range)), Some(Ok(())));
            let store = JobRecords::over(started.record.as_ref().unwrap().connection());
            assert_eq!(store.read(&pair.check.job).unwrap(), None, "the check goes with its full range");
            assert_eq!(store.read(&pair.range.job).unwrap(), None);
            assert_eq!(what_is_kept(started.record.as_ref(), None).jobs, 0);
        }

        /// **`h-checkwait`: the full range is still live**, so the check is
        /// its own row reading *In Queue* in grey, its box saying where the
        /// frames are with *Show in Queue*; nothing deletes it — not the
        /// control, not the key, not a clearing.
        #[test]
        fn a_check_whose_range_is_live_reads_in_queue_and_is_not_deleted() {
            let pair = a_pair("history-wait", Some(TheOutcomeAsWritten::Finished), None, true, true);
            let check = pair.check.job.as_text();
            let started = a_started_over(pair.record);
            let kept = what_is_kept(started.record.as_ref(), Some(&check));
            assert_eq!(kept.jobs, 1);
            let row = &kept.rows[0];
            assert_eq!(row.job, check);
            assert_eq!(row.in_place_of_the_ending, Some(IN_QUEUE));
            assert!(row.the_check_waits_on_its_range);
            let entry = kept.entry.clone().unwrap();
            assert!(entry.the_check_waits_on_its_range);
            assert!(entry.again.is_none(), "a check offers no Render it again");
            let the_check = entry.entry.the_check.unwrap();
            assert_eq!(the_check.show_in_queue, Some(pair.range.job.as_text()));
            assert!(the_check.picture.is_none(), "its own entry already shows it down the side");
            let text = said(&the_history_says(&kept));
            for key in ["hrow0Ending", "e-chk", "ffShowInQueueJob", "hrow0Pair0WhatValue"] {
                assert!(text.contains(key), "{key} is said: {text}");
            }
            assert!(!text.contains("\"delete\""), "no Delete");
            assert!(!text.contains("clearAllHistory"), "a clearing has nothing it may take");
            assert!(the_history_as_asked(kept.clone(), Some(&check), Some(TheHistoryAsk::Delete)).asking.is_none());
            assert!(the_history_as_asked(kept, Some(&check), Some(TheHistoryAsk::ClearAll)).asking.is_none());
            assert_eq!(the_act_is_performed(&started, TheActOnAJob::DeleteTheEntry, Some(&check)), Some(Err(())));
            assert_eq!(the_act_is_performed(&started, TheActOnAJob::ClearAllHistory, None), Some(Ok(())));
            let store = JobRecords::over(started.record.as_ref().unwrap().connection());
            assert!(store.read(&pair.check.job).unwrap().is_some(), "the waiting question still shows its picture");
            // **Clearing this server from the check's own entry** takes the
            // server's other entries and leaves the check.
            let other = crate::record::deletions::tests::a_record_in_history(&pair.server, "Bathroom — test");
            store.write_the_provisional_record(&other).unwrap();
            assert_eq!(the_act_is_performed(&started, TheActOnAJob::ClearThisServersHistory, Some(&check)), Some(Ok(())));
            assert!(store.read(&other.job).unwrap().is_none());
            assert!(store.read(&pair.check.job).unwrap().is_some());
        }

        /// **`h-checkonly`: a check whose full range was cancelled** reads
        /// *Check only*, its box says the full range was not rendered, it
        /// offers no *Render it again*, and it may be deleted; **one that
        /// failed** reads its own ending instead.
        #[test]
        fn a_check_whose_range_was_cancelled_reads_check_only() {
            let pair = a_pair("history-only", Some(TheOutcomeAsWritten::Finished), None, false, true);
            let check = pair.check.job.as_text();
            let started = a_started_over(pair.record);
            let kept = what_is_kept(started.record.as_ref(), Some(&check));
            let row = &kept.rows[0];
            assert_eq!(row.in_place_of_the_ending, Some(CHECK_ONLY));
            assert!(!row.the_check_waits_on_its_range);
            let entry = kept.entry.clone().unwrap();
            assert!(entry.again.is_none(), "a check offers no Render it again");
            assert!(!entry.deleted_with_its_check);
            let the_check = entry.entry.the_check.unwrap();
            assert_eq!(the_check.said.len(), 1);
            assert!(the_check.show_in_queue.is_none() && !the_check.you_chose_render_all);
            assert!(said(&the_history_says(&kept)).contains("\"delete\""));
            assert_eq!(the_act_is_performed(&started, TheActOnAJob::DeleteTheEntry, Some(&check)), Some(Ok(())));

            let failed = a_pair("history-only-failed", Some(TheOutcomeAsWritten::Failed), None, false, true);
            let check = failed.check.job.as_text();
            let started = a_started_over(failed.record);
            let kept = what_is_kept(started.record.as_ref(), Some(&check));
            assert_eq!(kept.rows[0].in_place_of_the_ending, None, "its own ending, as the row reads it");
            assert_eq!(kept.rows[0].ending, Some(AnEnding::Failed));
            assert_eq!(kept.rows[0].the_check[0].dot, "");
        }

        /// **Older records stay as they were**: no saved purpose, two
        /// ordinary rows, each with its own *Delete* and nothing about a
        /// check.
        #[test]
        fn a_legacy_record_with_no_pair_stays_ordinary() {
            let pair = a_pair("history-legacy", Some(TheOutcomeAsWritten::Finished), Some(TheOutcomeAsWritten::Finished), true, false);
            let started = a_started_over(pair.record);
            let kept = what_is_kept(started.record.as_ref(), Some(&pair.check.job.as_text()));
            assert_eq!(kept.jobs, 2);
            assert!(kept.rows.iter().all(|row| row.the_check.is_empty() && row.in_place_of_the_ending.is_none()));
            let entry = kept.entry.unwrap();
            assert!(entry.entry.the_check.is_none() && !entry.deleted_with_its_check);
            assert_eq!(
                the_act_is_performed(&started, TheActOnAJob::DeleteTheEntry, Some(&pair.check.job.as_text())),
                Some(Ok(()))
            );
            let store = JobRecords::over(started.record.as_ref().unwrap().connection());
            assert!(store.read(&pair.range.job).unwrap().is_some(), "an ordinary entry goes alone");
        }
    }

    fn a_started_over(record: LocalRecord) -> Started {
        Started {
            the_sweep_left_behind: Vec::new(),
            record: Some(record),
            record_trouble: None,
            diary: None,
            declarations: smoozy_reporting::lines::Declarations::new(),
            what_modules_declare: crate::presentation::declarations::TheDeclarations::default(),
            said_once: crate::presentation::said_once::TheThingsSaidOnce::new(),
            run: None,
            the_previous_run: smoozy_state::run_mark::WhatTheLaunchFound::NoPreviousRun,

        }
    }

    /// **Design, *The two ways to render it again*, drawn at last** (the 38's
    /// piece 2): where the record kept the setup and the asked order is known,
    /// a stopped job offers *Submit again — 240 frames* and *Resume — 181
    /// frames* from the boundary frame it recorded; a finished one offers only
    /// the first; **an entry with no kept setup, or no asked order, draws
    /// neither**, there being nothing to make a new job from or to count.
    #[test]
    fn the_two_ways_to_render_again_are_drawn_from_the_kept_setup_and_the_asked_order() {
        use crate::record::job_records::TheKeptSetup;
        use smoozy_queue_history::acting::note::{TheEndingOnTheNote, TheOutcomeAsWritten};
        use smoozy_queue_history::record::TheFramesAsked;
        use smoozy_render_settings::file_half::BlenderIdentity;
        use smoozy_render_settings::payload::{compose, Composition};
        use smoozy_state::identities::IdentityKind;

        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        let mut record = crate::record::deletions::tests::a_record_in_history(&server, "Kitchen");
        record.what_the_note_carried.ending = Some(TheEndingOnTheNote {
            outcome: TheOutcomeAsWritten::LeftStopped,
            exit_status: None,
            the_attempt_whose_capture_record_holds_the_words: 1,
            the_reason_the_machine_recorded: None,
            no_render_began: false,
            the_device_line_as_observed: None,
        });
        record.what_the_note_carried.last_frame_figure = Some(60);
        let kept = TheKeptSetup::Kept(compose(Composition {
            version_pick: &BlenderIdentity {
                path: "/b".to_string(),
                version: smoozy_server_setup::blender::Version { major: 4, minor: 5, patch: 12 },
            },
            scene: None,
            answers: &smoozy_render_settings::store::SceneEntries::default(),
            standing: None,
            stale: false,
        }));
        let beside = BesideTheRecord {
            the_setup: Some(kept),
            ..BesideTheRecord::default()
        };
        // No asked order: nothing to count, nothing drawn.
        assert_eq!(the_entry_for(&record, &beside, false, true).again, None);
        record.the_frames.asked = TheFramesAsked::from_the_order(&(1..=240).collect::<Vec<i64>>());
        let again = the_entry_for(&record, &beside, false, true).again.expect("the pair");
        assert_eq!(again.submit_again, 240);
        assert_eq!(
            again.carry_on,
            Some(TheCarryOn {
                frames: 181,
                at: 60,
                came_from: WhereTheFigureCameFrom::RecordedWhenTheJobEnded,
            })
        );
        // No kept setup: nothing to make a new job from.
        assert_eq!(the_entry_for(&record, &BesideTheRecord::default(), false, true).again, None);
        // A clean finish: *Submit again* alone, saying why.
        record.what_the_note_carried.ending.as_mut().unwrap().outcome = TheOutcomeAsWritten::Finished;
        let finished = the_entry_for(&record, &beside, false, true).again.unwrap();
        assert_eq!(finished.carry_on, None);
        assert_eq!(finished.why, Some(EVERY_FRAME_IS_RENDERED));
        assert_eq!(finished.the_line().as_deref(), Some(EVERY_FRAME_IS_RENDERED));

        // **`Queue & History` E3: only part of what it owed.** The case the 5.3
        // integration's piece 1 made: the regular output written and a node's
        // files missing. The ending stays *Finished*.
        use smoozy_queue_history::acting::note::TheFilesCounted;
        record.what_the_note_carried.the_files_counted = Some(TheFilesCounted { written: 240, owed: 480 });
        let entry = the_entry_for(&record, &beside, false, true);
        assert_eq!(entry.ending, Some(AnEnding::Finished));
        let short = entry.again.unwrap();
        assert_eq!(short.carry_on, None);
        assert_eq!(short.the_line().as_deref(), Some("240 of 480 files written."));
        record.what_the_note_carried.the_files_counted = Some(TheFilesCounted { written: 0, owed: 1 });
        assert_eq!(
            the_entry_for(&record, &beside, false, true).again.unwrap().the_line().as_deref(),
            Some("0 of 1 file written.")
        );
        // **All of it, and no count at all, stand as they did.**
        record.what_the_note_carried.the_files_counted = Some(TheFilesCounted { written: 480, owed: 480 });
        assert_eq!(the_entry_for(&record, &beside, false, true).again.unwrap().the_line().as_deref(), Some(EVERY_FRAME_IS_RENDERED));
        record.what_the_note_carried.the_files_counted = None;
        assert_eq!(the_entry_for(&record, &beside, false, true).again.unwrap().the_line().as_deref(), Some(EVERY_FRAME_IS_RENDERED));

        // **Only a finish takes it**: a failure's own line and pair are left alone.
        record.what_the_note_carried.the_files_counted = Some(TheFilesCounted { written: 60, owed: 240 });
        record.what_the_note_carried.ending.as_mut().unwrap().outcome = TheOutcomeAsWritten::Failed;
        let failed = the_entry_for(&record, &beside, false, true).again.unwrap();
        assert!(failed.carry_on.is_some());
        assert_eq!(failed.the_count_came_up_short, None);
    }

    /// **The 38's piece 3 — Design, *The rest of the entry*, `Queue & History`
    /// P2 and P10a**: the entry draws the newest copy held, under a heading
    /// naming its frame and, where the copy carries one, its view layer, with
    /// *last complete* under it; a single-layer copy names the frame alone; no
    /// copy held is no picture; and a job that finished no frame says so.
    #[test]
    fn piece_3_the_entry_draws_the_newest_copy_held_and_says_when_no_frame_finished() {
        use smoozy_queue_history::acting::note::{TheEndingOnTheNote, TheOutcomeAsWritten};
        use smoozy_state::identities::IdentityKind;

        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        let ended = |outcome, figure: Option<i64>| {
            let mut record = crate::record::deletions::tests::a_record_in_history(&server, "Kitchen");
            record.what_the_note_carried.ending = Some(TheEndingOnTheNote {
                outcome,
                exit_status: Some(0),
                the_attempt_whose_capture_record_holds_the_words: 1,
                the_reason_the_machine_recorded: None,
                no_render_began: false,
                the_device_line_as_observed: None,
            });
            record.what_the_note_carried.last_frame_figure = figure;
            record
        };
        let finished = ended(TheOutcomeAsWritten::Finished, None);
        let held = |layer: Option<&[u8]>| BesideTheRecord {
            the_picture: Some(crate::record::previews::AFrameOfTheSet {
                job: finished.job.clone(),
                frame: 147,
                operation: "op".into(),
                view_layer: layer.map(<[u8]>::to_vec),
                owed: false,
                copy: Some("/previews/a.png".into()),
                published_on_this_computer_at: Some(std::time::SystemTime::now()),
                the_main_copy: true,
                place: 0,
                source: None,
            }),
            ..BesideTheRecord::default()
        };

        let entry = the_entry_for(&finished, &held(Some(b"Reflections")), false, true).entry;
        let preview = entry.preview.clone().expect("a picture");
        assert_eq!(preview.frame, "147");
        assert_eq!(preview.layer.as_deref(), Some("Reflections"));
        assert_eq!(preview.picture, Some(finished.job.as_text()));
        assert_eq!(preview.which, LAST_COMPLETE);
        assert!(!preview.nothing_to_show);
        let said = format!("{:?}", the_entry_is_said(Event::about(About::TheRun, THE_HISTORY_SAYS), Some(&entry)));
        for key in ["previewFrame", "previewLayer", "previewPicture", "notColourAccurate"] {
            assert!(said.contains(key), "{key} is said: {said}");
        }
        // **The Creator's seventh notes, note 4: the heading and the controls'
        // words are the application's, and History follows nothing**, so it
        // is never handed *Show latest*.
        assert!(!preview.follows);
        for key in ["frameWord", "previousFrame", "nextFrame", "viewLayer", "didNotRenderOnThisFrame"] {
            assert!(said.contains(key), "{key} is said: {said}");
        }
        assert!(said.contains("Frame\\u{a0}preview"), "the heading reads Frame preview: {said}");
        assert!(!said.contains("showLatest"), "History has no Show latest: {said}");

        let alone = the_entry_for(&finished, &held(None), false, true).entry.preview.unwrap();
        assert_eq!(alone.layer, None, "a copy that carries no name gives the frame alone");

        // **The Creator's seventh notes, note 4: the surface hands the page
        // every frame held, each with its View Layers in the frame's order and
        // which is the main copy** — values only, the controls being Z9a's.
        let a_copy = |frame: i64, layer: Option<&[u8]>, main: bool, place: i64| crate::record::previews::AFrameOfTheSet {
            job: finished.job.clone(),
            frame,
            operation: "op".into(),
            view_layer: layer.map(<[u8]>::to_vec),
            owed: false,
            copy: Some(format!("/previews/{frame}-{place}.png").into()),
            published_on_this_computer_at: Some(std::time::SystemTime::now()),
            the_main_copy: main,
            place,
            source: None,
        };
        let layered = BesideTheRecord {
            the_copies_held: vec![
                a_copy(146, None, true, 0),
                a_copy(147, Some(b"Box"), false, 0),
                a_copy(147, Some(b"Reflections"), true, 1),
            ],
            ..held(Some(b"Reflections"))
        };
        let entry = the_entry_for(&finished, &layered, false, true).entry;
        let frames = entry.preview.clone().unwrap().frames_held;
        assert_eq!(frames.len(), 2);
        assert_eq!(frames[0].frame, "146");
        assert_eq!(frames[0].layers, vec![crate::surface::the_queue::ALayerHeld { name: None, source: None, place: "0".into(), main: true }]);
        assert_eq!(
            frames[1].layers.iter().map(|layer| (layer.name.as_deref(), layer.main)).collect::<Vec<_>>(),
            vec![(Some("Box"), false), (Some("Reflections"), true)],
            "the frame's own order, the main copy marked"
        );
        let said = format!("{:?}", the_entry_is_said(Event::about(About::TheRun, THE_HISTORY_SAYS), Some(&entry)));
        for key in ["previewHeldCount", "previewHeldFrame1", "previewHeldLayerCount1", "previewHeld1Layer0", "previewHeld1Place1", "previewHeldMain1"] {
            assert!(said.contains(key), "{key} is said: {said}");
        }

        // **The Creator's tenth notes, note 2: a main copy that is no one
        // View Layer's is handed over named by where it came from** — the
        // words *Output path*, or *File Output node* with the node's own name
        // as a value — first in its frame, with no place, since it is asked
        // for as the frame's main copy. **What it bites on**: such a copy
        // handed over unnamed, or with the first layer's place.
        {
            use smoozy_job_files::main_output::WhereThePictureCameFrom;
            let sourced = |frame: i64, source: WhereThePictureCameFrom| crate::record::previews::AFrameOfTheSet {
                source: Some(source),
                ..a_copy(frame, None, true, 0)
            };
            let node = WhereThePictureCameFrom::AFileOutputNode(smoozy_job_files::reading::ArtistsText::of(b"Beauty".to_vec()));
            let beside = BesideTheRecord {
                the_copies_held: vec![
                    sourced(147, WhereThePictureCameFrom::TheOutputPath),
                    a_copy(147, Some(b"Box"), false, 0),
                    a_copy(147, Some(b"Mattress"), false, 1),
                    sourced(148, node),
                ],
                ..held(None)
            };
            let entry = the_entry_for(&finished, &beside, false, true).entry;
            let frames = entry.preview.clone().unwrap().frames_held;
            let as_handed = |at: usize| {
                frames[at]
                    .layers
                    .iter()
                    .map(|layer| (layer.source, layer.name.clone(), layer.place.clone(), layer.main))
                    .collect::<Vec<_>>()
            };
            assert_eq!(
                as_handed(0),
                vec![
                    (Some("Output path"), None, String::new(), true),
                    (None, Some("Box".to_string()), "0".to_string(), false),
                    (None, Some("Mattress".to_string()), "1".to_string(), false),
                ],
                "the source first, then the layers"
            );
            assert_eq!(as_handed(1), vec![(Some("File Output node"), Some("Beauty".to_string()), String::new(), true)]);
            let said = format!("{:?}", the_entry_is_said(Event::about(About::TheRun, THE_HISTORY_SAYS), Some(&entry)));
            for key in ["previewHeld0Source0", "previewHeld0Layer1", "previewHeld0Place1", "previewHeld1Source0", "previewHeld1Layer0"] {
                assert!(said.contains(key), "{key} is said: {said}");
            }
            for key in ["previewHeld0Place0", "previewHeld0Layer0", "previewHeld1Place0", "previewHeld0Source1"] {
                assert!(!said.contains(key), "{key} is not said: {said}");
            }
        }

        assert!(
            the_entry_for(&finished, &BesideTheRecord::default(), false, true).entry.preview.is_none(),
            "no copy held is no picture, and no words invented for it"
        );
        let stopped_short = ended(TheOutcomeAsWritten::Failed, Some(60));
        assert!(the_entry_for(&stopped_short, &BesideTheRecord::default(), false, true)
            .entry
            .preview
            .is_none());
        let none_finished = ended(TheOutcomeAsWritten::Failed, None);
        let nothing = the_entry_for(&none_finished, &BesideTheRecord::default(), false, true)
            .entry
            .preview
            .expect("the line where the picture was");
        assert!(nothing.nothing_to_show);
    }
}

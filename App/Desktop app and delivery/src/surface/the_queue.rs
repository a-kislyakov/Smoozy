//! **The Queue screen** — the list of what every machine is working on, beside
//! the entry of the job that is chosen. The page half is `web/the-queue.js`.
//!
//! **This file is the surface and not the deciding.** Every judgement below is
//! `Queue & History`'s, in `App/Queue & History/`, and this file translates:
//! it takes that module's own composed values, puts Design's words beside them,
//! and posts the pair on the one ordered queue. **Nothing here decides a queue
//! state, an ending, a figure or a mark.**
//!
//! ## What this piece draws, cited the way the runner ruled on 2026-09-12
//!
//! **`Preparation/Design/Queue & History.md` is the specification and it
//! carries no clause marker**, which is why the runner's first ruling of
//! 2026-09-12 exists: a screen cites the Design document and the heading it
//! draws, and the mockup state names it draws.
//!
//! **The Design headings this file draws**: *The queue list*, *A rendering
//! entry*, *The job's name, and its folder*, *The marks*, *What the job
//! writes*, *The rest of the entry*, *The entry's controls*, *Work Smoozy is
//! not tracking*, *Jobs that leave without an entry*, *The catch-up notice*.
//!
//! **The mockup state names this file draws**, which are `<option>` values of
//! `Preparation/Design/Mockup/Smoozy.html`'s own state list: `q`, `q-marks`,
//! `q-catchup`, `q-dropped`, `q-untracked`, `q-part`, `q-stop`, `q-cancel`,
//! `q-first`, `q-gcancel`, `q-dismiss`, `q-wait`, `q-driver`, `q-fallback`,
//! `q-fallchosen`, `q-checkjob`, `q-large`, `q-pvmissing`, `q-pvbad`. **Nineteen, and the history's states are
//! [`crate::surface::the_history`]'s.** *(Ten until the Creator's second
//! notes, piece 4's queue half, added the waiting entry with no folder yet,
//! and the one or two lines under a folder that fell back; thirteen until
//! held-job piece B, 2026-10-12, added `q-driver`, a job held for the driver
//! update — `Design/Queue & History § A job held for the driver update`,
//! `Queue & History` H12; fourteen until the Creator's ninth notes, note 3
//! (built 2026-10-16), which made `q-catchup` the endings counted in the heading and
//! added `q-dropped`, a job dropped before it rendered, named in one line
//! under it — `Design/Queue & History § The catch-up notice`.)* The six `t-`
//! states in the same mockup group are piece 12's and nothing here touches
//! them.
//!
//! ## Where the reading comes from
//!
//! **`State, identity and recovery` D4 keeps the desktop from holding a durable
//! copy of an observed machine state the machine still holds, and the queue is
//! exactly that.** `Queue & History` C6 makes the last-known queue reading
//! **period-held** — process memory for the connected period — so this screen
//! reads a run-held reading and never a store, and Design says the consequence
//! in its own words: *"the queue is blank on a fresh launch until a server is
//! connected, because Smoozy keeps no lasting copy of it."*
//!
//! **What produces that reading is `Queue & History`'s
//! `acting::adoption::AsksAMachineForItsQueue`, and since step 10c piece 7 it
//! has its end**: `crate::servers::the_machines_work` takes it at `Servers` Q1
//! step 7 on every primary connection that settles on the confirmed branch,
//! and again on each P10d pass while this screen, the server list or that
//! server's own page is on screen (the runner's ruling on P10d, 2026-09-28,
//! step 10c's ninth-pass fix piece 2). [`what_is_held`]
//! composes the screen from that reading and the local record: **a row is a
//! job the machine holds and the local record names**; what the machine holds
//! and the record does not is the untracked-work offer; an ended job has left
//! the queue for history (`Queue & History` C11) — ended on the reading's word
//! or on the ending Smoozy learned, whichever came first. **The server list and
//! a server's own page take their work column from the same composition**
//! ([`what_the_servers_are_doing`]), so no two of them can disagree.
//!
//! **Where a server's break is declared the reading stays and says it is not
//! updating** (`Queue & History` C7), and it goes entirely when that server's
//! period ends (`Queue & History` C6).
//!
//! **The local job record does have a store, and since record version 23 it is
//! `src/record/job_records.rs`.** `Queue & History` N10 to `Queue & History`
//! N22's `LocalJobRecord` is what every entry on both screens is composed from,
//! `acting::submission::WritesTheProvisionalRecord` has a real end there, and
//! the History screen draws from it. **It changes nothing on this screen**, and
//! the reason is `Queue & History` C10: the queue row is the machine's live
//! state read on a connection, and the history row is the local record. **A
//! provisional record is drawn on neither list** — `State, identity and
//! recovery` V1 step 2, at that document's §7, says so in as many words: it
//! becomes a history entry only when Smoozy learns the job started rendering.
//!
//! ## Which of this screen's acts are built, and it is twenty-two of twenty-four
//!
//! **[`where_an_act_on_a_job_goes`] is the one place that answers**, and a test
//! reads every act the drawing carries against it, so this paragraph cannot
//! quietly stop being true the way `src/surface/servers.rs`'s header once did.
//! *(It read "three of twenty-one" from step 10b until the eleventh-pass fix
//! piece 8, though step 10c's S3 had made six more act — the fault that test
//! exists for, in the one place it does not reach. It read "eleven of
//! twenty-one" until the 38's piece 2, and "fifteen of twenty-two" through the
//! 38's piece 9, which made two more act and left this line behind; and
//! "eighteen of twenty-two" until the 38's piece 8 added the two moves, and
//! "twenty of twenty-four" until step 10c's *Files on the server* piece, and
//! "twenty-one of twenty-four" until owed piece 10 made *Track it* act.)*
//!
//! - **Three are History's**, landed by step 10b piece 8 on the store piece 6
//!   built: the delete box's *Delete*, *Clear this server's history* and
//!   *Clear all history*. Each is `Queue & History` X1's record and tombstone
//!   written as one unit, through [`crate::record::deletions::TheDeletions`].
//! - **Six are step 10c's S3's**: *Stop*, *Cancel* and *Cancel all* raise
//!   their boxes, and each box's go-ahead reaches the machine.
//! - **Two are the held entry's answers**, step 10c's eleventh-pass fix piece
//!   8: *Render all* and *Cancel* answer the hold (`Queue & History`
//!   H2).
//! - **Four are the 38's piece 2's**: *Submit again* and the carry-on make a
//!   new job through `crate::a_new_job_from_an_old_one`, a mark's way opens
//!   the earlier entry (`Queue & History` N20), and *Submit the remaining
//!   scenes* sends the scenes an all-scenes act left unsent (`Rendering`
//!   S11).
//!
//! - **Two are Transfers' own**, the 38's piece 9's: the entry's
//!   auto-download switch and *Files on your computer*.
//! - **One is the catch-up notice's *Dismiss***, the 38's piece 15. Since the
//!   Creator's ninth notes, note 3, it is a dropped job's line's own *Dismiss*,
//!   and it takes that one name off the notice record that holds it
//!   (`Queue & History` C2a as changed 2026-10-12).
//! - **Two are the moves on the line**, the 38's piece 8: a row dragged or
//!   moved to the top from its menu (`Queue & History` L6), and a group's
//!   heading dragged (`Queue & History` L7), each reaching the machine through
//!   [`crate::servers::the_queues_own_acts::the_move_is_begun`].
//!
//! - **One is *Files on the server***, step 10c's piece of that name: it
//!   opens Files on that server at the job's folder (`Queue & History` N18
//!   and `Queue & History` N10), and the *Writing to* link asks for the same act.
//!
//! - **One is the untracked-jobs banner's *Track them***, owed piece 10: the
//!   server's whole offer enters the local record in one write (`Queue &
//!   History` A6), through
//!   [`crate::servers::the_machines_work::the_offer_is_accepted`].
//!
//! **The other two want a machine**, and answer so.
//!
//! ## Prose, values, and the one trap
//!
//! **Every word the user reads is a `Prose` in this file and every figure and
//! every name is a `Verbatim`, and the two never meet before the page** (A6,
//! X9, X7). A job's name, a server's name, a scene's name, a path, a frame
//! number and a percentage are all values. **A sentence with a name in it is
//! composed as a lead, a value and a trail** — [`APart`] — because
//! `format!("{} could not be reached", name)` is the join X7 forbids and no
//! test anywhere would catch it.
//!
//! ## What this file does not draw, and why
//!
//! **No line of either of Blender's streams beside an ending, anywhere.** The
//! mockup draws one in a `.log .last` span on five of the history endings.
//! **M13b forbids it in as many words** — *"No line of that stream is lifted out
//! of it. No excerpt, no last line, no highlight, no leading summary"* — and the
//! head of `src/presentation/mod.rs` already records the sites and rules that the
//! clause wins under Rule 10. **The control that opens the record stays and is
//! drawn**; the line beside it is not.
//!
//! **M13b's one narrowing is the rendering entry's**, added 2026-10-03 out of the
//! Creator's note 12 on the test copy: while the job renders, the entry shows the
//! ordinary stream's newest three complete lines above the control
//! ([`TheJobEntry::lines`]). Never the errors stream, never once the job has
//! ended, never beside *Not updating*, never on History, so never beside a
//! sentence about a failure, which is the harm the clause names.
//!
//! ## This file carries no header list of other documents' markers, and that is
//! forced rather than chosen
//!
//! **Nearly every marker this file cites belongs to `Queue & History`, and this
//! crate's own plan defines a clause of the same short name for very nearly all
//! of them** — an M1, a P5, a P6, an X7, a C2, an L1, a Q2, an R6 and more.
//! **The one it does not is the trap of the rule rather than an exception to
//! it**: `Queue & History` N3 and `Queue & History` Y8 are named nowhere in
//! this plan, so a bare use of either would fail to resolve rather than
//! resolving wrongly — which is the easier fault to find and not the reason
//! the rule exists. A header list is not enough for such a marker, because a bare use
//! could then mean either document and nothing could tell. **So every one is
//! written out with its document at the site**, and a bare marker below means a
//! clause of this crate's own plan, `Preparation/Architecture/Desktop app and
//! delivery.md`.

use std::sync::Mutex;

use tauri::State;

use crate::events::{Event, Said, Subject as About, TheOneQueue};
use crate::servers::the_queues_own_acts::{where_the_act_stands, TheQueuesOwnAct, WhereTheActStands};
use crate::start::Started;
use crate::typography::{Prose, Verbatim};

/// **The one kind this screen's page listens for**, declared here beside the
/// service that posts it and copied into `web/the-queue.js`. Nothing holds the
/// two together, so copy carefully.
///
/// **One kind and not one per part of the screen.** P6: one ordered queue, and
/// a second kind would be a second drawing path for one surface — the list, the
/// notices, the entry and the box that asks are one answer to one question,
/// which is *what is the queue doing*.
pub const THE_QUEUE_SAYS: &str = "queue:the-queue";

// ───────────────────────────────────────────────────────────────────────────
// The words. Every one is `Preparation/Design/Queue & History.md`'s or the
// drawing's, and where neither has one, whose ruling it is is written at the
// site.
// ───────────────────────────────────────────────────────────────────────────

/// *The queue list.* The screen's own heading, and the rail's own word for it.
const THE_HEADING: &str = "Queue";

/// The heading's count line — two figures and two words, never one string.
const THE_JOBS_WORD: &str = "jobs";
const THE_JOB_WORD: &str = "job";
const THE_SERVERS_WORD: &str = "servers";
const THE_SERVER_WORD: &str = "server";

/// **Design, *The queue list*: the one line that teaches dragging, and it is
/// the only place dragging is taught** — the drawing's `.qfoot`, word for
/// word. Three strings because it names a menu item in the middle of it.
/// **Drawn from the 38's piece 8, when a move first reached a machine**; until
/// then it was left out, a line teaching a gesture that did nothing being a
/// false statement (must-do 5(f) of step 10a round 1 of the queue, history,
/// settings and downloads).
const WHAT_DRAGGING_DOES: &str =
    "Drag a row to reorder its server's queue, or a group's heading to move the group.";
const AND_THE_ROWS_OWN_MENU: &str = "Right-click a row for";
/// **Design: a row's own menu offers this, and nothing else about position.**
const MOVE_TO_THE_TOP: &str = "Move to the top of the queue";
/// The two grips' tooltips — the drawing's own `title`s.
const DRAG_IT_UP_THE_QUEUE: &str = "Drag to reorder";
const DRAG_THE_WHOLE_GROUP: &str = "Drag to move the group";
/// **A move's waiter gave up**: the shape of the cancel's, claiming no more
/// than it knows — the next reading says whether it landed. Ruled at this
/// site, the 38's piece 8: Design gives no words for it.
const THE_MOVE_WAS_SENT: &str =
    "Move sent. Timed out waiting for a reply.";

/// **Design, *The queue list*: a waiting job's place is not named, and what the
/// state column carries is the three states that are not readable from
/// position.** Label: **Creator-stated** (2026-08-13) for the third of them.
const RENDERING: &str = "Rendering";
const WAITING_ON_YOU: &str = "Needs your answer";
const NOT_UPDATING: &str = "Not updating";
/// **A fourth, `Design/Queue & History § The queue list` and *A job held for
/// the driver update*: a job held until the server's NVIDIA driver is
/// updated** (`Queue & History` H12), in the amber *Needs your answer* uses. The
/// row's word and the entry's status alike. Held-job piece B, 2026-10-12.
const WAITING_FOR_DRIVER_UPDATE: &str = "Waiting for driver update";
/// **`Design/Queue & History`, *A job just submitted is in the queue at
/// once*: the drawing's own word**, beside the spinner Smoozy already draws.
/// The Creator's seventh notes, piece Z7.
const SUBMITTING_ROW: &str = "Submitting…";
/// **The cross on a row saying *Not submitted.***, named for its tooltip and
/// for a reader that reads it aloud — `Queue & History` C10 as changed
/// 2026-10-19: *"with a cross that clears it"*. The cross itself is the
/// glyph `web/files.js` already draws for one.
const CLEAR_IT: &str = "Clear";

/// The folded server heading — *1 rendering · 2 waiting*. **Design: a folded
/// heading still says what is running**, so the words are its own and the two
/// figures are values.
const RENDERING_WORD: &str = "rendering";
const WAITING_WORD: &str = "waiting";
const LAST_SEEN: &str = "last seen";
/// The separator a folded heading's parts carry between them — the drawing's
/// own middle dot, sent as a word so the page writes none.
const A_DOT: &str = "·";

/// A row's second line, in the parts it is composed of. **Each is its own
/// string and the figures beside them are values** (X7).
pub(crate) const FRAMES_WORD: &str = "frames";
pub(crate) const FRAME_WORD: &str = "frame";
const FIRST_FRAME_RENDERED: &str = "first frame rendered";
const A_FILE_IT_NEEDS_WAS_REPLACED: &str = "a file it needs was replaced";

/// **Design, *The queue list*, the not-updating row.** Two sentences and two
/// strings (X10), with the frame and the moment beside them as values.
const WAS_RENDERING_FRAME: &str = "Was rendering frame";
const SMOOZY_DOES_NOT_KNOW_WHAT_IT_IS_DOING_NOW: &str =
    "Current status unknown.";
const AT_WORD: &str = "at";

/// **Design, *The queue list*: a group of scene-jobs reads as a group** (`Queue & History` L20).
/// The heading names the submission and how many scenes are in it, and its one
/// act is the cancel.
const ALL_SCENES_LEAD: &str = "all";
const SCENES_WORD: &str = "scenes";
const SCENE_WORD: &str = "scene";
const CANCEL_ALL: &str = "Cancel all";

// ── The entry's header — Design, *A rendering entry* ────────────────────────

/// **Design: where no scene was named the third field says so rather than going
/// blank.** `Queue & History` N22's mark says the same thing lower down; these are two
/// different
/// facts and neither says the other.
const NO_SCENE_NAMED: &str = "No scene named";

const DOWNLOAD_ALL: &str = "Download all";
const DOWNLOAD_ALL_TO: &str = "Download all to…";
const FILES_ON_THE_SERVER: &str = "Files on the server";
const FILES_ON_YOUR_COMPUTER: &str = "Files on your computer";
const STOP: &str = "Stop";
const CANCEL: &str = "Cancel";

/// **`Queue & History` N17: a reach-back is not offered where the machine is gone, and the reason
/// is stated.** The drawing's own shape for a control that cannot act is the
/// part-submission banner's — the control unavailable with the reason beside
/// it — and this is that shape reused rather than a second one invented.
/// **Two strings and a value between them** (X7).
const NEEDS_A_CONNECTION_TO: &str = "Needs a connection to";
const A_FULL_STOP: &str = ".";

/// **Design, *History*, quoted here because the queue entry uses the same
/// sentence**: where the server is no longer in Smoozy the control is absent
/// and one line sits where it was.
const THE_SERVER_IS_GONE_AND_ITS_FILES_WITH_IT: &str =
    "Server removed — files unavailable";

// ── The status line and the two bars — Design, *A rendering entry* ──────────

const STARTED_LEAD: &str = "started";
const AGO_WORD: &str = "ago";
const RENDERING_NOW: &str = "Rendering now";
const SIMULATING: &str = "Simulating:";
const SIMULATION_FRAME: &str = "frame";
const SIMULATION_OF: &str = "of";
const WHOLE_JOB: &str = "Whole job";
const VIEW_LAYER_WORD: &str = "View Layer";
const OF_WORD: &str = "of";
const TILES_WORD: &str = "tiles";
const SAMPLES_IN_THIS_TILE: &str = "samples in this tile";
const SAMPLES_WORD: &str = "samples";
const BLENDERS_ESTIMATE: &str = "Blender's estimate";
const LEFT_WORD: &str = "left";
const VRAM_WORD: &str = "VRAM";
const FREE_DISK: &str = "Free disk";

// **Design, *A rendering entry*, and `Queue & History` B4 behind it.** The frame's own bar goes
// to its end when the frame's piece ends rather than when the printed figures
// meet, and neither printed figure is a progress figure — so the two are drawn
// exactly as Blender printed them, and the qualifier above is what stops the
// sample pair reading as the frame's.

// ── The held job's two answers — Design, *What the job writes* ──────────────

const WAITING_ON_YOU_HEADING: &str = "Needs your answer";
/// **The first frame is the job's own first frame, and never *1*** — must-do 8
/// of step 10a round 1 of the queue, history, settings and downloads, and
/// `Specs/Render Settings`: *"Nothing in Smoozy assumes the first frame is
/// frame 1"*. Words, a value and words.
const THE_FIRST_FRAME_LEAD: &str = "Frame";
/// ***The full range has not started.*** since decision C (2026-10-02): the
/// held job is the whole range, its first frame included (`Rendering` S13 as
/// amended), so *the rest* would be untrue. Design's sentence.
const IS_RENDERED_AND_DOWNLOADED: &str =
    "is rendered and downloaded. The full range has not started.";
/// **The same sentence where the frame was not downloaded** — *Download frames
/// as they render* off, or the frame not down yet. Design's own sentence, less
/// the two words that would be untrue (step 10c's RunPod-pass fix piece 1).
const IS_RENDERED: &str = "is rendered. The full range has not started.";
/// ***Render all — 240 frames***, the count the held job's own (`Design/Queue &
/// History § What the job writes`, corrected 2026-10-02 from *Render the rest —
/// 239 frames*).
const RENDER_ALL: &str = "Render all —";
/// ***Render all*, where the count is not known** — after a restart (step
/// 10c's eleventh-pass fix piece 8b). Design's label, less the count.
const RENDER_ALL_WITH_NO_COUNT: &str = "Render all";
const CANCEL_IT: &str = "Cancel";
const CANCELLING_LEAVES_NO_RECORD: &str = "Cancelling leaves no record of this job.";
/// **The held entry's status line after *Needs your answer***: *— frame 1
/// rendered*. The drawing added *, 2 hours 14 minutes ago*; that is the
/// server's clock against this computer's, which `State, identity and
/// recovery` D5 forbids, so it is not said (2026-10-28).
const THE_DASH: &str = "\u{2014}";
const THE_FIRST_FRAME_RENDERED_LEAD: &str = "frame";
const THE_FIRST_FRAME_RENDERED_TRAIL: &str = "rendered";
/// **`Design/Queue & History § What the job writes`: an older record without
/// the saved values.**
const WAITING_FOR_FIRST_FRAME_APPROVAL: &str = "Needs your answer — first frame";

// ── A job held for the driver update — Design, *A job held for the driver
// update*, `Queue & History` H12. Held-job piece B, 2026-10-12.

/// ***Blender 5.2 needs a newer NVIDIA driver for OptiX.***, the box's bold
/// sentence, around the job's own Blender as a value.
const THE_JOBS_BLENDER_LEAD: &str = "Blender";
const NEEDS_A_NEWER_NVIDIA_DRIVER_FOR_OPTIX: &str = "needs a newer NVIDIA driver for OptiX.";
/// **The same from 5.3**, where the card cannot be used at all under Cycles
/// (`Rendering` L23a's bullet of 2026-11-02).
const NEEDS_A_NEWER_NVIDIA_DRIVER_FOR_CYCLES_GPU: &str = "needs a newer NVIDIA driver for Cycles GPU rendering.";
/// **What happens next, and the restart**, which the user would otherwise not
/// expect.
const SMOOZY_WILL_UPDATE_THE_DRIVER: &str =
    "When the server's other jobs finish, the driver is updated, the server restarts, and this job starts.";
/// **The box's note** (`Server Setup` §13 item 29: the swap needs Smoozy
/// connected).
const THIS_ONLY_HAPPENS_WHILE_CONNECTED: &str = "Requires a connection to the server.";

const STOP_THIS_JOB: &str = "Stop this job";
const CARRY_ON_ANYWAY: &str = "Continue anyway";
const NOTHING_HAS_RENDERED_YET: &str = "Nothing has rendered yet.";
const THIS_JOB_WRITES_NO_FILES: &str = "This job writes no files.";
const THE_SERVERS_TIME_WOULD_BE_SPENT: &str =
    "Rendering it costs server time and saves nothing.";
const TWO_DESTINATIONS_COLLIDE: &str =
    "Two of this job's outputs write to the same file.";
const ONE_OVERWRITES_THE_OTHER: &str = "One will overwrite the other.";
/// **`Job files and outputs` H1's third condition, which Design draws no
/// sentence for**: *"stop, or carry on knowing that part of what the job writes
/// will land somewhere Smoozy is not watching and will not bring home"*
/// (`Specs/What a job writes`). **Ruled at this site** in Rule 9's words, the
/// pair and its note being Design's (the 38's piece 10, item 26).
const PART_OF_IT_LANDS_OUTSIDE_ITS_FOLDER: &str =
    "Part of this job's output is written outside its folder.";
const THOSE_FILES_ARE_NOT_DOWNLOADED: &str = "Those files are not downloaded.";

// ── The marks — Design, *The marks* ────────────────────────────────────────

const ABOUT_THIS_JOB: &str = "About this job";
const A_FILE_THIS_JOB_NEEDS_WAS_REPLACED: &str = "A file this job needs was replaced.";
const IT_MAY_HAVE_BEEN_REPLACED: &str = "A file this job needs may have been replaced.";
const WAS_REPLACED_AFTER_SUBMISSION: &str = "was replaced after this job was submitted.";
const NOTHING_CHECKED_THIS_FILES_DEPENDENCIES: &str =
    "Dependencies were not checked before submission.";
// **Four unread copies were deleted here at step 13's words sweep
// (2026-10-28)** rather than reworded twice: the two dependency-check marks,
// the earlier-job mark's lead and the reused-frames mark. Their one live home
// is `src/surface/the_history.rs`.
/// **`Queue & History` N20's way to the earlier entry**, the drawing's own
/// words (`Design/Queue & History § The marks`).
pub(crate) const THAT_JOB: &str = "That job";
const SUBMITTED_WITH_NO_SCENE_NAMED: &str = "Submitted with no scene named.";
const BLENDER_RENDERED_LEAD: &str = "Blender rendered";
const THE_FILES_ACTIVE_SCENE: &str = ", the file's active scene.";
const IT_RENDERS_WHICHEVER_SCENE: &str =
    "It renders the file's active scene.";
const THE_FILE_CHANGED_BEFORE_ATTEMPT: &str = "The file changed before attempt";
const FRAMES_ALREADY_FINISHED_WERE_RENDERED_AGAIN: &str =
    "Frames already finished were rendered again.";

// ── What the job writes — Design, *What the job writes* ────────────────────

const WHAT_THIS_JOB_WRITES: &str = "What this job writes";
/// **The six statements as the drawing words them on the entry** — the 38's
/// piece 10. Where the drawing's sentence and the settings page's say the same
/// fact the page's own string is used (`crate::surface::render_settings::words`),
/// one fact, one wording.
const WRITING_INTO: &str = "Writing into";
const THE_JOB_FOLDER: &str = "the job folder";
const A_COMMA: &str = ",";
const A_DASH: &str = "—";
const NO_FRAME_HAS_AN_ACTIVE_VIEW_LAYER: &str =
    "No frame in the range has an active View Layer, so nothing renders.";
/// **`Job files and outputs` H1a's other reason**, which the drawing does not
/// word: nothing the render was asked for is aimed anywhere that writes.
/// Ruled at this site.
const THE_OUTPUT_SETTINGS_SAVE_NOTHING: &str = "The file's output settings save nothing.";
const IS_WRITTEN_BY_BOTH: &str = "is written by both; one overwrites the other.";
const SMOOZY_CHANGES_NOTHING_IN_YOUR_FILE: &str = "Your file is not changed.";
const SHARES_ITS_COMPOSITOR_WITH: &str = "This scene shares its compositor with";
const THE_PICTURES_MAY_BELONG_TO_THAT_SCENE: &str =
    "The images may come from that scene, not this one.";

// ── The setup — Design, *The rest of the entry* ────────────────────────────

const SUBMITTED_WITH: &str = "Submitted with";
const THE_LEGEND_ITALIC: &str = "italic";
const THE_LEGEND_THE_FILES: &str = "= the file's ·";
const THE_LEGEND_BOLD: &str = "bold";
const THE_LEGEND_YOURS: &str = "= yours";
const BLENDER_FACT: &str = "Blender";
const FRAMES_FACT: &str = "Frames";
const RESOLUTION_FACT: &str = "Resolution";
const SAMPLES_FACT: &str = "Samples";
const ENGINE_FACT: &str = "Engine";
const WRITING_TO_FACT: &str = "Writing to";

// ── Where the job's folder sits — Design, *The job's name, and its folder* ──
//
// **The one or two lines under *Writing to*'s path where the folder fell back**
// (`Job files and outputs` P19's two cases, `Job files and outputs` P23's passed-over
// mark), and the line a waiting job draws where its folder is not made yet
// (`Job files and outputs` P21). Design's
// own words. They stay into History, which draws the same entry. The
// Creator's second notes, piece 4's queue half.

/// `Job files and outputs` P19's case 1.
pub(crate) const THE_FILES_FOLDER_IS_ON_ANOTHER_COMPUTER: &str =
    "Your file’s output folder is on another computer, so the files go here.";
/// `Job files and outputs` P19's case 2.
pub(crate) const SMOOZY_CANT_USE_THE_FILES_FOLDER: &str =
    "Your file’s output folder can’t be used, so the files go here.";
/// `Job files and outputs` P23: the server's chosen fallback folder was passed over.
pub(crate) const THE_SERVERS_FALLBACK_FOLDER_WAS_PASSED_OVER: &str =
    "This server’s fallback folder can’t be used, so the files go to Smoozy’s folder.";

const THE_FILES_VALUE_LEAD: &str = "file:";
const NOT_KNOWN: &str = "not known";

// ── Blender's own words — Design, *The rest of the entry* ──────────────────

/// **The control that opens the record, and nothing beside it.** M13b: no line
/// of either stream is lifted out of it. The panel behind this control is
/// `src/presentation/the_machines_own_words.rs`'s and is already built.
const BLENDER_OUTPUT: &str = "Blender output";

// ── The preview column — Design, *The rest of the entry* ───────────────────

const FRAME_HEADING: &str = "Frame";
/// **The preview block's heading, and the words of its controls** — `Design/
/// Queue & History`, *How the stepping and the View Layers look*; `Queue &
/// History` P2 and P10a. The frame number between the arrows is
/// [`FRAME_HEADING`]'s word and the figure. The Creator's seventh notes, note 4.
const FRAME_PREVIEW: &str = "Frame preview";
const PREVIOUS_FRAME: &str = "Previous frame";
const NEXT_FRAME: &str = "Next frame";
const VIEW_LAYER: &str = "View Layer";
/// **A frame's main picture that is no one View Layer's is named by where it
/// came from** — *Output path*, or *File Output node* with the node's own name
/// beside it as a value, never joined into the words (X7, the collision line's
/// own rule) — and listed first. `Design/Queue & History` line 201; the
/// drawing's `q-pvsrc`; the Creator's tenth notes, note 2.
const OUTPUT_PATH: &str = "Output path";
const FILE_OUTPUT_NODE: &str = "File Output node";
const SHOW_LATEST: &str = "Show latest";
const SHOW_LAST: &str = "Show last";
const SHOWING_FRAME: &str = "Showing frame";
const ENTER_A_WHOLE_FRAME: &str = "Enter a whole frame number.";
const FRAME_NOT_AVAILABLE: &str = "is not available.";
const ENLARGE_PREVIEW: &str = "Enlarge preview";
const PREVIEW_CLOSE: &str = "Close";
const PREVIEW_NO_LONGER_AVAILABLE: &str = "This preview is no longer available.";
const FIRST_FRAME_CHECK: &str = "First-frame check · Frame";
const REVIEW_FRAME: &str = "Review frame";

// ── A first-frame check and its job — the nineteenth notes, note 7 ─────────
//
// `Design/Queue & History § A first-frame check and its job`, Creator-stated
// 2026-10-27: a check and the full range waiting on it are one row and one
// entry, in Queue and in History. The drawing's states are `q`, `q-checkjob`,
// `q-first`, `q-checkrun`, `h-checkjob`, `h-checkonly` and `h-checkwait`.

/// **The row's two part lines**, each before its state: *First-frame check ·
/// frame 1* and *Full range · frames 1–240*. Lower case on the row, as drawn;
/// the box on the entry is headed [`FIRST_FRAME_CHECK`].
pub(crate) const FIRST_FRAME_CHECK_ON_THE_ROW: &str = "First-frame check · frame";
const FULL_RANGE_ON_THE_ROW: &str = "Full range · frames";
/// A part's state: hollow dot before it starts, amber while it renders, green
/// when done; once the full range renders, the frame it is on.
const PART_NOT_STARTED: &str = "not started";
const PART_RENDERING: &str = "rendering";
const PART_RENDERED: &str = "rendered";
const PART_AT_FRAME: &str = "frame";
/// **While the check renders**, the box above the status line: *Frames 1–240
/// start only after you review this frame. Stopping the check removes them.*
const FRAMES_LEAD: &str = "Frames";
const START_ONLY_AFTER_YOU_REVIEW: &str =
    "start only after you review this frame. Stopping the check removes them.";
/// **While it waits**, the question in the box beside the picture: *Frame 1
/// downloaded. Render frames 1–240?* — *downloaded* only where it was
/// (`Download` R4), *rendered* otherwise. The phone's own question since
/// 2026-11-06 (the last words read before the release; it read *Frame 1 is
/// rendered and downloaded. Review it and choose whether to render frames
/// 1–240.*).
const IS_RENDERED_AND_DOWNLOADED_REVIEW_IT: &str = "downloaded.";
const IS_RENDERED_REVIEW_IT: &str = "rendered.";
const REVIEW_IT_AND_CHOOSE: &str = "Render frames";
const THE_QUESTION_MARK: &str = "?";
/// **Once the full range renders**: *Rendered in 2m. You chose **Render
/// all**.* — the duration in History's own form ([`crate::surface::the_history`]).
const RENDERED_IN: &str = "Rendered in";
const YOU_CHOSE: &str = "You chose";
/// **History, while the full range is still live**: *Frames 1–240 are in
/// Queue.* with *Show in Queue*.
const ARE_IN_QUEUE: &str = "are in Queue.";
const SHOW_IN_QUEUE: &str = "Show in Queue";
/// **History, a check whose full range never rendered.**
const THE_FULL_RANGE_WAS_NOT_RENDERED: &str = "The full range was not rendered.";
/// **What History's row reads where the ending goes**, in grey: the check
/// whose full range is still live, and the check alone after *Cancel*.
pub(crate) const IN_QUEUE: &str = "In Queue";
pub(crate) const CHECK_ONLY: &str = "Check only";
/// **One line where the picked View Layer did not render on the frame shown**
/// — the layer's own name goes before it: *Reflections did not render on this
/// frame.*
const DID_NOT_RENDER_ON_THIS_FRAME: &str = "did not render on this frame.";
const LATEST: &str = "latest";
const NOT_COLOUR_ACCURATE: &str = "Not colour-accurate.";
const NO_FRAME_FINISHED: &str = "No frames finished.";
/// **`Design/Queue & History § The entry's controls`: the group's heading**,
/// and the grey line on a server whose Smoozy is older. The twenty-first
/// notes, note 7.
const TELEGRAM_HEADING: &str = "Telegram";
const UPDATE_SMOOZY_ON_THIS_SERVER_TO_CHANGE_THIS: &str = "Update Smoozy on this server to change this.";
const WHAT_THIS_JOB_REPORTS: &str = "What this job reports";
/// **The narrowing control's words where the arrangement takes every format**
/// (`Download` N3) — the 38's piece 9.
const ALL_FORMATS: &str = "All formats";
const DOWNLOADED_WORD: &str = "downloaded";

// ── The catch-up notice — Design, *The catch-up notice* ────────────────────
//
// **A count, not a banner** — the Creator's ninth notes (2026-10-11), note 3:
// the endings are counted at the end of the Queue heading's grey line, and a
// job dropped before it rendered is named in one line under it. Until then
// this block also held the banner's heading, *What happened to your jobs*, its
// *Hide* and *Show*, the re-run line and *Show in History*; each went with
// the banner.

/// **Design, *The catch-up notice*: the count's words, one per ending** —
/// *finished*, *failed*, *stopped*, *abandoned*, lower case as the drawing's
/// *· 3 finished · 1 failed* writes them. `Queue & History` C1's endings; a
/// re-run is not one (*"Re-runs are said by the entry's mark only"*).
const ENDED_FINISHED: &str = "finished";
const ENDED_FAILED: &str = "failed";
const ENDED_STOPPED: &str = "stopped";
const ENDED_GIVEN_UP: &str = "abandoned";
/// **The count's hover** — the drawing's own `title`, and Design's words.
const SINCE_YOU_LAST_OPENED_HISTORY: &str = "Since you last opened History.";
/// **Design, *Jobs that leave without an entry*: a held job the machine
/// dropped says the first frame was never rendered** — the drawing's own
/// words after the job's bold name, *Kitchen — final v4 was not started — the
/// first frame was never rendered.* `Queue & History` H11's one telling.
const WAS_NOT_STARTED_THE_FIRST_FRAME_WAS_NEVER_RENDERED: &str =
    "was not started — the first frame was never rendered.";
/// **`Queue & History` L22's line: a waiting job dropped because the Blender it
/// was submitted with was removed**, named with that version — Design's own
/// *…was not started — its Blender 4.5 was removed.* The version is a value
/// between the two.
const WAS_NOT_STARTED_ITS_BLENDER: &str = "was not started — its Blender";
const WAS_REMOVED: &str = "was removed.";
/// **The same where the machine's drop words carried no version**, so the
/// line says only what is known.
const WAS_NOT_STARTED_ITS_BLENDER_WAS_REMOVED: &str = "was not started — its Blender was removed.";
const DISMISS: &str = "Dismiss";

// ── Work Smoozy is not tracking — Design, *Work Smoozy is not tracking* ────

const IS_RUNNING_WORK_SMOOZY_IS_NOT_TRACKING: &str =
    "has untracked jobs";
const IF_YOU_LEAVE_IT: &str =
    "If ignored, new jobs wait behind them.";
const TRACK_IT: &str = "Track them";
/// **`Queue & History` A6's write refused: nothing of the offer was
/// adopted**, all or nothing. Ruled at this site: Design gives no words for
/// it. Owed piece 10.
const THE_WORK_COULD_NOT_BE_TRACKED: &str = "Could not track these jobs. Nothing was changed.";
const IGNORE: &str = "Ignore";
/// **Must-do 19: the ending says a cause only where the reading carries one**,
/// and each cause has its own. `Specs/Queue & History` gives two: a server
/// removed, and a host or port edited while jobs ran. Each ending closes the
/// sentence the two figures open, so it carries its own comma.
/// **`Queue & History` A4: where *Ignore* stands, a job waiting behind the
/// work it declined says why it waits** — the fact of it and the count of it,
/// never the jobs themselves (`Specs/Queue & History`: *"Smoozy says that the
/// machine is busy with work the user chose not to track … so the wait has a
/// stated cause"*), in the banner's own *untracked jobs*. The
/// 38's piece 15.
const THE_SERVER_IS_BUSY_WITH: &str = "The server is busy with";
const JOBS_SMOOZY_IS_NOT_TRACKING: &str = "untracked jobs.";
const JOB_SMOOZY_IS_NOT_TRACKING: &str = "untracked job.";
const FROM_BEFORE_THIS_SERVER_WAS_REMOVED: &str =
    ", from before this server was removed.";
const FROM_BEFORE_ITS_ADDRESS_WAS_CHANGED: &str =
    ", from before this server's address was changed.";

// ── The part-submission banner — the mockup's `#qpart` ─────────────────────

const OF_SCENES_ARE_CONFIRMED: &str = "scenes are confirmed submitted";
const THE_CONNECTION_DROPPED_PART_WAY: &str =
    "Connection lost during submission.";
const SUBMITTED_LIST: &str = "Submitted";
const NOT_SUBMITTED_LIST: &str = "Not submitted";
const NOT_KNOWN_LIST: &str = "Not known";
const SMOOZY_COULD_NOT_READ_THE_SERVER: &str =
    "Could not read the server to confirm these.";
const SUBMIT_THE_REMAINING_SCENES: &str = "Submit the remaining scenes";
/// **The part banner's own *Hide*** — the drawing's `#qpart`. It sat in the
/// catch-up notice's block, which shared it, until that banner went.
const HIDE: &str = "Hide";

// ── The boxes that ask — Design, *The entry's controls* ────────────────────

const STOP_THIS_JOB_QUESTION: &str = "Stop this job?";
/// **What a stop may cost, and never more than Smoozy can stand behind.**
/// `Servers` W19a: a stop lands in one of exactly two places and nobody can
/// know which before the act — mid-render the frame never arrives, and during
/// that frame's own write the write finishes and the frame is whole — so the
/// statement is that the frame **may not be saved**, *"and never that it is
/// lost"*. **These are `Servers`' own words**, as
/// `src/surface/servers.rs`'s stop-and-clear statement already says them, and a
/// test holds the two together; one fact gets one wording (Design, *The entry's
/// controls*). *(Runner's item R1, 2026-09-17, overturning this round's
/// Verdict's own ruling under Rule 10.)*
const THE_FRAME_IT_IS_ON_MAY_NOT_BE_SAVED: &str = "The current frame may not be saved.";
const FRAMES_ALREADY_RENDERED_STAY: &str =
    "frames already rendered stay on the server.";
const FRAME_ALREADY_RENDERED_STAYS: &str =
    "frame already rendered stays on the server.";
const THE_WORD_THE: &str = "The";
/// **The same sentence where no count is known** — step 10c's S3.
const FRAMES_ALREADY_RENDERED_STAY_WITH_NO_COUNT: &str =
    "Frames already rendered stay on the server.";
const KEEP_RENDERING: &str = "Keep rendering";

const CANCEL_THIS_JOB_QUESTION: &str = "Cancel this job?";
const NO_FRAMES_ARE_LOST: &str =
    "It has not started, so no frames are lost. It will not appear in History.";
const ITS_SETTINGS_GO_TOO: &str =
    "Its settings are deleted too.";
const KEEP_THE_JOB: &str = "Keep the job";
const CANCEL_THE_JOB: &str = "Cancel the job";

const CANCEL_ALL_QUESTION_LEAD: &str = "Cancel all";
const JOBS_QUESTION_TRAIL: &str = "jobs?";
const JOB_QUESTION_TRAIL: &str = "job?";
/// **What the group's box says about the member rendering now**, with the
/// job's name as a value in front of it. **It is one sentence and stops**:
/// what a stop may cost is [`THE_FRAME_IT_IS_ON_MAY_NOT_BE_SAVED`], composed
/// beside it rather than written out again. ~~This string carried that sentence
/// as a second literal~~ — struck as the runner's item R3, 2026-09-17, on the
/// neighbouring round's close check, which found R1's one fact written in two
/// places within the same file. `a_stop_never_claims_the_frame_is_lost` now
/// fails if the two ever drift apart.
const IS_RENDERING_AND_WILL_BE_STOPPED: &str = "is rendering and will be stopped.";
const THE_OTHERS_HAVE_NOT_STARTED: &str =
    "The others have not started, so no frames are lost. None will appear in History, and their settings are deleted.";
/// **Must-do 11, and `Design/Queue & History § The queue list`: a group with no
/// rendering member** says none of them has started and no frames are lost, in
/// the single cancel box's own words.
const NONE_OF_THEM_HAS_STARTED: &str =
    "None has started, so no frames are lost. None will appear in History, and their settings are deleted.";
const KEEP_THEM: &str = "Keep them";

const DISMISS_THIS_NOTICE_QUESTION: &str = "Dismiss this notice?";
const WAS_NEVER_STARTED_AND_HAS_NO_ENTRY: &str =
    "was never started and is not in History. Dismissing removes its only record.";
const KEEP_THE_NOTICE: &str = "Keep the notice";

// ── The queue's own acts on the machine — step 10c's S3 ────────────────────

/// **`Queue & History` L12a's *stopping* and L2's *cancelling*** — Smoozy's
/// statement about its own act while the command is in hand, never a queue
/// state. **Ruled at this site**: Design leaves the wording to this stage
/// (L12a: *"Wording and presentation are Design's"*, and Design draws no
/// word), and these are the industry's own, in the state column's shape.
const STOPPING: &str = "Stopping";
const CANCELLING: &str = "Cancelling";
/// **`Rendering` K10, in Design's own words** (*The entry's controls*: *"Stop
/// sent. Timed out waiting for the render to end."*). It claims neither a
/// stop that failed nor a render slow to honour it.
const THE_STOP_WAS_SENT: &str =
    "Stop sent. Timed out waiting for the render to end.";
/// **What to do next, said only where *Stop* is offered beside it** —
/// `Design/Queue & History § The entry's controls` (added 2026-10-01, step
/// 10c's thirteenth pass). A second stop sends a second interrupt (K10's
/// fresh command), which ends a render that took the first as *skip this
/// frame*. **No *now***: the second stop meets the same wait, and nothing
/// Smoozy records says it ends at once. It says nothing of the first stop
/// (K11).
const PRESS_STOP_AGAIN: &str = "Press Stop again to end it.";
/// **The cancel's waiter gave up**: the same shape, claiming no more than it
/// knows. Ruled at this site.
const THE_CANCEL_WAS_SENT: &str =
    "Cancel sent. Timed out waiting for a reply.";
/// **The machine recorded *the stop failed***, with its own words beside it.
const THE_RENDER_WAS_NOT_STOPPED: &str = "The render was not stopped.";
/// **`Queue & History` L1 and L4: *not cancelled***, with the machine's own
/// words beside it saying why.
const THE_JOB_WAS_NOT_CANCELLED: &str = "The job was not cancelled.";
/// **Nothing reached the machine.**
const SMOOZY_COULD_NOT_REACH_THE_SERVER: &str =
    "Could not reach the server. Nothing was changed.";
/// **The machine was reached and did not take it** — an older Smoozy on the
/// server, or a refusal before anything was recorded. **Never *could not
/// reach***, which would be false of it (the plan pass's SC-2, for the pair).
const THE_SERVER_DID_NOT_ACCEPT_IT: &str = "The server rejected this. Nothing was changed.";
/// **`Queue & History` L16 in the group's form.**
const THE_WAITING_JOBS_COULD_NOT_BE_CANCELLED: &str =
    "The waiting jobs could not be cancelled. The render was not stopped.";
/// **What a press answers where nothing was sent at all.**
const THIS_JOB_IS_NO_LONGER_IN_THE_QUEUE: &str = "This job is no longer in the queue.";
/// **`Telegram` V1c: the server held the job and did not change it** — it
/// ended while the change travelled, or the server holds no such job. Ruled at
/// this site, the twenty-first notes, note 7: Design gives no words for it.
const THE_CHOICE_WAS_NOT_CHANGED: &str = THIS_JOB_IS_NO_LONGER_IN_THE_QUEUE;
/// **`Telegram` V1c: the change's waiter gave up**, in the move's shape.
const THE_CHANGE_WAS_SENT: &str = "Change sent. Timed out waiting for a reply.";
const THESE_JOBS_ARE_NO_LONGER_IN_THE_QUEUE: &str = "These jobs are no longer in the queue.";
const THIS_JOB_IS_NOT_RENDERING: &str = "This job is not rendering.";

// ── The held job's two answers, on the machine — step 10c's eleventh-pass fix piece 8 ──

/// **The answer's waiter gave up** — the same shape as the cancel's, claiming
/// no more than it knows. Ruled at this site: Design gives no words for it.
const THE_ANSWER_WAS_SENT: &str =
    "Answer sent. Timed out waiting for a reply.";
/// **`Rented-server runtime` U14: *Render all* met a *no* recorded
/// first** — from a phone, or a *Cancel* whose cancel did not land. The
/// loser is told what stood rather than that it failed. **Cancelled** is the
/// user's own word for it: `Specs/Queue & History` makes cancelling the job and
/// saying no one act. Ruled at this site.
const THIS_JOB_WAS_ALREADY_CANCELLED: &str = "This job was already cancelled.";

// ── What a control that cannot act answers — must-do 5 ─────────────────────

/// **A control that needs the server instructed** answers in the one sentence
/// Files, Set up, the render settings and the dependency check already say —
/// one constant, [`crate::surface::files::A_CONNECTION_IS_NOT_BUILT`], and no
/// second copy. **Its name is older than its words**: the sentence was reworded
/// on 2026-09-18 by the build runner's ruling, opening a connection having been
/// built, and the constant kept its name so the five screens stayed one edit.
const A_CONNECTION_IS_NOT_BUILT: &str = crate::surface::files::A_CONNECTION_IS_NOT_BUILT;

// **The sentence about the history store is gone, and this is where it was.**
// It said that keeping a job's history was not built, and it is not written out
// again here because the guard below counts every copy of one of these
// sentences in this file and a struck one is still a copy. Step 10b piece 6
// landed the local job record, and the three acts that answered for that reason
// — the delete box's *Delete* and the two clearing boxes — are acts now.
// **What is left below is not one sentence with fewer users; it is three
// different missing things, each named as itself**, because the controls that
// kept answering were not waiting on the store at all and one reworded sentence
// would have said the same wrong thing in new words.

/// **The way from a mark to the entry it names** — `Queue & History` N20 —
/// **where that entry was deleted between the draw and the press**. The way
/// is drawn only where the earlier record stands (`Queue & History` N20's
/// draw-time lookup), so this is the one thing a press can meet. *(Until the
/// 38's piece 2 this said opening the earlier entry was not built.)*
pub(crate) const THAT_ENTRY_WAS_DELETED: &str = "That entry was deleted.";

// ───────────────────────────────────────────────────────────────────────────
// The reading
// ───────────────────────────────────────────────────────────────────────────

/// **One piece of a line: words, a value, words.** Any may be empty.
///
/// **This is the whole answer to X7's trap.** A sentence with a name or a
/// figure in it never becomes one string: the words are prose that went through
/// the typography point and the value went by the verbatim route, and the page
/// draws the three in order. `format!("{} could not be reached", name)` cannot
/// be written with this type.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct APart {
    pub lead: &'static str,
    pub value: String,
    pub trail: &'static str,
}

impl APart {
    pub fn words(lead: &'static str) -> APart {
        APart {
            lead,
            ..APart::default()
        }
    }

    pub fn value(value: impl Into<String>) -> APart {
        APart {
            value: value.into(),
            ..APart::default()
        }
    }

    pub fn figure(value: impl Into<String>, trail: &'static str) -> APart {
        APart {
            value: value.into(),
            trail,
            ..APart::default()
        }
    }

    pub fn said(lead: &'static str, value: impl Into<String>, trail: &'static str) -> APart {
        APart {
            lead,
            value: value.into(),
            trail,
        }
    }

    fn is_empty(&self) -> bool {
        self.lead.is_empty() && self.value.is_empty() && self.trail.is_empty()
    }
}

/// One server's line in the queue list — Design, *The queue list*: grouped by
/// server, under a heading carrying that server's badge.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AServersLine {
    /// **The local server identifier** — what the page keeps this server's
    /// fold under for the run (must-do 7). A value, and never shown.
    pub row: String,
    /// The server's own label, as the record copied it at submission
    /// (`Queue & History` N12). A
    /// value.
    pub name: String,
    /// The badge hue as the row carried it at submission (`Queue & History` N12).
    /// **Absent draws
    /// no badge and never a default colour** (`Queue & History` N13).
    pub badge: Option<String>,
    /// **`Queue & History` C7: the reading is last-known, so the heading says it is not updating
    /// and when it was last seen.** A value, on the machine's own clock.
    pub last_seen: Option<String>,
    pub rendering: u32,
    pub waiting: u32,
    /// What the folded heading says — the job's name and how far through it is
    /// (Design: a folded heading still says what is running).
    pub folded: Option<(String, u32)>,
    pub rows: Vec<ARowInTheQueue>,
}

/// **A row's two bars, each a whole percentage and each absent on its own** —
/// the frame in hand's (`now_at`) and the whole job's (`whole`). W4,
/// 2026-10-18.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TheRowsBars {
    pub now_at: Option<u32>,
    pub whole: Option<u32>,
}

impl TheRowsBars {
    /// Both bars, where both are known.
    pub fn both(now_at: u32, whole: u32) -> Self {
        Self {
            now_at: Some(now_at),
            whole: Some(whole),
        }
    }
}

/// One row of the queue list.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ARowInTheQueue {
    /// **The job's own identity** — `LocalJobRecord::job`, as the entry carries
    /// it. A value, and never shown. **It is what choosing the row hands back**,
    /// so the entry beside the list is asked for by the job and not by its
    /// label, which the user may have typed twice.
    pub job: String,
    /// The job's whole label as the user typed it (`Queue & History` Y8). A value.
    pub name: String,
    /// The second line, in parts. Bounded at three by the drawing.
    pub sub: Vec<APart>,
    /// **A first-frame check and its full range are one row**, one line per
    /// part under the name (`Design/Queue & History § A first-frame check and
    /// its job`). Empty on every other row, a legacy check among them.
    pub the_pair: Vec<APartOfThePair>,
    /// *Rendering*, *Needs your answer*, *Not updating*, or nothing — Design's
    /// three states that are not readable from position.
    pub state: Option<&'static str>,
    /// The state's own colour class, which is the drawing's.
    pub state_is: &'static str,
    /// The two bars, stacked and unlabelled — Design, *The queue list*. Each is
    /// a whole percentage and both are absent on a row that is not rendering.
    /// **Each is there on its own** — the frame's bar and the whole job's —
    /// so a row draws each bar it has (W4, 2026-10-18, the Creator's twelfth
    /// notes, note 7). Until then the row carried both or neither, so a frame
    /// not yet sampled took the whole job's bar away with its own.
    pub bars: TheRowsBars,
    /// **`Queue & History` L9 and Design: the rendering row cannot be dragged and does not offer
    /// the grip**; a held job is not in the queue at all.
    pub grip: bool,
    /// Which group of scene-jobs this row belongs to, if any — an index into
    /// [`TheQueueOnScreen::groups`], where the group's own heading is.
    pub group: Option<usize>,
    /// **`Queue & History` C7 again: a last-known row is greyed.**
    pub not_updating: bool,
    /// **`Queue & History` C10: a job just submitted, while its act runs** —
    /// the *Submitting…* row, with no grip, bars, place or menu, and no job
    /// identity. The Creator's seventh notes, piece Z7.
    pub submitting: bool,
    /// **`Queue & History` C10 as changed 2026-10-19: a submission that ended
    /// with a job not in the machine's line**, saying what the panel says,
    /// with a cross. Absent on every other row.
    pub not_submitted: Option<ARowNotSubmitted>,
}

/// **One part of a first-frame pair's row** — *First-frame check · frame 1*
/// or *Full range · frames 1–240*, then its state, with a dot before it:
/// hollow before it starts, amber while it renders, green when done.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct APartOfThePair {
    pub what: APart,
    /// *not started*, *rendering*, *rendered*, or the frame the full range is
    /// on. **Empty where Smoozy learned no ending for a check that left the
    /// line**: nothing is said that it does not know.
    pub state: APart,
    /// The dot's class, which is the drawing's: `""`, `now` or `done`.
    pub dot: &'static str,
}

/// **The check on its pair's entry** — `Design/Queue & History § A first-frame
/// check and its job`. Which words it carries says which of the drawing's
/// readings it is; the page draws what arrives and decides nothing.
///
/// - **Queue, while the check renders** (`q-checkjob`): above the status line,
///   *Frames 1–240 start only after you review this frame. Stopping the check
///   removes them.*
/// - **Queue, while it waits** (`q-first`): inside the question box, the
///   picture beside the question and its two answers.
/// - **Queue once the full range renders, and History's pair** (`q-checkrun`,
///   `h-checkjob`): the picture, small, *Rendered in 2m. You chose **Render
///   all**.*, and *Blender output* for the check's own words.
/// - **History, the full range still live** (`h-checkwait`): *Frames 1–240 are
///   in Queue.* and *Show in Queue*.
/// - **History, the check alone** (`h-checkonly`): *The full range was not
///   rendered.*
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TheCheckOnTheEntry {
    /// **The check's own identity** — whose picture the box shows and whose
    /// words its *Blender output* opens. A value, never shown.
    pub job: String,
    /// *First-frame check · Frame 1*.
    pub heading: APart,
    /// The box's sentence, in parts. Empty in the question box, whose sentence
    /// is the held pair's own.
    pub said: Vec<APart>,
    /// ***Rendered in 2m***, where the check's record holds exactly one
    /// duration (`Queue & History` N4). A value.
    pub rendered_for: Option<String>,
    /// **History only: when the check ended**, in seconds, which the page
    /// writes as it writes every moment.
    pub when: Option<String>,
    /// ***You chose Render all.*** — true whichever route the answer came by,
    /// because the full range renders on no other answer.
    pub you_chose_render_all: bool,
    /// **The check's picture, in the box** — absent where the drawing draws
    /// none: while the check renders, and on History's check alone or still
    /// waiting, whose own entry already shows it down the side.
    pub picture: Option<ThePreview>,
    /// **The check's own *Blender output***, where its record keeps its words
    /// (`Queue & History` E9).
    pub blender_output: bool,
    /// ***Show in Queue***, naming the full range's job to choose there.
    pub show_in_queue: Option<String>,
    /// **The box is the question's**: the picture and heading go inside the
    /// held pair's box, and the side column draws no preview.
    pub in_the_question: bool,
}

/// **A row saying *Not submitted.* and why** — the sentence the Render
/// Settings panel says for the act
/// ([`crate::surface::render_settings::the_acts_answer`]), split where the
/// row draws it: what went, in the state's place and its red, and why, under
/// the name. **And what the cross hands back**: the act it is the row of.
/// Out of the live pass on Vast for the thirteenth and fourteenth notes,
/// fault 1.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ARowNotSubmitted {
    /// The act, as [`crate::submitting::AnActInFlight::which`] knows it. A
    /// value, and never shown.
    pub act: u64,
    /// *Not submitted.*, or *1 of 2 jobs submitted.* — the sentence's leading
    /// parts.
    pub what_went: Vec<crate::surface::servers::Words>,
    /// The reasons, each once, in the order the act met them.
    pub why: Vec<crate::surface::servers::Words>,
}

/// **The row for an act that ended not submitted**, from what it came to: the
/// panel's own sentence, its plain parts — or its first, *Not submitted.* —
/// in the state's place, and the rest under the name.
fn a_row_not_submitted(act: &crate::submitting::AnActInFlight, came_to: &crate::submitting::WhatTheActCameTo) -> ARowInTheQueue {
    let (mut said, plain) = crate::surface::render_settings::the_acts_answer(came_to);
    let why = said.split_off(plain.max(1).min(said.len()));
    ARowInTheQueue {
        name: act.name.clone(),
        state_is: "bad",
        not_submitted: Some(ARowNotSubmitted {
            act: act.which,
            what_went: said,
            why,
        }),
        ..ARowInTheQueue::default()
    }
}

/// A group of scene-jobs, under its own heading (`Queue & History` L20).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AGroupHeading {
    /// **The group's own identity** (`Queue & History` L18), as text. A value,
    /// and never shown: it is what *Cancel all* hands back, so the act is
    /// asked for by the group and not by its name. Step 10c's S3.
    pub group: String,
    /// The submission's own label. A value.
    pub name: String,
    /// **How many of its members this reading holds** — the *N* of *Cancel all
    /// N*.
    pub scenes: u32,
    /// **The member rendering now, by its scene's name**, where one is — what
    /// the group's box names as the one that will be stopped (Design, *The
    /// queue list*). Not sent on the event: only the box reads it.
    pub rendering: Option<String>,
    /// **Design: the group's heading is the grip for the whole group**, and
    /// it offers one where a member is waiting and could be moved
    /// (`Queue & History` L7). The 38's piece 8.
    pub grip: bool,
}

/// **The catch-up notice — `Queue & History` C2 and `Queue & History` C2a, and
/// Design, *The catch-up notice***: since the Creator's ninth notes
/// (2026-10-11), note 3, **a count of the endings in the Queue heading's grey
/// line, and one line per job dropped before it rendered**. Composed at draw
/// time over every standing notice record and stored nowhere (`Queue &
/// History` C2a). *(Until then it was a banner, one group per server, each
/// line a job's name and what happened to it.)*
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TheCatchUpNotice {
    /// **The endings counted, by kind, in Design's order** — finished,
    /// failed, stopped, given up — **and only kinds above nought**. Each
    /// counts a job whose History entry stands, since pressing the count
    /// opens History, where every one of those jobs is.
    pub counted: Vec<(smoozy_queue_history::ending::Ending, u32)>,
    /// **One line per job dropped before it rendered** (`Queue & History` H11
    /// and `Queue & History` L22): it has no History entry, so its line is
    /// the only place it is ever named, and its *Dismiss* asks first.
    pub dropped: Vec<ADroppedJob>,
}

/// **One dropped job's line under the Queue heading** — Design, *Jobs that
/// leave without an entry*: the job's name in bold, then what happened to it.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ADroppedJob {
    /// **The job's identity**, as the notice record keeps it beside the line —
    /// what the line's *Dismiss* hands back. A value, and never shown.
    pub job: String,
    /// The job's name, as the notice record kept it before the drop took the
    /// local record. A value.
    pub name: String,
    pub what: APart,
}

/// Work Smoozy is not tracking — `Queue & History` A5 to `Queue & History` A9, and
/// Design's own section.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TheUntrackedWork {
    /// The server's name, as the banner draws it. A value.
    pub server: String,
    /// **The local server identifier the reading carries** — what a decline is
    /// keyed on, and never shown (step 10b round 1 of *jobs, history and
    /// downloads*, must-do 6). Two servers can carry one name; they never carry
    /// one identifier.
    pub server_row: String,
    pub badge: Option<String>,
    pub rendering: u32,
    pub waiting: u32,
    /// **Which of the spec's two causes left it, where the reading says** —
    /// absent draws no cause at all (must-do 19).
    pub cause: Option<WhyItIsUntracked>,
}

/// `Specs/Queue & History`'s two ways work is left untracked.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhyItIsUntracked {
    TheServerWasRemoved,
    ItsAddressWasChanged,
}

/// The all-scenes submission the connection broke part-way — the mockup's
/// `#qpart`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ThePartSubmission {
    pub confirmed: u32,
    pub of: u32,
    /// The server the submission was going to. A value.
    pub server: String,
    /// **That server's identifier**, which the control hands back. Never
    /// shown.
    pub the_servers_row: String,
    /// **Whether *Submit the remaining scenes* can go** — the server is
    /// connected and some scene is resolved as not submitted (`Rendering`
    /// `Rendering` S11). Otherwise it reads unavailable, needing the connection.
    pub it_can_go: bool,
    /// The submission's own label. A value.
    pub submission: String,
    pub submitted: Vec<String>,
    pub not_submitted: Vec<String>,
    pub not_known: Vec<String>,
}

/// What a job's entry says. **The same shape serves both screens**, the history
/// entry adding its ending and its two ways to render again.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TheJobEntry {
    /// **The job's own identity** — `LocalJobRecord::job` in `Queue & History`
    /// N10. A value, and never shown. **It is what the entry's two download
    /// acts and its narrowing hand Transfers**, so a download is asked for by
    /// the job rather than by its label, which the user may have typed twice.
    pub job: String,
    /// **The check on its pair's entry**, where the entry is a first-frame
    /// pair's ([`TheCheckOnTheEntry`]). The nineteenth notes, note 7.
    pub the_check: Option<TheCheckOnTheEntry>,
    /// The job's whole label (`Queue & History` Y8). A value.
    pub name: String,
    pub server: String,
    pub badge: Option<String>,
    /// The root `.blend` file's machine path as the submission named it
    /// (`Queue & History` N11).
    /// A value.
    pub blend: String,
    /// **`Queue & History` N11 and Design: where no scene was named the field says so.**
    pub scene: Option<String>,
    /// **`Queue & History` N17: whether the machine-reaching controls are offered.**
    pub reach_backs: bool,
    /// **The server stands and is not connected**, so *Files on the server*
    /// and the *Writing to* link that asks for the same act are not drawn,
    /// and come back when it connects. `Specs/Queue & History` counts *show
    /// outputs* among the acts that need a connection, and `Design/File
    /// Browser § When another module moves the browser to a different server`
    /// says *"A machine that is not connected is not browsed and the action is
    /// not offered."* **The queue withholds them the way it withholds *Stop*
    /// and *Cancel***: while its reading is not updating. Step 10c's *Files on
    /// the server* follow-up.
    pub the_server_is_not_connected: bool,
    /// **Whether the job has a folder yet** (`Job files and outputs` P21,
    /// `Queue & History` Y8). Where it has none, *Files on the server*, *Download
    /// all* and *Download all to…* are absent, whatever [`Self::reach_backs`]
    /// says. Read off the record, never composed. The Creator's second notes,
    /// piece 4's queue half.
    pub the_folder: TheFolderOnTheEntry,
    /// The status line — a whole sentence's worth, in parts.
    pub status: Vec<APart>,
    /// The status dot's own class, which is the drawing's.
    pub status_is: &'static str,
    /// The two bars and their readings, on a rendering entry.
    pub progress: Option<TheProgress>,
    /// The readings from the machine — `Queue & History` R6 and `Queue & History` G1 to
    /// `Queue & History` G5a. Values,
    /// each with
    /// its own name.
    pub readings: Vec<(&'static str, String)>,
    /// **The marks — `Queue & History` N15 and `Queue & History` N22, each shown only
    /// when true** (Design, *The
    /// marks*). Each is a lead sentence and up to two parts under it.
    pub marks: Vec<AMarkOnTheEntry>,
    /// **What the job writes — the six statements** (Design, *What the job
    /// writes*).
    pub writes: Vec<AMarkOnTheEntry>,
    /// **The two that hold the job carry the choice** (Design). Each is a
    /// question with two answers and a note.
    pub held: Vec<TheJobIsHeld>,
    /// The setup it was submitted with, in the drawing's italic-and-bold
    /// convention (`Queue & History` N14).
    pub facts: Vec<AFactOnTheEntry>,
    /// Whether Blender's own words can be opened — the control.
    pub blender_output: bool,
    /// **The ordinary stream's newest three complete lines, while the job
    /// renders** — `Desktop app and delivery` M13b's one narrowing, oldest
    /// first. **Empty on every other entry**: an ended one, one beside *Not
    /// updating*, and every History entry, where M13b stands unchanged and no
    /// line of either stream is lifted out. The Creator's notes on the test
    /// copy, piece 8 (note 12).
    pub lines: Vec<String>,
    /// The preview column.
    pub preview: Option<ThePreview>,
    /// The auto-download arrangement (`Queue & History` L1a).
    pub downloading: Option<TheDownloading>,
    /// **`Telegram` V1c: the *Telegram* group under *Downloading***, on a
    /// live job's queue entry where Telegram is on in Settings; never on an
    /// ended job or in History. The twenty-first notes, note 7.
    pub telegram: Option<TheTelegramGroup>,
    /// **Design, *The entry's controls*: Cancel sits where Stop sits.** One act
    /// on the job, in the one place the entry's acts live, and no job is ever
    /// both. Absent on a history entry, which has neither.
    pub stop_or_cancel: Option<TheStopOrCancel>,
}

/// ***Writing to* is a link to Files on that folder**, where a folder is
/// named; the event says so only where *Files on the server* is offered
/// ([`the_entry_is_said`]). Queue and History both call this. The Creator's
/// third notes, note 11, and seventh notes, note 5.
pub(crate) fn the_writing_to_path_opens_files(entry: &mut TheJobEntry) {
    for fact in entry.facts.iter_mut() {
        if fact.name == WRITING_TO_FACT && fact.value != NOT_KNOWN {
            fact.opens_files_on_the_server = true;
        }
    }
}

/// **Whether the entry's job has a folder, as the local record holds it.**
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TheFolderOnTheEntry {
    /// It has one: chosen and learned, or named at submission as every job's
    /// was before 2026-10-04.
    #[default]
    ItHasOne,
    /// **Chosen at render start, and the render has not started** — the
    /// drawing's `q-wait`: one line, *The folder is made when the render
    /// starts.*, where the downloads were.
    ItIsMadeWhenTheRenderStarts,
    /// **Chosen at render start, the render has started, and the choice has
    /// not reached this record yet** (`Queue & History` N9 brings it). The
    /// controls are absent and no line is drawn: the waiting line would no
    /// longer be true.
    ItHasNotArrived,
}

/// **Which of the two acts sits in the header slot.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheStopOrCancel {
    Stop,
    Cancel,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TheProgress {
    /// `Queue & History` R6a: a simulation line has no render bars.
    pub simulating: bool,
    /// *Rendering now* — the frame, the view layer, the printed figures, and
    /// Blender's own estimate at the end of the same line (R9, R10, R15).
    pub now: Vec<APart>,
    /// **The first bar, as a whole percentage — absent where it shows none**
    /// (`Queue & History` B4's *no percentage*, or no sample pair printed).
    pub now_at: Option<u32>,
    /// **The second bar, as a whole percentage — absent where the machine
    /// carried no total or no finished part** (`Queue & History` R4: never a
    /// zero). Both were plain numbers until 2026-10-03, piece 6b, when
    /// nothing filled either.
    pub whole: Option<u32>,
}

/// **Design, *A rendering entry*, from `Queue & History` R5's figures** — the
/// *Rendering now* line in parts, and the two bars through `Queue & History`'s
/// own `bars` (`Queue & History` B3, `Queue & History` B3a, `Queue & History` B4,
/// `Queue & History` B5, `Queue & History` B6). Piece 6b.
///
/// - **The frame is the frame's own number** (`Queue & History` R9), never a
///   place in the range: *"Nothing in Smoozy assumes the first frame is frame
///   1"* (`Specs/Render Settings`).
/// - **The View Layer is named where the frame in hand renders more than
///   one** (`Design/Queue & History`: *"the View Layer where a frame has more
///   than one"*), and no ordinal is ever composed (`Queue & History` R10 and
///   `Queue & History` B3a). Every version carries the layer's name: 4.5 on
///   each progress line, and from piece 3b of the Creator's second notes the
///   newer three through Blender's `Start rendering:` line. **How many render
///   at the frame is the job's reading's**, kept by the run as
///   [`crate::servers::the_machines_work::WhatTheReadingSays`] with the job's
///   pins applied; **a frame it does not cover, a report with no frame, and a
///   job with no reading yet keep naming the layer** — the safe side. *Until
///   the Creator's second notes, piece 6b, every frame named it, a frame of
///   one layer (often just `ViewLayer`) included.*
/// - **The figures read exactly as Blender printed them** (`Queue & History`
///   R11, R13), and on a tiled frame the samples are named as the tile's.
/// - **Blender's own estimate is shown as Blender's** and is never fed to a bar
///   (`Queue & History` R15, `Queue & History` B9). Under EEVEE it is simply absent.
///
/// **Nothing where the figures carry nothing to draw.**
pub(crate) fn the_progress_from(
    figures: &smoozy_queue_history::readings::AWorkersReport,
    grammar: smoozy_queue_history::bars::WhichGrammar,
    reading: Option<&crate::servers::the_machines_work::WhatTheReadingSays>,
) -> Option<TheProgress> {
    use smoozy_queue_history::bars::{
        as_a_percentage, the_second_as_a_percentage, the_two_bars_from_what_the_machine_carries,
    };
    let held = figures.readings.value();
    if let Some((frame, to)) = held.simulation {
        return Some(TheProgress {
            simulating: true,
            now: vec![APart::said(SIMULATION_FRAME, frame.to_string(), SIMULATION_OF), APart::value(to.to_string())],
            now_at: None,
            whole: None,
        });
    }
    let mut now = Vec::new();
    if let Some(frame) = held.frame_in_hand {
        now.push(APart::said(FRAME_HEADING, frame.to_string(), ""));
    }
    // **Only where the frame renders more than one layer**, as the job's
    // reading says at that frame; where it does not say, the layer is named.
    let named_here = match (held.frame_in_hand, reading) {
        (Some(frame), Some(says)) => says.the_view_layer_is_named_at(frame),
        _ => true,
    };
    if let Some(layer) = held.view_layer.as_ref().filter(|_| named_here) {
        now.push(APart::value(layer.clone()));
    }
    match (held.tiles, held.samples) {
        (Some(tiles), samples) => {
            now.push(APart::figure(format!("{}/{}", tiles.done, tiles.of), TILES_WORD));
            if let Some(samples) = samples {
                now.push(APart::words(A_DOT));
                now.push(APart::figure(
                    format!("{}/{}", samples.seen, samples.of),
                    SAMPLES_IN_THIS_TILE,
                ));
            }
        }
        (None, Some(samples)) => {
            now.push(APart::figure(format!("{}/{}", samples.seen, samples.of), SAMPLES_WORD));
        }
        (None, None) => {}
    }
    if let Some(estimate) = &held.blenders_own_estimate_for_the_frame {
        now.push(APart::figure(estimate.clone(), LEFT_WORD));
        now.push(APart::words(A_DOT));
        now.push(APart::words(BLENDERS_ESTIMATE));
    }
    let (first, second) = the_two_bars_from_what_the_machine_carries(grammar, figures);
    let progress = TheProgress {
        simulating: false,
        now,
        now_at: as_a_percentage(&first),
        whole: second.as_ref().and_then(the_second_as_a_percentage),
    };
    (!progress.now.is_empty() || progress.now_at.is_some() || progress.whole.is_some()).then_some(progress)
}

/// **`Queue & History` R2's one boundary, read off the Blender the job was
/// submitted with** — 4.5's grammar names the View Layer, the newer three's
/// do not. The kept version is the record's (`Queue & History` N10a).
pub(crate) fn the_grammar_of(
    record: &smoozy_queue_history::record::LocalJobRecord,
) -> smoozy_queue_history::bars::WhichGrammar {
    use smoozy_queue_history::bars::WhichGrammar;
    if record.blender.pick.version.starts_with("4.") {
        WhichGrammar::FourPointFive
    } else {
        WhichGrammar::FivePointZeroAndLater
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AMarkOnTheEntry {
    pub said: &'static str,
    pub parts: Vec<APart>,
    /// A way to another entry, where the mark offers one (`Queue & History` N20).
    pub way: Option<&'static str>,
    /// **How many of [`Self::parts`] sit inside the bold with [`Self::said`]**
    /// — none on every mark but `Queue & History` N23's, whose bold sentence
    /// carries the driver's version and the job's Blender: ***The NVIDIA
    /// driver, 525.147.05, was too old for OptiX in Blender 5.1**, and Smoozy
    /// couldn't update it.* Held-job piece B, 2026-10-12. **And every part of
    /// the duplicate-picture line under *What this job writes***, which is
    /// bold whole (the Creator's ninth notes, note 1, its E4 half).
    pub bold: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TheJobIsHeld {
    /// **In parts**, because the first-frame check names the job's own first
    /// frame (must-do 8).
    pub said: Vec<APart>,
    pub second: &'static str,
    pub first_answer: Vec<APart>,
    pub second_answer: &'static str,
    pub note: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AFactOnTheEntry {
    pub name: &'static str,
    pub value: String,
    /// **`Queue & History` N14: the file's own value beside the user's, where the
    /// payload carries
    /// an override.** Absent where the value is the file's own.
    pub the_files_own: Option<String>,
    /// Whether the value is the file's own — the drawing's italic — or the
    /// user's, which is upright and bold.
    pub the_files: bool,
    /// **Neither face** — a value that is neither side's answer: Smoozy's own
    /// composition, like *Writing to*, or a setting whose record cannot say
    /// whether the user answered it. **Drawn plain**, because the legend above
    /// the group gives bold to the user and italic to the file, and a value
    /// that is neither would otherwise read as one of them. Step 10c's sixth
    /// pass read *Samples — not known* in bold under *bold = yours*.
    pub neither: bool,
    /// A second value on the same line, in the fixed-width face.
    pub beside: Option<String>,
    /// **Lines under the value, each a sentence of Smoozy's** — on *Writing
    /// to*, why the folder fell back (`Design/Queue & History`, *The job's
    /// name, and its folder*). Empty on every other fact.
    pub under: Vec<&'static str>,
    /// **On *Writing to*, in Queue and History alike: the path is a link that
    /// opens Files on that folder**, the same act as *Files on the server*,
    /// and drawn only where that control is offered (`Design/Queue & History`,
    /// *The job's name, and its folder*; the Creator's third notes, note 11,
    /// and seventh notes, note 5).
    pub opens_files_on_the_server: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ThePreview {
    /// The frame the picture is of. A value.
    pub frame: String,
    /// The view layer's own name, where the file holds more than one (P10a).
    pub layer: Option<String>,
    /// **Which job's held copy the picture is** — the job's identity, which
    /// the page hands back with [`ThePreview::frame`] to
    /// [`the_preview_picture`] for the bytes (H32). Absent
    /// where no copy is held. The 38's piece 3.
    pub picture: Option<String>,
    /// The word under the picture — *latest*, or *last complete*.
    pub which: &'static str,
    /// Queue starts following the newest frame; Show latest resumes it after
    /// a choice. History has Show last and never follows newer arrivals
    /// (`Queue & History` P2).
    pub follows: bool,
    /// Where no frame finished there is no picture and one line sits where it
    /// was (Design).
    pub nothing_to_show: bool,
    /// **Every frame the record holds a copy of, in frame order, each with
    /// the View Layers it has a copy of in the frame's own order and which is
    /// the main copy** (`Queue & History` P4, P10, P10a). What the page steps
    /// through and lists in the View Layer dropdown. Added 2026-10-10, the
    /// Creator's seventh notes, note 4.
    pub frames_held: Vec<AFrameHeld>,
}

/// **One frame the record holds a copy of** (P4).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AFrameHeld {
    /// The frame number. A value.
    pub frame: String,
    /// Its copies, in the frame's own order — the order of their places.
    pub layers: Vec<ALayerHeld>,
}

/// **One View Layer's copy of a frame** — or the frame's main copy that is no
/// one layer's, named by where it came from.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ALayerHeld {
    /// The View Layer's own name, where the copy carries one — the frame's
    /// one copy carries none where the file holds one layer (`Job files and
    /// outputs` B31a). **For a copy named by its source, the File Output
    /// node's own name**, and none for the output path. A value.
    pub name: Option<String>,
    /// **Where the picture came from, in words, for a copy that is no one
    /// View Layer's** — [`OUTPUT_PATH`] or [`FILE_OUTPUT_NODE`], the node's
    /// name being [`ALayerHeld::name`]. Absent on every layer's own copy. The
    /// Creator's tenth notes, note 2.
    pub source: Option<&'static str>,
    /// **Its place in the frame's set**, which the page hands back to
    /// [`the_preview_picture`] for this copy's bytes. A value. **Empty for a
    /// copy named by its source**, which is no layer's and is asked for as
    /// the frame's main copy.
    pub place: String,
    /// Whether it is the frame's main copy — what the preview opens on (P10).
    pub main: bool,
}

/// **The frames held, from the record's rows** ([`crate::record::previews::every_copy_held_for`]'s
/// order: frame, then place).
pub(crate) fn the_frames_held(copies: &[crate::record::previews::AFrameOfTheSet]) -> Vec<AFrameHeld> {
    let mut frames: Vec<AFrameHeld> = Vec::new();
    for copy in copies.iter().filter(|copy| copy.is_held()) {
        let frame = copy.frame.to_string();
        if frames.last().is_none_or(|last| last.frame != frame) {
            frames.push(AFrameHeld { frame: frame.clone(), layers: Vec::new() });
        }
        if let Some(last) = frames.last_mut() {
            use smoozy_job_files::main_output::WhereThePictureCameFrom;
            let (source, name) = match &copy.source {
                Some(WhereThePictureCameFrom::TheOutputPath) => (Some(OUTPUT_PATH), None),
                Some(WhereThePictureCameFrom::AFileOutputNode(node)) => {
                    (Some(FILE_OUTPUT_NODE), Some(String::from_utf8_lossy(node.as_bytes()).into_owned()))
                }
                None => (None, copy.view_layer.as_deref().map(|name| String::from_utf8_lossy(name).into_owned())),
            };
            last.layers.push(ALayerHeld {
                name,
                place: if source.is_some() { String::new() } else { copy.place.to_string() },
                source,
                main: copy.the_main_copy,
            });
        }
    }
    frames
}

/// **`Telegram` V1c and `Design/Queue & History § The entry's controls`: the
/// Render Settings foot's own control, reading the job's choice as the
/// server holds it** (the desk's record, version 55). One per first-frame
/// pair, sent as its held range job.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheTelegramGroup {
    /// The choice the record holds, or nothing for a job submitted before the
    /// record kept one.
    pub picked: Option<smoozy_render_settings::reporting::ReportingChoice>,
    /// The job the change names: the held range job of a pair, else the job.
    pub sent_as: String,
    pub stands: WhereTheTelegramControlStands,
}

/// **Where the entry's *Telegram* control stands.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhereTheTelegramControlStands {
    /// Offered: connected, and the server's Smoozy takes the change.
    Offered,
    /// The server is not connected: the choice is read, the control unavailable.
    NotConnected,
    /// The server's Smoozy is older: unavailable, with *Update Smoozy on this
    /// server to change this.*
    TheServersSmoozyIsOlder,
    /// Connected, and this run has not yet read the server's release:
    /// unavailable, with nothing guessed.
    NotYetKnown,
    /// **`Server Setup` G2a: the server is read as not having the role**, so
    /// the job will not report: unavailable, with one grey line saying so.
    TelegramIsNotInstalled,
}

/// **The choice a page identifier names, carrying `minutes` where it is the
/// fourth** — Render Settings' own identifiers ([`crate::surface::render_settings::REPORTING_CHOICES`]).
pub(crate) fn the_reporting_choice_named(
    id: &str,
    minutes: u8,
) -> Option<smoozy_render_settings::reporting::ReportingChoice> {
    use smoozy_render_settings::reporting::ReportingChoice;
    let at = crate::surface::render_settings::REPORTING_CHOICES.iter().position(|(value, _)| *value == id)?;
    Some(match ReportingChoice::ALL[at] {
        ReportingChoice::APictureEveryFewMinutes { .. } => ReportingChoice::APictureEveryFewMinutes { minutes },
        other => other,
    })
}

/// **The page identifier of a choice**, the inverse of [`the_reporting_choice_named`].
pub(crate) fn the_identifier_of(choice: smoozy_render_settings::reporting::ReportingChoice) -> &'static str {
    use smoozy_render_settings::reporting::ReportingChoice;
    let at = ReportingChoice::ALL
        .iter()
        .position(|one| std::mem::discriminant(one) == std::mem::discriminant(&choice))
        .unwrap_or(0);
    crate::surface::render_settings::REPORTING_CHOICES[at].0
}

/// **What the entry's control sends for a pick or a typed minutes entry**, as
/// the `Telegram` V1a value in the manifest's own form — or the refusal of a minutes entry
/// outside 1 to 60, the Render Settings foot's own. `from` is a choice's page
/// identifier, or `minutes:` and what was typed.
pub(crate) fn what_the_telegram_control_sends(
    from: &str,
    the_choice_now: Option<smoozy_render_settings::reporting::ReportingChoice>,
) -> Result<String, &'static str> {
    use smoozy_render_settings::reporting::{self, ReportingChoice};
    let minutes_now = match the_choice_now {
        Some(ReportingChoice::APictureEveryFewMinutes { minutes }) => minutes,
        _ => reporting::THE_MINUTES_TO_BEGIN_WITH,
    };
    let choice = match from.strip_prefix("minutes:") {
        Some(typed) => ReportingChoice::APictureEveryFewMinutes {
            minutes: reporting::the_minutes_typed(typed)
                .map_err(|_| crate::surface::render_settings::words::WHOLE_MINUTES_1_TO_60)?,
        },
        None => the_reporting_choice_named(from, minutes_now).ok_or(THIS_JOB_IS_NO_LONGER_IN_THE_QUEUE)?,
    };
    Ok(choice.in_the_manifests_form())
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TheDownloading {
    pub on: bool,
    /// What the narrowing says the arrangement takes. A value — the format
    /// names in Smoozy's words, side by side; **empty is every format**
    /// (`Download` N3), which the page says in words. Shown only: the
    /// arrangement keeps the identifiers.
    pub narrowing: String,
    pub done: u32,
    pub of: u32,
}

/// The box that asks — Design, *The entry's controls* and *The catch-up
/// notice*. **Every one is a statement ending in a named button, never a
/// question with Yes and No.**
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TheBoxThatAsks {
    pub which: &'static str,
    pub heading: Vec<APart>,
    pub said: Vec<Vec<APart>>,
    pub keep: &'static str,
    pub go_ahead: Vec<APart>,
    /// Whether the going-ahead control is the drawing's dangerous one.
    pub grave: bool,
    /// **What the going-ahead control answers when pressed** (must-do 5).
    pub go_ahead_act: TheActOnAJob,
    /// **Design, *Clearing history*: the box that clears every machine at once
    /// lists every server it covers with that server's own count**, because a
    /// single count does not show its reach. Empty on every other box.
    pub per_server: Vec<(String, u32)>,
}

/// **Every control on the two screens that cannot act yet — a closed set**
/// (H34), and what each answers. Must-do 5 of step 10a round 1 of the queue,
/// history, settings and downloads: **every control either acts or says why it
/// cannot**, in the one failure shape beside the control that was pressed, at
/// that moment only. **Nothing is recorded and nothing reaches the diary**:
/// nothing was attempted (`Desktop app and delivery` M32, second limb).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TheActOnAJob {
    // Needs a machine.
    #[default]
    Stop,
    Cancel,
    TrackIt,
    TheHeldJobsFirstAnswer,
    TheHeldJobsSecondAnswer,
    CancelAll,
    FilesOnTheServer,
    BlenderOutput,
    StopTheJob,
    CancelTheJob,
    CancelTheGroup,
    // Needs the history store.
    SubmitAgain,
    CarryOn,
    TheEarlierEntry,
    DeleteTheEntry,
    ClearThisServersHistory,
    ClearAllHistory,
    /// **`Queue & History` X6b: *Clear removed servers' history*'s box
    /// go-ahead.** The eighteenth notes, note 13.
    ClearRemovedServersHistory,
    DismissTheNotice,
    // Transfers' own two answers.
    FramesAsTheyRender,
    FilesOnYourComputer,
    /// **`Rendering` S11: *Submit the remaining scenes*** — the part banner's
    /// one control (the mockup's `#qpart`). The 38's piece 2.
    SubmitTheRemainingScenes,
    /// **`Queue & History` L6: a row dragged, or its menu's *Move to the top
    /// of the queue*.** The 38's piece 8.
    MoveTheJob,
    /// **`Queue & History` L7: a group's heading dragged.** The 38's piece 8.
    MoveTheGroup,
    /// **`Telegram` V1c: the entry's *Telegram* control picked** — the value
    /// rides the page's `from`. The twenty-first notes, note 7.
    WhatItReports,
}

impl TheActOnAJob {
    pub const ALL: [TheActOnAJob; 25] = [
        TheActOnAJob::Stop,
        TheActOnAJob::Cancel,
        TheActOnAJob::TrackIt,
        TheActOnAJob::TheHeldJobsFirstAnswer,
        TheActOnAJob::TheHeldJobsSecondAnswer,
        TheActOnAJob::CancelAll,
        TheActOnAJob::FilesOnTheServer,
        TheActOnAJob::BlenderOutput,
        TheActOnAJob::StopTheJob,
        TheActOnAJob::CancelTheJob,
        TheActOnAJob::CancelTheGroup,
        TheActOnAJob::SubmitAgain,
        TheActOnAJob::CarryOn,
        TheActOnAJob::TheEarlierEntry,
        TheActOnAJob::DeleteTheEntry,
        TheActOnAJob::ClearThisServersHistory,
        TheActOnAJob::ClearAllHistory,
        TheActOnAJob::ClearRemovedServersHistory,
        TheActOnAJob::DismissTheNotice,
        TheActOnAJob::FramesAsTheyRender,
        TheActOnAJob::FilesOnYourComputer,
        TheActOnAJob::SubmitTheRemainingScenes,
        TheActOnAJob::MoveTheJob,
        TheActOnAJob::MoveTheGroup,
        TheActOnAJob::WhatItReports,
    ];

    /// The name the page hands back, and the name the answer carries.
    pub fn name(self) -> &'static str {
        match self {
            TheActOnAJob::Stop => "stop",
            TheActOnAJob::Cancel => "cancel",
            TheActOnAJob::TrackIt => "track-it",
            TheActOnAJob::TheHeldJobsFirstAnswer => "the-held-jobs-first-answer",
            TheActOnAJob::TheHeldJobsSecondAnswer => "the-held-jobs-second-answer",
            TheActOnAJob::CancelAll => "cancel-all",
            TheActOnAJob::FilesOnTheServer => "files-on-the-server",
            TheActOnAJob::BlenderOutput => "blender-output",
            TheActOnAJob::StopTheJob => "stop-the-job",
            TheActOnAJob::CancelTheJob => "cancel-the-job",
            TheActOnAJob::CancelTheGroup => "cancel-the-group",
            TheActOnAJob::SubmitAgain => "submit-again",
            TheActOnAJob::CarryOn => "carry-on",
            TheActOnAJob::TheEarlierEntry => "the-earlier-entry",
            TheActOnAJob::DeleteTheEntry => "delete-the-entry",
            TheActOnAJob::ClearThisServersHistory => "clear-this-servers-history",
            TheActOnAJob::ClearAllHistory => "clear-all-history",
            TheActOnAJob::ClearRemovedServersHistory => "clear-removed-servers-history",
            TheActOnAJob::DismissTheNotice => "dismiss-the-notice",
            TheActOnAJob::FramesAsTheyRender => "frames-as-they-render",
            TheActOnAJob::FilesOnYourComputer => "files-on-your-computer",
            TheActOnAJob::SubmitTheRemainingScenes => "submit-the-remaining-scenes",
            TheActOnAJob::MoveTheJob => "move-the-job",
            TheActOnAJob::MoveTheGroup => "move-the-group",
            TheActOnAJob::WhatItReports => "what-it-reports",
        }
    }

    /// **What the control answers, where it cannot act.** Each sentence names
    /// its own missing thing and no two acts share one that is not true of
    /// both (must-do 5(a) to (c)).
    ///
    /// **The three acts step 10b piece 8 landed answer nothing and are not
    /// here**: [`where_an_act_on_a_job_goes`] is what says which is which, and
    /// this is the second half of that one fact rather than a list beside it.
    pub fn answers(self) -> &'static str {
        use crate::surface::transfers::words as transfers;
        match self {
            // **`BlenderOutput` reaches this only from History's entry**, which
            // is piece 6d's: on a rendering queue entry the press opens the
            // render's words and never answers here (piece 6c,
            // [`the_running_words_are_opened`]).
            TheActOnAJob::BlenderOutput => A_CONNECTION_IS_NOT_BUILT,
            // **Owed piece 10: *Track it* acts** (`Queue & History` A6). What
            // a press can meet with nothing adopted is the commoner of its two
            // — the server not reached for the reads each entry is composed
            // from; a write the record refused says
            // [`THE_WORK_COULD_NOT_BE_TRACKED`].
            TheActOnAJob::TrackIt => SMOOZY_COULD_NOT_REACH_THE_SERVER,
            // **Step 10c's S3: these six act.** What they answer is only the
            // one thing a press can meet with nothing sent — the job gone from
            // the reading the box was raised over; *not rendering* is said
            // beside it where that is the reason.
            // **Step 10c's eleventh-pass fix piece 8: the held job's two
            // answers act** (`Queue & History` H2), and answer only where the
            // job is no longer held in the reading they were drawn over.
            TheActOnAJob::Stop
            | TheActOnAJob::Cancel
            | TheActOnAJob::StopTheJob
            | TheActOnAJob::CancelTheJob
            | TheActOnAJob::TheHeldJobsFirstAnswer
            | TheActOnAJob::TheHeldJobsSecondAnswer => THIS_JOB_IS_NO_LONGER_IN_THE_QUEUE,
            TheActOnAJob::CancelAll | TheActOnAJob::CancelTheGroup => {
                THESE_JOBS_ARE_NO_LONGER_IN_THE_QUEUE
            }
            // **The 38's piece 2: these act**, through
            // [`crate::a_new_job_from_an_old_one`], whose own sentences say
            // what a press met. What is left here is the one thing every press
            // can meet before anything is read: the record not answering.
            TheActOnAJob::SubmitAgain | TheActOnAJob::CarryOn => {
                crate::a_new_job_from_an_old_one::words::THE_RECORD_WOULD_NOT_READ
            }
            TheActOnAJob::TheEarlierEntry => THAT_ENTRY_WAS_DELETED,
            // **`Rendering` S11's act needs the server**, and a press while it
            // is not connected meets that.
            TheActOnAJob::SubmitTheRemainingScenes => {
                crate::a_new_job_from_an_old_one::words::THE_SERVER_DID_NOT_ANSWER
            }
            // **The 38's piece 15: *Dismiss* acts**, on the notice record
            // version 38 keeps. What is left is the one thing a press can
            // meet: the record refusing the removal.
            TheActOnAJob::DismissTheNotice => crate::surface::the_history::THE_RECORD_REFUSED_IT,
            // **The three that act.** Their answer is only ever the one a
            // refused write earns, which is [`the_history`]'s to compose, and
            // asking for a sentence here is the fault the routing exists to
            // stop.
            TheActOnAJob::DeleteTheEntry
            | TheActOnAJob::ClearThisServersHistory
            | TheActOnAJob::ClearAllHistory
            | TheActOnAJob::ClearRemovedServersHistory => crate::surface::the_history::THE_RECORD_REFUSED_IT,
            // **The entry's two Transfers controls act since the 38's piece 9**
            // ([`where_an_act_on_a_job_goes`]): the switch turned off writes
            // `Download` R3's desired state, and *Files on your computer*
            // opens `Download` H5's set. What is left here is the one thing
            // either can meet with nothing done — the record not taking the
            // switch, or the job not reading from it.
            TheActOnAJob::FramesAsTheyRender => crate::surface::settings::words::THE_SETTING_DID_NOT_SAVE,
            TheActOnAJob::FilesOnYourComputer => transfers::THE_JOB_WOULD_NOT_READ,
            // **Step 10c's *Files on the server* piece: it acts**, opening
            // Files at the job's folder. What is left here is the commoner of
            // what a press can meet with nothing opened — the record not
            // reading the job; [`where_the_jobs_folder_is`] says the others.
            TheActOnAJob::FilesOnTheServer => crate::a_new_job_from_an_old_one::words::THE_RECORD_WOULD_NOT_READ,
            // **The 38's piece 8: a move acts**, and what a press can meet with
            // nothing sent is the job, or the group, gone from the reading the
            // row was dragged in. What the move came to once sent is
            // [`the_move_came_to`]'s.
            TheActOnAJob::MoveTheJob => THIS_JOB_IS_NO_LONGER_IN_THE_QUEUE,
            TheActOnAJob::MoveTheGroup => THESE_JOBS_ARE_NO_LONGER_IN_THE_QUEUE,
            // **`Telegram` V1c: the change acts**; what a press can meet with
            // nothing sent is the job gone from the reading. What it came to
            // once sent is [`the_reporting_change_came_to`]'s.
            TheActOnAJob::WhatItReports => THIS_JOB_IS_NO_LONGER_IN_THE_QUEUE,
        }
    }
}

/// **Where each act on a job goes, as a value, so that which of them are built
/// is one fact in one place and a test can read it.**
///
/// **The shape is `src/surface/servers.rs`'s `where_an_act_goes`**, taken
/// unchanged, and for the same reason: that file's header list of what is owed
/// had stopped being true the moment a route landed, and the test below is what
/// says so.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum WhereAnActOnAJobGoes {
    /// **`Queue & History` X1: the entry's record and its tombstone, in one
    /// local act.** The delete box's go-ahead.
    ADeletion,
    /// **`Queue & History` X6: one machine's history**, the machine being the
    /// server of the entry that is open.
    AClearingOfOneServersHistory,
    /// **`Queue & History` X6a: the whole of history.**
    AClearingOfTheWholeOfHistory,
    /// **`Queue & History` X6b: the history of every server no longer in
    /// Smoozy.** The eighteenth notes, note 13.
    AClearingOfTheRemovedServersHistory,
    /// **The press raises its box** — Design, *The entry's controls*:
    /// *"Stopping asks first, and so does cancelling"*, and the group's
    /// *Cancel all* asks the same way. Step 10c's S3.
    ItsBox,
    /// **The box's go-ahead reaches the machine** — `Server protocol and
    /// transfers` C21's stop, `Queue & History` L1's cancel, or
    /// `Queue & History` L19's group clear. Step 10c's S3.
    ItReachesTheMachine(crate::servers::the_queues_own_acts::TheQueuesOwnAct),
    /// **The held entry's answer reaches the machine** — `Queue & History` H2
    /// and `Rented-server runtime` U9: an answer naming the hold, and for
    /// *Cancel* the cancel after it. Step 10c's eleventh-pass fix piece 8.
    ItAnswersTheHold(crate::servers::the_machines_work::TheHeldJobsAnswer),
    /// **`Rendering` A1 and `Rendering` A4: a new job from the entry**, through
    /// [`crate::a_new_job_from_an_old_one`]. The 38's piece 2.
    ANewJobFromTheEntry,
    /// **`Queue & History` N20: the way to the earlier entry**, opened on
    /// History.
    TheEarlierEntryOpens,
    /// **`Rendering` S11: the scenes resolved as not submitted, as a fresh
    /// act.**
    TheRemainingScenesAreSubmitted,
    /// **`Download` R3: the entry's auto-download switch, turned off** — the
    /// page sends it only while the switch is on; turning it on goes through
    /// the format box, which asks where again (`Download` R11). The 38's
    /// piece 9.
    TheArrangementIsSwitchedOff,
    /// **`Download` H5 to H8: the job's downloaded files, opened** —
    /// `Queue & History` N18. The 38's piece 9.
    TheJobsFilesAreShown,
    /// **`Queue & History` N18's server-side shortcut, *show outputs***,
    /// labelled *Files on the server*: Files opens on that server at the
    /// folder `Queue & History` N10 names ([`where_the_jobs_folder_is`]). Step 10c's *Files on
    /// the server* piece.
    TheJobsFolderOpensInFiles,
    /// **`Queue & History` C2a: one dropped job's line dismissed** — that one
    /// name taken off the notice record that holds it, the user's own act on
    /// Smoozy's own record. The go-ahead of the box its *Dismiss* raises,
    /// which hands back the job. The 38's piece 15; one line rather than the
    /// whole message since the Creator's ninth notes, note 3.
    TheNoticeIsDismissed,
    /// **`Queue & History` L6 and `Queue & History` L7: a move on the line
    /// reaches the machine**, through [`crate::servers::the_queues_own_acts::the_move_is_begun`].
    /// The 38's piece 8.
    ItMovesOnTheLine,
    /// **`Queue & History` A6: the untracked-work offer accepted** — every
    /// job of one server's offer entered in the local record in one write,
    /// through [`crate::servers::the_machines_work::the_offer_is_accepted`].
    /// The banner's *Track them*. Owed piece 10.
    TheWorkIsTracked,
    /// **`Telegram` V1c: a live job's reporting changed on the machine**,
    /// through [`crate::servers::the_queues_own_acts::the_reporting_change_is_begun`].
    /// The twenty-first notes, note 7.
    TheReportingIsChanged,
    /// **Owed, and each for its own reason**, which [`TheActOnAJob::answers`]
    /// says in the user's own words.
    ItAnswers,
}

pub(crate) fn where_an_act_on_a_job_goes(act: TheActOnAJob) -> WhereAnActOnAJobGoes {
    use crate::servers::the_machines_work::TheHeldJobsAnswer;
    use crate::servers::the_queues_own_acts::TheQueuesOwnAct;
    match act {
        TheActOnAJob::Stop | TheActOnAJob::Cancel | TheActOnAJob::CancelAll => {
            WhereAnActOnAJobGoes::ItsBox
        }
        TheActOnAJob::StopTheJob => WhereAnActOnAJobGoes::ItReachesTheMachine(TheQueuesOwnAct::TheStop),
        TheActOnAJob::CancelTheJob => {
            WhereAnActOnAJobGoes::ItReachesTheMachine(TheQueuesOwnAct::TheCancel)
        }
        TheActOnAJob::CancelTheGroup => {
            WhereAnActOnAJobGoes::ItReachesTheMachine(TheQueuesOwnAct::TheGroupsCancel)
        }
        TheActOnAJob::TheHeldJobsFirstAnswer => {
            WhereAnActOnAJobGoes::ItAnswersTheHold(TheHeldJobsAnswer::RenderAll)
        }
        TheActOnAJob::TheHeldJobsSecondAnswer => {
            WhereAnActOnAJobGoes::ItAnswersTheHold(TheHeldJobsAnswer::CancelIt)
        }
        TheActOnAJob::DeleteTheEntry => WhereAnActOnAJobGoes::ADeletion,
        TheActOnAJob::ClearThisServersHistory => {
            WhereAnActOnAJobGoes::AClearingOfOneServersHistory
        }
        TheActOnAJob::ClearAllHistory => WhereAnActOnAJobGoes::AClearingOfTheWholeOfHistory,
        TheActOnAJob::ClearRemovedServersHistory => WhereAnActOnAJobGoes::AClearingOfTheRemovedServersHistory,
        TheActOnAJob::SubmitAgain | TheActOnAJob::CarryOn => WhereAnActOnAJobGoes::ANewJobFromTheEntry,
        TheActOnAJob::TheEarlierEntry => WhereAnActOnAJobGoes::TheEarlierEntryOpens,
        TheActOnAJob::SubmitTheRemainingScenes => WhereAnActOnAJobGoes::TheRemainingScenesAreSubmitted,
        TheActOnAJob::DismissTheNotice => WhereAnActOnAJobGoes::TheNoticeIsDismissed,
        TheActOnAJob::FramesAsTheyRender => WhereAnActOnAJobGoes::TheArrangementIsSwitchedOff,
        TheActOnAJob::FilesOnYourComputer => WhereAnActOnAJobGoes::TheJobsFilesAreShown,
        TheActOnAJob::FilesOnTheServer => WhereAnActOnAJobGoes::TheJobsFolderOpensInFiles,
        TheActOnAJob::MoveTheJob | TheActOnAJob::MoveTheGroup => WhereAnActOnAJobGoes::ItMovesOnTheLine,
        TheActOnAJob::TrackIt => WhereAnActOnAJobGoes::TheWorkIsTracked,
        TheActOnAJob::WhatItReports => WhereAnActOnAJobGoes::TheReportingIsChanged,
        _ => WhereAnActOnAJobGoes::ItAnswers,
    }
}

/// **What the page asked the queue to put in front of the user.** H34.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TheQueueAsk {
    /// **A dropped job's line's *Dismiss*** — must-do 6, and Design, *The
    /// catch-up notice*: it raises the box that asks first. The line's job
    /// travels beside it.
    Dismiss,
    /// **The screen was put away.** P10d repeats the queue reading only while
    /// a surface showing the queue is on screen, and this is how the
    /// application learns it no longer is. Nothing is drawn in answer.
    PutAway,
    /// **The words' panel was closed** — P10d repeats the commentary read only
    /// while it is on screen (`Queue & History` E8). Nothing is drawn in
    /// answer. Piece 6c.
    TheWordsArePutAway,
    /// **`Queue & History` C10 as changed 2026-10-19: the cross on a row
    /// saying *Not submitted.***, which clears it — the act it is the row of
    /// travels beside it, in `dropped`'s place — and the queue is drawn again
    /// without it, whether or not it was still there to clear. Through
    /// [`crate::submitting::a_not_submitted_row_is_cleared`].
    ClearTheRow,
    /// **The check's own *Blender output*, on its pair's entry once the full
    /// range renders** — `Design/Queue & History § A first-frame check and
    /// its job`. The check's job travels beside it, in `dropped`'s place, and
    /// its kept words (`Queue & History` E9) come back on the queue's answer.
    TheChecksBlenderOutput,
}

/// **The whole of what this screen hands the page** — H34's closed response
/// type, composed here and translated into an event by [`the_queue_says`].
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TheQueueOnScreen {
    pub jobs: u32,
    pub servers: u32,
    pub groups: Vec<AGroupHeading>,
    pub lines: Vec<AServersLine>,
    pub notice: Option<TheCatchUpNotice>,
    pub untracked: Option<TheUntrackedWork>,
    pub part: Option<ThePartSubmission>,
    pub entry: Option<TheJobEntry>,
    pub asking: Option<TheBoxThatAsks>,
    /// **The group the box that asks is about**, as its identity's text —
    /// what the group box's go-ahead hands back. Step 10c's S3.
    pub the_group_asked_about: Option<String>,
    /// **The dropped job the box that asks is about**, as its identity's
    /// text — what the dismiss box's go-ahead hands back, so the one line it
    /// names is the one that goes. The Creator's ninth notes, note 3.
    pub the_dropped_job_asked_about: Option<String>,
    /// **The entry's job waits behind work Smoozy is not tracking** — the
    /// server's local identifier and how much of that work there is — where
    /// that work is rendering or the machine would start it next
    /// (`Queue & History` A4). **Said on the entry only where *Ignore* stands
    /// for that server** ([`the_decline_is_applied`]); the banner says it
    /// otherwise. Never sent to the page. The 38's piece 15.
    pub the_entry_waits_behind_untracked_work: Option<(String, u32)>,
    /// **A check's kept words, where its *Blender output* was pressed**
    /// ([`TheQueueAsk::TheChecksBlenderOutput`]), read from the record at that
    /// one moment. Absent on every other answer.
    pub words: Option<crate::surface::the_history::TheKeptWords>,
}

/// **What a job is called on a screen — `Design/Queue & History § A rendering
/// entry`, Creator-stated 2026-08-10: *"Where the job carries no label of its
/// own, the name is the file."*** The label the user typed, whole (`Queue &
/// History` Y8); where there is none, the name of the `.blend` the job renders
/// — the last part of `Queue & History` N11's recorded path, which is how the
/// entry's own line names the file under the heading. **Nothing where neither
/// is recorded** (a record from before `Queue & History` N11), and nothing
/// composed from the identity: the job's folder is the one place the identity
/// names it (`Job files and outputs` N3).
///
/// **The case it was first written for, and the one it now also meets.**
/// Every screen took the label alone, written for a job the user named. The
/// label is optional (`Specs/Queue & History`), and step 10c's Vast pass sent
/// one with *Name this job* left empty: Queue and History drew a row with no
/// text, and History's pane, taking an empty name for no entry, drew nothing —
/// no *Download all*, no *Files on the server*. **One rule, so Queue,
/// History, the servers' work column and the setup card cannot call one job
/// two things.** A label of spaces is no label: it would draw as blank.
///
/// **The rule itself is no longer written here.** It moved on 2026-10-01
/// (Vast-pass fix piece 3) to `smoozy_queue_history::naming`, the half of
/// `Queue & History` the rented machine's program also takes, because the
/// phone names a job by the same rule (`Telegram` R7a) and its words are
/// composed on the machine. This function and the one below only hand it the
/// local record's two fields.
pub fn what_the_job_is_called(record: &smoozy_queue_history::record::LocalJobRecord) -> String {
    what_a_job_is_called(record.label.as_deref(), record.blend_path_on_the_machine.as_deref())
}

/// [`what_the_job_is_called`] over the two things it reads, for a caller
/// holding the machine's note rather than the local record — the note carries
/// the label, empty where none was given, and no file. **It is the shared
/// rule, re-exported at the path every screen already calls it by.**
pub use smoozy_queue_history::naming::what_a_job_is_called;

/// **What is held: every server's last-known reading, composed with the local
/// record.**
///
/// `Queue & History` C6 makes the reading period-held, so the only thing that
/// fills this is a machine read on a live connection — `Servers` Q1 step 7 and
/// P10d's passes (`crate::servers::the_machines_work`). **An empty answer is
/// Design's own *the queue is blank on a fresh launch until a server is
/// connected*.**
///
/// **Composed here, decided nowhere here.** Which jobs are the user's is the
/// local record's (`Queue & History` A2); what the machine is doing with each
/// is the reading's, less every job whose ending Smoozy has learned
/// ([`it_has_left_the_queue`], `Queue & History` C11); the offer is step 7's.
/// **A rendering job's figures are drawn from the reading** — `Queue &
/// History` R5's, through `Queue & History`'s own `bars`
/// ([`the_progress_from`]): the row's two bars, the folded heading's
/// percentage and the entry's *Rendering now* and *Whole job*. **Absent
/// figures stay absent** (`Queue & History` R4): a job carrying none, a
/// reading not updating, and figures that stopped moving
/// ([`crate::servers::the_machines_work::WhatAPeriodHolds::the_figures_to_draw`])
/// draw no bar and no figure, never a zero. *(Until 2026-10-03, piece 6b, no
/// row carried bars: the reading carried no figures.)*
///
/// **`chosen` is the job whose entry the user asked for** — `Design/Queue &
/// History § Keyboard shortcuts`, *"Return opens the chosen entry"*. The entry
/// is that job's, or the first row's where nothing is chosen or the chosen job
/// has left the reading.
pub fn what_is_held(
    holds: &crate::servers::the_machines_work::WhatThisRunHolds,
    record: Option<&crate::record::LocalRecord>,
    chosen: Option<&str>,
) -> TheQueueOnScreen {
    use crate::record::job_records::JobRecords;
    use smoozy_queue_history::line::QueueState;

    let mut screen = TheQueueOnScreen::default();
    let Some(record) = record else {
        return screen;
    };
    let connection = record.connection();
    let store = JobRecords::over(connection);
    let rows = crate::record::server_rows::all(connection).unwrap_or_default();
    let what_a_server_is = |server: &str| {
        let row = rows.iter().find(|row| row.local_server_identifier == server);
        let name = row
            .map(|row| crate::surface::servers::what_a_server_is_called_among(row, &rows))
            .unwrap_or_default();
        let badge = row
            .and_then(|row| row.colour.as_deref())
            .and_then(|colour| {
                crate::surface::the_rail::THE_EIGHT_BADGES
                    .iter()
                    .find(|badge| **badge == colour)
            })
            .map(|badge| badge.to_string());
        (name, badge)
    };

    // **The last of each is whether the job is held for the driver** —
    // `Queue & History` H12, by the hold's reason and nothing else
    // ([`crate::servers::the_driver_swap::a_hold_for_the_driver`]).
    let mut entries: Vec<(
        smoozy_queue_history::record::LocalJobRecord,
        QueueState,
        bool,
        Option<smoozy_queue_history::readings::WhatTheHoldSays>,
        Option<TheProgress>,
        bool,
        Option<ThePair>,
    )> = Vec::new();
    // **The header's *servers* are the servers whose queue this screen holds a
    // reading of**, whether or not anything is in it. Step 10c's sixth pass
    // read *0 jobs · 0 servers* with a server connected and its queue read
    // empty: counting only the servers with a job in the list made a read,
    // empty queue look like no server at all. **A server with no reading is
    // still not counted** — Design's *the queue is blank on a fresh launch
    // until a server is connected*.
    let mut servers_read = 0u32;
    // **Each server's untracked work, and whether the machine would start it
    // next** (`Queue & History` A4) — what a waiting entry's status reads.
    let mut untracked_on: std::collections::HashMap<String, (u32, u32, bool)> = std::collections::HashMap::new();
    for (server, period) in holds.every_server() {
        let Some(reading) = period.reading.as_ref() else {
            continue;
        };
        servers_read += 1;
        let (name, badge) = what_a_server_is(&server);
        let stale = period.the_break_is_declared;
        let mut line = AServersLine {
            row: server.clone(),
            name: name.clone(),
            badge: badge.clone(),
            // **`Queue & History` C7: the moment the reading was taken**, as
            // seconds; the page writes it as a time, as History writes its own.
            last_seen: stale.then(|| in_seconds(reading.taken_at())),
            ..AServersLine::default()
        };
        let (mut untracked_rendering, mut untracked_waiting) = (0u32, 0u32);
        // **The rendering row first, as the drawing draws it, then the rest in
        // the machine's own order** — the line's (`Queue & History` Q4), which
        // the reading lists first, and a held job after it, having no place in
        // the line (`Queue & History` Q5). **Until the 38's piece 8 every row
        // came in the reading's order**, which puts the rendering job after
        // every waiting one; with nothing to drag that read only oddly, and
        // with dragging it made *the top of the queue* a place below the
        // waiting rows. A stable sort keeps the line's order. **A held row
        // stays where the reading puts it** rather than second, as the drawing
        // has it: there it would sit between a group's rendering member and
        // its waiting ones and split the group under two headings.
        let mut jobs: Vec<&smoozy_queue_history::readings::AJobAsTheMachineReadsIt> =
            reading.value().jobs.iter().collect();
        jobs.sort_by_key(|job| !matches!(job.state, QueueState::Rendering { .. }));
        // **The user's jobs this line holds, by identity** — so a first-frame
        // pair is drawn once, on the row of the part in line
        // ([`where_the_pair_is_drawn`]). The nineteenth notes, note 7.
        let in_this_line: std::collections::HashMap<String, &smoozy_queue_history::readings::AJobAsTheMachineReadsIt> =
            jobs.iter()
                .filter(|job| {
                    let tracked = the_record_of(&store, &job.job, &server);
                    tracked.is_some() && !it_has_left_the_queue(&job.state, tracked.as_ref())
                })
                .map(|job| (job.job.as_text(), *job))
                .collect();
        let the_frame_in_hand = |job: &smoozy_queue_history::readings::AJobAsTheMachineReadsIt| {
            period.the_figures_to_draw(job).and_then(|figures| figures.readings.value().frame_in_hand)
        };
        for job in jobs {
            let tracked = the_record_of(&store, &job.job, &server);
            // **`Queue & History` C11: when the job ends it leaves the queue**
            // — on the reading's word or on Smoozy's own, whichever came first.
            if it_has_left_the_queue(&job.state, tracked.as_ref()) {
                continue;
            }
            let Some(held) = tracked else {
                if period.the_offer.contains(&job.job) {
                    match job.state {
                        QueueState::Rendering { .. } => untracked_rendering += 1,
                        _ => untracked_waiting += 1,
                    }
                }
                continue;
            };
            // **`Design/Queue & History § A first-frame check and its job`: a
            // check and the full range waiting on it are one row**, and a pair
            // counts once wherever jobs are counted — so the full range is not
            // a row, a waiting job or a group member of its own while its
            // check is in line. The nineteenth notes, note 7.
            let (the_pair, the_parts) = match where_the_pair_is_drawn(&store, job, &in_this_line, &the_frame_in_hand) {
                WhereThePairIsDrawn::OnTheChecksRow => continue,
                WhereThePairIsDrawn::NotAPair => (None, Vec::new()),
                WhereThePairIsDrawn::Here(pair, parts) => (Some(pair), parts),
            };
            // **`Queue & History` H12: a job held for the driver** says what
            // it waits for, in the held amber, and never *Needs your answer*.
            // Held-job piece B.
            let for_the_driver = crate::servers::the_driver_swap::a_hold_for_the_driver(job);
            let (state, state_is) = match (&job.state, stale) {
                (_, true) => (Some(NOT_UPDATING), ""),
                // **`Queue & History` L12a and L2: while Smoozy's own stop or
                // cancel is in hand the row says so** (step 10c's S3).
                (_, false) if the_act_in_hand(&held.job) == Some(TheQueuesOwnAct::TheStop) => {
                    (Some(STOPPING), "run")
                }
                (_, false) if the_act_in_hand(&held.job).is_some() => (Some(CANCELLING), ""),
                (QueueState::Rendering { .. }, false) => (Some(RENDERING), "run"),
                (QueueState::Held { .. }, false) if for_the_driver => (Some(WAITING_FOR_DRIVER_UPDATE), "held"),
                (QueueState::Held { .. }, false) => (Some(WAITING_ON_YOU), "held"),
                _ => (None, ""),
            };
            match job.state {
                QueueState::Rendering { .. } => line.rendering += 1,
                _ => line.waiting += 1,
            }
            // **`Queue & History` L20: a group is drawn from the group
            // identity on the members' own local records**, joined to the
            // reading by job identity. Until step 10c's S3 nothing composed
            // one, so no heading — and no *Cancel all* — was ever drawn.
            let group = held.group.as_ref().map(|group| {
                let at = match screen.groups.iter().position(|heading| heading.group == group.as_text()) {
                    Some(at) => at,
                    None => {
                        screen.groups.push(AGroupHeading {
                            group: group.as_text(),
                            name: what_the_job_is_called(&held),
                            ..AGroupHeading::default()
                        });
                        screen.groups.len() - 1
                    }
                };
                let heading = &mut screen.groups[at];
                heading.scenes += 1;
                if matches!(job.state, QueueState::Rendering { .. }) {
                    heading.rendering = Some(
                        held.scene_name
                            .clone()
                            .unwrap_or_else(|| what_the_job_is_called(&held)),
                    );
                }
                at
            });
            // **`Queue & History` R5's figures, where they are to be drawn** —
            // a rendering job's, while the reading is updating and the figures
            // still move. Piece 6b.
            let progress = match job.state {
                QueueState::Rendering { .. } => period
                    .the_figures_to_draw(job)
                    .and_then(|figures| {
                        the_progress_from(
                            figures,
                            the_grammar_of(&held),
                            holds.what_the_reading_says(&held.job.as_text()).as_ref(),
                        )
                    }),
                _ => None,
            };
            if let (Some(progress), QueueState::Rendering { .. }) = (&progress, &job.state) {
                if let Some(whole) = progress.whole {
                    line.folded = Some((what_the_job_is_called(&held), whole));
                }
            }
            // **`Queue & History` L9 and Design, *The queue list*: only a
            // waiting row offers the grip** — never the rendering one, never a
            // held one, never one whose server is not updating (no move could
            // reach it), and never one Smoozy is already cancelling. The 38's
            // piece 8.
            let grip = matches!(job.state, QueueState::Waiting) && !stale && the_act_in_hand(&held.job).is_none();
            if let (true, Some(at)) = (grip, group) {
                screen.groups[at].grip = true;
            }
            line.rows.push(ARowInTheQueue {
                job: held.job.as_text(),
                name: what_the_job_is_called(&held),
                the_pair: the_parts,
                state,
                state_is,
                not_updating: stale,
                grip,
                group,
                // **Design, *The queue list*: a row carries both bars**, each
                // where it is known — the row draws each bar it has (W4).
                bars: progress
                    .as_ref()
                    .map(|progress| TheRowsBars {
                        now_at: progress.now_at,
                        whole: progress.whole,
                    })
                    .unwrap_or_default(),
                ..ARowInTheQueue::default()
            });
            entries.push((held, job.state.clone(), stale, job.the_hold.clone(), progress, for_the_driver, the_pair));
        }
        if untracked_rendering + untracked_waiting > 0 {
            let next_is_untracked = reading
                .value()
                .the_admission_point_stands_at
                .as_ref()
                .is_some_and(|next| period.the_offer.contains(next));
            untracked_on.insert(server.clone(), (untracked_rendering, untracked_waiting, next_is_untracked));
        }
        if untracked_rendering + untracked_waiting > 0 && screen.untracked.is_none() {
            screen.untracked = Some(TheUntrackedWork {
                server: name.clone(),
                server_row: server.clone(),
                badge: badge.clone(),
                rendering: untracked_rendering,
                waiting: untracked_waiting,
                cause: None,
            });
        }
        screen.jobs += line.rows.len() as u32;
        if !line.rows.is_empty() {
            screen.lines.push(line);
        }
    }
    // **`Queue & History` C10: a job just submitted is in its server's line
    // at once**, at the end of it, until the machine's own row replaces it —
    // the Creator's seventh notes, piece Z7. **It counts among the header's
    // jobs and never as rendering or waiting.**
    let the_reading_of = |server: &str| {
        let period = holds.every_server().into_iter().find(|(held, _)| held == server)?.1;
        let reading = period.reading.as_ref()?;
        Some((
            reading.taken_at(),
            reading.value().jobs.iter().map(|job| job.job.clone()).collect(),
        ))
    };
    for act in crate::submitting::the_acts_in_flight_to_draw(&the_reading_of) {
        // **Only a server this record holds a row for** — the press submits
        // to no other.
        if !rows.iter().any(|row| row.local_server_identifier == act.server) {
            continue;
        }
        let at = match screen.lines.iter().position(|line| line.row == act.server) {
            Some(at) => at,
            None => {
                let (name, badge) = what_a_server_is(&act.server);
                screen.lines.push(AServersLine {
                    row: act.server.clone(),
                    name,
                    badge,
                    ..AServersLine::default()
                });
                screen.lines.len() - 1
            }
        };
        // **`Queue & History` C10 as changed 2026-10-19: an act that ended
        // with a job not in the line stays, saying so** — and is not a job
        // the header counts, nothing of it being in the line.
        if let Some(came_to) = &act.not_submitted {
            let row = a_row_not_submitted(&act, came_to);
            screen.lines[at].rows.push(row);
            continue;
        }
        screen.lines[at].rows.push(ARowInTheQueue {
            name: act.name,
            state: Some(SUBMITTING_ROW),
            submitting: true,
            ..ARowInTheQueue::default()
        });
        screen.jobs += 1;
    }
    screen.servers = servers_read;
    screen.notice = the_notice_from(connection, &store);
    // **`Rendering` S10: a several-scene act the connection broke part-way**,
    // held for the run and resolved against the record at each draw.
    screen.part = the_part_now(&store, &|server: &str| {
        // **A part for a server this record holds no row for says nothing**:
        // the server was removed, and its scenes with it.
        if !rows.iter().any(|row| row.local_server_identifier == server) {
            return None;
        }
        let connected = the_servers_connected(holds).contains(server);
        Some((what_a_server_is(server).0, connected))
    });

    // **A pair's row is its part in line**, so a page asking for the other
    // part's entry — the full range's, chosen before its check came back into
    // view — is answered with the row's. The nineteenth notes, note 7.
    let chosen_one = chosen
        .and_then(|job| {
            entries
                .iter()
                .find(|(held, ..)| held.job.as_text() == job)
                .or_else(|| {
                    entries.iter().find(|(.., pair)| {
                        pair.as_ref().is_some_and(|pair| pair.check.as_text() == job || pair.range.as_text() == job)
                    })
                })
        })
        .or_else(|| entries.first());
    if let Some((held, state, stale, the_hold, progress, for_the_driver, the_pair)) = chosen_one {
        let beside = crate::surface::the_history::BesideTheRecord::read(connection, &held.job);
        let mut entry = crate::surface::the_history::the_entry_for(held, &beside, false, true).entry;
        the_writing_to_path_opens_files(&mut entry);
        // The first-frame check's picture belongs to the job the hold names.
        // Only its preview is borrowed: the entry and both answers still
        // belong to the waiting job. The reading carries this link after a
        // reopen too (the seventeenth notes, piece 2).
        if matches!(state, QueueState::Held { .. }) && !*for_the_driver {
            if let Some(check) = the_saved_first_frame_check_of(connection, &held.job, the_hold.as_ref()) {
                entry.preview = the_checks_picture(connection, &check.the_first_job);
            }
        }
        // **Design, *The rest of the entry*: down the side, the latest frame** —
        // the same picture History draws as *last complete*. The 38's piece 3.
        if let Some(preview) = entry.preview.as_mut().filter(|preview| !preview.nothing_to_show) {
            preview.which = LATEST;
            preview.follows = true;
        }
        // **`Design/Queue & History § A first-frame check and its job`: the
        // check on its pair's entry**, in the reading the pair is at. While it
        // waits the picture moves into the question box and the side column
        // draws none. The nineteenth notes, note 7.
        if let Some(pair) = the_pair {
            entry.the_check = Some(the_check_in_the_queue(connection, pair, &held.job, state, *for_the_driver));
            if entry.the_check.as_ref().is_some_and(|check| check.in_the_question) {
                entry.preview = None;
            }
        }
        let (status, status_is, stop_or_cancel) = match (state, stale) {
            (_, true) => (vec![APart::words(NOT_UPDATING)], "", None),
            (QueueState::Rendering { .. }, false) => {
                (vec![APart::words(RENDERING)], "busy", Some(TheStopOrCancel::Stop))
            }
            // **`Queue & History` H12 and the drawing's `q-driver`**: the
            // status line says what it waits for, and *Cancel* sits where
            // *Stop* sits, nothing rendering to stop.
            (QueueState::Held { .. }, false) if *for_the_driver => {
                (vec![APart::words(WAITING_FOR_DRIVER_UPDATE)], "", Some(TheStopOrCancel::Cancel))
            }
            (QueueState::Held { .. }, false) => {
                (vec![APart::words(WAITING_ON_YOU)], "", Some(TheStopOrCancel::Cancel))
            }
            _ => (Vec::new(), "", Some(TheStopOrCancel::Cancel)),
        };
        // **Step 10c's S3: what Smoozy knows of its own act on this job**
        // speaks over the reading's word. **While the reading is not updating
        // `Queue & History` C7's statement comes first**, and the act is said
        // beside it rather than over it ([`what_a_not_updating_entry_says`]).
        let (status, status_is, stop_or_cancel) = match (*stale, where_the_act_stands(&held.job.as_text())) {
            // **While the reading is not updating, a stop or cancel still
            // waited on, or given up on, is said beside `Queue & History`
            // C7's word** — and nothing is offered, since until the server is
            // updating again no act can reach it. Step 10c's eleventh-pass
            // fix piece 9.
            (true, stands) => (what_a_not_updating_entry_says(stands.as_ref()), "", None),
            (false, Some(stands)) => {
                let (mut said, is) = what_the_entry_says_of(&stands);
                // **One press at a time**: while the act is in hand its control
                // is not offered; once it has ended, the reading's is again —
                // `Rendering` K10's *a new stop can be sent*.
                let offered = match stands {
                    WhereTheActStands::InHand(_) => None,
                    _ => stop_or_cancel,
                };
                // **And the entry says so, where *Stop* is beside it** — never
                // beside *Not updating*, where nothing is offered.
                if matches!(stands, WhereTheActStands::TheWaiterGaveUp(TheQueuesOwnAct::TheStop))
                    && offered == Some(TheStopOrCancel::Stop)
                {
                    said.push(APart::words(PRESS_STOP_AGAIN));
                }
                (said, is, offered)
            }
            _ => (status, status_is, stop_or_cancel),
        };
        // **`Queue & History` A4: a waiting job the reading has nothing to
        // say about, behind work Smoozy is not tracking that is rendering or
        // would start next**, has its stated cause — drawn where *Ignore*
        // stands ([`the_decline_is_applied`]). The 38's piece 15.
        if matches!(state, QueueState::Waiting) && !*stale && status.is_empty() {
            let server = held.server.local_server.as_text();
            if let Some(&(rendering, waiting, next_is_untracked)) = untracked_on.get(&server) {
                if rendering > 0 || next_is_untracked {
                    screen.the_entry_waits_behind_untracked_work = Some((server, rendering + waiting));
                }
            }
        }
        entry.status = status;
        entry.status_is = status_is;
        entry.stop_or_cancel = stop_or_cancel;
        // **While the reading is not updating no act reaches the machine**, so
        // *Files on the server* and its link go with *Stop* and *Cancel*.
        entry.the_server_is_not_connected = *stale;
        // **Design, *A rendering entry*: the two bars, each with its whole
        // reading above it** — `Queue & History` R5's figures through `bars`.
        // Piece 6b.
        entry.progress = progress.clone();
        // **`Queue & History` E8: a rendering entry offers *Blender output*
        // while the reading is updating and names the job's sinks** — the
        // control acts there ([`the_running_words_are_opened`]). Piece 6c.
        let rendering_on = (!*stale)
            .then(|| holds.where_a_rendering_jobs_words_are(&held.job.as_text()))
            .flatten();
        entry.blender_output = rendering_on.is_some();
        // **`Desktop app and delivery` M13b's one narrowing: the same entry
        // shows the ordinary stream's newest three lines**, and only while the
        // job renders on an updating reading — never on an ended entry, never
        // beside *Not updating*, never on History. Drawing the entry is what
        // puts them on screen for P10d's read (`Queue & History` E8). The
        // Creator's notes on the test copy, piece 8 (note 12).
        let job = held.job.as_text();
        entry.lines = holds.the_entry_is_drawn_for(rendering_on.as_deref().map(|server| (server, job.as_str())));
        // **`Queue & History` H1, `Queue & History` H5 and `Queue & History` H6,
        // and `Design/Queue & History §
        // What the job writes`: the first-frame check's held job offers both
        // answers**, naming the job's own first frame and the held job's own
        // count — step 10c's eleventh-pass fix piece 8. Until then nothing
        // composed the pair, so the entry read *Waiting on you* with nothing
        // to answer.
        //
        // - **Only while the reading is updating and the hold is unanswered as
        //   far as Smoozy knows** (`Queue & History` H5): not while an answer or a cancel of its
        //   own is in hand, and not once the machine has said the hold is
        //   answered.
        // - **Only for a first-frame check** ([`the_first_frame_check_of`]
        //   says how that is known): a job held at the start of its render
        //   asks the other question, in other words.
        // - ***Frame 40 is rendered and downloaded.* only once the first job
        //   has finished** — Rule 8: never said before it is true. **And
        //   *downloaded* only where the frame was** ([`the_frame_was_downloaded`],
        //   step 10c's RunPod-pass fix piece 1); otherwise *Frame 40 is
        //   rendered.*
        // **`Queue & History` H12: a job held for the driver offers no
        // answers** — H5's offer is for a hold awaiting a person — and draws
        // the held job's box with no buttons, saying what it waits for. Only
        // while the reading is updating, as every held box. Held-job piece B.
        if let (QueueState::Held { .. }, false, true) = (state, *stale, *for_the_driver) {
            entry.held = vec![the_job_is_held_for_the_driver(&held.blender.pick.version)];
        }
        if let (QueueState::Held { .. }, false, false) = (state, *stale, *for_the_driver) {
            let answer = where_the_answer_stands(&held.job.as_text());
            if let Some(said) = answer.as_ref().and_then(WhereTheAnswerStands::what_the_entry_says) {
                entry.status = said;
                entry.status_is = "";
            }
            let offered = the_act_in_hand(&held.job).is_none()
                && answer.as_ref().is_none_or(WhereTheAnswerStands::the_pair_is_offered_again);
            if let Some(check) = the_saved_first_frame_check_of(connection, &held.job, the_hold.as_ref()).filter(|_| offered) {
                let mut pair = match check.the_count {
                    Some((the_first_frame, frames_held)) => the_first_frame_is_held(
                        &the_first_frame.to_string(),
                        frames_held,
                        the_frame_was_downloaded(connection, &check.the_first_job, the_first_frame),
                    ),
                    None => the_first_frame_is_held_with_no_count(),
                };
                let the_first_frame_is_rendered = store
                    .read(&check.the_first_job)
                    .ok()
                    .flatten()
                    .and_then(|first| first.the_ending_learned())
                    == Some(smoozy_queue_history::ending::Ending::Finished);
                // **The no-count sentence claims nothing about the frame**, so
                // it stays whether or not the check has finished.
                if !the_first_frame_is_rendered && check.the_count.is_some() {
                    pair.said.clear();
                }
                let the_first_frame = the_pair
                    .as_ref()
                    .filter(|saved| saved.check == check.the_first_job)
                    .map(|saved| saved.first_frame.to_string())
                    .or_else(|| check.the_count.as_ref().map(|(first, _)| first.to_string()));
                if the_first_frame_is_rendered && entry.status == vec![APart::words(WAITING_ON_YOU)] {
                    if let Some(first) = &the_first_frame {
                        entry.status.push(APart::words(THE_DASH));
                        entry.status.push(the_first_frame_was_rendered(first));
                    }
                }
                // **A pair's question names the full range it asks about** —
                // *Frame 1001 downloaded. Render frames 1001–1240?*, the frames as typed
                // (`Queue & History` P5b). The nineteenth notes, note 7.
                if let Some(saved) = the_pair.as_ref().filter(|saved| saved.check == check.the_first_job) {
                    pair.said = the_question_of_a_pair(
                        saved,
                        the_first_frame_is_rendered,
                        the_frame_was_downloaded(connection, &saved.check, saved.first_frame),
                    );
                }
                entry.held = vec![pair];
            }
        }
        // **`Queue & History` H1 for a job held at the start of its render**:
        // the conditions the hold record names (`Rented-server runtime` U2),
        // each with Design's *Stop this job* and *Continue anyway* — only
        // while the reading is updating, the hold unanswered as far as Smoozy
        // knows, and no act of its own in hand. The 38's piece 10, item 26.
        let the_holds_conditions = match (state, *stale) {
            (QueueState::Held { .. }, false) if !*for_the_driver => the_hold
                .as_ref()
                .and_then(|hold| smoozy_job_files::holds::the_conditions_a_hold_reason_names(&hold.reason)),
            _ => None,
        };
        if let Some(conditions) = &the_holds_conditions {
            let offered = the_act_in_hand(&held.job).is_none()
                && where_the_answer_stands(&held.job.as_text())
                    .as_ref()
                    .is_none_or(WhereTheAnswerStands::the_pair_is_offered_again);
            if offered && entry.held.is_empty() {
                entry.held = conditions.iter().copied().map(the_job_is_held_at_its_start).collect();
            }
        }
        // **`Queue & History` N9 and `Job files and outputs` H8, H10: the six
        // statements, from the reading the fill-in read** — the two that hold
        // the job said by its pair instead while it is held.
        // **`Queue & History` L1a: the auto-download switch and the narrowing
        // sit on the entry**, read from the arrangement the record keeps
        // (`Download` R1) — the 38's piece 9, item 9. Until then nothing
        // composed the group, so neither control was ever drawn.
        entry.downloading = Some(the_downloading_on(connection, &held.job, holds.the_whole_of(&job).map(|whole| whole.files)));
        // **`Telegram` V1c: the *Telegram* group**, on a live job where
        // Telegram is on in Settings. The twenty-first notes, note 7.
        entry.telegram = the_telegram_group_on(connection, &held.job, &held.server.local_server.as_text(), state, *stale, the_pair.as_ref().map(|pair| pair.range.as_text()));
        if let Some(says) = holds.what_the_reading_says(&held.job.as_text()) {
            let held_by: &[smoozy_job_files::holds::WhatHoldsTheJob] = if entry.held.is_empty() {
                &[]
            } else {
                the_holds_conditions.as_deref().unwrap_or(&[])
            };
            entry.writes = what_the_job_writes(&says, held_by);
        }
        screen.entry = Some(entry);
    } else {
        holds.the_entry_is_drawn_for(None);
    }
    screen
}

/// **`Telegram` V1c: the entry's *Telegram* group**, or nothing — on an ended
/// job, or where Telegram is off in Settings (`Design/Queue & History § The
/// entry's controls`). Unavailable while the reading is not updating; on a
/// server whose Smoozy is older, unavailable with its sentence.
pub(crate) fn the_telegram_group_on(
    connection: &rusqlite::Connection,
    job: &smoozy_state::identities::Identity,
    server: &str,
    state: &smoozy_queue_history::line::QueueState,
    stale: bool,
    the_pairs_range: Option<String>,
) -> Option<TheTelegramGroup> {
    use crate::servers::published::{a_live_jobs_reporting_can_be_changed_on, ALiveJobsReportingCanBeChanged};
    if matches!(state, smoozy_queue_history::line::QueueState::Ended { .. }) {
        return None;
    }
    if !crate::record::settings::the_telegram_settings(connection).is_ok_and(|telegram| telegram.switched_on) {
        return None;
    }
    let picked = crate::record::job_records::JobRecords::over(connection)
        .the_reporting_choice_of(job)
        .ok()
        .flatten()
        .and_then(|text| smoozy_render_settings::reporting::ReportingChoice::from_the_manifests_form(&text));
    let stands = if stale {
        WhereTheTelegramControlStands::NotConnected
    } else if crate::servers::published::the_telegram_role_on(server) == Some(false) {
        WhereTheTelegramControlStands::TelegramIsNotInstalled
    } else {
        match a_live_jobs_reporting_can_be_changed_on(server) {
            Some(ALiveJobsReportingCanBeChanged::Yes) => WhereTheTelegramControlStands::Offered,
            Some(ALiveJobsReportingCanBeChanged::TheProgramIsOlder) => {
                WhereTheTelegramControlStands::TheServersSmoozyIsOlder
            }
            None => WhereTheTelegramControlStands::NotYetKnown,
        }
    };
    Some(TheTelegramGroup { picked, sent_as: the_pairs_range.unwrap_or_else(|| job.as_text()), stands })
}

/// **`Telegram` V1c: the *Telegram* group, said** — the heading, every choice
/// with its words (Render Settings' own, in its order), the one picked, what
/// the closed control reads, the minutes where the fourth is picked, and where
/// the control stands; on an older server, its grey line.
pub(crate) fn the_telegram_group_is_said(mut event: Event, group: Option<&TheTelegramGroup>) -> Event {
    use crate::surface::render_settings::{words, REPORTING_CHOICES};
    use smoozy_render_settings::reporting::ReportingChoice;
    let Some(group) = group else {
        return event;
    };
    event = event
        .saying("telegramHeading", Said::Prose(Prose::new(TELEGRAM_HEADING)))
        .saying("telegramLabel", Said::Prose(Prose::new(WHAT_THIS_JOB_REPORTS)))
        .saying("telegramJob", Said::Value(Verbatim::new(group.sent_as.clone())))
        .saying(
            "telegramStands",
            Said::Value(Verbatim::new(match group.stands {
                WhereTheTelegramControlStands::Offered => "offered",
                WhereTheTelegramControlStands::NotConnected => "not-connected",
                WhereTheTelegramControlStands::TheServersSmoozyIsOlder => "older",
                WhereTheTelegramControlStands::NotYetKnown => "not-yet-known",
                WhereTheTelegramControlStands::TelegramIsNotInstalled => "not-installed",
            })),
        );
    for (at, (value, said)) in REPORTING_CHOICES.iter().enumerate() {
        event = event
            .saying(a_name("telegramChoice", at, "Value"), Said::Value(Verbatim::new(*value)))
            .saying(a_name("telegramChoice", at, "Said"), Said::Prose(Prose::new(*said)));
    }
    event = event.saying(
        "telegramChoiceCount",
        Said::Value(Verbatim::new(REPORTING_CHOICES.len().to_string())),
    );
    if let Some(picked) = group.picked {
        let id = the_identifier_of(picked);
        let reads = match picked {
            ReportingChoice::APictureEveryFewMinutes { .. } => words::A_PICTURE_EVERY,
            _ => REPORTING_CHOICES.iter().find(|(value, _)| *value == id).map_or("", |(_, said)| *said),
        };
        event = event
            .saying("telegramPicked", Said::Value(Verbatim::new(id)))
            .saying("telegramReads", Said::Prose(Prose::new(reads)));
        if let ReportingChoice::APictureEveryFewMinutes { minutes } = picked {
            event = event
                .saying("telegramMinutes", Said::Value(Verbatim::new(minutes.to_string())))
                .saying("telegramMinutesWord", Said::Prose(Prose::new(words::MINUTES)))
                .saying("telegramMinutesName", Said::Prose(Prose::new(words::MINUTES_BETWEEN_PICTURES)));
        }
    }
    if group.stands == WhereTheTelegramControlStands::TheServersSmoozyIsOlder {
        event = event.saying(
            "telegramOlder",
            Said::Prose(Prose::new(UPDATE_SMOOZY_ON_THIS_SERVER_TO_CHANGE_THIS)),
        );
    }
    if group.stands == WhereTheTelegramControlStands::TelegramIsNotInstalled {
        event = event.saying(
            "telegramNotInstalled",
            Said::Prose(Prose::new(crate::surface::set_up::words::TELEGRAM_NOT_INSTALLED)),
        );
    }
    event
}

/// **The entry's *Downloading* group, from the arrangement the record keeps** —
/// `Queue & History` L1a and `Design/Queue & History § The rest of the entry`:
/// the switch, the narrowing under it, and the count both produce. **No
/// arrangement is a switch that is off**, narrowed to nothing and with no
/// count. **The count is files downloaded of the whole** — `Download` N6's
/// whole where a pass in this run has read it, otherwise the reckoning's
/// length, which is what Transfers' own row reads. The 38's piece 9, item 9.
pub(crate) fn the_downloading_on(
    connection: &rusqlite::Connection,
    job: &smoozy_state::identities::Identity,
    the_whole_read: Option<usize>,
) -> TheDownloading {
    use smoozy_download::arrangement::{DesiredState, KeepsTheArrangement as _, TheNarrowing, WhatSmoozyDid};
    let Some(arrangement) = crate::record::download_records::DownloadArrangements::over(connection)
        .the_arrangement_for(job)
        .ok()
        .flatten()
    else {
        return TheDownloading::default();
    };
    let fetched = arrangement
        .reckoning
        .iter()
        .filter(|entry| entry.account.state == WhatSmoozyDid::Fetched)
        .count();
    TheDownloading {
        on: arrangement.desired_state == DesiredState::On,
        narrowing: match &arrangement.narrowing {
            TheNarrowing::Everything => String::new(),
            // **Each format in Smoozy's words**, from piece 2's one table
            // (`render_settings::a_format_in_smoozys_words`, hole 17 of the
            // Creator's second notes) — *Multilayer EXR*, never Blender's
            // `OPEN_EXR_MULTILAYER`. Rule 9. **The identifier stays the
            // record's**: this is what the entry reads, and nothing hands it
            // back — the narrowing's control opens the format box over the job,
            // which reads the arrangement's own identifiers.
            TheNarrowing::TheseFormats(formats) => formats
                .iter()
                .map(|format| crate::surface::render_settings::a_format_in_smoozys_words(format))
                .collect::<Vec<_>>()
                .join(", "),
        },
        done: u32::try_from(fetched).unwrap_or(u32::MAX),
        of: u32::try_from(the_whole_read.unwrap_or(arrangement.reckoning.len())).unwrap_or(u32::MAX),
    }
}

/// **One condition of a job held at the start of its render, as its pair** —
/// `Design/Queue & History § What the job writes`: *"those two read as a
/// question with Stop and Continue beside them"*, the pair reading **Stop this
/// job** and **Continue anyway**, in the drawing's order. **The first control
/// is the *no* and the second the *yes*** on this pair, the opposite of the
/// first-frame check's, which [`the_answer_the_control_gives`] keeps. The 38's
/// piece 10, item 26.
pub fn the_job_is_held_at_its_start(condition: smoozy_job_files::holds::WhatHoldsTheJob) -> TheJobIsHeld {
    use smoozy_job_files::holds::WhatHoldsTheJob;
    let (said, second) = match condition {
        WhatHoldsTheJob::TheJobWillWriteNothingAtAll => (THIS_JOB_WRITES_NO_FILES, THE_SERVERS_TIME_WOULD_BE_SPENT),
        WhatHoldsTheJob::TwoDestinationsAimedAtOneNameInOnePlace => (TWO_DESTINATIONS_COLLIDE, ONE_OVERWRITES_THE_OTHER),
        WhatHoldsTheJob::ADestinationCannotBeBroughtUnderTheJobsFolder => {
            (PART_OF_IT_LANDS_OUTSIDE_ITS_FOLDER, THOSE_FILES_ARE_NOT_DOWNLOADED)
        }
    };
    TheJobIsHeld {
        said: vec![APart::words(said)],
        second,
        first_answer: vec![APart::words(STOP_THIS_JOB)],
        second_answer: CARRY_ON_ANYWAY,
        note: NOTHING_HAS_RENDERED_YET,
    }
}

/// **A job held for the driver update, as its box** — `Design/Queue & History
/// § A job held for the driver update` and `Queue & History` H12: the held
/// job's amber-edged box **with no buttons in it**, since nothing waits on a
/// person. It names the job's own Blender, says what happens next and the
/// restart, and that it needs Smoozy connected. **No first answer, no second
/// answer**: the page draws a button only where one is sent. Held-job piece B,
/// 2026-10-12.
pub fn the_job_is_held_for_the_driver(the_jobs_blender: &str) -> TheJobIsHeld {
    TheJobIsHeld {
        said: vec![APart::said(
            THE_JOBS_BLENDER_LEAD,
            the_blenders_line(the_jobs_blender),
            if the_blender_cannot_use_the_card_below_its_bar(the_jobs_blender) {
                NEEDS_A_NEWER_NVIDIA_DRIVER_FOR_CYCLES_GPU
            } else {
                NEEDS_A_NEWER_NVIDIA_DRIVER_FOR_OPTIX
            },
        )],
        second: SMOOZY_WILL_UPDATE_THE_DRIVER,
        first_answer: Vec::new(),
        second_answer: "",
        note: THIS_ONLY_HAPPENS_WHILE_CONNECTED,
    }
}

/// **Whether this Blender's row in `Rendering` L23a's table stops Cycles on the
/// card, not OptiX alone** (that table's bullet of 2026-11-02): what the words
/// about its driver follow. **A version that does not read keeps the OptiX
/// words**, which were never false of it. One question, asked of the table.
pub(crate) fn the_blender_cannot_use_the_card_below_its_bar(version: &str) -> bool {
    use smoozy_servers::hardware::{the_optix_bar_row, WhatTheBarStops};
    smoozy_server_setup::blender::Version::parse(version.split_whitespace().next().unwrap_or_default())
        .and_then(|read| the_optix_bar_row((read.major, read.minor, read.patch)))
        .is_some_and(|row| row.below_it == WhatTheBarStops::CyclesOnTheCard)
}

/// **A Blender named by its line** — *5.2* for 5.2.1, *5.1* for 5.1.0 — the
/// way `Design/Queue & History § A job held for the driver update` and its
/// seventh mark name the job's Blender beside its OptiX bar, which is a
/// line's. The exact version stays in the setup's *Blender* row. A version
/// with fewer than two parts is named whole.
pub(crate) fn the_blenders_line(version: &str) -> String {
    let mut parts = version.split('.');
    match (parts.next(), parts.next()) {
        (Some(major), Some(minor)) if !major.is_empty() && !minor.is_empty() => format!("{major}.{minor}"),
        _ => version.to_string(),
    }
}

/// **Which answer each of the held pair's two controls gives** — the
/// first-frame check's *Render all* is the *yes* and its *Cancel* the *no*;
/// a job held at the start of its render has **Stop this job** first, the
/// *no*, and **Continue anyway** second, the *yes*. Told apart by the hold
/// record's own reason (`Rented-server runtime` U2) and nothing else. The 38's
/// piece 10, item 26.
pub(crate) fn the_answer_the_control_gives(
    the_first: bool,
    the_holds_reason: Option<&str>,
) -> crate::servers::the_machines_work::TheHeldJobsAnswer {
    use crate::servers::the_machines_work::TheHeldJobsAnswer;
    let held_at_its_start = the_holds_reason
        .and_then(smoozy_job_files::holds::the_conditions_a_hold_reason_names)
        .is_some();
    match (the_first, held_at_its_start) {
        (true, false) | (false, true) => TheHeldJobsAnswer::RenderAll,
        (false, false) | (true, true) => TheHeldJobsAnswer::CancelIt,
    }
}

/// **The six statements on the entry** — `Design/Queue & History § What the
/// job writes`, `Job files and outputs` H8, in that order, then `Job files and
/// outputs` I9a's one line under them. **Each only where it is true**, and the two that hold the job
/// left to its pair while `held_by` names them. The 38's piece 10.
pub(crate) fn what_the_job_writes(
    says: &crate::servers::the_machines_work::WhatTheReadingSays,
    held_by: &[smoozy_job_files::holds::WhatHoldsTheJob],
) -> Vec<AMarkOnTheEntry> {
    use crate::surface::render_settings::words as the_page;
    use smoozy_job_files::holds::{WhatHoldsTheJob, WhyItWillWriteNothing};
    let six = &says.six;
    let beneath_the_job = |path: &[u8]| -> String {
        let beneath = path
            .strip_prefix(says.job_output_folder.as_bytes())
            .map(|rest| rest.strip_prefix(b"/").unwrap_or(rest))
            .unwrap_or(path);
        String::from_utf8_lossy(beneath).into_owned()
    };
    let mut writes = Vec::new();
    if six.it_is_writing_nothing.is_none() && !six.the_folders_it_is_writing_into.is_empty() {
        let mut parts = vec![APart::words(WRITING_INTO)];
        let last = six.the_folders_it_is_writing_into.len() - 1;
        for (at, folder) in six.the_folders_it_is_writing_into.iter().enumerate() {
            let trail = if at < last { A_COMMA } else { "" };
            if folder.is_empty() {
                parts.push(APart::said(THE_JOB_FOLDER, "", trail));
            } else {
                parts.push(APart::figure(String::from_utf8_lossy(folder).into_owned(), trail));
            }
        }
        if let Some(count) = says.how_many_files {
            parts.push(APart::said(
                A_DASH,
                count.to_string(),
                if count == 1 { the_page::FILE_WORD } else { the_page::FILES_WORD },
            ));
        }
        writes.push(AMarkOnTheEntry {
            said: "",
            parts,
            way: None,
            bold: 0,
        });
    }
    if let Some(why) = six.it_is_writing_nothing {
        if !held_by.contains(&WhatHoldsTheJob::TheJobWillWriteNothingAtAll) {
            writes.push(AMarkOnTheEntry {
                said: THIS_JOB_WRITES_NO_FILES,
                parts: vec![APart::words(match why {
                    WhyItWillWriteNothing::NoFrameHasAnActiveViewLayer => NO_FRAME_HAS_AN_ACTIVE_VIEW_LAYER,
                    WhyItWillWriteNothing::NothingWasAimedAnywhere => THE_OUTPUT_SETTINGS_SAVE_NOTHING,
                })],
                way: None,
                bold: 0,
            });
        }
    }
    if !six.the_colliding_pairs.is_empty()
        && !held_by.contains(&WhatHoldsTheJob::TwoDestinationsAimedAtOneNameInOnePlace)
    {
        let mut parts: Vec<APart> = six
            .the_colliding_pairs
            .iter()
            .map(|pair| APart::figure(beneath_the_job(&pair.predicted_path), IS_WRITTEN_BY_BOTH))
            .collect();
        parts.push(APart::words(SMOOZY_CHANGES_NOTHING_IN_YOUR_FILE));
        writes.push(AMarkOnTheEntry {
            said: TWO_DESTINATIONS_COLLIDE,
            parts,
            way: None,
            bold: 0,
        });
    }
    // **The duplicate-picture line names the places the walk found** — the
    // runner's ruling of 2026-10-12, in the settings page's own words. **It is
    // bold whole**, its places with its opening words, as the drawing's
    // `q-marks` state draws it (the Creator's ninth notes, note 1, its E4 half).
    let mut twice = crate::surface::render_settings::the_beauty_written_more_than_once(
        &six.where_the_same_picture_is_written,
    )
    .into_iter();
    if let (true, Some((lead, figure, trail))) =
        (six.the_same_picture_reaches_more_than_one_place, twice.next())
    {
        let mut parts = Vec::new();
        if let Some(count) = figure {
            parts.push(APart::figure(count.to_string(), trail));
        }
        parts.extend(twice.map(|(lead, figure, trail)| APart {
            lead,
            value: figure.map(|count| count.to_string()).unwrap_or_default(),
            trail,
        }));
        writes.push(AMarkOnTheEntry {
            said: lead,
            bold: parts.len(),
            parts,
            way: None,
        });
    }
    // **A video and frames are two outputs**, named as what they are and never
    // the same picture written twice (the Creator's eighth notes, note 4).
    if six.the_render_reaches_frames_and_a_video {
        writes.push(AMarkOnTheEntry {
            said: the_page::FRAMES_AND_A_VIDEO,
            parts: Vec::new(),
            way: None,
            bold: 0,
        });
    }
    if let Some(other) = &six.the_tree_is_shared_with {
        writes.push(AMarkOnTheEntry {
            said: SHARES_ITS_COMPOSITOR_WITH,
            parts: vec![
                APart::figure(String::from_utf8_lossy(other.as_bytes()).into_owned(), "."),
                APart::words(THE_PICTURES_MAY_BELONG_TO_THAT_SCENE),
            ],
            way: None,
            bold: 0,
        });
    }
    if six.its_output_is_a_single_video_file {
        writes.push(AMarkOnTheEntry {
            said: the_page::ONE_VIDEO_FILE_LEAD,
            parts: vec![
                APart::words(the_page::ONE_VIDEO_FILE_SPLITS),
                APart::words(the_page::ONE_VIDEO_FILE_REST),
            ],
            way: None,
            bold: 0,
        });
    }
    if says.the_frames_cannot_be_counted {
        writes.push(AMarkOnTheEntry {
            said: the_page::MORE_THAN_ONE_VIEW,
            parts: Vec::new(),
            way: None,
            bold: 0,
        });
    }
    writes
}

/// **Whether a first-frame check's frame was downloaded** — `Download` R4's
/// reckoning for the first job holds that frame, and every entry naming it is
/// *fetched* (`Download` R9). **No arrangement is no download**: with
/// *Download frames as they render* off the job never had one (`Download` R1).
/// A frame the user fetched by hand from Files is not counted, so the
/// sentence then says *rendered* alone, which is still true.
///
/// *Written for* *Frame N is rendered and downloaded.* *as a fixed sentence
/// (step 10c's eleventh-pass fix piece 8), which was true only while
/// downloads were on and the frame had come down. The pass before Vast on
/// RunPod met it with downloads off.*
fn the_frame_was_downloaded(
    connection: &rusqlite::Connection,
    the_first_job: &smoozy_state::identities::Identity,
    the_first_frame: i64,
) -> bool {
    use smoozy_download::arrangement::{KeepsTheArrangement, WhatSmoozyDid};
    let Ok(Some(arrangement)) =
        crate::record::download_records::DownloadArrangements::over(connection).the_arrangement_for(the_first_job)
    else {
        return false;
    };
    let mut naming_it = arrangement
        .reckoning
        .iter()
        .filter(|entry| entry.frame == Some(the_first_frame))
        .peekable();
    naming_it.peek().is_some() && naming_it.all(|entry| entry.account.state == WhatSmoozyDid::Fetched)
}

// ── A first-frame check and its job — the nineteenth notes, note 7 ─────────

/// **A first-frame pair, as the saved purpose names it** — the descriptive
/// values `Queue & History` P5a keeps on the local record, so it holds after
/// a reopen.
/// **A record without them is no pair**: a legacy check, one taken over from a
/// server, and every ordinary job are ordinary rows, and nothing guesses a
/// check from a one-frame range (`Design/Queue & History § A first-frame
/// check and its job`, *Older records stay as they were*). The saved purpose
/// never answers anything: the machine's hold stays the approval authority
/// ([`ThePair::the_hold_agrees`]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThePair {
    pub check: smoozy_state::identities::Identity,
    pub range: smoozy_state::identities::Identity,
    /// The check's one frame.
    pub first_frame: i64,
    /// **The full range's frames as the screen names them** — exactly as the
    /// user typed them where the range job's record holds that text
    /// (`Queue & History` P5b), and the submitted numbers otherwise.
    pub the_frames: String,
}

impl ThePair {
    /// **The pair `job` is a part of**, read off its own record.
    pub fn of(
        store: &crate::record::job_records::JobRecords<'_>,
        job: &smoozy_state::identities::Identity,
    ) -> Option<ThePair> {
        use smoozy_state::identities::{Identity, IdentityKind};
        let purpose = store.first_frame_purpose(job).ok().flatten()?;
        let check = Identity::read_back(IdentityKind::Job, &purpose.check_job)?;
        let range = Identity::read_back(IdentityKind::Job, &purpose.range_job)?;
        let the_frames = the_full_ranges_frames(store.the_typed_frames_of(&range).ok().flatten(), &purpose.submitted_range);
        Some(ThePair { check, range, first_frame: purpose.first_frame, the_frames })
    }

    pub fn is_the_check(&self, job: &smoozy_state::identities::Identity) -> bool {
        self.check == *job
    }

    /// **The machine's hold stays the authority** (`Queue & History` P5a): a
    /// held full range whose hold names another job is not this pair's.
    fn the_hold_agrees(&self, job: &smoozy_queue_history::readings::AJobAsTheMachineReadsIt) -> bool {
        job.job != self.range
            || job
                .the_hold
                .as_ref()
                .and_then(|hold| hold.held_behind.as_ref())
                .is_none_or(|behind| *behind == self.check)
    }

    /// The row's check line, before its state.
    pub(crate) fn the_checks_line(&self) -> APart {
        APart::said(FIRST_FRAME_CHECK_ON_THE_ROW, self.first_frame.to_string(), "")
    }

    /// The box's heading: *First-frame check · Frame 1*.
    fn the_heading(&self) -> APart {
        APart::said(FIRST_FRAME_CHECK, self.first_frame.to_string(), "")
    }
}

/// **`Queue & History` P5b: the full range's frames exactly as the user typed
/// them** — `typed` is the range job's own text (record version 51), *1,2-4*
/// — **and the submitted numbers where it holds none.** The eighteenth notes,
/// piece 7, note 8.
pub(crate) fn the_full_ranges_frames(
    typed: Option<String>,
    submitted: &smoozy_render_settings::frames::FrameForm,
) -> String {
    typed.unwrap_or_else(|| crate::surface::render_settings::written_out(submitted))
}

/// **Where a job's row draws its pair**, if it is a part of one.
#[derive(Debug, Clone, PartialEq, Eq)]
enum WhereThePairIsDrawn {
    NotAPair,
    /// **The full range, while its check is in the same line**: the pair is
    /// the check's row, so this job is not a row of its own.
    OnTheChecksRow,
    Here(ThePair, Vec<APartOfThePair>),
}

/// **A first-frame pair is one row, on the part in line** — the check's while
/// the check is in its server's line, so the row sits and drags where the
/// check sits; then the full range's, held with no grip until *Render all*
/// puts it in line (`Design/Queue & History § A first-frame check and its
/// job`). The row names both parts, each with its state. **A full range whose
/// check's record is gone is no pair**, there being nothing to say of the
/// check.
fn where_the_pair_is_drawn(
    store: &crate::record::job_records::JobRecords<'_>,
    job: &smoozy_queue_history::readings::AJobAsTheMachineReadsIt,
    in_this_line: &std::collections::HashMap<String, &smoozy_queue_history::readings::AJobAsTheMachineReadsIt>,
    the_frame_in_hand: &dyn Fn(&smoozy_queue_history::readings::AJobAsTheMachineReadsIt) -> Option<i64>,
) -> WhereThePairIsDrawn {
    use smoozy_queue_history::line::QueueState;
    let Some(pair) = ThePair::of(store, &job.job).filter(|pair| pair.the_hold_agrees(job)) else {
        return WhereThePairIsDrawn::NotAPair;
    };
    let the_ranges_line = |range: &smoozy_queue_history::readings::AJobAsTheMachineReadsIt| {
        let (state, dot) = match range.state {
            QueueState::Rendering { .. } => match the_frame_in_hand(range) {
                Some(frame) => (APart::said(PART_AT_FRAME, frame.to_string(), ""), "now"),
                None => (APart::words(PART_RENDERING), "now"),
            },
            _ => (APart::words(PART_NOT_STARTED), ""),
        };
        APartOfThePair { what: APart::said(FULL_RANGE_ON_THE_ROW, pair.the_frames.clone(), ""), state, dot }
    };
    if pair.is_the_check(&job.job) {
        let (state, dot) = match job.state {
            QueueState::Rendering { .. } => (APart::words(PART_RENDERING), "now"),
            _ => (APart::words(PART_NOT_STARTED), ""),
        };
        let mut parts = vec![APartOfThePair { what: pair.the_checks_line(), state, dot }];
        if let Some(range) = in_this_line.get(&pair.range.as_text()).filter(|range| pair.the_hold_agrees(range)) {
            parts.push(the_ranges_line(range));
        }
        return WhereThePairIsDrawn::Here(pair, parts);
    }
    if in_this_line.contains_key(&pair.check.as_text()) {
        return WhereThePairIsDrawn::OnTheChecksRow;
    }
    let Some(check) = store.read(&pair.check).ok().flatten() else {
        return WhereThePairIsDrawn::NotAPair;
    };
    let (state, dot) = a_check_that_ended(check.the_ending_learned());
    let parts = vec![APartOfThePair { what: pair.the_checks_line(), state, dot }, the_ranges_line(job)];
    WhereThePairIsDrawn::Here(pair, parts)
}

/// **A check that has left the line, by the ending its record learned** —
/// *rendered* with the green dot where it finished, and otherwise its own
/// ending, in the words the catch-up count gives each (the rows have no
/// word of their own for an unfinished check; History's row likewise reads
/// the check's own ending where it did not finish). **Nothing where no ending
/// was learned.**
pub(crate) fn a_check_that_ended(ending: Option<smoozy_queue_history::ending::Ending>) -> (APart, &'static str) {
    use smoozy_queue_history::ending::Ending;
    match ending {
        Some(Ending::Finished) => (APart::words(PART_RENDERED), "done"),
        Some(Ending::Failed) => (APart::words(ENDED_FAILED), ""),
        Some(Ending::Stopped) => (APart::words(ENDED_STOPPED), ""),
        Some(Ending::GivenUp) => (APart::words(ENDED_GIVEN_UP), ""),
        None => (APart::default(), ""),
    }
}

/// **The check's picture**, as its own entry draws it down the side — every
/// copy held, every View Layer — following, as every queue picture does.
fn the_checks_picture(
    connection: &rusqlite::Connection,
    check: &smoozy_state::identities::Identity,
) -> Option<ThePreview> {
    let first = crate::record::job_records::JobRecords::over(connection).read(check).ok().flatten()?;
    let beside = crate::surface::the_history::BesideTheRecord::read(connection, &first.job);
    let mut preview = crate::surface::the_history::the_entry_for(&first, &beside, false, true).entry.preview?;
    if !preview.nothing_to_show {
        preview.which = LATEST;
        preview.follows = true;
    }
    Some(preview)
}

/// **The check on its pair's queue entry** (`Design/Queue & History § A
/// first-frame check and its job`), by which part the entry is:
///
/// - **the check's own**, while it is in line: the box above the status line
///   saying what follows it and what stopping it does — drawn the same while
///   it waits in line, which is just as true then;
/// - **the full range's, held**: the picture and heading in the question box;
/// - **the full range's, in line after *Render all***: the picture, *Rendered
///   in 2m. You chose **Render all**.*, and the check's own *Blender output*.
///
/// **A full range held for the driver is not waiting on the user** (`Queue &
/// History` H12) and gets the last reading: the user chose *Render all*, and
/// the driver hold came after.
fn the_check_in_the_queue(
    connection: &rusqlite::Connection,
    pair: &ThePair,
    job: &smoozy_state::identities::Identity,
    state: &smoozy_queue_history::line::QueueState,
    for_the_driver: bool,
) -> TheCheckOnTheEntry {
    use smoozy_queue_history::line::QueueState;
    let mut the_check = TheCheckOnTheEntry {
        job: pair.check.as_text(),
        heading: pair.the_heading(),
        ..TheCheckOnTheEntry::default()
    };
    if pair.is_the_check(job) {
        the_check.said = vec![APart::said(FRAMES_LEAD, pair.the_frames.clone(), START_ONLY_AFTER_YOU_REVIEW)];
        return the_check;
    }
    the_check.picture = the_checks_picture(connection, &pair.check);
    if matches!(state, QueueState::Held { .. }) && !for_the_driver {
        the_check.in_the_question = true;
        return the_check;
    }
    let first = crate::record::job_records::JobRecords::over(connection).read(&pair.check).ok().flatten();
    the_check.rendered_for = first
        .as_ref()
        .and_then(|first| first.the_four_moments().exactly_one_duration())
        .map(crate::surface::the_history::for_how_long);
    the_check.you_chose_render_all = true;
    the_check.blender_output = crate::surface::the_history::BesideTheRecord::read(connection, &pair.check).the_words_are_kept();
    the_check
}

/// **The question in a pair's box** — *Frame 1001 downloaded. Render frames
/// 1001–1240?*, *downloaded* only where it was (`Download` R4) — and, where
/// the check did not finish, no word that it rendered: *Review frame 1001.
/// Render frames 1001–1240?*
fn the_question_of_a_pair(pair: &ThePair, the_check_finished: bool, downloaded: bool) -> Vec<APart> {
    if !the_check_finished {
        return vec![
            APart::said(REVIEW_FRAME, pair.first_frame.to_string(), "."),
            APart::said(REVIEW_IT_AND_CHOOSE, pair.the_frames.clone(), THE_QUESTION_MARK),
        ];
    }
    let rendered = if downloaded { IS_RENDERED_AND_DOWNLOADED_REVIEW_IT } else { IS_RENDERED_REVIEW_IT };
    vec![
        APart::said(THE_FIRST_FRAME_LEAD, pair.first_frame.to_string(), rendered),
        APart::said(REVIEW_IT_AND_CHOOSE, pair.the_frames.clone(), THE_QUESTION_MARK),
    ]
}

/// **The check on its pair's History entry** — the same box (`Design/Queue &
/// History § A first-frame check and its job`, *In History, a pair is one row
/// too*), in History's three readings. `the_range` is how the full range
/// stands.
pub(crate) fn the_check_in_history(
    connection: &rusqlite::Connection,
    pair: &ThePair,
    the_range: TheRangeInHistory,
) -> TheCheckOnTheEntry {
    let mut the_check = TheCheckOnTheEntry {
        job: pair.check.as_text(),
        heading: pair.the_heading(),
        ..TheCheckOnTheEntry::default()
    };
    match the_range {
        TheRangeInHistory::Ended => {
            let beside = crate::surface::the_history::BesideTheRecord::read(connection, &pair.check);
            let first = crate::record::job_records::JobRecords::over(connection).read(&pair.check).ok().flatten();
            the_check.picture = first.as_ref().and_then(|first| {
                crate::surface::the_history::the_entry_for(first, &beside, false, true).entry.preview
            });
            let moments = first.as_ref().map(|first| first.the_four_moments());
            the_check.rendered_for = moments
                .as_ref()
                .and_then(|moments| moments.exactly_one_duration())
                .map(crate::surface::the_history::for_how_long);
            the_check.when = moments
                .as_ref()
                .and_then(|moments| moments.ended.as_ref())
                .map(smoozy_state::observed::Reading::taken_at)
                .and_then(|at| at.duration_since(std::time::UNIX_EPOCH).ok())
                .map(|since| since.as_secs().to_string());
            the_check.you_chose_render_all = true;
            the_check.blender_output = beside.the_words_are_kept();
        }
        TheRangeInHistory::InQueue => {
            the_check.said = vec![APart::said(FRAMES_LEAD, pair.the_frames.clone(), ARE_IN_QUEUE)];
            the_check.show_in_queue = Some(pair.range.as_text());
        }
        TheRangeInHistory::NeverRendered => {
            the_check.said = vec![APart::words(THE_FULL_RANGE_WAS_NOT_RENDERED)];
        }
    }
    the_check
}

/// **How a pair's full range stands, as History reads it.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TheRangeInHistory {
    /// It is in History: the pair is one row, the full range's.
    Ended,
    /// Its record stands and History does not draw it — it waits or renders.
    InQueue,
    /// It left no record: *Cancel*, or dropped.
    NeverRendered,
}

fn the_saved_first_frame_check_of(
    connection: &rusqlite::Connection,
    job: &smoozy_state::identities::Identity,
    hold: Option<&smoozy_queue_history::readings::WhatTheHoldSays>,
) -> Option<TheCheckAsTheEntryKnowsIt> {
    // The live hold remains authoritative, including when it contradicts
    // descriptive metadata left by this desktop.
    let mut check = the_first_frame_check_of(job, hold)?;
    if let Some(purpose) = crate::record::job_records::JobRecords::over(connection).first_frame_purpose(job).ok().flatten()
        .filter(|purpose| purpose.check_job == check.the_first_job.as_text() && purpose.range_job == job.as_text()) {
        check.the_count = Some((purpose.first_frame, u32::try_from(purpose.submitted_range.count()).unwrap_or(u32::MAX)));
    }
    Some(check)
}

/// **What the entry knows of a first-frame check's held job**: the job it
/// waits behind, and — where this run of Smoozy sent it — the first frame and
/// how many frames the rest holds.
#[derive(Debug, Clone, PartialEq, Eq)]
struct TheCheckAsTheEntryKnowsIt {
    the_first_job: smoozy_state::identities::Identity,
    the_count: Option<(i64, u32)>,
}

/// The machine's hold names the check and remains the approval authority.
/// Legacy submissions may have only the run's remembered frame/count;
/// `the_saved_first_frame_check_of` adds the saved descriptive values for new
/// pairs, only where they agree with that dependency. No frame/count is
/// invented for older records.
fn the_first_frame_check_of(
    job: &smoozy_state::identities::Identity,
    the_hold: Option<&smoozy_queue_history::readings::WhatTheHoldSays>,
) -> Option<TheCheckAsTheEntryKnowsIt> {
    let this_run = crate::submitting::the_first_frame_check_behind(job);
    let the_count = this_run
        .as_ref()
        .map(|check| (check.the_first_frame, check.frames_held));
    let Some(the_hold) = the_hold else {
        return this_run.map(|check| TheCheckAsTheEntryKnowsIt {
            the_first_job: check.the_first_job,
            the_count,
        });
    };
    let the_first_job = the_hold.held_behind.clone()?;
    if the_hold.reason != smoozy_rendering::submission::the_hold_instruction(job, &the_first_job).reason {
        return None;
    }
    Some(TheCheckAsTheEntryKnowsIt { the_first_job, the_count })
}

/// **The act of Smoozy's own in hand for this job**, where one is.
fn the_act_in_hand(job: &smoozy_state::identities::Identity) -> Option<TheQueuesOwnAct> {
    match where_the_act_stands(&job.as_text()) {
        Some(WhereTheActStands::InHand(act)) => Some(act),
        _ => None,
    }
}

/// **What the entry's status line says of where Smoozy's own act stands** —
/// the words beside each of the queue's three acts' endings (step 10c's S3).
/// **The machine's own words are a value beside the sentence**, never joined
/// into it (X7).
pub(crate) fn what_the_entry_says_of(stands: &WhereTheActStands) -> (Vec<APart>, &'static str) {
    let with_the_machines_words = |sentence: &'static str, words: &Option<String>| {
        let mut said = vec![APart::words(sentence)];
        if let Some(words) = words {
            said.push(APart::value(words.clone()));
        }
        said
    };
    match stands {
        WhereTheActStands::InHand(TheQueuesOwnAct::TheStop) => (vec![APart::words(STOPPING)], "busy"),
        WhereTheActStands::InHand(_) => (vec![APart::words(CANCELLING)], ""),
        WhereTheActStands::NotReached(_) => (vec![APart::words(SMOOZY_COULD_NOT_REACH_THE_SERVER)], ""),
        WhereTheActStands::NotTaken(_, words) => (with_the_machines_words(THE_SERVER_DID_NOT_ACCEPT_IT, words), ""),
        WhereTheActStands::TheWaiterGaveUp(TheQueuesOwnAct::TheStop) => (vec![APart::words(THE_STOP_WAS_SENT)], ""),
        WhereTheActStands::TheWaiterGaveUp(_) => (vec![APart::words(THE_CANCEL_WAS_SENT)], ""),
        WhereTheActStands::TheStopFailed(words) => (with_the_machines_words(THE_RENDER_WAS_NOT_STOPPED, words), ""),
        WhereTheActStands::NotCancelled(words) => (with_the_machines_words(THE_JOB_WAS_NOT_CANCELLED, words), ""),
        WhereTheActStands::TheClearWasRefused => (vec![APart::words(THE_WAITING_JOBS_COULD_NOT_BE_CANCELLED)], ""),
    }
}

/// **What an entry says while its server is not updating** —
/// `Design/Queue & History § The same holds while the server is not updating`
/// (added 2026-09-30, E0-5), under `Rendering` K10 and `Queue & History` C7.
///
/// **Written for one case and now meeting two.** S3 wrote the entry's word
/// for Smoozy's own act for a server whose reading is live, and let the
/// reading's *Not updating* (`Queue & History` C7) speak alone otherwise —
/// **on the belief that an act and a quiet line do not meet**. They do: step
/// 10c's eleventh pass sent a stop and the line was cut half a second later.
/// **The moment the record and the world parted company is the break's
/// declaration**: from then on the waiter was still asking, the machine went
/// on to stop the render, and the entry said only *Not updating*, as if
/// nothing had been sent.
///
/// - **The reading's *Not updating* always comes first** (`Queue & History`
///   C7), so an entry last seen rendering never reads as still running.
/// - **A stop or cancel in hand is said beside it** — `Queue & History`
///   L12a's *Stopping*, `Queue & History` L2's *Cancelling* — **and once its
///   waiter has given up, the waiter's own sentence** (`Rendering` K10 for a
///   stop). Both are Smoozy's statements about its own act, and stay true
///   however long the line is quiet.
/// - **Nothing else is.** An act that ended any other way — not reached, not
///   taken, the stop failed, not cancelled, the clear refused — was answered
///   before the quiet, and the design keeps only *a stop sent before the line
///   went quiet*. **Ruled at this site.**
pub(crate) fn what_a_not_updating_entry_says(stands: Option<&WhereTheActStands>) -> Vec<APart> {
    let mut said = vec![APart::words(NOT_UPDATING)];
    if let Some(stands @ (WhereTheActStands::InHand(_) | WhereTheActStands::TheWaiterGaveUp(_))) = stands {
        said.push(APart::words(A_DOT));
        said.extend(what_the_entry_says_of(stands).0);
    }
    said
}

/// **The local record of a job the reading holds, where it is this server's**
/// — the one join both lists and the work column make (`Queue & History` C10:
/// two presentations of two records with one identity).
fn the_record_of(
    store: &crate::record::job_records::JobRecords<'_>,
    job: &smoozy_state::identities::Identity,
    server: &str,
) -> Option<smoozy_queue_history::record::LocalJobRecord> {
    store
        .read(job)
        .ok()
        .flatten()
        .filter(|held| held.server.local_server.as_text() == server)
}

/// **`Queue & History` C11: when the job ends it leaves the queue — and the
/// ending is the one Smoozy learns, from watching or from the note.**
///
/// **Written for one case and now meeting two.** It was written for the
/// reading that says so itself: a queue reading taken after the job ended
/// carries it as ended. **It now also meets the reading that is older than the
/// ending.** The queue reading repeats only while a surface showing that
/// server's work is on screen (`Desktop app and delivery` P10d), while the
/// read-forward runs on every pass whatever is on screen — so Smoozy learns an
/// ending, and writes it on the job's record, long before the next reading
/// arrives. Step 10c's ninth pass kept a finished job as *Rendering*, with
/// *Stop*, eleven minutes after History had its ending. **An ending, once
/// learned, is never un-learned** (`LocalJobRecord::the_fill_in_from_the_note`
/// writes an ending and clears none), so an older reading cannot bring the job
/// back.
///
/// **The other way round is not taken, and that is deliberate**: nothing here
/// ever puts a job in the queue, or makes it *rendering*, from the local record
/// alone. The reading is the machine's live state (`Queue & History` C10) and
/// only it says what is running.
pub(crate) fn it_has_left_the_queue(
    state: &smoozy_queue_history::line::QueueState,
    held: Option<&smoozy_queue_history::record::LocalJobRecord>,
) -> bool {
    matches!(state, smoozy_queue_history::line::QueueState::Ended { .. })
        || held.is_some_and(|held| held.the_ending_learned().is_some())
}

/// **What one server is doing, as its work column says it** —
/// `Design/Servers § A row`: *"Rendering and the job's name with how many are
/// waiting, or Idle, or Reconnecting… … Where Smoozy cannot say what the
/// server is doing, the column takes the dash"*. *Reconnecting…* and
/// *Restarting* are the connection's and not the queue's, so they are not
/// here; `src/surface/servers.rs` draws them first.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatAServerIsDoing {
    /// **A job renders in the reading**: the first one's name — the label as
    /// the local record holds it, or as the machine's note carries it for work
    /// Smoozy is not tracking — and how many other jobs the reading still
    /// holds.
    Rendering { name: String, waiting: u32 },
    /// **The reading holds jobs live and none of them rendering** —
    /// *Waiting* and how many. `Design/Servers § A row`, added 2026-09-28:
    /// *"Where the reading holds jobs waiting and none rendering, the column
    /// says Waiting and how many — Waiting, 2 jobs — in the same shape as
    /// Rendering, with the count where the job's name goes."* **Held jobs
    /// count as the Queue counts them**, as waiting — a server whose every job
    /// waits on the user, such as a first-frame check, is this case. So is a
    /// reading older than an ending Smoozy has learned, until the next pass
    /// reads the job that started since: the reading says the rest wait, and
    /// nothing may claim one is rendering from the local record alone.
    Waiting { how_many: u32 },
    /// **The reading holds nothing live** — *Idle*.
    NothingIsLive,
    /// **Smoozy cannot say**, and the column takes the dash: the reading is
    /// not updating (`Queue & History` C7). *(Until 2026-09-28 this also held
    /// live jobs with none rendering, for which Design then had no word; that
    /// case is [`WhatAServerIsDoing::Waiting`] now.)*
    ItCannotBeSaid,
}

/// **Every server whose queue reading this run holds, and what it is doing.**
/// A server with no reading has no entry: nothing is known of its work.
pub type WhatTheServersAreDoing = std::collections::BTreeMap<String, WhatAServerIsDoing>;

/// **The work column's reading, composed from the same two things the queue
/// is**: the period-held reading (`Queue & History` C6) less every job that
/// has left the queue ([`it_has_left_the_queue`]). **So the list, a server's
/// own page and the Queue screen cannot disagree about what is running**, and
/// a job whose ending is learned leaves all three at once.
pub fn what_the_servers_are_doing(
    holds: &crate::servers::the_machines_work::WhatThisRunHolds,
    record: Option<&crate::record::LocalRecord>,
) -> WhatTheServersAreDoing {
    use smoozy_queue_history::line::QueueState;
    let mut doing = WhatTheServersAreDoing::new();
    let store = record.map(|record| crate::record::job_records::JobRecords::over(record.connection()));
    for (server, period) in holds.every_server() {
        let Some(reading) = period.reading.as_ref() else {
            continue;
        };
        if period.the_break_is_declared {
            doing.insert(server, WhatAServerIsDoing::ItCannotBeSaid);
            continue;
        }
        let state = reading.value();
        let mut live = Vec::new();
        for job in &state.jobs {
            let held = store.as_ref().and_then(|store| the_record_of(store, &job.job, &server));
            if !it_has_left_the_queue(&job.state, held.as_ref()) {
                live.push((job, held));
            }
        }
        // **A first-frame pair counts once, as the Queue counts it**: its full
        // range is not counted while its check is live beside it (the
        // nineteenth notes, note 7).
        if let Some(store) = store.as_ref() {
            let checks_live: std::collections::HashSet<String> =
                live.iter().filter(|(_, held)| held.is_some()).map(|(job, _)| job.job.as_text()).collect();
            live.retain(|(job, held)| {
                held.is_none()
                    || ThePair::of(store, &job.job)
                        .filter(|pair| pair.the_hold_agrees(job))
                        .is_none_or(|pair| pair.is_the_check(&job.job) || !checks_live.contains(&pair.check.as_text()))
            });
        }
        let rendering = live
            .iter()
            .find(|(job, _)| matches!(job.state, QueueState::Rendering { .. }));
        let said = match rendering {
            Some((job, held)) => WhatAServerIsDoing::Rendering {
                // **Called what the queue calls it** ([`what_the_job_is_called`]);
                // the machine's note, which carries the label and no file,
                // where this computer holds no record of the job.
                name: held
                    .as_ref()
                    .map(what_the_job_is_called)
                    .or_else(|| {
                        state
                            .notes
                            .iter()
                            .find(|(noted, _)| *noted == job.job)
                            .map(|(_, note)| what_a_job_is_called(Some(&note.label), None))
                    })
                    .unwrap_or_default(),
                waiting: u32::try_from(live.len() - 1).unwrap_or(u32::MAX),
            },
            None if live.is_empty() => WhatAServerIsDoing::NothingIsLive,
            // **`Design/Servers § A row`: *Waiting* and how many**, every live
            // job counted, held ones as the Queue counts them.
            None => WhatAServerIsDoing::Waiting {
                how_many: u32::try_from(live.len()).unwrap_or(u32::MAX),
            },
        };
        doing.insert(server, said);
    }
    doing
}

/// A moment as whole seconds since 1970, the form every screen hands its page.
fn in_seconds(at: std::time::SystemTime) -> String {
    at.duration_since(std::time::UNIX_EPOCH)
        .map(|since| since.as_secs().to_string())
        .unwrap_or_default()
}

/// **The catch-up notice, composed at draw time over every standing notice
/// record** (`Queue & History` C2a), each composed by the read forward from
/// exactly the events it applied (`Queue & History` C1) and **read from the
/// local record** (`Queue & History` C2) — so it is drawn on a fresh launch
/// before anything is connected.
///
/// **What it counts is the endings** — finished, failed, stopped, given up —
/// each kind once, in that order, and only kinds above nought (Design, *The
/// catch-up notice*, the Creator's ninth notes, note 3). **An ending counts
/// only where its job's History entry stands**, since pressing the count
/// opens History, *"where every one of those jobs is"*; a job the user has
/// since deleted from History is counted nowhere. **A re-run is not counted
/// and not named**: its entry carries the mark.
///
/// **What it names is a job dropped before it rendered**, one line each: a
/// held job the machine dropped (`Queue & History` H11), and a waiting job
/// dropped because its Blender was removed, with the version the machine's
/// drop words carried (`Queue & History` L22). The name is the one the notice
/// record kept beside the line, since the local record is gone, and **that
/// line is the only place the job is named**, so its *Dismiss* asks first.
/// No server is named on either (Design).
fn the_notice_from(
    connection: &rusqlite::Connection,
    store: &crate::record::job_records::JobRecords<'_>,
) -> Option<TheCatchUpNotice> {
    use smoozy_queue_history::catch_up::ANoticeEntry;
    use smoozy_queue_history::ending::Ending;
    let in_designs_order = [Ending::Finished, Ending::Failed, Ending::Stopped, Ending::GivenUp];
    let mut how_many = [0u32; 4];
    let mut notice = TheCatchUpNotice::default();
    for account in crate::record::catch_up_notices::every_standing_notice(connection).unwrap_or_default() {
        for entry in &account.entries {
            match entry {
                ANoticeEntry::Ended { job, ending, .. } => {
                    if store.read(job).ok().flatten().is_none() {
                        continue;
                    }
                    // **A first-frame pair counts once**: once its full range
                    // has ended too, the pair is one History entry, the full
                    // range's, and the check's own ending is not counted
                    // beside it (the nineteenth notes, note 7).
                    if ThePair::of(store, job).is_some_and(|pair| {
                        pair.is_the_check(job)
                            && store.read(&pair.range).ok().flatten().is_some_and(|range| range.the_ending_learned().is_some())
                    }) {
                        continue;
                    }
                    if let Some(at) = in_designs_order.iter().position(|kind| kind == ending) {
                        how_many[at] += 1;
                    }
                }
                ANoticeEntry::HeldJobDropped { job, name, .. } => notice.dropped.push(ADroppedJob {
                    job: job.as_text(),
                    name: name.clone(),
                    what: APart::words(WAS_NOT_STARTED_THE_FIRST_FRAME_WAS_NEVER_RENDERED),
                }),
                ANoticeEntry::WaitingJobDroppedForARemovedBlender {
                    job,
                    a_bounded_rendering_of_the_label: name,
                    the_version,
                    ..
                } => notice.dropped.push(ADroppedJob {
                    job: job.as_text(),
                    name: name.clone(),
                    what: if the_version.is_empty() {
                        APart::words(WAS_NOT_STARTED_ITS_BLENDER_WAS_REMOVED)
                    } else {
                        APart::said(WAS_NOT_STARTED_ITS_BLENDER, the_version.clone(), WAS_REMOVED)
                    },
                }),
                ANoticeEntry::InterruptedAndReRunFrom { .. } => {}
            }
        }
    }
    notice.counted = in_designs_order
        .into_iter()
        .zip(how_many)
        .filter(|(_, many)| *many > 0)
        .collect();
    (!notice.counted.is_empty() || !notice.dropped.is_empty()).then_some(notice)
}

/// **`Desktop app and delivery` H32: the entry's picture, as bytes through a
/// typed application service** — the page makes a `blob:` address of them and
/// revokes it when it lets the picture go. The job and frame are the ones the
/// entry was drawn with ([`ThePreview::picture`]). The 38's piece 3.
#[tauri::command]
///
/// **`place` is a View Layer's place in the frame's set**, as
/// [`ALayerHeld::place`] handed it; absent asks for the frame's main copy
/// (P10), which is every caller's before the Creator's seventh notes, note 4.
pub fn the_preview_picture<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    job: String,
    frame: i64,
    place: Option<i64>,
) -> tauri::ipc::Response {
    use tauri::Manager;
    let bytes = app
        .try_state::<Mutex<crate::start::Started>>()
        .and_then(|started| {
            let started = started.lock().ok()?;
            let record = started.record.as_ref()?;
            Some(crate::the_previews::the_pictures_bytes(record.connection(), &job, frame, place))
        })
        .unwrap_or_default();
    tauri::ipc::Response::new(bytes)
}

/// **The queue, drawn again from what the run now holds** — after step 7, after
/// a P10d pass that moved something, and at a declared break. **It draws the
/// entry the screen last asked for**, so a pass never moves the user's choice.
/// **The servers this run holds a connected reading for** — a period with a
/// queue reading whose break is not declared. The one rule *Submit the
/// remaining scenes* (`Rendering` S11) and History's *Files on the server* are
/// offered by; a queue entry reads the same thing as its reading updating.
pub(crate) fn the_servers_connected(
    holds: &crate::servers::the_machines_work::WhatThisRunHolds,
) -> std::collections::BTreeSet<String> {
    holds
        .every_server()
        .into_iter()
        .filter(|(_, period)| period.reading.is_some() && !period.the_break_is_declared)
        .map(|(server, _)| server)
        .collect()
}

/// **Whether the servers connected differ from the last time the queue was
/// drawn**, and they become the last. History reads them for *Files on the
/// server*, and nothing else draws History again when a connection breaks, so
/// the queue's own redraw tells it — only when they have changed, since the
/// queue is drawn again far more often than that.
fn the_servers_connected_have_changed(now: &std::collections::BTreeSet<String>) -> bool {
    static LAST: std::sync::OnceLock<Mutex<Option<std::collections::BTreeSet<String>>>> = std::sync::OnceLock::new();
    let Ok(mut last) = LAST.get_or_init(Default::default).lock() else {
        return false;
    };
    if last.as_ref() == Some(now) {
        return false;
    }
    *last = Some(now.clone());
    true
}

pub fn the_queue_is_told_again<R: tauri::Runtime>(app: &tauri::AppHandle<R>) {
    use tauri::Manager;
    let Some(periods) = app.try_state::<std::sync::Arc<crate::servers::a_period::ThePeriodsThisRun>>() else {
        return;
    };
    let Some(queue) = app.try_state::<std::sync::Arc<TheOneQueue>>() else {
        return;
    };
    let Some(started) = app.try_state::<Mutex<Started>>() else {
        return;
    };
    let holds = periods.the_machines_work();
    let held = {
        let Ok(started) = started.lock() else {
            return;
        };
        what_is_held(holds, started.record.as_ref(), holds.the_chosen_job().as_deref())
    };
    the_queue_is_posted(&queue, the_queue_as_asked(held, None, None));
    // **A connection that came or went moves History's *Files on the
    // server*** — step 10c's *Files on the server* follow-up. **And so does
    // an ending a reading carries while it is owed to a full store**, which
    // leaves the queue here and is drawn in History until the landing
    // (`Queue & History` C11, `Rented-server runtime` S14a) — nothing durable
    // moved, so nothing else draws History again for it. Step 12's SC4
    // follow-up.
    let connected_changed = the_servers_connected_have_changed(&the_servers_connected(holds));
    let owed_changed = the_endings_owed_have_changed(&crate::surface::the_history::the_endings_owed_in(holds));
    if connected_changed || owed_changed {
        crate::surface::the_history::the_history_is_told_again(app);
    }
}

/// **Whether the endings the readings carry while they are owed differ from
/// the last time the queue was drawn**, and they become the last — the same
/// shape as [`the_servers_connected_have_changed`], for the same reason.
fn the_endings_owed_have_changed(now: &[crate::surface::the_history::AnEndingTheReadingCarries]) -> bool {
    static LAST: std::sync::OnceLock<Mutex<Option<Vec<crate::surface::the_history::AnEndingTheReadingCarries>>>> =
        std::sync::OnceLock::new();
    let Ok(mut last) = LAST.get_or_init(Default::default).lock() else {
        return false;
    };
    if last.as_deref() == Some(now) {
        return false;
    }
    *last = Some(now.to_vec());
    true
}

// ───────────────────────────────────────────────────────────────────────────
// A rendering job's words, live — `Queue & History` E8, piece 6c
// ───────────────────────────────────────────────────────────────────────────

/// **The kind a rendering job's words arrive on** — `web/the-queue.js` names
/// the same string.
pub const THE_RUNNING_WORDS: &str = "queue:the-running-words";

/// **The press on a rendering entry's *Blender output*: the words are opened
/// and the first read is taken at once**, on a thread of its own (P5), so the
/// panel does not wait for P10d's next pass; every pass after reads again
/// while the panel is open. **False where the job is not rendering on a
/// reading Smoozy holds**, and then nothing is opened.
pub(crate) fn the_running_words_are_opened<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    periods: &std::sync::Arc<crate::servers::a_period::ThePeriodsThisRun>,
    started: &Mutex<Started>,
    job: &str,
) -> bool {
    let holds = periods.the_machines_work();
    let Some(server) = holds.where_a_rendering_jobs_words_are(job) else {
        return false;
    };
    let name = started
        .lock()
        .ok()
        .and_then(|started| {
            let record = started.record.as_ref()?;
            let identity = smoozy_state::identities::Identity::read_back(
                smoozy_state::identities::IdentityKind::Job,
                job,
            )?;
            crate::record::job_records::JobRecords::over(record.connection())
                .read(&identity)
                .ok()
                .flatten()
        })
        .map(|record| what_the_job_is_called(&record))
        .unwrap_or_default();
    holds.the_words_are_opened(&server, job, &name);
    let (app, periods) = (app.clone(), periods.clone());
    let _ = std::thread::Builder::new()
        .name("smoozy a render's words".to_string())
        .spawn(move || {
            let Some(sessions) = periods.the_sessions_of(&server) else {
                return;
            };
            let mut reader = crate::servers::the_machines_work::TheSinksOverThisServersPoint { sessions };
            if let Some(arrived) =
                crate::servers::the_machines_work::the_words_are_read(periods.the_machines_work(), &server, &mut reader)
            {
                the_running_words_are_posted(&app, &arrived);
            }
        });
    true
}

/// **What arrived, posted**, if the queue is there to post on.
pub(crate) fn the_running_words_are_posted<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    arrived: &crate::servers::the_machines_work::TheWordsThatArrived,
) {
    use tauri::Manager;
    if let Some(queue) = app.try_state::<std::sync::Arc<TheOneQueue>>() {
        let arrived = arrived.clone();
        let _ = queue.read_then_post::<crate::servers::the_machines_work::TheWordsThatArrived, std::convert::Infallible>(
            move || Ok(arrived),
            |arrived| the_running_words_said(arrived),
        );
    }
}

/// **A render's words as they arrive, as an event.** Every string is on one of
/// the two routes (A6, X9).
///
/// - **Each stream on its own keys and under its own name** (`Desktop app and
///   delivery` M13, M13a) — the names are the panel's own.
/// - **The words are a value, whole and unedited** (M9), and only what is new
///   since the last arrival travels; the first arrival after opening says so,
///   so the page starts the panel over.
/// - **A stream that has sent nothing yet says so in M10a's own sentences** —
///   its outcome is not written while the render runs — never an empty box
///   (M10).
/// - **No line is lifted out of either stream** (M13b): there is no key for one.
pub(crate) fn the_running_words_said(arrived: &crate::servers::the_machines_work::TheWordsThatArrived) -> Event {
    use crate::presentation::the_machines_own_words::{
        Stream, SMOOZY_DOES_NOT_KNOW_YET, THE_OUTCOME_HAS_NOT_BEEN_RECORDED_YET,
    };
    let mut event = Event::about(About::TheRun, THE_RUNNING_WORDS)
        .saying("job", Said::Value(Verbatim::new(arrived.job.clone())))
        .saying("title", Said::Prose(Prose::new(BLENDER_OUTPUT)))
        .saying("subtitle", Said::Value(Verbatim::new(arrived.name.clone())))
        .saying(
            "note",
            Said::Prose(Prose::new(crate::surface::the_machines_own_words::THE_PANELS_NOTE)),
        )
        .saying("close", Said::Prose(Prose::new(crate::surface::the_machines_own_words::CLOSE)))
        .saying("copy", Said::Prose(Prose::new(crate::surface::the_machines_own_words::COPY)));
    if arrived.first {
        event = event.saying("fresh", Said::Value(Verbatim::new("yes")));
    }
    for (at, (stream, keys)) in [
        (Stream::TheOrdinaryOutput, ["theOutputTab", "theOutputSays", "theOutputSaid", "theOutputSaidNext", "theOutputMore"]),
        (Stream::TheComplaints, ["theErrorsTab", "theErrorsSays", "theErrorsSaid", "theErrorsSaidNext", "theErrorsMore"]),
    ]
    .into_iter()
    .enumerate()
    {
        event = event.saying(keys[0], Said::Prose(Prose::new(stream.the_name_of_the_stream())));
        if arrived.any_yet[at] {
            event = event.saying(keys[1], Said::Value(Verbatim::new("the-record")));
        } else {
            event = event
                .saying(keys[1], Said::Value(Verbatim::new("not-recorded-yet")))
                .saying(keys[2], Said::Prose(Prose::new(THE_OUTCOME_HAS_NOT_BEEN_RECORDED_YET)))
                .saying(keys[3], Said::Prose(Prose::new(SMOOZY_DOES_NOT_KNOW_YET)));
        }
        if !arrived.new[at].is_empty() {
            event = event.saying(keys[4], Said::Value(Verbatim::new(arrived.new[at].clone())));
        }
    }
    event
}

// ───────────────────────────────────────────────────────────────────────────
// The commands
// ───────────────────────────────────────────────────────────────────────────

// ── *Ignore*, remembered for the connection — must-do 19 ────────────────────

/// **The untracked-work banner the user declined, held beside the connected
/// period and keyed by the local server identifier of the server it was
/// about.**
///
/// **Must-do 19 left *Ignore* a hide on the page and routed the remembering
/// here.** Until step 10b piece 8 the page removed the element and the next
/// reading brought the banner straight back.
///
/// **It is held beside the reading rather than in the record, and that is
/// `Queue & History` C6 rather than a shortcut**: the untracked-work offer is
/// composed from the last-known queue reading, which is period-held — process
/// memory for the connected period, dropped whole at the period's end. **A
/// decline that outlived the period would silence an offer the next connection
/// made afresh**, and `State, identity and recovery` D4 keeps the desktop from
/// holding a durable copy of a machine state the machine still holds.
///
/// **So the decline lives no longer than that server's connected period, and
/// no longer than its offer**: it is dropped by [`a_connected_period_ended`]
/// when that server's period ends, and by [`the_declined_banner_is_applied`]
/// the moment a reading arrives that does not carry that server's offer. **The
/// second alone was not enough** (must-do 6): a disconnect and a reconnect with
/// no queue drawn between them left a stale decline to hide the fresh offer the
/// spec promises on the next connection — *"Declining is not final"*,
/// `Queue & History` C6.
///
/// **It is keyed by the server's identifier and never by its name**, which is
/// what the page draws: two servers may share a name, and a decline of one
/// must not hide the other's offer.
fn the_declined_untracked_banner() -> &'static Mutex<Option<String>> {
    static DECLINED: std::sync::OnceLock<Mutex<Option<String>>> = std::sync::OnceLock::new();
    DECLINED.get_or_init(|| Mutex::new(None))
}

/// **The user pressed *Ignore*.** Design, *Work Smoozy is not tracking*: it is
/// one offer per server, so the decline names the server it was about — by its
/// local server identifier.
pub fn the_untracked_banner_is_declined(server: &str) {
    if let Ok(mut declined) = the_declined_untracked_banner().lock() {
        *declined = Some(server.to_string());
    }
}

/// **Applied to every reading before it is drawn**: the declined server's offer
/// is not drawn, and a decline whose offer is gone is dropped.
///
/// **The decline is handed in rather than reached for**, so this is a function
/// over two values and the test below drives it without the run's own register
/// — which is the same split every command in this file already makes, and the
/// reason is the one the board records: a test that leans on something the
/// whole process shares is a test that fails when another test runs beside it.
fn the_decline_is_applied(held: &mut TheQueueOnScreen, declined: &mut Option<String>) {
    let offered = held.untracked.as_ref().map(|work| work.server_row.clone());
    match (&*declined, offered) {
        (Some(was), Some(now)) if *was == now => {
            held.untracked = None;
            // **`Queue & History` A4: with the banner gone, the waiting entry
            // behind that work says why it waits.** The 38's piece 15.
            if let (Some((server, how_many)), Some(entry)) =
                (&held.the_entry_waits_behind_untracked_work, held.entry.as_mut())
            {
                if server == was && entry.status.is_empty() {
                    entry.status = vec![APart::said(
                        THE_SERVER_IS_BUSY_WITH,
                        how_many.to_string(),
                        if *how_many == 1 { JOB_SMOOZY_IS_NOT_TRACKING } else { JOBS_SMOOZY_IS_NOT_TRACKING },
                    )];
                }
            }
        }
        // **The offer that was declined is gone**, so the decline has nothing
        // left to be about. A fresh offer — a different server, or the same
        // server after a period ended and another began — is drawn.
        _ => *declined = None,
    }
}

/// The same, over the run's own register.
fn the_declined_banner_is_applied(held: &mut TheQueueOnScreen) {
    if let Ok(mut declined) = the_declined_untracked_banner().lock() {
        the_decline_is_applied(held, &mut declined);
    }
}

/// **A server's connected period ended, so a decline made during it goes with
/// it** — must-do 6, and `Servers` S11: every period-held thing ends the moment
/// the period ends. A decline about another server stands.
///
/// **Its caller is the period's own ending and it is not in this file.** The
/// one place every period ends is `src/servers/a_period.rs`, beside
/// `ThePeriodsThisRun::a_period_has_ended`, and it is called there — landed by
/// the other Subject of step 10b round 1 on the runner's word (R5).
pub fn a_connected_period_ended(server: &str) {
    if let Ok(mut declined) = the_declined_untracked_banner().lock() {
        the_period_ending_drops_the_decline(&mut declined, server);
    }
}

/// The same, over a decline handed in, so a test can drive it.
fn the_period_ending_drops_the_decline(declined: &mut Option<String>, server: &str) {
    if declined.as_deref() == Some(server) {
        *declined = None;
    }
}

/// **The page asks Smoozy to stop offering to track one server's work.**
/// `server` is the local server identifier the reading carried, and never the
/// name the banner draws (must-do 6).
///
/// **It answers with the reading rather than with a sentence**, for the same
/// reason a connect that began says nothing: the user reads the screen they
/// changed. P6 keeps the telling on the one ordered queue.
#[tauri::command]
pub fn the_untracked_work_is_ignored(
    started: State<'_, Mutex<Started>>,
    queue: State<'_, std::sync::Arc<TheOneQueue>>,
    periods: State<'_, std::sync::Arc<crate::servers::a_period::ThePeriodsThisRun>>,
    server: String,
    chosen: Option<String>,
) {
    let Ok(started) = started.lock() else {
        return;
    };
    the_untracked_banner_is_declined(&server);
    let held = what_is_held(periods.the_machines_work(), started.record.as_ref(), chosen.as_deref());
    the_queue_is_posted(&queue, the_queue_as_asked(held, None, None));
}

/// **What the page's ask comes to**: the reading, with a box in it where the
/// ask raises one.
///
/// **Design, *The catch-up notice*: a dropped job's line's *Dismiss* asks
/// first**, in one sentence naming what will not be said again, because that
/// line is the only place the job is named. `dropped` is the job its line
/// names, as the page hands it back; the box is about that job, and the
/// reading says so (`boxJob`) for the go-ahead to hand back in turn. A job
/// the notice no longer names raises nothing. *(Until the Creator's ninth
/// notes, note 3, *Dismiss* was the banner's, asked first only where the
/// banner named such a job, and otherwise acted on the press.)*
pub fn the_queue_as_asked(
    mut held: TheQueueOnScreen,
    asking: Option<TheQueueAsk>,
    dropped: Option<&str>,
) -> TheQueueOnScreen {
    the_declined_banner_is_applied(&mut held);
    if asking == Some(TheQueueAsk::Dismiss) {
        let line = held
            .notice
            .as_ref()
            .and_then(|notice| notice.dropped.iter().find(|line| Some(line.job.as_str()) == dropped))
            .cloned();
        if let Some(line) = line {
            held.asking = Some(the_dismiss_box(&line.name));
            held.the_dropped_job_asked_about = Some(line.job);
        }
    }
    held
}

/// **Where *Files on the server* opens: the server, and the folder on it** —
/// `Queue & History` N18's server-side shortcut, *show outputs*, read off the
/// local record and never the machine. Step 10c's *Files on the server* piece.
///
/// **The folder is `Queue & History` N10's**: the recorded reading's main
/// output — the same path the entry's *Writing to* shows, verbatim
/// (`Queue & History` Y8) — and the job's own folder
/// where no read has landed it yet, as the record holds that folder
/// (`Queue & History` Y8, `Job files and outputs` P21).
///
/// **What it answers instead, each in words the screen already has**: the
/// server's row gone is the entry's own line for it (`Queue & History` N17);
/// a folder chosen at render start and not made yet is the entry's own line
/// for that wait (`Job files and outputs` P21); anything the record will not read back is the record
/// not reading.
pub(crate) fn where_the_jobs_folder_is(
    connection: &rusqlite::Connection,
    job: &str,
) -> Result<(String, String), &'static str> {
    use smoozy_queue_history::record::TheJobOutputFolderTheRecordHolds;
    use smoozy_state::identities::{Identity, IdentityKind};
    let nothing = TheActOnAJob::FilesOnTheServer.answers();
    let job = Identity::read_back(IdentityKind::Job, job).ok_or(nothing)?;
    let held = crate::record::job_records::JobRecords::over(connection)
        .read(&job)
        .ok()
        .flatten()
        .ok_or(nothing)?;
    let server = held.server.local_server.as_text();
    let row = crate::record::server_rows::all(connection)
        .map_err(|_| nothing)?
        .into_iter()
        .find(|row| row.local_server_identifier == server)
        .ok_or(THE_SERVER_IS_GONE_AND_ITS_FILES_WITH_IT)?;
    if let Some(main) = held.main_output_from_the_read.clone() {
        return Ok((server, main));
    }
    if held.the_job_output_folder == TheJobOutputFolderTheRecordHolds::ChosenAtRenderStart {
        return Err(crate::surface::transfers::words::THE_FOLDER_IS_MADE_WHEN_THE_RENDER_STARTS);
    }
    let folder = match &held.the_job_output_folder {
        TheJobOutputFolderTheRecordHolds::It(sits) => Some(sits.path.clone()),
        _ => crate::servers::installation::the_layout_recorded_for(&row)
            .and_then(|layout| held.the_job_output_folder(&layout)),
    };
    folder.map(|folder| (server, folder)).ok_or(nothing)
}

/// **The job's record shows no frame saved** — the fact *show outputs* hands
/// Files with the folder, so a folder that is not there yet is said as such
/// (`Design/File Browser § When another module moves the browser`, added
/// 2026-10-07; the Creator's fourth notes, piece O). Read off the local record
/// and never the machine, and **true only where the record says it
/// outright**:
///
/// - **the ending**, where one is learned: the count of the frames the server
///   held when the job ended (`Queue & History` N6's figure and every frame
///   before it) is nought, which is an ending short of a clean finish with no
///   figure — and where the frames cannot be counted, no clean finish and no
///   figure at all. *(Until the live pass on Vast, 2026-10-19, the count was of
///   the frames before the figure, so a figure on the first frame read as none
///   saved; that frame is on the server, so its folder is there.)*
/// - **and no frame in the preview set** (`Queue & History` P3): no
///   production of a finished frame has been read for the job.
///
/// A record that will not read, or a set that will not, says a frame may be
/// saved, so the words are never claimed on a guess.
pub(crate) fn the_record_shows_no_frame_saved(connection: &rusqlite::Connection, job: &str) -> bool {
    use smoozy_queue_history::ending::Ending;
    use smoozy_state::identities::{Identity, IdentityKind};
    let Some(job) = Identity::read_back(IdentityKind::Job, job) else {
        return false;
    };
    let Some(held) = crate::record::job_records::JobRecords::over(connection).read(&job).ok().flatten() else {
        return false;
    };
    let the_ending_says_one = match held.how_far_it_got() {
        Some((done, _)) => done > 0,
        None => {
            held.the_ending_learned() == Some(Ending::Finished)
                || held.what_the_note_carried.last_frame_figure.is_some()
                || held.what_the_note_carried.given_up_where_it_was.is_some()
        }
    };
    if the_ending_says_one {
        return false;
    }
    let in_the_set: bool = connection
        .query_row(
            &format!("SELECT count(*) > 0 FROM {} WHERE job = ?1", crate::record::previews::TABLE),
            [job.as_text()],
            |row| row.get(0),
        )
        .unwrap_or(true);
    !in_the_set
}

/// **The page is told to open Files on `server` at `folder`** — on the one
/// event a press that leads to another screen arrives on
/// ([`crate::a_new_job_from_an_old_one::A_NEW_JOB_OPENS`]), which
/// `web/a-new-job.js` turns into the same hand-over Set up's folder link
/// makes. The folder is the record's text, a value.
///
/// **Files is told first that *show outputs* is moving it**, so the answer it
/// gives the page carries the *Moved to …* line where the browser was on
/// another server (`Design/File Browser § When another module moves the
/// browser to a different server`). The page hands the folder back as the
/// bytes of this same text, which is how the two are matched. **And whether
/// the job's record shows no frame saved** ([`the_record_shows_no_frame_saved`]),
/// so a folder not there yet is said as such (piece O).
pub(crate) fn the_jobs_folder_is_told(queue: &TheOneQueue, server: &str, folder: &str, no_frame_is_saved: bool) {
    crate::surface::files::the_browser_is_moved_to(server, folder.as_bytes(), no_frame_is_saved);
    let event = Event::about(About::TheRun, crate::a_new_job_from_an_old_one::A_NEW_JOB_OPENS)
        .saying("goesTo", Said::Value(Verbatim::new("the-folder")))
        .saying("server", Said::Value(Verbatim::new(server.to_string())))
        .saying("folder", Said::Value(Verbatim::new(folder.to_string())));
    let _ = queue.read_then_post::<Event, std::convert::Infallible>(|| Ok(event.clone()), |event| event.clone());
}

/// **The page asks what the queue is doing.**
///
/// P6: a command is the page asking and an event is the application telling, so
/// this answers with nothing and the reading arrives on the queue.
///
/// **It is also how P10d learns the queue is on screen**: asking marks it so,
/// and the page's *put away* marks it not — the one condition P10d gives the
/// queue reading's repetition.
///
/// **`dropped` is the job a dropped job's line names**, handed back with that
/// line's *Dismiss* (the Creator's ninth notes, note 3) — **or, with
/// [`TheQueueAsk::ClearTheRow`], the act whose row saying *Not submitted.* the
/// cross clears** (`Queue & History` C10 as changed 2026-10-19). Nothing else
/// sends it.
#[tauri::command]
pub fn the_queue(
    started: State<'_, Mutex<Started>>,
    queue: State<'_, std::sync::Arc<TheOneQueue>>,
    periods: State<'_, std::sync::Arc<crate::servers::a_period::ThePeriodsThisRun>>,
    chosen: Option<String>,
    asking: Option<TheQueueAsk>,
    dropped: Option<String>,
) {
    let holds = periods.the_machines_work();
    if asking == Some(TheQueueAsk::PutAway) {
        holds.the_queue_is_put_away();
        return;
    }
    if asking == Some(TheQueueAsk::TheWordsArePutAway) {
        holds.the_words_are_put_away();
        return;
    }
    if asking == Some(TheQueueAsk::ClearTheRow) {
        if let Some(which) = dropped.as_deref().and_then(|which| which.parse::<u64>().ok()) {
            crate::submitting::a_not_submitted_row_is_cleared(which);
        }
    }
    holds.the_queue_is_shown(chosen.as_deref());
    let Ok(lent) = started.lock() else {
        return;
    };
    let held = what_is_held(holds, lent.record.as_ref(), chosen.as_deref());
    let mut says = the_queue_as_asked(held, asking, dropped.as_deref());
    // **The check's own *Blender output*: its kept words, read from the
    // record at the one moment it is pressed** (`Queue & History` E9), and
    // only for the check on the entry drawn — never a job the page names that
    // is not on screen. The nineteenth notes, note 7.
    if asking == Some(TheQueueAsk::TheChecksBlenderOutput) {
        says.words = the_checks_words(lent.record.as_ref(), &says, dropped.as_deref());
    }
    the_queue_is_posted(&queue, says);
}

/// **The kept words of the check on the entry drawn**, where `check` names it
/// and its *Blender output* is offered.
pub(crate) fn the_checks_words(
    records: Option<&crate::record::LocalRecord>,
    says: &TheQueueOnScreen,
    check: Option<&str>,
) -> Option<crate::surface::the_history::TheKeptWords> {
    let entry = says.entry.as_ref()?;
    let the_check = entry.the_check.as_ref().filter(|the_check| the_check.blender_output)?;
    if Some(the_check.job.as_str()) != check {
        return None;
    }
    crate::surface::the_history::the_kept_words_of(records, &the_check.job, &entry.name, &entry.server)
}

/// **`Queue & History` C2a as changed 2026-10-12: the user's *Dismiss* on one
/// dropped job's line takes that one name off the notice record that holds
/// it**, through `Queue & History`'s own act over the local record. Answers
/// whether the record took it; a job that is not an identity, or a record that
/// is not open, is a refusal. The 38's piece 15; one line rather than every
/// record since the Creator's ninth notes, note 3.
fn the_dropped_line_is_dismissed(started: &Mutex<Started>, job: Option<&str>) -> bool {
    let Some(job) = job.and_then(|job| {
        smoozy_state::identities::Identity::read_back(smoozy_state::identities::IdentityKind::Job, job)
    }) else {
        return false;
    };
    let Ok(started) = started.lock() else {
        return false;
    };
    let Some(record) = started.record.as_ref() else {
        return false;
    };
    crate::record::catch_up_notices::the_dropped_line_is_dismissed(record.connection(), &job).is_ok()
}

/// **The one kind an answer to a control arrives on**, for both screens —
/// `web/the-queue.js` names the same string.
pub const THE_ANSWER_SAYS: &str = "queue:the-answer";

/// **A control on either screen was pressed** (must-do 5, and step 10b piece
/// 8's own half of it).
///
/// **Three of them act now** — the delete box's *Delete* and the two clearing
/// boxes — and [`where_an_act_on_a_job_goes`] is what says which. **An act that
/// went through says nothing**: the history is posted again and the user reads
/// the list they changed, which is the same rule `src/surface/servers.rs` takes
/// for a connect that began. **An act that could not act answers**, in the one
/// failure shape beside the control that was pressed, at that moment only, with
/// nothing recorded and nothing to the diary.
///
/// `from` is the carry-on field's value, which goes with the carry-on act
/// (Design: *"it is a field and not a fixed number"*; `Rendering` A5); nothing
/// can act on it while making a new job from an entry is not built.
///
/// **Step 10c's S3: the entry's *Stop* and *Cancel* and a group's *Cancel all*
/// raise their boxes, and each box's go-ahead reaches the machine** —
/// [`crate::servers::the_queues_own_acts`], on a thread of its own (P5), the
/// entry saying *Stopping* or *Cancelling* from the moment it is pressed
/// (`Queue & History` L12a, L2). `group` is the group identity *Cancel all*
/// and its box hand back.
#[tauri::command]
#[allow(clippy::too_many_arguments)]
pub fn an_act_on_a_job<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    started: State<'_, Mutex<Started>>,
    queue: State<'_, std::sync::Arc<TheOneQueue>>,
    periods: State<'_, std::sync::Arc<crate::servers::a_period::ThePeriodsThisRun>>,
    act: TheActOnAJob,
    job: Option<String>,
    from: Option<String>,
    group: Option<String>,
    after: Option<String>,
) {
    let _ = from;
    // **`Queue & History` E8: *Blender output* on a rendering entry opens the
    // render's words as they arrive**, and never answers *not built* there.
    // Where the job is not rendering on a reading Smoozy holds, the press goes
    // on to the History entry's route below. Piece 6c.
    if act == TheActOnAJob::BlenderOutput {
        if let Some(job) = job.as_deref() {
            if the_running_words_are_opened(&app, periods.inner(), started.inner(), job) {
                return;
            }
        }
    }
    match where_an_act_on_a_job_goes(act) {
        WhereAnActOnAJobGoes::ItsBox => {
            let holds = periods.the_machines_work();
            if let Some(job) = job.as_deref() {
                holds.the_queue_is_shown(Some(job));
            }
            let held = {
                let Ok(started) = started.lock() else {
                    an_act_is_answered(&queue, act);
                    return;
                };
                what_is_held(holds, started.record.as_ref(), job.as_deref())
            };
            match the_box_is_raised(held, act, job.as_deref(), group.as_deref()) {
                Ok(says) => the_queue_is_posted(&queue, says),
                Err(sentence) => an_act_is_answered_with(&queue, act, sentence),
            }
            return;
        }
        WhereAnActOnAJobGoes::ItReachesTheMachine(own) => {
            use crate::servers::the_queues_own_acts::{the_act_is_begun, NothingWasSent};
            match the_act_is_begun(&app, own, job.as_deref(), group.as_deref()) {
                // **The entry says *Stopping* or *Cancelling* now**, drawn
                // again with the box gone.
                Ok(()) => the_queue_is_told_again(&app),
                Err(NothingWasSent::ItIsNotRendering) => {
                    an_act_is_answered_with(&queue, act, THIS_JOB_IS_NOT_RENDERING)
                }
                Err(NothingWasSent::ItIsNotInTheQueue) => an_act_is_answered(&queue, act),
            }
            return;
        }
        // **The 38's piece 8: a move on the line.** `after` is the job the
        // row or the group was dropped behind, and nothing for the front.
        // Nothing is drawn now: the move's own thread draws the queue again
        // from a fresh reading, or answers beside the row ([`the_move_came_to`]).
        WhereAnActOnAJobGoes::ItMovesOnTheLine => {
            let group = if act == TheActOnAJob::MoveTheGroup { group.as_deref() } else { None };
            if crate::servers::the_queues_own_acts::the_move_is_begun(&app, job.as_deref(), group, after.as_deref())
                .is_err()
            {
                an_act_is_answered(&queue, act);
            }
            return;
        }
        // **`Telegram` V1c, the twenty-first notes, note 7**: the picked value
        // rides `from`. Nothing is drawn now; the change's own thread draws
        // the queue again or answers beside the control.
        WhereAnActOnAJobGoes::TheReportingIsChanged => {
            let the_choice_now = started.lock().ok().and_then(|started| {
                let record = started.record.as_ref()?;
                let job = smoozy_state::identities::Identity::read_back(
                    smoozy_state::identities::IdentityKind::Job,
                    job.as_deref()?,
                )?;
                let text = crate::record::job_records::JobRecords::over(record.connection())
                    .the_reporting_choice_of(&job)
                    .ok()
                    .flatten()?;
                smoozy_render_settings::reporting::ReportingChoice::from_the_manifests_form(&text)
            });
            match what_the_telegram_control_sends(from.as_deref().unwrap_or_default(), the_choice_now) {
                Err(sentence) => an_act_is_answered_with(&queue, act, sentence),
                Ok(sends) => {
                    if crate::servers::the_queues_own_acts::the_reporting_change_is_begun(
                        &app,
                        job.as_deref(),
                        Some(&sends),
                    )
                    .is_err()
                    {
                        an_act_is_answered(&queue, act);
                    }
                }
            }
            return;
        }
        WhereAnActOnAJobGoes::ItAnswersTheHold(answer) => {
            match the_held_job_is_answered(&app, job.as_deref(), answer) {
                Ok(()) => the_queue_is_told_again(&app),
                Err(_) => an_act_is_answered(&queue, act),
            }
            return;
        }
        // **`Rendering` A1 and `Rendering` A4, the 38's piece 2.** The press reads the
        // machine (`Rendering` A10, and `Rendering` A4 on a carry-on), so it runs off the drawing
        // thread (P5); what it leads to is posted for the page, or said on
        // the control.
        WhereAnActOnAJobGoes::ANewJobFromTheEntry => {
            use crate::a_new_job_from_an_old_one::{the_page_is_told, the_press_is_taken, TheWayIn, WhereThePressLeads};
            let Some(job) = job else {
                an_act_is_answered(&queue, act);
                return;
            };
            let way = match act {
                TheActOnAJob::CarryOn => TheWayIn::CarryOn { typed: from },
                _ => TheWayIn::SubmitAgain,
            };
            let queue = std::sync::Arc::clone(queue.inner());
            std::thread::spawn(move || match the_press_is_taken(&app, &job, &way) {
                WhereThePressLeads::ItIsSaid(said) => an_act_is_answered_with(&queue, act, said),
                leads => the_page_is_told(&queue, &leads),
            });
            return;
        }
        // **`Queue & History` N20: the way to the earlier entry** — the mark's
        // copied identity, looked up against the record now.
        WhereAnActOnAJobGoes::TheEarlierEntryOpens => {
            let earlier = started.lock().ok().and_then(|started| {
                let record = started.record.as_ref()?;
                let records = crate::record::job_records::JobRecords::over(record.connection());
                let job = smoozy_state::identities::Identity::read_back(
                    smoozy_state::identities::IdentityKind::Job,
                    job.as_deref()?,
                )?;
                let earlier = records.read(&job).ok().flatten()?.carries_on_from?.earlier_job;
                records.read(&earlier).ok().flatten().map(|_| earlier)
            });
            match earlier {
                Some(earlier) => crate::a_new_job_from_an_old_one::the_earlier_entry_is_told(&queue, &earlier),
                None => an_act_is_answered(&queue, act),
            }
            return;
        }
        // **The 38's piece 9: the entry's switch turned off** (`Download` R3),
        // written to the record and the queue drawn again; a write the record
        // would not take is said beside the switch.
        WhereAnActOnAJobGoes::TheArrangementIsSwitchedOff => {
            match job.as_deref() {
                Some(job) if crate::surface::transfers::the_entrys_switch_is_turned_off(&app, job) => {
                    the_queue_is_told_again(&app);
                    crate::surface::transfers::the_transfers_moved(&app);
                }
                _ => an_act_is_answered(&queue, act),
            }
            return;
        }
        // **The 38's piece 9: *Files on your computer*** (`Download` H5 to H8,
        // `Queue & History` N18) — composed from the record, never the
        // machine, and opened off the drawing thread (P5); what it has to
        // say is said beside the control.
        WhereAnActOnAJobGoes::TheJobsFilesAreShown => {
            let Some(job) = job else {
                an_act_is_answered(&queue, act);
                return;
            };
            let queue = std::sync::Arc::clone(queue.inner());
            std::thread::spawn(move || {
                if let Some(said) = crate::surface::transfers::the_jobs_files_are_shown(&app, &job) {
                    an_act_is_answered_with(&queue, act, said);
                }
            });
            return;
        }
        // **Step 10c's *Files on the server* piece** (`Queue & History` N18):
        // the job's folder, read off the local record and never the machine,
        // and the page told to open Files there. **The control is not drawn
        // where the server is not connected** ([`TheJobEntry::
        // the_server_is_not_connected`]); a press on an entry drawn before the
        // connection went opens Files, which says so itself (`Design/File
        // Browser § While the connection is down`). A folder that is not there
        // is said by Files' own reading (`File Browser` P3), so neither is
        // answered here — **and Files is told whether the job's record shows
        // no frame saved**, so a folder not there yet says so (piece O).
        WhereAnActOnAJobGoes::TheJobsFolderOpensInFiles => {
            let found = match (started.lock(), job.as_deref()) {
                (Ok(started), Some(job)) => match started.record.as_ref() {
                    Some(record) => where_the_jobs_folder_is(record.connection(), job).map(|(server, folder)| {
                        (server, folder, the_record_shows_no_frame_saved(record.connection(), job))
                    }),
                    None => Err(act.answers()),
                },
                _ => Err(act.answers()),
            };
            match found {
                Ok((server, folder, no_frame_is_saved)) => {
                    the_jobs_folder_is_told(&queue, &server, &folder, no_frame_is_saved)
                }
                Err(sentence) => an_act_is_answered_with(&queue, act, sentence),
            }
            return;
        }
        // **`Rendering` S11: the scenes resolved as not submitted, sent as a
        // fresh act** — a new group with new job identities, off the drawing
        // thread like the Submit it follows.
        WhereAnActOnAJobGoes::TheRemainingScenesAreSubmitted => {
            let Some(server) = group else {
                an_act_is_answered(&queue, act);
                return;
            };
            let point = periods
                .the_sessions_of(&server)
                .filter(crate::servers::sessions::ThisServersSessions::a_connection_is_live);
            let remaining = started.lock().ok().and_then(|started| {
                let record = started.record.as_ref()?;
                the_remaining_act(&crate::record::job_records::JobRecords::over(record.connection()), &server)
            });
            let (Some(point), Some(mut remaining)) = (point, remaining) else {
                an_act_is_answered(&queue, act);
                return;
            };
            // **`Server Setup` R1c's readings, taken again at this press**, so
            // the fresh act holds by what stands now. Held-job piece A.
            remaining.the_driver_at_submission =
                crate::servers::the_driver_swap::the_driver_at_submission(&app, &server);
            std::thread::spawn(move || {
                let the_run = tauri::Manager::try_state::<std::sync::Arc<crate::downloading::TheRunsTransfers>>(&app)
                    .map(|the_run| std::sync::Arc::clone(the_run.inner()));
                if let Some(the_run) = the_run {
                    let door = crate::surface::render_settings::the_applications_record(app.clone());
                    let (came_to, named) = crate::submitting::the_act_is_performed_naming_its_jobs(
                        remaining.clone(),
                        &point,
                        &the_run,
                        &door,
                    );
                    the_remaining_act_ended(&remaining, &came_to, &named);
                }
                the_queue_is_told_again(&app);
            });
            return;
        }
        // **Owed piece 10: *Track it*, `Queue & History` A6 to `Queue & History` A9.** `group`
        // carries the server's identifier, as it does for the part banner's
        // control. The machine is read for each entry (A7) off the drawing
        // thread (P5); what went through is read on the screens drawn again,
        // and what did not is said beside the control.
        WhereAnActOnAJobGoes::TheWorkIsTracked => {
            use crate::servers::the_machines_work::{the_offer_is_accepted, WhyNothingWasTracked};
            let Some(server) = group.and_then(|text| {
                smoozy_state::identities::Identity::read_back(smoozy_state::identities::IdentityKind::LocalServer, &text)
            }) else {
                an_act_is_answered(&queue, act);
                return;
            };
            let queue = std::sync::Arc::clone(queue.inner());
            let periods = std::sync::Arc::clone(periods.inner());
            std::thread::spawn(move || {
                let Some(started) = tauri::Manager::try_state::<Mutex<Started>>(&app) else {
                    an_act_is_answered(&queue, act);
                    return;
                };
                let record = crate::keys::TheRunsRecord { started: started.inner() };
                let mut machine = crate::surface::servers::TheRemovalsOwnEnds { app: &app }.the_machine(&server.as_text());
                match the_offer_is_accepted(&server, &record, machine.as_mut(), periods.the_machines_work()) {
                    Ok(_) => {
                        the_queue_is_told_again(&app);
                        crate::surface::the_history::the_history_is_told_again(&app);
                        crate::surface::servers::the_work_column_is_told_again(&app);
                    }
                    Err(WhyNothingWasTracked::TheServerCouldNotBeReached) => an_act_is_answered(&queue, act),
                    Err(WhyNothingWasTracked::TheRecordRefusedIt) => {
                        an_act_is_answered_with(&queue, act, THE_WORK_COULD_NOT_BE_TRACKED)
                    }
                }
            });
            return;
        }
        // **The 38's piece 15: the box's *Dismiss***, `Queue & History` C2a's
        // act. The queue is drawn again without the notice; a removal the
        // record refuses is said beside the control.
        WhereAnActOnAJobGoes::TheNoticeIsDismissed => {
            if the_dropped_line_is_dismissed(started.inner(), job.as_deref()) {
                the_queue_is_told_again(&app);
            } else {
                an_act_is_answered(&queue, act);
            }
            return;
        }

        _ => {}
    }
    let Ok(started) = started.lock() else {
        an_act_is_answered(&queue, act);
        return;
    };
    match crate::surface::the_history::the_act_is_performed(&started, act, job.as_deref()) {
        Some(Ok(())) => {
            let kept = crate::surface::the_history::what_is_kept(
                started.record.as_ref(),
                job.as_deref(),
            );
            crate::surface::the_history::the_history_is_posted(&queue, kept);
        }
        Some(Err(())) | None => an_act_is_answered(&queue, act),
    }
}

// ════════════════════════════════════════════════════════════════════════════
// A submission that arrived in part — `Rendering` S10 and `Rendering` S11, the 38's piece 2
// ════════════════════════════════════════════════════════════════════════════

/// **A several-scene act the connection broke part-way, held for the run** —
/// the act as the page handed it over, and per scene-job the jobs it sent
/// with what became of each at the act. **Nothing durable copies it**: the
/// provisional records are the durable half (`Rendering` S10: *"the
/// provisional records are the list of the unaccounted"*), and each draw
/// reads them again, so a scene Smoozy could not account for moves to
/// *submitted* or *not submitted* as `State, identity and recovery` V1 step 2
/// resolves its record at a later connection.
#[derive(Clone)]
struct ThePartHeld {
    the_act: crate::submitting::WhatThePageHandsOver,
    scenes: Vec<(String, Vec<(smoozy_state::identities::Identity, crate::submitting::WhatBecameOfOneJob)>)>,
}

fn the_parts_this_run() -> &'static Mutex<std::collections::BTreeMap<String, ThePartHeld>> {
    static HELD: std::sync::OnceLock<Mutex<std::collections::BTreeMap<String, ThePartHeld>>> =
        std::sync::OnceLock::new();
    HELD.get_or_init(|| Mutex::new(std::collections::BTreeMap::new()))
}

/// **Where one scene stands**, read now.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WhereAScenesJobsStand {
    Submitted,
    NotSubmitted,
    NotKnown,
}

/// **One scene's jobs, resolved against the record now.** A job the act left
/// unaccounted is read again: a confirmed record is in the line, no record is
/// not submitted (V1 step 2's *absent*), and a provisional one is still not
/// known. A scene is submitted where every job of it is in the line.
fn where_a_scene_stands(
    store: &crate::record::job_records::JobRecords<'_>,
    jobs: &[(smoozy_state::identities::Identity, crate::submitting::WhatBecameOfOneJob)],
) -> WhereAScenesJobsStand {
    use crate::submitting::WhatBecameOfOneJob as Became;
    let mut not_known = false;
    for (job, became) in jobs {
        let now = match became {
            Became::ItIsInTheLine => WhereAScenesJobsStand::Submitted,
            Became::NotKnown => match store.read(job) {
                Ok(Some(record)) if record.standing == smoozy_queue_history::record::Standing::Confirmed => {
                    WhereAScenesJobsStand::Submitted
                }
                Ok(Some(_)) | Err(_) => WhereAScenesJobsStand::NotKnown,
                Ok(None) => WhereAScenesJobsStand::NotSubmitted,
            },
            _ => WhereAScenesJobsStand::NotSubmitted,
        };
        match now {
            WhereAScenesJobsStand::NotSubmitted => return WhereAScenesJobsStand::NotSubmitted,
            WhereAScenesJobsStand::NotKnown => not_known = true,
            WhereAScenesJobsStand::Submitted => {}
        }
    }
    if not_known {
        WhereAScenesJobsStand::NotKnown
    } else {
        WhereAScenesJobsStand::Submitted
    }
}

/// **An act ended: hold it where it is `Rendering` S10's case** — more than
/// one scene, and the connection broke part-way, which is a job Smoozy could
/// not account for or one not attempted behind it (`Rendering` S4a). **Any
/// other ending leaves a held part alone**: its scenes are still not
/// submitted until *Submit the remaining scenes* sends them, and the page
/// said that other act's own outcome.
pub(crate) fn an_act_ended(
    given: &crate::submitting::WhatThePageHandsOver,
    came_to: &crate::submitting::WhatTheActCameTo,
    named: &[crate::submitting::AJobAsSent],
) {
    use crate::submitting::{WhatBecameOfOneJob as Became, WhatTheActCameTo};
    let server = given.server.local_server.as_text();
    let Ok(mut parts) = the_parts_this_run().lock() else {
        return;
    };
    let WhatTheActCameTo::TheseJobs(jobs) = came_to else {
        return;
    };
    let it_broke_part_way = jobs
        .iter()
        .any(|job| matches!(job, Became::NotKnown | Became::NotAttemptedBehindAnUnaccountedOne));
    if given.scene_jobs.len() < 2 || !it_broke_part_way || jobs.len() != named.len() {
        return;
    }
    let scenes = given
        .scene_jobs
        .iter()
        .enumerate()
        .map(|(at, payload)| {
            let name = payload
                .scene
                .as_ref()
                .map(|scene| scene.name.as_text().to_string())
                .unwrap_or_default();
            let of_it = named
                .iter()
                .zip(jobs)
                .filter(|(sent, _)| sent.of_the_scene_job == at)
                .map(|(sent, became)| (sent.job.clone(), became.clone()))
                .collect();
            (name, of_it)
        })
        .collect();
    parts.insert(
        server,
        ThePartHeld {
            the_act: given.clone(),
            scenes,
        },
    );
}

/// **The part banner, read now** — the first held part with anything left to
/// say. `the_server` names a server and says whether it is connected, and
/// says nothing of a server the record holds no row for.
fn the_part_now(
    store: &crate::record::job_records::JobRecords<'_>,
    the_server: &dyn Fn(&str) -> Option<(String, bool)>,
) -> Option<ThePartSubmission> {
    let mut parts = the_parts_this_run().lock().ok()?;
    let mut drawn = None;
    let mut done: Vec<String> = Vec::new();
    for (server, part) in parts.iter() {
        let Some((name, connected)) = the_server(server) else {
            continue;
        };
        let mut said = ThePartSubmission {
            of: part.scenes.len() as u32,
            submission: part.the_act.label.clone().unwrap_or_else(|| {
                crate::surface::render_settings::the_files_name(&part.the_act.the_blend)
            }),
            the_servers_row: server.clone(),
            ..ThePartSubmission::default()
        };
        for (scene, jobs) in &part.scenes {
            match where_a_scene_stands(store, jobs) {
                WhereAScenesJobsStand::Submitted => said.submitted.push(scene.clone()),
                WhereAScenesJobsStand::NotSubmitted => said.not_submitted.push(scene.clone()),
                WhereAScenesJobsStand::NotKnown => said.not_known.push(scene.clone()),
            }
        }
        said.confirmed = said.submitted.len() as u32;
        // **Every scene accounted for and every one in the line**: nothing is
        // left to say, and the part goes.
        if said.not_submitted.is_empty() && said.not_known.is_empty() {
            done.push(server.clone());
            continue;
        }
        if drawn.is_none() {
            said.server = name;
            said.it_can_go = connected && !said.not_submitted.is_empty();
            drawn = Some(said);
        }
    }
    for server in done {
        parts.remove(&server);
    }
    drawn
}

/// **`Rendering` S11's fresh act ended: its scenes take the jobs it sent**, in
/// the part it was made from, so the banner reads what became of them — and
/// goes once every scene is in the line. **The fresh act is its own group**
/// (S11), and nothing else of the part moves.
pub(crate) fn the_remaining_act_ended(
    remaining: &crate::submitting::WhatThePageHandsOver,
    came_to: &crate::submitting::WhatTheActCameTo,
    named: &[crate::submitting::AJobAsSent],
) {
    use crate::submitting::{WhatBecameOfOneJob as Became, WhatTheActCameTo};
    let server = remaining.server.local_server.as_text();
    let Ok(mut parts) = the_parts_this_run().lock() else {
        return;
    };
    let Some(part) = parts.get_mut(&server) else {
        return;
    };
    let jobs: Vec<Became> = match came_to {
        WhatTheActCameTo::TheseJobs(jobs) if jobs.len() == named.len() => jobs.clone(),
        // **Nothing was sent** — a lock, or no first frame to split on: the
        // scenes stay as they were, not submitted.
        _ => return,
    };
    for (at, payload) in remaining.scene_jobs.iter().enumerate() {
        let name = payload.scene.as_ref().map(|scene| scene.name.as_text().to_string()).unwrap_or_default();
        let of_it: Vec<_> = named
            .iter()
            .zip(&jobs)
            .filter(|(sent, _)| sent.of_the_scene_job == at)
            .map(|(sent, became)| (sent.job.clone(), became.clone()))
            .collect();
        if let Some((_, held)) = part.scenes.iter_mut().find(|(scene, _)| *scene == name) {
            *held = of_it;
        }
    }
}

/// **`Rendering` S11's fresh act: the scene-jobs resolved as not submitted,
/// and nothing else** — the act it follows, with every other scene-job left
/// out. The fresh act issues its own group and job identities. **Nothing**
/// where no part is held for the server or no scene is resolved as not
/// submitted.
fn the_remaining_act(
    store: &crate::record::job_records::JobRecords<'_>,
    server: &str,
) -> Option<crate::submitting::WhatThePageHandsOver> {
    let parts = the_parts_this_run().lock().ok()?;
    let part = parts.get(server)?;
    let mut remaining = part.the_act.clone();
    remaining.scene_jobs = part
        .the_act
        .scene_jobs
        .iter()
        .zip(&part.scenes)
        .filter(|(_, (_, jobs))| where_a_scene_stands(store, jobs) == WhereAScenesJobsStand::NotSubmitted)
        .map(|(payload, _)| payload.clone())
        .collect();
    (!remaining.scene_jobs.is_empty()).then_some(remaining)
}

/// The post, with the act handed in.
pub fn an_act_is_answered(queue: &TheOneQueue, act: TheActOnAJob) {
    an_act_is_answered_with(queue, act, act.answers());
}

/// **The same, with the sentence handed in** — where a press met one of two
/// reasons and [`TheActOnAJob::answers`] holds the commoner.
pub fn an_act_is_answered_with(queue: &TheOneQueue, act: TheActOnAJob, sentence: &'static str) {
    let _ = queue.read_then_post::<(TheActOnAJob, &'static str), std::convert::Infallible>(
        || Ok((act, sentence)),
        |(act, sentence)| {
            Event::about(About::TheRun, THE_ANSWER_SAYS)
                .saying("act", Said::Value(Verbatim::new(act.name())))
                .saying("said", Said::Prose(Prose::new(*sentence)))
        },
    );
}

/// **What a move on the line says once it is over** — the 38's piece 8.
///
/// - **The machine answered**: the queue is drawn again, from the reading the
///   move's thread took, and that is the whole answer. A job moved is where
///   it was put; one that was rendering or had ended while the move travelled
///   is where the machine has it, saying *Rendering*, or gone from the queue —
///   Design's *"the row returns to where it was and says it is rendering
///   now"*, read off the machine rather than inferred (`Queue & History` L6).
/// - **Anything else changed nothing Smoozy can see**, and is said beside the
///   row that was dragged, in the one failure shape, by the sentences the
///   queue's other acts already use. **The queue is not drawn again then**,
///   since a drawing takes the answer away with it (must-do 5).
pub fn the_move_came_to<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    a_group: bool,
    came_to: crate::servers::the_queues_own_acts::WhatTheMoveCameTo,
) {
    use crate::servers::the_queues_own_acts::WhatTheMoveCameTo;
    use tauri::Manager;
    let sentence = match came_to {
        WhatTheMoveCameTo::TheMachineAnswered => {
            the_queue_is_told_again(app);
            crate::surface::servers::the_work_column_is_told_again(app);
            return;
        }
        WhatTheMoveCameTo::NotReached => SMOOZY_COULD_NOT_REACH_THE_SERVER,
        WhatTheMoveCameTo::NotTaken(_) => THE_SERVER_DID_NOT_ACCEPT_IT,
        WhatTheMoveCameTo::TheWaiterGaveUp => THE_MOVE_WAS_SENT,
    };
    let act = if a_group { TheActOnAJob::MoveTheGroup } else { TheActOnAJob::MoveTheJob };
    if let Some(queue) = app.try_state::<std::sync::Arc<TheOneQueue>>() {
        an_act_is_answered_with(&queue, act, sentence);
    }
}

/// **What a change of a live job's reporting came to, said** — `Telegram` V1c.
/// *Changed*: the record took the value, and the queue is drawn again from it.
/// Anything else is said in the ordinary failure shape beside the control, and
/// the control, drawn again from the record, goes back to what the server
/// holds (`Design/Queue & History § The entry's controls`).
pub fn the_reporting_change_came_to<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    came_to: crate::servers::the_queues_own_acts::WhatTheChangeCameTo,
) {
    use crate::servers::the_queues_own_acts::WhatTheChangeCameTo;
    use tauri::Manager;
    let sentence = match came_to {
        WhatTheChangeCameTo::Changed(_) => {
            the_queue_is_told_again(app);
            return;
        }
        WhatTheChangeCameTo::NotChanged(_) => THE_CHOICE_WAS_NOT_CHANGED,
        WhatTheChangeCameTo::NotTaken(_) => THE_SERVER_DID_NOT_ACCEPT_IT,
        WhatTheChangeCameTo::NotReached => SMOOZY_COULD_NOT_REACH_THE_SERVER,
        WhatTheChangeCameTo::TheWaiterGaveUp => THE_CHANGE_WAS_SENT,
    };
    the_queue_is_told_again(app);
    if let Some(queue) = app.try_state::<std::sync::Arc<TheOneQueue>>() {
        an_act_is_answered_with(&queue, TheActOnAJob::WhatItReports, sentence);
    }
}

// ════════════════════════════════════════════════════════════════════════════
// The held entry's two answers — step 10c's eleventh-pass fix piece 8
// ════════════════════════════════════════════════════════════════════════════

/// **Where the held entry's *Render all* stands**, for the entry to say.
/// ***Cancel* is a cancel** — `Specs/Queue & History`: *"Cancelling it and
/// saying no are one act"* — **so it stands in step 10c's S3's own record**
/// ([`WhereTheActStands`]), reads *Cancelling* while in hand, and ends in S3's
/// words. **Held for the run**, as S3's is; nothing durable copies it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum WhereTheAnswerStands {
    InHand,
    /// **`Rented-server runtime` U12 or `Rented-server runtime` U13: the hold is
    /// answered**, by this
    /// answer or one before it, so the pair is not offered again (`Queue &
    /// History` H5) and the next reading says what came of it.
    TheHoldIsAnswered,
    /// **`Rented-server runtime` U14: a *no* stood first** — told, not failed.
    AlreadyCancelled,
    NotReached,
    NotTaken,
    TheWaiterGaveUp,
}

impl WhereTheAnswerStands {
    /// **The words on the entry's status line**, where there are any. The two
    /// that reach nothing are S3's own sentences, one fact in one wording.
    fn what_the_entry_says(&self) -> Option<Vec<APart>> {
        let said = match self {
            WhereTheAnswerStands::InHand | WhereTheAnswerStands::TheHoldIsAnswered => return None,
            WhereTheAnswerStands::AlreadyCancelled => THIS_JOB_WAS_ALREADY_CANCELLED,
            WhereTheAnswerStands::NotReached => SMOOZY_COULD_NOT_REACH_THE_SERVER,
            WhereTheAnswerStands::NotTaken => THE_SERVER_DID_NOT_ACCEPT_IT,
            WhereTheAnswerStands::TheWaiterGaveUp => THE_ANSWER_WAS_SENT,
        };
        Some(vec![APart::words(said)])
    }

    /// **Whether the pair is offered again** — only where the answer did not
    /// reach the machine, or reached it with nothing written that Smoozy
    /// knows of.
    fn the_pair_is_offered_again(&self) -> bool {
        matches!(
            self,
            WhereTheAnswerStands::NotReached
                | WhereTheAnswerStands::NotTaken
                | WhereTheAnswerStands::TheWaiterGaveUp
        )
    }
}

fn the_answers_this_run() -> &'static Mutex<std::collections::HashMap<String, WhereTheAnswerStands>> {
    static HELD: std::sync::OnceLock<Mutex<std::collections::HashMap<String, WhereTheAnswerStands>>> =
        std::sync::OnceLock::new();
    HELD.get_or_init(|| Mutex::new(std::collections::HashMap::new()))
}

pub(crate) fn where_the_answer_stands(job: &str) -> Option<WhereTheAnswerStands> {
    the_answers_this_run().lock().ok()?.get(job).cloned()
}

pub(crate) fn the_answer_stands(job: &str, stands: Option<WhereTheAnswerStands>) {
    if let Ok(mut held) = the_answers_this_run().lock() {
        match stands {
            Some(stands) => {
                held.insert(job.to_string(), stands);
            }
            None => {
                held.remove(job);
            }
        }
    }
}

/// **Where each of the two ends leaves the entry**: the answer's own state,
/// S3's state for the cancel, whether the job is gone, and whether anything
/// went out — so the journal is read after it.
fn where_the_held_jobs_answer_left_it(
    answer: crate::servers::the_machines_work::TheHeldJobsAnswer,
    left: crate::servers::the_machines_work::WhatTheHeldJobsAnswerLeft,
) -> (Option<WhereTheAnswerStands>, Option<WhereTheActStands>, bool, bool) {
    use crate::servers::the_machines_work::{
        TheHeldJobsAnswer as Answer, WhatTheAnswerCameTo as CameTo, WhatTheHeldJobsAnswerLeft as Left,
    };
    use smoozy_runtime::holds::ReleaseOutcome;
    let cancel = TheQueuesOwnAct::TheCancel;
    match (answer, left) {
        (_, Left::ItIsInTheLine) => (Some(WhereTheAnswerStands::TheHoldIsAnswered), None, false, true),
        (_, Left::ItWasRemoved) => (None, None, true, true),
        (Answer::RenderAll, Left::AnotherAnswerStood(ReleaseOutcome::Stopped)) => {
            (Some(WhereTheAnswerStands::AlreadyCancelled), None, false, true)
        }
        (_, Left::AnotherAnswerStood(_)) => (Some(WhereTheAnswerStands::TheHoldIsAnswered), None, false, true),
        (_, Left::TheCancel(stands)) => (None, Some(stands), false, true),
        (Answer::RenderAll, Left::TheAnswer(came_to)) => (
            Some(match came_to {
                CameTo::NotReached => WhereTheAnswerStands::NotReached,
                CameTo::NotTaken => WhereTheAnswerStands::NotTaken,
                _ => WhereTheAnswerStands::TheWaiterGaveUp,
            }),
            None,
            false,
            came_to != CameTo::NotReached,
        ),
        (Answer::CancelIt, Left::TheAnswer(came_to)) => (
            None,
            Some(match came_to {
                CameTo::NotReached => WhereTheActStands::NotReached(cancel),
                CameTo::NotTaken => WhereTheActStands::NotTaken(cancel, None),
                _ => WhereTheActStands::TheWaiterGaveUp(cancel),
            }),
            false,
            came_to != CameTo::NotReached,
        ),
    }
}

/// **The held entry's answer, begun** — found in the period-held reading
/// (`Queue & History` C6), marked in hand, then sent on a thread of its own
/// (P5) through the server's own point. **When it ends**, a job the machine
/// removed takes its local record (`Queue & History` L2), one read-forward
/// pass runs over the same connection, and the three places are drawn again —
/// the shape step 10c's S3 gave the queue's other acts.
fn the_held_job_is_answered<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    job: Option<&str>,
    answer: crate::servers::the_machines_work::TheHeldJobsAnswer,
) -> Result<(), crate::servers::the_queues_own_acts::NothingWasSent> {
    use crate::servers::the_machines_work::TheHeldJobsAnswer;
    use crate::servers::the_queues_own_acts::{it_stands, NothingWasSent};
    use smoozy_queue_history::line::QueueState;
    use smoozy_state::identities::{Identity, IdentityKind};
    let job = job
        .and_then(|text| Identity::read_back(IdentityKind::Job, text))
        .ok_or(NothingWasSent::ItIsNotInTheQueue)?;
    let periods = crate::surface::servers::the_periods_of(app).ok_or(NothingWasSent::ItIsNotInTheQueue)?;
    // **`Rented-server runtime` U2: the hold identity is the reading's own**,
    // carried with the held job (`Queue & History` A1).
    let (server, hold, the_reason) = periods
        .the_machines_work()
        .every_server()
        .into_iter()
        .find_map(|(server, period)| {
            period.reading.as_ref()?.value().jobs.iter().find_map(|read| match &read.state {
                QueueState::Held { hold } if read.job == job => Some((
                    server.clone(),
                    hold.clone(),
                    read.the_hold.as_ref().map(|says| says.reason.clone()),
                )),
                _ => None,
            })
        })
        .ok_or(NothingWasSent::ItIsNotInTheQueue)?;
    // **Which answer the pressed control gives turns on which pair it is**:
    // a job held at the start of its render has *Stop this job* first, the
    // *no* (the 38's piece 10, item 26).
    let answer = the_answer_the_control_gives(answer == TheHeldJobsAnswer::RenderAll, the_reason.as_deref());
    let text = job.as_text();
    // **One press at a time.**
    if the_act_in_hand(&job).is_some() || where_the_answer_stands(&text) == Some(WhereTheAnswerStands::InHand) {
        return Ok(());
    }
    match answer {
        TheHeldJobsAnswer::RenderAll => {
            it_stands(&text, None);
            the_answer_stands(&text, Some(WhereTheAnswerStands::InHand));
        }
        TheHeldJobsAnswer::CancelIt => {
            the_answer_stands(&text, None);
            it_stands(&text, Some(WhereTheActStands::InHand(TheQueuesOwnAct::TheCancel)));
        }
    }
    let app = app.clone();
    std::thread::spawn(move || {
        let mut machine = crate::surface::servers::TheRemovalsOwnEnds { app: &app }.the_machine(&server);
        let left = crate::servers::the_machines_work::the_held_jobs_answer_is_sent(
            machine.as_mut(),
            &job,
            &hold,
            answer,
            crate::servers::stop_and_clear::TheWait::THE_STOPS_OWN,
        );
        let (the_answer, the_cancel, removed, it_went_out) = where_the_held_jobs_answer_left_it(answer, left);
        the_answer_stands(&text, the_answer);
        it_stands(&text, the_cancel);
        if it_went_out {
            the_journal_is_read_after_the_answer(&app, &server, machine.as_mut(), removed.then_some(&job));
        }
        the_queue_is_told_again(&app);
        crate::surface::the_history::the_history_is_told_again(&app);
        crate::surface::servers::the_work_column_is_told_again(&app);
    });
    Ok(())
}

/// **`Queue & History` L2 on the answer, then one read-forward pass** — the
/// same two steps step 10c's S3 takes after its own acts. A local record the
/// machine said is gone is removed only where it is this server's.
fn the_journal_is_read_after_the_answer<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    server: &str,
    machine: &mut dyn crate::servers::the_machines_work::AsksTheSupervisor,
    removed: Option<&smoozy_state::identities::Identity>,
) {
    use smoozy_state::identities::{Identity, IdentityKind};
    use tauri::Manager;
    let (Some(started), Some(periods), Some(identity)) = (
        app.try_state::<Mutex<Started>>(),
        crate::surface::servers::the_periods_of(app),
        Identity::read_back(IdentityKind::LocalServer, server),
    ) else {
        return;
    };
    if let Some(job) = removed {
        if let Ok(started) = started.lock() {
            if let Some(record) = started.record.as_ref() {
                let store = crate::record::job_records::JobRecords::over(record.connection());
                if let Ok(Some(held)) = store.read(job) {
                    if held.server.local_server == identity {
                        let _ = store.remove(job);
                    }
                }
            }
        }
    }
    let record = crate::keys::TheRunsRecord { started: started.inner() };
    let _ = crate::servers::the_machines_work::a_read_forward_pass(
        &identity,
        &record,
        machine,
        periods.the_machines_work(),
    );
}

/// **The box a press asks for, over the queue as it is held** — Design, *The
/// entry's controls* and *The queue list*. **The box is raised only over what
/// the reading still shows**: a stop over a job rendering, a cancel over a job
/// whose entry offers *Cancel*, and a group's over its heading; anything else
/// answers beside the control and raises nothing. Step 10c's S3.
pub(crate) fn the_box_is_raised(
    mut held: TheQueueOnScreen,
    act: TheActOnAJob,
    job: Option<&str>,
    group: Option<&str>,
) -> Result<TheQueueOnScreen, &'static str> {
    the_declined_banner_is_applied(&mut held);
    let the_entry = held
        .entry
        .as_ref()
        .filter(|entry| job.is_some_and(|job| entry.job == job));
    held.asking = match act {
        TheActOnAJob::Stop => match the_entry.map(|entry| entry.stop_or_cancel) {
            Some(Some(TheStopOrCancel::Stop)) => Some(the_stop_box(None)),
            Some(_) => return Err(THIS_JOB_IS_NOT_RENDERING),
            None => return Err(THIS_JOB_IS_NO_LONGER_IN_THE_QUEUE),
        },
        TheActOnAJob::Cancel => match the_entry.map(|entry| entry.stop_or_cancel) {
            Some(Some(TheStopOrCancel::Cancel)) => Some(the_cancel_box()),
            _ => return Err(THIS_JOB_IS_NO_LONGER_IN_THE_QUEUE),
        },
        TheActOnAJob::CancelAll => {
            let Some(heading) = held
                .groups
                .iter()
                .find(|heading| group.is_some_and(|group| heading.group == group))
            else {
                return Err(THESE_JOBS_ARE_NO_LONGER_IN_THE_QUEUE);
            };
            held.the_group_asked_about = Some(heading.group.clone());
            Some(the_group_cancel_box(heading.scenes, heading.rendering.as_deref()))
        }
        _ => return Err(act.answers()),
    };
    Ok(held)
}

/// **What a control answers, on the event**: the act's name as a value, so the
/// page draws it beside that control, and the sentence as prose.
pub fn the_answer_says(act: &TheActOnAJob) -> Event {
    Event::about(About::TheRun, THE_ANSWER_SAYS)
        .saying("act", Said::Value(Verbatim::new(act.name())))
        .saying("said", Said::Prose(Prose::new(act.answers())))
}

/// The post, with the reading handed in — **so the whole of it can be driven
/// without the shell's own managed state**, which the command cannot be. The
/// same split pieces B, C, F and 1 all make.
pub fn the_queue_is_posted(queue: &TheOneQueue, says: TheQueueOnScreen) {
    let _ = queue.read_then_post::<TheQueueOnScreen, std::convert::Infallible>(
        || Ok(says.clone()),
        the_queue_says,
    );
}

// ───────────────────────────────────────────────────────────────────────────
// The event
// ───────────────────────────────────────────────────────────────────────────

/// **Every name a list's field travels under is made by the one name-maker**,
/// [`crate::surface::settings::a_name`], so a list is as long as the reading
/// made it — L8a: there is no state in which a list is drawn short. **Until
/// must-do 3 of step 10a round 1 of the queue, history, settings and downloads
/// these names were fixed tables of four, eight and twenty-four**, and a longer
/// list was cut with nothing said. Every list is sent whole; no Design state
/// draws paging.
pub(crate) use crate::surface::settings::a_name;

/// **The counted word, chosen by the count** — must-do 12. The page never adds
/// an *s*, and neither does anything here.
pub(crate) fn by_count(count: impl TryInto<u64>, one: &'static str, many: &'static str) -> &'static str {
    if count.try_into().ok() == Some(1) {
        one
    } else {
        many
    }
}

pub(crate) fn a_part_is_said(
    mut event: Event,
    lead: &'static str,
    value: &'static str,
    trail: &'static str,
    binds: &'static str,
    part: &APart,
) -> Event {
    if !part.lead.is_empty() {
        event = event.saying(lead, Said::Prose(Prose::new(part.lead)));
    }
    if !part.value.is_empty() {
        event = event.saying(value, Said::Value(Verbatim::new(part.value.clone())));
    }
    if !part.trail.is_empty() {
        event = event.saying(trail, Said::Prose(Prose::new(part.trail)));
    }
    // **A figure and the words after it are bound** — *2 waiting*, *240
    // frames*, *62% done* — so the page writes a non-breaking space between
    // them and they never part across two lines. `Design/Whole app §
    // Typography`. Worked out here, where every part becomes what the page
    // reads, so a part composed later is bound without anybody remembering to.
    if the_trail_binds(part) {
        event = event.saying(binds, Said::Value(Verbatim::new("yes")));
    }
    // **Words that open on a comma or a full stop follow their value with no
    // space** — *525.147.05, was too old*, never *525.147.05 , was too old*.
    // Said here, beside the bind, so the page reads no word to decide it.
    // Held-job piece B, 2026-10-12, for `Queue & History` N23's mark; every
    // part composed with such words gains it.
    if the_trail_clings(part) {
        event = event.saying(
            crate::surface::settings::a_kept_name(format!("{trail}Clings")),
            Said::Value(Verbatim::new("yes")),
        );
    }
    event
}

/// **Whether a part's words follow its value with no space between**: where
/// a value stands before them and they open on punctuation.
pub(crate) fn the_trail_clings(part: &APart) -> bool {
    !part.value.is_empty() && part.trail.starts_with([',', '.', ';', ':', '?'])
}

/// **Whether a part's words follow its value with a non-breaking space**:
/// where the value is a figure and words follow it.
pub(crate) fn the_trail_binds(part: &APart) -> bool {
    !part.trail.is_empty() && crate::typography::is_a_figure(&part.value)
}

/// **A part under `base` + index + `tail` + `Lead` / `Value` / `Trail`.**
pub(crate) fn a_part_is_said_under(event: Event, base: &str, at: usize, tail: &str, part: &APart) -> Event {
    a_part_is_said(
        event,
        a_name(base, at, &format!("{tail}Lead")),
        a_name(base, at, &format!("{tail}Value")),
        a_name(base, at, &format!("{tail}Trail")),
        a_name(base, at, &format!("{tail}Binds")),
        part,
    )
}

/// **A sentence the Render Settings panel composed, said under one base** —
/// each part's lead, values, rest and the two marks, with the count beside
/// them. The keys are `{tail}{k}Lead`, `{tail}{k}Value{j}`, `{tail}{k}Rest`,
/// `{tail}{k}Joins` and `{tail}{k}Binds`. Added 2026-10-19 for a row saying
/// *Not submitted.* (`Queue & History` C10).
fn a_sentence_is_said(
    mut event: Event,
    base: &str,
    at: usize,
    tail: &str,
    sentence: &[crate::surface::servers::Words],
) -> Event {
    for (k, part) in sentence.iter().enumerate() {
        let key = |what: &str| a_name(base, at, &format!("{tail}{k}{what}"));
        event = event.saying(key("Lead"), Said::Prose(part.lead.clone()));
        for (j, value) in part.values.iter().enumerate() {
            event = event.saying(key(&format!("Value{j}")), Said::Value(value.clone()));
        }
        event = event.saying(key("ValueCount"), Said::Value(Verbatim::new(part.values.len().to_string())));
        if let Some(rest) = &part.rest {
            event = event.saying(key("Rest"), Said::Prose(rest.clone()));
        }
        if part.the_rest_joins {
            event = event.saying(key("Joins"), Said::Value(Verbatim::new("yes")));
        }
        if part.the_rest_binds() {
            event = event.saying(key("Binds"), Said::Value(Verbatim::new("yes")));
        }
    }
    event.saying(
        a_name(base, at, &format!("{tail}Count")),
        Said::Value(Verbatim::new(sentence.len().to_string())),
    )
}

/// **A list of parts under one base**, with its count beside it.
pub(crate) fn the_parts_are_said(mut event: Event, base: &str, at: usize, tail: &str, parts: &[APart]) -> Event {
    for (k, part) in parts.iter().enumerate() {
        event = a_part_is_said_under(event, base, at, &format!("{tail}{k}"), part);
    }
    event.saying(
        a_name(base, at, &format!("{tail}Count")),
        Said::Value(Verbatim::new(parts.len().to_string())),
    )
}

/// **What the queue says, in the two routes and never in one string.**
///
/// A6 and X9: prose through the typography point, everything else by the
/// verbatim route, and the two are the complete set. **Every figure and every
/// name goes by the verbatim route and every word beside it as prose**, because
/// a value joined into prose before the point is exactly the join X7 forbids.
pub fn the_queue_says(says: &TheQueueOnScreen) -> Event {
    let mut event = Event::about(About::TheRun, THE_QUEUE_SAYS)
        .saying("theHeading", Said::Prose(Prose::new(THE_HEADING)))
        .saying(
            "theJobsFigure",
            Said::Value(Verbatim::new(says.jobs.to_string())),
        )
        .saying(
            "theJobsWord",
            Said::Prose(Prose::new(by_count(says.jobs, THE_JOB_WORD, THE_JOBS_WORD))),
        )
        .saying(
            "theServersFigure",
            Said::Value(Verbatim::new(says.servers.to_string())),
        )
        .saying(
            "theServersWord",
            Said::Prose(Prose::new(by_count(says.servers, THE_SERVER_WORD, THE_SERVERS_WORD))),
        )
        // **C7: a last-known reading says it is not updating and when it was
        // last seen.** The two words travel whether or not a line carries the
        // reading, because which lines do is the reading's own business.
        .saying("notUpdatingWord", Said::Prose(Prose::new(NOT_UPDATING)))
        .saying("lastSeenWord", Said::Prose(Prose::new(LAST_SEEN)))
        .saying("renderingWord", Said::Prose(Prose::new(RENDERING_WORD)))
        .saying("waitingWord", Said::Prose(Prose::new(WAITING_WORD)));

    // **Design, *The queue list*: the teaching line, the row's menu item and
    // the grips' two tooltips** — sent only where something on the list can be
    // dragged, so no line teaches a gesture over a list where it does nothing
    // (must-do 5(f)). The 38's piece 8.
    let something_moves = says.lines.iter().flat_map(|line| &line.rows).any(|row| row.grip)
        || says.groups.iter().any(|group| group.grip);
    if something_moves {
        event = event
            .saying("whatDraggingDoes", Said::Prose(Prose::new(WHAT_DRAGGING_DOES)))
            .saying("andTheRowsOwnMenu", Said::Prose(Prose::new(AND_THE_ROWS_OWN_MENU)))
            .saying("moveToTheTop", Said::Prose(Prose::new(MOVE_TO_THE_TOP)))
            .saying("dragItUpTheQueue", Said::Prose(Prose::new(DRAG_IT_UP_THE_QUEUE)))
            .saying("dragTheWholeGroup", Said::Prose(Prose::new(DRAG_THE_WHOLE_GROUP)));
    }

    // **Must-do 21: the rail row shows nothing for nought**, which is the rule
    // Transfers' row already follows.
    if says.jobs > 0 {
        event = event.saying(
            "theRailsFigure",
            Said::Value(Verbatim::new(says.jobs.to_string())),
        );
    }

    event = the_lines_are_said(event, says);
    event = the_groups_are_said(event, &says.groups);
    event = the_notice_is_said(event, says.notice.as_ref());
    event = the_untracked_is_said(event, says.untracked.as_ref());
    event = the_part_is_said(event, says.part.as_ref());
    event = the_entry_is_said(event, says.entry.as_ref());
    event = the_box_is_said(event, says.asking.as_ref());
    // **A check's kept words, where its *Blender output* was pressed** — the
    // one panel's own keys, as History says them (`Queue & History` E9).
    if let Some(words) = &says.words {
        event = crate::surface::the_machines_own_words::the_kept_words_are_said(
            event,
            &words.job,
            &words.server,
            &words.attempts,
        );
    }
    // **The group the box is about**, handed back by its go-ahead (step 10c's S3).
    if let (Some(_), Some(group)) = (&says.asking, &says.the_group_asked_about) {
        event = event.saying("boxGroup", Said::Value(Verbatim::new(group.clone())));
    }
    // **The dropped job the box is about**, handed back by its go-ahead (the
    // Creator's ninth notes, note 3).
    if let (Some(_), Some(job)) = (&says.asking, &says.the_dropped_job_asked_about) {
        event = event.saying("boxJob", Said::Value(Verbatim::new(job.clone())));
    }
    event
}

/// **Must-do 7: a folded server heading says what Design says**, composed here
/// in parts for all three of its cases — *62% · 2 waiting* where a job is
/// rendering (its name and bar travel beside these), *2 waiting* where only
/// jobs wait, and *Not updating · last seen 11:04* where the reading is
/// last-known. `Design/Queue & History § The queue list`: *"a folded heading
/// still says what is running."*
pub fn the_folded_line(line: &AServersLine) -> Vec<APart> {
    if let Some(seen) = &line.last_seen {
        return vec![
            APart::words(NOT_UPDATING),
            APart::words(A_DOT),
            APart::said(LAST_SEEN, seen.clone(), ""),
        ];
    }
    let mut parts = Vec::new();
    if let Some((_, at)) = &line.folded {
        parts.push(APart::value(format!("{at}%")));
    }
    if line.waiting > 0 {
        if !parts.is_empty() {
            parts.push(APart::words(A_DOT));
        }
        parts.push(APart::figure(line.waiting.to_string(), WAITING_WORD));
    }
    parts
}

fn the_lines_are_said(mut event: Event, says: &TheQueueOnScreen) -> Event {
    let mut at_row = 0usize;
    let mut said_the_cross = false;
    for (at, line) in says.lines.iter().enumerate() {
        event = event
            .saying(
                a_name("line", at, "Server"),
                Said::Value(Verbatim::new(line.row.clone())),
            )
            .saying(
                a_name("line", at, "Name"),
                Said::Value(Verbatim::new(line.name.clone())),
            )
            .saying(
                a_name("line", at, "Rendering"),
                Said::Value(Verbatim::new(line.rendering.to_string())),
            )
            .saying(
                a_name("line", at, "Waiting"),
                Said::Value(Verbatim::new(line.waiting.to_string())),
            )
            .saying(
                a_name("line", at, "Rows"),
                Said::Value(Verbatim::new(line.rows.len().to_string())),
            );
        if let Some(badge) = &line.badge {
            event = event.saying(a_name("line", at, "Badge"), Said::Value(Verbatim::new(badge.clone())));
        }
        if let Some(seen) = &line.last_seen {
            event = event.saying(
                a_name("line", at, "LastSeen"),
                Said::Value(Verbatim::new(seen.clone())),
            );
        }
        if let (Some((name, at_percent)), None) = (&line.folded, &line.last_seen) {
            event = event
                .saying(
                    a_name("line", at, "FoldedName"),
                    Said::Value(Verbatim::new(name.clone())),
                )
                .saying(
                    a_name("line", at, "FoldedAt"),
                    Said::Value(Verbatim::new(at_percent.to_string())),
                );
        }
        event = the_parts_are_said(event, "line", at, "Folded", &the_folded_line(line));
        for row in &line.rows {
            event = event
                .saying(
                    a_name("row", at_row, "Line"),
                    Said::Value(Verbatim::new(at.to_string())),
                )
                .saying(a_name("row", at_row, "Name"), Said::Value(Verbatim::new(row.name.clone())))
                .saying(a_name("row", at_row, "Job"), Said::Value(Verbatim::new(row.job.clone())))
                .saying(
                    a_name("row", at_row, "StateIs"),
                    Said::Value(Verbatim::new(row.state_is)),
                );
            event = the_pair_is_said(event, "row", at_row, &row.the_pair);
            if let Some(state) = row.state {
                event = event.saying(a_name("row", at_row, "State"), Said::Prose(Prose::new(state)));
            }
            // **Each bar where the row has it** (W4): absent stays absent
            // (`Queue & History` R4), and one bar never waits on the other.
            if let Some(renderlet) = row.bars.now_at {
                event = event.saying(
                    a_name("row", at_row, "Renderlet"),
                    Said::Value(Verbatim::new(renderlet.to_string())),
                );
            }
            if let Some(whole) = row.bars.whole {
                event = event.saying(
                    a_name("row", at_row, "Whole"),
                    Said::Value(Verbatim::new(whole.to_string())),
                );
            }
            if row.grip {
                event = event.saying(a_name("row", at_row, "Grip"), Said::Value(Verbatim::new("yes")));
            }
            if row.not_updating {
                event = event.saying(a_name("row", at_row, "Stale"), Said::Value(Verbatim::new("yes")));
            }
            if row.submitting {
                event = event.saying(a_name("row", at_row, "Submitting"), Said::Value(Verbatim::new("yes")));
            }
            if let Some(not) = &row.not_submitted {
                event = event.saying(
                    a_name("row", at_row, "NotSubmitted"),
                    Said::Value(Verbatim::new(not.act.to_string())),
                );
                event = a_sentence_is_said(event, "row", at_row, "Went", &not.what_went);
                event = a_sentence_is_said(event, "row", at_row, "Why", &not.why);
                if !said_the_cross {
                    event = event.saying("clearIt", Said::Prose(Prose::new(CLEAR_IT)));
                    said_the_cross = true;
                }
            }
            if let Some(group) = row.group {
                event = event.saying(
                    a_name("row", at_row, "Group"),
                    Said::Value(Verbatim::new(group.to_string())),
                );
            }
            let parts: Vec<APart> = row.sub.iter().filter(|part| !part.is_empty()).cloned().collect();
            event = the_parts_are_said(event, "row", at_row, "Sub", &parts);
            at_row += 1;
        }
    }
    event
        .saying(
            "lineCount",
            Said::Value(Verbatim::new(says.lines.len().to_string())),
        )
        .saying("rowCount", Said::Value(Verbatim::new(at_row.to_string())))
}

/// **A pair's part lines, under `base` + index** — `Pair{k}What`,
/// `Pair{k}State` and `Pair{k}Dot`, with `PairCount` beside them. History's
/// row says its one check line the same way. The nineteenth notes, note 7.
pub(crate) fn the_pair_is_said(mut event: Event, base: &str, at: usize, parts: &[APartOfThePair]) -> Event {
    if parts.is_empty() {
        return event;
    }
    for (k, part) in parts.iter().enumerate() {
        event = a_part_is_said_under(event, base, at, &format!("Pair{k}What"), &part.what);
        event = a_part_is_said_under(event, base, at, &format!("Pair{k}State"), &part.state);
        if !part.dot.is_empty() {
            event = event.saying(a_name(base, at, &format!("Pair{k}Dot")), Said::Value(Verbatim::new(part.dot)));
        }
    }
    event.saying(a_name(base, at, "PairCount"), Said::Value(Verbatim::new(parts.len().to_string())))
}

/// **The check on its pair's entry, under `ff`** — the drawing's own prefix
/// for these parts (`.ffunit`, `.ffpic`, `.ffask`) — with the check's
/// picture's keys under the same prefix ([`the_preview_is_said`]), so the page
/// draws it with the preview's own builder. The nineteenth notes, note 7.
pub(crate) fn the_check_is_said(mut event: Event, the_check: Option<&TheCheckOnTheEntry>) -> Event {
    let Some(the_check) = the_check else {
        return event;
    };
    event = event.saying("ffJob", Said::Value(Verbatim::new(the_check.job.clone())));
    event = a_part_is_said(event, "ffHeadingLead", "ffHeadingValue", "ffHeadingTrail", "ffHeadingBinds", &the_check.heading);
    event = the_parts_are_said(event, "ff", 0, "Said", &the_check.said);
    if let Some(rendered_for) = &the_check.rendered_for {
        event = event
            .saying("ffRenderedIn", Said::Prose(Prose::new(RENDERED_IN)))
            .saying("ffRenderedFor", Said::Value(Verbatim::new(rendered_for.clone())));
    }
    if let Some(when) = &the_check.when {
        event = event.saying("ffWhen", Said::Value(Verbatim::new(when.clone())));
    }
    if the_check.you_chose_render_all {
        event = event
            .saying("ffYouChose", Said::Prose(Prose::new(YOU_CHOSE)))
            .saying("ffRenderAll", Said::Prose(Prose::new(RENDER_ALL_WITH_NO_COUNT)))
            .saying("ffFullStop", Said::Prose(Prose::new(A_FULL_STOP)));
    }
    if the_check.blender_output {
        event = event.saying("ffBlenderOutput", Said::Prose(Prose::new(BLENDER_OUTPUT)));
    }
    if let Some(range) = &the_check.show_in_queue {
        event = event
            .saying("ffShowInQueue", Said::Prose(Prose::new(SHOW_IN_QUEUE)))
            .saying("ffShowInQueueJob", Said::Value(Verbatim::new(range.clone())));
    }
    if the_check.in_the_question {
        event = event.saying("ffInTheQuestion", Said::Value(Verbatim::new("yes")));
    }
    if let Some(picture) = &the_check.picture {
        event = the_preview_is_said(event, picture, "ff");
    }
    event
}

fn the_groups_are_said(mut event: Event, groups: &[AGroupHeading]) -> Event {
    for (at, group) in groups.iter().enumerate() {
        event = event
            .saying(
                a_name("group", at, "Name"),
                Said::Value(Verbatim::new(group.name.clone())),
            )
            .saying(
                a_name("group", at, "Group"),
                Said::Value(Verbatim::new(group.group.clone())),
            )
            .saying(
                a_name("group", at, "Scenes"),
                Said::Value(Verbatim::new(group.scenes.to_string())),
            )
            .saying(
                a_name("group", at, "ScenesWord"),
                Said::Prose(Prose::new(by_count(group.scenes, SCENE_WORD, SCENES_WORD))),
            );
        if group.grip {
            event = event.saying(a_name("group", at, "Grip"), Said::Value(Verbatim::new("yes")));
        }
    }
    if groups.is_empty() {
        return event;
    }
    event
        .saying(
            "groupCount",
            Said::Value(Verbatim::new(groups.len().to_string())),
        )
        .saying("allScenesLead", Said::Prose(Prose::new(ALL_SCENES_LEAD)))
        .saying("cancelAll", Said::Prose(Prose::new(CANCEL_ALL)))
}

/// **The catch-up notice, said** — Design, *The catch-up notice*, the
/// Creator's ninth notes, note 3. **The count**: each kind's figure, its word
/// and the class of its History colour (the class History's own word takes,
/// [`crate::surface::the_history::AnEnding::the_class_of`]), and the hover
/// once. **Each dropped job's line**: its name, what happened to it, and its
/// job for *Dismiss* to hand back, with *Dismiss* said once.
fn the_notice_is_said(mut event: Event, notice: Option<&TheCatchUpNotice>) -> Event {
    use smoozy_queue_history::ending::Ending;
    let Some(notice) = notice else {
        return event;
    };
    if !notice.counted.is_empty() {
        event = event.saying("theCountSince", Said::Prose(Prose::new(SINCE_YOU_LAST_OPENED_HISTORY)));
        for (at, (ending, many)) in notice.counted.iter().enumerate() {
            let word = match ending {
                Ending::Finished => ENDED_FINISHED,
                Ending::Failed => ENDED_FAILED,
                Ending::Stopped => ENDED_STOPPED,
                Ending::GivenUp => ENDED_GIVEN_UP,
            };
            event = event
                .saying(a_name("counted", at, "Figure"), Said::Value(Verbatim::new(many.to_string())))
                .saying(a_name("counted", at, "Word"), Said::Prose(Prose::new(word)))
                .saying(
                    a_name("counted", at, "Is"),
                    Said::Value(Verbatim::new(crate::surface::the_history::AnEnding::the_class_of(*ending))),
                );
        }
        event = event.saying("countedCount", Said::Value(Verbatim::new(notice.counted.len().to_string())));
    }
    if !notice.dropped.is_empty() {
        for (at, line) in notice.dropped.iter().enumerate() {
            event = event
                .saying(a_name("dropped", at, "Name"), Said::Value(Verbatim::new(line.name.clone())))
                .saying(a_name("dropped", at, "Job"), Said::Value(Verbatim::new(line.job.clone())));
            event = a_part_is_said_under(event, "dropped", at, "What", &line.what);
        }
        event = event
            .saying("droppedCount", Said::Value(Verbatim::new(notice.dropped.len().to_string())))
            .saying("dismiss", Said::Prose(Prose::new(DISMISS)));
    }
    event
}

fn the_untracked_is_said(event: Event, untracked: Option<&TheUntrackedWork>) -> Event {
    let Some(untracked) = untracked else {
        return event;
    };
    // **Must-do 19: no removal is claimed that may not have happened.** Where
    // the reading does not say which cause it was, the sentence simply ends.
    let since = match untracked.cause {
        Some(WhyItIsUntracked::TheServerWasRemoved) => FROM_BEFORE_THIS_SERVER_WAS_REMOVED,
        Some(WhyItIsUntracked::ItsAddressWasChanged) => FROM_BEFORE_ITS_ADDRESS_WAS_CHANGED,
        None => A_FULL_STOP,
    };
    let mut event = event
        .saying(
            "theUntrackedServer",
            Said::Value(Verbatim::new(untracked.server.clone())),
        )
        // **What *Ignore* sends back**, so the decline is keyed by the server
        // and never by the name drawn beside it (must-do 6).
        .saying(
            "theUntrackedServerRow",
            Said::Value(Verbatim::new(untracked.server_row.clone())),
        )
        .saying(
            "theUntrackedSays",
            Said::Prose(Prose::new(IS_RUNNING_WORK_SMOOZY_IS_NOT_TRACKING)),
        )
        .saying(
            "theUntrackedRendering",
            Said::Value(Verbatim::new(untracked.rendering.to_string())),
        )
        .saying("renderingWord", Said::Prose(Prose::new(RENDERING_WORD)))
        .saying(
            "theUntrackedWaiting",
            Said::Value(Verbatim::new(untracked.waiting.to_string())),
        )
        .saying("waitingWord", Said::Prose(Prose::new(WAITING_WORD)))
        .saying("theUntrackedSince", Said::Prose(Prose::new(since)))
        .saying(
            "theUntrackedCost",
            Said::Prose(Prose::new(IF_YOU_LEAVE_IT)),
        )
        .saying("trackIt", Said::Prose(Prose::new(TRACK_IT)))
        .saying("ignore", Said::Prose(Prose::new(IGNORE)));
    if let Some(badge) = &untracked.badge {
        event = event.saying(
            "theUntrackedBadge",
            Said::Value(Verbatim::new(badge.clone())),
        );
    }
    event
}

fn the_part_is_said(event: Event, part: Option<&ThePartSubmission>) -> Event {
    let Some(part) = part else {
        return event;
    };
    let mut event = event
        .saying(
            "thePartConfirmed",
            Said::Value(Verbatim::new(part.confirmed.to_string())),
        )
        .saying("ofWord", Said::Prose(Prose::new(OF_WORD)))
        .saying(
            "thePartOf",
            Said::Value(Verbatim::new(part.of.to_string())),
        )
        .saying(
            "thePartSays",
            Said::Prose(Prose::new(OF_SCENES_ARE_CONFIRMED)),
        )
        .saying(
            "thePartSecond",
            Said::Prose(Prose::new(THE_CONNECTION_DROPPED_PART_WAY)),
        )
        .saying(
            "thePartServer",
            Said::Value(Verbatim::new(part.server.clone())),
        )
        .saying(
            "thePartSubmission",
            Said::Value(Verbatim::new(part.submission.clone())),
        )
        .saying(
            "thePartSubmittedName",
            Said::Prose(Prose::new(SUBMITTED_LIST)),
        )
        .saying(
            "thePartNotSubmittedName",
            Said::Prose(Prose::new(NOT_SUBMITTED_LIST)),
        )
        .saying(
            "thePartNotKnownName",
            Said::Prose(Prose::new(NOT_KNOWN_LIST)),
        )
        .saying(
            "thePartNotKnownWhy",
            Said::Prose(Prose::new(SMOOZY_COULD_NOT_READ_THE_SERVER)),
        )
        .saying(
            "thePartControl",
            Said::Prose(Prose::new(SUBMIT_THE_REMAINING_SCENES)),
        )
        .saying(
            "thePartServerRow",
            Said::Value(Verbatim::new(part.the_servers_row.clone())),
        )
        .saying(
            "thePartCanGo",
            Said::Value(Verbatim::new(if part.it_can_go { "yes" } else { "" })),
        )
        .saying(
            "needsAConnectionTo",
            Said::Prose(Prose::new(NEEDS_A_CONNECTION_TO)),
        )
        .saying("aFullStop", Said::Prose(Prose::new(A_FULL_STOP)))
        .saying("hide", Said::Prose(Prose::new(HIDE)));
    for (prefix, scenes, count) in [
        ("partSubmitted", &part.submitted, "partSubmittedCount"),
        ("partNotSubmitted", &part.not_submitted, "partNotSubmittedCount"),
        ("partNotKnown", &part.not_known, "partNotKnownCount"),
    ] {
        for (at, scene) in scenes.iter().enumerate() {
            event = event.saying(a_name(prefix, at, ""), Said::Value(Verbatim::new(scene.clone())));
        }
        event = event.saying(count, Said::Value(Verbatim::new(scenes.len().to_string())));
    }
    event
}

// ───────────────────────────────────────────────────────────────────────────
// The entry, which both screens draw
// ───────────────────────────────────────────────────────────────────────────
//
// **Design draws one entry and the two screens fill it.** The queue's is a job
// that is running or waiting; the history's is the same shape with the ending
// at the top of it and the two ways to render it again lower down. **So the
// composition is here, once**, and `the_history.rs` adds its own half rather
// than copying this one — the fact-in-two-places fault this stage keeps
// meeting, refused by there being one composer.

pub(crate) fn the_entry_is_said(event: Event, entry: Option<&TheJobEntry>) -> Event {
    let Some(entry) = entry else {
        return event;
    };
    let mut event = event
        .saying("entryName", Said::Value(Verbatim::new(entry.name.clone())))
        .saying("entryJob", Said::Value(Verbatim::new(entry.job.clone())))
        .saying(
            "entryServer",
            Said::Value(Verbatim::new(entry.server.clone())),
        )
        .saying("entryBlend", Said::Value(Verbatim::new(entry.blend.clone())))
        .saying(
            "entryStatusIs",
            Said::Value(Verbatim::new(entry.status_is)),
        )
        .saying("downloadAll", Said::Prose(Prose::new(DOWNLOAD_ALL)))
        .saying("downloadAllTo", Said::Prose(Prose::new(DOWNLOAD_ALL_TO)))
        .saying(
            "filesOnTheServer",
            Said::Prose(Prose::new(FILES_ON_THE_SERVER)),
        )
        .saying(
            "filesOnYourComputer",
            Said::Prose(Prose::new(FILES_ON_YOUR_COMPUTER)),
        );

    // **Design: Cancel sits where Stop sits**, and no job is ever both. The
    // act travels beside the label, so the page names nothing (must-do 5).
    if let Some(act) = entry.stop_or_cancel {
        let (label, act) = match act {
            TheStopOrCancel::Stop => (STOP, TheActOnAJob::Stop),
            TheStopOrCancel::Cancel => (CANCEL, TheActOnAJob::Cancel),
        };
        event = event
            .saying("entryStopOrCancel", Said::Prose(Prose::new(label)))
            .saying("entryStopOrCancelAct", Said::Value(Verbatim::new(act.name())));
    }

    if let Some(badge) = &entry.badge {
        event = event.saying("entryBadge", Said::Value(Verbatim::new(badge.clone())));
    }
    match &entry.scene {
        Some(scene) => {
            event = event.saying("entryScene", Said::Value(Verbatim::new(scene.clone())));
        }
        // **Design: the field says so rather than going blank.** A field that
        // disappeared reads the same as a scene the user did not notice.
        None => {
            event = event.saying("entryNoScene", Said::Prose(Prose::new(NO_SCENE_NAMED)));
        }
    }
    // **`Queue & History` N17: a reach-back is not offered where the machine is gone, and one
    // line sits where it was** — Design's own answer, absent rather than
    // greyed.
    if entry.reach_backs {
        event = event.saying("entryReachBacks", Said::Value(Verbatim::new("yes")));
        // **Not connected: *Files on the server* is not drawn** — the
        // downloads are Download's and wait for the connection themselves, so
        // they stay ([`TheJobEntry::the_server_is_not_connected`]).
        if entry.the_server_is_not_connected {
            event = event.saying("entryNotConnected", Said::Value(Verbatim::new("yes")));
        }
        // **`Job files and outputs` P21 and `Queue & History` Y8: a job with no
        // folder yet offers nothing that reaches into one**, and a waiting one
        // says why in the downloads' place (`Design/Queue & History`, *The
        // job's name, and its folder*). The Creator's second notes, piece 4's
        // queue half.
        match entry.the_folder {
            TheFolderOnTheEntry::ItHasOne => {}
            TheFolderOnTheEntry::ItIsMadeWhenTheRenderStarts => {
                event = event.saying("entryNoFolder", Said::Value(Verbatim::new("yes"))).saying(
                    "theFolderIsMadeWhenTheRenderStarts",
                    Said::Prose(Prose::new(
                        crate::surface::transfers::words::THE_FOLDER_IS_MADE_WHEN_THE_RENDER_STARTS,
                    )),
                );
            }
            TheFolderOnTheEntry::ItHasNotArrived => {
                event = event.saying("entryNoFolder", Said::Value(Verbatim::new("yes")));
            }
        }
    } else {
        event = event.saying(
            "theServerIsGone",
            Said::Prose(Prose::new(THE_SERVER_IS_GONE_AND_ITS_FILES_WITH_IT)),
        );
    }

    for (at, part) in entry.status.iter().enumerate() {
        event = a_part_is_said_under(event, "entryStatus", at, "", part);
    }
    event = event.saying(
        "entryStatusCount",
        Said::Value(Verbatim::new(entry.status.len().to_string())),
    );

    if let Some(progress) = &entry.progress {
        event = event
            .saying("renderingNow", Said::Prose(Prose::new(if progress.simulating { SIMULATING } else { RENDERING_NOW })))
            .saying("wholeJob", Said::Prose(Prose::new(WHOLE_JOB)))
            .saying(
                "progNowCount",
                Said::Value(Verbatim::new(progress.now.len().to_string())),
            );
        // **Each bar only where it shows a percentage** — absent stays absent
        // (`Queue & History` R4, `Queue & History` B4). Piece 6b.
        if let Some(at) = progress.now_at {
            event = event.saying("progNowAt", Said::Value(Verbatim::new(at.to_string())));
        }
        if let Some(whole) = progress.whole {
            event = event.saying("progWhole", Said::Value(Verbatim::new(whole.to_string())));
        }
        for (at, part) in progress.now.iter().enumerate() {
            event = a_part_is_said_under(event, "progNow", at, "", part);
        }
    }

    for (at, (name, value)) in entry.readings.iter().enumerate() {
        event = event
            .saying(a_name("reading", at, "Name"), Said::Prose(Prose::new(*name)))
            .saying(a_name("reading", at, "Value"), Said::Value(Verbatim::new(value.clone())));
    }
    event = event.saying(
        "readingCount",
        Said::Value(Verbatim::new(entry.readings.len().to_string())),
    );

    if !entry.marks.is_empty() {
        event = event.saying("aboutThisJob", Said::Prose(Prose::new(ABOUT_THIS_JOB)));
    }
    for (at, mark) in entry.marks.iter().enumerate() {
        event = event.saying(a_name("mark", at, "Said"), Said::Prose(Prose::new(mark.said)));
        event = the_parts_are_said(event, "mark", at, "Part", &mark.parts);
        if let Some(way) = mark.way {
            event = event.saying(a_name("mark", at, "Way"), Said::Prose(Prose::new(way)));
        }
        if mark.bold > 0 {
            event = event.saying(a_name("mark", at, "Bold"), Said::Value(Verbatim::new(mark.bold.to_string())));
        }
    }
    event = event.saying(
        "markCount",
        Said::Value(Verbatim::new(entry.marks.len().to_string())),
    );

    if !entry.writes.is_empty() {
        event = event.saying(
            "whatThisJobWrites",
            Said::Prose(Prose::new(WHAT_THIS_JOB_WRITES)),
        );
    }
    for (at, write) in entry.writes.iter().enumerate() {
        event = event.saying(a_name("write", at, "Said"), Said::Prose(Prose::new(write.said)));
        event = the_parts_are_said(event, "write", at, "Part", &write.parts);
        if write.bold > 0 {
            event = event.saying(a_name("write", at, "Bold"), Said::Value(Verbatim::new(write.bold.to_string())));
        }
    }
    event = event.saying(
        "writeCount",
        Said::Value(Verbatim::new(entry.writes.len().to_string())),
    );

    for (at, held) in entry.held.iter().enumerate() {
        event = event.saying(a_name("held", at, "Note"), Said::Prose(Prose::new(held.note)));
        // **A box with no answers sends none** — a job held for the driver
        // update (`Queue & History` H12). Held-job piece B.
        if !held.second_answer.is_empty() {
            event = event.saying(a_name("held", at, "Answer"), Said::Prose(Prose::new(held.second_answer)));
        }
        event = the_parts_are_said(event, "held", at, "Said", &held.said);
        if !held.second.is_empty() {
            event = event.saying(a_name("held", at, "Second"), Said::Prose(Prose::new(held.second)));
        }
        event = the_parts_are_said(event, "held", at, "First", &held.first_answer);
    }
    event = event.saying(
        "heldCount",
        Said::Value(Verbatim::new(entry.held.len().to_string())),
    );

    if !entry.facts.is_empty() {
        event = event
            .saying("submittedWith", Said::Prose(Prose::new(SUBMITTED_WITH)))
            .saying("legendItalic", Said::Prose(Prose::new(THE_LEGEND_ITALIC)))
            .saying(
                "legendTheFiles",
                Said::Prose(Prose::new(THE_LEGEND_THE_FILES)),
            )
            .saying("legendBold", Said::Prose(Prose::new(THE_LEGEND_BOLD)))
            .saying("legendYours", Said::Prose(Prose::new(THE_LEGEND_YOURS)))
            .saying(
                "theFilesValueLead",
                Said::Prose(Prose::new(THE_FILES_VALUE_LEAD)),
            );
    }
    for (at, fact) in entry.facts.iter().enumerate() {
        event = event
            .saying(a_name("fact", at, "Name"), Said::Prose(Prose::new(fact.name)))
            .saying(a_name("fact", at, "Value"), Said::Value(Verbatim::new(fact.value.clone())));
        if let Some(was) = &fact.the_files_own {
            event = event.saying(a_name("fact", at, "Was"), Said::Value(Verbatim::new(was.clone())));
        }
        if fact.the_files {
            event = event.saying(a_name("fact", at, "Files"), Said::Value(Verbatim::new("yes")));
        }
        if fact.neither {
            event = event.saying(a_name("fact", at, "Neither"), Said::Value(Verbatim::new("yes")));
        }
        if let Some(beside) = &fact.beside {
            event = event.saying(a_name("fact", at, "Beside"), Said::Value(Verbatim::new(beside.clone())));
        }
        for (line, under) in fact.under.iter().enumerate() {
            event = event.saying(a_name("fact", at, &format!("Under{line}")), Said::Prose(Prose::new(*under)));
        }
        if !fact.under.is_empty() {
            event = event.saying(
                a_name("fact", at, "UnderCount"),
                Said::Value(Verbatim::new(fact.under.len().to_string())),
            );
        }
        // **The same act as *Files on the server*, so offered only where that
        // control is**: the machine reachable, connected, and the job's folder
        // known.
        // A known folder stays copyable where it cannot open Files. This is
        // independent of connection state; the page never guesses from words.
        if fact.name == WRITING_TO_FACT && fact.value != NOT_KNOWN {
            event = event.saying(a_name("fact", at, "IsPath"), Said::Value(Verbatim::new("yes")));
        }
        if fact.opens_files_on_the_server
            && entry.reach_backs
            && !entry.the_server_is_not_connected
            && entry.the_folder == TheFolderOnTheEntry::ItHasOne
        {
            event = event.saying(a_name("fact", at, "OpensFiles"), Said::Value(Verbatim::new("yes")));
        }
    }
    event = event.saying(
        "factCount",
        Said::Value(Verbatim::new(entry.facts.len().to_string())),
    );

    // **The control that opens the record**, and on a rendering entry the
    // ordinary stream's newest three lines beside it, each a value (M13b's one
    // narrowing; the Creator's notes on the test copy, piece 8).
    if entry.blender_output {
        event = event.saying("blenderOutput", Said::Prose(Prose::new(BLENDER_OUTPUT)));
        for (at, line) in entry.lines.iter().enumerate() {
            event = event.saying(a_name("entryLine", at, ""), Said::Value(Verbatim::new(line.clone())));
        }
        if !entry.lines.is_empty() {
            event = event.saying("entryLineCount", Said::Value(Verbatim::new(entry.lines.len().to_string())));
        }
    }

    event = event.saying("previewUnavailable", Said::Prose(Prose::new(PREVIEW_NO_LONGER_AVAILABLE)));
    event = the_check_is_said(event, entry.the_check.as_ref());
    if let Some(preview) = &entry.preview {
        event = the_preview_is_said(event, preview, "");
    }

    event = the_telegram_group_is_said(event, entry.telegram.as_ref());
    if let Some(downloading) = &entry.downloading {
        event = event
            // **The switch's own words name it, with no heading over it**:
            // Render Settings' *Download frames as they render*, so the
            // switch is the first line beside *Telegram* (the Creator's
            // twenty-fifth notes, note 1).
            .saying(
                "framesAsTheyRender",
                Said::Prose(Prose::new(crate::surface::render_settings::words::DOWNLOAD_AS_THEY_RENDER)),
            )
            .saying(
                "downloadingOn",
                Said::Value(Verbatim::new(if downloading.on { "yes" } else { "no" })),
            )
            .saying(
                "downloadingNarrowing",
                Said::Value(Verbatim::new(downloading.narrowing.clone())),
            )
            // **No narrowing is every format** (`Download` N3), said in words
            // rather than as an empty value. The 38's piece 9.
            .saying("downloadingEverything", Said::Prose(Prose::new(ALL_FORMATS)))
            .saying(
                "downloadingDone",
                Said::Value(Verbatim::new(downloading.done.to_string())),
            )
            .saying(
                "downloadingOf",
                Said::Value(Verbatim::new(downloading.of.to_string())),
            )
            .saying("ofWord", Said::Prose(Prose::new(OF_WORD)))
            .saying("downloadedWord", Said::Prose(Prose::new(DOWNLOADED_WORD)));
    }

    event
}

/// **A preview's words and values, each key under `prefix`** — none for the
/// entry's own, `ff` for the check's picture on its pair's entry
/// ([`the_check_is_said`]), whose keys the page reads through the same
/// builder.
fn the_preview_is_said(mut event: Event, preview: &ThePreview, prefix: &str) -> Event {
    let key = |name: &str| crate::surface::settings::a_kept_name(format!("{prefix}{name}"));
    event = event
        .saying(key("frameHeading"), Said::Prose(Prose::new(FRAME_PREVIEW)))
        .saying(key("notColourAccurate"), Said::Prose(Prose::new(NOT_COLOUR_ACCURATE)));
    if preview.nothing_to_show {
        return event.saying(key("previewNothing"), Said::Prose(Prose::new(NO_FRAME_FINISHED)));
    }
    event = event
        .saying(key("previewFrame"), Said::Value(Verbatim::new(preview.frame.clone())))
        .saying(key("previewWhich"), Said::Prose(Prose::new(preview.which)));
    if let Some(layer) = &preview.layer {
        event = event.saying(key("previewLayer"), Said::Value(Verbatim::new(layer.clone())));
    }
    if let Some(picture) = &preview.picture {
        event = event.saying(key("previewPicture"), Said::Value(Verbatim::new(picture.clone())));
    }
    // **The arrows, the dropdown and the line where the pick did not
    // render** — their words; the stepping itself is the page's alone
    // and reads only held copies (P2).
    event = event
        .saying(key("frameWord"), Said::Prose(Prose::new(FRAME_HEADING)))
        .saying(key("previousFrame"), Said::Prose(Prose::new(PREVIOUS_FRAME)))
        .saying(key("nextFrame"), Said::Prose(Prose::new(NEXT_FRAME)))
        .saying(key("viewLayer"), Said::Prose(Prose::new(VIEW_LAYER)))
        .saying(key("showLast"), Said::Prose(Prose::new(SHOW_LAST)))
        .saying(key("showingFrame"), Said::Prose(Prose::new(SHOWING_FRAME)))
        .saying(key("frameInvalid"), Said::Prose(Prose::new(ENTER_A_WHOLE_FRAME)))
        .saying(key("frameUnavailable"), Said::Prose(Prose::new(FRAME_NOT_AVAILABLE)))
        .saying(key("enlargePreview"), Said::Prose(Prose::new(ENLARGE_PREVIEW)))
        .saying(key("previewClose"), Said::Prose(Prose::new(PREVIEW_CLOSE)))
        .saying(key("didNotRenderOnThisFrame"), Said::Prose(Prose::new(DID_NOT_RENDER_ON_THIS_FRAME)));
    if preview.follows {
        event = event.saying(key("showLatest"), Said::Prose(Prose::new(SHOW_LATEST)));
    }
    // **Every frame held and its View Layers** (P4, P10a; the
    // Creator's seventh notes, note 4): values only — the frame, each
    // copy's layer name and place, and which copy is the main one.
    event = event.saying(
        key("previewHeldCount"),
        Said::Value(Verbatim::new(preview.frames_held.len().to_string())),
    );
    for (at, held) in preview.frames_held.iter().enumerate() {
        event = event
            .saying(key(&format!("previewHeldFrame{at}")), Said::Value(Verbatim::new(held.frame.clone())))
            .saying(
                key(&format!("previewHeldLayerCount{at}")),
                Said::Value(Verbatim::new(held.layers.len().to_string())),
            );
        for (one, layer) in held.layers.iter().enumerate() {
            if let Some(name) = &layer.name {
                event = event.saying(
                    key(&format!("previewHeld{at}Layer{one}")),
                    Said::Value(Verbatim::new(name.clone())),
                );
            }
            // **Where a picture that is no one layer's came from**, in
            // words; a node's own name is the value above (the
            // Creator's tenth notes, note 2).
            if let Some(source) = layer.source {
                event = event.saying(key(&format!("previewHeld{at}Source{one}")), Said::Prose(Prose::new(source)));
            }
            if !layer.place.is_empty() {
                event = event.saying(
                    key(&format!("previewHeld{at}Place{one}")),
                    Said::Value(Verbatim::new(layer.place.clone())),
                );
            }
            if layer.main {
                event = event.saying(
                    key(&format!("previewHeldMain{at}")),
                    Said::Value(Verbatim::new(one.to_string())),
                );
            }
        }
    }
    event
}

/// The word beside each server's own count in the clear-all box, and on the
/// clear box's go-ahead. Its one home is here, where both screens' box is said.
pub(crate) const ENTRIES_WORD: &str = "entries";
pub(crate) const ENTRY_WORD: &str = "entry";

/// **The box that asks — a statement ending in a named button, never a question
/// with Yes and No.** Design says so for every one of the five on these two
/// screens, and the shape is `Servers`' own for a removal.
pub(crate) fn the_box_is_said(event: Event, asking: Option<&TheBoxThatAsks>) -> Event {
    let Some(asking) = asking else {
        return event;
    };
    let mut event = event
        .saying("boxWhich", Said::Value(Verbatim::new(asking.which)))
        .saying("boxKeep", Said::Prose(Prose::new(asking.keep)))
        .saying(
            "boxGrave",
            Said::Value(Verbatim::new(if asking.grave { "yes" } else { "no" })),
        )
        .saying(
            "boxGoAct",
            Said::Value(Verbatim::new(asking.go_ahead_act.name())),
        )
        .saying(
            "boxHeadCount",
            Said::Value(Verbatim::new(asking.heading.len().to_string())),
        )
        .saying(
            "boxSaidCount",
            Said::Value(Verbatim::new(asking.said.len().to_string())),
        )
        .saying(
            "boxGoCount",
            Said::Value(Verbatim::new(asking.go_ahead.len().to_string())),
        );
    for (at, part) in asking.heading.iter().enumerate() {
        event = a_part_is_said_under(event, "boxHead", at, "", part);
    }
    for (at, part) in asking.go_ahead.iter().enumerate() {
        event = a_part_is_said_under(event, "boxGo", at, "", part);
    }
    for (at, (server, how_many)) in asking.per_server.iter().enumerate() {
        event = event
            .saying(a_name("boxServer", at, ""), Said::Value(Verbatim::new(server.clone())))
            .saying(
                a_name("boxServer", at, "Entries"),
                Said::Value(Verbatim::new(how_many.to_string())),
            )
            .saying(
                a_name("boxServer", at, "EntriesWord"),
                Said::Prose(Prose::new(by_count(*how_many, ENTRY_WORD, ENTRIES_WORD))),
            );
    }
    if !asking.per_server.is_empty() {
        event = event.saying(
            "boxServerCount",
            Said::Value(Verbatim::new(asking.per_server.len().to_string())),
        );
    }
    for (at, sentence) in asking.said.iter().enumerate() {
        let parts: Vec<APart> = sentence.iter().filter(|part| !part.is_empty()).cloned().collect();
        event = the_parts_are_said(event, "boxSaid", at, "Part", &parts);
    }
    event
}

/// **The five boxes this screen raises**, each composed from Design's own
/// words. **Every one is a statement and a named button.**
///
/// **`frames_already_rendered` is `None` where nothing Smoozy reads holds the
/// count** — which is every entry until a render worker reports live figures
/// (`Queue & History` R4) — **and the sentence then carries no figure** rather
/// than a made-up one: *Frames already rendered stay on the server…*, the
/// Servers screen's own statement. Step 10c's S3, the first piece to raise the
/// box.
pub fn the_stop_box(frames_already_rendered: Option<u32>) -> TheBoxThatAsks {
    let what_stays = match frames_already_rendered {
        Some(frames) => vec![
            APart::words(THE_WORD_THE),
            APart::figure(
                frames.to_string(),
                by_count(frames, FRAME_ALREADY_RENDERED_STAYS, FRAMES_ALREADY_RENDERED_STAY),
            ),
        ],
        None => vec![APart::words(FRAMES_ALREADY_RENDERED_STAY_WITH_NO_COUNT)],
    };
    TheBoxThatAsks {
        which: "stop",
        heading: vec![APart::words(STOP_THIS_JOB_QUESTION)],
        said: vec![vec![APart::words(THE_FRAME_IT_IS_ON_MAY_NOT_BE_SAVED)], what_stays],
        keep: KEEP_RENDERING,
        go_ahead: vec![APart::words(STOP)],
        grave: true,
        go_ahead_act: TheActOnAJob::StopTheJob,
        per_server: Vec::new(),
    }
}

pub fn the_cancel_box() -> TheBoxThatAsks {
    TheBoxThatAsks {
        which: "cancel",
        heading: vec![APart::words(CANCEL_THIS_JOB_QUESTION)],
        said: vec![
            vec![APart::words(NO_FRAMES_ARE_LOST)],
            vec![APart::words(ITS_SETTINGS_GO_TOO)],
        ],
        keep: KEEP_THE_JOB,
        go_ahead: vec![APart::words(CANCEL_THE_JOB)],
        grave: true,
        go_ahead_act: TheActOnAJob::CancelTheJob,
        per_server: Vec::new(),
    }
}

/// **The group's box.** `rendering` is the member rendering now, where one is;
/// **where none is** — a group queued behind another job — the box says none
/// of them has started and no frames are lost, and the heading's count is the
/// same (must-do 11, and `Design/Queue & History § The queue list`).
pub fn the_group_cancel_box(jobs: u32, rendering: Option<&str>) -> TheBoxThatAsks {
    TheBoxThatAsks {
        which: "group-cancel",
        heading: vec![
            APart::words(CANCEL_ALL_QUESTION_LEAD),
            APart::figure(jobs.to_string(), by_count(jobs, JOB_QUESTION_TRAIL, JOBS_QUESTION_TRAIL)),
        ],
        said: match rendering {
            Some(rendering) => vec![
                // **Two pieces of prose in one paragraph, and the second is the
                // stop box's own constant** (R3). The page joins the parts of a
                // paragraph with a space, so what the user reads has not moved.
                vec![
                    APart::said("", rendering, IS_RENDERING_AND_WILL_BE_STOPPED),
                    APart::words(THE_FRAME_IT_IS_ON_MAY_NOT_BE_SAVED),
                ],
                vec![APart::words(THE_OTHERS_HAVE_NOT_STARTED)],
            ],
            None => vec![vec![APart::words(NONE_OF_THEM_HAS_STARTED)]],
        },
        keep: KEEP_THEM,
        go_ahead: vec![
            APart::words(CANCEL_ALL),
            APart::value(jobs.to_string()),
        ],
        grave: true,
        go_ahead_act: TheActOnAJob::CancelTheGroup,
        per_server: Vec::new(),
    }
}

pub fn the_dismiss_box(the_only_record_of: &str) -> TheBoxThatAsks {
    TheBoxThatAsks {
        which: "dismiss",
        heading: vec![APart::words(DISMISS_THIS_NOTICE_QUESTION)],
        said: vec![vec![APart::said(
            "",
            the_only_record_of,
            WAS_NEVER_STARTED_AND_HAS_NO_ENTRY,
        )]],
        keep: KEEP_THE_NOTICE,
        go_ahead: vec![APart::words(DISMISS)],
        grave: false,
        go_ahead_act: TheActOnAJob::DismissTheNotice,
        per_server: Vec::new(),
    }
}

/// **The first-frame check's held job** — `Design/Queue & History § What the
/// job writes`, with the job's own first frame as a value (must-do 8).
///
/// ***downloaded* only where it was** (`the_frame_was_downloaded`): step
/// 10c's pass before Vast on RunPod read *rendered and downloaded*
/// for two checks sent with downloads off, and no frame of either was on the
/// computer.
pub fn the_first_frame_is_held(
    first_frame: &str,
    frames_held: u32,
    the_frame_was_downloaded: bool,
) -> TheJobIsHeld {
    let is_rendered = if the_frame_was_downloaded { IS_RENDERED_AND_DOWNLOADED } else { IS_RENDERED };
    TheJobIsHeld {
        said: vec![APart::said(THE_FIRST_FRAME_LEAD, first_frame, is_rendered)],
        second: "",
        first_answer: vec![APart::said(
            RENDER_ALL,
            frames_held.to_string(),
            by_count(frames_held, FRAME_WORD, FRAMES_WORD),
        )],
        second_answer: CANCEL_IT,
        note: CANCELLING_LEAVES_NO_RECORD,
    }
}

/// **The first-frame check's held job where the count is not known** — a
/// Smoozy opened after the check was sent (step 10c's eleventh-pass fix
/// piece 8b). Design's pair, *Render all — 240 frames* and *Cancel*,
/// with the count dropped, because nothing Smoozy reads then holds it; and no
/// sentence naming the first frame, for the same reason (Rule 8). Rule 9,
/// *Short, too*: Design's own words, less the part that cannot be filled.
pub fn the_first_frame_is_held_with_no_count() -> TheJobIsHeld {
    TheJobIsHeld {
        said: vec![APart::words(WAITING_FOR_FIRST_FRAME_APPROVAL)],
        second: "",
        first_answer: vec![APart::words(RENDER_ALL_WITH_NO_COUNT)],
        second_answer: CANCEL_IT,
        note: CANCELLING_LEAVES_NO_RECORD,
    }
}

/// **The same check's words on the status line** — *Needs your answer — frame
/// 1001 rendered, …* — with the job's own first frame as a value.
pub fn the_first_frame_was_rendered(first_frame: &str) -> APart {
    APart::said(THE_FIRST_FRAME_RENDERED_LEAD, first_frame, THE_FIRST_FRAME_RENDERED_TRAIL)
}


// ───────────────────────────────────────────────────────────────────────────
// What the two screens share, exported for `the_history.rs`
// ───────────────────────────────────────────────────────────────────────────

/// The words the history entry needs from this file, so that one word has one
/// home. **Design draws one entry shape and the two screens fill it**, so a
/// second copy of these in the history file would be the fact-in-two-places
/// fault this stage keeps meeting.
pub(crate) struct TheSharedWords;

impl TheSharedWords {
    pub(crate) const ABOUT_THIS_JOB: &'static str = ABOUT_THIS_JOB;
    pub(crate) const BLENDER_OUTPUT: &'static str = BLENDER_OUTPUT;
    pub(crate) const DOWNLOAD_ALL: &'static str = DOWNLOAD_ALL;
    pub(crate) const DOWNLOAD_ALL_TO: &'static str = DOWNLOAD_ALL_TO;
    pub(crate) const FILES_ON_THE_SERVER: &'static str = FILES_ON_THE_SERVER;
    pub(crate) const FILES_ON_YOUR_COMPUTER: &'static str = FILES_ON_YOUR_COMPUTER;
    pub(crate) const NO_SCENE_NAMED: &'static str = NO_SCENE_NAMED;
    pub(crate) const NOT_COLOUR_ACCURATE: &'static str = NOT_COLOUR_ACCURATE;
    pub(crate) const NO_FRAME_FINISHED: &'static str = NO_FRAME_FINISHED;
    pub(crate) const FRAME_HEADING: &'static str = FRAME_HEADING;
    pub(crate) const SUBMITTED_WITH: &'static str = SUBMITTED_WITH;
    pub(crate) const THE_LEGEND_ITALIC: &'static str = THE_LEGEND_ITALIC;
    pub(crate) const THE_LEGEND_THE_FILES: &'static str = THE_LEGEND_THE_FILES;
    pub(crate) const THE_LEGEND_BOLD: &'static str = THE_LEGEND_BOLD;
    pub(crate) const THE_LEGEND_YOURS: &'static str = THE_LEGEND_YOURS;
    pub(crate) const THE_FILES_VALUE_LEAD: &'static str = THE_FILES_VALUE_LEAD;
    pub(crate) const THE_SERVER_IS_GONE: &'static str = THE_SERVER_IS_GONE_AND_ITS_FILES_WITH_IT;
    pub(crate) const WHAT_THIS_JOB_WRITES: &'static str = WHAT_THIS_JOB_WRITES;
    pub(crate) const DOWNLOADED_WORD: &'static str = DOWNLOADED_WORD;
    pub(crate) const OF_WORD: &'static str = OF_WORD;
    pub(crate) const LATEST: &'static str = LATEST;
    pub(crate) const NEEDS_A_CONNECTION_TO: &'static str = NEEDS_A_CONNECTION_TO;
    pub(crate) const A_FULL_STOP: &'static str = A_FULL_STOP;
    pub(crate) const STOP: &'static str = STOP;
    pub(crate) const CANCEL: &'static str = CANCEL;
    pub(crate) const RENDERING_NOW: &'static str = RENDERING_NOW;
    pub(crate) const WHOLE_JOB: &'static str = WHOLE_JOB;
    pub(crate) const VRAM_WORD: &'static str = VRAM_WORD;
    pub(crate) const FREE_DISK: &'static str = FREE_DISK;
    pub(crate) const KEEP_RENDERING: &'static str = KEEP_RENDERING;
    pub(crate) const KEEP_THE_JOB: &'static str = KEEP_THE_JOB;
    pub(crate) const KEEP_THEM: &'static str = KEEP_THEM;
    pub(crate) const KEEP_THE_NOTICE: &'static str = KEEP_THE_NOTICE;
}

#[cfg(test)]
pub(crate) mod tests {
    /// **The 38's piece 10**: what the machine read, on the queue entry.
    mod piece_10 {
        use super::super::*;
        use crate::servers::the_machines_work::{WhatTheReadingSays, WhatThisRunHolds};
        use smoozy_job_files::holds::{the_hold_reason_for, WhatHoldsTheJob};
        use smoozy_queue_history::acting::adoption::TheMachinesWorkingState;
        use smoozy_queue_history::line::QueueState;
        use smoozy_queue_history::readings::{AJobAsTheMachineReadsIt, WhatTheEntryHasLive, WhatTheHoldSays};
        use smoozy_state::identities::{Identity, IdentityKind};
        use smoozy_state::observed::{Clock, Reading};

        fn a_record(named: &str) -> crate::record::LocalRecord {
            let path = crate::scratch_folders::a_folder(format!("smoozy-queue-piece-10-{named}-{}", std::process::id()));
            crate::record::LocalRecord::open_at(&path.join("local-record.sqlite3")).unwrap()
        }

        /// **One job on one server, in the state given**, as the period holds
        /// its reading.
        fn the_queue_holds(
            holds: &WhatThisRunHolds,
            server: &Identity,
            job: &Identity,
            state: QueueState,
            the_hold: Option<WhatTheHoldSays>,
        ) {
            let connection = holds.a_connection_begins(&server.as_text());
            holds.a_reading_was_taken(
                &server.as_text(),
                connection,
                Reading::taken(
                    TheMachinesWorkingState {
                        jobs: vec![AJobAsTheMachineReadsIt {
                            job: job.clone(),
                            state,
                            live: WhatTheEntryHasLive::NoReadings,
                            the_hold,
                        }],
                        the_admission_point_stands_at: None,
                        notes: Vec::new(),
                        the_history_copys_count: 0,
                        the_history_copys_entries: Vec::new(),
                    },
                    std::time::SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(1_754_700_000),
                    Clock::ThisComputer,
                ),
                None,
            );
        }

        fn what_the_reading_says(
            job_output_folder: &str,
            views: smoozy_job_files::reading::HowManyViewsTheSceneWrites,
        ) -> WhatTheReadingSays {
            let composed = crate::servers::the_machines_work::tests::a_recorded_reading(job_output_folder, &[1, 2, 3], views);
            let derived = composed.as_the_manifest_carries_it();
            WhatTheReadingSays {
                six: smoozy_job_files::holds::the_six_things(&derived.reading, &derived.map, &derived.inventory, job_output_folder),
                how_many_files: match &derived.inventory {
                    smoozy_job_files::inventory::WhatTheInventoryIs::It(it) => Some(it.the_count() as u64),
                    _ => None,
                },
                the_frames_cannot_be_counted: !matches!(
                    &derived.inventory,
                    smoozy_job_files::inventory::WhatTheInventoryIs::It(_)
                ),
                job_output_folder: job_output_folder.to_string(),
                more_than_one_view_layer_at: Default::default(),
            }
        }

        /// **The 38's piece 3.** A rendering job whose record holds a copy
        /// draws it as the *latest* frame — the same picture History draws as
        /// *last complete*.
        #[test]
        fn piece_3_a_rendering_entry_draws_the_latest_copy_held() {
            let record = a_record("piece-3-latest");
            let server = Identity::issue(IdentityKind::LocalServer).unwrap();
            let store = crate::record::job_records::JobRecords::over(record.connection());
            let made = crate::record::deletions::tests::a_record_in_history(&server, "Kitchen");
            store.write_the_provisional_record(&made).unwrap();
            let holds = WhatThisRunHolds::default();
            the_queue_holds(&holds, &server, &made.job, QueueState::Rendering { operation: Identity::issue(IdentityKind::Operation).unwrap() }, None);
            assert!(what_is_held(&holds, Some(&record), None).entry.unwrap().preview.is_none());
            crate::record::previews::a_derivative_is_owed(record.connection(), &made.job, 12, "op", None, true, 0, None).unwrap();
            crate::record::previews::the_copy_was_published(
                record.connection(),
                &made.job,
                12,
                None,
                "op",
                std::path::Path::new("/previews/a.png"),
                std::time::SystemTime::now(),
            )
            .unwrap();
            let preview = what_is_held(&holds, Some(&record), None).entry.unwrap().preview.unwrap();
            assert_eq!(preview.frame, "12");
            assert_eq!(preview.which, LATEST);
            assert!(preview.follows, "a queue entry's preview follows the newest frame");
            assert_eq!(preview.picture, Some(made.job.as_text()));
        }

        /// The seventeenth notes, piece 2: the held range keeps its own
        /// answers, beside every picture of the first-frame job, also after
        /// the record and the connection have been reopened.
        #[test]
        fn seventeenth_piece_2_the_waiting_entry_shows_the_first_jobs_pictures_after_reopening() {
            for layers in [1, 3] {
                let record = a_record(&format!("seventeenth-piece-2-{layers}"));
                let path = record.connection().path().unwrap().to_owned();
                let server = Identity::issue(IdentityKind::LocalServer).unwrap();
                let first = crate::record::deletions::tests::a_record_in_history(&server, "First frame");
                let mut waiting = crate::record::deletions::tests::a_record_in_history(&server, "Full range");
                waiting.the_render_start_has_been_learned = false;
                let store = crate::record::job_records::JobRecords::over(record.connection());
                store.write_the_provisional_record(&first).unwrap();
                store.write_the_provisional_record(&waiting).unwrap();
                for place in 0..layers {
                    let layer = format!("Layer {place}").into_bytes();
                    crate::record::previews::a_derivative_is_owed(
                        record.connection(), &first.job, 1001, "first-op", Some(&layer), place == 0, place, None,
                    ).unwrap();
                    crate::record::previews::the_copy_was_published(
                        record.connection(), &first.job, 1001, Some(&layer), "first-op",
                        std::path::Path::new("/previews/first.png"), std::time::SystemTime::now(),
                    ).unwrap();
                }
                let check = |record: &crate::record::LocalRecord| {
                    let holds = WhatThisRunHolds::default();
                    let instruction = smoozy_rendering::submission::the_hold_instruction(&waiting.job, &first.job);
                    the_queue_holds(&holds, &server, &waiting.job,
                        QueueState::Held { hold: Identity::issue(IdentityKind::Operation).unwrap() },
                        Some(WhatTheHoldSays { reason: instruction.reason.to_string(), held_behind: Some(first.job.clone()) }),
                    );
                    let screen = what_is_held(&holds, Some(record), Some(&waiting.job.as_text()));
                    let entry = screen.entry.unwrap();
                    assert_eq!(entry.job, waiting.job.as_text(), "approval belongs to the waiting job");
                    assert_eq!(entry.held.len(), 1);
                    assert_eq!(entry.held[0].first_answer, vec![APart::words(RENDER_ALL_WITH_NO_COUNT)]);
                    assert_eq!(entry.held[0].second_answer, CANCEL_IT);
                    let preview = entry.preview.expect("the first-frame picture beside the answers");
                    assert_eq!(preview.picture, Some(first.job.as_text()));
                    assert_eq!(preview.frame, "1001");
                    assert!(preview.follows);
                    assert_eq!(preview.which, LATEST);
                    assert_eq!(preview.frames_held.len(), 1);
                    assert_eq!(preview.frames_held[0].layers.len(), layers as usize);
                    for (place, layer) in preview.frames_held[0].layers.iter().enumerate() {
                        assert_eq!(layer.name.as_deref(), Some(format!("Layer {place}").as_str()));
                        assert_eq!(layer.place, place.to_string());
                    }
                };
                check(&record);
                drop(record);
                let reopened = crate::record::LocalRecord::open_at(std::path::Path::new(&path)).unwrap();
                check(&reopened);
                drop(reopened);
                let _ = std::fs::remove_dir_all(std::path::Path::new(&path).parent().unwrap());
            }
        }

        #[test]
        fn seventeenth_piece_2_again_the_saved_waiting_range_keeps_its_count_and_approval_identity() {
            for layers in [1, 3] {
                let record = a_record(&format!("seventeenth-piece-2-again-{layers}"));
                let path = record.connection().path().unwrap().to_owned();
                let server = Identity::issue(IdentityKind::LocalServer).unwrap();
                let first = crate::record::deletions::tests::a_record_in_history(&server, "First frame");
                let mut waiting = crate::record::deletions::tests::a_record_in_history(&server, "Full range");
                waiting.the_render_start_has_been_learned = false;
                let store = crate::record::job_records::JobRecords::over(record.connection());
                store.write_the_provisional_record(&first).unwrap();
                store.write_the_provisional_record(&waiting).unwrap();
                record.connection().execute("UPDATE job_record SET first_frame_purpose = 'range', first_frame_pair = ?1, first_frame_number = 1001, first_frame_range = '1001,1004,1010' WHERE job = ?2", rusqlite::params![first.job.as_text(),waiting.job.as_text()]).unwrap();
                record.connection().execute("UPDATE job_record SET first_frame_purpose = 'check', first_frame_pair = ?1, first_frame_number = 1001, first_frame_range = '1001,1004,1010' WHERE job = ?2", rusqlite::params![waiting.job.as_text(),first.job.as_text()]).unwrap();
                for place in 0..layers {
                    let layer = format!("Layer {place}").into_bytes();
                    crate::record::previews::a_derivative_is_owed(
                        record.connection(), &first.job, 1001, "first-op", Some(&layer), place == 0, place, None,
                    ).unwrap();
                    crate::record::previews::the_copy_was_published(
                        record.connection(), &first.job, 1001, Some(&layer), "first-op",
                        std::path::Path::new("/previews/first.png"), std::time::SystemTime::now(),
                    ).unwrap();
                }
                let check = |record: &crate::record::LocalRecord| {
                    let holds = WhatThisRunHolds::default();
                    let instruction = smoozy_rendering::submission::the_hold_instruction(&waiting.job, &first.job);
                    the_queue_holds(&holds, &server, &waiting.job,
                        QueueState::Held { hold: Identity::issue(IdentityKind::Operation).unwrap() },
                        Some(WhatTheHoldSays { reason: instruction.reason.to_string(), held_behind: Some(first.job.clone()) }),
                    );
                    let screen = what_is_held(&holds, Some(record), Some(&waiting.job.as_text()));
                    let entry = screen.entry.unwrap();
                    assert_eq!(entry.job, waiting.job.as_text(), "approval belongs to the waiting job");
                    assert_eq!(entry.held.len(), 1);
                    assert_eq!(entry.held[0].first_answer, the_first_frame_is_held("1001", 3, false).first_answer);
                    // **The nineteenth notes, note 7: the question names the
                    // full range, and the pair is one row naming both parts.**
                    assert_eq!(entry.held[0].said[1].value, "1001,1004,1010");
                    let row = &screen.lines[0].rows[0];
                    assert_eq!(row.the_pair[0].what.value, "1001");
                    assert_eq!(row.the_pair[1].what.value, "1001,1004,1010");
                    assert_eq!(row.the_pair[1].state.lead, PART_NOT_STARTED);
                    let history = crate::surface::the_history::what_is_kept(Some(record), Some(&first.job.as_text()));
                    assert_eq!(history.entry.unwrap().entry.the_check.unwrap().show_in_queue, Some(waiting.job.as_text()));
                    let checks_row = history.rows.iter().find(|row| row.job == first.job.as_text()).unwrap();
                    assert_eq!(checks_row.the_check[0].what.value, "1001");
                    assert_eq!(checks_row.in_place_of_the_ending, Some(IN_QUEUE));
                    assert_eq!(entry.held[0].second_answer, CANCEL_IT);
                    // **The picture sits in the question box, and the side
                    // column draws none.**
                    assert!(entry.preview.is_none());
                    let the_check = entry.the_check.as_ref().unwrap();
                    assert!(the_check.in_the_question);
                    let preview = the_check.picture.clone().expect("the first-frame picture beside the answers");
                    assert_eq!(preview.picture, Some(first.job.as_text()));
                    assert_eq!(preview.frame, "1001");
                    assert!(preview.follows);
                    assert_eq!(preview.which, LATEST);
                    assert_eq!(preview.frames_held.len(), 1);
                    assert_eq!(preview.frames_held[0].layers.len(), layers as usize);
                    for (place, layer) in preview.frames_held[0].layers.iter().enumerate() {
                        assert_eq!(layer.name.as_deref(), Some(format!("Layer {place}").as_str()));
                        assert_eq!(layer.place, place.to_string());
                    }
                };
                check(&record);
                drop(record);
                let reopened = crate::record::LocalRecord::open_at(std::path::Path::new(&path)).unwrap();
                check(&reopened);
                drop(reopened);
                let _ = std::fs::remove_dir_all(std::path::Path::new(&path).parent().unwrap());
            }
        }

        /// **The 38's piece 10, item 10.** A job sent without *Read the file*
        /// says *Writing to: not known* until its reading is filled in, and
        /// then *Writing to* its folder with the six statements under *What
        /// this job writes*.
        #[test]
        fn a_job_sent_without_read_the_file_draws_writing_to_and_the_six_statements_once_filled_in() {
            let record = a_record("writing-to");
            let server = Identity::issue(IdentityKind::LocalServer).unwrap();
            let store = crate::record::job_records::JobRecords::over(record.connection());
            let made = crate::record::deletions::tests::a_record_in_history(&server, "Kitchen");
            store.write_the_provisional_record(&made).unwrap();
            let holds = WhatThisRunHolds::default();
            the_queue_holds(&holds, &server, &made.job, QueueState::Rendering { operation: Identity::issue(IdentityKind::Operation).unwrap() }, None);
            let writing_to = |held: &TheQueueOnScreen| {
                held.entry
                    .as_ref()
                    .expect("the entry")
                    .facts
                    .iter()
                    .find(|fact| fact.name == WRITING_TO_FACT)
                    .map(|fact| fact.value.clone())
            };
            let before = what_is_held(&holds, Some(&record), None);
            assert_eq!(writing_to(&before).as_deref(), Some(NOT_KNOWN));
            assert!(before.entry.as_ref().unwrap().writes.is_empty());

            let folder = "/ephemeral/smoozy/jobs/kitchen";
            store
                .the_read_is_filled_in(
                    &made.job,
                    &smoozy_queue_history::record::WhatTheReadProduced {
                        main_output_or_the_jobs_folder: Some(folder.to_string()),
                        ..Default::default()
                    },
                )
                .unwrap();
            holds.a_reading_was_filled_in(
                &made.job.as_text(),
                what_the_reading_says(folder, smoozy_job_files::reading::HowManyViewsTheSceneWrites::One),
            );
            let after = what_is_held(&holds, Some(&record), None);
            assert_eq!(writing_to(&after).as_deref(), Some(folder));
            let writes = &after.entry.as_ref().unwrap().writes;
            assert_eq!(writes[0].parts[0], APart::words(WRITING_INTO));
            assert!(
                writes[0].parts.iter().any(|part| part.value == "3" && part.trail == "files"),
                "{writes:?}"
            );
            assert!(format!("{:?}", the_queue_says(&after)).contains("whatThisJobWrites"));
        }

        /// **The 38's piece 10, item 26.** A job held at the start of its
        /// render, whose hold record carries `Job files and outputs` H1's
        /// conditions, offers each as its pair — *Stop this job* first, the
        /// *no*, and *Continue anyway*, the *yes* — and leaves those two
        /// statements to the pair. **A reason no reader can name offers
        /// only *Cancel***, which is what a debug print of the conditions did.
        #[test]
        fn a_hold_carries_its_conditions_and_the_entry_draws_them_with_carry_on_anyway() {
            use crate::servers::the_machines_work::TheHeldJobsAnswer;
            let record = a_record("held");
            let server = Identity::issue(IdentityKind::LocalServer).unwrap();
            let store = crate::record::job_records::JobRecords::over(record.connection());
            let made = crate::record::deletions::tests::a_record_in_history(&server, "Kitchen");
            store.write_the_provisional_record(&made).unwrap();
            let hold = Identity::issue(IdentityKind::Operation).unwrap();
            let held_for = |reason: String| {
                let holds = WhatThisRunHolds::default();
                the_queue_holds(
                    &holds,
                    &server,
                    &made.job,
                    QueueState::Held { hold: hold.clone() },
                    Some(WhatTheHoldSays {
                        reason,
                        held_behind: None,
                    }),
                );
                holds
            };

            let holds = held_for(the_hold_reason_for(&[WhatHoldsTheJob::TheJobWillWriteNothingAtAll]));
            let entry = what_is_held(&holds, Some(&record), None).entry.expect("the entry");
            assert_eq!(entry.held.len(), 1);
            assert_eq!(entry.held[0].said, vec![APart::words(THIS_JOB_WRITES_NO_FILES)]);
            assert_eq!(entry.held[0].first_answer, vec![APart::words(STOP_THIS_JOB)]);
            assert_eq!(entry.held[0].second_answer, CARRY_ON_ANYWAY);
            assert_eq!(entry.stop_or_cancel, Some(TheStopOrCancel::Cancel));

            let both = held_for(the_hold_reason_for(&[
                WhatHoldsTheJob::ADestinationCannotBeBroughtUnderTheJobsFolder,
                WhatHoldsTheJob::TwoDestinationsAimedAtOneNameInOnePlace,
            ]));
            let entry = what_is_held(&both, Some(&record), None).entry.expect("the entry");
            assert_eq!(
                entry.held.iter().map(|pair| pair.said.clone()).collect::<Vec<_>>(),
                vec![
                    vec![APart::words(PART_OF_IT_LANDS_OUTSIDE_ITS_FOLDER)],
                    vec![APart::words(TWO_DESTINATIONS_COLLIDE)]
                ]
            );

            // **The debug print the render worker wrote until this piece.**
            let unnamed = held_for("[TheJobWillWriteNothingAtAll]".to_string());
            let entry = what_is_held(&unnamed, Some(&record), None).entry.expect("the entry");
            assert!(entry.held.is_empty(), "nothing to answer but Cancel");

            // **Which answer each control gives.**
            let reason = the_hold_reason_for(&[WhatHoldsTheJob::TheJobWillWriteNothingAtAll]);
            assert_eq!(the_answer_the_control_gives(true, Some(&reason)), TheHeldJobsAnswer::CancelIt);
            assert_eq!(the_answer_the_control_gives(false, Some(&reason)), TheHeldJobsAnswer::RenderAll);
            assert_eq!(
                the_answer_the_control_gives(true, Some("the first-frame check")),
                TheHeldJobsAnswer::RenderAll
            );
            assert_eq!(the_answer_the_control_gives(false, None), TheHeldJobsAnswer::CancelIt);
        }

        /// **Held-job piece B, `Queue & History` H12 and the drawing's
        /// `q-driver`: a job held for the driver update** reads *Waiting for
        /// driver update* on its row, in the held amber and with no grip, and
        /// on its entry's status line; its box names the job's Blender by its
        /// line, says what happens next and that it needs Smoozy connected,
        /// and **offers no answer**; *Cancel* sits where *Stop* sits. **Nothing
        /// on the list moves**, so *Move to the top of the queue* is never
        /// sent. The hold's reason decides it and nothing else: the same job
        /// held for the first-frame check reads *Needs your answer*.
        #[test]
        fn a_job_held_for_the_driver_is_drawn_as_q_driver_with_no_answers() {
            let record = a_record("driver");
            let server = Identity::issue(IdentityKind::LocalServer).unwrap();
            let store = crate::record::job_records::JobRecords::over(record.connection());
            let mut made = crate::record::deletions::tests::a_record_in_history(&server, "Kitchen — night");
            made.blender.pick.version = "5.2.1".to_string();
            store.write_the_provisional_record(&made).unwrap();
            let held_for = |reason: &str| {
                let holds = WhatThisRunHolds::default();
                the_queue_holds(
                    &holds,
                    &server,
                    &made.job,
                    QueueState::Held { hold: Identity::issue(IdentityKind::Operation).unwrap() },
                    Some(WhatTheHoldSays { reason: reason.to_string(), held_behind: None }),
                );
                what_is_held(&holds, Some(&record), None)
            };

            let screen = held_for(smoozy_rendering::submission::THE_DRIVER);
            let row = &screen.lines[0].rows[0];
            assert_eq!((row.state, row.state_is, row.grip), (Some(WAITING_FOR_DRIVER_UPDATE), "held", false));
            let entry = screen.entry.as_ref().expect("the entry");
            assert_eq!(entry.status, vec![APart::words(WAITING_FOR_DRIVER_UPDATE)]);
            assert_eq!(entry.status_is, "", "the held job's default amber dot");
            assert_eq!(entry.stop_or_cancel, Some(TheStopOrCancel::Cancel));
            assert_eq!(entry.held.len(), 1, "{:?}", entry.held);
            let the_box = &entry.held[0];
            assert_eq!(
                the_box.said,
                vec![APart::said(THE_JOBS_BLENDER_LEAD, "5.2", NEEDS_A_NEWER_NVIDIA_DRIVER_FOR_OPTIX)]
            );
            assert_eq!(the_box.second, SMOOZY_WILL_UPDATE_THE_DRIVER);
            assert_eq!(the_box.note, THIS_ONLY_HAPPENS_WHILE_CONNECTED);
            // **From 5.3 the box says what stops there** (`Rendering` L23a's
            // bullet of 2026-11-02), and a later Blender reads the same row.
            for (version, line) in [("5.3.0", "5.3"), ("5.4.0", "5.4")] {
                assert_eq!(
                    the_job_is_held_for_the_driver(version).said,
                    vec![APart::said(THE_JOBS_BLENDER_LEAD, line, "needs a newer NVIDIA driver for Cycles GPU rendering.")]
                );
            }
            assert_eq!(
                the_job_is_held_for_the_driver("5.2.3").said,
                vec![APart::said(THE_JOBS_BLENDER_LEAD, "5.2", "needs a newer NVIDIA driver for OptiX.")]
            );
            assert!(the_box.first_answer.is_empty() && the_box.second_answer.is_empty(), "no answers");

            let text = format!("{:?}", the_queue_says(&screen));
            assert!(text.contains("held0Said0Value") && text.contains("held0Note"), "{text}");
            assert!(!text.contains("held0Answer") && !text.contains("held0First0"), "an answer was sent: {text}");
            assert!(!text.contains("moveToTheTop"), "nothing on the list moves");
            assert!(!text.contains("Needs your answer"), "{text}");

            // **The reason decides**: the check's hold reads as it always has.
            let checked = held_for("the first-frame check");
            assert_eq!(checked.lines[0].rows[0].state, Some(WAITING_ON_YOU));
            assert_eq!(checked.entry.as_ref().unwrap().status, vec![APart::words(WAITING_ON_YOU)]);
        }

        /// **The 38's piece 10, item 38.** A scene that writes more than one
        /// view separately has no inventory (`Job files and outputs` I9a), and
        /// its entry says the one line — and no count of files.
        #[test]
        fn a_two_view_jobs_entry_draws_the_line() {
            let says = what_the_reading_says(
                "/ephemeral/smoozy/jobs/kitchen",
                smoozy_job_files::reading::HowManyViewsTheSceneWrites::MoreThanOneWrittenSeparately { how_many: 2 },
            );
            assert!(says.the_frames_cannot_be_counted);
            let writes = what_the_job_writes(&says, &[]);
            assert_eq!(
                writes.last().map(|write| write.said),
                Some(crate::surface::render_settings::words::MORE_THAN_ONE_VIEW)
            );
            assert!(
                !writes.iter().flat_map(|write| &write.parts).any(|part| part.trail == "files"),
                "no count: {writes:?}"
            );
            let one = what_the_reading_says(
                "/ephemeral/smoozy/jobs/kitchen",
                smoozy_job_files::reading::HowManyViewsTheSceneWrites::One,
            );
            assert!(what_the_job_writes(&one, &[])
                .iter()
                .all(|write| write.said != crate::surface::render_settings::words::MORE_THAN_ONE_VIEW));
        }

        /// **The Creator's eighth notes, note 4, and their answer of
        /// 2026-10-12: a video is not a pass at all.** Where the beauty reaches
        /// a File Output node and the output path writes a video, the entry
        /// names the two outputs and draws no twice line. **Where the picture
        /// is written twice, the line names the places the walk found** — the
        /// runner's ruling that day — with its opening words in the bold.
        #[test]
        fn frames_and_a_video_are_named_and_the_twice_line_names_its_places() {
            use crate::surface::render_settings::words as the_page;
            use smoozy_job_files::main_output::ThePlacesTheBeautyIsWritten;
            let mut says = what_the_reading_says(
                "/ephemeral/smoozy/jobs/kitchen",
                smoozy_job_files::reading::HowManyViewsTheSceneWrites::One,
            );
            says.six.the_same_picture_reaches_more_than_one_place = false;
            says.six.where_the_same_picture_is_written = ThePlacesTheBeautyIsWritten::default();
            says.six.the_render_reaches_frames_and_a_video = true;
            let writes = what_the_job_writes(&says, &[]);
            assert!(writes.iter().any(|write| write.said == the_page::FRAMES_AND_A_VIDEO), "{writes:?}");
            assert!(
                !writes.iter().any(|write| write.said.starts_with(the_page::THE_BEAUTY_WRITTEN)),
                "a twice line: {writes:?}"
            );

            // **Two File Output nodes and a video**: both lines, and the twice
            // line never names the output path.
            says.six.the_same_picture_reaches_more_than_one_place = true;
            says.six.where_the_same_picture_is_written =
                ThePlacesTheBeautyIsWritten { file_output_nodes: 2, their_inputs: 2, the_output_path: false };
            let writes = what_the_job_writes(&says, &[]);
            let twice = writes
                .iter()
                .find(|write| write.said == the_page::THE_BEAUTY_WRITTEN_TIMES[0])
                .expect("the twice line");
            assert_eq!(
                twice.parts,
                vec![
                    APart::words(the_page::BEFORE_THE_PLACES),
                    APart { lead: "two", value: String::new(), trail: the_page::FILE_OUTPUT_NODES_FULL_STOP },
                ]
            );
            assert!(writes.iter().any(|write| write.said == the_page::FRAMES_AND_A_VIDEO));

            // **A File Output node and an image output path**: the twice line,
            // naming those two, and no naming of two outputs.
            says.six.the_render_reaches_frames_and_a_video = false;
            says.six.where_the_same_picture_is_written =
                ThePlacesTheBeautyIsWritten { file_output_nodes: 1, their_inputs: 1, the_output_path: true };
            let writes = what_the_job_writes(&says, &[]);
            let twice = writes
                .iter()
                .find(|write| write.said == the_page::THE_BEAUTY_WRITTEN_TIMES[0])
                .expect("the twice line");
            assert_eq!(
                twice.parts,
                vec![
                    APart::words(the_page::BEFORE_THE_PLACES),
                    APart::words(the_page::A_FILE_OUTPUT_NODE),
                    APart::words(the_page::AND_THE_OUTPUT_PATH),
                ]
            );
            assert!(!writes.iter().any(|write| write.said == the_page::FRAMES_AND_A_VIDEO));
        }

        /// **The ninth notes, the E4 half of note 1: the duplicate-picture line is bold
        /// whole**, the way the drawing's `q-marks` state draws it — every one
        /// of its places sits inside the bold with its opening words, and the
        /// event says so as a count, so no word moves into the page. The
        /// collision line beside it keeps its bold to its opening sentence.
        ///
        /// **What it bites on**: the twice line's places left outside the bold
        /// (E4's state), a count short of its places, and the collision line's
        /// path and words pulled into the bold.
        #[test]
        fn ninth_notes_n1_the_twice_line_is_bold_whole() {
            use crate::surface::render_settings::words as the_page;
            use smoozy_job_files::main_output::ThePlacesTheBeautyIsWritten;
            let mut says = what_the_reading_says(
                "/ephemeral/smoozy/jobs/kitchen",
                smoozy_job_files::reading::HowManyViewsTheSceneWrites::One,
            );
            says.six.the_render_reaches_frames_and_a_video = false;
            for places in [
                ThePlacesTheBeautyIsWritten { file_output_nodes: 1, their_inputs: 1, the_output_path: true },
                ThePlacesTheBeautyIsWritten { file_output_nodes: 2, their_inputs: 3, the_output_path: true },
                ThePlacesTheBeautyIsWritten { file_output_nodes: 12, their_inputs: 12, the_output_path: false },
            ] {
                says.six.the_same_picture_reaches_more_than_one_place = true;
                says.six.where_the_same_picture_is_written = places;
                let writes = what_the_job_writes(&says, &[]);
                let (at, twice) = writes
                    .iter()
                    .enumerate()
                    .find(|(_, write)| {
                        write.said.starts_with(the_page::THE_BEAUTY_WRITTEN)
                            || the_page::THE_BEAUTY_WRITTEN_TIMES.contains(&write.said)
                    })
                    .expect("the twice line");
                assert_eq!(twice.bold, twice.parts.len(), "every place in the bold: {twice:?}");
                let entry = TheJobEntry { writes: writes.clone(), ..TheJobEntry::default() };
                let text = super::said(&the_entry_is_said(the_queue_says(&super::a_queue()), Some(&entry)));
                let count = format!("\"write{at}Bold\": Value(Verbatim(\"{}\"))", twice.parts.len());
                assert!(text.contains(&count), "the event says the bold's count, {count}: {text}");
                // **Every other line keeps its bold to its opening words.**
                for (other, write) in writes.iter().enumerate().filter(|(other, _)| *other != at) {
                    assert_eq!(write.bold, 0, "{write:?}");
                    assert!(!text.contains(&format!("write{other}Bold")), "{text}");
                }
            }
        }
    }
    use super::*;

    fn a_queue() -> TheQueueOnScreen {
        TheQueueOnScreen {
            jobs: 9,
            servers: 3,
            lines: vec![AServersLine {
                name: "RunPod 4090".to_string(),
                badge: Some("b1".to_string()),
                rendering: 1,
                waiting: 2,
                rows: vec![ARowInTheQueue {
                    name: "Kitchen — final".to_string(),
                    sub: vec![
                        APart::value("Interior"),
                        APart::figure("240", FRAMES_WORD),
                        APart::said(FRAME_WORD, "148", ""),
                    ],
                    state: Some(RENDERING),
                    state_is: "run",
                    bars: TheRowsBars::both(40, 62),
                    ..ARowInTheQueue::default()
                }],
                ..AServersLine::default()
            }],
            ..TheQueueOnScreen::default()
        }
    }

    fn said(event: &Event) -> String {
        format!("{event:?}")
    }

    /// **The screen answers *nothing is held* on every run today, and it is
    /// Design's own empty queue rather than a stand-in.**
    ///
    /// This is the test that fails when the connection lands and somebody
    /// forgets to change [`what_is_held`] — which is the one place the absence
    /// lives.
    /// **Step 10c piece 7: the queue is composed from the period-held reading
    /// and the local record.** A row is a job the machine holds and the record
    /// names; what the record does not name is the untracked offer; an ended
    /// job has left the queue; and a declared break reads as not updating
    /// (`Queue & History` C7) until the period ends and drops it (`Queue &
    /// History` C6).
    /// **The Creator's seventh notes, piece Z7: a submitted job is in the
    /// queue at once.** From the press its *Submitting…* row is at the end of
    /// its server's line — no grip, no bars, no identity, counted among the
    /// jobs and never as waiting — and it is said so the page draws the
    /// drawing's `qrow sub`. **The machine's own row replaces it with one row
    /// throughout**, and a refusal takes it away. A server with nothing else
    /// in its line gets a line for it.
    #[test]
    fn z7_a_submitted_job_is_in_its_servers_line_at_once() {
        use smoozy_queue_history::acting::adoption::TheMachinesWorkingState;
        use smoozy_queue_history::line::QueueState;
        use smoozy_queue_history::readings::{AJobAsTheMachineReadsIt, WhatTheEntryHasLive};
        use smoozy_state::identities::{Identity, IdentityKind};
        use smoozy_state::observed::{Clock, Reading};

        let mut path = std::env::temp_dir();
        path.push(format!("smoozy-queue-z7-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).unwrap();
        let record = crate::record::LocalRecord::open_at(&path.join("local-record.sqlite3")).unwrap();
        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        let other = Identity::issue(IdentityKind::LocalServer).unwrap();
        for which in [&server, &other] {
            crate::record::server_rows::add(
                record.connection(),
                &smoozy_servers::store::ServerRow::saved(
                    which.as_text(),
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
        }
        let store = crate::record::job_records::JobRecords::over(record.connection());
        let waiting = crate::record::deletions::tests::a_record_in_history(&server, "Kitchen");
        store.write_the_provisional_record(&waiting).unwrap();
        let submitted = crate::record::deletions::tests::a_record_in_history(&server, "Bathroom");

        let holds = crate::servers::the_machines_work::WhatThisRunHolds::default();
        let connection = holds.a_connection_begins(&server.as_text());
        let a_reading = |seconds: u64, jobs: Vec<&Identity>| {
            Reading::taken(
                TheMachinesWorkingState {
                    jobs: jobs
                        .into_iter()
                        .map(|job| AJobAsTheMachineReadsIt {
                            job: job.clone(),
                            state: QueueState::Waiting,
                            live: WhatTheEntryHasLive::NoReadings,
                            the_hold: None,
                        })
                        .collect(),
                    the_admission_point_stands_at: None,
                    notes: Vec::new(),
                    the_history_copys_count: 0,
                    the_history_copys_entries: Vec::new(),
                },
                std::time::SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(seconds),
                Clock::ThisComputer,
            )
        };
        holds.a_reading_was_taken(&server.as_text(), connection, a_reading(1_754_700_000, vec![&waiting.job]), None);

        let mut in_flight = crate::submitting::an_act_is_begun_on(&server.as_text(), "Bathroom".to_string());
        let the_rows = |held: &TheQueueOnScreen| -> Vec<(String, bool)> {
            held.lines
                .iter()
                .filter(|line| line.row == server.as_text())
                .flat_map(|line| line.rows.iter().map(|row| (row.name.clone(), row.submitting)))
                .collect()
        };
        let at_the_press = what_is_held(&holds, Some(&record), None);
        assert_eq!(
            the_rows(&at_the_press),
            vec![("Kitchen".to_string(), false), ("Bathroom".to_string(), true)],
            "at the end of its server's line, from the press"
        );
        let row = &at_the_press.lines[0].rows[1];
        assert_eq!(row.state, Some(SUBMITTING_ROW));
        assert_eq!(
            (row.state_is, row.grip, row.bars, row.group, row.job.as_str()),
            ("", false, TheRowsBars::default(), None, "")
        );
        let every_row = |held: &TheQueueOnScreen| held.lines.iter().map(|line| line.rows.len() as u32).sum::<u32>();
        assert_eq!(at_the_press.jobs, every_row(&at_the_press), "it counts among the jobs");
        assert_eq!(at_the_press.lines[0].waiting, 1, "and never as waiting");
        let said = said(&the_queue_says(&at_the_press));
        assert!(said.contains("row1Submitting"), "{said}");
        assert!(said.contains("Submitting…"), "{said}");

        // **The act's record is written and the job sent; the reading has not
        // seen it yet.** Still one row for it, the act's.
        in_flight.its_jobs_are(&[submitted.job.clone()]);
        store.write_the_provisional_record(&submitted).unwrap();
        in_flight.it_ended(&crate::submitting::WhatTheActCameTo::TheseJobs(vec![
            crate::submitting::WhatBecameOfOneJob::ItIsInTheLine,
        ]));
        drop(in_flight);
        assert_eq!(
            the_rows(&what_is_held(&holds, Some(&record), None)),
            vec![("Kitchen".to_string(), false), ("Bathroom".to_string(), true)],
            "no gap before the machine's row"
        );
        // **The machine's reading holds it: its row, and no other.**
        holds.a_reading_was_taken(
            &server.as_text(),
            connection,
            a_reading(1_754_700_005, vec![&waiting.job, &submitted.job]),
            None,
        );
        let replaced = what_is_held(&holds, Some(&record), None);
        assert_eq!(
            the_rows(&replaced),
            vec![("Kitchen".to_string(), false), ("Bathroom".to_string(), false)],
            "the machine's own row, once"
        );
        assert_eq!(replaced.jobs, every_row(&replaced));

        // **A refusal takes the row away**, and a line with nothing else in
        // it is drawn for the row alone while it stands.
        let in_flight = crate::submitting::an_act_is_begun_on(&other.as_text(), "Hallway".to_string());
        let with_it = what_is_held(&holds, Some(&record), None);
        let line = with_it.lines.iter().find(|line| line.row == other.as_text()).expect("a line for it");
        assert_eq!(line.rows.len(), 1);
        assert!(line.rows[0].submitting);
        drop(in_flight);
        let without = what_is_held(&holds, Some(&record), None);
        assert!(without.lines.iter().all(|line| line.row != other.as_text()), "refused, and gone");
        let _ = std::fs::remove_dir_all(&path);
    }

    /// **`Queue & History` C10 and `Design/Queue & History` line 25 as changed
    /// 2026-10-19: a submission that ended with a job not in the machine's
    /// line stays in its server's line, saying what the panel says, with a
    /// cross** — *Not submitted.* in the state's place and its red, why under
    /// the name, not counted among the jobs, and in a line of its own where
    /// its server has nothing else. **Its cross clears it.** Out of the live
    /// pass on Vast for the thirteenth and fourteenth notes, fault 1: the act
    /// ended *files not placed* and the row went, with nothing said anywhere
    /// the user was looking.
    #[test]
    fn a_submission_not_submitted_stays_in_its_servers_line_saying_why() {
        use crate::submitting::{WhatBecameOfOneJob as Became, WhatTheActCameTo};
        use smoozy_state::identities::{Identity, IdentityKind};

        let mut path = std::env::temp_dir();
        path.push(format!("smoozy-queue-not-submitted-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).unwrap();
        let record = crate::record::LocalRecord::open_at(&path.join("local-record.sqlite3")).unwrap();
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
        let holds = crate::servers::the_machines_work::WhatThisRunHolds::default();
        let mut in_flight = crate::submitting::an_act_is_begun_on(&server.as_text(), "pv-anim.blend".to_string());
        let which = in_flight.which();
        in_flight.it_ended(&WhatTheActCameTo::TheseJobs(vec![Became::ItStoppedBeforeTheJobRecord {
            the_step: smoozy_rendering::submission::THE_ORDERED_UNIT[0],
        }]));
        drop(in_flight);

        let held = what_is_held(&holds, Some(&record), None);
        let line = held
            .lines
            .iter()
            .find(|line| line.row == server.as_text())
            .expect("**the row went with the act, and nothing said why**");
        assert_eq!(line.rows.len(), 1, "a line of its own");
        let row = &line.rows[0];
        let words = |sentence: &[crate::surface::servers::Words]| {
            sentence.iter().map(|part| part.lead.as_str().to_string()).collect::<Vec<_>>()
        };
        let not = row.not_submitted.as_ref().expect("it says it was not submitted");
        assert_eq!(not.act, which);
        let as_drawn = |text: &str| Prose::new(text).as_str().to_string();
        assert_eq!(words(&not.what_went), vec![as_drawn("Not submitted.")]);
        assert_eq!(words(&not.why), vec![as_drawn("Could not upload the job’s files to the server.")]);
        assert_eq!(
            (row.name.as_str(), row.state_is, row.state, row.submitting, row.grip, row.job.as_str()),
            ("pv-anim.blend", "bad", None, false, false, "")
        );
        assert_eq!(held.jobs, 0, "nothing of it is in the line, so it is no job the header counts");
        let said = said(&the_queue_says(&held));
        for key in ["row0NotSubmitted", "row0Went0Lead", "row0Why0Lead", "clearIt"] {
            assert!(said.contains(key), "{key} was not said: {said}");
        }
        for words in ["Not submitted.", "Could not upload the job’s files to the server."] {
            let words = format!("{:?}", Prose::new(words));
            assert!(said.contains(&words), "{words} was not said: {said}");
        }
        assert!(!said.contains("row0Submitting"), "{said}");

        // **The cross clears it** — the page's ask, read as the page sends it.
        let asked: TheQueueAsk = serde::Deserialize::deserialize(
            serde::de::value::StrDeserializer::<serde::de::value::Error>::new("clear-the-row"),
        )
        .expect("the cross's ask is one the queue takes");
        assert_eq!(asked, TheQueueAsk::ClearTheRow);
        assert!(crate::submitting::a_not_submitted_row_is_cleared(which));
        let cleared = what_is_held(&holds, Some(&record), None);
        assert!(cleared.lines.iter().all(|line| line.row != server.as_text()), "cleared, and still drawn");
        let _ = std::fs::remove_dir_all(&path);
    }

    /// **The page draws a row saying *Not submitted.* as the application says
    /// it, and its cross hands back the act** — `web/the-queue.js` under Node
    /// over the boxes harness's stand-ins: the state in its red, why under the
    /// name, no spinner and no job, and the cross named for its tooltip and
    /// asking the queue to clear the row, with the act. `Queue & History` C10 as
    /// changed 2026-10-19. **Skipped, and says so, where no Node is
    /// installed.**
    #[test]
    fn the_page_draws_a_row_not_submitted_and_its_cross_clears_it() {
        let Some(lines) = a_page_run(
            "not-submitted-page",
            &["the-queue.js", "the-value-drawn.js"],
            &[],
            r#"
globalThis.invoked = [];
globalThis.window.__TAURI__.core.invoke = (command, args) => {
  globalThis.invoked.push([command, args]);
  return Promise.resolve(command === "the_pane_layout" ? { seams: [] } : null);
};
await import("./the-queue.js");
const tell = (kind, said) => { for (const handler of globalThis.told.get(kind) || []) handler({ said }); };
const queue = anElement("div");
globalThis.theRoot.appendChild(queue);
globalThis.screens.get("queueview").itIsDrawn(queue);
tell("queue:the-queue", {
  theHeading: "Queue", lineCount: "1", line0Server: "s1", line0Name: "Vast 4090",
  rowCount: "1", row0Job: "", row0Line: "0", row0Name: "pv-anim.blend", row0StateIs: "bad",
  row0NotSubmitted: "7",
  row0WentCount: "1", row0Went0Lead: "Not submitted.", row0Went0ValueCount: "0",
  row0WhyCount: "1", row0Why0Lead: "Could not upload the job’s files to the server.", row0Why0ValueCount: "0",
  clearIt: "Clear",
});
const row = queue.querySelector("div.qrow.sub");
const textOf = (element) => element ? (element.text || "") + (element.children || []).map(textOf).join("") : "";
const state = row && row.querySelector("span.qst");
const why = row && row.querySelector("span.qs");
const cross = row && row.querySelector("button.fx");
const drawn = {
  state: textOf(state),
  bad: !!state && state.classList.contains("bad"),
  why: textOf(why),
  spinner: !!(row && row.querySelector("i.spin")),
  job: row ? row.dataset.job === undefined : false,
  cross: cross ? [cross.text, cross.title, cross.getAttribute("aria-label")] : null,
};
if (cross) cross.click();
drawn.asked = globalThis.invoked.filter(([command, args]) => command === "the_queue" && args && args.asking === "clear-the-row").map(([, args]) => args);
console.log(JSON.stringify(drawn));
"#,
        ) else {
            return;
        };
        assert_eq!(
            lines,
            vec![serde_json::json!({
                "state": "Not submitted.",
                "bad": true,
                "why": "Could not upload the job’s files to the server.",
                "spinner": false,
                "job": true,
                "cross": ["×", "Clear", "Clear"],
                "asked": [{ "chosen": null, "asking": "clear-the-row", "dropped": "7" }],
            })]
        );
    }

    #[test]
    fn the_queue_is_the_reading_and_the_record_together() {
        use smoozy_queue_history::acting::adoption::TheMachinesWorkingState;
        use smoozy_queue_history::line::QueueState;
        use smoozy_queue_history::readings::{AJobAsTheMachineReadsIt, WhatTheEntryHasLive};
        use smoozy_state::identities::{Identity, IdentityKind};
        use smoozy_state::observed::{Clock, Reading};

        let mut path = std::env::temp_dir();
        path.push(format!("smoozy-queue-held-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).unwrap();
        let record = crate::record::LocalRecord::open_at(&path.join("local-record.sqlite3")).unwrap();
        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        let store = crate::record::job_records::JobRecords::over(record.connection());
        let mut rendering = crate::record::deletions::tests::a_record_in_history(&server, "Kitchen");
        rendering.the_render_start_has_been_learned = false;
        store.write_the_provisional_record(&rendering).unwrap();
        let mut ended = crate::record::deletions::tests::a_record_in_history(&server, "Hallway");
        ended.the_render_start_has_been_learned = false;
        store.write_the_provisional_record(&ended).unwrap();
        let theirs = Identity::issue(IdentityKind::Job).unwrap();
        let an_operation = || Identity::issue(IdentityKind::Operation).unwrap();

        let holds = crate::servers::the_machines_work::WhatThisRunHolds::default();
        let connection = holds.a_connection_begins(&server.as_text());
        let job = |job: &Identity, state: QueueState| AJobAsTheMachineReadsIt {
            job: job.clone(),
            state,
            live: WhatTheEntryHasLive::NoReadings,
            the_hold: None,
        };
        holds.a_reading_was_taken(
            &server.as_text(),
            connection,
            Reading::taken(
                TheMachinesWorkingState {
                    jobs: vec![
                        job(&rendering.job, QueueState::Rendering { operation: an_operation() }),
                        job(&ended.job, QueueState::Ended { outcome: an_operation() }),
                        job(&theirs, QueueState::Waiting),
                    ],
                    the_admission_point_stands_at: None,
                    notes: Vec::new(),
                    the_history_copys_count: 0,
                    the_history_copys_entries: Vec::new(),
                },
                std::time::SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(1_754_700_000),
                Clock::ThisComputer,
            ),
            Some(vec![theirs.clone()]),
        );

        let held = what_is_held(&holds, Some(&record), None);
        assert_eq!(held.jobs, 1, "the ended job has left the queue");
        assert_eq!(held.servers, 1, "the one server read");
        assert_eq!(held.lines[0].rows[0].name, "Kitchen");
        assert_eq!(held.lines[0].rows[0].state, Some(RENDERING));
        assert_eq!(held.lines[0].rendering, 1);
        let untracked = held.untracked.as_ref().expect("the offer is drawn");
        assert_eq!((untracked.rendering, untracked.waiting), (0, 1));
        let entry = held.entry.as_ref().expect("the first row's entry");
        assert_eq!(entry.name, "Kitchen");
        assert_eq!(entry.stop_or_cancel, Some(TheStopOrCancel::Stop));
        // **Step 10c's *Files on the server* follow-up: connected, so it is
        // offered** — and not, below, once the break is declared.
        assert!(!entry.the_server_is_not_connected, "connected, and Files on the server withheld");
        assert!(the_servers_connected(&holds).contains(&server.as_text()));

        holds.the_connection_ended(&server.as_text());
        let stale = what_is_held(&holds, Some(&record), None);
        assert_eq!(stale.lines[0].last_seen.as_deref(), Some("1754700000"));
        assert!(stale.lines[0].rows[0].not_updating);
        assert_eq!(stale.lines[0].rows[0].state, Some(NOT_UPDATING));
        assert!(
            stale.entry.as_ref().expect("the entry").the_server_is_not_connected,
            "not connected, and Files on the server still offered"
        );
        assert!(!the_servers_connected(&holds).contains(&server.as_text()));

        holds.the_period_ended(&server.as_text());
        assert_eq!(what_is_held(&holds, Some(&record), None), TheQueueOnScreen::default());
        let _ = std::fs::remove_dir_all(&path);
    }

    /// **The 38's piece 15, `Queue & History` A4: with *Ignore* standing, a
    /// job waiting behind work Smoozy is not tracking says why it waits** —
    /// the fact and the count, never the jobs. **Only where that work is
    /// rendering or would start next**: behind a tracked job, the untracked
    /// work is not why it waits. Before *Ignore*, the banner says it.
    #[test]
    fn a_job_waiting_behind_ignored_work_says_the_server_is_busy_with_it() {
        use smoozy_queue_history::acting::adoption::TheMachinesWorkingState;
        use smoozy_queue_history::line::QueueState;
        use smoozy_queue_history::readings::{AJobAsTheMachineReadsIt, WhatTheEntryHasLive};
        use smoozy_state::identities::{Identity, IdentityKind};
        use smoozy_state::observed::{Clock, Reading};
        let mut path = std::env::temp_dir();
        path.push(format!("smoozy-queue-ignored-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).unwrap();
        let record = crate::record::LocalRecord::open_at(&path.join("local-record.sqlite3")).unwrap();
        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        let store = crate::record::job_records::JobRecords::over(record.connection());
        let mut mine = crate::record::deletions::tests::a_record_in_history(&server, "Kitchen");
        mine.the_render_start_has_been_learned = false;
        store.write_the_provisional_record(&mine).unwrap();
        let (one, two, three) = (
            Identity::issue(IdentityKind::Job).unwrap(),
            Identity::issue(IdentityKind::Job).unwrap(),
            Identity::issue(IdentityKind::Job).unwrap(),
        );
        let job = |job: &Identity, state: QueueState| AJobAsTheMachineReadsIt {
            job: job.clone(),
            state,
            live: WhatTheEntryHasLive::NoReadings,
            the_hold: None,
        };
        let chosen = mine.job.as_text();
        let theirs_render = || QueueState::Rendering { operation: Identity::issue(IdentityKind::Operation).unwrap() };
        let held_with = |jobs: Vec<AJobAsTheMachineReadsIt>, next: Option<&Identity>| {
            let holds = crate::servers::the_machines_work::WhatThisRunHolds::default();
            let connection = holds.a_connection_begins(&server.as_text());
            holds.a_reading_was_taken(
                &server.as_text(),
                connection,
                Reading::taken(
                    TheMachinesWorkingState {
                        jobs,
                        the_admission_point_stands_at: next.cloned(),
                        notes: Vec::new(),
                        the_history_copys_count: 0,
                        the_history_copys_entries: Vec::new(),
                    },
                    std::time::SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(1_754_700_000),
                    Clock::ThisComputer,
                ),
                Some(vec![one.clone(), two.clone(), three.clone()]),
            );
            what_is_held(&holds, Some(&record), Some(chosen.as_str()))
        };
        let status_after = |mut held: TheQueueOnScreen, declined: Option<String>| {
            let mut declined = declined;
            the_decline_is_applied(&mut held, &mut declined);
            (held.untracked.is_some(), held.entry.expect("the waiting entry").status)
        };

        // One rendering and two waiting, none of them tracked, ahead of mine.
        let behind_a_render = || {
            held_with(
                vec![
                    job(&one, theirs_render()),
                    job(&two, QueueState::Waiting),
                    job(&three, QueueState::Waiting),
                    job(&mine.job, QueueState::Waiting),
                ],
                Some(&two),
            )
        };
        assert_eq!(
            status_after(behind_a_render(), None),
            (true, Vec::new()),
            "before Ignore, the banner says it and the entry says nothing"
        );
        assert_eq!(
            status_after(behind_a_render(), Some(server.as_text())),
            (false, vec![APart::said("The server is busy with", "3", "untracked jobs.")])
        );
        // **One untracked job, next to start**: the count is one, in the
        // singular.
        let behind_one = held_with(vec![job(&one, QueueState::Waiting), job(&mine.job, QueueState::Waiting)], Some(&one));
        assert_eq!(
            status_after(behind_one, Some(server.as_text())),
            (false, vec![APart::said("The server is busy with", "1", "untracked job.")])
        );
        // **Nothing untracked is rendering and mine starts next**: the
        // untracked work is not why it waits, so nothing is said.
        let mine_next = held_with(vec![job(&one, QueueState::Waiting), job(&mine.job, QueueState::Waiting)], Some(&mine.job));
        assert_eq!(status_after(mine_next, Some(server.as_text())), (false, Vec::new()));
        // **Another server's decline says nothing here.**
        assert_eq!(
            status_after(behind_a_render(), Some("another server".to_string())),
            (true, Vec::new())
        );
        let _ = std::fs::remove_dir_all(&path);
    }

    /// **Piece 6b: a rendering entry draws the machine's figures, through
    /// `bars`, and absent figures stay absent.** The row carries both bars, the
    /// folded heading the whole job's percentage, and the entry *Rendering now*
    /// — the frame's own number, the samples as printed, Blender's estimate as
    /// Blender's — and *Whole job* as the machine's finished part over its
    /// total. A job carrying no figures draws none, and nothing is drawn beside
    /// *Not updating*.
    ///
    /// **What it bites on**: the entry built with no progress (the state before
    /// this piece), the row with no bars, and bars drawn after *Not updating*.
    #[test]
    fn piece_6b_a_rendering_entry_draws_the_machines_figures_and_absent_stays_absent() {
        use crate::servers::the_machines_work::tests::{a_queue_answer_with_figures, a_report_taken_at};
        use crate::servers::the_machines_work::{the_queue_reading_in, the_queue_reading_request};
        use smoozy_server_protocol::asking_the_supervisor::WhatCameBack;
        use smoozy_server_protocol::framing::{Answered, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS};
        use smoozy_state::identities::{Identity, IdentityKind};
        use smoozy_state::observed::{Clock, Reading};

        let mut path = std::env::temp_dir();
        path.push(format!("smoozy-queue-figures-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).unwrap();
        let record = crate::record::LocalRecord::open_at(&path.join("local-record.sqlite3")).unwrap();
        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        let store = crate::record::job_records::JobRecords::over(record.connection());
        let mut rendering = crate::record::deletions::tests::a_record_in_history(&server, "Kitchen");
        rendering.the_render_start_has_been_learned = false;
        store.write_the_provisional_record(&rendering).unwrap();
        let waiting = Identity::issue(IdentityKind::Job).unwrap();

        let holds = crate::servers::the_machines_work::WhatThisRunHolds::default();
        let connection = holds.a_connection_begins(&server.as_text());
        let read = |report: Option<&smoozy_queue_history::readings::AWorkersReport>| {
            let request = the_queue_reading_request().unwrap();
            the_queue_reading_in(WhatCameBack::ItAnswered(
                Answered::to(&request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, "test")
                    .carrying(a_queue_answer_with_figures(&rendering.job, &waiting, report, None)),
            ))
            .expect("the answer reads")
        };
        let at = |seconds: u64| std::time::SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(seconds);

        // **No figures carried: no bars, no figure, never a zero.**
        holds.a_reading_was_taken(
            &server.as_text(),
            connection,
            Reading::taken(read(None), at(1_754_700_000), Clock::ThisComputer),
            None,
        );
        let held = what_is_held(&holds, Some(&record), None);
        assert_eq!(held.lines[0].rows[0].bars, TheRowsBars::default());
        assert_eq!(held.lines[0].folded, None);
        assert_eq!(held.entry.as_ref().expect("the entry").progress, None);

        // **Figures carried: 64 of 128 printed, 768 of 1,280 plan units done.**
        let report = a_report_taken_at(1_790_000_000);
        holds.a_reading_was_taken(
            &server.as_text(),
            connection,
            Reading::taken(read(Some(&report)), at(1_754_700_005), Clock::ThisComputer),
            None,
        );
        let held = what_is_held(&holds, Some(&record), None);
        assert_eq!(held.lines[0].rows[0].bars, TheRowsBars::both(50, 60), "both bars, from the machine's figures");
        assert_eq!(held.lines[0].folded, Some(("Kitchen".to_string(), 60)));
        let progress = held.entry.as_ref().expect("the entry").progress.clone().expect("the two bars");
        assert_eq!((progress.now_at, progress.whole), (Some(50), Some(60)));
        assert_eq!(
            progress.now,
            vec![
                APart::said(FRAME_HEADING, "1007", ""),
                APart::figure("64/128", SAMPLES_WORD),
                APart::figure("00:12.58", LEFT_WORD),
                APart::words(A_DOT),
                APart::words(BLENDERS_ESTIMATE),
            ],
            "the frame's own number, the samples as printed, and Blender's estimate as Blender's"
        );
        let told = said_of(&held);
        assert!(told.contains("progNowAt") && told.contains("progWhole"), "both bars travel");

        // **The total not carried: the whole job's bar is absent, not nought.**
        let mut no_total = a_report_taken_at(1_790_000_002);
        no_total.the_plan_total = None;
        holds.a_reading_was_taken(
            &server.as_text(),
            connection,
            Reading::taken(read(Some(&no_total)), at(1_754_700_007), Clock::ThisComputer),
            None,
        );
        let held = what_is_held(&holds, Some(&record), None);
        let progress = held.entry.as_ref().unwrap().progress.clone().expect("the first bar alone");
        assert_eq!((progress.now_at, progress.whole), (Some(50), None));
        // **W4: the row draws each bar it has** — the frame's bar stays, and
        // only the whole job's is absent.
        assert_eq!(
            held.lines[0].rows[0].bars,
            TheRowsBars { now_at: Some(50), whole: None },
            "the row keeps the bar it has"
        );
        assert!(!said_of(&held).contains("progWhole"));

        // **W4, the Creator's twelfth notes, note 7: the next frame has begun
        // and Blender has printed no sample on it yet.** The top bar reads
        // nought and stays, on the row and on the entry, beside the whole
        // job's; until W4 both went.
        let mut begun = a_report_taken_at(1_790_000_004);
        begun.readings = Reading::taken(
            smoozy_queue_history::readings::LiveReadings {
                frame_in_hand: Some(1008),
                frames_completed: 7,
                ..Default::default()
            },
            at(1_790_000_004),
            Clock::AMachine(smoozy_state::observed::MachineClock("installation-1".to_string())),
        );
        holds.a_reading_was_taken(
            &server.as_text(),
            connection,
            Reading::taken(read(Some(&begun)), at(1_754_700_009), Clock::ThisComputer),
            None,
        );
        let held = what_is_held(&holds, Some(&record), None);
        assert_eq!(held.lines[0].rows[0].bars, TheRowsBars::both(0, 60), "the frame's bar stands empty");
        let progress = held.entry.as_ref().unwrap().progress.clone().expect("the two bars");
        assert_eq!((progress.now_at, progress.whole), (Some(0), Some(60)));
        assert_eq!(progress.now, vec![APart::said(FRAME_HEADING, "1008", "")], "no figure made up");
        let told = said_of(&held);
        assert!(told.contains("progNowAt") && told.contains("row0Renderlet"), "the empty bar travels");

        // **Not updating: nothing is drawn.**
        holds.the_connection_ended(&server.as_text());
        let stale = what_is_held(&holds, Some(&record), None);
        assert_eq!(stale.lines[0].rows[0].bars, TheRowsBars::default());
        assert_eq!(stale.entry.as_ref().unwrap().progress, None);
        let _ = std::fs::remove_dir_all(&path);

        fn said_of(held: &TheQueueOnScreen) -> String {
            said(&the_queue_says(held))
        }
    }

    /// **Piece 6c, `Queue & History` E8: a rendering entry offers *Blender
    /// output* where its words can be read, and what arrives travels under each
    /// stream's own keys** — the first arrival saying it is first, the words a
    /// value whole, and a stream with nothing yet saying so in M10a's
    /// sentences rather than an empty box. Nothing is offered beside *Not
    /// updating*.
    ///
    /// **What it bites on**: a rendering entry offering no *Blender output*
    /// (the state before this piece), one offering it beside *Not updating*, a
    /// stream's words under the other's keys, and an empty stream drawn empty.
    #[test]
    fn piece_6c_a_rendering_entry_offers_its_words_and_each_stream_keeps_its_own_keys() {
        use crate::servers::the_machines_work::tests::{a_queue_answer_with_figures, the_sinks_at};
        use crate::servers::the_machines_work::{the_queue_reading_in, the_queue_reading_request, TheWordsThatArrived};
        use smoozy_server_protocol::asking_the_supervisor::WhatCameBack;
        use smoozy_server_protocol::framing::{Answered, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS};
        use smoozy_state::identities::{Identity, IdentityKind};
        use smoozy_state::observed::{Clock, Reading};

        let mut path = std::env::temp_dir();
        path.push(format!("smoozy-queue-words-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).unwrap();
        let record = crate::record::LocalRecord::open_at(&path.join("local-record.sqlite3")).unwrap();
        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        let store = crate::record::job_records::JobRecords::over(record.connection());
        let mut rendering = crate::record::deletions::tests::a_record_in_history(&server, "Kitchen");
        rendering.the_render_start_has_been_learned = false;
        store.write_the_provisional_record(&rendering).unwrap();
        let waiting = Identity::issue(IdentityKind::Job).unwrap();
        let holds = crate::servers::the_machines_work::WhatThisRunHolds::default();
        let connection = holds.a_connection_begins(&server.as_text());
        let request = the_queue_reading_request().unwrap();
        let state = the_queue_reading_in(WhatCameBack::ItAnswered(
            Answered::to(&request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, "test").carrying(
                a_queue_answer_with_figures(&rendering.job, &waiting, None, Some(&the_sinks_at(1, 40, 0))),
            ),
        ))
        .unwrap();
        holds.a_reading_was_taken(
            &server.as_text(),
            connection,
            Reading::taken(state, std::time::SystemTime::UNIX_EPOCH, Clock::ThisComputer),
            None,
        );
        let held = what_is_held(&holds, Some(&record), None);
        assert!(held.entry.as_ref().unwrap().blender_output, "a rendering entry offers its words");
        assert!(said(&the_queue_says(&held)).contains("blenderOutput"));

        let arrived = TheWordsThatArrived {
            job: rendering.job.as_text(),
            name: "Kitchen".to_string(),
            first: true,
            new: ["Fra:1 Sample 1/128\n".to_string(), String::new()],
            any_yet: [true, false],
        };
        let told = said(&the_running_words_said(&arrived));
        assert!(told.contains("fresh"));
        assert!(told.contains("theOutputMore") && told.contains("Fra:1 Sample 1/128"));
        assert!(!told.contains("theErrorsMore"), "the complaints stream has nothing and no key");
        {
            use crate::presentation::the_machines_own_words::{
                SMOOZY_DOES_NOT_KNOW_YET, THE_OUTCOME_HAS_NOT_BEEN_RECORDED_YET,
            };
            assert!(
                told.contains(&format!("{:?}", Prose::new(THE_OUTCOME_HAS_NOT_BEEN_RECORDED_YET)))
                    && told.contains(&format!("{:?}", Prose::new(SMOOZY_DOES_NOT_KNOW_YET))),
                "a stream with nothing yet says so in M10a's two sentences"
            );
        }
        assert!(!told.contains("theOutputSaid"), "a stream with words says no sentence");
        let later = said(&the_running_words_said(&TheWordsThatArrived {
            first: false,
            new: [String::new(), "Error: no\n".to_string()],
            any_yet: [true, true],
            ..arrived
        }));
        assert!(!later.contains("fresh"));
        assert!(later.contains("theErrorsMore") && !later.contains("theOutputMore"), "each stream on its own keys");

        holds.the_connection_ended(&server.as_text());
        let stale = what_is_held(&holds, Some(&record), None);
        assert!(!stale.entry.as_ref().unwrap().blender_output, "nothing is offered beside Not updating");
        let _ = std::fs::remove_dir_all(&path);
    }

    /// **The Creator's notes on the test copy, piece 8 (note 12): a rendering
    /// entry draws the ordinary stream's newest three lines, and no other entry
    /// draws any** — `Desktop app and delivery` M13b's one narrowing.
    ///
    /// **What it bites on**: lines on a waiting entry, beside *Not updating*,
    /// or on History, and lines that never reach the event. **The M13b half is
    /// worth more than the drawing half**, the map's fifth note to the runner.
    #[test]
    fn piece_8_a_rendering_entry_draws_its_newest_three_lines_and_no_other_entry_any() {
        use crate::servers::the_machines_work::tests::{a_queue_answer_with_figures, the_sinks_at, SinksInHand};
        use crate::servers::the_machines_work::{the_entrys_lines_are_read, the_queue_reading_in, the_queue_reading_request};
        use smoozy_server_protocol::asking_the_supervisor::WhatCameBack;
        use smoozy_server_protocol::framing::{Answered, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS};
        use smoozy_state::identities::{Identity, IdentityKind};
        use smoozy_state::observed::{Clock, Reading};

        let mut path = std::env::temp_dir();
        path.push(format!("smoozy-queue-lines-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).unwrap();
        let record = crate::record::LocalRecord::open_at(&path.join("local-record.sqlite3")).unwrap();
        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        let store = crate::record::job_records::JobRecords::over(record.connection());
        let mut rendering = crate::record::deletions::tests::a_record_in_history(&server, "Kitchen");
        rendering.the_render_start_has_been_learned = false;
        store.write_the_provisional_record(&rendering).unwrap();
        let mut waiting = crate::record::deletions::tests::a_record_in_history(&server, "Hallway");
        waiting.the_render_start_has_been_learned = false;
        store.write_the_provisional_record(&waiting).unwrap();
        let holds = crate::servers::the_machines_work::WhatThisRunHolds::default();
        let connection = holds.a_connection_begins(&server.as_text());
        let printed = b"Fra:1 Sample 1/128\nFra:1 Sample 64/128\nFra:1 Sample 128/128\nFra:2 Sample 1/128\n".to_vec();
        let request = the_queue_reading_request().unwrap();
        let state = the_queue_reading_in(WhatCameBack::ItAnswered(
            Answered::to(&request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, "test").carrying(
                a_queue_answer_with_figures(
                    &rendering.job,
                    &waiting.job,
                    None,
                    Some(&the_sinks_at(1, printed.len() as u64, 9)),
                ),
            ),
        ))
        .unwrap();
        holds.a_reading_was_taken(
            &server.as_text(),
            connection,
            Reading::taken(state, std::time::SystemTime::UNIX_EPOCH, Clock::ThisComputer),
            None,
        );
        holds.the_queue_is_shown(Some(&rendering.job.as_text()));
        let first = what_is_held(&holds, Some(&record), Some(&rendering.job.as_text()));
        let entry = first.entry.as_ref().unwrap();
        assert!(entry.blender_output);
        assert!(entry.lines.is_empty(), "nothing is drawn before the first read lands");
        assert!(!said(&the_queue_says(&first)).contains("entryLine"));

        let mut machine = SinksInHand::default();
        machine.bytes.insert(the_sinks_at(1, 0, 0).ordinary.path, printed);
        machine.bytes.insert(the_sinks_at(1, 0, 0).complaints.path, b"Error: no\n".to_vec());
        assert!(the_entrys_lines_are_read(&holds, &server.as_text(), &mut machine));
        let drawn = what_is_held(&holds, Some(&record), Some(&rendering.job.as_text()));
        assert_eq!(
            drawn.entry.as_ref().unwrap().lines,
            vec!["Fra:1 Sample 64/128".to_string(), "Fra:1 Sample 128/128".to_string(), "Fra:2 Sample 1/128".to_string()]
        );
        let told = said(&the_queue_says(&drawn));
        assert!(told.contains("entryLineCount") && told.contains("entryLine0") && told.contains("entryLine2"));
        assert!(told.contains("Fra:2 Sample 1/128") && !told.contains("Fra:1 Sample 1/128\""));
        assert!(!told.contains("entryLine3"), "three lines and no more");
        assert!(!told.contains("Error: no"), "never the errors stream");

        // **A waiting entry draws none**, and drawing it stops the reads.
        let other = what_is_held(&holds, Some(&record), Some(&waiting.job.as_text()));
        assert!(other.entry.as_ref().unwrap().lines.is_empty());
        assert!(!said(&the_queue_says(&other)).contains("entryLine"));
        assert!(!holds.the_entrys_lines_are_on_screen_for(&server.as_text()));

        // **Beside *Not updating*, none.**
        what_is_held(&holds, Some(&record), Some(&rendering.job.as_text()));
        assert!(the_entrys_lines_are_read(&holds, &server.as_text(), &mut machine));
        holds.the_connection_ended(&server.as_text());
        let stale = what_is_held(&holds, Some(&record), Some(&rendering.job.as_text()));
        assert!(stale.entry.as_ref().unwrap().lines.is_empty());
        assert!(!said(&the_queue_says(&stale)).contains("entryLine"));

        // **History's entry of the same job, none** — M13b stands there.
        let beside = crate::surface::the_history::BesideTheRecord::read(record.connection(), &rendering.job);
        assert!(crate::surface::the_history::the_entry_for(&rendering, &beside, false, true).entry.lines.is_empty());
        let _ = std::fs::remove_dir_all(&path);
    }

    /// **`Design/Whole app § Typography`: a figure and the words after it are
    /// bound on the event**, so the page writes U+00A0 between them — *2
    /// waiting*, *240 frames* — and a name and its words are not.
    #[test]
    fn a_figure_and_its_trail_are_bound_on_the_event() {
        let figure = APart::figure("2", WAITING_WORD);
        assert!(the_trail_binds(&figure));
        let event = a_part_is_said(
            Event::about(crate::events::Subject::TheRun, "a test"),
            "l",
            "v",
            "t",
            "b",
            &figure,
        );
        assert!(said(&event).contains("\"b\""), "the bind mark travels with the part");
        assert!(!the_trail_binds(&APart::said("", "kitchen_v14", "is rendering")));
        assert!(!the_trail_binds(&APart::value("2")));
        let named = a_part_is_said(
            Event::about(crate::events::Subject::TheRun, "a test"),
            "l",
            "v",
            "t",
            "b",
            &APart::said("", "kitchen", "is rendering"),
        );
        assert!(!said(&named).contains("\"b\""));
    }

    /// **A reading held for `server` whose jobs are these**, taken at a fixed
    /// moment on this computer's clock.
    pub(crate) fn a_reading_of(
        holds: &crate::servers::the_machines_work::WhatThisRunHolds,
        server: &smoozy_state::identities::Identity,
        jobs: Vec<(smoozy_state::identities::Identity, smoozy_queue_history::line::QueueState)>,
    ) {
        a_reading_with_its_holds_of(holds, server, jobs.into_iter().map(|(job, state)| (job, state, None)).collect());
    }

    /// **The same, where each job's item may say what its hold says** — the
    /// reading step 10c's eleventh-pass fix piece 8b's program sends.
    pub(crate) fn a_reading_with_its_holds_of(
        holds: &crate::servers::the_machines_work::WhatThisRunHolds,
        server: &smoozy_state::identities::Identity,
        jobs: Vec<(
            smoozy_state::identities::Identity,
            smoozy_queue_history::line::QueueState,
            Option<smoozy_queue_history::readings::WhatTheHoldSays>,
        )>,
    ) {
        use smoozy_queue_history::acting::adoption::TheMachinesWorkingState;
        use smoozy_queue_history::readings::{AJobAsTheMachineReadsIt, WhatTheEntryHasLive};
        use smoozy_state::observed::{Clock, Reading};
        let connection = holds
            .the_connection_now(&server.as_text())
            .unwrap_or_else(|| holds.a_connection_begins(&server.as_text()));
        holds.a_reading_was_taken(
            &server.as_text(),
            connection,
            Reading::taken(
                TheMachinesWorkingState {
                    jobs: jobs
                        .into_iter()
                        .map(|(job, state, the_hold)| AJobAsTheMachineReadsIt {
                            job,
                            state,
                            live: WhatTheEntryHasLive::NoReadings,
                            the_hold,
                        })
                        .collect(),
                    the_admission_point_stands_at: None,
                    notes: Vec::new(),
                    the_history_copys_count: 0,
                    the_history_copys_entries: Vec::new(),
                },
                std::time::SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(1_759_000_000),
                Clock::ThisComputer,
            ),
            None,
        );
    }

    /// **Step 10c's Vast pass, fault 3, in Queue: a job sent with no name was a
    /// row with no text**, and the servers' work column named nothing while it
    /// rendered. **`Design/Queue & History § A rendering entry`, Creator-stated
    /// 2026-08-10: *"Where the job carries no label of its own, the name is the
    /// file."*** The pass's *pv-anim*, read waiting behind another job and then
    /// rendering.
    #[test]
    fn a_job_sent_with_no_name_is_called_by_its_file_in_the_queue() {
        use smoozy_queue_history::acting::adoption::TheMachinesWorkingState;
        use smoozy_queue_history::line::QueueState;
        use smoozy_queue_history::readings::{AJobAsTheMachineReadsIt, WhatTheEntryHasLive};
        use smoozy_state::identities::{Identity, IdentityKind};
        use smoozy_state::observed::{Clock, Reading};

        let mut path = std::env::temp_dir();
        path.push(format!("smoozy-queue-no-name-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).unwrap();
        let record = crate::record::LocalRecord::open_at(&path.join("local-record.sqlite3")).unwrap();
        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        let store = crate::record::job_records::JobRecords::over(record.connection());
        let mut unnamed = crate::record::deletions::tests::a_record_in_history(&server, "");
        unnamed.label = None;
        unnamed.blend_path_on_the_machine = Some("/pv-huge/blend/pv-anim.blend".to_string());
        unnamed.the_render_start_has_been_learned = false;
        store.write_the_provisional_record(&unnamed).unwrap();

        let holds = crate::servers::the_machines_work::WhatThisRunHolds::default();
        let a_reading = |state: QueueState| {
            let connection = holds.a_connection_begins(&server.as_text());
            holds.a_reading_was_taken(
                &server.as_text(),
                connection,
                Reading::taken(
                    TheMachinesWorkingState {
                        jobs: vec![AJobAsTheMachineReadsIt {
                            job: unnamed.job.clone(),
                            state,
                            live: WhatTheEntryHasLive::NoReadings,
                            the_hold: None,
                        }],
                        the_admission_point_stands_at: None,
                        notes: Vec::new(),
                        the_history_copys_count: 0,
                        the_history_copys_entries: Vec::new(),
                    },
                    std::time::SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(1_754_700_000),
                    Clock::ThisComputer,
                ),
                None,
            );
        };

        a_reading(QueueState::Waiting);
        let held = what_is_held(&holds, Some(&record), None);
        assert_eq!(held.lines[0].rows[0].name, "pv-anim.blend", "the waiting row has no name");
        assert_eq!(held.entry.as_ref().expect("the entry").name, "pv-anim.blend");

        a_reading(QueueState::Rendering { operation: Identity::issue(IdentityKind::Operation).unwrap() });
        let doing = what_the_servers_are_doing(&holds, Some(&record));
        let _ = std::fs::remove_dir_all(&path);
        assert_eq!(
            doing.get(&server.as_text()),
            Some(&WhatAServerIsDoing::Rendering { name: "pv-anim.blend".to_string(), waiting: 0 }),
            "the work column names nothing while the job renders"
        );
    }

    /// **The ending Smoozy learned is written on the job's record**, the way
    /// the read-forward writes it: through the note's fill-in.
    pub(crate) fn its_ending_is_learned(
        store: &crate::record::job_records::JobRecords<'_>,
        job: &smoozy_state::identities::Identity,
        label: &str,
    ) {
        use smoozy_queue_history::acting::note::{TheEndingOnTheNote, TheNote, TheOutcomeAsWritten};
        let mut note = TheNote::created_with_the_job_record(label);
        note.ending = Some(TheEndingOnTheNote {
            outcome: TheOutcomeAsWritten::Finished,
            exit_status: None,
            the_attempt_whose_capture_record_holds_the_words: 1,
            the_reason_the_machine_recorded: None,
            no_render_began: false,
            the_device_line_as_observed: None,
        });
        store.the_fill_in(job, &note).expect("the fill-in is written");
        assert!(store.read(job).unwrap().unwrap().the_ending_learned().is_some());
    }

    /// **Step 10c's ninth pass, fault 2: Queue kept a finished job as
    /// *Rendering*, with *Stop*, eleven minutes after it ended, and the rail
    /// kept its *1*.** The read-forward had learned the ending — History and
    /// *What happened to your jobs* both said so — but the queue reading was
    /// older than the ending and the row was dropped only on the reading's own
    /// word. **`Queue & History` C11: when the job ends it leaves the queue**,
    /// and the ending is the one Smoozy learns, whatever an older reading
    /// still says.
    #[test]
    fn a_job_whose_ending_smoozy_learned_has_left_the_queue() {
        use smoozy_queue_history::line::QueueState;
        use smoozy_state::identities::{Identity, IdentityKind};

        let mut path = std::env::temp_dir();
        path.push(format!("smoozy-queue-learned-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).unwrap();
        let record = crate::record::LocalRecord::open_at(&path.join("local-record.sqlite3")).unwrap();
        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        let store = crate::record::job_records::JobRecords::over(record.connection());
        let finished = crate::record::deletions::tests::a_record_in_history(&server, "p12h9 long2");
        store.write_the_provisional_record(&finished).unwrap();
        let holds = crate::servers::the_machines_work::WhatThisRunHolds::default();
        a_reading_of(
            &holds,
            &server,
            vec![(
                finished.job.clone(),
                QueueState::Rendering { operation: Identity::issue(IdentityKind::Operation).unwrap() },
            )],
        );

        // **Before the ending is learned the reading is the truth**, and the
        // job is drawn rendering, with *Stop*, and counted in the rail.
        let before = what_is_held(&holds, Some(&record), None);
        assert_eq!(before.jobs, 1);
        assert_eq!(
            before.entry.as_ref().and_then(|entry| entry.stop_or_cancel),
            Some(TheStopOrCancel::Stop)
        );
        assert!(said(&the_queue_says(&before)).contains("theRailsFigure"));

        // **The read-forward learns the ending; the reading is not taken again.**
        its_ending_is_learned(&store, &finished.job, "p12h9 long2");
        let after = what_is_held(&holds, Some(&record), None);
        assert_eq!(after.jobs, 0, "a job whose ending is learned is not counted");
        assert!(after.lines.is_empty(), "nor drawn");
        assert!(after.entry.is_none(), "nor opened, with Stop");
        assert_eq!(after.servers, 1, "the server's reading still stands");
        assert!(
            !said(&the_queue_says(&after)).contains("theRailsFigure"),
            "nor in the rail's figure"
        );
        drop(record);
        let _ = std::fs::remove_dir_all(&path);
    }

    /// **Step 10c's sixth pass: *0 jobs · 0 servers* with a server connected.**
    /// A server whose queue was read and found empty is one server with
    /// nothing queued, and the header says so.
    #[test]
    fn a_server_read_with_nothing_queued_is_counted_as_a_server() {
        use smoozy_queue_history::acting::adoption::TheMachinesWorkingState;
        use smoozy_state::identities::{Identity, IdentityKind};
        use smoozy_state::observed::{Clock, Reading};

        let mut path = std::env::temp_dir();
        path.push(format!("smoozy-queue-empty-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).unwrap();
        let record = crate::record::LocalRecord::open_at(&path.join("local-record.sqlite3")).unwrap();
        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        let holds = crate::servers::the_machines_work::WhatThisRunHolds::default();
        assert_eq!(what_is_held(&holds, Some(&record), None).servers, 0, "nothing read yet");
        let connection = holds.a_connection_begins(&server.as_text());
        holds.a_reading_was_taken(
            &server.as_text(),
            connection,
            Reading::taken(
                TheMachinesWorkingState {
                    jobs: Vec::new(),
                    the_admission_point_stands_at: None,
                    notes: Vec::new(),
                    the_history_copys_count: 0,
                    the_history_copys_entries: Vec::new(),
                },
                std::time::SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(1_758_900_000),
                Clock::ThisComputer,
            ),
            None,
        );
        let held = what_is_held(&holds, Some(&record), None);
        assert_eq!((held.jobs, held.servers), (0, 1));
        assert!(held.lines.is_empty(), "no line is drawn for a queue with nothing in it");
        drop(record);
        let _ = std::fs::remove_dir_all(&path);
    }

    #[test]
    fn nothing_is_held_until_a_machine_has_been_read() {
        let holds = crate::servers::the_machines_work::WhatThisRunHolds::default();
        assert_eq!(what_is_held(&holds, None, None), TheQueueOnScreen::default());
        assert!(what_is_held(&holds, None, None).lines.is_empty());
        assert!(what_is_held(&holds, None, Some("a-job")).entry.is_none());
    }

    /// **The entry's *Download all* and *Download all to…* go to Transfers, and
    /// its narrowing opens the format boxes** — `Design/Queue & History`:
    /// *"Downloads appear in Transfers"*, and the drawing's own `data-go="t"`
    /// and `data-go="fmt"`. Both are handed the job's identity, which travels
    /// as a value. The history entry is drawn by the same builder, so it is
    /// held by the same lines.
    #[test]
    fn the_downloads_go_to_transfers_and_the_narrowing_opens_the_format_box() {
        let entry = TheJobEntry {
            job: "job-7f3a".to_string(),
            name: "Kitchen — final".to_string(),
            reach_backs: true,
            downloading: Some(TheDownloading { on: true, narrowing: "EXR".to_string(), done: 1, of: 2 }),
            ..TheJobEntry::default()
        };
        let text = said(&the_entry_is_said(the_queue_says(&a_queue()), Some(&entry)));
        assert!(text.contains("entryJob"), "the job's identity travels on the event");
        assert!(text.contains("job-7f3a"));

        let page = include_str!("../../web/the-queue.js");
        assert!(page.contains("import { aTransferIsAskedFor, theFormatBoxIsOpenedFor } from \"./transfers.js\";"));
        assert!(page.contains("const DOWNLOAD_ALL = \"download-all\";"));
        assert!(page.contains("const DOWNLOAD_ALL_TO = \"download-all-to\";"));
        assert!(page.contains("aTransferIsAskedFor(DOWNLOAD_ALL, job)"));
        assert!(page.contains("aTransferIsAskedFor(DOWNLOAD_ALL_TO, job)"));
        assert!(page.contains("narrowing.addEventListener(\"click\", () => theFormatBoxIsOpenedFor(job));"));
        assert_eq!(page.matches("const job = said.entryJob || \"\";").count(), 2);
        assert!(include_str!("../../web/the-history.js").contains("theEntryIsDrawn(detail, said, {"));
    }

    /// **`Telegram` V1c: the entry's *Telegram* group** — drawn for a live
    /// job where Telegram is on, never for an ended one or with Telegram off;
    /// unavailable while not connected; on an older server its sentence; a
    /// pair sent as its range. The twenty-first notes, note 7.
    #[test]
    fn twenty_first_7_the_telegram_group_draws_for_a_live_job_and_says_where_it_stands() {
        use smoozy_queue_history::line::QueueState;
        use smoozy_state::identities::{Identity, IdentityKind};
        let mut path = std::env::temp_dir();
        path.push(format!("smoozy-queue-telegram-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).unwrap();
        let record = crate::record::LocalRecord::open_at(&path.join("local-record.sqlite3")).unwrap();
        let connection = record.connection();
        let job = Identity::issue(IdentityKind::Job).unwrap();
        let server = Identity::issue(IdentityKind::LocalServer).unwrap().as_text();
        let ended = QueueState::Ended { outcome: Identity::issue(IdentityKind::Operation).unwrap() };
        assert_eq!(the_telegram_group_on(connection, &job, &server, &QueueState::Waiting, false, None), None, "Telegram off");
        crate::record::settings::telegram_was_switched(connection, true).unwrap();
        assert_eq!(the_telegram_group_on(connection, &job, &server, &ended, false, None), None, "an ended job");
        let stale = the_telegram_group_on(connection, &job, &server, &QueueState::Waiting, true, None).unwrap();
        assert_eq!(stale.stands, WhereTheTelegramControlStands::NotConnected);
        let unread = the_telegram_group_on(connection, &job, &server, &QueueState::Waiting, false, None).unwrap();
        assert_eq!(unread.stands, WhereTheTelegramControlStands::NotYetKnown, "nothing guessed");
        // **`Server Setup` G2a: read as absent, the job does not read as if it
        // will report; read as present, or not read, as it stood.**
        let asked = crate::servers::published::a_reading_of_the_telegram_role_begins(&server);
        crate::servers::published::a_reading_of_the_telegram_role_came_back(&server, asked, Some(false));
        let absent = the_telegram_group_on(connection, &job, &server, &QueueState::Waiting, false, None).unwrap();
        assert_eq!(absent.stands, WhereTheTelegramControlStands::TelegramIsNotInstalled);
        let said_absent = said(&the_entry_is_said(
            the_queue_says(&a_queue()),
            Some(&TheJobEntry { telegram: Some(absent), ..TheJobEntry::default() }),
        ));
        assert!(said_absent.contains("\"not-installed\"") && said_absent.contains("\"telegramNotInstalled\""), "{said_absent}");
        assert!(said_absent.contains("reporting not"), "{said_absent}");
        assert!(!said_absent.contains("telegramOlder"), "{said_absent}");
        let stale_absent = the_telegram_group_on(connection, &job, &server, &QueueState::Waiting, true, None).unwrap();
        assert_eq!(stale_absent.stands, WhereTheTelegramControlStands::NotConnected);
        crate::servers::published::a_reading_of_the_telegram_role_came_back(&server, asked, None);
        crate::servers::published::the_servers_release_is_read(&server, Some(1812));
        let older = the_telegram_group_on(connection, &job, &server, &QueueState::Waiting, false, None).unwrap();
        assert_eq!(older.stands, WhereTheTelegramControlStands::TheServersSmoozyIsOlder);
        crate::servers::published::the_servers_release_is_read(&server, Some(1813));
        let range = Identity::issue(IdentityKind::Job).unwrap().as_text();
        let offered =
            the_telegram_group_on(connection, &job, &server, &QueueState::Waiting, false, Some(range.clone())).unwrap();
        assert_eq!(offered.stands, WhereTheTelegramControlStands::Offered);
        assert_eq!(offered.sent_as, range, "one control for a pair, sent as its range");

        // **What the page is told.**
        let said_of = |group: TheTelegramGroup| {
            said(&the_entry_is_said(
                the_queue_says(&a_queue()),
                Some(&TheJobEntry { telegram: Some(group), ..TheJobEntry::default() }),
            ))
        };
        let text = said_of(TheTelegramGroup {
            picked: Some(smoozy_render_settings::reporting::ReportingChoice::APictureEveryFewMinutes { minutes: 7 }),
            ..older
        });
        assert!(text.contains("\"telegramOlder\"") && text.contains("Update Smoozy on this server"), "{text}");
        assert!(text.contains("\"older\""), "{text}");
        assert!(text.contains("\"telegramPicked\": Value(Verbatim(\"a-picture-every-few-minutes\"))"), "{text}");
        assert!(text.contains("telegramMinutes"), "{text}");
        let text = said_of(TheTelegramGroup { picked: None, ..stale });
        assert!(text.contains("\"not-connected\""), "{text}");
        assert!(!text.contains("telegramOlder") && !text.contains("telegramPicked"), "{text}");
        assert!(!said(&the_entry_is_said(the_queue_says(&a_queue()), Some(&TheJobEntry::default())))
            .contains("telegramHeading"));

        // **What a pick and a typed minutes entry send**, and the refusal.
        use smoozy_render_settings::reporting::ReportingChoice;
        assert_eq!(what_the_telegram_control_sends("nothing", None), Ok("\"Nothing\"".to_string()));
        assert_eq!(
            what_the_telegram_control_sends(
                crate::surface::render_settings::A_PICTURE_EVERY_FEW_MINUTES,
                Some(ReportingChoice::APictureEveryFewMinutes { minutes: 12 })
            ),
            Ok("{\"APictureEveryFewMinutes\":{\"minutes\":12}}".to_string())
        );
        assert_eq!(
            what_the_telegram_control_sends("minutes:61", None),
            Err(crate::surface::render_settings::words::WHOLE_MINUTES_1_TO_60)
        );
        assert_eq!(
            what_the_telegram_control_sends("minutes: 30 ", None),
            Ok("{\"APictureEveryFewMinutes\":{\"minutes\":30}}".to_string())
        );

        // **The page**: the group under *Downloading*, unavailable unless
        // offered, saving on the pick, the minutes through the same act.
        let page = include_str!("../../web/the-queue.js");
        assert!(page.contains("theDownloadingIsDrawn(thePair, said);\n  theTelegramIsDrawn(thePair, said);"));
        assert!(page.contains("const offered = said.telegramStands === \"offered\";"));
        assert!(page.contains("face.disabled = !offered;"));
        assert!(page.contains("anActIsAskedFor(face, \"what-it-reports\", { job, from: choice.value })"));
        assert!(page.contains("from: \"minutes:\" + field.value"));
        assert!(page.contains("if (said.telegramOlder || said.telegramNotInstalled) {"));
        assert_eq!(TheActOnAJob::WhatItReports.name(), "what-it-reports");
        let _ = std::fs::remove_dir_all(&path);
    }

    /// **X7: every figure travels beside its words and never inside them.**
    ///
    /// The heading's two counts are the case the drawing writes as one string —
    /// *9 jobs · 3 servers* — so this is the join the split exists against.
    #[test]
    fn every_figure_travels_beside_its_words_and_never_inside_them() {
        let event = the_queue_says(&a_queue());
        let text = said(&event);
        assert!(text.contains("theJobsFigure"));
        assert!(text.contains("theJobsWord"));
        assert!(
            !text.contains("9 jobs"),
            "a figure was joined into its own words before the typography point. \
             **X7: the point never touches a name, a path, a host, a port or a \
             figure** — the words go on the event beside the value"
        );
        assert!(!text.contains("3 servers"));
    }

    /// **A row's second line is parts and never a sentence.** *Interior · 240
    /// frames · frame 148* is a scene name, a figure with a word after it and a
    /// word with a figure after it — three values and two words, and the
    /// drawing joins them.
    #[test]
    fn a_rows_second_line_is_parts_and_never_one_string() {
        let text = said(&the_queue_says(&a_queue()));
        assert!(text.contains("row0Sub0Value"));
        assert!(text.contains("row0Sub1Value"));
        assert!(text.contains("row0Sub1Trail"));
        assert!(text.contains("row0Sub2Lead"));
        assert!(!text.contains("240 frames"));
        assert!(!text.contains("frame 148"));
    }

    /// **The count of a list is on the event, so the page never draws one
    /// short.** L8a: there is no state in which a list is drawn short.
    #[test]
    fn every_list_says_how_many_it_held() {
        let text = said(&the_queue_says(&a_queue()));
        assert!(text.contains("lineCount"));
        assert!(text.contains("rowCount"));
    }

    /// **The singular and the plural are the application's choice, not the
    /// page's.** A page that wrote *s* on the end of a word would be composing
    /// prose, which A6 forbids.
    #[test]
    fn one_job_is_a_job_and_not_one_jobs() {
        let one = TheQueueOnScreen {
            jobs: 1,
            servers: 1,
            ..TheQueueOnScreen::default()
        };
        let text = said(&the_queue_says(&one));
        assert!(text.contains(THE_JOB_WORD));
        assert!(text.contains(THE_SERVER_WORD));
    }

    /// **No word of prose in this file is a metaphor and none of the three
    /// states Design names has been reworded.** Rule 9, over the one column
    /// Design says carries the states that are not readable from position.
    #[test]
    fn the_three_states_are_the_words_design_names() {
        assert_eq!(RENDERING, "Rendering");
        assert_eq!(WAITING_ON_YOU, "Needs your answer");
        assert_eq!(NOT_UPDATING, "Not updating");
        // **A fourth since held-job piece B** (`Design/Queue & History § The
        // queue list`, 2026-10-11), and the drawing's own `q-driver` row word.
        assert_eq!(WAITING_FOR_DRIVER_UPDATE, "Waiting for driver update");
        let drawing = include_str!("../../../../Preparation/Design/Mockup/Smoozy.html");
        assert!(drawing.contains("<option value=\"q-driver\">"));
        assert!(drawing.contains(&format!("<span class=\"qst held\">{WAITING_FOR_DRIVER_UPDATE}</span>")));
        assert!(drawing.contains(&format!("<b>{WAITING_FOR_DRIVER_UPDATE}</b>")));
    }

    /// **Held-job piece B: the page draws a held box's buttons only where they
    /// are sent**, so a job held for the driver update gets its box and no
    /// button; a mark's bold takes the parts the application says; and words
    /// that open on a comma or a full stop follow their value with no space.
    #[test]
    fn the_page_draws_a_held_box_with_no_buttons_a_marks_bold_and_clinging_words() {
        let page = include_str!("../../web/the-queue.js");
        assert!(page.contains("    if (firstParts) {\n"), "the first answer is drawn whether or not it is sent");
        assert!(page.contains("    if (said[\"held\" + at + \"Answer\"]) {\n"));
        assert!(page.contains("const inTheBold = howMany(said, \"mark\" + at + \"Bold\");"));
        assert!(page.contains("said[base + \"TrailClings\"]"));

        // **The event says it**: a value followed by *, was too old* clings,
        // a figure followed by *frames* binds, and a name followed by words
        // does neither.
        let event = a_part_is_said(
            Event::about(crate::events::Subject::TheRun, "a test"),
            "l",
            "v",
            "t",
            "b",
            &APart::said("", "525.147.05", ", was too old for OptiX in Blender"),
        );
        assert!(said(&event).contains("tClings"), "{}", said(&event));
        assert!(!the_trail_clings(&APart::figure("240", FRAMES_WORD)));
        assert!(!the_trail_clings(&APart::words(", alone")), "nothing to cling to");
        let marked = TheJobEntry {
            marks: vec![AMarkOnTheEntry {
                said: "The NVIDIA driver,",
                parts: vec![APart::said("", "525.147.05", ", was")],
                way: None,
                bold: 1,
            }],
            ..TheJobEntry::default()
        };
        let text = said(&the_entry_is_said(the_queue_says(&a_queue()), Some(&marked)));
        assert!(text.contains("mark0Bold") && text.contains("mark0Part0TrailClings"), "{text}");
    }

    /// **The screen says *queue*, never *line*.** Design is explicit: the
    /// architecture calls it the line throughout and no word of that reaches a
    /// screen.
    #[test]
    fn no_word_the_user_reads_calls_the_queue_a_line() {
        let source = include_str!("the_queue.rs");
        let (working, _) = source
            .split_once("#[cfg(test)]")
            .expect("the tests are the last thing in this file");
        for (at, _) in working.match_indices("const ") {
            let rest = &working[at..];
            let Some(opens) = rest.find(" = \"") else {
                continue;
            };
            let Some(closes) = rest[opens + 4..].find('"') else {
                continue;
            };
            let words = &rest[opens + 4..opens + 4 + closes];
            let lowered = words.to_lowercase();
            for forbidden in [" line ", " line.", "the line", "renderlet"] {
                assert!(
                    !lowered.contains(forbidden),
                    "a word the user reads says {forbidden:?}: {words:?}. **Design: \
                     the screen says queue, never line, and neither bar ever says \
                     renderlet** — both are this project's own words and no Blender \
                     user has heard the second"
                );
            }
        }
    }

    /// **R4: a row hands back its job, and the page asks for that job's
    /// entry** — `Design/Queue & History § Keyboard shortcuts`, *"Return opens
    /// the chosen entry"*. A row is a button, so Return is its press. **The
    /// delete key is bound to nothing here**: on a queue entry the act is
    /// Cancel, and that is not a key press.
    #[test]
    fn a_row_is_chosen_by_its_job_and_the_delete_key_does_nothing() {
        let says = TheQueueOnScreen {
            lines: vec![AServersLine {
                name: "RunPod 4090".to_string(),
                rows: vec![ARowInTheQueue {
                    job: "job-7".to_string(),
                    name: "Kitchen — final".to_string(),
                    ..ARowInTheQueue::default()
                }],
                ..AServersLine::default()
            }],
            ..TheQueueOnScreen::default()
        };
        let text = said(&the_queue_says(&says));
        assert!(text.contains("row0Job"), "the row does not carry its job");
        assert!(text.contains("job-7"));
        let page = include_str!("../../web/the-queue.js");
        // **The press that ends a drag is not a choice** (the 38's piece 8), so
        // the click is guarded before it chooses.
        assert!(page.contains("row.addEventListener(\"click\", () => {"));
        assert!(page.contains("    theRowIsChosen(job);\n"));
        assert!(page.contains("shell.core.invoke(\"the_queue\", { chosen: theChosenJob, asking: null });"));
        assert!(!page.contains("\"Delete\""), "the queue binds the delete key");
        assert!(!page.contains("\"Backspace\""), "the queue binds the delete key");
    }

    /// **Must-do 5: every control either acts or says why it cannot**, and
    /// every sentence is written exactly once.
    ///
    /// **The guard is *no sentence twice*, not *one sentence*.** It was the
    /// second of those until step 10b piece 8, when three acts became acts and
    /// the one sentence left over turned out to be covering four different
    /// missing things. **Copying a sentence is still what this fails on** —
    /// that is the fault the runner's item R3 found in this very file — but a
    /// second, different sentence for a second, different reason is the
    /// opposite of that fault.
    #[test]
    fn every_act_answers_in_a_sentence_that_already_exists() {
        for act in TheActOnAJob::ALL {
            let parsed: TheActOnAJob = serde::Deserialize::deserialize(
                serde::de::value::StrDeserializer::<serde::de::value::Error>::new(act.name()),
            )
            .unwrap_or_else(|_| panic!("{} is not a closed act", act.name()));
            assert_eq!(parsed, act);
            assert!(!act.answers().is_empty());
        }
        // **Owed piece 10: *Track it* acts** (`Queue & History` A6), and what a
        // press meets with nothing adopted is the server not reached — said
        // in the queue's own words for that, never *not built*.
        assert_eq!(TheActOnAJob::TrackIt.answers(), SMOOZY_COULD_NOT_REACH_THE_SERVER);
        assert_eq!(TheActOnAJob::BlenderOutput.answers(), crate::surface::files::A_CONNECTION_IS_NOT_BUILT);
        assert_eq!(
            where_an_act_on_a_job_goes(TheActOnAJob::TrackIt),
            WhereAnActOnAJobGoes::TheWorkIsTracked
        );
        // **Step 10c's S3: *Stop* acts**, and answers only where the job has
        // left the reading its box was raised over.
        assert_eq!(TheActOnAJob::Stop.answers(), THIS_JOB_IS_NO_LONGER_IN_THE_QUEUE);
        // **The 38's piece 2: the two ways to render again act**, and the one
        // thing every press can meet before anything is read is the record
        // not answering; the way to an earlier entry meets that entry gone.
        assert_eq!(
            TheActOnAJob::CarryOn.answers(),
            crate::a_new_job_from_an_old_one::words::THE_RECORD_WOULD_NOT_READ
        );
        // **The 38's piece 15: *Dismiss* acts**, and meets only the record
        // refusing it.
        assert_eq!(
            TheActOnAJob::DismissTheNotice.answers(),
            crate::surface::the_history::THE_RECORD_REFUSED_IT
        );
        assert_eq!(
            where_an_act_on_a_job_goes(TheActOnAJob::DismissTheNotice),
            WhereAnActOnAJobGoes::TheNoticeIsDismissed
        );
        assert_eq!(TheActOnAJob::TheEarlierEntry.answers(), THAT_ENTRY_WAS_DELETED);
        // **The 38's piece 9: the entry's two Transfers controls act**, and
        // answer only what a press can meet with nothing done.
        assert_eq!(
            TheActOnAJob::FramesAsTheyRender.answers(),
            crate::surface::settings::words::THE_SETTING_DID_NOT_SAVE
        );
        assert_eq!(
            TheActOnAJob::FilesOnYourComputer.answers(),
            crate::surface::transfers::words::THE_JOB_WOULD_NOT_READ
        );
        assert_eq!(
            where_an_act_on_a_job_goes(TheActOnAJob::FramesAsTheyRender),
            WhereAnActOnAJobGoes::TheArrangementIsSwitchedOff
        );
        assert_eq!(
            where_an_act_on_a_job_goes(TheActOnAJob::FilesOnYourComputer),
            WhereAnActOnAJobGoes::TheJobsFilesAreShown
        );
        // **No *cannot do this yet* sentence is written twice in this file**,
        // and every one of them is a whole sentence rather than a lead somebody
        // joined a name onto.
        let (working, _) = include_str!("the_queue.rs").split_once("#[cfg(test)]").expect("tests last");
        let mut written: Vec<&str> = Vec::new();
        for (at, _) in working.match_indices("Smoozy cannot do this yet") {
            let rest = &working[at..];
            let end = rest.find('"').unwrap_or(rest.len());
            written.push(&rest[..end]);
        }
        assert!(
            written.is_empty(),
            "this file names {written:?}, and defines none: the 38's piece 2 took \
             the two about a new job away and the 38's piece 15 the catch-up notice's"
        );
        let mut distinct = written.clone();
        distinct.sort_unstable();
        distinct.dedup();
        assert_eq!(
            distinct.len(),
            written.len(),
            "a second copy of a *cannot do this yet* sentence was written here"
        );
        // **The answer names the act it is about**, so the page draws it beside
        // the control that was pressed and nowhere else.
        let text = said(&the_answer_says(&TheActOnAJob::TheEarlierEntry));
        assert!(text.contains(&format!("{:?}", Verbatim::new("the-earlier-entry"))));
        assert!(text.contains(&format!("{:?}", Prose::new(THAT_ENTRY_WAS_DELETED))));
    }

    /// **The entry's *Downloading* group is the arrangement the record keeps**
    /// — `Queue & History` L1a: the switch, its narrowing and the count — and
    /// no arrangement is a switch that is off. The 38's piece 9, item 9: until
    /// then nothing composed the group, so neither control was drawn.
    #[test]
    fn the_entrys_downloading_group_reads_the_arrangement() {
        use smoozy_download::arrangement::{AReckoningEntry, AnArrangement, KeepsTheArrangement as _, TheNarrowing, WhatSmoozyDid};
        use smoozy_state::identities::{Identity, IdentityKind};
        let folder = std::env::temp_dir().join(format!("smoozy-queue-downloading-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a folder");
        let record = crate::record::LocalRecord::open_at(&folder.join("local-record.sqlite3")).expect("a record");
        let job = Identity::issue(IdentityKind::Job).expect("random bytes");
        let row = Identity::issue(IdentityKind::LocalServer).expect("random bytes");
        assert_eq!(the_downloading_on(record.connection(), &job, None), TheDownloading::default());

        let mut arrangement = AnArrangement::switched_on(
            job.clone(),
            row,
            b"/Users/artist/x".to_vec(),
            TheNarrowing::TheseFormats(vec![b"OPEN_EXR_MULTILAYER".to_vec(), b"PNG".to_vec()]),
        );
        for (name, state) in [(b"a".as_slice(), WhatSmoozyDid::Fetched), (b"b", WhatSmoozyDid::Owed), (b"c", WhatSmoozyDid::Fetched)] {
            arrangement.reckoning.push(AReckoningEntry::acted_on(name.to_vec(), None, state, b"/Users/artist/x".to_vec()));
        }
        crate::record::download_records::DownloadArrangements::over(record.connection())
            .write_it_down(&arrangement)
            .expect("written");
        assert_eq!(
            the_downloading_on(record.connection(), &job, None),
            TheDownloading { on: true, narrowing: "Multilayer EXR, PNG".into(), done: 2, of: 3 }
        );
        // **Smoozy's words on the entry, Blender's identifiers in the record**
        // (the Creator's second notes, piece 2's section 6, ruled into piece
        // 6): the arrangement still holds what the box hands back.
        assert_eq!(
            crate::record::download_records::DownloadArrangements::over(record.connection())
                .the_arrangement_for(&job)
                .unwrap()
                .unwrap()
                .narrowing,
            TheNarrowing::TheseFormats(vec![b"OPEN_EXR_MULTILAYER".to_vec(), b"PNG".to_vec()])
        );
        // **`Download` N6's whole, where a pass has read it, is the count.**
        assert_eq!(the_downloading_on(record.connection(), &job, Some(240)).of, 240);
        arrangement.switched_off();
        arrangement.narrowing = TheNarrowing::Everything;
        crate::record::download_records::DownloadArrangements::over(record.connection())
            .write_it_down(&arrangement)
            .expect("written");
        let off = the_downloading_on(record.connection(), &job, None);
        assert!(!off.on);
        assert!(off.narrowing.is_empty(), "every format is said in words, not as a value");
        let _ = std::fs::remove_dir_all(&folder);
    }

    /// **Every act on a job has a route, and which of them are owed is one fact
    /// in one place.**
    ///
    /// **This is `src/surface/servers.rs`'s guard, over this screen's own acts**
    /// — and it is what says the two screens' headers have stopped being true.
    /// Three acts landed with step 10b piece 8 and are gone from the list below
    /// by name; a fourth that gains a route, or a control added with none,
    /// fails here rather than on the user's press.
    #[test]
    fn every_act_on_a_job_either_acts_or_is_named_here_as_owed() {
        let mut owed: Vec<&str> = TheActOnAJob::ALL
            .into_iter()
            .filter(|act| where_an_act_on_a_job_goes(*act) == WhereAnActOnAJobGoes::ItAnswers)
            .map(TheActOnAJob::name)
            .collect();
        owed.sort_unstable();
        assert_eq!(
            owed,
            vec![
                "blender-output",
            ],
            "**an act on a job has no route, or one that had none has gained \
             one.** The two screens' headers name what is owed and why, and a \
             list that has moved without them moving with it is the fault this \
             build stage keeps meeting: a fact kept in two places"
        );
        // **The three that act are the three the store made possible**, and
        // none of them answers a sentence about something not being built.
        for act in [
            TheActOnAJob::DeleteTheEntry,
            TheActOnAJob::ClearThisServersHistory,
            TheActOnAJob::ClearAllHistory,
            TheActOnAJob::ClearRemovedServersHistory,
        ] {
            assert_ne!(where_an_act_on_a_job_goes(act), WhereAnActOnAJobGoes::ItAnswers);
            assert!(!act.answers().contains("cannot do this yet"));
        }
    }

    /// **Must-do 19: *Ignore* is remembered for as long as the offer stands,
    /// and no longer.**
    ///
    /// **The second half is the one worth a test.** A decline that outlived the
    /// connected period would silence an offer a later connection made afresh,
    /// and nothing on screen would say why the banner had stopped appearing.
    #[test]
    fn ignoring_the_untracked_work_hides_that_offer_and_no_other() {
        let offer = |server: &str| TheQueueOnScreen {
            untracked: Some(TheUntrackedWork {
                server: "the big box".to_string(),
                server_row: server.to_string(),
                rendering: 1,
                waiting: 2,
                ..TheUntrackedWork::default()
            }),
            ..TheQueueOnScreen::default()
        };
        let drawn = |declined: &mut Option<String>, mut held: TheQueueOnScreen| {
            the_decline_is_applied(&mut held, declined);
            held.untracked.is_some()
        };
        let mut declined = None;
        // Nothing declined: the offer is drawn.
        assert!(drawn(&mut declined, offer("box-one")));
        declined = Some("box-one".to_string());
        assert!(!drawn(&mut declined, offer("box-one")));
        // Still declined on the next reading, which is the whole point.
        assert!(!drawn(&mut declined, offer("box-one")));
        // **A different server's offer is a different offer**, and taking it
        // drops the decline the earlier one carried.
        assert!(drawn(&mut declined, offer("box-two")));
        assert_eq!(declined, None);
        assert!(drawn(&mut declined, offer("box-one")));
        // **A reading with no offer at all drops the decline**, which is what
        // the end of a connected period produces: `Queue & History` C6 drops
        // the whole period-held reading whole.
        declined = Some("box-one".to_string());
        assert!(!drawn(&mut declined, TheQueueOnScreen::default()));
        assert_eq!(declined, None);
        assert!(drawn(&mut declined, offer("box-one")));

        // **The page's own route is named here and not driven**, the register
        // being one value the whole run shares: a test that set it would decide
        // what another test running beside it saw, which is the flake this
        // stage has already spent a sitting on.
        assert!(include_str!("the_queue.rs").contains("the_untracked_work_is_ignored"));
        assert!(include_str!("../../web/the-queue.js").contains("the_untracked_work_is_ignored"));
    }

    /// **Must-do 6: a decline does not survive the end of its period, with no
    /// reading drawn between** — and it is keyed by the server, not its name.
    ///
    /// **This failed before the fix** in both halves: nothing dropped a
    /// decline at a period's end, so a disconnect and a reconnect with no queue
    /// drawn between left the old decline to hide the next connection's offer;
    /// and the decline named the server's drawn name, so two servers called
    /// *the big box* hid each other's offers.
    #[test]
    fn a_decline_ends_with_its_period_and_names_the_server_not_its_name() {
        let offer = |row: &str| TheQueueOnScreen {
            untracked: Some(TheUntrackedWork {
                server: "the big box".to_string(),
                server_row: row.to_string(),
                rendering: 1,
                ..TheUntrackedWork::default()
            }),
            ..TheQueueOnScreen::default()
        };
        let drawn = |declined: &mut Option<String>, mut held: TheQueueOnScreen| {
            the_decline_is_applied(&mut held, declined);
            held.untracked.is_some()
        };

        // Declined during the first period.
        let mut declined = Some("row-one".to_string());
        assert!(!drawn(&mut declined, offer("row-one")));
        // **Another server's period ending leaves it standing.**
        the_period_ending_drops_the_decline(&mut declined, "row-two");
        assert_eq!(declined.as_deref(), Some("row-one"));
        // **Its own period ends, and no reading is drawn in between** — the
        // next connection's offer is drawn.
        the_period_ending_drops_the_decline(&mut declined, "row-one");
        assert!(drawn(&mut declined, offer("row-one")));

        // **Two servers of one name are two offers.**
        let mut declined = Some("row-one".to_string());
        assert!(drawn(&mut declined, offer("row-two")));

        // **The page sends the identifier, and the event carries it.**
        let text = format!(
            "{:?}",
            the_queue_says(&offer("row-one"))
        );
        assert!(text.contains("theUntrackedServerRow"), "{text}");
        let page = include_str!("../../web/the-queue.js");
        assert!(page.contains("server: said.theUntrackedServerRow"));
        assert!(!page.contains("server: said.theUntrackedServer ||"));
    }

    /// **Owed piece 10: *Track them* hands back the server's identifier**, the
    /// one the banner was drawn for, never the name above it — `Queue &
    /// History` A5's offer is one per server, and two servers may share a
    /// name. [`an_act_on_a_job`] reads it from `group`, as the part banner's
    /// control hands its server.
    #[test]
    fn track_it_hands_back_the_servers_identifier() {
        let page = include_str!("../../web/the-queue.js");
        assert!(page.contains(r#"itAnswers(track, "track-it", { group: said.theUntrackedServerRow || "" });"#));
        assert!(!page.contains(r#"itAnswers(track, "track-it");"#));
    }

    /// **Must-do 5, on the page: no control is drawn that does nothing.** Every
    /// button either has a press handler, is the drawing's disabled control with
    /// its reason beside it, or is a seam the panes' own service drags.
    #[test]
    fn no_button_on_either_screen_is_drawn_dead() {
        for (name, page) in [
            ("the-queue.js", include_str!("../../web/the-queue.js")),
            ("the-history.js", include_str!("../../web/the-history.js")),
        ] {
            assert!(page.contains("theSeamsAreLive("), "web/{name} draws a seam nothing drags");
            for (at, _) in page.match_indices("aButton(") {
                // The maker itself, and the call inside it.
                let line_at = page[..at].rfind('\n').map(|found| found + 1).unwrap_or(0);
                let line = &page[line_at..page[at..].find('\n').map(|found| at + found).unwrap_or(page.len())];
                if line.contains("function aButton") {
                    continue;
                }
                let held = line.trim_start().strip_prefix("const ").and_then(|rest| rest.split(' ').next());
                let held = held.unwrap_or_else(|| {
                    panic!(
                        "web/{name} makes a control inline at {line:?}. **A control that cannot act \
                         says why it cannot** — hold it, and give it a press handler or the \
                         drawing's disabled shape"
                    )
                });
                let rest = &page[at..];
                let answers = rest.contains(&format!("{held}.addEventListener(\"click\""))
                    || rest.contains(&format!("itAnswers({held}"))
                    || rest.contains(&format!("{held}.disabled = true"))
                    || rest.contains(&format!("{held}.dataset.seam"));
                assert!(
                    answers,
                    "web/{name}'s control {held:?} has neither a press handler nor a stated reason. \
                     **Rule 8: going quiet is always a fault**"
                );
            }
        }
    }

    /// **Must-do 6, and Design, *The catch-up notice*: a dropped job's line's
    /// *Dismiss* asks first**, for that line's job and naming that job, and
    /// the reading says which job the box is about so its go-ahead hands it
    /// back; a job no line names raises nothing. *(Until the Creator's ninth
    /// notes, note 3, the banner's *Dismiss* asked only where it named such a
    /// job and otherwise acted on the press; that half, and the page's fold,
    /// went with the banner — the `notice.id = "catchup"` check with it.)*
    ///
    /// **What it bites on**: the box raised for the first line whatever line
    /// was pressed, a box raised for a job no line names, the go-ahead handed
    /// no job, and the banner's fold or identifier back on the page.
    #[test]
    fn a_dropped_lines_dismiss_asks_first_for_its_own_job() {
        let with_two = TheQueueOnScreen {
            notice: Some(TheCatchUpNotice {
                counted: Vec::new(),
                dropped: vec![
                    ADroppedJob {
                        job: "job-kitchen".to_string(),
                        name: "Kitchen — final v4".to_string(),
                        what: APart::words(WAS_NOT_STARTED_THE_FIRST_FRAME_WAS_NEVER_RENDERED),
                    },
                    ADroppedJob {
                        job: "job-attic".to_string(),
                        name: "Attic".to_string(),
                        what: APart::said(WAS_NOT_STARTED_ITS_BLENDER, "4.5", WAS_REMOVED),
                    },
                ],
            }),
            ..TheQueueOnScreen::default()
        };
        let asked = the_queue_as_asked(with_two.clone(), Some(TheQueueAsk::Dismiss), Some("job-attic"));
        assert_eq!(asked.asking, Some(the_dismiss_box("Attic")));
        assert_eq!(asked.asking.as_ref().unwrap().go_ahead_act, TheActOnAJob::DismissTheNotice);
        assert_eq!(asked.the_dropped_job_asked_about.as_deref(), Some("job-attic"));
        let text = said(&the_queue_says(&asked));
        assert!(text.contains("boxJob"), "the go-ahead is handed no job: {text}");
        assert!(text.contains("job-attic"));

        let nothing = the_queue_as_asked(with_two.clone(), Some(TheQueueAsk::Dismiss), Some("job-gone"));
        assert_eq!(nothing.asking, None, "a box for a job no line names");
        assert_eq!(the_queue_as_asked(with_two, Some(TheQueueAsk::Dismiss), None).asking, None);
        assert!(!said(&the_queue_says(&nothing)).contains("boxJob"));

        let page = include_str!("../../web/the-queue.js");
        assert!(!page.contains("notice.id = \"catchup\";"), "the banner's identifier is back");
        assert!(!page.contains("theNoticeIsFolded"), "the banner's fold is back");
        assert!(page.contains("aBoxIsAskedFor(dismiss, \"dismiss-the-notice\", DISMISS_IT, job)"));
        assert!(page.contains("dropped: dropped || null"));
        assert!(page.contains("job: said.boxJob || said.entryJob || \"\""));
    }

    /// **Must-do 7: a folded server heading says what Design says**, in all
    /// three cases, and the application composes every part of it.
    #[test]
    fn a_folded_heading_says_what_is_running_or_why_it_is_not() {
        let rendering = AServersLine {
            row: "row-1".to_string(),
            name: "RunPod 4090".to_string(),
            rendering: 1,
            waiting: 2,
            folded: Some(("Kitchen — final".to_string(), 62)),
            ..AServersLine::default()
        };
        let parts = the_folded_line(&rendering);
        assert_eq!(parts[0], APart::value("62%"), "the percentage lost its sign");
        assert_eq!(parts[2], APart::figure("2", WAITING_WORD));

        let waiting_only = AServersLine { folded: None, ..rendering.clone() };
        assert_eq!(the_folded_line(&waiting_only), vec![APart::figure("2", WAITING_WORD)]);

        let stale = AServersLine {
            last_seen: Some("11:04".to_string()),
            ..rendering.clone()
        };
        let parts = the_folded_line(&stale);
        assert_eq!(parts[0], APart::words(NOT_UPDATING), "a server that is not updating folds to nothing");
        assert_eq!(parts[2], APart::said(LAST_SEEN, "11:04", ""));

        let text = said(&the_queue_says(&TheQueueOnScreen {
            lines: vec![rendering],
            ..TheQueueOnScreen::default()
        }));
        assert!(text.contains("line0FoldedCount"));
        assert!(text.contains("line0Server"), "the page has nothing to keep a fold under");
        let page = include_str!("../../web/the-queue.js");
        assert!(page.contains("const theFoldedServers = new Set();"));
        assert!(page.contains("theFoldedServers.add(server)"));
        assert!(page.contains("section.dataset.open = shut ? \"false\" : \"true\";"), "every redraw opens every server again");
    }

    /// **Decision C's words, as Design has them** (`Design/Queue & History §
    /// What the job writes`, corrected 2026-10-02): *Render all — 240 frames*,
    /// *Render all* after a reopen, and *The full range has not started.* —
    /// never *the rest*, which stopped being true when the held job became the
    /// whole range (`Rendering` S13 as amended).
    #[test]
    fn the_held_pair_reads_render_all_and_the_full_range() {
        let words = |parts: &[APart]| {
            parts
                .iter()
                .map(|part| [part.lead.to_string(), part.value.clone(), part.trail.to_string()].join(" "))
                .collect::<Vec<_>>()
                .join(" ")
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ")
        };
        let held = the_first_frame_is_held("1", 240, true);
        assert_eq!(words(&held.first_answer), "Render all — 240 frames");
        assert_eq!(words(&held.said), "Frame 1 is rendered and downloaded. The full range has not started.");
        assert_eq!(
            words(&the_first_frame_is_held("1", 240, false).said),
            "Frame 1 is rendered. The full range has not started."
        );
        assert_eq!(words(&the_first_frame_is_held_with_no_count().first_answer), "Render all");
        assert_eq!(words(&the_first_frame_is_held_with_no_count().said), "Needs your answer — first frame");
        let (working, _) = include_str!("the_queue.rs").split_once("#[cfg(test)]").expect("tests last");
        assert!(!working.contains("\"Render the rest"), "the old label is still a string");
        assert!(!working.contains("The rest of the range has not started"));
    }

    /// **Must-do 8: the first-frame check names the job's own first frame.**
    /// Before the fix both sentences said *1*, whatever the job's range was.
    #[test]
    fn the_first_frame_check_names_the_jobs_own_first_frame() {
        let held = the_first_frame_is_held("1001", 239, true);
        assert_eq!(held.said, vec![APart::said(THE_FIRST_FRAME_LEAD, "1001", IS_RENDERED_AND_DOWNLOADED)]);
        assert_eq!(
            the_first_frame_is_held("1001", 239, false).said,
            vec![APart::said(THE_FIRST_FRAME_LEAD, "1001", IS_RENDERED)]
        );
        assert_eq!(held.first_answer[0].value, "239");
        assert_eq!(held.first_answer[0].trail, FRAMES_WORD);
        assert_eq!(the_first_frame_is_held("1001", 1, true).first_answer[0].trail, FRAME_WORD);
        assert_eq!(the_first_frame_was_rendered("1001").value, "1001");
        let (working, _) = include_str!("the_queue.rs").split_once("#[cfg(test)]").expect("tests last");
        assert!(!working.contains("Frame 1 is rendered"));
        assert!(!working.contains("frame 1 rendered"));
    }

    /// **Step 10c's eleventh pass, fault 4, as a person meets it at the
    /// queue: the held entry's *Render the rest* (now *Render all*) and *Cancel* answered
    /// *"Smoozy cannot do this yet. Sending instructions to the server is not
    /// built."*** `Queue & History` H2 and `Rented-server runtime` U9: the
    /// desk's answer is an ordinary command naming the hold, over the
    /// connection. Both now reach the machine and neither answers *not built*.
    #[test]
    fn the_held_jobs_two_answers_reach_the_machine_and_never_answer_not_built() {
        for act in [TheActOnAJob::TheHeldJobsFirstAnswer, TheActOnAJob::TheHeldJobsSecondAnswer] {
            assert_ne!(
                where_an_act_on_a_job_goes(act),
                WhereAnActOnAJobGoes::ItAnswers,
                "{} answers instead of acting",
                act.name()
            );
            assert_ne!(act.answers(), A_CONNECTION_IS_NOT_BUILT, "{}", act.name());
            assert!(!act.answers().contains("cannot do this yet"), "{}", act.name());
        }
    }

    /// **`Queue & History` H1 and `Queue & History` H5, and `Design/Queue &
    /// History § What the
    /// job writes`: the first-frame check's held job offers both answers on its
    /// entry, naming its count** — and says its first frame is rendered only
    /// once the first job has finished (Rule 8: never before it is true).
    ///
    /// *Until step 10c's eleventh-pass fix piece 8 nothing composed the pair
    /// at all: `the_first_frame_is_held` was called by a test and by nothing
    /// else, so a held entry read* Waiting on you *with no answer to give.*
    #[test]
    fn a_held_rest_this_run_sent_offers_both_answers_with_its_count() {
        use smoozy_queue_history::line::QueueState;
        use smoozy_state::identities::{Identity, IdentityKind};

        let mut path = std::env::temp_dir();
        path.push(format!("smoozy-queue-first-frame-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).unwrap();
        let record = crate::record::LocalRecord::open_at(&path.join("local-record.sqlite3")).unwrap();
        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        let store = crate::record::job_records::JobRecords::over(record.connection());
        let first = crate::record::deletions::tests::a_record_in_history(&server, "p11 held");
        store.write_the_provisional_record(&first).unwrap();
        let mut rest = crate::record::deletions::tests::a_record_in_history(&server, "p11 held");
        rest.the_render_start_has_been_learned = false;
        store.write_the_provisional_record(&rest).unwrap();
        crate::submitting::a_first_frame_check_is_sent(
            &rest.job,
            crate::submitting::TheFirstFrameCheck {
                the_first_job: first.job.clone(),
                the_first_frame: 1001,
                frames_held: 240,
            },
        );
        let hold = Identity::issue(IdentityKind::Operation).unwrap();
        let holds = crate::servers::the_machines_work::WhatThisRunHolds::default();
        a_reading_of(
            &holds,
            &server,
            vec![
                (first.job.clone(), QueueState::Waiting),
                (rest.job.clone(), QueueState::Held { hold: hold.clone() }),
            ],
        );

        // **While the first frame has not rendered**: both answers, and no
        // sentence saying it has.
        let before = what_is_held(&holds, Some(&record), Some(&rest.job.as_text()));
        let entry = before.entry.as_ref().expect("the held job's entry");
        assert_eq!(entry.job, rest.job.as_text());
        assert_eq!(entry.held.len(), 1, "the pair is offered: {entry:#?}");
        let pair = &entry.held[0];
        assert!(pair.said.is_empty(), "the first frame is not rendered yet: {pair:#?}");
        assert_eq!(pair.first_answer[0].lead, RENDER_ALL);
        assert_eq!(pair.first_answer[0].value, "240");
        assert_eq!(pair.second_answer, CANCEL_IT);

        // **Once the first job has finished**, the first frame is named.
        its_ending_is_learned(&store, &first.job, "p11 held");
        let after = what_is_held(&holds, Some(&record), Some(&rest.job.as_text()));
        let pair = &after.entry.as_ref().expect("the entry").held[0];
        // **Not *downloaded***: this job has no arrangement (step 10c's
        // RunPod-pass fix piece 1; this said *downloaded* until then).
        assert_eq!(pair.said, vec![APart::said(THE_FIRST_FRAME_LEAD, "1001", IS_RENDERED)]);

        // **A held job this run did not send** — a hold placed at the start of
        // a render, or a check sent before Smoozy was last opened — is not
        // given the first-frame check's words.
        let other = Identity::issue(IdentityKind::Job).unwrap();
        let mut theirs = crate::record::deletions::tests::a_record_in_history(&server, "held at its start");
        theirs.job = other.clone();
        theirs.the_render_start_has_been_learned = false;
        store.write_the_provisional_record(&theirs).unwrap();
        a_reading_of(
            &holds,
            &server,
            vec![(other.clone(), QueueState::Held { hold: Identity::issue(IdentityKind::Operation).unwrap() })],
        );
        let unknown = what_is_held(&holds, Some(&record), Some(&other.as_text()));
        assert!(unknown.entry.as_ref().expect("its entry").held.is_empty());

        // **`Queue & History` H5: offered while, and only while, the hold is
        // unanswered as far as Smoozy knows** — not while a cancel of its own
        // is in hand, not once the machine has said the hold is answered, and
        // offered again, with what happened said, where the answer reached
        // nothing.
        a_reading_of(
            &holds,
            &server,
            vec![(rest.job.clone(), QueueState::Held { hold: hold.clone() })],
        );
        let text = rest.job.as_text();
        let the_entry = || what_is_held(&holds, Some(&record), Some(&text)).entry.expect("the entry");
        crate::servers::the_queues_own_acts::it_stands(
            &text,
            Some(WhereTheActStands::InHand(TheQueuesOwnAct::TheCancel)),
        );
        assert!(the_entry().held.is_empty(), "not while Cancel is in hand");
        crate::servers::the_queues_own_acts::it_stands(&text, None);
        the_answer_stands(&text, Some(WhereTheAnswerStands::TheHoldIsAnswered));
        assert!(the_entry().held.is_empty(), "not once the hold is answered");
        the_answer_stands(&text, Some(WhereTheAnswerStands::NotReached));
        let entry = the_entry();
        assert_eq!(entry.held.len(), 1, "offered again where the answer reached nothing");
        assert_eq!(entry.status, vec![APart::words(SMOOZY_COULD_NOT_REACH_THE_SERVER)]);
        the_answer_stands(&text, None);
        drop(record);
        let _ = std::fs::remove_dir_all(&path);
    }

    /// **Step 10c's pass before Vast on RunPod, fault 2: *Frame 1 is rendered
    /// and downloaded.* was said of first-frame checks sent with *Download
    /// frames as they render* off**, and no frame of either was on the
    /// computer. **The sentence says *downloaded* only where `Download` R4's
    /// reckoning for the first job holds that frame fetched** — every entry
    /// naming it — and otherwise the drawing's own sentence less those two
    /// words (`Design/Queue & History § What the job writes`).
    #[test]
    fn a_first_frame_is_said_to_be_downloaded_only_where_it_was() {
        use crate::record::download_records::DownloadArrangements;
        use smoozy_download::arrangement::{
            AReckoningEntry, AnArrangement, KeepsTheArrangement, TheNarrowing, WhatSmoozyDid,
        };
        use smoozy_queue_history::line::QueueState;
        use smoozy_state::identities::{Identity, IdentityKind};

        let mut path = std::env::temp_dir();
        path.push(format!("smoozy-queue-first-frame-downloaded-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).unwrap();
        let record = crate::record::LocalRecord::open_at(&path.join("local-record.sqlite3")).unwrap();
        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        let store = crate::record::job_records::JobRecords::over(record.connection());
        let first = crate::record::deletions::tests::a_record_in_history(&server, "pr fc1");
        store.write_the_provisional_record(&first).unwrap();
        let mut rest = crate::record::deletions::tests::a_record_in_history(&server, "pr fc1");
        rest.the_render_start_has_been_learned = false;
        store.write_the_provisional_record(&rest).unwrap();
        crate::submitting::a_first_frame_check_is_sent(
            &rest.job,
            crate::submitting::TheFirstFrameCheck {
                the_first_job: first.job.clone(),
                the_first_frame: 1,
                frames_held: 2,
            },
        );
        let holds = crate::servers::the_machines_work::WhatThisRunHolds::default();
        a_reading_of(
            &holds,
            &server,
            vec![
                (first.job.clone(), QueueState::Waiting),
                (rest.job.clone(), QueueState::Held { hold: Identity::issue(IdentityKind::Operation).unwrap() }),
            ],
        );
        its_ending_is_learned(&store, &first.job, "pr fc1");
        let the_sentence = || {
            let screen = what_is_held(&holds, Some(&record), Some(&rest.job.as_text()));
            let pair = screen.entry.expect("the held job's entry").held;
            assert_eq!(pair.len(), 1, "the pair is offered");
            pair[0]
                .said
                .iter()
                .map(|part| format!("{} {} {}", part.lead, part.value, part.trail))
                .collect::<Vec<_>>()
                .join(" ")
        };

        // **Downloads off: the job never had an arrangement.** The pass's case.
        let said = the_sentence();
        assert!(said.contains("Frame 1 is rendered"), "the first frame is still named: {said}");
        assert!(!said.contains("downloaded"), "nothing was downloaded, and the queue said so: {said}");
        assert!(said.contains("The full range has not started."), "{said}");

        // **Downloads on, and the frame not yet down**: still not downloaded.
        let mut arrangement = AnArrangement::switched_on(
            first.job.clone(),
            server.clone(),
            b"/Users/someone/Renders".to_vec(),
            TheNarrowing::Everything,
        );
        arrangement.reckoning = vec![AReckoningEntry::acted_on(
            b"0001.png".to_vec(),
            Some(1),
            WhatSmoozyDid::Owed,
            b"/Users/someone/Renders".to_vec(),
        )];
        DownloadArrangements::over(record.connection()).write_it_down(&arrangement).unwrap();
        let said = the_sentence();
        assert!(!said.contains("downloaded"), "the frame is owed, not downloaded: {said}");

        // **Once it is fetched, it is said.**
        arrangement.reckoning[0].account.state = WhatSmoozyDid::Fetched;
        DownloadArrangements::over(record.connection()).write_it_down(&arrangement).unwrap();
        let said = the_sentence();
        assert!(said.contains("Frame 1 is rendered and downloaded."), "{said}");

        // **And another frame fetched is not this one.**
        arrangement.reckoning[0].frame = Some(2);
        arrangement.reckoning[0].predicted_path = b"0002.png".to_vec();
        DownloadArrangements::over(record.connection()).write_it_down(&arrangement).unwrap();
        let said = the_sentence();
        assert!(!said.contains("downloaded"), "frame 2 came down, not frame 1: {said}");
        drop(record);
        let _ = std::fs::remove_dir_all(&path);
    }

    /// **`Queue & History` H1 and `Queue & History` H6 after a restart: a held
    /// rest this run of Smoozy did not send offers both answers, from what the
    /// reading says of its hold** — step 10c's eleventh-pass fix piece 8b.
    ///
    /// *Until then the pair was drawn only from this run's memory of sending
    /// the check, so a Smoozy opened after the submission — the specs' own
    /// case of answering hours later — read* Waiting on you *with* Cancel *and
    /// neither answer.*
    ///
    /// **Nothing Smoozy reads after a restart holds the count or the first
    /// frame**, so the pair is drawn without either and nothing names the
    /// first frame (Rule 8). **The count comes back where this run did send
    /// it**, the reading still saying which job it waits behind. **A hold
    /// with another reason, or naming no job, is not a first-frame check.**
    #[test]
    fn a_held_rest_read_after_a_restart_offers_both_answers_from_the_reading() {
        use smoozy_queue_history::line::QueueState;
        use smoozy_queue_history::readings::WhatTheHoldSays;
        use smoozy_state::identities::{Identity, IdentityKind};

        let mut path = std::env::temp_dir();
        path.push(format!("smoozy-queue-after-a-restart-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).unwrap();
        let record = crate::record::LocalRecord::open_at(&path.join("local-record.sqlite3")).unwrap();
        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        let store = crate::record::job_records::JobRecords::over(record.connection());
        let first = crate::record::deletions::tests::a_record_in_history(&server, "8b after a restart");
        store.write_the_provisional_record(&first).unwrap();
        let mut rest = crate::record::deletions::tests::a_record_in_history(&server, "8b after a restart");
        rest.the_render_start_has_been_learned = false;
        store.write_the_provisional_record(&rest).unwrap();
        let hold = Identity::issue(IdentityKind::Operation).unwrap();
        let holds = crate::servers::the_machines_work::WhatThisRunHolds::default();
        let a_hold_saying = |reason: &str, behind: Option<&Identity>| {
            Some(WhatTheHoldSays { reason: reason.to_string(), held_behind: behind.cloned() })
        };
        let the_pair_with = |the_hold| {
            a_reading_with_its_holds_of(
                &holds,
                &server,
                vec![
                    (first.job.clone(), QueueState::Waiting, None),
                    (rest.job.clone(), QueueState::Held { hold: hold.clone() }, the_hold),
                ],
            );
            what_is_held(&holds, Some(&record), Some(&rest.job.as_text())).entry.expect("the held job's entry").held
        };

        // **Nothing in this run's memory: the reading says it is a check.**
        let pair = the_pair_with(a_hold_saying("the first-frame check", Some(&first.job)));
        assert_eq!(pair.len(), 1, "the pair is offered after a restart: {pair:#?}");
        assert_eq!(pair[0].first_answer, vec![APart::words(RENDER_ALL_WITH_NO_COUNT)]);
        assert_eq!(pair[0].second_answer, CANCEL_IT);
        // **Design § What the job writes: an older record says *Waiting for
        // first-frame approval*** (2026-10-28), naming no frame.
        assert_eq!(pair[0].said, vec![APart::words(WAITING_FOR_FIRST_FRAME_APPROVAL)]);
        // **Once the first job has finished, still no frame is named**: nothing
        // Smoozy reads says which frame it was.
        its_ending_is_learned(&store, &first.job, "8b after a restart");
        let pair = the_pair_with(a_hold_saying("the first-frame check", Some(&first.job)));
        assert_eq!(
            pair[0].said,
            vec![APart::words(WAITING_FOR_FIRST_FRAME_APPROVAL)],
            "Rule 8: no frame is named that nothing holds: {pair:#?}"
        );

        // **Not a first-frame check**: another reason, or no job named.
        assert!(the_pair_with(a_hold_saying("writes nothing", Some(&first.job))).is_empty());
        assert!(the_pair_with(a_hold_saying("the first-frame check", None)).is_empty());

        // **Where this run sent it, the count comes back**, and the job it
        // waits behind is still the reading's.
        crate::submitting::a_first_frame_check_is_sent(
            &rest.job,
            crate::submitting::TheFirstFrameCheck {
                the_first_job: Identity::issue(IdentityKind::Job).unwrap(),
                the_first_frame: 1001,
                frames_held: 240,
            },
        );
        let pair = the_pair_with(a_hold_saying("the first-frame check", Some(&first.job)));
        assert_eq!(pair[0].first_answer[0].lead, RENDER_ALL);
        assert_eq!(pair[0].first_answer[0].value, "240");
        assert_eq!(
            pair[0].said,
            vec![APart::said(THE_FIRST_FRAME_LEAD, "1001", IS_RENDERED)],
            "the first job the reading names has finished"
        );
        // **And the reading's word that it is not a check stands over the
        // memory's.**
        assert!(the_pair_with(a_hold_saying("writes nothing", Some(&first.job))).is_empty());
        drop(record);
        let _ = std::fs::remove_dir_all(&path);
    }

    /// **Where each end of the held entry's answer leaves the entry** — the
    /// answer's own state, the cancel's in step 10c's S3's record, whether the
    /// job is gone, and whether anything went out to read the journal after.
    #[test]
    fn each_end_of_a_held_answer_leaves_the_entry_saying_what_happened() {
        use crate::servers::the_machines_work::{
            TheHeldJobsAnswer as Answer, WhatTheAnswerCameTo as CameTo, WhatTheHeldJobsAnswerLeft as Left,
        };
        use smoozy_runtime::holds::ReleaseOutcome;
        let cancel = TheQueuesOwnAct::TheCancel;
        assert_eq!(
            where_the_held_jobs_answer_left_it(Answer::RenderAll, Left::ItIsInTheLine),
            (Some(WhereTheAnswerStands::TheHoldIsAnswered), None, false, true)
        );
        assert_eq!(
            where_the_held_jobs_answer_left_it(Answer::CancelIt, Left::ItWasRemoved),
            (None, None, true, true),
            "removed: the local record goes"
        );
        assert_eq!(
            where_the_held_jobs_answer_left_it(Answer::RenderAll, Left::AnotherAnswerStood(ReleaseOutcome::Stopped)),
            (Some(WhereTheAnswerStands::AlreadyCancelled), None, false, true)
        );
        assert_eq!(
            where_the_held_jobs_answer_left_it(Answer::CancelIt, Left::AnotherAnswerStood(ReleaseOutcome::Released)),
            (Some(WhereTheAnswerStands::TheHoldIsAnswered), None, false, true)
        );
        assert_eq!(
            where_the_held_jobs_answer_left_it(Answer::CancelIt, Left::TheAnswer(CameTo::NotReached)),
            (None, Some(WhereTheActStands::NotReached(cancel)), false, false),
            "Cancel that reached nothing is said in the cancel's own words, and nothing is read"
        );
        assert_eq!(
            where_the_held_jobs_answer_left_it(Answer::RenderAll, Left::TheAnswer(CameTo::TheWaiterGaveUp)),
            (Some(WhereTheAnswerStands::TheWaiterGaveUp), None, false, true)
        );
        assert_eq!(
            WhereTheAnswerStands::TheWaiterGaveUp.what_the_entry_says(),
            Some(vec![APart::words(THE_ANSWER_WAS_SENT)])
        );
        assert_eq!(
            WhereTheAnswerStands::AlreadyCancelled.what_the_entry_says(),
            Some(vec![APart::words(THIS_JOB_WAS_ALREADY_CANCELLED)])
        );
    }

    /// **Must-do 11: a group with no rendering member says so**, in the single
    /// cancel box's words, with the heading's count unchanged.
    #[test]
    fn a_group_with_nothing_rendering_says_none_of_them_has_started() {
        let none = the_group_cancel_box(4, None);
        assert_eq!(none.said, vec![vec![APart::words(NONE_OF_THEM_HAS_STARTED)]]);
        assert_eq!(none.heading[1].value, "4");
        let one = the_group_cancel_box(4, Some("Scene 01"));
        assert_eq!(one.said.len(), 2);
        assert_eq!(one.said[0][0].value, "Scene 01");
    }

    /// **Must-do 12: one of anything is singular**, wherever this file joins a
    /// count to a counted word.
    #[test]
    fn one_frame_is_a_frame_and_one_job_is_a_job() {
        assert_eq!(the_stop_box(Some(1)).said[1][1].trail, FRAME_ALREADY_RENDERED_STAYS);
        assert_eq!(the_stop_box(Some(2)).said[1][1].trail, FRAMES_ALREADY_RENDERED_STAY);
        assert_eq!(the_group_cancel_box(1, None).heading[1].trail, JOB_QUESTION_TRAIL);
        assert_eq!(the_group_cancel_box(2, None).heading[1].trail, JOBS_QUESTION_TRAIL);
        let one = said(&the_queue_says(&TheQueueOnScreen {
            jobs: 1,
            servers: 1,
            groups: vec![AGroupHeading { name: "Kitchen".to_string(), scenes: 1, ..AGroupHeading::default() }],
            ..TheQueueOnScreen::default()
        }));
        assert!(one.contains(&format!("{:?}", Prose::new(SCENE_WORD))));
        assert!(one.contains(&format!("{:?}", Prose::new(THE_JOB_WORD))));
        let many = said(&the_queue_says(&TheQueueOnScreen {
            jobs: 4,
            servers: 2,
            groups: vec![AGroupHeading { name: "Kitchen".to_string(), scenes: 4, ..AGroupHeading::default() }],
            ..TheQueueOnScreen::default()
        }));
        assert!(many.contains(&format!("{:?}", Prose::new(SCENES_WORD))));
    }

    /// **Must-do 19: the banner claims no removal that may not have
    /// happened.** The spec gives two causes and the reading says which, or the
    /// sentence simply ends.
    #[test]
    fn the_untracked_banner_names_a_cause_only_where_the_reading_carries_one() {
        let untracked = |cause| TheQueueOnScreen {
            untracked: Some(TheUntrackedWork {
                server: "RunPod 4090".to_string(),
                server_row: "row-1".to_string(),
                badge: None,
                rendering: 1,
                waiting: 2,
                cause,
            }),
            ..TheQueueOnScreen::default()
        };
        let silent = said(&the_queue_says(&untracked(None)));
        assert!(!silent.contains("was removed"), "a removal was claimed that may not have happened");
        assert!(silent.contains(&format!("{:?}", Prose::new(A_FULL_STOP))));
        let removed = said(&the_queue_says(&untracked(Some(WhyItIsUntracked::TheServerWasRemoved))));
        assert!(removed.contains(&format!("{:?}", Prose::new(FROM_BEFORE_THIS_SERVER_WAS_REMOVED))));
        let edited = said(&the_queue_says(&untracked(Some(WhyItIsUntracked::ItsAddressWasChanged))));
        assert!(edited.contains(&format!("{:?}", Prose::new(FROM_BEFORE_ITS_ADDRESS_WAS_CHANGED))));
    }

    /// **Must-do 21: the rail's row shows nothing for nought**, as Transfers'
    /// already does, so the two rows follow one rule.
    #[test]
    fn an_empty_queue_sends_the_rail_no_figure() {
        let empty = said(&the_queue_says(&TheQueueOnScreen::default()));
        assert!(!empty.contains("theRailsFigure"));
        let some = said(&the_queue_says(&a_queue()));
        assert!(some.contains("theRailsFigure"));
        let page = include_str!("../../web/the-queue.js");
        assert!(page.contains("theRowsFigureIs(THE_QUEUE, said.theRailsFigure || \"\");"));
    }

    /// **Design, *The queue list*, the 38's piece 8: the teaching line, the
    /// row's menu item and the two grips' tooltips travel only where something
    /// on the list can be moved** — so must-do 5(f)'s rule still holds, no line
    /// teaching a gesture over a list where it does nothing — **and the page
    /// draws them**: the foot line, the grips' tooltips, the drag and the
    /// row's own menu.
    #[test]
    fn the_drag_is_taught_only_where_a_row_or_a_group_can_move() {
        let only_rendering = said(&the_queue_says(&a_queue()));
        for gone in ["whatDraggingDoes", "andTheRowsOwnMenu", "moveToTheTop", "dragItUpTheQueue", "dragTheWholeGroup"] {
            assert!(!only_rendering.contains(gone), "{gone} is taught over a list where nothing moves");
        }
        assert!(!only_rendering.contains("row0Grip"));

        let mut one_waits = a_queue();
        one_waits.lines[0].rows.push(ARowInTheQueue {
            job: "job-2".to_string(),
            name: "Kitchen — clay".to_string(),
            grip: true,
            ..ARowInTheQueue::default()
        });
        let text = said(&the_queue_says(&one_waits));
        for (key, words) in [
            ("whatDraggingDoes", WHAT_DRAGGING_DOES),
            ("andTheRowsOwnMenu", AND_THE_ROWS_OWN_MENU),
            ("moveToTheTop", MOVE_TO_THE_TOP),
            ("dragItUpTheQueue", DRAG_IT_UP_THE_QUEUE),
            ("dragTheWholeGroup", DRAG_THE_WHOLE_GROUP),
        ] {
            assert!(text.contains(key), "{key} is not sent");
            assert!(text.contains(&format!("{:?}", Prose::new(words))));
        }
        assert!(text.contains("row1Grip") && !text.contains("row0Grip"));

        let mut a_group_waits = a_queue();
        a_group_waits.groups.push(AGroupHeading { group: "g".to_string(), grip: true, ..AGroupHeading::default() });
        let text = said(&the_queue_says(&a_group_waits));
        assert!(text.contains("group0Grip") && text.contains("whatDraggingDoes"));

        let page = include_str!("../../web/the-queue.js");
        for drawn in [
            "foot.className = \"qfoot\"",
            "grip.title = said.dragItUpTheQueue",
            "grip.title = said.dragTheWholeGroup",
            "\"pointerdown\"",
            "\"contextmenu\"",
            "said.moveToTheTop",
        ] {
            assert!(page.contains(drawn), "the page does not draw {drawn}");
        }
        // **The page names the act by the application's own names.**
        assert!(page.contains(&format!("\"{}\"", TheActOnAJob::MoveTheJob.name())));
        assert!(page.contains(&format!("\"{}\"", TheActOnAJob::MoveTheGroup.name())));
        assert_eq!(
            where_an_act_on_a_job_goes(TheActOnAJob::MoveTheJob),
            WhereAnActOnAJobGoes::ItMovesOnTheLine
        );
        assert_eq!(
            where_an_act_on_a_job_goes(TheActOnAJob::MoveTheGroup),
            WhereAnActOnAJobGoes::ItMovesOnTheLine
        );
    }

    /// **The 38's piece 8, `Queue & History` L9 and Design, *The queue list*:
    /// only a waiting row offers the grip, a group's heading offers one where
    /// a member waits, and the rendering row is drawn first** — then the
    /// waiting rows in the machine's line order, then a held one. **A server
    /// that is not updating offers no grip at all**: no move could reach it.
    #[test]
    fn only_a_waiting_row_moves_and_the_rendering_row_is_drawn_first() {
        use smoozy_queue_history::acting::adoption::TheMachinesWorkingState;
        use smoozy_queue_history::line::QueueState;
        use smoozy_queue_history::readings::{AJobAsTheMachineReadsIt, WhatTheEntryHasLive};
        use smoozy_state::identities::{Identity, IdentityKind};
        use smoozy_state::observed::{Clock, Reading};

        let path = std::env::temp_dir().join(format!("smoozy-queue-grips-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).unwrap();
        let record = crate::record::LocalRecord::open_at(&path.join("local-record.sqlite3")).unwrap();
        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        let group = Identity::issue(IdentityKind::Group).unwrap();
        let store = crate::record::job_records::JobRecords::over(record.connection());
        let tracked = |label: &str, in_the_group: bool| {
            let mut held = crate::record::deletions::tests::a_record_in_history(&server, label);
            held.the_render_start_has_been_learned = false;
            if in_the_group {
                held.group = Some(group.clone());
            }
            store.write_the_provisional_record(&held).unwrap();
            held.job
        };
        let (first, scene_1, second, scene_2) =
            (tracked("First", false), tracked("Scene 1", true), tracked("Second", false), tracked("Scene 2", true));
        let (rendering, held) = (tracked("Rendering", false), tracked("Held", false));
        let operation = || Identity::issue(IdentityKind::Operation).unwrap();
        let job = |job: &Identity, state: QueueState| AJobAsTheMachineReadsIt {
            job: job.clone(),
            state,
            live: WhatTheEntryHasLive::NoReadings,
            the_hold: None,
        };
        let holds = crate::servers::the_machines_work::WhatThisRunHolds::default();
        let connection = holds.a_connection_begins(&server.as_text());
        // **The machine's own order**: the line first, then the rest.
        holds.a_reading_was_taken(
            &server.as_text(),
            connection,
            Reading::taken(
                TheMachinesWorkingState {
                    jobs: vec![
                        job(&first, QueueState::Waiting),
                        job(&scene_1, QueueState::Waiting),
                        job(&second, QueueState::Waiting),
                        job(&scene_2, QueueState::Waiting),
                        job(&held, QueueState::Held { hold: operation() }),
                        job(&rendering, QueueState::Rendering { operation: operation() }),
                    ],
                    the_admission_point_stands_at: Some(first.clone()),
                    notes: Vec::new(),
                    the_history_copys_count: 0,
                    the_history_copys_entries: Vec::new(),
                },
                std::time::SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(1_754_700_000),
                Clock::ThisComputer,
            ),
            Some(Vec::new()),
        );

        let on_screen = what_is_held(&holds, Some(&record), None);
        let rows: Vec<(String, bool)> =
            on_screen.lines[0].rows.iter().map(|row| (row.name.clone(), row.grip)).collect();
        assert_eq!(
            rows,
            vec![
                ("Rendering".to_string(), false),
                ("First".to_string(), true),
                ("Scene 1".to_string(), true),
                ("Second".to_string(), true),
                ("Scene 2".to_string(), true),
                ("Held".to_string(), false),
            ]
        );
        assert_eq!(on_screen.groups.len(), 1);
        assert!(on_screen.groups[0].grip, "a member waits, so the heading moves the group");
        assert_eq!(
            on_screen.entry.as_ref().map(|entry| entry.name.as_str()),
            Some("Rendering"),
            "the first row drawn is the entry shown where nothing is chosen"
        );

        holds.the_connection_ended(&server.as_text());
        let stale = what_is_held(&holds, Some(&record), None);
        assert!(stale.lines[0].rows.iter().all(|row| !row.grip), "a server not updating offers no grip");
        assert!(!stale.groups[0].grip);
        let _ = std::fs::remove_dir_all(&path);
    }

    /// **Must-do 22: the two stale notes say what the code does.**
    #[test]
    fn the_notes_this_round_corrected_say_what_the_code_does() {
        let page = include_str!("../../web/the-queue.js");
        assert!(!page.contains("custom property the stylesheet reads —\n/// which is `the-queue.css`'s one job"));
        let (working, _) = include_str!("the_queue.rs").split_once("#[cfg(test)]").expect("tests last");
        assert!(!working.contains("`AServersLine::rows`' companion list"));
        assert!(working.contains("an index into\n    /// [`TheQueueOnScreen::groups`]"));
    }

    /// **The runner's item R1: the stop box says the frame *may not be
    /// saved*, and never that it is lost** — `Servers` W19a, under Rule 10: a
    /// stop lands in one of two places and the flat form is false in the one
    /// case anybody has measured. **The words are `Servers`' own**, and this
    /// fails if that screen's sentence moves away from them.
    #[test]
    fn a_stop_never_claims_the_frame_is_lost() {
        let the_box = the_stop_box(Some(60));
        let said: String = the_box
            .said
            .iter()
            .flatten()
            .map(|part| format!("{} {} {}", part.lead, part.value, part.trail))
            .collect::<Vec<_>>()
            .join(" ");
        assert!(said.contains(THE_FRAME_IT_IS_ON_MAY_NOT_BE_SAVED), "{said}");
        for flat in ["is lost", "is not saved."] {
            assert!(!said.contains(flat), "the box says the frame {flat:?}: {said}");
        }
        assert!(!the_group_cancel_box(4, Some("Scene 01")).said[0][0].trail.contains("is lost"));

        // **The group's box says the same thing with the same words, because it
        // says it with the same constant** (the runner's item R3, 2026-09-17).
        // **It had a second literal of its own** and nothing held the two
        // together, which is the one-fact-in-two-places fault R1 existed to
        // end. **Read out of the working half rather than off the composed box**:
        // a box built from two copies of the sentence would pass a reading of
        // what it says, and what this is about is there being one copy.
        let group: String = the_group_cancel_box(4, Some("Scene 01")).said[0]
            .iter()
            .map(|part| format!("{}{}{}", part.lead, part.value, part.trail))
            .collect::<Vec<_>>()
            .join(" ");
        assert!(
            group.ends_with(THE_FRAME_IT_IS_ON_MAY_NOT_BE_SAVED),
            "the group's cancel box no longer ends on {THE_FRAME_IT_IS_ON_MAY_NOT_BE_SAVED:?}: {group}"
        );
        let (working, _) = include_str!("the_queue.rs")
            .split_once("#[cfg(test)]")
            .expect("the tests come last");
        assert_eq!(
            working.matches(THE_FRAME_IT_IS_ON_MAY_NOT_BE_SAVED).count(),
            1,
            "one fact, one wording: {THE_FRAME_IT_IS_ON_MAY_NOT_BE_SAVED:?} is written out \
             more than once in this file, so the two can drift apart"
        );

        let servers = include_str!("servers.rs");
        assert!(
            servers.contains(THE_FRAME_IT_IS_ON_MAY_NOT_BE_SAVED.trim_end_matches('.')),
            "the Servers screen no longer says {THE_FRAME_IT_IS_ON_MAY_NOT_BE_SAVED:?}; one fact has two wordings"
        );
    }

    /// **S3, as the user meets it: the entry's *Stop*, *Cancel* and a group's
    /// *Cancel all* act.** Before step 10c's S3 each of the six answered
    /// *"Smoozy cannot do this yet, because nothing in Smoozy connects to a
    /// server yet"*.
    #[test]
    fn s3_the_queues_stop_cancel_and_cancel_all_do_not_answer_not_built() {
        for act in [
            TheActOnAJob::Stop,
            TheActOnAJob::Cancel,
            TheActOnAJob::CancelAll,
            TheActOnAJob::StopTheJob,
            TheActOnAJob::CancelTheJob,
            TheActOnAJob::CancelTheGroup,
        ] {
            assert_ne!(
                where_an_act_on_a_job_goes(act),
                WhereAnActOnAJobGoes::ItAnswers,
                "{} answers {:?}",
                act.name(),
                act.answers()
            );
        }
    }

    /// **`Queue & History` L20: a group is drawn as a group from the group
    /// identity on the members' own local records** — the heading that
    /// carries *Cancel all N*.
    #[test]
    fn s3_two_members_of_one_group_are_drawn_under_its_heading() {
        use smoozy_queue_history::line::QueueState;
        use smoozy_state::identities::{Identity, IdentityKind};
        let mut path = std::env::temp_dir();
        path.push(format!("smoozy-queue-s3-group-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).unwrap();
        let record = crate::record::LocalRecord::open_at(&path.join("local-record.sqlite3")).unwrap();
        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        let store = crate::record::job_records::JobRecords::over(record.connection());
        let group = Identity::issue(IdentityKind::Group).unwrap();
        let mut members = Vec::new();
        for _ in 0..2 {
            let mut member = crate::record::deletions::tests::a_record_in_history(&server, "Kitchen");
            member.the_render_start_has_been_learned = false;
            member.group = Some(group.clone());
            store.write_the_provisional_record(&member).unwrap();
            members.push(member.job);
        }
        let holds = crate::servers::the_machines_work::WhatThisRunHolds::default();
        a_reading_of(
            &holds,
            &server,
            vec![(members[0].clone(), QueueState::Waiting), (members[1].clone(), QueueState::Waiting)],
        );
        let held = what_is_held(&holds, Some(&record), None);
        let _ = std::fs::remove_dir_all(&path);
        assert_eq!(
            held.groups,
            vec![AGroupHeading {
                group: group.as_text(),
                name: "Kitchen".to_string(),
                scenes: 2,
                rendering: None,
                // **Both members wait, so the heading moves the group** (the 38's piece 8).
                grip: true,
            }]
        );
        assert!(held.lines[0].rows.iter().all(|row| row.group == Some(0)));
    }

    /// **One server's reading holding three jobs, as the tests below press
    /// them**: two members of one group — *Scene 01* rendering, *Scene 02*
    /// waiting — and one held job outside it.
    struct AQueueWithAGroup {
        holds: crate::servers::the_machines_work::WhatThisRunHolds,
        record: crate::record::LocalRecord,
        jobs: Vec<String>,
        group: String,
        path: std::path::PathBuf,
    }

    impl AQueueWithAGroup {
        fn made(tag: &str) -> Self {
            use smoozy_queue_history::line::QueueState;
            use smoozy_state::identities::{Identity, IdentityKind};
            let mut path = std::env::temp_dir();
            path.push(format!("smoozy-queue-s3-{tag}-{}", std::process::id()));
            let _ = std::fs::remove_dir_all(&path);
            std::fs::create_dir_all(&path).unwrap();
            let record = crate::record::LocalRecord::open_at(&path.join("local-record.sqlite3")).unwrap();
            let server = Identity::issue(IdentityKind::LocalServer).unwrap();
            let group = Identity::issue(IdentityKind::Group).unwrap();
            let mut jobs = Vec::new();
            {
                let store = crate::record::job_records::JobRecords::over(record.connection());
                for (scene, in_the_group) in [("Scene 01", true), ("Scene 02", true), ("Hallway", false)] {
                    let mut job = crate::record::deletions::tests::a_record_in_history(&server, "Kitchen");
                    job.the_render_start_has_been_learned = false;
                    job.scene_name = Some(scene.to_string());
                    job.group = in_the_group.then(|| group.clone());
                    store.write_the_provisional_record(&job).unwrap();
                    jobs.push(job.job);
                }
            }
            let holds = crate::servers::the_machines_work::WhatThisRunHolds::default();
            let an_operation = || Identity::issue(IdentityKind::Operation).unwrap();
            a_reading_of(
                &holds,
                &server,
                vec![
                    (jobs[0].clone(), QueueState::Rendering { operation: an_operation() }),
                    (jobs[1].clone(), QueueState::Waiting),
                    (jobs[2].clone(), QueueState::Held { hold: an_operation() }),
                ],
            );
            AQueueWithAGroup {
                holds,
                record,
                jobs: jobs.iter().map(|job| job.as_text()).collect(),
                group: group.as_text(),
                path,
            }
        }

        fn over(&self, chosen: usize) -> TheQueueOnScreen {
            what_is_held(&self.holds, Some(&self.record), Some(&self.jobs[chosen]))
        }
    }

    impl Drop for AQueueWithAGroup {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.path);
        }
    }

    /// **Design, *The entry's controls*: *Stopping asks first, and so does
    /// cancelling*, and the group's *Cancel all* asks the same way.** Each box
    /// is raised only over what the reading still shows; a stop pressed over a
    /// job that is not rendering raises nothing and says why.
    #[test]
    fn s3_each_press_raises_its_own_box_over_what_the_reading_shows() {
        let queue = AQueueWithAGroup::made("boxes");
        let (rendering, waiting, held) = (&queue.jobs[0], &queue.jobs[1], &queue.jobs[2]);

        let raised = the_box_is_raised(queue.over(0), TheActOnAJob::Stop, Some(rendering), None).unwrap();
        let the_box = raised.asking.as_ref().expect("the stop box");
        assert_eq!(the_box.go_ahead_act, TheActOnAJob::StopTheJob);
        assert_eq!(
            the_box.said[1],
            vec![APart::words(FRAMES_ALREADY_RENDERED_STAY_WITH_NO_COUNT)],
            "no count is made up"
        );

        assert_eq!(
            the_box_is_raised(queue.over(1), TheActOnAJob::Stop, Some(waiting), None),
            Err(THIS_JOB_IS_NOT_RENDERING)
        );
        for job in [waiting, held] {
            let at = queue.jobs.iter().position(|one| one == job).unwrap();
            let raised = the_box_is_raised(queue.over(at), TheActOnAJob::Cancel, Some(job), None).unwrap();
            assert_eq!(raised.asking.as_ref().unwrap().go_ahead_act, TheActOnAJob::CancelTheJob);
        }
        assert_eq!(
            the_box_is_raised(queue.over(0), TheActOnAJob::Cancel, Some(rendering), None),
            Err(THIS_JOB_IS_NO_LONGER_IN_THE_QUEUE),
            "a rendering job is stopped, not cancelled (`Queue & History` L4)"
        );

        let raised = the_box_is_raised(queue.over(0), TheActOnAJob::CancelAll, None, Some(&queue.group)).unwrap();
        assert_eq!(raised.asking.as_ref(), Some(&the_group_cancel_box(2, Some("Scene 01"))));
        assert_eq!(raised.the_group_asked_about.as_deref(), Some(queue.group.as_str()));
        // **Under its own name**: the heading's own copy of the identity is a
        // different field, and the box's go-ahead reads this one.
        let event = said(&the_queue_says(&raised));
        let at = event.find("\"boxGroup\"").expect("the group rides the event for the go-ahead");
        assert!(
            event[at..].chars().take(120).collect::<String>().contains(&queue.group),
            "boxGroup does not carry the group: {event}"
        );
        assert_eq!(
            the_box_is_raised(queue.over(0), TheActOnAJob::CancelAll, None, Some("another group")),
            Err(THESE_JOBS_ARE_NO_LONGER_IN_THE_QUEUE)
        );
    }

    /// **`Queue & History` L12a and L2: while Smoozy's own act is in hand the
    /// row and the entry say so and the control is not offered**; once the
    /// waiter has given up the entry says `Rendering` K10's words and *Stop*
    /// is offered again (*a new stop can be sent*); *not cancelled* carries
    /// the machine's own words as a value beside the sentence.
    #[test]
    fn s3_the_row_and_the_entry_say_where_smoozys_own_act_stands() {
        use crate::servers::the_queues_own_acts::{it_stands, TheQueuesOwnAct, WhereTheActStands};
        let queue = AQueueWithAGroup::made("stands");
        let rendering = &queue.jobs[0];

        it_stands(rendering, Some(WhereTheActStands::InHand(TheQueuesOwnAct::TheStop)));
        let stopping = queue.over(0);
        let entry = stopping.entry.as_ref().unwrap();
        assert_eq!(entry.status, vec![APart::words(STOPPING)]);
        assert_eq!(entry.stop_or_cancel, None, "one press at a time");
        let row = stopping.lines[0].rows.iter().find(|row| &row.job == rendering).unwrap();
        assert_eq!(row.state, Some(STOPPING));

        it_stands(rendering, Some(WhereTheActStands::TheWaiterGaveUp(TheQueuesOwnAct::TheStop)));
        let given_up = queue.over(0);
        let entry = given_up.entry.as_ref().unwrap();
        assert_eq!(entry.status, vec![APart::words(THE_STOP_WAS_SENT), APart::words(PRESS_STOP_AGAIN)]);
        assert_eq!(
            the_status_line_reads(entry),
            "Stop sent. Timed out waiting for the render to end. Press Stop again to end it.",
            "the entry says what to do, beside the Stop it offers (thirteenth pass)"
        );
        assert_eq!(entry.stop_or_cancel, Some(TheStopOrCancel::Stop), "a new stop can be sent");

        it_stands(&queue.jobs[1], Some(WhereTheActStands::NotCancelled(Some("it has already ended".to_string()))));
        let entry = queue.over(1).entry.unwrap();
        assert_eq!(
            entry.status,
            vec![APart::words(THE_JOB_WAS_NOT_CANCELLED), APart::value("it has already ended")]
        );
        assert_eq!(
            what_the_entry_says_of(&WhereTheActStands::NotTaken(TheQueuesOwnAct::TheCancel, None)).0,
            vec![APart::words(THE_SERVER_DID_NOT_ACCEPT_IT)],
            "a refusal is never said as a server Smoozy could not reach"
        );

        // **A group's cancel in hand marks every member it covers.**
        it_stands(rendering, Some(WhereTheActStands::InHand(TheQueuesOwnAct::TheStop)));
        it_stands(&queue.jobs[1], Some(WhereTheActStands::InHand(TheQueuesOwnAct::TheCancel)));
        let rows = queue.over(0).lines[0].rows.clone();
        assert_eq!(rows[0].state, Some(STOPPING));
        assert_eq!(rows[1].state, Some(CANCELLING));
        assert_eq!(rows[2].state, Some(WAITING_ON_YOU), "the job outside the group is untouched");
        it_stands(rendering, None);
        it_stands(&queue.jobs[1], None);
    }

    /// **The page hands the group back by its identity**, from the heading's
    /// *Cancel all* and from the group box's go-ahead.
    #[test]
    fn s3_the_page_hands_the_group_back_by_its_identity() {
        let page = include_str!("../../web/the-queue.js");
        assert!(page.contains(r#"itAnswers(cancel, "cancel-all", { group: said["group" + group + "Group"] || "" });"#));
        assert!(page.contains("group: said.boxGroup || \"\""));
        assert!(page.contains("group: with_.group || null,"));
    }

    /// **`Preparation/Scenarios/The user stops a running job.md`, played as
    /// far as the desk can play it.** One server, one job rendering, nothing
    /// else queued. **Before step 10c's S3 it failed at its step 2**: *Stop*
    /// answered *not built*.
    ///
    /// 2. The user presses *Stop*: the box asks, and its go-ahead sends
    ///    `Server protocol and transfers` C21's stop naming the render; the
    ///    entry says *Stopping* while the machine is asked by the stop's own
    ///    identity (here a stand-in answering *still in hand* twice, then *left
    ///    stopped*).
    /// 3. The job leaves the queue on the ending the read-forward learns, and
    ///    History shows it *Stopped* with its failure shape.
    ///
    /// **What only a rental shows** is the render's real interrupt and the
    /// frame at its final name (`Rendering` K4); S1's account names both.
    #[test]
    fn s3_the_user_stops_a_running_job_as_far_as_the_desk_plays_it() {
        use crate::servers::stop_and_clear::TheWait;
        use crate::servers::the_queues_own_acts::{it_stands, the_stop_is_sent, TheQueuesOwnAct, WhereTheActStands};
        use smoozy_queue_history::acting::note::{TheEndingOnTheNote, TheNote, TheOutcomeAsWritten};
        use smoozy_queue_history::line::QueueState;
        use smoozy_server_protocol::asking_the_supervisor::WhatCameBack;
        use smoozy_server_protocol::framing::{Answered, Fields, Request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS};
        use smoozy_server_protocol::requests as asked;
        use smoozy_state::identities::{Identity, IdentityKind};

        let mut path = std::env::temp_dir();
        path.push(format!("smoozy-queue-s3-scenario-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).unwrap();
        let record = crate::record::LocalRecord::open_at(&path.join("local-record.sqlite3")).unwrap();
        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        let store = crate::record::job_records::JobRecords::over(record.connection());
        // **Frame 6 is twenty minutes in**, so the read-forward has long since
        // learned the render started (`Queue & History` C9).
        let mut job = crate::record::deletions::tests::a_record_in_history(&server, "Shot 040");
        job.the_render_start_has_been_learned = true;
        store.write_the_provisional_record(&job).unwrap();
        let render = Identity::issue(IdentityKind::Operation).unwrap();
        let holds = crate::servers::the_machines_work::WhatThisRunHolds::default();
        a_reading_of(&holds, &server, vec![(job.job.clone(), QueueState::Rendering { operation: render.clone() })]);
        let text = job.job.as_text();

        // **Step 2: *Stop*, then the box's *Stop*.**
        let held = what_is_held(&holds, Some(&record), Some(&text));
        assert_eq!(where_an_act_on_a_job_goes(TheActOnAJob::Stop), WhereAnActOnAJobGoes::ItsBox);
        let raised = the_box_is_raised(held, TheActOnAJob::Stop, Some(&text), None).expect("the box asks");
        assert_eq!(raised.asking.as_ref().unwrap().go_ahead_act, TheActOnAJob::StopTheJob);
        it_stands(&text, Some(WhereTheActStands::InHand(TheQueuesOwnAct::TheStop)));
        let stopping = what_is_held(&holds, Some(&record), Some(&text));
        assert_eq!(stopping.entry.as_ref().unwrap().status, vec![APart::words(STOPPING)]);

        struct TheMachine {
            asked: usize,
            render: Identity,
        }
        impl crate::servers::the_machines_work::AsksTheSupervisor for TheMachine {
            fn ask(&mut self, request: &Request) -> WhatCameBack {
                assert_eq!(request.name, asked::THE_STOP_REQUEST);
                assert_eq!(
                    request.fields.text(asked::THE_OPERATION_TO_STOP_FIELD),
                    Some(self.render.as_text().as_str())
                );
                self.asked += 1;
                let fields = if self.asked < 3 {
                    Fields::none().and(asked::WHAT_BECAME_OF_IT_FIELD, asked::IT_IS_STILL_IN_HAND)
                } else {
                    Fields::none()
                        .and(asked::WHAT_BECAME_OF_IT_FIELD, asked::IT_HAS_A_RECORDED_OUTCOME)
                        .and(asked::THE_STOPS_OUTCOME_FIELD, asked::IT_WAS_LEFT_STOPPED)
                };
                WhatCameBack::ItAnswered(
                    Answered::to(request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, "test").carrying(fields),
                )
            }
        }
        let mut machine = TheMachine { asked: 0, render: render.clone() };
        let wait = TheWait { deadline: std::time::Duration::from_secs(5), pause: std::time::Duration::from_millis(1) };
        let stands = the_stop_is_sent(&mut machine, &render, wait);
        assert_eq!(stands, None, "left stopped is the stop done");
        it_stands(&text, stands);

        // **The read-forward learns the ending** — the note's *left stopped*.
        let mut note = TheNote::created_with_the_job_record("Shot 040");
        note.ending = Some(TheEndingOnTheNote {
            outcome: TheOutcomeAsWritten::LeftStopped,
            exit_status: None,
            the_attempt_whose_capture_record_holds_the_words: 1,
            the_reason_the_machine_recorded: None,
            no_render_began: false,
            the_device_line_as_observed: None,
        });
        store.the_fill_in(&job.job, &note).expect("the fill-in is written");

        // **Step 3: the queue and the history.**
        let queue = what_is_held(&holds, Some(&record), Some(&text));
        assert_eq!(queue.jobs, 0, "the stopped job has left the queue");
        let history = crate::surface::the_history::what_is_kept(Some(&record), Some(&text));
        let row = history.rows.iter().find(|row| row.job == text).expect("it is in History");
        assert_eq!(row.ending, Some(crate::surface::the_history::AnEnding::Stopped));
        let _ = std::fs::remove_dir_all(&path);
    }

    /// **The entry's status line as the page writes it**: each part's words,
    /// value and trail with a space between, and a space between parts
    /// (`web/the-queue.js`, the entry's `statusline`).
    fn the_status_line_reads(entry: &TheJobEntry) -> String {
        entry
            .status
            .iter()
            .map(|part| {
                [part.lead, part.value.as_str(), part.trail]
                    .into_iter()
                    .filter(|piece| !piece.is_empty())
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// **Step 10c's eleventh pass, S3-5, as the tester met it: a stop sent, and
    /// the line cut a moment later.** The machine took the stop and stopped
    /// the render; the desktop's waiter went on asking into a dead line.
    /// **At 1 min 50 s and at 2 min 31 s the entry read *Not updating* and
    /// nothing else** — no *Stop*, and no word of the stop. It corrected itself
    /// only once the line came back.
    ///
    /// `Design/Queue & History § The same holds while the server is not
    /// updating` (E0-5), under `Rendering` K10 and `Queue & History` C7:
    ///
    /// 1. The stop goes out and the machine answers *still in hand*; then the
    ///    line goes quiet and the break is declared. **The entry reads *Not
    ///    updating · Stopping*** while the waiter runs, and offers no *Stop*.
    /// 2. The waiter gives up. **The entry reads *Not updating ·* K10's
    ///    sentence**, and still offers no *Stop*: until the server is updating
    ///    again a stop cannot reach it.
    /// 3. The reading comes back. K10's sentence, and *Stop* is offered again.
    /// 4. The render's ending is learned: the job leaves the queue and History
    ///    reads *Stopped*.
    #[test]
    fn s3_5_a_stop_sent_before_the_line_went_quiet_stays_said_beside_not_updating() {
        use crate::servers::stop_and_clear::TheWait;
        use crate::servers::the_queues_own_acts::{it_stands, the_stop_is_sent, TheQueuesOwnAct, WhereTheActStands};
        use smoozy_queue_history::acting::note::{TheEndingOnTheNote, TheNote, TheOutcomeAsWritten};
        use smoozy_queue_history::line::QueueState;
        use smoozy_server_protocol::asking_the_supervisor::WhatCameBack;
        use smoozy_server_protocol::framing::{Answered, Fields, Request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS};
        use smoozy_server_protocol::requests as asked;
        use smoozy_state::identities::{Identity, IdentityKind};

        let mut path = std::env::temp_dir();
        path.push(format!("smoozy-queue-s3-5-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).unwrap();
        let record = crate::record::LocalRecord::open_at(&path.join("local-record.sqlite3")).unwrap();
        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        let store = crate::record::job_records::JobRecords::over(record.connection());
        let mut job = crate::record::deletions::tests::a_record_in_history(&server, "p11 cut");
        job.the_render_start_has_been_learned = true;
        store.write_the_provisional_record(&job).unwrap();
        let render = Identity::issue(IdentityKind::Operation).unwrap();
        let holds = crate::servers::the_machines_work::WhatThisRunHolds::default();
        a_reading_of(&holds, &server, vec![(job.job.clone(), QueueState::Rendering { operation: render.clone() })]);
        let text = job.job.as_text();

        // **The box's *Stop***: the entry says *Stopping*, as S3 built it.
        let held = what_is_held(&holds, Some(&record), Some(&text));
        the_box_is_raised(held, TheActOnAJob::Stop, Some(&text), None).expect("the box asks");
        it_stands(&text, Some(WhereTheActStands::InHand(TheQueuesOwnAct::TheStop)));

        // **The machine takes the stop, then the line is cut.** At the cut the
        // break is declared, as `Servers` K3 declares it, and what the user
        // reads is taken on each ask the waiter makes into the dead line.
        struct TheMachine<'a> {
            asked: usize,
            holds: &'a crate::servers::the_machines_work::WhatThisRunHolds,
            record: &'a crate::record::LocalRecord,
            server: String,
            job: String,
            seen: Vec<(String, Option<TheStopOrCancel>, Option<&'static str>)>,
        }
        impl crate::servers::the_machines_work::AsksTheSupervisor for TheMachine<'_> {
            fn ask(&mut self, request: &Request) -> WhatCameBack {
                assert_eq!(request.name, asked::THE_STOP_REQUEST);
                self.asked += 1;
                if self.asked == 1 {
                    return WhatCameBack::ItAnswered(
                        Answered::to(request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, "test").carrying(
                            Fields::none().and(asked::WHAT_BECAME_OF_IT_FIELD, asked::IT_IS_STILL_IN_HAND),
                        ),
                    );
                }
                if self.asked == 2 {
                    self.holds.the_connection_ended(&self.server);
                }
                let screen = what_is_held(self.holds, Some(self.record), Some(&self.job));
                let entry = screen.entry.as_ref().expect("the entry");
                self.seen.push((the_status_line_reads(entry), entry.stop_or_cancel, screen.lines[0].rows[0].state));
                WhatCameBack::ItWasNeverAsked
            }
        }
        let mut machine = TheMachine {
            asked: 0,
            holds: &holds,
            record: &record,
            server: server.as_text(),
            job: text.clone(),
            seen: Vec::new(),
        };
        let wait = TheWait { deadline: std::time::Duration::from_millis(60), pause: std::time::Duration::from_millis(5) };
        let stands = the_stop_is_sent(&mut machine, &render, wait);
        let seen = std::mem::take(&mut machine.seen);
        assert!(!seen.is_empty(), "the waiter asked into the cut line");
        for (reads, offered, row) in &seen {
            assert_eq!(reads, "Not updating · Stopping", "while the waiter runs");
            assert_eq!(*offered, None, "no Stop while the server is not updating");
            assert_eq!(*row, Some(NOT_UPDATING), "the row keeps Not updating");
        }

        // **The waiter gives up** — `Server protocol and transfers` C17.
        assert_eq!(stands, Some(WhereTheActStands::TheWaiterGaveUp(TheQueuesOwnAct::TheStop)));
        it_stands(&text, stands);
        let given_up = what_is_held(&holds, Some(&record), Some(&text));
        let entry = given_up.entry.as_ref().unwrap();
        assert_eq!(
            the_status_line_reads(entry),
            "Not updating · Stop sent. Timed out waiting for the render to end."
        );
        assert_eq!(entry.stop_or_cancel, None, "a stop cannot reach a server that is not updating");
        assert!(!entry.status.iter().any(|part| part.lead == RENDERING), "never read as still running");

        // **The reading comes back**: K10's sentence, and a new stop can be sent.
        a_reading_of(&holds, &server, vec![(job.job.clone(), QueueState::Rendering { operation: render.clone() })]);
        let back = what_is_held(&holds, Some(&record), Some(&text));
        let entry = back.entry.as_ref().unwrap();
        assert_eq!(entry.status, vec![APart::words(THE_STOP_WAS_SENT), APart::words(PRESS_STOP_AGAIN)]);
        assert_eq!(entry.stop_or_cancel, Some(TheStopOrCancel::Stop));

        // **The render ended** — the machine wrote *left stopped* before the
        // cut, and the read-forward learns it.
        let mut note = TheNote::created_with_the_job_record("p11 cut");
        note.ending = Some(TheEndingOnTheNote {
            outcome: TheOutcomeAsWritten::LeftStopped,
            exit_status: None,
            the_attempt_whose_capture_record_holds_the_words: 1,
            the_reason_the_machine_recorded: None,
            no_render_began: false,
            the_device_line_as_observed: None,
        });
        store.the_fill_in(&job.job, &note).expect("the fill-in is written");
        assert_eq!(what_is_held(&holds, Some(&record), Some(&text)).jobs, 0, "it has left the queue");
        let history = crate::surface::the_history::what_is_kept(Some(&record), Some(&text));
        let row = history.rows.iter().find(|row| row.job == text).expect("it is in History");
        assert_eq!(row.ending, Some(crate::surface::the_history::AnEnding::Stopped));
        assert_eq!(
            history.entry.as_ref().and_then(|entry| entry.ending),
            Some(crate::surface::the_history::AnEnding::Stopped)
        );
        it_stands(&text, None);
        let _ = std::fs::remove_dir_all(&path);
    }

    /// **The same for a cancel, and nothing else under *Not updating*
    /// changes.** `Design/Queue & History § The same holds while the server is
    /// not updating`, and `Queue & History` C7: *Cancelling* while the cancel
    /// is in hand and its own sentence once the waiter gives up, beside *Not
    /// updating*; an act that ended any other way, and an entry with no act of
    /// Smoozy's own, read *Not updating* alone, with nothing to press — the
    /// rendering entry is never read as still running, and the held entry
    /// offers no answers.
    #[test]
    fn s3_5_a_cancel_is_said_beside_not_updating_and_nothing_else_is() {
        use crate::servers::the_queues_own_acts::{it_stands, TheQueuesOwnAct, WhereTheActStands};
        let queue = AQueueWithAGroup::made("s3-5");
        let (rendering, waiting, held) = (&queue.jobs[0], &queue.jobs[1], &queue.jobs[2]);
        let the_server = queue.holds.every_server()[0].0.clone();
        queue.holds.the_connection_ended(&the_server);

        let reads = |chosen: usize| {
            let screen = queue.over(chosen);
            let entry = screen.entry.clone().expect("the entry");
            (the_status_line_reads(&entry), entry.stop_or_cancel, entry.held.len(), screen.lines[0].rows[chosen].state)
        };

        it_stands(waiting, Some(WhereTheActStands::InHand(TheQueuesOwnAct::TheCancel)));
        assert_eq!(reads(1), ("Not updating · Cancelling".to_string(), None, 0, Some(NOT_UPDATING)));
        it_stands(waiting, Some(WhereTheActStands::TheWaiterGaveUp(TheQueuesOwnAct::TheCancel)));
        assert_eq!(
            reads(1),
            (format!("Not updating · {THE_CANCEL_WAS_SENT}"), None, 0, Some(NOT_UPDATING))
        );
        // **A group's cancel marks its rendering member as a stop.**
        it_stands(rendering, Some(WhereTheActStands::InHand(TheQueuesOwnAct::TheStop)));
        assert_eq!(reads(0), ("Not updating · Stopping".to_string(), None, 0, Some(NOT_UPDATING)));

        // **Nothing else changes.**
        for other in [
            WhereTheActStands::NotReached(TheQueuesOwnAct::TheStop),
            WhereTheActStands::NotTaken(TheQueuesOwnAct::TheStop, Some("older".to_string())),
            WhereTheActStands::TheStopFailed(None),
        ] {
            it_stands(rendering, Some(other.clone()));
            assert_eq!(reads(0), ("Not updating".to_string(), None, 0, Some(NOT_UPDATING)), "{other:?}");
        }
        it_stands(rendering, None);
        assert_eq!(reads(0), ("Not updating".to_string(), None, 0, Some(NOT_UPDATING)));
        assert_eq!(reads(2), ("Not updating".to_string(), None, 0, Some(NOT_UPDATING)));
        it_stands(held, Some(WhereTheActStands::InHand(TheQueuesOwnAct::TheCancel)));
        assert_eq!(reads(2), ("Not updating · Cancelling".to_string(), None, 0, Some(NOT_UPDATING)));
        it_stands(held, None);
        it_stands(waiting, None);
    }

    /// **S3's loose ends, item 2, as the user meets it: a held job the machine
    /// drops behind a cancel is named in one line under the Queue heading,
    /// once.** A first-frame
    /// check: the user cancels the first-frame job while it waits, and the
    /// machine drops the range held behind it (`Queue & History`
    /// H9) — its record, then its hold's release *dropped*, then its removal,
    /// in `Queue & History` H11's order. **Before this, the pass took both
    /// local records and the notice said nothing**, so the held job left the
    /// queue in silence. `Queue & History` H11 and C1, and Design, *Jobs that
    /// leave without an entry*: the job's name, then that the first frame was
    /// never rendered, and *Dismiss* asks first because this is the only place
    /// the job is named. *(The line was a banner's until the Creator's ninth
    /// notes, note 3.)*
    #[test]
    fn a_held_job_dropped_behind_a_cancel_is_named_in_the_notice_once() {
        use crate::servers::the_machines_work::{a_read_forward_pass, AsksTheSupervisor, WhatThisRunHolds};
        use smoozy_queue_history::record::Standing;
        use smoozy_server_protocol::asking_the_supervisor::WhatCameBack;
        use smoozy_server_protocol::framing::{Answered, Fields, Request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS};
        use smoozy_server_protocol::requests as asked;
        use smoozy_state::identities::{Identity, IdentityKind};

        let mut path = std::env::temp_dir();
        path.push(format!("smoozy-queue-held-dropped-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).unwrap();
        let record = crate::record::LocalRecord::open_at(&path.join("local-record.sqlite3")).unwrap();
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
        let store = crate::record::job_records::JobRecords::over(record.connection());
        let mut first = crate::record::deletions::tests::a_record_in_history(&server, "Kitchen — first frame");
        let mut rest = crate::record::deletions::tests::a_record_in_history(&server, "Kitchen — final v4");
        for job in [&mut first, &mut rest] {
            job.standing = Standing::Confirmed;
            job.the_render_start_has_been_learned = false;
            store.write_the_provisional_record(job).unwrap();
        }
        let hold = Identity::issue(IdentityKind::Operation).unwrap();

        /// **The machine's journal after the cancel**, read forward once and
        /// then empty.
        struct TheMachine {
            events: Vec<(&'static str, Identity, &'static str)>,
            read: bool,
        }
        impl AsksTheSupervisor for TheMachine {
            fn ask(&mut self, request: &Request) -> WhatCameBack {
                let answered = Answered::to(request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, "test");
                let fields = match request.name.as_str() {
                    asked::THE_READ_FORWARD_REQUEST => {
                        let from: u64 = request.fields.text(asked::THE_CURSOR_FIELD).unwrap().parse().unwrap();
                        let events = if self.read { &[][..] } else { &self.events[..] };
                        self.read = true;
                        let mut fields = Fields::none()
                            .and(asked::HOW_MANY_EVENTS_FIELD, events.len().to_string())
                            .and(asked::THE_NEXT_SEQUENCE_FIELD, (from + events.len() as u64).to_string())
                            .and(asked::THE_OLDEST_RETAINED_FIELD, "0");
                        for (at, (kind, about, says)) in events.iter().enumerate() {
                            let item = |field| asked::the_field_of_item(asked::AN_EVENT, at, field);
                            fields = fields
                                .and(&item(asked::THE_SEQUENCE_FIELD), (from + at as u64).to_string())
                                .and(&item(asked::THE_KIND_FIELD), *kind)
                                .and(&item(asked::THE_ABOUT_FIELD), about.as_text())
                                .and(&item(asked::THE_SAYS_FIELD), *says);
                        }
                        fields
                    }
                    asked::THE_CURSOR_MOVES_FORWARD_REQUEST => Fields::none(),
                    other => return WhatCameBack::ItAnswered(answered.saying_it_does_not_answer_that_request(other)),
                };
                WhatCameBack::ItAnswered(answered.carrying(fields))
            }
        }
        let the_journal = vec![
            (asked::THE_JOB_RECORD_REMOVED_EVENT, first.job.clone(), asked::IT_WAS_REMOVED),
            (asked::THE_RELEASE_EVENT, hold.clone(), "the hold's release outcome was written: dropped"),
            (asked::THE_JOB_RECORD_REMOVED_EVENT, rest.job.clone(), asked::IT_WAS_REMOVED),
        ];
        let holds = WhatThisRunHolds::default();
        let lent = crate::keys::TheRecordItself::over(record.connection());
        a_read_forward_pass(&server, &lent, &mut TheMachine { events: the_journal.clone(), read: false }, &holds);

        assert!(store.read(&first.job).unwrap().is_none(), "the cancelled job's record stands");
        assert!(store.read(&rest.job).unwrap().is_none(), "the dropped job's record stands");
        let held = what_is_held(&holds, Some(&record), None);
        assert_eq!(held.jobs, 0);
        let notice = held.notice.clone().expect("the dropped job is told nowhere");
        let the_line = ADroppedJob {
            job: rest.job.as_text(),
            name: "Kitchen — final v4".to_string(),
            what: APart::words("was not started — the first frame was never rendered."),
        };
        assert_eq!(
            notice,
            TheCatchUpNotice { counted: Vec::new(), dropped: vec![the_line.clone()] },
            "only the dropped job is named, and nothing counted; a cancel is the user's own act (`Queue & History` L2)"
        );
        let asked = the_queue_as_asked(held, Some(TheQueueAsk::Dismiss), Some(&rest.job.as_text()));
        assert_eq!(asked.asking, Some(the_dismiss_box("Kitchen — final v4")));

        // **Told once** (`Queue & History` H11): the same events read again —
        // a cursor put back — name nothing twice.
        let mut row = crate::record::server_rows::one(record.connection(), &server.as_text()).unwrap().unwrap();
        row.event_cursor = None;
        crate::record::server_rows::write(record.connection(), &row).unwrap();
        a_read_forward_pass(&server, &lent, &mut TheMachine { events: the_journal, read: false }, &holds);
        assert_eq!(what_is_held(&holds, Some(&record), None).notice.map(|notice| notice.dropped.len()), Some(1));

        // **The 38's piece 15, `Queue & History` C2: the line survives a
        // restart until it is dismissed.** A new run holds nothing of the last
        // one's — no reading, no name held for the run — and draws the same
        // line, the dropped job named by what the record kept beside it;
        // quitting again changes nothing; **opening History leaves it**, since
        // the job has no entry there (the Creator's ninth notes, note 3); its
        // own *Dismiss* ends it for good.
        drop(record);
        for _ in 0..2 {
            let record = crate::record::LocalRecord::open_at(&path.join("local-record.sqlite3")).unwrap();
            let a_new_run = WhatThisRunHolds::default();
            let again = what_is_held(&a_new_run, Some(&record), None);
            assert_eq!(
                again.notice.map(|notice| notice.dropped),
                Some(vec![the_line.clone()]),
                "the line did not outlive the run"
            );
        }
        let record = crate::record::LocalRecord::open_at(&path.join("local-record.sqlite3")).unwrap();
        crate::surface::the_history::history_is_opened(Some(&record));
        assert_eq!(
            what_is_held(&WhatThisRunHolds::default(), Some(&record), None).notice.map(|notice| notice.dropped),
            Some(vec![the_line]),
            "opening History took the dropped job's line"
        );
        crate::record::catch_up_notices::the_dropped_line_is_dismissed(record.connection(), &rest.job).unwrap();
        assert_eq!(what_is_held(&WhatThisRunHolds::default(), Some(&record), None).notice, None);
        let _ = std::fs::remove_dir_all(&path);
    }

    /// **The 38's piece 14c, `Queue & History` L22: a waiting job the machine
    /// dropped because its Blender was removed is named in the notice with the
    /// version** — read off the drop's words by the requests crate's own
    /// helper — its local record goes, and *Dismiss* asks first because the
    /// line is the only place the job is named. A cancel's removal beside it
    /// tells nothing.
    #[test]
    fn a_waiting_job_dropped_for_a_removed_blender_is_named_with_the_version() {
        use crate::servers::the_machines_work::{a_read_forward_pass, AsksTheSupervisor, WhatThisRunHolds};
        use smoozy_queue_history::record::Standing;
        use smoozy_server_protocol::asking_the_supervisor::WhatCameBack;
        use smoozy_server_protocol::framing::{Answered, Fields, Request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS};
        use smoozy_server_protocol::requests as asked;
        use smoozy_state::identities::{Identity, IdentityKind};

        let mut path = std::env::temp_dir();
        path.push(format!("smoozy-queue-removed-blender-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).unwrap();
        let record = crate::record::LocalRecord::open_at(&path.join("local-record.sqlite3")).unwrap();
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
        let store = crate::record::job_records::JobRecords::over(record.connection());
        let mut dropped = crate::record::deletions::tests::a_record_in_history(&server, "Kitchen — clay");
        let mut cancelled = crate::record::deletions::tests::a_record_in_history(&server, "Kitchen — first");
        for job in [&mut dropped, &mut cancelled] {
            job.standing = Standing::Confirmed;
            job.the_render_start_has_been_learned = false;
            store.write_the_provisional_record(job).unwrap();
        }

        struct TheMachine {
            events: Vec<(&'static str, Identity, String)>,
            read: bool,
        }
        impl AsksTheSupervisor for TheMachine {
            fn ask(&mut self, request: &Request) -> WhatCameBack {
                let answered = Answered::to(request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, "test");
                let fields = match request.name.as_str() {
                    asked::THE_READ_FORWARD_REQUEST => {
                        let from: u64 = request.fields.text(asked::THE_CURSOR_FIELD).unwrap().parse().unwrap();
                        let events = if self.read { &[][..] } else { &self.events[..] };
                        self.read = true;
                        let mut fields = Fields::none()
                            .and(asked::HOW_MANY_EVENTS_FIELD, events.len().to_string())
                            .and(asked::THE_NEXT_SEQUENCE_FIELD, (from + events.len() as u64).to_string())
                            .and(asked::THE_OLDEST_RETAINED_FIELD, "0");
                        for (at, (kind, about, says)) in events.iter().enumerate() {
                            let item = |field| asked::the_field_of_item(asked::AN_EVENT, at, field);
                            fields = fields
                                .and(&item(asked::THE_SEQUENCE_FIELD), (from + at as u64).to_string())
                                .and(&item(asked::THE_KIND_FIELD), *kind)
                                .and(&item(asked::THE_ABOUT_FIELD), about.as_text())
                                .and(&item(asked::THE_SAYS_FIELD), says.as_str());
                        }
                        fields
                    }
                    asked::THE_CURSOR_MOVES_FORWARD_REQUEST => Fields::none(),
                    other => return WhatCameBack::ItAnswered(answered.saying_it_does_not_answer_that_request(other)),
                };
                WhatCameBack::ItAnswered(answered.carrying(fields))
            }
        }
        let the_journal = vec![
            (asked::THE_JOB_RECORD_REMOVED_EVENT, cancelled.job.clone(), asked::IT_WAS_REMOVED.to_string()),
            (
                asked::THE_JOB_RECORD_REMOVED_EVENT,
                dropped.job.clone(),
                asked::the_drops_words("4.6.0", "Kitchen — cl"),
            ),
        ];
        let holds = WhatThisRunHolds::default();
        let lent = crate::keys::TheRecordItself::over(record.connection());
        a_read_forward_pass(&server, &lent, &mut TheMachine { events: the_journal, read: false }, &holds);

        assert!(store.read(&dropped.job).unwrap().is_none(), "the dropped job's record stands");
        let held = what_is_held(&holds, Some(&record), None);
        let notice = held.notice.clone().expect("the dropped job is told nowhere");
        assert_eq!(
            notice,
            TheCatchUpNotice {
                counted: Vec::new(),
                dropped: vec![ADroppedJob {
                    job: dropped.job.as_text(),
                    name: "Kitchen — clay".to_string(),
                    what: APart::said("was not started — its Blender", "4.6.0", "was removed."),
                }],
            },
            "only the dropped job is named"
        );
        let asked = the_queue_as_asked(held, Some(TheQueueAsk::Dismiss), Some(&dropped.job.as_text()));
        assert_eq!(asked.asking, Some(the_dismiss_box("Kitchen — clay")));
        drop(record);
        let _ = std::fs::remove_dir_all(&path);
    }

    /// **The Creator's ninth notes, note 3: a re-run is said by the entry's
    /// mark only** (`Design/Queue & History § The catch-up notice`: *"Re-runs
    /// are said by the entry's mark only"*). A notice record holding two
    /// re-runs — one from frame 61, one whose frame is not known yet — draws
    /// no count and no line, while the job's own entry carries *Interrupted
    /// and re-run* from frame 61 (`Queue & History` N1). *(Until then the
    /// banner named each, as the 38's piece 15 built it.)*
    ///
    /// **What it bites on**: a re-run counted as an ending, or named in a
    /// line under the heading, and the entry losing its mark.
    #[test]
    fn a_re_run_is_said_by_the_entrys_mark_only() {
        use smoozy_queue_history::acting::note::AnAttemptMark;
        use smoozy_queue_history::catch_up::{ANoticeEntry, NoticeRecord};
        use smoozy_state::identities::{Identity, IdentityKind};
        let mut path = std::env::temp_dir();
        path.push(format!("smoozy-queue-re-run-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).unwrap();
        let record = crate::record::LocalRecord::open_at(&path.join("local-record.sqlite3")).unwrap();
        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        let store = crate::record::job_records::JobRecords::over(record.connection());
        let mut clay = crate::record::deletions::tests::a_record_in_history(&server, "Kitchen — clay");
        clay.what_the_note_carried.re_run_marks = vec![AnAttemptMark::for_a_re_run(2, &[61, 62])];
        let hall = crate::record::deletions::tests::a_record_in_history(&server, "Hallway — final");
        for made in [&clay, &hall] {
            store.write_the_provisional_record(made).unwrap();
        }
        crate::record::catch_up_notices::the_notice_is_written(
            record.connection(),
            &NoticeRecord {
                server: server.clone(),
                entries: vec![
                    ANoticeEntry::InterruptedAndReRunFrom { job: clay.job.clone(), attempt: 2, frame: Some(61), sequence: 3 },
                    ANoticeEntry::InterruptedAndReRunFrom { job: hall.job.clone(), attempt: 2, frame: None, sequence: 4 },
                ],
                the_account_is_incomplete: false,
            },
        )
        .unwrap();
        let held = what_is_held(&crate::servers::the_machines_work::WhatThisRunHolds::default(), Some(&record), None);
        assert_eq!(held.notice, None, "a re-run was counted or named under the heading");
        let text = said(&the_queue_says(&held));
        assert!(!text.contains("countedCount") && !text.contains("droppedCount"), "{text}");

        let marks = crate::surface::the_history::the_entry_for(
            &clay,
            &crate::surface::the_history::BesideTheRecord::default(),
            false,
            true,
        )
        .entry
        .marks;
        let re_run = marks
            .iter()
            .find(|mark| mark.said == "Interrupted and re-run")
            .expect("the entry lost its re-run mark");
        assert_eq!(re_run.parts, vec![APart::said("from frame", "61", "")]);
        drop(store);
        drop(record);
        let _ = std::fs::remove_dir_all(&path);
    }

    /// **Step 10c's eleventh-pass fix piece 5: the queue's and History's boxes
    /// stay until answered — and this runs the two pages rather than reading
    /// them.** `Design/Queue & History § Stopping asks first`; `Desktop app and
    /// delivery` P10d.
    ///
    /// `web/the-queue.js` and `web/the-history.js` are loaded under Node beside
    /// stand-ins for the modules they import and the least of a document they
    /// need. Each step posts what the application posts and counts the boxes on
    /// the screen. **The eleventh pass on Hyperstack**: the *Stop* and *Cancel*
    /// boxes closed by themselves after about five seconds, because every
    /// redraw removed every box, and P10d posts the queue again about every five
    /// seconds carrying none. History removed its boxes the same way.
    ///
    /// **Skipped, and says so, where no Node is installed.**
    #[test]
    fn the_queues_and_historys_boxes_stay_until_answered() {
        let folder = std::env::temp_dir().join(format!("smoozy-queue-page-harness-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a scratch folder");
        let web = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        for page in ["the-queue.js", "the-history.js", "the-value-drawn.js"] {
            std::fs::copy(web.join(page), folder.join(page)).expect("the page is copied");
        }
        for (name, text) in THE_BOXES_STAND_INS {
            std::fs::write(folder.join(name), text).expect("a stand-in is written");
        }
        let ran = std::process::Command::new("node").arg(folder.join("harness.js")).current_dir(&folder).output();
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the queue's and History's pages were not run");
            let _ = std::fs::remove_dir_all(&folder);
            return;
        };
        let _ = std::fs::remove_dir_all(&folder);
        assert!(ran.status.success(), "the pages did not run: {}", String::from_utf8_lossy(&ran.stderr));
        let said = String::from_utf8_lossy(&ran.stdout).trim().to_string();
        let expected = [
            // The queue's Stop box, raised over a rendering job.
            "raised 1",
            // P10d's next post, the job still there and no box on it.
            "a post with the job still there 1",
            "the same box, untouched yes",
            "a post with the job gone 0",
            "closed by its own control 0",
            "answered, then the post after the go-ahead 0",
            "answered and refused, then a post 1",
            "then closed 0",
            "a group's box over a post with the group still there 1",
            "a group's box over a post with the group gone 0",
            "a fresh box in a post replaces the open one 1 cancel",
            "the notice's box, over a post with no jobs at all 1",
            // Piece Z7's row: its word, the spinner, no job, a grip that does
            // not move, beside the one waiting row.
            "a submitting row Submitting… true true true 2",
            // History's Delete box, and its clear-all box, which is about no job.
            "history raised 1",
            "history, a post with the job still there 1",
            "history, a post with the job gone 0",
            "history's clear-all box over a post with no entries 1",
        ]
        .join("\n");
        assert_eq!(said, expected);
    }

    /// **What the two pages import, standing in**, and the harness that posts
    /// to them. The document is the least the two pages need: elements that
    /// hold children, classes, data and attributes, and a selector reading of
    /// one tag, classes and one attribute — enough for `.modal`,
    /// `.qrow[data-job]` and the rest the pages ask for.
    pub(crate) const THE_BOXES_STAND_INS: [(&str, &str); 10] = [
        ("package.json", r#"{ "type": "module" }"#),
        (
            "event-route.js",
            r#"export function whenTold(kind, handler) {
  const held = globalThis.told.get(kind) || [];
  held.push(handler);
  globalThis.told.set(kind, held);
}"#,
        ),
        ("panes.js", r#"export function theSeamsAreLive() {}"#),
        (
            "the-rail.js",
            r#"export const THE_QUEUE = "queueview";
export const THE_HISTORY = "historyview";
export function aScreenIsRegistered(screen) { globalThis.screens.set(screen.place, screen); }
export function theRowsFigureIs() {}
export function theCurrentPlace() { return THE_HISTORY; }
export function theCurrentPlaceIs(place, which) { if (globalThis.placesGoneTo) globalThis.placesGoneTo.push([place, which === undefined ? null : which]); }"#,
        ),
        (
            "transfers.js",
            r#"export function aTransferIsAskedFor() {}
export function theFormatBoxIsOpenedFor() {}"#,
        ),
        (
            "the-failure-shape.js",
            r#"export const EXPANDED = "expanded";
export function theControlsFrom() { return []; }
export function theFailureShape() {
  const drawn = document.createElement("span");
  drawn.classList.add("afailure");
  return drawn;
}"#,
        ),
        ("servers.js", r#"export function theMomentReads(seconds) { return String(seconds); } export function theConnectionIsDrawnInto() {}"#),
        (
            "the-machines-own-words.js",
            r#"export function theWordsPanel() { const panel = document.createElement("div"); panel.hidden = true; return panel; }
export function theNewestWordsStayInView() {}"#,
        ),
        (
            "the-typed-text.js",
            r#"export function theTypingIsTakenFrom() { return null; }
export function theTypingIsPutBackInto() {}"#,
        ),
        (
            "harness.js",
            r#"function aTest(selector) {
  const found = /^([a-z]*)((?:\.[\w-]+)*)(?:\[([\w-]+)(?:="?([^"\]]*)"?)?\])?$/.exec(selector.trim());
  if (!found) return () => false;
  const [, tag, classes, attribute, value] = found;
  const names = classes ? classes.split(".").filter(Boolean) : [];
  return (element) => {
    if (!element || !element.classList) return false;
    if (tag && element.tag !== tag) return false;
    if (!names.every((name) => element.classList.contains(name))) return false;
    if (attribute) {
      const held = element.attributeOf(attribute);
      if (held === undefined || held === null) return false;
      if (value !== undefined && held !== value) return false;
    }
    return true;
  };
}
function everyUnder(element, test, into) {
  for (const child of element.children || []) {
    if (test(child)) into.push(child);
    everyUnder(child, test, into);
  }
  return into;
}
function anElement(tag) {
  const names = new Set();
  const attributes = new Map();
  const listeners = new Map();
  const element = {
    tag, children: [], parentNode: null, dataset: {}, hidden: false, text: "", type: "", id: "", title: "",
    style: { setProperty() {} },
    classList: {
      add: (...more) => more.forEach((one) => names.add(one)),
      remove: (...less) => less.forEach((one) => names.delete(one)),
      toggle: (one, on) => { if (on === undefined ? !names.has(one) : on) names.add(one); else names.delete(one); },
      contains: (one) => names.has(one),
    },
    set className(value) { names.clear(); String(value).split(" ").filter(Boolean).forEach((one) => names.add(one)); },
    get className() { return Array.from(names).join(" "); },
    set textContent(value) { element.children = []; element.text = String(value); },
    get textContent() { return element.text; },
    get childNodes() { return element.children; },
    get isConnected() { let at = element; while (at.parentNode) at = at.parentNode; return at === globalThis.theRoot; },
    appendChild(child) {
      if (child.parentNode && child.parentNode.removeChild) child.parentNode.removeChild(child);
      child.parentNode = element;
      element.children.push(child);
      return child;
    },
    insertBefore(child) { return element.appendChild(child); },
    removeChild(child) { element.children = element.children.filter((one) => one !== child); child.parentNode = null; return child; },
    replaceChildren(...fresh) { for (const one of element.children) one.parentNode = null; element.children = []; fresh.forEach((one) => element.appendChild(one)); },
    replaceWith(fresh) {
      const holder = element.parentNode;
      if (!holder) return;
      holder.children = holder.children.map((one) => (one === element ? fresh : one));
      fresh.parentNode = holder;
      element.parentNode = null;
    },
    remove() { if (element.parentNode) element.parentNode.removeChild(element); },
    after(node) {
      const holder = element.parentNode;
      if (!holder) return;
      node.parentNode = holder;
      holder.children.splice(holder.children.indexOf(element) + 1, 0, node);
    },
    setAttribute(name, value) { attributes.set(name, String(value)); },
    getAttribute(name) { return attributes.has(name) ? attributes.get(name) : null; },
    removeAttribute(name) { attributes.delete(name); },
    attributeOf(name) {
      if (name.startsWith("data-")) return element.dataset[name.slice(5).replace(/-([a-z])/g, (_, letter) => letter.toUpperCase())];
      return attributes.has(name) ? attributes.get(name) : undefined;
    },
    addEventListener(kind, handler) { const held = listeners.get(kind) || []; held.push(handler); listeners.set(kind, held); },
    click() { for (const handler of listeners.get("click") || []) handler({ target: element, preventDefault() {} }); },
    focus() { globalThis.document.activeElement = element; },
    querySelector(selector) { return everyUnder(element, aTest(selector), [])[0] || null; },
    querySelectorAll(selector) { return everyUnder(element, aTest(selector), []); },
    closest(selector) { const test = aTest(selector); let at = element; while (at) { if (test(at)) return at; at = at.parentNode; } return null; },
    contains(other) { let at = other; while (at) { if (at === element) return true; at = at.parentNode; } return false; },
    getBoundingClientRect() { return { left: 0, bottom: 0, width: 0, height: 0 }; },
  };
  return element;
}
globalThis.told = new Map();
globalThis.screens = new Map();
globalThis.theRoot = anElement("body");
globalThis.document = {
  createElement: anElement,
  createElementNS: (space, tag) => anElement(tag),
  createTextNode: (text) => ({ text, children: [] }),
  addEventListener() {}, removeEventListener() {},
  documentElement: { dataset: {} },
  body: globalThis.theRoot,
};
globalThis.window = {
  innerWidth: 1000, innerHeight: 800,
  __TAURI__: {
    core: {
      invoke(command) {
        if (command === "the_pane_layout") return Promise.resolve({ seams: [] });
        return Promise.resolve(null);
      },
    },
  },
};

await import("./the-queue.js");
await import("./the-history.js");

const tell = (kind, said) => { for (const handler of globalThis.told.get(kind) || []) handler({ said }); };
const boxesOn = (host) => host.children.filter((child) => child.classList && child.classList.contains("modal"));
const say = (what, host) => console.log(what + " " + boxesOn(host).length);
const theButtonsOf = (box) => box.querySelectorAll("button.btn");

// ── The queue ──
const queue = anElement("div");
globalThis.theRoot.appendChild(queue);
globalThis.screens.get("queueview").itIsDrawn(queue);
const QUEUE = "queue:the-queue";
const ANSWER = "queue:the-answer";
const aQueue = (more) => Object.assign({
  theHeading: "Queue", lineCount: "1", line0Server: "s1", line0Name: "RunPod 4090",
  rowCount: "2", row0Job: "j1", row0Line: "0", row0Name: "Kitchen", row1Job: "j2", row1Line: "0", row1Name: "Hallway",
  entryJob: "j1",
}, more || {});
const aStopBox = { boxWhich: "stop", boxKeep: "Keep rendering", boxGoAct: "stop-the-job", boxGoCount: "1", boxGo0Lead: "Stop" };

tell(QUEUE, aQueue(aStopBox));
say("raised", queue);
const first = boxesOn(queue)[0];
tell(QUEUE, aQueue());
say("a post with the job still there", queue);
console.log("the same box, untouched " + (boxesOn(queue)[0] === first ? "yes" : "no"));
tell(QUEUE, aQueue({ rowCount: "1", row0Job: "j2", row0Name: "Hallway", entryJob: "j2" }));
say("a post with the job gone", queue);

tell(QUEUE, aQueue(aStopBox));
theButtonsOf(boxesOn(queue)[0])[0].click();
say("closed by its own control", queue);

tell(QUEUE, aQueue(aStopBox));
theButtonsOf(boxesOn(queue)[0])[1].click();
tell(QUEUE, aQueue());
say("answered, then the post after the go-ahead", queue);

tell(QUEUE, aQueue(aStopBox));
theButtonsOf(boxesOn(queue)[0])[1].click();
tell(ANSWER, { act: "stop-the-job", said: "This job is not rendering." });
tell(QUEUE, aQueue());
say("answered and refused, then a post", queue);
if (boxesOn(queue)[0]) theButtonsOf(boxesOn(queue)[0])[0].click();
say("then closed", queue);

const aGroup = { groupCount: "1", group0Group: "g1", group0Name: "Kitchen", group0Scenes: "2", row0Group: "0", row1Group: "0" };
tell(QUEUE, aQueue(Object.assign({}, aGroup, { boxWhich: "group-cancel", boxGroup: "g1", boxKeep: "Keep them", boxGoAct: "cancel-the-group" })));
tell(QUEUE, aQueue(aGroup));
say("a group's box over a post with the group still there", queue);
tell(QUEUE, aQueue());
say("a group's box over a post with the group gone", queue);

tell(QUEUE, aQueue(aStopBox));
tell(QUEUE, aQueue({ entryJob: "j2", boxWhich: "cancel", boxKeep: "Keep the job", boxGoAct: "cancel-the-job" }));
const fresh = boxesOn(queue);
console.log("a fresh box in a post replaces the open one " + fresh.length + " " + (fresh[0] ? fresh[0].querySelector("button.quiet").textContent === "Keep the job" ? "cancel" : "stop" : "none"));
theButtonsOf(fresh[0])[0].click();

tell(QUEUE, aQueue({ boxWhich: "dismiss", boxKeep: "Keep the notice", boxGoAct: "dismiss-the-notice" }));
tell(QUEUE, aQueue({ rowCount: "0", entryJob: "" }));
say("the notice's box, over a post with no jobs at all", queue);

// **The Creator's seventh notes, piece Z7: a job just submitted** — the
// drawing's `qrow sub`, its word Rust's and the spinner after it, with no job,
// no grip that moves and nothing to press.
tell(QUEUE, aQueue({ row1Job: "", row1Submitting: "yes", row1Name: "Bathroom", row1State: "Submitting…" }));
const theSubmitting = queue.querySelector("div.qrow.sub");
console.log("a submitting row " + (theSubmitting ? [
  theSubmitting.querySelector("span.qst").textContent,
  !!theSubmitting.querySelector("i.spin"),
  theSubmitting.dataset.job === undefined,
  !!theSubmitting.querySelector("span.qgrip.nodrag"),
  queue.querySelectorAll(".qrow").length,
].join(" ") : "none"));

// ── History ──
const history = anElement("div");
globalThis.theRoot.appendChild(history);
globalThis.screens.get("historyview").itIsDrawn(history);
const HISTORY = "history:the-history";
const aHistory = (more) => Object.assign({
  theHeading: "History", hrowCount: "2", hrow0Job: "j1", hrow0Name: "Kitchen", hrow1Job: "j2", hrow1Name: "Hallway",
  hrowChosen: "j1", entryJob: "j1",
}, more || {});
tell(HISTORY, aHistory({ boxWhich: "delete", boxKeep: "Keep it", boxGoAct: "delete-the-entry" }));
say("history raised", history);
tell(HISTORY, aHistory());
say("history, a post with the job still there", history);
tell(HISTORY, aHistory({ hrowCount: "1", hrow0Job: "j2", hrow0Name: "Hallway", hrowChosen: "j2", entryJob: "j2" }));
say("history, a post with the job gone", history);
tell(HISTORY, aHistory({ boxWhich: "clear-all", boxKeep: "Keep them", boxGoAct: "clear-all-history" }));
tell(HISTORY, aHistory({ hrowCount: "0", hrowChosen: "", entryJob: "" }));
say("history's clear-all box over a post with no entries", history);
"#,
        ),
    ];

    /// **`Rendering` S10 and S11: an all-scenes act the connection broke
    /// part-way names its scenes, and *Submit the remaining scenes* sends the
    /// ones resolved as not submitted and nothing else** (the 38's piece 2).
    /// A scene Smoozy could not account for moves as its record resolves; the
    /// banner goes once every scene is in the line.
    #[test]
    fn a_submission_that_arrived_in_part_names_its_scenes_and_sends_only_the_rest() {
        use crate::submitting::{AJobAsSent, WhatBecameOfOneJob as Became, WhatTheActCameTo};
        use smoozy_render_settings::names::{ExactName, Provenance};
        use smoozy_state::identities::{Identity, IdentityKind};

        let mut path = std::env::temp_dir();
        path.push(format!("smoozy-queue-part-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).unwrap();
        let record = crate::record::LocalRecord::open_at(&path.join("local-record.sqlite3")).unwrap();
        let mut given = crate::submitting::tests::a_page_that_hands_over(true, None);
        let server = given.server.local_server.clone();
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
        let a_scene = |name: &str| {
            smoozy_render_settings::payload::compose(smoozy_render_settings::payload::Composition {
                version_pick: &smoozy_render_settings::file_half::BlenderIdentity {
                    path: "/b".to_string(),
                    version: smoozy_server_setup::blender::Version {
                        major: 4,
                        minor: 5,
                        patch: 12,
                    },
                },
                scene: Some(&smoozy_render_settings::names::NamedAnswer {
                    name: ExactName::as_the_reading_produced_it(name),
                    provenance: Provenance::AKeptSetup {
                        the_old_job: "a".to_string(),
                    },
                }),
                answers: &smoozy_render_settings::store::SceneEntries::default(),
                standing: None,
                stale: false,
            })
        };
        given.scene_jobs = vec![a_scene("Kitchen"), a_scene("Hall"), a_scene("Garden")];
        given.label = Some("Flat".to_string());
        let jobs: Vec<Identity> = (0..3).map(|_| Identity::issue(IdentityKind::Job).unwrap()).collect();
        let named: Vec<AJobAsSent> = jobs
            .iter()
            .enumerate()
            .map(|(at, job)| AJobAsSent {
                job: job.clone(),
                of_the_scene_job: at,
            })
            .collect();
        // Hall's record stands provisional: Smoozy could not account for it.
        let store = crate::record::job_records::JobRecords::over(record.connection());
        let mut hall = crate::record::deletions::tests::a_record_in_history(&server, "Flat — Hall");
        hall.job = jobs[1].clone();
        hall.standing = smoozy_queue_history::record::Standing::Provisional;
        store.write_the_provisional_record(&hall).unwrap();
        let came_to = WhatTheActCameTo::TheseJobs(vec![
            Became::ItIsInTheLine,
            Became::NotKnown,
            Became::NotAttemptedBehindAnUnaccountedOne,
        ]);
        // **An act that arrived whole, or one scene, holds nothing.**
        let mut whole = given.clone();
        whole.server.local_server = Identity::issue(IdentityKind::LocalServer).unwrap();
        an_act_ended(&whole, &WhatTheActCameTo::TheseJobs(vec![Became::ItIsInTheLine; 3]), &named);
        assert!(the_parts_this_run().lock().unwrap().get(&whole.server.local_server.as_text()).is_none());

        an_act_ended(&given, &came_to, &named);
        let connected = |at: &str| (at == server.as_text()).then(|| ("the big box".to_string(), true));
        let part = the_part_now(&store, &connected).expect("the part is drawn");
        assert_eq!((part.confirmed, part.of), (1, 3));
        assert_eq!(part.submitted, vec!["Kitchen".to_string()]);
        assert_eq!(part.not_known, vec!["Hall".to_string()]);
        assert_eq!(part.not_submitted, vec!["Garden".to_string()]);
        assert_eq!(part.server, "the big box");
        assert_eq!(part.submission, "Flat");
        assert_eq!(part.the_servers_row, server.as_text());
        assert!(part.it_can_go);
        // **Not connected: it waits.** A server the record holds no row for
        // says nothing at all.
        assert!(!the_part_now(&store, &|at: &str| (at == server.as_text()).then(|| (String::new(), false)))
            .unwrap()
            .it_can_go);
        assert_eq!(the_part_now(&store, &|_: &str| None), None);

        // **S11: the remaining act is the scenes resolved as not submitted.**
        let remaining = the_remaining_act(&store, &server.as_text()).expect("a scene remains");
        let names = |given: &crate::submitting::WhatThePageHandsOver| -> Vec<String> {
            given
                .scene_jobs
                .iter()
                .map(|payload| payload.scene.as_ref().unwrap().name.as_text().to_string())
                .collect()
        };
        assert_eq!(names(&remaining), vec!["Garden".to_string()]);

        // **V1 step 2 resolves Hall at a later connection: absent is not
        // submitted**, and the remaining act then takes it too.
        crate::record::job_records::JobRecords::over(record.connection())
            .remove(&jobs[1])
            .unwrap();
        let part = the_part_now(&store, &connected).unwrap();
        assert_eq!(part.not_submitted, vec!["Hall".to_string(), "Garden".to_string()]);
        let remaining = the_remaining_act(&store, &server.as_text()).unwrap();
        assert_eq!(names(&remaining), vec!["Hall".to_string(), "Garden".to_string()]);

        // **The remaining act lands whole: the banner goes.**
        let sent: Vec<AJobAsSent> = (0..2)
            .map(|at| AJobAsSent {
                job: Identity::issue(IdentityKind::Job).unwrap(),
                of_the_scene_job: at,
            })
            .collect();
        the_remaining_act_ended(
            &remaining,
            &WhatTheActCameTo::TheseJobs(vec![Became::ItIsInTheLine; 2]),
            &sent,
        );
        assert_eq!(the_part_now(&store, &connected), None);
        assert!(the_parts_this_run().lock().unwrap().get(&server.as_text()).is_none());
        let _ = std::fs::remove_dir_all(&path);
    }

    /// **The part banner's control goes where it can, and waits with its
    /// reason where it cannot** (`Rendering` S11, the 38's piece 2) — the
    /// queue's page, run under Node over the boxes harness's own stand-ins:
    /// pressed, it asks for *Submit the remaining scenes* on the server the
    /// application named; told it cannot go, it is unavailable and says it
    /// needs the connection. **Skipped, and says so, where no Node is
    /// installed.**
    #[test]
    fn the_part_banners_control_asks_for_the_remaining_scenes_or_waits() {
        let folder = std::env::temp_dir().join(format!("smoozy-queue-part-page-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a scratch folder");
        let web = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        for page in ["the-queue.js", "the-value-drawn.js"] {
            std::fs::copy(web.join(page), folder.join(page)).expect("the page is copied");
        }
        for (name, text) in THE_BOXES_STAND_INS {
            std::fs::write(folder.join(name), text).expect("a stand-in is written");
        }
        let shim = THE_BOXES_STAND_INS
            .iter()
            .find(|(name, _)| *name == "harness.js")
            .map(|(_, text)| *text)
            .and_then(|text| text.split("await import(\"./the-queue.js\");").next())
            .expect("the boxes harness's own page shim");
        let harness = format!(
            r#"{shim}
globalThis.invoked = [];
globalThis.window.__TAURI__.core.invoke = (command, args) => {{
  globalThis.invoked.push([command, args]);
  return Promise.resolve(command === "the_pane_layout" ? {{ seams: [] }} : null);
}};
await import("./the-queue.js");
const tell = (kind, said) => {{ for (const handler of globalThis.told.get(kind) || []) handler({{ said }}); }};
const queue = anElement("div");
globalThis.theRoot.appendChild(queue);
globalThis.screens.get("queueview").itIsDrawn(queue);
const part = (canGo) => ({{
  theHeading: "Queue", lineCount: "0", rowCount: "0",
  thePartConfirmed: "1", ofWord: "of", thePartOf: "3", thePartSays: "scenes are confirmed submitted",
  thePartSecond: "Connection lost during submission.",
  thePartServer: "the big box", thePartSubmission: "Flat", thePartControl: "Submit the remaining scenes",
  needsAConnectionTo: "Needs a connection to", aFullStop: ".", hide: "Hide",
  thePartServerRow: "s1", thePartCanGo: canGo, partNotSubmittedCount: "1", partNotSubmitted0: "Garden",
}});
const theControl = () => queue.querySelectorAll("button.btn").find((one) => one.text === "Submit the remaining scenes");
tell("queue:the-queue", part("yes"));
const going = theControl();
going.click();
const asked = globalThis.invoked.filter(([command]) => command === "an_act_on_a_job").map(([, args]) => args);
const why = (control) => (control.parentNode.children.find((one) => one.classList && one.classList.contains("ax-why")) || {{}}).text || "";
console.log(JSON.stringify({{ disabled: going.disabled === true, asked, why: why(going) }}));
tell("queue:the-queue", part(""));
const waiting = theControl();
console.log(JSON.stringify({{ disabled: waiting.disabled === true, why: why(waiting) }}));
"#
        );
        std::fs::write(folder.join("harness.js"), harness).expect("the harness is written");
        let ran = std::process::Command::new("node").arg(folder.join("harness.js")).current_dir(&folder).output();
        let _ = std::fs::remove_dir_all(&folder);
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the queue's page was not run");
            return;
        };
        assert!(ran.status.success(), "the page did not run: {}", String::from_utf8_lossy(&ran.stderr));
        let said = String::from_utf8_lossy(&ran.stdout).trim().to_string();
        let lines: Vec<serde_json::Value> =
            said.lines().map(|line| serde_json::from_str(line).expect("one JSON line")).collect();
        assert_eq!(
            lines,
            vec![
                serde_json::json!({
                    "disabled": false,
                    "asked": [{ "act": "submit-the-remaining-scenes", "job": null, "from": null, "group": "s1", "after": null }],
                    "why": "",
                }),
                serde_json::json!({ "disabled": true, "why": "Needs a connection to" }),
            ]
        );
    }

    #[test]
    fn film_page_a_chosen_frame_survives_the_move_to_history() {
        let lines = a_page_run("film-frame", &["the-queue.js", "the-history.js", "the-value-drawn.js"], &[], r#"
await import("./the-queue.js"); await import("./the-history.js");
const tell = (kind, said) => { for (const handler of globalThis.told.get(kind) || []) handler({said}); };
const make = (job, frames) => {
 const said = {theHeading:"Queue",lineCount:"1",line0Server:"s1",line0Name:"Server",rowCount:"1",row0Job:job,row0Line:"0",row0Name:"Shot",
 entryJob:job,entryName:"Shot",previewPicture:job,frameHeading:"Frame preview",frameWord:"Frame",previousFrame:"Previous frame",nextFrame:"Next frame",
 previewHeldCount:String(frames.length), previewFrame:String(frames.at(-1)),showLatest:"Show latest",showLast:"Show last"};
 frames.forEach((frame,i) => {said["previewHeldFrame"+i]=String(frame);said["previewHeldLayerCount"+i]="2";said["previewHeldMain"+i]="0";
 for(let k=0;k<2;k++){said["previewHeld"+i+"Layer"+k]=["Beauty","FX"][k];said["previewHeld"+i+"Place"+k]=String(k);}});
 return said;
};
const queue = anElement("div"), history = anElement("div");
globalThis.theRoot.appendChild(queue); globalThis.theRoot.appendChild(history);
globalThis.screens.get("queueview").itIsDrawn(queue); globalThis.screens.get("historyview").itIsDrawn(history);
tell("queue:the-queue", make("shot",[342,344,346,348]));
queue.querySelector("aside.qdside").querySelectorAll("button.sb")[0].click(); // 346
queue.querySelector("button.pkb").click();
queue.querySelector("span.ddl").children[1].click(); // FX
const final = make("shot",[342,344,346,348,430]);
Object.assign(final,{theHeading:"History",hrowCount:"1",hrow0Job:"shot",hrow0Name:"Shot",hrowChosen:"shot"});
tell("queue:the-queue",{theHeading:"Queue",lineCount:"0",rowCount:"0"});
tell("history:the-history",final);
const side = () => history.querySelector("aside.qdside");
console.log(JSON.stringify({frame:side().querySelector("input").value,layer:side().querySelector("button.pkb").textContent}));
side().querySelectorAll("button.sb")[1].click();
console.log(JSON.stringify({frame:side().querySelector("input").value}));
tell("queue:the-queue",make("following",[346,348]));
const following=make("following",[346,348,430]);
Object.assign(following,{theHeading:"History",hrowCount:"1",hrow0Job:"following",hrow0Name:"Shot",hrowChosen:"following"});
tell("history:the-history",following);
console.log(JSON.stringify({frame:side().querySelector("input").value}));
"#).unwrap();
        assert_eq!(lines[0], serde_json::json!({"frame":"346","layer":"FX"}));
        assert_eq!(lines[1], serde_json::json!({"frame":"348"}));
        assert_eq!(lines[2], serde_json::json!({"frame":"430"}));
    }

    /// **Runs a page harness built on the boxes harness's own stand-ins**:
    /// `pages` copied from `web/`, `real` copied over the stand-in of the same
    /// name, and `body` after the stand-in document. Answers each line the
    /// harness printed, as JSON, or nothing where no Node is installed.

    fn a_page_run(name: &str, pages: &[&str], real: &[&str], body: &str) -> Option<Vec<serde_json::Value>> {
        let folder = std::env::temp_dir().join(format!("smoozy-queue-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a scratch folder");
        let web = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        for (stand_in, text) in THE_BOXES_STAND_INS {
            std::fs::write(folder.join(stand_in), text).expect("a stand-in is written");
        }
        for page in pages.iter().chain(real) {
            std::fs::copy(web.join(page), folder.join(page)).expect("the page is copied");
        }
        let shim = THE_BOXES_STAND_INS
            .iter()
            .find(|(stand_in, _)| *stand_in == "harness.js")
            .map(|(_, text)| *text)
            .and_then(|text| text.split("await import(\"./the-queue.js\");").next())
            .expect("the boxes harness's own page shim");
        std::fs::write(folder.join("harness.js"), [shim, body].concat()).expect("the harness is written");
        let ran = std::process::Command::new("node").arg(folder.join("harness.js")).current_dir(&folder).output();
        let _ = std::fs::remove_dir_all(&folder);
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the page was not run");
            return None;
        };
        assert!(ran.status.success(), "the page did not run: {}", String::from_utf8_lossy(&ran.stderr));
        Some(
            String::from_utf8_lossy(&ran.stdout)
                .lines()
                .map(|line| serde_json::from_str(line).expect("one JSON line"))
                .collect(),
        )
    }

    /// Capture Chrome's own output, with a file address made from the path
    /// by the host system. The deadline kills and reaps Chrome on expiry.
    fn the_browser_measurement(chrome: &std::path::Path, folder: &std::path::Path) -> serde_json::Value {
        let ran = std::process::Command::new("python3")
            .args(["-c", r#"import pathlib, subprocess, sys
page = pathlib.Path(sys.argv[2]).resolve().as_uri()
try:
    ran = subprocess.run([sys.argv[1], *sys.argv[3:], page], capture_output=True, timeout=45)
except subprocess.TimeoutExpired as failed:
    sys.stdout.buffer.write(failed.stdout or b"")
    sys.stderr.buffer.write(failed.stderr or b"")
    sys.stderr.write("\nChrome exceeded the 45-second measurement deadline: " + page + "\n")
    sys.exit(124)
sys.stdout.buffer.write(ran.stdout)
sys.stderr.buffer.write(ran.stderr)
sys.stderr.write("\nChrome return code: " + str(ran.returncode) + "; page: " + page + "\n")
sys.exit(ran.returncode)
"#])
            .arg(chrome)
            .arg(folder.join("index.html"))
            .args(["--headless", "--disable-gpu", "--no-first-run", "--allow-file-access-from-files",
                "--virtual-time-budget=5000", "--window-size=1200,900", "--dump-dom"])
            .arg(format!("--user-data-dir={}", folder.join("profile").display()))
            .output();
        let _ = std::fs::remove_dir_all(folder);
        let ran = ran.expect("the browser measurement starts");
        let page = String::from_utf8_lossy(&ran.stdout);
        let errors = String::from_utf8_lossy(&ran.stderr);
        assert!(ran.status.success(), "browser measurement failed ({}):\nstdout: {page}\nstderr: {errors}", ran.status);
        let result = page.split("<pre id=\"out\">").nth(1).and_then(|part| part.split("</pre>").next()).unwrap_or("");
        let measured: serde_json::Value = serde_json::from_str(result)
            .unwrap_or_else(|error| panic!("no browser measurement: {error} ({}):\nstdout: {page}\nstderr: {errors}", ran.status));
        assert!(measured.get("error").is_none(), "{measured}");
        measured
    }

    /// The seventeenth notes, piece 2: measure the actual shared preview
    /// on both pages, with the application's stylesheets. The two arrows'
    /// text boxes are centred, and changing the number or Show latest moves
    /// none of the surrounding controls. No installed app is opened.
    #[test]
    fn seventeenth_piece_2_the_frame_controls_are_centred_and_stay_still_in_a_real_browser() {
        let chrome = [
            "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
            "/Applications/Chromium.app/Contents/MacOS/Chromium",
            "/usr/bin/google-chrome", "/usr/bin/chromium", "/usr/bin/chromium-browser",
            "C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe",
        ].into_iter().map(std::path::PathBuf::from).find(|path| path.exists());
        let Some(chrome) = chrome else {
            eprintln!("SKIPPED: no Chrome; the frame controls were not measured");
            return;
        };
        let folder = std::env::temp_dir().join(format!("smoozy-seventeenth-piece-2-browser-{}", std::process::id()));
        std::fs::create_dir_all(&folder).unwrap();
        let web = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        for (name, text) in THE_BOXES_STAND_INS {
            if name != "harness.js" { std::fs::write(folder.join(name), text).unwrap(); }
        }
        for entry in std::fs::read_dir(&web).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().and_then(|ext| ext.to_str()) == Some("css") {
                std::fs::copy(&path, folder.join(path.file_name().unwrap())).unwrap();
            }
        }
        for name in ["the-queue.js", "the-history.js", "the-value-drawn.js"] {
            std::fs::copy(web.join(name), folder.join(name)).unwrap();
        }
        let words = serde_json::json!({
            "frameHeading": FRAME_PREVIEW, "frameWord": FRAME_WORD,
            "previousFrame": PREVIOUS_FRAME, "nextFrame": NEXT_FRAME,
            "showLatest": SHOW_LATEST, "notColourAccurate": NOT_COLOUR_ACCURATE,
            "held0First0Lead": RENDER_ALL_WITH_NO_COUNT, "held0Answer": CANCEL_IT,
        });
        std::fs::write(folder.join("words.js"), format!("export default {words};")).unwrap();
        std::fs::write(folder.join("index.html"), THE_FRAME_CONTROLS_MEASURED).unwrap();
        let measured = the_browser_measurement(&chrome, &folder);
        for screen in ["queue", "history"] {
            let cases = measured[screen]["steps"].as_array().expect("measured steps");
            for one in cases {
                for arrow in one["arrows"].as_array().unwrap() {
                    assert!(arrow["dx"].as_f64().unwrap().abs() <= 0.5, "{screen}: glyph off centre horizontally: {arrow}");
                    assert!(arrow["dy"].as_f64().unwrap().abs() <= 0.5, "{screen}: glyph off centre vertically: {arrow}");
                }
                for part in ["back", "number", "forward", "below"] {
                    for edge in ["x", "y", "width", "height"] {
                        let before = cases[0][part][edge].as_f64().unwrap();
                        let after = one[part][edge].as_f64().unwrap();
                        assert!((before - after).abs() <= 0.5, "{screen}: {part} {edge} moved from {before} to {after}: {measured}");
                    }
                }
            }
        }
        assert_eq!(measured["queue"]["latest"], serde_json::json!([true, true, true, true]));
        assert_eq!(measured["history"]["latest"], serde_json::json!([true, true, true, true]));
        for answer in measured["answers"].as_array().unwrap() {
            assert_eq!(answer["job"], "waiting-job", "the preview's job must not receive approval");
        }
        assert_eq!(measured["answers"].as_array().unwrap().len(), 2);
    }

    #[test]
    fn seventeenth_piece_2_again_typed_frames_large_preview_and_copy_in_a_real_browser() {
        let chrome = [
            "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
            "/Applications/Chromium.app/Contents/MacOS/Chromium",
            "/usr/bin/google-chrome", "/usr/bin/chromium", "/usr/bin/chromium-browser",
            "C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe",
        ].into_iter().map(std::path::PathBuf::from).find(|path| path.exists());
        let Some(chrome) = chrome else {
            eprintln!("SKIPPED: no Chrome; the frame controls were not measured");
            return;
        };
        let folder = std::env::temp_dir().join(format!("smoozy-seventeenth-piece-2-again-browser-{}", std::process::id()));
        std::fs::create_dir_all(&folder).unwrap();
        let web = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        for (name, text) in THE_BOXES_STAND_INS {
            if name != "harness.js" { std::fs::write(folder.join(name), text).unwrap(); }
        }
        for entry in std::fs::read_dir(&web).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().and_then(|ext| ext.to_str()) == Some("css") {
                std::fs::copy(&path, folder.join(path.file_name().unwrap())).unwrap();
            }
        }
        for name in ["the-queue.js", "the-history.js", "the-value-drawn.js", "the-machines-own-words.js", "the-typed-text.js"] {
            std::fs::copy(web.join(name), folder.join(name)).unwrap();
        }
        let words = serde_json::json!({
            "frameHeading": FRAME_PREVIEW, "frameWord": FRAME_WORD,
            "previousFrame": PREVIOUS_FRAME, "nextFrame": NEXT_FRAME,
            "showLatest": SHOW_LATEST, "showLast": "Show last", "showingFrame": "Showing frame",
            "frameInvalid": "Enter a whole frame number.", "frameUnavailable": "is not available.",
            "previewClose": "Close", "previewUnavailable": "This preview is no longer available.",
            "enlargePreview": ENLARGE_PREVIEW, "notColourAccurate": NOT_COLOUR_ACCURATE,
            "didNotRenderOnThisFrame": DID_NOT_RENDER_ON_THIS_FRAME, "viewLayer": VIEW_LAYER,
            "renderItAgain": "Render it again", "submitAgain": "Submit again", "submitAgainFrames": "3", "submitAgainFramesWord": "frames",
            "carryOn": "Resume", "carryOnFrom": "12", "carryOnFrames": "2", "carryOnFramesWord": "frames", "carryingOnFromFrame": "From frame",
            "held0First0Lead": RENDER_ALL_WITH_NO_COUNT, "held0Answer": CANCEL_IT,
        });
        std::fs::write(folder.join("words.js"), format!("export default {words};")).unwrap();
        std::fs::write(folder.join("index.html"), THE_NEW_PREVIEW_CONTROLS).unwrap();
        let measured = the_browser_measurement(&chrome, &folder);
        for screen in ["queue", "history"] {
            let checks = measured[screen].as_object().unwrap();
            for (name, passed) in checks { assert_eq!(passed, &serde_json::json!(true), "{screen}: {name}: {measured}"); }
        }
        for (name, passed) in measured["copy"].as_object().unwrap() {
            assert_eq!(passed, &serde_json::json!(true), "copy: {name}: {measured}");
        }
    }

    /// **The queue's page with the Telegram group, run in a real browser**
    /// with the application's stylesheets, the real `the-typed-text.js`, and
    /// `page` as the document; the measurement it writes, or nothing where no
    /// Chrome is installed. The twenty-fifth notes, note 1.
    fn the_telegram_group_in_a_real_browser(name: &str, page: &str) -> Option<serde_json::Value> {
        let chrome = [
            "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
            "/Applications/Chromium.app/Contents/MacOS/Chromium",
            "/usr/bin/google-chrome", "/usr/bin/chromium", "/usr/bin/chromium-browser",
            "C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe",
        ].into_iter().map(std::path::PathBuf::from).find(|path| path.exists());
        let Some(chrome) = chrome else {
            eprintln!("SKIPPED: no Chrome; the Telegram group was not run in a browser");
            return None;
        };
        let folder = std::env::temp_dir().join(format!("smoozy-twenty-fifth-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).unwrap();
        let web = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        for (stand_in, text) in THE_BOXES_STAND_INS {
            if stand_in != "harness.js" { std::fs::write(folder.join(stand_in), text).unwrap(); }
        }
        for entry in std::fs::read_dir(&web).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().and_then(|ext| ext.to_str()) == Some("css") {
                std::fs::copy(&path, folder.join(path.file_name().unwrap())).unwrap();
            }
        }
        for page in ["the-queue.js", "the-value-drawn.js", "the-typed-text.js"] {
            std::fs::copy(web.join(page), folder.join(page)).unwrap();
        }
        std::fs::write(folder.join("entry.js"), THE_TELEGRAM_ENTRY).unwrap();
        std::fs::write(folder.join("index.html"), page).unwrap();
        Some(the_browser_measurement(&chrome, &folder))
    }

    /// **The twenty-fifth notes, note 1 (the minutes): the Queue's minutes
    /// field keeps what is typed across a redraw** — the map *The live
    /// Telegram pass's faults*, section *The Queue's minutes field drops typing
    /// while a job renders*; `Design/Whole app § The window`. The entry is
    /// drawn again while a job renders, and the field was made new from the
    /// server's value each time, so a click and then a redraw left the keys
    /// landing on nothing. Now: a field clicked keeps the focus across a
    /// redraw; typed into, it keeps the text and the caret over the server's
    /// value; the field taken out sends nothing; Enter sends what was typed;
    /// a field nobody is typing in shows the server's value; and another job's
    /// entry does not take the typing. **It failed before the fix**: the new
    /// field read the server's 7, had no focus, and Enter sent nothing.
    #[test]
    fn twenty_fifth_1_the_minutes_field_keeps_its_typing_across_a_redraw_in_a_real_browser() {
        let Some(measured) = the_telegram_group_in_a_real_browser("minutes", THE_MINUTES_ACROSS_A_REDRAW) else {
            return;
        };
        for (name, passed) in measured.as_object().unwrap() {
            assert_eq!(passed, &serde_json::json!(true), "{name}: {measured}");
        }
    }

    /// **The twenty-fifth notes, note 1: *Downloading* and *Telegram* line up
    /// in the Queue's entry, two lines each** — the Creator's picture, *"see
    /// the atrocious alignment"*, then, on the first fix's picture, *"unite the
    /// Download section name and the checkbox text and align that with
    /// Telegram section name. That way we'll have two lines for both"*. No
    /// *Downloading* heading: the switch reads *Download frames as they
    /// render*. Where the window's first narrowing step puts the two side by
    /// side, the switch's line is centred on the *Telegram* heading's, and the
    /// two lists are level, one height and one width on the line below. In
    /// the preview's column, where they stack, the lists are one width too.
    /// Measured with the application's stylesheets.
    #[test]
    fn twenty_fifth_1_downloading_and_telegram_line_up_in_a_real_browser() {
        let Some(measured) = the_telegram_group_in_a_real_browser("alignment", THE_TWO_LISTS_MEASURED) else {
            return;
        };
        let at = |one: &serde_json::Value| one.as_f64().unwrap();
        let near = |one: &serde_json::Value, other: &serde_json::Value| (at(one) - at(other)).abs() <= 0.5;
        let middle = |one: &serde_json::Value| at(&one["y"]) + at(&one["height"]) / 2.0;
        for case in ["side", "minutes", "stacked"] {
            assert_eq!(measured[case]["headings"], serde_json::json!(["Telegram"]), "{case}: one heading: {measured}");
            assert_eq!(measured[case]["switchSays"], "Download frames as they render", "{case}: {measured}");
            assert!(near(&measured[case]["dlList"]["width"], &measured[case]["tgList"]["width"]), "{case}: one width: {measured}");
            assert!(near(&measured[case]["dlList"]["height"], &measured[case]["tgList"]["height"]), "{case}: one height: {measured}");
            assert!(at(&measured[case]["switch"]["y"]) + at(&measured[case]["switch"]["height"]) <= at(&measured[case]["dlList"]["y"]),
                "{case}: the switch above Downloading's list: {measured}");
        }
        for case in ["side", "minutes"] {
            let side = &measured[case];
            assert!((middle(&side["switch"]) - middle(&side["tgHeading"])).abs() <= 0.5, "{case}: switch centred on the heading: {measured}");
            assert!(near(&side["dlList"]["y"], &side["tgList"]["y"]), "{case}: lists level: {measured}");
            assert!(at(&side["tgList"]["x"]) > at(&side["dlList"]["x"]) + at(&side["dlList"]["width"]), "{case}: side by side: {measured}");
            assert!(at(&side["switch"]["x"]) + at(&side["switch"]["width"]) < at(&side["tgList"]["x"]), "{case}: the switch in its own column: {measured}");
        }
        let stacked = &measured["stacked"];
        assert!(near(&stacked["dlList"]["x"], &stacked["tgList"]["x"]), "stacked, one edge: {measured}");
        assert!(at(&stacked["tgHeading"]["y"]) > at(&stacked["dlList"]["y"]), "stacked: {measured}");
        assert!(near(&stacked["tgGap"], &stacked["dlGap"]), "stacked, each line to its list as before: {measured}");
        // **Where the list has room under it, it opens downward; at the
        // window's foot it opens upward**, whole and inside the window.
        let room = &measured["roomBelow"];
        assert_eq!((&room["up"], &room["hidden"]), (&serde_json::json!(false), &serde_json::json!(false)), "{measured}");
        assert!(at(&room["top"]) >= at(&room["faceBottom"]), "downward with room: {measured}");
        let foot = &measured["atTheFoot"];
        assert_eq!((&foot["up"], &foot["hidden"]), (&serde_json::json!(true), &serde_json::json!(false)), "upward at the foot: {measured}");
        assert!(at(&foot["bottom"]) <= at(&foot["faceTop"]) && at(&foot["top"]) >= at(&foot["scrollerTop"]), "upward, whole: {measured}");
        assert!(at(&foot["faceBottom"]) <= at(&foot["window"]), "the control inside the window: {measured}");
    }

    /// The said of a queue entry with *Downloading* and the Telegram group,
    /// shared by the two twenty-fifth-notes pages.
    const THE_TELEGRAM_ENTRY: &str = r#"
globalThis.told = new Map(); globalThis.screens = new Map();
window.asked = [];
window.__TAURI__ = {core:{invoke(command,args) {
  if (command === 'an_act_on_a_job') asked.push(args);
  return Promise.resolve(command === 'the_pane_layout' ? {seams:[]} : null);
}}};
window.anEntry = (more) => Object.assign({
  theHeading:'Queue', lineCount:'1', line0Server:'s1', line0Name:'Server', rowCount:'2',
  row0Job:'j1', row0Line:'0', row0Name:'Kitchen', row1Job:'j2', row1Line:'0', row1Name:'Hallway',
  entryJob:'j1', entryName:'Kitchen',
  downloadingOn:'no', framesAsTheyRender:'Download frames as they render',
  downloadingEverything:'All formats',
  telegramHeading:'Telegram', telegramStands:'offered', telegramJob:'j1', telegramLabel:'What this job reports',
  telegramChoiceCount:'5',
  telegramChoice0Value:'everything', telegramChoice0Said:'Everything',
  telegramChoice1Value:'pictures-only', telegramChoice1Said:'Pictures only',
  telegramChoice2Value:'a-picture-every-few-minutes', telegramChoice2Said:'A picture every few minutes',
  telegramChoice3Value:'alerts-only', telegramChoice3Said:'Alerts only',
  telegramChoice4Value:'nothing', telegramChoice4Said:'Off',
  telegramPicked:'alerts-only', telegramReads:'Alerts only',
}, more || {});
"#;

    const THE_MINUTES_ACROSS_A_REDRAW: &str = r#"<!doctype html><html><head><meta charset="utf-8">
<link rel="stylesheet" href="smoozy.css">
<style>body { margin:0; } .fixture { width:1100px; height:700px; } #out { display:none; }</style>
</head><body><div class="fixture" id="fixture"></div><pre id="out"></pre>
<script src="entry.js"></script>
<script type="module">
import './the-queue.js';
try {
  const host = document.createElement('div'); document.getElementById('fixture').appendChild(host);
  screens.get('queueview').itIsDrawn(host);
  const minutes = {telegramPicked:'a-picture-every-few-minutes', telegramReads:'A picture every few minutes',
    telegramMinutes:'7', telegramMinutesWord:'minutes', telegramMinutesName:'Minutes between pictures'};
  const tell = (more) => { for (const handler of told.get('queue:the-queue') || []) handler({said: anEntry(Object.assign({}, minutes, more || {}))}); };
  const field = () => host.querySelector('.sftg input');
  const check = {};
  tell();
  const clicked = field(); clicked.focus();
  tell(); await Promise.resolve();
  check.aClickKeepsTheFocus = field() !== clicked && document.activeElement === field() && field().value === '7';
  const typed = field(); typed.value = '23'; typed.setSelectionRange(1, 1); typed.dispatchEvent(new Event('input', {bubbles:true}));
  tell(); await Promise.resolve();
  check.theTypingIsKept = field() !== typed && field().value === '23';
  check.theFocusIsKept = document.activeElement === field();
  check.theCaretIsKept = field().selectionStart === 1 && field().selectionEnd === 1;
  check.theRedrawSendsNothing = asked.length === 0;
  tell({telegramMinutes:'9'}); await Promise.resolve();
  check.theServerDoesNotOverwriteTheTyping = field().value === '23';
  const before = asked.length;
  field().dispatchEvent(new KeyboardEvent('keyup', {key:'Enter', bubbles:true}));
  check.enterSendsWhatWasTyped = JSON.stringify(asked.slice(before)) === JSON.stringify([{act:'what-it-reports', job:'j1', from:'minutes:23', group:null, after:null}]);
  tell({telegramMinutes:'45'}); await Promise.resolve();
  check.answeredItReadsTheServer = field().value === '45' && document.activeElement === field();
  field().blur();
  tell({telegramMinutes:'15'}); await Promise.resolve();
  check.untouchedItReadsTheServer = field().value === '15' && document.activeElement !== field();
  const other = field(); other.focus(); other.value = '30'; other.dispatchEvent(new Event('input', {bubbles:true}));
  tell({entryJob:'j2', entryName:'Hallway', telegramJob:'j2', telegramMinutes:'4'}); await Promise.resolve();
  check.anotherJobDoesNotTakeIt = field().value === '4';
  document.getElementById('out').textContent = JSON.stringify(check);
} catch(error) {document.getElementById('out').textContent=JSON.stringify({error:String(error.stack || error)});}
</script></body></html>"#;

    const THE_TWO_LISTS_MEASURED: &str = r#"<!doctype html><html><head><meta charset="utf-8">
<link rel="stylesheet" href="smoozy.css">
<style>body { margin:0; } .app { display:block; height:auto; } .fixture { width:1100px; height:700px; } #out { display:none; }</style>
</head><body><div class="app" id="app"><div class="fixture" id="fixture"></div></div><pre id="out"></pre>
<script src="entry.js"></script>
<script type="module">
import './the-queue.js';
try {
  const rect = (element) => {const r = element.getBoundingClientRect(); return {x:r.x, y:r.y, width:r.width, height:r.height};};
  const host = document.createElement('div'); host.style.cssText = 'height:100%; display:flex; flex-direction:column;';
  document.getElementById('fixture').appendChild(host);
  screens.get('queueview').itIsDrawn(host);
  const app = document.getElementById('app');
  const tell = (more) => { for (const handler of told.get('queue:the-queue') || []) handler({said: anEntry(more)}); };
  const read = () => {
    const side = host.querySelector('.qdside');
    const headings = Array.from(side.querySelectorAll('h3'));
    const tg = headings.find((one) => one.textContent === 'Telegram');
    const dlList = side.querySelector('.narrowb'), tgList = side.querySelector('.w-tg');
    return {headings:headings.map((one) => one.textContent), switchSays:side.querySelector('.adlt').textContent,
      tgHeading:rect(tg), switch:rect(side.querySelector('.adl')), dlList:rect(dlList), tgList:rect(tgList),
      dlGap: dlList.getBoundingClientRect().top - side.querySelector('.adl').getBoundingClientRect().bottom,
      tgGap: tgList.getBoundingClientRect().top - tg.getBoundingClientRect().bottom};
  };
  const result = {};
  app.dataset.narrow = 's1'; tell(); result.side = read();
  tell({telegramPicked:'a-picture-every-few-minutes', telegramReads:'A picture every few minutes', telegramMinutes:'7',
    telegramMinutesWord:'minutes', telegramMinutesName:'Minutes between pictures'});
  result.minutes = read();
  delete app.dataset.narrow; tell(); result.stacked = read();
  // **The list opens upward where the window has no room under it** (the
  // Creator's word on the second picture): first with room, then with the
  // pair at the window's foot.
  // An entry's own parts stand above the pair, as on a real job.
  app.dataset.narrow = 's1'; document.head.insertAdjacentHTML('beforeend', '<style>.qdmain { min-height: 230px; }</style>'); tell();
  const open = () => { host.querySelector('.w-tg .pkb').click(); const list = host.querySelector('.w-tg .ddl');
    const face = host.querySelector('.w-tg .pkb').getBoundingClientRect(), r = list.getBoundingClientRect();
    const seen = {up:list.classList.contains('up'), hidden:list.hidden, top:r.top, bottom:r.bottom, faceTop:face.top, faceBottom:face.bottom, window:innerHeight,
      scrollerTop:host.querySelector('.qdbody').getBoundingClientRect().top};
    host.querySelector('.w-tg .pkb').click(); return seen; };
  result.roomBelow = open();
  const foot = host.querySelector('.dltg').getBoundingClientRect().bottom;
  document.getElementById('fixture').style.marginTop = (innerHeight - foot - 12) + 'px'; tell();
  result.atTheFoot = open();
  document.getElementById('out').textContent = JSON.stringify(result);
} catch(error) {document.getElementById('out').textContent=JSON.stringify({error:String(error.stack || error)});}
</script></body></html>"#;

    /// **The Creator's eighteenth notes, notes 9 and 11, in a real browser,
    /// with the application's stylesheets and a window laid out as the
    /// application lays it out** (`#pane`, its `.body` holding the rail and the
    /// screen).
    ///
    /// - **Note 9: the large preview opens over the whole window**, in `.body`
    ///   as the drawing's `#pvlarge` sits, covering it edge to edge rather
    ///   than the right pane; a reading that arrives while it is open keeps the
    ///   one box there, and Escape still closes it and gives the preview back
    ///   the focus.
    /// - **Note 11: the View Layer list opens upward when a box around it cuts
    ///   it off**, though the window has room under it: a box that clips,
    ///   ending just under the dropdown, in the entry's column and then in the
    ///   large box; with room inside the box it opens downward.
    /// - **The stepping row reads as the drawing's of 2026-10-24**: arrow,
    ///   field and arrow touching, the shortcut after — which needs the
    ///   drawing's own hooks on the buttons, since the lifted rules order by
    ///   them.
    ///
    /// Skipped, and says so, where no Chrome is installed.
    #[test]
    fn eighteenth_notes_9_and_11_the_large_preview_and_the_view_layer_list_in_a_real_browser() {
        let chrome = [
            "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
            "/Applications/Chromium.app/Contents/MacOS/Chromium",
            "/usr/bin/google-chrome", "/usr/bin/chromium", "/usr/bin/chromium-browser",
            "C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe",
        ].into_iter().map(std::path::PathBuf::from).find(|path| path.exists());
        let Some(chrome) = chrome else {
            eprintln!("SKIPPED: no Chrome; the large preview and the list were not measured");
            return;
        };
        let folder = std::env::temp_dir().join(format!("smoozy-eighteenth-9-11-browser-{}", std::process::id()));
        std::fs::create_dir_all(&folder).unwrap();
        let web = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        for (name, text) in THE_BOXES_STAND_INS {
            if name != "harness.js" { std::fs::write(folder.join(name), text).unwrap(); }
        }
        for entry in std::fs::read_dir(&web).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().and_then(|ext| ext.to_str()) == Some("css") {
                std::fs::copy(&path, folder.join(path.file_name().unwrap())).unwrap();
            }
        }
        for name in ["the-queue.js", "the-value-drawn.js"] {
            std::fs::copy(web.join(name), folder.join(name)).unwrap();
        }
        let words = serde_json::json!({
            "frameHeading": FRAME_PREVIEW, "frameWord": "Frame",
            "previousFrame": PREVIOUS_FRAME, "nextFrame": NEXT_FRAME,
            "showLatest": SHOW_LATEST, "showingFrame": "Showing frame", "previewClose": "Close",
            "enlargePreview": ENLARGE_PREVIEW, "notColourAccurate": NOT_COLOUR_ACCURATE,
            "didNotRenderOnThisFrame": DID_NOT_RENDER_ON_THIS_FRAME, "viewLayer": VIEW_LAYER,
        });
        std::fs::write(folder.join("words.js"), format!("export default {words};")).unwrap();
        std::fs::write(folder.join("index.html"), THE_WINDOW_WITH_THE_LARGE_PREVIEW).unwrap();
        let measured = the_browser_measurement(&chrome, &folder);
        for (name, passed) in measured["checks"].as_object().expect("the checks") {
            assert_eq!(passed, &serde_json::json!(true), "{name}: {measured}");
        }
    }

    const THE_WINDOW_WITH_THE_LARGE_PREVIEW: &str = r#"<!doctype html><html><head><meta charset="utf-8">
<link rel="stylesheet" href="smoozy.css">
<style>html, body { margin:0; height:100%; overflow:hidden; } .app { height:100%; } #out { display:none; }</style>
</head><body><div class="app"><div class="titlebar"></div><div class="pane" id="pane"><div class="body"><nav class="rail"></nav><div class="main" id="main"></div></div></div></div><pre id="out"></pre>
<script>
globalThis.told = new Map(); globalThis.screens = new Map();
window.__TAURI__ = {core:{invoke(command) {
  return Promise.resolve(command === 'the_pane_layout' ? {seams:[]} : null);
}}};
</script>
<script type="module">
import words from './words.js';
import './the-queue.js';
try {
 const checks = {}; const seen = {};
 const host = document.createElement('div'); document.getElementById('main').appendChild(host);
 screens.get('queueview').itIsDrawn(host);
 const said = {...words, theHeading:'Queue', lineCount:'1', line0Server:'s1', line0Name:'Server', rowCount:'1',
   row0Job:'j1', row0Line:'0', row0Name:'Kitchen', entryJob:'j1', entryName:'Kitchen', previewPicture:'j1',
   previewFrame:'120', previewWhich:'latest', previewHeldCount:'1', previewHeldFrame0:'120',
   previewHeldLayerCount0:'3', previewHeldMain0:'0',
   previewHeld0Layer0:'Interior', previewHeld0Place0:'0', previewHeld0Layer1:'FX', previewHeld0Place1:'1',
   previewHeld0Layer2:'Reflections', previewHeld0Place2:'2'};
 const tell = () => { for (const handler of told.get('queue:the-queue') || []) handler({said}); };
 tell();
 const edges = (one) => { const r = one.getBoundingClientRect(); return {top:r.top, bottom:r.bottom, left:r.left, right:r.right, width:r.width, height:r.height}; };
 const body = document.querySelector('#pane > .body');
 const side = () => host.querySelector('.qdside');

 // **The stepping row, in the drawing's order.**
 const row = side().querySelector('.pvstep');
 const arrows = row.querySelectorAll('.sb');
 const back = edges(arrows[0]), field = edges(row.querySelector('.pvframe input'));
 const next = edges(arrows[1]), latest = edges(row.querySelector('.mini'));
 seen.row = {back, field, next, latest};
 checks.rowInOrder = back.right <= field.left + 0.5 && field.right <= next.left + 0.5 && next.right <= latest.left + 0.5;
 checks.arrowsTouchTheField = field.left - back.right <= 3 && next.left - field.right <= 3;

 // **Note 11 in the entry's column**: the column made a box that clips,
 // ending just under the dropdown, with the window's room under it.
 const opened = (scope, clip) => {
   const face = scope.querySelector('.pvvl .pkb');
   const list = scope.querySelector('.pvvl .ddl');
   if (!list.hidden) face.click();
   face.click();
   const answer = { up: list.classList.contains('up'), face: edges(face), list: edges(list), clip: edges(clip), window: innerHeight };
   face.click();
   return answer;
 };
 const clipped = (clip, roomInside) => {
   const face = clip.querySelector('.pvvl .pkb');
   clip.style.height = (edges(face).bottom - edges(clip).top + roomInside) + 'px';
   clip.style.overflow = 'auto'; clip.style.flex = 'none';
   return opened(clip, clip);
 };
 seen.columnFoot = clipped(side(), 12);
 checks.columnFootOpensUp = seen.columnFoot.up && seen.columnFoot.clip.bottom - seen.columnFoot.face.bottom < seen.columnFoot.list.height &&
   seen.columnFoot.face.bottom + 200 < innerHeight;
 seen.columnRoom = clipped(side(), 400);
 checks.columnRoomOpensDown = !seen.columnRoom.up;
 side().style.height = ''; side().style.overflow = ''; side().style.flex = '';

 // **Note 9: the large box over the whole window.**
 side().querySelector('.preview').click();
 const large = () => document.querySelector('.pvlarge');
 checks.largeInTheWindowsBody = !!large() && large().parentNode === body;
 seen.large = large() ? edges(large()) : null; seen.body = edges(body); seen.side = edges(side());
 checks.largeCoversTheWindow = !!seen.large && Math.abs(seen.large.left - seen.body.left) <= 1 &&
   Math.abs(seen.large.width - seen.body.width) <= 1 && Math.abs(seen.large.height - seen.body.height) <= 1;
 checks.largeIsWiderThanThePane = !!seen.large && seen.large.width > seen.side.width + 400;
 tell();
 checks.oneLargeAfterAReading = document.querySelectorAll('.pvlarge').length === 1 && large().parentNode === body;

 // **Note 11 in the large box**, held shorter than the window as the
 // drawing's cap of 1000 by 720 held it when the Creator met the fault: the
 // dropdown sits near the box's foot and the window has room under the box.
 const box = large().querySelector('.pvbox');
 box.style.maxHeight = '500px';
 seen.largeFoot = opened(large(), box);
 checks.largeFootOpensUp = seen.largeFoot.up && seen.largeFoot.clip.bottom + 100 < innerHeight;
 box.style.maxHeight = '';

 large().dispatchEvent(new KeyboardEvent('keydown', {key:'Escape', bubbles:true}));
 checks.escapeCloses = !large() && document.activeElement === side().querySelector('.preview');
 document.getElementById('out').textContent = JSON.stringify({checks, seen});
} catch (error) { document.getElementById('out').textContent = JSON.stringify({error:String(error.stack || error)}); }
</script></body></html>"#;

    const THE_NEW_PREVIEW_CONTROLS: &str = r#"<!doctype html><html><head><meta charset="utf-8">
<link rel="stylesheet" href="smoozy.css">
<style>body { margin:0; } .fixture { width:1100px; height:700px; } #out { display:none; }</style>
</head><body><div class="fixture" id="fixture"></div><pre id="out"></pre>
<script>
globalThis.told = new Map(); globalThis.screens = new Map();
window.asked = [];
window.__TAURI__ = {core:{invoke(command,args) {
  if (command === 'an_act_on_a_job') asked.push(args);
  return Promise.resolve(command === 'the_pane_layout' ? {seams:[]} : null);
}}};
</script>
<script type="module">

import words from './words.js';
import './the-queue.js'; import './the-history.js';
import {theWordsPanel, theNewestWordsStayInView} from './the-machines-own-words.js';
try {
 const result={}; const fixture=document.getElementById('fixture');
 for(const screen of ['queue','history']) {
  const host=document.createElement('div'); fixture.replaceChildren(host);
  screens.get(screen+'view').itIsDrawn(host);
  const said={...words,entryJob:screen+'-new',entryName:'Job',previewPicture:'check-job',previewFrame:'9',
   previewWhich:'latest',previewHeldCount:'3',previewHeldFrame0:'-1',previewHeldFrame1:'0',previewHeldFrame2:'9',
   lineCount:'0',rowCount:'0',hrowCount:'1',hrow0Job:screen+'-new',hrowChosen:screen+'-new'};
  for(let i=0;i<3;i++){said['previewHeldLayerCount'+i]='1';said['previewHeld'+i+'Layer0']='Beauty';}
  if(screen==='history')delete said.showLatest;
  const tell=()=>{for(const handler of told.get(screen+':the-'+screen)||[])handler({said});}; tell();
  const current=()=>host.querySelector('.qdside');
  const enter=value=>{const field=current().querySelector('.pvframe input');field.value=value;field.dispatchEvent(new Event('input',{bubbles:true}));field.dispatchEvent(new KeyboardEvent('keydown',{key:'Enter',bubbles:true}));};
  const shown=()=>current().querySelector('.pvshown').textContent;
  const check={typedField:!!current().querySelector('.pvframe input')};
  if(!check.typedField){current().querySelector('.preview').click();result[screen]={typedField:false,large:!!host.querySelector('.pvlarge'),shortcut:!!Array.from(current().querySelectorAll('button')).find(one=>one.textContent===(screen==='queue'?words.showLatest:words.showLast))};continue;}
  enter('bad');check.invalidFollows=screen!=='queue'||current().querySelector('.pvstep .mini').disabled;
  enter('999');check.missingFollows=screen!=='queue'||current().querySelector('.pvstep .mini').disabled;
  enter(' -1 '); check.signed=shown().includes('-1');
  enter('0');check.zero=shown().endsWith('0');
  enter('999');check.missing=shown().endsWith('0')&&current().querySelector('.pverror').textContent.includes('999');
  enter('1.5');check.invalid=shown().endsWith('0')&&current().querySelector('.pvframe input').getAttribute('aria-invalid')==='true';
  enter('');check.empty=shown().endsWith('0');
  enter('9');check.cleared=!current().querySelector('.pverror');
  const shortcut=()=>current().querySelector('.pvstep .mini');
  check.shortcut=shortcut().textContent===(screen==='queue'?words.showLatest:words.showLast)&&!shortcut().hidden;
  if(screen==='queue'){
   check.typedNewestStopsFollowing=!shortcut().disabled;
   said.previewHeldCount='4';said.previewHeldFrame3='10';said.previewHeldLayerCount3='1';said.previewHeld3Layer0='Beauty';tell();
   check.keptOnRefresh=shown().endsWith('9');shortcut().click();check.latest=shown().endsWith('10')&&shortcut().disabled;
  } else {
   enter('-1');shortcut().click();check.last=shown().endsWith('9')&&shortcut().disabled;
   said.previewHeldCount='4';said.previewHeldFrame3='10';said.previewHeldLayerCount3='1';said.previewHeld3Layer0='Beauty';tell();
   check.historyDoesNotFollow=shown().endsWith('9')&&!shortcut().disabled;
   shortcut().click();check.lastAfterRefresh=shown().endsWith('10');
  }
  const draft=current().querySelector('.pvframe input');draft.focus();draft.value='-999';draft.setSelectionRange(2,3);draft.dispatchEvent(new Event('input',{bubbles:true}));tell();await Promise.resolve();
  const restored=current().querySelector('.pvframe input');check.draft=restored.value==='-999'&&document.activeElement===restored&&restored.selectionStart===2;
  if(screen==='history'){
   const carry=host.querySelector('.qdmain input.fld');check.previewDoesNotChangeCarryOn=carry.value==='12';
   carry.focus();carry.value='17';carry.dispatchEvent(new Event('input',{bubbles:true}));tell();await Promise.resolve();
   const kept=host.querySelector('.qdmain input.fld');check.carryOnDraft=kept.value==='17'&&document.activeElement===kept&&current().querySelector('.pvframe input').value==='-999';
  }
  enter('0'); const picture=current().querySelector('.preview');picture.focus();picture.click();
  const large=()=>host.querySelector('.pvlarge');
  const focusable=()=>Array.from(large().querySelectorAll('button,input')).filter(one=>!one.disabled&&!one.closest('[hidden]'));
  const last=focusable().at(-1);last.focus();last.dispatchEvent(new KeyboardEvent('keydown',{key:'Tab',bubbles:true}));check.tabForward=document.activeElement===focusable()[0];
  focusable()[0].dispatchEvent(new KeyboardEvent('keydown',{key:'Tab',shiftKey:true,bubbles:true}));check.tabBack=document.activeElement===focusable().at(-1);
  const field=large().querySelector('input');field.focus();field.dispatchEvent(new KeyboardEvent('keydown',{key:'ArrowLeft',bubbles:true}));check.fieldKeys=shown().endsWith('0');
  check.fit=getComputedStyle(large().querySelector('.thumb')).objectFit==='contain'&&large().querySelector('.pvbox').scrollWidth<=large().querySelector('.pvbox').clientWidth;
  check.large=!!large()&&large().querySelector('.pvshown').textContent.endsWith('0')&&large().textContent.includes('Beauty')&&large().textContent.includes(words.notColourAccurate);
  if(screen==='queue'){
   large().querySelector('.pvstep .mini').click();said.previewHeldCount='5';said.previewHeldFrame4='11';said.previewHeldLayerCount4='1';said.previewHeld4Layer0='Beauty';tell();
   check.largeFollows=!!large()&&large().querySelector('.pvshown').textContent.endsWith('11');
   const typed=large().querySelector('input');typed.value='0';typed.dispatchEvent(new KeyboardEvent('keydown',{key:'Enter',bubbles:true}));check.largeTyped=shown().endsWith('0');
  }
  large().querySelectorAll('.sb')[0].click();check.largeSteps=shown().endsWith('-1');tell();
  check.largeRefresh=!!large()&&large().querySelector('.pvshown').textContent.endsWith('-1');
  large().dispatchEvent(new KeyboardEvent('keydown',{key:'ArrowRight',bubbles:true}));check.keys=shown().endsWith('0');
  large().dispatchEvent(new KeyboardEvent('keydown',{key:'Escape',bubbles:true}));check.escape=!large()&&document.activeElement===current().querySelector('.preview');
  current().querySelector('.preview').click();large().click();check.outside=!large();
  current().querySelector('.preview').click();said.previewHeldFrame1='2';tell();check.clearedCopy=!large()&&current().textContent.includes(words.previewUnavailable);
  result[screen]=check;
 }
 const panel=theWordsPanel({title:'Output',copy:'Copy',close:'Close',tabs:[{name:'Output',said:'Nothing printed'},{name:'Errors',record:'Failure'}]});
 panel.hidden=false;fixture.replaceChildren(panel);
 const copy=()=>panel.querySelector('.words-copy');const tabs=panel.querySelectorAll('.ltab');
 result.copy={empty:copy().hidden};tabs[1].click();result.copy.printed=!copy().hidden;tabs[0].click();result.copy.switched=copy().hidden;
 const body=panel.querySelector('.words-body');const pre=document.createElement('pre');pre.className='lbody';pre.textContent='First line';body.appendChild(pre);theNewestWordsStayInView(body);result.copy.arrived=!copy().hidden;
 document.getElementById('out').textContent=JSON.stringify(result);
} catch(error){document.getElementById('out').textContent=JSON.stringify({error:String(error.stack||error)});}
</script></body></html>"#;

    const THE_FRAME_CONTROLS_MEASURED: &str = r#"<!doctype html><html><head><meta charset="utf-8">
<link rel="stylesheet" href="smoozy.css">
<style>body { margin:0; } .fixture { width:1100px; height:700px; } #out { display:none; }</style>
</head><body><div class="fixture" id="fixture"></div><pre id="out"></pre>
<script>
globalThis.told = new Map(); globalThis.screens = new Map();
window.asked = [];
window.__TAURI__ = {core:{invoke(command,args) {
  if (command === 'an_act_on_a_job') asked.push(args);
  return Promise.resolve(command === 'the_pane_layout' ? {seams:[]} : null);
}}};
</script>
<script type="module">
import words from './words.js';
import './the-queue.js';
import './the-history.js';
try {
  const fixture = document.getElementById('fixture');
  const rect = (element) => {const r=element.getBoundingClientRect(); return {x:r.x,y:r.y,width:r.width,height:r.height};};
  const result = {};
  for (const screen of ['queue','history']) {
    const host = document.createElement('div'); fixture.replaceChildren(host);
    screens.get(screen+'view').itIsDrawn(host);
    const said = Object.assign({}, words, {
      theHeading: screen, lineCount:'1', line0Server:'s1', line0Name:'Server', rowCount:'1',
      row0Job:'waiting-job', row0Line:'0', row0Name:'Range',
      hrowCount:'1',hrow0Job:'waiting-job',hrow0Name:'Range',hrowChosen:'waiting-job',
      entryJob:'waiting-job',entryName:'Range',previewPicture:'first-job', previewFrame:'1001',
      previewHeldCount:'3',previewHeldFrame0:'-1',previewHeldFrame1:'9',previewHeldFrame2:'1001',
      heldCount:'1',held0FirstCount:'1',
    });
    for(let at=0;at<3;at++) {
      said['previewHeldLayerCount'+at]='1'; said['previewHeld'+at+'Layer0']='Layer';
    }
    if(screen === 'history') delete said.showLatest;
    for(const handler of told.get(screen+':the-'+screen) || []) handler({said});
    const read = () => {
      const stepper=host.querySelector('.pvstep');
      const arrows=Array.from(stepper.querySelectorAll('.sb'));
      const number=stepper.querySelector('.sn');
      const below=stepper.nextElementSibling;
      return {back:rect(arrows[0]),number:rect(number),forward:rect(arrows[1]),below:rect(below),
        arrows:arrows.map(button=>{const r=document.createRange();r.selectNodeContents(button);
          const text=r.getBoundingClientRect(),box=button.getBoundingClientRect();
          return {dx:(text.left+text.right-box.left-box.right)/2,dy:(text.top+text.bottom-box.top-box.bottom)/2};})};
    };
    const latest = () => {const button=host.querySelector('.pvstep .mini');return !!button && getComputedStyle(button).visibility !== 'hidden' && !button.hidden;};
    const steps=[read()], shown=[latest()];
    for(let at=0;at<2;at++) {host.querySelectorAll('.pvstep .sb')[0].click();steps.push(read());shown.push(latest());}
    if(screen==='queue') host.querySelector('.pvstep .mini').click();
    else {host.querySelectorAll('.pvstep .sb')[1].click();host.querySelectorAll('.pvstep .sb')[1].click();}
    steps.push(read());shown.push(latest());
    if(screen==='queue') for(const button of host.querySelectorAll('.jwheld button')) button.click();
    result[screen]={steps,latest:shown};
  }
  result.answers=asked;
  document.getElementById('out').textContent=JSON.stringify(result);
} catch(error) {document.getElementById('out').textContent=JSON.stringify({error:String(error.stack || error)});}
</script></body></html>"#;

    /// **The Creator's seventh notes, note 4, on the page: the preview steps
    /// through the frames held and shows each frame's View Layers.** The
    /// heading reads *Frame preview*; the arrows step with *Frame N* between
    /// them, and an arrow with no frame beyond it does nothing; a queue entry
    /// follows the newest frame until stepped away, then offers *Show
    /// latest*, which follows again, as does stepping onto the newest; the
    /// dropdown lists only the frame's own layers and is absent with one; a
    /// named single picture says its name; a frame lacking the pick shows its
    /// first layer and one line says so, and the pick comes back; History
    /// has the same arrows and no *Show latest*. `Design/Queue & History`,
    /// *How the stepping and the View Layers look*; `Queue & History` P2 and
    /// P10a. `web/the-queue.js` and `web/the-history.js` run under Node beside
    /// the boxes harness's stand-ins.
    ///
    /// **What it bites on**: the old heading (*Frame* and the number), the
    /// old *147 of 148* stepper that asked the application, a step undone by
    /// the next redraw, *Show latest* drawn on History or before a step, a
    /// dropdown listing another frame's layers or drawn for one, a lacking
    /// pick drawn as nothing or forgotten, and a picture asked without its
    /// layer's place. **Skipped, and says so, where no Node is installed.**
    #[test]
    fn seventh_notes_4_the_preview_steps_through_frames_and_view_layers() {
        let Some(lines) = a_page_run(
            "seventh-notes-4-page",
            &["the-queue.js", "the-history.js", "the-value-drawn.js"],
            &[],
            r#"
globalThis.invoked = [];
globalThis.window.__TAURI__.core.invoke = (command, args) => {
  globalThis.invoked.push([command, args]);
  return Promise.resolve(command === "the_pane_layout" ? { seams: [] } : null);
};
await import("./the-queue.js");
await import("./the-history.js");
const tell = (kind, said) => { for (const handler of globalThis.told.get(kind) || []) handler({ said }); };
const textOf = (element) => (element.text || "") + (element.children || []).map(textOf).join("");
const held = (frames) => {
  const said = { previewHeldCount: String(frames.length) };
  frames.forEach(([frame, names, main], at) => {
    said["previewHeldFrame" + at] = String(frame);
    said["previewHeldLayerCount" + at] = String(names.length);
    said["previewHeldMain" + at] = String(main || 0);
    names.forEach((name, one) => {
      if (name) said["previewHeld" + at + "Layer" + one] = name;
      said["previewHeld" + at + "Place" + one] = String(one);
    });
  });
  return said;
};
const words = {
  frameHeading: "Frame preview", frameWord: "Frame", previousFrame: "Previous frame", nextFrame: "Next frame",
  viewLayer: "View Layer", didNotRenderOnThisFrame: "did not render on this frame.", notColourAccurate: "Not colour-accurate.",
};
const L3 = ["Interior", "FX", "Reflections"], L2 = ["Interior", "FX"];
const anEntry = (job, frames, more) => Object.assign({
  theHeading: "Queue", lineCount: "1", line0Server: "s1", line0Name: "RunPod", rowCount: "1", row0Job: job, row0Line: "0",
  row0Name: "Kitchen", entryJob: job, entryName: "Kitchen", previewPicture: job,
  previewFrame: String(frames[frames.length - 1][0]), previewWhich: "latest", showLatest: "Show latest",
}, words, held(frames), more || {});
const look = (host) => {
  const group = host.querySelector("aside.qdside").children[0];
  const arrows = group.querySelectorAll("button.sb");
  const dd = group.querySelector("span.pick");
  const list = dd ? dd.querySelector("span.ddl") : null;
  const latest = group.querySelector("button.mini");
  const gone = group.querySelector("p[data-pvgone]");
  const alone = group.querySelector("p.sn");
  const asked = globalThis.invoked.filter(([command]) => command === "the_preview_picture").map(([, args]) => args);
  return {
    heading: textOf(group.children[0]),
    frame: words.frameWord + " " + group.querySelector("input").value,
    back: arrows.length ? !arrows[0].disabled : null,
    on: arrows.length ? !arrows[1].disabled : null,
    tag: !group.querySelector("span[data-pvtag]").hidden,
    latest: latest && !latest.disabled ? textOf(latest) : null,
    layers: list ? list.children.map(textOf) : null,
    shown: dd ? textOf(dd.querySelector("button.pkb")) : null,
    bold: list ? list.children.filter((one) => one.classList.contains("hd")).map(textOf) : null,
    alone: alone ? textOf(alone) : null,
    gone: gone ? textOf(gone) : null,
    picture: asked[asked.length - 1] || null,
  };
};
const queue = anElement("div");
globalThis.theRoot.appendChild(queue);
globalThis.screens.get("queueview").itIsDrawn(queue);
const arrow = (host, at) => host.querySelector("aside.qdside").querySelectorAll("button.sb")[at].click();
// Frame 119's main copy is its second. `Queue & History` P10
// requires the main copy when the chosen layer is absent.
const frames = [[118, L2], [119, L2, 1], [120, L3]];
tell("queue:the-queue", anEntry("j1", frames));
console.log(JSON.stringify(look(queue)));
queue.querySelector("button.pkb").click();
queue.querySelector("span.ddl").children[2].click();
console.log(JSON.stringify(look(queue)));
arrow(queue, 0);
console.log(JSON.stringify(look(queue)));
tell("queue:the-queue", anEntry("j1", frames.concat([[121, L3]])));
console.log(JSON.stringify(look(queue)));
queue.querySelector("aside.qdside").querySelector("button.mini").click();
console.log(JSON.stringify(look(queue)));
arrow(queue, 0);
arrow(queue, 1);
tell("queue:the-queue", anEntry("j1", frames.concat([[121, L3], [122, L3]])));
const followed = look(queue);
console.log(JSON.stringify({ frame: followed.frame, latest: followed.latest, tag: followed.tag, on: followed.on }));
arrow(queue, 1);
console.log(JSON.stringify({ frame: look(queue).frame }));
tell("queue:the-queue", anEntry("j2", [[1, [""]]]));
const one = look(queue);
console.log(JSON.stringify({ layers: one.layers, alone: one.alone, back: one.back, on: one.on }));
tell("queue:the-queue", anEntry("j3", [[5, ["Mist"]]]));
const named = look(queue);
console.log(JSON.stringify({ layers: named.layers, alone: named.alone }));

const history = anElement("div");
globalThis.theRoot.appendChild(history);
globalThis.screens.get("historyview").itIsDrawn(history);
const kept = Object.assign({
  theHeading: "History", hrowCount: "1", hrow0Job: "h1", hrow0Name: "Kitchen", hrowChosen: "h1",
  entryJob: "h1", entryName: "Kitchen", previewPicture: "h1", previewFrame: "240", previewWhich: "last complete", showLast: "Show last",
}, words, held([[239, L2], [240, L3]]));
tell("history:the-history", kept);
const before = look(history);
arrow(history, 0);
const stepped = look(history);
tell("history:the-history", kept);
const redrawn = look(history);
console.log(JSON.stringify({
  before: [before.heading, before.frame, before.tag, before.latest],
  stepped: [stepped.frame, stepped.tag, stepped.latest, stepped.back, stepped.on],
  redrawn: redrawn.frame,
}));
"#,
        ) else {
            return;
        };
        let picture = |job: &str, frame: i64, place: i64| serde_json::json!({ "job": job, "frame": frame, "place": place });
        assert_eq!(
            lines,
            vec![
                serde_json::json!({
                    "heading": "Frame preview", "frame": "Frame 120", "back": true, "on": false, "tag": true,
                    "latest": null, "layers": ["Interior", "FX", "Reflections"], "shown": "Interior",
                    "bold": ["Interior"], "alone": null, "gone": null, "picture": picture("j1", 120, 0),
                }),
                serde_json::json!({
                    "heading": "Frame preview", "frame": "Frame 120", "back": true, "on": false, "tag": true,
                    "latest": null, "layers": ["Interior", "FX", "Reflections"], "shown": "Reflections",
                    "bold": ["Reflections"], "alone": null, "gone": null, "picture": picture("j1", 120, 2),
                }),
                serde_json::json!({
                    "heading": "Frame preview", "frame": "Frame 119", "back": true, "on": true, "tag": false,
                    "latest": "Show latest", "layers": ["Interior", "FX"], "shown": "FX",
                    "bold": ["FX"], "alone": null, "gone": "Reflections did not render on this frame.",
                    "picture": picture("j1", 119, 1),
                }),
                serde_json::json!({
                    "heading": "Frame preview", "frame": "Frame 119", "back": true, "on": true, "tag": false,
                    "latest": "Show latest", "layers": ["Interior", "FX"], "shown": "FX",
                    "bold": ["FX"], "alone": null, "gone": "Reflections did not render on this frame.",
                    "picture": picture("j1", 119, 1),
                }),
                serde_json::json!({
                    "heading": "Frame preview", "frame": "Frame 121", "back": true, "on": false, "tag": true,
                    "latest": null, "layers": ["Interior", "FX", "Reflections"], "shown": "Reflections",
                    "bold": ["Reflections"], "alone": null, "gone": null, "picture": picture("j1", 121, 2),
                }),
                serde_json::json!({ "frame": "Frame 122", "latest": null, "tag": true, "on": false }),
                serde_json::json!({ "frame": "Frame 122" }),
                serde_json::json!({ "layers": null, "alone": null, "back": false, "on": false }),
                serde_json::json!({ "layers": null, "alone": "Mist" }),
                serde_json::json!({
                    "before": ["Frame preview", "Frame 240", true, null],
                    "stepped": ["Frame 239", false, "Show last", false, true],
                    "redrawn": "Frame 239",
                }),
            ]
        );
    }

    /// **The Creator's tenth notes, note 2, on the page: a main picture that
    /// is no one View Layer's is named by where it came from, listed first,
    /// and the dropdown is never blank.** The Creator's shape (the output
    /// path for every layer, a node per layer) lists *Output path* first, then
    /// the layers, opens on it and asks for it as the frame's main copy, with
    /// no place; a node is *File Output node* with its own name after it; one
    /// named copy beside the main shows both; a frame of one such picture says
    /// *Output path* as plain words; a frame of one layer is unchanged; a copy
    /// held before its source was kept shows the list's own label rather than
    /// a blank face; and stepping from *Output path* onto a frame of one layer
    /// says nothing did not render. `Design/Queue & History` line 201; the
    /// drawing's `q-pvsrc`. **Every word is the application's**: the harness
    /// hands them in, and the page script holds none.
    ///
    /// **What it bites on**: the blank dropdown over an unnamed main picture;
    /// one named copy beside the main drawing nothing; the source not listed
    /// first or not named; the main picture asked for by the first layer's
    /// place; *Output path did not render on this frame.* **Skipped, and says
    /// so, where no Node is installed.**
    #[test]
    fn tenth_notes_2_the_page_names_where_a_main_picture_came_from_and_never_draws_a_blank_dropdown() {
        let Some(lines) = a_page_run(
            "tenth-notes-2-page",
            &["the-queue.js", "the-history.js", "the-value-drawn.js"],
            &[],
            r#"
globalThis.invoked = [];
globalThis.window.__TAURI__.core.invoke = (command, args) => {
  globalThis.invoked.push([command, args]);
  return Promise.resolve(command === "the_pane_layout" ? { seams: [] } : null);
};
await import("./the-queue.js");
await import("./the-history.js");
const tell = (kind, said) => { for (const handler of globalThis.told.get(kind) || []) handler({ said }); };
const textOf = (element) => (element.text || "") + (element.children || []).map(textOf).join("");
// Each copy is [layer value, source words, place]; the source is said first.
const held = (frames) => {
  const said = { previewHeldCount: String(frames.length) };
  frames.forEach(([frame, copies, main], at) => {
    said["previewHeldFrame" + at] = String(frame);
    said["previewHeldLayerCount" + at] = String(copies.length);
    said["previewHeldMain" + at] = String(main || 0);
    copies.forEach(([value, source, place], one) => {
      if (value) said["previewHeld" + at + "Layer" + one] = value;
      if (source) said["previewHeld" + at + "Source" + one] = source;
      if (place !== undefined) said["previewHeld" + at + "Place" + one] = String(place);
    });
  });
  return said;
};
const words = {
  frameHeading: "Frame preview", frameWord: "Frame", previousFrame: "Previous frame", nextFrame: "Next frame",
  viewLayer: "View Layer", didNotRenderOnThisFrame: "did not render on this frame.", notColourAccurate: "Not colour-accurate.",
};
const anEntry = (job, frames) => Object.assign({
  theHeading: "Queue", lineCount: "1", line0Server: "s1", line0Name: "RunPod", rowCount: "1", row0Job: job, row0Line: "0",
  row0Name: "Kitchen", entryJob: job, entryName: "Kitchen", previewPicture: job,
  previewFrame: String(frames[frames.length - 1][0]), previewWhich: "latest", showLatest: "Show latest",
}, words, held(frames));
const look = (host) => {
  const group = host.querySelector("aside.qdside").children[0];
  const dd = group.querySelector("span.pick");
  const list = dd ? dd.querySelector("span.ddl") : null;
  const gone = group.querySelector("p[data-pvgone]");
  const alone = group.querySelector("p.sn");
  const asked = globalThis.invoked.filter(([command]) => command === "the_preview_picture").map(([, args]) => args);
  return {
    frame: words.frameWord + " " + group.querySelector("input").value,
    layers: list ? list.children.map(textOf) : null,
    shown: dd ? textOf(dd.querySelector("button.pkb")) : null,
    alone: alone ? textOf(alone) : null,
    gone: gone ? textOf(gone) : null,
    picture: asked[asked.length - 1] || null,
  };
};
const queue = anElement("div");
globalThis.theRoot.appendChild(queue);
globalThis.screens.get("queueview").itIsDrawn(queue);
const PATH = ["", "Output path"];
// The Creator's shape: the output path's file is the main copy, then each
// layer's own, the first layer's at place nought too.
tell("queue:the-queue", anEntry("j1", [[146, [["Interior", "", 0]]], [147, [PATH, ["Interior", "", 0], ["FX", "", 1], ["Reflections", "", 2]]]]));
console.log(JSON.stringify(look(queue)));
queue.querySelector("button.pkb").click();
queue.querySelector("span.ddl").children[1].click();
console.log(JSON.stringify(look(queue)));
queue.querySelector("button.pkb").click();
queue.querySelector("span.ddl").children[0].click();
queue.querySelector("aside.qdside").querySelectorAll("button.sb")[0].click();
console.log(JSON.stringify(look(queue)));
tell("queue:the-queue", anEntry("j2", [[5, [["Beauty", "File Output node"], ["Box", "", 0]]]]));
console.log(JSON.stringify(look(queue)));
tell("queue:the-queue", anEntry("j3", [[7, [PATH, ["Mattress", "", 1]]]]));
console.log(JSON.stringify(look(queue)));
tell("queue:the-queue", anEntry("j4", [[8, [PATH]]]));
console.log(JSON.stringify(look(queue)));
tell("queue:the-queue", anEntry("j5", [[9, [["", "", 0]]]]));
console.log(JSON.stringify(look(queue)));
tell("queue:the-queue", anEntry("j6", [[10, [["", "", 0], ["Box", "", 0], ["Mattress", "", 1]]]]));
console.log(JSON.stringify(look(queue)));
"#,
        ) else {
            return;
        };
        let main = |job: &str, frame: i64| serde_json::json!({ "job": job, "frame": frame });
        let at = |job: &str, frame: i64, place: i64| serde_json::json!({ "job": job, "frame": frame, "place": place });
        assert_eq!(
            lines,
            vec![
                serde_json::json!({
                    "frame": "Frame 147", "layers": ["Output path", "Interior", "FX", "Reflections"],
                    "shown": "Output path", "alone": null, "gone": null, "picture": main("j1", 147),
                }),
                serde_json::json!({
                    "frame": "Frame 147", "layers": ["Output path", "Interior", "FX", "Reflections"],
                    "shown": "Interior", "alone": null, "gone": null, "picture": at("j1", 147, 0),
                }),
                serde_json::json!({
                    "frame": "Frame 146", "layers": null, "shown": null, "alone": "Interior", "gone": null,
                    "picture": at("j1", 146, 0),
                }),
                serde_json::json!({
                    "frame": "Frame 5", "layers": ["File Output node Beauty", "Box"],
                    "shown": "File Output node Beauty", "alone": null, "gone": null, "picture": main("j2", 5),
                }),
                serde_json::json!({
                    "frame": "Frame 7", "layers": ["Output path", "Mattress"],
                    "shown": "Output path", "alone": null, "gone": null, "picture": main("j3", 7),
                }),
                serde_json::json!({
                    "frame": "Frame 8", "layers": null, "shown": null, "alone": "Output path", "gone": null,
                    "picture": main("j4", 8),
                }),
                serde_json::json!({
                    "frame": "Frame 9", "layers": null, "shown": null, "alone": null, "gone": null,
                    "picture": at("j5", 9, 0),
                }),
                serde_json::json!({
                    "frame": "Frame 10", "layers": ["Box", "Mattress"], "shown": "View Layer", "alone": null,
                    "gone": null, "picture": at("j6", 10, 0),
                }),
            ]
        );
    }

    /// **The Creator's notes on the test copy, piece 8, on the page: the entry
    /// header is two rows, a rendering entry draws the three lines it is sent
    /// and nothing it is not, and *Show in Queue*'s job is the one chosen.**
    /// `web/the-queue.js` and `web/the-history.js` run under Node beside the
    /// boxes harness's stand-ins.
    ///
    /// **What it bites on**: the spacer back between the name and the acts,
    /// the acts back in the name's row, *Stop* or *Delete* not last, the lines
    /// not drawn or drawn where none were sent, and the job handed over
    /// dropped. **Skipped, and says so, where no Node is installed.**
    #[test]
    fn piece_8_the_header_is_two_rows_the_lines_are_drawn_and_the_job_handed_over_is_chosen() {
        let Some(lines) = a_page_run(
            "piece-8-page",
            &["the-queue.js", "the-history.js", "the-value-drawn.js"],
            &[],
            r#"
globalThis.invoked = [];
globalThis.window.__TAURI__.core.invoke = (command, args) => {
  globalThis.invoked.push([command, args]);
  return Promise.resolve(command === "the_pane_layout" ? { seams: [] } : null);
};
await import("./the-queue.js");
await import("./the-history.js");
const tell = (kind, said) => { for (const handler of globalThis.told.get(kind) || []) handler({ said }); };
const textOf = (element) => (element.text || "") + (element.children || []).map(textOf).join("");
const shape = (element) => [element.tag].concat(element.className ? element.className.split(" ") : []).join(".");
const queue = anElement("div");
globalThis.theRoot.appendChild(queue);
globalThis.screens.get("queueview").itIsDrawn(queue);
const anEntry = {
  theHeading: "Queue", lineCount: "1", line0Server: "s1", line0Name: "RunPod 4090",
  rowCount: "1", row0Job: "j1", row0Line: "0", row0Name: "Kitchen",
  entryJob: "j1", entryName: "Kitchen", entryServer: "RunPod 4090", entryBadge: "b1", entryBlend: "kitchen.blend",
  entryScene: "Interior", entryReachBacks: "yes", downloadAll: "Download all", downloadAllTo: "Download all to…",
  filesOnTheServer: "Files on the server", filesOnYourComputer: "Files on your computer",
  entryStopOrCancel: "Stop", entryStopOrCancelAct: "stop-the-job", blenderOutput: "Blender output",
};
tell("queue:the-queue", Object.assign({}, anEntry, {
  entryLineCount: "3", entryLine0: "Fra:1 Sample 64/128", entryLine1: "Fra:1 Sample 128/128", entryLine2: "Fra:2 Sample 1/128",
}));
const header = queue.querySelector("div.qdh");
const drawn = queue.querySelector("span.qlines");
console.log(JSON.stringify({
  header: header.children.map(shape),
  acts: header.children[1].children.map(textOf),
  // The Creator's ninth notes, note 1: the queue entry is headed by its
  // name alone, so its line, and the badge in it, is no longer drawn.
  badge: header.querySelector("span.badge") ? header.querySelector("span.badge").className : null,
  lines: drawn ? [shape(drawn)].concat(drawn.children.map(textOf)) : null,
  beside: queue.querySelector("div.log").children.map(shape),
}));
tell("queue:the-queue", anEntry);
console.log(JSON.stringify({ lines: queue.querySelector("span.qlines") ? "drawn" : null, beside: queue.querySelector("div.log").children.map(shape) }));
globalThis.invoked.length = 0;
globalThis.screens.get("queueview").itIsCurrentNow("j-new");
console.log(JSON.stringify({ chosen: globalThis.invoked.filter(([command]) => command === "the_queue").map(([, args]) => args.chosen) }));
globalThis.screens.get("queueview").itIsCurrentNow();
console.log(JSON.stringify({ kept: globalThis.invoked.filter(([command]) => command === "the_queue").map(([, args]) => args.chosen) }));
const history = anElement("div");
globalThis.theRoot.appendChild(history);
globalThis.screens.get("historyview").itIsDrawn(history);
tell("history:the-history", {
  theHeading: "History", hrowCount: "1", hrow0Job: "j1", hrow0Name: "Kitchen", hrowChosen: "j1",
  entryJob: "j1", entryName: "Kitchen", entryReachBacks: "yes", downloadAll: "Download all",
  downloadAllTo: "Download all to…", filesOnTheServer: "Files on the server",
  filesOnYourComputer: "Files on your computer", delete: "Delete",
});
const kept = history.querySelector("div.qdh");
console.log(JSON.stringify({ header: kept.children.map(shape), acts: kept.children[1].children.map(textOf), lines: history.querySelector("span.qlines") ? "drawn" : null }));
"#,
        ) else {
            return;
        };
        assert_eq!(
            lines,
            vec![
                serde_json::json!({
                    "header": ["div", "div.qdacts"],
                    "acts": ["Download all", "Download all to…", "Files on the serverFiles on your computer", "Stop"],
                    "badge": null,
                    "lines": ["span.qlines.selectable.mono", "Fra:1 Sample 64/128", "Fra:1 Sample 128/128", "Fra:2 Sample 1/128"],
                    "beside": ["span.qlines.selectable.mono", "button"],
                }),
                serde_json::json!({ "lines": null, "beside": ["button"] }),
                serde_json::json!({ "chosen": ["j-new"] }),
                serde_json::json!({ "kept": ["j-new", "j-new"] }),
                serde_json::json!({
                    "header": ["div", "div.qdacts"],
                    "acts": ["Download all", "Download all to…", "Files on the serverFiles on your computer", "Delete"],
                    "lines": null,
                }),
            ]
        );
    }

    /// **W4, the Creator's twelfth notes, note 7, on the page: the row draws
    /// each bar it has, and the entry draws an empty top bar where it is sent
    /// a nought.** `web/the-queue.js` runs under Node beside the boxes
    /// harness's stand-ins. A row sent both bars draws two; a row sent only
    /// the frame's, or only the whole job's, draws that one; a row sent
    /// neither draws none. The entry sent `progNowAt` 0 draws its top bar.
    ///
    /// **What it bites on**: the row drawing no bar unless the frame's bar
    /// was sent, and drawing the whole job's bar from nothing (the state
    /// before W4). **Skipped, and says so, where no Node is installed.**
    #[test]
    fn w4_the_row_draws_each_bar_it_has_and_an_empty_top_bar_is_drawn() {
        let Some(lines) = a_page_run(
            "w4-bars-page",
            &["the-queue.js", "the-value-drawn.js"],
            &[],
            r#"
globalThis.window.__TAURI__.core.invoke = (command) =>
  Promise.resolve(command === "the_pane_layout" ? { seams: [] } : null);
await import("./the-queue.js");
const tell = (kind, said) => { for (const handler of globalThis.told.get(kind) || []) handler({ said }); };
const queue = anElement("div");
globalThis.theRoot.appendChild(queue);
globalThis.screens.get("queueview").itIsDrawn(queue);
const aRow = (bars) => Object.assign({
  theHeading: "Queue", lineCount: "1", line0Server: "s1", line0Name: "RunPod 4090",
  rowCount: "1", row0Job: "j1", row0Line: "0", row0Name: "Kitchen",
  entryJob: "j1", entryName: "Kitchen", renderingNow: "Rendering now", wholeJob: "Whole job",
  progNowCount: "1", progNow0Lead: "Frame", progNow0Value: "1008",
}, bars);
const theBars = () => queue.querySelector("span.qbars").children.map((one) => one.className);
const theTopBar = () => queue.querySelectorAll("div.bar").filter((one) => !one.classList.contains("whole")).length;
for (const bars of [
  { row0Renderlet: "0", row0Whole: "60", progNowAt: "0", progWhole: "60" },
  { row0Renderlet: "0", progNowAt: "0" },
  { row0Whole: "60", progWhole: "60" },
  {},
]) {
  tell("queue:the-queue", aRow(bars));
  console.log(JSON.stringify({ row: theBars(), top: theTopBar() }));
}
"#,
        ) else {
            return;
        };
        assert_eq!(
            lines,
            vec![
                serde_json::json!({ "row": ["qbar", "qbar w"], "top": 1 }),
                serde_json::json!({ "row": ["qbar"], "top": 1 }),
                serde_json::json!({ "row": ["qbar w"], "top": 0 }),
                serde_json::json!({ "row": [], "top": 0 }),
            ],
            "the row does not draw each bar it has"
        );
    }

    /// **The 38's piece 8, on the page: a waiting row dragged names the job
    /// it now follows, or none for the front, and never an index; a group's
    /// heading moves the group; a row's own menu moves it to the top; a drag
    /// that lands off its server's queue, or where it started, sends nothing;
    /// a drawing that arrives while a row is held waits until it is let go;
    /// and the teaching line is drawn.** `web/the-queue.js` runs under Node
    /// beside the boxes harness's stand-ins, with rows twenty pixels tall.
    ///
    /// **What it bites on**: an index sent, the front sent as a job, a row
    /// that cannot move offering the drag or the menu, a drop off the list
    /// sending a move, the group's own members counted as places to land, a
    /// drawing pulling the row from under the pointer, and the foot line not
    /// drawn. **Skipped, and says so, where no Node is installed.**
    #[test]
    fn piece_8_a_dragged_row_names_the_job_it_follows_and_the_menu_moves_it_to_the_top() {
        let Some(lines) = a_page_run(
            "piece-8-moves",
            &["the-queue.js", "the-history.js", "the-value-drawn.js"],
            &[],
            r#"
const plain = globalThis.document.createElement;
globalThis.document.createElement = (tag) => {
  const element = plain(tag);
  const held = new Map();
  const add = element.addEventListener;
  element.addEventListener = (kind, handler) => { add(kind, handler); held.set(kind, (held.get(kind) || []).concat([handler])); };
  element.removeEventListener = (kind, handler) => held.set(kind, (held.get(kind) || []).filter((one) => one !== handler));
  element.fire = (kind, event) => (held.get(kind) || []).slice().forEach((handler) => handler(event));
  element.setPointerCapture = () => {};
  return element;
};
globalThis.invoked = [];
globalThis.window.__TAURI__.core.invoke = (command, args) => {
  globalThis.invoked.push([command, args]);
  return Promise.resolve(command === "the_pane_layout" ? { seams: [] } : null);
};
await import("./the-queue.js");
await import("./the-history.js");
const tell = (kind, said) => { for (const handler of globalThis.told.get(kind) || []) handler({ said }); };
const textOf = (element) => (element.text || "") + (element.children || []).map(textOf).join("");
const queue = anElement("div");
globalThis.theRoot.appendChild(queue);
globalThis.screens.get("queueview").itIsDrawn(queue);
const teaching = {
  whatDraggingDoes: "Drag a row to reorder its server's queue, or a group's heading to move the group.",
  andTheRowsOwnMenu: "Right-click a row for", moveToTheTop: "Move to the top of the queue",
  dragItUpTheQueue: "Drag to reorder", dragTheWholeGroup: "Drag to move the group",
};
const four = Object.assign({
  theHeading: "Queue", lineCount: "1", line0Server: "s1", line0Name: "RunPod 4090", rowCount: "4",
  row0Job: "j1", row0Line: "0", row0Name: "Rendering",
  row1Job: "j2", row1Line: "0", row1Name: "Two", row1Grip: "yes",
  row2Job: "j3", row2Line: "0", row2Name: "Three", row2Grip: "yes",
  row3Job: "j4", row3Line: "0", row3Name: "Four", row3Grip: "yes",
  entryJob: "j1",
}, teaching);
const laidOut = () => {
  const section = queue.querySelector("section.qmgroup");
  section.getBoundingClientRect = () => ({ top: 0, bottom: 200, left: 0, right: 300, height: 200 });
  queue.querySelectorAll("button.qrow").forEach((row, at) => {
    row.getBoundingClientRect = () => ({ top: 20 * at, bottom: 20 * at + 20, height: 20, left: 0, right: 300 });
  });
};
const press = (grip) => ({ button: 0, pointerId: 1, currentTarget: grip, preventDefault() {}, stopPropagation() {} });
const drag = (grip, y) => {
  grip.fire("pointerdown", press(grip));
  grip.fire("pointermove", { clientX: 10, clientY: y });
  grip.fire("pointerup", {});
};
const moves = () => globalThis.invoked
  .filter(([command, args]) => command === "an_act_on_a_job" && args.act.startsWith("move"))
  .map(([, args]) => [args.act, args.job, args.group, args.after]);
const gripOf = (job) => queue.querySelectorAll("button.qrow").find((row) => row.dataset.job === job).children[0];

tell("queue:the-queue", four);
laidOut();
const foot = queue.querySelector("div.qfoot");
console.log(JSON.stringify({
  foot: foot ? textOf(foot) : null,
  grips: queue.querySelectorAll("button.qrow").map((row) => row.children[0].className),
  titles: queue.querySelectorAll("button.qrow").map((row) => row.children[0].title),
}));

drag(gripOf("j4"), 5);
drag(gripOf("j4"), 35);
drag(gripOf("j2"), 75);
drag(gripOf("j3"), 50);
drag(gripOf("j2"), 500);
console.log(JSON.stringify({ moves: moves() }));

// **A drawing that arrives while a row is held waits.**
globalThis.invoked.length = 0;
const held = gripOf("j3");
held.fire("pointerdown", press(held));
tell("queue:the-queue", Object.assign({}, four, { row1Name: "Two, drawn again" }));
const during = textOf(queue.querySelectorAll("button.qrow")[1].children[1]);
held.fire("pointerup", {});
const after = textOf(queue.querySelectorAll("button.qrow")[1].children[1]);
console.log(JSON.stringify({ during, after, moves: moves() }));

// **The row's own menu**, on a waiting row and on no other.
tell("queue:the-queue", four);
const rows = queue.querySelectorAll("button.qrow");
rows[0].fire("contextmenu", { preventDefault() {} });
const none = queue.querySelector("div.ctx");
rows[2].fire("contextmenu", { preventDefault() {} });
const menu = queue.querySelector("div.ctx");
const item = menu.children[0];
item.fire("click", { stopPropagation() {} });
console.log(JSON.stringify({ onTheRendering: none ? "a menu" : null, item: textOf(item), moves: moves(), shut: queue.querySelector("div.ctx") ? "open" : "shut" }));

// **A group's heading moves the group, and its own members are no place to land.**
globalThis.invoked.length = 0;
tell("queue:the-queue", Object.assign({}, four, {
  groupCount: "1", group0Group: "g1", group0Name: "Spins", group0Scenes: "2", group0Grip: "yes",
  row1Group: "0", row2Group: "0",
}));
laidOut();
const heading = queue.querySelector("div.qgh").children[0];
drag(heading, 5);
drag(heading, 35);
drag(heading, 75);
console.log(JSON.stringify({ title: heading.title, moves: moves() }));

// **Nothing waits, so nothing is taught.**
tell("queue:the-queue", { theHeading: "Queue", lineCount: "1", line0Server: "s1", rowCount: "1", row0Job: "j1", row0Line: "0" });
console.log(JSON.stringify({ foot: queue.querySelector("div.qfoot") ? "drawn" : null }));
"#,
        ) else {
            return;
        };
        assert_eq!(
            lines,
            vec![
                serde_json::json!({
                    "foot": "Drag a row to reorder its server's queue, or a group's heading to move the group. Right-click a row for Move to the top of the queue.",
                    "grips": ["qgrip nodrag", "qgrip", "qgrip", "qgrip"],
                    "titles": ["", "Drag to reorder", "Drag to reorder", "Drag to reorder"],
                }),
                serde_json::json!({ "moves": [
                    ["move-the-job", "j4", null, null],
                    ["move-the-job", "j4", null, "j2"],
                    ["move-the-job", "j2", null, "j4"],
                ] }),
                serde_json::json!({ "during": "Two", "after": "Two, drawn again", "moves": [] }),
                serde_json::json!({
                    "onTheRendering": null,
                    "item": "Move to the top of the queue",
                    "moves": [["move-the-job", "j3", null, null]],
                    "shut": "shut",
                }),
                serde_json::json!({ "title": "Drag to move the group", "moves": [
                    ["move-the-group", null, "g1", null],
                    ["move-the-group", null, "g1", "j4"],
                ] }),
                serde_json::json!({ "foot": null }),
            ]
        );
    }

    /// **`Design/Whole app`, the raw-text panel, as the Creator's notes on the
    /// test copy, piece 8 (note 12) ruled it: the panel opens at the end, new
    /// lines keep it there, a scroll up of more than a line lets go, and back
    /// at the end it holds again** — the live panel and a panel opened the way
    /// History opens one. The real `web/the-machines-own-words.js` runs under
    /// Node with a document that scrolls: twenty pixels a line, a hundred in
    /// view, and a resize observer the harness fires as a layout would.
    ///
    /// **What it bites on**: a panel opening at its top, new words not
    /// followed, a small scroll letting go, a large one not, a return to the
    /// end not holding, and a tab chosen not opening at its end. **Skipped, and
    /// says so, where no Node is installed.**
    #[test]
    fn piece_8_the_words_panel_opens_at_the_end_and_holds_it_until_a_scroll_up() {
        let Some(lines) = a_page_run(
            "piece-8-panel",
            &["the-queue.js", "the-value-drawn.js"],
            &["the-machines-own-words.js"],
            r#"
const textOf = (element) => (element.text || "") + (element.children || []).map(textOf).join("");
const shownIs = (element) => { for (let at = element; at; at = at.parentNode) if (at.hidden) return false; return element.isConnected; };
const theirListeners = new Map();
const made = globalThis.document.createElement;
globalThis.document.createElement = (tag) => {
  const element = made(tag);
  let top = 0;
  const listen = element.addEventListener;
  element.addEventListener = (kind, handler) => {
    listen(kind, handler);
    const held = theirListeners.get(element) || [];
    held.push([kind, handler]);
    theirListeners.set(element, held);
  };
  Object.defineProperty(element, "scrollHeight", { get: () => (textOf(element).match(/\n/g) || []).length * 20 });
  Object.defineProperty(element, "clientHeight", { get: () => (shownIs(element) ? 100 : 0) });
  Object.defineProperty(element, "scrollTop", {
    get: () => top,
    set: (value) => { top = Math.max(0, Math.min(value, element.scrollHeight - element.clientHeight)); },
  });
  return element;
};
const observers = [];
globalThis.ResizeObserver = class { constructor(callback) { this.callback = callback; observers.push(this); } observe() {} };
const aLayout = () => { for (const one of observers) one.callback([]); };
const theUserScrolls = (element, by) => {
  element.scrollTop = element.scrollTop + by;
  for (const [kind, handler] of theirListeners.get(element) || []) if (kind === "scroll") handler({});
};
globalThis.CSS = { escape: (text) => text };
const { theWordsPanel } = await import("./the-machines-own-words.js");
await import("./the-queue.js");
const tell = (kind, said) => { for (const handler of globalThis.told.get(kind) || []) handler({ said }); };
const lines = (from, many) => Array.from({ length: many }, (_, at) => "Fra:" + (from + at) + " Sample 1/128\n").join("");
const queue = anElement("div");
globalThis.theRoot.appendChild(queue);
globalThis.screens.get("queueview").itIsDrawn(queue);
const words = (more, fresh) => ({
  job: "j1", title: "Blender output", close: "Close", theOutputTab: "Output", theOutputSays: "the-record",
  theOutputMore: more, theErrorsTab: "Errors", theErrorsSays: "the-record", theErrorsMore: fresh ? "Error: one\n" : "",
  fresh: fresh ? "yes" : "",
});
const at = (body, said) => console.log(JSON.stringify({ said, top: body.scrollTop, end: body.scrollHeight - body.clientHeight }));

tell("queue:the-running-words", words(lines(1, 30), true));
const panel = queue.children.find((child) => child.classList && child.classList.contains("words-panel"));
const [output, errors] = panel.querySelectorAll("div.words-body");
aLayout();
at(output, "opened");
tell("queue:the-running-words", words(lines(31, 5), false));
at(output, "five more lines");
theUserScrolls(output, -10);
tell("queue:the-running-words", words(lines(36, 1), false));
at(output, "up less than a line, then a line");
theUserScrolls(output, -60);
const left = output.scrollTop;
tell("queue:the-running-words", words(lines(37, 1), false));
at(output, "up three lines, then a line");
console.log(JSON.stringify({ said: "left where it was", same: output.scrollTop === left }));
theUserScrolls(output, 1000);
tell("queue:the-running-words", words(lines(38, 1), false));
at(output, "back at the end, then a line");
theUserScrolls(output, -200);
const [outputTab, errorsTab] = panel.querySelectorAll("button.ltab");
errorsTab.click();
aLayout();
outputTab.click();
aLayout();
at(output, "its tab chosen again");

// **History's way**: a panel made, opened and put on the screen by the caller.
const kept = theWordsPanel({ title: "Blender output", tabs: [{ name: "Output", says: "the-record", record: lines(1, 40) }, { name: "Errors", says: "the-record", record: "" }], close: "Close" });
const holder = anElement("div");
globalThis.theRoot.appendChild(holder);
holder.appendChild(kept);
aLayout();
const [keptOutput] = kept.querySelectorAll("div.words-body");
at(keptOutput, "made, not opened");
kept.hidden = false;
aLayout();
at(keptOutput, "opened");
"#,
        ) else {
            return;
        };
        let read = |said: &str, top: u64, end: u64| serde_json::json!({ "said": said, "top": top, "end": end });
        assert_eq!(
            lines,
            vec![
                read("opened", 500, 500),
                read("five more lines", 600, 600),
                read("up less than a line, then a line", 620, 620),
                read("up three lines, then a line", 560, 640),
                serde_json::json!({ "said": "left where it was", "same": true }),
                read("back at the end, then a line", 660, 660),
                read("its tab chosen again", 660, 660),
                read("made, not opened", 0, 800),
                read("opened", 700, 700),
            ]
        );
    }

    /// **The Creator's ninth notes, note 3: the heading counts the endings by
    /// kind** — `Design/Queue & History § The catch-up notice`: *· 3 finished
    /// · 1 failed*, only endings above nought, in the order *finished*,
    /// *failed*, *stopped*, *given up*, each word with the class of its
    /// History colour, and the hover *Since you last opened History.* Counted
    /// across every standing record (`Queue & History` C2a), one per job whose
    /// History entry stands; a job deleted from History since is counted
    /// nowhere, a re-run is not an ending, and a dropped job is a line rather
    /// than a count. **Opening History clears the count and keeps the dropped
    /// job's line** (`Queue & History` C2a as changed 2026-10-12).
    ///
    /// **What it bites on**: a kind at nought drawn, the kinds out of
    /// Design's order or merged, a word in another kind's colour, a re-run,
    /// a dropped job or a deleted entry counted, the count left standing after
    /// History is opened, the dropped line taken with it, and History's own
    /// command no longer clearing it.
    #[test]
    fn ninth_notes_3_the_heading_counts_by_ending_and_opening_history_clears_it() {
        use smoozy_queue_history::catch_up::{ANoticeEntry, NoticeRecord};
        use smoozy_queue_history::ending::Ending;
        use smoozy_state::identities::{Identity, IdentityKind};
        let path = std::env::temp_dir().join(format!("smoozy-queue-ninth-notes-3-count-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).unwrap();
        let record = crate::record::LocalRecord::open_at(&path.join("local-record.sqlite3")).unwrap();
        let (one, two) = (
            Identity::issue(IdentityKind::LocalServer).unwrap(),
            Identity::issue(IdentityKind::LocalServer).unwrap(),
        );
        let store = crate::record::job_records::JobRecords::over(record.connection());
        let made: Vec<_> = ["Hallway — final", "Bathroom — test", "Stairs — v2", "Turntable — v3", "Kitchen — clay"]
            .into_iter()
            .map(|label| crate::record::deletions::tests::a_record_in_history(&one, label))
            .collect();
        for job in &made {
            store.write_the_provisional_record(job).unwrap();
        }
        let deleted = Identity::issue(IdentityKind::Job).unwrap();
        let kitchen = Identity::issue(IdentityKind::Job).unwrap();
        let ended = |at: usize, ending, sequence| ANoticeEntry::Ended { job: made[at].job.clone(), ending, sequence };
        // **Failed first and finished on two servers**: the count still reads
        // in Design's order, across both records, and stopped stays at nought.
        crate::record::catch_up_notices::the_notice_is_written(
            record.connection(),
            &NoticeRecord {
                server: one.clone(),
                entries: vec![
                    ended(1, Ending::Failed, 1),
                    ended(0, Ending::Finished, 2),
                    ANoticeEntry::Ended { job: deleted, ending: Ending::Finished, sequence: 3 },
                    ANoticeEntry::InterruptedAndReRunFrom { job: made[4].job.clone(), attempt: 2, frame: Some(61), sequence: 4 },
                    ANoticeEntry::HeldJobDropped {
                        job: kitchen.clone(),
                        name: "Kitchen — final v4".to_string(),
                        why: "the hold's release outcome was written: dropped".to_string(),
                        sequence: 5,
                    },
                ],
                the_account_is_incomplete: false,
            },
        )
        .unwrap();
        crate::record::catch_up_notices::the_notice_is_written(
            record.connection(),
            &NoticeRecord {
                server: two.clone(),
                entries: vec![ended(2, Ending::Finished, 1), ended(3, Ending::GivenUp, 2)],
                the_account_is_incomplete: false,
            },
        )
        .unwrap();
        let holds = crate::servers::the_machines_work::WhatThisRunHolds::default();
        let notice = what_is_held(&holds, Some(&record), None).notice.expect("drawn before anything is connected");
        assert_eq!(
            notice.counted,
            vec![(Ending::Finished, 2), (Ending::Failed, 1), (Ending::GivenUp, 1)],
            "a deleted entry or a re-run was counted, a kind at nought kept, or the order is not Design's"
        );
        assert_eq!(notice.dropped.len(), 1, "the dropped job is a line, not a count");

        let text = said(&the_queue_says(&TheQueueOnScreen { notice: Some(notice), ..TheQueueOnScreen::default() }));
        assert!(text.contains(&format!("{:?}", Prose::new(SINCE_YOU_LAST_OPENED_HISTORY))), "no hover: {text}");
        for (at, (figure, word, class)) in
            [("2", ENDED_FINISHED, "e-ok"), ("1", ENDED_FAILED, "e-fail"), ("1", ENDED_GIVEN_UP, "e-gave")]
                .into_iter()
                .enumerate()
        {
            let says = |tail: &str, what: Said| {
                let pair = format!("{:?}: {:?}", format!("counted{at}{tail}"), what);
                assert!(text.contains(&pair), "{pair} is not said: {text}");
            };
            says("Figure", Said::Value(Verbatim::new(figure)));
            says("Word", Said::Prose(Prose::new(word)));
            says("Is", Said::Value(Verbatim::new(class)));
        }
        assert!(!text.contains("counted3"), "a kind at nought was said");

        // **Opening History clears it**, on both servers, and keeps the line.
        crate::surface::the_history::history_is_opened(Some(&record));
        let after = what_is_held(&holds, Some(&record), None).notice.expect("the dropped job's line went too");
        assert_eq!(after.counted, Vec::new(), "the count outlived opening History");
        assert_eq!(after.dropped.len(), 1);
        assert_eq!(after.dropped[0].job, kitchen.as_text());

        // **History's own command is what opens it**, on every ask from its
        // page and before the reading is posted.
        let history = include_str!("the_history.rs");
        let command = &history[history.find("pub fn the_history(").expect("the command")..];
        let command = &command[..command.find("\n}\n").expect("its end")];
        assert!(command.contains("history_is_opened(started.record.as_ref());"), "History's command clears nothing");
        drop(store);
        drop(record);
        let _ = std::fs::remove_dir_all(&path);
    }

    /// **The Creator's second notes, piece 6, on the page** — notes 10, 12 and
    /// 15. `web/the-queue.js` and `web/the-history.js` run under Node beside
    /// the boxes harness's stand-ins.
    ///
    /// - **Note 10** *(went with the banner, the Creator's ninth notes, note
    ///   3: the count itself opens History)*: History handed a job — now by
    ///   Transfers' *Open the job* alone — asks for that job.
    /// - **Note 12**: with no lines — a rendering entry before Blender prints,
    ///   and every History entry — *Blender output* is the control alone, in
    ///   the drawing's `.log.nolines`, with nothing beside it.
    /// - **Note 15**: each of a rendering entry's lines is drawn as one
    ///   isolated left-to-right run (`bdi`, `ltr`), which is what keeps its
    ///   characters in order inside the drawing's right-to-left cut.
    ///
    /// **What it bites on**: History dropping the job it is handed, the
    /// spacer or the grey window back on either screen, and a line drawn as
    /// plain text. **Skipped, and says so, where no Node is installed.**
    #[test]
    fn piece_6_the_lone_control_and_the_lines_on_the_page() {
        let Some(lines) = a_page_run(
            "piece-6-page",
            &["the-queue.js", "the-history.js", "the-value-drawn.js"],
            &[],
            r#"
globalThis.invoked = [];
globalThis.placesGoneTo = [];
globalThis.window.__TAURI__.core.invoke = (command, args) => {
  globalThis.invoked.push([command, args]);
  return Promise.resolve(command === "the_pane_layout" ? { seams: [] } : null);
};
await import("./the-queue.js");
await import("./the-history.js");
const tell = (kind, said) => { for (const handler of globalThis.told.get(kind) || []) handler({ said }); };
const textOf = (element) => (element.text || "") + (element.children || []).map(textOf).join("");
const shape = (element) => [element.tag].concat(element.className ? element.className.split(" ").sort() : []).join(".");
const queue = anElement("div");
globalThis.theRoot.appendChild(queue);
globalThis.screens.get("queueview").itIsDrawn(queue);
const anEntry = {
  theHeading: "Queue", lineCount: "1", line0Server: "s1", line0Name: "RunPod 4090",
  rowCount: "1", row0Job: "j1", row0Line: "0", row0Name: "Kitchen",
  entryJob: "j1", entryName: "Kitchen", entryServer: "RunPod 4090", blenderOutput: "Blender output",
};
tell("queue:the-queue", anEntry);
const log = queue.querySelector("div.log");
console.log(JSON.stringify({ log: shape(log), beside: log.children.map(shape) }));
tell("queue:the-queue", Object.assign({}, anEntry, {
  entryLineCount: "2", entryLine0: "00:00.430  render | Fra: 1 | Remaining: 00:00.05 | Mem: 2M | Sample 1/16",
  entryLine1: "Saved: '/out/f_0001.png'",
}));
const drawn = queue.querySelector("span.qlines").children.map((line) => ({
  inside: line.children.map((one) => one.tag + "." + (one.dir || "")),
  reads: textOf(line),
}));
const logged = queue.querySelector("div.log");
console.log(JSON.stringify({ drawn, log: shape(logged) }));
const history = anElement("div");
globalThis.theRoot.appendChild(history);
globalThis.screens.get("historyview").itIsDrawn(history);
globalThis.invoked.length = 0;
globalThis.screens.get("historyview").itIsCurrentNow("j-hall");
const asked = globalThis.invoked.filter(([command]) => command === "the_history").map(([, args]) => args.chosen);
tell("history:the-history", {
  theHeading: "History", hrowCount: "1", hrow0Job: "j-hall", hrow0Name: "Hallway — final", hrowChosen: "j-hall",
  entryJob: "j-hall", entryName: "Hallway — final", historyBlenderOutput: "Blender output", delete: "Delete",
});
const kept = history.querySelector("div.log");
console.log(JSON.stringify({ asked, log: shape(kept), beside: kept.children.map(shape) }));
"#,
        ) else {
            return;
        };
        assert_eq!(
            lines,
            vec![
                serde_json::json!({
                    "log": "div.log.nolines",
                    "beside": ["button"],
                }),
                serde_json::json!({
                    "drawn": [
                        { "inside": ["bdi.ltr"], "reads": "00:00.430  render | Fra: 1 | Remaining: 00:00.05 | Mem: 2M | Sample 1/16" },
                        { "inside": ["bdi.ltr"], "reads": "Saved: '/out/f_0001.png'" },
                    ],
                    "log": "div.log",
                }),
                serde_json::json!({
                    "asked": ["j-hall"],
                    "log": "div.log.nolines",
                    "beside": ["button"],
                }),
            ]
        );
    }

    /// **The rules the page's shapes above lean on are the drawing's** — the
    /// Creator's second notes, notes 12 and 15. `.log.nolines` takes the grey
    /// window away (note 12), and `.qlines > span` cuts a line at its start,
    /// right to left (note 15). Both are the drawing's own and
    /// `web/the-drawing.css` carries them unchanged; this holds that they are
    /// there for the page's classes to meet.
    #[test]
    fn piece_6_the_drawings_rules_the_page_leans_on() {
        let web = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        let drawing = std::fs::read_to_string(web.join("the-drawing.css")).expect("the drawing is readable");
        assert!(drawing.contains(".log.nolines { background: none; border: 0; padding: 0; }"));
        let cut = drawing
            .lines()
            .find(|line| line.starts_with(".qlines > span {"))
            .expect("the lines' own rule");
        for part in ["text-overflow: ellipsis", "white-space: nowrap", "direction: rtl", "text-align: left"] {
            assert!(cut.contains(part), "the lines' rule lost {part:?}: {cut}");
        }
    }

    // ── The Creator's second notes, piece 4's queue half ─────────────────────

    /// **The entry reads its folder off the record and never composes one**
    /// (`Job files and outputs` P21, `Queue & History` Y8): a waiting job whose
    /// folder is chosen at render start offers nothing that reaches into it,
    /// says *The folder is made when the render starts.* and draws no *Writing
    /// to* (the drawing's `q-wait`); once its render has started and the choice
    /// has not arrived, nothing reaches in and nothing is said; and a job with a
    /// folder offers the controls. **Where the folder fell back, *Writing to*
    /// carries `Job files and outputs` P19's case and
    /// `Job files and outputs` P23's passed-over line** (`q-fallback`,
    /// `q-fallchosen`), and the History entry, composed by the same function,
    /// carries the same lines.
    ///
    /// **What it bites on**: the controls offered before the folder exists,
    /// the waiting line drawn once the render has started, a case line on a
    /// folder where the file points, either case's sentence swapped, the
    /// second line missing where passed over or drawn where not, and *Writing
    /// to* drawn on the waiting entry.
    #[test]
    fn piece_4q_the_entry_reads_its_folder_off_the_record_and_says_why_it_fell_back() {
        use crate::surface::the_history::{the_entry_for, BesideTheRecord};
        use smoozy_job_files::where_the_folder_sits::{
            WhereItIs, WhereTheJobOutputFolderSits, WhyTheFilesOwnPlaceWasNotUsed,
        };
        use smoozy_queue_history::record::TheJobOutputFolderTheRecordHolds as Holds;
        use smoozy_state::identities::{Identity, IdentityKind};

        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        let mut record = crate::record::deletions::tests::a_record_in_history(&server, "Kitchen");
        let entry_of = |record: &smoozy_queue_history::record::LocalJobRecord| {
            the_entry_for(record, &BesideTheRecord::default(), false, true).entry
        };
        let writing_to = |entry: &TheJobEntry| entry.facts.iter().find(|fact| fact.name == WRITING_TO_FACT).cloned();
        let event_of = |entry: &TheJobEntry| said(&the_entry_is_said(the_queue_says(&a_queue()), Some(entry)));

        // **Waiting, chosen at render start**: the drawing's `q-wait`.
        record.the_job_output_folder = Holds::ChosenAtRenderStart;
        record.the_render_start_has_been_learned = false;
        record.main_output_from_the_read = None;
        let waiting = entry_of(&record);
        assert_eq!(waiting.the_folder, TheFolderOnTheEntry::ItIsMadeWhenTheRenderStarts);
        assert!(writing_to(&waiting).is_none(), "a job with no folder writes to nothing");
        let text = event_of(&waiting);
        assert!(text.contains("entryNoFolder"));
        assert!(text.contains("The folder is made when the render"));

        // **Started, the choice not arrived**: nothing reaches in, nothing said.
        record.the_render_start_has_been_learned = true;
        let started = entry_of(&record);
        assert_eq!(started.the_folder, TheFolderOnTheEntry::ItHasNotArrived);
        let text = event_of(&started);
        assert!(text.contains("entryNoFolder"));
        assert!(!text.contains("The folder is made when the render"));

        // **A job named before 2026-10-04** has its folder, and no line.
        record.the_job_output_folder = Holds::AsEveryJobBeforeItHadIt;
        record.main_output_from_the_read = Some("/smoozy/jobs/kitchen/".to_string());
        let older = entry_of(&record);
        assert_eq!(older.the_folder, TheFolderOnTheEntry::ItHasOne);
        assert!(!event_of(&older).contains("entryNoFolder"));
        assert!(writing_to(&older).unwrap().under.is_empty());

        let sits = |where_it_is: WhereItIs| {
            Holds::It(WhereTheJobOutputFolderSits { path: "/smoozy/jobs/kitchen".to_string(), where_it_is })
        };
        let under = |record: &smoozy_queue_history::record::LocalJobRecord| {
            let entry = entry_of(record);
            assert_eq!(entry.the_folder, TheFolderOnTheEntry::ItHasOne);
            writing_to(&entry).expect("Writing to is drawn").under
        };

        // **Where the file points**: no line.
        record.the_job_output_folder = sits(WhereItIs::WhereTheFilePoints);
        assert!(under(&record).is_empty());

        // **`q-fallback`**: one line, each case its own sentence.
        record.the_job_output_folder = sits(WhereItIs::TheFallback {
            why: Some(WhyTheFilesOwnPlaceWasNotUsed::ItIsAnotherComputers),
            the_servers_fallback_folder_was_passed_over: false,
        });
        assert_eq!(under(&record), vec![THE_FILES_FOLDER_IS_ON_ANOTHER_COMPUTER]);
        record.the_job_output_folder = sits(WhereItIs::TheFallback {
            why: Some(WhyTheFilesOwnPlaceWasNotUsed::ItIsAFolderSmoozyCannotUse),
            the_servers_fallback_folder_was_passed_over: false,
        });
        assert_eq!(under(&record), vec![SMOOZY_CANT_USE_THE_FILES_FOLDER]);

        // **`q-fallchosen`**: the second line under the first.
        record.the_job_output_folder = sits(WhereItIs::TheFallback {
            why: Some(WhyTheFilesOwnPlaceWasNotUsed::ItIsAnotherComputers),
            the_servers_fallback_folder_was_passed_over: true,
        });
        assert_eq!(
            under(&record),
            vec![THE_FILES_FOLDER_IS_ON_ANOTHER_COMPUTER, THE_SERVERS_FALLBACK_FOLDER_WAS_PASSED_OVER]
        );
        // **The lines travel on the event**, in order, under *Writing to*.
        let fell = entry_of(&record);
        let at = fell.facts.iter().position(|fact| fact.name == WRITING_TO_FACT).unwrap();
        let text = event_of(&fell);
        assert!(text.contains(&format!("fact{at}UnderCount")));
        assert!(text.contains("This server’s fallback folder can’t be used, so the files go to"));

        // **No case recorded** (no render destination, `Job files and outputs` P18): no case line.
        record.the_job_output_folder = sits(WhereItIs::TheFallback {
            why: None,
            the_servers_fallback_folder_was_passed_over: false,
        });
        assert!(under(&record).is_empty());

        // **Design's own words**, apostrophes as the screen writes them.
        let design = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../Preparation/Design/Queue & History.md"),
        )
        .expect("the Design is readable");
        for words in [
            THE_FILES_FOLDER_IS_ON_ANOTHER_COMPUTER,
            SMOOZY_CANT_USE_THE_FILES_FOLDER,
            THE_SERVERS_FALLBACK_FOLDER_WAS_PASSED_OVER,
            crate::surface::transfers::words::THE_FOLDER_IS_MADE_WHEN_THE_RENDER_STARTS,
        ] {
            assert!(design.contains(&words.replace('’', "'")), "not the Design's words: {words}");
        }
    }

    /// **The same on the page**: a waiting entry with no folder draws *The
    /// folder is made when the render starts.* where the downloads were, and
    /// neither download nor *Files on the server*; a started one with none
    /// draws neither and no line; *Writing to*'s lines sit under its path, one
    /// a line, on the queue and on History. **What it bites on**: the controls
    /// drawn where the application said there is no folder, the line missing or
    /// drawn where not said, *The server is gone* drawn where the server
    /// stands, the lines run into the path, and the rule that puts each on its
    /// own line gone. **Skipped, and says so, where no Node is installed.**
    #[test]
    fn piece_4q_no_folder_and_the_fallback_lines_on_the_page() {
        let css = include_str!("../../web/the-queue.css");
        assert!(css.contains(".facts dd .qfell { display: block; margin-top: 3px; }"));
        let Some(lines) = a_page_run(
            "piece-4q-page",
            &["the-queue.js", "the-history.js", "the-value-drawn.js"],
            &[],
            r#"
globalThis.window.__TAURI__.core.invoke = (command) =>
  Promise.resolve(command === "the_pane_layout" ? { seams: [] } : null);
await import("./the-queue.js");
await import("./the-history.js");
const tell = (kind, said) => { for (const handler of globalThis.told.get(kind) || []) handler({ said }); };
const textOf = (element) => (element.text || "") + (element.children || []).map(textOf).join("");
const shape = (element) => [element.tag].concat(element.className ? element.className.split(" ") : []).join(".");
const queue = anElement("div");
globalThis.theRoot.appendChild(queue);
globalThis.screens.get("queueview").itIsDrawn(queue);
const anEntry = {
  theHeading: "Queue", lineCount: "1", line0Server: "s1", line0Name: "RunPod 4090",
  rowCount: "1", row0Job: "j1", row0Line: "0", row0Name: "Kitchen",
  entryJob: "j1", entryName: "Kitchen", entryServer: "RunPod 4090", entryBlend: "kitchen.blend",
  entryReachBacks: "yes", downloadAll: "Download all", downloadAllTo: "Download all to…",
  filesOnTheServer: "Files on the server", filesOnYourComputer: "Files on your computer",
  entryStopOrCancel: "Cancel", entryStopOrCancelAct: "cancel-the-job",
  theServerIsGone: "unused",
};
const acts = () => queue.querySelector("div.qdacts").children.map((one) => [shape(one), textOf(one)]);
tell("queue:the-queue", Object.assign({}, anEntry, {
  entryNoFolder: "yes", theFolderIsMadeWhenTheRenderStarts: "The folder is made when the render starts.",
}));
console.log(JSON.stringify({ waiting: acts() }));
tell("queue:the-queue", Object.assign({}, anEntry, { entryNoFolder: "yes" }));
console.log(JSON.stringify({ started: acts() }));
const fell = {
  factCount: "1", fact0Name: "Writing to", fact0Value: "/smoozy/jobs/kitchen/", fact0Neither: "yes",
  fact0UnderCount: "2",
  fact0Under0: "Your file’s output folder is on another computer, so the files go here.",
  fact0Under1: "This server’s fallback folder can’t be used, so the files go to Smoozy’s folder.",
};
const theFact = (root) => {
  const value = root.querySelector("dl.facts").children[0].children[1];
  return [shape(value)].concat(value.children.map((one) => [shape(one), textOf(one)]));
};
tell("queue:the-queue", Object.assign({}, anEntry, fell));
console.log(JSON.stringify({ queue: theFact(queue), acts: acts().map(([, text]) => text) }));
tell("queue:the-queue", Object.assign({}, anEntry, { factCount: "1", fact0Name: "Writing to", fact0Value: "/home/ubuntu/Render/k/", fact0Neither: "yes" }));
console.log(JSON.stringify({ plain: theFact(queue) }));
const history = anElement("div");
globalThis.theRoot.appendChild(history);
globalThis.screens.get("historyview").itIsDrawn(history);
tell("history:the-history", Object.assign({
  theHeading: "History", hrowCount: "1", hrow0Job: "j1", hrow0Name: "Kitchen", hrowChosen: "j1",
  entryJob: "j1", entryName: "Kitchen", entryReachBacks: "yes", downloadAll: "Download all",
  downloadAllTo: "Download all to…", filesOnTheServer: "Files on the server",
  filesOnYourComputer: "Files on your computer", delete: "Delete",
}, fell));
console.log(JSON.stringify({ history: theFact(history) }));
"#,
        ) else {
            return;
        };
        assert_eq!(
            lines,
            vec![
                serde_json::json!({ "waiting": [
                    ["span.mnote2", "The folder is made when the render starts."],
                    ["span.agroup", "Files on your computer"],
                    ["button.mini.danger", "Cancel"],
                ] }),
                serde_json::json!({ "started": [
                    ["span.agroup", "Files on your computer"],
                    ["button.mini.danger", "Cancel"],
                ] }),
                serde_json::json!({
                    "queue": [
                        "dd",
                        ["span.mono.selectable", "/smoozy/jobs/kitchen/"],
                        ["span.qfell", "Your file’s output folder is on another computer, so the files go here."],
                        ["span.qfell", "This server’s fallback folder can’t be used, so the files go to Smoozy’s folder."],
                    ],
                    "acts": ["Download all", "Download all to…", "Files on the serverFiles on your computer", "Cancel"],
                }),
                serde_json::json!({ "plain": ["dd.mono", ["bdi", "/home/ubuntu/Render/k/"]] }),
                serde_json::json!({
                    "history": [
                        "dd",
                        ["span.mono.selectable", "/smoozy/jobs/kitchen/"],
                        ["span.qfell", "Your file’s output folder is on another computer, so the files go here."],
                        ["span.qfell", "This server’s fallback folder can’t be used, so the files go to Smoozy’s folder."],
                    ],
                }),
            ]
        );
    }

    // ── The Creator's third notes, piece B ───────────────────────────────────

    /// **A rule of the drawing's stylesheet, as written**: each declaration of
    /// the rule whose selector is exactly `selector`.
    fn the_drawings_rule(selector: &str) -> Vec<(String, String)> {
        let css = include_str!("../../web/the-drawing.css");
        let opens = format!("\n{selector} {{");
        let at = css.find(&opens).unwrap_or_else(|| panic!("no rule for {selector}")) + opens.len();
        let body = &css[at..at + css[at..].find('}').expect("the rule closes")];
        body.split(';')
            .filter_map(|one| one.split_once(':'))
            .map(|(name, value)| (name.trim().to_string(), value.trim().to_string()))
            .collect()
    }

    fn the_value_of(rule: &[(String, String)], name: &str) -> Option<String> {
        rule.iter().rev().find(|(one, _)| one == name).map(|(_, value)| value.clone())
    }

    /// **Note 10: the whole frame shows inside the preview, never cropped or
    /// stretched** (`Design/Queue & History`, 2026-10-06). The box a square
    /// picture is drawn in is worked out from the stylesheet's own `.preview`
    /// and `.thumb` rules, the way a browser fits a picture to its box, and
    /// must sit wholly inside the frame with the picture's own shape. Queue and
    /// History draw the preview with the one builder and the one rule.
    ///
    /// **Undone** (`object-fit: cover`, or the rule without it, which
    /// stretches), the box spills past the frame or loses its shape.
    #[test]
    fn third_notes_10_a_square_pictures_box_sits_wholly_inside_the_frame() {
        let preview = the_drawings_rule(".preview");
        let thumb = the_drawings_rule(".thumb");
        let ratio = the_value_of(&preview, "aspect-ratio").expect("the frame has a shape");
        let (wide, tall) = ratio.split_once('/').expect("a ratio");
        let (wide, tall): (f64, f64) = (wide.trim().parse().unwrap(), tall.trim().parse().unwrap());
        // The frame at the width the entry's side column draws it.
        let (frame_w, frame_h) = (320.0, 320.0 * tall / wide);
        let fit = the_value_of(&thumb, "object-fit").unwrap_or_else(|| "fill".to_string());
        // The browser's own default places the picture at the centre.
        let place = the_value_of(&thumb, "object-position").unwrap_or_else(|| "50% 50%".to_string());
        assert!(["50% 50%", "center", "center center"].contains(&place.as_str()), "{place}");
        for (picture_w, picture_h) in [(1000.0_f64, 1000.0_f64), (4096.0, 4096.0), (64.0, 64.0)] {
            let (box_w, box_h) = match fit.as_str() {
                "contain" => {
                    let scale = (frame_w / picture_w).min(frame_h / picture_h);
                    (picture_w * scale, picture_h * scale)
                }
                "scale-down" => {
                    let scale = (frame_w / picture_w).min(frame_h / picture_h).min(1.0);
                    (picture_w * scale, picture_h * scale)
                }
                "cover" => {
                    let scale = (frame_w / picture_w).max(frame_h / picture_h);
                    (picture_w * scale, picture_h * scale)
                }
                "none" => (picture_w, picture_h),
                _ => (frame_w, frame_h),
            };
            let (left, top) = ((frame_w - box_w) / 2.0, (frame_h - box_h) / 2.0);
            let inside = left >= -1e-9 && top >= -1e-9 && left + box_w <= frame_w + 1e-9 && top + box_h <= frame_h + 1e-9;
            assert!(inside, "a {picture_w}×{picture_h} picture is cut by the frame under object-fit: {fit}");
            assert!(
                (box_w / box_h - picture_w / picture_h).abs() < 1e-9,
                "a {picture_w}×{picture_h} picture is stretched under object-fit: {fit}"
            );
        }
        // **The picture fills the frame's box before it is fitted**, so the
        // fitting above is the frame's own and nothing else clips it.
        assert_eq!(the_value_of(&thumb, "inset").as_deref(), Some("0"));
        assert_eq!(the_value_of(&thumb, "width").as_deref(), Some("100%"));
        assert_eq!(the_value_of(&thumb, "height").as_deref(), Some("100%"));
        // **History draws the same box**: the one builder, exported once.
        let history = include_str!("../../web/the-history.js");
        assert!(!history.contains("\"thumb\""), "History draws a picture of its own");
    }

    /// **Note 14: in Queue the output control sits under the three lines**, not
    /// beside them, so the lines have the entry's whole width
    /// (`Design/Queue & History`, corrected 2026-10-06). The page draws the
    /// lines first and the control after them (piece 8's page test holds that
    /// order); the stylesheet stacks the two, and the lines take the width.
    ///
    /// **Undone** (`.log` a row again, or the lines not stretched), it fails.
    #[test]
    fn third_notes_14_the_output_control_sits_under_the_three_lines() {
        let log = the_drawings_rule(".log");
        assert_eq!(the_value_of(&log, "display").as_deref(), Some("flex"));
        assert_eq!(the_value_of(&log, "flex-direction").as_deref(), Some("column"), "the control sits beside the lines");
        let lines = the_drawings_rule(".log .qlines");
        assert_eq!(the_value_of(&lines, "align-self").as_deref(), Some("stretch"), "the lines do not take the width");
        // **The lines come first in the page, then the control.**
        let page = include_str!("../../web/the-queue.js");
        let drawn = &page[page.find("function theBlenderOutputIsDrawn").expect("the control's drawing")..];
        let drawn = &drawn[..drawn.find("\n}\n").expect("its end")];
        let (lines_at, control_at) = (
            drawn.find("qlines").expect("the lines are drawn"),
            drawn.rfind("appendChild(open)").expect("the control is drawn"),
        );
        assert!(lines_at < control_at, "the control is drawn before the lines");
    }

    /// **Note 11: Queue's *Writing to* is a link to Files on that folder**,
    /// the same act as *Files on the server*, and only where that control is
    /// offered; History's stays plain. **Undone** (no flag set, or set on
    /// History, or said where the server is gone), it fails.
    #[test]
    fn third_notes_11_writing_to_opens_files_on_queue_only_where_files_on_the_server_is() {
        use crate::surface::the_history::{the_entry_for, BesideTheRecord};
        use smoozy_state::identities::{Identity, IdentityKind};

        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        let mut record = crate::record::deletions::tests::a_record_in_history(&server, "Kitchen");
        record.main_output_from_the_read = Some("/home/ubuntu/Render/kitchen/".to_string());
        let event_of = |entry: &TheJobEntry| said(&the_entry_is_said(the_queue_says(&a_queue()), Some(entry)));
        let the_link = |entry: &TheJobEntry| {
            let at = entry.facts.iter().position(|fact| fact.name == WRITING_TO_FACT).expect("Writing to");
            event_of(entry).contains(&format!("fact{at}OpensFiles"))
        };

        let mut history = the_entry_for(&record, &BesideTheRecord::default(), false, true).entry;
        history.reach_backs = true;
        history.the_folder = TheFolderOnTheEntry::ItHasOne;
        assert!(!the_link(&history), "History's Writing to is plain");

        let mut queue = history.clone();
        the_writing_to_path_opens_files(&mut queue);
        assert!(the_link(&queue), "Queue's Writing to is a link");
        let mut gone = queue.clone();
        gone.reach_backs = false;
        assert!(!the_link(&gone), "no link where the server is gone");
        let mut not_yet = queue.clone();
        not_yet.the_folder = TheFolderOnTheEntry::ItHasNotArrived;
        assert!(!the_link(&not_yet), "no link where the folder has not arrived");

        record.main_output_from_the_read = None;
        let mut unknown = the_entry_for(&record, &BesideTheRecord::default(), false, true).entry;
        unknown.reach_backs = true;
        unknown.the_folder = TheFolderOnTheEntry::ItHasOne;
        the_writing_to_path_opens_files(&mut unknown);
        assert!(!the_link(&unknown), "*not known* is no folder to open");
    }

    /// **The same on the page**: the path, in both forms, is drawn as the
    /// drawing's `.jflink`, and a press asks for *Files on the server* on the
    /// entry's job; without the application's word it is plain text.
    /// **Undone**, the path is plain and no act is asked for. **Skipped, and
    /// says so, where no Node is installed.**
    #[test]
    fn third_notes_11_writing_to_is_a_link_asking_for_files_at_that_folder_on_the_page() {
        let Some(lines) = a_page_run(
            "third-notes-11-page",
            &["the-queue.js", "the-history.js", "the-value-drawn.js"],
            &[],
            r#"
globalThis.invoked = [];
globalThis.window.__TAURI__.core.invoke = (command, args) => {
  globalThis.invoked.push([command, args]);
  return Promise.resolve(command === "the_pane_layout" ? { seams: [] } : null);
};
await import("./the-queue.js");
const tell = (kind, said) => { for (const handler of globalThis.told.get(kind) || []) handler({ said }); };
const textOf = (element) => (element.text || "") + (element.children || []).map(textOf).join("");
const shape = (element) => [element.tag].concat(element.className ? element.className.split(" ") : []).join(".");
const queue = anElement("div");
globalThis.theRoot.appendChild(queue);
globalThis.screens.get("queueview").itIsDrawn(queue);
const anEntry = {
  theHeading: "Queue", lineCount: "1", line0Server: "s1", line0Name: "RunPod 4090",
  rowCount: "1", row0Job: "j1", row0Line: "0", row0Name: "Kitchen",
  entryJob: "j1", entryName: "Kitchen", entryServer: "RunPod 4090", entryBlend: "kitchen.blend",
  entryReachBacks: "yes", downloadAll: "Download all", downloadAllTo: "Download all to…",
  filesOnTheServer: "Files on the server", filesOnYourComputer: "Files on your computer",
  entryStopOrCancel: "Cancel", entryStopOrCancelAct: "cancel-the-job",
};
const theValue = () => queue.querySelector("dl.facts").children[0].children[1];
const drawn = (value) => [shape(value)].concat(value.children.map((one) => [shape(one), textOf(one)]));
const pressed = () => {
  globalThis.invoked.length = 0;
  const link = theValue().children.find((one) => String(one.tag).toLowerCase() === "a");
  if (link) link.click();
  return globalThis.invoked.filter(([command]) => command === "an_act_on_a_job").map(([, args]) => [args.act, args.job]);
};
tell("queue:the-queue", Object.assign({}, anEntry, {
  factCount: "1", fact0Name: "Writing to", fact0Value: "/home/ubuntu/Render/k/", fact0Neither: "yes", fact0OpensFiles: "yes",
}));
console.log(JSON.stringify({ own: drawn(theValue()), asks: pressed() }));
tell("queue:the-queue", Object.assign({}, anEntry, {
  factCount: "1", fact0Name: "Writing to", fact0Value: "/smoozy/jobs/kitchen/", fact0Neither: "yes", fact0OpensFiles: "yes",
  fact0UnderCount: "1", fact0Under0: "Your file’s output folder is on another computer, so the files go here.",
}));
console.log(JSON.stringify({ fell: drawn(theValue()), asks: pressed() }));
tell("queue:the-queue", Object.assign({}, anEntry, {
  factCount: "1", fact0Name: "Writing to", fact0Value: "/home/ubuntu/Render/k/", fact0Neither: "yes",
}));
console.log(JSON.stringify({ plain: drawn(theValue()), asks: pressed() }));
"#,
        ) else {
            return;
        };
        assert_eq!(
            lines,
            vec![
                serde_json::json!({
                    "own": ["dd.mono", ["a.jflink", "/home/ubuntu/Render/k/"]],
                    "asks": [["files-on-the-server", "j1"]],
                }),
                serde_json::json!({
                    "fell": [
                        "dd",
                        ["a.jflink.mono", "/smoozy/jobs/kitchen/"],
                        ["span.qfell", "Your file’s output folder is on another computer, so the files go here."],
                    ],
                    "asks": [["files-on-the-server", "j1"]],
                }),
                serde_json::json!({ "plain": ["dd.mono", ["bdi", "/home/ubuntu/Render/k/"]], "asks": [] }),
            ]
        );
    }

    /// **Step 10c's *Files on the server* follow-up: on a server that is not
    /// connected, neither *Files on the server* nor the *Writing to* link that
    /// asks for the same act is offered, and both come back when it
    /// connects** — `Design/File Browser § When another module moves the
    /// browser to a different server`: *"A machine that is not connected is
    /// not browsed and the action is not offered."* The downloads stay: they
    /// are Download's, and wait for the connection themselves.
    ///
    /// **Undone** (the flag not said, or the link said regardless), it fails.
    #[test]
    fn follow_up_files_on_the_server_and_its_link_are_not_offered_until_the_server_connects() {
        use crate::surface::the_history::{the_entry_for, BesideTheRecord};
        use smoozy_state::identities::{Identity, IdentityKind};

        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        let mut record = crate::record::deletions::tests::a_record_in_history(&server, "Kitchen");
        record.main_output_from_the_read = Some("/home/ubuntu/Render/kitchen/".to_string());
        let mut entry = the_entry_for(&record, &BesideTheRecord::default(), false, true).entry;
        entry.the_folder = TheFolderOnTheEntry::ItHasOne;
        the_writing_to_path_opens_files(&mut entry);
        let at = entry.facts.iter().position(|fact| fact.name == WRITING_TO_FACT).expect("Writing to");
        let drawn = |entry: &TheJobEntry| {
            let text = said(&the_entry_is_said(the_queue_says(&a_queue()), Some(entry)));
            (text.contains("entryNotConnected"), text.contains(&format!("fact{at}OpensFiles")))
        };

        assert_eq!(drawn(&entry), (false, true), "connected: the control and the link");
        entry.the_server_is_not_connected = true;
        assert_eq!(drawn(&entry), (true, false), "not connected: neither");
        entry.the_server_is_not_connected = false;
        assert_eq!(drawn(&entry), (false, true), "connected again: both back");
    }

    /// **The same on the page**: `entryNotConnected` leaves *Files on the
    /// server* out of the entry's acts, on Queue and on History, and the
    /// downloads and *Files on your computer* stay. **Undone**, the control is
    /// drawn. **Skipped, and says so, where no Node is installed.**
    #[test]
    fn follow_up_files_on_the_server_is_not_drawn_where_the_server_is_not_connected_on_the_page() {
        let Some(lines) = a_page_run(
            "follow-up-not-connected-page",
            &["the-queue.js", "the-history.js", "the-value-drawn.js"],
            &[],
            r#"
globalThis.invoked = [];
globalThis.window.__TAURI__.core.invoke = (command, args) => {
  globalThis.invoked.push([command, args]);
  return Promise.resolve(command === "the_pane_layout" ? { seams: [] } : null);
};
await import("./the-queue.js");
await import("./the-history.js");
const tell = (kind, said) => { for (const handler of globalThis.told.get(kind) || []) handler({ said }); };
const textOf = (element) => (element.text || "") + (element.children || []).map(textOf).join("");
const queue = anElement("div");
globalThis.theRoot.appendChild(queue);
globalThis.screens.get("queueview").itIsDrawn(queue);
const anEntry = {
  theHeading: "Queue", lineCount: "1", line0Server: "s1", line0Name: "RunPod 4090",
  rowCount: "1", row0Job: "j1", row0Line: "0", row0Name: "Kitchen",
  entryJob: "j1", entryName: "Kitchen", entryServer: "RunPod 4090", entryBlend: "kitchen.blend",
  entryReachBacks: "yes", downloadAll: "Download all", downloadAllTo: "Download all to…",
  filesOnTheServer: "Files on the server", filesOnYourComputer: "Files on your computer",
};
const acts = (host) => host.querySelector("div.qdh").children[1].children.map(textOf);
tell("queue:the-queue", anEntry);
console.log(JSON.stringify({ connected: acts(queue) }));
tell("queue:the-queue", Object.assign({}, anEntry, { entryNotConnected: "yes" }));
console.log(JSON.stringify({ notConnected: acts(queue) }));
const history = anElement("div");
globalThis.theRoot.appendChild(history);
globalThis.screens.get("historyview").itIsDrawn(history);
tell("history:the-history", {
  theHeading: "History", hrowCount: "1", hrow0Job: "j1", hrow0Name: "Kitchen", hrowChosen: "j1",
  entryJob: "j1", entryName: "Kitchen", entryReachBacks: "yes", entryNotConnected: "yes", downloadAll: "Download all",
  downloadAllTo: "Download all to…", filesOnTheServer: "Files on the server",
  filesOnYourComputer: "Files on your computer", delete: "Delete",
});
console.log(JSON.stringify({ history: acts(history) }));
"#,
        ) else {
            return;
        };
        assert_eq!(
            lines,
            vec![
                serde_json::json!({ "connected": ["Download all", "Download all to…", "Files on the serverFiles on your computer"] }),
                serde_json::json!({ "notConnected": ["Download all", "Download all to…", "Files on your computer"] }),
                serde_json::json!({ "history": ["Download all", "Download all to…", "Files on your computer", "Delete"] }),
            ]
        );
    }

    /// **The Creator's ninth notes, note 1, on the page: in Queue the entry is
    /// headed by the job's name alone.** No badge, server, file or scene under
    /// it, and none for a job with no scene named either; **History keeps the
    /// line**, with the scene or *No scene named*. `Design/Queue & History`,
    /// *A rendering entry*. **And the duplicate-picture line is bold whole**
    /// where the application says how many of its parts sit in the bold, the
    /// drawing's `q-marks` state; a line it says nothing of keeps its bold to
    /// its opening words.
    ///
    /// **What it bites on**: the line drawn on a queue entry (the state before
    /// this piece), the line dropped from History, the places of the twice line
    /// drawn outside the bold (E4's state), and the bold taking a line's words
    /// the application did not put there. **Skipped, and says so, where no Node
    /// is installed.**
    #[test]
    fn ninth_notes_n1_the_queue_entry_is_headed_by_its_name_alone_on_the_page() {
        let Some(lines) = a_page_run(
            "ninth-notes-n1-page",
            &["the-queue.js", "the-history.js", "the-value-drawn.js"],
            &[],
            r#"
globalThis.invoked = [];
globalThis.window.__TAURI__.core.invoke = (command, args) => {
  globalThis.invoked.push([command, args]);
  return Promise.resolve(command === "the_pane_layout" ? { seams: [] } : null);
};
await import("./the-queue.js");
await import("./the-history.js");
const tell = (kind, said) => { for (const handler of globalThis.told.get(kind) || []) handler({ said }); };
const textOf = (element) => (element.text || "") + (element.children || []).map(textOf).join("");
const theLine = { entryBadge: "b1", entryServer: "RunPod 4090", entryBlend: "kitchen.blend", entryScene: "Interior" };
const noScene = { entryBadge: "b1", entryServer: "RunPod 4090", entryBlend: "kitchen.blend", entryNoScene: "No scene named" };
const writes = {
  whatThisJobWrites: "What this job writes", writeCount: "2",
  write0Said: "Two of this job’s outputs write to the same file.", write0PartCount: "1",
  write0Part0Lead: "Your file is not changed.",
  write1Said: "Beauty pass written twice", write1PartCount: "3", write1Bold: "3",
  write1Part0Lead: "—", write1Part1Lead: "a File Output node", write1Part2Lead: "and the output path.",
};
const header = (host) => {
  const naming = host.querySelector("div.qdh").children[0];
  const sub = naming.querySelector("p.qdsub");
  return {
    name: textOf(naming.children[0]),
    sub: sub ? textOf(sub) : null,
    badge: sub ? sub.querySelectorAll("span.badge").length : 0,
  };
};
const theWrites = (host) => host.querySelectorAll("p.jw1").map((write) => ({
  bold: textOf(write.querySelector("b")),
  whole: textOf(write),
}));
const queue = anElement("div");
globalThis.theRoot.appendChild(queue);
globalThis.screens.get("queueview").itIsDrawn(queue);
const anEntry = (more) => Object.assign({
  theHeading: "Queue", lineCount: "1", line0Server: "s1", line0Name: "RunPod 4090",
  rowCount: "1", row0Job: "j1", row0Line: "0", row0Name: "Kitchen",
  entryJob: "j1", entryName: "Kitchen",
}, more);
tell("queue:the-queue", anEntry(Object.assign({}, theLine, writes)));
console.log(JSON.stringify({ queue: header(queue), writes: theWrites(queue) }));
tell("queue:the-queue", anEntry(noScene));
console.log(JSON.stringify({ queueNoScene: header(queue) }));
const history = anElement("div");
globalThis.theRoot.appendChild(history);
globalThis.screens.get("historyview").itIsDrawn(history);
const kept = (more) => Object.assign({
  theHeading: "History", hrowCount: "1", hrow0Job: "h1", hrow0Name: "Kitchen", hrowChosen: "h1",
  entryJob: "h1", entryName: "Kitchen",
}, more);
tell("history:the-history", kept(theLine));
console.log(JSON.stringify({ history: header(history) }));
tell("history:the-history", kept(noScene));
console.log(JSON.stringify({ historyNoScene: header(history) }));
"#,
        ) else {
            return;
        };
        assert_eq!(
            lines,
            vec![
                serde_json::json!({
                    "queue": { "name": "Kitchen", "sub": null, "badge": 0 },
                    "writes": [
                        {
                            "bold": "Two of this job’s outputs write to the same file.",
                            "whole": "Two of this job’s outputs write to the same file. Your file is not changed.",
                        },
                        {
                            "bold": "Beauty pass written twice — a File Output node and the output path.",
                            "whole": "Beauty pass written twice — a File Output node and the output path.",
                        },
                    ],
                }),
                serde_json::json!({ "queueNoScene": { "name": "Kitchen", "sub": null, "badge": 0 } }),
                serde_json::json!({
                    "history": { "name": "Kitchen", "sub": "RunPod 4090 · kitchen.blend · Interior", "badge": 1 }
                }),
                serde_json::json!({
                    "historyNoScene": { "name": "Kitchen", "sub": "RunPod 4090 · kitchen.blend · No scene named", "badge": 1 }
                }),
            ]
        );
    }

    // ── The Creator's ninth notes, note 3 ────────────────────────────────────

    /// **The Creator's ninth notes, note 3, on the page: the endings are
    /// counted at the end of the Queue heading's grey line, and each job
    /// dropped before it rendered is named in one line under it** —
    /// `Design/Queue & History § The catch-up notice`, the drawing's
    /// `#qcount` and `#qdropped`. The count reads *9 jobs · 3 servers · 3
    /// finished · 1 failed*, each kind in the class the application names,
    /// with the hover, and **pressing it opens History**. Each dropped line is
    /// the job's name in bold, then what happened to it, then *Dismiss*;
    /// **pressing a line's *Dismiss* asks the application for its box,
    /// handing back that line's job**, and the box's go-ahead hands back the
    /// job the box is about. With nothing counted and nothing dropped the grey
    /// line is as it was and no line is drawn.
    ///
    /// **What it bites on**: the banner back (no count, or a count that opens
    /// nothing), a kind drawn without its colour or apart from its figure, the
    /// hover missing, a dropped line without its name in bold or its
    /// *Dismiss*, the second line's *Dismiss* asking for the first line's job,
    /// the go-ahead handed the entry's job instead, and a count or line drawn
    /// from nothing. **Skipped, and says so, where no Node is installed.**
    #[test]
    fn ninth_notes_3_the_count_and_the_dropped_lines_on_the_page() {
        let Some(lines) = a_page_run(
            "ninth-notes-3-page",
            &["the-queue.js", "the-history.js", "the-value-drawn.js"],
            &[],
            r#"
globalThis.invoked = [];
globalThis.placesGoneTo = [];
globalThis.window.__TAURI__.core.invoke = (command, args) => {
  globalThis.invoked.push([command, args]);
  return Promise.resolve(command === "the_pane_layout" ? { seams: [] } : null);
};
await import("./the-queue.js");
const tell = (kind, said) => { for (const handler of globalThis.told.get(kind) || []) handler({ said }); };
const textOf = (element) => (element.text || "") + (element.children || []).map(textOf).join("");
const shape = (element) => [element.tag].concat(element.className ? element.className.split(" ").sort() : []).join(".");
const queue = anElement("div");
globalThis.theRoot.appendChild(queue);
globalThis.screens.get("queueview").itIsDrawn(queue);
const aQueue = (more) => Object.assign({
  theHeading: "Queue", theJobsFigure: "9", theJobsWord: "jobs", theServersFigure: "3", theServersWord: "servers",
  lineCount: "0", rowCount: "0",
}, more || {});
const counted = {
  theCountSince: "Since you last opened History.", countedCount: "2",
  counted0Figure: "3", counted0Word: "finished", counted0Is: "e-ok",
  counted1Figure: "1", counted1Word: "failed", counted1Is: "e-fail",
};
const dropped = {
  droppedCount: "2", dismiss: "Dismiss",
  dropped0Name: "Kitchen — final v4", dropped0Job: "j-kitchen",
  dropped0WhatLead: "was not started — the first frame was never rendered.",
  dropped1Name: "Attic", dropped1Job: "j-attic",
  dropped1WhatLead: "was not started — its Blender", dropped1WhatValue: "4.5", dropped1WhatTrail: "was removed.",
};
tell("queue:the-queue", aQueue(Object.assign({}, counted, dropped)));
const sub = queue.querySelector("span.qsub");
const count = sub.children.find((one) => one.id === "qcount");
const opens = count ? count.querySelector("button.lnkb") : null;
const theLines = () => queue.querySelectorAll("div.qbanner");
console.log(JSON.stringify({
  sub: textOf(sub),
  title: opens ? opens.title : null,
  kinds: opens ? opens.children.map((one) => ({ shape: shape(one), reads: textOf(one) })) : [],
  lines: theLines().map((line) => ({
    id: line.id,
    bold: textOf(line.querySelector("b")),
    reads: textOf(line.querySelector("p.qb2")),
    dismiss: line.querySelector("button.mini") ? [textOf(line.querySelector("button.mini")), line.querySelector("button.mini").id] : null,
  })),
}));
if (opens) opens.click();
globalThis.invoked.length = 0;
const second = theLines()[1];
if (second && second.querySelector("button.mini")) second.querySelector("button.mini").click();
console.log(JSON.stringify({
  went: globalThis.placesGoneTo,
  asked: globalThis.invoked.filter(([command]) => command === "the_queue").map(([, args]) => [args.asking, args.dropped]),
}));
tell("queue:the-queue", aQueue(Object.assign({}, dropped, {
  entryJob: "j-entry", entryName: "Kitchen",
  boxWhich: "dismiss", boxKeep: "Keep the notice", boxGoAct: "dismiss-the-notice", boxGoCount: "1", boxGo0Lead: "Dismiss",
  boxJob: "j-attic",
})));
globalThis.invoked.length = 0;
const box = queue.children.find((one) => one.classList && one.classList.contains("modal"));
if (box) box.querySelectorAll("button.btn")[1].click();
console.log(JSON.stringify({
  goAhead: globalThis.invoked.filter(([command]) => command === "an_act_on_a_job").map(([, args]) => [args.act, args.job]),
}));
tell("queue:the-queue", aQueue());
console.log(JSON.stringify({
  plain: textOf(queue.querySelector("span.qsub")),
  count: !!queue.querySelector("button.lnkb"),
  lines: theLines().length,
}));
"#,
        ) else {
            return;
        };
        assert_eq!(
            lines,
            vec![
                serde_json::json!({
                    "sub": "9\u{a0}jobs · 3\u{a0}servers · 3\u{a0}finished · 1\u{a0}failed",
                    "title": "Since you last opened History.",
                    "kinds": [
                        { "shape": "span.e-ok.qst", "reads": "3\u{a0}finished" },
                        { "shape": "span.qst", "reads": " · " },
                        { "shape": "span.e-fail.qst", "reads": "1\u{a0}failed" },
                    ],
                    "lines": [
                        {
                            "id": "qdropped",
                            "bold": "Kitchen — final v4",
                            "reads": "Kitchen — final v4 was not started — the first frame was never rendered.",
                            "dismiss": ["Dismiss", "catchdismiss"],
                        },
                        {
                            "id": "",
                            "bold": "Attic",
                            "reads": "Attic was not started — its Blender 4.5 was removed.",
                            "dismiss": ["Dismiss", "catchdismiss"],
                        },
                    ],
                }),
                serde_json::json!({
                    "went": [["historyview", null]],
                    "asked": [["dismiss", "j-attic"]],
                }),
                serde_json::json!({ "goAhead": [["dismiss-the-notice", "j-attic"]] }),
                serde_json::json!({ "plain": "9\u{a0}jobs · 3\u{a0}servers", "count": false, "lines": 0 }),
            ]
        );
    }

    /// **The head's list of the mockup states this file draws is the
    /// drawing's own, and since the Creator's ninth notes, note 3, it holds
    /// `q-dropped`** — a job dropped before it rendered — beside `q-catchup`,
    /// which now draws the endings counted in the heading. Every state the
    /// head names is an option of the drawing's *Queue & history* group, and
    /// there are as many as the head says.
    ///
    /// **What it bites on**: `q-dropped` missing from the head (the state
    /// before this piece), a state named that the drawing does not have, and
    /// the head's count left behind its list.
    #[test]
    fn the_heads_state_list_is_the_drawings_and_names_q_dropped() {
        let drawing = include_str!("../../../../Preparation/Design/Mockup/Smoozy.html");
        let opens = drawing.find("<optgroup label=\"Queue &amp; history\">").expect("the drawing's group");
        let group = &drawing[opens..opens + drawing[opens..].find("</optgroup>").expect("it closes")];
        let head = include_str!("the_queue.rs");
        let from = head.find("//! **The mockup state names this file draws**").expect("the head's list");
        let to = from + head[from..].find("**Nineteen, and").expect("the head says nineteen");
        let named: Vec<&str> = head[from..to]
            .split('`')
            .skip(1)
            .step_by(2)
            .filter(|name| *name == "q" || name.starts_with("q-"))
            .collect();
        assert_eq!(named.len(), 19, "the head says nineteen and names {named:?}");
        for name in &named {
            assert!(group.contains(&format!("<option value=\"{name}\"")), "the drawing has no state {name}");
        }
        assert!(named.contains(&"q-dropped"), "the head does not name q-dropped");
        assert!(named.contains(&"q-catchup"));
        for state in ["q-checkjob", "q-large", "q-pvmissing", "q-pvbad"] {assert!(named.contains(&state));}
        assert!(group.contains("<option value=\"q-catchup\">Jobs that ended, counted in the heading</option>"));
        assert!(group.contains("<option value=\"q-dropped\">A job dropped before it rendered</option>"));
    }

    // ── The Creator's second notes, piece 6b ─────────────────────────────────

    /// **The View Layer is named only on a frame that renders more than one**
    /// (`Design/Queue & History`, *A rendering entry*: *"the View Layer where a
    /// frame has more than one"*), as the job's reading says at that frame; a
    /// frame the reading does not cover, a report with no frame, and a job with
    /// no reading yet keep naming it — the safe side.
    ///
    /// **What it bites on**: the layer named on a frame of one (the state
    /// before this piece), dropped on a frame of several, dropped where the
    /// reading does not say, and dropped where there is no reading.
    #[test]
    fn piece_6b2_the_view_layer_is_named_only_on_a_frame_that_renders_more_than_one() {
        use crate::servers::the_machines_work::WhatTheReadingSays;
        let derived = crate::servers::the_machines_work::tests::a_recorded_reading(
            "/smoozy/jobs/kitchen",
            &[1, 2, 3],
            smoozy_job_files::reading::HowManyViewsTheSceneWrites::One,
        );
        let says = WhatTheReadingSays {
            six: smoozy_job_files::holds::the_six_things(
                &derived.reading.outputs,
                &derived.map,
                &derived.inventory,
                "/smoozy/jobs/kitchen",
            ),
            how_many_files: None,
            the_frames_cannot_be_counted: false,
            job_output_folder: "/smoozy/jobs/kitchen".to_string(),
            more_than_one_view_layer_at: [(1, false), (2, false), (3, true)].into_iter().collect(),
        };
        let at = |frame: Option<i64>, reading: Option<&WhatTheReadingSays>| {
            let mut report = crate::servers::the_machines_work::tests::a_report_taken_at(10);
            let mut live = report.readings.value().clone();
            live.frame_in_hand = frame;
            live.view_layer = Some("ViewLayer".to_string());
            report.readings = smoozy_state::observed::Reading::taken(
                live,
                std::time::SystemTime::UNIX_EPOCH,
                smoozy_state::observed::Clock::ThisComputer,
            );
            the_progress_from(&report, smoozy_queue_history::bars::WhichGrammar::FivePointZeroAndLater, reading)
                .expect("figures to draw")
                .now
                .iter()
                .any(|part| part.value == "ViewLayer")
        };
        assert!(!at(Some(2), Some(&says)), "a frame of one layer names it");
        assert!(at(Some(3), Some(&says)), "a frame of several layers does not name it");
        assert!(at(Some(9), Some(&says)), "a frame the reading does not cover keeps naming it");
        assert!(at(None, Some(&says)), "a report with no frame keeps naming it");
        assert!(at(Some(2), None), "a job with no reading yet keeps naming it");
    }

    // ── Step 10c's *Files on the server* piece ───────────────────────────────

    /// A record store holding one server whose row records its machine root,
    /// and a job on it written the way Submit writes one.
    fn a_job_on_a_rooted_server(
        folder: smoozy_queue_history::record::TheJobOutputFolderTheRecordHolds,
        main_output: Option<&str>,
    ) -> (rusqlite::Connection, String, String) {
        use smoozy_servers::identity::{Host, KeyReference, Port, Username};
        use smoozy_state::identities::{Identity, IdentityKind};
        let connection = crate::record::deletions::tests::a_record_store();
        let server = Identity::issue(IdentityKind::LocalServer).expect("random bytes");
        let mut row = smoozy_servers::store::ServerRow::saved(
            server.as_text(),
            Host::what_the_user_typed("127.0.0.1").expect("a host"),
            Port::what_the_user_typed(22).expect("a port"),
            Username::what_the_user_typed("root").expect("a username"),
            KeyReference { local_key_identifier: "key-1".into(), fingerprint: "SHA256:aaa".into() },
        );
        row.recorded_machine_root = Some(smoozy_servers::store::RecordedMachineRoot {
            path: "/root/smoozy".into(),
            the_package_areas_own_location: None,
            read_under: "inst-1".into(),
        });
        crate::record::server_rows::add(&connection, &row).expect("the row");
        let mut record = crate::record::deletions::tests::a_record_in_history(&server, "Kitchen");
        record.the_job_output_folder = folder;
        record.main_output_from_the_read = main_output.map(str::to_string);
        crate::record::job_records::JobRecords::over(&connection)
            .write_the_provisional_record(&record)
            .expect("the record");
        (connection, server.as_text(), record.job.as_text())
    }

    /// **Pressing *Files on the server*, or the *Writing to* link — which asks
    /// for the same act (`third_notes_11_writing_to_is_a_link_asking_for_files_at_that_folder_on_the_page`)
    /// — goes to Files and no longer answers *not built*.** Undone (the act
    /// routed back to answering), it fails.
    #[test]
    fn files_on_the_server_opens_files_and_never_answers_not_built() {
        assert_eq!(
            where_an_act_on_a_job_goes(TheActOnAJob::FilesOnTheServer),
            WhereAnActOnAJobGoes::TheJobsFolderOpensInFiles
        );
        assert_ne!(TheActOnAJob::FilesOnTheServer.answers(), A_CONNECTION_IS_NOT_BUILT);
        assert!(!TheActOnAJob::FilesOnTheServer.answers().contains("cannot do this yet"));
    }

    /// **The folder Files is asked to open is the one the entry names**:
    /// `Queue & History` N10's main output, the path *Writing to* shows, where the read landed
    /// one; the folder the render worker chose where it did not; the job
    /// area's for an older job; and on that job's own server. **What it
    /// answers instead is in the screen's own words**: the folder not made
    /// yet, the server gone, the job not in the record. Undone (any other
    /// folder, or no answer), it fails.
    #[test]
    fn files_on_the_server_opens_at_the_folder_the_entry_names_on_its_server() {
        use smoozy_job_files::where_the_folder_sits::{WhereItIs, WhereTheJobOutputFolderSits};
        use smoozy_queue_history::record::TheJobOutputFolderTheRecordHolds as Held;

        let chosen = Held::It(WhereTheJobOutputFolderSits {
            path: "/home/ubuntu/Render/kitchen-p8w2ktr6".into(),
            where_it_is: WhereItIs::WhereTheFilePoints,
        });
        let (connection, server, job) = a_job_on_a_rooted_server(chosen.clone(), Some("/home/ubuntu/Render/kitchen-p8w2ktr6/beauty"));
        assert_eq!(
            where_the_jobs_folder_is(&connection, &job),
            Ok((server, "/home/ubuntu/Render/kitchen-p8w2ktr6/beauty".to_string())),
            "the main output, as Writing to shows it"
        );

        let (connection, server, job) = a_job_on_a_rooted_server(chosen, None);
        assert_eq!(
            where_the_jobs_folder_is(&connection, &job),
            Ok((server, "/home/ubuntu/Render/kitchen-p8w2ktr6".to_string())),
            "the folder chosen at render start, before the read"
        );

        let (connection, server, job) = a_job_on_a_rooted_server(Held::AsEveryJobBeforeItHadIt, None);
        let (on, folder) = where_the_jobs_folder_is(&connection, &job).expect("an older job has its folder");
        assert_eq!(on, server);
        assert!(folder.starts_with("/root/smoozy/"), "the job area's: {folder}");

        let (connection, _, job) = a_job_on_a_rooted_server(Held::ChosenAtRenderStart, None);
        assert_eq!(
            where_the_jobs_folder_is(&connection, &job),
            Err(crate::surface::transfers::words::THE_FOLDER_IS_MADE_WHEN_THE_RENDER_STARTS)
        );

        // **A job whose server has no row any more** — removed.
        let removed = smoozy_state::identities::Identity::issue(smoozy_state::identities::IdentityKind::LocalServer)
            .expect("random bytes");
        let mut orphan = crate::record::deletions::tests::a_record_in_history(&removed, "Kitchen");
        orphan.main_output_from_the_read = Some("/x".to_string());
        crate::record::job_records::JobRecords::over(&connection)
            .write_the_provisional_record(&orphan)
            .expect("the record");
        assert_eq!(
            where_the_jobs_folder_is(&connection, &orphan.job.as_text()),
            Err(THE_SERVER_IS_GONE_AND_ITS_FILES_WITH_IT)
        );

        assert_eq!(
            where_the_jobs_folder_is(&connection, "not a job"),
            Err(TheActOnAJob::FilesOnTheServer.answers())
        );
    }

    /// **Files then shows that folder**: the page is told to open Files on
    /// that server at that folder, and Files, asked for it, answers with that
    /// folder being read. Undone (no event, another place, or the folder
    /// dropped), it fails.
    #[test]
    fn files_is_told_to_open_there_and_shows_that_folder() {
        use std::sync::{Arc, Mutex};
        struct Held(Arc<Mutex<Vec<(String, String, String)>>>);
        impl crate::events::Delivers for Held {
            fn deliver(&self, delivery: &crate::events::Delivery) {
                if let Ok(mut held) = self.0.lock() {
                    for (name, said) in &delivery.said {
                        let value = match said {
                            Said::Prose(prose) => prose.as_str().to_string(),
                            Said::Value(value) => value.as_str().to_string(),
                        };
                        held.push((delivery.kind.to_string(), name.to_string(), value));
                    }
                }
            }
        }
        let (connection, server, job) = a_job_on_a_rooted_server(
            smoozy_queue_history::record::TheJobOutputFolderTheRecordHolds::ChosenAtRenderStart,
            Some("/home/ubuntu/Render/kitchen/"),
        );
        let (on, folder) = where_the_jobs_folder_is(&connection, &job).expect("a folder");
        let got: Arc<Mutex<Vec<(String, String, String)>>> = Arc::new(Mutex::new(Vec::new()));
        let queue = TheOneQueue::new();
        queue.the_page_is_ready(Box::new(Held(got.clone())));
        // **The browser was on another server** (step 10c's *Files on the
        // server* follow-up), so the press moves it and Files' answer at the
        // job's folder carries the moved line — the one test that touches the
        // run's own record of where the browser is.
        assert!(!crate::surface::files::the_move_stands("another-server", None));
        // **And whether the job's record shows no frame saved** (piece O).
        the_jobs_folder_is_told(&queue, &on, &folder, the_record_shows_no_frame_saved(&connection, &job));
        assert!(
            crate::surface::files::the_move_stands(&on, Some(folder.as_bytes())),
            "the press did not tell Files it moved the browser"
        );
        assert!(
            crate::surface::files::no_frame_is_saved_here(),
            "the press did not tell Files the job has saved no frame"
        );
        let got = got.lock().unwrap().clone();
        let kind = crate::a_new_job_from_an_old_one::A_NEW_JOB_OPENS.to_string();
        assert_eq!(
            got,
            vec![
                (kind.clone(), "folder".to_string(), "/home/ubuntu/Render/kitchen/".to_string()),
                (kind.clone(), "goesTo".to_string(), "the-folder".to_string()),
                (kind, "server".to_string(), server.clone()),
            ]
        );

        let row = crate::record::server_rows::all(&connection).unwrap().into_iter().next().unwrap();
        let browser = crate::surface::files::the_tab_from(
            crate::surface::files::WhatThereIs::TheServer(Box::new(row), None),
            &server,
            Some(folder.clone().into_bytes()),
            None,
            true,
            false,
        );
        assert_eq!(browser.folder.bytes, folder.into_bytes(), "Files shows the job's folder");
        assert!(browser.a_reading_follows, "and reads it");
    }

    /// **The Creator's fourth notes, piece O: the press hands Files whether
    /// the job's record shows no frame saved** — true only where the record
    /// says it outright: no frame in the preview set, and an ending, where one
    /// is learned, with no figure. A frame in the set, a clean finish, any
    /// figure — the first frame's included, since that frame is on the server
    /// (the live pass on Vast, 2026-10-19, fault 5) — or a figure that cannot
    /// be counted all say a frame may be saved; so does a job the record does
    /// not hold. Undone (the set or the ending not read, or the
    /// fact always one way), it fails.
    #[test]
    fn piece_o_the_record_shows_no_frame_saved_only_where_it_says_so_outright() {
        use smoozy_queue_history::acting::note::{TheEndingOnTheNote, TheOutcomeAsWritten};
        use smoozy_queue_history::record::TheFramesAsked;
        use smoozy_state::identities::{Identity, IdentityKind};
        let (connection, server, job) = a_job_on_a_rooted_server(
            Default::default(),
            Some("/root/Render/kitchen/beauty/"),
        );
        assert!(the_record_shows_no_frame_saved(&connection, &job), "nothing is recorded of any frame");
        let identity = Identity::read_back(IdentityKind::Job, &job).unwrap();
        crate::record::previews::a_derivative_is_owed(&connection, &identity, 1001, "op-1", None, true, 0, None).unwrap();
        assert!(!the_record_shows_no_frame_saved(&connection, &job), "a frame is in the set");

        let server = Identity::read_back(IdentityKind::LocalServer, &server).unwrap();
        let ended = |outcome, figure, counted: bool| {
            let mut record = crate::record::deletions::tests::a_record_in_history(&server, "Kitchen");
            record.what_the_note_carried.ending = Some(TheEndingOnTheNote {
                outcome,
                exit_status: None,
                the_attempt_whose_capture_record_holds_the_words: 1,
                the_reason_the_machine_recorded: None,
                no_render_began: false,
                the_device_line_as_observed: None,
            });
            record.what_the_note_carried.last_frame_figure = figure;
            let records = crate::record::job_records::JobRecords::over(&connection);
            records.write_the_provisional_record(&record).expect("the record");
            // **The frames asked land with the read**, as they do in the run.
            if counted {
                let read = smoozy_queue_history::record::WhatTheReadProduced {
                    the_frames: smoozy_queue_history::record::TheFramesTheReadFound {
                        asked: TheFramesAsked::from_the_order(&(1..=240).collect::<Vec<i64>>()),
                        they_cannot_be_counted: false,
                    },
                    ..Default::default()
                };
                records.the_read_is_filled_in(&record.job, &read).expect("the read");
            }
            the_record_shows_no_frame_saved(&connection, &record.job.as_text())
        };
        assert!(ended(TheOutcomeAsWritten::Failed, None, true), "failed with no figure finished no frame");
        assert!(
            !ended(TheOutcomeAsWritten::Failed, Some(1), true),
            "the first frame is on the server, so a frame is saved"
        );
        assert!(!ended(TheOutcomeAsWritten::Failed, Some(61), true), "sixty-one frames on the server");
        assert!(!ended(TheOutcomeAsWritten::Finished, None, true), "a clean finish");
        assert!(!ended(TheOutcomeAsWritten::Finished, None, false), "a clean finish, frames not counted");
        assert!(!ended(TheOutcomeAsWritten::Failed, Some(61), false), "a figure that cannot be counted");
        assert!(!the_record_shows_no_frame_saved(&connection, "not a job"));
        let elsewhere = Identity::issue(IdentityKind::Job).unwrap().as_text();
        assert!(!the_record_shows_no_frame_saved(&connection, &elsewhere), "a job the record does not hold");
    }

    /// **The Creator's fourth notes, piece O, note 3: *Writing to* showed the
    /// Render Settings panel they had submitted from**, still open over that
    /// server and shown again over Files on arrival. Run under Node with the
    /// real `web/a-server.js` and `web/a-new-job.js` over a stand-in document:
    /// a panel open over a server on its Set up tab, the user goes to the
    /// queue, and the press arrives. **The panel collapses to its strip, Files
    /// is the tab showing, at the folder handed over; the strip brings the
    /// same panel back with what was typed in it** — `Design/Whole app §
    /// Panels over a server`, added 2026-10-07. Undone (no collapse), the
    /// panel shows on arrival and there is no strip, and it fails. **Skipped,
    /// and says so, where no Node is installed.**
    #[test]
    fn piece_o_writing_to_collapses_the_panel_over_its_server_and_the_strip_brings_it_back() {
        use std::fs;
        let folder = std::env::temp_dir().join(format!("smoozy-piece-o-{}", std::process::id()));
        let _ = fs::remove_dir_all(&folder);
        fs::create_dir_all(&folder).expect("a scratch folder");
        let web = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        for page in ["a-new-job.js", "a-server.js"] {
            fs::copy(web.join(page), folder.join(page)).expect("the page is copied");
        }
        for (name, text) in [
            ("package.json", r#"{ "type": "module" }"#),
            (
                "event-route.js",
                "export function whenTold(kind, handler) { ((globalThis.told ||= {})[kind] ||= []).push(handler); }",
            ),
            (
                "the-rail.js",
                "export const A_SERVER = 'machineview'; export const ALL_SERVERS = 'serversview'; \
                 let screen = null; let at = null; \
                 export function aScreenIsRegistered(one) { screen = one; } \
                 export function theCurrentPlaceIs(place, which) { \
                   if (at === A_SERVER && place !== A_SERVER) screen.itIsPutAway(); \
                   at = place; if (place !== A_SERVER) return; \
                   if (!globalThis.serverScreen) { globalThis.serverScreen = document.createElement('div'); screen.itIsDrawn(globalThis.serverScreen); } \
                   screen.itIsCurrentNow(which); }",
            ),
            ("servers.js", "export function theMomentReads(value) { return value; } export function theConnectionIsDrawnInto() {}"),
            ("the-value-drawn.js", "export function theValueIsDrawnInto(node, value) { node.textContent = value; }"),
            ("render-settings.js", "export async function theSettingsAreOpenedOn() {}"),
            (
                "files.js",
                "export function theBrowserIsAskedAgain() {} \
                 export function theBrowserIsOpenedAt(...args) { globalThis.openedAt = args; } \
                 export function theSavedWithVersionReadFor() { return ''; }",
            ),
            ("the-history.js", "export function theHistoryEntryIsOpened() {}"),
            (
                "document.mjs",
                r#"class TextPiece {
  constructor(text) { this.data = String(text); this.parentNode = null; }
  get textContent() { return this.data; }
  remove() { if (this.parentNode) this.parentNode.removeChild(this); }
}
class Piece {
  constructor(tag) {
    this.tagName = tag.toUpperCase(); this.children = []; this.own = ""; this.dataset = {}; this.hidden = false;
    this.attributes = {}; this.parentNode = null; this.listeners = {}; this.id = ""; this.title = ""; this.type = "";
    const names = new Set();
    this.names = names;
    this.classList = {
      add: (...more) => more.forEach((one) => names.add(one)),
      remove: (...more) => more.forEach((one) => names.delete(one)),
      contains: (one) => names.has(one),
      toggle: (one, on) => { const want = on === undefined ? !names.has(one) : Boolean(on); if (want) names.add(one); else names.delete(one); return want; },
    };
  }
  set className(value) { this.names.clear(); String(value).split(/\s+/).filter(Boolean).forEach((one) => this.names.add(one)); }
  get className() { return [...this.names].join(" "); }
  set textContent(value) { this.children.forEach((one) => { one.parentNode = null; }); this.children = []; this.own = String(value); }
  get textContent() { return this.own + this.children.map((one) => one.textContent).join(""); }
  appendChild(child) { if (child.parentNode) child.parentNode.removeChild(child); child.parentNode = this; this.children.push(child); return child; }
  insertBefore(child, before) {
    if (child.parentNode) child.parentNode.removeChild(child);
    child.parentNode = this;
    const at = this.children.indexOf(before);
    if (at < 0) this.children.push(child); else this.children.splice(at, 0, child);
    return child;
  }
  removeChild(child) { const at = this.children.indexOf(child); if (at >= 0) this.children.splice(at, 1); child.parentNode = null; return child; }
  remove() { if (this.parentNode) this.parentNode.removeChild(this); }
  contains(node) { for (let at = node; at; at = at.parentNode) if (at === this) return true; return false; }
  get childElementCount() { return this.children.filter((one) => one instanceof Piece).length; }
  setAttribute(name, value) { this.attributes[name] = String(value); }
  addEventListener(kind, handler) { (this.listeners[kind] ||= []).push(handler); }
  press() { for (const handler of this.listeners.click || []) handler({ preventDefault() {}, target: this }); }
  focus() { document.activeElement = this; }
  querySelectorAll(selector) { return everyPiece(this).filter((one) => one !== this && matches(one, selector)); }
  querySelector(selector) { return this.querySelectorAll(selector)[0] || null; }
}
function matches(one, selector) {
  const named = /^\.([\w-]+)$/.exec(selector);
  if (named) return one.names.has(named[1]);
  const data = /^\[data-([\w-]+)="(.*)"\]$/.exec(selector);
  if (data) return one.dataset[data[1].replace(/-(\w)/g, (_, letter) => letter.toUpperCase())] === data[2];
  throw new Error("a selector the stand-in does not know: " + selector);
}
export function everyPiece(root, found = []) {
  if (!(root instanceof Piece)) return found;
  found.push(root);
  root.children.forEach((one) => everyPiece(one, found));
  return found;
}
export function shows(one) { for (let at = one; at; at = at.parentNode) if (at.hidden) return false; return Boolean(one); }
globalThis.document = {
  createElement: (tag) => new Piece(tag),
  createElementNS: (_, tag) => new Piece(tag),
  createTextNode: (text) => new TextPiece(text),
  addEventListener() {},
  activeElement: null,
};
"#,
            ),
        ] {
            fs::write(folder.join(name), text).expect("a stand-in is written");
        }
        let harness = format!(
            r##"
import {{ everyPiece, shows }} from "./document.mjs";
const answer = {{
  the_head: {{ badge: "b1", name: "Hetzner box", address: "203.0.113.4", the_hardware: {{ state: "read" }}, the_work: [], the_rental_line: {{}} }},
  the_tabs: [{{ tab: "files", name: "Files" }}, {{ tab: "setupview", name: "Set up" }}],
  the_panels: [{{ panel: "chk", name: "Dependency check" }}, {{ panel: "set", name: "Render settings" }}],
  the_stand_in: "", collapse: "Collapse", open: "Open",
}};
globalThis.window = {{ __TAURI__: {{ core: {{ invoke(command) {{
  return Promise.resolve(command === "the_servers_own_screen" ? answer : null);
}} }} }} }};
const settle = async () => {{ for (let at = 0; at < 8; at += 1) await new Promise((go) => setImmediate(go)); }};
const rail = await import("./the-rail.js");
const theServer = await import("./a-server.js");
await import("./a-new-job.js");
const seen = [];
theServer.aTabIsRegistered({{ tab: theServer.FILES, itIsShowing(which) {{ seen.push(["files showing", which]); }} }});
theServer.aPanelIsRegistered({{
  panel: theServer.RENDER_SETTINGS,
  itIsDrawn(parts) {{ const typed = document.createElement("input"); typed.dataset.typed = "half-typed"; parts.body.appendChild(typed); }},
  itIsShowingAgain(which) {{ seen.push(["panel again", which]); }},
}});
// **The user on that server's Set up tab, the panel they submitted from
// open over it**, then off to the queue.
rail.theCurrentPlaceIs(rail.A_SERVER, "s1");
await settle();
theServer.theCurrentTabIs(theServer.SET_UP);
theServer.thePanelIsOpenedOver("s1", theServer.RENDER_SETTINGS);
const host = globalThis.serverScreen;
const thePanel = () => everyPiece(host).find((one) => one.dataset.panel === "set" && one.names.has("sheetwrap"));
const theStrip = () => everyPiece(host).find((one) => one.dataset.panel === "set" && one.names.has("strip"));
const before = thePanel();
const out = {{ openAtFirst: shows(before) }};
rail.theCurrentPlaceIs("queueview");
seen.length = 0;
// **Writing to** — the press, as the application posts it.
globalThis.told["{A_NEW_JOB_OPENS}"].forEach((handler) => handler({{ said: {{ goesTo: "the-folder", server: "s1", folder: "/root/out/beauty" }} }}));
await settle();
const files = everyPiece(host).find((one) => one.dataset.tabHost === "files");
out.onArrival = {{
  panelShows: shows(thePanel()),
  filesShows: shows(files),
  stripShows: shows(theStrip()),
  seen: seen.slice(),
  openedAt: globalThis.openedAt,
}};
seen.length = 0;
// No strip to press is a failure the comparison below names.
if (theStrip()) theStrip().press();
out.fromTheStrip = {{
  theSamePanel: thePanel() === before,
  panelShows: shows(thePanel()),
  stillTyped: everyPiece(thePanel()).some((one) => one.dataset.typed === "half-typed"),
  stripShows: shows(theStrip()),
  seen: seen.slice(),
}};
console.log(JSON.stringify(out));
"##,
            A_NEW_JOB_OPENS = crate::a_new_job_from_an_old_one::A_NEW_JOB_OPENS,
        );
        fs::write(folder.join("harness.mjs"), harness).expect("the harness is written");
        let ran = std::process::Command::new("node")
            .arg(folder.join("harness.mjs"))
            .current_dir(&folder)
            .output();
        let _ = fs::remove_dir_all(&folder);
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so web/a-server.js and web/a-new-job.js were not run");
            return;
        };
        assert!(ran.status.success(), "the pages did not run: {}", String::from_utf8_lossy(&ran.stderr));
        let got: serde_json::Value = serde_json::from_slice(&ran.stdout).expect("one answer");
        assert_eq!(
            got,
            serde_json::json!({
                "openAtFirst": true,
                "onArrival": {
                    "panelShows": false,
                    "filesShows": true,
                    "stripShows": true,
                    "seen": [["files showing", "s1"]],
                    "openedAt": ["s1", [47, 114, 111, 111, 116, 47, 111, 117, 116, 47, 98, 101, 97, 117, 116, 121]],
                },
                "fromTheStrip": {
                    "theSamePanel": true,
                    "panelShows": true,
                    "stillTyped": true,
                    "stripShows": false,
                    "seen": [["panel again", "s1"]],
                },
            })
        );
    }

    /// **The Creator's fourteenth notes, notes 7a and 7b, on the page: the View
    /// Layer dropdown carries its label, and opens upward at the window's
    /// foot.** The dropdown sits in the drawing's `.pvvlrow` after a `span.sn`
    /// reading the application's *View Layer*; a frame of one picture draws no
    /// row. Its list, measured as it opens, takes `up` where it would pass the
    /// window's bottom with more room above its control than under it, loses
    /// it with room below, stays down where there is more room below than
    /// above, and keeps the side it was measured to when the entry is drawn
    /// again with the list open. `Design/Queue & History § How the stepping
    /// and the View Layers look`; `Design/Whole app`, *A list with no room
    /// below it opens upward*; the drawing's `q-pv`. **Every word is the
    /// application's**: the harness hands them in.
    ///
    /// **What it bites on**: the dropdown with no label, or the label drawn
    /// where there is one picture; a list at the foot that opens downward; an
    /// `up` that stays once there is room. **Skipped, and says so, where no
    /// Node is installed.**
    #[test]
    fn fourteenth_notes_7_the_view_layer_dropdown_is_labelled_and_opens_upward_at_the_foot() {
        let Some(lines) = a_page_run(
            "fourteenth-notes-7-page",
            &["the-queue.js", "the-history.js", "the-value-drawn.js"],
            &[],
            r#"
globalThis.invoked = [];
globalThis.window.__TAURI__.core.invoke = (command, args) => {
  globalThis.invoked.push([command, args]);
  return Promise.resolve(command === "the_pane_layout" ? { seams: [] } : null);
};
await import("./the-queue.js");
await import("./the-history.js");
const tell = (kind, said) => { for (const handler of globalThis.told.get(kind) || []) handler({ said }); };
const textOf = (element) => (element.text || "") + (element.children || []).map(textOf).join("");
const held = (names) => {
  const said = { previewHeldCount: "1", previewHeldFrame0: "120", previewHeldLayerCount0: String(names.length), previewHeldMain0: "0" };
  names.forEach((name, one) => { said["previewHeld0Layer" + one] = name; said["previewHeld0Place" + one] = String(one); });
  return said;
};
const anEntry = (job, names) => Object.assign({
  theHeading: "Queue", lineCount: "1", line0Server: "s1", line0Name: "RunPod", rowCount: "1", row0Job: job, row0Line: "0",
  row0Name: "Kitchen", entryJob: job, entryName: "Kitchen", previewPicture: job, previewFrame: "120", previewWhich: "latest",
  frameHeading: "Frame preview", frameWord: "Frame", previousFrame: "Previous frame", nextFrame: "Next frame",
  viewLayer: "View Layer", didNotRenderOnThisFrame: "did not render on this frame.", notColourAccurate: "Not colour-accurate.",
}, held(names));
const queue = anElement("div");
globalThis.theRoot.appendChild(queue);
globalThis.screens.get("queueview").itIsDrawn(queue);
const look = () => {
  const row = queue.querySelector("div.pvvlrow");
  return row ? { row: row.children.map((one) => one.tag + "." + one.className.split(" ").sort().join(".")), label: textOf(row.children[0]) } : { row: null };
};
tell("queue:the-queue", anEntry("j1", ["Interior", "FX", "Reflections"]));
console.log(JSON.stringify(look()));
// The window is 800 tall. Each opening is measured against where the
// control and its list stand at that moment.
const at = (faceTop, listBottom) => {
  const pick = queue.querySelector("span.pick");
  pick.querySelector("button.pkb").getBoundingClientRect = () => ({ top: faceTop, bottom: faceTop + 26, left: 0, width: 200, height: 26 });
  pick.querySelector("span.ddl").getBoundingClientRect = () => ({ top: faceTop + 30, bottom: listBottom, left: 0, width: 200, height: listBottom - faceTop - 30 });
  return pick;
};
const opens = (faceTop, listBottom) => {
  const pick = at(faceTop, listBottom);
  const face = pick.querySelector("button.pkb"), list = pick.querySelector("span.ddl");
  if (!list.hidden) face.click();
  face.click();
  return { open: !list.hidden, up: list.classList.contains("up") };
};
const foot = opens(740, 860);
const room = opens(200, 320);
const longerBelow = opens(300, 900);
const footAgain = opens(740, 860);
tell("queue:the-queue", anEntry("j1", ["Interior", "FX", "Reflections"]));
const redrawn = queue.querySelector("span.ddl");
console.log(JSON.stringify({ foot, room, longerBelow, footAgain, redrawn: { open: !redrawn.hidden, up: redrawn.classList.contains("up") } }));
tell("queue:the-queue", anEntry("j2", ["Mist"]));
console.log(JSON.stringify(look()));
"#,
        ) else {
            return;
        };
        assert_eq!(
            lines,
            vec![
                serde_json::json!({ "row": ["span.sn", "span.pick.pvvl"], "label": "View Layer" }),
                serde_json::json!({
                    "foot": { "open": true, "up": true },
                    "room": { "open": true, "up": false },
                    "longerBelow": { "open": true, "up": false },
                    "footAgain": { "open": true, "up": true },
                    "redrawn": { "open": true, "up": true },
                }),
                serde_json::json!({ "row": null }),
            ]
        );
        // **The word is the application's**, and the drawing's label is it.
        assert_eq!(VIEW_LAYER, "View Layer");
        let drawing = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../Preparation/Design/Mockup/Smoozy.html"),
        )
        .expect("the drawing is readable");
        assert!(drawing.contains(&format!("<div class=\"pvvlrow\" data-pv hidden><span class=\"sn\">{VIEW_LAYER}</span>")));
    }

    /// **The Creator's fourteenth notes, note 8, on the page: a refused act
    /// pressed in the entry's acts row answers on its own line directly under
    /// the row; inside a box it stays after the control.** On History, *Files
    /// on your computer* refused with *This folder is no longer there.*: the
    /// answer is the acts row's next sibling, not inside `.agroup` and not
    /// inside the row, so *Delete* stays in the row. On the queue, a stop box's
    /// go-ahead refused: the answer sits right after the pressed button, inside
    /// the box. The drawing's `h-gone` and its `.qdacts + .qanswer`.
    ///
    /// **What it bites on**: the answer placed after the pressed button in the
    /// acts row, which put it inside `.agroup` beside the buttons. **Skipped,
    /// and says so, where no Node is installed.**
    #[test]
    fn fourteenth_notes_8_a_refused_acts_answer_goes_under_the_acts_row() {
        let Some(lines) = a_page_run(
            "fourteenth-notes-8-page",
            &["the-queue.js", "the-history.js", "the-value-drawn.js"],
            &[],
            r#"
globalThis.invoked = [];
globalThis.window.__TAURI__.core.invoke = (command, args) => {
  globalThis.invoked.push([command, args]);
  return Promise.resolve(command === "the_pane_layout" ? { seams: [] } : null);
};
await import("./the-queue.js");
await import("./the-history.js");
const tell = (kind, said) => { for (const handler of globalThis.told.get(kind) || []) handler({ said }); };
const textOf = (element) => (element.text || "") + (element.children || []).map(textOf).join("");
const shape = (element) => element ? [element.tag].concat(element.className ? element.className.split(" ").sort() : []).join(".") : null;
const history = anElement("div");
globalThis.theRoot.appendChild(history);
globalThis.screens.get("historyview").itIsDrawn(history);
tell("history:the-history", {
  theHeading: "History", hrowCount: "1", hrow0Job: "j-hall", hrow0Name: "Hallway — final", hrowChosen: "j-hall",
  entryJob: "j-hall", entryName: "Hallway — final", entryReachBacks: "yes",
  downloadAll: "Download all", downloadAllTo: "Download all to…", filesOnTheServer: "Files on the server",
  filesOnYourComputer: "Files on your computer", delete: "Delete",
});
const acts = history.querySelector("div.qdacts");
const pressed = acts.querySelectorAll("button").find((one) => textOf(one) === "Files on your computer");
pressed.click();
tell("queue:the-answer", { act: "files-on-your-computer", said: "Folder not found." });
const answer = history.querySelector("div.qanswer");
const beside = answer.parentNode.children;
console.log(JSON.stringify({
  holder: shape(answer.parentNode),
  before: shape(beside[beside.indexOf(answer) - 1]),
  inTheRow: acts.contains(answer),
  rowEndsWith: textOf(acts.children[acts.children.length - 1]),
}));

const queue = anElement("div");
globalThis.theRoot.appendChild(queue);
globalThis.screens.get("queueview").itIsDrawn(queue);
tell("queue:the-queue", {
  theHeading: "Queue", lineCount: "1", line0Server: "s1", line0Name: "RunPod 4090",
  rowCount: "1", row0Job: "j1", row0Line: "0", row0Name: "Kitchen", entryJob: "j1",
  boxWhich: "stop", boxKeep: "Keep rendering", boxGoAct: "stop-the-job", boxGoCount: "1", boxGo0Lead: "Stop",
});
const box = queue.children.find((one) => one.classList && one.classList.contains("modal"));
const go = box.querySelectorAll("button.btn")[1];
go.click();
tell("queue:the-answer", { act: "stop-the-job", said: "This job is not rendering." });
const inBox = box.querySelector("div.qanswer");
const around = inBox ? inBox.parentNode.children : [];
console.log(JSON.stringify({ inTheBox: Boolean(inBox), afterTheControl: inBox ? around[around.indexOf(inBox) - 1] === go : false }));
"#,
        ) else {
            return;
        };
        assert_eq!(
            lines,
            vec![
                serde_json::json!({
                    "holder": "div.qdh",
                    "before": "div.qdacts",
                    "inTheRow": false,
                    "rowEndsWith": "Delete",
                }),
                serde_json::json!({ "inTheBox": true, "afterTheControl": true }),
            ]
        );
        // **The drawing's rule is the shipped sheet's**, with no margin.
        let sheet = std::fs::read_to_string(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web/the-drawing.css"))
            .expect("the drawing's sheet is readable");
        assert!(sheet.contains(".qdacts + .qanswer { flex-basis: 100%; }"));
    }

    /// **The nineteenth notes, note 7** — `Design/Queue & History § A
    /// first-frame check and its job`: a check and the full range waiting on
    /// it are one row and one entry, in Queue and in History, in each of the
    /// drawing's readings (`q-checkjob`, `q-first`, `q-checkrun`,
    /// `h-checkjob`, `h-checkwait`, `h-checkonly`), and older records stay as
    /// they were.
    pub(crate) mod nineteenth_note_7 {
        use super::super::*;
        use crate::servers::the_machines_work::WhatThisRunHolds;
        use smoozy_queue_history::acting::adoption::TheMachinesWorkingState;
        use smoozy_queue_history::acting::note::{TheEndingOnTheNote, TheOutcomeAsWritten};
        use smoozy_queue_history::line::QueueState;
        use smoozy_queue_history::readings::{AJobAsTheMachineReadsIt, WhatTheEntryHasLive, WhatTheHoldSays};
        use smoozy_queue_history::record::LocalJobRecord;
        use smoozy_state::identities::{Identity, IdentityKind};
        use smoozy_state::observed::{Clock, MachineClock, Reading};
        use std::time::{Duration, SystemTime};

        /// **A check and its full range, both written, the pair saved as a
        /// submission saves it** (`Queue & History` P5a) — frame 1 of 1–240.
        pub(crate) struct APair {
            pub(crate) record: crate::record::LocalRecord,
            pub(crate) server: Identity,
            pub(crate) check: LocalJobRecord,
            pub(crate) range: LocalJobRecord,
        }

        /// `check_ended` is the check's ending, with two minutes rendered;
        /// `range_ended` the full range's; `range_written` whether the full
        /// range left a record at all (*Cancel* leaves none).
        pub(crate) fn a_pair(
            named: &str,
            check_ended: Option<TheOutcomeAsWritten>,
            range_ended: Option<TheOutcomeAsWritten>,
            range_written: bool,
            saved: bool,
        ) -> APair {
            let path = crate::scratch_folders::a_folder(format!("smoozy-note-7-{named}-{}", std::process::id()));
            let record = crate::record::LocalRecord::open_at(&path.join("local-record.sqlite3")).unwrap();
            let server = Identity::issue(IdentityKind::LocalServer).unwrap();
            let mut check = crate::record::deletions::tests::a_record_in_history(&server, "Hallway — final");
            let mut range = crate::record::deletions::tests::a_record_in_history(&server, "Hallway — final");
            let machine = Clock::AMachine(MachineClock(server.as_text()));
            let at = SystemTime::UNIX_EPOCH + Duration::from_secs(1_754_700_000);
            let ended = |made: &mut LocalJobRecord, outcome: Option<TheOutcomeAsWritten>, from: SystemTime, took: u64| {
                let Some(outcome) = outcome else {
                    made.the_render_start_has_been_learned = false;
                    return;
                };
                made.what_the_note_carried.started = Some(Reading::taken((), from, machine.clone()));
                made.what_the_note_carried.ended = Some(Reading::taken((), from + Duration::from_secs(took), machine.clone()));
                made.what_the_note_carried.ending = Some(TheEndingOnTheNote {
                    outcome,
                    exit_status: None,
                    the_attempt_whose_capture_record_holds_the_words: 1,
                    the_reason_the_machine_recorded: None,
                    no_render_began: false,
                    the_device_line_as_observed: None,
                });
            };
            ended(&mut check, check_ended, at, 120);
            ended(&mut range, range_ended, at + Duration::from_secs(600), 18_000);
            let store = crate::record::job_records::JobRecords::over(record.connection());
            store.write_the_provisional_record(&check).unwrap();
            if range_written {
                store.write_the_provisional_record(&range).unwrap();
            }
            if saved {
                for (role, pair, job) in [("check", &range.job, &check.job), ("range", &check.job, &range.job)] {
                    record.connection().execute(
                        "UPDATE job_record SET first_frame_purpose = ?1, first_frame_pair = ?2, first_frame_number = 1, first_frame_range = '1-240' WHERE job = ?3",
                        rusqlite::params![role, pair.as_text(), job.as_text()],
                    ).unwrap();
                }
            }
            // **The check's picture**, frame 1, as the record holds it.
            crate::record::previews::a_derivative_is_owed(record.connection(), &check.job, 1, "op", None, true, 0, None).unwrap();
            crate::record::previews::the_copy_was_published(
                record.connection(), &check.job, 1, None, "op",
                std::path::Path::new("/previews/check.png"), SystemTime::now(),
            ).unwrap();
            APair { record, server, check, range }
        }

        /// **What the line holds**, in its own order.
        fn the_line_holds(holds: &WhatThisRunHolds, server: &Identity, jobs: Vec<AJobAsTheMachineReadsIt>) {
            let connection = holds.a_connection_begins(&server.as_text());
            holds.a_reading_was_taken(
                &server.as_text(),
                connection,
                Reading::taken(
                    TheMachinesWorkingState {
                        jobs,
                        the_admission_point_stands_at: None,
                        notes: Vec::new(),
                        the_history_copys_count: 0,
                        the_history_copys_entries: Vec::new(),
                    },
                    SystemTime::UNIX_EPOCH + Duration::from_secs(1_754_700_000),
                    Clock::ThisComputer,
                ),
                None,
            );
        }

        fn in_line(job: &Identity, state: QueueState) -> AJobAsTheMachineReadsIt {
            AJobAsTheMachineReadsIt { job: job.clone(), state, live: WhatTheEntryHasLive::NoReadings, the_hold: None }
        }

        fn rendering() -> QueueState {
            QueueState::Rendering { operation: Identity::issue(IdentityKind::Operation).unwrap() }
        }

        /// The full range, held behind its check, as the machine's hold names it.
        fn held_behind(range: &Identity, check: &Identity) -> AJobAsTheMachineReadsIt {
            let instruction = smoozy_rendering::submission::the_hold_instruction(range, check);
            AJobAsTheMachineReadsIt {
                job: range.clone(),
                state: QueueState::Held { hold: Identity::issue(IdentityKind::Operation).unwrap() },
                live: WhatTheEntryHasLive::NoReadings,
                the_hold: Some(WhatTheHoldSays { reason: instruction.reason.to_string(), held_behind: Some(check.clone()) }),
            }
        }

        fn the_frames() -> String {
            the_full_ranges_frames(None, &smoozy_render_settings::frames::parse_the_frame_field("1-240").unwrap())
        }

        /// **`q-checkjob`: while the check renders**, the pair is one row on
        /// the check's place, naming both parts, and counts once; its entry is
        /// the check's, with the box above the status line — and a page that
        /// asks for the full range is answered with the pair's entry.
        #[test]
        fn while_the_check_renders_the_pair_is_one_row_and_its_entry_says_what_follows() {
            let pair = a_pair("check-renders", None, None, true, true);
            let holds = WhatThisRunHolds::default();
            the_line_holds(&holds, &pair.server, vec![in_line(&pair.check.job, rendering()), held_behind(&pair.range.job, &pair.check.job)]);
            for chosen in [None, Some(pair.range.job.as_text()), Some(pair.check.job.as_text())] {
                let screen = what_is_held(&holds, Some(&pair.record), chosen.as_deref());
                assert_eq!(screen.jobs, 1, "a pair counts once");
                let line = &screen.lines[0];
                assert_eq!((line.rendering, line.waiting), (1, 0));
                assert_eq!(line.rows.len(), 1);
                let row = &line.rows[0];
                assert_eq!(row.job, pair.check.job.as_text());
                assert_eq!(row.state, Some(RENDERING));
                assert_eq!(row.the_pair.len(), 2);
                assert_eq!(row.the_pair[0].what, APart::said(FIRST_FRAME_CHECK_ON_THE_ROW, "1", ""));
                assert_eq!((row.the_pair[0].state.lead, row.the_pair[0].dot), (PART_RENDERING, "now"));
                assert_eq!(row.the_pair[1].what, APart::said(FULL_RANGE_ON_THE_ROW, the_frames(), ""));
                assert_eq!((row.the_pair[1].state.lead, row.the_pair[1].dot), (PART_NOT_STARTED, ""));
                let entry = screen.entry.unwrap();
                assert_eq!(entry.job, pair.check.job.as_text(), "the readings and Stop are the check's");
                assert_eq!(entry.stop_or_cancel, Some(TheStopOrCancel::Stop));
                let the_check = entry.the_check.unwrap();
                assert_eq!(the_check.heading, APart::said(FIRST_FRAME_CHECK, "1", ""));
                assert_eq!(the_check.said, vec![APart::said(FRAMES_LEAD, the_frames(), START_ONLY_AFTER_YOU_REVIEW)]);
                assert!(the_check.picture.is_none() && !the_check.in_the_question && !the_check.you_chose_render_all);
            }
            // **The server row's work column counts the pair once too**, as
            // the Queue does.
            assert_eq!(
                what_the_servers_are_doing(&holds, Some(&pair.record)).get(&pair.server.as_text()),
                Some(&WhatAServerIsDoing::Rendering { name: "Hallway — final".to_string(), waiting: 0 })
            );
        }

        /// **`q-first`: while it waits**, the row is the full range's, held,
        /// with the check rendered; the picture sits in the question box,
        /// which names the full range, and the side column draws none.
        #[test]
        fn while_it_waits_the_picture_sits_in_the_question_box() {
            let pair = a_pair("waits", Some(TheOutcomeAsWritten::Finished), None, true, true);
            let holds = WhatThisRunHolds::default();
            the_line_holds(&holds, &pair.server, vec![held_behind(&pair.range.job, &pair.check.job)]);
            let screen = what_is_held(&holds, Some(&pair.record), Some(&pair.check.job.as_text()));
            let row = &screen.lines[0].rows[0];
            assert_eq!(row.job, pair.range.job.as_text());
            assert_eq!(row.state, Some(WAITING_ON_YOU));
            assert!(!row.grip, "held, with no grip");
            assert_eq!((row.the_pair[0].state.lead, row.the_pair[0].dot), (PART_RENDERED, "done"));
            assert_eq!((row.the_pair[1].state.lead, row.the_pair[1].dot), (PART_NOT_STARTED, ""));
            let entry = screen.entry.unwrap();
            assert_eq!(entry.job, pair.range.job.as_text(), "the answers are the full range's");
            assert!(entry.preview.is_none(), "the side column draws no preview while the pair waits");
            let the_check = entry.the_check.unwrap();
            assert!(the_check.in_the_question);
            let picture = the_check.picture.unwrap();
            assert_eq!(picture.picture, Some(pair.check.job.as_text()));
            assert_eq!(picture.frame, "1");
            assert_eq!(entry.held.len(), 1);
            assert_eq!(
                entry.held[0].said,
                vec![
                    APart::said(THE_FIRST_FRAME_LEAD, "1", IS_RENDERED_REVIEW_IT),
                    APart::said(REVIEW_IT_AND_CHOOSE, the_frames(), THE_QUESTION_MARK),
                ]
            );
            assert_eq!(entry.held[0].first_answer, the_first_frame_is_held("1", 240, false).first_answer);
            let text = format!("{:?}", the_queue_says(&what_is_held(&holds, Some(&pair.record), None)));
            for key in ["row0Pair0WhatValue", "row0Pair1StateLead", "row0Pair0Dot", "ffInTheQuestion", "ffpreviewPicture", "ffHeadingValue"] {
                assert!(text.contains(key), "{key} is said");
            }
            assert!(!text.contains("Purpose"), "the old label is not said");
        }

        /// **`q-checkrun`: once the full range renders**, the box keeps the
        /// check — its picture, how long it rendered, *You chose Render all*
        /// — and the rest of the entry is the full range's.
        #[test]
        fn once_the_full_range_renders_the_box_keeps_the_check() {
            let pair = a_pair("range-renders", Some(TheOutcomeAsWritten::Finished), None, true, true);
            let holds = WhatThisRunHolds::default();
            the_line_holds(&holds, &pair.server, vec![in_line(&pair.range.job, rendering())]);
            let screen = what_is_held(&holds, Some(&pair.record), None);
            let row = &screen.lines[0].rows[0];
            assert_eq!(row.state, Some(RENDERING));
            assert_eq!((row.the_pair[0].state.lead, row.the_pair[0].dot), (PART_RENDERED, "done"));
            assert_eq!((row.the_pair[1].state.lead, row.the_pair[1].dot), (PART_RENDERING, "now"));
            let entry = screen.entry.unwrap();
            assert_eq!(entry.job, pair.range.job.as_text());
            let the_check = entry.the_check.unwrap();
            assert!(!the_check.in_the_question);
            assert!(the_check.you_chose_render_all);
            assert_eq!(the_check.rendered_for.as_deref(), Some("2m"));
            assert_eq!(the_check.picture.unwrap().picture, Some(pair.check.job.as_text()));
            assert!(!the_check.blender_output, "no words are kept for this check");
        }

        /// **A check whose full range was cancelled while it rendered** is a
        /// row with its own line alone; **a check that failed** says so on
        /// the full range's row, never *rendered*.
        #[test]
        fn a_check_whose_range_is_gone_or_that_failed_says_only_what_is_true() {
            let pair = a_pair("range-gone", None, None, false, true);
            let holds = WhatThisRunHolds::default();
            the_line_holds(&holds, &pair.server, vec![in_line(&pair.check.job, rendering())]);
            let row = what_is_held(&holds, Some(&pair.record), None).lines[0].rows[0].clone();
            assert_eq!(row.the_pair.len(), 1);
            assert_eq!(row.the_pair[0].what.value, "1");

            let pair = a_pair("check-failed", Some(TheOutcomeAsWritten::Failed), None, true, true);
            let holds = WhatThisRunHolds::default();
            the_line_holds(&holds, &pair.server, vec![held_behind(&pair.range.job, &pair.check.job)]);
            let screen = what_is_held(&holds, Some(&pair.record), None);
            let row = &screen.lines[0].rows[0];
            assert_eq!((row.the_pair[0].state.lead, row.the_pair[0].dot), (ENDED_FAILED, ""));
            assert_eq!(
                screen.entry.unwrap().held[0].said,
                vec![APart::said(REVIEW_FRAME, "1", "."), APart::said(REVIEW_IT_AND_CHOOSE, the_frames(), THE_QUESTION_MARK)],
                "no word that frame 1 rendered"
            );
        }

        /// **Older records stay as they were**: a check and its held range
        /// with no saved purpose are two ordinary rows, and nothing guesses
        /// a check from a one-frame range.
        #[test]
        fn a_legacy_record_with_no_pair_stays_two_ordinary_rows() {
            let pair = a_pair("legacy", None, None, true, false);
            let holds = WhatThisRunHolds::default();
            the_line_holds(&holds, &pair.server, vec![in_line(&pair.check.job, rendering()), held_behind(&pair.range.job, &pair.check.job)]);
            let screen = what_is_held(&holds, Some(&pair.record), Some(&pair.range.job.as_text()));
            assert_eq!(screen.jobs, 2);
            assert!(screen.lines[0].rows.iter().all(|row| row.the_pair.is_empty()));
            let entry = screen.entry.unwrap();
            assert_eq!(entry.job, pair.range.job.as_text());
            assert!(entry.the_check.is_none());
        }

        /// **A pair counts once in the queue's count of endings** — while the
        /// full range waits the check is its own History entry and is
        /// counted; once both have ended they are one entry, counted once.
        #[test]
        fn a_pair_counts_once_in_the_count_of_endings() {
            use smoozy_queue_history::catch_up::{ANoticeEntry, NoticeRecord};
            use smoozy_queue_history::ending::Ending;
            let count = |pair: &APair| {
                let mut entries = vec![ANoticeEntry::Ended { job: pair.check.job.clone(), ending: Ending::Finished, sequence: 1 }];
                if pair.range.the_ending_learned().is_some() {
                    entries.push(ANoticeEntry::Ended { job: pair.range.job.clone(), ending: Ending::Finished, sequence: 2 });
                }
                crate::record::catch_up_notices::the_notice_is_written(
                    pair.record.connection(),
                    &NoticeRecord {
                        server: pair.server.clone(),
                        entries,
                        the_account_is_incomplete: false,
                    },
                )
                .unwrap();
                let holds = WhatThisRunHolds::default();
                what_is_held(&holds, Some(&pair.record), None).notice.map(|notice| notice.counted).unwrap_or_default()
            };
            let both = a_pair("count-both", Some(TheOutcomeAsWritten::Finished), Some(TheOutcomeAsWritten::Finished), true, true);
            assert_eq!(count(&both), vec![(Ending::Finished, 1)], "one entry, counted once");
            let waits = a_pair("count-waits", Some(TheOutcomeAsWritten::Finished), None, true, true);
            assert_eq!(count(&waits), vec![(Ending::Finished, 1)], "the check alone has ended");
            let legacy = a_pair("count-legacy", Some(TheOutcomeAsWritten::Finished), Some(TheOutcomeAsWritten::Finished), true, false);
            assert_eq!(count(&legacy), vec![(Ending::Finished, 2)], "older records are two entries");
        }

        /// **The page draws the drawing's own parts, and the old label is
        /// gone** — `.ffparts`, `.hff`, `.ffunit`, `.ffpic` and `.jwheld.ffask`
        /// are drawn by the page and carried by the drawing's sheet; the
        /// check's label `.firstlabel` is in neither.
        #[test]
        fn the_page_draws_the_drawings_parts_and_the_old_label_is_gone() {
            let web = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
            let read = |name: &str| std::fs::read_to_string(web.join(name)).expect("a page file");
            let (queue, history, sheet) = (read("the-queue.js"), read("the-history.js"), read("the-drawing.css"));
            for class in ["ffparts", "ffunit", "ffpic", "ffask", "ffw1", "ffw2", "ffq"] {
                assert!(queue.contains(&format!("\"{class}\"")), "the queue page draws {class}");
                assert!(sheet.contains(&format!(".{class}")), "the drawing's sheet carries {class}");
            }
            assert!(history.contains("thePairIsDrawn(said, \"hrow\" + at, \"hff\")"));
            assert!(sheet.contains(".hff {") && sheet.contains(".qst.e-chk {"));
            for page in [&queue, &history, &sheet] {
                assert!(!page.contains("firstlabel") && !page.contains("thePurposeIsDrawn"));
            }
        }

        /// **A pair's hold that names another job is not this pair's**: the
        /// machine's hold stays the authority (`Queue & History` P5a).
        #[test]
        fn a_hold_naming_another_job_is_no_pair() {
            let pair = a_pair("other-hold", Some(TheOutcomeAsWritten::Finished), None, true, true);
            let holds = WhatThisRunHolds::default();
            let elsewhere = Identity::issue(IdentityKind::Job).unwrap();
            the_line_holds(&holds, &pair.server, vec![held_behind(&pair.range.job, &elsewhere)]);
            let screen = what_is_held(&holds, Some(&pair.record), None);
            assert!(screen.lines[0].rows[0].the_pair.is_empty());
            assert!(screen.entry.unwrap().the_check.is_none());
        }
    }
}


#[cfg(test)]
mod wait_count_tests {
    use super::*;
    #[test]
    fn wait_count_draws_scene_numbers_without_render_bars() {
        use smoozy_queue_history::readings::{AWorkersReport, LiveReadings};
        let report = AWorkersReport {
            attempt: 1,
            readings: smoozy_state::observed::Reading::taken(
                LiveReadings { simulation: Some((-2, 430)), ..Default::default() },
                std::time::SystemTime::UNIX_EPOCH, smoozy_state::observed::Clock::ThisComputer),
            newest_frame_saved: None, the_plan_total: Some(100), the_units_finished: Some(0),
        };
        let progress = the_progress_from(&report, smoozy_queue_history::bars::WhichGrammar::FivePointZeroAndLater, None).unwrap();
        assert!(progress.simulating);
        assert_eq!(progress.now, vec![APart::said("frame", "-2", "of"), APart::value("430")]);
        assert_eq!(progress.now_at, None);
        assert_eq!(progress.whole, None);
    }
}
